// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Package mcp owns the repo Model Context Protocol server: the bounded JSON-RPC contract, the hash-chained event log, the line-delimited stdio transport, the session state machine, request routing and the repository tool/resource/prompt surface. The process that serves it over stdio lives in `🚀️bin`.

// #endregion 🧲️Header

package mcp

import (
	"bufio"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"

	"github.com/usalu/semio/repo/cli"
	"github.com/usalu/semio/repo/move"
	"github.com/usalu/semio/repo/providers"
	"github.com/usalu/semio/repo/tickets"
	"github.com/usalu/semio/repo/workspace"
)

// #region 📜️Protocol

const (
	JSONRPCVersion  = "2.0"
	ProtocolVersion = "2025-11-25"
)

var SupportedProtocolVersions = []string{ProtocolVersion, "2025-06-18", "2025-03-26", "2024-11-05", "2024-10-07"}

func negotiateProtocolVersion(requested string) string {
	for _, supported := range SupportedProtocolVersions {
		if requested == supported {
			return requested
		}
	}
	return ProtocolVersion
}

const (
	CodeParseError       = -32700
	CodeInvalidRequest   = -32600
	CodeMethodNotFound   = -32601
	CodeInvalidParams    = -32602
	CodeInternalError    = -32603
	CodePayloadTooLarge  = -32001
	CodeNotInitialized   = -32002
	CodeDuplicateRequest = -32003
	CodeStaleSession     = -32004
	CodeServerBusy       = -32005
	CodeRequestCancelled = -32800
)

var (
	ErrPayloadTooLarge = errors.New("mcp: payload too large")
	ErrNestingTooDeep  = errors.New("mcp: nesting too deep")
	ErrStaleSession    = errors.New("mcp: stale session")
	ErrPeerDropped     = errors.New("mcp: peer dropped")
	ErrClosed          = errors.New("mcp: session closed")
	ErrLimit           = errors.New("mcp: limit exceeded")
)

type ID struct {
	kind  byte
	text  string
	value int64
}

func StringID(value string) ID { return ID{kind: 's', text: value} }
func NumberID(value int64) ID  { return ID{kind: 'n', value: value} }
func (id ID) Valid() bool      { return id.kind == 's' || id.kind == 'n' }

func (id ID) String() string {
	if id.kind == 's' {
		return "s:" + id.text
	}
	if id.kind == 'n' {
		return "n:" + strconv.FormatInt(id.value, 10)
	}
	return ""
}

func (id ID) MarshalJSON() ([]byte, error) {
	switch id.kind {
	case 's':
		return json.Marshal(id.text)
	case 'n':
		return []byte(strconv.FormatInt(id.value, 10)), nil
	default:
		return nil, errors.New("mcp: invalid request id")
	}
}

func (id *ID) UnmarshalJSON(data []byte) error {
	if len(data) == 0 || bytes.Equal(data, []byte("null")) {
		return errors.New("mcp: request id must be a string or integer")
	}
	if data[0] == '"' {
		if err := json.Unmarshal(data, &id.text); err != nil {
			return errors.New("mcp: invalid string request id")
		}
		id.kind = 's'
		return nil
	}
	value, err := strconv.ParseInt(string(data), 10, 64)
	if err != nil {
		return errors.New("mcp: request id must be an integer")
	}
	id.kind, id.value = 'n', value
	return nil
}

type Request struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      *ID             `json:"id,omitempty"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params,omitempty"`
}

type Response struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id"`
	Result  json.RawMessage `json:"result,omitempty"`
	Error   *RPCError       `json:"error,omitempty"`
}

type RPCError struct {
	Code    int             `json:"code"`
	Message string          `json:"message"`
	Data    json.RawMessage `json:"data,omitempty"`
}

func (e *RPCError) Error() string {
	if e == nil {
		return ""
	}
	return fmt.Sprintf("mcp: %d %s", e.Code, e.Message)
}

type HandlerError struct {
	Code    int
	Message string
	Data    json.RawMessage
}

func (e *HandlerError) Error() string {
	if e == nil {
		return ""
	}
	return e.Message
}

// 🔒️decodeExact rejects unknown members. It owns the JSON-RPC envelope, the event log and the fixture files, never a params object.
func decodeExact(data []byte, target any) error { return decodeJSON(data, target, true) }

// 🪟️decodeOpen ignores unknown members so a params object stays open for extension.
func decodeOpen(data []byte, target any) error { return decodeJSON(data, target, false) }

func decodeJSON(data []byte, target any, strict bool) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	if strict {
		decoder.DisallowUnknownFields()
	}
	decoder.UseNumber()
	if err := decoder.Decode(target); err != nil {
		return err
	}
	var trailing any
	if err := decoder.Decode(&trailing); !errors.Is(err, io.EOF) {
		if err == nil {
			return errors.New("mcp: trailing JSON value")
		}
		return err
	}
	return nil
}

func paramsObject(raw json.RawMessage) (json.RawMessage, error) {
	if len(raw) == 0 {
		raw = []byte("{}")
	}
	trimmed := bytes.TrimSpace(raw)
	if len(trimmed) == 0 || trimmed[0] != '{' {
		return nil, errors.New("mcp: params must be an object")
	}
	return raw, nil
}

// 🪟️DecodeParams decodes any MCP `params` object. Unknown members are ignored: the specification declares
// every params object open for extension, and real clients send `clientInfo.title`, extra capability
// objects and `_meta` keys that a strict decoder would turn into a fatal -32602 handshake failure.
func DecodeParams(raw json.RawMessage, target any) error {
	object, err := paramsObject(raw)
	if err != nil {
		return err
	}
	return decodeOpen(object, target)
}

// 🔒️DecodeArguments decodes a `tools/call` arguments object. Tool arguments are the one closed surface:
// an unrecognised argument is a caller mistake that must fail loudly instead of being silently dropped.
func DecodeArguments(raw json.RawMessage, target any) error {
	object, err := paramsObject(raw)
	if err != nil {
		return err
	}
	return decodeExact(object, target)
}

type Implementation struct {
	Name    string `json:"name"`
	Version string `json:"version"`
}

type ClientCapabilities struct {
	Roots       *RootsCapability `json:"roots,omitempty"`
	Sampling    *EmptyCapability `json:"sampling,omitempty"`
	Elicitation *EmptyCapability `json:"elicitation,omitempty"`
}

type ServerCapabilities struct {
	Logging     *EmptyCapability    `json:"logging,omitempty"`
	Prompts     *ListCapability     `json:"prompts,omitempty"`
	Resources   *ResourceCapability `json:"resources,omitempty"`
	Tools       *ListCapability     `json:"tools,omitempty"`
	Completions *EmptyCapability    `json:"completions,omitempty"`
}

type EmptyCapability struct{}

type ListCapability struct {
	ListChanged bool `json:"listChanged,omitempty"`
}

type ResourceCapability struct {
	Subscribe   bool `json:"subscribe,omitempty"`
	ListChanged bool `json:"listChanged,omitempty"`
}

type RootsCapability struct {
	ListChanged bool `json:"listChanged,omitempty"`
}

type InitializeParams struct {
	ProtocolVersion string             `json:"protocolVersion"`
	Capabilities    ClientCapabilities `json:"capabilities"`
	ClientInfo      Implementation     `json:"clientInfo"`
}

type InitializeResult struct {
	ProtocolVersion string             `json:"protocolVersion"`
	Capabilities    ServerCapabilities `json:"capabilities"`
	ServerInfo      Implementation     `json:"serverInfo"`
	Instructions    string             `json:"instructions,omitempty"`
}

type ListParams struct {
	Cursor string `json:"cursor,omitempty"`
}

type ListMeta struct {
	ProgressToken json.RawMessage `json:"progressToken,omitempty"`
}

type Schema struct {
	Type                 string            `json:"type,omitempty"`
	Description          string            `json:"description,omitempty"`
	Properties           map[string]Schema `json:"properties,omitempty"`
	Required             []string          `json:"required,omitempty"`
	Items                *Schema           `json:"items,omitempty"`
	AdditionalProperties *bool             `json:"additionalProperties,omitempty"`
	Enum                 []string          `json:"enum,omitempty"`
}

type Tool struct {
	Name        string `json:"name"`
	Title       string `json:"title,omitempty"`
	Description string `json:"description,omitempty"`
	InputSchema Schema `json:"inputSchema"`
}

type ListToolsResult struct {
	Tools      []Tool `json:"tools"`
	NextCursor string `json:"nextCursor,omitempty"`
}

type CallToolParams struct {
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments,omitempty"`
	Meta      ListMeta        `json:"_meta,omitempty"`
}

type Content struct {
	Type     string `json:"type"`
	Text     string `json:"text,omitempty"`
	Data     string `json:"data,omitempty"`
	MIMEType string `json:"mimeType,omitempty"`
	URI      string `json:"uri,omitempty"`
}

type CallToolResult struct {
	Content           []Content       `json:"content"`
	StructuredContent json.RawMessage `json:"structuredContent,omitempty"`
	IsError           bool            `json:"isError,omitempty"`
}

type Resource struct {
	URI         string `json:"uri"`
	Name        string `json:"name"`
	Title       string `json:"title,omitempty"`
	Description string `json:"description,omitempty"`
	MIMEType    string `json:"mimeType,omitempty"`
}

type ResourceTemplate struct {
	URITemplate string `json:"uriTemplate"`
	Name        string `json:"name"`
	Title       string `json:"title,omitempty"`
	Description string `json:"description,omitempty"`
	MIMEType    string `json:"mimeType,omitempty"`
}

type ListResourcesResult struct {
	Resources  []Resource `json:"resources"`
	NextCursor string     `json:"nextCursor,omitempty"`
}

