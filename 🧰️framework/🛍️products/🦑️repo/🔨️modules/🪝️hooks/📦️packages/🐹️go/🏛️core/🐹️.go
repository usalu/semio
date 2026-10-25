package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔖️HookError

// ⚠️HookError is every hook failure, carrying the message the Rust implementation formats verbatim.
type HookError struct {
	Message string `json:"message"`
}

// 🆕️NewHookError wraps a message.
func NewHookError(message string) *HookError { return &HookError{Message: message} }

// 🔤️Error renders the failure message.
func (e *HookError) Error() string { return e.Message }

// #endregion 🔖️HookError

// #region 🏷️HookKind

// 🔤️ValidateHookEvent resolves a neutral event slug, listing every valid slug when it does not.
func ValidateHookEvent(raw string) (model.HookEvent, error) {
	event, err := model.ValidateHookEvent(raw)
	if err != nil {
		return "", NewHookError(fmt.Sprintf("invalid hook event %q, valid events: %s", raw, strings.Join(HookEventSlugs(), ", ")))
	}
	return event, nil
}

// 📋️HookEventSlugs lists every valid neutral event slug, in declaration order.
func HookEventSlugs() []string {
	slugs := make([]string, 0, len(model.AllHookEvents))
	for _, event := range model.AllHookEvents {
		slugs = append(slugs, string(event))
	}
	return slugs
}

// #endregion 🏷️HookKind

// #region 🎯️HookContext

// 🎯️HookContext is everything one hook invocation knows: the resolved event, who sent it and the native payload.
type HookContext struct {
	Event      string            `json:"event"`
	Client     string            `json:"client"`
	Second     string            `json:"second"`
	RepoRoot   string            `json:"repoRoot"`
	ToolName   string            `json:"toolName,omitempty"`
	ToolArgs   string            `json:"toolArgs,omitempty"`
	FilePath   string            `json:"filePath,omitempty"`
	ParentInfo string            `json:"parentInfo,omitempty"`
	Extra      map[string]string `json:"extra,omitempty"`
	Input      json.RawMessage   `json:"input,omitempty"`
}

// 🆕️NewHookContext builds a context for one resolved event and client.
func NewHookContext(event model.HookEvent, client string) HookContext {
	return HookContext{Event: string(event), Client: client}
}

// 🕰️AtSecond sets the RFC 3339 second the caller stamped this invocation with.
func (c HookContext) AtSecond(second string) HookContext {
	c.Second = second
	return c
}

// 🧰️WithTool sets the tool name and raw argument string the agent client reported.
func (c HookContext) WithTool(toolName string, toolArgs string) HookContext {
	c.ToolName = toolName
	c.ToolArgs = toolArgs
	return c
}

// 👪️WithParent sets the parent hint (`subagent`, or a parent session id).
func (c HookContext) WithParent(parentInfo string) HookContext {
	c.ParentInfo = parentInfo
	return c
}

// 📨️WithInput attaches the native payload the agent client wrote to stdin.
func (c HookContext) WithInput(input json.RawMessage) HookContext {
	c.Input = input
	return c
}

// 🏠️AtRoot sets the repository root the hook runs against.
func (c HookContext) AtRoot(repoRoot string) HookContext {
	c.RepoRoot = repoRoot
	return c
}

// 📡️ResolvedEvent answers the resolved event, or false when the slug is not a neutral event.
func (c HookContext) ResolvedEvent() (model.HookEvent, bool) {
	for _, event := range model.AllHookEvents {
		if string(event) == c.Event {
			return event, true
		}
	}
	return "", false
}

// #endregion 🎯️HookContext

// #region 🔌️Ports

// 🌍️HookEnvironment is what a hook needs from the machine, with an inert default so a caller that
// wants pure normalisation supplies nothing.
type HookEnvironment interface {
	CurrentCheckpoint() string
	CheckpointMessage(repoRoot string) string
	FormatFile(path string)
}

// 🚫️InertEnvironment is the environment that does nothing and knows nothing.
type InertEnvironment struct{}

// 🔖️CurrentCheckpoint answers nothing.
func (InertEnvironment) CurrentCheckpoint() string { return "" }

// 📝️CheckpointMessage answers nothing.
func (InertEnvironment) CheckpointMessage(repoRoot string) string { return "" }

// 🖌️FormatFile does nothing.
func (InertEnvironment) FormatFile(path string) {}

// 💻️SystemEnvironment reads the checkpoint identity and message off the real machine.
type SystemEnvironment struct{}

// 🔖️CurrentCheckpoint answers the version control checkpoint the working tree is on.
func (SystemEnvironment) CurrentCheckpoint() string {
	return workspace.GetGitCheckpoint(workspace.GetRootDir())
}

// 📝️CheckpointMessage answers the checkpoint message being prepared under repoRoot.
func (SystemEnvironment) CheckpointMessage(repoRoot string) string {
	data, err := os.ReadFile(filepath.Join(repoRoot, ".git", "COMMIT_EDITMSG"))
	if err != nil {
		return ""
	}
	return strings.TrimSpace(string(data))
}

// 🖌️FormatFile formats a file a code edit just finished writing.
func (SystemEnvironment) FormatFile(path string) { statutes.RunFormatterForFile(path) }

// 🧪️TestFileResolver resolves a shell command into the test files it would run.
type TestFileResolver interface {
	TestFiles(command string, cwd string) []string
	LabIDs(filePaths []string) []string
}

// 🚫️InertTestFileResolver is the resolver that resolves nothing.
type InertTestFileResolver struct{}

// 📄️TestFiles answers nothing.
func (InertTestFileResolver) TestFiles(command string, cwd string) []string { return nil }

// 🧫️LabIDs answers nothing.
func (InertTestFileResolver) LabIDs(filePaths []string) []string { return nil }

// 🏃️WorkspaceTestFileResolver resolves test files through the repository's own test runner.
type WorkspaceTestFileResolver struct{}

// 📄️TestFiles answers the test files the command selects.
func (WorkspaceTestFileResolver) TestFiles(command string, cwd string) []string {
	return resolveFullTestFilesFromCommand(command, cwd)
}

// 🧫️LabIDs answers the lab ids of those files.
func (WorkspaceTestFileResolver) LabIDs(filePaths []string) []string {
	return resolveTestLabIDs(filePaths)
}

// #endregion 🔌️Ports

// #region 🛤️Paths

// 📄️BaseName answers the last path element, splitting on both separators.
func BaseName(path string) string {
	trimmed := strings.TrimRight(path, "/\\")
	if trimmed == "" {
		if path == "" {
			return "."
		}
		return "/"
	}
	if index := strings.LastIndexAny(trimmed, "/\\"); index >= 0 {
		return trimmed[index+1:]
	}
	return trimmed
}

// 🛤️NormalizeHookPath normalises a path the way hook payloads carry it: forward separators, no `./` prefix.
func NormalizeHookPath(path string) string {
	path = strings.TrimSpace(path)
	if path == "" {
		return ""
	}
	normalized := strings.ReplaceAll(path, "\\", "/")
	return strings.TrimPrefix(normalized, "./")
}

// #endregion 🛤️Paths
