// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🦀️hooks carries the hook events, contexts and results.

// #endregion 🧲️Header

package model

import (
	json "encoding/json"
	fmt "fmt"
	strings "strings"
)

// #region 🦀️Hooks

// 📡️HookEvent represents a lifecycle event kind for hooks.
type HookEvent string

const HookVersionCheckpointStarting HookEvent = "version.checkpoint.starting"

const HookVersionCheckpointEnded HookEvent = "version.checkpoint.ended"

const HookVersionCheckinStarting HookEvent = "version.checkin.starting"

const HookVersionCheckinEnded HookEvent = "version.checkin.ended"

const HookVersionCheckoutStarting HookEvent = "version.checkout.starting"

const HookVersionCheckoutEnded HookEvent = "version.checkout.ended"

const HookAgentStarted HookEvent = "agent.started"

const HookAgentEnded HookEvent = "agent.ended"

const HookAgentPromptSubmitting HookEvent = "agent.prompt.submitting"

const HookAgentCompacting HookEvent = "agent.compacting"

const HookAgentToolStarting HookEvent = "agent.tool.starting"

const HookAgentToolEnded HookEvent = "agent.tool.ended"

const HookAgentToolPlanUpdatingStarting HookEvent = "agent.tool.plan.updating.starting"

const HookAgentToolPlanUpdatingEnded HookEvent = "agent.tool.plan.updating.ended"

const HookAgentToolSearchStarting HookEvent = "agent.file.read.starting"

const HookAgentToolSearchEnded HookEvent = "agent.file.read.ended"

const HookAgentToolCodeEditStarting HookEvent = "agent.tool.code.edit.starting"

const HookAgentToolCodeEditEnded HookEvent = "agent.tool.code.edit.ended"

const HookAgentToolTestStarting HookEvent = "agent.tool.test.starting"

const HookAgentToolTestEnded HookEvent = "agent.tool.test.ended"

const HookAgentToolBuildStarting HookEvent = "agent.tool.build.starting"

const HookAgentToolBuildEnded HookEvent = "agent.tool.build.ended"

const HookAgentToolTerminalStarting HookEvent = "agent.tool.terminal.starting"

const HookAgentToolTerminalEnded HookEvent = "agent.tool.terminal.ended"

const HookAgentThinkingStarting HookEvent = "agent.thinking.starting"

const HookAgentThinkingEnded HookEvent = "agent.thinking.ended"

// 📋️AllHookEvents lists every valid hook event slug.
var AllHookEvents = []HookEvent{
	HookVersionCheckpointStarting,
	HookVersionCheckpointEnded,
	HookVersionCheckinStarting,
	HookVersionCheckinEnded,
	HookVersionCheckoutStarting,
	HookVersionCheckoutEnded,
	HookAgentStarted,
	HookAgentEnded,
	HookAgentPromptSubmitting,
	HookAgentCompacting,
	HookAgentToolStarting,
	HookAgentToolEnded,
	HookAgentToolPlanUpdatingStarting,
	HookAgentToolPlanUpdatingEnded,
	HookAgentToolSearchStarting,
	HookAgentToolSearchEnded,
	HookAgentToolCodeEditStarting,
	HookAgentToolCodeEditEnded,
	HookAgentToolTestStarting,
	HookAgentToolTestEnded,
	HookAgentToolBuildStarting,
	HookAgentToolBuildEnded,
	HookAgentToolTerminalStarting,
	HookAgentToolTerminalEnded,
	HookAgentThinkingStarting,
	HookAgentThinkingEnded,
}

// 🏷️HookKind categorizes a hook as either a git hook or an agent hook.
type HookKind string

const HookKindVersion HookKind = "version"

const HookKindAgent HookKind = "agent"