type ListResourceTemplatesResult struct {
	ResourceTemplates []ResourceTemplate `json:"resourceTemplates"`
	NextCursor        string             `json:"nextCursor,omitempty"`
}

type ReadResourceParams struct {
	URI  string   `json:"uri"`
	Meta ListMeta `json:"_meta,omitempty"`
}

type ResourceContent struct {
	URI      string `json:"uri"`
	MIMEType string `json:"mimeType,omitempty"`
	Text     string `json:"text,omitempty"`
	Blob     string `json:"blob,omitempty"`
}

type ReadResourceResult struct {
	Contents []ResourceContent `json:"contents"`
}

type PromptArgument struct {
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	Required    bool   `json:"required,omitempty"`
}

type Prompt struct {
	Name        string           `json:"name"`
	Title       string           `json:"title,omitempty"`
	Description string           `json:"description,omitempty"`
	Arguments   []PromptArgument `json:"arguments,omitempty"`
}

type ListPromptsResult struct {
	Prompts    []Prompt `json:"prompts"`
	NextCursor string   `json:"nextCursor,omitempty"`
}

type GetPromptParams struct {
	Name      string            `json:"name"`
	Arguments map[string]string `json:"arguments,omitempty"`
	Meta      ListMeta          `json:"_meta,omitempty"`
}

type PromptMessage struct {
	Role    string  `json:"role"`
	Content Content `json:"content"`
}

type GetPromptResult struct {
	Description string          `json:"description,omitempty"`
	Messages    []PromptMessage `json:"messages"`
}

type CancelParams struct {
	RequestID ID     `json:"requestId"`
	Reason    string `json:"reason,omitempty"`
}

type ProgressParams struct {
	ProgressToken json.RawMessage `json:"progressToken"`
	Progress      float64         `json:"progress"`
	Total         *float64        `json:"total,omitempty"`
	Message       string          `json:"message,omitempty"`
}

// #endregion 📜️Protocol

// #region 📡️Event

const EventSchema = "semio.mcp.event/1"

type Event struct {
	Schema     string          `json:"schema"`
	Sequence   uint64          `json:"sequence"`
	Kind       string          `json:"kind"`
	Peer       string          `json:"peer"`
	Generation uint64          `json:"generation"`
	RequestID  string          `json:"requestId,omitempty"`
	Payload    json.RawMessage `json:"payload"`
	Previous   string          `json:"previous,omitempty"`
	Hash       string          `json:"hash"`
}

type EventInput struct {
	Kind       string
	Peer       string
	Generation uint64
	RequestID  string
	Payload    json.RawMessage
}

type EventLog struct {
	mu       sync.RWMutex
	events   []Event
	data     []byte
	maxBytes int
	maxCount int
}

func NewEventLog(maxBytes, maxCount int) *EventLog {
	return &EventLog{maxBytes: maxBytes, maxCount: maxCount}
}

