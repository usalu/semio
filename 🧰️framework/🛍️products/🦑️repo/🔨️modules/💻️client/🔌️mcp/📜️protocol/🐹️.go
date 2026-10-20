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

// 🤝️DecodeOpenParams decodes one params object WITHOUT rejecting unknown fields. It exists for the
// handshake alone: MCP declares `initialize` forward-extensible, so a conforming client may send
// `clientInfo.title`, `capabilities.tasks`, `capabilities.experimental` or any later addition, and
// rejecting those with -32602 closes the session before `tools/list` for every modern client.
// Every other frame stays on DecodeParams, which rejects unknown fields except the open `_meta` map.
func DecodeOpenParams(raw json.RawMessage, target any) error {
	if len(raw) == 0 {
		raw = []byte("{}")
	}
	trimmed := bytes.TrimSpace(raw)
	if len(trimmed) == 0 || trimmed[0] != '{' {
		return errors.New("mcp: params must be an object")
	}
	decoder := json.NewDecoder(bytes.NewReader(raw))
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
	Title   string `json:"title,omitempty"`
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
	Tasks       *TasksCapability    `json:"tasks,omitempty"`
}

// ⏳️TasksCapability declares the experimental task utility: `tasks/list`, `tasks/cancel` and task-augmented `tools/call`.
type TasksCapability struct {
	List     *EmptyCapability       `json:"list,omitempty"`
	Cancel   *EmptyCapability       `json:"cancel,omitempty"`
	Requests *TaskRequestCapability `json:"requests,omitempty"`
}

type TaskRequestCapability struct {
	Tools *TaskToolsCapability `json:"tools,omitempty"`
}

type TaskToolsCapability struct {
	Call *EmptyCapability `json:"call,omitempty"`
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
	Cursor string   `json:"cursor,omitempty"`
	Meta   ListMeta `json:"_meta,omitempty"`
}

// 🏷️ListMeta is the open MCP `_meta` map. `progressToken` is the only key this server reads; clients may send any other key.
type ListMeta map[string]json.RawMessage

// 🏷️ProgressToken returns the optional MCP progress token stored in `_meta`.
func (meta ListMeta) ProgressToken() json.RawMessage {
	if meta == nil {
		return nil
	}
	return meta["progressToken"]
}

type Schema struct {
	Type                 string            `json:"type,omitempty"`
	Description          string            `json:"description,omitempty"`
	Properties           map[string]Schema `json:"properties,omitempty"`
	Required             []string          `json:"required,omitempty"`
	Items                *Schema           `json:"items,omitempty"`
	AdditionalProperties *bool             `json:"additionalProperties,omitempty"`
	Enum                 []string          `json:"enum,omitempty"`
	MinLength            int               `json:"minLength,omitempty"`
	Pattern              string            `json:"pattern,omitempty"`
	Default              json.RawMessage   `json:"default,omitempty"`
	Minimum              *int64            `json:"minimum,omitempty"`
	Maximum              *int64            `json:"maximum,omitempty"`
	MinItems             int               `json:"minItems,omitempty"`
	MaxItems             int               `json:"maxItems,omitempty"`
	UniqueItems          bool              `json:"uniqueItems,omitempty"`
	AnyOf                []Schema          `json:"anyOf,omitempty"`
}

// 🏷️ToolAnnotations are the optional behaviour hints of a tool; an unset hint is omitted.
type ToolAnnotations struct {
	Title           string `json:"title,omitempty"`
	ReadOnlyHint    *bool  `json:"readOnlyHint,omitempty"`
	DestructiveHint *bool  `json:"destructiveHint,omitempty"`
	IdempotentHint  *bool  `json:"idempotentHint,omitempty"`
	OpenWorldHint   *bool  `json:"openWorldHint,omitempty"`
}

const (
	TaskSupportForbidden = "forbidden"
	TaskSupportOptional  = "optional"
	TaskSupportRequired  = "required"
)

// ⏳️ToolExecution declares whether a tool may, must or must not run as a task.
type ToolExecution struct {
	TaskSupport string `json:"taskSupport,omitempty"`
}

type Tool struct {
	Name         string           `json:"name"`
	Title        string           `json:"title,omitempty"`
	Description  string           `json:"description,omitempty"`
	InputSchema  Schema           `json:"inputSchema"`
	OutputSchema *Schema          `json:"outputSchema,omitempty"`
	Annotations  *ToolAnnotations `json:"annotations,omitempty"`
	Execution    *ToolExecution   `json:"execution,omitempty"`
}

// 📤️ToolResultOutputSchema is the one `outputSchema` every repository tool declares: object data is returned as is, array or scalar data is wrapped as `{"result": <value>}` (see `CallToolResult.MarshalJSON` and `$defs.ToolOutput` of the module schema).
func ToolResultOutputSchema() *Schema {
	additional := true
	return &Schema{Type: "object", Description: "Object data as is; array or scalar data wrapped as result.", Properties: map[string]Schema{"result": {Description: "Array or scalar data of the tool."}}, AdditionalProperties: &additional}
}

