// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔌️mcp carries the MCP entry of the cli: the stdio runner, result conversion, argument and path guards.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	math "math"
	os "os"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔌️Mcp Surface

// #region 🦀️Mcp

// 📝️textResult holds the data fields for a textResult record.
func textResult(text string) *CallToolResult {
	return NewToolResultText(text)
}

// 🔷️toolResultToMCP holds the data fields for a toolResultToMCP record.
func toolResultToMCP(result workspace.ToolResult) (*CallToolResult, error) {
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return NewToolResultText(strings.Join(lines, "\n")), nil
}

// #endregion 🦀️Mcp

// #region 🎼️Args
// ⌨️Argument parsing utilities for CLI and MCP commands.
func getArgs(request CallToolRequest) map[string]interface{} {
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

// #endregion 🎼️Args

// #region 🔧️Paths
// 🛠️Path resolution utilities for file and folder operations.
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

// #endregion 🔧️Paths

// #endregion 🔌️Mcp Surface
