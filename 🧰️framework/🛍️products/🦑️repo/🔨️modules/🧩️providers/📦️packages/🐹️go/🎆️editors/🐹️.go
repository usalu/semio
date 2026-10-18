// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎆️editors carries the editor providers: Copilot, Cursor, Windsurf, Claude Code, Droid, Codex, Antigravity and Kiro.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎆️EditorProviders

// 🔌️CopilotEditorProvider holds the data fields for a copilot editor provider record.
type CopilotEditorProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *CopilotEditorProvider) Kind() string { return "copilot-chat" }

// 📡️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CopilotEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCopilotEvent(nativeEvent, toolKind)
}

// 📋️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CopilotEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	return FormatVSCodeHookOutput(hookEventName, result)
}

// 🔷️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CopilotEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return vsCodeEventFromHookEvent(event, parentInfo)
}

// 📝️CursorEditorProvider holds the data fields for a cursor editor provider record.
type CursorEditorProvider struct{}

// 🏷️Kind holds the data fields for a Kind record.
func (p *CursorEditorProvider) Kind() string { return "cursor-chat" }

// 🔸️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CursorEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCursorEvent(nativeEvent, toolKind)
}

// 🔺️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CursorEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🔻️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CursorEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💻️WindsurfEditorProvider holds the data fields for a windsurf editor provider record.
type WindsurfEditorProvider struct{}

// 🟥️Kind holds the data fields for a Kind record.
func (p *WindsurfEditorProvider) Kind() string { return "windsurf-chat" }

// 🟨️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *WindsurfEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveWindsurfEvent(nativeEvent, toolKind)
}

// 🟩️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *WindsurfEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟦️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *WindsurfEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💠️ClaudeCodeEditorProvider holds the data fields for a claude code editor provider record.
type ClaudeCodeEditorProvider struct{}

// 🔳️Kind holds the data fields for a Kind record.
func (p *ClaudeCodeEditorProvider) Kind() string { return "claude-code" }

// ▪️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *ClaudeCodeEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// ▫️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *ClaudeCodeEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// ◾NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *ClaudeCodeEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ◼DroidEditorProvider holds the data fields for a droid editor provider record.
type DroidEditorProvider struct{}

// 🔵️Kind holds the data fields for a Kind record.
func (p *DroidEditorProvider) Kind() string { return "droid" }

// 🟠️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *DroidEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🟡️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *DroidEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟢️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *DroidEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ⚪️CodexEditorProvider holds the data fields for a codex editor provider record.
type CodexEditorProvider struct{}

// ⚫️Kind holds the data fields for a Kind record.
func (p *CodexEditorProvider) Kind() string { return "codex" }

// 🩶️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CodexEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🩷️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CodexEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 💜️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CodexEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💛️AntigravityEditorProvider holds the data fields for an antigravity editor provider record.
type AntigravityEditorProvider struct{}

// 🧡️Kind holds the data fields for a Kind record.
func (p *AntigravityEditorProvider) Kind() string { return "antigravity-chat" }

// 🤍️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *AntigravityEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🖤️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *AntigravityEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🤎️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *AntigravityEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💝️KiroEditorProvider holds the data fields for a kiro editor provider record.
type KiroEditorProvider struct{}

func (p *KiroEditorProvider) Kind() string { return "kiro-cli" }

func (p *KiroEditorProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveKiroEvent(nativeEvent, toolKind)
}

func (p *KiroEditorProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

func (p *KiroEditorProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// #endregion 🎆️EditorProviders
