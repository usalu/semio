// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖲️command-files carries the command segment extraction and the test file resolvers by runner.

// #endregion 🧲️Header

package testrunner

import (
	os "os"
	filepath "path/filepath"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🖲️Missing Test Functions

// 🧪️pyTestRunnerBins are direct Python test runner binary names.
var PyTestRunnerBins = map[string]bool{
	"pytest": true, "py.test": true, "nosetests": true, "nose2": true,
}

// 🧲️extractTestSegmentFromCommand extracts the test-relevant segment from a command.
func ExtractTestSegmentFromCommand(command string) (string, string) {
	command = strings.TrimSpace(command)
	if command == "" {
		return "", ""
	}
	segments := workspace.SplitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	cwd := ""
	hasComposite := len(segments) > 1
	for _, segment := range segments {
		segment = strings.TrimSpace(segment)
		if segment == "" {
			continue
		}
		if strings.HasPrefix(segment, "cd ") {
			parts := strings.Fields(segment)
			if len(parts) >= 2 {
				cwd = parts[1]
			}
			continue
		}
		if strings.HasPrefix(segment, "export ") || strings.HasPrefix(segment, "set ") {
			continue
		}
		fields := strings.Fields(segment)
		if len(fields) == 0 {
			continue
		}
		bin := filepath.Base(fields[0])
		if isTestCommandSegment(segment, bin) {
			return trimPipelineTail(segment), cwd
		}
		if bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun" || bin == "uv" {
			if isTestCommandSegment(segment, "") {
				return trimPipelineTail(segment), cwd
			}
		}
	}
	if hasComposite {
		return "", cwd
	}
	return "", ""
}

func isTestCommandSegment(segment string, bin string) bool {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return false
	}
	if bin == "" {
		fields := strings.Fields(trimmed)
		if len(fields) == 0 {
			return false
		}
		bin = filepath.Base(fields[0])
	}
	if model.ClassifyCommandKind(trimmed) == model.ToolKindTest {
		return true
	}
	fields := strings.Fields(trimmed)
	if len(fields) < 2 {
		return false
	}
	if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun") && model.ClassifyCommandKind(strings.Join(fields[1:], " ")) == model.ToolKindTest {
		return true
	}
	if bin == "uv" {
		if fields[1] == "run" && len(fields) > 2 && model.ClassifyCommandKind(strings.Join(fields[2:], " ")) == model.ToolKindTest {
			return true
		}
		if model.ClassifyCommandKind(strings.Join(fields[1:], " ")) == model.ToolKindTest {
			return true
		}
	}
	return false
}

func trimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	var current strings.Builder
	quote := rune(0)
	for i := 0; i < len(trimmed); i++ {
		ch := rune(trimmed[i])
		if quote != 0 {
			current.WriteRune(ch)
			if ch == quote {
				quote = 0
			}
			continue
		}
		if ch == '\'' || ch == '"' {
			quote = ch
			current.WriteRune(ch)
			continue
		}
		if ch == '|' {
			break
		}
		current.WriteRune(ch)
	}
	return strings.TrimSpace(current.String())
}

// 📄️resolveGoTestFiles resolves test files for Go commands.
func ResolveGoTestFiles(args []string, cwd string) []string {
	var files []string
	seen := map[string]bool{}
	targets := make([]string, 0)
	for i := 2; i < len(args); i++ {
		arg := strings.TrimSpace(args[i])
		if arg == "" || strings.HasPrefix(arg, "-") {
			continue
		}
		targets = append(targets, arg)
	}
	addFile := func(path string) {
		if !seen[path] {
			seen[path] = true
			files = append(files, path)
		}
	}
	scanDir := func(root string, recursive bool) {
		_ = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if info.IsDir() {
				if !recursive && path != root {
					return filepath.SkipDir
				}
				return nil
			}
			if strings.HasSuffix(path, "_test.go") {
				addFile(path)
			}
			return nil
		})
	}
	if len(targets) == 0 {
		scanDir(cwd, true)
		return files
	}
	for _, target := range targets {
		recursive := strings.HasSuffix(target, "/...")
		targetPath := strings.TrimSuffix(target, "/...")
		if targetPath == "." || targetPath == "./" || targetPath == "" {
			targetPath = cwd
		} else if !filepath.IsAbs(targetPath) {
			targetPath = filepath.Join(cwd, targetPath)
		}
		info, err := os.Stat(targetPath)
		if err != nil {
			continue
		}
		if info.IsDir() {
			scanDir(targetPath, recursive)
			continue
		}
		if strings.HasSuffix(targetPath, "_test.go") {
			addFile(targetPath)
		}
	}
	return files
}

