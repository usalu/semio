// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪝️hooks is the hooks domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package hooks

import (
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	io "io"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	statutes "github.com/usalu/semio/repo/statutes"
	testrunner "github.com/usalu/semio/repo/testrunner"
	tickets "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
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

// 🧰️WithTool sets the tool name and raw argument string the IDE reported.
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

// 📨️WithInput attaches the native payload the IDE wrote to stdin.
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
func (SystemEnvironment) CurrentCheckpoint() string { return workspace.GetGitCheckpoint() }

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

// #region 🧭️PayloadReading

// 🔍️DecodeHookInput answers the payload object, when the input is a JSON object at all.
func DecodeHookInput(input json.RawMessage) map[string]interface{} {
	return decodeHookInputMap(input)
}

// 🪆️FindNestedString answers the first non-blank string reachable under any of keys, searched
// breadth-first at each level then depth-first into every child, visiting children in key order.
func FindNestedString(input json.RawMessage, keys ...string) string {
	if len(input) == 0 {
		return ""
	}
	var decoded interface{}
	if err := json.Unmarshal(input, &decoded); err != nil {
		return ""
	}
	return findNestedOrdered(decoded, keys)
}

func findNestedOrdered(value interface{}, keys []string) string {
	switch typed := value.(type) {
	case map[string]interface{}:
		for _, key := range keys {
			if raw, ok := typed[key].(string); ok && strings.TrimSpace(raw) != "" {
				return strings.TrimSpace(raw)
			}
		}
		names := make([]string, 0, len(typed))
		for name := range typed {
			names = append(names, name)
		}
		sort.Strings(names)
		for _, name := range names {
			if found := findNestedOrdered(typed[name], keys); found != "" {
				return found
			}
		}
	case []interface{}:
		for _, nested := range typed {
			if found := findNestedOrdered(nested, keys); found != "" {
				return found
			}
		}
	}
	return ""
}

// 🗺️ExtractToolInputMap answers the `tool_input` object, at the top level or under `event` or `native.event`.
func ExtractToolInputMap(input json.RawMessage) map[string]interface{} {
	return extractToolInputMapFromData(decodeHookInputMap(input))
}

// 🔹️ExtractCommand answers the shell command the payload carries.
func ExtractCommand(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var literal string
	if err := json.Unmarshal(input, &literal); err == nil {
		return strings.TrimSpace(literal)
	}
	decoded := decodeHookInputMap(input)
	if decoded == nil {
		return ""
	}
	if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
		if value, ok := toolInput["command"].(string); ok {
			return strings.TrimSpace(value)
		}
	}
	return FindNestedString(input, "command", "command_line", "commandLine")
}

// 📁️ExtractCommandAndCwd answers the command and the working directory it runs in; an absent
// directory stays empty rather than silently becoming the host's cwd.
func ExtractCommandAndCwd(input json.RawMessage) (string, string) {
	command := ExtractCommand(input)
	cwd := ""
	if toolInput := ExtractToolInputMap(input); len(toolInput) > 0 {
		if value, ok := toolInput["cwd"].(string); ok {
			cwd = strings.TrimSpace(value)
		}
	}
	if cwd == "" {
		cwd = FindNestedString(input, "cwd", "working_directory", "workingDirectory")
	}
	return command, cwd
}

// 🔷️ExtractToolName answers the tool name the payload names.
func ExtractToolName(input json.RawMessage) string {
	return FindNestedString(input, "tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName")
}

// 📡️ExtractHookEventName answers the native hook event name the payload names.
func ExtractHookEventName(input json.RawMessage) string {
	return FindNestedString(input, "hookEventName", "hook_event_name", "hook_event", "hookEvent", "event")
}

// 🧾️ExtractTranscript answers the transcript path the payload names.
func ExtractTranscript(input json.RawMessage) string {
	return FindNestedString(input, "transcript", "transcript_path", "transcriptPath", "log_path", "logPath")
}

// 🧠️ExtractLLM answers the model the payload names.
func ExtractLLM(input json.RawMessage) string {
	return FindNestedString(input, "llm", "model", "model_name", "modelName")
}

// 🏋️ExtractEffort answers the reasoning effort the payload names.
func ExtractEffort(input json.RawMessage) string {
	return FindNestedString(input, "effort", "reasoning_effort", "reasoningEffort")
}

func topLevelString(input json.RawMessage, keys ...string) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	for _, key := range keys {
		if raw, ok := data[key].(string); ok && raw != "" {
			return strings.TrimSpace(raw)
		}
	}
	return ""
}

// ▪️ExtractMessageID answers the message identifier of the turn.
func ExtractMessageID(input json.RawMessage) string {
	return topLevelString(input, "messageId", "message_id", "turnId", "turn_id", "requestId", "request_id")
}

// ▫️ExtractParentMessageID answers the message identifier of the parent turn.
func ExtractParentMessageID(input json.RawMessage) string {
	return topLevelString(input, "parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id")
}

// 🪪️ExtractSessionID answers the session identifier, falling back to the basename of the transcript path.
func ExtractSessionID(input json.RawMessage) string {
	session := FindNestedString(input, "session_id", "sessionId", "trajectory_id", "trajectoryId", "conversation_id", "conversationId", "agent_id", "agentId")
	if session != "" {
		return session
	}
	transcript := ExtractTranscript(input)
	if transcript == "" {
		return ""
	}
	base := BaseName(strings.TrimSpace(transcript))
	if index := strings.LastIndex(base, "."); index > 0 {
		return strings.TrimSpace(base[:index])
	}
	return strings.TrimSpace(base)
}

// 🪪️NormalizeParentSessionID normalises a parent hint: the literal `subagent` is a shape, not a session id.
func NormalizeParentSessionID(parent string) string { return normalizeParentSessionID(parent) }

// 👪️ExtractParent answers the parent session the payload names, at the top level, under `event` or under `native`.
func ExtractParent(input json.RawMessage) string { return extractParentFromInput(input) }

// 🔳️ResolveParentSessionID answers the parent session: the caller's hint when it names one, otherwise the payload's.
func ResolveParentSessionID(parentInfo string, input json.RawMessage) string {
	return resolveParentSessionID(parentInfo, input)
}

// 💬️ExtractChat answers the conversation the compaction event is about.
func ExtractChat(input json.RawMessage) string { return extractChatFromInput(input) }

// 📦️ExtractReport answers the report an agent-ended payload carries.
func ExtractReport(input json.RawMessage) string { return extractReportFromInput(input) }

// 🔖️ExtractCheckpointID answers the checkpoint identifier the payload names, falling back to the environment.
func ExtractCheckpointID(input json.RawMessage, environment HookEnvironment) string {
	if checkpoint := topLevelString(input, "sha", "checkpoint_sha", "checkpointSha", "hash"); checkpoint != "" {
		return checkpoint
	}
	return environment.CurrentCheckpoint()
}

// 📝️ExtractCheckpointMessage answers the checkpoint message the payload names, falling back to the environment.
func ExtractCheckpointMessage(input json.RawMessage, repoRoot string, environment HookEnvironment) string {
	if message := topLevelString(input, "message", "commit_message", "commitMessage"); message != "" {
		return message
	}
	return environment.CheckpointMessage(repoRoot)
}

// 📝️CodeEdit is what a code edit event carries.
type CodeEdit struct {
	Path string `json:"path"`
	Old  string `json:"old"`
	New  string `json:"new"`
	All  bool   `json:"all"`
}

// 📝️ExtractCodeEdit reads a code edit out of the payload, or out of the raw tool argument string.
func ExtractCodeEdit(input json.RawMessage, toolArgs string) CodeEdit {
	path, old, replacement, all := extractCodeEditFromInput(input, toolArgs)
	return CodeEdit{Path: path, Old: old, New: replacement, All: all}
}

// 🟫️ExtractTerminalCommand answers the terminal command the payload or the tool argument string carries.
func ExtractTerminalCommand(input json.RawMessage, toolArgs string) string {
	if command := ExtractCommand(input); command != "" {
		return command
	}
	args := parseToolArgs(toolArgs)
	if args == nil {
		return ""
	}
	if command, ok := args["command"].(string); ok {
		return command
	}
	return ""
}

// 💠️TerminalOutcome is what a finished terminal invocation reports.
type TerminalOutcome struct {
	Command    string `json:"command"`
	PID        string `json:"pid"`
	Terminated bool   `json:"terminated"`
	Stdout     string `json:"stdout"`
	Stderr     string `json:"stderr"`
}

// 💠️ExtractTerminalOutcome reads a finished terminal invocation out of the payload.
func ExtractTerminalOutcome(input json.RawMessage) TerminalOutcome {
	command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
	return TerminalOutcome{Command: command, PID: pid, Terminated: terminated, Stdout: stdout, Stderr: stderr}
}

// 🔎️SearchReads is the web pages and file line ranges a search event touched.
type SearchReads struct {
	Pages  []string `json:"pages"`
	Ranges []string `json:"ranges"`
}

// 🔎️ExtractSearchReads reads the search coverage out of the payload, or out of the raw tool argument string.
func ExtractSearchReads(input json.RawMessage, toolArgs string) SearchReads {
	reads := SearchReads{Pages: []string{}, Ranges: []string{}}
	source := planSource(input, toolArgs)
	if source == nil {
		return reads
	}
	if url, ok := source["url"].(string); ok && url != "" {
		reads.Pages = append(reads.Pages, url)
	}
	if pages, ok := source["pages"].([]interface{}); ok {
		for _, page := range pages {
			if text, ok := page.(string); ok && (strings.HasPrefix(text, "http://") || strings.HasPrefix(text, "https://")) {
				reads.Pages = append(reads.Pages, text)
			}
		}
	}
	filePath := ""
	if raw, ok := source["file_path"].(string); ok {
		filePath = raw
	} else if raw, ok := source["filePath"].(string); ok {
		filePath = raw
	}
	if filePath == "" {
		return reads
	}
	number := func(key string) int64 {
		if raw, ok := source[key].(float64); ok {
			return int64(raw)
		}
		return 0
	}
	startLine := number("startLine")
	endLine := number("endLine")
	limit := number("limit")
	offset := int64(1)
	if _, present := source["offset"]; present {
		offset = number("offset")
	}
	switch {
	case startLine > 0:
		if endLine <= 0 {
			endLine = startLine
		}
		if startLine == endLine {
			reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d", filePath, startLine))
		} else {
			reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d-L%d", filePath, startLine, endLine))
		}
	case limit > 0:
		start := offset
		if start <= 0 {
			start = 1
		}
		reads.Ranges = append(reads.Ranges, fmt.Sprintf("%s#L%d-L%d", filePath, start, start+limit-1))
	default:
		reads.Ranges = append(reads.Ranges, filePath)
	}
	return reads
}

func parseToolArgs(toolArgs string) map[string]interface{} {
	if toolArgs == "" {
		return nil
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(toolArgs), &parsed); err != nil {
		return nil
	}
	return parsed
}

func planSource(input json.RawMessage, toolArgs string) map[string]interface{} {
	data := decodeHookInputMap(input)
	if data == nil {
		data = parseToolArgs(toolArgs)
	}
	if data == nil {
		return nil
	}
	raw, present := data["tool_input"]
	if !present {
		return data
	}
	toolInput, ok := raw.(map[string]interface{})
	if !ok {
		return nil
	}
	return toolInput
}

// #endregion 🧭️PayloadReading

// #region 🏛️ToolClassification

// 🏛️ClassifyTool answers the tool kind of a named tool, across every IDE's tool vocabulary.
func ClassifyTool(toolName string) model.ToolKind { return todos.ClassifyTool(toolName) }

