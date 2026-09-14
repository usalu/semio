// 🔬️ Tests of the statutes domain, split out of the pre-split godfile suite.

package statutes

import (
	bytes "bytes"
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestPolicyStatutesHaveMetadata(t *testing.T) {
	for _, policy := range GetPolicies() {
		for _, statute := range policy.AllKinds() {
			info, ok := model.StatuteInfoTable[statute]
			if !ok {
				t.Fatalf("policy %q statute %q is missing metadata", policy.ID, statute)
			}
			if info.Kind != statute {
				t.Fatalf("policy %q statute %q metadata kind = %q", policy.ID, statute, info.Kind)
			}
		}
	}
}

func TestFixConfigIgnored(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// comment in config\nconst x = 1;\n"
	testFile := "package.json"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := languages.NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	if len(breachs) != 0 {
		t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
	}
}

func TestScanCommentsGo(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewGoLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// this is a comment\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n/* block */\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// TODO: fix later\n// continuation of todo\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("block TODO skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n/* TODO: fix later */\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for block TODO, got %d", len(breachs))
		}
	})

	t.Run("nolint skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// nolint:errcheck\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for nolint, got %d", len(breachs))
		}
	})

	t.Run("raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nvar s = `// not a comment`\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in raw string, got %d", len(breachs))
		}
	})

	t.Run("multi-line raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nvar s = `line1\n// not a comment\nline3`\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line raw string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "// #region 🔖️Header\n\n// header comment\n\n// #endregion 🔖️Header\n\n// #region 🔖️Section\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("debug marker skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nfmt.Println(\"[DEBUG] test\")\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for debug marker, got %d", len(breachs))
		}
	})

	t.Run("url scheme not flagged", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nvar url = \"https://example.com\"\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for URL scheme, got %d", len(breachs))
		}
	})

	t.Run("grouped inline comments", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// comment one\n\n// comment two\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 2 {
			t.Errorf("expected 2 breachs for separate comment blocks, got %d", len(breachs))
		}
	})
}

func TestScanCommentsPython(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewPythonLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "# region Section\n\n# TODO: fix later\n# continuation of todo\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("noqa skipped", func(t *testing.T) {
		content := "# region Section\n\n# noqa: E501\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for noqa, got %d", len(breachs))
		}
	})

	t.Run("type ignore skipped", func(t *testing.T) {
		content := "# region Section\n\n# type: ignore[assignment]\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for type: ignore, got %d", len(breachs))
		}
	})

	t.Run("triple double quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"# not a comment\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("triple single quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = '''# not a comment'''\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-single-quoted string, got %d", len(breachs))
		}
	})

	t.Run("multi-line triple quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"\n# not a comment\n\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "# region Header\n#\n# header comment\n#\n# endregion Header\n\n# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("comment in regular string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"# not a comment\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("trailing comment", func(t *testing.T) {
		content := "# region Section\n\nx = 1  # trailing comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach for trailing comment, got %d", len(breachs))
		}
		if breachs[0].Column <= 1 {
			t.Errorf("expected column > 1 for trailing comment, got %d", breachs[0].Column)
		}
	})
}

func TestScanCommentsCSharp(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewCSharpLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "#region 🔖️Section\n\n// this is a comment\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "#region 🔖️Section\n\n/* block */\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "#region 🔖️Section\n\n// TODO: fix later\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("pragma skipped", func(t *testing.T) {
		content := "#region 🔖️Section\n\n// pragma warning disable\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for pragma, got %d", len(breachs))
		}
	})

	t.Run("verbatim string skipped", func(t *testing.T) {
		content := "#region 🔖️Section\n\nvar s = @\"// not a comment\";\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in verbatim string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "#region 🔖️Section\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "#region 🔖️Header\n// header comment\n#endregion 🔖️Header\n\n#region 🔖️Section\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("no JSDoc for csharp", func(t *testing.T) {
		content := "#region 🔖️Section\n\n/** not jsdoc in csharp */\n\npublic class C {}\n\n#endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentBlock {
			t.Errorf("expected block comment (not JSDoc) for C#, got %s", breachs[0].Kind)
		}
	})
}

