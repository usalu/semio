// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Engine event, error, command, and request protocol types.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
)

// 🏷️Kind represents a kind value.
type Kind string

const (
	KindStart    Kind = "start"
	KindLog      Kind = "log"
	KindProgress Kind = "progress"
	KindResult   Kind = "result"
	KindArtifact Kind = "artifact"
	KindError    Kind = "error"
	KindDone     Kind = "done"
)

// 💿️Event holds the data fields for a event record.
type Event struct {
	Kind     Kind            `json:"kind"`
	Command  string          `json:"command,omitempty"`
	ID       string          `json:"id,omitempty"`
	Message  string          `json:"message,omitempty"`
	Level    string          `json:"level,omitempty"`
	Progress *Progress       `json:"progress,omitempty"`
	Data     json.RawMessage `json:"data,omitempty"`
	Artifact *Artifact       `json:"artifact,omitempty"`
	Error    *ErrPayload     `json:"error,omitempty"`
	Done     *DonePayload    `json:"done,omitempty"`
}

// 🔶️Progress holds the data fields for a progress record.
type Progress struct {
	Current int    `json:"current,omitempty"`
	Total   int    `json:"total,omitempty"`
	Percent int    `json:"percent,omitempty"`
	Step    string `json:"step,omitempty"`
}

// 🏺️Artifact holds the data fields for a artifact record.
type Artifact struct {
	Type string `json:"type"`
	URI  string `json:"uri"`
	Note string `json:"note,omitempty"`
}

type ErrPayload struct {
	Code    string `json:"code"`
	Message string `json:"message"`
	Detail  string `json:"detail,omitempty"`
	Fatal   bool   `json:"fatal,omitempty"`
}

// 📦️DonePayload holds the data fields for a done payload record.
type DonePayload struct {
	ExitCode int    `json:"exit_code"`
	Status   string `json:"status"`
}

// ❌️ErrorCode represents a error code value.
type ErrorCode string

const (
	ErrInternal ErrorCode = "E_INTERNAL"
	ErrParse    ErrorCode = "E_PARSE"
	ErrCanceled ErrorCode = "E_CANCELED"
	ErrNetwork  ErrorCode = "E_NETWORK"
	ErrAuth     ErrorCode = "E_AUTH"
)

// 🔷️Command represents a command value.
type Command string

const (
	CmdGraphQL Command = "graphql"
	CmdAnalyze Command = "analyze"
	CmdAutofix Command = "autofix"
	CmdPolicy  Command = "policy"
	CmdTicket  Command = "ticket"
	CmdBundle  Command = "bundle"
	CmdFolder  Command = "folder"
	CmdFile    Command = "file"
	CmdSection Command = "section"
	CmdDef     Command = "definition"
)

type Request struct {
	Command  Command
	Args     json.RawMessage
	RepoRoot string
	Verbose  bool
}

// 🕸️GraphQLArgs holds the data fields for a graph q l args record.
type GraphQLArgs struct {
	Query     string         `json:"query"`
	Variables map[string]any `json:"variables,omitempty"`
}
