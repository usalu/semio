// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖥️mcp_adapter carries the MCP server adapter of the cli: server registry, listings and the stdio JSON-RPC loop.

// #endregion 🧲️Header

package cli

import (
	bufio "bufio"
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	sort "sort"
 strings "strings"
)

// #region 🔌️Mcp Surface

// #region 🖥️Mcp Server Adapter

type ServerOption func(*MCPServer)

type ToolHandler func(context.Context, CallToolRequest) (*CallToolResult, error)

type PromptHandler func(context.Context, GetPromptRequest) (*GetPromptResult, error)

type ResourceHandler func(context.Context, ReadResourceRequest) ([]ResourceContents, error)

type ServerTool struct {
	Tool    Tool
	Handler ToolHandler
}

type MCPServer struct {
	Name          string
	Version       string
	tools         map[string]ServerTool
	prompts       map[string]PromptHandler
	promptCards   map[string]Prompt
	resources     map[string]ResourceHandler
	resourceCards map[string]Resource
	templateCards map[string]Resource
}

func WithToolCapabilities(_ bool) ServerOption { return func(*MCPServer) {} }

func WithPromptCapabilities(_ bool) ServerOption { return func(*MCPServer) {} }

func NewMCPServer(name, version string, options ...ServerOption) *MCPServer {
	server := &MCPServer{Name: name, Version: version, tools: map[string]ServerTool{}, prompts: map[string]PromptHandler{}, promptCards: map[string]Prompt{}, resources: map[string]ResourceHandler{}, resourceCards: map[string]Resource{}, templateCards: map[string]Resource{}}
	for _, option := range options {
		option(server)
	}
	return server
}

func (server *MCPServer) AddTool(tool Tool, handler ToolHandler) {
	server.tools[tool.Name] = ServerTool{Tool: tool, Handler: handler}
}

func (server *MCPServer) AddPrompt(prompt Prompt, handler PromptHandler) {
	server.prompts[prompt.Name] = handler
	server.promptCards[prompt.Name] = prompt
}

func (server *MCPServer) AddResource(resource Resource, handler ResourceHandler) {
	server.resources[resource.URI] = handler
	server.resourceCards[resource.URI] = resource
}

func (server *MCPServer) AddResourceTemplate(resource ResourceTemplate, handler ResourceHandler) {
	server.resources[resource.URI] = handler
	server.templateCards[resource.URI] = resource
}

// 🧩️matchResourceTemplate requires one nonempty identity between the authored URI parts.
func matchResourceTemplate(template,uri string) bool {
 prefix,rest,found:=strings.Cut(template,"{");if !found { return false };name,suffix,closed:=strings.Cut(rest,"}");if !closed || name=="" || strings.ContainsAny(name,"{}") || strings.ContainsAny(suffix,"{}") { return false };return strings.HasPrefix(uri,prefix)&&strings.HasSuffix(uri,suffix)&&len(uri)>len(prefix)+len(suffix)
}

// 🧭️resourceHandler resolves a concrete URI before registered templates.
func (server *MCPServer) resourceHandler(uri string) (ResourceHandler,bool) {
 if handler,found:=server.resources[uri];found { return handler,true }
 for _,card:=range server.ListResourceTemplates() { if matchResourceTemplate(card.URI,uri) { return server.resources[card.URI],true } };return nil,false
}

// 📃️ListResources returns every declared concrete resource, ordered by URI so two runs of the same
// server declare the same vocabulary in the same order. A templated URI is not a concrete resource
// and is answered by [MCPServer.ListResourceTemplates] instead.
func (server *MCPServer) ListResources() []Resource {
	return sortedCards(server.resourceCards)
}

// 📃️ListResourceTemplates returns every declared resource template, ordered by URI.
func (server *MCPServer) ListResourceTemplates() []Resource {
	return sortedCards(server.templateCards)
}

// 🔤️sortedCards returns the values of a URI-keyed card table in URI order.
func sortedCards(cards map[string]Resource) []Resource {
	uris := make([]string, 0, len(cards))
	for uri := range cards {
		uris = append(uris, uri)
	}
	sort.Strings(uris)
	ordered := make([]Resource, 0, len(uris))
	for _, uri := range uris {
		ordered = append(ordered, cards[uri])
	}
	return ordered
}

// 📃️ListPrompts returns every declared prompt, ordered by name.
func (server *MCPServer) ListPrompts() []Prompt {
	names := make([]string, 0, len(server.promptCards))
	for name := range server.promptCards {
		names = append(names, name)
	}
	sort.Strings(names)
	prompts := make([]Prompt, 0, len(names))
	for _, name := range names {
		prompts = append(prompts, server.promptCards[name])
	}
	return prompts
}

