//go:build exhaustive

// 🔁️ Exhaustive tests of the cli domain: end-to-end lifecycles, JSON, markdown and lifecycle commands.

package cli

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"
	testing "testing"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	graphql "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}
	withExhaustiveRepo(t)

	fileRel := filepath.ToSlash(filepath.Join("go", "repo", "main.go"))

	openOut, openErr, err := executeCommand(
		"ticket", "open",
		"🎫️",
		"E2E Ticket Positional",
		"E2E prompt positional",
		"cursor-chat",
		"sonnet-4-5",
		"--goal", "TEST-GOAL",
		"--no-issue",
		"--no-management",
	)
	if err != nil {
		t.Fatalf("ticket open positional failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	y, m, d, slug := parseTicketOpenResult(t, openOut)
	defer os.RemoveAll(ticketspkg.GetTicketPath(y, m, d, slug))
	ticketPath := fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug)

	_, reopenOpenErr, reopenOpenCmdErr := executeCommand(
		"ticket", "reopen",
		ticketPath,
		"prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-management",
	)
	if reopenOpenCmdErr == nil {
		t.Fatal("expected error when reopening an already-open ticket")
	}
	if !strings.Contains(reopenOpenErr, "ticket is already open") {
		t.Errorf("expected 'ticket is already open' error, got: %s", reopenOpenErr)
	}

	fileID := codebase.FileHeaderId(fileRel)
	fileURI := model.BuildFileUriFromPath(fileRel)
	absFile := filepath.Join(workspace.GetRootDir(), fileRel)
	closeOut, closeErr, err := executeCommand(
		"ticket", "close",
		"--no-management",
		"--year", strconv.Itoa(y),
		"--month", strconv.Itoa(m),
		"--day", strconv.Itoa(d),
		"--slug", slug,
		"--summary", "E2E summary",
		"--files", fileRel,
		"--files", fileID,
		"--files", fileURI,
		"--files", absFile,
	)
	if err != nil {
		t.Fatalf("ticket close flags failed: %v\nStdout: %s\nStderr: %s", err, closeOut, closeErr)
	}
	if status := parseTicketCloseStatus(t, closeOut); status != "closed" {
		t.Fatalf("expected closed status, got %s", status)
	}

	_, closeAgainErr, closeAgainCmdErr := executeCommand(
		"ticket", "close",
		"--no-management",
		"--year", strconv.Itoa(y),
		"--month", strconv.Itoa(m),
		"--day", strconv.Itoa(d),
		"--slug", slug,
		"--summary", "E2E summary again",
		"--files", fileRel,
	)
	if closeAgainCmdErr == nil {
		t.Fatal("expected error when closing an already-closed ticket")
	}
	if !strings.Contains(closeAgainErr, "ticket is not open") {
		t.Errorf("expected 'ticket is not open' error, got: %s", closeAgainErr)
	}

	reopenOut, reopenErr, err := executeCommand(
		"ticket", "reopen",
		fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug),
		"E2E reopen prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-management",
	)
	if err != nil {
		t.Fatalf("ticket reopen mix failed: %v\nStdout: %s\nStderr: %s", err, reopenOut, reopenErr)
	}
	if status := parseTicketReopenStatus(t, reopenOut); status != "open" {
		t.Fatalf("expected open status, got %s", status)
	}

	listOut, listErr, err := executeCommand("list", "--only-ticket", "--only-year", strconv.Itoa(y))
	if err != nil {
		t.Fatalf("ticket list failed: %v\nStdout: %s\nStderr: %s", err, listOut, listErr)
	}
}

func TestExhaustiveCliE2E_GoalLifecycle_Syntaxes_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}
	withExhaustiveRepo(t)
	openOut, openErr, err := executeCommand(
		"goal", "open",
		"E2E Goal Title",
		"E2E Goal Description",
		"E2E Goal Prompt",
		"cursor-chat",
		"gpt-5-mini",
		"--due-date", "2026-02-15",
		"--no-management",
	)
	if err != nil {
		t.Fatalf("goal open failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	goalID := parseGoalCreateID(t, openOut)
	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), goalID))

	_, reopenOpenErr, reopenOpenCmdErr := executeCommand("goal", "reopen", goalID, "prompt", "cursor-chat", "gpt-5-mini", "--no-management")
	if reopenOpenCmdErr == nil {
		t.Fatal("expected error when reopening an already-open goal")
	}
	if !strings.Contains(reopenOpenErr, "goal is already open") {
		t.Errorf("expected 'goal is already open' error, got: %s", reopenOpenErr)
	}

	_, closeErr, err := executeCommand("goal", "close", goalID, "E2E Goal Summary", "--no-management")
	if err != nil {
		t.Fatalf("goal close failed: %v\nStderr: %s", err, closeErr)
	}

	_, closeAgainErr, closeAgainCmdErr := executeCommand("goal", "close", goalID, "E2E Goal Summary Again", "--no-management")
	if closeAgainCmdErr == nil {
		t.Fatal("expected error when closing an already-closed goal")
	}
	if !strings.Contains(closeAgainErr, "goal is already closed") {
		t.Errorf("expected 'goal is already closed' error, got: %s", closeAgainErr)
	}

	_, reopenErr, err := executeCommand("goal", "reopen", goalID, "E2E Goal Reopen Prompt", "cursor-chat", "gpt-5-mini", "--no-management")
	if err != nil {
		t.Fatalf("goal reopen failed: %v\nStderr: %s", err, reopenErr)
	}
}

