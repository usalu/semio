// 🔬️ Tests of the tree domain, split out of the pre-split godfile suite.

package tree

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
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

func TestTreeFilterIsKindVisible(t *testing.T) {
	t.Run("all visible by default", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible by default")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should be visible by default")
		}
	})

	t.Run("only-kind filters to specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeTechnology: true, TreeNodeBundle: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeTechnology) {
			t.Error("technology should be visible with only-technology")
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible with only-bundle")
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when not in only-kinds")
		}
		if f.IsKindVisible(TreeNodeFile) {
			t.Error("file should not be visible when not in only-kinds")
		}
	})

	t.Run("exclude-kind hides specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: map[TreeNodeKind]bool{TreeNodeFolder: true},
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when excluded")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should still be visible")
		}
	})

	t.Run("category always visible", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeTechnology: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeCategory) {
			t.Error("category should always be visible")
		}
	})
}

func TestTreeFilterMatchesSubKind(t *testing.T) {
	t.Run("matches all when no sub-kind filters", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match any sub-kind by default")
		}
	})

	t.Run("only sub-kind includes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("library should match only-library")
		}
		if f.MatchesSubKind(TreeNodeBundle, "schema") {
			t.Error("schema should not match only-library")
		}
	})

	t.Run("exclude sub-kind removes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: map[TreeNodeKind][]string{TreeNodeFolder: {"required"}},
		}
		if f.MatchesSubKind(TreeNodeFolder, "required") {
			t.Error("required should not match when excluded")
		}
		if !f.MatchesSubKind(TreeNodeFolder, "organization") {
			t.Error("organization should still match")
		}
	})

	t.Run("empty sub-kind always matches", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "") {
			t.Error("empty sub-kind should always match")
		}
	})

	t.Run("case insensitive matching", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"Library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesDate(t *testing.T) {
	t.Run("matches all when no date filters", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesDate(2026, 1, 15) {
			t.Error("should match any date by default")
		}
	})

	t.Run("only-year includes specified year", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}}
		if !f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should match only-year 2026")
		}
		if f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should not match only-year 2026")
		}
	})

	t.Run("exclude-year removes specified year", func(t *testing.T) {
		f := &TreeFilter{ExcludeYears: []int{2026}}
		if f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should not match no-year 2026")
		}
		if !f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should still match")
		}
	})

	t.Run("month filter", func(t *testing.T) {
		f := &TreeFilter{OnlyMonths: []int{6}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("June should match")
		}
		if f.MatchesDate(2026, 7, 1) {
			t.Error("July should not match")
		}
	})

	t.Run("combined year and month", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}, ExcludeMonths: []int{12}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("2026/06 should match")
		}
		if f.MatchesDate(2026, 12, 1) {
			t.Error("2026/12 should not match")
		}
		if f.MatchesDate(2025, 6, 1) {
			t.Error("2025 should not match")
		}
	})
}

func TestTreeFilterMatchesStatus(t *testing.T) {
	t.Run("matches all when no status filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesStatus("open") {
			t.Error("should match any status by default")
		}
		if !f.MatchesStatus("closed") {
			t.Error("should match any status by default")
		}
	})

	t.Run("only-open filters to open", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "open"}
		if !f.MatchesStatus("open") {
			t.Error("open should match only-open")
		}
		if f.MatchesStatus("closed") {
			t.Error("closed should not match only-open")
		}
	})

	t.Run("only-closed filters to closed", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "closed"}
		if !f.MatchesStatus("closed") {
			t.Error("closed should match only-closed")
		}
		if f.MatchesStatus("open") {
			t.Error("open should not match only-closed")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "Open"}
		if !f.MatchesStatus("open") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesContributor(t *testing.T) {
	t.Run("matches all when no contributor filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesContributor("usalu") {
			t.Error("should match any contributor by default")
		}
	})

	t.Run("only-contributor includes specified", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("usalu should match")
		}
		if f.MatchesContributor("other") {
			t.Error("other should not match")
		}
	})

	t.Run("exclude-contributor removes specified", func(t *testing.T) {
		f := &TreeFilter{ExcludeContributors: []string{"usalu"}}
		if f.MatchesContributor("usalu") {
			t.Error("usalu should not match when excluded")
		}
		if !f.MatchesContributor("other") {
			t.Error("other should still match")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"Usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestFilterMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj1", Label: "proj1", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "b1", Label: "bundle1", SubKind: "library", Children: []*TreeNode{
							{Kind: TreeNodeFolder, ID: "f1", Label: "src", SubKind: "organization", Children: []*TreeNode{
								{Kind: TreeNodeFile, ID: "file1", Label: "index.ts", SubKind: "code"},
								{Kind: TreeNodeFile, ID: "file2", Label: "README.md", SubKind: "docs"},
							}},
						}},
						{Kind: TreeNodeBundle, ID: "b2", Label: "bundle2", SubKind: "schema"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "g1", Label: "Goal1", Status: "open", Children: []*TreeNode{
						{Kind: TreeNodeTicket, ID: "t1", Label: "Ticket1", Status: "open", Year: 2026, Month: 2, Day: 5},
						{Kind: TreeNodeTicket, ID: "t2", Label: "Ticket2", Status: "closed", Year: 2025, Month: 12, Day: 1},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "contributors", Label: "Contributors", Children: []*TreeNode{
					{Kind: TreeNodeContributor, ID: "c1", Label: "usalu", Contributor: "usalu"},
					{Kind: TreeNodeContributor, ID: "c2", Label: "other", Contributor: "other"},
				}},
			},
		}
	}

	t.Run("no filter returns full tree", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		if len(result.Children) != 3 {
			t.Errorf("expected 3 top-level categories, got %d", len(result.Children))
		}
	})

	t.Run("exclude-bundle removes bundles", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle {
				t.Error("bundles should be collapsed out")
			}
		}
	})

	t.Run("no-folder collapses folders", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		bundle := proj.Children[0]
		hasFile := false
		for _, c := range bundle.Children {
			if c.Kind == TreeNodeFolder {
				t.Error("folders should be collapsed")
			}
			if c.Kind == TreeNodeFile {
				hasFile = true
			}
		}
		if !hasFile {
			t.Error("files should be promoted to bundle level")
		}
	})

	t.Run("only-library sub-kind filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle && c.SubKind != "library" {
				t.Errorf("only library bundles expected, got %s", c.SubKind)
			}
		}
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 bundle (library), got %d", len(proj.Children))
		}
	})

	t.Run("status filter open", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			OnlyStatus:      "open",
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Status != "open" {
				t.Error("only open tickets should be visible")
			}
		}
	})

	t.Run("year filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			ExcludeYears:    []int{2025},
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Year == 2025 {
				t.Error("2025 tickets should be excluded")
			}
		}
	})

	t.Run("contributor filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:           make(map[TreeNodeKind]bool),
			ExcludeKinds:        make(map[TreeNodeKind]bool),
			OnlySubKinds:        make(map[TreeNodeKind][]string),
			ExcludeSubKinds:     make(map[TreeNodeKind][]string),
			ExcludeContributors: []string{"usalu"},
		}
		result := FilterMonorepoTree(tree, filter)
		contribNode := result.Children[2]
		for _, c := range contribNode.Children {
			if c.Contributor == "usalu" {
				t.Error("usalu should be excluded")
			}
		}
		if len(contribNode.Children) != 1 {
			t.Errorf("expected 1 contributor, got %d", len(contribNode.Children))
		}
	})

	t.Run("nil filter returns same tree", func(t *testing.T) {
		tree := makeTree()
		result := FilterMonorepoTree(tree, nil)
		if result != tree {
			t.Error("nil filter should return same tree")
		}
	})
}

func TestSearchMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj:compose", Label: "compose", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "bundle:cli", Label: "cli", SubKind: "binary"},
						{Kind: TreeNodeBundle, ID: "bundle:docs", Label: "docs", SubKind: "site"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "goal:test", Label: "Test Goal", Description: "testing search"},
				}},
			},
		}
	}

	t.Run("empty query returns full tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "")
		if len(result.Children) != 2 {
			t.Errorf("expected 2 categories, got %d", len(result.Children))
		}
	})

	t.Run("query matches items", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		found := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.ID == "bundle:cli" {
				found = true
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(result)
		if !found {
			t.Error("search for 'cli' should find bundle:cli")
		}
	})

	t.Run("query with no matches returns empty tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "zzzznonexistent")
		totalChildren := 0
		for _, c := range result.Children {
			totalChildren += len(c.Children)
		}
		if totalChildren != 0 {
			t.Errorf("search for nonexistent term should return empty, got %d children", totalChildren)
		}
	})

	t.Run("parent chain preserved", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		if len(result.Children) == 0 {
			t.Fatal("expected at least one category")
		}
		technologiesNode := result.Children[0]
		if technologiesNode.ID != "codebase" {
			t.Errorf("expected codebase category, got %s", technologiesNode.ID)
		}
		if len(technologiesNode.Children) == 0 {
			t.Fatal("expected technology under technologies")
		}
		proj := technologiesNode.Children[0]
		if proj.ID != "proj:compose" {
			t.Errorf("expected compose technology, got %s", proj.ID)
		}
	})
}

func TestSearchMonorepoTreeWithCache(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow search monorepo tree cache test in short mode")
	}
	tmpDir := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo"), 0755); err != nil {
		t.Fatalf("mkdir .🦑️repo: %v", err)
	}
	oldRoot := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(oldRoot)

	makeTree := func(description string) *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj:repo", Label: "repo", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "bundle:search", Label: "search-bundle", Description: description},
					}},
				}},
			},
		}
	}

	t.Run("uses indexed matches from cache", func(t *testing.T) {
		indexedTree := makeTree("ultrafastneedle")
		idx, err := ensureCacheIndexed(context.Background(), indexedTree, nil)
		if err != nil {
			t.Fatalf("ensureCacheIndexed failed: %v", err)
		}
		if err := idx.Close(); err != nil {
			t.Fatalf("close index failed: %v", err)
		}

		mutatedTree := makeTree("different-text")
		result, err := SearchMonorepoTreeWithCache(context.Background(), mutatedTree, "ultrafastneedle")
		if err != nil {
			t.Fatal(err)
		}

		found := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			if node.ID == "bundle:search" {
				found = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(result)
		if !found {
			t.Fatal("expected cached index to find bundle:search")
		}
	})

	t.Run("returns empty tree for miss", func(t *testing.T) {
		result, err := SearchMonorepoTreeWithCache(context.Background(), makeTree("something else"), "no-match-here")
		if err != nil {
			t.Fatal(err)
		}
		if len(result.Children) != 0 {
			t.Fatalf("expected empty tree, got %d children", len(result.Children))
		}
	})
}

