// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
)

// #region ⚙️Configuration

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

// #endregion ⚙️Configuration

// #region 🖥️Server

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

// #endregion 🖥️Server

// #region 🔐Session

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
	server       *Server
	peer         string
	generation   uint64
	sink         Sink
	mu           sync.Mutex
	phase        sessionPhase
	active       map[string]*activeRequest
	seen         map[string]struct{}
	seenOrder    []string
	preCancelled map[string]string
	wait         sync.WaitGroup
	done         chan struct{}
	shutdownOnce sync.Once
	closeErr     error
}

func newSession(server *Server, peer string, generation uint64, sink Sink) *Session {
	return &Session{server: server, peer: peer, generation: generation, sink: sink, active: map[string]*activeRequest{}, seen: map[string]struct{}{}, preCancelled: map[string]string{}, done: make(chan struct{})}
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

// #endregion 🔐Session

// #region 🚦Routing

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
		if err := DecodeParams(request.Params, &params); err != nil || params.ProtocolVersion != ProtocolVersion || params.ClientInfo.Name == "" || params.ClientInfo.Version == "" {
			return nil, rpcError(CodeInvalidParams, "invalid initialize params")
		}
		session.mu.Lock()
		if session.phase != phaseConnected {
			session.mu.Unlock()
			return nil, rpcError(CodeInvalidRequest, "already initialized")
		}
		session.phase = phaseInitialized
		session.mu.Unlock()
		return InitializeResult{ProtocolVersion: ProtocolVersion, Capabilities: session.server.capabilities(), ServerInfo: session.server.config.ServerInfo, Instructions: session.server.config.Instructions}, nil
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
			if session.phase == phaseInitialized {
				session.phase = phaseReady
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

// #endregion 🚦Routing

// #region 📦️Encoding

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

// #endregion 📦️Encoding