func TestScanCommentsTypeScript(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewTypeScriptLanguage()

	t.Run("JSDoc detected", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n/** jsdoc */\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentJSDoc {
			t.Errorf("expected JSDoc breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("template literal skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nconst s = `// not a comment`;\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in template literal, got %d", len(breachs))
		}
	})

	t.Run("template expression not skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nconst s = `${x} // comment`;\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for template expression context, got %d", len(breachs))
		}
	})

	t.Run("eslint directive skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// eslint-disable-next-line\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for eslint directive, got %d", len(breachs))
		}
	})

	t.Run("@ts directive skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\n// @ts-ignore\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for @ts directive, got %d", len(breachs))
		}
	})

	t.Run("string literals skipped", func(t *testing.T) {
		content := "// #region 🔖️Section\n\nconst a = '// not a comment';\nconst b = \"// not a comment\";\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in strings, got %d", len(breachs))
		}
	})

	t.Run("config file skipped", func(t *testing.T) {
		content := "// inline comment\nconst x = 1;\n"
		breachs := lang.ScanComments(ctx, "tsconfig.json", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
		}
	})
}

func TestScanCommentsShell(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewShellLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("comment in string skipped", func(t *testing.T) {
		content := "# region Section\n\necho \"# not a comment\"\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})
}

func TestScanCommentsRust(t *testing.T) {
	ctx := &PolicyContext{}
	lang := languages.NewRustLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "mod section { // 🔖️Section\n\n// this is a comment\n\nfn main() {}\n\n} // 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "mod section { // 🔖️Section\n\n/* block comment */\n\nfn main() {}\n\n} // 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "mod section { // 🔖️Section\n\n// TODO: fix later\n\nfn main() {}\n\n} // 🔖️Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})
}

func TestFixExtractFileFromScope(t *testing.T) {
	tests := []struct {
		scope    string
		expected string
	}{
		{"file.ts", "file.ts"},
		{"file.ts#Section", "file.ts"},
		{"file.ts::definition", "file.ts"},
		{"path/to/file.ts#Section/Sub", "path/to/file.ts"},
		{"path/to/file.ts::myFunc", "path/to/file.ts"},
	}
	for _, tt := range tests {
		result := extractFileFromScope(tt.scope)
		if result != tt.expected {
			t.Errorf("extractFileFromScope(%q) = %q, want %q", tt.scope, result, tt.expected)
		}
	}
}

func TestSectionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[model.BreachCodeSectionMissingSummary] == 0 {
		t.Fatal("expected section missing summary breach")
	}
}

func TestSectionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeSectionMissingSummary {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestDefinitionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[model.BreachCodeDefMissingSummary] == 0 {
		t.Fatal("expected definition missing summary breach")
	}
}

func TestDefinitionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefMissingSummary || v.Kind == model.BreachCodeDefMissingRequirements {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestSectionDocLinesExemptsDocComments(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n// Functions MUST be exported.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentInline {
			t.Fatalf("section doc comment wrongly flagged as inline at line %d", v.Line)
		}
	}
}

func TestDefinitionNativeDocstring(t *testing.T) {
	tests := []struct {
		name         string
		file         string
		content      string
		expectBreach bool
	}{
		{
			name:         "TypeScript // comments should flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: true,
		},
		{
			name:         "TypeScript JSDoc should NOT flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "Go // comments should NOT flag breach (native format)",
			file:         "src/app.go",
			content:      "package main\n\n// #region 🔖️Header\n\n// [💻️src/app.go](repo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.go#Functions](repo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](repo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "Python # comments should flag breach (should use triple-quote docstring)",
			file:         "src/app.py",
			content:      "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖️Functions\n",
			expectBreach: true,
		},
		{
			name:         "Python triple-quote docstring should NOT flag breach",
			file:         "src/app.py",
			content:      "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "CSharp // comments should flag breach (should use ///)",
			file:         "src/App.cs",
			content:      "// #region 🔖️Header\n\n// [💻️src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖️Types\n",
			expectBreach: true,
		},
		{
			name:         "CSharp /// comments should NOT flag breach",
			file:         "src/App.cs",
			content:      "// #region 🔖️Header\n\n// [💻️src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖️Types\n",
			expectBreach: false,
		},
		{
			name:         "Rust // comments should flag breach (should use ///)",
			file:         "src/lib.rs",
			content:      "// #region 🔖️Header\n\n// [💻️src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖️Types\n",
			expectBreach: true,
		},
		{
			name:         "Rust /// comments should NOT flag breach",
			file:         "src/lib.rs",
			content:      "// #region 🔖️Header\n\n// [💻️src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖️Types\n",
			expectBreach: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := workspace.RootDir
			workspace.RootDir = tmpDir
			defer func() { workspace.RootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tt.file))
			os.MkdirAll(dir, 0o755)
			absPath := filepath.Join(tmpDir, tt.file)
			if err := workspace.WriteTextFile(absPath, tt.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: tt.file}
			ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{tt.file})
			breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			hasBreach := false
			for _, v := range breachs {
				if v.Kind == model.BreachCodeDefNotNativeDocstring {
					hasBreach = true
					break
				}
			}
			if tt.expectBreach && !hasBreach {
				t.Fatal("expected DefNotNativeDocstring breach but got none")
			}
			if !tt.expectBreach && hasBreach {
				t.Fatal("unexpected DefNotNativeDocstring breach")
			}
		})
	}
}

func TestPythonTripleQuoteDocstringExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentBlock {
			t.Fatalf("Python triple-quote docstring should not be flagged as block comment at line %d", v.Line)
		}
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			t.Fatalf("Python triple-quote docstring should not flag DefNotNativeDocstring at line %d", v.Line)
		}
	}
}

func TestDefinitionJSDocExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentJSDoc {
			t.Fatalf("definition JSDoc should not be flagged as comment breach at line %d", v.Line)
		}
		if v.Kind == model.BreachCodeCommentBlock {
			t.Fatalf("definition JSDoc should not be flagged as block comment breach at line %d", v.Line)
		}
	}
}

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

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []model.Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []model.Statute{model.BreachCodeFileMissingHeaderRegion, model.BreachCodeFileMissingSummary},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []model.Statute{model.BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []model.Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []model.Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestFolderPolicyEmptyFolder(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "some", "empty")
	os.MkdirAll(emptyDir, 0755)
	nonEmptyDir := filepath.Join(tmpDir, "some", "nonempty")
	os.MkdirAll(nonEmptyDir, 0755)
	os.WriteFile(filepath.Join(nonEmptyDir, "file.txt"), []byte("content"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	foundEmpty := false
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty && v.Excerpt == "some/empty" {
			foundEmpty = true
			if !v.Autofixable() {
				t.Error("BreachFolderIllegalEmpty should be autofixable")
			}
		}
	}
	if !foundEmpty {
		t.Error("expected BreachFolderIllegalEmpty for some/empty")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty && v.Excerpt == "some/nonempty" {
			t.Error("should not report BreachFolderIllegalEmpty for non-empty folder")
		}
	}
}

func TestPathEmojiStatutesLanguageNeutralFixture(t *testing.T) {
	fixturePath := filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "🔏️path-emoji-statutes", "🔣️.json")
	data, err := os.ReadFile(fixturePath)
	if err != nil {
		t.Fatalf("read shared path emoji fixture: %v", err)
	}
	var fixture struct {
		GenericEmojiIdentities []string `json:"genericEmojiIdentities"`
		Cases                  []struct {
			ID       string             `json:"id"`
			Entries  []pathEmojiEntry   `json:"entries"`
			Expected []pathEmojiFinding `json:"expected"`
		} `json:"cases"`
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatalf("parse shared path emoji fixture: %v", err)
	}
	for _, scenario := range fixture.Cases {
		observed := pathEmojiStatuteFindings(scenario.Entries, fixture.GenericEmojiIdentities)
		observedJSON, _ := json.Marshal(observed)
		expectedJSON, _ := json.Marshal(scenario.Expected)
		if !bytes.Equal(observedJSON, expectedJSON) {
			t.Errorf("%s: observed %s, expected %s", scenario.ID, observedJSON, expectedJSON)
		}
	}
}

func TestFolderPolicySkipsExcludedDirs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	for _, dir := range []string{".git/objects", ".🦑️repo/cache", "node_modules/.cache"} {
		os.MkdirAll(filepath.Join(tmpDir, dir), 0755)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFolderIllegalEmpty {
			if strings.HasPrefix(v.Excerpt, ".git") || strings.HasPrefix(v.Excerpt, ".🧬semio") || strings.HasPrefix(v.Excerpt, "node_modules") {
				t.Errorf("should skip excluded dir, got breach for %s", v.Excerpt)
			}
		}
	}
}

func TestFilePolicyGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/main.ts"]`
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(srcDir, 0755)
	os.WriteFile(filepath.Join(srcDir, "main.ts"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/main.ts") {
			t.Errorf("should not report breach for allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSupportsGlobPatterns(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/**/*.ts", "docs/*.md"]`
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcNestedDir := filepath.Join(tmpDir, "src", "nested")
	os.MkdirAll(srcNestedDir, 0755)
	os.WriteFile(filepath.Join(srcNestedDir, "main.ts"), []byte("ok"), 0644)
	docsDir := filepath.Join(tmpDir, "docs")
	os.MkdirAll(docsDir, 0755)
	os.WriteFile(filepath.Join(docsDir, "guide.md"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/nested/main.ts" || v.Excerpt == "docs/guide.md") {
			t.Errorf("should not report breach for glob-allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsComposeRepo(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(`[]`), 0644)
	os.WriteFile(filepath.Join(metaDir, "some_internal.json"), []byte("internal"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && strings.HasPrefix(v.Excerpt, ".🧬semio") {
			t.Errorf("should skip .🦑️repo files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsNestedNodeModules(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.SetRootDir(tmpDir)
	defer func() { workspace.SetRootDir(oldRoot) }()
	os.WriteFile(filepath.Join(tmpDir, ".gitignore"), []byte("node_modules/\n"), 0644)
	metaDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "📁️files.json"), []byte(`[]`), 0644)
	nested := filepath.Join(tmpDir, "repo", "vscode", "node_modules", "undici-types")
	os.MkdirAll(nested, 0755)
	os.WriteFile(filepath.Join(nested, "fetch.d.ts"), []byte("export {};"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == model.BreachFileIllegalUseGodfile && strings.Contains(v.Excerpt, "node_modules/") {
			t.Errorf("should skip ignored node_modules files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyNoGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("content"), 0644)
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	if len(breachs) != 0 {
		t.Errorf("expected no breachs when godfile is missing, got %d", len(breachs))
	}
}

func TestFolderPolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("folder")
	if !found {
		t.Fatal("folder policy not registered")
	}
	if policy.Name != "Folder" {
		t.Errorf("expected policy name Folder, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == model.BreachFolderIllegalEmpty {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("folder policy should contain BreachFolderIllegalEmpty kind")
	}
}

func TestFilePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("file")
	if !found {
		t.Fatal("file policy not registered")
	}
	if policy.Name != "File" {
		t.Errorf("expected policy name File, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == model.BreachFileIllegalUseGodfile {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("file policy should contain BreachFileIllegalUseGodfile kind")
	}
}

func TestComposePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("compose")
	if !found {
		t.Fatal("compose policy not registered")
	}
	if policy.Name != "Compose" {
		t.Errorf("expected policy name Compose, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == model.BreachComposeNoUiDependency {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("compose policy should contain BreachComposeNoUiDependency kind")
	}
}

func TestComposePolicyNoUiDependency(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	t.Run("detects tailwind-merge import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { twMerge } from \"tailwind-merge\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects elements/ui import", func(t *testing.T) {
		relPath := "compose.test.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as UI from \"../../elements/ui\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != model.BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects clsx import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { ClassValue, clsx } from \"clsx\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects three import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as THREE from \"three\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects multiple ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\nimport { twMerge } from \"tailwind-merge\";\nimport * as THREE from \"three\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 3 {
			t.Fatalf("expected 3 breachs, got %d", len(breachs))
		}
	})
	t.Run("allows non-ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { z } from \"zod\";\nimport { v7 as uuidv7 } from \"uuid\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("ignores non-compose files", func(t *testing.T) {
		relPath := "other.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\n"), 0644)
		scope := workspace.Scope{Kind: workspace.ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for non-compose file, got %d", len(breachs))
		}
	})
	t.Run("not autofixable", func(t *testing.T) {
		info := model.BreachComposeNoUiDependency.Info()
		if info.Autofixable {
			t.Error("BreachComposeNoUiDependency should not be autofixable")
		}
	})
}
