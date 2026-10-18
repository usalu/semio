// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Tree node kind constants, filter predicates, and collapse/sort.

// #endregion 🧲️Header

package client

import (
	command "github.com/usalu/semio/repo/client/internal/command"
	"testing"
)

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
		sortTreeChildren(tree)
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
		sortTreeChildren(tree)
		if tree.Children[0].Kind != TreeNodeFolder {
			t.Errorf("expected folder first, got %s", tree.Children[0].Kind)
		}
	})
}

func TestTreeCommandFlags(t *testing.T) {
	t.Run("builds filter from flags", func(t *testing.T) {
		cmd := &command.Command{}
		bindTreeFlags(cmd)
		cmd.Flags().Set("only-technology", "true")
		cmd.Flags().Set("no-folder", "true")
		cmd.Flags().Set("only-library", "true")
		cmd.Flags().Set("only-open", "true")
		cmd.Flags().Set("no-year", "2025")

		filter := buildTreeFilterFromFlags(cmd)

		if !filter.OnlyKinds[TreeNodeTechnology] {
			t.Error("expected only-technology to be set")
		}
		if !filter.ExcludeKinds[TreeNodeFolder] {
			t.Error("expected no-folder to be set")
		}
		if len(filter.OnlySubKinds[TreeNodeBundle]) != 1 || filter.OnlySubKinds[TreeNodeBundle][0] != string(BundleKindLibrary) {
			t.Error("expected only-library sub-kind")
		}
		if filter.OnlyStatus != "open" {
			t.Errorf("expected only-open status, got %q", filter.OnlyStatus)
		}
		if len(filter.ExcludeYears) != 1 || filter.ExcludeYears[0] != 2025 {
			t.Errorf("expected no-year 2025, got %v", filter.ExcludeYears)
		}
	})

	t.Run("empty flags produce empty filter", func(t *testing.T) {
		cmd := &command.Command{}
		bindTreeFlags(cmd)
		filter := buildTreeFilterFromFlags(cmd)
		if filter.HasOnlyKinds() {
			t.Error("empty flags should not set only-kinds")
		}
		if len(filter.ExcludeKinds) != 0 {
			t.Error("empty flags should not set exclude-kinds")
		}
		if filter.OnlyStatus != "" {
			t.Error("empty flags should not set status")
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
			got := treeNodeKindToEntityKind(tt.kind)
			if got != tt.expected {
				t.Errorf("treeNodeKindToEntityKind(%q) = %q, want %q", tt.kind, got, tt.expected)
			}
		})
	}
	t.Run("unknown returns empty", func(t *testing.T) {
		got := treeNodeKindToEntityKind(TreeNodeKind("unknown"))
		if got != "" {
			t.Errorf("treeNodeKindToEntityKind(unknown) = %q, want empty", got)
		}
	})
}
