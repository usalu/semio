// 🐹️ Go side of the native event normalisation case, written against the same frozen specification
// vectors as the Rust adapter — the fixture pins the expected neutral event beside every native one,
// so neither adapter reads the other's source.
package adapter

import (
	"encoding/json"
	"fmt"

	hooks "github.com/usalu/semio/repo/hooks"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const nativeEventVectors = "local://🔀️native-events.json"

type nativeEventVector struct {
	ID             string          `json:"id"`
	Client         string          `json:"client"`
	NativeEvent    string          `json:"nativeEvent"`
	ToolName       string          `json:"toolName"`
	Payload        json.RawMessage `json:"payload"`
	ExpectedEvent  string          `json:"expectedEvent"`
	ExpectedParent string          `json:"expectedParent"`
}

type nativeEventVectorFile struct {
	Vectors      []nativeEventVector `json:"vectors"`
	Refusals     []nativeEventVector `json:"refusals"`
	PayloadFacts []nativeEventVector `json:"payloadFacts"`
}

func loadNativeEventVectors(ctx *host.Context) (nativeEventVectorFile, error) {
	var file nativeEventVectorFile
	raw, err := ctx.FixtureBytes(nativeEventVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("native event vectors are not valid JSON: %w", err)
	}
	return file, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyNativeEventResolvesToItsPinnedNeutralEvent(ctx *host.Context) (host.Outcome, error) {
	file, err := loadNativeEventVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Vectors {
		event, parent, resolveErr := hooks.ResolveHookEvent(vector.NativeEvent, vector.Client, vector.ToolName, vector.Payload)
		if resolveErr != nil {
			return host.Outcome{}, fmt.Errorf("%s: %w", vector.ID, resolveErr)
		}
		projection[vector.ID] = map[string]any{
			"event":                string(event),
			"parent":               parent,
			"matchesSpecification": string(event) == vector.ExpectedEvent && parent == vector.ExpectedParent,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func theCommandOutranksTheToolNameWhenItIsMoreSpecific(ctx *host.Context) (host.Outcome, error) {
	file, err := loadNativeEventVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Vectors {
		command, _ := hooks.ExtractCommandAndCwd(vector.Payload)
		byCommand := ""
		if command != "" {
			byCommand = string(hooks.ClassifyCommandKind(command))
		}
		projection[vector.ID] = map[string]any{
			"byToolName": string(hooks.ClassifyTool(vector.ToolName)),
			"command":    command,
			"byCommand":  byCommand,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func anUnrecognisedNativeEventIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := loadNativeEventVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.Refusals {
		event, parent, resolveErr := hooks.ResolveHookEvent(vector.NativeEvent, vector.Client, vector.ToolName, nil)
		if resolveErr != nil {
			projection[vector.ID] = map[string]any{"refused": true, "message": resolveErr.Error()}
			continue
		}
		projection[vector.ID] = map[string]any{"refused": false, "event": string(event), "parent": parent}
	}
	return host.Outcome{Projection: projection}, nil
}

func neutralFactsAreReadOutOfEveryPayloadShape(ctx *host.Context) (host.Outcome, error) {
	file, err := loadNativeEventVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, vector := range file.PayloadFacts {
		command, cwd := hooks.ExtractCommandAndCwd(vector.Payload)
		projection[vector.ID] = map[string]any{
			"session":       hooks.ExtractSessionID(vector.Payload),
			"transcript":    hooks.ExtractTranscript(vector.Payload),
			"toolName":      hooks.ExtractToolName(vector.Payload),
			"command":       command,
			"cwd":           cwd,
			"parent":        hooks.ResolveParentSessionID("", vector.Payload),
			"llm":           hooks.ExtractLLM(vector.Payload),
			"effort":        hooks.ExtractEffort(vector.Payload),
			"messageId":     hooks.ExtractMessageID(vector.Payload),
			"hookEventName": hooks.ExtractHookEventName(vector.Payload),
		}
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-native-event-resolves-to-its-pinned-neutral-event", everyNativeEventResolvesToItsPinnedNeutralEvent).
		Subject("the-command-outranks-the-tool-name-when-it-is-more-specific", theCommandOutranksTheToolNameWhenItIsMoreSpecific).
		Subject("an-unrecognised-native-event-is-refused", anUnrecognisedNativeEventIsRefused).
		Subject("neutral-facts-are-read-out-of-every-payload-shape", neutralFactsAreReadOutOfEveryPayloadShape)
}

// endregion 🔖️Registration
