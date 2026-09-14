// 🐹️ Go side of the hook result formatting case, rendered against the same frozen invocation vectors
// as the Rust adapter — the fixture pins the verdict, the exit code and the wrapped/plain split.
package adapter

import (
	"encoding/json"
	"fmt"
	"sort"

	hooks "github.com/usalu/semio/repo/hooks"
	providers "github.com/usalu/semio/repo/providers"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const formattingVectors = "local://🖨️invocations.json"

const bareClient = "claude-code"

const wrappingClient = "copilot-chat"

type formattingInvocation struct {
	ID              string          `json:"id"`
	Event           string          `json:"event"`
	ToolName        string          `json:"toolName"`
	ToolArgs        string          `json:"toolArgs"`
	ParentInfo      string          `json:"parentInfo"`
	Input           json.RawMessage `json:"input"`
	ExpectedAllowed bool            `json:"expectedAllowed"`
	ExpectedExit    int             `json:"expectedExit"`
}

type microCommitVector struct {
	ID   string   `json:"id"`
	Args []string `json:"args"`
}

type formattingVectorFile struct {
	Clients     []string               `json:"clients"`
	Invocations []formattingInvocation `json:"invocations"`
	MicroCommit []microCommitVector    `json:"microCommit"`
}

func loadFormattingVectors(ctx *host.Context) (formattingVectorFile, error) {
	var file formattingVectorFile
	raw, err := ctx.FixtureBytes(formattingVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("hook invocation vectors are not valid JSON: %w", err)
	}
	return file, nil
}

func contextOf(invocation formattingInvocation, client string) (hooks.HookContext, error) {
	event, err := hooks.ValidateHookEvent(invocation.Event)
	if err != nil {
		return hooks.HookContext{}, err
	}
	ctx := hooks.NewHookContext(event, client).AtSecond("2026-09-06T12:00:00Z").AtRoot("/repo").WithTool(invocation.ToolName, invocation.ToolArgs).WithParent(invocation.ParentInfo)
	if len(invocation.Input) > 0 && string(invocation.Input) != "null" {
		ctx = ctx.WithInput(invocation.Input)
	}
	return ctx, nil
}

func recordOf(document string) map[string]any {
	if document == "" {
		return nil
	}
	var record map[string]any
	if err := json.Unmarshal([]byte(document), &record); err != nil {
		return nil
	}
	return record
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyClientReceivesTheShapeItReads(ctx *host.Context) (host.Outcome, error) {
	file, err := loadFormattingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		perClient := map[string]any{}
		for _, client := range file.Clients {
			hookContext, contextErr := contextOf(invocation, client)
			if contextErr != nil {
				return host.Outcome{}, contextErr
			}
			event, known := hookContext.ResolvedEvent()
			if !known {
				return host.Outcome{}, fmt.Errorf("the fixture names an unknown event")
			}
			result := hooks.DispatchHook(hookContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
			native := hooks.ResolveNativeEventName(client, event, hookContext.ParentInfo, hookContext.Input)
			output := hooks.RenderHookOutput(client, event, hookContext.ParentInfo, native, result, false)
			record := recordOf(output.Stdout)
			_, wrapped := record["hookSpecificOutput"]
			perClient[client] = map[string]any{
				"wrapped":       wrapped,
				"exit":          output.ExitCode,
				"stdoutIsEmpty": output.Stdout == "",
				"stderrIsEmpty": output.Stderr == "",
			}
		}
		projection[invocation.ID] = perClient
	}
	return host.Outcome{Projection: projection}, nil
}

func aRefusalIsUnmistakableInBothShapes(ctx *host.Context) (host.Outcome, error) {
	file, err := loadFormattingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		wrappingContext, contextErr := contextOf(invocation, wrappingClient)
		if contextErr != nil {
			return host.Outcome{}, contextErr
		}
		event, known := wrappingContext.ResolvedEvent()
		if !known {
			return host.Outcome{}, fmt.Errorf("the fixture names an unknown event")
		}
		wrappingResult := hooks.DispatchHook(wrappingContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
		native := hooks.ResolveNativeEventName(wrappingClient, event, wrappingContext.ParentInfo, wrappingContext.Input)
		wrappedOutput := hooks.RenderHookOutput(wrappingClient, event, wrappingContext.ParentInfo, native, wrappingResult, false)
		wrappedRecord := recordOf(wrappedOutput.Stdout)
		if wrappedRecord == nil {
			return host.Outcome{}, fmt.Errorf("%s: the wrapped record is not parseable JSON", invocation.ID)
		}
		decision := ""
		if specific, ok := wrappedRecord["hookSpecificOutput"].(map[string]any); ok {
			decision, _ = specific["permissionDecision"].(string)
		}
		bareContext, bareErr := contextOf(invocation, bareClient)
		if bareErr != nil {
			return host.Outcome{}, bareErr
		}
		bareResult := hooks.DispatchHook(bareContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
		bareOutput := hooks.RenderHookOutput(bareClient, event, bareContext.ParentInfo, "", bareResult, false)
		projection[invocation.ID] = map[string]any{
			"allowedMatchesSpecification":      bareResult.Allowed == invocation.ExpectedAllowed,
			"bareExitMatchesSpecification":     bareOutput.ExitCode == invocation.ExpectedExit,
			"wrappedExitIsAlwaysZero":          wrappedOutput.ExitCode == 0,
			"permissionDecision":               decision,
			"refusalReachesStderrOnlyWhenBare": (bareOutput.Stderr == "") == bareResult.Allowed && wrappedOutput.Stderr == "",
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func jsonModeAlwaysPrintsTheBareRecordToStdout(ctx *host.Context) (host.Outcome, error) {
	file, err := loadFormattingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		hookContext, contextErr := contextOf(invocation, bareClient)
		if contextErr != nil {
			return host.Outcome{}, contextErr
		}
		event, known := hookContext.ResolvedEvent()
		if !known {
			return host.Outcome{}, fmt.Errorf("the fixture names an unknown event")
		}
		result := hooks.DispatchHook(hookContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
		output := hooks.RenderHookOutput(bareClient, event, hookContext.ParentInfo, "", result, true)
		record := recordOf(output.Stdout)
		if record == nil {
			return host.Outcome{}, fmt.Errorf("%s: the record is not parseable JSON", invocation.ID)
		}
		allowed, _ := record["allowed"].(bool)
		_, wrapped := record["hookSpecificOutput"]
		projection[invocation.ID] = map[string]any{
			"exit":             output.ExitCode,
			"stderrIsEmpty":    output.Stderr == "",
			"recordIsAnObject": true,
			"allowed":          allowed,
			"carriesNoWrapper": !wrapped,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func theRecordCarriesTheEventSpecificMembers(ctx *host.Context) (host.Outcome, error) {
	file, err := loadFormattingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		hookContext, contextErr := contextOf(invocation, bareClient)
		if contextErr != nil {
			return host.Outcome{}, contextErr
		}
		result := hooks.DispatchHook(hookContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
		members := []string{}
		for key := range result.Payload() {
			members = append(members, key)
		}
		sort.Strings(members)
		projection[invocation.ID] = members
	}
	return host.Outcome{Projection: projection}, nil
}

func microCommitIsDelegatedThroughOneArgv(ctx *host.Context) (host.Outcome, error) {
	file, err := loadFormattingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.MicroCommit {
		args := vector.Args
		if args == nil {
			args = []string{}
		}
		argv := hooks.MicroCommitArgv("/bin/bun", args)
		runner := providers.NewRecordedProcessRunner(providers.ProcessTranscript{Exchanges: []providers.ProcessExchange{{Argv: argv, Stdout: "micro-commit done\n", Stderr: "", Status: 0}}})
		outcome := hooks.RunMicroCommit(runner, "/repo", "/bin/bun", args)
		issued := []string{}
		for _, entry := range runner.Issued() {
			issued = append(issued, joinArgv(entry))
		}
		projection[vector.ID] = map[string]any{
			"argv":   argv,
			"issued": issued,
			"stdout": outcome.Stdout,
			"status": outcome.Status,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func joinArgv(argv []string) string {
	joined := ""
	for index, entry := range argv {
		if index > 0 {
			joined += " "
		}
		joined += entry
	}
	return joined
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-client-receives-the-shape-it-reads", everyClientReceivesTheShapeItReads).
		Subject("a-refusal-is-unmistakable-in-both-shapes", aRefusalIsUnmistakableInBothShapes).
		Subject("json-mode-always-prints-the-bare-record-to-stdout", jsonModeAlwaysPrintsTheBareRecordToStdout).
		Subject("the-record-carries-the-event-specific-members", theRecordCarriesTheEventSpecificMembers).
		Subject("micro-commit-is-delegated-through-one-argv", microCommitIsDelegatedThroughOneArgv)
}

// endregion 🔖️Registration
