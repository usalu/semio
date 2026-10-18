// 🔦️ Tests of the tree search, its query cache, fingerprints and reindex laws.

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
	strings "strings"
	testing "testing"

	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

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
	run(childWorkTree, "config", "commit.gpgsign", "false")
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
	data, err := os.ReadFile(filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "🧫️fixtures", "1️⃣g1-contract", "🔣️.json"))
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