// 🧰️ListToolCards returns every declared tool, ordered by name.
func (server *MCPServer) ListToolCards() []Tool {
	names := make([]string, 0, len(server.tools))
	for name := range server.tools {
		names = append(names, name)
	}
	sort.Strings(names)
	tools := make([]Tool, 0, len(names))
	for _, name := range names {
		tools = append(tools, server.tools[name].Tool)
	}
	return tools
}

func (server *MCPServer) ListTools() map[string]ServerTool {
	result := make(map[string]ServerTool, len(server.tools))
	for name, tool := range server.tools {
		result[name] = tool
	}
	return result
}

type request struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

type response struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id,omitempty"`
	Result  interface{}     `json:"result,omitempty"`
	Error   *rpcError       `json:"error,omitempty"`
}

type rpcError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// 📡️ServeStdio serves the session on the process standard input and output.
func ServeStdio(ctx context.Context, server *MCPServer) error {
	return Serve(ctx,server,os.Stdin,os.Stdout)
}

// 📡️Serve reads one JSON-RPC message per line and writes one response per request. A notification
// — a message carrying no identifier — is answered with nothing, as the protocol requires.
func Serve(ctx context.Context, server *MCPServer, in io.ReadCloser, out io.Writer) error {
 stop:=context.AfterFunc(ctx,func(){ _=in.Close() });defer stop()
	scanner := bufio.NewScanner(in)
	scanner.Buffer(make([]byte, 64*1024), 4*1024*1024)
	encoder := json.NewEncoder(out)
	for scanner.Scan() {
 if err:=ctx.Err();err!=nil { return err }
		var incoming request
		if err := json.Unmarshal(scanner.Bytes(), &incoming); err != nil {
			if err := encoder.Encode(response{JSONRPC: "2.0", Error: &rpcError{Code: -32700, Message: "parse error"}}); err != nil {
				return err
			}
			continue
		}
		if len(incoming.ID) == 0 {
			continue
		}
		outgoing := response{JSONRPC: "2.0", ID: incoming.ID}
		switch incoming.Method {
		case "initialize":
			outgoing.Result = map[string]interface{}{"protocolVersion": "2025-06-18", "serverInfo": map[string]string{"name": server.Name, "version": server.Version}, "capabilities": map[string]interface{}{"tools": map[string]interface{}{}, "prompts": map[string]interface{}{}, "resources": map[string]interface{}{}}}
		case "tools/list":
			outgoing.Result = map[string]interface{}{"tools": server.ListToolCards()}
		case "prompts/list":
			outgoing.Result = map[string]interface{}{"prompts": server.ListPrompts()}
  case "prompts/get":
   var get GetPromptRequest
   if err:=json.Unmarshal(incoming.Params,&get.Params);err!=nil { outgoing.Error=&rpcError{Code:-32602,Message:"invalid params"};break }
   handler,found:=server.prompts[get.Params.Name];if !found { outgoing.Error=&rpcError{Code:-32601,Message:"unknown prompt"};break }
   result,err:=handler(ctx,get);if err!=nil { outgoing.Error=&rpcError{Code:-32000,Message:err.Error()} } else { outgoing.Result=result }
		case "resources/list":
			outgoing.Result = map[string]interface{}{"resources": server.ListResources()}
		case "resources/templates/list":
			outgoing.Result = map[string]interface{}{"resourceTemplates": server.ListResourceTemplates()}
		case "resources/read":
			var read ReadResourceRequest
			if err := json.Unmarshal(incoming.Params, &read.Params); err != nil {
				outgoing.Error = &rpcError{Code: -32602, Message: "invalid params"}
				break
			}
			handler, ok := server.resourceHandler(read.Params.URI)
			if !ok {
				outgoing.Error = &rpcError{Code: -32601, Message: "unknown resource"}
				break
			}
			contents, err := handler(ctx, read)
			if err != nil {
				outgoing.Error = &rpcError{Code: -32000, Message: err.Error()}
			} else {
				outgoing.Result = map[string]interface{}{"contents": contents}
			}
		case "tools/call":
			var call CallToolRequest
			if err := json.Unmarshal(incoming.Params, &call.Params); err != nil {
				outgoing.Error = &rpcError{Code: -32602, Message: "invalid params"}
				break
			}
			registered, ok := server.tools[call.Params.Name]
			if !ok {
				outgoing.Error = &rpcError{Code: -32601, Message: "unknown tool"}
				break
			}
			result, err := registered.Handler(ctx, call)
			if err != nil {
				outgoing.Error = &rpcError{Code: -32000, Message: err.Error()}
			} else {
				outgoing.Result = result
			}
		default:
			outgoing.Error = &rpcError{Code: -32601, Message: fmt.Sprintf("unknown method %q", incoming.Method)}
		}
		if err := encoder.Encode(outgoing); err != nil {
			return err
		}
	}
	if err:=ctx.Err();err!=nil { return err };return scanner.Err()
}

// #endregion 🖥️Mcp Server Adapter

// #endregion 🔌️Mcp Surface
