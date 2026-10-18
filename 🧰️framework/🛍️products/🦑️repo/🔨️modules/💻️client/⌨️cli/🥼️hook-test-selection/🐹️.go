// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook test selection resolution.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

func resolveFullTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return nil
	}
	if cwd == "" {
		cwd = GetRootDir()
	}

	testSeg, extractedCwd := extractTestSegmentFromCommand(command)
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
		return resolveGoTestFiles([]string{"go", "test", "./..."}, cwd)
	case "cargo":
		return resolveCargoTestFiles([]string{"cargo", "test"}, cwd)
	case "dotnet":
		return resolveDotnetTestFiles([]string{"dotnet", "test"}, cwd)
	case "python", "python3":
		return resolvePythonTestFiles([]string{bin, "-m", "pytest"}, cwd)
	case "pytest", "py.test":
		return resolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" {
			runner := filepath.Base(parts[2])
			if runner == "pytest" || pyTestRunnerBins[runner] {
				return resolvePytestFiles(nil, cwd)
			}
		}
		return nil
	case "uvx":
		if len(parts) > 1 && pyTestRunnerBins[filepath.Base(parts[1])] {
			return resolvePytestFiles(nil, cwd)
		}
		return nil
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return findJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 {
			runner := filepath.Base(parts[1])
			if jsTestRunnerBins[runner] {
				return findJSTestFiles(cwd)
			}
			if runner == "phpunit" {
				return resolvePHPTestFiles(cwd)
			}
		}
		return nil
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return findJSTestFiles(cwd)
		}
		return nil
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && filepath.Base(parts[2]) == "rspec" {
			return resolveRspecFiles(nil, cwd)
		}
		return nil
	case "rspec":
		return resolveRspecFiles(nil, cwd)
	default:
		if strings.HasPrefix(bin, "phpunit") || strings.HasSuffix(bin, "/phpunit") {
			return resolvePHPTestFiles(cwd)
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
		id := resolvePathToFileID(filePath)
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

	exact := resolveTestNamesToDefinitionIDs(filePaths, selectors)
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
		normalized := normalizeHookPath(filePath)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(GetRootDir(), absPath)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		definitions := collectTestDefinitionsFromContent(string(content), normalized)
		for i := range definitions {
			definitionID := definitions[i].GetID()
			if definitionID == "" || seen[definitionID] {
				continue
			}
			for _, selector := range remaining {
				if testSelectorMatchesDefinitionName(selector, definitions[i].Name) {
					seen[definitionID] = true
					result = append(result, definitionID)
					break
				}
			}
		}
	}
	return result
}

func collectTestDefinitionsFromContent(content string, normalized string) []Definition {
	definitions := ParseDefinitions(content, normalized)
	var tests []Definition
	seen := map[string]bool{}
	for i := range definitions {
		if definitions[i].Kind == "" && isTestOrBenchmarkFile(normalized) {
			definitions[i].Kind = DefinitionKindTest
		}
		if definitions[i].Kind != DefinitionKindTest {
			continue
		}
		id := definitions[i].GetID()
		if id == "" || seen[id] {
			continue
		}
		seen[id] = true
		tests = append(tests, definitions[i])
	}
	if len(tests) > 0 {
		return tests
	}
	return fallbackTestDefinitionsFromContent(content, normalized)
}

func fallbackTestDefinitionsFromContent(content string, normalized string) []Definition {
	patterns := []*regexp.Regexp{
		regexp.MustCompile(`(?m)^[\t ]*func[\t ]+(Test[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*def[\t ]+(test_[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*(?:it|test)\s*\(?\s*['"]([^'"]+)['"]`),
		regexp.MustCompile(`(?m)^[\t ]*public[\t ]+function[\t ]+(test[A-Za-z0-9_]+)\s*\(`),
	}
	var tests []Definition
	seen := map[string]bool{}
	for _, pattern := range patterns {
		matches := pattern.FindAllStringSubmatch(content, -1)
		for _, match := range matches {
			if len(match) < 2 {
				continue
			}
			name := strings.TrimSpace(match[1])
			if name == "" {
				continue
			}
			definition := Definition{Name: name, Kind: DefinitionKindTest, FilePath: normalized}
			id := definition.GetID()
			if id == "" || seen[id] {
				continue
			}
			seen[id] = true
			tests = append(tests, definition)
		}
	}
	return tests
}

func testSelectorMatchesDefinitionName(selector string, definitionName string) bool {
	selector = strings.TrimSpace(selector)
	selector = strings.Trim(selector, `"'`)
	if selector == "" || definitionName == "" {
		return false
	}
	selectorLower := strings.ToLower(selector)
	definitionLower := strings.ToLower(definitionName)
	if selectorLower == definitionLower || strings.Contains(definitionLower, selectorLower) {
		return true
	}
	pattern, err := regexp.Compile(selector)
	if err == nil && pattern.MatchString(definitionName) {
		return true
	}
	quoted, quoteErr := regexp.Compile(regexp.QuoteMeta(selector))
	return quoteErr == nil && quoted.MatchString(definitionName)
}
