// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook search extraction.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// 📋️extractPlanStepsFromInput extracts plan steps from input JSON or toolArgs string.
func extractPlanStepsFromInput(input json.RawMessage, toolArgs string) []HookPlanStep {
	var data map[string]interface{}
	var source interface{}

	if input != nil {
		if err := json.Unmarshal(input, &data); err != nil {
			if toolArgs != "" {
				if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
					return nil
				}
			} else {
				return nil
			}
		}
	} else if toolArgs != "" {
		if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
			return nil
		}
	} else {
		return nil
	}

	if ti, ok := data["tool_input"]; ok {
		source = ti
	} else {
		source = data
	}

	sourceMap, ok := source.(map[string]interface{})
	if !ok {
		return nil
	}

	var steps []HookPlanStep
	if todoList, ok := sourceMap["todoList"]; ok {
		if todoListSlice, ok := todoList.([]interface{}); ok {
			for _, item := range todoListSlice {
				if itemMap, ok := item.(map[string]interface{}); ok {
					name, _ := itemMap["title"].(string)
					status, _ := itemMap["status"].(string)
					if name != "" {
						steps = append(steps, HookPlanStep{Name: name, Status: status})
					}
				}
			}
		}
	}
	if stepsList, ok := sourceMap["steps"]; ok {
		if stepsSlice, ok := stepsList.([]interface{}); ok {
			for _, item := range stepsSlice {
				if itemMap, ok := item.(map[string]interface{}); ok {
					name, _ := itemMap["name"].(string)
					status, _ := itemMap["status"].(string)
					if name != "" {
						steps = append(steps, HookPlanStep{Name: name, Status: status})
					}
				}
			}
		}
	}

	return steps
}

// 🔎️extractSearchFromInput extracts search information from input JSON or toolArgs string.
func extractSearchFromInput(input json.RawMessage, toolArgs string) ([]string, []string) {
	var data map[string]interface{}
	var source interface{}

	if input != nil {
		if err := json.Unmarshal(input, &data); err != nil {
			if toolArgs != "" {
				if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
					return nil, nil
				}
			} else {
				return nil, nil
			}
		}
	} else if toolArgs != "" {
		if err := json.Unmarshal([]byte(toolArgs), &data); err != nil {
			return nil, nil
		}
	} else {
		return nil, nil
	}

	if ti, ok := data["tool_input"]; ok {
		source = ti
	} else {
		source = data
	}

	sourceMap, ok := source.(map[string]interface{})
	if !ok {
		return nil, nil
	}

	var pages []string
	var ranges []string
	if url, ok := sourceMap["url"].(string); ok && url != "" {
		pages = append(pages, url)
	}
	if pagesList, ok := sourceMap["pages"].([]interface{}); ok {
		for _, page := range pagesList {
			if pageStr, ok := page.(string); ok && pageStr != "" {
				if strings.HasPrefix(pageStr, "http://") || strings.HasPrefix(pageStr, "https://") {
					pages = append(pages, pageStr)
				}
			}
		}
	}

	filePath := ""
	if fp, ok := sourceMap["file_path"].(string); ok {
		filePath = fp
	} else if fp, ok := sourceMap["filePath"].(string); ok {
		filePath = fp
	}
	if filePath != "" {
		startLine := 0
		endLine := 0
		if v, ok := sourceMap["startLine"].(float64); ok {
			startLine = int(v)
		}
		if v, ok := sourceMap["endLine"].(float64); ok {
			endLine = int(v)
		}
		limit := 0
		offset := 1
		if l, ok := sourceMap["limit"].(float64); ok {
			limit = int(l)
		}
		if o, ok := sourceMap["offset"].(float64); ok {
			offset = int(o)
		}
		if startLine > 0 {
			if endLine <= 0 {
				endLine = startLine
			}
			if startLine == endLine {
				ranges = append(ranges, fmt.Sprintf("%s#L%d", filePath, startLine))
			} else {
				ranges = append(ranges, fmt.Sprintf("%s#L%d-L%d", filePath, startLine, endLine))
			}
		} else if limit == 0 {
			if content, err := os.ReadFile(filePath); err == nil {
				trimmed := strings.TrimRight(string(content), "\n")
				lines := 1
				if trimmed != "" {
					lines = len(strings.Split(trimmed, "\n"))
				}
				ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", filePath, lines))
			}
		} else {
			endLine := offset + limit - 1
			ranges = append(ranges, fmt.Sprintf("%s#L%d-L%d", filePath, offset, endLine))
		}
	}

	if native, ok := data["native"].(map[string]interface{}); ok {
		if event, ok := native["event"].(map[string]interface{}); ok {
			if toolName, ok := event["tool_name"].(string); ok && toolName == "Read" {
				if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
					if fp, ok := toolInput["file_path"].(string); ok && fp != "" {
						if l, ok := toolInput["limit"].(float64); ok {
							ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", fp, int(l)))
						} else if content, err := os.ReadFile(fp); err == nil {
							trimmed := strings.TrimRight(string(content), "\n")
							lines := 1
							if trimmed != "" {
								lines = len(strings.Split(trimmed, "\n"))
							}
							ranges = append(ranges, fmt.Sprintf("%s#L1-L%d", fp, lines))
						}
					}
				}
			}
		}
	}

	return pages, ranges
}

