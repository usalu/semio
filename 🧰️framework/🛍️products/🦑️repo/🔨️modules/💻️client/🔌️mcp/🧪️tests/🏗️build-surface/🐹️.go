// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Build surface tests: the tools are derived from the neutral catalogue, cover every catalogue row, refuse drift, and agree with the language-neutral golden vectors.

// #endregion 🧲️Header

package main

import (
	"encoding/json"
	"os"
	"reflect"
	"slices"
	"sort"
	"strings"
	"testing"

	cli "github.com/usalu/semio/repo/cli"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// #region 🏗️BuildSurfaceTests

func originFile(t *testing.T, segments ...string) []byte {
	t.Helper()
	data, err := os.ReadFile(sandbox.OriginPath(append([]string{"..", ".."}, segments...)...))
	if err != nil {
		t.Fatal(err)
	}
	return data
}

func authoredSurface(t *testing.T) (Catalogue, Policy) {
	t.Helper()
	backends, err := ParseBackendIds(originFile(t, "📚️library", "🏗️build", "🔌️backends", "📋️registry", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	catalogue, err := ParseCatalogue(originFile(t, "📚️library", "🏗️build", "⚙️operations", "🔣️catalog.json"), backends)
	if err != nil {
		t.Fatal(err)
	}
	policy, err := ParsePolicy(originFile(t, "🔌️mcp", "🖼️assets", "🏗️build-surface.json"))
	if err != nil {
		t.Fatal(err)
	}
	return catalogue, policy
}

type goldenVector struct {
	ID          string            `json:"id"`
	Tool        string            `json:"tool"`
	Arguments   json.RawMessage   `json:"arguments"`
	Valid       bool              `json:"valid"`
	Argv        []string          `json:"argv"`
	Environment map[string]string `json:"environment"`
	Agent       string            `json:"agent"`
}

type goldenSurface struct {
	Schema  string         `json:"schema"`
	Tools   []string       `json:"tools"`
	Vectors []goldenVector `json:"vectors"`
}

func golden(t *testing.T) goldenSurface {
	t.Helper()
	var fixture goldenSurface
	if err := decodeExact(originFile(t, "🔌️mcp", "🧪️tests", "🧫️fixtures", "🏗️build-surface.json"), &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.mcp.build-surface/1" {
		t.Fatalf("unexpected golden schema %q", fixture.Schema)
	}
	return fixture
}

func toolsByName(tools []BuildTool) map[string]BuildTool {
	index := map[string]BuildTool{}
	for _, tool := range tools {
		index[tool.Name] = tool
	}
	return index
}

func TestBuildToolsAreDerivedFromTheCatalogue(t *testing.T) {
	catalogue, policy := authoredSurface(t)
	tools, err := Derive(catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	names := make([]string, len(tools))
	for index, tool := range tools {
		names[index] = tool.Name
	}
	if want := golden(t).Tools; !slices.Equal(names, want) {
		t.Fatalf("tools got=%q want=%q", names, want)
	}
	for _, tool := range tools {
		operation, _ := catalogue.operation(tool.Operation)
		summary, synopsis := catalogue.summary(operation, tool.Subcommand)
		for _, policyTool := range policy.Tools {
			if (policyTool.Name == tool.Name || (policyTool.Name == "" && ToolName(policyTool.Operation, policyTool.Subcommand) == tool.Name)) && len(policyTool.Summary) > 0 {
				summary = policyTool.Summary
			}
		}
		for _, language := range []string{"en", "de"} {
			if summary[language] == "" || !strings.Contains(tool.Schema.Description, summary[language]) {
				t.Errorf("%s: description lacks the %s summary %q", tool.Name, language, summary[language])
			}
		}
		if !strings.Contains(tool.Schema.Description, "repo "+synopsis) {
			t.Errorf("%s: description lacks the synopsis %q", tool.Name, synopsis)
		}
		if tool.Schema.InputSchema.AdditionalProperties == nil || *tool.Schema.InputSchema.AdditionalProperties {
			t.Errorf("%s: arguments must be closed", tool.Name)
		}
		if tool.Schema.OutputSchema == nil || tool.Schema.Execution == nil || tool.Schema.Annotations == nil {
			t.Errorf("%s: output schema, execution and annotations are required", tool.Name)
		}
		for _, input := range tool.Inputs {
			if !strings.Contains(input.Description, "EN: ") || !strings.Contains(input.Description, "DE: ") {
				t.Errorf("%s.%s: description is not bilingual", tool.Name, input.Name)
			}
		}
	}
}

func TestCatalogueExpandsTheRegisteredBackends(t *testing.T) {
	backends, err := ParseBackendIds(originFile(t, "📚️library", "🏗️build", "🔌️backends", "📋️registry", "🔣️.json"))
	if err != nil || len(backends) < 2 {
		t.Fatalf("the registry names at least two backends: %v %v", backends, err)
	}
	catalogue, policy := authoredSurface(t)
	row, ok := catalogue.flag("--tool", "")
	if !ok || row.ValueName != strings.Join(backends, "|") {
		t.Fatalf("--tool value set got=%q want=%q", row.ValueName, strings.Join(backends, "|"))
	}
	tools, err := Derive(catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	if len(tools) == 0 {
		t.Fatal("no build tools derived")
	}
	for _, tool := range tools {
		if strings.Contains(tool.Schema.Description, "{backends}") {
			t.Errorf("%s: placeholder left in the description", tool.Name)
		}
	}
	for _, bad := range []string{`{"backends":[]}`, `{"backends":[{"id":"a|b"}]}`, `{"backends":[{"id":""}]}`, `[`} {
		if _, err := ParseBackendIds([]byte(bad)); err == nil {
			t.Errorf("registry %s must be refused", bad)
		}
	}
}

func TestEveryCatalogueFlagOfAnExposedRowIsAnInputBoundOrOmitted(t *testing.T) {
	catalogue, policy := authoredSurface(t)
	tools, err := Derive(catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	omitted := map[string]bool{}
	for _, row := range policy.OmittedFlags {
		omitted[row.Flag] = true
	}
	for _, tool := range tools {
		operation, _ := catalogue.operation(tool.Operation)
		var spec PolicyTool
		for _, candidate := range policy.Tools {
			if candidate.Name == tool.Name || (candidate.Name == "" && ToolName(candidate.Operation, candidate.Subcommand) == tool.Name) {
				spec = candidate
			}
		}
		flags, _ := catalogue.effectiveFlags(operation, tool.Subcommand)
		if spec.Flags != nil {
			flags = *spec.Flags
		}
		inputs := map[string]bool{}
		for _, input := range tool.Inputs {
			inputs[input.Flag] = true
		}
		bound := map[string]bool{}
		for _, row := range tool.Bound {
			bound[row.Flag] = true
		}
		for _, flag := range flags {
			if !inputs[flag] && !bound[flag] && !omitted[flag] {
				t.Errorf("%s: %s is neither an input, bound nor omitted", tool.Name, flag)
			}
		}
	}
}

func TestBuildSurfaceRefusesDriftBetweenCatalogueAndPolicy(t *testing.T) {
	catalogue, policy := authoredSurface(t)
	extra := catalogue
	extra.Operations = append(append([]CatalogueOperation(nil), catalogue.Operations...), CatalogueOperation{Name: "brand-new", Summary: Text{"en": "New.", "de": "Neu."}, Synopsis: "brand-new"})
	if _, err := Derive(extra, policy); err == nil || !strings.Contains(err.Error(), "neither exposed nor excluded") || !strings.Contains(err.Error(), "brand-new") {
		t.Fatalf("a new catalogue row must not be forgotten silently: %v", err)
	}
	unknown := policy
	unknown.Tools = append(append([]PolicyTool(nil), policy.Tools...), PolicyTool{Operation: "vanished", Kind: "query"})
	if _, err := Derive(catalogue, unknown); err == nil || !strings.Contains(err.Error(), "unknown operation") {
		t.Fatalf("a policy row without a catalogue row must be refused: %v", err)
	}
	both := policy
	both.Excluded = append(append([]PolicyExcluded(nil), policy.Excluded...), PolicyExcluded{Operation: "run", Reason: Text{"en": "x", "de": "y"}})
	if _, err := Derive(catalogue, both); err == nil || !strings.Contains(err.Error(), "both exposed and excluded") {
		t.Fatalf("a row cannot be exposed and excluded: %v", err)
	}
	unreasoned := policy
	unreasoned.Excluded = append([]PolicyExcluded(nil), policy.Excluded...)
	unreasoned.Excluded[0].Reason = Text{"en": "only English"}
	if _, err := Derive(catalogue, unreasoned); err == nil {
		t.Fatal("an exclusion needs a reason in both languages")
	}
	unknownFlag := policy
	unknownFlag.Tools = append([]PolicyTool(nil), policy.Tools...)
	unknownFlag.Tools[0].Required = []string{"--nope"}
	if _, err := Derive(catalogue, unknownFlag); err == nil {
		t.Fatal("a required flag the row does not accept must be refused")
	}
	if _, err := ParsePolicy([]byte(`{"version":1,"tools":[{}],"bogus":true}`)); err == nil {
		t.Fatal("the authored policy is closed")
	}
}

func TestGoldenVectorsAgree(t *testing.T) {
	catalogue, policy := authoredSurface(t)
	tools, err := Derive(catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	index := toolsByName(tools)
	for _, vector := range golden(t).Vectors {
		t.Run(vector.ID, func(t *testing.T) {
			tool, found := index[vector.Tool]
			if !found {
				t.Fatalf("no tool %q", vector.Tool)
			}
			invocation, err := tool.Plan(vector.Arguments)
			if !vector.Valid {
				var refusal *ArgumentError
				if err == nil {
					t.Fatalf("accepted %s", vector.Arguments)
				}
				if ok := asArgumentError(err, &refusal); !ok {
					t.Fatalf("refusal is not an ArgumentError: %v", err)
				}
				return
			}
			if err != nil {
				t.Fatal(err)
			}
			if !slices.Equal(invocation.Argv, vector.Argv) {
				t.Fatalf("argv got=%q want=%q", invocation.Argv, vector.Argv)
			}
			want := vector.Environment
			if want == nil {
				want = map[string]string{}
			}
			if !reflect.DeepEqual(invocation.Environment, want) {
				t.Fatalf("environment got=%v want=%v", invocation.Environment, want)
			}
			if invocation.Agent != vector.Agent {
				t.Fatalf("agent got=%q want=%q", invocation.Agent, vector.Agent)
			}
		})
	}
}

func asArgumentError(err error, target **ArgumentError) bool {
	refusal, ok := err.(*ArgumentError)
	if ok {
		*target = refusal
	}
	return ok
}

func TestEveryCatalogueOperationRoutesToTheBuildOwner(t *testing.T) {
	catalogue, _ := authoredSurface(t)
	for _, operation := range catalogue.Operations {
		if plan := cli.PlanNativeEntrypoint([]string{operation.Name, "--opaque=value"}); plan.Kind != "build" {
			t.Errorf("repo %s routes to %q, not to the build owner", operation.Name, plan.Kind)
		}
	}
}

func TestNativeEntrypointVectorsCoverEveryCatalogueOperation(t *testing.T) {
	var fixture struct {
		BuildVerbs []string `json:"buildVerbs"`
		Vectors    []struct {
			Argv      []string `json:"argv"`
			Kind      string   `json:"kind"`
			Arguments []string `json:"arguments"`
		} `json:"vectors"`
	}
	if err := json.Unmarshal(originFile(t, "⌨️cli", "🧫️fixtures", "🚪️entrypoint", "🔣️.json"), &fixture); err != nil {
		t.Fatal(err)
	}
	catalogue, _ := authoredSurface(t)
	for _, operation := range catalogue.Operations {
		if !slices.Contains(fixture.BuildVerbs, operation.Name) {
			t.Errorf("the shared entrypoint fixture lacks the build verb %s", operation.Name)
		}
	}
	for _, vector := range fixture.Vectors {
		plan := cli.PlanNativeEntrypoint(vector.Argv)
		if plan.Kind != vector.Kind || !slices.Equal(plan.Arguments, vector.Arguments) {
			t.Errorf("%v: got %+v want %s %q", vector.Argv, plan, vector.Kind, vector.Arguments)
		}
	}
}

func TestInstructionsNameNoDefaultBuildTool(t *testing.T) {
	for _, stale := range []string{"remains the default", "Nx is the default", "Bazel is the default"} {
		if strings.Contains(RepositoryInstructions, stale) {
			t.Errorf("instructions state a default tool: %q", stale)
		}
	}
	for _, required := range []string{"build policy chooses the default build tool", "Deutsch:", "--tool=nx|bazel", "agent", "graph_policy=current"} {
		if !strings.Contains(RepositoryInstructions, required) {
			t.Errorf("instructions lack %q", required)
		}
	}
}

func TestListingNamesExposedAndExcludedRows(t *testing.T) {
	catalogue, policy := authoredSurface(t)
	surface, err := NewBuildSurface(t.TempDir(), catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	data, err := surface.Listing()
	if err != nil {
		t.Fatal(err)
	}
	var listing struct {
		Tools    []struct{ Tool string }
		Excluded []struct {
			Operation string
			Reason    Text
		}
	}
	if err := json.Unmarshal(data, &listing); err != nil {
		t.Fatal(err)
	}
	var names []string
	for _, row := range listing.Tools {
		names = append(names, row.Tool)
	}
	if !sort.StringsAreSorted(names) || len(names) != len(surface.Tools) || len(listing.Excluded) != len(policy.Excluded) {
		t.Fatalf("listing does not match the surface: %s", data)
	}
	for _, row := range listing.Excluded {
		if row.Reason["en"] == "" || row.Reason["de"] == "" {
			t.Errorf("exclusion %s lacks a reason", row.Operation)
		}
	}
}

// #endregion 🏗️BuildSurfaceTests
