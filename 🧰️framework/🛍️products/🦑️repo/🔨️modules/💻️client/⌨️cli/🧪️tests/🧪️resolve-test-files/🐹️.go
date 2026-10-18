// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Test file and definition ID resolution from hook input.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestResolveTestFilesFromCommand(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "pkg", "main")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "main.go"), []byte("package main\n"), 0644)
	os.WriteFile(filepath.Join(goDir, "main_test.go"), []byte("package main\nfunc TestFoo(t *testing.T) {}\n"), 0644)
	subDir := filepath.Join(goDir, "sub")
	os.MkdirAll(subDir, 0755)
	os.WriteFile(filepath.Join(subDir, "sub_test.go"), []byte("package sub\nfunc TestBar(t *testing.T) {}\n"), 0644)

	pyDir := filepath.Join(tmpDir, "tests")
	os.MkdirAll(pyDir, 0755)
	os.WriteFile(filepath.Join(pyDir, "test_foo.py"), []byte("def test_foo(): pass\n"), 0644)
	os.WriteFile(filepath.Join(pyDir, "helper.py"), []byte("def helper(): pass\n"), 0644)

	jsDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(jsDir, 0755)
	os.WriteFile(filepath.Join(jsDir, "app.test.ts"), []byte("test('works', () => {})\n"), 0644)
	os.WriteFile(filepath.Join(jsDir, "app.ts"), []byte("export const a = 1\n"), 0644)

	rsDir := filepath.Join(tmpDir, "rsrc", "tests")
	os.MkdirAll(rsDir, 0755)
	os.WriteFile(filepath.Join(rsDir, "integration_test.rs"), []byte("#[test]\nfn test_it() {}\n"), 0644)

	specDir := filepath.Join(tmpDir, "spec")
	os.MkdirAll(specDir, 0755)
	os.WriteFile(filepath.Join(specDir, "app_spec.rb"), []byte("describe App do; end\n"), 0644)

	phpDir := filepath.Join(tmpDir, "tests", "Feature")
	os.MkdirAll(phpDir, 0755)
	os.WriteFile(filepath.Join(phpDir, "ExampleTest.php"), []byte("<?php\nfinal class ExampleTest {}\n"), 0644)

	t.Run("go_test_recursive", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test -v ./pkg/main/...", tmpDir)
		if len(files) != 2 {
			t.Fatalf("expected 2 Go test files, got %d: %v", len(files), files)
		}
		found := map[string]bool{}
		for _, f := range files {
			found[filepath.Base(f)] = true
		}
		if !found["main_test.go"] || !found["sub_test.go"] {
			t.Errorf("expected main_test.go and sub_test.go, got %v", files)
		}
	})

	t.Run("go_test_single_package", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test ./pkg/main", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Go test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "main_test.go" {
			t.Errorf("expected main_test.go, got %s", files[0])
		}
	})

	t.Run("go_test_with_run_flag", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test -run TestFoo -v ./pkg/main/...", tmpDir)
		if len(files) != 2 {
			t.Fatalf("expected 2 Go test files (run flag doesn't filter files), got %d: %v", len(files), files)
		}
	})

	t.Run("pytest_directory", func(t *testing.T) {
		files := resolveTestFilesFromCommand("pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "test_foo.py" {
			t.Errorf("expected test_foo.py, got %s", files[0])
		}
	})

	t.Run("pytest_no_args", func(t *testing.T) {
		files := resolveTestFilesFromCommand("pytest", tmpDir)

		foundTestFile := false
		for _, f := range files {
			if filepath.Base(f) == "test_foo.py" {
				foundTestFile = true
			}
		}
		if !foundTestFile {
			t.Errorf("expected to find test_foo.py, got %v", files)
		}
	})

	t.Run("python_m_pytest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("python -m pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("uv_run_pytest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("uv run pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("npx_vitest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("npx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app.test.ts" {
			t.Errorf("expected app.test.ts, got %s", files[0])
		}
	})

	t.Run("cargo_test", func(t *testing.T) {
		files := resolveTestFilesFromCommand("cargo test", filepath.Join(tmpDir, "rsrc"))
		if len(files) != 1 {
			t.Fatalf("expected 1 Rust test file, got %d: %v", len(files), files)
		}
	})

	t.Run("bunx_vitest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("bunx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
	})

	t.Run("rspec", func(t *testing.T) {
		files := resolveTestFilesFromCommand("rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("bundle_exec_rspec", func(t *testing.T) {
		files := resolveTestFilesFromCommand("bundle exec rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("vendor_phpunit", func(t *testing.T) {
		files := resolveTestFilesFromCommand("./vendor/bin/phpunit tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 PHP test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "ExampleTest.php" {
			t.Errorf("expected ExampleTest.php, got %s", files[0])
		}
	})

	t.Run("unsupported_command", func(t *testing.T) {
		files := resolveTestFilesFromCommand("echo hello", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for unsupported command, got %d", len(files))
		}
	})

	t.Run("empty_command", func(t *testing.T) {
		files := resolveTestFilesFromCommand("", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for empty command, got %d", len(files))
		}
	})
}

func TestResolveAllTestDefinitionIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	t.Run("go_test_file", func(t *testing.T) {
		goDir := filepath.Join(tmpDir, "pkg")
		os.MkdirAll(goDir, 0755)
		goContent := `package pkg
// 🧪️#region 📐️Tests
func TestAlpha(t *testing.T) {}
func TestBeta(t *testing.T) {}
func helperNotATest() {}
// #endregion 📐️Tests
`
		testFile := filepath.Join(goDir, "pkg_test.go")
		os.WriteFile(testFile, []byte(goContent), 0644)

		ids := resolveAllTestDefinitionIDs([]string{"pkg/pkg_test.go"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}

		foundAlpha := false
		foundBeta := false
		for _, id := range ids {
			if strings.Contains(id, "testalpha") {
				foundAlpha = true
			}
			if strings.Contains(id, "testbeta") {
				foundBeta = true
			}
		}
		if !foundAlpha {
			t.Errorf("expected to find TestAlpha in IDs, got %v", ids)
		}
		if !foundBeta {
			t.Errorf("expected to find TestBeta in IDs, got %v", ids)
		}

		for _, id := range ids {
			if strings.Contains(id, "helpernotatest") {
				t.Errorf("expected helperNotATest to be excluded, but found in IDs: %s", id)
			}
		}
	})

	t.Run("python_test_file", func(t *testing.T) {
		pyDir := filepath.Join(tmpDir, "pytests")
		os.MkdirAll(pyDir, 0755)
		pyContent := `def test_foo():
    pass

def test_bar():
    pass

def helper():
    pass
`
		testFile := filepath.Join(pyDir, "test_stuff.py")
		os.WriteFile(testFile, []byte(pyContent), 0644)

		ids := resolveAllTestDefinitionIDs([]string{"pytests/test_stuff.py"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}
		foundFoo := false
		foundBar := false
		for _, id := range ids {
			if strings.Contains(id, "testfoo") {
				foundFoo = true
			}
			if strings.Contains(id, "testbar") {
				foundBar = true
			}
		}
		if !foundFoo {
			t.Errorf("expected test_foo in IDs, got %v", ids)
		}
		if !foundBar {
			t.Errorf("expected test_bar in IDs, got %v", ids)
		}
	})

	t.Run("no_files", func(t *testing.T) {
		ids := resolveAllTestDefinitionIDs(nil)
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nil files, got %d", len(ids))
		}
	})

	t.Run("nonexistent_file", func(t *testing.T) {
		ids := resolveAllTestDefinitionIDs([]string{"nonexistent/file_test.go"})
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nonexistent file, got %d", len(ids))
		}
	})
}

