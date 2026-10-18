// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Gzip-compressed tree cache.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"compress/gzip"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"time"
)

// 🌳️getTreeCachePath holds the data fields for a getTreeCachePath record.
func getTreeCachePath() string {
	return filepath.Join(getCacheDir(), "tree.json.gz")
}

func getTreeCacheMetaPath() string {
	return filepath.Join(getCacheDir(), "tree-meta.json")
}

// 💿️saveTreeCache holds the data fields for a saveTreeCache record.
func saveTreeCache(tree *TreeNode, meta *cacheMeta) error {
	dir := getCacheDir()
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	data, err := json.Marshal(tree)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	gz, err := gzip.NewWriterLevel(&buf, gzip.BestSpeed)
	if err != nil {
		return err
	}
	if _, err := gz.Write(data); err != nil {
		gz.Close()
		return err
	}
	if err := gz.Close(); err != nil {
		return err
	}
	if err := os.WriteFile(getTreeCachePath(), buf.Bytes(), 0644); err != nil {
		return err
	}
	metaData, err := json.MarshalIndent(meta, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(getTreeCacheMetaPath(), metaData, 0644)
}

func loadTreeCache() (*TreeNode, *cacheMeta, error) {
	metaData, err := os.ReadFile(getTreeCacheMetaPath())
	if err != nil {
		return nil, nil, err
	}
	var meta cacheMeta
	if err := json.Unmarshal(metaData, &meta); err != nil {
		return nil, nil, err
	}
	if meta.SchemaVersion != cacheSchemaVersion {
		return nil, nil, fmt.Errorf("schema version mismatch")
	}
	compressed, err := os.ReadFile(getTreeCachePath())
	if err != nil {
		return nil, nil, err
	}
	gz, err := gzip.NewReader(bytes.NewReader(compressed))
	if err != nil {
		return nil, nil, err
	}
	defer gz.Close()
	data, err := io.ReadAll(gz)
	if err != nil {
		return nil, nil, err
	}
	var tree TreeNode
	if err := json.Unmarshal(data, &tree); err != nil {
		return nil, nil, err
	}
	return &tree, &meta, nil
}

// 🏢️BuildMonorepoTreeCached holds the data fields for a BuildMonorepoTreeCached record.
func BuildMonorepoTreeCached(ctx context.Context, opts ...TreeBuildOptions) *TreeNode {
	var buildOpts TreeBuildOptions
	if len(opts) > 0 {
		buildOpts = opts[0]
	}
	repoRoot := GetRootDir()

	// Acquire a directory-based lock so only one CLI process builds the tree at a time.
	// Other concurrent processes will block here and then use the cache.
	lockPath := filepath.Join(getCacheDir(), "tree.lock")
	os.MkdirAll(filepath.Dir(lockPath), 0755)
	for {
		err := os.Mkdir(lockPath, 0755)
		if err == nil {
			break
		}
		// If the lock directory exists, another process holds the lock.
		// Check if it's stale (older than 60 seconds) and remove it.
		if info, statErr := os.Stat(lockPath); statErr == nil {
			if time.Since(info.ModTime()) > 60*time.Second {
				os.RemoveAll(lockPath)
				continue
			}
		}
		time.Sleep(100 * time.Millisecond)
	}
	defer os.RemoveAll(lockPath)

	fp, newMeta := computeCompositeFingerprint(repoRoot)
	newMeta.IncludeSections = buildOpts.IncludeSections
	cached, cachedMeta, err := loadTreeCache()
	if err == nil && cachedMeta.Fingerprint == fp && cachedMeta.IncludeSections == buildOpts.IncludeSections {
		return cached
	}
	tree := BuildMonorepoTree(ctx, buildOpts)
	saveTreeCache(tree, newMeta)
	return tree
}
