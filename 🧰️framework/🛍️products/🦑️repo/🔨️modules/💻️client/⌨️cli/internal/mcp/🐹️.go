// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package mcp defines the owned MCP wire schema used by the CLI surface.

// #endregion 🧲️Header

package mcp

// #region 📜️ProtocolSchema

type TextContent struct {
	Type string `json:"type"`
	Text string `json:"text"`
}

type CallToolResult struct {
	Content []TextContent `json:"content"`
	IsError bool          `json:"isError,omitempty"`
}

type CallToolRequest struct {
	Params struct {
		Name      string      `json:"name"`
		Arguments interface{} `json:"arguments"`
	} `json:"params"`
}

type GetPromptRequest struct {
	Params struct {
		Name      string            `json:"name"`
		Arguments map[string]string `json:"arguments"`
	} `json:"params"`
}

type Role string

const RoleUser Role = "user"

type PromptMessage struct {
	Role    Role        `json:"role"`
	Content TextContent `json:"content"`
}

type GetPromptResult struct {
	Description string          `json:"description"`
	Messages    []PromptMessage `json:"messages"`
}

type ReadResourceRequest struct {
	Params struct {
		URI string `json:"uri"`
	} `json:"params"`
}

type ResourceContents interface{ resourceContents() }

type TextResourceContents struct {
	URI      string `json:"uri"`
	MIMEType string `json:"mimeType,omitempty"`
	Text     string `json:"text"`
}

func (TextResourceContents) resourceContents() {}

type InputSchema struct {
	Type       string                 `json:"type"`
	Properties map[string]interface{} `json:"properties"`
	Required   []string               `json:"required,omitempty"`
}

type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description,omitempty"`
	InputSchema InputSchema `json:"inputSchema"`
}

type Prompt struct {
	Name        string              `json:"name"`
	Description string              `json:"description,omitempty"`
	Arguments   map[string]Argument `json:"arguments,omitempty"`
}

type Argument struct {
	Description string `json:"description,omitempty"`
	Required    bool   `json:"required,omitempty"`
}

type Resource struct {
	URI         string `json:"uri"`
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	MIMEType    string `json:"mimeType,omitempty"`
}

type ResourceTemplate = Resource

// #endregion 📜️ProtocolSchema

// #region 🏭️Constructors

func NewTextContent(text string) TextContent { return TextContent{Type: "text", Text: text} }
func NewToolResultText(text string) *CallToolResult {
	return &CallToolResult{Content: []TextContent{NewTextContent(text)}}
}
func NewPromptMessage(role Role, content TextContent) PromptMessage {
	return PromptMessage{Role: role, Content: content}
}
func NewGetPromptResult(description string, messages []PromptMessage) *GetPromptResult {
	return &GetPromptResult{Description: description, Messages: messages}
}

type PromptOption func(*Prompt)
type ArgumentOption func(*Argument)
type ResourceOption func(*Resource)
type ToolOption func(*Tool)
type SchemaOption func(map[string]interface{})

type ArgumentDescription string

func RequiredArgument() ArgumentOption { return func(argument *Argument) { argument.Required = true } }

func WithArgument(name string, description ArgumentDescription, options ...ArgumentOption) PromptOption {
	return func(prompt *Prompt) {
		argument := Argument{Description: string(description)}
		for _, option := range options {
			option(&argument)
		}
		if prompt.Arguments == nil {
			prompt.Arguments = map[string]Argument{}
		}
		prompt.Arguments[name] = argument
	}
}

func WithPromptDescription(description string) PromptOption {
	return func(prompt *Prompt) { prompt.Description = description }
}

func NewPrompt(name string, options ...PromptOption) Prompt {
	prompt := Prompt{Name: name, Arguments: map[string]Argument{}}
	for _, option := range options {
		option(&prompt)
	}
	return prompt
}

func WithMIMEType(mimeType string) ResourceOption {
	return func(resource *Resource) { resource.MIMEType = mimeType }
}

func NewResource(uri, description string, options ...ResourceOption) Resource {
	resource := Resource{URI: uri, Name: uri, Description: description}
	for _, option := range options {
		option(&resource)
	}
	return resource
}

func NewResourceTemplate(uri, description string, options ...ResourceOption) ResourceTemplate {
	return NewResource(uri, description, options...)
}

func NewTool(name string, options ...ToolOption) Tool {
	tool := Tool{Name: name, InputSchema: InputSchema{Type: "object", Properties: map[string]interface{}{}}}
	for _, option := range options {
		option(&tool)
	}
	return tool
}

func Description(description string) SchemaOption {
	return func(schema map[string]interface{}) { schema["description"] = description }
}
func Required() SchemaOption {
	return func(schema map[string]interface{}) { schema["required"] = true }
}
func WithStringItems() SchemaOption {
	return func(schema map[string]interface{}) { schema["items"] = map[string]interface{}{"type": "string"} }
}
func WithDescription(description string) ToolOption {
	return func(tool *Tool) { tool.Description = description }
}

func WithString(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "string", options...)
}
func WithBoolean(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "boolean", options...)
}
func WithArray(name string, options ...SchemaOption) ToolOption {
	return withProperty(name, "array", options...)
}

func withProperty(name, kind string, options ...SchemaOption) ToolOption {
	return func(tool *Tool) {
		schema := map[string]interface{}{"type": kind}
		for _, option := range options {
			option(schema)
		}
		if required, _ := schema["required"].(bool); required {
			delete(schema, "required")
			tool.InputSchema.Required = append(tool.InputSchema.Required, name)
		}
		tool.InputSchema.Properties[name] = schema
	}
}

// #endregion 🏭️Constructors
