// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Header extraction and technology docs generation.

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
	if !isHeaderMetaLine("💻️🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file/🐍️.py") {
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
	summary := ExtractFileHeaderSummary("🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🟦️.tsx")
	if strings.Contains(summary, "GNU") || strings.Contains(summary, "license") || strings.Contains(summary, "redistribute") {
		t.Errorf("should not contain license text, got: %q", summary)
	}
	if strings.HasPrefix(summary, "#region") || strings.HasPrefix(summary, "region ") {
		t.Errorf("should not start with region markers, got: %q", summary)
	}
}

func TestExtractFileHeaderSummaryReturnsActualSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🟦️.tsx")
	if strings.Contains(summary, "free software") {
		t.Errorf("should not return license as summary, got: %q", summary)
	}
}

func TestExtractFileHeaderRequirementsNoLicense(t *testing.T) {
	requirements := ExtractFileHeaderRequirements("🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file/🐍️.py")
	if strings.Contains(requirements, "GNU") || strings.Contains(requirements, "license") || strings.Contains(requirements, "redistribute") {
		t.Errorf("should not contain license text, got: %q", requirements)
	}
}

func TestExtractSectionLeadCommentsSkipsLicense(t *testing.T) {
	content := "# region License\n\n# This program is free software: you can redistribute it and/or modify\n# it under the terms of the GNU Affero General Public License.\n\n# endregion License\n"
	sections := GetLanguage("test.py").ParseSections(content)
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
	sections := GetLanguage("test.ts").ParseSections(content)
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

const technologyDocsFixtureSource = `// #region 🧲️Header
// 2025 Test User <test@test.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// A fixed TypeScript component for testing.

// #endregion 🧲️Header

// #region ⚙️Types
// Type definitions for the fixed component.
// Types MUST be exported when used externally.

/**
 * Properties of a fixed component.
 * FixedType MUST have a name and value.
 **/
interface FixedType {
  name: string;
  value: number;
}

// #endregion ⚙️Types
`

// 🧫️withTechnologyDocsFixture materialises a throwaway monorepo whose coda and compose technologies each hold one sectioned source with requirement comments.
func withTechnologyDocsFixture(t *testing.T) {
	t.Helper()
	root := withMonorepoFixture(t)
	write := func(rel, content string) {
		abs := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
			t.Fatalf("mkdir %s: %v", rel, err)
		}
		if err := os.WriteFile(abs, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", rel, err)
		}
	}
	write("compose/js/index.ts", technologyDocsFixtureSource)
	write("coda/lib/AGENTS.md", "---\nbundle:\n  emoji: 📜️\n---\n")
	write("coda/lib/index.ts", technologyDocsFixtureSource)
	InvalidateTechnologyCache()
}

func TestGenerateTechnologyRequirements(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyRequirements("coda")
	if err != nil {
		t.Fatalf("GenerateTechnologyRequirements failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read SPECS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 💯 Requirements") {
		t.Error("SPECS.md should start with '# 💯 Requirements'")
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") {
		t.Error("SPECS.md should not contain license text")
	}
	if !strings.Contains(content, "MUST") {
		t.Error("SPECS.md should contain spec keywords")
	}
}

func TestGenerateTechnologyDocs(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyDocs("coda")
	if err != nil {
		t.Fatalf("GenerateTechnologyDocs failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "DOCS.md"))
	if err != nil {
		t.Fatalf("failed to read DOCS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 📚 Docs") {
		t.Error("DOCS.md should start with '# 📚 Docs'")
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("DOCS.md should not contain license text")
	}
	if strings.Contains(content, "region ") {
		t.Error("DOCS.md should not contain region markers")
	}
}

func TestGenerateTechnologyTodos(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyTodos("coda")
	if err != nil {
		t.Fatalf("GenerateTechnologyTodos failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "TODOS.md"))
	if err != nil {
		t.Fatalf("failed to read TODOS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 🔳 TODOs") {
		t.Error("TODOS.md should start with '# 🔳 TODOs'")
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

func TestGenerateTechnologyRequirementsCompose(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyRequirements("compose")
	if err != nil {
		t.Fatalf("GenerateTechnologyRequirements compose failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "compose", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read compose/SPECS.md: %v", err)
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("compose SPECS.md should not contain license text")
	}
	if !strings.Contains(content, "MUST") {
		t.Error("compose SPECS.md should contain MUST keywords")
	}
}

func TestGenerateTechnologyDocsCompose(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyDocs("compose")
	if err != nil {
		t.Fatalf("GenerateTechnologyDocs compose failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "compose", "DOCS.md"))
	if err != nil {
		t.Fatalf("failed to read compose/DOCS.md: %v", err)
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("compose DOCS.md should not contain license text")
	}
}

func TestGenerateTechnologyRequirementsComposeRepo(t *testing.T) {
	withTechnologyDocsFixture(t)
	err := GenerateTechnologyRequirements("repo")
	if err != nil {
		t.Fatalf("GenerateTechnologyRequirements repo failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "repo", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read repo/SPECS.md: %v", err)
	}
	if strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("repo SPECS.md should not contain license text")
	}
}

// 🧲️#endregion 🌡️Technology Generate
func TestExtractSearchFromInputLineNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		toolArgs string
		expected string
	}{
		{
			name: "single line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
		{
			name: "line range",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35540
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490-L35540",
		},
		{
			name: "only start line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, ranges := extractSearchFromInput(json.RawMessage(tt.input), tt.toolArgs)
			if len(ranges) != 1 || ranges[0] != tt.expected {
				t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, tt.expected)
			}
		})
	}
}

func TestExtractSearchFromInputCompleteFileRange(t *testing.T) {
	tempFile := filepath.Join(t.TempDir(), "all.go")
	if err := os.WriteFile(tempFile, []byte("a\nb\nc"), 0o644); err != nil {
		t.Fatalf("failed to write temp file: %v", err)
	}

	input := fmt.Sprintf(`{
		"tool_name": "read_file",
		"tool_input": {
			"filePath": %q
		}
	}`, tempFile)

	pages, ranges := extractSearchFromInput(json.RawMessage(input), "")
	if len(pages) != 0 {
		t.Errorf("expected no webpages, got %v", pages)
	}
	expected := tempFile + "#L1-L3"
	if len(ranges) != 1 || ranges[0] != expected {
		t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, expected)
	}
}
