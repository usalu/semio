// 🐹️ Go side of the rename casings case. The Go twin renames real directory entries, so every
// workspace scenario materializes the recorded before-tree under the scenario work directory and
// reads the whole tree back afterwards.
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

// 🌳️ renameTree is one recorded workspace: its empty and populated directories and its files.
type renameTree struct {
	Directories []string          `json:"directories"`
	Files       map[string]string `json:"files"`
}

// 🔤️ renameVectors is the recorded vector set of this case.
type renameVectors struct {
	TokenVectors []struct {
		Name     string `json:"name"`
		Old      string `json:"old"`
		New      string `json:"new"`
		Content  string `json:"content"`
		Expected string `json:"expected"`
	} `json:"tokenVectors"`
	Trees []struct {
		Name     string         `json:"name"`
		Old      string         `json:"old"`
		New      string         `json:"new"`
		Scope    string         `json:"scope"`
		Before   renameTree     `json:"before"`
		After    renameTree     `json:"after"`
		Stats    map[string]int `json:"stats"`
		Messages []string       `json:"messages"`
	} `json:"trees"`
	Errors []struct {
		Name          string     `json:"name"`
		Old           string     `json:"old"`
		New           string     `json:"new"`
		Scope         string     `json:"scope"`
		Before        renameTree `json:"before"`
		ExpectedError string     `json:"expectedError"`
	} `json:"errors"`
}

// 📥️ vectors decodes the shared vector set of this case.
func vectors(ctx *host.Context) (*renameVectors, error) {
	raw, err := ctx.FixtureBytes("shared://🔤️rename-vectors.json")
	if err != nil {
		return nil, err
	}
	decoded := &renameVectors{}
	if err := json.Unmarshal(raw, decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

// endregion 🔖️Vectors

// region 🔖️Helpers

// 🗄️ materialize writes one recorded before-tree into a fresh directory the Go twin then owns.
func materialize(ctx *host.Context, name string, tree renameTree) (string, error) {
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

// 📊️ stats is the counter map a rename reports, always a map and never null.
func stats(result workspace.ToolResult) map[string]int {
	if counters, ok := result.Data.(map[string]int); ok {
		return counters
	}
	return map[string]int{}
}

// ↩️ reverseScope is the scope a reverse rename names: the same path with its last segment
// rewritten, because the forward rename renames the scope root itself whenever that root carries
// the token.
func reverseScope(scope string, old string, replacement string) string {
	if scope == "" {
		return ""
	}
	if index := strings.LastIndex(scope, "/"); index >= 0 {
		return scope[:index+1] + move.ApplyRenameCasings(scope[index+1:], old, replacement)
	}
	return move.ApplyRenameCasings(scope, old, replacement)
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

func everySpellingFoldsTheSameWay(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	rewrites := []any{}
	for _, vector := range file.TokenVectors {
		produced := move.ApplyRenameCasings(vector.Content, vector.Old, vector.New)
		if err := expect(vector.Name, "rewritten content", vector.Expected, produced); err != nil {
			return host.Outcome{}, err
		}
		rewrites = append(rewrites, map[string]any{"name": vector.Name, "rewritten": produced})
	}
	return host.Outcome{Projection: map[string]any{"rewrites": rewrites}}, nil
}

func aWorkspaceRenamesDeepestFirst(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	trees := []any{}
	for _, vector := range file.Trees {
		dir, err := materialize(ctx, "rename-casings", vector.Before)
		if err != nil {
			return host.Outcome{}, err
		}
		result := move.ToolRename(vector.Old, vector.New, vector.Scope)
		if result.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + result.Error)
		}
		after, err := render(dir)
		if err != nil {
			return host.Outcome{}, err
		}
		counters := stats(result)
		messages := lines(result)
		if err := expect(vector.Name, "workspace", map[string]any{"directories": vector.After.Directories, "files": vector.After.Files}, after); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "counters", vector.Stats, counters); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "output lines", vector.Messages, messages); err != nil {
			return host.Outcome{}, err
		}
		trees = append(trees, map[string]any{"name": vector.Name, "after": after, "stats": counters, "messages": messages})
	}
	return host.Outcome{Projection: map[string]any{"trees": trees}}, nil
}

func renamingBackRestoresTheWorkspace(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	roundTrips := []any{}
	for _, vector := range file.Trees {
		dir, err := materialize(ctx, "rename-casings-round-trip", vector.Before)
		if err != nil {
			return host.Outcome{}, err
		}
		forward := move.ToolRename(vector.Old, vector.New, vector.Scope)
		if forward.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + forward.Error)
		}
		back := move.ToolRename(vector.New, vector.Old, reverseScope(vector.Scope, vector.Old, vector.New))
		if back.Error != "" {
			return host.Outcome{}, errors.New(vector.Name + ": " + back.Error)
		}
		restored, err := render(dir)
		if err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "restored workspace", map[string]any{"directories": vector.Before.Directories, "files": vector.Before.Files}, restored); err != nil {
			return host.Outcome{}, err
		}
		roundTrips = append(roundTrips, map[string]any{"name": vector.Name, "restored": restored})
	}
	return host.Outcome{Projection: map[string]any{"roundTrips": roundTrips}}, nil
}

func aRefusedRenameNamesItsReason(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	refusals := []any{}
	for _, vector := range file.Errors {
		if _, err := materialize(ctx, "rename-casings-refused", vector.Before); err != nil {
			return host.Outcome{}, err
		}
		result := move.ToolRename(vector.Old, vector.New, vector.Scope)
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

// 🧭️ Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-spelling-folds-the-same-way", everySpellingFoldsTheSameWay).
		Subject("a-workspace-renames-deepest-first", aWorkspaceRenamesDeepestFirst).
		Subject("renaming-back-restores-the-workspace", renamingBackRestoresTheWorkspace).
		Subject("a-refused-rename-names-its-reason", aRefusedRenameNamesItsReason)
}

// endregion 🔖️Registration