// 🎛️resolveCargoTestFiles resolves test files for Cargo commands.
func ResolveCargoTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for test directories and files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".rs") {
			content, err := os.ReadFile(path)
			if err != nil {
				return nil
			}
			if strings.Contains(string(content), "#[test") || strings.Contains(string(content), "#[cfg(test)") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔷️resolveDotnetTestFiles resolves test files for .NET commands.
func ResolveDotnetTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for .cs files with test attributes
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".cs") {
			content, err := os.ReadFile(path)
			if err != nil {
				return nil
			}
			if strings.Contains(string(content), "[Test") || strings.Contains(string(content), "[Fact") || strings.Contains(string(content), "[TestMethod") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔶️resolvePythonTestFiles resolves test files for Python commands.
func ResolvePythonTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for test_*.py and *_test.py files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".py") {
			base := filepath.Base(path)
			if strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔹️resolvePytestFiles resolves test files for pytest commands.
func ResolvePytestFiles(args []string, cwd string) []string {
	var files []string
	seen := map[string]bool{}
	addFile := func(path string) {
		if !seen[path] {
			seen[path] = true
			files = append(files, path)
		}
	}
	isPytestFile := func(path string) bool {
		base := filepath.Base(path)
		return strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") || strings.EqualFold(base, "conftest.py")
	}
	scanDir := func(root string) {
		_ = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if !info.IsDir() && strings.HasSuffix(path, ".py") && isPytestFile(path) {
				addFile(path)
			}
			return nil
		})
	}
	targets := make([]string, 0)
	for i := 0; i < len(args); i++ {
		arg := strings.TrimSpace(args[i])
		if arg == "" {
			continue
		}
		if strings.HasPrefix(arg, "-") {
			if (arg == "-k" || arg == "-m" || arg == "--maxfail" || arg == "-c") && i+1 < len(args) {
				i++
			}
			continue
		}
		targets = append(targets, arg)
	}
	if len(targets) == 0 {
		scanDir(cwd)
		return files
	}
	for _, target := range targets {
		targetPath := target
		if !filepath.IsAbs(targetPath) {
			targetPath = filepath.Join(cwd, targetPath)
		}
		info, err := os.Stat(targetPath)
		if err != nil {
			continue
		}
		if info.IsDir() {
			scanDir(targetPath)
			continue
		}
		if strings.HasSuffix(targetPath, ".py") && isPytestFile(targetPath) {
			addFile(targetPath)
		}
	}
	return files
}

// 🔸️findJSTestFiles resolves test files for JavaScript test runners.
func FindJSTestFiles(cwd string) []string {
	var files []string
	// Look for test files with common patterns
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && (strings.HasSuffix(path, ".js") || strings.HasSuffix(path, ".ts") || strings.HasSuffix(path, ".jsx") || strings.HasSuffix(path, ".tsx")) {
			base := filepath.Base(path)
			if strings.HasPrefix(base, "test.") || strings.HasSuffix(base, ".test.js") ||
				strings.HasSuffix(base, ".test.ts") || strings.HasSuffix(base, ".spec.js") ||
				strings.HasSuffix(base, ".spec.ts") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// #endregion 🖲️Missing Test Functions

// #region 💾️Missing Utility Functions

// 🧪️jsTestRunnerBins are JavaScript test runner binary names.
var JsTestRunnerBins = map[string]bool{
	"jest": true, "vitest": true, "mocha": true, "jasmine": true,
	"ava": true, "tape": true, "qunit": true, "karma": true,
}

// 📄️resolveRspecFiles resolves test files for RSpec commands.
func ResolveRspecFiles(args []string, cwd string) []string {
	var files []string
	// Look for _spec.rb files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, "_spec.rb") {
			files = append(files, path)
		}
		return nil
	})
	return files
}

// 🔺️resolveTestFilesFromCommand resolves test files from a command.
func ResolveTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return nil
	}
	if cwd == "" {
		cwd = workspace.GetRootDir()
	}
	testSeg, extractedCwd := ExtractTestSegmentFromCommand(command)
	if testSeg != "" {
		command = testSeg
	}
	if extractedCwd != "" {
		if filepath.IsAbs(extractedCwd) {
			cwd = extractedCwd
		} else {
			cwd = filepath.Join(cwd, extractedCwd)
		}
	}
	parts := strings.Fields(command)
	if len(parts) == 0 {
		return nil
	}

	bin := filepath.Base(parts[0])

	switch bin {
	case "go":
		return ResolveGoTestFiles(parts, cwd)
	case "cargo":
		return ResolveCargoTestFiles(parts, cwd)
	case "dotnet":
		return ResolveDotnetTestFiles(parts, cwd)
	case "python", "python3":
		return ResolvePythonTestFiles(parts, cwd)
	case "pytest", "py.test":
		return ResolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" {
			runner := filepath.Base(parts[2])
			if runner == "pytest" || PyTestRunnerBins[runner] {
				return ResolvePytestFiles(nil, cwd)
			}
		}
		return nil
	case "uvx":
		if len(parts) > 1 && PyTestRunnerBins[filepath.Base(parts[1])] {
			return ResolvePytestFiles(nil, cwd)
		}
		return nil
	case "rspec":
		return ResolveRspecFiles(parts, cwd)
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return FindJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 {
			runner := filepath.Base(parts[1])
			if JsTestRunnerBins[runner] {
				return FindJSTestFiles(cwd)
			}
			if runner == "phpunit" {
				return todospkg.ResolvePHPTestFiles(cwd)
			}
		}
		return nil
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return FindJSTestFiles(cwd)
		}
		return nil
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && filepath.Base(parts[2]) == "rspec" {
			return ResolveRspecFiles(parts[2:], cwd)
		}
		return nil
	default:
		if strings.HasPrefix(bin, "phpunit") || strings.HasSuffix(bin, "/phpunit") {
			return todospkg.ResolvePHPTestFiles(cwd)
		}
		return nil
	}
}

// #endregion 💾️Missing Utility Functions
