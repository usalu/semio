// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧪️test-input carries the extraction of test and build events from a hook input and the test file and lab resolution.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	testrunner "github.com/usalu/semio/repo/testrunner"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🦀️Hooks

// 🔷️HookEventKind returns the hook kind for the given event.
func HookEventKind(e model.HookEvent) model.HookKind {
	switch e {
	case model.HookVersionCheckpointStarting, model.HookVersionCheckpointEnded,
		model.HookVersionCheckinStarting, model.HookVersionCheckinEnded,
		model.HookVersionCheckoutStarting, model.HookVersionCheckoutEnded:
		return model.HookKindVersion
	default:
		return model.HookKindAgent
	}
}

func resolveFullTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return nil
	}
	if cwd == "" {
		cwd = workspace.GetRootDir()
	}

	testSeg, extractedCwd := testrunner.ExtractTestSegmentFromCommand(command)
	if testSeg != "" && testSeg != command {
		command = testSeg
		if extractedCwd != "" {
			if filepath.IsAbs(extractedCwd) {
				cwd = extractedCwd
			} else {
				cwd = filepath.Join(cwd, extractedCwd)
			}
		}
	}
	parts := strings.Fields(command)
	if len(parts) == 0 {
		return nil
	}
	bin := filepath.Base(parts[0])

	switch bin {
	case "go":
		return testrunner.ResolveGoTestFiles([]string{"go", "test", "./..."}, cwd)
	case "cargo":
		return testrunner.ResolveCargoTestFiles([]string{"cargo", "test"}, cwd)
	case "dotnet":
		return testrunner.ResolveDotnetTestFiles([]string{"dotnet", "test"}, cwd)
	case "python", "python3":
		return testrunner.ResolvePythonTestFiles([]string{bin, "-m", "pytest"}, cwd)
	case "pytest", "py.test":
		return testrunner.ResolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" {
			runner := filepath.Base(parts[2])
			if runner == "pytest" || testrunner.PyTestRunnerBins[runner] {
				return testrunner.ResolvePytestFiles(nil, cwd)
			}
		}
		return nil
	case "uvx":
		if len(parts) > 1 && testrunner.PyTestRunnerBins[filepath.Base(parts[1])] {
			return testrunner.ResolvePytestFiles(nil, cwd)
		}
		return nil
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return testrunner.FindJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 {
			runner := filepath.Base(parts[1])
			if testrunner.JsTestRunnerBins[runner] {
				return testrunner.FindJSTestFiles(cwd)
			}
			if runner == "phpunit" {
				return todos.ResolvePHPTestFiles(cwd)
			}
		}
		return nil
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return testrunner.FindJSTestFiles(cwd)
		}
		return nil
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && filepath.Base(parts[2]) == "rspec" {
			return testrunner.ResolveRspecFiles(nil, cwd)
		}
		return nil
	case "rspec":
		return testrunner.ResolveRspecFiles(nil, cwd)
	default:
		if strings.HasPrefix(bin, "phpunit") || strings.HasSuffix(bin, "/phpunit") {
			return todos.ResolvePHPTestFiles(cwd)
		}
		return nil
	}
}

func resolveTestLabIDs(filePaths []string) []string {
	if len(filePaths) == 0 {
		return nil
	}
	var result []string
	seen := map[string]bool{}
	for _, filePath := range filePaths {
		id := codebase.ResolvePathToFileID(filePath)
		if id == "" || seen[id] {
			continue
		}
		seen[id] = true
		result = append(result, id)
	}
	return result
}

func sameStringSet(left []string, right []string) bool {
	leftSeen := map[string]bool{}
	rightSeen := map[string]bool{}
	for _, item := range left {
		if item != "" {
			leftSeen[item] = true
		}
	}
	for _, item := range right {
		if item != "" {
			rightSeen[item] = true
		}
	}
	if len(leftSeen) != len(rightSeen) {
		return false
	}
	for item := range leftSeen {
		if !rightSeen[item] {
			return false
		}
	}
	return true
}

func resolveSelectedTestDefinitionIDs(filePaths []string, selectors []string) []string {
	if len(filePaths) == 0 || len(selectors) == 0 {
		return nil
	}

	exact := codebase.ResolveTestNamesToDefinitionIDs(filePaths, selectors)
	var result []string
	seen := map[string]bool{}
	for _, selector := range selectors {
		if id, ok := exact[selector]; ok && id != "" && !seen[id] {
			seen[id] = true
			result = append(result, id)
		}
	}

	remaining := make([]string, 0, len(selectors))
	for _, selector := range selectors {
		if _, ok := exact[selector]; !ok {
			remaining = append(remaining, selector)
		}
	}
	if len(remaining) == 0 {
		return result
	}

	for _, filePath := range filePaths {
		normalized := workspace.NormalizeHookPath(filePath)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(workspace.GetRootDir(), absPath)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		definitions := codebase.CollectTestDefinitionsFromContent(string(content), normalized)
		for i := range definitions {
			definitionID := definitions[i].GetID()
			if definitionID == "" || seen[definitionID] {
				continue
			}
			for _, selector := range remaining {
				if codebase.TestSelectorMatchesDefinitionName(selector, definitions[i].Name) {
					seen[definitionID] = true
					result = append(result, definitionID)
					break
				}
			}
		}
	}
	return result
}

