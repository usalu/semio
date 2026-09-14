// 🐹️ Go side of the artifact id builders case.
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

type idVector struct {
	Name           string   `json:"name"`
	Kind           string   `json:"kind"`
	Path           string   `json:"path"`
	SectionPath    []string `json:"sectionPath"`
	DefinitionName string   `json:"definitionName"`
	DefinitionKind string   `json:"definitionKind"`
}

type idVectorFile struct {
	Schema  string     `json:"schema"`
	Vectors []idVector `json:"vectors"`
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

func loadIDVectors(ctx *host.Context) (idVectorFile, error) {
	data, err := ctx.FixtureBytes("shared://📡️artifact-id-vectors.json")
	if err != nil {
		return idVectorFile{}, err
	}
	var file idVectorFile
	err = json.Unmarshal(data, &file)
	return file, err
}

func definitionKindOf(raw string) model.DefinitionKind {
	switch raw {
	case "interface":
		return model.DefinitionKindInterface
	case "constant":
		return model.DefinitionKindConstant
	case "test":
		return model.DefinitionKindTest
	}
	return model.DefinitionKindImplementation
}

// endregion 🔖️Fixture

// region 🔖️Scenarios

func idsAgreeForEveryVector(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	file, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	ids := []string{}
	for _, vector := range file.Vectors {
		var id string
		switch vector.Kind {
		case "folder":
			id = model.BuildFolderID(vector.Path, nil)
		case "file":
			id = model.BuildFileID(vector.Path, nil)
		case "section":
			id = model.BuildSectionID(model.BuildFileID(vector.Path, nil), vector.SectionPath)
		case "definition":
			id = model.BuildDefinitionID(model.BuildFileID(vector.Path, nil), vector.SectionPath, vector.DefinitionName, definitionKindOf(vector.DefinitionKind))
		default:
			return host.Outcome{}, fmt.Errorf("vector %s names an unknown id kind %s", vector.Name, vector.Kind)
		}
		ids = append(ids, fmt.Sprintf("%s=%s", vector.Name, id))
	}
	return host.Outcome{Projection: map[string]any{"ids": ids}}, nil
}

func urisAgreeForEveryFileVector(ctx *host.Context) (host.Outcome, error) {
	if _, err := materializeTree(ctx); err != nil {
		return host.Outcome{}, err
	}
	file, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	uris := []string{}
	for _, vector := range file.Vectors {
		if vector.Kind != "file" {
			continue
		}
		uris = append(uris, fmt.Sprintf("%s=%s", vector.Name, model.BuildFileUriFromPath(vector.Path)))
	}
	return host.Outcome{Projection: map[string]any{"uris": uris}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("ids-agree-for-every-vector", idsAgreeForEveryVector).
		Subject("uris-agree-for-every-file-vector", urisAgreeForEveryFileVector)
}

// endregion 🔖️Registration