type ListToolsResult struct {
	Tools      []Tool `json:"tools"`
	NextCursor string `json:"nextCursor,omitempty"`
}

type CallToolParams struct {
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments,omitempty"`
	Meta      ListMeta        `json:"_meta,omitempty"`
	Task      *TaskRequest    `json:"task,omitempty"`
}

type Content struct {
	Type     string `json:"type"`
	Text     string `json:"text,omitempty"`
	Data     string `json:"data,omitempty"`
	MIMEType string `json:"mimeType,omitempty"`
	URI      string `json:"uri,omitempty"`
}

// 📝️MarshalJSON keeps `text` on every text block, including an empty one, because a text block without `text` is invalid.
func (content Content) MarshalJSON() ([]byte, error) {
	type wire struct {
		Type     string  `json:"type"`
		Text     *string `json:"text,omitempty"`
		Data     string  `json:"data,omitempty"`
		MIMEType string  `json:"mimeType,omitempty"`
		URI      string  `json:"uri,omitempty"`
	}
	encoded := wire{Type: content.Type, Data: content.Data, MIMEType: content.MIMEType, URI: content.URI}
	if content.Type == "text" || content.Text != "" {
		encoded.Text = &content.Text
	}
	return json.Marshal(encoded)
}

type CallToolResult struct {
	Content           []Content       `json:"content"`
	StructuredContent json.RawMessage `json:"structuredContent,omitempty"`
	IsError           bool            `json:"isError,omitempty"`
	Meta              map[string]any  `json:"_meta,omitempty"`
	OutputSchemaDeclared bool         `json:"-"`
}

// 🧯️MarshalJSON keeps `content` an array and emits `structuredContent` only as a JSON object: null is omitted, object data is kept, array or scalar data is wrapped as `{"result": <value>}` when the tool declares an `outputSchema` and omitted otherwise.
func (result CallToolResult) MarshalJSON() ([]byte, error) {
	type wire CallToolResult
	encoded := wire(result)
	if encoded.Content == nil { encoded.Content = []Content{} }
	trimmed := bytes.TrimSpace(encoded.StructuredContent)
	switch {
	case len(trimmed) == 0 || bytes.Equal(trimmed, []byte("null")): encoded.StructuredContent = nil
	case trimmed[0] == '{': encoded.StructuredContent = trimmed
	case result.OutputSchemaDeclared: encoded.StructuredContent = append(append([]byte(`{"result":`), trimmed...), '}')
	default: encoded.StructuredContent = nil
	}
	return json.Marshal(encoded)
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

// 📤️MarshalJSON emits exactly one resource payload, preserving empty text resources.
func (content ResourceContent) MarshalJSON() ([]byte, error) {
	result := struct {
		URI string `json:"uri"`
		MIMEType string `json:"mimeType,omitempty"`
		Text *string `json:"text,omitempty"`
		Blob *string `json:"blob,omitempty"`
	}{URI: content.URI, MIMEType: content.MIMEType}
	if content.Blob != "" { result.Blob = &content.Blob } else { result.Text = &content.Text }
	return json.Marshal(result)
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

// ⏳️Task states of the experimental task utility.
const (
	TaskWorking   = "working"
	TaskCompleted = "completed"
	TaskFailed    = "failed"
	TaskCancelled = "cancelled"
)

// ⏳️TaskRequest asks the server to run a request as a task that outlives the request; `ttl` is the retention in milliseconds.
type TaskRequest struct {
	TTL *int64 `json:"ttl,omitempty"`
}

// ⏳️Task is the state of one task.
type Task struct {
	TaskID        string `json:"taskId"`
	Status        string `json:"status"`
	StatusMessage string `json:"statusMessage,omitempty"`
	CreatedAt     string `json:"createdAt"`
	LastUpdatedAt string `json:"lastUpdatedAt"`
	TTL           *int64 `json:"ttl"`
	PollInterval  int64  `json:"pollInterval,omitempty"`
}

// ⏳️CreateTaskResult answers a task-augmented request at once.
type CreateTaskResult struct {
	Task Task `json:"task"`
}

// ⏳️TaskIDParams are the params of `tasks/get`, `tasks/result` and `tasks/cancel`.
type TaskIDParams struct {
	TaskID string   `json:"taskId"`
	Meta   ListMeta `json:"_meta,omitempty"`
}

// ⏳️ListTasksResult is one page of `tasks/list`.
type ListTasksResult struct {
	Tasks      []Task `json:"tasks"`
	NextCursor string `json:"nextCursor,omitempty"`
}

const RelatedTaskMeta = "io.modelcontextprotocol/related-task"

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
