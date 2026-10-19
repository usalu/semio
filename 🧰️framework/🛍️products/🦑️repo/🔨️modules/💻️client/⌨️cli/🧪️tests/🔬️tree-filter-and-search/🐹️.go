// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Monorepo tree filter and search.

// #endregion 🧲️Header

package client

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

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
		t.Fatalf("mkdir repo metadata directory: %v", err)
	}
	oldRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(oldRoot)

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
		result, err := searchMonorepoTreeWithCache(context.Background(), mutatedTree, "ultrafastneedle")
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
		result, err := searchMonorepoTreeWithCache(context.Background(), makeTree("something else"), "no-match-here")
		if err != nil {
			t.Fatal(err)
		}
		if len(result.Children) != 0 {
			t.Fatalf("expected empty tree, got %d children", len(result.Children))
		}
	})
}

func TestSearchIndexIsNotReusedAcrossSectionChoice(t *testing.T) {
	tmpDir := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo"), 0755); err != nil {
		t.Fatalf("mkdir repo metadata directory: %v", err)
	}
	oldRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(oldRoot)

	withoutSections := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{{Kind: TreeNodeFile, ID: "file:main", Label: "main"}}}
	withSections := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{{Kind: TreeNodeFile, ID: "file:main", Label: "main", Children: []*TreeNode{
		{Kind: TreeNodeSection, ID: "section:models", Label: "Models"},
	}}}}

	first, err := searchMonorepoTreeWithCache(context.Background(), withoutSections, "Models")
	if err != nil {
		t.Fatalf("search without sections failed: %v", err)
	}
	if len(first.Children) != 0 {
		t.Fatalf("a tree without sections must not match a section name, got %d children", len(first.Children))
	}
	second, err := searchMonorepoTreeWithCache(context.Background(), withSections, "Models")
	if err != nil {
		t.Fatalf("search with sections failed: %v", err)
	}
	if len(second.Children) == 0 {
		t.Fatal("the index built for a tree without sections must not answer a search over a tree with sections")
	}
}
