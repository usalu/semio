// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Monorepo tree rendering and building.

// #endregion 🧲️Header

package client

import (
 "errors"
 search "github.com/usalu/semio/repo/search"
	"context"
	"encoding/json"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestRenderMonorepoTree(t *testing.T) {
	t.Run("renders basic tree", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
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
		output := RenderMonorepoTree(tree)
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
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "├️─️─️ ") || !strings.Contains(output, "└️─️─️ ") {
			t.Errorf("output should contain tree connectors, got: %s", output)
		}
	})

	t.Run("empty tree renders nothing", func(t *testing.T) {
		tree := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
		output := RenderMonorepoTree(tree)
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
		output := RenderMonorepoTreeMarkdown(tree)
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
		goalEmoji := emojiText(EmojiGoal)
		output := RenderMonorepoTree(tree)
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
			t.Errorf("tree text should contain short child ID %schildgoal, got:\n%s", goalEmoji, output)
		}
		if !grandchildFound {
			t.Errorf("tree text should contain short grandchild ID %sgrandchildgoal, got:\n%s", goalEmoji, output)
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
		renderTreeNodeText(&sb, node, "", true, true)
		if data["parentId"] != "🎯️parent" {
			t.Errorf("renderTreeNodeText should restore parentId, got: %v", data["parentId"])
		}
	})
}

func TestExhaustiveBuildMonorepoTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree build test in short mode")
	}

	withExhaustiveRepo(t)

	ctx := context.Background()
	treeNoSections, err := BuildMonorepoTreeCached(ctx)
	if err != nil {
		t.Fatal(err)
	}
	treeSections, err := BuildMonorepoTreeCached(ctx, TreeBuildOptions{IncludeSections: true})
	if err != nil {
		t.Fatal(err)
	}
	t.Run("builds tree with categories", func(t *testing.T) {
		if treeNoSections == nil {
			t.Fatal("tree should not be nil")
		}
		if len(treeNoSections.Children) == 0 {
			t.Fatal("tree should have categories")
		}
		categoryIDs := make(map[string]bool)
		for _, c := range treeNoSections.Children {
			if c.Kind != TreeNodeCategory {
				t.Errorf("top-level children should be categories, got %s", c.Kind)
			}
			categoryIDs[c.ID] = true
		}
		expected := []string{"codebase", "goals", "drafts", "policies", "contributors", "checkpoints"}
		for _, id := range expected {
			if !categoryIDs[id] {
				t.Errorf("missing category: %s", id)
			}
		}
	})

	t.Run("codebase category has folder and file document", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		if len(codebaseNode.Children) == 0 {
			t.Fatal("codebase category should have children")
		}
		hasFolder := false
		hasFile := false
		hasSection := false
		hasDefinition := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			switch node.Kind {
			case TreeNodeFolder:
				hasFolder = true
			case TreeNodeFile:
				hasFile = true
			case TreeNodeSection:
				hasSection = true
			case TreeNodeDefinition:
				hasDefinition = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(codebaseNode)
		if !hasFolder {
			t.Error("codebase category should include folder nodes")
		}
		if !hasFile {
			t.Error("codebase category should include file nodes")
		}
		if !hasSection {
			t.Error("codebase category should include section nodes when IncludeSections is true")
		}
		if !hasDefinition {
			t.Error("codebase category should include definition nodes when IncludeSections is true")
		}
	})

	t.Run("codebase category has technology children", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		hasTechnologies := false
		hasBundles := false
		for _, p := range codebaseNode.Children {
			if p.Kind == TreeNodeTechnology {
				hasTechnologies = true
				for _, b := range p.Children {
					if b.Kind == TreeNodeBundle {
						hasBundles = true
					}
				}
			}
		}
		if !hasTechnologies {
			t.Error("codebase should have technology children")
		}
		if !hasBundles {
			t.Error("at least one technology should have bundles")
		}
	})

	t.Run("policies category uses entitykind grouping", func(t *testing.T) {
		var policiesNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "policies" {
				policiesNode = c
				break
			}
		}
		if policiesNode == nil {
			t.Fatal("policies category not found")
		}
		if len(policiesNode.Children) == 0 {
			t.Fatal("policies should have children")
		}
		policy := policiesNode.Children[0]
		if policy.Kind != TreeNodePolicy {
			t.Fatalf("expected policy node, got %s", policy.Kind)
		}
		if len(policy.Children) == 0 {
			t.Fatal("policy should contain entitykind children")
		}
		entityKind := policy.Children[0]
		if entityKind.Kind != TreeNodeCategory {
			t.Fatalf("expected entitykind category node, got %s", entityKind.Kind)
		}
		if entityKind.SubKind != "entitykind" {
			t.Fatalf("expected entitykind subkind, got %s", entityKind.SubKind)
		}
		if len(entityKind.Children) == 0 {
			t.Fatal("entitykind should contain statute children")
		}
		for _, statute := range entityKind.Children {
			if statute.Kind != TreeNodeStatute {
				t.Fatalf("expected statute child under entitykind, got %s", statute.Kind)
			}
		}
	})

	t.Run("with sections includes sections", func(t *testing.T) {
		hasSections := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				hasSections = true
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeSections)
		if !hasSections {
			t.Error("tree with IncludeSections should have section nodes")
		}
	})

	t.Run("without sections excludes sections", func(t *testing.T) {
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				t.Error("tree without IncludeSections should not have section nodes")
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeNoSections)
	})
}

