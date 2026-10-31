// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌿️git carries command execution, git author and checkpoint queries, ignored set lookups and checkpoint file reads.

// #endregion 🧲️Header

package workspace

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	sync "sync"
	unicode "unicode"
	utf8 "unicode/utf8"
)

// #region 🌿️Git

// ▪️ExecCommand MUST complete the operation and return consistent results.
func ExecCommand(command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	return ExecCommandContext(context.Background(), command, args, cwd)
}

// ▫️ExecCommandContext runs a child process with explicit cancellation and deadline propagation.
func ExecCommandContext(ctx context.Context, command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	if cwd == "" {
		cwd = GetRootDir()
	}
	cmd := ToolCommandContext(ctx, command, args...)
	cmd.Dir = cwd
	var stdoutBuf, stderrBuf strings.Builder
	cmd.Stdout = &stdoutBuf
	cmd.Stderr = &stderrBuf
	err := cmd.Run()
	exitCode = 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = 1
		}
	}
	return stdoutBuf.String(), stderrBuf.String(), exitCode
}

// 🪪️registeredTool is the part of one tool registry row the Go side reads: its id, the executable names it owns, where to look for it and the environment it gets.
// 📣️registeredInvocation maps one owned secondary name to the same primary executable.
type registeredInvocation struct {
	Command string `json:"command"`
	Prefix []string `json:"prefix"`
}

// 🔒️UnmarshalJSON keeps the invocation row closed without constraining unrelated registry fields.
func (invocation *registeredInvocation) UnmarshalJSON(data []byte) error {
	type row registeredInvocation
	var value row
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&value); err != nil { return err }
	*invocation = registeredInvocation(value)
	return nil
}

// 📋️registeredInvocations distinguishes an omitted optional policy from invalid null DATA.
type registeredInvocations []registeredInvocation

// 🔒️UnmarshalJSON requires an actual array when invocation DATA is present.
func (invocations *registeredInvocations) UnmarshalJSON(data []byte) error {
	if bytes.Equal(bytes.TrimSpace(data), []byte("null")) { return fmt.Errorf("invocations must be an array") }
	var rows []registeredInvocation
	if err := json.Unmarshal(data, &rows); err != nil { return err }
	*invocations = rows
	return nil
}

type registeredTool struct {
	ID       string   `json:"id"`
	Commands []string `json:"commands"`
	Invocations registeredInvocations `json:"invocations"`
	Locate   []struct {
		Kind        string   `json:"kind"`
		Name        string   `json:"name"`
		Directories []string `json:"directories"`
	} `json:"locate"`
	Environment struct {
		Deny []string          `json:"deny"`
		Set  map[string]string `json:"set"`
	} `json:"environment"`
}

// 📖️toolRegistryDocument is the part of the tool registry (🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json) the Go side reads; the document is the schema-first contract the TypeScript, Rust and Go adapters share.
type toolRegistryDocument struct {
	Environment struct {
		Deny []string `json:"deny"`
	} `json:"environment"`
	Tools []registeredTool `json:"tools"`
}

var (
	toolRegistryOnce  sync.Once
	toolRegistryValue *toolRegistryDocument
)

// 🪢️loadToolRegistry reads the registry once from the nearest ancestor of the directory that holds MODULE.bazel; a missing or unreadable registry leaves every command unchanged.
func loadToolRegistry(cwd string) *toolRegistryDocument {
	toolRegistryOnce.Do(func() {
		if root := os.Getenv("SEMIO_WORKSPACE_ROOT"); root != "" {
			cwd = root
		} else if cwd == "" {
			cwd, _ = os.Getwd()
		}
		for directory := cwd; directory != "" && directory != filepath.Dir(directory); directory = filepath.Dir(directory) {
			if _, err := os.Stat(filepath.Join(directory, "MODULE.bazel")); err != nil {
				continue
			}
			text, err := os.ReadFile(filepath.Join(directory, "🧰️framework", "🔨️modules", "🏃️process", "🧰️tools", "🔣️.json"))
			if err != nil {
				return
			}
			var document toolRegistryDocument
			if json.Unmarshal(text, &document) == nil && validRegisteredInvocations(document) {
				toolRegistryValue = &document
			}
			return
		}
	})
	return toolRegistryValue
}

