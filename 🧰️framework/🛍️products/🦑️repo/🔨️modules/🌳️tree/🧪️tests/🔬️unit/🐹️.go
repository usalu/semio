// 🔬️ Tests of the tree domain, split out of the pre-split godfile suite.

package tree

import (
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

func findTestRepoRoot(start string) string {
	for _, candidate := range []string{start, func() string {
		_, file, _, ok := runtime.Caller(0)
		if !ok {
			return ""
		}
		return filepath.Dir(file)
	}()} {
		dir := candidate
		if dir == "" {
			continue
		}
		for {
			if _, err := os.Stat(filepath.Join(dir, "repo", "client", "main.go")); err == nil {
				return dir
			}
			if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
				return dir
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break
			}
			dir = parent
		}
	}
	return start
}

func TestBuildTerritoryTree(t *testing.T) {
	t.Run("single group with kinds", func(t *testing.T) {
		groups := []model.Territory{
			{
				Name:        "File",
				Description: "File breachs",
				Scopes:      []string{"**/*.ts"},
				Kinds:       []model.Statute{model.BreachCodeFileMissingHeaderRegion, model.BreachCodeFileMissingSummary},
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
		groups := []model.Territory{
			{
				Name:   "Code",
				Scopes: []string{"**/*.ts"},
				Groups: []model.Territory{
					{
						Name:   "File",
						Scopes: []string{"**/*.ts"},
						Kinds:  []model.Statute{model.BreachCodeFileMissingHeaderRegion},
					},
					{
						Name:   "Section",
						Scopes: []string{"**/*.ts"},
						Kinds:  []model.Statute{model.BreachCodeSectionEmpty},
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
		groups := []model.Territory{
			{
				Name:        "Sketchpad",
				Description: "Sketchpad breachs",
				Scopes:      []string{"js/sketchpad/**/*.ts", "js/sketchpad/**/*.tsx"},
				Kinds:       []model.Statute{model.BreachCodeFileMissingHeaderRegion},
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

func TestPropagateParentIDs(t *testing.T) {
	root := &TreeNode{Kind: TreeNodeCategory, Data: map[string]interface{}{}, Children: []*TreeNode{
		{Kind: TreeNodeTechnology, Data: map[string]interface{}{"name": "compose", "kind": "user"}, Children: []*TreeNode{
			{Kind: TreeNodeBundle, Data: map[string]interface{}{"name": "compose/js", "kind": "library"}, Children: []*TreeNode{
				{Kind: TreeNodeFolder, Data: map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization"}, Children: []*TreeNode{
					{Kind: TreeNodeFile, Data: map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code"}, Children: []*TreeNode{
						{Kind: TreeNodeSection, Data: map[string]interface{}{"name": "Store"}, Children: []*TreeNode{
							{Kind: TreeNodeDefinition, Data: map[string]interface{}{"name": "createStore", "kind": "implementation"}},
						}},
					}},
				}},
			}},
		}},
	}}
	PropagateParentIDs(root, "", DefaultArtifactIdentifier{})
	technologyId := model.EmojiText(model.EmojiTechnologyUser) + "compose"
	bundleId := technologyId + model.EmojiText(model.EmojiBundleLibrary) + "js"
	folderId := bundleId + model.EmojiText(model.EmojiFolderOrg) + "sketchpad"
	fileId := folderId + model.EmojiText(model.EmojiFileCode) + "design"
	sectionId := fileId + model.EmojiText(model.EmojiSection) + "store"
	defId := sectionId + model.EmojiText(model.EmojiDefinitionImpl) + "createstore"
	checks := []struct {
		label    string
		node     *TreeNode
		expected string
	}{
		{"technology", root.Children[0], technologyId},
		{"bundle", root.Children[0].Children[0], bundleId},
		{"folder", root.Children[0].Children[0].Children[0], folderId},
		{"file", root.Children[0].Children[0].Children[0].Children[0], fileId},
		{"section", root.Children[0].Children[0].Children[0].Children[0].Children[0], sectionId},
		{"definition", root.Children[0].Children[0].Children[0].Children[0].Children[0].Children[0], defId},
	}
	for _, c := range checks {
		t.Run(c.label, func(t *testing.T) {
			entityKind := TreeNodeKindToEntityKind(c.node.Kind)
			got := model.GetArtifactID(entityKind, c.node.Data)
			if got != c.expected {
				t.Errorf("expected %q, got %q", c.expected, got)
			}
		})
	}
}

func verifyTreeDocument(t *testing.T, node *TreeNode, parentPrefix string) {
	t.Helper()
	entityKind := TreeNodeKindToEntityKind(node.Kind)
	if entityKind == "" {
		for _, child := range node.Children {
			verifyTreeDocument(t, child, parentPrefix)
		}
		return
	}
	id := model.GetArtifactID(entityKind, node.Data)
	if id == "" {
		return
	}
	if parentPrefix != "" && !strings.HasPrefix(id, parentPrefix) {
		t.Errorf("%s %q: id %q should start with parent prefix %q", entityKind, node.Label, id, parentPrefix)
	}
	for _, child := range node.Children {
		verifyTreeDocument(t, child, id)
	}
}

func TestEntityKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "file", "line", "range",
		"section", "definition", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(EntityKinds) != len(expected) {
		t.Fatalf("EntityKinds length: expected %d, got %d", len(expected), len(EntityKinds))
	}
	for i, e := range expected {
		if EntityKinds[i] != e {
			t.Errorf("EntityKinds[%d]: expected %q, got %q", i, e, EntityKinds[i])
		}
	}
}

func TestArtifactKinds(t *testing.T) {
	expected := []string{"repo", "technology", "bundle", "folder", "file", "section", "definition"}
	if len(ArtifactKinds) != len(expected) {
		t.Fatalf("ArtifactKinds length: expected %d, got %d", len(expected), len(ArtifactKinds))
	}
	for i, e := range expected {
		if ArtifactKinds[i] != e {
			t.Errorf("ArtifactKinds[%d]: expected %q, got %q", i, e, ArtifactKinds[i])
		}
	}
}

func TestDiffableKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour",
		"technology", "bundle", "folder", "file", "section", "definition",
		"goal", "ticket", "contributor", "checkpoint", "interaction", "session",
	}
	if len(DiffableKinds) != len(expected) {
		t.Fatalf("DiffableKinds length: expected %d, got %d", len(expected), len(DiffableKinds))
	}
	for i, e := range expected {
		if DiffableKinds[i] != e {
			t.Errorf("DiffableKinds[%d]: expected %q, got %q", i, e, DiffableKinds[i])
		}
	}
}

func TestRelatedToFileKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(RelatedToFileKinds) != len(expected) {
		t.Fatalf("RelatedToFileKinds length: expected %d, got %d", len(expected), len(RelatedToFileKinds))
	}
	for i, e := range expected {
		if RelatedToFileKinds[i] != e {
			t.Errorf("RelatedToFileKinds[%d]: expected %q, got %q", i, e, RelatedToFileKinds[i])
		}
	}
}

// 🌳️#region 🗿️Monorepo Tree
func TestTreeNodeKindConstants(t *testing.T) {
	t.Run("all kinds are distinct", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeTechnology, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
			TreeNodeContributor, TreeNodeCheckpoint, TreeNodeCategory,
		}
		seen := make(map[TreeNodeKind]bool)
		for _, k := range kinds {
			if seen[k] {
				t.Errorf("duplicate TreeNodeKind: %s", k)
			}
			seen[k] = true
		}
	})

	t.Run("kinds are non-empty strings", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeTechnology, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
			TreeNodeContributor, TreeNodeCheckpoint, TreeNodeCategory,
		}
		for _, k := range kinds {
			if string(k) == "" {
				t.Error("TreeNodeKind should not be empty")
			}
		}
	})
}
