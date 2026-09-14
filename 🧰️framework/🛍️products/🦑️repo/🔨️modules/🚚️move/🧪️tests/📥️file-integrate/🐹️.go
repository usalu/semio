// 🐹️ Go side of the file integrate case. The Go twin edits a real directory tree, so every scenario
// materializes the two recorded files under the scenario work directory and reads the target back.
package adapter

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 📥️ fileIntegrateVector is one recorded integration.
type fileIntegrateVector struct {
	Name          string   `json:"name"`
	SourceFile    string   `json:"sourceFile"`
	SourceContent string   `json:"sourceContent"`
	TargetFile    string   `json:"targetFile"`
	TargetContent string   `json:"targetContent"`
	TargetSection string   `json:"targetSection"`
	ParentSection string   `json:"parentSection"`
	Expected      string   `json:"expected"`
	Messages      []string `json:"messages"`
}

// 📥️ fileIntegrateVectors is the recorded vector set of this case.
type fileIntegrateVectors struct {
	Cases  []fileIntegrateVector `json:"cases"`
	Errors []struct {
		Name          string `json:"name"`
		SourceFile    string `json:"sourceFile"`
		TargetFile    string `json:"targetFile"`
		TargetSection string `json:"targetSection"`
		ParentSection string `json:"parentSection"`
		ExpectedError string `json:"expectedError"`
	} `json:"errors"`
}

// 📥️ vectors decodes the shared vector set of this case.
func vectors(ctx *host.Context) (*fileIntegrateVectors, error) {
	raw, err := ctx.FixtureBytes("shared://📥️file-integrate-trees.json")
	if err != nil {
		return nil, err
	}
	decoded := &fileIntegrateVectors{}
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

// 🧬️ integrate folds one file into a section of another and returns the new target content.
func integrate(ctx *host.Context, vector fileIntegrateVector) (string, []string, error) {
	dir, err := root(ctx, "file-integrate")
	if err != nil {
		return "", nil, err
	}
	if err := write(dir, vector.SourceFile, vector.SourceContent); err != nil {
		return "", nil, err
	}
	if err := write(dir, vector.TargetFile, vector.TargetContent); err != nil {
		return "", nil, err
	}
	result := move.ToolIntegrate(vector.SourceFile, vector.TargetSection, vector.TargetFile, vector.ParentSection)
	if result.Error != "" {
		return "", nil, errors.New(result.Error)
	}
	produced, err := os.ReadFile(filepath.Join(dir, filepath.FromSlash(vector.TargetFile)))
	if err != nil {
		return "", nil, err
	}
	return string(produced), lines(result), nil
}

// 🏷️ sectionNames collects every section name, parents before children, in document order.
func sectionNames(sections []model.Section, out *[]string) {
	for index := range sections {
		*out = append(*out, sections[index].Name)
		sectionNames(sections[index].Children, out)
	}
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

func theSourceLandsInsideTheMarkers(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	integrated := []any{}
	for _, vector := range file.Cases {
		content, messages, err := integrate(ctx, vector)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		if err := expect(vector.Name, "integrated file", vector.Expected, content); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "output lines", vector.Messages, messages); err != nil {
			return host.Outcome{}, err
		}
		integrated = append(integrated, map[string]any{"name": vector.Name, "content": content, "messages": messages})
	}
	return host.Outcome{Projection: map[string]any{"integrated": integrated}}, nil
}

func theNewSectionIsParseable(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	readBack := []any{}
	for _, vector := range file.Cases {
		content, _, err := integrate(ctx, vector)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		language := languages.GetLanguage(vector.TargetFile)
		if language == nil {
			return host.Outcome{}, errors.New(vector.Name + ": the recorded target file has no language")
		}
		names := []string{}
		sectionNames(language.ParseSections(content), &names)
		found := false
		for _, name := range names {
			if name == vector.TargetSection {
				found = true
				break
			}
		}
		if !found {
			return host.Outcome{}, errors.New(vector.Name + ": the section " + vector.TargetSection + " written by the integration is not found again, only " + strings.Join(names, ", "))
		}
		readBack = append(readBack, map[string]any{"name": vector.Name, "sections": names})
	}
	return host.Outcome{Projection: map[string]any{"readBack": readBack}}, nil
}

func anUnknownParentSectionIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	sample := fileIntegrateVector{}
	for _, vector := range file.Cases {
		if vector.TargetFile == "sample.ts" {
			sample = vector
			break
		}
	}
	if sample.TargetFile == "" {
		return host.Outcome{}, errors.New("the vector set carries no sample.ts case")
	}
	dir, err := root(ctx, "file-integrate-errors")
	if err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "gamma.ts", sample.SourceContent); err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "sample.ts", sample.TargetContent); err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "plain.txt", "no sections here\n"); err != nil {
		return host.Outcome{}, err
	}
	refusals := []any{}
	for _, vector := range file.Errors {
		result := move.ToolIntegrate(vector.SourceFile, vector.TargetSection, vector.TargetFile, vector.ParentSection)
		if result.Error == "" {
			return host.Outcome{}, errors.New(vector.Name + ": the integration was accepted but the vector records a refusal")
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
		Subject("the-source-lands-inside-the-markers", theSourceLandsInsideTheMarkers).
		Subject("the-new-section-is-parseable", theNewSectionIsParseable).
		Subject("an-unknown-parent-section-is-refused", anUnknownParentSectionIsRefused)
}

// endregion 🔖️Registration
