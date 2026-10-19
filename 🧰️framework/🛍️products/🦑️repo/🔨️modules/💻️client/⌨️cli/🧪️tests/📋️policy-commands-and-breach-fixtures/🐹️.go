// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Policy commands, grouped breach fixtures, and header formatting.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

//#endregion 📤️Event Export

// 📜️#region 🔬️Policy
func TestPolicyListCommand(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]PolicyDef)
	if !ok {
		t.Error("ToolPolicyList data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyList returned no policies")
	}
	foundCode := false
	for _, p := range policies {
		if p.ID == "code" {
			foundCode = true
			break
		}
	}
	if !foundCode {
		t.Error("Expected to find 'code' policy")
	}
}

func TestPolicyTreeCommand(t *testing.T) {
	result := ToolPolicyTree()
	if result.Error != "" {
		t.Errorf("ToolPolicyTree returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyTree returned nil data")
	}
	policies, ok := result.Data.([]PolicyDef)
	if !ok {
		t.Error("ToolPolicyTree data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyTree returned no policies")
	}
	text := result.Output.Lines[0].Text
	if !strings.Contains(text, "code") {
		t.Error("Expected policy tree output to contain 'code' policy")
	}
	if !strings.Contains(text, "code") && !strings.Contains(text, "Missing") {
		t.Error("Expected policy tree output to contain policy info")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "compose/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestExhaustivePolicyBreachListCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy breach list test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolPolicyBreachList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyBreachList returned error: %s", result.Error)
	}
}

func TestFixtureBreachsGroupedInline(t *testing.T) {
	root := withStatuteFixtures(t)
	path := "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx"
	if _, err := os.Stat(filepath.Join(root, path)); err != nil {
		t.Fatalf("fixture input does not resolve: %v", err)
	}
	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("fixture policy check failed: %v", err)
	}
	if len(breachs) == 0 {
		t.Fatal("expected fixture breachs")
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	required := []Statute{
		BreachCodeSectionMissingSummary,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range required {
		if counts[kind] == 0 {
			t.Fatalf("expected statute %s", kind)
		}
	}
}

func TestFixtureBreachsByLanguage(t *testing.T) {
	withStatuteFixtures(t)
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []Statute
	}{
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
			requiredKinds: []Statute{BreachCodeDefMissingSummary},
		},
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
		{
			path:          "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
	}
	for _, fixture := range fixtures {
		scope := Scope{Kind: ScopeFile, FilePath: fixture.path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixture.path})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", fixture.path, err)
		}
		if len(breachs) == 0 {
			t.Fatalf("expected fixture breachs for %s", fixture.path)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		for _, kind := range fixture.requiredKinds {
			if counts[kind] == 0 {
				t.Fatalf("expected statute %s in %s", kind, fixture.path)
			}
		}
	}
	clean := []string{
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐹️.go",
	}
	for _, path := range clean {
		scope := Scope{Kind: ScopeFile, FilePath: path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", path, err)
		}
		if len(breachs) != 0 {
			for _, v := range breachs {
				t.Logf("breach in %s: kind=%s scope=%s line=%d summary=%s", path, v.Kind, v.Scope, v.Line, v.Summary)
			}
			t.Fatalf("expected no breachs for %s, got %d", path, len(breachs))
		}
	}
}

func TestFormatHeaderStructure(t *testing.T) {
	lang := NewTypeScriptLanguage()
	fileId := emojiText(EmojiFileCode) + "test/file.ts"
	fileUri := "repo://file/" + emojiText(EmojiFileCode) + "test"
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

func TestToolPolicyListIsSortedByIdentifier(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Fatalf("ToolPolicyList returned error: %s", result.Error)
	}
	policies := sortedRegisteredPolicies()
	if len(policies) < 2 {
		t.Fatalf("expected at least two registered policies, got %d", len(policies))
	}
	for index := 1; index < len(policies); index++ {
		if policies[index].ID < policies[index-1].ID {
			t.Errorf("policy %q is listed after %q", policies[index].ID, policies[index-1].ID)
		}
	}
}
