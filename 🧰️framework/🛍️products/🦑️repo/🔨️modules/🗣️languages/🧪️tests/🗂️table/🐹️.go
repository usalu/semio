package languages

import "testing"

// #region 🔖️Table

// 🗂️TestTableLoads asserts the module-relative loader found and decoded the shared table.
func TestTableLoads(t *testing.T) {
	table, err := LoadTable()
	if err != nil {
		t.Fatalf("table did not load from %q: %v", TablePath(), err)
	}
	if table.SchemaVersion != 1 {
		t.Fatalf("schemaVersion = %d, want 1", table.SchemaVersion)
	}
	if len(table.Registry) != 12 {
		t.Fatalf("registry has %d entries, want 12", len(table.Registry))
	}
	for _, name := range table.Registry {
		if LanguageByName(name) == nil {
			t.Errorf("registered language %q is missing from languages", name)
		}
	}
}

// 🏪️TestLanguageForPath asserts extension lookup is case-insensitive and registry-ordered.
func TestLanguageForPath(t *testing.T) {
	cases := map[string]string{
		"a/b/🐹️.go": "go",
		"🟦️.TSX":    "typescript",
		"x/y/📰️.md": "markdown",
		"x.unknown": "",
	}
	for path, want := range cases {
		lang := LanguageForPath(path)
		got := ""
		if lang != nil {
			got = lang.Name
		}
		if got != want {
			t.Errorf("LanguageForPath(%q) = %q, want %q", path, got, want)
		}
	}
}

// 🔑️TestSharedVocabularyIsPresent asserts the cross-language vocabulary survived decoding.
func TestSharedVocabularyIsPresent(t *testing.T) {
	table, err := LoadTable()
	if err != nil {
		t.Fatalf("table did not load: %v", err)
	}
	if table.DefinitionKeywords.Fallback != "definition" {
		t.Errorf("fallback = %q, want %q", table.DefinitionKeywords.Fallback, "definition")
	}
	if table.DefinitionKindMap["interface"] != "interface" || table.DefinitionKindMap["const"] != "constant" {
		t.Error("definitionKindMap lost entries during decoding")
	}
	if table.ScopeRegionMarker.StartKeyword != "#region " {
		t.Errorf("startKeyword = %q", table.ScopeRegionMarker.StartKeyword)
	}
	if len(table.ScopeDefinitionPatterns[".go"]) != 4 {
		t.Errorf("scopeDefinitionPatterns['.go'] has %d patterns, want 4", len(table.ScopeDefinitionPatterns[".go"]))
	}
	if len(table.Refinements.Promotes) != 3 {
		t.Errorf("refinements.promotes has %d entries, want 3", len(table.Refinements.Promotes))
	}
}

// #endregion 🔖️Table