// 🔤️ClassifyCommandKind answers the tool kind a raw shell command implies.
func ClassifyCommandKind(command string) model.ToolKind { return model.ClassifyCommandKind(command) }

// ✂️TrimPipelineTail cuts a command at its first unquoted pipe.
func TrimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	var current strings.Builder
	quote := rune(0)
	for _, character := range trimmed {
		if quote != 0 {
			current.WriteRune(character)
			if character == quote {
				quote = 0
			}
			continue
		}
		if character == '\'' || character == '"' {
			quote = character
			current.WriteRune(character)
			continue
		}
		if character == '|' {
			break
		}
		current.WriteRune(character)
	}
	return strings.TrimSpace(current.String())
}

// 🧪️ExtractTestSegment answers the test-running segment of a composite command, with the directory a leading `cd` selected.
func ExtractTestSegment(command string) (string, string) {
	return testrunner.ExtractTestSegmentFromCommand(command)
}

// #endregion 🏛️ToolClassification

// #region 🛡️BlockingPolicy

// ⌨️BlockedGitVerbs lists the git subcommands that modify repository state and are therefore refused.
var BlockedGitVerbs = []string{"add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag", "clean"}

// 🔷️SplitCommandSegments splits a command at every unquoted `;`, `&&`, `|` and `||`.
func SplitCommandSegments(command string) []string {
	segments := workspace.SplitCommandSegments(command)
	if segments == nil {
		return []string{}
	}
	return segments
}

// 🌿️ContainsBlockedGitInCode scans arbitrary inline code (`python -c`, `node -e`, …) for a blocked git invocation.
func ContainsBlockedGitInCode(code string) (bool, string) { return containsBlockedGitInCode(code) }

// 🐙️IsCommandSegmentBlocked answers why one command segment is refused, or false when it is allowed.
func IsCommandSegmentBlocked(segment string) (bool, string) { return isCommandSegmentBlocked(segment) }

// #endregion 🛡️BlockingPolicy

// #region 🗺️PlanSteps

// 🔁️HookPlanStep is one step of a plan / task list update event.
type HookPlanStep = model.HookPlanStep

// 🗺️ExtractPlanSteps reads the plan steps out of a payload: VS Code's `todoList` entries and the
// neutral `steps` entries, in that order, from `tool_input` when present.
func ExtractPlanSteps(input json.RawMessage, toolArgs string) []HookPlanStep {
	steps := []HookPlanStep{}
	source := planSource(input, toolArgs)
	if source == nil {
		return steps
	}
	if entries, ok := source["todoList"].([]interface{}); ok {
		for _, entry := range entries {
			if fields, ok := entry.(map[string]interface{}); ok {
				name, _ := fields["title"].(string)
				status, _ := fields["status"].(string)
				if name != "" {
					steps = append(steps, HookPlanStep{Name: name, Status: status})
				}
			}
		}
	}
	if entries, ok := source["steps"].([]interface{}); ok {
		for _, entry := range entries {
			if fields, ok := entry.(map[string]interface{}); ok {
				name, _ := fields["name"].(string)
				status, _ := fields["status"].(string)
				if name != "" {
					steps = append(steps, HookPlanStep{Name: name, Status: status})
				}
			}
		}
	}
	return steps
}

// 🔀️MergePlanSteps folds an incoming plan into the recorded one, keeping every lifecycle timestamp.
func MergePlanSteps(existing []model.TicketAgentPlanStep, incoming []HookPlanStep, second string) []model.TicketAgentPlanStep {
	return todos.MergeTicketAgentPlanSteps(existing, incoming, second)
}

// #endregion 🗺️PlanSteps

// #region 🔶️Dispatch

// 🔶️HookResult is the outcome of a hook invocation: the two fields every hook exposes, plus the
// event-specific payload the calling IDE reads back.
type HookResult struct {
	Allowed bool
	Message string
	Extra   map[string]interface{}
	// 🧾️Order is the declaration order of the event-specific fields. `json.Marshal` of the typed
	// `model.HookResult*` records the repo binary returns writes them in declaration order, and a
	// Go map marshals sorted, so the order is carried explicitly here.
	Order []string
	// 🙈️MessageShadowed reports that `message` belongs to the event payload, not to the base.
	// `model.HookResultAgentBase` declares `MessageID string \`json:"message,omitempty"\``, which
	// shadows `model.HookResultBase.Message` on every agent result: the block reason stays readable
	// through `GetMessage()` and never reaches the marshalled object.
	MessageShadowed bool
}

// 🆕️NewHookResult builds a result carrying only the base fields.
func NewHookResult(allowed bool, message string) HookResult {
	return HookResult{Allowed: allowed, Message: message, Extra: map[string]interface{}{}}
}

// ✔️IsAllowed reports whether the invocation was allowed.
func (r HookResult) IsAllowed() bool { return r.Allowed }

// 💬️GetMessage answers the message the invocation carries.
func (r HookResult) GetMessage() string { return r.Message }

// ➕️With adds one event-specific field, keeping the order it was first added in.
func (r HookResult) With(key string, value interface{}) HookResult {
	if r.Extra == nil {
		r.Extra = map[string]interface{}{}
	}
	if _, seen := r.Extra[key]; !seen {
		order := make([]string, len(r.Order), len(r.Order)+1)
		copy(order, r.Order)
		r.Order = append(order, key)
	}
	r.Extra[key] = value
	return r
}

// 🧾️Fields answers the members this result marshals to, in declaration order: `allowed`,
// `message` when it is non-empty and not shadowed, then every event-specific field.
func (r HookResult) Fields() []HookResultField {
	fields := []HookResultField{{Key: "allowed", Value: r.Allowed}}
	if r.Message != "" && !r.MessageShadowed {
		fields = append(fields, HookResultField{Key: "message", Value: r.Message})
	}
	for _, key := range r.Order {
		fields = append(fields, HookResultField{Key: key, Value: r.Extra[key]})
	}
	return fields
}

// 🧾️HookResultField is one marshalled member of a hook result.
type HookResultField struct {
	Key   string
	Value interface{}
}

// 🧾️Payload answers the same members as an unordered map, for a caller that only looks values up.
func (r HookResult) Payload() map[string]interface{} {
	payload := map[string]interface{}{}
	for _, field := range r.Fields() {
		payload[field.Key] = field.Value
	}
	return payload
}

// 🧾️WriteOrderedObject writes an object whose members keep the given order, which a Go map cannot.
func WriteOrderedObject(fields []HookResultField) ([]byte, error) {
	var out strings.Builder
	out.WriteString("{")
	for index, field := range fields {
		if index > 0 {
			out.WriteString(",")
		}
		key, err := json.Marshal(field.Key)
		if err != nil {
			return nil, err
		}
		value, err := json.Marshal(field.Value)
		if err != nil {
			return nil, err
		}
		out.Write(key)
		out.WriteString(":")
		out.Write(value)
	}
	out.WriteString("}")
	return []byte(out.String()), nil
}

// 📤️MarshalJSON renders the flat record every IDE reads, in declaration order.
func (r HookResult) MarshalJSON() ([]byte, error) { return WriteOrderedObject(r.Fields()) }

func insertWhenPresent(result HookResult, key string, value string) HookResult {
	if value == "" {
		return result
	}
	return result.With(key, value)
}

// 🔤️FirstNonEmpty answers the first non-blank value.
func FirstNonEmpty(values ...string) string { return todos.FirstNonEmpty(values...) }

func agentBase(ctx HookContext, environment HookEnvironment) HookResult {
	input := ctx.Input
	session := ExtractSessionID(input)
	second := FirstNonEmpty(FindNestedString(input, "second"), ctx.Second)
	parent := ResolveParentSessionID("", input)
	if ctx.ParentInfo != "" {
		parent = ResolveParentSessionID(ctx.ParentInfo, input)
		if ctx.ParentInfo == "subagent" {
			agentID := FindNestedString(input, "agent_id", "agentId")
			if agentID != "" {
				parentSession := FirstNonEmpty(ExtractParent(input), ExtractSessionID(input))
				session = agentID
				parent = NormalizeParentSessionID(parentSession)
			} else if parent == "" {
				parent = session
			}
		}
	}
	result := NewHookResult(true, "")
	result.MessageShadowed = true
	result = insertWhenPresent(result, "checkpoint", ExtractCheckpointID(input, environment))
	result = insertWhenPresent(result, "session", session)
	result = insertWhenPresent(result, "second", second)
	result = insertWhenPresent(result, "client", ctx.Client)
	result = insertWhenPresent(result, "llm", ExtractLLM(input))
	result = insertWhenPresent(result, "effort", ExtractEffort(input))
	result = insertWhenPresent(result, "transcript", ExtractTranscript(input))
	result = insertWhenPresent(result, "message", ExtractMessageID(input))
	return result.With("parent", parent)
}