// 🟥️extractToolInputFromStdin extracts tool input from stdin JSON.
func extractToolInputFromStdin(input json.RawMessage) json.RawMessage {
	if input == nil {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	if native, ok := data["native"].(map[string]interface{}); ok {
		if event, ok := native["event"].(map[string]interface{}); ok {
			if toolInput, ok := event["tool_input"]; ok {
				if bytes, err := json.Marshal(toolInput); err == nil {
					return json.RawMessage(bytes)
				}
			}
		}
	}
	if toolInput, ok := data["tool_input"]; ok {
		if bytes, err := json.Marshal(toolInput); err == nil {
			return json.RawMessage(bytes)
		}
	}
	return nil
}

// 📩️extractToolResponseFromStdin extracts tool response from stdin JSON.
func extractToolResponseFromStdin(input json.RawMessage) json.RawMessage {
	if input == nil {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	if toolResponse, ok := data["tool_response"]; ok {
		if bytes, err := json.Marshal(toolResponse); err == nil {
			return json.RawMessage(bytes)
		}
	}
	if toolOutput, ok := data["tool_output"]; ok {
		if bytes, err := json.Marshal(toolOutput); err == nil {
			return json.RawMessage(bytes)
		}
	}
	return nil
}

func extractSearchDefinitionReadsFromInput(input json.RawMessage, toolArgs string) []HookSearchDefinition {
	pages, ranges := extractSearchFromInput(input, toolArgs)
	_ = pages
	filesToLines := map[string]map[int]struct{}{}
	addLine := func(path string, line int) {
		path = normalizeHookPath(path)
		if path == "" {
			return
		}
		if _, ok := filesToLines[path]; !ok {
			filesToLines[path] = map[int]struct{}{}
		}
		if line > 0 {
			filesToLines[path][line] = struct{}{}
		}
	}
	for _, ref := range ranges {
		parts := strings.SplitN(ref, "#L", 2)
		path := parts[0]
		if len(parts) == 1 {
			addLine(path, 0)
			continue
		}
		lineSpec := parts[1]
		if strings.Contains(lineSpec, "-L") {
			rangeParts := strings.SplitN(lineSpec, "-L", 2)
			start, _ := strconv.Atoi(rangeParts[0])
			end, _ := strconv.Atoi(rangeParts[1])
			for line := start; line <= end; line++ {
				addLine(path, line)
			}
		} else if line, err := strconv.Atoi(lineSpec); err == nil {
			addLine(path, line)
		}
	}
	data := decodeHookInputMap(input)
	if toolInfo, ok := data["tool_info"].(map[string]interface{}); ok {
		if filePath, _ := toolInfo["file_path"].(string); filePath != "" {
			addLine(filePath, 0)
		}
		commandLine, _ := toolInfo["command_line"].(string)
		cwd, _ := toolInfo["cwd"].(string)
		if commandLine != "" && shouldExecuteSearchCommand(commandLine) {
			commandFiles, pattern := extractSearchCommandFilesAndPattern(commandLine, cwd)
			for _, filePath := range commandFiles {
				matchedLines := searchLinesInFile(filePath, pattern)
				if len(matchedLines) == 0 {
					addLine(filePath, 0)
					continue
				}
				for _, line := range matchedLines {
					addLine(filePath, line)
				}
			}
		}
	}
	return buildDefinitionReads(filesToLines)
}

func buildDefinitionReads(filesToLines map[string]map[int]struct{}) []HookSearchDefinition {
	var result []HookSearchDefinition
	for filePath, lines := range filesToLines {
		normalized := normalizeHookPath(filePath)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(GetRootDir(), normalized)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		if _, ok := lines[0]; ok {
			result = append(result, HookSearchDefinition{ID: resolvePathToFileID(normalized), Loc: 0})
			continue
		}
		lineCount := strings.Count(string(content), "\n") + 1
		if len(lines) == 0 || spansEntireFile(lines, string(content), lineCount) {
			result = append(result, HookSearchDefinition{ID: resolvePathToFileID(normalized), Loc: 0})
			continue
		}
		definitions := ParseDefinitions(string(content), normalized)
		matched := false
		for _, definition := range definitions {
			loc := 0
			for line := range lines {
				if line >= definition.StartLine && line <= definition.EndLine {
					loc++
				}
			}
			if loc > 0 {
				matched = true
				result = append(result, HookSearchDefinition{ID: definition.GetID(), Loc: loc})
			}
		}
		if !matched {
			result = append(result, HookSearchDefinition{ID: resolvePathToFileID(normalized), Loc: 0})
		}
	}
	return result
}

func shouldExecuteSearchCommand(command string) bool {
	command = strings.TrimSpace(command)
	if command == "" {
		return false
	}
	if strings.Contains(command, ">") || strings.Contains(command, "sed -i") {
		return false
	}
	segments := splitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	for _, segment := range segments {
		fields := strings.Fields(segment)
		if len(fields) == 0 {
			continue
		}
		switch filepath.Base(fields[0]) {
		case "grep", "rg", "ripgrep", "ag", "ack", "cat", "bat", "batcat", "less", "head", "tail":
			return true
		}
	}
	return false
}
