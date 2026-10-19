// 🔬️ Tests of the cli domain: shared harness helpers.

package cli

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	regexp "regexp"
	strings "strings"
	testing "testing"
	time "time"

	graphql "github.com/usalu/semio/repo/graphql"
	workspace "github.com/usalu/semio/repo/workspace"
)

func firstJSONLine(output string) (json.RawMessage, bool) {
	for _, line := range strings.Split(strings.TrimSpace(output), "\n") {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		return json.RawMessage(trimmed), true
	}
	return nil, false
}

func writeExecutableFile(t *testing.T, path string, content string) {
	t.Helper()
	if err := os.WriteFile(path, []byte(content), 0755); err != nil {
		t.Fatalf("failed to write executable %s: %v", path, err)
	}
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

func parseTicketOpenResult(t *testing.T, output string) (int, int, int, string) {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketOpen struct {
			Slug  string `json:"slug"`
			Year  int    `json:"year"`
			Month int    `json:"month"`
			Day   int    `json:"day"`
		} `json:"ticketOpen"`
	}
	if err := json.Unmarshal(data, &resp); err != nil || resp.TicketOpen.Slug == "" || resp.TicketOpen.Year == 0 {
		t.Fatalf("unable to parse ticket open response: %s", output)
	}
	return resp.TicketOpen.Year, resp.TicketOpen.Month, resp.TicketOpen.Day, resp.TicketOpen.Slug
}

func parseGoalCreateID(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		GoalCreate struct {
			ID string `json:"id"`
		} `json:"goalCreate"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse goalCreate: %v\nOutput: %s", err, output)
	}
	if resp.GoalCreate.ID == "" {
		t.Fatalf("missing goal id in output: %s", output)
	}
	return resp.GoalCreate.ID
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

func testEngineFactory(config Config) (*Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		repoRoot = workspace.GetRootDir()
	}
	workspace.SetRootDir(repoRoot)
	executor, err := graphql.NewExecutor(repoRoot)
	if err != nil {
		return nil, err
	}
	return NewEngine(executor), nil
}

// 🔷️#region 🔊️Cli
// ⌨️#region 🎼️Helpers
func executeCommand(args ...string) (string, string, error) {
	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(stdout)
	root.SetErr(stderr)
	root.SetArgs(args)
	config.Format = "json"
	err := root.Execute()
	if err != nil {
		fmt.Fprintln(stderr, err)
	}
	return stdout.String(), stderr.String(), err
}

func executeCommandMd(args ...string) (string, string, error) {
	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(stdout)
	root.SetErr(stderr)
	root.SetArgs(args)
	config.Format = "md"
	err := root.Execute()
	if err != nil {
		fmt.Fprintln(stderr, err)
	}
	return stdout.String(), stderr.String(), err
}

type recordingGraphQLExecutor struct {
	queries []string
}

func (e *recordingGraphQLExecutor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	e.queries = append(e.queries, query)
	return map[string]interface{}{"syncManagement": true}, nil
}

func toolOutputText(result workspace.ToolResult) string {
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return strings.Join(lines, "\n")
}

var relativeTimePattern = regexp.MustCompile(`\b(opened |closed |created )?(a long while ago|\d+ (?:second|minute|hour|day|week|month|year)s? (?:ago|from now))\b`)

func normalizeRelativeTimes(s string) string {
	return relativeTimePattern.ReplaceAllString(s, "<TIME>")
}
