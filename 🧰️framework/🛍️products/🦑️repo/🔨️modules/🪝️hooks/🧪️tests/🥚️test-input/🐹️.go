// 🧪️ Tests of the extraction of test starting and ended events from a hook input.

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestExtractTestStartingFromInputResolvesTestIDs(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

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
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved from command, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
		if !strings.Contains(labs[0], model.EmojiText(model.EmojiFileLab)) {
			t.Errorf("expected normalized lab IDs, got %v", labs)
		}
	})

	t.Run("command_from_tool_input", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":"go test -v ./mypackage/..."},"tool_info":{"cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
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
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full suite, got %v", tests)
		}
		if !strings.Contains(labs[0], model.EmojiText(model.EmojiFileLab)) || !strings.Contains(labs[0], "mypackage") {
			t.Errorf("expected normalized lab id for my_test.go, got %v", labs)
		}
	})

	t.Run("go_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -run TestOne ./mypackage/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
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
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"python -m pytest tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped pytest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped pytest full suite, got %v", tests)
		}
	})

	t.Run("uv_pytest_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"uv run pytest -k test_alpha tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
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
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"npx vitest run src/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped vitest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped vitest full suite, got %v", tests)
		}
	})

	t.Run("bundle_rspec_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"bundle exec rspec --example alpha","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted rspec run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted rspec run, got none")
		}
	})

	t.Run("phpunit_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"./vendor/bin/phpunit tests/","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
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
	sandbox.BindRoot(t, tmpDir)

	goDir := filepath.Join(tmpDir, "pkg")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "pkg_test.go"), []byte("package pkg\nfunc TestX(t *testing.T) {}\n"), 0644)

	t.Run("resolves_files_from_command", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test ./pkg/...","cwd":"%s"}}`, filepath.ToSlash(tmpDir)))
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
