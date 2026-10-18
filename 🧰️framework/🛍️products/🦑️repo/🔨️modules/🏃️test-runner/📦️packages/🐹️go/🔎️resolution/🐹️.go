// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔎️resolution carries command classification and the resolution of test files from a command.

// #endregion 🧲️Header

package testrunner

import (
	strings "strings"
)

// #region 🔎️Resolution

// 🧰️PythonTestRunnerBins are the Python test runner executables the resolver recognises.
var PythonTestRunnerBins = []string{"pytest", "py.test", "nosetests", "nose2"}

// 🧰️JavaScriptTestRunnerBins are the JavaScript test runner executables the resolver recognises.
var JavaScriptTestRunnerBins = []string{"jest", "vitest", "mocha", "jasmine", "ava", "tape", "qunit", "karma"}

// 🏷️CommandKind is the tool kind classification restricted to the four verdicts the resolver needs.
type CommandKind string

// 🧪️CommandKindTest is a test command.
const CommandKindTest CommandKind = "test"

// 🔎️CommandKindCodeSearch is a read-only inspection command.
const CommandKindCodeSearch CommandKind = "code-search"

// ✏️CommandKindCodeEdit is a command that writes files.
const CommandKindCodeEdit CommandKind = "code-edit"

// 🖥️CommandKindTerminal is anything else.
const CommandKindTerminal CommandKind = "terminal"

// ✂️SplitCommandSegments splits on `;`, `&&`, `|` and `||` outside quotes.
func SplitCommandSegments(command string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return []string{}
	}
	segments := []string{}
	current := strings.Builder{}
	quote := rune(0)
	runes := []rune(command)
	for index := 0; index < len(runes); index++ {
		value := runes[index]
		if quote != 0 {
			current.WriteRune(value)
			if value == quote {
				quote = 0
			}
			continue
		}
		if value == '\'' || value == '"' {
			quote = value
			current.WriteRune(value)
			continue
		}
		if value == ';' {
			segments = flushSegment(segments, &current)
			continue
		}
		if value == '&' && index+1 < len(runes) && runes[index+1] == '&' {
			segments = flushSegment(segments, &current)
			index++
			continue
		}
		if value == '|' {
			segments = flushSegment(segments, &current)
			if index+1 < len(runes) && runes[index+1] == '|' {
				index++
			}
			continue
		}
		current.WriteRune(value)
	}
	return flushSegment(segments, &current)
}

// 🏷️ClassifyCommandKind answers which of the four kinds a shell command is.
func ClassifyCommandKind(command string) CommandKind {
	trimmed := strings.TrimSpace(command)
	if trimmed == "" {
		return CommandKindTerminal
	}
	if segments := SplitCommandSegments(trimmed); len(segments) > 1 {
		best := CommandKindTerminal
		for _, segment := range segments {
			kind := ClassifyCommandKind(segment)
			if kind == CommandKindTest || kind == CommandKindCodeEdit {
				return kind
			}
			if kind == CommandKindCodeSearch {
				best = CommandKindCodeSearch
			}
		}
		if best != CommandKindTerminal {
			return best
		}
	}
	fields := strings.Fields(trimmed)
	if len(fields) == 0 {
		return CommandKindTerminal
	}
	base := BaseName(fields[0])
	if base == "test" || base == "[" {
		return CommandKindCodeSearch
	}
	if strings.HasPrefix(base, "phpunit") || containsWord(commandTestBins, base) {
		return CommandKindTest
	}
	if strings.Contains(trimmed, "cargo nextest") {
		return CommandKindTest
	}
	for _, candidate := range commandTestNeedles {
		if strings.Contains(trimmed, candidate) {
			return CommandKindTest
		}
	}
	for _, prefix := range commandTestPrefixes {
		if strings.HasPrefix(trimmed, prefix) {
			return CommandKindTest
		}
	}
	if strings.Contains(trimmed, "pytest") || strings.Contains(trimmed, "unittest") || strings.Contains(trimmed, "phpunit") {
		return CommandKindTest
	}
	if containsWord(commandSearchBins, base) {
		return CommandKindCodeSearch
	}
	if containsWord(commandSearchTailBins, base) && !((base == "sed" || base == "awk" || base == "gawk") && strings.Contains(trimmed, "-i")) {
		return CommandKindCodeSearch
	}
	if containsWord(commandEditBins, base) {
		return CommandKindCodeEdit
	}
	if (base == "sed" || base == "awk" || base == "gawk" || base == "mawk" || base == "nawk") && strings.Contains(trimmed, "-i") {
		return CommandKindCodeEdit
	}
	return CommandKindTerminal
}