func (log *EventLog) Commit(ctx context.Context, inputs ...EventInput) error {
	if len(inputs) == 0 {
		return nil
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	log.mu.Lock()
	defer log.mu.Unlock()
	if log.maxCount > 0 && len(log.events)+len(inputs) > log.maxCount {
		return ErrLimit
	}
	previous := ""
	if len(log.events) > 0 {
		previous = log.events[len(log.events)-1].Hash
	}
	stagedEvents := make([]Event, 0, len(inputs))
	var staged bytes.Buffer
	for index, input := range inputs {
		if err := ctx.Err(); err != nil {
			return err
		}
		if input.Kind == "" || !json.Valid(input.Payload) {
			return errors.New("mcp: invalid event")
		}
		event := Event{Schema: EventSchema, Sequence: uint64(len(log.events) + index + 1), Kind: input.Kind, Peer: input.Peer, Generation: input.Generation, RequestID: input.RequestID, Payload: append(json.RawMessage(nil), input.Payload...), Previous: previous}
		digest, err := eventDigest(event)
		if err != nil {
			return err
		}
		event.Hash = digest
		encoded, err := json.Marshal(event)
		if err != nil {
			return err
		}
		staged.Write(encoded)
		staged.WriteByte('\n')
		stagedEvents = append(stagedEvents, event)
		previous = digest
	}
	if log.maxBytes > 0 && len(log.data)+staged.Len() > log.maxBytes {
		return ErrLimit
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	log.events = append(log.events, stagedEvents...)
	log.data = append(log.data, staged.Bytes()...)
	return nil
}

func (log *EventLog) Snapshot() []byte {
	log.mu.RLock()
	defer log.mu.RUnlock()
	return append([]byte(nil), log.data...)
}

func (log *EventLog) Events() []Event {
	log.mu.RLock()
	defer log.mu.RUnlock()
	result := make([]Event, len(log.events))
	copy(result, log.events)
	return result
}

func ReplayEvents(ctx context.Context, data []byte, maxBytes, maxCount int) ([]Event, error) {
	if maxBytes > 0 && len(data) > maxBytes {
		return nil, ErrLimit
	}
	scanner := bufio.NewScanner(bytes.NewReader(data))
	limit := maxBytes
	if limit <= 0 {
		limit = 64 << 20
	}
	scanner.Buffer(make([]byte, 64*1024), limit)
	result := make([]Event, 0)
	previous := ""
	for scanner.Scan() {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if maxCount > 0 && len(result) == maxCount {
			return nil, ErrLimit
		}
		var event Event
		if err := decodeExact(scanner.Bytes(), &event); err != nil {
			return nil, fmt.Errorf("mcp: corrupt event %d: %w", len(result)+1, err)
		}
		if event.Schema != EventSchema || event.Sequence != uint64(len(result)+1) || event.Previous != previous || event.Kind == "" || !json.Valid(event.Payload) {
			return nil, fmt.Errorf("mcp: corrupt event %d", len(result)+1)
		}
		digest, err := eventDigest(event)
		if err != nil || digest != event.Hash {
			return nil, fmt.Errorf("mcp: corrupt event %d", len(result)+1)
		}
		result = append(result, event)
		previous = event.Hash
	}
	if err := scanner.Err(); err != nil {
		if errors.Is(err, bufio.ErrTooLong) {
			return nil, ErrLimit
		}
		return nil, err
	}
	return result, nil
}

func eventDigest(event Event) (string, error) {
	event.Hash = ""
	encoded, err := json.Marshal(event)
	if err != nil {
		return "", err
	}
	digest := sha256.Sum256(encoded)
	return hex.EncodeToString(digest[:]), nil
}

func cloneRaw(raw json.RawMessage) json.RawMessage {
	return append(json.RawMessage(nil), raw...)
}

func readOneJSON(decoder *json.Decoder, target any) error {
	if err := decoder.Decode(target); err != nil {
		return err
	}
	var extra any
	if err := decoder.Decode(&extra); !errors.Is(err, io.EOF) {
		if err == nil {
			return errors.New("mcp: trailing JSON value")
		}
		return err
	}
	return nil
}

// #endregion 📡️Event

// #region 🚚️Transport

type Transport interface {
	io.Reader
	io.Writer
	io.Closer
}

func (server *Server) Serve(ctx context.Context, peer string, transport Transport) error {
	var writeMu sync.Mutex
	write := func(ctx context.Context, payload []byte) error {
		if err := ctx.Err(); err != nil {
			return err
		}
		writeMu.Lock()
		defer writeMu.Unlock()
		if _, err := transport.Write(append(append([]byte(nil), payload...), '\n')); err != nil {
			return ErrPeerDropped
		}
		return nil
	}
	session, err := server.Connect(peer, write)
	if err != nil {
		return err
	}
	defer transport.Close()
	jobs := make(chan []byte, server.config.Limits.MaxHandlers)
	workerErrors := make(chan error, 1)
	var workers sync.WaitGroup
	for range server.config.Limits.MaxHandlers {
		workers.Add(1)
		server.handlerWorkers.Add(1)
		go func() {
			defer workers.Done()
			defer server.handlerWorkers.Add(-1)
			for payload := range jobs {
				server.handlerQueued.Add(-1)
				server.handlerActive.Add(1)
				response, err := session.Dispatch(ctx, payload)
				if err == nil && len(response) > 0 {
					err = write(ctx, response)
				}
				server.handlerActive.Add(-1)
				if err != nil {
					select {
					case workerErrors <- err:
					default:
					}
					session.drop(err)
				}
			}
		}()
	}
	// 🚰️drain lets every payload the peer already delivered reach a handler before the session is
	// dropped. A client that writes its whole burst and closes stdin (`printf … | semio-repo-mcp`)
	// reaches EOF while requests are still queued; dropping first answered them `session closed`.
	drain := func(cause error) error {
		close(jobs)
		workers.Wait()
		session.drop(cause)
		select {
		case workerErr := <-workerErrors:
			return workerErr
		default:
			return cause
		}
	}
	finish := func(cause error) error {
		session.drop(cause)
		return drain(cause)
	}
	scanner := bufio.NewScanner(transport)
	scanner.Buffer(make([]byte, 64*1024), server.config.Limits.MaxPayloadBytes+1)
	for scanner.Scan() {
		if err := ctx.Err(); err != nil {
			return finish(err)
		}
		payload := append([]byte(nil), scanner.Bytes()...)
		request, _, protocolError := session.decodeRequest(payload)
		if protocolError == nil && request.ID == nil {
			if _, err := session.Dispatch(ctx, payload); err != nil && !errors.Is(err, ErrStaleSession) {
				return finish(err)
			}
			continue
		}
		// 🤝️The latch is armed BEFORE the payload reaches a worker: arming it afterwards races the
		// worker that already answered, and the decrement would then be lost for good.
		handshake := protocolError == nil && request.Method == "initialize"
		if handshake {
			session.ExpectHandshake()
		}
		server.handlerQueued.Add(1)
		select {
		case jobs <- payload:
		default:
			server.handlerQueued.Add(-1)
			if handshake {
				session.SettleHandshake()
			}
			response, err := session.reject(payload, rpcError(CodeServerBusy, "handler queue full"))
			if err != nil {
				return finish(err)
			}
			if err := write(ctx, response); err != nil {
				return finish(err)
			}
		}
	}
	if err := scanner.Err(); err != nil {
		if errors.Is(err, bufio.ErrTooLong) {
			return finish(ErrPayloadTooLarge)
		}
		return finish(err)
	}
	if err := ctx.Err(); err != nil {
		return finish(err)
	}
	return drain(ErrPeerDropped)
}

func (session *Session) reject(payload []byte, rejection *RPCError) ([]byte, error) {
	request, id, protocolError := session.decodeRequest(payload)
	if protocolError != nil {
		rejection = protocolError
	}
	response, err := session.encodeError(id, rejection)
	if err != nil {
		return nil, err
	}
	key := ""
	if request.ID != nil {
		key = request.ID.String()
	}
	if err := session.commitExchange(key, payload, response); err != nil {
		return nil, err
	}
	return response, nil
}

// #endregion 🚚️Transport

// #region 🔐️Session

type Limits struct {
	MaxPayloadBytes  int
	MaxNesting       int
	MaxPageItems     int
	MaxRegistryItems int
	MaxRecentIDs     int
	MaxEventBytes    int
	MaxEvents        int
	MaxHandlers      int
}

func DefaultLimits() Limits {
	return Limits{MaxPayloadBytes: 1 << 20, MaxNesting: 64, MaxPageItems: 64, MaxRegistryItems: 4096, MaxRecentIDs: 4096, MaxEventBytes: 64 << 20, MaxEvents: 100000, MaxHandlers: 8}
}

func (limits Limits) normalized() (Limits, error) {
	defaults := DefaultLimits()
	values := []*int{&limits.MaxPayloadBytes, &limits.MaxNesting, &limits.MaxPageItems, &limits.MaxRegistryItems, &limits.MaxRecentIDs, &limits.MaxEventBytes, &limits.MaxEvents, &limits.MaxHandlers}
	fallbacks := []int{defaults.MaxPayloadBytes, defaults.MaxNesting, defaults.MaxPageItems, defaults.MaxRegistryItems, defaults.MaxRecentIDs, defaults.MaxEventBytes, defaults.MaxEvents, defaults.MaxHandlers}
	for index, value := range values {
		if *value < 0 {
			return Limits{}, errors.New("mcp: negative limit")
		}
		if *value == 0 {
			*value = fallbacks[index]
		}
	}
	return limits, nil
}

type Config struct {
	ServerInfo   Implementation
	Instructions string
	Limits       Limits
}

type Sink func(context.Context, []byte) error

type ToolHandler func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error)
type ResourceHandler func(context.Context, ReadResourceParams, ProgressReporter) (ReadResourceResult, error)
type PromptHandler func(context.Context, GetPromptParams, ProgressReporter) (GetPromptResult, error)

type ProgressReporter interface {
	Report(context.Context, float64, *float64, string) error
}

type toolRegistration struct {
	schema  Tool
	handler ToolHandler
}

type resourceRegistration struct {
	schema  Resource
	handler ResourceHandler
}

type promptRegistration struct {
	schema  Prompt
	handler PromptHandler
}

type Server struct {
	mu             sync.RWMutex
	config         Config
	log            *EventLog
	tools          map[string]toolRegistration
	resources      map[string]resourceRegistration
	templates      map[string]ResourceTemplate
	prompts        map[string]promptRegistration
	generations    map[string]uint64
	sessions       map[string]*Session
	handlerWorkers atomic.Int64
	handlerActive  atomic.Int64
	handlerQueued  atomic.Int64
}

func NewServer(config Config) (*Server, error) {
	limits, err := config.Limits.normalized()
	if err != nil {
		return nil, err
	}
	if strings.TrimSpace(config.ServerInfo.Name) == "" || strings.TrimSpace(config.ServerInfo.Version) == "" {
		return nil, errors.New("mcp: server name and version are required")
	}
	config.Limits = limits
	return &Server{config: config, log: NewEventLog(limits.MaxEventBytes, limits.MaxEvents), tools: map[string]toolRegistration{}, resources: map[string]resourceRegistration{}, templates: map[string]ResourceTemplate{}, prompts: map[string]promptRegistration{}, generations: map[string]uint64{}, sessions: map[string]*Session{}}, nil
}

func (server *Server) RegisterTool(schema Tool, handler ToolHandler) error {
	if strings.TrimSpace(schema.Name) == "" || handler == nil || schema.InputSchema.Type != "object" {
		return errors.New("mcp: invalid tool registration")
	}
	server.mu.Lock()
	defer server.mu.Unlock()
	if _, exists := server.tools[schema.Name]; exists {
		return errors.New("mcp: duplicate tool")
	}
	if len(server.tools) == server.config.Limits.MaxRegistryItems {
		return ErrLimit
	}
	server.tools[schema.Name] = toolRegistration{schema: schema, handler: handler}
	return nil
}

func (server *Server) RegisterResource(schema Resource, handler ResourceHandler) error {
	if strings.TrimSpace(schema.URI) == "" || strings.TrimSpace(schema.Name) == "" || handler == nil {
		return errors.New("mcp: invalid resource registration")
	}
	server.mu.Lock()
	defer server.mu.Unlock()
	if _, exists := server.resources[schema.URI]; exists {
		return errors.New("mcp: duplicate resource")
	}
	if len(server.resources) == server.config.Limits.MaxRegistryItems {
		return ErrLimit
	}
	server.resources[schema.URI] = resourceRegistration{schema: schema, handler: handler}
	return nil
}

func (server *Server) RegisterResourceTemplate(schema ResourceTemplate) error {
	if strings.TrimSpace(schema.URITemplate) == "" || strings.TrimSpace(schema.Name) == "" {
		return errors.New("mcp: invalid resource template registration")
	}
	server.mu.Lock()
	defer server.mu.Unlock()
	if _, exists := server.templates[schema.URITemplate]; exists {
		return errors.New("mcp: duplicate resource template")
	}
	if len(server.templates) == server.config.Limits.MaxRegistryItems {
		return ErrLimit
	}
	server.templates[schema.URITemplate] = schema
	return nil
}

func (server *Server) RegisterPrompt(schema Prompt, handler PromptHandler) error {
	if strings.TrimSpace(schema.Name) == "" || handler == nil {
		return errors.New("mcp: invalid prompt registration")
	}
	server.mu.Lock()
	defer server.mu.Unlock()
	if _, exists := server.prompts[schema.Name]; exists {
		return errors.New("mcp: duplicate prompt")
	}
	if len(server.prompts) == server.config.Limits.MaxRegistryItems {
		return ErrLimit
	}
	server.prompts[schema.Name] = promptRegistration{schema: schema, handler: handler}
	return nil
}

func (server *Server) Connect(peer string, sink Sink) (*Session, error) {
	if strings.TrimSpace(peer) == "" {
		return nil, errors.New("mcp: peer is required")
	}
	server.mu.Lock()
	previous := server.sessions[peer]
	server.generations[peer]++
	generation := server.generations[peer]
	session := newSession(server, peer, generation, sink)
	server.sessions[peer] = session
	server.mu.Unlock()
	if previous != nil {
		previous.drop(ErrPeerDropped)
	}
	payload := json.RawMessage(fmt.Sprintf(`{"generation":%d}`, generation))
	if err := server.log.Commit(context.Background(), EventInput{Kind: "session.opened", Peer: peer, Generation: generation, Payload: payload}); err != nil {
		session.drop(err)
		return nil, err
	}
	return session, nil
}

func (server *Server) Events() *EventLog { return server.log }

type HandlerStats struct {
	Capacity int
	Workers  int64
	Active   int64
	Queued   int64
	Credits  int64
}

func (server *Server) HandlerStats() HandlerStats {
	queued := server.handlerQueued.Load()
	return HandlerStats{Capacity: server.config.Limits.MaxHandlers, Workers: server.handlerWorkers.Load(), Active: server.handlerActive.Load(), Queued: queued, Credits: int64(server.config.Limits.MaxHandlers) - queued}
}

func (server *Server) owns(session *Session) bool {
	server.mu.RLock()
	defer server.mu.RUnlock()
	return server.sessions[session.peer] == session && server.generations[session.peer] == session.generation
}

func (server *Server) capabilities() ServerCapabilities {
	server.mu.RLock()
	defer server.mu.RUnlock()
	capabilities := ServerCapabilities{}
	if len(server.tools) > 0 {
		capabilities.Tools = &ListCapability{}
	}
	if len(server.resources) > 0 || len(server.templates) > 0 {
		capabilities.Resources = &ResourceCapability{}
	}
	if len(server.prompts) > 0 {
		capabilities.Prompts = &ListCapability{}
	}
	return capabilities
}

type sessionPhase uint8

const (
	phaseConnected sessionPhase = iota
	phaseInitialized
	phaseReady
	phaseClosing
	phaseClosed
)

type activeRequest struct {
	cancel context.CancelCauseFunc
}

type Session struct {
	server     *Server
	peer       string
	generation uint64
	sink       Sink
	mu         sync.Mutex
	phase      sessionPhase
	// 📥️ A `notifications/initialized` that overtook its own `initialize` response. A client is
	// entitled to pipeline the two, and the reader hands notifications straight through while the
	// request is still on a worker — so the notification is REMEMBERED here instead of dropped, and
	// the initialize path applies it the moment the phase it is waiting for exists.
	pendingInitialized bool
	// 🤝️ How many `initialize` requests the transport has delivered and no worker has answered yet.
	// A burst is one write, so a request that follows `initialize` on the wire can reach a second
	// worker before the first has promoted the phase; without this latch it answers `-32002`.
	handshakePending int
	handshake        *sync.Cond
	active           map[string]*activeRequest
	seen             map[string]struct{}
	seenOrder        []string
	preCancelled     map[string]string
	wait             sync.WaitGroup
	done             chan struct{}
	shutdownOnce     sync.Once
	closeErr         error
}

func newSession(server *Server, peer string, generation uint64, sink Sink) *Session {
	session := &Session{server: server, peer: peer, generation: generation, sink: sink, active: map[string]*activeRequest{}, seen: map[string]struct{}{}, preCancelled: map[string]string{}, done: make(chan struct{})}
	session.handshake = sync.NewCond(&session.mu)
	return session
}

// 🤝️ExpectHandshake records that an `initialize` request has been delivered and is not answered yet.
// The transport calls it before the payload reaches the worker pool, so a request delivered after it
// in the same burst cannot overtake the phase promotion.
func (session *Session) ExpectHandshake() {
	session.mu.Lock()
	session.handshakePending++
	session.mu.Unlock()
}

// 🤝️SettleHandshake releases every request that queued behind a delivered `initialize`.
func (session *Session) SettleHandshake() {
	session.mu.Lock()
	if session.handshakePending > 0 {
		session.handshakePending--
	}
	session.mu.Unlock()
	session.handshake.Broadcast()
}

// ⏳️awaitHandshake blocks while an `initialize` delivered before this request is still unanswered.
func (session *Session) awaitHandshake() {
	session.mu.Lock()
	for session.handshakePending > 0 && session.phase < phaseClosing {
		session.handshake.Wait()
	}
	session.mu.Unlock()
}

func (session *Session) Peer() string       { return session.peer }
func (session *Session) Generation() uint64 { return session.generation }

func (session *Session) Dispatch(ctx context.Context, payload []byte) ([]byte, error) {
	request, idRaw, protocolError := session.decodeRequest(payload)
	if protocolError != nil {
		return session.encodeError(idRaw, protocolError)
	}
	if request.ID == nil {
		return nil, session.handleNotification(request, payload)
	}
	if request.Method == "initialize" {
		defer session.SettleHandshake()
	} else {
		session.awaitHandshake()
	}
	if !session.server.owns(session) {
		return session.encodeError(idRaw, rpcError(CodeStaleSession, "stale session"))
	}
	requestKey := request.ID.String()
	requestContext, finish, beginError := session.beginRequest(ctx, requestKey)
	if beginError != nil {
		response, err := session.encodeError(idRaw, beginError)
		if err == nil {
			err = session.commitExchange(requestKey, payload, response)
		}
		if err != nil {
			return nil, err
		}
		return response, err
	}
	defer finish()
	result, callError := session.route(requestContext, request)
	response, err := session.encodeResult(idRaw, result, callError)
	if err != nil {
		return nil, err
	}
	if err := session.commitExchange(requestKey, payload, response); err != nil {
		return nil, err
	}
	return response, nil
}

func (session *Session) beginRequest(parent context.Context, key string) (context.Context, func(), *RPCError) {
	session.mu.Lock()
	if session.phase >= phaseClosing {
		session.mu.Unlock()
		return nil, func() {}, rpcError(CodeStaleSession, "session closed")
	}
	if _, exists := session.active[key]; exists {
		session.mu.Unlock()
		return nil, func() {}, rpcError(CodeDuplicateRequest, "duplicate request id")
	}
	if _, exists := session.seen[key]; exists {
		session.mu.Unlock()
		return nil, func() {}, rpcError(CodeDuplicateRequest, "stale request id")
	}
	requestContext, cancel := context.WithCancelCause(parent)
	if reason, cancelled := session.preCancelled[key]; cancelled {
		delete(session.preCancelled, key)
		cancel(errors.New(reason))
	}
	session.active[key] = &activeRequest{cancel: cancel}
	session.wait.Add(1)
	session.mu.Unlock()
	return requestContext, func() {
		session.mu.Lock()
		delete(session.active, key)
		session.remember(key)
		session.mu.Unlock()
		session.wait.Done()
	}, nil
}

func (session *Session) remember(key string) {
	session.seen[key] = struct{}{}
	session.seenOrder = append(session.seenOrder, key)
	if len(session.seenOrder) > session.server.config.Limits.MaxRecentIDs {
		delete(session.seen, session.seenOrder[0])
		session.seenOrder = session.seenOrder[1:]
	}
}

func (session *Session) Close(ctx context.Context) error {
	session.shutdown(ErrClosed, true)
	select {
	case <-session.done:
		session.mu.Lock()
		defer session.mu.Unlock()
		return session.closeErr
	case <-ctx.Done():
		return ctx.Err()
	}
}

func (session *Session) shutdown(cause error, record bool) {
	session.shutdownOnce.Do(func() {
		session.mu.Lock()
		session.phase = phaseClosing
		for _, active := range session.active {
			active.cancel(cause)
		}
		session.mu.Unlock()
		session.handshake.Broadcast()
		go func() {
			session.wait.Wait()
			session.mu.Lock()
			session.phase = phaseClosed
			session.mu.Unlock()
			if record {
				payload := json.RawMessage(`{"reason":"closed"}`)
				if err := session.server.log.Commit(context.Background(), EventInput{Kind: "session.closed", Peer: session.peer, Generation: session.generation, Payload: payload}); err != nil {
					session.mu.Lock()
					session.closeErr = err
					session.mu.Unlock()
				}
			}
			close(session.done)
		}()
	})
}

func (session *Session) drop(cause error) {
	session.shutdown(cause, false)
}

func (session *Session) Done() <-chan struct{} { return session.done }

// #endregion 🔐️Session

// #region 🚦️Routing

func (session *Session) route(ctx context.Context, request Request) (any, *RPCError) {
	if err := ctx.Err(); err != nil {
		return nil, rpcError(CodeRequestCancelled, "request cancelled")
	}
	session.mu.Lock()
	phase := session.phase
	session.mu.Unlock()
	if request.Method == "initialize" {
		if phase != phaseConnected {
			return nil, rpcError(CodeInvalidRequest, "already initialized")
		}
		var params InitializeParams
		if err := DecodeParams(request.Params, &params); err != nil || params.ProtocolVersion == "" || params.ClientInfo.Name == "" || params.ClientInfo.Version == "" {
			return nil, rpcError(CodeInvalidParams, "invalid initialize params")
		}
		session.mu.Lock()
		if session.phase != phaseConnected {
			session.mu.Unlock()
			return nil, rpcError(CodeInvalidRequest, "already initialized")
		}
		session.phase = phaseInitialized
		if session.pendingInitialized {
			session.pendingInitialized = false
			session.phase = phaseReady
		}
		session.mu.Unlock()
		return InitializeResult{ProtocolVersion: negotiateProtocolVersion(params.ProtocolVersion), Capabilities: session.server.capabilities(), ServerInfo: session.server.config.ServerInfo, Instructions: session.server.config.Instructions}, nil
	}
	if phase != phaseReady {
		return nil, rpcError(CodeNotInitialized, "session not initialized")
	}
	switch request.Method {
	case "ping":
		var params struct{}
		if err := DecodeParams(request.Params, &params); err != nil {
			return nil, rpcError(CodeInvalidParams, "invalid params")
		}
		return struct{}{}, nil
	case "tools/list":
		return session.listTools(request.Params)
	case "tools/call":
		return session.callTool(ctx, request.Params)
	case "resources/list":
		return session.listResources(request.Params)
	case "resources/templates/list":
		return session.listResourceTemplates(request.Params)
	case "resources/read":
		return session.readResource(ctx, request.Params)
	case "prompts/list":
		return session.listPrompts(request.Params)
	case "prompts/get":
		return session.getPrompt(ctx, request.Params)
	default:
		return nil, rpcError(CodeMethodNotFound, "method not found")
	}
}

func (session *Session) handleNotification(request Request, payload []byte) error {
	if !session.server.owns(session) {
		return ErrStaleSession
	}
	switch request.Method {
	case "notifications/initialized":
		var params struct{}
		if DecodeParams(request.Params, &params) == nil {
			session.mu.Lock()
			switch session.phase {
			case phaseInitialized:
				session.phase = phaseReady
			case phaseConnected:
				session.pendingInitialized = true
			}
			session.mu.Unlock()
		}
	case "notifications/cancelled":
		var params CancelParams
		if DecodeParams(request.Params, &params) == nil && params.RequestID.Valid() {
			session.cancel(params)
		}
	}
	return session.server.log.Commit(context.Background(), EventInput{Kind: "notification.received", Peer: session.peer, Generation: session.generation, Payload: cloneRaw(payload)})
}

func (session *Session) cancel(params CancelParams) {
	key := params.RequestID.String()
	reason := params.Reason
	if reason == "" {
		reason = "cancelled by peer"
	}
	session.mu.Lock()
	defer session.mu.Unlock()
	if active := session.active[key]; active != nil {
		active.cancel(errors.New(reason))
		return
	}
	if _, complete := session.seen[key]; complete {
		return
	}
	if len(session.preCancelled) < session.server.config.Limits.MaxRecentIDs {
		session.preCancelled[key] = reason
	}
}

func (session *Session) callTool(ctx context.Context, raw json.RawMessage) (any, *RPCError) {
	var params CallToolParams
	if err := DecodeParams(raw, &params); err != nil || params.Name == "" {
		return nil, rpcError(CodeInvalidParams, "invalid tool params")
	}
	if len(params.Arguments) == 0 {
		params.Arguments = json.RawMessage(`{}`)
	}
	if trimmed := bytes.TrimSpace(params.Arguments); !json.Valid(params.Arguments) || len(trimmed) == 0 || trimmed[0] != '{' {
		return nil, rpcError(CodeInvalidParams, "invalid tool arguments")
	}
	session.server.mu.RLock()
	registration, exists := session.server.tools[params.Name]
	session.server.mu.RUnlock()
	if !exists {
		return nil, rpcError(CodeInvalidParams, "tool not found")
	}
	result, err := registration.handler(ctx, params, progress{session: session, token: params.Meta.ProgressToken})
	if err != nil {
		return nil, handlerRPCError(ctx, err)
	}
	if err := ctx.Err(); err != nil {
		return nil, rpcError(CodeRequestCancelled, "request cancelled")
	}
	return result, nil
}

func (session *Session) readResource(ctx context.Context, raw json.RawMessage) (any, *RPCError) {
	var params ReadResourceParams
	if err := DecodeParams(raw, &params); err != nil || params.URI == "" {
		return nil, rpcError(CodeInvalidParams, "invalid resource params")
	}
	session.server.mu.RLock()
	registration, exists := session.server.resources[params.URI]
	session.server.mu.RUnlock()
	if !exists {
		return nil, rpcError(CodeInvalidParams, "resource not found")
	}
	result, err := registration.handler(ctx, params, progress{session: session, token: params.Meta.ProgressToken})
	if err != nil {
		return nil, handlerRPCError(ctx, err)
	}
	if err := ctx.Err(); err != nil {
		return nil, rpcError(CodeRequestCancelled, "request cancelled")
	}
	return result, nil
}

func (session *Session) getPrompt(ctx context.Context, raw json.RawMessage) (any, *RPCError) {
	var params GetPromptParams
	if err := DecodeParams(raw, &params); err != nil || params.Name == "" {
		return nil, rpcError(CodeInvalidParams, "invalid prompt params")
	}
	session.server.mu.RLock()
	registration, exists := session.server.prompts[params.Name]
	session.server.mu.RUnlock()
	if !exists {
		return nil, rpcError(CodeInvalidParams, "prompt not found")
	}
	result, err := registration.handler(ctx, params, progress{session: session, token: params.Meta.ProgressToken})
	if err != nil {
		return nil, handlerRPCError(ctx, err)
	}
	if err := ctx.Err(); err != nil {
		return nil, rpcError(CodeRequestCancelled, "request cancelled")
	}
	return result, nil
}

func (session *Session) listTools(raw json.RawMessage) (any, *RPCError) {
	params, offset, callError := session.listOffset(raw)
	if callError != nil {
		return nil, callError
	}
	_ = params
	session.server.mu.RLock()
	items := make([]Tool, 0, len(session.server.tools))
	for _, registration := range session.server.tools {
		items = append(items, registration.schema)
	}
	session.server.mu.RUnlock()
	sort.Slice(items, func(left, right int) bool { return items[left].Name < items[right].Name })
	end, next, err := page(offset, len(items), session.server.config.Limits.MaxPageItems)
	if err != nil {
		return nil, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return ListToolsResult{Tools: items[offset:end], NextCursor: next}, nil
}

func (session *Session) listResources(raw json.RawMessage) (any, *RPCError) {
	_, offset, callError := session.listOffset(raw)
	if callError != nil {
		return nil, callError
	}
	session.server.mu.RLock()
	items := make([]Resource, 0, len(session.server.resources))
	for _, registration := range session.server.resources {
		items = append(items, registration.schema)
	}
	session.server.mu.RUnlock()
	sort.Slice(items, func(left, right int) bool { return items[left].URI < items[right].URI })
	end, next, err := page(offset, len(items), session.server.config.Limits.MaxPageItems)
	if err != nil {
		return nil, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return ListResourcesResult{Resources: items[offset:end], NextCursor: next}, nil
}

func (session *Session) listResourceTemplates(raw json.RawMessage) (any, *RPCError) {
	_, offset, callError := session.listOffset(raw)
	if callError != nil {
		return nil, callError
	}
	session.server.mu.RLock()
	items := make([]ResourceTemplate, 0, len(session.server.templates))
	for _, schema := range session.server.templates {
		items = append(items, schema)
	}
	session.server.mu.RUnlock()
	sort.Slice(items, func(left, right int) bool { return items[left].URITemplate < items[right].URITemplate })
	end, next, err := page(offset, len(items), session.server.config.Limits.MaxPageItems)
	if err != nil {
		return nil, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return ListResourceTemplatesResult{ResourceTemplates: items[offset:end], NextCursor: next}, nil
}

func (session *Session) listPrompts(raw json.RawMessage) (any, *RPCError) {
	_, offset, callError := session.listOffset(raw)
	if callError != nil {
		return nil, callError
	}
	session.server.mu.RLock()
	items := make([]Prompt, 0, len(session.server.prompts))
	for _, registration := range session.server.prompts {
		items = append(items, registration.schema)
	}
	session.server.mu.RUnlock()
	sort.Slice(items, func(left, right int) bool { return items[left].Name < items[right].Name })
	end, next, err := page(offset, len(items), session.server.config.Limits.MaxPageItems)
	if err != nil {
		return nil, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return ListPromptsResult{Prompts: items[offset:end], NextCursor: next}, nil
}

func (session *Session) listOffset(raw json.RawMessage) (ListParams, int, *RPCError) {
	var params ListParams
	if err := DecodeParams(raw, &params); err != nil {
		return params, 0, rpcError(CodeInvalidParams, "invalid list params")
	}
	if params.Cursor == "" {
		return params, 0, nil
	}
	offset, err := strconv.Atoi(params.Cursor)
	if err != nil || offset < 0 {
		return params, 0, rpcError(CodeInvalidParams, "invalid cursor")
	}
	return params, offset, nil
}

func page(offset, length, size int) (int, string, error) {
	if offset > length {
		return 0, "", errors.New("mcp: cursor exceeds collection")
	}
	end := offset + size
	if end >= length {
		return length, "", nil
	}
	return end, strconv.Itoa(end), nil
}

func (session *Session) decodeRequest(payload []byte) (Request, json.RawMessage, *RPCError) {
	if len(payload) > session.server.config.Limits.MaxPayloadBytes {
		return Request{}, nil, rpcError(CodePayloadTooLarge, "payload too large")
	}
	if err := validateNesting(payload, session.server.config.Limits.MaxNesting); err != nil {
		if errors.Is(err, ErrNestingTooDeep) {
			return Request{}, nil, rpcError(CodeInvalidRequest, "nesting too deep")
		}
		return Request{}, nil, rpcError(CodeParseError, "parse error")
	}
	decoder := json.NewDecoder(bytes.NewReader(payload))
	decoder.DisallowUnknownFields()
	decoder.UseNumber()
	var wire struct {
		JSONRPC string          `json:"jsonrpc"`
		ID      json.RawMessage `json:"id"`
		Method  string          `json:"method"`
		Params  json.RawMessage `json:"params,omitempty"`
	}
	if err := decoder.Decode(&wire); err != nil {
		var syntax *json.SyntaxError
		if errors.As(err, &syntax) {
			return Request{}, nil, rpcError(CodeParseError, "parse error")
		}
		return Request{}, nil, rpcError(CodeInvalidRequest, "invalid request")
	}
	var extra any
	if err := decoder.Decode(&extra); err == nil {
		return Request{}, nil, rpcError(CodeInvalidRequest, "trailing data")
	} else if !strings.Contains(err.Error(), "EOF") {
		return Request{}, nil, rpcError(CodeParseError, "parse error")
	}
	request := Request{JSONRPC: wire.JSONRPC, Method: wire.Method, Params: cloneRaw(wire.Params)}
	idRaw := cloneRaw(wire.ID)
	if len(wire.ID) > 0 {
		var id ID
		if err := json.Unmarshal(wire.ID, &id); err != nil || !id.Valid() {
			return Request{}, nil, rpcError(CodeInvalidRequest, "invalid request id")
		}
		request.ID = &id
	}
	if request.JSONRPC != JSONRPCVersion || request.Method == "" {
		return Request{}, idRaw, rpcError(CodeInvalidRequest, "invalid request")
	}
	return request, idRaw, nil
}

func (session *Session) encodeResult(id json.RawMessage, result any, callError *RPCError) ([]byte, error) {
	if callError != nil {
		return session.encodeError(id, callError)
	}
	encodedResult, err := json.Marshal(result)
	if err != nil {
		return session.encodeError(id, rpcError(CodeInternalError, "internal error"))
	}
	response := Response{JSONRPC: JSONRPCVersion, ID: id, Result: encodedResult}
	encoded, err := json.Marshal(response)
	if err != nil {
		return nil, err
	}
	if len(encoded) > session.server.config.Limits.MaxPayloadBytes || validateNesting(encoded, session.server.config.Limits.MaxNesting) != nil {
		return session.encodeError(id, rpcError(CodePayloadTooLarge, "response too large"))
	}
	return encoded, nil
}

func (session *Session) encodeError(id json.RawMessage, protocolError *RPCError) ([]byte, error) {
	if id == nil {
		id = json.RawMessage(`null`)
	}
	encoded, err := json.Marshal(Response{JSONRPC: JSONRPCVersion, ID: id, Error: protocolError})
	if err != nil {
		encoded, err = json.Marshal(Response{JSONRPC: JSONRPCVersion, ID: id, Error: rpcError(CodeInternalError, "internal error")})
		if err != nil {
			return nil, err
		}
	}
	limits := session.server.config.Limits
	if len(encoded) <= limits.MaxPayloadBytes && validateNesting(encoded, limits.MaxNesting) == nil {
		return encoded, nil
	}
	encoded, err = json.Marshal(Response{JSONRPC: JSONRPCVersion, ID: id, Error: rpcError(CodePayloadTooLarge, "response too large")})
	if err != nil {
		return nil, err
	}
	if len(encoded) > limits.MaxPayloadBytes || validateNesting(encoded, limits.MaxNesting) != nil {
		return nil, ErrPayloadTooLarge
	}
	return encoded, nil
}

func (session *Session) commitExchange(key string, request, response []byte) error {
	return session.server.log.Commit(context.Background(),
		EventInput{Kind: "request.received", Peer: session.peer, Generation: session.generation, RequestID: key, Payload: cloneRaw(request)},
		EventInput{Kind: "response.sent", Peer: session.peer, Generation: session.generation, RequestID: key, Payload: cloneRaw(response)},
	)
}

func rpcError(code int, message string) *RPCError { return &RPCError{Code: code, Message: message} }

func handlerRPCError(ctx context.Context, err error) *RPCError {
	if ctx.Err() != nil || errors.Is(err, context.Canceled) || errors.Is(err, context.DeadlineExceeded) {
		return rpcError(CodeRequestCancelled, "request cancelled")
	}
	var owned *HandlerError
	if errors.As(err, &owned) && owned.Code <= -32000 && owned.Code >= -32099 && owned.Message != "" {
		return &RPCError{Code: owned.Code, Message: owned.Message, Data: cloneRaw(owned.Data)}
	}
	return rpcError(CodeInternalError, "internal error")
}

func validateNesting(payload []byte, maximum int) error {
	depth := 0
	inString := false
	escaped := false
	for _, value := range payload {
		if inString {
			if escaped {
				escaped = false
			} else if value == '\\' {
				escaped = true
			} else if value == '"' {
				inString = false
			}
			continue
		}
		switch value {
		case '"':
			inString = true
		case '{', '[':
			depth++
			if depth > maximum {
				return ErrNestingTooDeep
			}
		case '}', ']':
			depth--
			if depth < 0 {
				return errors.New("mcp: invalid nesting")
			}
		}
	}
	if depth != 0 || inString {
		return errors.New("mcp: invalid nesting")
	}
	return nil
}

type progress struct {
	session *Session
	token   json.RawMessage
}

func (reporter progress) Report(ctx context.Context, value float64, total *float64, message string) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	if len(reporter.token) == 0 {
		return nil
	}
	params := ProgressParams{ProgressToken: cloneRaw(reporter.token), Progress: value, Total: total, Message: message}
	payload, err := json.Marshal(struct {
		JSONRPC string         `json:"jsonrpc"`
		Method  string         `json:"method"`
		Params  ProgressParams `json:"params"`
	}{JSONRPC: JSONRPCVersion, Method: "notifications/progress", Params: params})
	if err != nil {
		return err
	}
	if len(payload) > reporter.session.server.config.Limits.MaxPayloadBytes {
		return ErrPayloadTooLarge
	}
	if err := reporter.session.server.log.Commit(context.Background(), EventInput{Kind: "notification.sent", Peer: reporter.session.peer, Generation: reporter.session.generation, Payload: cloneRaw(payload)}); err != nil {
		return err
	}
	if reporter.session.sink == nil {
		return nil
	}
	if err := reporter.session.sink(ctx, payload); err != nil {
		reporter.session.drop(ErrPeerDropped)
		return ErrPeerDropped
	}
	return nil
}

// #endregion 🚦️Routing

// #region 🗄️Repository

const DescriptionsSchema = "semio.repo.mcp.descriptions/1"

// 🗣️DescriptionsEnvironment overrides the module-relative lookup of the shared description table.
const DescriptionsEnvironment = "SEMIO_REPO_MCP_DESCRIPTIONS"

// 🗣️DescriptionTable is the per-profile, per-key description table both implementations read.
// It is authored once at `🔌️mcp/🧬️schema/🔣️descriptions.json` and never duplicated in source.
type DescriptionTable struct {
	Schema       string                       `json:"schema"`
	Kinds        []string                     `json:"kinds"`
	Descriptions map[string]map[string]string `json:"descriptions"`
}

var (
	descriptionsOnce  sync.Once
	descriptionsTable DescriptionTable
	descriptionsError error
)

// 🧭️descriptionsPath resolves the schema file relative to this source file, because `go:embed` cannot
// reach a parent directory and the table is owned by the module, not by the Go package.
func descriptionsPath() (string, error) {
	if override := strings.TrimSpace(os.Getenv(DescriptionsEnvironment)); override != "" {
		return override, nil
	}
	_, file, _, ok := runtime.Caller(0)
	if ok {
		candidate := filepath.Join(filepath.Dir(file), "..", "..", "🧬️schema", "🔣️descriptions.json")
		if _, err := os.Stat(candidate); err == nil {
			return filepath.Clean(candidate), nil
		}
	}
	suffix := filepath.Join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔌️mcp", "🧬️schema", "🔣️descriptions.json")
	directory, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for {
		candidate := filepath.Join(directory, suffix)
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			return "", errors.New("mcp: description table not found")
		}
		directory = parent
	}
}

// 🗣️Descriptions loads the shared description table once per process.
func Descriptions() (DescriptionTable, error) {
	descriptionsOnce.Do(func() {
		path, err := descriptionsPath()
		if err != nil {
			descriptionsError = err
			return
		}
		data, err := os.ReadFile(path)
		if err != nil {
			descriptionsError = err
			return
		}
		if err := decodeExact(data, &descriptionsTable); err != nil {
			descriptionsError = err
			return
		}
		if descriptionsTable.Schema != DescriptionsSchema {
			descriptionsError = errors.New("mcp: unexpected description schema")
		}
	})
	return descriptionsTable, descriptionsError
}

// 🗣️Describe picks the description of one key for one profile, falling back to the generic wording.
func Describe(table DescriptionTable, profile providers.McpClientKind, key string) string {
	kind := string(profile)
	if kind == "" {
		kind = string(providers.McpClientGeneric)
	}
	entry, exists := table.Descriptions[key]
	if !exists {
		return ""
	}
	if text := entry[kind]; text != "" {
		return text
	}
	return entry[string(providers.McpClientGeneric)]
}

type RepositoryResult struct {
	Text       string
	Structured json.RawMessage
	IsError    bool
}

type RepositoryHandlers interface {
	Call(context.Context, string, json.RawMessage) (RepositoryResult, error)
	Read(context.Context, string) (ResourceContent, error)
	Prompt(context.Context, string, map[string]string) (GetPromptResult, error)
}

type ClientRepository struct {
	profile providers.McpClientKind
}

func NewClientRepository(profile providers.McpClientKind) ClientRepository {
	if profile == "" {
		profile = providers.McpClientGeneric
	}
	return ClientRepository{profile: profile}
}

func (repository ClientRepository) Call(ctx context.Context, name string, raw json.RawMessage) (RepositoryResult, error) {
	if err := ctx.Err(); err != nil {
		return RepositoryResult{}, err
	}
	var result workspace.ToolResult
	switch name {
	case "ticket_open":
		var params struct {
			Emoji        string `json:"emoji"`
			Title        string `json:"title"`
			Prompt       string `json:"prompt"`
			Goal         string `json:"goal"`
			Client       string `json:"client"`
			LLM          string `json:"llm"`
			Effort       string `json:"effort"`
			Draft        string `json:"draft"`
			Parent       string `json:"parent"`
			Issue        string `json:"issue"`
			PlanID       string `json:"plan_id"`
			SpecID       string `json:"spec_id"`
			NoIssue      bool   `json:"no_issue"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.Emoji == "" || params.Title == "" || params.Prompt == "" || params.Goal == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_open arguments"}
		}
		result = cli.ToolTicketOpen(params.Emoji, params.Title, params.Prompt, params.LLM, params.Effort, params.Client, params.Draft, params.NoIssue, params.Goal, params.Parent, params.NoManagement, params.Issue, repository.profile, params.PlanID, params.SpecID)
	case "ticket_close":
		var params struct {
			Path         string   `json:"path"`
			Summary      string   `json:"summary"`
			Files        []string `json:"files"`
			Title        string   `json:"title"`
			NoManagement bool     `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.Summary == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_close arguments"}
		}
		year, month, day, slug, err := resolveTicketPath(params.Path)
		if err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket path"}
		}
		result = cli.ToolTicketClose(year, month, day, slug, params.Summary, params.Files, params.Title, params.NoManagement)
	case "ticket_reopen":
		var params struct {
			Path         string `json:"path"`
			Prompt       string `json:"prompt"`
			LLM          string `json:"llm"`
			Effort       string `json:"effort"`
			Client       string `json:"client"`
			Draft        string `json:"draft"`
			Title        string `json:"title"`
			Goal         string `json:"goal"`
			Parent       string `json:"parent"`
			PlanID       string `json:"plan_id"`
			SpecID       string `json:"spec_id"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket_reopen arguments"}
		}
		year, month, day, slug, err := resolveTicketPath(params.Path)
		if err != nil {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid ticket path"}
		}
		result = cli.ToolTicketReopen(year, month, day, slug, params.Prompt, params.LLM, params.Effort, params.Client, params.Draft, params.Title, params.Goal, params.Parent, params.NoManagement, repository.profile, params.PlanID, params.SpecID)
	case "section_move":
		var params struct {
			File    string `json:"file"`
			OldName string `json:"old_name"`
			NewName string `json:"new_name"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.File == "" || params.OldName == "" || params.NewName == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid section_move arguments"}
		}
		result = move.ToolSectionMove(params.File, params.OldName, params.NewName)
	case "file_integrate":
		var params struct {
			Source              string `json:"source"`
			TargetSection       string `json:"target_section"`
			TargetFile          string `json:"target_file"`
			TargetParentSection string `json:"target_parent_section"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.Source == "" || params.TargetSection == "" || params.TargetFile == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid file_integrate arguments"}
		}
		result = move.ToolIntegrate(params.Source, params.TargetSection, params.TargetFile, params.TargetParentSection)
	case "section_extract":
		var params struct {
			SourceFile    string `json:"source_file"`
			SourceSection string `json:"source_section"`
			TargetFile    string `json:"target_file"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.SourceFile == "" || params.SourceSection == "" || params.TargetFile == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid section_extract arguments"}
		}
		result = move.ToolExtract(params.SourceFile, params.SourceSection, params.TargetFile)
	case "goal_open":
		var params struct {
			Title        string `json:"title"`
			Description  string `json:"description"`
			Prompt       string `json:"prompt"`
			DueDate      string `json:"due_date"`
			LLM          string `json:"llm"`
			Client       string `json:"client"`
			Parent       string `json:"parent"`
			Milestone    string `json:"milestone"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.Title == "" || params.Prompt == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid goal_open arguments"}
		}
		result = cli.ToolGoalCreate(params.Title, params.Description, params.Prompt, params.DueDate, params.LLM, params.Client, params.NoManagement, params.Parent, params.Milestone)
	case "goal_close":
		var params struct {
			ID           string `json:"id"`
			Summary      string `json:"summary"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.ID == "" || params.Summary == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid goal_close arguments"}
		}
		result = cli.ToolGoalClose(params.ID, params.Summary, params.NoManagement)
	case "goal_reopen":
		var params struct {
			ID           string `json:"id"`
			Prompt       string `json:"prompt"`
			LLM          string `json:"llm"`
			Client       string `json:"client"`
			Title        string `json:"title"`
			Description  string `json:"description"`
			DueDate      string `json:"due_date"`
			NoManagement bool   `json:"no_management"`
		}
		if err := DecodeArguments(raw, &params); err != nil || params.ID == "" || params.Prompt == "" || params.LLM == "" || params.Client == "" {
			return RepositoryResult{}, &HandlerError{Code: -32010, Message: "invalid goal_reopen arguments"}
		}
		result = cli.ToolGoalReopen(params.ID, params.Prompt, params.LLM, params.Client, params.Title, params.Description, params.DueDate, params.NoManagement)
	default:
		return RepositoryResult{}, &HandlerError{Code: -32010, Message: "tool not found"}
	}
	if err := ctx.Err(); err != nil {
		return RepositoryResult{}, err
	}
	return repositoryResult(result)
}

func (ClientRepository) Read(ctx context.Context, uri string) (ResourceContent, error) {
	if err := ctx.Err(); err != nil {
		return ResourceContent{}, err
	}
	var result workspace.ToolResult
	switch uri {
	case "repo://":
		result = cli.ToolCodebase()
	case "repo://bundles":
		result = cli.ToolBundleList()
	case "repo://folders":
		result = cli.ToolFolderList("")
	case "repo://files":
		result = cli.ToolFileList("")
	case "repo://tickets":
		result = cli.ToolTicketList(nil, nil, nil)
	case "repo://goals":
		result = cli.ToolGoalList()
	case "repo://policies":
		result = cli.ToolPolicyList()
	case "repo://contributors":
		result = cli.ToolContributorList()
	default:
		return ResourceContent{}, errors.New("mcp: resource not found")
	}
	converted, err := repositoryResult(result)
	if err != nil {
		return ResourceContent{}, err
	}
	if converted.IsError {
		return ResourceContent{}, errors.New("mcp: resource handler failed")
	}
	return ResourceContent{URI: uri, MIMEType: "text/plain", Text: converted.Text}, nil
}

func (ClientRepository) Prompt(ctx context.Context, name string, arguments map[string]string) (GetPromptResult, error) {
	if err := ctx.Err(); err != nil {
		return GetPromptResult{}, err
	}
	prompt := arguments["prompt"]
	var instruction string
	switch name {
	case "enhance":
		instruction = "Enhance the request while preserving its intent and constraints."
	case "refactor":
		instruction = "Refactor the requested scope completely and preserve observable behavior."
	case "test":
		instruction = "Test the requested scope with executable success and hostile cases."
	case "comply":
		instruction = "Apply the repository instructions and resolve every in-scope breach."
	default:
		return GetPromptResult{}, errors.New("mcp: prompt not found")
	}
	return GetPromptResult{Description: instruction, Messages: []PromptMessage{{Role: "user", Content: Content{Type: "text", Text: instruction + "\n\n" + prompt}}}}, nil
}

func repositoryResult(result workspace.ToolResult) (RepositoryResult, error) {
	structured, err := json.Marshal(result.Data)
	if err != nil {
		return RepositoryResult{}, errors.New("mcp: repository result encoding failed")
	}
	var text strings.Builder
	for index, line := range result.Output.Lines {
		if index > 0 {
			text.WriteByte('\n')
		}
		text.WriteString(line.Text)
	}
	if result.Error != "" {
		if text.Len() > 0 {
			text.WriteByte('\n')
		}
		text.WriteString(result.Error)
	}
	return RepositoryResult{Text: text.String(), Structured: structured, IsError: result.Error != "" || result.Output.ExitCode != 0}, nil
}

func resolveTicketPath(path string) (int, int, int, string, error) {
	if path == "" {
		ticket, err := tickets.LatestTicket()
		if err != nil || ticket == nil {
			return 0, 0, 0, "", errors.New("mcp: ticket not found")
		}
		return ticket.Year, ticket.Month, ticket.Day, ticket.Slug, nil
	}
	parts := strings.Split(strings.Trim(path, "/"), "/")
	if len(parts) != 4 || parts[3] == "" {
		return 0, 0, 0, "", errors.New("mcp: invalid ticket path")
	}
	year, yearErr := strconv.Atoi(parts[0])
	month, monthErr := strconv.Atoi(parts[1])
	day, dayErr := strconv.Atoi(parts[2])
	if yearErr != nil || monthErr != nil || dayErr != nil || year < 0 || year > 99 || month < 1 || month > 12 || day < 1 || day > 31 {
		return 0, 0, 0, "", errors.New("mcp: invalid ticket path")
	}
	return year, month, day, parts[3], nil
}

func NewRepositoryServer(repository RepositoryHandlers) (*Server, error) {
	return NewRepositoryServerFor(repository, providers.McpClientGeneric)
}

func NewRepositoryServerFor(repository RepositoryHandlers, profile providers.McpClientKind) (*Server, error) {
	return NewRepositoryServerWithLimitsFor(repository, profile, DefaultLimits())
}

func NewRepositoryServerWithLimits(repository RepositoryHandlers, limits Limits) (*Server, error) {
	return NewRepositoryServerWithLimitsFor(repository, providers.McpClientGeneric, limits)
}

func NewRepositoryServerWithLimitsFor(repository RepositoryHandlers, profile providers.McpClientKind, limits Limits) (*Server, error) {
	if repository == nil {
		return nil, errors.New("mcp: repository handlers are required")
	}
	resolvedProfile, err := providers.ParseMcpClientKind(string(profile))
	if err != nil {
		return nil, err
	}
	table, err := Descriptions()
	if err != nil {
		return nil, err
	}
	describe := func(key string) string { return Describe(table, resolvedProfile, key) }
	server, err := NewServer(Config{ServerInfo: Implementation{Name: providers.McpServerName(resolvedProfile), Version: "1.0.0"}, Instructions: "Use repository tools and resources through their owned schemas.", Limits: limits})
	if err != nil {
		return nil, err
	}
	object := func(properties map[string]Schema, required ...string) Schema {
		additional := false
		return Schema{Type: "object", Properties: properties, Required: required, AdditionalProperties: &additional}
	}
	stringField := func(description string) Schema { return Schema{Type: "string", Description: description} }
	booleanField := func(description string) Schema { return Schema{Type: "boolean", Description: description} }
	arrayField := func(description string) Schema {
		return Schema{Type: "array", Description: description, Items: &Schema{Type: "string"}}
	}
	openProperties := map[string]Schema{"emoji": stringField("Ticket emoji."), "title": stringField("Ticket title."), "prompt": stringField("Task description."), "goal": stringField("Goal id."), "client": stringField("Agent client."), "llm": stringField("Model."), "effort": stringField("Reasoning effort."), "draft": stringField("Draft id."), "parent": stringField("Parent ticket."), "issue": stringField("Existing issue URL."), "no_issue": booleanField("Skip issue creation."), "no_management": booleanField("Skip management integration.")}
	reopenProperties := map[string]Schema{"path": stringField("YY/MM/DD/SLUG path."), "prompt": stringField("Additional task description."), "client": stringField("Agent client."), "llm": stringField("Model."), "effort": stringField("Reasoning effort."), "draft": stringField("Draft id."), "title": stringField("Updated title."), "goal": stringField("Goal id."), "parent": stringField("Parent ticket."), "no_management": booleanField("Skip management integration.")}
	switch resolvedProfile {
	case providers.McpClientCursor, providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:
		openProperties["plan_id"] = stringField(describe("arg_plan_id"))
		reopenProperties["plan_id"] = stringField(describe("arg_plan_id"))
	case providers.McpClientKiro:
		openProperties["spec_id"] = stringField(describe("arg_spec_id"))
		reopenProperties["spec_id"] = stringField(describe("arg_spec_id"))
	}
	tools := []Tool{
		{Name: "ticket_open", Description: describe("tool_ticket_open"), InputSchema: object(openProperties, "emoji", "title", "prompt", "goal")},
		{Name: "ticket_close", Description: describe("tool_ticket_close"), InputSchema: object(map[string]Schema{"path": stringField("YY/MM/DD/SLUG path."), "summary": stringField("Completion summary."), "files": arrayField("Changed files."), "title": stringField("Updated title."), "no_management": booleanField("Skip management integration.")}, "summary")},
		{Name: "ticket_reopen", Description: describe("tool_ticket_reopen"), InputSchema: object(reopenProperties)},
		{Name: "section_move", Description: describe("tool_section_move"), InputSchema: object(map[string]Schema{"file": stringField("Source file."), "old_name": stringField("Current section."), "new_name": stringField("New section.")}, "file", "old_name", "new_name")},
		{Name: "file_integrate", Description: describe("tool_file_integrate"), InputSchema: object(map[string]Schema{"source": stringField("Source file."), "target_section": stringField("Target section."), "target_file": stringField("Target file."), "target_parent_section": stringField("Optional parent section.")}, "source", "target_section", "target_file")},
		{Name: "section_extract", Description: describe("tool_section_extract"), InputSchema: object(map[string]Schema{"source_file": stringField("Source file."), "source_section": stringField("Source section."), "target_file": stringField("Target file.")}, "source_file", "source_section", "target_file")},
		{Name: "goal_open", Description: describe("tool_goal_open"), InputSchema: object(map[string]Schema{"title": stringField("Goal title."), "description": stringField("Goal description."), "prompt": stringField("Goal prompt."), "due_date": stringField("YYYY-MM-DD due date."), "llm": stringField("Model."), "client": stringField("Agent client."), "parent": stringField("Parent goal id."), "milestone": stringField("Management milestone."), "no_management": booleanField("Skip management integration.")}, "title", "prompt")},
		{Name: "goal_close", Description: describe("tool_goal_close"), InputSchema: object(map[string]Schema{"id": stringField("Goal id."), "summary": stringField("Completion summary."), "no_management": booleanField("Skip management integration.")}, "id", "summary")},
		{Name: "goal_reopen", Description: describe("tool_goal_reopen"), InputSchema: object(map[string]Schema{"id": stringField("Goal id."), "prompt": stringField("Additional goal prompt."), "llm": stringField("Model."), "client": stringField("Agent client."), "title": stringField("Updated title."), "description": stringField("Updated description."), "due_date": stringField("YYYY-MM-DD due date."), "no_management": booleanField("Skip management integration.")}, "id", "prompt", "llm", "client")},
	}
	for _, schema := range tools {
		name := schema.Name
		if err := server.RegisterTool(schema, func(ctx context.Context, params CallToolParams, progress ProgressReporter) (CallToolResult, error) {
			if err := progress.Report(ctx, 0, nil, "started"); err != nil {
				return CallToolResult{}, err
			}
			result, err := repository.Call(ctx, name, params.Arguments)
			if err != nil {
				return CallToolResult{}, err
			}
			if err := progress.Report(ctx, 1, nil, "completed"); err != nil {
				return CallToolResult{}, err
			}
			return CallToolResult{Content: []Content{{Type: "text", Text: result.Text}}, StructuredContent: result.Structured, IsError: result.IsError}, nil
		}); err != nil {
			return nil, err
		}
	}
	resources := []Resource{
		{URI: "repo://", Name: "repo", Description: describe("res_root"), MIMEType: "text/plain"},
		{URI: "repo://bundles", Name: "bundles", Description: describe("res_bundles"), MIMEType: "text/plain"},
		{URI: "repo://folders", Name: "folders", Description: describe("res_folders"), MIMEType: "text/plain"},
		{URI: "repo://files", Name: "files", Description: describe("res_files"), MIMEType: "text/plain"},
		{URI: "repo://tickets", Name: "tickets", Description: describe("res_tickets"), MIMEType: "text/plain"},
		{URI: "repo://goals", Name: "goals", Description: describe("res_goals"), MIMEType: "text/plain"},
		{URI: "repo://policies", Name: "policies", Description: describe("res_policies"), MIMEType: "text/plain"},
		{URI: "repo://contributors", Name: "contributors", Description: describe("res_contributors"), MIMEType: "text/plain"},
	}
	for _, schema := range resources {
		uri := schema.URI
		if err := server.RegisterResource(schema, func(ctx context.Context, _ ReadResourceParams, progress ProgressReporter) (ReadResourceResult, error) {
			if err := progress.Report(ctx, 0, nil, "started"); err != nil {
				return ReadResourceResult{}, err
			}
			content, err := repository.Read(ctx, uri)
			if err != nil {
				return ReadResourceResult{}, err
			}
			return ReadResourceResult{Contents: []ResourceContent{content}}, nil
		}); err != nil {
			return nil, err
		}
	}
	for _, name := range []string{"enhance", "refactor", "test", "comply"} {
		promptName := name
		schema := Prompt{Name: promptName, Description: describe("prompt_" + promptName), Arguments: []PromptArgument{{Name: "prompt", Required: true}}}
		if err := server.RegisterPrompt(schema, func(ctx context.Context, params GetPromptParams, _ ProgressReporter) (GetPromptResult, error) {
			return repository.Prompt(ctx, promptName, params.Arguments)
		}); err != nil {
			return nil, err
		}
	}
	return server, nil
}

// 🚚️StdioTransport carries the line-delimited protocol over any reader/writer pair.
type StdioTransport struct {
	Reader io.Reader
	Writer io.Writer
}

func (transport StdioTransport) Read(data []byte) (int, error)  { return transport.Reader.Read(data) }
func (transport StdioTransport) Write(data []byte) (int, error) { return transport.Writer.Write(data) }
func (StdioTransport) Close() error                             { return nil }

// #endregion 🗄️Repository

// #region 🦀️Entrypoint

// 🪪️MCPProfileEnvironment names the environment variable that selects the client profile.
const MCPProfileEnvironment = "SEMIO_REPO_MCP_CLIENT"

// 🪪️ResolveMCPProfile reads the profile from the environment and refuses every command argument.
func ResolveMCPProfile(arguments []string, lookupEnvironment func(string) (string, bool)) (providers.McpClientKind, error) {
	if len(arguments) != 0 {
		return "", errors.New("repo MCP accepts no command arguments")
	}
	raw, _ := lookupEnvironment(MCPProfileEnvironment)
	return providers.ParseMcpClientKind(raw)
}

// 🚀️RunStdio serves the production repository of one profile over the process standard streams.
func RunStdio(ctx context.Context, arguments []string, lookupEnvironment func(string) (string, bool), reader io.Reader, writer io.Writer) error {
	profile, err := ResolveMCPProfile(arguments, lookupEnvironment)
	if err != nil {
		return err
	}
	_, err = RunMCPForProfile(ctx, StdioTransport{Reader: reader, Writer: writer}, NewClientRepository(profile), profile)
	return err
}

// ▶️RunMCP serves one repository under the generic profile.
func RunMCP(ctx context.Context, transport Transport, repository RepositoryHandlers) (*Server, error) {
	return RunMCPForProfile(ctx, transport, repository, providers.McpClientGeneric)
}

// ▶️RunMCPForProfile builds the repository server of one profile and serves it until the peer drops.
func RunMCPForProfile(ctx context.Context, transport Transport, repository RepositoryHandlers, profile providers.McpClientKind) (*Server, error) {
	server, err := NewRepositoryServerFor(repository, profile)
	if err != nil {
		return nil, err
	}
	return ServeMCP(ctx, transport, server)
}

// ▶️ServeMCP serves one built server and reports a dropped peer as a clean end of conversation.
func ServeMCP(ctx context.Context, transport Transport, server *Server) (*Server, error) {
	err := server.Serve(ctx, "stdio", transport)
	if errors.Is(err, ErrPeerDropped) {
		err = nil
	}
	return server, err
}

// #endregion 🦀️Entrypoint

// #region 🧪️Projection

// 🧪️NewContractServer builds the fixed tool/resource/prompt surface the golden call vectors pin, so a
// language-agnostic adapter observes the same server the Rust twin's `contract_server` builds.
func NewContractServer(limits Limits) (*Server, error) {
	server, err := NewServer(Config{ServerInfo: Implementation{Name: "repo", Version: "1.0.0"}, Instructions: "owned", Limits: limits})
	if err != nil {
		return nil, err
	}
	if err := server.RegisterTool(Tool{Name: "echo", InputSchema: Schema{Type: "object"}}, func(ctx context.Context, params CallToolParams, progress ProgressReporter) (CallToolResult, error) {
		var arguments struct {
			Text string `json:"text"`
		}
		if err := DecodeParams(params.Arguments, &arguments); err != nil {
			return CallToolResult{}, &HandlerError{Code: -32010, Message: "invalid echo arguments"}
		}
		if err := progress.Report(ctx, 1, nil, "echo"); err != nil {
			return CallToolResult{}, err
		}
		return CallToolResult{Content: []Content{{Type: "text", Text: arguments.Text}}}, nil
	}); err != nil {
		return nil, err
	}
	if err := server.RegisterResource(Resource{URI: "repo://goals", Name: "goals", MIMEType: "application/json"}, func(context.Context, ReadResourceParams, ProgressReporter) (ReadResourceResult, error) {
		return ReadResourceResult{Contents: []ResourceContent{{URI: "repo://goals", MIMEType: "application/json", Text: "[]"}}}, nil
	}); err != nil {
		return nil, err
	}
	if err := server.RegisterResourceTemplate(ResourceTemplate{URITemplate: "repo://ticket/{id}", Name: "ticket"}); err != nil {
		return nil, err
	}
	if err := server.RegisterPrompt(Prompt{Name: "review", Arguments: []PromptArgument{{Name: "scope", Required: true}}}, func(_ context.Context, params GetPromptParams, _ ProgressReporter) (GetPromptResult, error) {
		scope := params.Arguments["scope"]
		return GetPromptResult{Description: "Review " + scope, Messages: []PromptMessage{{Role: "user", Content: Content{Type: "text", Text: scope}}}}, nil
	}); err != nil {
		return nil, err
	}
	return server, nil
}

// #endregion 🧪️Projection