// 🔶️DispatchHook builds the event-specific hook result for one context. Blocking is applied here
// and nowhere else: `agent.tool.starting` and `agent.tool.terminal.starting` are the two events that
// can refuse an invocation, and a refusal is carried as `allowed = false` plus the reason.
func DispatchHook(ctx HookContext, environment HookEnvironment, tests TestFileResolver) HookResult {
	event, known := ctx.ResolvedEvent()
	if !known {
		return NewHookResult(false, "unknown event: "+ctx.Event)
	}
	input := ctx.Input
	if HookEventKind(event) == model.HookKindVersion {
		message := ""
		if event == model.HookVersionCheckpointStarting {
			message = "checkpoint starting hook passed"
		}
		result := NewHookResult(true, message)
		result = insertWhenPresent(result, "checkpoint", ExtractCheckpointID(input, environment))
		if event == model.HookVersionCheckpointStarting || event == model.HookVersionCheckpointEnded {
			result = insertWhenPresent(result, "description", ExtractCheckpointMessage(input, ctx.RepoRoot, environment))
		}
		return result
	}
	base := agentBase(ctx, environment)
	switch event {
	case model.HookAgentStarted:
		return base
	case model.HookAgentEnded:
		return insertWhenPresent(base, "report", ExtractReport(input))
	case model.HookAgentPromptSubmitting:
		return insertWhenPresent(base, "prompt", FindNestedString(input, "prompt", "text"))
	case model.HookAgentCompacting:
		return insertWhenPresent(base, "chat", ExtractChat(input))
	case model.HookAgentToolStarting:
		toolName := FirstNonEmpty(ctx.ToolName, ExtractToolName(input))
		args := FirstNonEmpty(ctx.ToolArgs, ExtractTerminalCommand(input, ctx.ToolArgs), ExtractCommand(input))
		blocked, reason := IsToolBlocked(toolName, args)
		result := HookResult{Allowed: !blocked, Message: reason, Extra: base.Extra, Order: base.Order, MessageShadowed: base.MessageShadowed}
		result = insertWhenPresent(result, "name", toolName)
		if toolInput := ExtractToolInputMap(input); toolInput != nil {
			result = result.With("input", toolInput)
		}
		return result
	case model.HookAgentToolEnded:
		result := insertWhenPresent(base, "name", FirstNonEmpty(ctx.ToolName, ExtractToolName(input)))
		if toolInput := ExtractToolInputMap(input); toolInput != nil {
			result = result.With("input", toolInput)
		}
		if response, present := extractToolResponse(input); present {
			result = result.With("response", response)
		}
		return result
	case model.HookAgentToolPlanUpdatingStarting, model.HookAgentToolPlanUpdatingEnded:
		steps := ExtractPlanSteps(input, ctx.ToolArgs)
		if len(steps) == 0 {
			return base
		}
		return base.With("steps", steps)
	case model.HookAgentToolSearchStarting, model.HookAgentToolSearchEnded:
		reads := ExtractSearchReads(input, ctx.ToolArgs)
		result := base
		if len(reads.Pages) > 0 {
			result = result.With("pages", reads.Pages)
		}
		if len(reads.Ranges) > 0 {
			result = result.With("ranges", reads.Ranges)
		}
		return result
	case model.HookAgentToolCodeEditStarting:
		edit := ExtractCodeEdit(input, ctx.ToolArgs)
		result := insertWhenPresent(base, "path", edit.Path)
		result = insertWhenPresent(result, "old", edit.Old)
		result = insertWhenPresent(result, "new", edit.New)
		if edit.All {
			result = result.With("all", true)
		}
		return result
	case model.HookAgentToolCodeEditEnded:
		edit := ExtractCodeEdit(input, ctx.ToolArgs)
		if edit.Path != "" {
			environment.FormatFile(edit.Path)
		}
		result := insertWhenPresent(base, "path", edit.Path)
		result = insertWhenPresent(result, "old", edit.Old)
		return insertWhenPresent(result, "new", edit.New)
	case model.HookAgentToolTestStarting:
		selection := ExtractTestSelection(input, ctx.ToolArgs, tests)
		result := base
		if len(selection.Labs) > 0 {
			result = result.With("labs", selection.Labs)
		}
		if len(selection.Tests) > 0 {
			result = result.With("tests", selection.Tests)
		}
		return insertWhenPresent(result, "timeout", selection.Timeout)
	case model.HookAgentToolTestEnded:
		outcome := ExtractTestOutcome(input, tests)
		result := base
		if len(outcome.Files) > 0 {
			result = result.With("files", outcome.Files)
		}
		if len(outcome.Succeeded) > 0 {
			result = result.With("succeeded", outcome.Succeeded)
		}
		if len(outcome.Failed) > 0 {
			result = result.With("failed", outcome.Failed)
		}
		return result
	case model.HookAgentToolBuildStarting:
		bundles := ExtractBuildBundles(input, ctx.ToolArgs)
		result := insertWhenPresent(base, "name", FirstNonEmpty(ctx.ToolName, "build"))
		return result.With("input", map[string]interface{}{"bundles": bundles})
	case model.HookAgentToolBuildEnded:
		outcome := ExtractBuildOutcome(input)
		result := insertWhenPresent(base, "name", FirstNonEmpty(ctx.ToolName, "build"))
		return result.With("response", map[string]interface{}{"succeeded": outcome.Succeeded, "failed": outcome.Failed})
	case model.HookAgentToolTerminalStarting:
		command := ExtractTerminalCommand(input, ctx.ToolArgs)
		tool := FirstNonEmpty(ctx.ToolName, "run_in_terminal")
		blocked, reason := IsToolBlocked(tool, FirstNonEmpty(command, ctx.ToolArgs))
		result := HookResult{Allowed: !blocked, Message: reason, Extra: base.Extra, Order: base.Order, MessageShadowed: base.MessageShadowed}
		result = insertWhenPresent(result, "name", FirstNonEmpty(ctx.ToolName, ExtractToolName(input)))
		if toolInput := ExtractToolInputMap(input); toolInput != nil {
			result = result.With("input", toolInput)
		}
		return insertWhenPresent(result, "command", command)
	case model.HookAgentToolTerminalEnded:
		outcome := ExtractTerminalOutcome(input)
		result := insertWhenPresent(base, "name", FirstNonEmpty(ctx.ToolName, ExtractToolName(input)))
		if toolInput := ExtractToolInputMap(input); toolInput != nil {
			result = result.With("input", toolInput)
		}
		result = insertWhenPresent(result, "command", outcome.Command)
		pid, _ := strconv.ParseInt(outcome.PID, 10, 64)
		result = result.With("pid", pid)
		if outcome.Terminated {
			result = result.With("terminated", true)
		}
		result = result.With("stdout", outcome.Stdout)
		return result.With("stderr", outcome.Stderr)
	case model.HookAgentThinkingStarting, model.HookAgentThinkingEnded:
		return HookResult{Allowed: base.Allowed, Message: FindNestedString(input, "text"), Extra: base.Extra, Order: base.Order, MessageShadowed: base.MessageShadowed}
	default:
		return base
	}
}

func extractToolResponse(input json.RawMessage) (interface{}, bool) {
	data := decodeHookInputMap(input)
	if data == nil {
		return nil, false
	}
	if response, present := data["tool_response"]; present {
		return response, true
	}
	if output, present := data["tool_output"]; present {
		return output, true
	}
	output := FindNestedString(input, "tool_output", "toolOutput", "output", "response")
	if output == "" {
		return nil, false
	}
	return output, true
}

// 🧪️TestSelection is what a starting test event selected.
type TestSelection struct {
	Labs    []string `json:"labs"`
	Tests   []string `json:"tests"`
	Timeout string   `json:"timeout"`
}

// 🧪️TestOutcome is what a finished test event reports.
type TestOutcome struct {
	Files     []string `json:"files"`
	Succeeded []string `json:"succeeded"`
	Failed    []string `json:"failed"`
}

// 🏗️BuildOutcome is what a finished build event reports.
type BuildOutcome struct {
	Succeeded []string `json:"succeeded"`
	Failed    []string `json:"failed"`
}

func stringMembers(source map[string]interface{}, keys ...string) []string {
	collected := []string{}
	for _, key := range keys {
		switch typed := source[key].(type) {
		case []interface{}:
			for _, item := range typed {
				if text, ok := item.(string); ok && text != "" {
					collected = append(collected, text)
				}
			}
		case string:
			if typed != "" {
				collected = append(collected, typed)
			}
		}
	}
	return collected
}

// 🧪️ExtractTestSelection reads a starting test event: the files and test selectors it names, and its timeout.
func ExtractTestSelection(input json.RawMessage, toolArgs string, tests TestFileResolver) TestSelection {
	source := ExtractToolInputMap(input)
	if source == nil {
		source = parseToolArgs(toolArgs)
	}
	if source == nil {
		source = map[string]interface{}{}
	}
	selection := TestSelection{Labs: []string{}, Tests: []string{}}
	targetFiles := stringMembers(source, "files")
	for _, named := range stringMembers(source, "testNames", "test_names", "tests") {
		if looksLikeFilePath(named) {
			targetFiles = append(targetFiles, named)
		} else {
			selection.Tests = append(selection.Tests, named)
		}
	}
	for _, key := range []string{"timeout", "timeoutMs"} {
		if raw, ok := source[key].(string); ok && raw != "" {
			selection.Timeout = raw
			break
		}
		if raw, ok := source[key].(float64); ok {
			selection.Timeout = fmt.Sprintf("%d", int64(raw))
			break
		}
	}
	command := ExtractCommand(input)
	if command == "" {
		if raw, ok := source["command"].(string); ok {
			command = raw
		}
	}
	if command != "" && len(targetFiles) == 0 {
		_, cwd := ExtractCommandAndCwd(input)
		if resolved := tests.TestFiles(command, cwd); len(resolved) > 0 {
			targetFiles = resolved
		}
	}
	if len(targetFiles) > 0 && allBlank(selection.Tests) {
		selection.Labs = tests.LabIDs(targetFiles)
		if len(selection.Labs) > 0 {
			selection.Tests = []string{}
			return selection
		}
	}
	return selection
}

func allBlank(values []string) bool {
	for _, value := range values {
		if value != "" {
			return false
		}
	}
	return true
}

// 🧪️ExtractTestOutcome reads a finished test event: the files it ran and which cases passed or failed.
func ExtractTestOutcome(input json.RawMessage, tests TestFileResolver) TestOutcome {
	outcome := TestOutcome{Files: []string{}, Succeeded: []string{}, Failed: []string{}}
	data := decodeHookInputMap(input)
	if data == nil {
		return outcome
	}
	if toolInput := ExtractToolInputMap(input); toolInput != nil {
		outcome.Files = stringMembers(toolInput, "files")
	}
	if len(outcome.Files) == 0 {
		command, cwd := ExtractCommandAndCwd(input)
		if command != "" {
			if resolved := tests.TestFiles(command, cwd); resolved != nil {
				outcome.Files = resolved
			}
		}
	}
	toolOutput := data
	if nested, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = nested
	}
	outcome.Succeeded = stringMembers(toolOutput, "succeeded", "passed", "passing")
	outcome.Failed = stringMembers(toolOutput, "failed", "failing", "errors")
	return outcome
}

// 🏗️ExtractBuildBundles answers the bundles a starting build event names.
func ExtractBuildBundles(input json.RawMessage, toolArgs string) []string {
	source := ExtractToolInputMap(input)
	if source == nil {
		source = parseToolArgs(toolArgs)
	}
	if source == nil {
		return []string{}
	}
	return stringMembers(source, "bundles", "targets", "technologies", "label")
}

// 🏗️ExtractBuildOutcome reads a finished build event.
func ExtractBuildOutcome(input json.RawMessage) BuildOutcome {
	data := decodeHookInputMap(input)
	if data == nil {
		return BuildOutcome{Succeeded: []string{}, Failed: []string{}}
	}
	toolOutput := data
	if nested, ok := data["tool_output"].(map[string]interface{}); ok {
		toolOutput = nested
	}
	return BuildOutcome{Succeeded: stringMembers(toolOutput, "succeeded", "passed", "built"), Failed: stringMembers(toolOutput, "failed", "errors")}
}

// #endregion 🔶️Dispatch

// #region 🖨️Formatting

// 🖨️HookOutput is what the repo binary writes after a hook, and with which exit code.
type HookOutput struct {
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
	ExitCode int    `json:"exitCode"`
}

// 🧾️FormatPlainHookOutput renders the bare record every non-VS-Code editor reads.
func FormatPlainHookOutput(result HookResult) string {
	document, err := json.Marshal(result)
	if err != nil {
		return ""
	}
	return string(document)
}

// 🖨️RenderHookOutput renders a hook result the way the calling client reads it. Copilot Chat is the
// only client whose result is wrapped, and it is written to stdout whether or not the invocation was
// allowed. JSON mode prints the plain record for every other client; otherwise a refusal goes to
// stderr with exit code 2 and an allowed result prints only its message.
func RenderHookOutput(client string, event model.HookEvent, parentInfo string, nativeEventName string, result HookResult, jsonMode bool) HookOutput {
	if provider := providers.GetEditorProvider(client); provider != nil && provider.Kind() == "copilot-chat" {
		hookEventName := nativeEventName
		if hookEventName == "" {
			hookEventName = provider.NativeEventFromHookEvent(event, parentInfo)
		}
		return HookOutput{Stdout: provider.FormatHookOutput(hookEventName, result)}
	}
	if jsonMode {
		return HookOutput{Stdout: FormatPlainHookOutput(result)}
	}
	if !result.Allowed {
		return HookOutput{Stderr: result.Message, ExitCode: 2}
	}
	return HookOutput{Stdout: result.Message}
}

// 🔤️ResolveNativeEventName answers the native event name the client would use for a neutral event, or the payload's own name.
func ResolveNativeEventName(client string, event model.HookEvent, parentInfo string, input json.RawMessage) string {
	if named := ExtractHookEventName(input); named != "" {
		return named
	}
	provider := providers.GetEditorProvider(client)
	if provider == nil {
		return ""
	}
	return provider.NativeEventFromHookEvent(event, parentInfo)
}

// 🎆️EditorForClient answers the editor provider for a client slug.
func EditorForClient(client string) providers.EditorProvider {
	return providers.GetEditorProvider(client)
}

