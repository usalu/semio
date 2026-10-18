// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📇️registry carries the language registry and the lookup of a language by path or name.

// #endregion 🧲️Header

package languages

import (
	filepath "path/filepath"
	strings "strings"
)

// #region 🎶️TypeScript

// 💿️languageRegistry holds the data fields for a languageRegistry record.
var languageRegistry = []LanguagePlugin{
	NewTypeScriptLanguage(),
	NewGoLanguage(),
	NewPythonLanguage(),
	NewCSharpLanguage(),
	NewMarkdownLanguage(),
	NewRustLanguage(),
	NewRubyLanguage(),
	NewShellLanguage(),
	NewTomlLanguage(),
	NewYamlLanguage(),
	NewSqlLanguage(),
	NewGraphqlLanguage(),
}

// 🏪️GetLanguage MUST return the stored value without modification.
// 📖️GetLanguage returns the language of the value.
func GetLanguage(filePath string) LanguagePlugin {
	ext := strings.ToLower(filepath.Ext(filePath))
	for _, lang := range languageRegistry {
		if lang.MatchesExtension(ext) {
			return lang
		}
	}
	return nil
}

// 🔷️GetLanguageByName MUST return the stored value without modification.
// 🔤️GetLanguageByName returns the language by name of the value.
func GetLanguageByName(name string) LanguagePlugin {
	for _, lang := range languageRegistry {
		if lang.Name() == name {
			return lang
		}
	}
	return nil
}

// #endregion 🎶️TypeScript
