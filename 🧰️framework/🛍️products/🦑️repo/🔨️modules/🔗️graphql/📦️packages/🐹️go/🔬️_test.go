// 🔬️ Tests of the graphql domain, split out of the pre-split godfile suite.

package graphql

import (
	context "context"
	json "encoding/json"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributors "github.com/usalu/semio/repo/contributors"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	statutes "github.com/usalu/semio/repo/statutes"
	tickets "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestMain(m *testing.M) {
	ensureExecutor()
	os.Exit(m.Run())
}

func findTestRepoRoot(start string) string {
	for _, candidate := range []string{start, func() string {
		_, file, _, ok := runtime.Caller(0)
		if !ok {
			return ""
		}
		return filepath.Dir(file)
	}()} {
		dir := candidate
		if dir == "" {
			continue
		}
		for {
			if _, err := os.Stat(filepath.Join(dir, "repo", "client", "main.go")); err == nil {
				return dir
			}
			if _, err := os.Stat(filepath.Join(dir, ".git")); err == nil {
				return dir
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break
			}
			dir = parent
		}
	}
	return start
}

func execCommandWithTimeout(t *testing.T, timeout time.Duration, dir string, env []string, name string, args ...string) []byte {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	cmd := exec.CommandContext(ctx, name, args...)
	if dir != "" {
		cmd.Dir = dir
	}
	if env != nil {
		cmd.Env = env
	}
	output, err := cmd.CombinedOutput()
	if ctx.Err() == context.DeadlineExceeded {
		t.Fatalf("%s %v timed out after %s:\n%s", name, args, timeout, output)
	}
	if err != nil {
		t.Fatalf("%s %v failed: %v\n%s", name, args, err, output)
	}
	return output
}

func getTestExecutor(t *testing.T) *Executor {
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}

	workspace.RootDir = findTestRepoRoot(cwd)
	ex, err := NewExecutor(workspace.RootDir)
	if err != nil {
		t.Fatalf("failed to create executor: %v", err)
	}
	return ex
}

func TestAnalyzeReadsBreachCacheJSON(t *testing.T) {
	tmp := t.TempDir()
	cacheDir := filepath.Join(tmp, ".🧬semio", "🦑️repo", "⚡️cache", "breaches")
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		t.Fatal(err)
	}
	cachePath := filepath.Join(cacheDir, "unit-test.json")
	payload := `{
  "entityId": "test",
  "script": "unit.test.lint.script.ts",
  "breachs": [
    {
      "id": "e1",
      "summary": "hello",
      "kind": "lint/test/rule",
      "scope": "repo/example.go",
      "priority": "medium"
    }
  ]
}`
	if err := os.WriteFile(cachePath, []byte(payload), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := workspace.RootDir
	workspace.RootDir = tmp
	defer func() { workspace.RootDir = oldRoot }()
	ctx := NewRepoContext(tmp)
	ar, err := ctx.Analyze(nil)
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if ar == nil || len(ar.Breachs) != 1 {
		t.Fatalf("expected 1 breach, got %#v", ar)
	}
	if ar.Breachs[0].Summary != "hello" {
		t.Fatalf("unexpected breach: %+v", ar.Breachs[0])
	}
}

// 🧫️statutesFixtureDir is the surviving statute fixture tree. The godfile-era
// `repo/asset/fixture/some/folder` was deleted with the legacy tree.
const statutesFixtureDir = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder"

func readStatutesFixture(t *testing.T, folder string) string {
	t.Helper()
	_, thisFile, _, _ := runtime.Caller(0)
	root := findTestRepoRoot(filepath.Dir(thisFile))
	abs := filepath.Join(root, filepath.FromSlash(statutesFixtureDir), folder, "🟦️.tsx")
	content, err := os.ReadFile(abs)
	if err != nil {
		t.Fatalf("failed to read fixture %s: %v", abs, err)
	}
	return string(content)
}

func TestFixApplyAutofixes(t *testing.T) {
	original := readStatutesFixture(t, "🧪️file-fixable")
	expectedContent := readStatutesFixture(t, "🧪️file-fixable-expected")

	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	fixtureSrc := statutesFixtureDir + "/🧪️file-fixable/🟦️.tsx"
	srcAbs := filepath.Join(tmpDir, filepath.FromSlash(fixtureSrc))
	if err := os.MkdirAll(filepath.Dir(srcAbs), 0o755); err != nil {
		t.Fatalf("failed to create fixture dir: %v", err)
	}
	if err := os.WriteFile(srcAbs, []byte(original), 0o644); err != nil {
		t.Fatalf("failed to write fixture copy: %v", err)
	}

	bundles := codebase.LoadBundles()
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: fixtureSrc}
	ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{fixtureSrc})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	var autofixable []model.Breach
	for _, v := range breachs {
		if v.Autofixable() {
			autofixable = append(autofixable, v)
		}
	}
	if len(autofixable) == 0 {
		t.Fatal("expected autofixable breachs in fixture")
	}

	fixed, err := applyAutofixes(fixtureSrc, autofixable)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Error("expected at least one fix applied")
	}

	fixedContent, err := workspace.ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}

	if strings.TrimSpace(fixedContent) != strings.TrimSpace(expectedContent) {
		t.Errorf("fixed content does not match expected.\nGot:\n%s\n\nExpected:\n%s", fixedContent, expectedContent)
	}
}