func TestExhaustiveCliE2E_MiscCommands_NoSideEffects(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}
	withExhaustiveRepo(t)

	cmds := []struct {
		name string
		args []string
	}{
		{"bundle list", []string{"list", "--only-bundle"}},
		{"bundle tree", []string{"search", "--only-bundle"}},
		{"folder list", []string{"list", "--only-folder", "go"}},
		{"file list", []string{"list", "--only-file", "go"}},
		{"section list", []string{"list", "--only-section", "compose/js/compose.ts"}},
		{"definition list", []string{"list", "--only-definition", "compose/js/compose.ts"}},
		{"policy list", []string{"list", "--only-policy"}},
		{"analyze", []string{"analyze", "compose/go"}},
		{"goal list", []string{"list", "--only-goal"}},
		{"goal tree", []string{"search", "--only-goal"}},
		{"ticket list", []string{"list", "--only-ticket"}},
		{"ticket tree", []string{"search", "--only-ticket"}},
		{"contributor list", []string{"list", "--only-contributor"}},
		{"mcp dry-run", []string{"mcp", "--dry-run"}},
		{"update", []string{"update"}},
	}

	for _, c := range cmds {
		t.Run(c.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(c.args...)
			if err != nil {
				t.Fatalf("%s failed: %v\nStdout: %s\nStderr: %s", c.name, err, stdout, stderr)
			}
		})
	}
}

func TestExhaustiveCliJsonPureData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow CLI JSON pure data test in short mode")
	}
	withExhaustiveRepo(t)
	cmds := []struct {
		name string
		args []string
	}{
		{"bundle list", []string{"list", "--only-bundle"}},
		{"ticket list", []string{"list", "--only-ticket"}},
		{"folder list", []string{"list", "--only-folder", "repo/go"}},
		{"file list", []string{"list", "--only-file", "repo/go"}},
		{"section list", []string{"list", "--only-section", "repo/go/main.go"}},
		{"definition list", []string{"list", "--only-definition", "repo/go/main.go"}},
		{"policy list", []string{"list", "--only-policy"}},
		{"contributor list", []string{"list", "--only-contributor"}},
		{"goal list", []string{"list", "--only-goal"}},
	}

	for _, c := range cmds {
		t.Run(c.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(c.args...)
			if err != nil {
				t.Fatalf("%s failed: %v\nStderr: %s", c.name, err, stderr)
			}
			lines := strings.Split(strings.TrimSpace(stdout), "\n")
			for _, line := range lines {
				trimmed := strings.TrimSpace(line)
				if trimmed == "" {
					continue
				}
				var data map[string]interface{}
				if jsonErr := json.Unmarshal([]byte(trimmed), &data); jsonErr != nil {
					t.Errorf("invalid JSON line: %s\nError: %v", trimmed, jsonErr)
					continue
				}
				if _, hasKind := data["kind"]; hasKind {
					if _, hasCmd := data["command"]; hasCmd {
						t.Errorf("expected pure data, got event wrapper: %s", trimmed)
					}
				}
				if _, hasData := data["data"]; hasData {
					inner, ok := data["data"].(map[string]interface{})
					if ok && len(data) == 1 {
						_ = inner
						t.Errorf("expected pure data without {\"data\": ...} wrapper: %s", trimmed)
					}
				}
			}
		})
	}
}

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
	repoRoot := withExhaustiveRepo(t)

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
			defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), goalID))

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

			defer os.RemoveAll(ticketspkg.GetTicketPath(y, m, d, slug))

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

			ticketDir := ticketspkg.GetTicketPath(y, m, d, slug)
			jsonContent, err := os.ReadFile(filepath.Join(ticketDir, "🎫️ticket.json"))
			if err == nil {
				var tm model.Ticket
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
