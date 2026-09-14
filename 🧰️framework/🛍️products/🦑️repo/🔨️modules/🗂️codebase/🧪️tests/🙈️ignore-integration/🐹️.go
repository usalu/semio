// 🐹️ Go side of the ignore integration case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

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

type ignoreVector struct {
	Name string `json:"name"`
	Path string `json:"path"`
}

type ignoreVectorFile struct {
	Schema  string         `json:"schema"`
	Vectors []ignoreVector `json:"vectors"`
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

func everyVectorGetsTheSameThreeVerdicts(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	data, err := ctx.FixtureBytes("shared://📡️ignore-vectors.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var file ignoreVectorFile
	if err := json.Unmarshal(data, &file); err != nil {
		return host.Outcome{}, err
	}
	verdicts := []string{}
	for _, vector := range file.Vectors {
		verdicts = append(verdicts, fmt.Sprintf("%s=%t,%t,%t", vector.Name, workspace.IsRepoExcludedPath(vector.Path), workspace.IsGitIgnored(vector.Path), model.IsGeneratedFolder(vector.Path)))
	}
	return host.Outcome{Projection: map[string]any{"verdicts": verdicts}}, nil
}

func filteringAWalkDropsTheSamePaths(ctx *host.Context) (host.Outcome, error) {
	root, err := materializeTree(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	raw, err := workspace.GlobByExtension(root, "**/*", []string{"ts", "tsx", "py", "cs", "go", "rs"}, []string{"**/node_modules/**", "**/.venv/**"}, true)
	if err != nil {
		return host.Outcome{}, err
	}
	kept := model.FilterConsideredFiles(raw)
	keptSet := make(map[string]struct{}, len(kept))
	for _, path := range kept {
		keptSet[path] = struct{}{}
	}
	dropped := []string{}
	for _, path := range raw {
		if _, ok := keptSet[path]; !ok {
			dropped = append(dropped, path)
		}
	}
	return host.Outcome{Projection: map[string]any{"kept": kept, "dropped": dropped}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-vector-gets-the-same-three-verdicts", everyVectorGetsTheSameThreeVerdicts).
		Subject("filtering-a-walk-drops-the-same-paths", filteringAWalkDropsTheSamePaths)
}

// endregion 🔖️Registration
