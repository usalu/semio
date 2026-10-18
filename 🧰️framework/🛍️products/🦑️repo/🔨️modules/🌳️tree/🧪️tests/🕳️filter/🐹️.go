// 🧫️ Tests of the tree filter, filtering, collapsing and sorting.

package tree

import (
	testing "testing"
)

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
