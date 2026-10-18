// 🔬️ Tests of the testrunner domain, split out of the pre-split godfile suite.

package testrunner

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	languages "github.com/usalu/semio/repo/languages"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestDetectBundleLanguage(t *testing.T) {

	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

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

	scopes := ResolveTestScopes(nil)
	if len(scopes) != 1 {
		t.Fatalf("resolveTestScopes(nil) len = %d, want 1", len(scopes))
	}
	if scopes[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes(nil)[0].Kind = %q, want %q", scopes[0].Kind, testScopeAll)
	}

	scopes2 := ResolveTestScopes([]string{})
	if len(scopes2) != 1 || scopes2[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes([]) should return testScopeAll scope")
	}

	scopes3 := ResolveTestScopes([]string{"notavalidid"})
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

func TestParseTestInfoFromCommand(t *testing.T) {
	cases := []struct {
		name          string
		command       string
		expectTests   []string
		expectTimeout string
	}{
		{"go test all", "go test ./...", []string{""}, ""},
		{"go test -run", "go test -run TestFoo ./...", []string{"TestFoo"}, ""},
		{"go test -run -timeout", "go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"go test -timeout 2m", "go test -timeout 2m ./...", []string{""}, "120"},
		{"pytest all", "pytest", []string{""}, ""},
		{"pytest -k", "pytest -k test_foo", []string{"test_foo"}, ""},
		{"jest all", "jest", []string{""}, ""},
		{"jest -t", "jest -t MyTest", []string{"MyTest"}, ""},
		{"jest --testNamePattern", "jest --testNamePattern=MyTest", []string{"MyTest"}, ""},
		{"jest --timeout", "jest --timeout 5000", []string{""}, "5000"},
		{"mocha --grep", "mocha --grep mytest", []string{"mytest"}, ""},
		{"mocha --timeout", "mocha --timeout 3000 test/", []string{""}, "3000"},
		{"cargo test -- filter", "cargo test -- my_test", []string{"my_test"}, ""},
		{"dotnet --filter", "dotnet test --filter Category=Unit", []string{"Category=Unit"}, ""},
		{"rspec --example", "rspec --example myexample", []string{"myexample"}, ""},

		{"cd && go test -run", "cd /path && go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"cd && go test all", "cd /path && go test ./...", []string{""}, ""},
		{"cd && go test piped", "cd /workspaces/semio/repo/client && go test -v -run TestBar -timeout 60s 2>&1 | tail -80", []string{"TestBar"}, "60"},
		{"cd && pytest -k", "cd /app && pytest -k test_integration", []string{"test_integration"}, ""},
		{"cd && jest -t", "cd frontend && jest -t MyComponent", []string{"MyComponent"}, ""},
		{"cd; cargo test -- filter", "cd /path; cargo test -- my_test", []string{"my_test"}, ""},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v -run TestBaz ./...", []string{"TestBaz"}, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			tests, timeout := ParseTestInfoFromCommand(tc.command)
			if len(tests) != len(tc.expectTests) {
				t.Errorf("parseTestInfoFromCommand(%q) tests = %v, want %v", tc.command, tests, tc.expectTests)
			} else {
				for i := range tests {
					if tests[i] != tc.expectTests[i] {
						t.Errorf("parseTestInfoFromCommand(%q) tests[%d] = %q, want %q", tc.command, i, tests[i], tc.expectTests[i])
					}
				}
			}
			if timeout != tc.expectTimeout {
				t.Errorf("parseTestInfoFromCommand(%q) timeout = %q, want %q", tc.command, timeout, tc.expectTimeout)
			}
		})
	}
}

func TestExtractTestSegmentFromCommand(t *testing.T) {
	cases := []struct {
		name      string
		command   string
		expectSeg string
		expectCwd string
	}{
		{"simple go test", "go test ./...", "go test ./...", ""},
		{"cd && go test", "cd /workspaces/semio/repo/client && go test -v -run TestFoo ./...", "go test -v -run TestFoo ./...", "/workspaces/semio/repo/client"},
		{"cd && go test piped", "cd /path && go test -v ./... 2>&1 | tail -80", "go test -v ./... 2>&1", "/path"},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /src && go test -v ./...", "go test -v ./...", "/src"},
		{"cd && npm test", "cd frontend && npm test", "npm test", "frontend"},
		{"cd; cargo test", "cd /path; cargo test", "cargo test", "/path"},
		{"no test segment", "cd /path && go build ./...", "", "/path"},
		{"empty", "", "", ""},
		{"just cd", "cd /path", "", ""},
		{"piped go test", "go test -v ./... | head -50", "go test -v ./...", ""},
		{"vitest piped", "vitest run 2>&1 | tail -20", "vitest run 2>&1", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			seg, cwd := ExtractTestSegmentFromCommand(tc.command)
			if seg != tc.expectSeg {
				t.Errorf("extractTestSegmentFromCommand(%q) seg = %q, want %q", tc.command, seg, tc.expectSeg)
			}
			if cwd != tc.expectCwd {
				t.Errorf("extractTestSegmentFromCommand(%q) cwd = %q, want %q", tc.command, cwd, tc.expectCwd)
			}
		})
	}
}