func TestApplyAutofixesRunsFormatterAfterEdit(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	if err := workspace.WriteTextFile(filepath.Join(tmpDir, "package.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write package.json: %v", err)
	}
	if err := workspace.WriteTextFile(filepath.Join(tmpDir, ".prettierrc.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write .prettierrc.json: %v", err)
	}
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin", "prettier")
	if err := workspace.WriteTextFile(prettierBin, "#!/usr/bin/env sh\nexit 0\n"); err != nil {
		t.Fatalf("failed to write prettier stub: %v", err)
	}
	if err := os.Chmod(prettierBin, 0755); err != nil {
		t.Fatalf("failed to chmod prettier stub: %v", err)
	}

	originalLookup := workspace.FormatterBinaryLookup
	originalRun := workspace.FormatterCommandRun
	defer func() {
		workspace.FormatterBinaryLookup = originalLookup
		workspace.FormatterCommandRun = originalRun
	}()

	var ranBinary string
	var ranArgs []string
	var ranDir string
	workspace.FormatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}
	workspace.FormatterCommandRun = func(binary string, args []string, workDir string) error {
		ranBinary = binary
		ranArgs = append([]string{}, args...)
		ranDir = workDir
		return nil
	}

	testFile := "formatted.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖️A\n\nconst x = 1; // remove\n\n// #endregion 🔖️A\n"
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	expectedBinary := filepath.Join("node_modules", ".bin", "prettier")
	if ranBinary != expectedBinary {
		t.Fatalf("expected formatter to run with %q, got %q", expectedBinary, ranBinary)
	}
	if ranDir != tmpDir {
		t.Fatalf("expected formatter work dir %q, got %q", tmpDir, ranDir)
	}
	if len(ranArgs) < 3 || ranArgs[0] != "--write" || ranArgs[len(ranArgs)-1] != testFile {
		t.Fatalf("unexpected formatter args: %v", ranArgs)
	}
}

func TestApplyAutofixesFormatterFallbackNormalizesText(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	originalLookup := workspace.FormatterBinaryLookup
	defer func() { workspace.FormatterBinaryLookup = originalLookup }()
	workspace.FormatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}

	testFile := "fallback.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖️A\n\nconst x = 1;   \n\n// #endregion"
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}
	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	result, err := workspace.ReadTextFile(absPath)
	if err != nil {
		t.Fatalf("failed to read result: %v", err)
	}
	if strings.Contains(result, "   \n") {
		t.Fatalf("expected fallback formatter to trim trailing spaces, got %q", result)
	}
}

func TestFixSectionMissingEndName(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️MySection\n\nconst x = 1;\n\n// #endregion\n"
	expected := "// #region 🔖️MySection\n\nconst x = 1;\n\n// #endregion 🔖️MySection\n"

	testFile := "test_missing_end.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
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
}

func TestFixSectionNameMismatch(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Alpha\n\nconst x = 1;\n\n// #endregion 🔖️Beta\n"
	expected := "// #region 🔖️Alpha\n\nconst x = 1;\n\n// #endregion 🔖️Alpha\n"

	testFile := "test_mismatch.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionNameMismatch, Scope: testFile, Line: 5},
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
}

func TestFixSectionEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Keep\n\nconst x = 1;\n\n// #endregion 🔖️Keep\n\n// #region 🔖️Empty\n\n// #endregion 🔖️Empty\n\n// #region 🔖️Also\n\nconst y = 2;\n\n// #endregion 🔖️Also\n"
	expected := "// #region 🔖️Keep\n\nconst x = 1;\n\n// #endregion 🔖️Keep\n\n// #region 🔖️Also\n\nconst y = 2;\n\n// #endregion 🔖️Also\n"

	testFile := "test_empty.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionEmpty, Scope: testFile + "#Empty", Line: 7},
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
}

func TestFixInlineComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Section\n\n// inline one\n\n// inline two\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_inline.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

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
}

func TestFixBlockComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Section\n\n/* block comment */\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_block.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

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
}

func TestFixJSDocComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️Section\n\n/** jsdoc comment */\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_jsdoc.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentJSDoc, Scope: testFile, Line: 3},
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
}

