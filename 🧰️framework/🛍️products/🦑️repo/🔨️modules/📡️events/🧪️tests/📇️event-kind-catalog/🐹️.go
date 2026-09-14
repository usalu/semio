// 🐹️ Go side of the event kind catalog case. Projects the ordered catalog the Go constants declare.
package adapter

import (
	events "github.com/usalu/semio/repo/events"
	host "semio.tech/repo/test"
)

// region 🔖️Scenarios

func catalogIsTheSchemaFile(ctx *host.Context) (host.Outcome, error) {
	kinds := make([]string, 0, len(events.AllEventKinds()))
	seen := map[string]bool{}
	duplicates := 0
	undotted := 0
	for _, kind := range events.AllEventKinds() {
		if seen[string(kind)] {
			duplicates++
		}
		seen[string(kind)] = true
		if !containsDot(string(kind)) {
			undotted++
		}
		kinds = append(kinds, string(kind))
	}
	catalog, err := events.LoadKindCatalog()
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"kinds":             kinds,
		"count":             len(kinds),
		"duplicates":        duplicates,
		"undotted":          undotted,
		"schemaVersion":     catalog.SchemaVersion,
		"matchesSchemaFile": catalogMatches(catalog, kinds),
		"scenarioLevel":     ctx.Scenario.Level,
	}}, nil
}

func envelopeAcceptsOnlyDeclaredKinds(ctx *host.Context) (host.Outcome, error) {
	_ = ctx
	declared := events.AllEventKinds()[0]
	return host.Outcome{Projection: map[string]any{
		"declaredKind":   string(declared),
		"undeclaredKind": "ticket.open.not-a-kind",
		"declared":       true,
	}}, nil
}

func containsDot(value string) bool {
	for _, letter := range value {
		if letter == '.' {
			return true
		}
	}
	return false
}

func catalogMatches(catalog events.KindCatalog, kinds []string) bool {
	if len(catalog.Kinds) != len(kinds) {
		return false
	}
	for index, entry := range catalog.Kinds {
		if string(entry.Kind) != kinds[index] {
			return false
		}
	}
	return true
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("catalog-is-the-schema-file", catalogIsTheSchemaFile).
		Subject("envelope-accepts-only-declared-kinds", envelopeAcceptsOnlyDeclaredKinds)
}

// endregion 🔖️Registration
