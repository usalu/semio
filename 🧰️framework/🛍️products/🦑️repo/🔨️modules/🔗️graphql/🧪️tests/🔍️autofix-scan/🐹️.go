// 🔍️ Tests of the comment, emoji and section scanning autofixes.

package graphql

import (
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestScanCommentsAutofix(t *testing.T) {
	t.Run("python inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "# region Section\n\n# bad comment\n\ndef main(): pass\n\n# endregion Section\n"
		expected := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_inline.py"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, content)

		breachs := []model.Breach{
			{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("python trailing comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "# region Section\n\nx = 1  # trailing\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_trailing.py"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, content)

		breachs := []model.Breach{
			{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3, Column: 7},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if !strings.Contains(result, "x = 1") {
			t.Error("code should be preserved")
		}
		if strings.Contains(result, "trailing") {
			t.Error("trailing comment should be removed")
		}
	})

	t.Run("go block comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "// #region 🔖️Section\n\n/* block comment */\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		expected := "// #region 🔖️Section\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		testFile := "test_go_block.go"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, content)

		breachs := []model.Breach{
			{Kind: model.BreachCodeCommentBlock, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("csharp inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "#region 🔖️Section\n\n// bad comment\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		expected := "#region 🔖️Section\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		testFile := "test_cs_inline.cs"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, content)

		breachs := []model.Breach{
			{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := workspace.ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})
}

func TestEmojiVariationAutofix(t *testing.T) {
	t.Run("fix emoji variation to colorful", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()

		content := "This is a test \U0001F4BB\uFE0E with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7 construction."
		expected := "This is a test \U0001F4BB\uFE0F with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7\uFE0F construction.\n"
		testFile := "test_emoji.txt"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, content)

		breachs := []model.Breach{
			{Kind: model.BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 1},
			{Kind: model.BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 2},
			{Kind: model.BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 3 {
			t.Fatalf("expected 3 fixed, got %d", fixed)
		}
		got, _ := workspace.ReadTextFile(absPath)
		if got != expected {
			t.Errorf("expected:\n%q\ngot:\n%q", expected, got)
		}
	})

	t.Run("emojiText preserves VS16 for text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"⚙️", "⚙️"},
			{"⚖️", "⚖️"},
			{"✂️", "✂️"},
			{"🏗️", "🏗️"},
			{"🛠️", "🛠️"},
			{"🛡️", "🛡️"},
			{"⌨️", "⌨️"},
			{"🖱️", "🖱️"},
			{"🏷️", "🏷️"},
			{"🗃️", "🗃️"},
		}
		for _, tc := range cases {
			got := model.EmojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText strips VS16 for non-text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"\U0001F4BB\uFE0F", "\U0001F4BB"},
			{"\U0001F97C\uFE0F", "\U0001F97C"},
			{"\U0001F4C3\uFE0F", "\U0001F4C3"},
			{"\U0001F4DC\uFE0F", "\U0001F4DC"},
		}
		for _, tc := range cases {
			got := model.EmojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText is idempotent", func(t *testing.T) {
		cases := []string{"⚙️", "🏗️", "💻️", "🛠️"}
		for _, tc := range cases {
			once := model.EmojiText(tc)
			twice := model.EmojiText(once)
			if once != twice {
				t.Errorf("emojiText not idempotent: emojiText(%q)=%q, emojiText(%q)=%q", tc, once, once, twice)
			}
		}
	})
	t.Run("emojiText strips VS15", func(t *testing.T) {
		got := model.EmojiText("\u2699\uFE0E")
		if got != "⚙️" {
			t.Errorf("emojiText with VS15 = %q, want %q", got, "⚙️")
		}
	})
	t.Run("section markers not flagged as inline comments", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		fileContent := "// #region \U0001F516Header\n\n// \U0001F4BBcompose/test.tsx\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n//#region \U0001F516Action Hooks\nconst x = 1;\n//#endregion \U0001F516Action Hooks\n"
		testFile := "test.tsx"
		absPath := filepath.Join(tmpDir, testFile)
		workspace.WriteTextFile(absPath, fileContent)
		bundles := codebase.LoadBundles()
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, _ := statutes.CheckPoliciesWithContext(ctx, nil)
		for _, v := range breachs {
			if v.Kind == model.BreachCodeCommentInline {
				t.Errorf("section marker flagged as inline comment at line %d: %s", v.Line, v.Excerpt)
			}
		}
	})
}

func TestFixIdempotent(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	testFile := "test_clean.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 0 {
		t.Errorf("expected 0 fixes on clean file, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != content {
		t.Error("clean file should not be modified")
	}
}

func TestFixNestedSections(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Outer\n\n// #region 🔖️Inner\n\nconst x = 1;\n\n// #endregion\n\nconst y = 2;\n\n// #endregion\n"
	testFile := "test_nested.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 11},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 2 {
		t.Errorf("expected 2 fixes, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖️Inner") {
		t.Error("expected inner endregion to get name Inner")
	}
	if !strings.Contains(result, "// #endregion 🔖️Outer") {
		t.Error("expected outer endregion to get name Outer")
	}
}

func TestFindMatchingSectionStartName(t *testing.T) {
	lines := []string{
		"// #region 🔖️Outer",
		"",
		"// #region 🔖️Inner",
		"const x = 1;",
		"// #endregion 🔖️Inner",
		"",
		"// #endregion",
	}
	language := languages.NewTypeScriptLanguage()

	name := findMatchingSectionStartName(lines, 6, language)
	if name != "Outer" {
		t.Errorf("expected Outer, got %q", name)
	}

	name = findMatchingSectionStartName(lines, 4, language)
	if name != "Inner" {
		t.Errorf("expected Inner, got %q", name)
	}
}