func extractTestStartingFromInput(input json.RawMessage, toolArgs string) (labs []string, tests []string, timeout string) {
	var toolInput map[string]interface{}
	var rawData map[string]interface{}
	if len(input) > 0 {
		if err := json.Unmarshal(input, &rawData); err == nil {
			toolInput = extractToolInputMapFromData(rawData)
		}
	}
	if toolInput == nil && toolArgs != "" {
		_ = json.Unmarshal([]byte(toolArgs), &toolInput)
	}
	if toolInput == nil {
		toolInput = map[string]interface{}{}
	}

	var targetFiles []string

	for _, k := range []string{"files"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					targetFiles = append(targetFiles, s)
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			targetFiles = append(targetFiles, v)
		}
	}
	for _, k := range []string{"testNames", "test_names", "tests"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					if looksLikeFilePath(s) {
						targetFiles = append(targetFiles, s)
					} else {
						tests = append(tests, s)
					}
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			if looksLikeFilePath(v) {
				targetFiles = append(targetFiles, v)
			} else {
				tests = append(tests, v)
			}
		}
	}
	for _, k := range []string{"timeout", "timeoutMs"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			timeout = v
			break
		}
		if v, ok := toolInput[k].(float64); ok {
			timeout = fmt.Sprintf("%d", int64(v))
			break
		}
	}

	cmd := extractCommandFromStdin(input)
	if cmd == "" {
		if c, ok := toolInput["command"].(string); ok {
			cmd = c
		}
	}
	if len(tests) == 0 && cmd != "" {
		parsedTests, parsedTimeout := testrunner.ParseTestInfoFromCommand(cmd)
		if len(parsedTests) > 0 {
			tests = parsedTests
		}
		if timeout == "" && parsedTimeout != "" {
			timeout = parsedTimeout
		}
	}

	if cmd != "" && len(targetFiles) == 0 {
		cwd := ""
		if rawData != nil {
			_, cwd = extractCommandCwdFromInput(input)
		}
		commandFiles := testrunner.ResolveTestFilesFromCommand(cmd, cwd)
		if len(commandFiles) > 0 {
			targetFiles = commandFiles
		}
	}

	fullRunSelection := false
	if len(targetFiles) > 0 && (len(tests) == 0 || (len(tests) == 1 && tests[0] == "")) {
		cwd := ""
		if rawData != nil {
			_, cwd = extractCommandCwdFromInput(input)
		}
		fullRunFiles := resolveFullTestFilesFromCommand(cmd, cwd)
		fullRunSelection = len(fullRunFiles) > 0 && sameStringSet(targetFiles, fullRunFiles)
	}
	if fullRunSelection {
		labs = resolveTestLabIDs(targetFiles)
		return labs, nil, timeout
	}

	if len(targetFiles) > 0 {
		resolvedTests := resolveSelectedTestDefinitionIDs(targetFiles, tests)
		if len(resolvedTests) == 0 {
			resolvedTests = todos.ResolveAllTestDefinitionIDs(targetFiles)
		}
		if len(resolvedTests) > 0 {
			tests = resolvedTests
		}
	}

	return nil, tests, timeout
}

// 🧲️extractTestEndedFromInput holds the data fields for a extractTestEndedFromInput record.
func extractTestEndedFromInput(input json.RawMessage) (files []string, succeeded []string, failed []string) {
	if len(input) == 0 {
		return nil, nil, nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil, nil, nil
	}

	if ti := extractToolInputMapFromData(data); ti != nil {
		for _, k := range []string{"files"} {
			if arr, ok := ti[k].([]interface{}); ok {
				for _, item := range arr {
					if s, ok := item.(string); ok && s != "" {
						files = append(files, s)
					}
				}
			} else if v, ok := ti[k].(string); ok && v != "" {
				files = append(files, v)
			}
		}
	}

	if len(files) == 0 {
		cmd := extractCommandFromStdin(input)
		if cmd != "" {
			_, cwd := extractCommandCwdFromInput(input)
			commandFiles := testrunner.ResolveTestFilesFromCommand(cmd, cwd)
			if len(commandFiles) > 0 {
				files = commandFiles
			}
		}
	}
	toolOutput := data
	if to, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = to
	}
	for _, k := range []string{"succeeded", "passed", "passing"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					succeeded = append(succeeded, s)
				}
			}
		}
	}
	for _, k := range []string{"failed", "failing", "errors"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					failed = append(failed, s)
				}
			}
		}
	}
	return files, succeeded, failed
}

func extractBuildBundlesFromInput(input json.RawMessage, toolArgs string) []string {
	var toolInput map[string]interface{}
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			toolInput = extractToolInputMapFromData(data)
		}
	}
	if toolInput == nil && toolArgs != "" {
		_ = json.Unmarshal([]byte(toolArgs), &toolInput)
	}
	if toolInput == nil {
		return nil
	}
	var bundles []string
	for _, k := range []string{"bundles", "targets", "technologies", "label"} {
		if arr, ok := toolInput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					bundles = append(bundles, s)
				}
			}
		} else if v, ok := toolInput[k].(string); ok && v != "" {
			bundles = append(bundles, v)
		}
	}
	return bundles
}

// 🏗️extractBuildEndedFromInput holds the data fields for a extractBuildEndedFromInput record.
func extractBuildEndedFromInput(input json.RawMessage) (succeeded []string, failed []string) {
	if len(input) == 0 {
		return nil, nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil, nil
	}
	toolOutput := data
	if to, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = to
	}
	for _, k := range []string{"succeeded", "passed", "built"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					succeeded = append(succeeded, s)
				}
			}
		}
	}
	for _, k := range []string{"failed", "errors"} {
		if arr, ok := toolOutput[k].([]interface{}); ok {
			for _, item := range arr {
				if s, ok := item.(string); ok && s != "" {
					failed = append(failed, s)
				}
			}
		}
	}
	return succeeded, failed
}

// #endregion 🦀️Hooks