func TestRenderMonorepoTree(t *testing.T) {
	t.Run("renders basic tree", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "🖥️Codebase") {
			t.Error("output should contain Codebase label")
		}
		if !strings.Contains(output, "compose") {
			t.Error("output should contain technology name")
		}
	})

	t.Run("renders category URI", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals"},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "[🎯️Goals](repo://goals)") {
			t.Errorf("output should contain category with URI link, got: %s", output)
		}
	})

	t.Run("renders nested tree with connectors", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "proj1"},
					{Kind: TreeNodeTechnology, ID: "p2", Label: "proj2"},
				}},
			},
		}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "├️─️─️ ") || !strings.Contains(output, "└️─️─️ ") {
			t.Errorf("output should contain tree connectors, got: %s", output)
		}
	})

	t.Run("empty tree renders nothing", func(t *testing.T) {
		tree := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		if output != "" {
			t.Errorf("empty tree should render nothing, got: %q", output)
		}
	})

	t.Run("markdown renderer uses list bullets", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTreeMarkdown(tree, DefaultEntityRenderer{})
		if !strings.Contains(output, "- [🖥️Codebase](repo://codebase)") {
			t.Errorf("markdown tree should contain markdown link list item, got: %s", output)
		}
		if !strings.Contains(output, "  - compose") {
			t.Errorf("markdown tree should contain nested bullet item, got: %s", output)
		}
		if strings.Contains(output, "├️─️─️ ") || strings.Contains(output, "└️─️─️ ") {
			t.Errorf("markdown tree must not contain ascii connectors, got: %s", output)
		}
	})

	t.Run("text tree shows only own ID segment not full parent chain", func(t *testing.T) {
		parentGoalData := map[string]interface{}{
			"id":     "parentgoal",
			"title":  "Parent Goal",
			"status": "open",
		}
		childGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal",
			"title":    "Child Goal",
			"status":   "open",
			"parentId": "🎯️parentgoal",
		}
		grandchildGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal/grandchildgoal",
			"title":    "Grandchild Goal",
			"status":   "open",
			"parentId": "🎯️parentgoal🎯️childgoal",
		}
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "parentgoal", Label: "Parent Goal", Data: parentGoalData, Children: []*TreeNode{
						{Kind: TreeNodeGoal, ID: "childgoal", Label: "Child Goal", Data: childGoalData, Children: []*TreeNode{
							{Kind: TreeNodeGoal, ID: "grandchildgoal", Label: "Grandchild Goal", Data: grandchildGoalData},
						}},
					}},
				}},
			},
		}
		goalEmoji := model.EmojiText(model.EmojiGoal)
		output := RenderMonorepoTree(tree, DefaultEntityRenderer{})
		lines := strings.Split(strings.TrimRight(output, "\n"), "\n")
		for _, line := range lines {
			if strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji+"childgoal") {
				t.Errorf("tree text should not contain full hierarchical ID, got line: %s", line)
			}
			if strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji) {
				t.Errorf("tree text should not contain parent prefix in child line, got line: %s", line)
			}
		}
		childFound := false
		grandchildFound := false
		for _, line := range lines {
			if strings.Contains(line, goalEmoji+"childgoal") && !strings.Contains(line, goalEmoji+"parentgoal"+goalEmoji+"childgoal") {
				childFound = true
			}
			if strings.Contains(line, goalEmoji+"grandchildgoal") && !strings.Contains(line, goalEmoji+"childgoal"+goalEmoji+"grandchildgoal") {
				grandchildFound = true
			}
		}
		if !childFound {
			t.Errorf("tree text should contain short child ID 🎯️childgoal, got:\n%s", output)
		}
		if !grandchildFound {
			t.Errorf("tree text should contain short grandchild ID 🎯️grandchildgoal, got:\n%s", output)
		}
	})

	t.Run("text tree preserves parentId on data after rendering", func(t *testing.T) {
		data := map[string]interface{}{
			"id":       "parent/child",
			"title":    "Child",
			"status":   "open",
			"parentId": "🎯️parent",
		}
		node := &TreeNode{Kind: TreeNodeGoal, ID: "child", Label: "Child", Data: data}
		var sb strings.Builder
		RenderTreeNodeText(&sb, node, "", true, true, DefaultEntityRenderer{})
		if data["parentId"] != "🎯️parent" {
			t.Errorf("renderTreeNodeText should restore parentId, got: %v", data["parentId"])
		}
	})
}

func TestCollapseFilteredKinds(t *testing.T) {
	t.Run("collapses folders promoting files to parent", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
					{Kind: TreeNodeFolder, ID: "f1", Label: "src", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "file1", Label: "index.ts"},
						{Kind: TreeNodeFile, ID: "file2", Label: "app.ts"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		bundle := tree.Children[0]
		if len(bundle.Children) != 2 {
			t.Errorf("expected 2 files promoted to bundle, got %d", len(bundle.Children))
		}
		for _, c := range bundle.Children {
			if c.Kind != TreeNodeFile {
				t.Errorf("expected file, got %s", c.Kind)
			}
		}
	})

	t.Run("nested collapse", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeTechnology, ID: "p1", Label: "proj", Children: []*TreeNode{
					{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "f1", Label: "main.go"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		proj := tree.Children[0]
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 file promoted to technology, got %d", len(proj.Children))
		}
		if proj.Children[0].Kind != TreeNodeFile {
			t.Errorf("expected file, got %s", proj.Children[0].Kind)
		}
	})
}

