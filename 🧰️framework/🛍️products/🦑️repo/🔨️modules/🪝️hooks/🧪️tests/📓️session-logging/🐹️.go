// 🐹️ Go side of the session logging case, replayed against the same frozen session vectors as the
// Rust adapter and the TypeScript oracle. The store is a port, so nothing here touches a disk.
package adapter

import (
	"encoding/json"
	"fmt"

	hooks "github.com/usalu/semio/repo/hooks"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const sessionVectors = "shared://📓️session-logging/📓️sessions.json"

type sessionInvocation struct {
	Event    string          `json:"event"`
	Second   string          `json:"second"`
	ToolArgs string          `json:"toolArgs"`
	Input    json.RawMessage `json:"input"`
}

type sessionVector struct {
	ID            string              `json:"id"`
	Client        string              `json:"client"`
	KiroParentPid int                 `json:"kiroParentPid"`
	Invocations   []sessionInvocation `json:"invocations"`
}

type configVector struct {
	ID       string `json:"id"`
	Document string `json:"document"`
}

type sessionVectorFile struct {
	Configs  []configVector  `json:"configs"`
	Sessions []sessionVector `json:"sessions"`
}

func loadSessionVectors(ctx *host.Context) (sessionVectorFile, error) {
	var file sessionVectorFile
	raw, err := ctx.FixtureBytes(sessionVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("session vectors are not valid JSON: %w", err)
	}
	return file, nil
}

func sessionByID(file sessionVectorFile, id string) (sessionVector, error) {
	for _, session := range file.Sessions {
		if session.ID == id {
			return session, nil
		}
	}
	return sessionVector{}, fmt.Errorf("the fixture has no session %s", id)
}

// ▶️replay runs one session's invocations into a fresh memory store and answers what was recorded.
func replay(session sessionVector, logging workspace.LoggingConfig) (*hooks.SessionMeta, error) {
	store := hooks.NewMemorySessionStore()
	var last *hooks.SessionMeta
	for _, invocation := range session.Invocations {
		event, err := hooks.ValidateHookEvent(invocation.Event)
		if err != nil {
			return nil, err
		}
		hookContext := hooks.NewHookContext(event, session.Client).AtSecond(invocation.Second).AtRoot("/repo").WithTool("", invocation.ToolArgs)
		if len(invocation.Input) > 0 && string(invocation.Input) != "null" {
			hookContext = hookContext.WithInput(invocation.Input)
		}
		result := hooks.DispatchHook(hookContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
		sessionID := hooks.ResolveLogSessionID(hookContext, session.KiroParentPid)
		recorded, recordErr := hooks.RecordSessionHook(store, hookContext, result, sessionID, logging, hooks.InertEnvironment{})
		if recordErr != nil {
			return nil, recordErr
		}
		if recorded != nil {
			last = recorded
		}
	}
	return last, nil
}

func loggingOn(detail string, plan bool) workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: plan, Detail: detail}
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func theConfigurationDecidesWhetherAnythingIsRecorded(ctx *host.Context) (host.Outcome, error) {
	file, err := loadSessionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	session, err := sessionByID(file, "an-agent-session-that-plans-and-is-refused")
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, config := range file.Configs {
		logging := workspace.ParseRepoConfig(config.Document).Logging
		recorded, replayErr := replay(session, logging)
		if replayErr != nil {
			return host.Outcome{}, replayErr
		}
		entries := 0
		if recorded != nil {
			entries = len(recorded.Events)
		}
		projection[config.ID] = map[string]any{
			"session":         logging.Session,
			"operations":      logging.Operations,
			"plan":            logging.Plan,
			"detail":          logging.Detail,
			"includeResponse": logging.IncludeResponse(),
			"includeNative":   logging.IncludeNative(),
			"recordedEntries": entries,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func detailDecidesHowMuchOfEachEntrySurvives(ctx *host.Context) (host.Outcome, error) {
	file, err := loadSessionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	session, err := sessionByID(file, "an-agent-session-that-plans-and-is-refused")
	if err != nil {
		return host.Outcome{}, err
	}
	var refused *sessionInvocation
	for index := range session.Invocations {
		if session.Invocations[index].Event == "agent.tool.terminal.starting" {
			refused = &session.Invocations[index]
			break
		}
	}
	if refused == nil {
		return host.Outcome{}, fmt.Errorf("the fixture has no refused invocation")
	}
	single := sessionVector{Client: session.Client, Invocations: []sessionInvocation{*refused}}
	projection := map[string]any{}
	for _, detail := range []string{"minimal", "standard", "full"} {
		recorded, replayErr := replay(single, loggingOn(detail, true))
		if replayErr != nil {
			return host.Outcome{}, replayErr
		}
		if recorded == nil || len(recorded.Events) == 0 {
			return host.Outcome{}, fmt.Errorf("session logging was on but nothing was recorded")
		}
		entry := recorded.Events[0]
		blocked := false
		if entry.Response != nil && entry.Response.Blocked != nil {
			blocked = *entry.Response.Blocked
		}
		projection[detail] = map[string]any{
			"carriesNative":   entry.Native != nil,
			"carriesResponse": entry.Response != nil,
			"blocked":         blocked,
			"entry":           entry,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func aSessionIsRecordedUnderAResolvableIdentity(ctx *host.Context) (host.Outcome, error) {
	file, err := loadSessionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, session := range file.Sessions {
		recorded, replayErr := replay(session, loggingOn("standard", true))
		if replayErr != nil {
			return host.Outcome{}, replayErr
		}
		if recorded == nil {
			projection[session.ID] = map[string]any{"id": "", "uri": "", "client": "", "second": "", "transcript": "", "contributor": "", "entries": 0}
			continue
		}
		projection[session.ID] = map[string]any{
			"id":          recorded.ID,
			"uri":         recorded.URI,
			"client":      recorded.Client,
			"second":      recorded.Second,
			"transcript":  recorded.Transcript,
			"contributor": recorded.Contributor,
			"entries":     len(recorded.Events),
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func aVersionHookIsNeverRecorded(ctx *host.Context) (host.Outcome, error) {
	file, err := loadSessionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	session, err := sessionByID(file, "a-version-hook-is-never-recorded")
	if err != nil {
		return host.Outcome{}, err
	}
	recorded, replayErr := replay(session, loggingOn("full", true))
	if replayErr != nil {
		return host.Outcome{}, replayErr
	}
	return host.Outcome{Projection: map[string]any{"recordedNothing": recorded == nil}}, nil
}

func theRecordedPlanFollowsThePlanSwitch(ctx *host.Context) (host.Outcome, error) {
	file, err := loadSessionVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	session, err := sessionByID(file, "an-agent-session-that-plans-and-is-refused")
	if err != nil {
		return host.Outcome{}, err
	}
	withPlan, err := replay(session, loggingOn("standard", true))
	if err != nil {
		return host.Outcome{}, err
	}
	if withPlan == nil {
		return host.Outcome{}, fmt.Errorf("nothing was recorded with the plan switch on")
	}
	withoutPlan, err := replay(session, loggingOn("standard", false))
	if err != nil {
		return host.Outcome{}, err
	}
	if withoutPlan == nil {
		return host.Outcome{}, fmt.Errorf("nothing was recorded with the plan switch off")
	}
	return host.Outcome{Projection: map[string]any{"withPlan": withPlan.Plan, "withoutPlanIsAbsent": withoutPlan.Plan == nil}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-configuration-decides-whether-anything-is-recorded", theConfigurationDecidesWhetherAnythingIsRecorded).
		Subject("detail-decides-how-much-of-each-entry-survives", detailDecidesHowMuchOfEachEntrySurvives).
		Subject("a-session-is-recorded-under-a-resolvable-identity", aSessionIsRecordedUnderAResolvableIdentity).
		Subject("a-version-hook-is-never-recorded", aVersionHookIsNeverRecorded).
		Subject("the-recorded-plan-follows-the-plan-switch", theRecordedPlanFollowsThePlanSwitch)
}

// endregion 🔖️Registration
