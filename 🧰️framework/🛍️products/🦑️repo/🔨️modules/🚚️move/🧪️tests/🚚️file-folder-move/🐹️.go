// 🐹️ Go side of the file and folder move case. The Go twin moves real directory entries, so every
// scenario materializes the recorded before-tree under the scenario work directory and reads the
// whole tree back afterwards.
package adapter

import (
	"encoding/json"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"

	move "github.com/usalu/semio/repo/move"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 🌳️ moveTree is one recorded workspace: its empty and populated directories and its files.
type moveTree struct {
	Directories []string          `json:"directories"`
	Files       map[string]string `json:"files"`
}

// 🚚️ fileFolderMoveVectors is the recorded vector set of this case.
type fileFolderMoveVectors struct {
	Cases []struct {
		Name     string   `json:"name"`
		Kind     string   `json:"kind"`
		Source   string   `json:"source"`
		Target   string   `json:"target"`
		Before   moveTree `json:"before"`
		After    moveTree `json:"after"`
		Messages []string `json:"messages"`
	} `json:"cases"`
	Errors []struct {
		Name          string `json:"name"`
		Kind          string `json:"kind"`
		Source        string `json:"source"`
		Target        string `json:"target"`
		ExpectedError string `json:"expectedError"`
	} `json:"errors"`
}

// 📥️ vectors decodes the shared vector set of this case.
func vectors(ctx *host.Context) (*fileFolderMoveVectors, error) {
	raw, err := ctx.FixtureBytes("shared://🚚️file-folder-move-trees.json")
	if err != nil {
		return nil, err
	}
	decoded := &fileFolderMoveVectors{}
	if err := json.Unmarshal(raw, decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// endregion 🔖️Vectors

// region 🔖️Helpers

// 🗄️ materialize writes one recorded before-tree into a fresh directory the Go twin then owns.
func materialize(ctx *host.Context, name string, tree moveTree) (string, error) {
	dir, err := os.MkdirTemp(ctx.WorkDir, name)
	if err != nil {
		return "", err
	}
	workspace.RootDir = dir
	for _, directory := range tree.Directories {
		if err := os.MkdirAll(filepath.Join(dir, filepath.FromSlash(directory)), 0o755); err != nil {
			return "", err
		}
	}
	for path, content := range tree.Files {
		absolute := filepath.Join(dir, filepath.FromSlash(path))
		if err := os.MkdirAll(filepath.Dir(absolute), 0o755); err != nil {
			return "", err
		}
		if err := os.WriteFile(absolute, []byte(content), 0o644); err != nil {
			return "", err
		}
	}
	return dir, nil
}

// 🧾️ render reads a whole directory tree back the way every implementation of this case reports it.
func render(dir string) (map[string]any, error) {
	directories := []string{}
	files := map[string]string{}
	err := filepath.WalkDir(dir, func(path string, entry fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		relative, err := filepath.Rel(dir, path)
		if err != nil {
			return err
		}
		if relative == "." {
			return nil
		}
		slashed := strings.ReplaceAll(relative, "\\", "/")
		if entry.IsDir() {
			directories = append(directories, slashed)
			return nil
		}
		content, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		files[slashed] = string(content)
		return nil
	})
	if err != nil {
		return nil, err
	}
	sort.Strings(directories)
	return map[string]any{"directories": directories, "files": files}, nil
}

// 📣️ lines is the command output of a tool result, always a list and never null.
func lines(result workspace.ToolResult) []string {
	out := []string{}
	for _, line := range result.Output.Lines {
		out = append(out, line.Text)
	}
	return out
}

// 🚚️ apply performs one move of the recorded kind against the directory the twin currently owns.
func apply(kind string, source string, target string) workspace.ToolResult {
	if kind == "folder" {
		return move.ToolFolderMove(source, target)
	}
	return move.ToolFileMove(source, target)
}

// ⚖️ expect fails the scenario when the produced value is not the recorded one.
func expect(name string, field string, recorded any, produced any) error {
	wanted, err := json.Marshal(recorded)
	if err != nil {
		return err
	}
	got, err := json.Marshal(produced)
	if err != nil {
		return err
	}
	if string(wanted) == string(got) {
		return nil
	}
	return errors.New(name + ": " + field + " differs from the recorded expectation\nrecorded: " + string(wanted) + "\nproduced: " + string(got))
}

// endregion 🔖️Helpers

// region 🔖️Scenarios

func aMoveCarriesTheSubtreeAndTheDocs(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	moves := []any{}
	for _, vector := range file.Cases {
		dir, err := materialize(ctx, "file-folder-move", vector.Before)
		if err != nil {
			return host.Outcome{}, err
		}
		result := apply(vector.Kind, vector.Source, vector.Target)
		if result.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + result.Error)
		}
		after, err := render(dir)
		if err != nil {
			return host.Outcome{}, err
		}
		messages := lines(result)
		if err := expect(vector.Name, "workspace", map[string]any{"directories": vector.After.Directories, "files": vector.After.Files}, after); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "output lines", vector.Messages, messages); err != nil {
			return host.Outcome{}, err
		}
		moves = append(moves, map[string]any{"name": vector.Name, "after": after, "messages": messages})
	}
	return host.Outcome{Projection: map[string]any{"moves": moves}}, nil
}

func movingBackRestoresTheTree(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	roundTrips := []any{}
	for _, vector := range file.Cases {
		dir, err := materialize(ctx, "file-folder-move-round-trip", vector.Before)
		if err != nil {
			return host.Outcome{}, err
		}
		original, err := render(dir)
		if err != nil {
			return host.Outcome{}, err
		}
		forward := apply(vector.Kind, vector.Source, vector.Target)
		if forward.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + forward.Error)
		}
		backward := apply(vector.Kind, vector.Target, vector.Source)
		if backward.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + backward.Error)
		}
		restored, err := render(dir)
		if err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "restored workspace", original, restored); err != nil {
			return host.Outcome{}, err
		}
		roundTrips = append(roundTrips, map[string]any{"name": vector.Name, "restored": restored})
	}
	return host.Outcome{Projection: map[string]any{"roundTrips": roundTrips}}, nil
}

func anOccupiedTargetIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	if len(file.Cases) == 0 {
		return host.Outcome{}, errors.New("the vector set carries no case")
	}
	if _, err := materialize(ctx, "file-folder-move-errors", file.Cases[0].Before); err != nil {
		return host.Outcome{}, err
	}
	refusals := []any{}
	for _, vector := range file.Errors {
		result := apply(vector.Kind, vector.Source, vector.Target)
		if result.Error == "" {
			return host.Outcome{}, errors.New(vector.Name + ": the move was accepted but the vector records a refusal")
		}
		if err := expect(vector.Name, "refusal message", vector.ExpectedError, result.Error); err != nil {
			return host.Outcome{}, err
		}
		refusals = append(refusals, map[string]any{"name": vector.Name, "message": result.Error})
	}
	return host.Outcome{Projection: map[string]any{"refusals": refusals}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("a-move-carries-the-subtree-and-the-docs", aMoveCarriesTheSubtreeAndTheDocs).
		Subject("moving-back-restores-the-tree", movingBackRestoresTheTree).
		Subject("an-occupied-target-is-refused", anOccupiedTargetIsRefused)
}

// endregion 🔖️Registration
