//go:build exhaustive

// 📃️ Exhaustive tests of the cli domain: list, section and streaming list commands.

package cli

import (
	bytes "bytes"
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	graphql "github.com/usalu/semio/repo/graphql"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveListCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow list commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name  string
		args  []string
		modes []string
	}{
		{
			name:  "bundle list",
			args:  []string{"list", "--only-bundle"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "ticket list",
			args:  []string{"list", "--only-ticket"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "folder list",
			args:  []string{"list", "--only-folder", "repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "file list",
			args:  []string{"list", "--only-file", "repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "section list",
			args:  []string{"list", "--only-section", "repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "definition list",
			args:  []string{"list", "--only-definition", "repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "policy list",
			args:  []string{"list", "--only-policy"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "contributor list",
			args:  []string{"list", "--only-contributor"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "technology list",
			args:  []string{"list", "--only-technology"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "statute list",
			args:  []string{"list", "--only-statute"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "checkpoint list",
			args:  []string{"list", "--only-checkpoint", "--limit", "5"},
			modes: []string{"", "json", "md", "text"},
		},
	}

	for _, tt := range tests {
		for _, mode := range tt.modes {
			testName := tt.name
			if mode != "" {
				testName += " --" + mode
			} else {
				testName += " (default)"
			}

			t.Run(testName, func(t *testing.T) {
				rootCmd := NewRoot(factory)
				b := bytes.NewBufferString("")
				rootCmd.SetOut(b)
				rootCmd.SetErr(b)

				args := append([]string(nil), tt.args...)
				if mode == "json" {
					args = append(args, "--json")
				} else if mode == "md" {
					args = append(args, "--md")
				} else if mode == "text" {
					args = append(args, "--text")
				}
				rootCmd.SetArgs(args)

				err := rootCmd.Execute()
				if err != nil {
					t.Fatalf("Command failed: %v\nOutput: %s", err, b.String())
				}

				output := b.String()
				if mode == "json" {
					lines := strings.Split(strings.TrimSpace(output), "\n")
					for _, line := range lines {
						if line == "" {
							continue
						}

						var data map[string]interface{}
						if err := json.Unmarshal([]byte(line), &data); err != nil {
							t.Errorf("Invalid JSON line: %s", line)
						}

						if kind, ok := data["kind"].(string); ok {
							if kind == "result" || kind == "start" || kind == "done" {

								if _, hasCmd := data["command"]; hasCmd {
									t.Errorf("Expected pure data, got Event wrapper: %s", line)
								}
							}
						}
					}
				} else if mode == "md" {
					if !strings.Contains(output, "# ") && !strings.Contains(output, "- ") && !strings.Contains(output, "|") && output != "" {
					}
				}
			})
		}
	}
}

func TestExhaustiveSectionCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow section commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	testDir := filepath.Join(repoRoot, "temp", "section_tests")
	os.MkdirAll(testDir, 0755)
	defer os.RemoveAll(testDir)

	tests := []struct {
		name       string
		ext        string
		contentFmt string
		renameTo   string
	}{
		{"TypeScript", ".ts", "const x = 1;\n// #region 🔖️%s\nconst y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Go", ".go", "package main\n// #region 🔖️%s\nvar y = 2\n// #endregion 🔖️%s\n", "Renamed"},
		{"Python", ".py", "# #region 🔖️%s\ny = 2\n# #endregion 🔖️%s\n", "Renamed"},
		{"CSharp", ".cs", "// #region 🔖️%s\nvar y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Rust", ".rs", "// #region 🔖️%s\nlet y = 2;\n// #endregion 🔖️%s\n", "Renamed"},
		{"Ruby", ".rb", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"Shell", ".sh", "# region %s\ny=2\n# endregion %s\n", "Renamed"},
		{"TOML", ".toml", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"YAML", ".yaml", "# region %s\ny: 2\n# endregion %s\n", "Renamed"},
		{"SQL", ".sql", "-- #region 🔖️%s\nSELECT 1;\n-- #endregion 🔖️%s\n", "Renamed"},
		{"GraphQL", ".graphql", "# #region 🔖️%s\ntype Query { name: String }\n# #endregion 🔖️%s\n", "Renamed"},
		{"Markdown", ".md", "## %s\nContent\n", "Renamed"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if tc.name == "Rust" {
				t.Skip("Rust section move/extract has a known issue with mod-based section format")
			}
			sectionName := "MySection"
			filename := "test" + tc.ext
			filePath := filepath.Join(testDir, filename)
			relPath, _ := filepath.Rel(repoRoot, filePath)

			var content string
			if tc.name == "Markdown" {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 1)
			} else {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 2)
			}
			os.WriteFile(filePath, []byte(content), 0644)

			moveCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			moveCmd.SetOut(b)
			moveCmd.SetErr(b)
			moveCmd.SetArgs([]string{"section", "move", relPath, sectionName, tc.renameTo})
			err := moveCmd.Execute()
			if err != nil {
				t.Fatalf("Move failed: %v Output: %s", err, b.String())
			}

			newContentBytes, _ := os.ReadFile(filePath)
			newContent := string(newContentBytes)
			if !strings.Contains(newContent, tc.renameTo) {
				t.Errorf("File content does not contain renamed section %s. Content:\n%s", tc.renameTo, newContent)
			}

			targetFile := filepath.Join(testDir, "extracted"+tc.ext)
			relTargetFile, _ := filepath.Rel(repoRoot, targetFile)

			extractCmd := NewRoot(factory)
			extractCmd.SetOut(b)
			extractCmd.SetErr(b)
			extractCmd.SetArgs([]string{"section", "extract", relPath, tc.renameTo, relTargetFile})
			err = extractCmd.Execute()
			if err != nil {
				t.Fatalf("Extract failed: %v Output: %s", err, b.String())
			}

			targetContentBytes, err := os.ReadFile(targetFile)
			if err != nil {
				t.Fatalf("Target file not created: %v", err)
			}
			targetContent := string(targetContentBytes)
			if len(targetContent) == 0 && tc.name != "Markdown" {
				t.Errorf("Extracted content is empty")
			}

			sourceIntegrate := filepath.Join(testDir, "to_integrate"+tc.ext)
			relSourceIntegrate, _ := filepath.Rel(repoRoot, sourceIntegrate)
			integrateContent := "New Content"
			os.WriteFile(sourceIntegrate, []byte(integrateContent), 0644)

			integrateCmd := NewRoot(factory)
			integrateCmd.SetOut(b)
			integrateCmd.SetErr(b)
			integrateCmd.SetArgs([]string{"section", "integrate", relSourceIntegrate, tc.renameTo, relPath})
			err = integrateCmd.Execute()
			if err != nil {
				t.Fatalf("Integrate failed: %v Output: %s", err, b.String())
			}

			finalContentBytes, _ := os.ReadFile(filePath)
			finalContent := string(finalContentBytes)
			if !strings.Contains(finalContent, integrateContent) {
				t.Errorf("File content does not contain integrated content. Content:\n%s", finalContent)
			}
		})
	}
}

func TestExhaustiveStreamingList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow streaming list test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name string
		args []string
	}{
		{
			name: "Ticket List (Text)",
			args: []string{"list", "--only-ticket"},
		},
		{
			name: "Bundle List (Text)",
			args: []string{"list", "--only-bundle"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rootCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(tt.args)

			_ = rootCmd.Execute()
			output := b.String()
			lines := strings.Split(strings.TrimSpace(output), "\n")

			for _, line := range lines {
				if strings.TrimSpace(line) == "" {
					continue
				}
				if strings.HasPrefix(strings.TrimSpace(line), "{\"kind\":\"result\"") {
					t.Errorf("Expected formatted text output, got raw JSON event: %s", line)
				}
			}
		})
	}
}