// 🧲️ExtractTestSegment answers the test segment of a command and the `cd` that precedes it. It is the
// twin of the Rust `extract_test_segment_from_command`, renamed because the 🚚️Split region still
// exports the pre-split `ExtractTestSegmentFromCommand`.
func ExtractTestSegment(command string) (string, string) {
	command = strings.TrimSpace(command)
	if command == "" {
		return "", ""
	}
	segments := SplitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	hasComposite := len(segments) > 1
	cwd := ""
	for _, segment := range segments {
		segment = strings.TrimSpace(segment)
		if segment == "" {
			continue
		}
		if rest, ok := strings.CutPrefix(segment, "cd "); ok {
			if fields := strings.Fields(rest); len(fields) > 0 {
				cwd = fields[0]
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
		bin := BaseName(fields[0])
		if IsTestCommandSegment(segment, bin) {
			return TrimPipelineTail(segment), cwd
		}
		if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun" || bin == "uv") && IsTestCommandSegment(segment, "") {
			return TrimPipelineTail(segment), cwd
		}
	}
	if hasComposite {
		return "", cwd
	}
	return "", ""
}

// 🧪️IsTestCommandSegment says whether one segment of a shell command runs tests.
func IsTestCommandSegment(segment string, bin string) bool {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return false
	}
	fields := strings.Fields(trimmed)
	if bin == "" && len(fields) > 0 {
		bin = BaseName(fields[0])
	}
	if bin == "" {
		return false
	}
	if ClassifyCommandKind(trimmed) == CommandKindTest {
		return true
	}
	if len(fields) < 2 {
		return false
	}
	if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun") && ClassifyCommandKind(strings.Join(fields[1:], " ")) == CommandKindTest {
		return true
	}
	if bin == "uv" {
		if fields[1] == "run" && len(fields) > 2 && ClassifyCommandKind(strings.Join(fields[2:], " ")) == CommandKindTest {
			return true
		}
		if ClassifyCommandKind(strings.Join(fields[1:], " ")) == CommandKindTest {
			return true
		}
	}
	return false
}

// ✂️TrimPipelineTail keeps everything up to the first unquoted `|`.
func TrimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	current := strings.Builder{}
	quote := rune(0)
	for _, value := range trimmed {
		if quote != 0 {
			current.WriteRune(value)
			if value == quote {
				quote = 0
			}
			continue
		}
		if value == '\'' || value == '"' {
			quote = value
			current.WriteRune(value)
			continue
		}
		if value == '|' {
			break
		}
		current.WriteRune(value)
	}
	return strings.TrimSpace(current.String())
}

