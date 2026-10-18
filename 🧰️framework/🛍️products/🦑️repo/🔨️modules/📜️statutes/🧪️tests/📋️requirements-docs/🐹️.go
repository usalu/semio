package statutes

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📋️Requirements And Docs Tests

func TestRequirementsBreach(t *testing.T) {
	t.Run("isSpecText detects RFC 2119 keywords", func(t *testing.T) {
		cases := []struct {
			text   string
			expect bool
		}{
			{"File headers MUST contain License subregions.", true},
			{"Implementations SHOULD follow the standard.", true},
			{"This feature MAY be omitted.", true},
			{"Clients SHALL NOT modify the data.", true},
			{"This is REQUIRED for all files.", true},
			{"This approach is RECOMMENDED.", true},
			{"This field is OPTIONAL.", true},
			{"MUST NOT contain inline code.", true},
			{"This is a normal comment.", false},
			{"Just some text here.", false},
			{"", false},
		}
		for _, tc := range cases {
			got := IsSpecText(tc.text)
			if got != tc.expect {
				t.Errorf("isSpecText(%q) = %v, want %v", tc.text, got, tc.expect)
			}
		}
	})

	t.Run("hasImplementationSyntax detects backticks", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"File headers MUST contain `License` subregions.", true},
			{"Use `FormatHeader` to build headers.", true},
			{"File headers MUST contain License subregions.", false},
			{"Requirements MUST be implementation-agnostic.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("hasImplementationSyntax detects function calls", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"FormatHeader() MUST build the header.", true},
			{"Call ctx.ReadText() for content.", true},
			{"File headers MUST contain License subregions.", false},
			{"Requirements MUST be clean.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("requirementsPolicy detects implementation syntax in header Requirements", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain `License` subregions.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeRequirementsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeRequirementsSyntax for backtick-wrapped code in header Requirements")
		}
	})

	t.Run("requirementsPolicy clean requirements no breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain License subregions.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == model.BreachCodeRequirementsSyntax {
				t.Errorf("unexpected BreachCodeRequirementsSyntax for clean spec: %s", v.Summary)
			}
		}
	})

	t.Run("requirementsPolicy detects implementation syntax in section requirements", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️MySection\n\n// Validation MUST call `ctx.Check()` internally.\n\nconst x = 1;\n\n// #endregion 🔖️MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeRequirementsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeRequirementsSyntax for backtick in section spec")
		}
	})

	t.Run("section spec comments exempt from inline breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️MySection\n\n// Validation MUST check constraints.\n\nconst x = 1;\n\n// #endregion 🔖️MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == model.BreachCodeCommentInline {
				t.Errorf("spec comment should be exempt from inline breach: line %d %s", v.Line, v.Excerpt)
			}
		}
	})

	t.Run("JSDoc spec comments exempt from JSDoc breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️MySection\n\n/**\n * Kits MUST be editable offline.\n */\nconst x = 1;\n\n// #endregion 🔖️MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == model.BreachCodeCommentJSDoc {
				t.Errorf("JSDoc spec comment should be exempt from JSDoc breach: line %d", v.Line)
			}
		}
	})

	t.Run("non-spec JSDoc still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️MySection\n\nx = 1;\n\n/**\n * This is a regular docstring without spec keywords.\n */\n\n// #endregion 🔖️MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeCommentJSDoc {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec JSDoc to still be flagged")
		}
	})

	t.Run("non-spec inline comment still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Header\n\n// 🥼️test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️MySection\n\nconst x = 1;\n\n// This is a regular comment not a spec.\n\n// #endregion 🔖️MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeCommentInline {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec inline comment to be flagged")
		}
	})

	t.Run("BreachCodeRequirementsSyntax in breach info table", func(t *testing.T) {
		info := model.BreachCodeRequirementsSyntax.Info()
		if info.Kind != model.BreachCodeRequirementsSyntax {
			t.Errorf("expected kind %s, got %s", model.BreachCodeRequirementsSyntax, info.Kind)
		}
		if info.Autofixable {
			t.Error("requirements syntax breach should not be autofixable")
		}
	})
}

func TestDocsBreach(t *testing.T) {
	t.Run("docsPolicy detects missing README.md", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []model.Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeDocsMissingReadme {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing README.md")
		}
	})
	t.Run("docsPolicy detects missing Summary section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := workspace.WriteTextFile(readmePath, "# 💯️Requirements\n\nSome requirements here.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Summary") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # Summary section")
		}
	})
	t.Run("docsPolicy detects missing Requirements section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := workspace.WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Requirements") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # 💯️Requirements section")
		}
	})
	t.Run("docsPolicy clean README no breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := workspace.WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n\n# Docs\n\n# 💯️Requirements\n\nSome requirements.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		for _, v := range breachs {
			if v.Kind == model.BreachCodeDocsMissingReadme {
				t.Errorf("unexpected BreachCodeDocsMissingReadme: %s", v.Summary)
			}
		}
	})
	t.Run("docsPolicy deduplicates bundles with same root", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []model.Bundle{
			{Name: "bundle-a", Root: bundleRoot},
			{Name: "bundle-b", Root: bundleRoot},
		}
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		count := 0
		for _, v := range breachs {
			if v.Kind == model.BreachCodeDocsMissingReadme {
				count++
			}
		}
		if count != 1 {
			t.Errorf("expected 1 breach for deduplicated root, got %d", count)
		}
	})
}

// #endregion 📋️Requirements And Docs Tests