func TestSortTreeChildren(t *testing.T) {
	t.Run("sorts alphabetically", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "z.ts"},
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFile, Label: "m.ts"},
			},
		}
		SortTreeChildren(tree)
		if tree.Children[0].Label != "a.ts" {
			t.Errorf("expected a.ts first, got %s", tree.Children[0].Label)
		}
		if tree.Children[2].Label != "z.ts" {
			t.Errorf("expected z.ts last, got %s", tree.Children[2].Label)
		}
	})

	t.Run("folders before files", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFolder, Label: "src"},
				{Kind: TreeNodeFile, Label: "b.ts"},
			},
		}
		SortTreeChildren(tree)
		if tree.Children[0].Kind != TreeNodeFolder {
			t.Errorf("expected folder first, got %s", tree.Children[0].Kind)
		}
	})
}

// 🌳️#region 📋️Unified Rendering Identity
func TestTreeNodeKindToEntityKindCoversAll(t *testing.T) {
	kinds := []struct {
		kind     TreeNodeKind
		expected string
	}{
		{TreeNodeTechnology, "technology"},
		{TreeNodeBundle, "bundle"},
		{TreeNodeFolder, "folder"},
		{TreeNodeFile, "file"},
		{TreeNodeSection, "section"},
		{TreeNodeDefinition, "definition"},
		{TreeNodeGoal, "goal"},
		{TreeNodeTicket, "ticket"},
		{TreeNodeDraft, "draft"},
		{TreeNodePolicy, "policy"},
		{TreeNodeStatute, ""},
		{TreeNodeContributor, "contributor"},
		{TreeNodeCheckpoint, "checkpoint"},
		{TreeNodeCategory, ""},
	}
	for _, tt := range kinds {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := TreeNodeKindToEntityKind(tt.kind)
			if got != tt.expected {
				t.Errorf("treeNodeKindToEntityKind(%q) = %q, want %q", tt.kind, got, tt.expected)
			}
		})
	}
	t.Run("unknown returns empty", func(t *testing.T) {
		got := TreeNodeKindToEntityKind(TreeNodeKind("unknown"))
		if got != "" {
			t.Errorf("treeNodeKindToEntityKind(unknown) = %q, want empty", got)
		}
	})
}

