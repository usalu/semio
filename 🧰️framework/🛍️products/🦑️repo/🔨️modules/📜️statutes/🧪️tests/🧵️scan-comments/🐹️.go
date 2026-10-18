package statutes

import (
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧵️Scan Comments Tests

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
		content := "// #region 🔖️Section\n\nfmt.Println(\"[TRACE] test\")\n\nfunc main() {}\n\n// #endregion 🔖️Section\n"
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

// #endregion 🧵️Scan Comments Tests
