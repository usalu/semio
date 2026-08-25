// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package mcpserver provides the owned MCP registry and bounded JSON-RPC stdio loop.

// #endregion 🧲️Header

package mcpserver

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/usalu/semio/repo/client/internal/mcp"
)

// #region 📜️ServerSchema

type ServerOption func(*MCPServer)
type ToolHandler func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error)
type PromptHandler func(context.Context, mcp.GetPromptRequest) (*mcp.GetPromptResult, error)
type ResourceHandler func(context.Context, mcp.ReadResourceRequest) ([]mcp.ResourceContents, error)

type ServerTool struct {
	Tool    mcp.Tool
	Handler ToolHandler
}

type MCPServer struct {
	Name      string
	Version   string
	tools     map[string]ServerTool
	prompts   map[string]PromptHandler
	resources map[string]ResourceHandler
}

// #endregion 📜️ServerSchema

// #region 📚️Registry

func WithToolCapabilities(_ bool) ServerOption   { return func(*MCPServer) {} }
func WithPromptCapabilities(_ bool) ServerOption { return func(*MCPServer) {} }

func NewMCPServer(name, version string, options ...ServerOption) *MCPServer {
	server := &MCPServer{Name: name, Version: version, tools: map[string]ServerTool{}, prompts: map[string]PromptHandler{}, resources: map[string]ResourceHandler{}}
	for _, option := range options {
		option(server)
	}
	return server
}

func (server *MCPServer) AddTool(tool mcp.Tool, handler ToolHandler) {
	server.tools[tool.Name] = ServerTool{Tool: tool, Handler: handler}
}

func (server *MCPServer) AddPrompt(prompt mcp.Prompt, handler PromptHandler) {
	server.prompts[prompt.Name] = handler
}
func (server *MCPServer) AddResource(resource mcp.Resource, handler ResourceHandler) {
	server.resources[resource.URI] = handler
}
func (server *MCPServer) AddResourceTemplate(resource mcp.ResourceTemplate, handler ResourceHandler) {
	server.resources[resource.URI] = handler
}
func (server *MCPServer) ListTools() map[string]ServerTool {
	result := make(map[string]ServerTool, len(server.tools))
	for name, tool := range server.tools {
		result[name] = tool
	}
	return result
}

// #endregion 📚️Registry

// #region 🔌️Stdio

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

func ServeStdio(server *MCPServer) error {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 64*1024), 4*1024*1024)
	encoder := json.NewEncoder(os.Stdout)
	for scanner.Scan() {
		var incoming request
		if err := json.Unmarshal(scanner.Bytes(), &incoming); err != nil {
			if err := encoder.Encode(response{JSONRPC: "2.0", Error: &rpcError{Code: -32700, Message: "parse error"}}); err != nil {
				return err
			}
			continue
		}
		outgoing := response{JSONRPC: "2.0", ID: incoming.ID}
		switch incoming.Method {
		case "initialize":
			outgoing.Result = map[string]interface{}{"protocolVersion": "2025-06-18", "serverInfo": map[string]string{"name": server.Name, "version": server.Version}, "capabilities": map[string]interface{}{"tools": map[string]interface{}{}, "prompts": map[string]interface{}{}, "resources": map[string]interface{}{}}}
		case "tools/list":
			tools := make([]mcp.Tool, 0, len(server.tools))
			for _, registered := range server.tools {
				tools = append(tools, registered.Tool)
			}
			outgoing.Result = map[string]interface{}{"tools": tools}
		case "tools/call":
			var call mcp.CallToolRequest
			if err := json.Unmarshal(incoming.Params, &call.Params); err != nil {
				outgoing.Error = &rpcError{Code: -32602, Message: "invalid params"}
				break
			}
			registered, ok := server.tools[call.Params.Name]
			if !ok {
				outgoing.Error = &rpcError{Code: -32601, Message: "unknown tool"}
				break
			}
			result, err := registered.Handler(context.Background(), call)
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
	return scanner.Err()
}

// #endregion 🔌️Stdio