func TestFixMultipleBreachsSameFile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := "// #region 🔖️A\n\n// bad comment\n\nconst a = 1;\n\n// #endregion\n\n// #region 🔖️B\n\n// another bad\n\nconst b = 2;\n\n// #endregion 🔖️Wrong\n"
	testFile := "test_multi.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 11},
		{Kind: model.BreachCodeSectionNameMismatch, Scope: testFile, Line: 15},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 4 {
		t.Errorf("expected 4 fixes, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖️A") {
		t.Error("expected missing end name to be fixed to A")
	}
	if !strings.Contains(result, "// #endregion 🔖️B") {
		t.Error("expected mismatch to be fixed to B")
	}
	if strings.Contains(result, "// bad comment") {
		t.Error("expected inline comment to be removed")
	}
	if strings.Contains(result, "// another bad") {
		t.Error("expected second inline comment to be removed")
	}
}

func TestFixImprovedCommentLogic(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	content := `// #region 🔖️Section
const x = 1; // trailing comment
// TODO: fix this
// this line is part of the todo description

// another normal comment
/* TODO: block todo */
const y = 2; // normal trailing
// #endregion 🎁️Fix
`

	testFile := "test_improved.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &statutes.PolicyContext{}
	lang := languages.NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	expectedBreachs := 3
	if len(breachs) != expectedBreachs {
		t.Errorf("expected %d breachs, got %d", expectedBreachs, len(breachs))
		for i, v := range breachs {
			t.Logf("Breach %d: %s at %d:%d", i, v.Kind, v.Line, v.Column)
		}
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}

	if fixed != 3 {
		t.Errorf("expected 3 fixes, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)

	if strings.Contains(result, "trailing comment") {
		t.Errorf("trailing comment should be removed")
	}
	if !strings.Contains(result, "const x = 1;") {
		t.Errorf("code 'const x = 1;' should be kept")
	}
	if !strings.Contains(result, "// TODO: fix this") {
		t.Errorf("TODO comment should be kept")
	}
	if !strings.Contains(result, "// this line is part of the todo description") {
		t.Errorf("TODO description should be kept")
	}
	if strings.Contains(result, "// another normal comment") {
		t.Errorf("normal comment should be removed")
	}
	if !strings.Contains(result, "/* TODO: block todo */") {
		t.Errorf("block TODO should be kept")
	}

	lines := strings.Split(result, "\n")
	foundX := false
	for _, l := range lines {
		if strings.HasPrefix(l, "const x = 1;") {
			foundX = true
			if strings.Contains(l, "//") {
				t.Errorf("line 2 should not contain comment: %q", l)
			}
			if strings.HasSuffix(l, " ") {
				t.Errorf("line 2 should be trimmed right: %q", l)
			}
		}
	}
	if !foundX {
		t.Errorf("did not find 'const x = 1;' line in result")
	}
}

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

func TestDefinitionNativeDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "/**") {
		t.Fatal("expected JSDoc opening after autofix")
	}
	if !strings.Contains(fixedContent, "**/") {
		t.Fatal("expected JSDoc closing after autofix")
	}
	if !strings.Contains(fixedContent, " * Does work.") {
		t.Fatal("expected summary line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * doWork MUST be idempotent.") {
		t.Fatal("expected spec line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, "§doWork") {
		t.Fatal("expected identification in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("expected identification emitted as a single JSDoc line after autofix")
	}
	if strings.Contains(fixedContent, " *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect doubled asterisk marker before definition identification after autofix")
	}
	if strings.Contains(fixedContent, " *\n * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect an extra blank JSDoc separator before definition identification after autofix")
	}
}

func TestPythonTripleQuoteDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, `"""Does work.`) {
		t.Fatal("expected triple-quote docstring with summary after autofix")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec line in docstring after autofix")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification in docstring after autofix")
	}
	if !strings.Contains(fixedContent, `"""`) {
		t.Fatal("expected closing triple-quote after autofix")
	}
	if strings.Contains(fixedContent, "# Does work.") {
		t.Fatal("# comment should be removed after autofix")
	}
}

func TestPythonTripleQuoteDocstringMerge(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    \"\"\"Does work.\"\"\"\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach for # comments above existing docstring")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "Does work.") {
		t.Fatal("expected existing summary preserved after merge")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec from # comment merged into docstring")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification merged into docstring")
	}
	if strings.Contains(fixedContent, "# do_work MUST") {
		t.Fatal("# comment should be removed after merge autofix")
	}
}

