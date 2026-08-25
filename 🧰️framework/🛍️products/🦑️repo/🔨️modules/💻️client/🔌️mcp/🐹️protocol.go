// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package mcp owns the bounded JSON-RPC and Model Context Protocol contract.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"strconv"
)

// #region 📜️JSONRPC

const (
	JSONRPCVersion  = "2.0"
	ProtocolVersion = "2025-06-18"
)

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

func decodeExact(data []byte, target any) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
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

func DecodeParams(raw json.RawMessage, target any) error {
	if len(raw) == 0 {
		raw = []byte("{}")
	}
	trimmed := bytes.TrimSpace(raw)
	if len(trimmed) == 0 || trimmed[0] != '{' {
		return errors.New("mcp: params must be an object")
	}
	return decodeExact(raw, target)
}

// #endregion 📜️JSONRPC

// #region 🧬️MCP

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

// #endregion 🧬️MCP
