package todos

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
)

// #region 🔎️Hook Search Extraction

// 📋️extractPlanStepsFromInput extracts plan steps from input JSON or toolArgs string.
func ExtractPlanStepsFromInput(input json.RawMessage, toolArgs string) []model.HookPlanStep {
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

	var steps []model.HookPlanStep
	if todoList, ok := sourceMap["todoList"]; ok {
		if todoListSlice, ok := todoList.([]interface{}); ok {
			for _, item := range todoListSlice {
				if itemMap, ok := item.(map[string]interface{}); ok {
					name, _ := itemMap["title"].(string)
					status, _ := itemMap["status"].(string)
					if name != "" {
						steps = append(steps, model.HookPlanStep{Name: name, Status: status})
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
						steps = append(steps, model.HookPlanStep{Name: name, Status: status})
					}
				}
			}
		}
	}

	return steps
}

// 🔎️extractSearchFromInput extracts search information from input JSON or toolArgs string.
func ExtractSearchFromInput(input json.RawMessage, toolArgs string) ([]string, []string) {
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
func ExtractToolInputFromStdin(input json.RawMessage) json.RawMessage {
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
func ExtractToolResponseFromStdin(input json.RawMessage) json.RawMessage {
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

func ResolveEventCheckpointID(event interface{}) string {
	s, _ := event.(string)
	return strings.TrimSpace(s)
}

func resolveRangeRef(ref string) (string, error) {
	parts := strings.SplitN(strings.TrimSpace(ref), "#L", 2)
	if len(parts) != 2 {
		return "", fmt.Errorf("invalid range ref %q", ref)
	}
	fileID := codebase.ResolvePathToFileID(parts[0])
	if fileID == "" {
		return "", fmt.Errorf("invalid file path %q", parts[0])
	}
	lineSpec := parts[1]
	if strings.Contains(lineSpec, "-L") {
		rangeParts := strings.SplitN(lineSpec, "-L", 2)
		start, errStart := strconv.Atoi(rangeParts[0])
		end, errEnd := strconv.Atoi(rangeParts[1])
		if errStart != nil || errEnd != nil {
			return "", fmt.Errorf("invalid line range %q", lineSpec)
		}
		return fileID + model.EmojiText(model.EmojiLine) + strconv.Itoa(start) + model.EmojiText(model.EmojiLine) + strconv.Itoa(end), nil
	}
	line, err := strconv.Atoi(lineSpec)
	if err != nil {
		return "", fmt.Errorf("invalid line %q", lineSpec)
	}
	return fileID + model.EmojiText(model.EmojiLine) + strconv.Itoa(line), nil
}

func ExtractSearchCommandFilesAndPattern(command string, cwd string) ([]string, string) {
	fields := strings.Fields(command)
	if len(fields) == 0 {
		return nil, ""
	}
	var pattern string
	var files []string
	for i := 1; i < len(fields); i++ {
		field := strings.Trim(fields[i], `"'`)
		if field == "" {
			continue
		}
		if strings.HasPrefix(field, "-") {
			if field == "-e" && i+1 < len(fields) {
				pattern = strings.Trim(fields[i+1], `"'`)
				i++
			}
			continue
		}
		if pattern == "" {
			pattern = field
			continue
		}
		if !filepath.IsAbs(field) && cwd != "" {
			field = filepath.Join(cwd, field)
		}
		files = append(files, field)
	}
	if len(files) == 0 && cwd != "" && len(fields) > 0 {
		for _, field := range fields[1:] {
			trimmed := strings.Trim(field, `"'`)
			if strings.HasPrefix(trimmed, "-") || trimmed == pattern || trimmed == "" {
				continue
			}
			files = append(files, filepath.Join(cwd, trimmed))
		}
	}
	return files, pattern
}

func SearchLinesInFile(filePath string, pattern string) []int {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return nil
	}
	var lines []int
	needle := strings.TrimSpace(pattern)
	for index, line := range strings.Split(string(content), "\n") {
		if needle == "" || strings.Contains(line, needle) {
			lines = append(lines, index+1)
		}
	}
	return lines
}

// #endregion 🔎️Hook Search Extraction