// #endregion 🖨️Formatting

// #region 📓️SessionLogging

// 📍️SessionEventEntry is one recorded hook invocation in a session log.
type SessionEventEntry struct {
	Event    json.RawMessage        `json:"event"`
	Native   *SessionNativeEnvelope `json:"native,omitempty"`
	Response *SessionResponse       `json:"response,omitempty"`
}

// 🧾️SessionNativeEnvelope is the native payload, kept only at `full` detail.
type SessionNativeEnvelope struct {
	Event json.RawMessage `json:"event"`
}

// 📤️SessionResponse is what the hook answered, kept at `standard` and `full` detail.
type SessionResponse struct {
	Blocked *bool  `json:"blocked,omitempty"`
	Message string `json:"message,omitempty"`
	Reason  string `json:"reason,omitempty"`
}

// 🔸️SessionMeta is the `session.json` record of one agent session.
type SessionMeta struct {
	ID          string                 `json:"id"`
	URI         string                 `json:"uri,omitempty"`
	Client      string                 `json:"client,omitempty"`
	Second      string                 `json:"second,omitempty"`
	Checkpoint  string                 `json:"checkpoint,omitempty"`
	Contributor string                 `json:"contributor,omitempty"`
	Transcript  string                 `json:"transcript,omitempty"`
	Events      []SessionEventEntry    `json:"events,omitempty"`
	Plan        *model.TicketAgentPlan `json:"plan,omitempty"`
}

// 💾️SessionStore is where a session log lives. The filesystem implementation is one of several: a
// caller under test hands in a memory store and never touches a disk.
type SessionStore interface {
	Read(sessionID string) (SessionMeta, bool)
	Write(sessionID string, meta SessionMeta) error
}

// 🧠️MemorySessionStore is an in-memory session store — the deterministic default for tests and dry runs.
type MemorySessionStore struct {
	sessions map[string]SessionMeta
}

// 🆕️NewMemorySessionStore builds an empty store.
func NewMemorySessionStore() *MemorySessionStore {
	return &MemorySessionStore{sessions: map[string]SessionMeta{}}
}

// 📥️Read answers the recorded session, when one exists.
func (s *MemorySessionStore) Read(sessionID string) (SessionMeta, bool) {
	meta, present := s.sessions[sessionID]
	return meta, present
}

// 📤️Write replaces the recorded session.
func (s *MemorySessionStore) Write(sessionID string, meta SessionMeta) error {
	if s.sessions == nil {
		s.sessions = map[string]SessionMeta{}
	}
	s.sessions[sessionID] = meta
	return nil
}

// 📋️Sessions lists every recorded session, by id.
func (s *MemorySessionStore) Sessions() map[string]SessionMeta { return s.sessions }

// 💽️DirectorySessionStore is a session store rooted at one directory, writing `<root>/<session id>/session.json`.
type DirectorySessionStore struct {
	Root string
}

// 🆕️NewDirectorySessionStore builds a store under root.
func NewDirectorySessionStore(root string) *DirectorySessionStore {
	return &DirectorySessionStore{Root: root}
}

// 📄️Path answers the file one session is recorded in.
func (s *DirectorySessionStore) Path(sessionID string) string {
	return filepath.Join(s.Root, sessionID, SessionFileName)
}

// 📥️Read answers the recorded session, when one exists.
func (s *DirectorySessionStore) Read(sessionID string) (SessionMeta, bool) {
	data, err := os.ReadFile(s.Path(sessionID))
	if err != nil {
		return SessionMeta{}, false
	}
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		return SessionMeta{}, false
	}
	return meta, true
}

// 📤️Write replaces the recorded session.
func (s *DirectorySessionStore) Write(sessionID string, meta SessionMeta) error {
	path := s.Path(sessionID)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return NewHookError(fmt.Sprintf("cannot create %s: %v", filepath.Dir(path), err))
	}
	return workspace.WriteJSONFile(path, meta)
}

// 📄️SessionFileName is the file name a session log is recorded in.
const SessionFileName = "session.json"

// 📁️SessionLogDir answers the directory a session log lives in.
func SessionLogDir(repoRoot string, year int, month int, day int, sessionID string) string {
	return filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🤖️generated", model.FormatYearDir(year%100), model.FormatMonthDir(month), model.FormatDayDir(day), sessionID)
}

// 🪪️ResolveLogSessionID answers the session id a log is filed under: the payload's, `kiro-<pid>` for
// the Kiro CLI, otherwise the literal `unknown`.
func ResolveLogSessionID(ctx HookContext, kiroParentPID int) string {
	if session := ExtractSessionID(ctx.Input); session != "" {
		return session
	}
	if ctx.Client == "kiro-cli" && kiroParentPID > 0 {
		return fmt.Sprintf("kiro-%d", kiroParentPID)
	}
	return "unknown"
}

// 📓️RecordSessionHook appends one hook invocation to a session log, creating the record on first
// sight. Nothing is written for a `version.*` event or when `[logging] session` is off.
func RecordSessionHook(store SessionStore, ctx HookContext, result HookResult, sessionID string, logging workspace.LoggingConfig, environment HookEnvironment) (*SessionMeta, error) {
	event, known := ctx.ResolvedEvent()
	if !known {
		return nil, nil
	}
	if HookEventKind(event) == model.HookKindVersion || !logging.Session {
		return nil, nil
	}
	input := ctx.Input
	meta, _ := store.Read(sessionID)
	second := FirstNonEmpty(ctx.Second, FindNestedString(input, "second"))
	checkpoint := ExtractCheckpointID(input, environment)
	transcript := ExtractTranscript(input)
	if meta.ID == "" {
		meta.ID = sessionID
		meta.URI = "repo://session/" + sessionID
		meta.Client = ctx.Client
		meta.Second = second
		meta.Checkpoint = checkpoint
		meta.Contributor = "unknown"
		meta.Transcript = transcript
	} else {
		if meta.Client == "" {
			meta.Client = ctx.Client
		}
		if meta.Second == "" {
			meta.Second = second
		}
		if meta.Checkpoint == "" {
			meta.Checkpoint = checkpoint
		}
		if meta.Transcript == "" {
			meta.Transcript = transcript
		}
	}
	neutral := map[string]interface{}{
		"kind":        string(event),
		"client":      ctx.Client,
		"session":     sessionID,
		"second":      second,
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	}
	if transcript != "" {
		neutral["transcript"] = transcript
	}
	neutralJSON, err := json.Marshal(neutral)
	if err != nil {
		return nil, NewHookError(err.Error())
	}
	entry := SessionEventEntry{Event: neutralJSON}
	if logging.IncludeNative() && len(input) > 0 {
		entry.Native = &SessionNativeEnvelope{Event: append(json.RawMessage(nil), input...)}
	}
	if logging.IncludeResponse() {
		entry.Response = &SessionResponse{}
		if !result.Allowed {
			blocked := true
			entry.Response.Blocked = &blocked
			entry.Response.Message = result.Message
			entry.Response.Reason = result.Message
		}
	}
	meta.Events = append(meta.Events, entry)
	if logging.Plan && event == model.HookAgentToolPlanUpdatingEnded {
		if steps := ExtractPlanSteps(input, ctx.ToolArgs); len(steps) > 0 {
			var existing []model.TicketAgentPlanStep
			if meta.Plan != nil {
				existing = meta.Plan.Steps
			}
			meta.Plan = &model.TicketAgentPlan{Steps: MergePlanSteps(existing, steps, second)}
		}
	}
	if err := store.Write(sessionID, meta); err != nil {
		return nil, err
	}
	return &meta, nil
}

// #endregion 📓️SessionLogging

// #region 🔖️MicroCommitDelegation

// 📜️MicroCommitArgv answers the argv the repo binary hands to bun for a micro-commit subcommand.
func MicroCommitArgv(bun string, args []string) []string {
	argv := []string{bun, "./📜️script.ts", "micro-commit"}
	return append(argv, args...)
}

// 🕯️BunCandidates lists every place a bun binary is looked for, in order.
func BunCandidates(repoRoot string, pinned string, bunInstall string) []string {
	candidates := []string{}
	if strings.TrimSpace(pinned) != "" {
		candidates = append(candidates, strings.TrimSpace(pinned))
	}
	candidates = append(candidates, filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "🐹️compose-micro-commit-bun"))
	if strings.TrimSpace(bunInstall) != "" {
		for _, name := range []string{"bun", "bun.exe"} {
			candidates = append(candidates, filepath.Join(strings.TrimSpace(bunInstall), "bin", name))
		}
	}
	for _, relative := range []string{"node_modules/.bin/bun", "node_modules/.bin/bun.exe"} {
		candidates = append(candidates, filepath.Join(repoRoot, relative))
	}
	return candidates
}

// 🔖️RunMicroCommit delegates a micro-commit subcommand to the monorepo script through the process runner.
func RunMicroCommit(runner providers.ProcessRunner, repoRoot string, bun string, args []string) providers.ProcessOutcome {
	return runner.Run(providers.ProcessRequest{Argv: MicroCommitArgv(bun, args), Cwd: repoRoot})
}

// ♻️RunMicroCommitReset is the `micro-commit reset` delegation every post-checkpoint git hook performs.
func RunMicroCommitReset(runner providers.ProcessRunner, repoRoot string, bun string) providers.ProcessOutcome {
	return RunMicroCommit(runner, repoRoot, bun, []string{"reset"})
}

// #endregion 🔖️MicroCommitDelegation

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

// #region 🚚️Split

// #region 🔑️Engine Events

func spansEntireFile(lines map[int]struct{}, content string, lineCount int) bool {
	if len(lines) == 0 {
		return true
	}
	effectiveLineCount := lineCount
	if strings.HasSuffix(content, "\n") && effectiveLineCount > 1 {
		effectiveLineCount--
	}
	if len(lines) < effectiveLineCount {
		return false
	}
	for line := 1; line <= effectiveLineCount; line++ {
		if _, ok := lines[line]; !ok {
			return false
		}
	}
	return true
}

// #endregion 🔑️Engine Events

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

// 📝️extractCodeEditFromInput holds the data fields for a extractCodeEditFromInput record.
func extractCodeEditFromInput(input json.RawMessage, toolArgs string) (path string, old string, new_ string, all bool) {
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
		return "", "", "", false
	}
	for _, k := range []string{"filePath", "path", "file_path", "file"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			path = v
			break
		}
	}
	for _, k := range []string{"oldString", "old", "old_string", "search"} {
		if v, ok := toolInput[k].(string); ok {
			old = v
			break
		}
	}
	for _, k := range []string{"newString", "new", "new_string", "replace"} {
		if v, ok := toolInput[k].(string); ok {
			new_ = v
			break
		}
	}
	if v, ok := toolInput["all"].(bool); ok {
		all = v
	} else if v, ok := toolInput["replaceAll"].(bool); ok {
		all = v
	}
	return path, old, new_, all
}

// 🟫️extractTerminalCommandFromInput holds the data fields for a extractTerminalCommandFromInput record.
func extractTerminalCommandFromInput(input json.RawMessage, toolArgs string) string {
	cmd := extractCommandFromStdin(input)
	if cmd != "" {
		return cmd
	}
	if toolArgs != "" {
		var args map[string]interface{}
		if err := json.Unmarshal([]byte(toolArgs), &args); err == nil {
			if c, ok := args["command"].(string); ok {
				return c
			}
		}
	}
	return ""
}

