// 🐹️ Go side of the autofix round trip case.
package adapter

import (
	"fmt"
	"os"
	"strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	host "semio.tech/repo/test"
)

// region 🔖️Rendering

// readFixture reads one declared fixture as text.
func readFixture(ctx *host.Context, uri string) (string, error) {
	path, err := ctx.Fixture(uri)
	if err != nil {
		return "", err
	}
	content, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("cannot read %s: %w", uri, err)
	}
	return string(content), nil
}

// renderBreachs renders every breach as `kind|scope|line`.
func renderBreachs(breachs []model.Breach) []string {
	rows := make([]string, 0, len(breachs))
	for _, breach := range breachs {
		rows = append(rows, fmt.Sprintf("%s|%s|%d", string(breach.Kind), breach.Scope, breach.Line))
	}
	return rows
}

// renderStatutes renders every repaired statute identifier.
func renderStatutes(kinds []model.Statute) []string {
	rows := make([]string, 0, len(kinds))
	for _, kind := range kinds {
		rows = append(rows, string(kind))
	}
	return rows
}

// analyzeOne analyses exactly one source.
func analyzeOne(source statutes.SourceFile) []model.Breach {
	return statutes.Analyze(statutes.NewSourceSet([]statutes.SourceFile{source}))
}

// endregion 🔖️Rendering

// region 🔖️Scenarios

func theFixableSourceBecomesTheExpectedSource(ctx *host.Context) (host.Outcome, error) {
	fixableURI := "shared://📁️some/📁️folder/🧪️file-fixable/🟦️.tsx"
	expectedURI := "shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx"
	fixableContent, err := readFixture(ctx, fixableURI)
	if err != nil {
		return host.Outcome{}, err
	}
	expected, err := readFixture(ctx, expectedURI)
	if err != nil {
		return host.Outcome{}, err
	}
	source := statutes.SourceFile{Path: strings.TrimPrefix(fixableURI, "shared://"), Content: fixableContent}
	before := analyzeOne(source)
	first := statutes.Autofix(source)
	second := statutes.Autofix(statutes.SourceFile{Path: source.Path, Content: first.Content})
	after := analyzeOne(statutes.SourceFile{Path: source.Path, Content: first.Content})
	if first.Content != expected {
		return host.Outcome{}, fmt.Errorf("the repaired text is not the expected text")
	}
	if second.Content != first.Content || len(second.Fixed) != 0 {
		return host.Outcome{}, fmt.Errorf("the repair is not idempotent")
	}
	return host.Outcome{Projection: map[string]any{
		"before":          renderBreachs(before),
		"fixed":           renderStatutes(first.Fixed),
		"matchesExpected": fmt.Sprintf("%t", first.Content == expected),
		"idempotent":      fmt.Sprintf("%t", second.Content == first.Content),
		"after":           renderBreachs(after),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-fixable-source-becomes-the-expected-source", theFixableSourceBecomesTheExpectedSource)
}

// endregion 🔖️Registration
