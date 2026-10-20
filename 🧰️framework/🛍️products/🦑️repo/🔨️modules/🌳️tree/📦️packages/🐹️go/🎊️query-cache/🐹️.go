// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎊️query-cache carries the search query cache: fingerprints, change detection, index swaps and cached queries.

// #endregion 🧲️Header

package tree

import (
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎊️Query Cache

// 🗄️Local event index under .🦑️repo/⚡️cache for keyword search. Uses a composite repository fingerprint for invalidation.
// 📌️cacheSchemaVersion holds the data fields for a cacheSchemaVersion record.
const cacheSchemaVersion = 3

// 💿️cacheMeta holds the data fields for a cacheMeta record.
type cacheMeta struct {
	SchemaVersion     int               `json:"SchemaVersion"`
	SuperHead         string            `json:"SuperHead"`
	SuperDirtyHash    string            `json:"SuperDirtyHash"`
	SubmodulePointers map[string]string `json:"SubmodulePointers"`
	SubmoduleHeads    map[string]string `json:"SubmoduleHeads,omitempty"`
	SubmoduleDirty    map[string]string `json:"SubmoduleDirty,omitempty"`
	PointersHash      string            `json:"PointersHash"`
	SubWorkingHash    string            `json:"SubWorkingHash"`
	Fingerprint       string            `json:"Fingerprint"`
	IncludeSections   bool              `json:"IncludeSections"`
}

// 💾️getCacheDir holds the data fields for a getCacheDir record.
func getCacheDir() string {
	abs, _ := filepath.Abs(workspace.GetRootDir())
	h := sha256.Sum256([]byte(abs))
	return filepath.Join(workspace.GetRepoMetaDir(), "⚡️cache", hex.EncodeToString(h[:]))
}

func computeCompositeFingerprint(repoRoot string) (fp string, meta *cacheMeta) {
	fp, meta, _ = computeCompositeFingerprintContext(context.Background(), repoRoot)
	return fp, meta
}

func computeCompositeFingerprintContext(ctx context.Context, repoRoot string) (fp string, meta *cacheMeta, err error) {
	meta = &cacheMeta{
		SchemaVersion:     cacheSchemaVersion,
		SubmodulePointers: map[string]string{},
		SubmoduleHeads:    map[string]string{},
		SubmoduleDirty:    map[string]string{},
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	stdout, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SuperHead = strings.TrimSpace(stdout)
	statusOut, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SuperDirtyHash = hashString(statusOut)
	subOut, err := getSubmoduleStatusContext(ctx, repoRoot)
	if err != nil {
		return "", nil, err
	}
	lines := strings.Split(strings.TrimSpace(subOut), "\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		if len(parts) < 2 {
			continue
		}
		path := parts[1]
		meta.SubmodulePointers[path] = parts[0]
		if strings.HasPrefix(parts[0], "-") {
			continue
		}
		if err := ctx.Err(); err != nil {
			return "", nil, err
		}
		subPath := filepath.Join(repoRoot, path)
		subHead, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, subPath)
		meta.SubmoduleHeads[path] = strings.TrimSpace(subHead)
		subStatus, _, _ := workspace.ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, subPath)
		if err := ctx.Err(); err != nil {
			return "", nil, err
		}
		meta.SubmoduleDirty[path] = hashString(subStatus)
	}
	meta.PointersHash = hashString(subOut)
	paths := make([]string, 0, len(meta.SubmoduleHeads))
	for k := range meta.SubmoduleHeads {
		paths = append(paths, k)
	}
	sort.Strings(paths)
	var subWork []string
	for _, path := range paths {
		subWork = append(subWork, path+"="+meta.SubmoduleHeads[path]+":"+meta.SubmoduleDirty[path])
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	composeMetaHash := hashComposeMetaState(repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SubWorkingHash = hashString(strings.Join(subWork, "|"))
	fp = hashString(meta.SuperHead + meta.SuperDirtyHash + meta.PointersHash + meta.SubWorkingHash + composeMetaHash + strconv.Itoa(cacheSchemaVersion))
	meta.Fingerprint = fp
	return fp, meta, nil
}

// 🧭️getSubmoduleStatus returns recursive submodule status when available.
func getSubmoduleStatus(repoRoot string) string {
	stdout, _ := getSubmoduleStatusContext(context.Background(), repoRoot)
	return stdout
}

func getSubmoduleStatusContext(ctx context.Context, repoRoot string) (string, error) {
	stdout, _, exitCode := workspace.ExecCommandContext(ctx, "git", []string{"submodule", "status", "--recursive"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if exitCode == 0 {
		return stdout, nil
	}
	fallback, _, fallbackExitCode := workspace.ExecCommandContext(ctx, "git", []string{"submodule", "status"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if fallbackExitCode == 0 {
		return fallback, nil
	}
	return stdout, nil
}

// ♻️hashComposeMetaState MUST produce a stable hash for compose metadata state changes.
// hashComposeMetaState computes and returns a hash for compose metadata content relevant to tree cache invalidation.
// Only structural changes (new goals, tickets, policies, drafts) invalidate the cache.
// ✏️Ephemeral data (agent session logs, ticket agent tracking updates) is excluded.
func hashComposeMetaState(repoRoot string) string {
	metaRoot := workspace.RepoMetaDirForRoot(repoRoot)
	if !workspace.FileExists(metaRoot) {
		return ""
	}
	var parts []string
	for _, top := range []string{"🎯️goals", "👮️", "📝️", "📊️metrics", "✍️notes"} {
		p := filepath.Join(metaRoot, top)
		if st, err := os.Stat(p); err == nil {
			parts = append(parts, "d:"+top+":"+strconv.FormatInt(st.ModTime().UnixNano(), 10))
		}
	}
	tickets, _ := filepath.Glob(filepath.Join(metaRoot, "🎫️tickets", "*", "*", "*", "*"))
	sort.Strings(tickets)
	parts = append(parts, "tickets:"+strings.Join(tickets, ","))
	return hashString(strings.Join(parts, "|"))
}

func treeNodeScopePath(node *TreeNode) string {
	if node.Data == nil {
		return ""
	}
	switch node.Kind {
	case TreeNodeFile, TreeNodeFolder:
		if p, ok := node.Data["path"].(string); ok {
			return p
		}
	case TreeNodeBundle:
		if r, ok := node.Data["root"].(string); ok {
			return r
		}
	case TreeNodeTechnology:
		if n, ok := node.Data["name"].(string); ok {
			return "technology:" + n
		}
	}
	return ""
}

// 🔤️hashString holds the data fields for a hashString record.
func hashString(s string) string {
	h := sha256.Sum256([]byte(s))
	return hex.EncodeToString(h[:])
}

// 🔷️loadCacheMeta holds the data fields for a loadCacheMeta record.
func loadCacheMeta(indexPath string) (*cacheMeta, error) {
	p := filepath.Join(indexPath, "meta.json")
	data, err := os.ReadFile(p)
	if err != nil {
		return nil, err
	}
	var m cacheMeta
	if err := json.Unmarshal(data, &m); err != nil {
		return nil, err
	}
	if m.SchemaVersion != cacheSchemaVersion {
		return nil, fmt.Errorf("schema version mismatch")
	}
	return &m, nil
}

// 🔶️saveCacheMeta holds the data fields for a saveCacheMeta record.
func saveCacheMeta(indexPath string, m *cacheMeta) error {
	dir := indexPath
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(m, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(dir, "meta.json"), data, 0644)
}

// 🛤️getChangedPathsFromGit holds the data fields for a getChangedPathsFromGit record.
func getChangedPathsFromGit(repoRoot string) []string {
	statusOut, _, _ := workspace.ExecCommand("git", []string{"status", "--porcelain", "-z"}, repoRoot)
	diffOut, _, _ := workspace.ExecCommand("git", []string{"diff", "--name-only", "HEAD"}, repoRoot)
	untrackedOut, _, _ := workspace.ExecCommand("git", []string{"ls-files", "-o", "--exclude-standard"}, repoRoot)
	paths := make(map[string]bool)
	for _, s := range strings.Split(strings.TrimSpace(statusOut), "\x00") {
		s = strings.TrimSpace(s)
		if len(s) >= 4 {
			paths[strings.TrimSpace(s[3:])] = true
		}
	}
	for _, p := range strings.Split(strings.TrimSpace(diffOut), "\n") {
		p = strings.TrimSpace(p)
		if p != "" {
			paths[p] = true
		}
	}
	for _, p := range strings.Split(strings.TrimSpace(untrackedOut), "\n") {
		if p != "" {
			paths[p] = true
		}
	}
	var result []string
	for p := range paths {
		result = append(result, p)
	}
	return result
}

// 🔹️expandPathsWithAncestors holds the data fields for a expandPathsWithAncestors record.
func expandPathsWithAncestors(paths []string) []string {
	technologies := codebase.LoadTechnologies()
	bundleByPath := make(map[string]string)
	technologyByBundle := make(map[string]string)
	for _, p := range technologies {
		for _, b := range p.Bundles {
			root := b.Root
			if b.SourceRoot != "" {
				root = b.SourceRoot
			}
			bundleByPath[root] = root
			technologyByBundle[root] = "technology:" + p.Name
		}
	}
	expanded := make(map[string]bool)
	for _, p := range paths {
		expanded[p] = true
		dir := p
		for {
			dir = filepath.Dir(dir)
			if dir == "." || dir == "" {
				break
			}
			expanded[dir] = true
		}
		var bundle string
		for root := range bundleByPath {
			if strings.HasPrefix(p, root+"/") || p == root {
				if len(root) > len(bundle) {
					bundle = root
				}
			}
		}
		if bundle != "" {
			expanded[bundle] = true
			if proj := technologyByBundle[bundle]; proj != "" {
				expanded[proj] = true
			}
		}
	}
	var result []string
	for p := range expanded {
		result = append(result, p)
	}
	return result
}

func pathToNodesMap(root *TreeNode) map[string][]*TreeNode {
	m := make(map[string][]*TreeNode)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind != TreeNodeCategory {
			p := treeNodeScopePath(node)
			if p != "" {
				m[p] = append(m[p], node)
			}
		}
		for _, c := range node.Children {
			walk(c)
		}
	}
	walk(root)
	return m
}

const maxSearchCacheLockAttempts = 60

// 🔸️ensureCacheIndexed builds a bounded staged index and atomically commits it after verification.
func ensureCacheIndexed(ctx context.Context, root *TreeNode, progress func(search.Progress)) (search.Index, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	repoRoot := workspace.GetRootDir()
	reportSearchProgress(progress, search.Progress{Step: "fingerprinting"})
	fingerprint, newMeta, err := computeCompositeFingerprintContext(ctx, repoRoot)
	if err != nil {
		return nil, err
	}
	includesSections, err := treeIncludesSections(ctx, root)
	if err != nil {
		return nil, err
	}
	newMeta.IncludeSections = includesSections
	dir := getCacheDir()
	indexPath := filepath.Join(dir, "index.search")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}
	release, err := acquireSearchCacheLock(ctx, filepath.Join(dir, ".lock"), progress)
	if err != nil {
		return nil, err
	}
	defer release()
	if err := recoverSearchIndexSwap(indexPath); err != nil {
		return nil, err
	}
	existing, metaErr := loadCacheMeta(indexPath)
	if metaErr == nil && existing.Fingerprint == fingerprint && existing.IncludeSections == includesSections {
		index, err := search.Open(indexPath)
		if err != nil {
			return nil, fmt.Errorf("open current search index: %w", err)
		}
		reportSearchProgress(progress, search.Progress{Current: 1, Total: 1, Step: "ready"})
		return index, nil
	}
	if metaErr != nil && !os.IsNotExist(metaErr) {
		return nil, fmt.Errorf("load search index metadata: %w", metaErr)
	}
	nodes, err := collectSearchIndexNodes(ctx, root, progress)
	if err != nil {
		return nil, err
	}
	stagedPath := indexPath + ".next"
	if err := os.RemoveAll(stagedPath); err != nil {
		return nil, err
	}
	staged, err := search.New(stagedPath, search.NewIndexMapping())
	if err != nil {
		return nil, err
	}
	cleanup := func() { _ = os.RemoveAll(stagedPath) }
	for offset, node := range nodes {
		if err := ctx.Err(); err != nil {
			cleanup()
			return nil, err
		}
		documentID := node.ID
		if documentID == "" {
			documentID = node.Label
		}
		document := map[string]interface{}{"text": buildSearchDocumentText(node)}
		if path := treeNodeScopePath(node); path != "" {
			document["path"] = path
		}
		if err := staged.IndexContext(ctx, documentID, document, nil); err != nil {
			cleanup()
			return nil, fmt.Errorf("index %q: %w", documentID, err)
		}
		reportSearchProgress(progress, search.Progress{Current: offset + 1, Total: len(nodes), Step: "indexed"})
	}
	if err := ctx.Err(); err != nil {
		cleanup()
		return nil, err
	}
	if err := staged.CloseContext(ctx, func(value search.Progress) {
		value.Step = "persisting-" + value.Step
		reportSearchProgress(progress, value)
	}); err != nil {
		cleanup()
		return nil, fmt.Errorf("persist staged search index: %w", err)
	}
	if err := saveCacheMeta(stagedPath, newMeta); err != nil {
		cleanup()
		return nil, err
	}
	verified, err := search.Open(stagedPath)
	if err != nil {
		cleanup()
		return nil, fmt.Errorf("verify staged search index: %w", err)
	}
	if err := verified.Close(); err != nil {
		cleanup()
		return nil, err
	}
	if err := ctx.Err(); err != nil {
		cleanup()
		return nil, err
	}
	if err := commitSearchIndexSwap(indexPath, stagedPath); err != nil {
		cleanup()
		return nil, err
	}
	reportSearchProgress(progress, search.Progress{Current: len(nodes), Total: len(nodes), Step: "committed"})
	index, err := search.Open(indexPath)
	if err != nil {
		return nil, fmt.Errorf("open committed search index: %w", err)
	}
	return index, nil
}

func acquireSearchCacheLock(ctx context.Context, path string, progress func(search.Progress)) (func(), error) {
	for attempt := 0; attempt < maxSearchCacheLockAttempts; attempt++ {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		lock, err := os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o644)
		if err == nil {
			if err := lock.Close(); err != nil {
				_ = os.Remove(path)
				return nil, err
			}
			return func() { _ = os.Remove(path) }, nil
		}
		if !os.IsExist(err) {
			return nil, err
		}
		if info, statErr := os.Stat(path); statErr == nil && time.Since(info.ModTime()) > 5*time.Minute {
			if err := os.Remove(path); err != nil && !os.IsNotExist(err) {
				return nil, err
			}
			continue
		}
		reportSearchProgress(progress, search.Progress{Current: attempt + 1, Total: maxSearchCacheLockAttempts, Step: "waiting-lock"})
		timer := time.NewTimer(200 * time.Millisecond)
		select {
		case <-ctx.Done():
			if !timer.Stop() {
				<-timer.C
			}
			return nil, ctx.Err()
		case <-timer.C:
		}
	}
	return nil, errors.New("search index lock timeout")
}

// 🧩️treeIncludesSections reports whether the tree carries section or definition nodes, the part of a tree an index must not be reused without.
func treeIncludesSections(ctx context.Context, root *TreeNode) (bool, error) {
	if root == nil {
		return false, errors.New("search tree is required")
	}
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return false, err
		}
		last := len(stack) - 1
		node := stack[last]
		stack = stack[:last]
		if node.Kind == TreeNodeSection || node.Kind == TreeNodeDefinition {
			return true, nil
		}
		stack = append(stack, node.Children...)
	}
	return false, nil
}

