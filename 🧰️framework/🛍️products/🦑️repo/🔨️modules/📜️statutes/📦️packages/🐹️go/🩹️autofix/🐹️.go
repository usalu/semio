package statutes

import (
	filepath "path/filepath"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🩹️Autofix

// 🔧️runFormatterAfterAutofix holds the data fields for a runFormatterAfterAutofix record.
func RunFormatterAfterAutofix(relPath string, language languages.LanguagePlugin) error {
	absPath := filepath.Join(workspace.RootDir, relPath)
	langName := ""
	if language != nil {
		langName = language.Name()
	}
	plans := workspace.FormatterPlansForLanguage(langName, relPath)
	for _, plan := range plans {
		if !workspace.IsFormatterPlanAvailable(plan, workspace.RootDir) {
			continue
		}
		if err := workspace.FormatterCommandRun(plan.Binary, plan.Args, workspace.RootDir); err == nil {
			return nil
		}
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return err
	}
	formatted := workspace.FallbackFormatText(content)
	if formatted == content {
		return nil
	}
	return workspace.WriteTextFile(absPath, formatted)
}

// ⬛️runFormatterForFile runs the language-appropriate formatter on the given file.
// 🖊️Unlike runFormatterAfterAutofix, this only runs the formatter with no fallback.
func RunFormatterForFile(relPath string) {
	lang := languages.GetLanguage(relPath)
	langName := ""
	if lang != nil {
		langName = lang.Name()
	}
	plans := workspace.FormatterPlansForLanguage(langName, relPath)
	for _, plan := range plans {
		if !workspace.IsFormatterPlanAvailable(plan, workspace.RootDir) {
			continue
		}
		if err := workspace.FormatterCommandRun(plan.Binary, plan.Args, workspace.RootDir); err == nil {
			return
		}
	}
}

// 🩹️FixedFile is the outcome of applying every autofix transformation to one source.
type FixedFile struct {
	Path    string
	Content string
	Fixed   []model.Statute
}

// 🩹️Autofix MUST leave the content untouched when nothing is repairable.
// 🩹️Autofix applies every autofixable transformation to one source and reports the statutes it repaired.
func Autofix(source SourceFile) FixedFile {
	content := source.Content
	fixed := []model.Statute{}
	if next, ok := fixNewlineAfterRegion(source.Path, content); ok && next != content {
		content = next
		fixed = append(fixed, model.BreachCodeSectionWrongFormatNewlineAfterRegion)
	}
	return FixedFile{Path: source.Path, Content: content, Fixed: fixed}
}

// 🧹️fixNewlineAfterRegion drops every blank line that follows a region start marker.
func fixNewlineAfterRegion(path string, content string) (string, bool) {
	language := languages.GetLanguage(path)
	if language == nil || !language.SupportsSections() {
		return content, false
	}
	lines := strings.Split(content, "\n")
	drop := make([]bool, len(lines))
	for index, raw := range lines {
		line := strings.TrimSuffix(raw, "\r")
		if matched, _ := language.PolicySectionStartMatch(line); !matched {
			continue
		}
		for next := index + 1; next < len(lines) && strings.TrimSpace(lines[next]) == ""; next++ {
			drop[next] = true
		}
	}
	kept := make([]string, 0, len(lines))
	for index, line := range lines {
		if !drop[index] {
			kept = append(kept, line)
		}
	}
	return strings.Join(kept, "\n"), true
}

// #endregion 🩹️Autofix
