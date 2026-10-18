// ⚖️ Tests of the statute meta and the territories.

package model

import (
	strings "strings"
	testing "testing"
)

func TestFixStatuteMeta(t *testing.T) {
	autofixableKinds := []Statute{
		BreachCodeFileMissingHeaderRegion,
		BreachCodeFileMissingLicense,
		BreachCodeFileWrongLicense,
		BreachCodeSectionEmpty,
		BreachCodeSectionMissingEndName,
		BreachCodeSectionNameMismatch,
		BreachCodeCommentInline,
		BreachCodeCommentBlock,
		BreachCodeCommentJSDoc,
	}
	for _, kind := range autofixableKinds {
		info := kind.Info()
		if !info.Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
		if info.Reason == "" {
			t.Errorf("statute %s has empty reason", kind)
		}
		if info.Solution == "" {
			t.Errorf("statute %s has empty solution", kind)
		}
	}

	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		info := kind.Info()
		if info.Autofixable {
			t.Errorf("statute %s should NOT be autofixable", kind)
		}
	}
}

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