func TestResolveTestFilesFromCommand(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

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
		files := ResolveTestFilesFromCommand("go test -v ./pkg/main/...", tmpDir)
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
		files := ResolveTestFilesFromCommand("go test ./pkg/main", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Go test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "main_test.go" {
			t.Errorf("expected main_test.go, got %s", files[0])
		}
	})

	t.Run("go_test_with_run_flag", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("go test -run TestFoo -v ./pkg/main/...", tmpDir)
		if len(files) != 2 {
			t.Fatalf("expected 2 Go test files (run flag doesn't filter files), got %d: %v", len(files), files)
		}
	})

	t.Run("pytest_directory", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "test_foo.py" {
			t.Errorf("expected test_foo.py, got %s", files[0])
		}
	})

	t.Run("pytest_no_args", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("pytest", tmpDir)

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
		files := ResolveTestFilesFromCommand("python -m pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("uv_run_pytest", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("uv run pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("npx_vitest", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("npx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app.test.ts" {
			t.Errorf("expected app.test.ts, got %s", files[0])
		}
	})

	t.Run("cargo_test", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("cargo test", filepath.Join(tmpDir, "rsrc"))
		if len(files) != 1 {
			t.Fatalf("expected 1 Rust test file, got %d: %v", len(files), files)
		}
	})

	t.Run("bunx_vitest", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("bunx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
	})

	t.Run("rspec", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("bundle_exec_rspec", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("bundle exec rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("vendor_phpunit", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("./vendor/bin/phpunit tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 PHP test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "ExampleTest.php" {
			t.Errorf("expected ExampleTest.php, got %s", files[0])
		}
	})

	t.Run("unsupported_command", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("echo hello", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for unsupported command, got %d", len(files))
		}
	})

	t.Run("empty_command", func(t *testing.T) {
		files := ResolveTestFilesFromCommand("", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for empty command, got %d", len(files))
		}
	})
}

// 🛠️#region 🌡️Technology Generate
func TestIsLicenseText(t *testing.T) {
	if !isLicenseText("This program is free software: you can redistribute it and/or modify") {
		t.Error("should detect 'free software' and 'redistribute'")
	}
	if !isLicenseText("it under the terms of the GNU Affero General Public License as") {
		t.Error("should detect 'GNU' and 'License'")
	}
	if !isLicenseText("WITHOUT ANY WARRANTY; without even the implied warranty of") {
		t.Error("should detect 'warranty'")
	}
	if !isLicenseText("Copyright 2025 Test User") {
		t.Error("should detect 'copyright'")
	}
	if isLicenseText("This function MUST return a valid result.") {
		t.Error("should not match spec text as license")
	}
	if isLicenseText("Functions for parsing SVG files.") {
		t.Error("should not match summary text as license")
	}
}

func TestIsHeaderMetaLine(t *testing.T) {
	if !isHeaderMetaLine("[🧰️repo⌨️client💻️main](repo://p/i/repo/b/b/cli/f/main.go)") {
		t.Error("should detect ID link")
	}
	if !isHeaderMetaLine("#region Header") {
		t.Error("should detect #region")
	}
	if !isHeaderMetaLine("#endregion Header") {
		t.Error("should detect #endregion")
	}
	if !isHeaderMetaLine("region Header") {
		t.Error("should detect region (Python style)")
	}
	if !isHeaderMetaLine("endregion Header") {
		t.Error("should detect endregion (Python style)")
	}
	if !isHeaderMetaLine("2025 Ueli Saluz <ueli@semio-tech.com>") {
		t.Error("should detect contributor line starting with year")
	}
	if !isHeaderMetaLine("💻️repo/asset/fixture/some/folder/🧪️file/🐍️.py") {
		t.Error("should detect file ID emoji prefix")
	}
	if isHeaderMetaLine("This function handles parsing.") {
		t.Error("should not match summary text")
	}
}