// 🎯️HookContext carries event metadata and a codebase handle for hook handlers.
type HookContext struct {
	Event      HookEvent         `json:"event"`
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

// 🔁️HookPlanStep represents a single step in a plan/task list update event.
type HookPlanStep struct {
	Name   string `json:"name"`
	Status string `json:"status"`
}

// 🔶️HookResult represents the outcome of a hook invocation with event-specific data.
type HookResult interface {
	IsAllowed() bool
	GetMessage() string
}

// 💻️HookResultBase provides common fields for all hook results.
type HookResultBase struct {
	Allowed bool   `json:"allowed"`
	Message string `json:"message,omitempty"`
	Raw     any    `json:"-"`
}

// 💿️IsAllowed holds the data fields for a IsAllowed record.
func (h HookResultBase) IsAllowed() bool { return h.Allowed }

// 🔹️GetMessage holds the data fields for a GetMessage record.
func (h HookResultBase) GetMessage() string { return h.Message }

// 🔸️HookResultAgentBase provides shared fields for all agent hook results.
type HookResultAgentBase struct {
	HookResultBase
	Checkpoint string `json:"checkpoint,omitempty"`
	Session    string `json:"session,omitempty"`
	Second     string `json:"second,omitempty"`
	Client     string `json:"client,omitempty"`
	LLM        string `json:"llm,omitempty"`
	Effort     string `json:"effort,omitempty"`
	Transcript string `json:"transcript,omitempty"`
	MessageID  string `json:"message,omitempty"`
	Parent     string `json:"parent"`
}

type HookResultVersionCheckpointStarting struct {
	HookResultBase
	Checkpoint  string `json:"checkpoint,omitempty"`
	Description string `json:"description,omitempty"`
}

type HookResultVersionCheckpointEnded struct {
	HookResultBase
	Checkpoint  string `json:"checkpoint,omitempty"`
	Description string `json:"description,omitempty"`
}

// ▶️HookResultAgentStarted represents the result of an agent started event.
type HookResultAgentStarted struct {
	HookResultAgentBase
}

// 🔺️HookResultAgentEnded represents the result of an agent ended event.
type HookResultAgentEnded struct {
	HookResultAgentBase
	Report string `json:"report,omitempty"`
}

// 🔻️HookResultAgentPromptSubmitting represents the result of an agent prompt submitting event.
type HookResultAgentPromptSubmitting struct {
	HookResultAgentBase
	Prompt string `json:"prompt,omitempty"`
}

// ⬛️HookResultAgentCompacting represents the result of an agent compacting event.
type HookResultAgentCompacting struct {
	HookResultAgentBase
	Chat string `json:"chat,omitempty"`
}

// ⬜️HookResultAgentToolStarting represents the result of an agent tool starting event.
type HookResultAgentToolStarting struct {
	HookResultAgentBase
	Name  string          `json:"name,omitempty"`
	Input json.RawMessage `json:"input,omitempty"`
}

// 🟥️HookResultAgentToolEnded represents the result of an agent tool ended event.
type HookResultAgentToolEnded struct {
	HookResultAgentBase
	Name     string          `json:"name,omitempty"`
	Input    json.RawMessage `json:"input,omitempty"`
	Response json.RawMessage `json:"response,omitempty"`
}

// 🟧️HookResultAgentToolPlanUpdating represents the result of an agent tool plan updating event.
type HookResultAgentToolPlanUpdating struct {
	HookResultAgentBase
	Steps []HookPlanStep `json:"steps,omitempty"`
}

// 🔎️HookResultAgentToolSearchStarting represents the result of an agent tool search starting event.
type HookResultAgentToolSearchStarting struct {
	HookResultAgentBase
	Pages       []string               `json:"pages,omitempty"`
	Ranges      []string               `json:"ranges,omitempty"`
	Definitions []HookSearchDefinition `json:"definitions,omitempty"`
}

// 🟨️HookResultAgentToolSearchEnded represents the result of an agent tool search ended event.
type HookResultAgentToolSearchEnded struct {
	HookResultAgentBase
	Pages       []string               `json:"pages,omitempty"`
	Ranges      []string               `json:"ranges,omitempty"`
	Definitions []HookSearchDefinition `json:"definitions,omitempty"`
	Error       string                 `json:"error,omitempty"`
}

// 📖️HookSearchDefinition holds search line coverage for a resolved definition.
type HookSearchDefinition struct {
	ID  string `json:"id,omitempty"`
	Loc int    `json:"loc,omitempty"`
}

// 🟩️HookResultAgentToolCodeEditStarting represents the result of an agent tool code edit starting event.
type HookResultAgentToolCodeEditStarting struct {
	HookResultAgentBase
	Path string `json:"path,omitempty"`
	Old  string `json:"old,omitempty"`
	New  string `json:"new,omitempty"`
	All  bool   `json:"all,omitempty"`
}

// 🟦️HookResultAgentToolCodeEditEnded represents the result of an agent tool code edit ended event.
type HookResultAgentToolCodeEditEnded struct {
	HookResultAgentBase
	Path string `json:"path,omitempty"`
	Old  string `json:"old,omitempty"`
	New  string `json:"new,omitempty"`
}

// 🧪️HookResultAgentToolTestStarting represents the result of an agent tool test starting event.
type HookResultAgentToolTestStarting struct {
	HookResultAgentBase
	Labs    []string `json:"labs,omitempty"`
	Tests   []string `json:"tests,omitempty"`
	Timeout string   `json:"timeout,omitempty"`
}

// 🟪️HookResultAgentToolTestEnded represents the result of an agent tool test ended event.
type HookResultAgentToolTestEnded struct {
	HookResultAgentBase
	Files     []string `json:"files,omitempty"`
	Succeeded []string `json:"succeeded,omitempty"`
	Failed    []string `json:"failed,omitempty"`
}

// 🔤️ValidateHookEvent checks if the given string is a valid hook event.
func ValidateHookEvent(s string) (HookEvent, error) {
	for _, e := range AllHookEvents {
		if string(e) == s {
			return e, nil
		}
	}
	return "", fmt.Errorf("invalid hook event %q, valid events: %s", s, strings.Join(hookEventStrings(), ", "))
}

func hookEventStrings() []string {
	result := make([]string, len(AllHookEvents))
	for i, e := range AllHookEvents {
		result[i] = string(e)
	}
	return result
}

// 🏷️HookResultAgentThinkingEnded represents the result of an agent thinking ended event.
type HookResultAgentThinkingEnded struct {
	HookResultAgentBase
}

// ▶️HookResultAgentThinkingStarting represents the result of an agent thinking starting event.
type HookResultAgentThinkingStarting struct {
	HookResultAgentBase
}

// 📜️HookLogEntry is an alias for EventEntry for hook log entries.
type HookLogEntry = EventEntry

// ▶️HookResultAgentToolTerminalStarting represents the result of an agent terminal tool starting event.
type HookResultAgentToolTerminalStarting struct {
	HookResultAgentBase
	Name    string          `json:"name,omitempty"`
	Input   json.RawMessage `json:"input,omitempty"`
	Command string          `json:"command,omitempty"`
}

// ⬜️HookResultAgentToolTerminalEnded represents the result of an agent terminal tool ended event.
type HookResultAgentToolTerminalEnded struct {
	HookResultAgentBase
	Name       string          `json:"name,omitempty"`
	Input      json.RawMessage `json:"input,omitempty"`
	Output     json.RawMessage `json:"output,omitempty"`
	Command    string          `json:"command,omitempty"`
	PID        int             `json:"pid,omitempty"`
	Terminated bool            `json:"terminated,omitempty"`
	Stdout     json.RawMessage `json:"stdout,omitempty"`
	Stderr     json.RawMessage `json:"stderr,omitempty"`
}

// #endregion 🦀️Hooks