// 🧬️validRegisteredInvocations checks same-owner secondary declarations before routing.
func validRegisteredInvocations(document toolRegistryDocument) bool {
	for _, tool := range document.Tools {
		seen := map[string]bool{}
		if len(tool.Invocations) > 16 { return false }
		for _, invocation := range tool.Invocations {
			owned := false
			for _, command := range tool.Commands { if command == invocation.Command { owned = true } }
			if !owned || len(tool.Commands) == 0 || invocation.Command == tool.Commands[0] || seen[invocation.Command] || len(invocation.Prefix) == 0 || len(invocation.Prefix) > 16 { return false }
			for _, argument := range invocation.Prefix { if len([]rune(argument)) == 0 || len([]rune(argument)) > 256 || strings.IndexFunc(argument, func(value rune) bool { return value < 32 || value == 127 }) >= 0 { return false } }
			seen[invocation.Command] = true
		}
	}
	return true
}

// 🪟️registeredCommandName applies the declared bare executable suffix normalization.
func registeredCommandName(command string) string { extension := filepath.Ext(command); if strings.EqualFold(extension, ".exe") || strings.EqualFold(extension, ".cmd") || strings.EqualFold(extension, ".bat") { return strings.TrimSuffix(command, extension) }; return command }

// 🧭️registeredToolInvocation keeps unmapped siblings and clones declared primary prefixes.
func registeredToolInvocation(tool registeredTool, command string) (string, []string) {
	command = registeredCommandName(command)
	for _, invocation := range tool.Invocations { if invocation.Command == command { return tool.Commands[0], append([]string{}, invocation.Prefix...) } }
	return command, nil
}

// 🧷️toolNameMatches reports whether an environment variable name matches a registry pattern whose * stands for any characters; Windows names ignore case.
func toolNameMatches(pattern string, name string) bool {
	if runtime.GOOS == "windows" {
		pattern, name = strings.ToUpper(pattern), strings.ToUpper(name)
	}
	matched, err := filepath.Match(pattern, name)
	return err == nil && matched
}

// 🪛️resolveRegisteredTool finds the executable of a registered tool: the SEMIO_TOOL_<ID> override, then PATH, then the directories of the tool's path locator; empty when the host does not have it.
func resolveRegisteredTool(tool registeredTool, command string) string {
	if override := os.Getenv("SEMIO_TOOL_" + strings.ToUpper(strings.ReplaceAll(tool.ID, "-", "_"))); len(tool.Commands) > 0 && command == tool.Commands[0] && override != "" {
		if _, err := os.Stat(override); err == nil {
			return override
		}
	}
	if found, err := exec.LookPath(command); err == nil {
		return found
	}
	for _, locator := range tool.Locate {
		for _, directory := range locator.Directories {
			if strings.HasPrefix(directory, "~/") {
				if home, err := os.UserHomeDir(); err == nil {
					directory = filepath.Join(home, directory[2:])
				}
			}
			candidate := filepath.Join(directory, command)
			if runtime.GOOS == "windows" {
				candidate += ".exe"
			}
			if _, err := os.Stat(candidate); err == nil {
				return candidate
			}
		}
	}
	return ""
}

// 🧶️registeredToolEnvironment is the process environment minus the variables the registry denies, plus the tool's fixed values and the run marker.
func registeredToolEnvironment(registry *toolRegistryDocument, tool registeredTool) []string {
	environment := []string{}
	for _, entry := range os.Environ() {
		name, _, _ := strings.Cut(entry, "=")
		denied := false
		for _, pattern := range append(append([]string{}, registry.Environment.Deny...), tool.Environment.Deny...) {
			denied = denied || toolNameMatches(pattern, name)
		}
		if _, fixed := tool.Environment.Set[name]; !denied && !fixed && name != "SEMIO_TOOL_RUN" {
			environment = append(environment, entry)
		}
	}
	for name, value := range tool.Environment.Set {
		environment = append(environment, name+"="+value)
	}
	return append(environment, "SEMIO_TOOL_RUN="+tool.ID)
}

