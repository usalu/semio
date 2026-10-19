// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩹️autofix carries the breach autofix applicator.

// #endregion 🧲️Header

package graphql

import (
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	statutespkg "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
)

// #region 🩹️Autofix

// 🧾️autofixState holds the mutable text of one file while its breaches are fixed.
type autofixState struct {
	file          string
	absPath       string
	language      languages.LanguagePlugin
	content       string
	lines         []string
	linesToRemove map[int]bool
	fixed         int
}

// 📥️loadAutofixState reads the file and prepares its line buffer.
func loadAutofixState(file string) (*autofixState, error) {
	absPath := filepath.Join(workspace.GetRootDir(), file)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return nil, err
	}
	return &autofixState{
		file:          file,
		absPath:       absPath,
		language:      languages.GetLanguage(file),
		content:       content,
		lines:         strings.Split(content, "\n"),
		linesToRemove: map[int]bool{},
	}, nil
}

// 🩹️applyAutofixes fixes the breaches of one file from the bottom up and returns the number of applied fixes.
func applyAutofixes(file string, breachs []model.Breach) (int, error) {
	s, err := loadAutofixState(file)
	if err != nil {
		return 0, err
	}
	sort.Slice(breachs, func(i, j int) bool {
		return breachs[i].Line > breachs[j].Line
	})
	for _, v := range breachs {
		s.fix(v)
	}
	systemFixed, systemErr := applySystemAutofixes(breachs)
	if systemErr != nil {
		return s.fixed, systemErr
	}
	s.fixed += systemFixed
	return s.finish()
}

// 🔧️fix dispatches one breach to the fixer of its kind.
func (s *autofixState) fix(v model.Breach) {
	switch v.Kind {
	case model.BreachCodeFileMissingHeaderRegion:
		s.fixMissingHeaderRegion(v)
	case model.BreachCodeSectionEmpty:
		s.fixEmptySection(v)
	case model.BreachCodeSectionWrongFormatNewlineAfterRegion:
		s.fixNewlineAfterRegion(v)
	case model.BreachCodeSectionMissingEndName:
		s.fixMissingSectionEndName(v)
	case model.BreachCodeSectionNameMismatch:
		s.fixSectionNameMismatch(v)
	case model.BreachCodeDefNotNativeDocstring:
		s.fixNativeDocstring(v)
	case model.BreachCodeDefMissingSummary:
		s.fixMissingSummary(v)
	case model.BreachCodeDefMissingRequirements:
		s.fixMissingRequirements(v)
	case model.BreachCodeSectionMissingSummary:
		s.fixMissingSectionSummary(v)
	case model.BreachCodeCommentInline:
		s.fixInlineComment(v)
	case model.BreachCodeCommentBlock, model.BreachCodeCommentJSDoc:
		s.fixBlockComment(v)
	case model.BreachCodeUnicodeEmojiVariation:
		s.fixEmojiVariation(v)
	case model.BreachCodeFileMissingLicense, model.BreachCodeFileWrongLicense:
		s.fixLicense(v)
	}
}

// 💾️finish drops the removed lines, collapses blank runs, writes the file and runs the formatter.
func (s *autofixState) finish() (int, error) {
	content := strings.Join(s.lines, "\n")
	if len(s.linesToRemove) > 0 {
		var kept []string
		for i, line := range s.lines {
			if !s.linesToRemove[i+1] {
				kept = append(kept, line)
			}
		}
		var collapsed []string
		for i, line := range kept {
			if strings.TrimSpace(line) == "" && i > 0 && strings.TrimSpace(kept[i-1]) == "" {
				continue
			}
			collapsed = append(collapsed, line)
		}
		content = strings.Join(collapsed, "\n")
	}
	if s.fixed > 0 {
		if err := workspace.WriteTextFile(s.absPath, content); err != nil {
			return 0, err
		}
		if err := statutespkg.RunFormatterAfterAutofix(s.file, s.language); err != nil {
			return 0, err
		}
	}
	return s.fixed, nil
}

// 🧲️fixMissingHeaderRegion prepends the generated file header region when the language supports headers.
func (s *autofixState) fixMissingHeaderRegion(v model.Breach) {
	if s.language != nil && s.language.SupportsHeaders() {
		headerContent := move.GenerateFileHeader(s.file, s.language)
		if headerContent != "" {
			s.content = headerContent + "\n" + s.content
			s.lines = strings.Split(s.content, "\n")
			s.fixed++
		}
	}
}

// 🛠️Fix applies the autofixes of every file the scope names and reports what the analyzer still finds.
func (c *RepoContext) Fix(scope *string) (*model.FixResult, error) {
	text := "compose"
	if scope != nil {
		text = *scope
	}
	parsed := workspace.ParseScope(text)
	check := func() ([]model.Breach, error) {
		return statutespkg.CheckPoliciesWithContext(statutespkg.NewPolicyContext(parsed, c.bundles), nil)
	}
	before, err := check()
	if err != nil {
		return nil, err
	}
	byFile := map[string][]model.Breach{}
	files := []string{}
	for _, breach := range before {
		file := workspace.ParseScope(breach.Scope).FilePath
		if file == "" || !breach.Autofixable() {
			continue
		}
		if _, seen := byFile[file]; !seen {
			files = append(files, file)
		}
		byFile[file] = append(byFile[file], breach)
	}
	sort.Strings(files)
	for _, file := range files {
		if _, err := applyAutofixes(file, byFile[file]); err != nil {
			return nil, err
		}
	}
	after, err := check()
	if err != nil {
		return nil, err
	}
	remaining := make([]*model.Breach, len(after))
	for index := range after {
		remaining[index] = &after[index]
	}
	return &model.FixResult{Fixed: max(len(before)-len(after), 0), Remaining: len(after), Breachs: remaining}, nil
}

// #endregion 🩹️Autofix