func TestSystemPolicy(t *testing.T) {
	t.Run("detects settings.json outside devcontainer", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		ctx := statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})
		breachs := statutes.SystemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachSystemDevcontainerVscodeSettingsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected settings-outside-devcontainer breach")
		}
	})
	t.Run("detects extensions.json missing devcontainer recommendations", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		ctx := statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})
		breachs := statutes.SystemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == model.BreachSystemDevcontainerVscodeExtensionsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected extensions-outside-devcontainer breach")
		}
	})
	t.Run("no extensions breach when workspace recommendations include devcontainer extensions", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python","golang.go","ms-vscode-remote.remote-containers"]}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		ctx := statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})
		breachs := statutes.SystemPolicy(ctx)
		for _, v := range breachs {
			if v.Kind == model.BreachSystemDevcontainerVscodeExtensionsOutside {
				t.Error("expected no extensions breach when workspace recommendations include devcontainer extensions")
			}
		}
	})
	t.Run("no breachs when .vscode files absent", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		ctx := statutes.NewPolicyContext(workspace.Scope{Kind: workspace.ScopeRepo}, []model.Bundle{})
		breachs := statutes.SystemPolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("autofix moves settings.json into devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		breachs := []model.Breach{
			{Kind: model.BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		if _, err := os.Stat(filepath.Join(tmpDir, ".vscode", "settings.json")); !os.IsNotExist(err) {
			t.Error("expected .vscode/settings.json to be removed")
		}
		dcPath := filepath.Join(tmpDir, ".devcontainer", "devcontainer.json")
		dcData, err := os.ReadFile(dcPath)
		if err != nil {
			t.Fatalf("expected devcontainer.json to exist: %v", err)
		}
		var dc map[string]interface{}
		if err := json.Unmarshal(dcData, &dc); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		if customizations == nil {
			t.Fatal("expected customizations key")
		}
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode == nil {
			t.Fatal("expected vscode key in customizations")
		}
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings == nil {
			t.Fatal("expected settings key in customizations.vscode")
		}
		if settings["editor.fontSize"] != float64(14) {
			t.Errorf("expected editor.fontSize=14, got %v", settings["editor.fontSize"])
		}
	})
	t.Run("autofix syncs extensions.json from devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		breachs := []model.Breach{
			{Kind: model.BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		extPath := filepath.Join(tmpDir, ".vscode", "extensions.json")
		if _, err := os.Stat(extPath); err != nil {
			t.Fatalf("expected .vscode/extensions.json to remain: %v", err)
		}
		extData, err := os.ReadFile(extPath)
		if err != nil {
			t.Fatalf("read extensions.json: %v", err)
		}
		var extFile map[string]interface{}
		if err := json.Unmarshal(extData, &extFile); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		recommendations, _ := extFile["recommendations"].([]interface{})
		if len(recommendations) != 2 {
			t.Fatalf("expected 2 recommendations, got %d", len(recommendations))
		}
		if recommendations[0] != "ms-python.python" {
			t.Errorf("expected first recommendation ms-python.python, got %v", recommendations[0])
		}
		if recommendations[1] != "golang.go" {
			t.Errorf("expected second recommendation golang.go, got %v", recommendations[1])
		}
	})
	t.Run("autofix merges into existing devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.tabSize": 2}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"name": "test", "image": "ubuntu"}`)
		breachs := []model.Breach{
			{Kind: model.BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		if dc["name"] != "test" {
			t.Errorf("expected existing name=test to be preserved, got %v", dc["name"])
		}
		if dc["image"] != "ubuntu" {
			t.Errorf("expected existing image=ubuntu to be preserved, got %v", dc["image"])
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings["editor.tabSize"] != float64(2) {
			t.Errorf("expected editor.tabSize=2, got %v", settings["editor.tabSize"])
		}
	})
	t.Run("autofix both settings and extensions together", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		workspace.WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		breachs := []model.Breach{
			{Kind: model.BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
			{Kind: model.BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 2 {
			t.Fatalf("expected 2 fixes, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode["settings"] == nil {
			t.Error("expected settings in devcontainer.json")
		}
		extData, _ := os.ReadFile(filepath.Join(tmpDir, ".vscode", "extensions.json"))
		var extFile map[string]interface{}
		json.Unmarshal(extData, &extFile)
		recommendations, _ := extFile["recommendations"].([]interface{})
		if len(recommendations) != 2 {
			t.Fatalf("expected synced extensions.json recommendations, got %v", recommendations)
		}
	})
	t.Run("policy registered with correct id", func(t *testing.T) {
		p, found := statutes.FindPolicy("system")
		if !found {
			t.Fatal("expected system policy to be registered")
		}
		if p.Name != "System" {
			t.Errorf("expected name System, got %s", p.Name)
		}
		kinds := p.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 statutes, got %d", len(kinds))
		}
		kindSet := map[model.Statute]bool{}
		for _, k := range kinds {
			kindSet[k] = true
		}
		if !kindSet[model.BreachSystemDevcontainerVscodeSettingsOutside] {
			t.Error("expected settings-outside-devcontainer kind")
		}
		if !kindSet[model.BreachSystemDevcontainerVscodeExtensionsOutside] {
			t.Error("expected extensions-outside-devcontainer kind")
		}
	})
	t.Run("statute meta is correct", func(t *testing.T) {
		settingsMeta := model.BreachSystemDevcontainerVscodeSettingsOutside.Info()
		if !settingsMeta.Autofixable {
			t.Error("expected settings breach to be autofixable")
		}
		if settingsMeta.Priority != model.BreachPriorityHigh {
			t.Error("expected settings breach to be high priority")
		}
		extMeta := model.BreachSystemDevcontainerVscodeExtensionsOutside.Info()
		if !extMeta.Autofixable {
			t.Error("expected extensions breach to be autofixable")
		}
		if extMeta.Priority != model.BreachPriorityHigh {
			t.Error("expected extensions breach to be high priority")
		}
	})
}

// 🕸️#region 🎙️GraphQL
func TestGraphQLRepoQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "compose") {
		t.Errorf("Expected result to contain 'compose', got: %s", result)
	}
}

// 🏗️withMonorepoFixture builds a throwaway monorepo in `t.TempDir()` and points the
// workspace at it, so the walk-backed queries assert against a tree this file owns
// instead of against the developer's own checkout.
func withMonorepoFixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	write := func(rel, content string) {
		abs := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
			t.Fatalf("mkdir %s: %v", rel, err)
		}
		if err := os.WriteFile(abs, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", rel, err)
		}
	}
	write("compose/README.md", "---\nname: compose\nkind: user\n---\n\n# compose\n")
	write("repo/README.md", "---\nname: repo\nkind: infrastructure\n---\n\n# repo\n")
	write("coda/README.md", "---\nname: coda\nkind: research\n---\n\n# coda\n")
	write("compose/js/AGENTS.md", "---\nbundle:\n  emoji: 📜️\n---\n")
	write("compose/js/index.ts", fixtureSectionedTypeScript)
	write("compose/go/AGENTS.md", "---\nbundle:\n  emoji: 🐹️\n---\n")
	write("compose/go/main.go", fixtureSectionedGo)
	write("repo/client/AGENTS.md", "---\nbundle:\n  emoji: ⌨️\n---\n")
	write("repo/client/main.go", fixtureSectionedGo)
	previous := workspace.RootDir
	workspace.RootDir = root
	codebase.InvalidateTechnologyCache()
	t.Cleanup(func() {
		workspace.RootDir = previous
		codebase.InvalidateTechnologyCache()
	})
	return root
}

const fixtureSectionedTypeScript = `// #region 🔖️Header
// [💻️js/index.ts](repo://file/💻️index)
// #endregion 🔖️Header