// 🧱️routeRegisteredTool resolves a bare command name that belongs to a registered tool to its executable and the registry's sanitized environment. Explicit paths, unknown commands, a host without the tool, an unreadable registry and SEMIO_TOOL_GUARD=off answer the command unchanged and a nil environment.
func routeRegisteredTool(command string, cwd string) (string, []string, []string) {
	if strings.ContainsAny(command, "/\\") || os.Getenv("SEMIO_TOOL_GUARD") == "off" {
		return command, nil, nil
	}
	registry := loadToolRegistry(cwd)
	if registry == nil {
		return command, nil, nil
	}
	normalized := registeredCommandName(command)
	for _, tool := range registry.Tools {
		for _, name := range tool.Commands {
			if name != normalized {
				continue
			}
			selected, prefix := registeredToolInvocation(tool, normalized)
			if program := resolveRegisteredTool(tool, selected); program != "" {
				return program, prefix, registeredToolEnvironment(registry, tool)
			}
			return command, nil, nil
		}
	}
	return command, nil, nil
}

// 🛑️ToolError says why a registered tool could not be run at all; a tool that ran and failed answers its status in the result instead.
type ToolError struct {
	Tool   string
	Reason string
}

// 🗣️Error renders the failure.
func (e *ToolError) Error() string { return "tool " + e.Tool + ": " + e.Reason }

// 🧵️ToolRunOptions tells one registered tool run where to start, what to add to its environment (explicit values win over the registry's) and what to feed it.
type ToolRunOptions struct {
	Context context.Context
	Dir     string
	Env     []string
	Stdin   io.Reader
}

// 🏁️ToolRunResult is what a finished tool run left behind: the exit status and both output streams.
type ToolRunResult struct {
	Status int
	Stdout string
	Stderr string
}

// 🏃️RunTool runs a registered tool by its registry id with the registry's executable and sanitized environment and answers its status and output; an error means the tool could not be started. It is the Go twin of runToolV1 in the TypeScript tools module and of run_tool in the Rust workspace crate.
func RunTool(toolID string, args []string, options ToolRunOptions) (ToolRunResult, error) {
	registry := loadToolRegistry(options.Dir)
	command := toolID
	if registry != nil {
		known := false
		for _, tool := range registry.Tools {
			if tool.ID == toolID {
				known, command = true, tool.Commands[0]
			}
		}
		if !known {
			return ToolRunResult{}, &ToolError{Tool: toolID, Reason: "not registered"}
		}
	}
	ctx := options.Context
	if ctx == nil {
		ctx = context.Background()
	}
	cmd := ToolCommandContext(ctx, command, args...)
	cmd.Dir = options.Dir
	if refusal, ok := cmd.Err.(*ToolCapabilityError); ok { return ToolRunResult{}, refusal }
	cmd.Env = append(append([]string{}, cmd.Environ()...), options.Env...)
	cmd.Stdin = options.Stdin
	var stdout, stderr bytes.Buffer
	cmd.Stdout, cmd.Stderr = &stdout, &stderr
	result := ToolRunResult{}
	if err := cmd.Run(); err != nil {
		exitErr, ok := err.(*exec.ExitError)
		if !ok {
			return result, &ToolError{Tool: toolID, Reason: err.Error()}
		}
		result.Status = exitErr.ExitCode()
	}
	result.Stdout, result.Stderr = stdout.String(), stderr.String()
	return result, nil
}

// 🧰️ToolCommand is exec.Command for a registered tool: the program is the registry's resolution of the command name and the environment is the registry's sanitized one; every other command is exec.Command unchanged.
func ToolCommand(command string, args ...string) *exec.Cmd {
	return ToolCommandContext(context.Background(), command, args...)
}

