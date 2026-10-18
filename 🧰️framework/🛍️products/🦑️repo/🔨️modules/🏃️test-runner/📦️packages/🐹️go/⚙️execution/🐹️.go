// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⚙️execution carries plan execution with progress and cancellation and the JSON text vectors of plans, scopes and outcomes.

// #endregion 🧲️Header

package testrunner

import (
	json "encoding/json"
	fmt "fmt"
	atomic "sync/atomic"
)

// #region ⚙️Execution

// 🛑️CancellationToken is a cooperative cancellation flag shared with whoever drives the run.
type CancellationToken struct {
	flag atomic.Bool
}

// 🆕️NewCancellationToken answers a token that is not cancelled.
func NewCancellationToken() *CancellationToken { return &CancellationToken{} }

// 🛑️Cancel requests cancellation; every later invocation is skipped.
func (token *CancellationToken) Cancel() { token.flag.Store(true) }

// ❓️IsCancelled says whether cancellation was requested.
func (token *CancellationToken) IsCancelled() bool { return token != nil && token.flag.Load() }

// 📣️ProgressEventKind names what the caller is being told.
type ProgressEventKind string

// 🚦️ProgressPlanStarted says the plan is about to start.
const ProgressPlanStarted ProgressEventKind = "plan-started"

// ▶️ProgressInvocationStarted says one invocation is starting.
const ProgressInvocationStarted ProgressEventKind = "invocation-started"

// ⏹️ProgressInvocationFinished says one invocation finished.
const ProgressInvocationFinished ProgressEventKind = "invocation-finished"

// 🛑️ProgressCancelled says cancellation was observed before the next invocation started.
const ProgressCancelled ProgressEventKind = "cancelled"

// 🏁️ProgressPlanFinished says the plan is done.
const ProgressPlanFinished ProgressEventKind = "plan-finished"

// 📣️ProgressEvent is one entry of the progress stream a run reports.
type ProgressEvent struct {
	// 📣️Event is which of the five kinds this is.
	Event ProgressEventKind
	// 🔢️Total is the number of invocations in the plan.
	Total int
	// 🔢️Index is the zero-based position in the plan.
	Index int
	// 📜️Argv is the argv of the invocation.
	Argv []string
	// 🚦️Status is how the invocation ended.
	Status RunStatus
	// 🔢️Failed is the number of failing tests the invocation reported.
	Failed int
	// 🔢️Completed is the number of invocations that had already completed.
	Completed int
}

// 📣️MarshalJSON renders only the fields the event's kind carries.
func (event ProgressEvent) MarshalJSON() ([]byte, error) {
	switch event.Event {
	case ProgressPlanStarted:
		return json.Marshal(map[string]any{"event": string(event.Event), "total": event.Total})
	case ProgressInvocationStarted:
		argv := event.Argv
		if argv == nil {
			argv = []string{}
		}
		return json.Marshal(map[string]any{"event": string(event.Event), "index": event.Index, "argv": argv})
	case ProgressInvocationFinished:
		return json.Marshal(map[string]any{"event": string(event.Event), "index": event.Index, "status": string(event.Status), "failed": event.Failed})
	}
	return json.Marshal(map[string]any{"event": string(event.Event), "completed": event.Completed})
}

// 📊️ExecutionReport is what a whole plan produced.
type ExecutionReport struct {
	// 📊️Outcomes is one outcome per completed invocation, in plan order.
	Outcomes []TestOutcome `json:"outcomes"`
	// 🛑️Cancelled says whether the run stopped early because cancellation was requested.
	Cancelled bool `json:"cancelled"`
	// 🔢️Completed is the number of invocations that completed.
	Completed int `json:"completed"`
	// 🔢️Total is the number of invocations the plan contained.
	Total int `json:"total"`
	// 🚫️Problems are the planning refusals plus any runner that could not be started.
	Problems []string `json:"problems"`
}

