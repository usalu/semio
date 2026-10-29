package client

import (
	"context"
	search "github.com/usalu/semio/repo/search"
	"path/filepath"
	"strconv"
)

// 🏢️ BuildMonorepoTreeCached preserves cancellation and errors through discovery and cache publication.
func BuildMonorepoTreeCached(ctx context.Context, opts ...TreeBuildOptions) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	var options TreeBuildOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	reportSearchProgress(options.Progress, search.Progress{Step: "fingerprinting"})
	fingerprint, _, err := computeCompositeFingerprintContext(ctx, GetRootDir(), options.Progress)
	if err != nil {
		return nil, err
	}
	tree, err := search.LoadOrBuildSnapshot(ctx, search.SnapshotCacheRequest[*TreeNode]{
		Directory:   filepath.Join(getCacheDir(), "tree"),
		Fingerprint: fingerprint,
		Shape:       "authored-codebase:" + strconv.FormatBool(options.IncludeSections),
		Progress:    options.Progress,
		Build:       func(ctx context.Context) (*TreeNode, error) { return buildRepositoryCodebase(ctx, options) },
	})
	if err != nil {
		return nil, err
	}
	return buildRepositoryDomains(ctx, tree, options)
}
