// 🛡️ Tests of the breach cache analysis and the system policy.

package graphql

import (
	"encoding/json"
	"github.com/usalu/semio/repo/model"
	"github.com/usalu/semio/repo/statutes"
	"github.com/usalu/semio/repo/workspace"
	"github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"path/filepath"
	"testing"
)

func TestAnalyzeReadsBreachCacheJSON(t *testing.T) {
	tmp := t.TempDir()
	cacheDir := filepath.Join(tmp, ".🧬semio", "🦑️repo", "⚡️cache", "breaches")
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		t.Fatal(err)
	}
	cachePath := filepath.Join(cacheDir, "unit-test.json")
	payload := `{
  "entityId": "test",
  "script": "unit.test.lint.script.ts",
  "breachs": [
    {
      "id": "e1",
      "summary": "hello",
      "kind": "lint/test/rule",
      "scope": "repo/example.go",
      "priority": "medium"
    }
  ]
}`
	if err := os.WriteFile(cachePath, []byte(payload), 0o644); err != nil {
		t.Fatal(err)
	}
	sandbox.BindRoot(t, tmp)
	ctx := NewRepoContext(tmp)
	ar, err := ctx.Analyze(nil)
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if ar == nil || len(ar.Breachs) != 1 {
		t.Fatalf("expected 1 breach, got %#v", ar)
	}
	if ar.Breachs[0].Summary != "hello" {
		t.Fatalf("unexpected breach: %+v", ar.Breachs[0])
	}
}

func TestSystemPolicy(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "🎛️dashboard", "🧪️tests", "🧫️fixtures", "🧭️authority", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		PathCases []struct {
			Path     string
			Accepted bool
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.PathCases {
		t.Run(row.Path, func(t *testing.T) {
			ctx := statutes.NewPolicyContextWithFiles(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{}, []string{row.Path})
			breachs := statutes.SystemPolicy(ctx)
			if (len(breachs) == 0) != row.Accepted {
				t.Fatalf("accepted = %v, breachs = %v", row.Accepted, breachs)
			}
			if len(breachs) > 0 && breachs[0].Kind != model.BreachSystemIdeConfiguration {
				t.Fatalf("unexpected statute %v", breachs[0].Kind)
			}
		})
	}
	policy, found := statutes.FindPolicy("system")
	if !found || len(policy.AllKinds()) != 1 || policy.AllKinds()[0] != model.BreachSystemIdeConfiguration {
		t.Fatalf("unexpected system policy %v", policy)
	}
	if model.BreachSystemIdeConfiguration.Info().Autofixable || model.BreachSystemIdeConfiguration.Info().Priority != model.BreachPriorityHigh {
		t.Fatal("IDE configuration removal must be explicit and high priority")
	}
}