// 💠️extractTerminalEndedFromInput holds the data fields for a extractTerminalEndedFromInput record.
func extractTerminalEndedFromInput(input json.RawMessage) (command string, pid string, terminated bool, stdout string, stderr string) {
	if len(input) == 0 {
		return "", "", false, "", ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return "", "", false, "", ""
	}
	toolInput := data
	if ti, ok := data["tool_input"].(map[string]interface{}); ok {
		toolInput = ti
	}
	for _, k := range []string{"command", "command_line"} {
		if v, ok := toolInput[k].(string); ok && v != "" {
			command = v
			break
		}
	}
	if ti, ok := data["tool_info"].(map[string]interface{}); ok {
		if v, ok := ti["command_line"].(string); ok && v != "" {
			command = v
		}
	}
	for _, k := range []string{"pid", "process_id", "execution_id", "id"} {
		if v, ok := data[k].(string); ok && v != "" {
			pid = v
			break
		}
		if v, ok := data[k].(float64); ok {
			pid = fmt.Sprintf("%d", int64(v))
			break
		}
	}
	for _, k := range []string{"terminated", "has_terminated", "exited", "stopped"} {
		if v, ok := data[k].(bool); ok {
			terminated = v
			break
		}
	}
	for _, k := range []string{"stdout", "output", "tool_output"} {
		if v, ok := data[k].(string); ok {
			stdout = v
			break
		}
	}
	for _, k := range []string{"stderr", "error_output"} {
		if v, ok := data[k].(string); ok {
			stderr = v
			break
		}
	}
	return command, pid, terminated, stdout, stderr
}

func extractChatFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"chat", "conversation", "context", "messages"} {
		if value, ok := data[key].(string); ok {
			return strings.TrimSpace(value)
		}
		if arr, ok := data[key].([]interface{}); ok {
			bytes, err := json.Marshal(arr)
			if err == nil {
				return string(bytes)
			}
		}
	}
	return ""
}

// 📦️extractReportFromInput extracts a text report from agent-ended payloads.
func extractReportFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	getReport := func(node map[string]interface{}) string {
		if node == nil {
			return ""
		}
		if report, ok := node["report"].(string); ok && strings.TrimSpace(report) != "" {
			return strings.TrimSpace(report)
		}
		if toolInfo, ok := node["tool_info"].(map[string]interface{}); ok {
			if response, ok := toolInfo["response"].(string); ok && strings.TrimSpace(response) != "" {
				return strings.TrimSpace(response)
			}
		}
		return ""
	}
	if report := getReport(data); report != "" {
		return report
	}
	if nested, ok := data["event"].(map[string]interface{}); ok {
		if report := getReport(nested); report != "" {
			return report
		}
	}
	if nested, ok := data["input"].(map[string]interface{}); ok {
		if report := getReport(nested); report != "" {
			return report
		}
	}
	if native, ok := data["native"].(map[string]interface{}); ok {
		if report := getReport(native); report != "" {
			return report
		}
		if eventData, ok := native["event"].(map[string]interface{}); ok {
			if report := getReport(eventData); report != "" {
				return report
			}
		}
	}
	return ""
}

func extractCheckpointMessageFromInput(input json.RawMessage, repoRoot string) string {
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			for _, key := range []string{"message", "commit_message", "commitMessage"} {
				if v, ok := data[key].(string); ok && v != "" {
					return strings.TrimSpace(v)
				}
			}
		}
	}
	checkpointMsgFile := filepath.Join(repoRoot, ".git", "COMMIT_EDITMSG")
	if data, err := os.ReadFile(checkpointMsgFile); err == nil {
		msg := strings.TrimSpace(string(data))
		if msg != "" {
			return msg
		}
	}
	return ""
}

// 💾️extractCheckpointSHAFromInput holds the data fields for a extractCheckpointSHAFromInput record.
func extractCheckpointSHAFromInput(input json.RawMessage) string {
	if len(input) > 0 {
		var data map[string]interface{}
		if err := json.Unmarshal(input, &data); err == nil {
			for _, key := range []string{"sha", "checkpoint_sha", "checkpointSha", "hash"} {
				if v, ok := data[key].(string); ok && v != "" {
					return strings.TrimSpace(v)
				}
			}
		}
	}
	return workspace.GetGitCheckpoint()
}

// 🪪️normalizeParentSessionID holds the data fields for a normalizeParentSessionID record.
func normalizeParentSessionID(parent string) string {
	parent = strings.TrimSpace(parent)
	if parent == "" {
		return ""
	}
	if strings.EqualFold(parent, "subagent") {
		return ""
	}
	return parent
}

func extractParentFromMap(data map[string]interface{}) string {
	for _, key := range []string{"parent", "parentInfo", "parent_info", "parentSessionId", "parent_session_id", "parentId", "parent_id"} {
		if v, ok := data[key].(string); ok {
			parent := normalizeParentSessionID(v)
			if parent != "" {
				return parent
			}
		}
	}
	return ""
}

func extractParentFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	if parent := extractParentFromMap(data); parent != "" {
		return parent
	}
	for _, key := range []string{"event", "native"} {
		if nested, ok := data[key].(map[string]interface{}); ok {
			if parent := extractParentFromMap(nested); parent != "" {
				return parent
			}
			if key == "native" {
				if nativeEvent, ok := nested["event"].(map[string]interface{}); ok {
					if parent := extractParentFromMap(nativeEvent); parent != "" {
						return parent
					}
				}
			}
		}
	}
	return ""
}

// 🔳️resolveParentSessionID holds the data fields for a resolveParentSessionID record.
func resolveParentSessionID(parentInfo string, input json.RawMessage) string {
	parent := normalizeParentSessionID(parentInfo)
	if parent != "" {
		return parent
	}
	return extractParentFromInput(input)
}

func decodeHookInputMap(input json.RawMessage) map[string]interface{} {
	if len(input) == 0 {
		return nil
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return nil
	}
	return data
}

func findNestedStringValue(value interface{}, keys ...string) string {
	switch typed := value.(type) {
	case map[string]interface{}:
		for _, key := range keys {
			if raw, ok := typed[key].(string); ok && strings.TrimSpace(raw) != "" {
				return strings.TrimSpace(raw)
			}
		}
		for _, nested := range typed {
			if found := findNestedStringValue(nested, keys...); found != "" {
				return found
			}
		}
	case []interface{}:
		for _, nested := range typed {
			if found := findNestedStringValue(nested, keys...); found != "" {
				return found
			}
		}
	}
	return ""
}

// 🔲️extractTranscriptFromInput holds the data fields for a extractTranscriptFromInput record.
func ExtractTranscriptFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "transcript", "transcript_path", "transcriptPath", "log_path", "logPath")
}

