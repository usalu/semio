// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📡️engine carries the cli engine: the event stream protocol, the error codes, the engine requests and the engine dispatch.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
)

// #region 🔑️Engine Events

// 🏷️Kind represents a kind value.
type Kind string

const KindStart Kind = "start"

const KindLog Kind = "log"

const KindProgress Kind = "progress"

const KindResult Kind = "result"

const KindArtifact Kind = "artifact"

const KindError Kind = "error"

const KindDone Kind = "done"

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

// #endregion 🔑️Engine Events

// #region 🕌️Engine Errors

// ❌️ErrorCode represents a error code value.
type ErrorCode string

const ErrInternal ErrorCode = "E_INTERNAL"

const ErrParse ErrorCode = "E_PARSE"

const ErrCanceled ErrorCode = "E_CANCELED"

const ErrNetwork ErrorCode = "E_NETWORK"

const ErrAuth ErrorCode = "E_AUTH"

// #endregion 🕌️Engine Errors

// #region 🎸️Engine Requests

// 🔷️Command represents a command value.
type EngineCommand string

const CmdGraphQL EngineCommand = "graphql"

const CmdAnalyze EngineCommand = "analyze"

const CmdAutofix EngineCommand = "autofix"

const CmdPolicy EngineCommand = "policy"

const CmdTicket EngineCommand = "ticket"

const CmdBundle EngineCommand = "bundle"

const CmdFolder EngineCommand = "folder"

const CmdFile EngineCommand = "file"

const CmdSection EngineCommand = "section"

const CmdDef EngineCommand = "definition"

type Request struct {
	Command  EngineCommand
	Args     json.RawMessage
	RepoRoot string
	Verbose  bool
}

// 🕸️GraphQLArgs holds the data fields for a graph q l args record.
type GraphQLArgs struct {
	Query     string         `json:"query"`
	Variables map[string]any `json:"variables,omitempty"`
}

// #endregion 🎸️Engine Requests

// #region 🎖️Engine

// 🕸️GraphQLExecutor defines the interface contract for graph q l executor operations.
type GraphQLExecutor interface {
	Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error)
}

// 💿️Engine holds the data fields for a engine record.
type Engine struct {
	GraphQL GraphQLExecutor
}

// 🔷️NewEngine MUST initialize all required fields and return a valid Engine.
// 🆕️NewEngine creates and returns a new Engine instance.
func NewEngine(graphql GraphQLExecutor) *Engine {
	return &Engine{GraphQL: graphql}
}

// 📡️Run MUST emit start, result or error, and done events in order.
// 📤️Run dispatches the request and returns an event channel.
func (e *Engine) Run(ctx context.Context, req Request) <-chan Event {
	out := make(chan Event)
	go func() {
		defer func() {
			if recovered := recover(); recovered != nil {
				e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "internal error", Detail: fmt.Sprintf("%v", recovered), Fatal: true})
				e.emitDone(out, exitCodeError, "error")
			}
			close(out)
		}()

		e.emitStart(out, req)

		if ctx.Err() != nil {
			e.emitError(out, req, ErrPayload{Code: string(ErrCanceled), Message: ctx.Err().Error(), Fatal: true})
			e.emitDone(out, exitCodeCanceled, "canceled")
			return
		}

		switch req.Command {
		case CmdGraphQL, CmdAnalyze, CmdAutofix, CmdPolicy, CmdTicket, CmdBundle, CmdFolder, CmdFile, CmdSection, CmdDef:
			e.runGraphQL(ctx, req, out)
		default:
			e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "unsupported command", Fatal: true})
			e.emitDone(out, exitCodeError, "error")
		}
	}()
	return out
}

// 🔶️runGraphQL holds the data fields for a runGraphQL record.
func (e *Engine) runGraphQL(ctx context.Context, req Request, out chan<- Event) {
	var args GraphQLArgs
	if err := json.Unmarshal(req.Args, &args); err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrParse), Message: "invalid arguments", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeUsage, "error")
		return
	}
	if e.GraphQL == nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "graphql executor missing", Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	result, err := e.GraphQL.Execute(ctx, args.Query, args.Variables)
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	payload, err := json.Marshal(result)
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "failed to encode result", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	out <- Event{Kind: KindResult, Command: string(req.Command), Data: payload}
	e.emitDone(out, exitCodeOK, "ok")
}

func (e *Engine) emitStart(out chan<- Event, req Request) {
	out <- Event{Kind: KindStart, Command: string(req.Command)}
}

// ❌️emitError holds the data fields for a emitError record.
func (e *Engine) emitError(out chan<- Event, req Request, payload ErrPayload) {
	out <- Event{Kind: KindError, Command: string(req.Command), Error: &payload}
}

func (e *Engine) emitDone(out chan<- Event, code int, status string) {
	out <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: code, Status: status}}
}

const exitCodeOK = 0

const exitCodeError = 1

const exitCodeUsage = 2

const exitCodeCanceled = 130

// #endregion 🎖️Engine