// 🪟️ToolCapabilityError refuses unsupported registered process semantics before launch.
type ToolCapabilityError struct { Capability string; Reason string }

// 🚧️Error reports the closed capability and reason without echoing caller arguments.
func (refusal *ToolCapabilityError) Error() string { return "Tool capability " + refusal.Capability + ": " + refusal.Reason }

// 🔐️registeredBatchFragment implements only the declared literal batch argument subset.
func registeredBatchFragment(program string, args []string) (string, error) {
 refusal := func(reason string) (string, error) { return "", &ToolCapabilityError{Capability: "windows-batch-arguments", Reason: reason} }
 if strings.TrimRight(program, ". ") != program { return refusal("target-alias") }
 if !(len(program) > 2 && program[1] == ':' && (program[2] == '/' || program[2] == '\\')) && !strings.HasPrefix(program, "\\\\") { return refusal("invalid-target") }
 units := 0
 for _, value := range append([]string{program}, args...) {
  if !utf8.ValidString(value) { return refusal("utf8") }
  for _, point := range value {
   if point == '"' { return refusal("quote") }
   if point == '%' { return refusal("percent") }
   if unicode.IsControl(point) { return refusal("control") }
   units++
   if point > 0xffff { units++ }
   if units > 8191 { return refusal("line-bound") }
  }
 }
 escape := func(value string) string {
  var result strings.Builder
  for _, point := range value { if strings.ContainsRune("()[]%!^\"`<>&|;, *?", point) { result.WriteByte('^') }; result.WriteRune(point) }
  return result.String()
 }
 var result strings.Builder
 result.WriteByte('"')
 result.WriteString(escape(program))
 for _, argument := range args {
  tail := len(argument) - len(strings.TrimRight(argument, "\\"))
  result.WriteByte(' ')
  result.WriteString(escape("\"" + argument + strings.Repeat("\\", tail) + "\""))
 }
 result.WriteByte('"')
 line := result.String()
 units = 0
 for _, point := range "cmd.exe /d /v:off /s /c " + line { units++; if point > 0xffff { units++ } }
 if units > 8191 { return refusal("line-bound") }
 return line, nil
}

// 🧭️registeredBatchTarget recognizes Windows normalized batch aliases before refusing them.
func registeredBatchTarget(program string) bool { extension := strings.ToLower(filepath.Ext(strings.TrimRight(program, ". "))); return extension == ".cmd" || extension == ".bat" }

// 🪟️registeredToolCommandArguments keeps ordinary executable argv and owns batch refusal.
func registeredToolCommandArguments(program string, prefix []string, args []string, windows bool, shell string) (string, []string, error) {
 arguments := append(append([]string{}, prefix...), args...)
 if windows && registeredBatchTarget(program) {
  line, refusal := registeredBatchFragment(program, arguments)
  if refusal != nil { return program, arguments, refusal }
  if shell == "" { shell = "cmd.exe" }
  return shell, []string{"/d", "/v:off", "/s", "/c", line}, nil
 }
 return program, arguments, nil
}

// 🪡️ToolCommandContext is exec.CommandContext for a registered tool.
func ToolCommandContext(ctx context.Context, command string, args ...string) *exec.Cmd {
	program, prefix, environment := routeRegisteredTool(command, "")
	batch := runtime.GOOS == "windows" && environment != nil && registeredBatchTarget(program)
	program, arguments, refusal := registeredToolCommandArguments(program, prefix, args, batch, os.Getenv("COMSPEC"))
	cmd := exec.CommandContext(ctx, program, arguments...)
	if refusal != nil { cmd.Err = refusal }
	if refusal == nil && batch { setRegisteredBatchCommandLine(cmd, arguments[4]) }
	if environment != nil {
		cmd.Env = environment
	}
	return cmd
}

// ✍️GetGitAuthor MUST return the stored value without modification.
// 🔐️GetGitAuthor returns the git author of the value.
func GetGitAuthor(rootDir string) string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, rootDir)
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, rootDir)
	name = strings.TrimSpace(name)
	email = strings.TrimSpace(email)
	if email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	return name
}

