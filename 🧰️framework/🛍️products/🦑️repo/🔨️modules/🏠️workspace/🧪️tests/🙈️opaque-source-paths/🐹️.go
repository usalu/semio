package workspace_test

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestOpaqueSourcePathsMatchNeutralCorpus(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "📚️library", "⚡️caching", "🔍️discovery", "📂️source", "🧪️tests", "🧫️fixtures", "cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	type vector struct {
		Name   string
		Opaque bool
	}
	var fixture struct {
		Cases          []vector
		PathCases      []vector
		GeneratedCases []vector
	}
	if err = json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, value := range append(append(fixture.Cases, fixture.PathCases...), fixture.GeneratedCases...) {
		t.Run(value.Name, func(t *testing.T) {
			if actual := workspace.IsRepoExcludedPath(value.Name); actual != value.Opaque {
				t.Fatalf("excluded(%q)=%v expected %v", value.Name, actual, value.Opaque)
			}
		})
	}
}

func TestOpaqueSourcePolicyTracksCanonicalSchemas(t *testing.T) {
	var descriptor struct {
		Definitions struct {
			RelativePreservationProtocol struct {
				Const struct {
					CapsulePrefix    string
					CapsuleDigestHex int
				} `json:"const"`
			}
		}
	}
	var retirement struct {
		Definitions struct {
			Policy struct {
				Const struct{ MarkerPrefix string } `json:"const"`
			}
		}
	}
	read := func(target any, parts ...string) {
		t.Helper()
		data, err := os.ReadFile(sandbox.OriginPath(parts...))
		if err != nil {
			t.Fatal(err)
		}
		if err := json.Unmarshal(data, target); err != nil {
			t.Fatal(err)
		}
	}
	read(&descriptor, "..", "..", "..", "..", "..", "..", "🔨️modules", "📁️filesystem", "🧾️observation", "📁️descriptor", "🧬️schema", "🔣️.json")
	read(&retirement, "..", "..", "..", "📚️library", "⚡️caching", "🚀️bootstrap", "📦️dependencies", "💾️storage", "🧹️retirement", "🧬️schema", "🔣️.json")
	var cachePolicy struct{ GeneratedDirectories []string }
	read(&cachePolicy, "..", "..", "..", "📚️library", "⚡️caching", "🔣️policy.json")
	for _, component := range cachePolicy.GeneratedDirectories {
		if !workspace.IsRepoExcludedPath(component+"/input.go") || !workspace.IsRepoExcludedPath("src/"+component+"/input.go") {
			t.Fatalf("canonical generated directory visible: %q", component)
		}
		if workspace.IsRepoExcludedPath("src/" + component + "-notes/input.go") {
			t.Fatalf("authored near-miss excluded: %q", component)
		}
	}
	protocol := descriptor.Definitions.RelativePreservationProtocol.Const
	if protocol.CapsulePrefix == "" || protocol.CapsuleDigestHex <= 0 || retirement.Definitions.Policy.Const.MarkerPrefix == "" {
		t.Fatal("invalid canonical opacity policy")
	}
	for _, prefix := range []string{protocol.CapsulePrefix, retirement.Definitions.Policy.Const.MarkerPrefix} {
		component := prefix + strings.Repeat("0", protocol.CapsuleDigestHex)
		if !workspace.IsRepoExcludedPath("src/" + component + "/pending.go") {
			t.Fatalf("canonical reserved component %q is visible", component)
		}
		if workspace.IsRepoExcludedPath("src/" + component + "0/pending.go") {
			t.Fatalf("nonprotocol component %q is opaque", component+"0")
		}
	}
}

func TestSourceExclusionsUseRepositoryRelativeCoordinates(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "📚️library", "⚡️caching", "🔍️discovery", "📂️source", "🧪️tests", "🧫️fixtures", "cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		CoordinateCases []struct {
			Root   string
			Path   string
			Opaque bool
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.CoordinateCases {
		t.Run(row.Root+"/"+row.Path, func(t *testing.T) {
			root := filepath.Join(t.TempDir(), filepath.FromSlash(workspace.NormalizePath(row.Root)))
			if err := os.MkdirAll(root, 0755); err != nil {
				t.Fatal(err)
			}
			sandbox.BindRoot(t, root)
			absolute := filepath.Join(root, filepath.FromSlash(workspace.NormalizePath(row.Path)))
			if actual := workspace.IsRepoExcludedPath(absolute); actual != row.Opaque {
				t.Fatalf("root=%q path=%q excluded=%v expected %v", root, absolute, actual, row.Opaque)
			}
		})
	}
}
