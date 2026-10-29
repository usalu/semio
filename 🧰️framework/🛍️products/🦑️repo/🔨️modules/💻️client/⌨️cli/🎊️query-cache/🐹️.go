// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎊️query-cache carries the search query cache: fingerprints, change detection, index swaps and cached queries.

// #endregion 🧲️Header

package client

import (
	bytes "bytes"
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	search "github.com/usalu/semio/repo/search"
	io "io"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	sync "sync"
)

// #region 🎊️Query Cache

// 🗄️Local event index under .🦑️repo/⚡️cache for keyword search. Uses a composite repository fingerprint for invalidation.
// 📌️cacheSchemaVersion holds the data fields for a cacheSchemaVersion record.
const cacheSchemaVersion = 6

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
	Scope             string            `json:"Scope"`
	IncludeSections   bool              `json:"IncludeSections"`
}

// 💾️getCacheDir holds the data fields for a getCacheDir record.
func getCacheDir() string {
	abs, _ := filepath.Abs(GetRootDir())
	h := sha256.Sum256([]byte(abs))
	return filepath.Join(GetRepoMetaDir(), "⚡️cache", hex.EncodeToString(h[:]))
}

func computeCompositeFingerprintContext(ctx context.Context, repoRoot string, progress func(search.Progress)) (fp string, meta *cacheMeta, err error) {
	meta = &cacheMeta{
		SchemaVersion:     cacheSchemaVersion,
		SubmodulePointers: map[string]string{},
		SubmoduleHeads:    map[string]string{},
		SubmoduleDirty:    map[string]string{},
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	stdout, _, _ := ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SuperHead = strings.TrimSpace(stdout)
	statusOut, _, _ := ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, repoRoot)
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
		subHead, _, _ := ExecCommandContext(ctx, "git", []string{"rev-parse", "HEAD"}, subPath)
		meta.SubmoduleHeads[path] = strings.TrimSpace(subHead)
		subStatus, _, _ := ExecCommandContext(ctx, "git", []string{"status", "--porcelain", "-z", "--untracked-files=no"}, subPath)
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
	sourceDigest, sourceError := search.FingerprintSource(ctx, repoRoot, isRepoExcludedPath, progress)
	if sourceError != nil {
		return "", nil, sourceError
	}
	if err := ctx.Err(); err != nil {
		return "", nil, err
	}
	meta.SubWorkingHash = hashString(strings.Join(subWork, "|"))
	fp = hashString(meta.SuperHead + meta.SuperDirtyHash + meta.PointersHash + meta.SubWorkingHash + sourceDigest + strconv.Itoa(cacheSchemaVersion))
	meta.Fingerprint = fp
	return fp, meta, nil
}

// 🧭️getSubmoduleStatus returns recursive submodule status when available.

