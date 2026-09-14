// 🐹️ Go side of the tool blocking policy case, judged against the same frozen invocation vectors as
// the Rust adapter and the TypeScript oracle — the written rules are the contract, not either source.
package adapter

import (
	"encoding/json"
	"fmt"

	hooks "github.com/usalu/semio/repo/hooks"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const blockingVectors = "local://🛡️invocations.json"

type blockingInvocation struct {
	ID      string `json:"id"`
	Tool    string `json:"tool"`
	Args    string `json:"args"`
	Blocked bool   `json:"blocked"`
}

type blockingVectorFile struct {
	Invocations []blockingInvocation `json:"invocations"`
	Segments    []string             `json:"segments"`
	InlineCode  []string             `json:"inlineCode"`
}

func loadBlockingVectors(ctx *host.Context) (blockingVectorFile, error) {
	var file blockingVectorFile
	raw, err := ctx.FixtureBytes(blockingVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("blocking vectors are not valid JSON: %w", err)
	}
	return file, nil
}

func verdict(blocked bool, reason string) map[string]any {
	if !blocked {
		return map[string]any{"blocked": false, "reason": ""}
	}
	return map[string]any{"blocked": true, "reason": reason}
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyInvocationIsJudgedTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := loadBlockingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		blocked, reason := hooks.IsToolBlocked(invocation.Tool, invocation.Args)
		projection[invocation.ID] = verdict(blocked, reason)
	}
	return host.Outcome{Projection: projection}, nil
}

func theVerdictMatchesThePinnedSpecification(ctx *host.Context) (host.Outcome, error) {
	file, err := loadBlockingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		blocked, _ := hooks.IsToolBlocked(invocation.Tool, invocation.Args)
		projection[invocation.ID] = blocked == invocation.Blocked
	}
	return host.Outcome{Projection: projection}, nil
}

func aCommandSplitsIntoTheSameSegments(ctx *host.Context) (host.Outcome, error) {
	file, err := loadBlockingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, command := range file.Segments {
		projection[command] = hooks.SplitCommandSegments(command)
	}
	return host.Outcome{Projection: projection}, nil
}

func inlineCodeIsScannedTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := loadBlockingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, code := range file.InlineCode {
		blocked, reason := hooks.ContainsBlockedGitInCode(code)
		projection[code] = verdict(blocked, reason)
	}
	return host.Outcome{Projection: projection}, nil
}

func refusingIsIdempotentAndOrderFree(ctx *host.Context) (host.Outcome, error) {
	file, err := loadBlockingVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, invocation := range file.Invocations {
		blocked, _ := hooks.IsToolBlocked(invocation.Tool, invocation.Args)
		if !blocked || invocation.Args == "" {
			continue
		}
		appendedBlocked, appendedReason := hooks.IsToolBlocked(invocation.Tool, invocation.Args+" && echo done")
		prependedBlocked, prependedReason := hooks.IsToolBlocked(invocation.Tool, "echo start && "+invocation.Args)
		projection[invocation.ID] = map[string]any{
			"appendedStillRefused":  appendedBlocked,
			"prependedStillRefused": prependedBlocked,
			"appendedReason":        appendedReason,
			"prependedReason":       prependedReason,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-invocation-is-judged-the-same-way", everyInvocationIsJudgedTheSameWay).
		Subject("the-verdict-matches-the-pinned-specification", theVerdictMatchesThePinnedSpecification).
		Subject("a-command-splits-into-the-same-segments", aCommandSplitsIntoTheSameSegments).
		Subject("inline-code-is-scanned-the-same-way", inlineCodeIsScannedTheSameWay).
		Subject("refusing-is-idempotent-and-order-free", refusingIsIdempotentAndOrderFree)
}

// endregion 🔖️Registration
