// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// CLI end-to-end ticket and goal lifecycles and JSON output.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

func parseTicketOpenResult(t *testing.T, output string) (int, int, int, string) {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketOpen struct {
			Year  int    `json:"year"`
			Month int    `json:"month"`
			Day   int    `json:"day"`
			Slug  string `json:"slug"`
		} `json:"ticketOpen"`
	}
	if err := json.Unmarshal(data, &resp); err != nil || resp.TicketOpen.Slug == "" {
		t.Fatalf("unable to parse ticket open response: %s", output)
	}
	return resp.TicketOpen.Year, resp.TicketOpen.Month, resp.TicketOpen.Day, resp.TicketOpen.Slug
}

func parseTicketCloseStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketClose struct {
			Status string `json:"status"`
		} `json:"ticketClose"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse ticketClose: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(resp.TicketClose.Status)
}

func parseTicketReopenStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketReopen struct {
			Status string `json:"status"`
		} `json:"ticketReopen"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse ticketReopen: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(resp.TicketReopen.Status)
}

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

	fileID := FileHeaderId(fileRel)
	fileURI := buildFileUriFromPath(fileRel)
	absFile := filepath.Join(GetRootDir(), fileRel)
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
		{"analyze", []string{"analyze", emojiText(EmojiTechnologyUser) + "compose"}},
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

func TestCliJsonErrorsToStderr(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"ticket open missing title", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"ticket close missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}},
		{"ticket reopen missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"goal open missing title", []string{"goal", "open", "--no-management"}},
		{"goal close missing id", []string{"goal", "close", "--no-management"}},
		{"goal reopen missing id", []string{"goal", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"policy check missing id", []string{"policy", "check"}},
		{"folder create missing path", []string{"folder", "create"}},
		{"file create missing path", []string{"file", "create"}},
		{"section delete missing args", []string{"section", "delete"}},
		{"section extract missing args", []string{"section", "extract"}},
		{"section integrate missing args", []string{"section", "integrate"}},
		{"todo create missing args", []string{"todo", "create"}},
		{"todo chaete missing id", []string{"todo", "delete"}},
		{"graphql missing query", []string{"graphql"}},
		{"contributor add missing github", []string{"contributor", "add"}},
		{"contributor remove missing github", []string{"contributor", "remove"}},
		{"extract missing args", []string{"extract"}},
		{"integrate missing args", []string{"integrate"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, _, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s", tt.name)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
		})
	}
}