func TestClientCachedTreeRefreshesPersistentDomainRecordsWithoutSourceChanges(t *testing.T) {
	root := initTestGitRepo(t, "main")
	previousRoot := GetRootDir()
	SetRootDir(root)
	t.Cleanup(func() { SetRootDir(previousRoot) })
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "🔎️search", "🧪️tests", "🧫️fixtures", "🌲️snapshot", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct{ DomainRefresh struct{ Titles []string } }
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	firstTitle, secondTitle := fixture.DomainRefresh.Titles[0], fixture.DomainRefresh.Titles[1]

	path := filepath.Join(GetRepoGoalsDir(), "identity", "🎯️goal.json")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatal(err)
	}
	write := func(title string) {
		data, _ := json.Marshal(map[string]string{"title": title, "status": "open"})
		if err := os.WriteFile(path, data, 0600); err != nil {
			t.Fatal(err)
		}
	}
	label := func(root *TreeNode) string {
		var find func(*TreeNode) string
		find = func(node *TreeNode) string {
			if node.Kind == TreeNodeGoal {
				return node.Label
			}
			for _, child := range node.Children {
				if found := find(child); found != "" {
					return found
				}
			}
			return ""
		}
		return find(root)
	}
	write("baseline title")
	git := func(args ...string) string {
		command := exec.Command("git", args...)
		command.Dir = root
		output, err := command.CombinedOutput()
		if err != nil {
			t.Fatalf("fixture git %v: %v %s", args, err, output)
		}
		return string(output)
	}
	git("add", "--", path)
	git("commit", "-m", "record fixture")
	write(firstTitle)
	beforeStatus := git("status", "--porcelain", "--untracked-files=no")
	first, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	snapshotPath := filepath.Join(getCacheDir(), "tree", "snapshot", "payload.json.gz")
	beforeSnapshot, err := os.Stat(snapshotPath)
	if err != nil {
		t.Fatal(err)
	}
	if label(first) != firstTitle {
		t.Fatalf("initial goal=%q", label(first))
	}
	indexed, err := SearchMonorepoTreeWithOptions(context.Background(), first, "first", TreeSearchOptions{Limit: 4})
	if err != nil {
		t.Fatal(err)
	}
	if label(indexed) != firstTitle {
		t.Fatal("initial domain record absent from index")
	}
	write(secondTitle)
	afterStatus := git("status", "--porcelain", "--untracked-files=no")
	if beforeStatus != afterStatus || !strings.Contains(afterStatus, "M ") {
		t.Fatalf("fixture status differs: %q -> %q", beforeStatus, afterStatus)
	}
	second, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	afterSnapshot, err := os.Stat(snapshotPath)
	if err != nil {
		t.Fatal(err)
	}
	if !os.SameFile(beforeSnapshot, afterSnapshot) {
		t.Fatal("domain-only edit rebuilt authored codebase snapshot")
	}
	if label(second) != secondTitle {
		t.Fatalf("warm goal retained stale value %q", label(second))
	}
	indexed, err = SearchMonorepoTreeWithOptions(context.Background(), second, "second", TreeSearchOptions{Limit: 4})
	if err != nil {
		t.Fatal(err)
	}
	if label(indexed) != secondTitle {
		t.Fatalf("warm index retained stale domain value %q", label(indexed))
	}
}