// ▶️ExecutePlan runs a plan through the port, reporting progress and honouring cancellation between
// invocations.
func ExecutePlan(plan InvocationPlan, runner ProcessRunner, cancellation *CancellationToken, progress func(ProgressEvent)) ExecutionReport {
	total := len(plan.Invocations)
	report := ExecutionReport{Outcomes: []TestOutcome{}, Total: total, Problems: append([]string{}, plan.Problems...)}
	announce := func(event ProgressEvent) {
		if progress != nil {
			progress(event)
		}
	}
	announce(ProgressEvent{Event: ProgressPlanStarted, Total: total})
	for index, invocation := range plan.Invocations {
		if cancellation.IsCancelled() {
			report.Cancelled = true
			announce(ProgressEvent{Event: ProgressCancelled, Completed: report.Completed})
			return report
		}
		announce(ProgressEvent{Event: ProgressInvocationStarted, Index: index, Argv: invocation.Argv})
		request := ProcessRequest{Args: []string{}, Cwd: invocation.Cwd, Env: invocation.Env}
		if len(invocation.Argv) > 0 {
			request.Program = invocation.Argv[0]
			request.Args = invocation.Argv[1:]
		}
		outcome := TestOutcome{Runner: invocation.Runner, Status: RunStatusNotRun, Tests: []TestCaseOutcome{}}
		if output, err := runner.Run(request); err != nil {
			report.Problems = append(report.Problems, err.Error())
		} else {
			outcome = ParseOutcome(invocation.Runner, output)
		}
		announce(ProgressEvent{Event: ProgressInvocationFinished, Index: index, Status: outcome.Status, Failed: outcome.Totals.Failed})
		report.Outcomes = append(report.Outcomes, outcome)
		report.Completed++
	}
	announce(ProgressEvent{Event: ProgressPlanFinished, Completed: report.Completed})
	return report
}

// #endregion ⚙️Execution

// #region 🧫️Vectors

// 🗺️PlanToJSONText renders a plan as JSON text, with every absent collection rendered as empty.
func PlanToJSONText(plan InvocationPlan) string {
	normalized := InvocationPlan{Invocations: []RunnerInvocation{}, Problems: []string{}}
	normalized.Problems = append(normalized.Problems, plan.Problems...)
	for _, invocation := range plan.Invocations {
		if invocation.Argv == nil {
			invocation.Argv = []string{}
		}
		if invocation.Env == nil {
			invocation.Env = map[string]string{}
		}
		normalized.Invocations = append(normalized.Invocations, invocation)
	}
	return marshalText(normalized)
}

// 🔭️ScopeToJSONText renders a scope as JSON text.
func ScopeToJSONText(scope TestScope) string { return marshalText(scope) }

// 📊️OutcomeToJSONText renders one outcome as JSON text.
func OutcomeToJSONText(outcome TestOutcome) string { return marshalText(normalizedOutcome(outcome)) }

// 📊️ReportToJSONText renders a whole execution report as JSON text.
func ReportToJSONText(report ExecutionReport) string {
	normalized := ExecutionReport{Outcomes: []TestOutcome{}, Cancelled: report.Cancelled, Completed: report.Completed, Total: report.Total, Problems: []string{}}
	normalized.Problems = append(normalized.Problems, report.Problems...)
	for _, outcome := range report.Outcomes {
		normalized.Outcomes = append(normalized.Outcomes, normalizedOutcome(outcome))
	}
	return marshalText(normalized)
}

// 📣️ProgressToJSONText renders a progress event stream as JSON text.
func ProgressToJSONText(events []ProgressEvent) string {
	if events == nil {
		events = []ProgressEvent{}
	}
	return marshalText(events)
}

func normalizedOutcome(outcome TestOutcome) TestOutcome {
	if outcome.Tests == nil {
		outcome.Tests = []TestCaseOutcome{}
	}
	return outcome
}

func marshalText(value any) string {
	encoded, err := json.Marshal(value)
	if err != nil {
		return fmt.Sprintf("{\"error\":%q}", err.Error())
	}
	return string(encoded)
}

// #endregion 🧫️Vectors