// ▪️extractMessageIDFromInput holds the data fields for a extractMessageIDFromInput record.
func extractMessageIDFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"messageId", "message_id", "turnId", "turn_id", "requestId", "request_id"} {
		if v, ok := data[key].(string); ok && v != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

func extractParentMessageIDFromInput(input json.RawMessage) string {
	if len(input) == 0 {
		return ""
	}
	var data map[string]interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return ""
	}
	for _, key := range []string{"parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id"} {
		if v, ok := data[key].(string); ok && v != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

// 🎫️ensureTicketAgent holds the data fields for a ensureTicketAgent record.
func ensureTicketAgent(ticket *model.Ticket, sessionID string, client string) int {
	for i := range ticket.Agents {
		if ticket.Agents[i].Session == sessionID {
			if ticket.Agents[i].Client == "" {
				ticket.Agents[i].Client = client
			}
			return i
		}
	}
	ticket.Agents = append(ticket.Agents, model.TicketAgent{
		Session:     sessionID,
		Contributor: contributors.GetGitAuthorAlias(),
		System:      languages.GetSystem(),
		Client:      client,
	})
	return len(ticket.Agents) - 1
}

// 📬️trackHookInOpenTicket updates agent metadata and plan in the latest open ticket.
func trackHookInOpenTicket(hctx model.HookContext, result model.HookResult) {
	ticket, err := todos.LatestOpenTicket()
	if err != nil || ticket == nil {
		return
	}
	sessionID := ExtractSessionIDFromInput(hctx.Input)
	if sessionID == "" {
		if len(ticket.Agents) > 0 {
			sessionID = ticket.Agents[len(ticket.Agents)-1].Session
		} else {
			sessionID = contributors.GenerateHookSessionID()
		}
	}
	agentIndex := ensureTicketAgent(ticket, sessionID, hctx.Client)
	agent := &ticket.Agents[agentIndex]
	llm := extractLLMFromInput(hctx.Input)
	if llm != "" && agent.LLM == "" {
		agent.LLM = llm
	}
	effort := extractEffortFromInput(hctx.Input)
	if effort != "" && agent.Effort == "" {
		agent.Effort = effort
	}
	transcript := ExtractTranscriptFromInput(hctx.Input)
	if transcript != "" && agent.Transcript == "" {
		agent.Transcript = transcript
	}
	if hctx.Event == model.HookAgentToolPlanUpdatingEnded {
		second := hctx.Second
		if second == "" {
			second = time.Now().UTC().Format(time.RFC3339)
		}
		if planResult, ok := result.(model.HookResultAgentToolPlanUpdating); ok && len(planResult.Steps) > 0 {
			var existing []model.TicketAgentPlanStep
			if agent.Plan != nil {
				existing = agent.Plan.Steps
			}
			merged := todos.MergeTicketAgentPlanSteps(existing, planResult.Steps, second)
			agent.Plan = &model.TicketAgentPlan{Steps: merged}
		}
	}
	_ = tickets.SaveTicket(ticket)
}

// ♻️computeCheckpointDiff computes a semantic code diff from git changes for a checkpoint.
func computeCheckpointDiff(repoRoot string, checkpointID string) *model.CheckpointDiff {
	diff := &model.CheckpointDiff{}
	stdout, _, exitCode := workspace.ExecCommand("git", []string{"diff-tree", "--no-commit-id", "-r", "--name-status", checkpointID}, repoRoot)
	if exitCode != 0 {
		return diff
	}
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		if len(parts) < 2 {
			continue
		}
		status := parts[0]
		filePath := parts[1]
		fileID := model.BuildFileID(filePath, nil)
		dir := filepath.Dir(filePath)
		folderID := ""
		if dir != "." && dir != "" {
			folderID = model.BuildFolderID(dir, nil)
		}
		var bundleID, technologyID string
		bundle := model.GetBundleByPath(filePath)
		if bundle != nil {
			bundleID = bundle.GetID()
			technologyName := bundle.TechnologyName
			if technologyName == "" {
				nameParts := strings.SplitN(bundle.Name, "/", 2)
				technologyName = nameParts[0]
			}
			pKind := model.DeriveTechnologyKind(technologyName)
			technologyID = model.EmojiText(string(pKind)) + workspace.Flat(technologyName)
		}
		switch {
		case status == "A":
			diff.Files.Created = todos.AppendUniqueString(diff.Files.Created, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case status == "D":
			diff.Files.Deleted = todos.AppendUniqueString(diff.Files.Deleted, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		case strings.HasPrefix(status, "R"):
			if len(parts) >= 3 {
				oldPath := parts[1]
				newPath := parts[2]
				oldFileID := model.BuildFileID(oldPath, nil)
				newFileID := model.BuildFileID(newPath, nil)
				diff.Files.Renamed = append(diff.Files.Renamed, model.CheckpointDiffRename{From: oldFileID, To: newFileID})
			} else {
				diff.Files.Modified = todos.AppendUniqueString(diff.Files.Modified, fileID)
			}
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		default:
			diff.Files.Modified = todos.AppendUniqueString(diff.Files.Modified, fileID)
			if folderID != "" {
				diff.Folders.Modified = todos.AppendUniqueString(diff.Folders.Modified, folderID)
			}
			if bundleID != "" {
				diff.Bundles.Modified = todos.AppendUniqueString(diff.Bundles.Modified, bundleID)
			}
			if technologyID != "" {
				diff.Technologies.Modified = todos.AppendUniqueString(diff.Technologies.Modified, technologyID)
			}
		}
	}
	return diff
}

// 🏪️storeCheckpointDiff stores a semantic code diff at .🦑️repo/🔀️/YY/MM/DD/<checkpoint-id>.json.
func storeCheckpointDiff(repoRoot string, checkpointID string) {
	if checkpointID == "" || checkpointID == "unknown" {
		return
	}
	diff := computeCheckpointDiff(repoRoot, checkpointID)
	now := time.Now().UTC()
	yy := model.FormatYearDir(now.Year() % 100)
	mm := model.FormatMonthDir(int(now.Month()))
	dd := model.FormatDayDir(now.Day())
	repoRoot = workspace.FindRepoRoot(repoRoot)
	diffDir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🔀️diff", yy, mm, dd)
	if err := os.MkdirAll(diffDir, 0755); err != nil {
		return
	}
	data, err := json.MarshalIndent(diff, "", "  ")
	if err != nil {
		return
	}
	_ = os.WriteFile(filepath.Join(diffDir, checkpointID+".json"), data, 0644)
}

// ✔️runCheckpointStartingHook holds the data fields for a runCheckpointStartingHook record.
func runCheckpointStartingHook(hctx model.HookContext) model.HookResultVersionCheckpointStarting {
	repoRoot := hctx.RepoRoot
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return model.HookResultVersionCheckpointStarting{HookResultBase: model.HookResultBase{Allowed: false, Message: fmt.Sprintf("cannot determine cwd: %v", err)}}
		}
		repoRoot = workspace.FindRepoRoot(cwd)
	}
	workspace.SetRootDir(repoRoot)
	return model.HookResultVersionCheckpointStarting{
		HookResultBase: model.HookResultBase{Allowed: true, Message: "checkpoint starting hook passed"},
		Checkpoint:     extractCheckpointSHAFromInput(hctx.Input),
		Description:    extractCheckpointMessageFromInput(hctx.Input, repoRoot),
	}
}

// runHookExecution runs the hook pipeline (shared by CLI `hook` and per-IDE `RunHookFor`).
func runHookExecution(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	return RunHookExecutionCtx(context.Background(), client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
}

func RunHookExecutionCtx(ctx context.Context, client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	done := make(chan error, 1)
	go func() {
		done <- runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
	}()
	select {
	case err := <-done:
		return err
	case <-ctx.Done():
		return fmt.Errorf("hook timed out: %w", ctx.Err())
	}
}

func runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	if toolName == "" {
		if tn := extractToolNameFromStdin(input); tn != "" {
			toolName = tn
		}
	}
	event, resolvedParent, err := ResolveHookEvent(eventStr, client, toolName, input)
	if err != nil {
		return err
	}
	if parentInfo == "" {
		parentInfo = resolvedParent
	}
	hctx := model.HookContext{
		Event:      event,
		Client:     client,
		Second:     time.Now().UTC().Format(time.RFC3339),
		RepoRoot:   repoRoot,
		ToolName:   toolName,
		ToolArgs:   toolArgs,
		FilePath:   filePath,
		ParentInfo: parentInfo,
		Input:      input,
	}
	result := RunHook(hctx)
	provider := providers.GetEditorProvider(client)
	if provider != nil && provider.Kind() == "copilot-chat" {
		hookEventName := extractHookEventNameFromStdin(input)
		if hookEventName == "" {
			hookEventName = provider.NativeEventFromHookEvent(event, parentInfo)
		}
		out := provider.FormatHookOutput(hookEventName, result)
		fmt.Fprintln(stdout, out)
		return nil
	}
	if jsonMode {
		out, _ := json.Marshal(result)
		fmt.Fprintln(stdout, string(out))
		return nil
	}
	if !result.IsAllowed() {
		fmt.Fprintln(stderr, result.GetMessage())
		return workspace.ExitError{Code: 2}
	}
	msg := result.GetMessage()
	if msg != "" {
		fmt.Fprintln(stdout, msg)
	}
	return nil
}

// #endregion 🦀️Hooks

// #region 🔷️Configure

// 🪝️repoManagedGitHooks lists git hook filenames this repo may install and must never leave enabled.
var repoManagedGitHooks = []string{
	"pre-commit",
	"post-commit",
	"prepare-commit-msg",
	"commit-msg",
	"pre-push",
	"pre-rebase",
	"post-merge",
	"post-checkout",
	"post-rewrite",
}

// 🔄️installMicroCommitHook copies a repo hook script into `.git/hooks` (must stay non-blocking).
func installMicroCommitHook(repoRoot, hookName string) error {
	if repoRoot == "" {
		return nil
	}
	src := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks", hookName)
	data, err := os.ReadFile(src)
	if err != nil {
		return fmt.Errorf("read micro-commit hook %s: %w", hookName, err)
	}
	hookPath := filepath.Join(repoRoot, ".git", "hooks", hookName)
	if err := os.WriteFile(hookPath, data, 0o755); err != nil {
		return fmt.Errorf("install micro-commit hook %s: %w", hookName, err)
	}
	return nil
}

// 🔄️installMicroCommitHooks installs micro-commit git hooks (prepare-commit-msg, post-commit reset).
func InstallMicroCommitHooks(repoRoot string) error {
	for _, name := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		if err := installMicroCommitHook(repoRoot, name); err != nil {
			return err
		}
	}
	return nil
}

// 🧹️removeGitHooks deletes repo-managed git hooks so commits, rebases, and squashes stay unblocked.
func RemoveGitHooks(repoRoot string) ([]string, error) {
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = workspace.FindRepoRoot(cwd)
	}
	if repoRoot == "" {
		return nil, nil
	}
	hooksDir := filepath.Join(repoRoot, ".git", "hooks")
	var removed []string
	for _, hookName := range repoManagedGitHooks {
		hookPath := filepath.Join(hooksDir, hookName)
		if err := os.Remove(hookPath); err != nil {
			if !os.IsNotExist(err) {
				return removed, fmt.Errorf("remove git hook %s: %w", hookName, err)
			}
			continue
		}
		removed = append(removed, hookName)
	}
	return removed, nil
}

// #endregion 🔷️Configure

// #region 🔖️MicroCommit

func RunMicroCommitScript(ctx context.Context, repoRoot string, args []string) error {
	bun, err := resolveBunBinary(repoRoot)
	if err != nil {
		return err
	}
	cmdArgs := append([]string{"./📜️script.ts", "micro-commit"}, args...)
	c := exec.CommandContext(ctx, bun, cmdArgs...)
	c.Dir = repoRoot
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	c.Stdin = os.Stdin
	if err := c.Run(); err != nil {
		var exitErr *exec.ExitError
		if errors.As(err, &exitErr) {
			return workspace.ExitError{Code: exitErr.ExitCode()}
		}
		return err
	}
	return nil
}

func resolveBunBinary(repoRoot string) (string, error) {
	if pinned := strings.TrimSpace(os.Getenv("COMPOSE_BUN")); pinned != "" {
		if _, err := os.Stat(pinned); err == nil {
			return pinned, nil
		}
	}
	pinPath := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "🐹️compose-micro-commit-bun")
	if data, err := os.ReadFile(pinPath); err == nil {
		if line := strings.TrimSpace(strings.Split(string(data), "\n")[0]); line != "" {
			if _, statErr := os.Stat(line); statErr == nil {
				return line, nil
			}
		}
	}
	if bunInstall := strings.TrimSpace(os.Getenv("BUN_INSTALL")); bunInstall != "" {
		for _, name := range []string{"bun", "bun.exe"} {
			candidate := filepath.Join(bunInstall, "bin", name)
			if _, err := os.Stat(candidate); err == nil {
				return candidate, nil
			}
		}
	}
	for _, rel := range []string{"node_modules/.bin/bun", "node_modules/.bin/bun.exe"} {
		candidate := filepath.Join(repoRoot, rel)
		if _, err := os.Stat(candidate); err == nil {
			return candidate, nil
		}
	}
	if path, err := exec.LookPath("bun"); err == nil {
		return path, nil
	}
	return "", fmt.Errorf("bun not found — install bun or set COMPOSE_BUN")
}

// #endregion 🔖️MicroCommit

// #region 💾️Missing Utility Functions

// 🧩️BlockedToolPatterns are patterns for blocked tools.
var BlockedToolPatterns = map[string]bool{
	"rm":              true,
	"rmdir":           true,
	"mv":              true,
	"cp":              true,
	"git add":         true,
	"git branch":      true,
	"git checkout":    true,
	"git cherry-pick": true,
	"git clean":       true,
	"git clone":       true,
	"git commit":      true,
	"git config":      true,
	"git fetch":       true,
	"git init":        true,
	"git merge":       true,
	"git mv":          true,
	"git pull":        true,
	"git push":        true,
	"git rebase":      true,
	"git remote":      true,
	"git reset":       true,
	"git restore":     true,
	"git revert":      true,
	"git rm":          true,
	"git stash":       true,
	"git switch":      true,
	"git tag":         true,
}

// ✔️IsToolBlocked checks if a tool is blocked.
func IsToolBlocked(tool string, args string) (bool, string) {
	for _, segment := range workspace.SplitCommandSegments(args) {
		if blocked, reason := isCommandSegmentBlocked(segment); blocked {
			return true, reason + "; other developers and agents may be editing the same files concurrently"
		}
	}
	tool = strings.ToLower(strings.TrimSpace(tool))
	if pattern, ok := BlockedToolPatterns[tool]; ok && pattern {
		return true, "blocked: " + tool + "; other developers and agents may be editing the same files concurrently"
	}
	return false, ""
}

// 🐙️isCommandSegmentBlocked checks if a command segment is blocked.
// ⌨️blockedGitVerbs are git subcommands that modify repository state.
var blockedGitVerbs = map[string]bool{
	"add": true, "branch": true, "checkout": true, "cherry-pick": true, "clone": true,
	"commit": true, "config": true, "fetch": true, "init": true, "merge": true,
	"mv": true, "pull": true, "push": true, "rebase": true, "remote": true,
	"reset": true, "restore": true, "revert": true, "rm": true, "stash": true,
	"switch": true, "tag": true, "clean": true,
}

// 🎯️blockedKillLsofPortPattern matches kill commands that use lsof to select a TCP port PID.
// This is intentionally denied because in containerized environments the matched PID can be critical (e.g. PID 1),
// ⏹️causing the entire devcontainer to terminate and stopping all running work.
var blockedKillLsofPortPattern = regexp.MustCompile(`(?i)\bkill\b[\s\S]*\$\(\s*lsof\b[\s\S]*-t[\s\S]*-i\s*:\s*\d+[\s\S]*\)`)

// 🔤️blockedGitVerbPattern matches any blocked git verb in inline code strings (shell-style invocation).
var blockedGitVerbPattern = regexp.MustCompile(`(?i)\bgit\s+(add|branch|checkout|cherry-pick|clone|commit|config|fetch|init|merge|mv|pull|push|rebase|remote|reset|restore|revert|rm|stash|switch|tag|clean)\b`)

// 📚️blockedGitListPattern matches array/list-style git invocations like ['git', 'stash'] or ["git", "checkout"].
var blockedGitListPattern = regexp.MustCompile(`(?i)['"]\s*git\s*['"]\s*,\s*['"]\s*(add|branch|checkout|cherry-pick|clone|commit|config|fetch|init|merge|mv|pull|push|rebase|remote|reset|restore|revert|rm|stash|switch|tag|clean)\s*['"]`)