// 🔎️ResolveTestFilesFromCommand answers the test files a shell command would exercise.
func (snapshot *FilesystemSnapshot) ResolveTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return []string{}
	}
	if cwd == "" {
		cwd = snapshot.Root
	}
	segment, extractedCwd := ExtractTestSegment(command)
	if segment != "" {
		command = segment
	}
	if extractedCwd != "" {
		if IsAbsolutePath(extractedCwd) {
			cwd = CleanPath(extractedCwd)
		} else {
			cwd = JoinPath(cwd, extractedCwd)
		}
	}
	parts := strings.Fields(command)
	if len(parts) == 0 {
		return []string{}
	}
	switch BaseName(parts[0]) {
	case "go":
		return snapshot.ResolveGoTestFiles(parts, cwd)
	case "cargo":
		return snapshot.ResolveCargoTestFiles(cwd)
	case "dotnet":
		return snapshot.ResolveDotnetTestFiles(cwd)
	case "python", "python3":
		return snapshot.ResolvePythonTestFiles(cwd)
	case "pytest", "py.test":
		return snapshot.ResolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" && containsWord(PythonTestRunnerBins, BaseName(parts[2])) {
			return snapshot.ResolvePytestFiles(nil, cwd)
		}
	case "uvx":
		if len(parts) > 1 && containsWord(PythonTestRunnerBins, BaseName(parts[1])) {
			return snapshot.ResolvePytestFiles(nil, cwd)
		}
	case "rspec":
		return snapshot.ResolveRspecFiles(cwd)
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return snapshot.FindJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 && containsWord(JavaScriptTestRunnerBins, BaseName(parts[1])) {
			return snapshot.FindJSTestFiles(cwd)
		}
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return snapshot.FindJSTestFiles(cwd)
		}
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && BaseName(parts[2]) == "rspec" {
			return snapshot.ResolveRspecFiles(cwd)
		}
	}
	return []string{}
}

// 🐹️ResolveGoTestFiles answers the package targets after `go test`, with `/...` meaning recursive.
func (snapshot *FilesystemSnapshot) ResolveGoTestFiles(args []string, cwd string) []string {
	targets := []string{}
	for index := 2; index < len(args); index++ {
		if strings.TrimSpace(args[index]) == "" || strings.HasPrefix(args[index], "-") {
			continue
		}
		targets = append(targets, args[index])
	}
	files := []string{}
	if len(targets) == 0 {
		return appendSuffixed(files, snapshot.WalkFiles(cwd, true), "_test.go")
	}
	for _, target := range targets {
		recursive := strings.HasSuffix(target, "/...")
		stripped := strings.TrimSuffix(target, "/...")
		targetPath := cwd
		if stripped != "." && stripped != "./" && stripped != "" {
			targetPath = JoinPath(cwd, stripped)
			if IsAbsolutePath(stripped) {
				targetPath = CleanPath(stripped)
			}
		}
		if snapshot.DirectoryExists(targetPath) {
			files = appendSuffixed(files, snapshot.WalkFiles(targetPath, recursive), "_test.go")
		} else if snapshot.FileExists(targetPath) && strings.HasSuffix(targetPath, "_test.go") && !containsWord(files, targetPath) {
			files = append(files, targetPath)
		}
	}
	return files
}

// 🦀️ResolveCargoTestFiles answers the `.rs` files carrying a test attribute.
func (snapshot *FilesystemSnapshot) ResolveCargoTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".rs") {
			continue
		}
		if text, ok := snapshot.ReadText(path); ok && (strings.Contains(text, "#[test") || strings.Contains(text, "#[cfg(test)")) {
			files = append(files, path)
		}
	}
	return files
}

// 🔷️ResolveDotnetTestFiles answers the `.cs` files carrying a test attribute.
func (snapshot *FilesystemSnapshot) ResolveDotnetTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".cs") {
			continue
		}
		if text, ok := snapshot.ReadText(path); ok && (strings.Contains(text, "[Test") || strings.Contains(text, "[Fact") || strings.Contains(text, "[TestMethod")) {
			files = append(files, path)
		}
	}
	return files
}

// 🐍️ResolvePythonTestFiles answers `test_*.py` and `*_test.py`.
func (snapshot *FilesystemSnapshot) ResolvePythonTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		base := BaseName(path)
		if strings.HasSuffix(path, ".py") && (strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.")) {
			files = append(files, path)
		}
	}
	return files
}

