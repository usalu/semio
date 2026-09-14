// 🐹️ Go side of the section move case. The Go twin edits a real directory tree, so every scenario
// materializes the recorded file under the scenario work directory and reads it back.
package adapter

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"

	move "github.com/usalu/semio/repo/move"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📑️ sectionMoveVectors is the recorded vector set of this case.
type sectionMoveVectors struct {
	Cases []struct {
		Name     string   `json:"name"`
		File     string   `json:"file"`
		Content  string   `json:"content"`
		OldPath  string   `json:"oldPath"`
		NewPath  string   `json:"newPath"`
		Expected string   `json:"expected"`
		Messages []string `json:"messages"`
	} `json:"cases"`
	Errors []struct {
		Name          string `json:"name"`
		File          string `json:"file"`
		OldPath       string `json:"oldPath"`
		NewPath       string `json:"newPath"`
		ExpectedError string `json:"expectedError"`
	} `json:"errors"`
}

// 📥️ vectors decodes the shared vector set of this case.
func vectors(ctx *host.Context) (*sectionMoveVectors, error) {
	raw, err := ctx.FixtureBytes("shared://📑️section-move-trees.json")
	if err != nil {
		return nil, err
	}
	decoded := &sectionMoveVectors{}
	if err := json.Unmarshal(raw, decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// endregion 🔖️Vectors

// region 🔖️Helpers

// 🗄️ root points the Go twin at a fresh empty directory and returns it.
func root(ctx *host.Context, name string) (string, error) {
	dir, err := os.MkdirTemp(ctx.WorkDir, name)
	if err != nil {
		return "", err
	}
	workspace.RootDir = dir
	return dir, nil
}

// ✏️ write materializes one file of a recorded workspace.
func write(dir string, path string, content string) error {
	absolute := filepath.Join(dir, filepath.FromSlash(path))
	if err := os.MkdirAll(filepath.Dir(absolute), 0o755); err != nil {
		return err
	}
	return os.WriteFile(absolute, []byte(content), 0o644)
}

// 📣️ lines is the command output of a tool result, always a list and never null.
func lines(result workspace.ToolResult) []string {
	out := []string{}
	for _, line := range result.Output.Lines {
		out = append(out, line.Text)
	}
	return out
}

// 🎬️ rename applies a section rename to a one-file workspace and returns the new content.
func rename(ctx *host.Context, file string, content string, old string, renamed string) (string, []string, error) {
	dir, err := root(ctx, "section-move")
	if err != nil {
		return "", nil, err
	}
	if err := write(dir, file, content); err != nil {
		return "", nil, err
	}
	result := move.ToolSectionMove(file, old, renamed)
	if result.Error != "" {
		return "", nil, errors.New(result.Error)
	}
	produced, err := os.ReadFile(filepath.Join(dir, filepath.FromSlash(file)))
	if err != nil {
		return "", nil, err
	}
	return string(produced), lines(result), nil
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

func markersFollowTheNewName(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	renamed := []any{}
	for _, vector := range file.Cases {
		content, messages, err := rename(ctx, vector.File, vector.Content, vector.OldPath, vector.NewPath)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		if err := expect(vector.Name, "renamed file", vector.Expected, content); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "output lines", vector.Messages, messages); err != nil {
			return host.Outcome{}, err
		}
		renamed = append(renamed, map[string]any{"name": vector.Name, "content": content, "messages": messages})
	}
	return host.Outcome{Projection: map[string]any{"renamed": renamed}}, nil
}

func renamingBackRestoresTheFile(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	roundTrips := []any{}
	for _, vector := range file.Cases {
		forward, _, err := rename(ctx, vector.File, vector.Content, vector.OldPath, vector.NewPath)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		restored, _, err := rename(ctx, vector.File, forward, vector.NewPath, vector.OldPath)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		if err := expect(vector.Name, "restored file", vector.Content, restored); err != nil {
			return host.Outcome{}, err
		}
		roundTrips = append(roundTrips, map[string]any{"name": vector.Name, "restored": restored})
	}
	return host.Outcome{Projection: map[string]any{"roundTrips": roundTrips}}, nil
}

func aMissingFileIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	dir, err := root(ctx, "section-move-errors")
	if err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "present.ts", "// #region 🔖️Alpha\n// #endregion 🔖️Alpha\n"); err != nil {
		return host.Outcome{}, err
	}
	refusals := []any{}
	for _, vector := range file.Errors {
		result := move.ToolSectionMove(vector.File, vector.OldPath, vector.NewPath)
		if result.Error == "" {
			return host.Outcome{}, errors.New(vector.Name + ": the rename was accepted but the vector records a refusal")
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
		Subject("markers-follow-the-new-name", markersFollowTheNewName).
		Subject("renaming-back-restores-the-file", renamingBackRestoresTheFile).
		Subject("a-missing-file-is-refused", aMissingFileIsRefused)
}

// endregion 🔖️Registration
