// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Test function names, definition kinds, bundle languages, and test scope resolution.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🧪️#region 🕸️Test Command
func TestIsTestFunctionName(t *testing.T) {

	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		{"Go TestXxx", "TestSomething", true},
		{"Go TestAbc", "TestAbc", true},
		{"Go BenchmarkXxx", "BenchmarkSomething", true},
		{"Go FuzzXxx", "FuzzSomething", true},
		{"Python test_xxx", "test_something", true},
		{"Go Test without capital", "Testnotcapital", false},
		{"plain func", "doSomething", false},
		{"plain func capitalized", "DoSomething", false},
		{"Test alone not enough", "Test", false},
		{"Benchmark alone not enough", "Benchmark", false},
		{"Fuzz alone not enough", "Fuzz", false},
		{"test_ prefix not Go", "test_go_style", true},
		{"lowercase test", "testsomething", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := isTestFunctionName(tt.input)
			if got != tt.expected {
				t.Errorf("isTestFunctionName(%q) = %v, want %v", tt.input, got, tt.expected)
			}
		})
	}
}

func TestDefinitionKindTestEmoji(t *testing.T) {

	emoji := definitionKindEmoji(map[string]interface{}{"kind": string(DefinitionKindTest)})
	expected := emojiText(EmojiDefinitionTest)
	if emoji != expected {
		t.Errorf("definitionKindEmoji(test) = %q, want %q", emoji, expected)
	}
}

func TestDefinitionKindTestCode(t *testing.T) {

	code := definitionKindCode(map[string]interface{}{"kind": string(DefinitionKindTest)})
	if code != "t" {
		t.Errorf("definitionKindCode(DefinitionKindTest) = %q, want %q", code, "t")
	}

	emoji := definitionEmojiFromCode("t")
	if emoji != EmojiDefinitionTest {
		t.Errorf("definitionEmojiFromCode(t) = %q, want %q", emoji, EmojiDefinitionTest)
	}
}

func TestDefinitionKindTestIsValid(t *testing.T) {
	if !DefinitionKindTest.IsValid() {
		t.Error("DefinitionKindTest.IsValid() should return true")
	}
}

func TestBuildDefinitionIDTestKind(t *testing.T) {

	labFileID := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiFileLab) + "maintest"

	id := buildDefinitionID(labFileID, nil, "TestSomething", DefinitionKindImplementation)
	testEmoji := emojiText(EmojiDefinitionTest)
	implEmoji := emojiText(EmojiDefinitionImpl)

	if !strings.Contains(id, testEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should contain test emoji %q, got %q", testEmoji, id)
	}
	if strings.Contains(id, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should NOT contain impl emoji %q, got %q", implEmoji, id)
	}

	id2 := buildDefinitionID(labFileID, nil, "helperFunc", DefinitionKindImplementation)
	if !strings.Contains(id2, implEmoji) {
		t.Errorf("buildDefinitionID for helperFunc in lab file should contain impl emoji %q, got %q", implEmoji, id2)
	}

	codeFileID := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiFileCode) + "main"
	id3 := buildDefinitionID(codeFileID, nil, "TestSomething", DefinitionKindImplementation)
	if !strings.Contains(id3, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in code file should contain impl emoji %q, got %q", implEmoji, id3)
	}
}