// 🐍️ResolvePytestFiles answers pytest naming plus `conftest.py`, honouring positional targets.
func (snapshot *FilesystemSnapshot) ResolvePytestFiles(args []string, cwd string) []string {
	targets := []string{}
	for index := 0; index < len(args); index++ {
		arg := strings.TrimSpace(args[index])
		if arg == "" {
			continue
		}
		if strings.HasPrefix(arg, "-") {
			if (arg == "-k" || arg == "-m" || arg == "--maxfail" || arg == "-c") && index+1 < len(args) {
				index++
			}
			continue
		}
		targets = append(targets, arg)
	}
	files := []string{}
	if len(targets) == 0 {
		return snapshot.scanPytestFiles(cwd, files)
	}
	for _, target := range targets {
		targetPath := JoinPath(cwd, target)
		if IsAbsolutePath(target) {
			targetPath = CleanPath(target)
		}
		if snapshot.DirectoryExists(targetPath) {
			files = snapshot.scanPytestFiles(targetPath, files)
		} else if snapshot.FileExists(targetPath) && strings.HasSuffix(targetPath, ".py") && isPytestFile(targetPath) && !containsWord(files, targetPath) {
			files = append(files, targetPath)
		}
	}
	return files
}

// 💎️ResolveRspecFiles answers the `_spec.rb` files below the working directory.
func (snapshot *FilesystemSnapshot) ResolveRspecFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if strings.HasSuffix(path, "_spec.rb") {
			files = append(files, path)
		}
	}
	return files
}

// 🟦️FindJSTestFiles answers `test.*`, `*.test.{js,ts}` and `*.spec.{js,ts}`.
func (snapshot *FilesystemSnapshot) FindJSTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".js") && !strings.HasSuffix(path, ".ts") && !strings.HasSuffix(path, ".jsx") && !strings.HasSuffix(path, ".tsx") {
			continue
		}
		base := BaseName(path)
		if strings.HasPrefix(base, "test.") || strings.HasSuffix(base, ".test.js") || strings.HasSuffix(base, ".test.ts") || strings.HasSuffix(base, ".spec.js") || strings.HasSuffix(base, ".spec.ts") {
			files = append(files, path)
		}
	}
	return files
}

func (snapshot *FilesystemSnapshot) scanPytestFiles(root string, files []string) []string {
	for _, path := range snapshot.WalkFiles(root, true) {
		if strings.HasSuffix(path, ".py") && isPytestFile(path) && !containsWord(files, path) {
			files = append(files, path)
		}
	}
	return files
}

func isPytestFile(path string) bool {
	base := BaseName(path)
	return strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") || strings.EqualFold(base, "conftest.py")
}

func appendSuffixed(files []string, found []string, suffix string) []string {
	for _, path := range found {
		if strings.HasSuffix(path, suffix) && !containsWord(files, path) {
			files = append(files, path)
		}
	}
	return files
}

func containsWord(values []string, wanted string) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}

func flushSegment(segments []string, current *strings.Builder) []string {
	segment := strings.TrimSpace(current.String())
	current.Reset()
	if segment == "" {
		return segments
	}
	return append(segments, segment)
}

var commandTestBins = []string{"rspec", "pytest", "py.test", "jest", "vitest", "mocha", "gradle", "mvn", "bundle"}

var commandTestNeedles = []string{"pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"}

var commandTestPrefixes = []string{
	"npm test", "npm run test", "pnpm test", "yarn test", "bun test", "go test", "cargo test", "cargo nextest", "make test", "make check", "dotnet test", "swift test", "dart test",
	"flutter test", "mix test", "mvn test", "mvn verify", "gradle test", "./gradlew test", "gradlew test", "cabal test", "stack test", "lein test", "sbt test", "bundle exec rspec",
	"tox", "rspec ",
}

var commandSearchBins = []string{
	"grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind", "locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less", "more", "head", "tail",
	"wc", "file", "stat", "du", "which", "whereis", "type", "command", "hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath", "basename", "dirname", "jq",
	"yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column", "rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env",
}

var commandSearchTailBins = []string{
	"printenv", "set", "export", "pwd", "id", "whoami", "hostname", "uname", "date", "uptime", "free", "df", "ps", "top", "htop", "lsof", "netstat", "ss", "test", "sed", "awk", "gawk",
}

var commandEditBins = []string{
	"rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown", "chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip", "gunzip", "bzip2",
	"bunzip2", "xz", "unxz", "zstd",
}

// #endregion 🔎️Resolution
