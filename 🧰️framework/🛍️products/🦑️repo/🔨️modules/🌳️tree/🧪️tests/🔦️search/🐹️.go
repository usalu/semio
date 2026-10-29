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
	time "time"

	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
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
		result, err := SearchTreeContext(context.Background(), tree, "", TreeSearchOptions{Limit: 100})
		if err != nil {
			t.Fatal(err)
		}
		if len(result.Children) != 2 {
			t.Errorf("expected 2 categories, got %d", len(result.Children))
		}
	})

	t.Run("query matches items", func(t *testing.T) {
		tree := makeTree()
		result, err := SearchTreeContext(context.Background(), tree, "cli", TreeSearchOptions{Limit: 100})
		if err != nil {
			t.Fatal(err)
		}
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
		result, err := SearchTreeContext(context.Background(), tree, "zzzznonexistent", TreeSearchOptions{Limit: 100})
		if err != nil {
			t.Fatal(err)
		}
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
		result, err := SearchTreeContext(context.Background(), tree, "cli", TreeSearchOptions{Limit: 100})
		if err != nil {
			t.Fatal(err)
		}
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
	sandbox.BindRoot(t, tmpDir)

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

	t.Run("refreshes indexed matches when projection changes", func(t *testing.T) {
		indexedTree := makeTree("ultrafastneedle")
		idx, err := ensureCacheIndexed(context.Background(), indexedTree, "", nil)
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
		if found {
			t.Fatal("stale cached index retained a previous projection match")
		}
		fresh, err := SearchMonorepoTreeWithCache(context.Background(), mutatedTree, "different-text")
		if err != nil {
			t.Fatal(err)
		}
		found = false
		walk(fresh)
		if !found {
			t.Fatal("current projection missing from rebuilt index")
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
	monoRoot := workspace.CanonicalPath(t.TempDir())
	clientDir := filepath.Join(monoRoot, "repo", "cli")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(monoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	oldRoot := workspace.GetRootDir()
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

func TestComputeCompositeFingerprintIncludesAuthoredUntrackedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	firstFingerprint, firstMeta, fingerprintErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if fingerprintErr != nil {
		t.Fatal(fingerprintErr)
	}
	if firstFingerprint == "" || firstMeta == nil {
		t.Fatalf("expected first fingerprint and meta to be set, got %q / %+v", firstFingerprint, firstMeta)
	}

	if err := os.WriteFile(filepath.Join(tmpDir, "untracked.txt"), []byte("untracked"), 0644); err != nil {
		t.Fatalf("failed to create untracked file: %v", err)
	}

	secondFingerprint, secondMeta, secondErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if secondErr != nil {
		t.Fatal(secondErr)
	}
	if secondFingerprint == "" || secondMeta == nil {
		t.Fatalf("expected second fingerprint and meta to be set, got %q / %+v", secondFingerprint, secondMeta)
	}
	if firstFingerprint == secondFingerprint {
		t.Fatalf("authored untracked file must change fingerprint: %q", firstFingerprint)
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

	_, meta, fingerprintErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if fingerprintErr != nil {
		t.Fatal(fingerprintErr)
	}
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
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "📚️library", "🧪️tests", "🧫️fixtures", "1️⃣g1-contract", "🔣️.json"))
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
			_, err := ensureCacheIndexed(ctx, tree, "", func(value search.Progress) {
				if value.Step == phase {
					cancel()
				}
			})
			if !errors.Is(err, context.Canceled) {
				t.Fatalf("%s cancellation = %v", phase, err)
			}
			assertSearchIndexBytes(t, indexPath, beforeEvents)
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
	if _, err := ensureCacheIndexed(context.Background(), oversized, "", nil); !errors.Is(err, search.ErrTooLarge) {
		t.Fatalf("maximum + 1 reindex = %v", err)
	}
	assertSearchIndexBytes(t, indexPath, beforeEvents)
	assertFileBytes(t, filepath.Join(indexPath, "meta.json"), beforeMeta)
	if _, err := os.Stat(indexPath + ".next"); !os.IsNotExist(err) {
		t.Fatalf("maximum + 1 left staged index: %v", err)
	}
}

func TestG1CorruptCurrentIndexErrorPropagates(t *testing.T) {
	tree, indexPath, _, _ := prepareG1SearchCache(t)
	eventPath := firstSearchShardPath(t, indexPath)
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
	if _, err := ensureCacheIndexed(context.Background(), tree, "", nil); !errors.Is(err, search.ErrCorrupt) {
		t.Fatalf("corrupt current index = %v", err)
	}
}

func TestG1ReindexLockWaitCancellation(t *testing.T) {
	tree, indexPath, beforeEvents, beforeMeta := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "lock")
	lockPath := filepath.Join(filepath.Dir(indexPath), ".lock")
	held, err := search.AcquireGenerationLease(context.Background(), lockPath, false, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer held.Close()
	ctx, cancel := context.WithCancel(context.Background())
	_, err = ensureCacheIndexed(ctx, tree, "", func(value search.Progress) {
		if value.Step == "waiting-lock" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("lock wait cancellation = %v", err)
	}
	assertSearchIndexBytes(t, indexPath, beforeEvents)
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
	sandbox.BindRoot(t, repoRoot)
	tree := g1SearchTree("last-valid", "stable")
	steps := map[string]bool{}
	indexValue, err := ensureCacheIndexed(context.Background(), tree, "", func(value search.Progress) { steps[value.Step] = true })
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
	events := snapshotSearchIndex(t, indexPath)
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

func TestSearchIndexIsNotReusedAcrossSectionChoice(t *testing.T) {
	tmpDir := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo"), 0755); err != nil {
		t.Fatalf("mkdir .🦑️repo: %v", err)
	}
	sandbox.BindRoot(t, tmpDir)

	file := &TreeNode{Kind: TreeNodeFile, ID: "file:main", Label: "main"}
	withoutSections := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{file}}
	withSections := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{{Kind: TreeNodeFile, ID: "file:main", Label: "main", Children: []*TreeNode{
		{Kind: TreeNodeSection, ID: "section:models", Label: "Models"},
	}}}}

	first, err := SearchMonorepoTreeWithCache(context.Background(), withoutSections, "Models")
	if err != nil {
		t.Fatalf("search without sections failed: %v", err)
	}
	if len(first.Children) != 0 {
		t.Fatalf("a tree without sections must not match a section name, got %d children", len(first.Children))
	}
	second, err := SearchMonorepoTreeWithCache(context.Background(), withSections, "Models")
	if err != nil {
		t.Fatalf("search with sections failed: %v", err)
	}
	if len(second.Children) == 0 {
		t.Fatal("the index built for a tree without sections must not answer a search over a tree with sections")
	}
}

func snapshotSearchIndex(t *testing.T, path string) []byte {
	t.Helper()
	files := map[string][]byte{}
	err := filepath.WalkDir(path, func(name string, entry os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.IsDir() {
			return nil
		}
		relative, err := filepath.Rel(path, name)
		if err != nil {
			return err
		}
		if relative == "meta.json" {
			return nil
		}
		data, err := os.ReadFile(name)
		if err != nil {
			return err
		}
		files[relative] = data
		return nil
	})
	if err != nil {
		t.Fatal(err)
	}
	data, err := json.Marshal(files)
	if err != nil {
		t.Fatal(err)
	}
	return data
}
func assertSearchIndexBytes(t *testing.T, path string, before []byte) {
	t.Helper()
	if !bytes.Equal(before, snapshotSearchIndex(t, path)) {
		t.Fatal("committed search manifest or shard bytes changed")
	}
}
func firstSearchShardPath(t *testing.T, path string) string {
	t.Helper()
	data, err := os.ReadFile(filepath.Join(path, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest search.ShardManifest
	if err = json.Unmarshal(data, &manifest); err != nil || len(manifest.Shards) == 0 {
		t.Fatalf("manifest: %+v %v", manifest, err)
	}
	return filepath.Join(path, manifest.Shards[0].Name, "events.jsonl")
}

func TestSearchTreeStreamsBeyondSingleShardAndFiltersBeforeTopK(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "🔎️search", "🧪️tests", "🧫️fixtures", "🧵️shards", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct{ Large struct{ Documents int } }
	if err = json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	_, _, _, _ = prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "streamed")
	root := &TreeNode{Kind: TreeNodeCategory, Label: "."}
	for i := 0; i < fixture.Large.Documents; i++ {
		description := "plain"
		kind := TreeNodeBundle
		if i == 0 || i == fixture.Large.Documents-1 {
			description = "needle"
		}
		if i == fixture.Large.Documents-1 {
			kind = TreeNodeFile
		}
		root.Children = append(root.Children, &TreeNode{Kind: kind, ID: fmt.Sprintf("d-%07d", i), Label: fmt.Sprintf("doc-%07d", i), Description: description})
	}
	filter := &TreeFilter{OnlyKinds: map[TreeNodeKind]bool{TreeNodeFile: true}}
	for attempt := 0; attempt < 2; attempt++ {
		result, err := SearchMonorepoTreeWithOptions(context.Background(), root, "needle", TreeSearchOptions{Limit: 1, Filter: filter})
		if err != nil {
			t.Fatal(err)
		}
		if len(result.Children) != 1 || result.Children[0].ID != "d-0250000" {
			t.Fatalf("late eligible match missing: %+v", result.Children)
		}
	}
}

func TestSearchReaderOwnsPublicationLockUntilClose(t *testing.T) {
	tree, path, before, _ := prepareG1SearchCache(t)
	reader, err := ensureCacheIndexed(context.Background(), tree, "", nil)
	if err != nil {
		t.Fatal(err)
	}
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "writer")
	ctx, cancel := context.WithCancel(context.Background())
	_, err = ensureCacheIndexed(ctx, g1SearchTree("replacement"), "", func(value search.Progress) {
		if value.Step == "waiting-lock" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("concurrent publication: %v", err)
	}
	assertSearchIndexBytes(t, path, before)
	if err = reader.Close(); err != nil {
		t.Fatal(err)
	}
	replacement, err := ensureCacheIndexed(context.Background(), g1SearchTree("replacement"), "", nil)
	if err != nil {
		t.Fatal(err)
	}
	if err = replacement.Close(); err != nil {
		t.Fatal(err)
	}
}

func TestSearchRejectsCyclesDepthAndExpandedResultBudgets(t *testing.T) {
	cyclic := &TreeNode{Kind: TreeNodeCategory}
	cyclic.Children = []*TreeNode{cyclic}
	if _, err := treeIncludesSections(context.Background(), cyclic); err == nil {
		t.Fatal("cyclic traversal accepted")
	}
	root := &TreeNode{ID: "parent", Kind: TreeNodeFolder}
	node := root
	for i := 0; i < search.MaxTraversalDepth; i++ {
		child := &TreeNode{ID: fmt.Sprint(i), Kind: TreeNodeFolder}
		node.Children = []*TreeNode{child}
		node = child
	}
	if _, err := treeIncludesSections(context.Background(), root); !errors.Is(err, search.ErrTooLarge) {
		t.Fatalf("depth: %v", err)
	}
	root = &TreeNode{ID: "parent", Kind: TreeNodeFolder}
	for i := 0; i < search.MaxResultTreeNodes; i++ {
		root.Children = append(root.Children, &TreeNode{ID: fmt.Sprint(i), Kind: TreeNodeFile})
	}
	if _, err := pruneUnmatchedContext(context.Background(), root, map[string]bool{"parent": true}, nil); !errors.Is(err, search.ErrTooLarge) {
		t.Fatalf("expanded result: %v", err)
	}
}

func TestSearchCacheSeparatesScopedProjections(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "🔎️search", "🧪️tests", "🧫️fixtures", "🧵️shards", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Scopes []struct {
			Scope     string
			Documents map[string]string
			Hits      []string
		}
	}
	if err = json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	_, _, _, _ = prepareG1SearchCache(t)
	for _, value := range fixture.Scopes {
		root := &TreeNode{Kind: TreeNodeCategory, Label: "."}
		for id, text := range value.Documents {
			root.Children = append(root.Children, &TreeNode{Kind: TreeNodeBundle, ID: id, Label: "record", Description: text})
		}
		result, err := SearchMonorepoTreeWithOptions(context.Background(), root, "needle", TreeSearchOptions{Limit: 1, Scope: value.Scope})
		if err != nil {
			t.Fatal(err)
		}
		if len(result.Children) != len(value.Hits) {
			t.Fatalf("scope %s leaked another projection: %+v", value.Scope, result.Children)
		}
		for i, id := range value.Hits {
			if result.Children[i].ID != id {
				t.Fatalf("scope %s: %+v", value.Scope, result.Children)
			}
		}
	}
}

func TestSearchProgressPanicReleasesGenerationOwnership(t *testing.T) {
	_, path, before, _ := prepareG1SearchCache(t)
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "panic-staging")
	expectPanic := func(action func()) {
		t.Helper()
		found := false
		func() {
			defer func() {
				if value := recover(); value != nil {
					if value != "expected search progress panic" {
						t.Fatalf("unexpected panic: %v", value)
					}
					found = true
				}
			}()
			action()
		}()
		if !found {
			t.Fatal("progress callback did not run")
		}
	}
	expectPanic(func() {
		_, _ = ensureCacheIndexed(context.Background(), g1SearchTree("needle"), "", func(value search.Progress) {
			if value.Step == "indexed" {
				panic("expected search progress panic")
			}
		})
	})
	assertSearchIndexBytes(t, path, before)
	if _, err := os.Lstat(path + ".next"); !os.IsNotExist(err) {
		t.Fatalf("panic left staged generation: %v", err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	writer, err := ensureCacheIndexed(ctx, g1SearchTree("needle"), "", nil)
	if err != nil {
		t.Fatal(err)
	}
	if err = writer.Close(); err != nil {
		t.Fatal(err)
	}
	expectPanic(func() {
		_, _ = SearchMonorepoTreeWithOptions(context.Background(), g1SearchTree("needle"), "needle", TreeSearchOptions{Limit: 1, Progress: func(value search.Progress) {
			if value.Step == "searching" {
				panic("expected search progress panic")
			}
		}})
	})
	invalidateG1SearchFingerprint(t, workspace.GetRootDir(), "panic-reader")
	writer, err = ensureCacheIndexed(ctx, g1SearchTree("replacement"), "", nil)
	if err != nil {
		t.Fatalf("reader ownership leaked: %v", err)
	}
	if err = writer.Close(); err != nil {
		t.Fatal(err)
	}
}

func TestCachedTreeRejectsCancelledDiscoveryBeforePublishing(t *testing.T) {
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, ".git"), 0755); err != nil {
		t.Fatal(err)
	}
	sandbox.BindRoot(t, root)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	tree, err := BuildMonorepoTreeCached(ctx)
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("cancelled cache: %v", err)
	}
	if tree != nil {
		t.Fatal("cancelled tree cache returned a partial tree")
	}
	if _, err := os.Stat(filepath.Join(getCacheDir(), "tree", "snapshot")); !os.IsNotExist(err) {
		t.Fatalf("cancelled cache published payload: %v", err)
	}
}

func TestComputeCompositeFingerprintChangesWhilePorcelainRemainsModified(t *testing.T) {
	if testing.Short() {
		t.Skip("real Git source identity oracle")
	}
	root := initTestGitRepo(t, "main")
	source := filepath.Join(root, "file.txt")
	if err := os.WriteFile(source, []byte("first edit"), 0600); err != nil {
		t.Fatal(err)
	}
	first, _, err := computeCompositeFingerprintContext(context.Background(), root, nil)
	if err != nil {
		t.Fatal(err)
	}
	status := exec.Command("git", "status", "--porcelain", "-z")
	status.Dir = root
	before, err := status.Output()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(source, []byte("second edit"), 0600); err != nil {
		t.Fatal(err)
	}
	second, _, err := computeCompositeFingerprintContext(context.Background(), root, nil)
	if err != nil {
		t.Fatal(err)
	}
	status = exec.Command("git", "status", "--porcelain", "-z")
	status.Dir = root
	after, err := status.Output()
	if err != nil {
		t.Fatal(err)
	}
	if string(before) != string(after) {
		t.Fatalf("oracle status changed: %q -> %q", before, after)
	}
	if first == second {
		t.Fatal("changed authored bytes retained stale fingerprint")
	}
}

func TestRepositoryTreeDiscoveryJoinsOnlyStartedProducers(t *testing.T) {
	sandbox.BindRoot(t, t.TempDir())
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	done := make(chan error, 1)
	go func() { _, err := BuildMonorepoTreeFromRepo(ctx); done <- err }()
	select {
	case err := <-done:
		if err != nil {
			t.Fatal(err)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("repository tree joined an unstarted producer")
	}
}

func TestCachedTreeRefreshesPersistentDomainRecordsWithoutSourceChanges(t *testing.T) {
	root := initTestGitRepo(t, "main")
	sandbox.BindRoot(t, root)
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "🔎️search", "🧪️tests", "🧫️fixtures", "🌲️snapshot", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct{ DomainRefresh struct{ Titles []string } }
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	firstTitle, secondTitle := fixture.DomainRefresh.Titles[0], fixture.DomainRefresh.Titles[1]

	path := filepath.Join(workspace.GetRepoGoalsDir(), "identity", "🎯️goal.json")
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

func TestCachedTreeCancelsDuringIdentityResolutionWithoutPublishing(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "📐️model", "🧪️tests", "🧫️fixtures", "🆔️artifact-id", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		DiscoveryCancellation struct {
			Files, CancelAt int
			Phase           string
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	sandbox.BindRoot(t, t.TempDir())
	for position := 0; position < fixture.DiscoveryCancellation.Files; position++ {
		if err := os.WriteFile(filepath.Join(workspace.GetRootDir(), fmt.Sprintf("f%06d.go", position)), []byte("package fixture"), 0600); err != nil {
			t.Fatal(err)
		}
	}
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	reached := false
	value, err := BuildMonorepoTreeCached(ctx, TreeBuildOptions{Progress: func(progress search.Progress) {
		if progress.Step == fixture.DiscoveryCancellation.Phase && progress.Current >= fixture.DiscoveryCancellation.CancelAt {
			reached = true
			cancel()
		}
	}})
	if !reached || !errors.Is(err, context.Canceled) || value != nil {
		t.Fatalf("late cancellation: reached=%v value=%v error=%v", reached, value, err)
	}
	if _, err := os.Stat(filepath.Join(getCacheDir(), "tree", "snapshot")); !os.IsNotExist(err) {
		t.Fatalf("cancelled identity discovery published snapshot: %v", err)
	}
}

func TestSearchProjectionStableAcrossMapReadsAndSnapshotRoundTrip(t *testing.T) {
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "..", "🔎️search", "🧪️tests", "🧫️fixtures", "🌲️snapshot", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		ProjectionText struct {
			Data            map[string]interface{}
			Expected, Alias string
			Repeats         int
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for attempt := 0; attempt < fixture.ProjectionText.Repeats; attempt++ {
		if found := search.FieldText(fixture.ProjectionText.Data); found != fixture.ProjectionText.Expected {
			t.Fatalf("map iteration changed indexed text: %q, expected %q", found, fixture.ProjectionText.Expected)
		}
	}
	type alias string
	root := &TreeNode{Kind: TreeNodeCategory, Children: []*TreeNode{{Kind: TreeNodeFile, ID: "record", Label: "record", Data: map[string]interface{}{"kind": alias(fixture.ProjectionText.Alias), "labels": []alias{"first", "second"}, "nullable": []string(nil), "nilAliases": []alias(nil), "nilMap": map[string]string(nil), "nilPointer": (*string)(nil)}}}}
	bytes, err := json.Marshal(root)
	if err != nil {
		t.Fatal(err)
	}
	var decoded TreeNode
	if err := json.Unmarshal(bytes, &decoded); err != nil {
		t.Fatal(err)
	}
	before, err := fingerprintSearchProjection(context.Background(), root)
	if err != nil {
		t.Fatal(err)
	}
	after, err := fingerprintSearchProjection(context.Background(), &decoded)
	if err != nil {
		t.Fatal(err)
	}
	if before != after {
		t.Fatal("snapshot roundtrip changed indexed projection identity")
	}
}

func TestSearchIndexReusesEquivalentSnapshotProjection(t *testing.T) {
	_, _, _, _ = prepareG1SearchCache(t)
	root := g1SearchTree("needle")
	root.Children[0].Data = map[string]interface{}{"zeta": "last", "alpha": "first", "middle": []string{"second", "third"}}
	first, err := ensureCacheIndexed(context.Background(), root, "", nil)
	if err != nil {
		t.Fatal(err)
	}
	if err := first.Close(); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(getCacheDir(), "index.search", "meta.json")
	before, err := os.Stat(path)
	if err != nil {
		t.Fatal(err)
	}
	data, err := json.Marshal(root)
	if err != nil {
		t.Fatal(err)
	}
	var decoded TreeNode
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatal(err)
	}
	for attempt := 0; attempt < 8; attempt++ {
		rebuilt := false
		index, err := ensureCacheIndexed(context.Background(), &decoded, "", func(value search.Progress) {
			if value.Step == "indexed" {
				rebuilt = true
			}
		})
		if err != nil {
			t.Fatal(err)
		}
		if err := index.Close(); err != nil {
			t.Fatal(err)
		}
		after, err := os.Stat(path)
		if err != nil {
			t.Fatal(err)
		}
		if rebuilt || !os.SameFile(before, after) {
			t.Fatal("equivalent snapshot projection rebuilt search index")
		}
	}
}