// 💾️GetGitCheckpoint MUST return the stored value without modification.
// ✔️GetGitCheckpoint returns the git checkpoint of the value.
func GetGitCheckpoint(rootDir string) string {
	sha, _, _ := ExecCommand("git", []string{"rev-parse", "HEAD"}, rootDir)
	return strings.TrimSpace(sha)
}

// 🗃️GetGitIgnoredSet MUST return the stored value without modification.

// 🧭️gitTopLevelCache remembers, per root directory, whether git resolves that directory as its own
// repository top level.
var gitTopLevelCache sync.Map

// 🧭️RootDirIsGitTopLevel MUST return true only when git treats the configured root as the top level
// of its own repository. A root that is not one — a materialised fixture tree, a temporary
// workspace — would otherwise make `git check-ignore` answer for the enclosing repository and
// silently drop every file below it.
func RootDirIsGitTopLevel() bool {
	root := GetRootDir()
	if strings.TrimSpace(root) == "" {
		return false
	}
	if cached, ok := gitTopLevelCache.Load(root); ok {
		return cached.(bool)
	}
	stdout, _, exitCode := ExecCommand("git", []string{"rev-parse", "--show-toplevel"}, root)
	verdict := false
	if exitCode == 0 {
		verdict = NormalizePath(CanonicalPath(strings.TrimSpace(stdout))) == NormalizePath(CanonicalPath(root))
	}
	gitTopLevelCache.Store(root, verdict)
	return verdict
}

// ▫️GetGitIgnoredSet returns the git ignored set of the value.
func GetGitIgnoredSet(rootDir string, paths []string) map[string]bool {
	if len(paths) == 0 {
		return make(map[string]bool)
	}
	top, _, code := ExecCommand("git", []string{"rev-parse", "--show-toplevel"}, rootDir)
	if code != 0 || NormalizePath(CanonicalPath(strings.TrimSpace(top))) != NormalizePath(CanonicalPath(rootDir)) { return make(map[string]bool) }
	args := append([]string{"-c", "core.quotepath=off", "check-ignore"}, paths...)
	stdout, _, _ := ExecCommand("git", args, rootDir)
	ignored := make(map[string]bool)
	for _, line := range strings.Split(stdout, "\n") {
		line = strings.TrimSpace(line)
		if line != "" {
			ignored[NormalizePath(line)] = true
		}
	}
	return ignored
}

// 💾️CountLinesAtCheckpoint MUST complete the operation and return consistent results.
func CountLinesAtCheckpoint(checkpoint, filePath string) int {
	stdout, _, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return 0
	}
	return CountLines(stdout)
}

// ❌️ReadTextFileAtCheckpoint MUST return the text file at checkpoint content or an error if unavailable.
// ✔️ReadTextFileAtCheckpoint reads and returns text file at checkpoint from the source.
func ReadTextFileAtCheckpoint(checkpoint, filePath string) (string, error) {
	if checkpoint == "" {
		return ReadTextFile(filepath.Join(GetRootDir(), filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

// 🔷️splitCommandSegments splits a command into segments.
func SplitCommandSegments(cmd string) []string {
	cmd = strings.TrimSpace(cmd)
	if cmd == "" {
		return nil
	}
	var segments []string
	var current strings.Builder
	quote := rune(0)
	flush := func() {
		segment := strings.TrimSpace(current.String())
		if segment != "" {
			segments = append(segments, segment)
		}
		current.Reset()
	}
	for i := 0; i < len(cmd); i++ {
		ch := rune(cmd[i])
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
		if ch == ';' {
			flush()
			continue
		}
		if ch == '&' && i+1 < len(cmd) && cmd[i+1] == '&' {
			flush()
			i++
			continue
		}
		if ch == '|' {
			flush()
			if i+1 < len(cmd) && cmd[i+1] == '|' {
				i++
			}
			continue
		}
		current.WriteRune(ch)
	}
	flush()
	return segments
}

// #endregion 🌿️Git
