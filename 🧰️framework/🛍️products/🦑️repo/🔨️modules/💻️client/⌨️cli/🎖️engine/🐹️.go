// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Engine dispatching requests and emitting events.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
)

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

const (
	exitCodeOK       = 0
	exitCodeError    = 1
	exitCodeUsage    = 2
	exitCodeCanceled = 130
)