func TestClientCachedSearchCapturesOneSourceIdentityPerOperation(t *testing.T) {
 root := initTestGitRepo(t,"main")
 previous := GetRootDir(); SetRootDir(root); t.Cleanup(func(){SetRootDir(previous)})
 raw,err := os.ReadFile(sandbox.OriginPath("..","..","🔎️search","🧪️tests","🧫️fixtures","🌲️snapshot","💾️streaming","🔣️.json"))
 if err != nil {t.Fatal(err)}
 var fixture struct{ QueryCapture struct{ SourceCaptures int; Query string; Cases []struct{Title string; Source string; Reuse bool} } }
 if err=json.Unmarshal(raw,&fixture);err!=nil{t.Fatal(err)}
 var before os.FileInfo
 for _,row:=range fixture.QueryCapture.Cases{
  if err=os.WriteFile(filepath.Join(root,"capture.go"),[]byte(row.Source),0600);err!=nil{t.Fatal(err)}
  goal:=filepath.Join(GetRepoGoalsDir(),"capture","🎯️goal.json");if err=os.MkdirAll(filepath.Dir(goal),0755);err!=nil{t.Fatal(err)}
  body,_:=json.Marshal(map[string]string{"title":row.Title,"status":"open"});if err=os.WriteFile(goal,body,0600);err!=nil{t.Fatal(err)}
  captures:=0
  progress:=func(value search.Progress){if value.Step=="fingerprinting"{captures++}}
  tree,err:=SearchMonorepoTreeCachedWithOptions(context.Background(),fixture.QueryCapture.Query,TreeBuildOptions{Progress:progress},TreeSearchOptions{Limit:4,Progress:progress})
  if err!=nil{t.Fatal(err)}
  if captures!=fixture.QueryCapture.SourceCaptures{t.Fatalf("source captures=%d expected=%d",captures,fixture.QueryCapture.SourceCaptures)}
  var find func(*TreeNode) bool
  find=func(node *TreeNode)bool{if node.Kind==TreeNodeGoal&&node.Label==row.Title{return true};for _,child:=range node.Children{if find(child){return true}};return false}
  if !find(tree){t.Fatalf("fresh domain record missing: %q",row.Title)}
  after,err:=os.Stat(filepath.Join(getCacheDir(),"tree","snapshot","payload.json.gz"));if err!=nil{t.Fatal(err)}
  if before!=nil&&os.SameFile(before,after)!=row.Reuse{t.Fatalf("authored snapshot reuse=%v expected=%v",os.SameFile(before,after),row.Reuse)}
  before=after
 }
}

func TestClientCachedSearchRefusesRootChangesDuringCapture(t *testing.T) {
 first, second := initTestGitRepo(t,"main"),initTestGitRepo(t,"main")
 previous:=GetRootDir();SetRootDir(first);t.Cleanup(func(){SetRootDir(previous)})
 progress:=func(value search.Progress){if value.Step=="fingerprinting"{SetRootDir(second)}}
 _,err:=SearchMonorepoTreeCachedWithOptions(context.Background(),"capture",TreeBuildOptions{Progress:progress},TreeSearchOptions{Limit:4})
 if !errors.Is(err,search.ErrCorrupt){t.Fatalf("changed Source root accepted: %v",err)}
}