func collectSearchIndexNodes(ctx context.Context, root *TreeNode, progress func(search.Progress)) ([]*TreeNode, error) {
	if root == nil {
		return nil, errors.New("search tree is required")
	}
	stack := []*TreeNode{root}
	nodes := make([]*TreeNode, 0)
	visited := 0
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		last := len(stack) - 1
		node := stack[last]
		stack = stack[:last]
		visited++
		if visited > search.MaxTraversalNodes {
			return nil, fmt.Errorf("%w: tree nodes > %d", search.ErrTooLarge, search.MaxTraversalNodes)
		}
		if node.Kind != TreeNodeCategory {
			if len(nodes) >= search.MaxDocuments {
				return nil, fmt.Errorf("%w: tree documents > %d", search.ErrTooLarge, search.MaxDocuments)
			}
			nodes = append(nodes, node)
		}
		for index := len(node.Children) - 1; index >= 0; index-- {
			stack = append(stack, node.Children[index])
		}
		reportSearchProgress(progress, search.Progress{Current: visited, Step: "collecting"})
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	return nodes, nil
}

func recoverSearchIndexSwap(indexPath string) error {
	stagedPath := indexPath + ".next"
	backupPath := indexPath + ".previous"
	_, backupErr := os.Stat(backupPath)
	if backupErr == nil {
		if _, currentErr := os.Stat(indexPath); os.IsNotExist(currentErr) {
			if err := os.Rename(backupPath, indexPath); err != nil {
				return err
			}
		} else if currentErr != nil {
			return currentErr
		} else if err := os.RemoveAll(backupPath); err != nil {
			return err
		}
	} else if !os.IsNotExist(backupErr) {
		return backupErr
	}
	return os.RemoveAll(stagedPath)
}

