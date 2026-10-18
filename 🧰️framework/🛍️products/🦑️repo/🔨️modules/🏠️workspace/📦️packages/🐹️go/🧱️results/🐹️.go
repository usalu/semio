// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧱️results carries the command result types: exit errors, tool results, scopes, output lines, command output and command streams.

// #endregion 🧲️Header

package workspace

import (
	json "encoding/json"
	fmt "fmt"
	io "io"
)

// #region 🧱️Results

// 💿️ExitError holds the data fields for a exit error record.
type ExitError struct {
	Code int
}

// 🔤️Error MUST return a formatted string representation.
// ❌️Error returns the string representation of the error.
func (e ExitError) Error() string {
	return fmt.Sprintf("exit status %d", e.Code)
}

// ❌️toolErrorResult holds the data fields for a toolErrorResult record.
func ToolErrorResult(err error) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %v", err))
	return ToolResult{Output: *output, Error: err.Error()}
}

// 🔺️toolErrorMsg holds the data fields for a toolErrorMsg record.
func ToolErrorMsg(msg string) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %s", msg))
	return ToolResult{Output: *output, Error: msg}
}

// 🔭️ScopeKind represents a scope kind value.
type ScopeKind string

const ScopeRepo ScopeKind = "repo"

const ScopeTechnology ScopeKind = "bundle"

const ScopeFolder ScopeKind = "folder"

const ScopeFile ScopeKind = "file"

const ScopeSection ScopeKind = "section"

const ScopeDefinition ScopeKind = "definition"

// 💿️Scope holds the data fields for a scope record.
type Scope struct {
	Raw            string    `json:"raw"`
	Kind           ScopeKind `json:"kind"`
	TechnologyName string    `json:"technologyName,omitempty"`
	FilePath       string    `json:"filePath,omitempty"`
	SectionPath    []string  `json:"sectionPath,omitempty"`
	DefinitionName string    `json:"definitionName,omitempty"`
}

// 💚️OutputType represents a output type value.
type OutputType string

const OutputInfo OutputType = "info"

const OutputSuccess OutputType = "success"

const OutputError OutputType = "error"

const OutputWarn OutputType = "warn"

const OutputPlain OutputType = "plain"

// 💛️OutputLine holds the data fields for a output line record.
type OutputLine struct {
	Type OutputType `json:"type"`
	Text string     `json:"text"`
}

// 🧡️CommandOutput holds the data fields for a command output record.
type CommandOutput struct {
	Lines    []OutputLine `json:"lines"`
	ExitCode int          `json:"exitCode"`
}

// ❤️ToolResult holds the data fields for a tool result record.
type ToolResult struct {
	Output CommandOutput `json:"output"`
	Data   interface{}   `json:"data,omitempty"`
	Error  string        `json:"error,omitempty"`
}

// ▫️NewOutput MUST initialize all required fields and return a valid Output.
// 🆕️NewOutput creates and returns a new Output instance.
func NewOutput() *CommandOutput {
	return &CommandOutput{Lines: []OutputLine{}, ExitCode: 0}
}

// 📍️Info MUST return the metadata entry for the statute.
// ℹInfo returns the metadata for the statute.
func (o *CommandOutput) Info(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputInfo, Text: text})
}

// ◾Success MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Success(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputSuccess, Text: text})
}

// ❌️Error MUST return a formatted string representation.
// ❌️Error returns the string representation of the error.
func (o *CommandOutput) Error(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputError, Text: text})
	o.ExitCode = 1
}

// ⚠️Warn MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Warn(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputWarn, Text: text})
}

// ◽Plain MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Plain(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: text})
}

// ◻Print MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Print() {
	for _, line := range o.Lines {
		fmt.Println(line.Text)
	}
}

// ◼Json MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Json(data interface{}) {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err == nil {
		o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: string(bytes)})
	}
}

// 🖨️CommandStreams is the narrow view of a CLI command a domain routine needs to write output. It
// lives here so the domain modules never have to import ⌨️cli; `*command.Command` satisfies it.
type CommandStreams interface {
	// 📤️OutOrStdout answers the writer for regular output.
	OutOrStdout() io.Writer
	// 📥️ErrOrStderr answers the writer for error output.
	ErrOrStderr() io.Writer
}

// #endregion 🧱️Results
