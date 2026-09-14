// 🐹️ Go side of the ignore directive case.
package adapter

import (
	"encoding/json"
	"fmt"
	"sort"
	"strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type directiveProbe struct {
	Line int    `json:"line"`
	Kind string `json:"kind"`
}

type directiveDocument struct {
	Name    string           `json:"name"`
	Content string           `json:"content"`
	Probes  []directiveProbe `json:"probes"`
}

type directiveVectorFile struct {
	Documents []directiveDocument `json:"documents"`
}

func loadDirectiveVectors(ctx *host.Context) (directiveVectorFile, error) {
	data, err := ctx.FixtureBytes("local://🔣️vectors.json")
	if err != nil {
		return directiveVectorFile{}, err
	}
	var file directiveVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

// renderDirectives renders the parsed directives of one document as `line=prefix,prefix` rows.
func renderDirectives(content string) []string {
	parsed := statutes.ParseIgnoreDirectives(content)
	lines := make([]int, 0, len(parsed))
	for line := range parsed {
		lines = append(lines, line)
	}
	sort.Ints(lines)
	rows := make([]string, 0, len(lines))
	for _, line := range lines {
		rows = append(rows, fmt.Sprintf("%d=%s", line, strings.Join(parsed[line], ",")))
	}
	return rows
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func aDirectiveSuppressesItsPrefixes(ctx *host.Context) (host.Outcome, error) {
	file, err := loadDirectiveVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	directives := []string{}
	decisions := []string{}
	for _, document := range file.Documents {
		for _, row := range renderDirectives(document.Content) {
			directives = append(directives, document.Name+":"+row)
		}
		parsed := statutes.ParseIgnoreDirectives(document.Content)
		for _, probe := range document.Probes {
			kind := model.Statute(probe.Kind)
			decisions = append(decisions, fmt.Sprintf("%s:%d:%s=%t", document.Name, probe.Line, probe.Kind, statutes.IsIgnored(parsed, probe.Line, kind)))
		}
	}
	return host.Outcome{Projection: map[string]any{"directives": directives, "decisions": decisions}}, nil
}

func aDirectiveOnlyReachesForward(ctx *host.Context) (host.Outcome, error) {
	file, err := loadDirectiveVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	var document directiveDocument
	for _, candidate := range file.Documents {
		if candidate.Name == "single-prefix" {
			document = candidate
		}
	}
	if document.Name == "" {
		return host.Outcome{}, fmt.Errorf("the single-prefix document is missing")
	}
	parsed := statutes.ParseIgnoreDirectives(document.Content)
	kind := model.Statute("code/section/empty")
	window := []string{}
	for _, line := range []int{1, 2, 3, 102, 103} {
		window = append(window, fmt.Sprintf("%d=%t", line, statutes.IsIgnored(parsed, line, kind)))
	}
	if statutes.IsIgnored(parsed, 2, kind) {
		return host.Outcome{}, fmt.Errorf("a directive suppressed its own line")
	}
	if statutes.IsIgnored(parsed, 103, kind) {
		return host.Outcome{}, fmt.Errorf("a directive reached beyond its hundred line window")
	}
	if !statutes.IsIgnored(parsed, 102, kind) {
		return host.Outcome{}, fmt.Errorf("a directive did not reach the last line of its window")
	}
	return host.Outcome{Projection: map[string]any{"window": window}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("a-directive-suppresses-its-prefixes", aDirectiveSuppressesItsPrefixes).
		Subject("a-directive-only-reaches-forward", aDirectiveOnlyReachesForward)
}

// endregion 🔖️Registration