// 🌿️containsBlockedGitInCode scans arbitrary inline code (e.g. python -c, node -e) for blocked git invocations.
func containsBlockedGitInCode(code string) (bool, string) {
	if m := blockedGitVerbPattern.FindString(code); m != "" {
		return true, "blocked: " + strings.ToLower(strings.TrimSpace(m))
	}
	if m := blockedGitListPattern.FindString(code); m != "" {
		return true, "blocked: git (list form) in inline code"
	}
	return false, ""
}

func isCommandSegmentBlocked(segment string) (bool, string) {
	segment = strings.TrimSpace(segment)
	if segment == "" {
		return false, ""
	}
	lower := strings.ToLower(segment)
	if blockedKillLsofPortPattern.MatchString(lower) {
		return true, "blocked: kill $(lsof -t -i:PORT); this can match PID 1 in containers and terminate the devcontainer, stopping all running work"
	}
	allowedPrefixes := []string{"grep ", "rg ", "ripgrep ", "echo ", "printf ", "ls ", "pwd", "cat ", "sed ", "awk "}
	for _, prefix := range allowedPrefixes {
		if strings.HasPrefix(lower, prefix) {
			return false, ""
		}
	}
	tokens := strings.Fields(lower)
	for len(tokens) > 0 {
		first := tokens[0]
		if strings.Contains(first, "=") {
			tokens = tokens[1:]
			continue
		}
		if first == "env" || first == "command" || first == "sudo" {
			tokens = tokens[1:]
			continue
		}
		// Shell interpreters: scan full joined string for git invocations.
		if first == "bash" || first == "sh" || first == "zsh" || first == "fish" ||
			first == "ksh" || first == "csh" || first == "tcsh" || first == "dash" {
			joined := strings.Join(tokens, " ")
			if idx := strings.Index(joined, "git "); idx >= 0 {
				return isCommandSegmentBlocked(joined[idx:])
			}
			return false, ""
		}
		// Script interpreters with inline code flags (-c, -e): scan inline code for git.
		if first == "python" || first == "python3" || first == "python2" ||
			first == "node" || first == "nodejs" ||
			first == "perl" || first == "ruby" || first == "php" ||
			first == "lua" || first == "tclsh" || first == "groovy" || first == "scala" {
			// Extract the inline code argument (after -c or -e flag).
			joined := strings.Join(tokens[1:], " ")
			return containsBlockedGitInCode(joined)
		}
		// xargs can forward arguments to git directly.
		if first == "xargs" {
			joined := strings.Join(tokens, " ")
			if idx := strings.Index(joined, "git "); idx >= 0 {
				return isCommandSegmentBlocked(joined[idx:])
			}
			return false, ""
		}
		break
	}
	if len(tokens) == 0 {
		return false, ""
	}
	if strings.HasSuffix(tokens[0], "/git") {
		tokens[0] = "git"
	}
	if tokens[0] != "git" {
		return false, ""
	}
	verbIndex := 1
	for verbIndex < len(tokens) && strings.HasPrefix(tokens[verbIndex], "-") {
		verbIndex++
		if verbIndex < len(tokens) && !strings.HasPrefix(tokens[verbIndex], "-") && (tokens[verbIndex-1] == "-c" || tokens[verbIndex-1] == "-C") {
			verbIndex++
		}
	}
	if verbIndex >= len(tokens) {
		return false, ""
	}
	verb := strings.Trim(tokens[verbIndex], `"'`)
	if !blockedGitVerbs[verb] {
		return false, ""
	}
	if verb == "clean" && !strings.Contains(lower, "-fd") && !strings.Contains(lower, "-df") {
		return false, ""
	}
	return true, "blocked: git " + verb
}

// 📡️resolveEventSessionID resolves the session ID from input.
func ResolveEventSessionID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 📝️resolveEventSecondID resolves the second ID from input.
func ResolveEventSecondID(input string) string {
	if input == "" {
		return ""
	}
	return input
}

// 🗺️extractToolInputMapFromData extracts tool input map from data.
func extractToolInputMapFromData(data interface{}) map[string]interface{} {
	if m, ok := data.(map[string]interface{}); ok {
		if toolInput, ok := m["tool_input"].(map[string]interface{}); ok {
			return toolInput
		}
		if event, ok := m["event"].(map[string]interface{}); ok {
			if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
				return toolInput
			}
		}
		if native, ok := m["native"].(map[string]interface{}); ok {
			if event, ok := native["event"].(map[string]interface{}); ok {
				if toolInput, ok := event["tool_input"].(map[string]interface{}); ok {
					return toolInput
				}
			}
		}
		return nil
	}
	return nil
}

// 🔶️looksLikeFilePath checks if a string looks like a file path.
func looksLikeFilePath(s string) bool {
	// Check if string contains path separators or file extensions
	return strings.Contains(s, "/") || strings.Contains(s, "\\") ||
		strings.Contains(s, ".") && !strings.HasPrefix(s, ".")
}

// 🔹️extractCommandFromStdin extracts command from stdin.
func extractCommandFromStdin(data json.RawMessage) string {
	var cmd string
	if err := json.Unmarshal(data, &cmd); err == nil {
		return strings.TrimSpace(cmd)
	}
	decoded := decodeHookInputMap(data)
	if decoded == nil {
		return ""
	}
	if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
		if value, ok := toolInput["command"].(string); ok {
			return strings.TrimSpace(value)
		}
	}
	return strings.TrimSpace(findNestedStringValue(decoded, "command", "command_line", "commandLine"))
}

// 🔸️extractCommandCwdFromInput extracts command and cwd from input.
func extractCommandCwdFromInput(input json.RawMessage) (string, string) {
	decoded := decodeHookInputMap(input)
	command := extractCommandFromStdin(input)
	cwd := ""
	if decoded != nil {
		if toolInput := extractToolInputMapFromData(decoded); len(toolInput) > 0 {
			if value, ok := toolInput["cwd"].(string); ok {
				cwd = strings.TrimSpace(value)
			}
		}
		if cwd == "" {
			cwd = strings.TrimSpace(findNestedStringValue(decoded, "cwd", "working_directory", "workingDirectory"))
		}
	}
	if cwd == "" {
		cwd, _ = os.Getwd()
	}
	return command, cwd
}

// #endregion 💾️Missing Utility Functions

// #region 📰️Todos

// 🧲️extractSessionIDFromInput extracts session ID from input.
func ExtractSessionIDFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	sessionID := findNestedStringValue(
		data,
		"session_id",
		"sessionId",
		"trajectory_id",
		"trajectoryId",
		"conversation_id",
		"conversationId",
		"agent_id",
		"agentId",
	)
	if sessionID != "" {
		return sessionID
	}
	transcript := ExtractTranscriptFromInput(input)
	if transcript == "" {
		return ""
	}
	base := strings.TrimSpace(filepath.Base(transcript))
	ext := filepath.Ext(base)
	if ext != "" {
		base = strings.TrimSuffix(base, ext)
	}
	return strings.TrimSpace(base)
}

// 📝️extractLLMFromInput extracts LLM from input.
func extractLLMFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "llm", "model", "model_name", "modelName")
}

// 🏋️extractEffortFromInput extracts reasoning effort from input.
func extractEffortFromInput(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "effort", "reasoning_effort", "reasoningEffort")
}

// 🔷️extractToolNameFromStdin extracts tool name from stdin.
func extractToolNameFromStdin(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data, "tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName")
}

// 📡️extractHookEventNameFromStdin extracts hook event name from stdin.
func extractHookEventNameFromStdin(input json.RawMessage) string {
	data := decodeHookInputMap(input)
	if data == nil {
		return ""
	}
	return findNestedStringValue(data,
		"hookEventName",
		"hook_event_name",
		"hook_event",
		"hookEvent",
		"event",
	)
}

// 🔶️ResolveHookEvent resolves a hook event.
func ResolveHookEvent(name string, client string, toolName string, input json.RawMessage) (model.HookEvent, string, error) {
	if event, err := model.ValidateHookEvent(name); err == nil {
		return event, "", nil
	}
	kind := model.ToolKindGeneric
	if strings.TrimSpace(toolName) != "" {
		kind = todos.ClassifyTool(toolName)
	}
	if command := extractCommandFromStdin(input); command != "" {
		segment, _ := testrunner.ExtractTestSegmentFromCommand(command)
		commandKind := model.ClassifyCommandKind(todos.FirstNonEmpty(segment, command))
		if kind == model.ToolKindGeneric || kind == model.ToolKindTerminal || commandKind == model.ToolKindTest || commandKind == model.ToolKindBuild || commandKind == model.ToolKindCodeSearch || commandKind == model.ToolKindCodeEdit {
			kind = commandKind
		}
	}
	if toolName == "Glob" && len(input) > 0 {
		kind = model.ToolKindGeneric
	}
	provider := providers.GetEditorProvider(client)
	if provider != nil {
		return provider.ResolveNativeEvent(name, kind)
	}
	return providers.ResolveClaudeCompatibleEvent(name, kind)
}

// 🔹️RunHook runs a hook.
func RunHook(ctx model.HookContext) model.HookResult {
	previousRoot := workspace.RootDir
	defer func() {
		workspace.RootDir = previousRoot
		workspace.GitignoreMutex.Lock()
		workspace.CachedGitignore = nil
		workspace.GitignoreLoaded = false
		workspace.GitignoreMutex.Unlock()
		codebase.InvalidateTechnologyCache()
	}()
	result := runHookCore(ctx)
	writeHookArtifacts(ctx, result, SystemEnvironment{})
	trackHookInOpenTicket(ctx, result)
	return result
}

func runHookCore(ctx model.HookContext) model.HookResult {
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = workspace.GetRootDir()
	}
	if repoRoot != "" {
		workspace.SetRootDir(repoRoot)
	}
	result := dispatchHook(ctx)
	hookResult, ok := result.(model.HookResult)
	if !ok {
		return model.HookResultBase{Allowed: false, Message: "invalid hook result"}
	}
	return hookResult
}

func writeHookArtifacts(ctx model.HookContext, result model.HookResult, environment HookEnvironment) {
	if HookEventKind(ctx.Event) == model.HookKindVersion {
		return
	}
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = workspace.GetRootDir()
	}
	if repoRoot == "" {
		return
	}
	repoRoot = workspace.FindRepoRoot(repoRoot)
	workspace.SetRootDir(repoRoot)
	cfg := workspace.LoadRepoConfig(repoRoot)
	if !cfg.Logging.Session {
		return
	}
	now := time.Now().UTC()
	sessionID := ExtractSessionIDFromInput(ctx.Input)
	if sessionID == "" && ctx.Client == "kiro-cli" {
		sessionID = fmt.Sprintf("kiro-%d", os.Getppid())
	}
	if sessionID == "" {
		sessionID = "unknown"
	}
	logDir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🤖️generated",
		model.FormatYearDir(now.Year()%100),
		model.FormatMonthDir(int(now.Month())),
		model.FormatDayDir(now.Day()),
		sessionID,
	)
	if err := os.MkdirAll(logDir, 0o755); err != nil {
		return
	}
	writeSessionHookLog(ctx, result, logDir, sessionID, cfg.Logging, environment)
	if cfg.Logging.Operations {
		logRepoOperationHook(ctx, result, logDir, now, sessionID, cfg.Logging)
	}
}

