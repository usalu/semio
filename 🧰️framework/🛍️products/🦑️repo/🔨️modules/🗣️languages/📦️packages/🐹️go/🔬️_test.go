// 🔬️ Tests of the languages domain, split out of the pre-split godfile suite.

package languages

import (
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestFormatterPlansCoverRegisteredLanguages(t *testing.T) {
	for _, language := range languageRegistry {
		plans := workspace.FormatterPlansForLanguage(language.Name(), "example.file")
		if len(plans) == 0 {
			t.Fatalf("expected formatter plans for language %q", language.Name())
		}
		for _, plan := range plans {
			if strings.TrimSpace(plan.Binary) == "" {
				t.Fatalf("formatter plan for language %q has empty binary", language.Name())
			}
		}
	}
}

func TestFormatHeaderStructure(t *testing.T) {
	lang := NewTypeScriptLanguage()
	fileId := model.EmojiText(model.EmojiFileCode) + "test/file.ts"
	fileUri := "repo://file/" + model.EmojiText(model.EmojiFileCode) + "test"
	header := lang.FormatHeader(fileId, fileUri, "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some requirements")
	if !strings.Contains(header, "// #region 🔖️Header") {
		t.Error("header missing Header region start")
	}
	if !strings.Contains(header, "// #endregion 🔖️Header") {
		t.Error("header missing Header region end")
	}
	if !strings.Contains(header, "["+fileId+"]("+fileUri+")") {
		t.Errorf("header missing [ID](URI) identification, got: %s", header)
	}
	if !strings.Contains(header, "A test file") {
		t.Error("header missing summary")
	}
	if !strings.Contains(header, "2025 Test User <test@test.com>") {
		t.Error("header missing contributors")
	}
	if !strings.Contains(header, "AGPL license text here") {
		t.Error("header missing license text")
	}
	if !strings.Contains(header, "Some requirements") {
		t.Error("header missing requirements text")
	}
}

func TestFormatHeaderEmptyRequirements(t *testing.T) {
	lang := NewGoLanguage()
	header := lang.FormatHeader("💻️test/file.go", "repo://file/💻️test", "", "2025 Dev <dev@dev.com>", "AGPL text", "")
	if strings.Contains(header, "Requirements") {
		t.Error("header should not contain Requirements subregion when requirements is empty")
	}
	if !strings.Contains(header, "// #region 🔖️Header") {
		t.Error("header missing Header region start")
	}
}

func TestFormatHeaderAllLanguages(t *testing.T) {
	languages := []LanguagePlugin{
		NewTypeScriptLanguage(),
		NewGoLanguage(),
		NewPythonLanguage(),
		NewCSharpLanguage(),
		NewRustLanguage(),
		NewRubyLanguage(),
		NewShellLanguage(),
		NewSqlLanguage(),
		NewGraphqlLanguage(),
	}
	for _, lang := range languages {
		header := lang.FormatHeader("💻️test/file", "repo://file/💻️test", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header == "" {
			t.Errorf("%s: FormatHeader returned empty", lang.Name())
		}
		if !strings.Contains(header, "[💻️test/file](repo://file/💻️test)") {
			t.Errorf("%s: header missing [ID](URI) identification", lang.Name())
		}
	}
	noHeader := []LanguagePlugin{
		NewMarkdownLanguage(),
		NewTomlLanguage(),
		NewYamlLanguage(),
	}
	for _, lang := range noHeader {
		header := lang.FormatHeader("💻️test/file", "repo://file/💻️test", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header != "" {
			t.Errorf("%s: FormatHeader should return empty for non-header language", lang.Name())
		}
	}
}
