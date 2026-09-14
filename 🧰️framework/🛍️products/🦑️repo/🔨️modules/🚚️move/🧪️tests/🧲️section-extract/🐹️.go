// 🐹️ Go side of the section extract case. The Go twin edits a real directory tree, so every
// scenario materializes the recorded file under the scenario work directory and reads both results
// back.
package adapter

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	languages "github.com/usalu/semio/repo/languages"
	move "github.com/usalu/semio/repo/move"
	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 🧲️ sectionExtractVectors is the recorded vector set of this case.
type sectionExtractVectors struct {
	Cases []struct {
		Name           string   `json:"name"`
		SourceFile     string   `json:"sourceFile"`
		Content        string   `json:"content"`
		Section        string   `json:"section"`
		TargetFile     string   `json:"targetFile"`
		ExpectedTarget string   `json:"expectedTarget"`
		ExpectedSource string   `json:"expectedSource"`
		Messages       []string `json:"messages"`
	} `json:"cases"`
	Errors []struct {
		Name          string `json:"name"`
		SourceFile    string `json:"sourceFile"`
		Section       string `json:"section"`
		TargetFile    string `json:"targetFile"`
		ExpectedError string `json:"expectedError"`
	} `json:"errors"`
}

// 📥️ vectors decodes the shared vector set of this case.
func vectors(ctx *host.Context) (*sectionExtractVectors, error) {
	raw, err := ctx.FixtureBytes("shared://🧲️section-extract-trees.json")
	if err != nil {
		return nil, err
	}
	decoded := &sectionExtractVectors{}
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

// 📖️ read returns one file of a workspace, empty when it is absent.
func read(dir string, path string) string {
	produced, err := os.ReadFile(filepath.Join(dir, filepath.FromSlash(path)))
	if err != nil {
		return ""
	}
	return string(produced)
}

// 📣️ lines is the command output of a tool result, always a list and never null.
func lines(result workspace.ToolResult) []string {
	out := []string{}
	for _, line := range result.Output.Lines {
		out = append(out, line.Text)
	}
	return out
}

// 🧲️ extract lifts a section out of a one-file workspace and returns the two resulting files.
func extract(ctx *host.Context, source string, content string, section string, target string) (string, string, []string, error) {
	dir, err := root(ctx, "section-extract")
	if err != nil {
		return "", "", nil, err
	}
	if err := write(dir, source, content); err != nil {
		return "", "", nil, err
	}
	result := move.ToolExtract(source, section, target)
	if result.Error != "" {
		return "", "", nil, errors.New(result.Error)
	}
	return read(dir, target), read(dir, source), lines(result), nil
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

func theSectionLeavesWithItsImports(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	extracted := []any{}
	for _, vector := range file.Cases {
		target, source, messages, err := extract(ctx, vector.SourceFile, vector.Content, vector.Section, vector.TargetFile)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		if err := expect(vector.Name, "extracted file", vector.ExpectedTarget, target); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "remaining source", vector.ExpectedSource, source); err != nil {
			return host.Outcome{}, err
		}
		if err := expect(vector.Name, "output lines", vector.Messages, messages); err != nil {
			return host.Outcome{}, err
		}
		extracted = append(extracted, map[string]any{"name": vector.Name, "target": target, "source": source, "messages": messages})
	}
	return host.Outcome{Projection: map[string]any{"extracted": extracted}}, nil
}

func extractionRemovesExactlyTheSection(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	arithmetic := []any{}
	for _, vector := range file.Cases {
		language := languages.GetLanguage(vector.SourceFile)
		if language == nil {
			return host.Outcome{}, errors.New(vector.Name + ": the recorded source file has no language")
		}
		section := languages.FindSection(language.ParseSections(vector.Content), vector.Section)
		if section == nil {
			return host.Outcome{}, errors.New(vector.Name + ": the recorded section is not in the file")
		}
		span := section.EndLine - section.StartLine + 1
		_, remaining, _, err := extract(ctx, vector.SourceFile, vector.Content, vector.Section, vector.TargetFile)
		if err != nil {
			return host.Outcome{}, errors.New(vector.Name + ": " + err.Error())
		}
		before := len(strings.Split(vector.Content, "\n"))
		after := len(strings.Split(remaining, "\n"))
		if before-after != span {
			return host.Outcome{}, fmt.Errorf("%s: the source lost %d lines but the section spans %d", vector.Name, before-after, span)
		}
		arithmetic = append(arithmetic, fmt.Sprintf("%s: before=%d after=%d span=%d", vector.Name, before, after, span))
	}
	return host.Outcome{Projection: map[string]any{"arithmetic": arithmetic}}, nil
}

func aMissingSectionIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := vectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	sample := ""
	for _, vector := range file.Cases {
		if vector.SourceFile == "sample.ts" {
			sample = vector.Content
			break
		}
	}
	if sample == "" {
		return host.Outcome{}, errors.New("the vector set carries no sample.ts case")
	}
	dir, err := root(ctx, "section-extract-errors")
	if err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "sample.ts", sample); err != nil {
		return host.Outcome{}, err
	}
	if err := write(dir, "plain.txt", "no sections here\n"); err != nil {
		return host.Outcome{}, err
	}
	refusals := []any{}
	for _, vector := range file.Errors {
		result := move.ToolExtract(vector.SourceFile, vector.Section, vector.TargetFile)
		if result.Error == "" {
			return host.Outcome{}, errors.New(vector.Name + ": the extraction was accepted but the vector records a refusal")
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
		Subject("the-section-leaves-with-its-imports", theSectionLeavesWithItsImports).
		Subject("extraction-removes-exactly-the-section", extractionRemovesExactlyTheSection).
		Subject("a-missing-section-is-refused", aMissingSectionIsRefused)
}

// endregion 🔖️Registration