// 📓️writeSessionHookLog records one hook invocation through the session store port, so the
// filesystem is one implementation of where a session log lives rather than the only one.
func writeSessionHookLog(ctx model.HookContext, result model.HookResult, logDir string, sessionID string, lg workspace.LoggingConfig, environment HookEnvironment) {
	store := NewDirectorySessionStore(filepath.Dir(logDir))
	_, _ = RecordSessionHook(store, HookContextOf(ctx), HookResultOf(result), sessionID, lg, environment)
}

// 🎯️HookContextOf lifts the model-level hook context into the port-isolated one this module dispatches on.
func HookContextOf(ctx model.HookContext) HookContext {
	return HookContext{Event: string(ctx.Event), Client: ctx.Client, Second: ctx.Second, RepoRoot: ctx.RepoRoot, ToolName: ctx.ToolName, ToolArgs: ctx.ToolArgs, FilePath: ctx.FilePath, ParentInfo: ctx.ParentInfo, Extra: ctx.Extra, Input: ctx.Input}
}

// 🔶️HookResultOf reduces a typed hook result to the flat record the session log and the formatter read.
func HookResultOf(result model.HookResult) HookResult {
	return HookResult{Allowed: result.IsAllowed(), Message: result.GetMessage()}
}

// ⬛️logRepoOperationHook logs repo operation hook.
func logRepoOperationHook(hctx model.HookContext, result interface{}, operation string, t time.Time, msg string, lg workspace.LoggingConfig) {
	if !lg.Operations {
		return
	}
	logDir := strings.TrimSpace(operation)
	sessionID := strings.TrimSpace(msg)
	if logDir == "" || sessionID == "" {
		return
	}
	resultToolName, resultCommand := todos.ExtractHookResultToolInfo(result)
	toolName := todos.FirstNonEmpty(resultToolName, hctx.ToolName, extractToolNameFromStdin(hctx.Input))
	repoOperation := todos.DeriveRepoOpFromMCPTool(toolName)
	if repoOperation == "" {
		repoOperation = todos.DeriveRepoOpFromCLICommand(todos.FirstNonEmpty(resultCommand, hctx.ToolArgs, extractCommandFromStdin(hctx.Input)))
	}
	if repoOperation == "" {
		return
	}
	phase := "starting"
	switch hctx.Event {
	case model.HookAgentToolEnded, model.HookAgentToolPlanUpdatingEnded, model.HookAgentToolSearchEnded, model.HookAgentToolCodeEditEnded, model.HookAgentToolTestEnded, model.HookAgentToolBuildEnded, model.HookAgentToolTerminalEnded:
		phase = "ended"
	}
	metaPath := filepath.Join(logDir, "session.json")
	var meta model.SessionMeta
	if data, err := os.ReadFile(metaPath); err == nil {
		_ = json.Unmarshal(data, &meta)
	}
	inputMap := decodeHookInputMap(hctx.Input)
	second := todos.FirstNonEmpty(findNestedStringValue(inputMap, "second"), hctx.Second, t.UTC().Format(time.RFC3339))
	checkpoint := todos.ResolveEventCheckpointID(extractCheckpointSHAFromInput(hctx.Input))
	eventJSON, _ := json.Marshal(map[string]interface{}{
		"kind":        "agent." + repoOperation + "." + phase,
		"client":      hctx.Client,
		"session":     ResolveEventSessionID(sessionID),
		"second":      ResolveEventSecondID(second),
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	})
	entry := model.EventEntry{Event: eventJSON}
	meta.Events = append(meta.Events, entry)
	_ = workspace.WriteJSONFile(metaPath, meta)
}

func extractSearchDefinitionReadsFromInput(input json.RawMessage, toolArgs string) []model.HookSearchDefinition {
	pages, ranges := todos.ExtractSearchFromInput(input, toolArgs)
	_ = pages
	filesToLines := map[string]map[int]struct{}{}
	addLine := func(path string, line int) {
		path = workspace.NormalizeHookPath(path)
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
			commandFiles, pattern := todos.ExtractSearchCommandFilesAndPattern(commandLine, cwd)
			for _, filePath := range commandFiles {
				matchedLines := todos.SearchLinesInFile(filePath, pattern)
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

func buildDefinitionReads(filesToLines map[string]map[int]struct{}) []model.HookSearchDefinition {
	var result []model.HookSearchDefinition
	for filePath, lines := range filesToLines {
		normalized := workspace.NormalizeHookPath(filePath)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(workspace.GetRootDir(), normalized)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		if _, ok := lines[0]; ok {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
			continue
		}
		lineCount := strings.Count(string(content), "\n") + 1
		if len(lines) == 0 || spansEntireFile(lines, string(content), lineCount) {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
			continue
		}
		definitions := languages.ParseDefinitions(string(content), normalized)
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
				result = append(result, model.HookSearchDefinition{ID: definition.GetID(), Loc: loc})
			}
		}
		if !matched {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
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
	segments := workspace.SplitCommandSegments(command)
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

func dispatchHook(ctx model.HookContext) interface{} {
	base := model.HookResultAgentBase{
		HookResultBase: model.HookResultBase{Allowed: true},
		Checkpoint:     extractCheckpointSHAFromInput(ctx.Input),
		Session:        ExtractSessionIDFromInput(ctx.Input),
		Second:         todos.FirstNonEmpty(findNestedStringValue(decodeHookInputMap(ctx.Input), "second"), ctx.Second),
		Client:         ctx.Client,
		LLM:            extractLLMFromInput(ctx.Input),
		Effort:         extractEffortFromInput(ctx.Input),
		Transcript:     ExtractTranscriptFromInput(ctx.Input),
		MessageID:      extractMessageIDFromInput(ctx.Input),
		Parent:         resolveParentSessionID("", ctx.Input),
	}
	if ctx.ParentInfo != "" {
		base.Parent = resolveParentSessionID(ctx.ParentInfo, ctx.Input)
		if ctx.ParentInfo == "subagent" {
			if agentID := findNestedStringValue(decodeHookInputMap(ctx.Input), "agent_id", "agentId"); agentID != "" {
				parentSession := extractParentFromInput(ctx.Input)
				if parentSession == "" {
					parentSession = ExtractSessionIDFromInput(ctx.Input)
				}
				base.Session = agentID
				base.Parent = normalizeParentSessionID(parentSession)
			} else if base.Parent == "" {
				base.Parent = base.Session
			}
		}
	}
	if len(ctx.Input) > 0 {
		var raw map[string]interface{}
		if err := json.Unmarshal(ctx.Input, &raw); err == nil {
			base.Raw = raw
		}
	}
	switch ctx.Event {
	case model.HookAgentStarted:
		return model.HookResultAgentStarted{HookResultAgentBase: base}
	case model.HookAgentEnded:
		return model.HookResultAgentEnded{HookResultAgentBase: base, Report: extractReportFromInput(ctx.Input)}
	case model.HookAgentPromptSubmitting:
		return model.HookResultAgentPromptSubmitting{HookResultAgentBase: base, Prompt: findNestedStringValue(decodeHookInputMap(ctx.Input), "prompt", "text")}
	case model.HookAgentCompacting:
		return model.HookResultAgentCompacting{HookResultAgentBase: base, Chat: extractChatFromInput(ctx.Input)}
	case model.HookAgentToolStarting:
		toolName := todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input))
		toolInput := todos.ExtractToolInputFromStdin(ctx.Input)
		blocked, reason := IsToolBlocked(toolName, todos.FirstNonEmpty(ctx.ToolArgs, extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs), extractCommandFromStdin(ctx.Input)))
		base.Allowed = !blocked
		base.Message = reason
		return model.HookResultAgentToolStarting{HookResultAgentBase: base, Name: toolName, Input: toolInput}
	case model.HookAgentToolEnded:
		response := todos.ExtractToolResponseFromStdin(ctx.Input)
		if len(response) == 0 {
			if output := findNestedStringValue(decodeHookInputMap(ctx.Input), "tool_output", "toolOutput", "output", "response"); output != "" {
				response, _ = json.Marshal(output)
			}
		}
		return model.HookResultAgentToolEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Response: response}
	case model.HookAgentToolPlanUpdatingStarting, model.HookAgentToolPlanUpdatingEnded:
		return model.HookResultAgentToolPlanUpdating{HookResultAgentBase: base, Steps: todos.ExtractPlanStepsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolSearchStarting:
		pages, ranges := todos.ExtractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolSearchStarting{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolSearchEnded:
		pages, ranges := todos.ExtractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolSearchEnded{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolCodeEditStarting:
		path, oldText, newText, all := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolCodeEditStarting{HookResultAgentBase: base, Path: path, Old: oldText, New: newText, All: all}
	case model.HookAgentToolCodeEditEnded:
		path, oldText, newText, _ := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		// Auto-format the edited file.
		if path != "" {
			statutes.RunFormatterForFile(path)
		}
		return model.HookResultAgentToolCodeEditEnded{HookResultAgentBase: base, Path: path, Old: oldText, New: newText}
	case model.HookAgentToolTestStarting:
		labs, tests, timeout := extractTestStartingFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolTestStarting{HookResultAgentBase: base, Labs: labs, Tests: tests, Timeout: timeout}
	case model.HookAgentToolTestEnded:
		files, succeeded, failed := extractTestEndedFromInput(ctx.Input)
		return model.HookResultAgentToolTestEnded{HookResultAgentBase: base, Files: files, Succeeded: succeeded, Failed: failed}
	case model.HookAgentToolBuildStarting:
		bundles := extractBuildBundlesFromInput(ctx.Input, ctx.ToolArgs)
		payload, _ := json.Marshal(map[string]interface{}{"bundles": bundles})
		return model.HookResultAgentToolStarting{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, "build"), Input: payload}
	case model.HookAgentToolBuildEnded:
		succeeded, failed := extractBuildEndedFromInput(ctx.Input)
		payload, _ := json.Marshal(map[string]interface{}{"succeeded": succeeded, "failed": failed})
		return model.HookResultAgentToolEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, "build"), Response: payload}
	case model.HookAgentToolTerminalStarting:
		command := extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs)
		blocked, reason := IsToolBlocked(todos.FirstNonEmpty(ctx.ToolName, "run_in_terminal"), todos.FirstNonEmpty(command, ctx.ToolArgs))
		base.Allowed = !blocked
		base.Message = reason
		return model.HookResultAgentToolTerminalStarting{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Command: command}
	case model.HookAgentToolTerminalEnded:
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(ctx.Input)
		pidValue, _ := strconv.Atoi(pid)
		stdoutJSON, _ := json.Marshal(stdout)
		stderrJSON, _ := json.Marshal(stderr)
		return model.HookResultAgentToolTerminalEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Output: stdoutJSON, Command: command, PID: pidValue, Terminated: terminated, Stdout: stdoutJSON, Stderr: stderrJSON}
	case model.HookAgentThinkingStarting:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return model.HookResultAgentThinkingStarting{HookResultAgentBase: base}
	case model.HookAgentThinkingEnded:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return model.HookResultAgentThinkingEnded{HookResultAgentBase: base}
	case model.HookVersionCheckpointStarting:
		return runCheckpointStartingHook(ctx)
	case model.HookVersionCheckpointEnded:
		return model.HookResultVersionCheckpointEnded{HookResultBase: model.HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input), Description: extractCheckpointMessageFromInput(ctx.Input, ctx.RepoRoot)}
	case model.HookVersionCheckinStarting, model.HookVersionCheckinEnded, model.HookVersionCheckoutStarting, model.HookVersionCheckoutEnded:
		return model.HookResultVersionCheckpointEnded{HookResultBase: model.HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input)}
	default:
		return model.HookResultBase{Allowed: false, Message: "unknown event: " + string(ctx.Event)}
	}
}

// #endregion 📰️Todos

// #endregion 🚚️Split