// #region 🧩️State
export const state = 1;

// #region 🏪️Store
export const store = 2;
// #endregion 🏪️Store
// #endregion 🧩️State
`

const fixtureSectionedGo = `// #region 🔖️Header
// [💻️client/main.go](repo://file/💻️main)
// #endregion 🔖️Header

package main

// #region 🎖️Entry
func main() {}
// #endregion 🎖️Entry
`

func TestGraphQLBundlesQuery(t *testing.T) {
	root := withMonorepoFixture(t)
	fixtureExecutor, err := NewExecutorWithContext(root, NewRepoContext(root))
	if err != nil {
		t.Fatalf("failed to create fixture executor: %v", err)
	}
	result, err := fixtureExecutor.ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Fatalf("ExecuteGraphQL bundles returned error: %v", err)
	}
	var payload struct {
		Repo struct {
			Bundles []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Root string `json:"root"`
			} `json:"bundles"`
		} `json:"repo"`
	}
	if err := json.Unmarshal([]byte(result), &payload); err != nil {
		t.Fatalf("failed to decode bundles payload %s: %v", result, err)
	}
	names := map[string]string{}
	for _, bundle := range payload.Repo.Bundles {
		if bundle.ID == "" {
			t.Errorf("bundle %q has an empty id in %s", bundle.Name, result)
		}
		names[bundle.Name] = bundle.Root
	}
	for _, want := range []string{"compose/js", "compose/go", "repo/client"} {
		if _, ok := names[want]; !ok {
			t.Errorf("Expected bundles to contain %q, got: %s", want, result)
		}
	}
	if len(payload.Repo.Bundles) != 3 {
		t.Errorf("expected exactly the fixture's three bundles, got %d: %s", len(payload.Repo.Bundles), result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

func TestTicketLifecycle_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle no-management test in short mode")
	}
	tmpDir := t.TempDir()

	run := func(name string, field ...string) {
		cmd := exec.Command(name, field...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("run %s %v failed: %v\nOutput: %s", name, field, err, out)
		}
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")

	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	goal, err := tickets.OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}

	contributors.TestSessionIDOverride = "session-open-1"
	defer func() { contributors.TestSessionIDOverride = "" }()
	ticket, err := tickets.OpenTicket("🎫️", "Test Title NoGH", "Test Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("OpenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.Management != nil {
		t.Error("OpenTicket: GitHub data should be nil")
	}
	openSessionCount := len(ticket.Sessions)
	if openSessionCount == 0 {
		t.Fatal("OpenTicket must persist one session")
	}

	testFile := "test.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("content"), 0644); err != nil {
		t.Fatal(err)
	}

	if goal.Title != "Goal Title" {
		t.Errorf("expected title 'Goal Title', got '%s'", goal.Title)
	}
	if goal.Prompt != "Goal Prompt" {
		t.Errorf("expected prompt 'Goal Prompt', got '%s'", goal.Prompt)
	}
	if goal.Client != "copilot-chat" {
		t.Errorf("expected ui 'copilot-chat', got '%s'", goal.Client)
	}
	if goal.LLM != "gemini-3-pro" {
		t.Errorf("expected llm 'gemini-3-pro', got '%s'", goal.LLM)
	}
	if goal.Management != nil {
		t.Error("OpenGoal: GitHub data should be nil")
	}

	goalPath := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎯️goals", "GOAL-TITLE", "🎯️goal.json")
	if _, err := os.Stat(goalPath); os.IsNotExist(err) {
		t.Errorf("goal file not created at %s", goalPath)
	}

	run("git", "add", testFile)
	run("git", "commit", "-m", "add test file")

	err = tickets.FinishTicket(ticket, "Summary", []string{testFile}, true, false)
	if err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.GetStatus() != model.TicketStatusClosed {
		t.Errorf("Ticket status mismatch: got %v, want closed", ticket.GetStatus())
	}
	if len(ticket.Interactions) < 2 {
		t.Fatalf("expected at least 2 interactions after close, got %d", len(ticket.Interactions))
	}
	if ticket.Interactions[0].Kind != "ticket.open" {
		t.Errorf("interaction[0].Kind = %q, want %q", ticket.Interactions[0].Kind, "ticket.open")
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.close" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.close")
	}
	closeFiles := ticket.Interactions[len(ticket.Interactions)-1].Files
	if len(closeFiles) != 1 || closeFiles[0].Path != testFile {
		t.Fatalf("no-management close files = %#v, want %q", closeFiles, testFile)
	}
	if len(ticket.Sessions) != openSessionCount {
		t.Fatalf("FinishTicket must not append sessions: before=%d after=%d", openSessionCount, len(ticket.Sessions))
	}

	contributors.TestSessionIDOverride = "session-reopen-2"
	err = tickets.ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "", "copilot-chat", "", "", "", true, providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("ReopenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.GetStatus() != model.TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.reopen" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.reopen")
	}
	if len(ticket.Sessions) != openSessionCount+1 {
		t.Fatalf("ReopenTicket must append exactly one session: expected=%d got=%d", openSessionCount+1, len(ticket.Sessions))
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := model.GoalCreateInput{
		Title:        "Test Goal NoGH 2",
		Description:  "Desc",
		Prompt:       "Prompt",
		DueDate:      "2026-02-15",
		Client:       "cursor",
		LLM:          "gpt-5-2-codex",
		NoManagement: true,
	}

	goal2, err := ctx.GoalCreate(goalInput)
	if err != nil {
		t.Fatalf("GoalCreate failed: %v", err)
	}
	if goal2.Title != "Test Goal NoGH 2" {
		t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
	}

	_, err = ctx.GoalClose(model.GoalCloseInput{ID: goal2.ID, Summary: "Done", NoManagement: true})
	if err != nil {
		t.Fatalf("GoalClose failed: %v", err)
	}
	closedGoalPath := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎯️goals", goal2.ID, "🎯️goal.json")
	closedGoalContent, err := workspace.ReadTextFile(closedGoalPath)
	if err != nil {
		t.Fatalf("failed to read closed goal: %v", err)
	}
	var closedGoal model.Goal
	if err := json.Unmarshal([]byte(closedGoalContent), &closedGoal); err != nil {
		t.Fatalf("failed to unmarshal closed goal: %v", err)
	}
}

// ♻️TestRepoContextTodoChange MUST rewrite the matching TODO source line and return an error for an unknown ID.
// ✏️TestRepoContextTodoChange verifies the repoContext TodoChange mutation against a temp-dir-isolated fixture.
func TestRepoContextTodoChange(t *testing.T) {
	root := t.TempDir()

	mdDir := filepath.Join(root, "sub")
	if err := os.MkdirAll(mdDir, 0755); err != nil {
		t.Fatal(err)
	}
	todosPath := filepath.Join(mdDir, ".todos.md")
	if err := os.WriteFile(todosPath, []byte("- TODO OriginalName: original description\n"), 0644); err != nil {
		t.Fatal(err)
	}

	codePath := filepath.Join(root, "main.go")
	if err := os.WriteFile(codePath, []byte("package main\n\n// TODO CodeTodo: code description\nfunc main() {}\n"), 0644); err != nil {
		t.Fatal(err)
	}

	ctx := &RepoContext{rootDir: root}

	name := "RenamedName"
	description := "renamed description"
	mdID := workspace.Slugify("OriginalName")
	changed, err := ctx.TodoChange(model.TodoChangeInput{ID: mdID, Name: &name, Description: &description})
	if err != nil {
		t.Fatalf("markdown TodoChange returned error: %v", err)
	}
	if changed.Name != "RenamedName" || changed.Description != "renamed description" {
		t.Fatalf("unexpected changed todo: %+v", changed)
	}
	content, _ := os.ReadFile(todosPath)
	if !strings.Contains(string(content), "- TODO RenamedName: renamed description") {
		t.Fatalf("markdown file not updated, got: %s", content)
	}
	if strings.Contains(string(content), "OriginalName") {
		t.Fatalf("old name still present: %s", content)
	}

	codeName := "CodeTodoRenamed"
	codeDescription := "new code description"
	codeID := workspace.Slugify("CodeTodo")
	changedCode, err := ctx.TodoChange(model.TodoChangeInput{ID: codeID, Name: &codeName, Description: &codeDescription})
	if err != nil {
		t.Fatalf("inline TodoChange returned error: %v", err)
	}
	if changedCode.Name != "CodeTodoRenamed" || changedCode.Description != "new code description" {
		t.Fatalf("unexpected changed code todo: %+v", changedCode)
	}
	codeContent, _ := os.ReadFile(codePath)
	if !strings.Contains(string(codeContent), "// TODO CodeTodoRenamed: new code description") {
		t.Fatalf("code file not updated, got: %s", codeContent)
	}

	missingName := "X"
	if _, err := ctx.TodoChange(model.TodoChangeInput{ID: "does-not-exist", Name: &missingName}); err == nil {
		t.Fatal("expected error for missing todo")
	}
}

func TestFixHeaderWithShebang(t *testing.T) {
	tmpDir := t.TempDir()
	originalRootDir := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(originalRootDir)

	filePath := "script.py"
	absPath := filepath.Join(tmpDir, filePath)
	content := "#!/usr/bin/env python3\n" +
		"#region 🔖️Header\n\n" +
		"# wrong/path.py\n\n" +
		"# 2025 Test <t@t.com>\n\n" +
		"# #region 🔖️License\n" +
		"# AGPL\n" +
		"# #endregion 🔖️License\n\n" +
		"# #region 🔖️Requirements\n" +
		"# 💯️Requirements\n" +
		"# #endregion 🔖️Requirements\n\n" +
		"#endregion 🔖️Header\n\n" +
		"print(\"hello\")\n"
	if err := os.WriteFile(absPath, []byte(content), 0644); err != nil {
		t.Fatalf("failed to write fixture: %v", err)
	}

	bundles := codebase.LoadBundles()
	autofixableBreachs := func() []model.Breach {
		t.Helper()
		breachs, err := statutes.CheckPolicies(workspace.ParseScope(filePath), bundles, nil)
		if err != nil {
			t.Fatalf("CheckPolicies failed: %v", err)
		}
		pending := make([]model.Breach, 0, len(breachs))
		for _, v := range breachs {
			if v.Autofixable() {
				pending = append(pending, v)
			}
		}
		return pending
	}

	before := autofixableBreachs()
	if len(before) == 0 {
		t.Fatal("expected autofixable breachs in the shebang fixture")
	}

	fixed, err := applyAutofixes(filePath, before)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Fatal("expected at least one fix applied")
	}

	newContent, err := workspace.ReadTextFile(absPath)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}
	if !strings.HasPrefix(newContent, "#!/usr/bin/env python3\n") {
		t.Errorf("shebang must stay on the first line of the fixed content:\n%s", newContent)
	}
	if !strings.Contains(newContent, "print(\"hello\")") {
		t.Errorf("body lost by the autofix:\n%s", newContent)
	}

	after := autofixableBreachs()
	if len(after) >= len(before) {
		t.Errorf("autofix did not reduce the autofixable breachs: %d before, %d after", len(before), len(after))
	}
}

func TestFolderPolicyEmptyFolderAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "remove", "me")
	os.MkdirAll(emptyDir, 0755)
	breachs := []model.Breach{{
		Kind:    model.BreachFolderIllegalEmpty,
		Scope:   "remove/me/",
		Excerpt: "remove/me",
	}}
	fixed, err := applySystemAutofixes(breachs)
	if err != nil {
		t.Fatalf("applySystemAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}
	if _, statErr := os.Stat(emptyDir); !os.IsNotExist(statErr) {
		t.Error("empty folder should have been removed")
	}
}

func TestSectionNewlineAfterRegion(t *testing.T) {
	t.Run("detect_blank_line_after_region_typescript", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_go", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.go](repo://file/test.go)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Package\n\n// [🔖️test.go#Package](repo://section/test.go/Package)\n// Package declaration.\n\npackage main\n\n// #endregion 🔖️Package\n"
		testFile := "test.go"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Package section")
		}
	})

	t.Run("detect_blank_line_after_region_python", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "# region Header\n# [💻️test.py](repo://file/test.py)\n# 2025 Test <t@t.com>\n# AGPL\n# endregion Header\n\n# region Functions\n\n# [🔖️test.py#Functions](repo://section/test.py/Functions)\n# Utility functions.\n\ndef add(a, b):\n    return a + b\n\n# endregion Functions\n"
		testFile := "test.py"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_csharp", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "#region 🔖️Header\n// [💻️test.cs](repo://file/test.cs)\n// 2025 Test <t@t.com>\n// AGPL\n#endregion 🔖️Header\n\n#region 🔖️Classes\n\n// [🔖️test.cs#Classes](repo://section/test.cs/Classes)\n// Domain classes.\n\npublic class Foo {}\n\n#endregion 🔖️Classes\n"
		testFile := "test.cs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Classes section")
		}
	})

	t.Run("detect_blank_line_after_region_rust", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\npub mod structs { // 🔖️Structs\n\n// Struct definitions.\n\nstruct Foo {}\n\n} // 🔖️Structs\n"
		testFile := "test.rs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Structs section")
		}
	})

	t.Run("no_false_positive_without_blank_line", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		for _, v := range breachs {
			if v.Kind == model.BreachCodeSectionWrongFormatNewlineAfterRegion {
				t.Fatal("unexpected newline-after-region breach when no blank line exists")
			}
		}
	})

	t.Run("autofix_removes_blank_line_after_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		expected := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []model.Breach{
			{Kind: model.BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Functions", Line: 8},
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

	t.Run("autofix_removes_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		expected := "// #region 🔖️Header\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// [🔖️test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []model.Breach{
			{Kind: model.BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Header", Line: 2},
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

	t.Run("detect_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := workspace.RootDir
		workspace.RootDir = tmpDir
		defer func() { workspace.RootDir = oldRoot }()
		content := "// #region 🔖️Header\n\n// [💻️test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖️Header\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := workspace.WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []model.Bundle{}
		scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
		ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[model.Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[model.BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Header section")
		}
	})
}

// 🌐️TestGraphQLEffortMutationsAndQueries tests that GraphQL mutations and queries support reasoning effort.
func TestGraphQLEffortMutationsAndQueries(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	schema, err := buildSchema(NewResolver(tmpDir))
	if err != nil {
		t.Fatalf("buildSchema failed: %v", err)
	}

	goalMutation := `
		mutation {
			goalCreate(input: {
				title: "GQL Goal",
				description: "GQL Goal Desc",
				prompt: "GQL Goal Prompt",
				dueDate: "2026-12-31",
				client: "copilot-chat",
				llm: "gemini-3-7-pro",
				effort: "high",
				noManagement: true
			}) {
				id
				title
				llm
				effort
			}
		}
	`
	result := Do(Params{
		Schema:        schema,
		RequestString: goalMutation,
		Context:       context.Background(),
	})
	if len(result.Errors) > 0 {
		t.Fatalf("goalCreate GraphQL errors: %v", result.Errors)
	}
	goalData := result.Data.(map[string]interface{})["goalCreate"].(map[string]interface{})
	if goalData["effort"] != "high" {
		t.Errorf("goal.effort = %v, want 'high'", goalData["effort"])
	}
	if goalData["llm"] != "gemini-3-7-pro" {
		t.Errorf("goal.llm = %v, want 'gemini-3-7-pro'", goalData["llm"])
	}

	ticketMutation := `
		mutation {
			ticketOpen(input: {
				emoji: "🎫️",
				title: "GQL Ticket",
				prompt: "GQL Ticket Prompt",
				client: COPILOT_CHAT,
				llm: "opus-4-7",
				effort: "max",
				goal: "GQL-GOAL",
				noManagement: true
			}) {
				year
				month
				day
				slug
				effort
				llm
			}
		}
	`
	tResult := Do(Params{
		Schema:        schema,
		RequestString: ticketMutation,
		Context:       context.Background(),
	})
	if len(tResult.Errors) > 0 {
		t.Fatalf("ticketOpen GraphQL errors: %v", tResult.Errors)
	}
	ticketData := tResult.Data.(map[string]interface{})["ticketOpen"].(map[string]interface{})
	if ticketData["effort"] != "max" {
		t.Errorf("ticket.effort = %v, want 'max'", ticketData["effort"])
	}
	if ticketData["llm"] != "opus-4-7" {
		t.Errorf("ticket.llm = %v, want 'opus-4-7'", ticketData["llm"])
	}
}