func getSubmoduleStatusContext(ctx context.Context, repoRoot string) (string, error) {
	stdout, _, exitCode := ExecCommandContext(ctx, "git", []string{"submodule", "status", "--recursive"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if exitCode == 0 {
		return stdout, nil
	}
	fallback, _, fallbackExitCode := ExecCommandContext(ctx, "git", []string{"submodule", "status"}, repoRoot)
	if err := ctx.Err(); err != nil {
		return "", err
	}
	if fallbackExitCode == 0 {
		return fallback, nil
	}
	return stdout, nil
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
	info, err := os.Lstat(p)
	if err != nil {
		return nil, err
	}
	if !info.Mode().IsRegular() || info.Mode()&os.ModeSymlink != 0 {
		return nil, fmt.Errorf("%w: ordinary search metadata required", search.ErrCorrupt)
	}
	file, err := os.Open(p)
	if err != nil {
		return nil, err
	}
	opened, err := file.Stat()
	if err != nil || !os.SameFile(info, opened) {
		file.Close()
		return nil, fmt.Errorf("%w: search metadata identity changed", search.ErrCorrupt)
	}
	data, err := io.ReadAll(io.LimitReader(file, search.MaxManifestBytes+1))
	closing := file.Close()
	if err != nil || closing != nil {
		return nil, errors.Join(err, closing)
	}
	if len(data) > search.MaxManifestBytes {
		return nil, fmt.Errorf("%w: search metadata byte budget", search.ErrCorrupt)
	}
	var m cacheMeta
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err = decoder.Decode(&m); err != nil {
		return nil, err
	}
	if err = decoder.Decode(new(interface{})); err != io.EOF {
		return nil, fmt.Errorf("%w: trailing search metadata", search.ErrCorrupt)
	}
	if m.SchemaVersion != cacheSchemaVersion {
		return nil, os.ErrNotExist
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
	if len(data) > search.MaxManifestBytes {
		return fmt.Errorf("%w: search metadata byte budget", search.ErrTooLarge)
	}
	file, err := os.OpenFile(filepath.Join(dir, "meta.json"), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
	if err != nil {
		return err
	}
	count, writeErr := file.Write(data)
	if writeErr == nil && count != len(data) {
		writeErr = io.ErrShortWrite
	}
	syncErr := file.Sync()
	return errors.Join(writeErr, syncErr, file.Close())
}

// 🛤️getChangedPathsFromGit holds the data fields for a getChangedPathsFromGit record.

// 🔹️expandPathsWithAncestors holds the data fields for a expandPathsWithAncestors record.
func expandPathsWithAncestors(paths []string) []string {
	technologies := LoadTechnologies()
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

// 🔸️ensureCacheIndexed builds a bounded staged index and atomically commits it after verification.
func ensureCacheIndexed(ctx context.Context, root *TreeNode, scope string, progress func(search.Progress)) (search.Index, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	repoRoot := GetRootDir()
	reportSearchProgress(progress, search.Progress{Step: "fingerprinting"})
	fingerprint, newMeta, err := computeCompositeFingerprintContext(ctx, repoRoot, progress)
	if err != nil {
		return nil, err
	}
	projectionDigest, err := fingerprintSearchProjection(ctx, root)
	if err != nil {
		return nil, err
	}
	fingerprint = hashString(fingerprint + projectionDigest)
	newMeta.Fingerprint = fingerprint
	includesSections, err := treeIncludesSections(ctx, root)
	if err != nil {
		return nil, err
	}
	newMeta.IncludeSections = includesSections
	newMeta.Scope = scope
	if len(scope) > 4096 {
		return nil, fmt.Errorf("%w: search scope bytes", search.ErrTooLarge)
	}
	dir := getCacheDir()
	if scope != "" {
		dir = filepath.Join(dir, "scope-"+hashString(scope))
	}
	indexPath := filepath.Join(dir, "index.search")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}
	readLease, err := search.AcquireGenerationLease(ctx, filepath.Join(dir, ".lock"), true, progress)
	if err != nil {
		return nil, err
	}
	release := readLease.Close
	returned := false
	defer func() {
		if !returned {
			_ = release()
		}
	}()
	retain := func(index search.Index) search.Index {
		returned = true
		return &lockedSearchIndex{heldSearchIndex: index, release: release}
	}
	warm, warmErr := loadCacheMeta(indexPath)
	pending := false
	for _, path := range []string{indexPath + ".next", indexPath + ".previous"} {
		if _, err := os.Lstat(path); err == nil {
			pending = true
		} else if !os.IsNotExist(err) {
			return nil, err
		}
	}
	if warmErr == nil && warm.Fingerprint == fingerprint && warm.Scope == scope && warm.IncludeSections == includesSections && !pending {
		index, err := search.OpenShardedContext(ctx, indexPath, nil)
		if err != nil {
			return nil, fmt.Errorf("open current search index: %w", err)
		}
		reportSearchProgress(progress, search.Progress{Current: 1, Total: 1, Step: "ready"})
		return retain(index), nil
	}
	if err = release(); err != nil {
		return nil, err
	}
	release, err = acquireSearchCacheLock(ctx, filepath.Join(dir, ".lock"), progress)
	if err != nil {
		release = func() error { return nil }
		return nil, err
	}
	if err := search.RecoverGenerationSwap(indexPath); err != nil {
		return nil, err
	}
	existing, metaErr := loadCacheMeta(indexPath)
	if metaErr == nil && existing.Fingerprint == fingerprint && existing.Scope == scope && existing.IncludeSections == includesSections {
		index, err := search.OpenShardedContext(ctx, indexPath, nil)
		if err != nil {
			return nil, fmt.Errorf("open current search index: %w", err)
		}
		reportSearchProgress(progress, search.Progress{Current: 1, Total: 1, Step: "ready"})
		return retain(index), nil
	}
	if metaErr != nil && !os.IsNotExist(metaErr) {
		return nil, fmt.Errorf("load search index metadata: %w", metaErr)
	}
	stagedPath := indexPath + ".next"
	if err := os.RemoveAll(stagedPath); err != nil {
		return nil, err
	}
	staged, err := search.NewSharded(stagedPath, hashString(fingerprint+":"+scope+":"+strconv.FormatBool(includesSections)))
	if err != nil {
		return nil, err
	}
	defer func() { _ = staged.Abort(); _ = os.RemoveAll(stagedPath) }()
	cleanup := func(failure error) error { return errors.Join(failure, staged.Abort(), os.RemoveAll(stagedPath)) }
	indexed, err := streamTreeSearchIndex(ctx, root, staged, progress)
	if err != nil {
		return nil, cleanup(err)
	}
	if err := ctx.Err(); err != nil {
		return nil, cleanup(err)
	}
	if err := staged.CloseContext(ctx, func(value search.Progress) {
		value.Step = "persisting-" + value.Step
		reportSearchProgress(progress, value)
	}); err != nil {
		return nil, cleanup(fmt.Errorf("persist staged search index: %w", err))
	}
	if err := saveCacheMeta(stagedPath, newMeta); err != nil {
		return nil, cleanup(err)
	}
	verified, err := search.OpenShardedContext(ctx, stagedPath, nil)
	if err != nil {
		return nil, cleanup(fmt.Errorf("verify staged search index: %w", err))
	}
	if err := verified.Close(); err != nil {
		return nil, cleanup(err)
	}
	if err := ctx.Err(); err != nil {
		return nil, cleanup(err)
	}
	if err := search.CommitGenerationSwap(indexPath, stagedPath); err != nil {
		return nil, cleanup(err)
	}
	reportSearchProgress(progress, search.Progress{Current: indexed, Total: indexed, Step: "committed"})
	index, err := search.OpenShardedContext(ctx, indexPath, nil)
	if err != nil {
		return nil, fmt.Errorf("open committed search index: %w", err)
	}
	return retain(index), nil
}

func acquireSearchCacheLock(ctx context.Context, path string, progress func(search.Progress)) (func() error, error) {
	lease, err := search.AcquireGenerationLease(ctx, path, false, progress)
	if err != nil {
		return nil, err
	}
	return lease.Close, nil
}

// 🧩️treeIncludesSections reports whether the tree carries section or definition nodes, the part of a tree an index must not be reused without.
func treeIncludesSections(ctx context.Context, root *TreeNode) (bool, error) {
	found := false
	stop := errors.New("sections found")
	err := walkSearchTree(ctx, root, func(node *TreeNode, depth int) error {
		if node.Kind == TreeNodeSection || node.Kind == TreeNodeDefinition {
			found = true
			return stop
		}
		return nil
	})
	if errors.Is(err, stop) {
		return true, nil
	}
	return found, err
}

// 🚶️ Visits a tree with one frame per ancestor and rejects cyclic or overdeep inputs.
func walkSearchTree(ctx context.Context, root *TreeNode, visit func(*TreeNode, int) error) error {
	if root == nil {
		return errors.New("search tree is required")
	}
	type frame struct {
		node    *TreeNode
		child   int
		entered bool
	}
	stack := []frame{{node: root}}
	ancestors := map[*TreeNode]bool{}
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return err
		}
		current := &stack[len(stack)-1]
		if !current.entered {
			if current.node == nil {
				return errors.New("nil search tree node")
			}
			if ancestors[current.node] {
				return errors.New("cyclic search tree")
			}
			if len(stack) > search.MaxTraversalDepth {
				return fmt.Errorf("%w: search tree depth", search.ErrTooLarge)
			}
			ancestors[current.node] = true
			current.entered = true
			if err := visit(current.node, len(stack)); err != nil {
				return err
			}
		}
		if current.child < len(current.node.Children) {
			child := current.node.Children[current.child]
			current.child++
			stack = append(stack, frame{node: child})
			continue
		}
		delete(ancestors, current.node)
		stack = stack[:len(stack)-1]
	}
	return ctx.Err()
}

func streamTreeSearchIndex(ctx context.Context, root *TreeNode, staged *search.ShardedIndex, progress func(search.Progress)) (int, error) {
	indexed := 0
	err := walkSearchTree(ctx, root, func(node *TreeNode, depth int) error {
		reportSearchProgress(progress, search.Progress{Current: indexed + 1, Step: "collecting"})
		if err := ctx.Err(); err != nil {
			return err
		}
		if node.Kind == TreeNodeCategory {
			return nil
		}
		id, document := treeSearchDocument(node)
		if err := staged.IndexContext(ctx, id, document, nil); err != nil {
			return fmt.Errorf("index %q: %w", id, err)
		}
		indexed++
		reportSearchProgress(progress, search.Progress{Current: indexed, Step: "indexed"})
		return ctx.Err()
	})
	return indexed, err
}

func searchNodeAttributes(node *TreeNode) map[string]string {
	return map[string]string{
		"kind": string(node.Kind), "subkind": node.SubKind, "year": strconv.Itoa(node.Year), "month": strconv.Itoa(node.Month), "day": strconv.Itoa(node.Day), "status": node.Status, "contributor": node.Contributor,
	}
}

type heldSearchIndex = search.Index
type lockedSearchIndex struct {
	heldSearchIndex
	release func() error
	once    sync.Once
	err     error
}

func (index *lockedSearchIndex) Close() error { return index.CloseContext(context.Background(), nil) }
func (index *lockedSearchIndex) CloseContext(ctx context.Context, progress func(search.Progress)) error {
	index.once.Do(func() {
		index.err = errors.Join(index.heldSearchIndex.CloseContext(context.Background(), progress), index.release())
	})
	return errors.Join(ctx.Err(), index.err)
}

func reportSearchProgress(progress func(search.Progress), value search.Progress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion 🎊️Query Cache

// 🔏️fingerprintSearchProjection binds cached index identity to the current CQRS projection.
func fingerprintSearchProjection(ctx context.Context, root *TreeNode) (string, error) {
	digest := sha256.New()
	err := walkSearchTree(ctx, root, func(node *TreeNode, depth int) error {
		if node.Kind == TreeNodeCategory {
			return nil
		}
		id, document := treeSearchDocument(node)
		data, err := json.Marshal(struct {
			ID       string
			Document map[string]interface{}
		}{id, document})
		if err != nil {
			return err
		}
		if len(data) > search.MaxDocumentBytes {
			return fmt.Errorf("%w: projection document", search.ErrTooLarge)
		}
		fmt.Fprintf(digest, "%d:", len(data))
		_, err = digest.Write(data)
		return err
	})
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(digest.Sum(nil)), ctx.Err()
}

func treeSearchDocument(node *TreeNode) (string, map[string]interface{}) {
	id := node.ID
	if id == "" {
		id = node.Label
	}
	document := map[string]interface{}{"text": buildSearchDocumentText(node), "attributes": searchNodeAttributes(node)}
	if path := treeNodeScopePath(node); path != "" {
		document["path"] = path
	}
	return id, document
}