func TestUnifiedRenderingGoalIdentity(t *testing.T) {
	data := map[string]interface{}{
		"id":          "TEST-GOAL",
		"title":       "Test Goal",
		"status":      "open",
		"dueDate":     "2030-01-01",
		"createdAt":   "2025-01-01T00:00:00Z",
		"description": "A test goal",
	}

	mdLink := model.RenderEntityMarkdownLink("goal", data)
	mdItem := model.RenderEntityMarkdown("goal", data)
	humanItem := model.RenderEntityHuman("goal", data, false)

	t.Run("renderEntityMarkdown is dash-prefixed renderEntityMarkdownLink", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("markdown link has artifact ID and URI", func(t *testing.T) {
		if !strings.Contains(mdLink, "["+model.EmojiText(model.EmojiGoal)) {
			t.Errorf("markdown link missing goal emoji prefix: %s", mdLink)
		}
		if !strings.Contains(mdLink, "](repo://goal/") {
			t.Errorf("markdown link missing goal URI: %s", mdLink)
		}
	})

	t.Run("human has artifact ID", func(t *testing.T) {
		if !strings.Contains(humanItem, model.EmojiText(model.EmojiGoal)) {
			t.Errorf("human output missing goal emoji: %s", humanItem)
		}
	})

	t.Run("both formats share same props from collectEntityProps", func(t *testing.T) {
		props := model.CollectEntityProps("goal", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("goalNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &model.GoalNode{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}
		nodeData := goalNodeToData(node)
		fromNode := model.RenderEntityMarkdownLink("goal", nodeData)
		fromDirect := model.RenderEntityMarkdownLink("goal", data)
		if fromNode != fromDirect {
			t.Errorf("goalNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree markdown uses renderEntityMarkdownLink for content", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := RenderModelGoalTreeNodes(roots, "md")
		expectedLink := model.RenderEntityMarkdownLink("goal", data)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("goal tree markdown must not have double dash: %q", treeOutput)
		}
	})

	t.Run("goal tree text uses renderEntityHuman for content", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := RenderModelGoalTreeNodes(roots, "text")
		expectedHuman := model.RenderEntityHuman("goal", data, false)
		if !strings.Contains(treeOutput, expectedHuman) {
			t.Errorf("goal tree text should contain renderEntityHuman output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("monorepo tree node markdown matches goal tree markdown", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + model.EmojiText(model.EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		RenderTreeNodeMarkdown(&sb, treeNode, "", DefaultEntityRenderer{})
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*model.GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(RenderModelGoalTreeNodes(roots, "md"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree markdown and goal tree markdown differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})

	t.Run("monorepo tree node text matches goal tree text", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + model.EmojiText(model.EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		RenderTreeNodeText(&sb, treeNode, "", true, true, DefaultEntityRenderer{})
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*model.GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(RenderModelGoalTreeNodes(roots, "text"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree text and goal tree text differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})
}

func TestUnifiedRenderingTicketIdentity(t *testing.T) {
	data := map[string]interface{}{
		"slug":     "MY-TICKET",
		"title":    "My Ticket",
		"status":   "open",
		"started":  "2025-01-01T00:00:00Z",
		"finished": "",
		"prompt":   "Fix something",
		"summary":  "",
		"year":     float64(2025),
		"month":    float64(1),
		"day":      float64(1),
	}

	mdLink := model.RenderEntityMarkdownLink("ticket", data)
	mdItem := model.RenderEntityMarkdown("ticket", data)
	humanItem := model.RenderEntityHuman("ticket", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := model.CollectEntityProps("ticket", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("ticketNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &model.TicketNode{
			Slug:        "MY-TICKET",
			Title:       "My Ticket",
			Status:      "open",
			Created:     "2025-01-01T00:00:00Z",
			Finished:    "",
			Description: "Fix something",
			Summary:     "",
		}
		nodeData := ticketNodeToData(node)
		nodeData["year"] = float64(2025)
		nodeData["month"] = float64(1)
		nodeData["day"] = float64(1)
		fromNode := model.RenderEntityMarkdownLink("ticket", nodeData)
		fromDirect := model.RenderEntityMarkdownLink("ticket", data)
		if fromNode != fromDirect {
			t.Errorf("ticketNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree ticket markdown uses renderEntityMarkdownLink", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID: "G1", Title: "Parent", Status: "open",
			Tickets: []*model.TicketNode{{
				Slug: "MY-TICKET", Title: "My Ticket", Status: "open",
				Created: "2025-01-01T00:00:00Z", Description: "Fix something",
			}},
		}}
		treeOutput := RenderModelGoalTreeNodes(roots, "md")
		ticketData := ticketNodeToData(roots[0].Tickets[0])
		expectedLink := model.RenderEntityMarkdownLink("ticket", ticketData)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree ticket markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("ticket in goal tree must not have double dash: %q", treeOutput)
		}
	})

	t.Run("ticket list markdown matches renderEntityMarkdown", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(RenderTicketList(tickets, false, true))
		directMD := strings.TrimSpace(model.RenderEntityMarkdown("ticket", data))
		if listOutput != directMD {
			t.Errorf("ticket list markdown should match renderEntityMarkdown.\n  List:   %q\n  Direct: %q", listOutput, directMD)
		}
	})

	t.Run("ticket list text matches renderEntityHuman", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(RenderTicketList(tickets, false, false))
		directHuman := model.RenderEntityHuman("ticket", data, false)
		if !strings.Contains(listOutput, directHuman) {
			t.Errorf("ticket list text should contain renderEntityHuman output.\n  List:   %q\n  Direct: %q", listOutput, directHuman)
		}
	})
}

func TestUnifiedRenderingSectionIdentity(t *testing.T) {
	data := map[string]interface{}{
		"path":      "test/file.ts#MySection",
		"name":      "MySection",
		"startLine": float64(10),
		"endLine":   float64(20),
	}

	mdLink := model.RenderEntityMarkdownLink("section", data)
	mdItem := model.RenderEntityMarkdown("section", data)
	humanItem := model.RenderEntityHuman("section", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := model.CollectEntityProps("section", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("section tree markdown uses renderEntityMarkdown", func(t *testing.T) {
		s := &model.Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(RenderSectionTree(s, false, true))
		expectedMD := strings.TrimSpace(model.RenderEntityMarkdown("section", data))
		if treeOutput != expectedMD {
			t.Errorf("section tree markdown root should match renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedMD)
		}
	})

	t.Run("section tree text uses renderEntityHuman", func(t *testing.T) {
		s := &model.Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(RenderSectionTree(s, false, false))
		expectedHuman := model.RenderEntityHuman("section", data, false)
		if treeOutput != expectedHuman {
			t.Errorf("section tree text root should match renderEntityHuman.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("section tree markdown preserves indentation for children", func(t *testing.T) {
		s := &model.Section{
			Path:      "test/file.ts#Parent",
			Name:      "Parent",
			StartLine: 1,
			EndLine:   30,
			Children: []model.Section{{
				Path:      "test/file.ts#Child",
				Name:      "Child",
				StartLine: 5,
				EndLine:   15,
			}},
		}
		treeOutput := RenderSectionTree(s, false, true)
		lines := strings.Split(strings.TrimSpace(treeOutput), "\n")
		if len(lines) < 2 {
			t.Fatalf("expected at least 2 lines, got %d: %q", len(lines), treeOutput)
		}
		if !strings.HasPrefix(lines[0], "- [") {
			t.Errorf("root section should start with '- [': %q", lines[0])
		}
		if !strings.HasPrefix(lines[1], "  - [") {
			t.Errorf("child section should start with '  - [' for 2-space indent: %q", lines[1])
		}
	})

	t.Run("monorepo tree node markdown matches direct rendering", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeSection,
			ID:    "sec1",
			Label: "MySection",
			URI:   "repo://section/" + model.EmojiText(model.EmojiFileCode) + "file" + model.EmojiText(model.EmojiSection) + "mysection",
			Data:  data,
		}
		var sb strings.Builder
		RenderTreeNodeMarkdown(&sb, treeNode, "", DefaultEntityRenderer{})
		monorepoOutput := strings.TrimSpace(sb.String())
		directMD := strings.TrimSpace(model.RenderEntityMarkdown("section", data))
		if monorepoOutput != directMD {
			t.Errorf("monorepo tree section markdown should match renderEntityMarkdown.\n  Monorepo: %q\n  Direct:   %q", monorepoOutput, directMD)
		}
	})
}

func TestUnifiedRenderingAllKindIdentity(t *testing.T) {
	entities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"technology", TreeNodeTechnology, map[string]interface{}{
			"name": "mytechnology", "description": "A technology",
		}},
		{"bundle", TreeNodeBundle, map[string]interface{}{
			"name": "mybundle", "root": "path/to/bundle",
		}},
		{"folder", TreeNodeFolder, map[string]interface{}{
			"path": "src/folder", "name": "folder",
		}},
		{"file", TreeNodeFile, map[string]interface{}{
			"path": "src/file.ts", "name": "file.ts",
		}},
		{"contributor", TreeNodeContributor, map[string]interface{}{
			"github": "dev1", "name": "Developer One",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "code-hygiene", "name": "Code Hygiene", "description": "Clean code policy",
		}},
		{"statute", TreeNodeStatute, map[string]interface{}{
			"id": "inline-comment", "description": "No inline comments",
		}},
		{"draft", TreeNodeDraft, map[string]interface{}{
			"id": "draft-1", "slug": "my-draft",
		}},
		{"checkpoint", TreeNodeCheckpoint, map[string]interface{}{
			"sha": "abc1234567890", "message": "fix: something",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind+"_markdown_identity", func(t *testing.T) {
			directMD := model.RenderEntityMarkdown(tt.kind, tt.data)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			RenderTreeNodeMarkdown(&sb, treeNode, "", DefaultEntityRenderer{})
			treeOutput := strings.TrimSpace(sb.String())
			directMDTrimmed := strings.TrimSpace(directMD)
			if treeOutput != directMDTrimmed {
				t.Errorf("%s: monorepo tree markdown differs from direct renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directMDTrimmed)
			}
		})

		t.Run(tt.kind+"_text_identity", func(t *testing.T) {
			directHuman := model.RenderEntityHuman(tt.kind, tt.data, false)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			RenderTreeNodeText(&sb, treeNode, "", true, true, DefaultEntityRenderer{})
			treeOutput := strings.TrimSpace(sb.String())
			if treeOutput != directHuman {
				t.Errorf("%s: monorepo tree text differs from direct renderEntityHuman.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directHuman)
			}
		})

		t.Run(tt.kind+"_props_in_both_formats", func(t *testing.T) {
			props := model.CollectEntityProps(tt.kind, tt.data, false)
			mdLink := model.RenderEntityMarkdownLink(tt.kind, tt.data)
			human := model.RenderEntityHuman(tt.kind, tt.data, false)
			for _, p := range props {
				if !strings.Contains(mdLink, p) {
					t.Errorf("%s: markdown link missing prop %q: %s", tt.kind, p, mdLink)
				}
				if !strings.Contains(human, p) {
					t.Errorf("%s: human output missing prop %q: %s", tt.kind, p, human)
				}
			}
		})
	}
}

func TestNoDoubleDashInMarkdownOutput(t *testing.T) {
	kinds := []struct {
		kind string
		data map[string]interface{}
	}{
		{"goal", map[string]interface{}{
			"id": "G1", "title": "Goal", "status": "open",
		}},
		{"ticket", map[string]interface{}{
			"slug": "T1", "title": "Ticket", "status": "open",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"section", map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(1), "endLine": float64(5),
		}},
		{"bundle", map[string]interface{}{
			"name": "b1", "root": "path",
		}},
		{"folder", map[string]interface{}{
			"path": "src/f",
		}},
		{"file", map[string]interface{}{
			"path": "src/a.ts",
		}},
		{"contributor", map[string]interface{}{
			"github": "dev",
		}},
		{"checkpoint", map[string]interface{}{
			"sha": "abc",
		}},
	}

	for _, tt := range kinds {
		t.Run(tt.kind+"_renderEntityMarkdown", func(t *testing.T) {
			output := model.RenderEntityMarkdown(tt.kind, tt.data)
			if count := strings.Count(output, "- "); count > 1 {
				dashPositions := []int{}
				idx := 0
				for {
					pos := strings.Index(output[idx:], "- ")
					if pos == -1 {
						break
					}
					dashPositions = append(dashPositions, idx+pos)
					idx += pos + 2
				}
				if len(dashPositions) >= 2 && dashPositions[1]-dashPositions[0] <= 3 {
					t.Errorf("renderEntityMarkdown(%s) has double dash at start: %q", tt.kind, output)
				}
			}
		})

		t.Run(tt.kind+"_treeNodeMarkdown", func(t *testing.T) {
			nodeKind := TreeNodeKind(tt.kind)
			switch tt.kind {
			case "goal":
				nodeKind = TreeNodeGoal
			case "ticket":
				nodeKind = TreeNodeTicket
			case "section":
				nodeKind = TreeNodeSection
			case "bundle":
				nodeKind = TreeNodeBundle
			case "folder":
				nodeKind = TreeNodeFolder
			case "file":
				nodeKind = TreeNodeFile
			case "contributor":
				nodeKind = TreeNodeContributor
			case "checkpoint":
				nodeKind = TreeNodeCheckpoint
			}

			treeNode := &TreeNode{Kind: nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			RenderTreeNodeMarkdown(&sb, treeNode, "", DefaultEntityRenderer{})
			output := sb.String()
			if strings.HasPrefix(output, "- - ") {
				t.Errorf("renderTreeNodeMarkdown(%s) has double dash: %q", tt.kind, output)
			}
		})
	}

	t.Run("goalTreeNodes_no_double_dash", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID: "G1", Title: "Goal", Status: "open",
			Tickets: []*model.TicketNode{{
				Slug: "T1", Title: "Ticket", Status: "open",
			}},
		}}
		output := RenderModelGoalTreeNodes(roots, "md")
		for i, line := range strings.Split(output, "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if strings.HasPrefix(trimmed, "- - ") {
				t.Errorf("line %d has double dash: %q", i, line)
			}
		}
	})
}

func TestGetCacheDirUsesMonorepoRoot(t *testing.T) {
	monoRoot := t.TempDir()
	clientDir := filepath.Join(monoRoot, "repo", "cli")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	oldRoot := workspace.RootDir
	defer func() { workspace.SetRootDir(oldRoot) }()
	workspace.SetRootDir(clientDir)
	cacheDir := getCacheDir()
	wantPrefix := filepath.Join(monoRoot, ".🧬semio", "🦑️repo", "⚡️cache") + string(os.PathSeparator)
	if !strings.HasPrefix(cacheDir, wantPrefix) {
		t.Fatalf("expected cache dir under %q, got %q", wantPrefix, cacheDir)
	}
}

func TestMermaidEscapeLabel(t *testing.T) {
	if got := mermaidEscapeLabel("hello \"world\""); got != "hello 'world'" {
		t.Errorf("expected hello 'world', got: %s", got)
	}
	if got := mermaidEscapeLabel("no quotes"); got != "no quotes" {
		t.Errorf("expected no quotes, got: %s", got)
	}
}

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

func TestComputeCompositeFingerprintIgnoresUntrackedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	firstFingerprint, firstMeta := computeCompositeFingerprint(tmpDir)
	if firstFingerprint == "" || firstMeta == nil {
		t.Fatalf("expected first fingerprint and meta to be set, got %q / %+v", firstFingerprint, firstMeta)
	}

	if err := os.WriteFile(filepath.Join(tmpDir, "untracked.txt"), []byte("untracked"), 0644); err != nil {
		t.Fatalf("failed to create untracked file: %v", err)
	}

	secondFingerprint, secondMeta := computeCompositeFingerprint(tmpDir)
	if secondFingerprint == "" || secondMeta == nil {
		t.Fatalf("expected second fingerprint and meta to be set, got %q / %+v", secondFingerprint, secondMeta)
	}
	if firstFingerprint != secondFingerprint {
		t.Fatalf("expected untracked files to be ignored, fingerprint changed from %q to %q", firstFingerprint, secondFingerprint)
	}
}

func TestComputeCompositeFingerprintFallsBackWhenRecursiveSubmoduleStatusFails(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess submodule fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	childDir := initTestGitRepo(t, "main")
	nestedDir := initTestGitRepo(t, "main")
	run := func(dir string, args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = dir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v in %s failed: %s\n%s", args, dir, err, string(out))
		}
	}

	run(tmpDir, "-c", "protocol.file.allow=always", "submodule", "add", childDir, "child")
	run(tmpDir, "commit", "-am", "add child submodule")
	childWorkTree := filepath.Join(tmpDir, "child")
	run(childWorkTree, "config", "user.email", "test@test.com")
	run(childWorkTree, "config", "user.name", "Test")
	run(childWorkTree, "-c", "protocol.file.allow=always", "submodule", "add", nestedDir, "nested")
	run(childWorkTree, "commit", "-am", "add nested submodule")

	nestedGitDir := filepath.Join(tmpDir, ".git", "modules", "child", "modules", "nested")
	if err := os.RemoveAll(nestedGitDir); err != nil {
		t.Fatalf("failed to remove nested gitdir: %v", err)
	}

	_, meta := computeCompositeFingerprint(tmpDir)
	if _, ok := meta.SubmodulePointers["child"]; !ok {
		t.Fatalf("expected fallback submodule status to include child pointer, got %+v", meta.SubmodulePointers)
	}
	if _, ok := meta.SubmodulePointers["child/nested"]; ok {
		t.Fatalf("expected broken nested submodule to be excluded after fallback, got %+v", meta.SubmodulePointers)
	}
}

type g1Fixture struct {
	Schema  string `json:"schema"`
	Command struct {
		InvalidArgs   []string `json:"invalidArgs"`
		ErrorContains string   `json:"errorContains"`
		HelpArgs      []string `json:"helpArgs"`
		HelpContains  string   `json:"helpContains"`
		DispatchArgs  []string `json:"dispatchArgs"`
	} `json:"command"`
	Glob struct {
		Pattern string   `json:"pattern"`
		Paths   []string `json:"paths"`
		Matches []string `json:"matches"`
	} `json:"glob"`
	Template struct {
		Invalid string `json:"invalid"`
		Error   bool   `json:"error"`
	} `json:"template"`
	Search struct {
		Query                  string            `json:"query"`
		Documents              map[string]string `json:"documents"`
		Matches                []string          `json:"matches"`
		ReindexInterruptPhases []string          `json:"reindexInterruptPhases"`
		MaxDocumentBytes       int               `json:"maxDocumentBytes"`
	} `json:"search"`
	YAML struct {
		Source  string   `json:"source"`
		Name    string   `json:"name"`
		Paths   []string `json:"paths"`
		Enabled bool     `json:"enabled"`
	} `json:"yaml"`
}

func loadG1Fixture(t *testing.T) g1Fixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "1️⃣g1-contract", "🧫️fixtures", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture g1Fixture
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.cli.g1/1" {
		t.Fatalf("unexpected fixture schema %q", fixture.Schema)
	}
	return fixture
}

func TestG1ReindexCancellationPreservesLastValidIndex(t *testing.T) {
	fixture := loadG1Fixture(t)
	for _, phase := range fixture.Search.ReindexInterruptPhases {
		t.Run(phase, func(t *testing.T) {
			_, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
			invalidateG1SearchFingerprint(t, workspace.GetRootDir(), phase)
			ctx, cancel := context.WithCancel(context.Background())
			tree := g1SearchTree("replacement", "second")
			_, err := ensureCacheIndexed(ctx, tree, func(value search.Progress) {
				if value.Step == phase {
					cancel()
				}
			})
			if !errors.Is(err, context.Canceled) {
				t.Fatalf("%s cancellation = %v", phase, err)
			}
			assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
			assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
			if _, err := os.Stat(indexPath + ".next"); !os.IsNotExist(err) {
				t.Fatalf("%s cancellation left staged index: %v", phase, err)
			}
		})
	}
}

func TestG1ReindexMaximumPlusOnePreservesLastValidIndex(t *testing.T) {
	fixture := loadG1Fixture(t)
	if fixture.Search.MaxDocumentBytes != search.MaxDocumentBytes {
		t.Fatalf("fixture maximum = %d, implementation = %d", fixture.Search.MaxDocumentBytes, search.MaxDocumentBytes)
	}
	_, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "maximum")
	oversized := g1SearchTree(strings.Repeat("x", fixture.Search.MaxDocumentBytes+1))
	if _, err := ensureCacheIndexed(context.Background(), oversized, nil); !errors.Is(err, search.ErrTooLarge) {
		t.Fatalf("maximum + 1 reindex = %v", err)
	}
	assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
	assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
	if _, err := os.Stat(indexPath + ".next"); !os.IsNotExist(err) {
		t.Fatalf("maximum + 1 left staged index: %v", err)
	}
}

func TestG1CorruptCurrentIndexErrorPropagates(t *testing.T) {
	tree, indexPath, _, _ := prepareG1SearchCache(t)
	eventPath := filepath.Join(indexPath, "events.jsonl")
	file, err := os.OpenFile(eventPath, os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := file.WriteString("{broken\n"); err != nil {
		file.Close()
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := ensureCacheIndexed(context.Background(), tree, nil); !errors.Is(err, search.ErrCorrupt) {
		t.Fatalf("corrupt current index = %v", err)
	}
}

func TestG1ReindexLockWaitCancellation(t *testing.T) {
	tree, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "lock")
	lockPath := filepath.Join(filepath.Dir(indexPath), ".lock")
	if err := os.WriteFile(lockPath, []byte("held"), 0o644); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	_, err := ensureCacheIndexed(ctx, tree, func(value search.Progress) {
		if value.Step == "waiting-lock" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("lock wait cancellation = %v", err)
	}
	assertFileBytes(t, filepath.Join(indexPath, "events.jsonl"), beforeEvents)
	assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
}

func prepareG1SearchCache(t *testing.T) (*TreeNode, string, []byte, []byte) {
	t.Helper()
	repoRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(workspace.RepoMetaDirForRoot(repoRoot), 0o755); err != nil {
		t.Fatal(err)
	}
	previousRoot := workspace.GetRootDir()
	workspace.SetRootDir(repoRoot)
	t.Cleanup(func() { workspace.SetRootDir(previousRoot) })
	tree := g1SearchTree("last-valid", "stable")
	steps := map[string]bool{}
	indexValue, err := ensureCacheIndexed(context.Background(), tree, func(value search.Progress) { steps[value.Step] = true })
	if err != nil {
		t.Fatal(err)
	}
	if err := indexValue.Close(); err != nil {
		t.Fatal(err)
	}
	for _, step := range []string{"collecting", "indexed", "committed"} {
		if !steps[step] {
			t.Fatalf("missing reindex progress %q: %v", step, steps)
		}
	}
	indexPath := filepath.Join(getCacheDir(), "index.search")
	events, err := os.ReadFile(filepath.Join(indexPath, "events.jsonl"))
	if err != nil {
		t.Fatal(err)
	}
	meta, err := os.ReadFile(filepath.Join(indexPath, "meta.json"))
	if err != nil {
		t.Fatal(err)
	}
	return tree, indexPath, events, meta
}

func invalidateG1SearchFingerprint(t *testing.T, repoRoot, value string) {
	t.Helper()
	directory := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "✍️notes")
	if err := os.MkdirAll(directory, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(directory, value), []byte(value), 0o644); err != nil {
		t.Fatal(err)
	}
}

func g1SearchTree(descriptions ...string) *TreeNode {
	root := &TreeNode{Kind: TreeNodeCategory, Label: "."}
	for index, description := range descriptions {
		root.Children = append(root.Children, &TreeNode{
			Kind:        TreeNodeBundle,
			ID:          fmt.Sprintf("bundle:%d", index),
			Label:       fmt.Sprintf("bundle-%d", index),
			Description: description,
		})
	}
	return root
}

func assertFileBytes(t *testing.T, path string, expected []byte) {
	t.Helper()
	actual, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(actual, expected) {
		t.Fatalf("%s changed", path)
	}
}
