// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔧️autofix_system carries the system level autofix helpers.

// #endregion 🧲️Header

package graphql

import (
	os "os"
	filepath "path/filepath"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔧️Autofix System

// 🟩applySystemAutofixes holds the data fields for a applySystemAutofixes record.
func applySystemAutofixes(breachs []model.Breach) (int, error) {
	fixed := 0
	for _, v := range breachs {
		switch v.Kind {
		case model.BreachFolderIllegalEmpty:
			folderPath := filepath.Join(workspace.GetRootDir(), v.Excerpt)
			entries, readErr := os.ReadDir(folderPath)
			if readErr == nil && len(entries) == 0 {
				if err := os.Remove(folderPath); err == nil {
					fixed++
				}
			}
		}
	}
	return fixed, nil
}

// 🔍️findMatchingSectionStartName locates the section start name for autofix helpers.
func findMatchingSectionStartName(lines []string, endLineIdx int, language languages.LanguagePlugin) string {
	depth := 0
	for i := endLineIdx - 1; i >= 0; i-- {
		if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
			depth++
			continue
		}
		if matched, name := language.PolicySectionStartMatch(lines[i]); matched {
			if depth > 0 {
				depth--
				continue
			}
			return name
		}
	}
	return ""
}

// #endregion 🔧️Autofix System