func TestDetectBundleLanguage(t *testing.T) {

	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	t.Run("go", func(t *testing.T) {
		goDir := filepath.Join(tmpDir, "gomod")
		if err := os.MkdirAll(goDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(goDir, "go.mod"), []byte("module test\n\ngo 1.21\n"), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(goDir)
		if lang != "go" {
			t.Errorf("detectBundleLanguage with go.mod = %q, want %q", lang, "go")
		}
	})

	t.Run("python", func(t *testing.T) {
		pyDir := filepath.Join(tmpDir, "pyproj")
		if err := os.MkdirAll(pyDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(pyDir, "pyproject.toml"), []byte("[technology]\nname = \"test\"\n"), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(pyDir)
		if lang != "python" {
			t.Errorf("detectBundleLanguage with pyproject.toml = %q, want %q", lang, "python")
		}
	})

	t.Run("typescript", func(t *testing.T) {
		tsDir := filepath.Join(tmpDir, "tspack")
		if err := os.MkdirAll(tsDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(tsDir, "package.json"), []byte(`{"name":"test"}`), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(tsDir)
		if lang != "typescript" {
			t.Errorf("detectBundleLanguage with package.json = %q, want %q", lang, "typescript")
		}
	})

	t.Run("unknown", func(t *testing.T) {
		emptyDir := filepath.Join(tmpDir, "emptydir")
		if err := os.MkdirAll(emptyDir, 0755); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(emptyDir)
		if lang != "" {
			t.Errorf("detectBundleLanguage empty dir = %q, want empty string", lang)
		}
	})
}

func TestUnflattenTestName(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"testsomething", "TestSomething"},
		{"benchmarksomething", "BenchmarkSomething"},
		{"fuzzsomething", "FuzzSomething"},
		{"testbenchmarksomething", "TestbenchmarkSomething"},
		{"myfunction", "Myfunction"},
		{"", ""},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got := unflattenTestName(tt.input)
			if got != tt.expected {
				t.Errorf("unflattenTestName(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

func TestResolveTestScopes(t *testing.T) {

	scopes := resolveTestScopes(nil)
	if len(scopes) != 1 {
		t.Fatalf("resolveTestScopes(nil) len = %d, want 1", len(scopes))
	}
	if scopes[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes(nil)[0].Kind = %q, want %q", scopes[0].Kind, testScopeAll)
	}

	scopes2 := resolveTestScopes([]string{})
	if len(scopes2) != 1 || scopes2[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes([]) should return testScopeAll scope")
	}

	scopes3 := resolveTestScopes([]string{"notavalidid"})
	if len(scopes3) != 1 || scopes3[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes([invalid]) should return testScopeAll scope")
	}
}

func TestCollectGoTestsInSection(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "foo_test.go")
	content := `package foo

// 🧪️#region 📷️Alpha
func TestAlpha(t *testing.T) {}

func TestAlphaBeta(t *testing.T) {}

// #endregion 📷️Alpha

// 🧪️#region ⭐️Gamma
func TestGamma(t *testing.T) {}

// #endregion ⭐️Gamma
`
	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	pattern := collectGoTestsInSection(testFile, "Alpha")
	if pattern == "" {
		t.Fatal("expected non-empty pattern for Alpha section")
	}
	if !strings.Contains(pattern, "TestAlpha") {
		t.Errorf("pattern should contain TestAlpha, got: %s", pattern)
	}
	if !strings.Contains(pattern, "TestAlphaBeta") {
		t.Errorf("pattern should contain TestAlphaBeta, got: %s", pattern)
	}
	if strings.Contains(pattern, "TestGamma") {
		t.Errorf("pattern should NOT contain TestGamma from different section, got: %s", pattern)
	}

	empty := collectGoTestsInSection(testFile, "Nonexistent")
	if empty != "" {
		t.Errorf("collectGoTestsInSection for missing section should return empty, got: %s", empty)
	}
}

func TestResolveTestFunctionName(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "foo_test.go")
	content := `package foo

func TestMyFunction(t *testing.T) {}

func BenchmarkMyFunction(b *testing.B) {}
`
	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	name := resolveTestFunctionName(testFile, "testmyfunction")
	if name != "TestMyFunction" {
		t.Errorf("resolveTestFunctionName(testmyfunction) = %q, want %q", name, "TestMyFunction")
	}

	bname := resolveTestFunctionName(testFile, "benchmarkmyfunction")
	if bname != "BenchmarkMyFunction" {
		t.Errorf("resolveTestFunctionName(benchmarkmyfunction) = %q, want %q", bname, "BenchmarkMyFunction")
	}

	missing := resolveTestFunctionName(testFile, "nonexistent")
	if missing != "" {
		t.Errorf("resolveTestFunctionName for missing function should return empty, got: %q", missing)
	}
}

func TestTestCommandHelp(t *testing.T) {
	stdout, _, err := executeCommand("test", "--help")
	if err != nil {
		t.Fatalf("test --help returned error: %v", err)
	}
	if !strings.Contains(stdout, "test") {
		t.Errorf("test --help output should mention 'test', got: %s", stdout)
	}
}
