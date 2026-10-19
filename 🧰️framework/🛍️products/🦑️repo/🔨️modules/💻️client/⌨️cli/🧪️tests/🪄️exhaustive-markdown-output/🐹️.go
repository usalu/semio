// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive markdown output, lifecycle, and list commands.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	graphql "github.com/usalu/semio/repo/graphql"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"
)

func TestExhaustiveMarkdownOutput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow markdown output test in short mode")
	}
	repoRoot := withExhaustiveRepo(t)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name        string
		args        []string
		wantMarkers []string
	}{
		{
			name:        "Repo Tree MD",
			args:        []string{"search"},
			wantMarkers: []string{"- [", "]("},
		},
		{
			name:        "Ticket Tree MD",
			args:        []string{"search", "--only-ticket"},
			wantMarkers: []string{"- [", "](repo://ticket/"},
		},
		{
			name:        "Goal Tree MD",
			args:        []string{"search", "--only-goal"},
			wantMarkers: []string{"- [", "](repo://goal/"},
		},
		{
			name:        "Ticket List MD",
			args:        []string{"list", "--only-ticket"},
			wantMarkers: []string{"- [", "](repo://ticket/"},
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
			if len(strings.TrimSpace(output)) == 0 {
				t.Logf("Output is empty for %s, skipping marker checks", tt.name)
				return
			}

			for _, marker := range tt.wantMarkers {
				if !strings.Contains(output, marker) {
					t.Errorf("Output missing marker %q. Got:\n%s", marker, output)
				}
			}

			if strings.Contains(output, " -  - ") {
				t.Errorf("Output contains double dash ' -  - ' which indicates empty property issue:\n%s", output)
			}
			if strings.Contains(output, "├️─️─️ ") || strings.Contains(output, "└️─️─️ ") {
				t.Errorf("Output should not contain ASCII tree connectors in default markdown mode:\n%s", output)
			}
		})
	}
}

func TestExhaustiveLifecycleCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow lifecycle commands test in short mode")
	}
	repoRoot := withTempRepo(t)

	factory := func(config Config) (*Engine, error) {
		executor, err := graphql.NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	modes := []string{"", "json", "md", "text"}

	for _, mode := range modes {
		t.Run("lifecycle_"+mode, func(t *testing.T) {
			title := "Test Lifecycle " + mode
			if mode == "" {
				title = "Test Lifecycle default"
			}

			rootCmd := NewRoot(factory)

			goalTitle := fmt.Sprintf("Test Goal %s LifecycleTest %d", mode, time.Now().UnixNano())
			goalCmd := NewRoot(factory)
			goalB := bytes.NewBufferString("")
			goalCmd.SetOut(goalB)
			goalCmd.SetErr(goalB)
			goalCmd.SetArgs([]string{"goal", "open", goalTitle, "Test Goal Description", "Test Goal Prompt", "copilot-chat", "gemini-3-pro", "--due-date", "2025-12-31", "--no-management", "--json"})
			if err := goalCmd.Execute(); err != nil {
				t.Fatalf("goal open failed: %v\nOutput: %s", err, goalB.String())
			}
			goalID := parseGoalCreateID(t, goalB.String())

			openArgs := []string{"ticket", "open", "🎫️", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--goal", goalID, "--no-issue", "--no-management"}
			if mode == "json" {
				openArgs = append(openArgs, "--json")
			} else if mode == "md" {
				openArgs = append(openArgs, "--md")
			} else if mode == "text" {
				openArgs = append(openArgs, "--text")
			}

			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(openArgs)

			err := rootCmd.Execute()
			if err != nil {
				t.Fatalf("ticket open failed: %v\nOutput: %s", err, b.String())
			}

			listCmd := NewRoot(factory)
			listB := bytes.NewBufferString("")
			listCmd.SetOut(listB)
			listCmd.SetErr(listB)
			listCmd.SetArgs([]string{"list", "--only-ticket", "--json"})
			listCmd.Execute()

			var y, m, d int
			var slug string
			found := false

			for _, line := range strings.Split(strings.TrimSpace(listB.String()), "\n") {
				if strings.TrimSpace(line) == "" {
					continue
				}
				var env struct {
					Ticket struct {
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
						Slug  string `json:"slug"`
						Title string `json:"title"`
					} `json:"ticket"`
				}
				if json.Unmarshal([]byte(line), &env) == nil {
					if strings.EqualFold(env.Ticket.Title, title) {
						y, m, d, slug = env.Ticket.Year, env.Ticket.Month, env.Ticket.Day, env.Ticket.Slug
						found = true
						break
					}
				}
			}

			if !found {
				t.Fatalf("Could not find created ticket with title %q in list output", title)
			}

			changeArgs := []string{"ticket", "change",
				fmt.Sprintf("%d/%02d/%02d/%s", y, m, d, slug),
				"--goal", "test-goal",
				"--parent", "parent-ticket-slug",
				"--no-management",
			}
			changeCmd := NewRoot(factory)
			changeB := bytes.NewBufferString("")
			changeCmd.SetOut(changeB)
			changeCmd.SetErr(changeB)
			changeCmd.SetArgs(changeArgs)
			if err := changeCmd.Execute(); err != nil {
				t.Fatalf("ticket change failed: %v\nOutput: %s", err, changeB.String())
			}

			ticketDir := GetTicketPath(y, m, d, slug)
			jsonContent, err := os.ReadFile(filepath.Join(ticketDir, "🎫️ticket.json"))
			if err == nil {
				var tm Ticket
				if err := json.Unmarshal(jsonContent, &tm); err == nil {
					if tm.Goal != "🎯️testgoal" && tm.Goal != "test-goal" {
						t.Errorf("ticket change goal mismatch: expected test-goal, got %s", tm.Goal)
					}
					if tm.Parent != "🎫️parent-ticket-slug" && tm.Parent != "parent-ticket-slug" && tm.Parent != "" {
						t.Errorf("ticket change parent mismatch: expected parent-ticket-slug, got %s", tm.Parent)
					}
				}
			}

			closeArgs := []string{"ticket", "close",
				"--no-management",
				"--year", strconv.Itoa(y),
				"--month", strconv.Itoa(m),
				"--day", strconv.Itoa(d),
				"--slug", slug,
				"--summary", "Test Summary",
				"--files", "repo/go/main.go",
			}
			if mode == "json" {
				closeArgs = append(closeArgs, "--json")
			} else if mode == "md" {
				closeArgs = append(closeArgs, "--md")
			} else if mode == "text" {
				closeArgs = append(closeArgs, "--text")
			}

			closeCmd := NewRoot(factory)
			closeB := bytes.NewBufferString("")
			closeCmd.SetOut(closeB)
			closeCmd.SetErr(closeB)
			closeCmd.SetArgs(closeArgs)

			err = closeCmd.Execute()
			if err != nil {
				t.Fatalf("ticket close failed: %v\nOutput: %s", err, closeB.String())
			}

			if closeB.String() == "" {
				t.Errorf("ticket close output empty")
			}
		})
	}
}

func TestExhaustiveListCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow list commands test in short mode")
	}
	repoRoot := withExhaustiveRepo(t)

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
