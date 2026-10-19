// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Territory, policy definitions, system policy, and policy groups.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"os"
	"strings"
	"testing"

)

func TestTerritory(t *testing.T) {
	t.Run("AllKinds flat", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
			Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingSummary {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingSummary, kinds[1])
		}
	})
	t.Run("AllKinds nested groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
				{
					Name:        "Section",
					Description: "Section-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeSectionEmpty},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
	})
	t.Run("AllKinds mixed kinds and groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Kinds:       []Statute{BreachCodeCommentInline},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeCommentInline {
			t.Errorf("expected %s first, got %s", BreachCodeCommentInline, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s second, got %s", BreachCodeFileMissingHeaderRegion, kinds[1])
		}
	})
	t.Run("AllKinds deeply nested", func(t *testing.T) {
		g := Territory{
			Name:   "Root",
			Scopes: []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "Level1",
					Scopes: []string{"**/*"},
					Groups: []Territory{
						{
							Name:   "Level2",
							Scopes: []string{"**/*"},
							Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
						},
					},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 1 {
			t.Fatalf("expected 1 kind, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
	})
	t.Run("AllKinds empty group", func(t *testing.T) {
		g := Territory{
			Name:   "Empty",
			Scopes: []string{"**/*"},
		}
		kinds := g.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
	t.Run("GetID and GetURI", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
		}
		id := g.GetID()
		if id == "" {
			t.Error("expected non-empty ID")
		}
		if !strings.Contains(id, "File") {
			t.Errorf("expected ID to contain 'File', got %s", id)
		}
		uri := g.GetURI()
		if uri == "" {
			t.Error("expected non-empty URI")
		}
		if !strings.HasPrefix(uri, "repo://") {
			t.Errorf("expected URI to start with 'repo://', got %s", uri)
		}
	})
}

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestSystemPolicy(t *testing.T) {
	data, err := os.ReadFile("../../../../🎛️dashboard/🧫️fixtures/🧭️authority/🔣️.json")
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
			ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeRepo}, []Bundle{}, []string{row.Path})
			breachs := systemPolicy(ctx)
			if (len(breachs) == 0) != row.Accepted {
				t.Fatalf("accepted = %v, breachs = %v", row.Accepted, breachs)
			}
			if len(breachs) > 0 && breachs[0].Kind != BreachSystemIdeConfiguration {
				t.Fatalf("unexpected statute %v", breachs[0].Kind)
			}
		})
	}
	policy, found := FindPolicy("system")
	if !found || len(policy.AllKinds()) != 1 || policy.AllKinds()[0] != BreachSystemIdeConfiguration {
		t.Fatalf("unexpected system policy %v", policy)
	}
	if BreachSystemIdeConfiguration.Info().Autofixable || BreachSystemIdeConfiguration.Info().Priority != BreachPriorityHigh {
		t.Fatal("IDE configuration removal must be explicit and high priority")
	}
}

func TestBuildTerritoryTree(t *testing.T) {
	t.Run("single group with kinds", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "File",
				Description: "File breachs",
				Scopes:      []string{"**/*.ts"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 node, got %d", len(nodes))
		}
		if nodes[0].Label != "File" {
			t.Errorf("expected label 'File', got %s", nodes[0].Label)
		}
		if nodes[0].Kind != TreeNodeCategory {
			t.Errorf("expected category kind, got %s", nodes[0].Kind)
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		for _, child := range nodes[0].Children {
			if child.Kind != TreeNodeStatute {
				t.Errorf("expected statute node, got %s", child.Kind)
			}
		}
	})
	t.Run("nested groups", func(t *testing.T) {
		groups := []Territory{
			{
				Name:   "Code",
				Scopes: []string{"**/*.ts"},
				Groups: []Territory{
					{
						Name:   "File",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
					},
					{
						Name:   "Section",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeSectionEmpty},
					},
				},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 root node, got %d", len(nodes))
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		fileGroup := nodes[0].Children[0]
		if fileGroup.Label != "File" {
			t.Errorf("expected label 'File', got %s", fileGroup.Label)
		}
		if len(fileGroup.Children) != 1 {
			t.Fatalf("expected 1 child in File group, got %d", len(fileGroup.Children))
		}
	})
	t.Run("empty groups", func(t *testing.T) {
		nodes := buildTerritoryTree(nil)
		if len(nodes) != 0 {
			t.Fatalf("expected 0 nodes, got %d", len(nodes))
		}
	})
	t.Run("group node data contains scopes", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "Sketchpad",
				Description: "Sketchpad breachs",
				Scopes:      []string{"js/sketchpad/**/*.ts", "js/sketchpad/**/*.tsx"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
			},
		}
		nodes := buildTerritoryTree(groups)
		data := nodes[0].Data
		if data == nil {
			t.Fatal("expected non-nil data")
		}
		scopes, ok := data["scopes"].([]string)
		if !ok {
			t.Fatal("expected scopes in data")
		}
		if len(scopes) != 2 {
			t.Fatalf("expected 2 scopes, got %d", len(scopes))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestExhaustivePolicyGroupsGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy groups graphql test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	query := `{ policies { id name groups { name description scopes kinds { id } groups { name kinds { id } } } } }`
	result, err := executor.Execute(context.Background(), query, nil)
	if err != nil {
		t.Fatalf("GraphQL query failed: %v", err)
	}
	data, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected map result")
	}
	policiesData, ok := data["policies"].([]interface{})
	if !ok {
		t.Fatal("expected policies array")
	}
	if len(policiesData) == 0 {
		t.Fatal("expected at least one policy")
	}
	for _, pRaw := range policiesData {
		p, ok := pRaw.(map[string]interface{})
		if !ok {
			continue
		}
		groups, ok := p["groups"].([]interface{})
		if !ok {
			t.Fatalf("expected groups array for policy %v", p["id"])
		}
		if len(groups) == 0 {
			t.Errorf("expected at least one group for policy %v", p["id"])
		}
	}
}
