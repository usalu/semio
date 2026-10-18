// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔶️dispatch carries the hook result, the hook dispatch and the hook output formatting.

// #endregion 🧲️Header

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	todos "github.com/usalu/semio/repo/todos"
)

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