func TestExtractMarkdownSection(t *testing.T) {
	content := "# Summary\n\nThis is the summary.\n\n# 💯️Requirements\n\nSpec line one MUST work.\nSpec line two SHOULD also work.\n\n# Docs\n\nDocumentation here.\n"
	summary := ExtractMarkdownSection(content, "Summary")
	if !strings.Contains(summary, "This is the summary.") {
		t.Errorf("expected summary content, got: %q", summary)
	}
	requirements := ExtractMarkdownSection(content, "Requirements")
	if !strings.Contains(requirements, "Spec line one MUST work.") {
		t.Errorf("expected requirements content, got: %q", requirements)
	}
	docs := ExtractMarkdownSection(content, "Docs")
	if !strings.Contains(docs, "Documentation here.") {
		t.Errorf("expected docs content, got: %q", docs)
	}
	missing := ExtractMarkdownSection(content, "Nonexistent")
	if missing != "" {
		t.Errorf("expected empty for missing section, got: %q", missing)
	}
}

func TestExtractFileHeaderSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("repo/asset/fixture/some/folder/⚛️file_empty_region.tsx")
	if strings.Contains(summary, "GNU") || strings.Contains(summary, "license") || strings.Contains(summary, "redistribute") {
		t.Errorf("should not contain license text, got: %q", summary)
	}
	if strings.HasPrefix(summary, "#region") || strings.HasPrefix(summary, "region ") {
		t.Errorf("should not start with region markers, got: %q", summary)
	}
}

func TestExtractFileHeaderSummaryReturnsActualSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("repo/asset/fixture/some/folder/⚛️file_empty_region.tsx")
	if strings.Contains(summary, "free software") {
		t.Errorf("should not return license as summary, got: %q", summary)
	}
}

func TestExtractFileHeaderRequirementsNoLicense(t *testing.T) {
	requirements := ExtractFileHeaderRequirements("repo/asset/fixture/some/folder/🧪️file/🐍️.py")
	if strings.Contains(requirements, "GNU") || strings.Contains(requirements, "license") || strings.Contains(requirements, "redistribute") {
		t.Errorf("should not contain license text, got: %q", requirements)
	}
}

func TestExtractSectionLeadCommentsSkipsLicense(t *testing.T) {
	content := "# region License\n\n# This program is free software: you can redistribute it and/or modify\n# it under the terms of the GNU Affero General Public License.\n\n# endregion License\n"
	sections := languages.GetLanguage("test.py").ParseSections(content)
	for _, s := range sections {
		if s.Name == "License" {
			requirements, summary := ExtractSectionLeadComments(content, s, "#")
			if requirements != "" || summary != "" {
				t.Errorf("license section should return empty requirements and summary, got requirements=%q summary=%q", requirements, summary)
			}
		}
	}
}

func TestExtractSectionLeadCommentsSkipsRegionMarkers(t *testing.T) {
	content := "//#region 🔖️Exports\n// Re-exports of icons.\n// Data MUST be valid.\n//#endregion 🔖️Exports\n"
	sections := languages.GetLanguage("test.ts").ParseSections(content)
	for _, s := range sections {
		if s.Name == "Exports" {
			requirements, summary := ExtractSectionLeadComments(content, s, "//")
			if strings.Contains(summary, "region") {
				t.Errorf("should not contain region text in summary, got: %q", summary)
			}
			if !strings.Contains(summary, "Re-exports of icons.") {
				t.Errorf("should contain actual summary text, got: %q", summary)
			}
			if !strings.Contains(requirements, "Data MUST be valid.") {
				t.Errorf("should contain spec text, got: %q", requirements)
			}
		}
	}
}

func TestGenerateTechnologyRequirementsInvalidTechnology(t *testing.T) {
	err := GenerateTechnologyRequirements("nonexistent-technology")
	if err == nil {
		t.Error("should return error for nonexistent technology")
	}
}

func TestGenerateTechnologyDocsInvalidTechnology(t *testing.T) {
	err := GenerateTechnologyDocs("nonexistent-technology")
	if err == nil {
		t.Error("should return error for nonexistent technology")
	}
}

func TestGenerateTechnologyTodosInvalidTechnology(t *testing.T) {
	err := GenerateTechnologyTodos("nonexistent-technology")
	if err == nil {
		t.Error("should return error for nonexistent technology")
	}
}