func commitSearchIndexSwap(indexPath, stagedPath string) error {
	backupPath := indexPath + ".previous"
	hadCurrent := false
	if _, err := os.Stat(indexPath); err == nil {
		hadCurrent = true
		if err := os.Rename(indexPath, backupPath); err != nil {
			return err
		}
	} else if !os.IsNotExist(err) {
		return err
	}
	if err := os.Rename(stagedPath, indexPath); err != nil {
		if hadCurrent {
			_ = os.Rename(backupPath, indexPath)
		}
		return err
	}
	if hadCurrent {
		_ = os.RemoveAll(backupPath)
	}
	return nil
}

func reportSearchProgress(progress func(search.Progress), value search.Progress) {
	if progress != nil {
		progress(value)
	}
}

// 🔍️queryCacheIndex queries the owned index with cancellation and progress.
func queryCacheIndex(ctx context.Context, idx search.Index, query string, limit int, progress func(int, int)) ([]string, error) {
	if query == "" {
		return nil, nil
	}
	terms := strings.Fields(query)
	queries := make([]search.Query, 0, len(terms))
	for _, term := range terms {
		match := search.NewMatchQuery(term)
		match.SetFuzziness(2)
		queries = append(queries, match)
	}
	var q search.Query
	switch len(queries) {
	case 0:
		return nil, nil
	case 1:
		q = queries[0]
	default:
		q = search.NewConjunctionQuery(queries...)
	}
	req := search.NewSearchRequest(q)
	req.Size = limit
	results, err := idx.SearchContext(ctx, req, progress)
	if err != nil {
		return nil, err
	}
	var ids []string
	for _, hit := range results.Hits {
		ids = append(ids, hit.ID)
	}
	return ids, nil
}

// #endregion 🎊️Query Cache
