// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP server runtime, argument, and path helpers.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	mcp "github.com/usalu/semio/repo/client/internal/mcp"
	templatefunc "github.com/usalu/semio/repo/client/internal/templatefunc"
	yaml "github.com/usalu/semio/repo/yaml"
	"math"
	"os"
	"path/filepath"
	"strings"
	"text/template"
)

// 💿️runMcpServer is kept for command wiring compatibility.
func runMcpServer(cmd *command.Command, args []string) error {
	kind := McpClientGeneric
	if len(args) > 0 {
		parsed, err := ParseMcpClientKind(args[0])
		if err != nil {
			return err
		}
		kind = parsed
	}
	return serveMcp(cmd.Context(), kind, DefaultCommandTimeout)
}

// 📝️textResult holds the data fields for a textResult record.
func textResult(text string) *mcp.CallToolResult {
	return mcp.NewToolResultText(text)
}

// 🔷️toolResultToMCP holds the data fields for a toolResultToMCP record.
func toolResultToMCP(result ToolResult) (*mcp.CallToolResult, error) {
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return mcp.NewToolResultText(strings.Join(lines, "\n")), nil
}

func getArgs(request mcp.CallToolRequest) map[string]interface{} {
	if args, ok := request.Params.Arguments.(map[string]interface{}); ok {
		return args
	}
	return make(map[string]interface{})
}

func getStringArg(args map[string]interface{}, key string) (string, bool, error) {
	value, ok := args[key]
	if !ok {
		return "", false, nil
	}
	str, ok := value.(string)
	if !ok || str == "" {
		return "", true, fmt.Errorf("invalid %s", key)
	}
	return str, true, nil
}

// 💿️requireStringArg holds the data fields for a requireStringArg record.
func requireStringArg(args map[string]interface{}, key string) (string, error) {
	value, ok, err := getStringArg(args, key)
	if err != nil {
		return "", err
	}
	if !ok {
		return "", fmt.Errorf("missing %s", key)
	}
	return value, nil
}

// 🔷️getIntArg holds the data fields for a getIntArg record.
func getIntArg(args map[string]interface{}, key string) (int, bool, error) {
	value, ok := args[key]
	if !ok {
		return 0, false, nil
	}
	number, ok := value.(float64)
	if !ok || number != math.Trunc(number) {
		return 0, true, fmt.Errorf("invalid %s", key)
	}
	return int(number), true, nil
}

// 🔶️requireIntArg holds the data fields for a requireIntArg record.
func requireIntArg(args map[string]interface{}, key string) (int, error) {
	value, ok, err := getIntArg(args, key)
	if err != nil {
		return 0, err
	}
	if !ok {
		return 0, fmt.Errorf("missing %s", key)
	}
	return value, nil
}

// 🔤️getStringSliceArg holds the data fields for a getStringSliceArg record.
func getStringSliceArg(args map[string]interface{}, key string) ([]string, bool, error) {
	value, ok := args[key]
	if !ok {
		return nil, false, nil
	}
	list, ok := value.([]interface{})
	if !ok || len(list) == 0 {
		return nil, true, fmt.Errorf("invalid %s", key)
	}
	result := make([]string, 0, len(list))
	for _, item := range list {
		str, ok := item.(string)
		if !ok || str == "" {
			return nil, true, fmt.Errorf("invalid %s", key)
		}
		result = append(result, str)
	}
	return result, true, nil
}

// 🔘️getBoolArg holds the data fields for a getBoolArg record.
func getBoolArg(args map[string]interface{}, key string) (bool, bool, error) {
	value, ok := args[key]
	if !ok {
		return false, false, nil
	}
	boolVal, ok := value.(bool)
	if !ok {
		return false, true, fmt.Errorf("invalid %s", key)
	}
	return boolVal, true, nil
}

func requireFilePath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid file path: %s", path)
	}
	if info.IsDir() {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

// 📁️requireFolderPath holds the data fields for a requireFolderPath record.
func requireFolderPath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	if !info.IsDir() {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// 💿️requireFileTargetPath holds the data fields for a requireFileTargetPath record.
func requireFileTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if info.IsDir() {
			return fmt.Errorf("invalid file path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

// 🛤️requireFolderTargetPath holds the data fields for a requireFolderTargetPath record.
func requireFolderTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if !info.IsDir() {
			return fmt.Errorf("invalid folder path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// 🔖️jsonToYaml holds the data fields for a jsonToYaml record.
func jsonToYaml(jsonStr string) (string, error) {
	var data interface{}
	if err := json.Unmarshal([]byte(jsonStr), &data); err != nil {
		return "", err
	}
	yamlBytes, err := yaml.Marshal(data)
	if err != nil {
		return "", err
	}
	return string(yamlBytes), nil
}

// 💿️gql holds the data fields for a gql record.
func gql(query string, variables map[string]interface{}) (string, error) {
	ensureExecutor()
	if executor == nil {
		return "", fmt.Errorf("GraphQL executor not initialized")
	}
	return executor.ExecuteJSON(context.Background(), query, variables)
}

// ⏹️renderPromptTemplate holds the data fields for a renderPromptTemplate record.
func renderPromptTemplate(name string, data map[string]string) (string, error) {
	path := GetRepoMetaPath(filepath.Join("💬️prompts", name+".tpl"))
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	tmpl, err := template.New(name).Funcs(templatefunc.TxtFuncMap()).Parse(string(content))
	if err != nil {
		return "", err
	}
	var out strings.Builder
	if err := tmpl.Execute(&out, data); err != nil {
		return "", err
	}
	return out.String(), nil
}