func TestExtractTestStartingFromInputResolvesTestIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "mypackage")
	os.MkdirAll(goDir, 0755)
	goContent := `package mypackage
func TestOne(t *testing.T) {}
func TestTwo(t *testing.T) {}
`
	os.WriteFile(filepath.Join(goDir, "my_test.go"), []byte(goContent), 0644)

	pyDir := filepath.Join(tmpDir, "tests")
	os.MkdirAll(pyDir, 0755)
	pyContent := `def test_alpha():
    pass

def test_beta():
    pass
`
	os.WriteFile(filepath.Join(pyDir, "test_sample.py"), []byte(pyContent), 0644)

	jsDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(jsDir, 0755)
	jsContent := `import { describe, it } from 'vitest'

describe('suite', () => {
  it('alpha', () => {})
  it('beta', () => {})
})
`
	os.WriteFile(filepath.Join(jsDir, "app.test.ts"), []byte(jsContent), 0644)

	rubyDir := filepath.Join(tmpDir, "spec")
	os.MkdirAll(rubyDir, 0755)
	rubyContent := `describe 'suite' do
  it 'alpha' do
  end

  it 'beta' do
  end
end
`
	os.WriteFile(filepath.Join(rubyDir, "app_spec.rb"), []byte(rubyContent), 0644)

	phpDir := filepath.Join(tmpDir, "tests", "Feature")
	os.MkdirAll(phpDir, 0755)
	phpContent := `<?php
final class ExampleTest extends TestCase {
    public function testAlpha(): void {}
    public function testBeta(): void {}
}
`
	os.WriteFile(filepath.Join(phpDir, "ExampleTest.php"), []byte(phpContent), 0644)

	t.Run("command_from_tool_info", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved from command, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
		if !strings.Contains(labs[0], emojiText(EmojiFileLab)) {
			t.Errorf("expected normalized lab IDs, got %v", labs)
		}
	})

	t.Run("command_from_tool_input", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":"go test -v ./mypackage/..."},"tool_info":{"cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
	})

	t.Run("explicit_test_names_not_overwritten", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"tests":["TestSpecific"],"files":["some_test.go"]}}`)
		_, tests, _ := extractTestStartingFromInput(input, "")

		if len(tests) != 1 || tests[0] != "TestSpecific" {
			t.Errorf("expected explicit test name to be preserved, got %v", tests)
		}
	})

	t.Run("go_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full suite, got %v", tests)
		}
		if !strings.Contains(labs[0], emojiText(EmojiFileLab)) || !strings.Contains(labs[0], "mypackage") {
			t.Errorf("expected normalized lab id for my_test.go, got %v", labs)
		}
	})

	t.Run("go_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -run TestOne ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testone") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted TestOne id, got %v", tests)
		}
	})

	t.Run("python_wrapped_runner_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"python -m pytest tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped pytest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped pytest full suite, got %v", tests)
		}
	})

	t.Run("uv_pytest_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"uv run pytest -k test_alpha tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted uv pytest run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted uv pytest run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testalpha") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted test_alpha id, got %v", tests)
		}
	})

	t.Run("vitest_wrapped_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"npx vitest run src/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped vitest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped vitest full suite, got %v", tests)
		}
	})

	t.Run("bundle_rspec_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"bundle exec rspec --example alpha","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted rspec run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted rspec run, got none")
		}
	})

	t.Run("phpunit_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"./vendor/bin/phpunit tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for phpunit full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for phpunit full suite, got %v", tests)
		}
	})
}

func TestExtractTestEndedFromInputResolvesFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "pkg")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "pkg_test.go"), []byte("package pkg\nfunc TestX(t *testing.T) {}\n"), 0644)

	t.Run("resolves_files_from_command", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test ./pkg/...","cwd":"%s"}}`, tmpDir))
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) == 0 {
			t.Fatalf("expected files to be resolved from command, got empty")
		}
	})

	t.Run("explicit_files_preserved", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"files":["explicit_test.go"]},"tool_output":{"succeeded":["TestX"]}}`)
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) != 1 || files[0] != "explicit_test.go" {
			t.Errorf("expected explicit file, got %v", files)
		}
	})
}
