// 🐹️ Go side of the editor hook output case, written against the same frozen fixture as the Rust
// adapter — pairwise equivalence between two independently written implementations is the whole
// evidence this case has, so neither adapter reads the other's source.
package adapter

import (
	"encoding/json"
	"fmt"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const hookVectors = "local://🪝️hook-outputs.json"

type hookRecord struct {
	ID            string `json:"id"`
	HookEventName string `json:"hookEventName"`
	Allowed       bool   `json:"allowed"`
	Message       string `json:"message"`
}

type hookNative struct {
	Event    string `json:"event"`
	ToolKind string `json:"toolKind"`
}

type hookVectorFile struct {
	Editors []string     `json:"editors"`
	Records []hookRecord `json:"records"`
	Events  []string     `json:"events"`
	Parents []string     `json:"parents"`
	Natives []hookNative `json:"natives"`
}

func loadHookVectors(ctx *host.Context) (hookVectorFile, error) {
	var file hookVectorFile
	raw, err := ctx.FixtureBytes(hookVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("hook output vectors are not valid JSON: %w", err)
	}
	return file, nil
}

func editorProvider(kind string) (providers.EditorProvider, error) {
	provider := providers.GetEditorProvider(kind)
	if provider == nil {
		return nil, fmt.Errorf("no editor provider for %s", kind)
	}
	return provider, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyEditorFormatsTheSameResult(ctx *host.Context) (host.Outcome, error) {
	file, err := loadHookVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, record := range file.Records {
		result := model.HookResultBase{Allowed: record.Allowed, Message: record.Message}
		perEditor := map[string]any{}
		for _, kind := range file.Editors {
			provider, err := editorProvider(kind)
			if err != nil {
				return host.Outcome{}, err
			}
			var parsed any
			if err := json.Unmarshal([]byte(provider.FormatHookOutput(record.HookEventName, result)), &parsed); err != nil {
				return host.Outcome{}, fmt.Errorf("%s formatted %s into invalid JSON: %w", kind, record.ID, err)
			}
			perEditor[kind] = parsed
		}
		projection[record.ID] = perEditor
	}
	return host.Outcome{Projection: projection}, nil
}

func nativeEventNamesAreDerivedTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := loadHookVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	known := map[string]bool{}
	for _, event := range model.AllHookEvents {
		known[string(event)] = true
	}
	projection := map[string]any{}
	for _, kind := range file.Editors {
		provider, err := editorProvider(kind)
		if err != nil {
			return host.Outcome{}, err
		}
		names := []string{}
		for _, slug := range file.Events {
			if !known[slug] {
				return host.Outcome{}, fmt.Errorf("fixture names an unknown hook event %s", slug)
			}
			for _, parent := range file.Parents {
				names = append(names, fmt.Sprintf("%s|%s=%s", slug, parent, provider.NativeEventFromHookEvent(model.HookEvent(slug), parent)))
			}
		}
		projection[kind] = names
	}
	return host.Outcome{Projection: projection}, nil
}

func anUnknownNativeEventIsRejected(ctx *host.Context) (host.Outcome, error) {
	file, err := loadHookVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, kind := range file.Editors {
		provider, err := editorProvider(kind)
		if err != nil {
			return host.Outcome{}, err
		}
		resolved := []string{}
		for _, native := range file.Natives {
			answer := "refused"
			event, parent, resolveErr := provider.ResolveNativeEvent(native.Event, model.ToolKind(native.ToolKind))
			if resolveErr == nil {
				answer = fmt.Sprintf("%s|%s", string(event), parent)
			}
			resolved = append(resolved, fmt.Sprintf("%s/%s=%s", native.Event, native.ToolKind, answer))
		}
		projection[kind] = resolved
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-editor-formats-the-same-result", everyEditorFormatsTheSameResult).
		Subject("native-event-names-are-derived-the-same-way", nativeEventNamesAreDerivedTheSameWay).
		Subject("an-unknown-native-event-is-rejected", anUnknownNativeEventIsRejected)
}

// endregion 🔖️Registration
