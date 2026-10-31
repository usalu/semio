package client

import (
	"context"
	"fmt"
	search "github.com/usalu/semio/repo/search"
	"path/filepath"
	"strconv"
)

// 🏢️ BuildMonorepoTreeCached preserves cancellation and errors through discovery and cache publication.
func BuildMonorepoTreeCached(ctx context.Context, opts ...TreeBuildOptions) (*TreeNode, error) {
	root, _, err := buildMonorepoTreeCaptured(ctx, opts...)
	return root, err
}

func buildMonorepoTreeCaptured(ctx context.Context, opts ...TreeBuildOptions) (*TreeNode, *treeSourceCapture, error) {
	if err := ctx.Err(); err != nil {
		return nil, nil, err
	}
	var options TreeBuildOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	capturedRoot := GetRootDir()
	reportSearchProgress(options.Progress, search.Progress{Step: "fingerprinting"})
	fingerprint, meta, err := computeCompositeFingerprintContext(ctx, capturedRoot, options.Progress)
	if err != nil {
		return nil, nil, err
	}
	if GetRootDir() != capturedRoot { return nil, nil, fmt.Errorf("%w: captured Source root changed", search.ErrCorrupt) }
	tree, err := search.LoadOrBuildSnapshot(ctx, search.SnapshotCacheRequest[*TreeNode]{
		Directory:   filepath.Join(getCacheDir(), "tree"),
		Fingerprint: fingerprint,
		Shape:       "authored-codebase:" + strconv.FormatBool(options.IncludeSections),
		Progress:    options.Progress,
		Build:       func(ctx context.Context) (*TreeNode, error) { return buildRepositoryCodebase(ctx, options) },
	})
	if err != nil {
		return nil, nil, err
	}
	projected, err := buildRepositoryDomains(ctx, tree, options)
	return projected, &treeSourceCapture{root: capturedRoot, fingerprint: fingerprint, metadata: meta}, err
}
