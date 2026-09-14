// 🐹️ Go side of the technology and bundle detection case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Fixture

type treeEntry struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type repoTree struct {
	Schema  string      `json:"schema"`
	Entries []treeEntry `json:"entries"`
}

// materializeTree writes the committed tree into a private directory, points the repository root at
// it and drops every cache the global-root Go implementation keeps.
func materializeTree(ctx *host.Context) (string, error) {
	data, err := ctx.FixtureBytes("shared://📡️repo-tree.json")
	if err != nil {
		return "", err
	}
	var tree repoTree
	if err := json.Unmarshal(data, &tree); err != nil {
		return "", err
	}
	root := filepath.Join(ctx.WorkDir, "🌳️tree")
	if err := os.RemoveAll(root); err != nil {
		return "", err
	}
	for _, entry := range tree.Entries {
		path := filepath.Join(root, filepath.FromSlash(entry.Path))
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			return "", err
		}
		if err := os.WriteFile(path, []byte(entry.Content), 0o644); err != nil {
			return "", err
		}
	}
	workspace.SetRootDir(root)
	codebase.InvalidateTechnologyCache()
	return root, nil
}

// endregion 🔖️Fixture

// region 🔖️Scenarios

func technologiesAndBundlesAgree(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	technologies := []string{}
	bundles := []string{}
	for _, technology := range codebase.LoadTechnologies() {
		technologies = append(technologies, fmt.Sprintf("%s|%s|%s|%s", technology.Name, technology.Root, technology.Kind, technology.Emoji))
		for _, bundle := range technology.Bundles {
			bundles = append(bundles, fmt.Sprintf("%s|%s|%s|%s|%s|%s", bundle.Name, bundle.Root, bundle.Kind, bundle.Emoji, bundle.SourceRoot, strings.Join(bundle.Tags, ",")))
		}
	}
	return host.Outcome{Projection: map[string]any{"technologies": technologies, "bundles": bundles}}, nil
}

func bundleIDsAndLabelsAgree(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	loaded := codebase.LoadBundles()
	bundles := []string{}
	for index := range loaded {
		bundle := &loaded[index]
		bundles = append(bundles, fmt.Sprintf("%s|%s|%s", bundle.Name, bundle.GetID(), model.NormalizeBundleLabel(bundle.Name)))
	}
	return host.Outcome{Projection: map[string]any{"bundles": bundles}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("technologies-and-bundles-agree", technologiesAndBundlesAgree).
		Subject("bundle-ids-and-labels-agree", bundleIDsAndLabelsAgree)
}

// endregion 🔖️Registration
