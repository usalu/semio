// 🔬️ Tests of the cli domain, split out of the pre-split godfile suite.

package cli

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	reflect "reflect"
	regexp "regexp"
	runtime "runtime"
	strconv "strconv"
	strings "strings"
	testing "testing"
	template "text/template"
	time "time"
	unicode "unicode"

	codebase "github.com/usalu/semio/repo/codebase"
	eventspkg "github.com/usalu/semio/repo/events"
	goalspkg "github.com/usalu/semio/repo/goals"
	graphql "github.com/usalu/semio/repo/graphql"
	hooks "github.com/usalu/semio/repo/hooks"
	metricspkg "github.com/usalu/semio/repo/metrics"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	search "github.com/usalu/semio/repo/search"
	statutes "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	tree "github.com/usalu/semio/repo/tree"
	workspace "github.com/usalu/semio/repo/workspace"
	yaml "github.com/usalu/semio/repo/yaml"
)

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
			Slug string `json:"slug"`
			Path string `json:"path"`
		} `json:"ticketOpen"`
	}
	if err := json.Unmarshal(data, &resp); err == nil && resp.TicketOpen.Path != "" {
		normalized := filepath.ToSlash(resp.TicketOpen.Path)
		parts := strings.Split(strings.TrimPrefix(normalized, "/"), "/")
		for i := 0; i+3 < len(parts); i++ {
			if parts[i] == "🎫️" {
				y, _ := strconv.Atoi(parts[i+1])
				m, _ := strconv.Atoi(parts[i+2])
				d, _ := strconv.Atoi(parts[i+3])
				return y, m, d, resp.TicketOpen.Slug
			}
		}
	}
	t.Fatalf("unable to parse ticket open response: %s", output)
	return 0, 0, 0, ""
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

func TestMcpBootstrapAssetsStayRepoRelative(t *testing.T) {
	repoRoot := findTestRepoRoot(".")

	cases := []struct {
		name               string
		path               string
		requiredFragments  []string
		forbiddenFragments []string
	}{
		{
			name: "codex uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".codex", "config.toml"),
			requiredFragments: []string{
				`command = "bun"`,
				`args = ["./📜️script.ts", "dev", "mcp", "stdio", "codex"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "cursor uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".cursor", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "cursor"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "generic clients use the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "copilot uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".vscode", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "copilot"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "kiro uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".kiro", "settings", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "kiro"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "windsurf uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".windsurf", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "devcontainer builds the repo client from its canonical source",
			path: filepath.Join(repoRoot, ".devcontainer", "post-create.sh"),
			requiredFragments: []string{
				"cargo build --release -p semio-framework-repo-cli",
				`.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
				"bun nx run workspace:setup",
			},
			forbiddenFragments: []string{
				"repo/client/client",
				"./repo/client/mcp/go",
				"💻️client",
			},
		},
		{
			name: "native shell bootstrap builds the repo client from its canonical source",
			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🐚️.sh"),
			requiredFragments: []string{
				"cargo build --release -p semio-framework-repo-cli",
				`.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
				"bun nx run workspace:setup",
			},
			forbiddenFragments: []string{
				"repo/client/client",
				"./repo/client/mcp/go",
				"💻️client",
			},
		},
		{
			name: "native windows bootstrap builds the repo client from its canonical source",
			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🔵️.ps1"),
			requiredFragments: []string{
				`@("build", "--release", "-p", "semio-framework-repo-cli")`,
				`semio-repo.exe`,
				`.🧬semio/🦑️repo/⚡️cache/🗃️bin`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🚀️bin`,
				`./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🐹️go/🚀️bin`,
				`@("nx", "run", "workspace:setup")`,
			},
			forbiddenFragments: []string{
				"repo/client/client.exe",
				"./repo/client/mcp/go",
				"💻️client",
			},
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			data, err := os.ReadFile(tc.path)
			if err != nil {
				t.Fatalf("failed to read %s: %v", tc.path, err)
			}
			text := string(data)
			compact := func(value string) string {
				return strings.Map(func(character rune) rune {
					if unicode.IsSpace(character) {
						return -1
					}
					return character
				}, value)
			}
			for _, fragment := range tc.requiredFragments {
				if !strings.Contains(compact(text), compact(fragment)) {
					t.Fatalf("expected %s to contain %q", tc.path, fragment)
				}
			}
			for _, fragment := range tc.forbiddenFragments {
				if strings.Contains(text, fragment) {
					t.Fatalf("expected %s to exclude %q", tc.path, fragment)
				}
			}
		})
	}
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
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = findTestRepoRoot(cwd)
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

func TestSyncCommandRunsGitHubSynchronization(t *testing.T) {
	newRoot := func(recorder *recordingGraphQLExecutor) *Command {
		factory := func(config Config) (*Engine, error) {
			return NewEngine(recorder), nil
		}
		root, config := NewRootWithConfig(factory)
		config.Format = "json"
		return root
	}

	t.Run("github target executes sync management mutation", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		stdout := new(bytes.Buffer)
		stderr := new(bytes.Buffer)
		root.SetOut(stdout)
		root.SetErr(stderr)
		root.SetArgs([]string{"sync", "github"})

		if err := root.Execute(); err != nil {
			t.Fatalf("sync github failed: %v\nstdout: %s\nstderr: %s", err, stdout.String(), stderr.String())
		}
		if len(recorder.queries) != 1 {
			t.Fatalf("expected one GraphQL query, got %d", len(recorder.queries))
		}
		if !strings.Contains(recorder.queries[0], "syncManagement") {
			t.Fatalf("expected syncManagement mutation, got: %s", recorder.queries[0])
		}
		if !strings.Contains(stdout.String(), "syncManagement") {
			t.Fatalf("expected sync result in stdout, got: %s", stdout.String())
		}
	})

	t.Run("management target executes same mutation", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		root.SetOut(new(bytes.Buffer))
		root.SetErr(new(bytes.Buffer))
		root.SetArgs([]string{"sync", "management"})

		if err := root.Execute(); err != nil {
			t.Fatalf("sync management failed: %v", err)
		}
		if len(recorder.queries) != 1 || !strings.Contains(recorder.queries[0], "syncManagement") {
			t.Fatalf("expected syncManagement mutation, got queries: %v", recorder.queries)
		}
	})

	t.Run("unknown target fails instead of printing help as success", func(t *testing.T) {
		recorder := &recordingGraphQLExecutor{}
		root := newRoot(recorder)
		root.SetOut(new(bytes.Buffer))
		root.SetErr(new(bytes.Buffer))
		root.SetArgs([]string{"sync", "githb"})

		err := root.Execute()
		if err == nil {
			t.Fatal("expected unknown sync target to fail")
		}
		if !strings.Contains(err.Error(), `unknown sync target "githb"`) {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(recorder.queries) != 0 {
			t.Fatalf("unknown sync target must not execute GraphQL, got queries: %v", recorder.queries)
		}
	})
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

// 🔬️#region 🗝️Analyze
func TestAnalyzeCommand(t *testing.T) {
	result := ToolAnalyze("compose/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := ToolAnalyze("compose/js/compose.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

func TestTestCommandHelp(t *testing.T) {
	stdout, _, err := executeCommand("test", "--help")
	if err != nil {
		t.Fatalf("test --help returned error: %v", err)
	}
	if !strings.Contains(stdout, "test") {
		t.Errorf("test --help output should mention 'test', got: %s", stdout)
	}
}

// 📤️#region 📤️Event Export
// 📤️testExportContext is a mock RepoContext for testing event export.
type testExportContext struct {
	rootDir      string
	technologies []*model.Technology
	bundles      []*model.Bundle
	folders      []*model.Folder
	files        []*model.File
	sections     []*model.Section
	definitions  []*model.Definition
}

func (c *testExportContext) GetRootDir() string { return c.rootDir }

func (c *testExportContext) GetTechnologies() []*model.Technology { return c.technologies }

func (c *testExportContext) GetBundles() []*model.Bundle { return c.bundles }

func (c *testExportContext) GetFolders() []*model.Folder { return c.folders }

func (c *testExportContext) GetFiles() []*model.File { return c.files }

func (c *testExportContext) GetSections() []*model.Section { return c.sections }

func (c *testExportContext) GetDefinitions() []*model.Definition { return c.definitions }

func (c *testExportContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) { return nil, nil }

func (c *testExportContext) GetContributors() ([]*model.Contributor, error) { return nil, nil }

func (c *testExportContext) GetGoals() ([]*model.Goal, error) { return nil, nil }

func (c *testExportContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) GetPolicies() []*model.Policy { return nil }

func (c *testExportContext) GetDrafts() ([]*model.Draft, error) { return nil, nil }

func (c *testExportContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) GetStatutes() []*model.StatuteMeta { return nil }

func (c *testExportContext) GetInteractions() ([]model.InteractionResource, error) {
	return nil, nil
}

func (c *testExportContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	return &model.AnalyzeResult{}, nil
}

func (c *testExportContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalDelete(input model.GoalDeleteInput) (bool, error) { return false, nil }

func (c *testExportContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) TodoDelete(id string) (bool, error) { return false, nil }

func (c *testExportContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	return nil, nil
}

func (c *testExportContext) DraftDelete(id string) (bool, error) { return false, nil }

func (c *testExportContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	return false, nil
}

func (c *testExportContext) FolderCreate(path string) (*model.Folder, error) { return nil, nil }

func (c *testExportContext) FolderMove(src, dst string) (*model.Folder, error) { return nil, nil }

func (c *testExportContext) FolderDelete(path string) error { return nil }

func (c *testExportContext) FileCreate(path string) (*model.File, error) { return nil, nil }

func (c *testExportContext) FileMove(src, dst string) (*model.File, error) { return nil, nil }

func (c *testExportContext) FileDelete(path string) error { return nil }

func (c *testExportContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionDelete(file, name string) error { return nil }

func (c *testExportContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	return nil, nil
}

func (c *testExportContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	return nil, nil
}

func (c *testExportContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	return nil, nil
}

func (c *testExportContext) ContributorRemove(github string) error { return nil }

func (c *testExportContext) SyncManagement() (bool, error) { return false, nil }

func TestExportToEventLogSchema(t *testing.T) {
	tmpDir := t.TempDir()
	srcDir := filepath.Join(tmpDir, "mytechnology", "mybundle", "src")
	os.MkdirAll(srcDir, 0755)

	tsContent := `// #region 🔖️Header

// 💻️src/app.ts

// 2025 Test <t@t.com>

// GNU Affero General Public License
// https://www.gnu.org/licenses/

// App module summary.

// #endregion 📤️Event Export

// #region ⚗️Functions
// Processes work items.
export function doWork(): void {}

// #endregion ⚗️Functions
`
	tsFile := filepath.Join(srcDir, "app.ts")
	os.WriteFile(tsFile, []byte(tsContent), 0644)

	technologyReadme := "# My Technology\n\n### Summary\n\nThis is the technology summary.\n\n### Specs\n\nSome specs.\n"
	os.WriteFile(filepath.Join(tmpDir, "mytechnology", "README.md"), []byte(technologyReadme), 0644)

	bundleReadme := "# My Bundle\n\n### Summary\n\nThis is the bundle summary.\n"
	os.WriteFile(filepath.Join(tmpDir, "mytechnology", "mybundle", "README.md"), []byte(bundleReadme), 0644)

	ctx := &testExportContext{
		rootDir: tmpDir,
		technologies: []*model.Technology{
			{Name: "mytechnology", Root: "mytechnology", Kind: model.TechnologyKindUser},
		},
		bundles: []*model.Bundle{
			{Name: "mytechnology/mybundle", Root: "mytechnology/mybundle", TechnologyName: "mytechnology", Kind: model.BundleKindLibrary},
		},
		folders: []*model.Folder{
			{Path: "mytechnology", Name: "mytechnology", Kind: model.FolderKindOrganization},
			{Path: "mytechnology/mybundle", Name: "mybundle", Kind: model.FolderKindOrganization},
			{Path: "mytechnology/mybundle/src", Name: "src", Kind: model.FolderKindOrganization},
		},
		files: []*model.File{
			{Path: "mytechnology/mybundle/src/app.ts", Name: "app.ts", Extension: "ts", Kind: model.FileKindCode},
		},
	}

	outputPath := filepath.Join(tmpDir, "test.events.jsonl")
	result, err := ExportToEventLog(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToEventLog failed: %v", err)
	}

	if result.Technologies != 1 {
		t.Errorf("expected 1 technology, got %d", result.Technologies)
	}
	if result.Bundles != 1 {
		t.Errorf("expected 1 bundle, got %d", result.Bundles)
	}
	if result.Folders != 3 {
		t.Errorf("expected 3 folders, got %d", result.Folders)
	}
	if result.Files != 1 {
		t.Errorf("expected 1 file, got %d", result.Files)
	}
	events, err := (eventspkg.Store{Path: outputPath}).Replay(context.Background(), nil)
	if err != nil {
		t.Fatalf("event replay failed: %v", err)
	}
	if len(events) != result.Technologies+result.Bundles+result.Folders+result.Files+result.Sections+result.Definitions {
		t.Fatalf("event count = %d, result = %+v", len(events), result)
	}
	if events[0].Kind != "bundle.recorded" {
		t.Fatalf("first deterministic event kind = %q", events[0].Kind)
	}
}

func TestExportToEventLogEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	ctx := &testExportContext{
		rootDir:      tmpDir,
		technologies: []*model.Technology{},
		bundles:      []*model.Bundle{},
		folders:      []*model.Folder{},
		files:        []*model.File{},
	}

	outputPath := filepath.Join(tmpDir, "empty.events.jsonl")
	result, err := ExportToEventLog(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToEventLog failed: %v", err)
	}
	if result.Technologies != 0 || result.Bundles != 0 || result.Folders != 0 || result.Files != 0 || result.Sections != 0 || result.Definitions != 0 {
		t.Errorf("expected all counts to be 0, got technologies=%d bundles=%d folders=%d files=%d sections=%d definitions=%d",
			result.Technologies, result.Bundles, result.Folders, result.Files, result.Sections, result.Definitions)
	}
	if _, err := os.Stat(outputPath); !os.IsNotExist(err) {
		t.Fatalf("empty export created an unnecessary log: %v", err)
	}
}

// 📜️#region 🔬️Policy
func TestPolicyListCommand(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]statutes.PolicyDef)
	if !ok {
		t.Error("ToolPolicyList data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyList returned no policies")
	}
	foundCode := false
	for _, p := range policies {
		if p.ID == "code" {
			foundCode = true
			break
		}
	}
	if !foundCode {
		t.Error("Expected to find 'code' policy")
	}
}

func TestPolicyTreeCommand(t *testing.T) {
	result := ToolPolicyTree()
	if result.Error != "" {
		t.Errorf("ToolPolicyTree returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyTree returned nil data")
	}
	policies, ok := result.Data.([]statutes.PolicyDef)
	if !ok {
		t.Error("ToolPolicyTree data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyTree returned no policies")
	}
	text := result.Output.Lines[0].Text
	if !strings.Contains(text, "code") {
		t.Error("Expected policy tree output to contain 'code' policy")
	}
	if !strings.Contains(text, "code") && !strings.Contains(text, "Missing") {
		t.Error("Expected policy tree output to contain policy info")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "compose/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

// 📦️#region 📌️Bundle
func TestBundleListCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolBundleList()
	if result.Error != "" {
		t.Errorf("ToolBundleList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolBundleList returned nil data")
	}
	bundles, ok := result.Data.([]model.Bundle)
	if !ok {
		t.Error("ToolBundleList data is not []Bundle")
		return
	}
	if len(bundles) == 0 {
		t.Error("ToolTechnologyList returned no bundles")
	}
	foundJS := false
	for _, b := range bundles {
		if b.Name == "compose/js" {
			foundJS = true
			break
		}
	}
	if !foundJS {
		t.Error("Expected to find 'compose/js' bundle")
	}
}

// 📁️#region 🌨️Folder
func TestFolderListCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFolderList("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFolderTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

// 📄️#region ✏️File
func TestFileListCommand(t *testing.T) {
	result := ToolFileList("compose/js")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestFileTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFileTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestSectionTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolSectionTree("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// 🎫️#region 🦉️Ticket
func TestTicketListCommand(t *testing.T) {
	year := 2025
	result := ToolTicketList(&year, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
}

func TestTicketOpenNoticketKeyword(t *testing.T) {
	result := ToolTicketOpen("🎫️", "Skip Ticket", "NOTICKET skip ticket creation", "gpt-5-mini", "", "codex", "", true, "", "", false, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	if result.Data != nil {
		t.Fatalf("expected no ticket data for NOTICKET keyword")
	}
}

func TestTicketOpenContinueKeyword(t *testing.T) {
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
	first := ToolTicketOpen("🌱️", "Seed Ticket", "Seed prompt", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if first.Error != "" {
		t.Fatalf("failed to seed ticket: %s", first.Error)
	}
	seed, ok := first.Data.(*model.Ticket)
	if !ok || seed == nil {
		t.Fatalf("expected seeded ticket data")
	}
	second := ToolTicketOpen("🎫️", "Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if second.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", second.Error)
	}
	continued, ok := second.Data.(*model.Ticket)
	if !ok || continued == nil {
		t.Fatalf("expected continued ticket data")
	}
	if continued.Slug != seed.Slug {
		t.Fatalf("expected continued ticket %s, got %s", seed.Slug, continued.Slug)
	}
}

// 🆕️#region 💌️Goal
func TestGoalCreateValidation(t *testing.T) {

	result := ToolGoalCreate("", "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing title")
	}

	result = ToolGoalCreate("Test Goal", "", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing description")
	}

	result = ToolGoalCreate("Test Goal", "desc", "", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing prompt")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing due date")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing client")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "invalid-llm", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "invalid-client", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid client")
	}
}

func TestGoalCreateAndCleanup(t *testing.T) {

	result := ToolGoalCreate("Test Goal Creation", "Test description", "Test prompt for goal", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error != "" {
		t.Fatalf("ToolGoalCreate returned error: %s", result.Error)
	}
	goal, ok := result.Data.(*model.Goal)
	if !ok || goal == nil {
		t.Fatal("expected goal data")
	}
	if goal.Title != "Test Goal Creation" {
		t.Errorf("expected title 'Test Goal Creation', got '%s'", goal.Title)
	}
	if goal.Dates.Due != "2026-02-15" {
		t.Errorf("expected due date '2026-02-15', got '%s'", goal.Dates.Due)
	}

	goalPath := filepath.Join(workspace.GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestGoalList(t *testing.T) {
	result := ToolGoalList()

	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok {
		t.Fatalf("ToolGoalList Data was not []*Goal, got %T", result.Data)
	}
	if len(goals) == 0 {
		t.Fatal("ToolGoalList returned no goals; expected the seeded goal document")
	}
	for _, g := range goals {
		if g.ID == "" {
			t.Errorf("goal has empty ID: %+v", g)
		}
		if g.Title == "" {
			t.Errorf("goal %s has empty title", g.ID)
		}
	}
}

func TestGoalsMcpResource(t *testing.T) {
	req := ReadResourceRequest{}
	req.Params.URI = "repo://goals"
	contents, err := handleGoalsResource(context.Background(), req)
	if err != nil {
		t.Fatalf("handleGoalsResource returned error: %v", err)
	}
	if len(contents) != 1 {
		t.Fatalf("expected 1 ResourceContents entry, got %d", len(contents))
	}
	text, ok := contents[0].(TextResourceContents)
	if !ok {
		t.Fatalf("expected TextResourceContents, got %T", contents[0])
	}
	if text.URI != "repo://goals" {
		t.Errorf("URI: expected repo://goals, got %q", text.URI)
	}
	if text.MIMEType != "text/plain" {
		t.Errorf("MIMEType: expected text/plain, got %q", text.MIMEType)
	}
	if text.Text == "" {
		t.Fatal("expected non-empty YAML text body")
	}
	goals, err := goalspkg.ListGoals()
	if err != nil {
		t.Fatalf("ListGoals failed: %v", err)
	}
	if len(goals) == 0 {
		t.Skip("no goals seeded in repo; cannot assert content")
	}
	for _, g := range goals {
		if !strings.Contains(text.Text, g.Title) {
			t.Errorf("YAML body missing goal title %q", g.Title)
		}
	}
}

// 🤝️#region 🩻️Contributor
func TestContributorListCommand(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestTechnologyListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Fatalf("ToolTechnologyList returned error: %s", result.Error)
	}
	technologies, ok := result.Data.([]model.Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	expectedIDs := map[string]string{
		"compose": model.EmojiText(model.EmojiTechnologyUser) + "compose",
		"repo":    model.EmojiText(model.EmojiTechnologyInfra) + "repo",
		"coda":    model.EmojiText(model.EmojiTechnologyResearch) + "coda",
	}
	for _, p := range technologies {
		expected, ok := expectedIDs[p.Name]
		if !ok {
			continue
		}
		got := p.GetID()
		if got != expected {
			t.Errorf("technology %q id: expected %q, got %q", p.Name, expected, got)
		}
		delete(expectedIDs, p.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected technology %q not found in list", name)
	}
}

func TestBundleListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolBundleList()
	if result.Error != "" {
		t.Fatalf("ToolBundleList returned error: %s", result.Error)
	}
	bundles, ok := result.Data.([]model.Bundle)
	if !ok {
		t.Fatal("ToolBundleList data is not []Bundle")
	}
	expectedIDs := map[string]string{
		"compose/js":  model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("📜️") + "js",
		"compose/go":  model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("🐹️") + "go",
		"repo/client": model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText("⌨️") + "client",
	}
	for _, b := range bundles {
		expected, ok := expectedIDs[b.Name]
		if !ok {
			continue
		}
		got := b.GetID()
		if got != expected {
			t.Errorf("bundle %q id: expected %q, got %q", b.Name, expected, got)
		}
		delete(expectedIDs, b.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected bundle %q not found in list", name)
	}
}

func TestContributorListIDs(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Fatalf("ToolContributorList returned error: %s", result.Error)
	}
	contributors, ok := result.Data.([]model.Contributor)
	if !ok {
		t.Fatal("ToolContributorList data is not []Contributor")
	}
	if len(contributors) == 0 {
		t.Fatal("ToolContributorList returned no contributors")
	}
	for _, c := range contributors {
		id := c.GetID()
		expectedPrefix := model.EmojiText(model.EmojiContributor)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("contributor %q id %q should start with %q", c.Alias, id, expectedPrefix)
		}
		expectedID := expectedPrefix + workspace.Flat(c.Alias)
		if id != expectedID {
			t.Errorf("contributor %q id: expected %q, got %q", c.Alias, expectedID, id)
		}
	}
	foundUeli := false
	for _, c := range contributors {
		if c.Alias == "ueli" {
			if c.GetID() != model.EmojiText(model.EmojiContributor)+"ueli" {
				t.Errorf("ueli contributor id: expected %q, got %q", model.EmojiText(model.EmojiContributor)+"ueli", c.GetID())
			}
			foundUeli = true
		}
	}
	if !foundUeli {
		t.Error("expected to find contributor 'ueli'")
	}
}

func TestGoalListIDs(t *testing.T) {
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok {
		t.Fatalf("ToolGoalList data is not []*Goal, got %T", result.Data)
	}
	for _, g := range goals {
		id := g.GetID()
		goalEmoji := model.EmojiText(model.EmojiGoal)
		if !strings.HasPrefix(id, goalEmoji) {
			t.Errorf("goal %q id %q should start with %q", g.ID, id, goalEmoji)
		}
		var expected strings.Builder
		for _, segment := range strings.Split(g.ID, "/") {
			expected.WriteString(goalEmoji)
			expected.WriteString(workspace.Flat(segment))
		}
		if id != expected.String() {
			t.Errorf("goal %q id: expected %q, got %q", g.ID, expected.String(), id)
		}
	}
}

func TestDraftListIDs(t *testing.T) {
	result := ToolDraftList()
	if result.Error != "" {
		t.Skipf("ToolDraftList returned error: %s", result.Error)
	}
	drafts, ok := result.Data.([]*model.Draft)
	if !ok {
		t.Skip("ToolDraftList data is not []*Draft")
	}
	for _, d := range drafts {
		id := d.GetID()
		expectedPrefix := model.EmojiText(model.EmojiDraft)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
		}
	}
}

// 🌳️#region 📜️Tree
func executeTreeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	root, _ := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)

	err := root.Execute()
	return buf.String(), err
}

// 🎫️#region 🎬️Wrong Argument
func TestCliWrongArgs_TicketOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing emoji", []string{"ticket", "open", "--goal", "TEST", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing title", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--title", "Valid Title", "--opus-4-5", "--no-management"}},
		{"missing goal", []string{"ticket", "open", "--emoji", "🎫️", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}},
		{"missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--files", "f"}},
		{"missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--summary", "s"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"invalid path format", []string{"ticket", "reopen", "bad-path", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing prompt", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"invalid path format", []string{"ticket", "change", "bad-path", "--no-management"}},
		{"nonexistent ticket", []string{"ticket", "change", "9999/01/01/NONEXISTENT", "--title", "New Title", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing title", []string{"goal", "open", "--no-management"}},
		{"missing description", []string{"goal", "open", "Valid Title", "--no-management", "--copilot-chat", "--opus-4-5", "--due-date", "2026-02-15"}},
		{"missing client", []string{"goal", "open", "Valid Title", "desc", "prompt", "--opus-4-5", "--due-date", "2026-02-15", "--no-management"}},
		{"missing llm", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--due-date", "2026-02-15", "--no-management"}},
		{"missing due-date", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "close", "--no-management"}},
		{"missing summary", []string{"goal", "close", "NONEXISTENT-GOAL", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "reopen", "--no-management"}},
		{"missing prompt", []string{"goal", "reopen", "NONEXISTENT-GOAL", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--opus-4-5", "--no-management"}},
		{"missing llm", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--copilot-chat", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing slug", []string{"goal", "change", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_PolicyCheck(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing policy id", []string{"policy", "check"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FolderOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"folder", "create"}},
		{"move missing args", []string{"folder", "move"}},
		{"move missing target", []string{"folder", "move", "src"}},
		{"delete missing path", []string{"folder", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FileOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"file", "create"}},
		{"move missing args", []string{"file", "move"}},
		{"move missing target", []string{"file", "move", "src"}},
		{"delete missing path", []string{"file", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_SectionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing args", []string{"section", "create"}},
		{"move missing args", []string{"section", "move"}},
		{"delete missing args", []string{"section", "delete"}},
		{"extract missing args", []string{"section", "extract"}},
		{"integrate missing args", []string{"section", "integrate"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_DefinitionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_ContributorOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"add missing github", []string{"contributor", "add"}},
		{"remove missing github", []string{"contributor", "remove"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GraphQL(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing query", []string{"graphql"}},
		{"invalid query syntax", []string{"graphql", "{ invalid @@@ }"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TodoOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing all", []string{"todo", "create"}},
		{"create missing name", []string{"todo", "create", "some-parent"}},
		{"change missing id", []string{"todo", "change"}},
		{"delete missing id", []string{"todo", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TopLevelOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"move missing all args", []string{"move"}},
		{"move missing target", []string{"move", "source"}},
		{"extract missing all", []string{"extract"}},
		{"integrate missing all", []string{"integrate"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_ErrorMessages(t *testing.T) {
	tests := []struct {
		name        string
		args        []string
		expectedErr string
	}{

		{"ticket open missing emoji", []string{"ticket", "open", "--goal", "TEST", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing emoji"},
		{"ticket open missing title", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing title"},
		{"ticket open missing goal", []string{"ticket", "open", "--emoji", "🎫️", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal"},
		{"ticket close missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}, "missing ticket path"},
		{"ticket close missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--files", "f"}, "missing summary"},
		{"ticket close missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--summary", "s"}, "missing files"},
		{"ticket reopen missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing ticket path"},

		{"goal open missing title", []string{"goal", "open", "--no-management"}, "missing title"},
		{"goal close missing id", []string{"goal", "close", "--no-management"}, "missing goal id"},
		{"goal close missing summary", []string{"goal", "close", "NONEXISTENT", "--no-management"}, "missing summary"},
		{"goal reopen missing id", []string{"goal", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal id"},
		{"goal reopen missing prompt", []string{"goal", "reopen", "NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing prompt"},
		{"goal reopen missing client", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}, "missing client"},
		{"goal reopen missing llm", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--copilot-chat", "--no-management"}, "missing llm"},

		{"todo create missing parent or name", []string{"todo", "create"}, "missing parent-id or name"},
		{"todo create missing name only", []string{"todo", "create", "parent"}, "missing parent-id or name"},
		{"todo change missing id", []string{"todo", "change"}, "missing id"},
		{"todo delete missing id", []string{"todo", "delete"}, "missing id"},

		{"folder create missing path", []string{"folder", "create"}, "missing path"},
		{"folder move missing args", []string{"folder", "move"}, "missing"},
		{"folder delete missing path", []string{"folder", "delete"}, "missing path"},

		{"file create missing path", []string{"file", "create"}, "missing path"},
		{"file move missing args", []string{"file", "move"}, "missing"},
		{"file delete missing path", []string{"file", "delete"}, "missing path"},

		{"section create missing args", []string{"section", "create"}, "missing"},
		{"section move missing args", []string{"section", "move"}, "missing"},
		{"section delete missing args", []string{"section", "delete"}, "missing file or name"},
		{"section extract missing args", []string{"section", "extract"}, "missing source file, source section, or target file"},
		{"section integrate missing args", []string{"section", "integrate"}, "missing source, target section, or target file"},
		{"contributor remove missing github", []string{"contributor", "remove"}, "missing"},

		{"graphql missing query", []string{"graphql"}, "missing query"},

		{"extract missing args", []string{"extract"}, "missing file, section, or target-file"},
		{"integrate missing args", []string{"integrate"}, "missing file, target-file, or target-section"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s", tt.name)
			}
			if !strings.Contains(strings.ToLower(stderr), strings.ToLower(tt.expectedErr)) {
				t.Errorf("expected stderr to contain %q, got: %s", tt.expectedErr, stderr)
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

// 📑️#region 🧬️Consolidated
func TestFormatResult_Section(t *testing.T) {
	payload := map[string]interface{}{
		"section": map[string]interface{}{
			"name":      "MySection",
			"filePath":  "path/to/file.ts",
			"startLine": float64(10),
			"endLine":   float64(20),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("section list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mysection",
		":10-20",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Definition(t *testing.T) {
	payload := map[string]interface{}{
		"definition": map[string]interface{}{
			"name":      "MyDefinition",
			"kind":      "function",
			"filePath":  "path/to/file.ts",
			"startLine": float64(30),
			"endLine":   float64(40),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("definition list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mydefinition",
		"MyDefinition",
		":30-40",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Bundle(t *testing.T) {
	payload := map[string]interface{}{
		"bundle": map[string]interface{}{
			"name": "MyBundle",
			"root": "/path/to/bundle",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("bundle list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"mybundle",
		"/path/to/bundle",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Folder(t *testing.T) {
	payload := map[string]interface{}{
		"folder": map[string]interface{}{
			"path": "path/to/folder",
			"kind": "custom",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("folder list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"folder",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_File(t *testing.T) {
	payload := map[string]interface{}{
		"file": map[string]interface{}{
			"id": "path/to/file.ts",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("file list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"file",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Additional(t *testing.T) {
	t.Run("Goal", func(t *testing.T) {
		payload := map[string]interface{}{
			"goal": map[string]interface{}{
				"id":          "SKETCHPAD/MVP",
				"title":       "Sketchpad MVP",
				"status":      "open",
				"description": "Get sketchpad running at MVP level",
				"dates": map[string]interface{}{
					"due": "2026-02-15",
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)
		var p map[string]interface{}
		json.Unmarshal(jsonBytes, &p)

		output := formatResult("goal list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"goal\":") {
			t.Errorf("expected formatted goal, got: %s", output)
		}
		if !strings.Contains(output, "Sketchpad MVP") {
			t.Error("output missing title")
		}
		if strings.Contains(output, "2026-02-15") {
			t.Error("output should not contain absolute due date")
		}
		if !strings.Contains(output, "in ") && !strings.Contains(output, "from now") && !strings.Contains(output, "ago") {
			t.Error("output missing relative due date")
		}
		if !strings.Contains(strings.ToLower(output), "mvp") {
			t.Error("output missing id/slug")
		}
	})

	t.Run("Contributor", func(t *testing.T) {
		payload := map[string]interface{}{
			"contributor": map[string]interface{}{
				"github": "octocat",
				"name":   "The Octocat",
				"contributions": map[string]interface{}{
					"checkpoints": 10,
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("contributor list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"contributor\":") {
			t.Errorf("expected formatted contributor, got: %s", output)
		}
		if !strings.Contains(output, "octocat") {
			t.Error("output missing github handle")
		}
		if !strings.Contains(output, "The Octocat") {
			t.Error("output missing name")
		}
	})

	t.Run("Policy", func(t *testing.T) {
		payload := map[string]interface{}{
			"policy": map[string]interface{}{
				"id":          "code",
				"description": "Validates source file headers",
				"kinds":       []interface{}{"code/header"},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("policy list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"policy\":") {
			t.Errorf("expected formatted policy, got: %s", output)
		}
		if !strings.Contains(output, "code") {
			t.Error("output missing id")
		}
		if !strings.Contains(output, "Validates") {
			t.Error("output missing description")
		}
	})

	t.Run("File", func(t *testing.T) {
		payload := map[string]interface{}{
			"file": map[string]interface{}{
				"id":        "path/to/file.md",
				"extension": ".md",
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("file list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"file\":") {
			t.Errorf("expected formatted file, got: %s", output)
		}
		if !strings.Contains(output, "file") {
			t.Error("output missing path")
		}
	})

	t.Run("Fix", func(t *testing.T) {
		payload := map[string]interface{}{
			"fix": map[string]interface{}{
				"fixed":     5,
				"remaining": 2,
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("fix", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"fix\":") {
			t.Errorf("expected formatted fix, got: %s", output)
		}
		if !strings.Contains(output, "fixed 5 breachs") {
			t.Error("output missing fixed count")
		}
	})
}

func assertValidMarkdownLink(t *testing.T, output string) {
	t.Helper()
	trimmed := strings.TrimSpace(output)
	if trimmed == "" {
		t.Error("output is empty")
		return
	}
	for _, line := range strings.Split(trimmed, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		stripped := strings.TrimLeft(line, " ")
		if strings.HasPrefix(stripped, "- ") {
			stripped = strings.TrimPrefix(stripped, "- ")
		}
		if strings.HasPrefix(stripped, "**") {
			continue
		}
		if !strings.Contains(stripped, "[") || !strings.Contains(stripped, "](") {
			t.Errorf("line missing markdown link syntax [...](...): %q", line)
		}
		if strings.Contains(line, "```json") || strings.Contains(line, "```\n") {
			t.Errorf("output contains JSON code block: %q", line)
		}
	}
}

func TestFormatMarkdownResult_MutationKeys(t *testing.T) {
	mutations := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticketOpen", "ticketOpen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketClose", "ticketClose", map[string]interface{}{
			"slug": "MY-TICKET", "status": "closed", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketReopen", "ticketReopen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketChange", "ticketChange", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goalCreate", "goalCreate", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalClose", "goalClose", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "closed",
		}},
		{"goalReopen", "goalReopen", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalChange", "goalChange", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"folderCreate", "folderCreate", map[string]interface{}{
			"path": "new/folder", "kind": "custom",
		}},
		{"folderDelete", "folderDelete", map[string]interface{}{
			"path": "old/folder", "kind": "custom",
		}},
		{"folderMove", "folderMove", map[string]interface{}{
			"path": "moved/folder", "kind": "custom",
		}},
		{"fileCreate", "fileCreate", map[string]interface{}{
			"id": "new/file.ts",
		}},
		{"fileDelete", "fileDelete", map[string]interface{}{
			"id": "old/file.ts",
		}},
		{"fileMove", "fileMove", map[string]interface{}{
			"id": "moved/file.ts",
		}},
		{"sectionCreate", "sectionCreate", map[string]interface{}{
			"name": "NewSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(10),
		}},
		{"sectionDelete", "sectionDelete", map[string]interface{}{
			"name": "OldSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"sectionMove", "sectionMove", map[string]interface{}{
			"name": "MovedSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"contributorRemove", "contributorRemove", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
		{"todoCreate", "todoCreate", map[string]interface{}{
			"name": "My Todo",
		}},
		{"todoChange", "todoChange", map[string]interface{}{
			"name": "Changed Todo",
		}},
		{"todoDelete", "todoDelete", map[string]interface{}{
			"name": "Deleted Todo",
		}},
	}

	for _, tt := range mutations {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.name, json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
			if strings.Contains(output, "```") {
				t.Errorf("markdown output contains code fence for %s:\n%s", tt.name, output)
			}
		})
	}
}

func TestFormatMarkdownResult_SingleEntities(t *testing.T) {
	entities := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticket", "ticket", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goal", "goal", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"bundle", "bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path/to/bundle",
		}},
		{"folder", "folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"file", "file", map[string]interface{}{
			"id": "some/file.ts",
		}},
		{"definition", "definition", map[string]interface{}{
			"name": "myFunc", "kind": "function", "filePath": "file.ts",
			"startLine": float64(10), "endLine": float64(20),
		}},
		{"contributor", "contributor", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
		{"policy", "policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"technology", "technology", map[string]interface{}{
			"id": "myTechnology", "description": "My technology",
		}},
		{"draft", "draft", map[string]interface{}{
			"id": "some-draft",
		}},
		{"todo", "todo", map[string]interface{}{
			"name": "My Todo",
		}},
		{"checkpoint", "checkpoint", map[string]interface{}{
			"sha": "abc123", "message": "Initial commit",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.name+" get", json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
		})
	}
}

func TestFormatMarkdownResult_Lists(t *testing.T) {
	repoLists := []struct {
		name string
		key  string
		kind string
		data map[string]interface{}
	}{
		{"tickets", "tickets", "ticket", map[string]interface{}{
			"slug": "T1", "status": "open", "title": "Ticket One",
			"year": float64(2026), "month": float64(1), "day": float64(1),
		}},
		{"bundles", "bundles", "bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path",
		}},
		{"folders", "folders", "folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"files", "files", "file", map[string]interface{}{
			"id": "some/file.ts",
		}},
		{"contributors", "contributors", "contributor", map[string]interface{}{
			"github": "octocat", "name": "Cat",
		}},
		{"policies", "policies", "policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"technologies", "technologies", "technology", map[string]interface{}{
			"id": "proj", "description": "Technology",
		}},
		{"statutes", "statutes", "statute", map[string]interface{}{
			"id": "vk1", "description": "Statute",
		}},
	}

	for _, tt := range repoLists {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{
				"repo": map[string]interface{}{
					tt.key: []interface{}{tt.data, tt.data},
				},
			}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.key+" list", json.RawMessage(jsonBytes))
			lines := strings.Split(strings.TrimSpace(output), "\n")
			if len(lines) < 2 {
				t.Errorf("expected at least 2 lines for list of 2 items, got %d: %s", len(lines), output)
			}
			for _, line := range lines {
				line = strings.TrimSpace(line)
				if line == "" {
					continue
				}
				if !strings.HasPrefix(line, "- [") {
					t.Errorf("list line missing '- [' prefix: %q", line)
				}
				if !strings.Contains(line, "](") {
					t.Errorf("list line missing markdown link: %q", line)
				}
			}
		})
	}

	topLists := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"todos", "todos", map[string]interface{}{"name": "Todo 1"}},
		{"sections", "sections", map[string]interface{}{"name": "Sec1", "filePath": "f.ts", "startLine": float64(1), "endLine": float64(5)}},
		{"definitions", "definitions", map[string]interface{}{"name": "def1", "filePath": "f.ts", "startLine": float64(1), "endLine": float64(5)}},
		{"drafts", "drafts", map[string]interface{}{"id": "draft1"}},
	}
	for _, tt := range topLists {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{
				tt.key: []interface{}{tt.data},
			}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.key+" list", json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
			if !strings.Contains(output, "- [") {
				t.Errorf("list output missing '- [' prefix: %s", output)
			}
		})
	}
}

func TestFormatMarkdownResult_Analyze(t *testing.T) {
	payload := map[string]interface{}{
		"analyze": map[string]interface{}{
			"metrics": map[string]interface{}{
				"total":       float64(3),
				"autofixable": float64(1),
			},
			"breachs": []interface{}{
				map[string]interface{}{
					"kind":    map[string]interface{}{"id": "inline-comment"},
					"scope":   "file.ts",
					"line":    float64(10),
					"summary": "Remove inline comment",
				},
			},
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("analyze", json.RawMessage(jsonBytes))
	if !strings.Contains(output, "Total Breachs") {
		t.Error("analyze output missing 'Total Breachs'")
	}
	if !strings.Contains(output, "inline-comment") {
		t.Error("analyze output missing statute")
	}
}

func TestFormatMarkdownResult_Fix(t *testing.T) {
	payload := map[string]interface{}{
		"fix": map[string]interface{}{
			"fixed":     float64(3),
			"remaining": float64(1),
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("fix", json.RawMessage(jsonBytes))
	if strings.Contains(output, "```") {
		t.Errorf("fix output contains code fence: %s", output)
	}
}

func TestFormatMarkdownResult_FileWithSections(t *testing.T) {
	payload := map[string]interface{}{
		"file": map[string]interface{}{
			"id": "src/main.ts",
			"sections": []interface{}{
				map[string]interface{}{
					"name": "Header", "path": "src/main.ts#Header", "filePath": "src/main.ts",
					"startLine": float64(1), "endLine": float64(5),
					"children": []interface{}{
						map[string]interface{}{
							"name": "SubHeader", "path": "src/main.ts#Header#SubHeader", "filePath": "src/main.ts",
							"startLine": float64(2), "endLine": float64(4),
						},
					},
				},
			},
			"definitions": []interface{}{
				map[string]interface{}{
					"name": "myFunc", "kind": "function", "filePath": "src/main.ts",
					"id":        "src/main.ts§myFunc",
					"startLine": float64(10), "endLine": float64(20),
				},
			},
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("file get", json.RawMessage(jsonBytes))
	assertValidMarkdownLink(t, output)
	if !strings.Contains(strings.ToLower(output), "header") {
		t.Errorf("output missing section name 'header', got: %s", output)
	}
	if !strings.Contains(strings.ToLower(output), "myfunc") {
		t.Errorf("output missing definition name 'myfunc', got: %s", output)
	}
}

func TestFormatMarkdownResult_NoJSONFallback(t *testing.T) {
	payload := map[string]interface{}{
		"unknownKey": map[string]interface{}{
			"id": "test",
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("unknown", json.RawMessage(jsonBytes))
	if strings.Contains(output, "```") {
		t.Errorf("output contains JSON code block: %s", output)
	}
	if strings.Contains(output, "{") && strings.Contains(output, "}") {
		trimmed := strings.TrimSpace(output)
		if trimmed[0] == '{' {
			t.Errorf("output is raw JSON: %s", output)
		}
	}
}

func TestFormatResult_MutationKeys(t *testing.T) {
	mutations := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticketOpen", "ticketOpen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketClose", "ticketClose", map[string]interface{}{
			"slug": "MY-TICKET", "status": "closed", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketReopen", "ticketReopen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goalCreate", "goalCreate", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalClose", "goalClose", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "closed",
		}},
		{"goalReopen", "goalReopen", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"folderCreate", "folderCreate", map[string]interface{}{
			"path": "new/folder", "kind": "custom",
		}},
		{"folderDelete", "folderDelete", map[string]interface{}{
			"path": "old/folder", "kind": "custom",
		}},
		{"fileCreate", "fileCreate", map[string]interface{}{
			"id": "new/file.ts",
		}},
		{"fileDelete", "fileDelete", map[string]interface{}{
			"id": "old/file.ts",
		}},
		{"sectionCreate", "sectionCreate", map[string]interface{}{
			"name": "NewSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(10),
		}},
		{"contributorRemove", "contributorRemove", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
	}

	for _, tt := range mutations {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatResult(tt.name, json.RawMessage(jsonBytes), false)
			trimmed := strings.TrimSpace(output)
			if trimmed == "" {
				t.Errorf("output is empty for %s", tt.name)
			}
			if trimmed[0] == '{' {
				t.Errorf("output is raw JSON for %s: %s", tt.name, output)
			}
			if strings.Contains(output, "\""+tt.key+"\"") {
				t.Errorf("output contains raw key %q for %s: %s", tt.key, tt.name, output)
			}
		})
	}
}

func TestTrackHookInOpenTicketUsesStableSessionIDs(t *testing.T) {
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
	goal, err := ticketspkg.OpenGoal("Hook Goal", "Hook Goal Desc", "Hook Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}
	ticket, err := ticketspkg.OpenTicket("🪝️", "Hook Ticket", "Hook Ticket Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("expected 0 agents at open (no synthetic agent), got %d", len(ticket.Agents))
	}
	stableInput := json.RawMessage(`{"session_id":"agent-session-123","request_id":"req-1","second":"2026-02-23T00:00:00Z"}`)
	hooks.RunHook(model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:00Z",
		RepoRoot: tmpDir,
		Input:    stableInput,
	})
	ticketData, err := os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json: %v", err)
	}
	var saved model.Ticket
	if err := json.Unmarshal(ticketData, &saved); err != nil {
		t.Fatalf("unmarshal ticket.json: %v", err)
	}
	if len(saved.Agents) != 0 {
		t.Fatalf("expected no persisted agents, got %d", len(saved.Agents))
	}
	if len(saved.Sessions) != len(ticket.Sessions) {
		t.Fatalf("hook should not change sessions length: before=%d after=%d", len(ticket.Sessions), len(saved.Sessions))
	}
	for i := range ticket.Sessions {
		if saved.Sessions[i] != ticket.Sessions[i] {
			t.Fatalf("hook should not change sessions: before=%+v after=%+v", ticket.Sessions, saved.Sessions)
		}
	}
	requestOnlyInput := json.RawMessage(`{"request_id":"req-2","second":"2026-02-23T00:00:01Z"}`)
	hooks.RunHook(model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:01Z",
		RepoRoot: tmpDir,
		ToolName: "Read",
		Input:    requestOnlyInput,
	})
	ticketData, err = os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json after request-only hook: %v", err)
	}
	if err := json.Unmarshal(ticketData, &saved); err != nil {
		t.Fatalf("unmarshal ticket.json after request-only hook: %v", err)
	}
	if len(saved.Agents) != 0 {
		t.Fatalf("request-only hooks should not persist agents, got %d", len(saved.Agents))
	}
	if len(saved.Sessions) != len(ticket.Sessions) {
		t.Fatalf("request-only hook should not change sessions length: before=%d after=%d", len(ticket.Sessions), len(saved.Sessions))
	}
	for i := range ticket.Sessions {
		if saved.Sessions[i] != ticket.Sessions[i] {
			t.Fatalf("request-only hook should not change sessions: before=%+v after=%+v", ticket.Sessions, saved.Sessions)
		}
	}
}

// 🧪️#endregion 🧬️Consolidated
func TestMcpToolsSchemas(t *testing.T) {
	s := CreateMcpServer(providers.McpClientGeneric, DefaultCommandTimeout)
	tools := s.ListTools()
	allowedTools := []string{
		"ticket_open",
		"ticket_close",
		"ticket_reopen",
		"goal_open",
		"goal_close",
		"goal_reopen",
	}
	if len(tools) != len(allowedTools) {
		t.Fatalf("expected %d MCP tools, got %d", len(allowedTools), len(tools))
	}
	for _, allowed := range allowedTools {
		if _, exists := tools[allowed]; !exists {
			t.Fatalf("required MCP tool %q is not registered", allowed)
		}
	}

	var validateSchema func(path string, schema map[string]any) error
	validateSchema = func(path string, schema map[string]any) error {
		typeVal, ok := schema["type"].(string)

		if ok && typeVal == "array" {
			if _, hasItems := schema["items"]; !hasItems {
				return fmt.Errorf("property '%s' is of type 'array' but missing 'items' field", path)
			}
		}

		if props, ok := schema["properties"].(map[string]any); ok {
			for k, v := range props {
				if propMap, ok := v.(map[string]any); ok {
					if err := validateSchema(path+"."+k, propMap); err != nil {
						return err
					}
				}
			}
		}

		if items, ok := schema["items"].(map[string]any); ok {
			if err := validateSchema(path+".items", items); err != nil {
				return err
			}
		}

		return nil
	}

	for name, tool := range tools {
		t.Run(name, func(t *testing.T) {

			for propName, propSchema := range tool.Tool.InputSchema.Properties {
				if propMap, ok := propSchema.(map[string]any); ok {
					if err := validateSchema(propName, propMap); err != nil {
						t.Errorf("Invalid schema for tool '%s': %v", name, err)
					}
				}
			}
		})
	}
}

func setupToolTest(t *testing.T) {
	t.Helper()
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	workspace.RootDir = findTestRepoRoot(cwd)
	codebase.InvalidateTechnologyCache()
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

// 🧫️withMonorepoFixture materialises a throwaway monorepo — three technologies, three bundles and
// two sectioned sources — and points the workspace root at it, so the codebase walk answers from a
// tree the test owns instead of from whatever this checkout happens to contain.
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

func TestToolTechnologyList(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Errorf("ToolTechnologyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTechnologyList returned nil data")
	}
	technologies, ok := result.Data.([]model.Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	if len(technologies) == 0 {
		t.Error("ToolTechnologyList returned empty technologies")
	}
}

func TestToolContributorList(t *testing.T) {
	setupToolTest(t)
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestToolGoalList(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Errorf("ToolGoalList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolGoalList returned nil data")
	}
}

func TestToolDraftList(t *testing.T) {
	setupToolTest(t)
	result := ToolDraftList()
	if result.Error != "" {
		t.Errorf("ToolDraftList returned error: %s", result.Error)
	}
}

func TestToolFolderList(t *testing.T) {
	setupToolTest(t)
	result := ToolFolderList(".")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestToolFileList(t *testing.T) {
	setupToolTest(t)
	result := ToolFileList("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestToolSectionTree(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolSectionTree("repo/client/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

func TestToolPolicyList(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
}

func TestToolPolicyCheck(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyCheck("code", "repo/client")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestToolAnalyzeScope(t *testing.T) {
	setupToolTest(t)
	result := ToolAnalyze("repo/client", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestToolFolderCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	result := move.ToolFolderCreate("test-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderCreate returned error: %s", result.Error)
	}

	result = ToolFolderList(".")
	if result.Error != "" {
		t.Fatalf("ToolFolderList returned error: %s", result.Error)
	}

	result = move.ToolFolderMove("test-folder", "renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderMove returned error: %s", result.Error)
	}

	result = move.ToolFolderDelete("renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderDelete returned error: %s", result.Error)
	}
}

func TestParseTicketPath(t *testing.T) {
	tests := []struct {
		name    string
		path    string
		year    int
		month   int
		day     int
		slug    string
		wantErr bool
	}{
		{"two-digit year", "26/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"four-digit year normalized", "2026/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"nested slug", "26/03/27/PARENT/CHILD", 26, 3, 27, "PARENT/CHILD", false},
		{"too few parts", "26/03", 0, 0, 0, "", true},
		{"empty slug", "26/03/27/", 0, 0, 0, "", true},
		{"non-numeric year", "abc/03/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric month", "26/abc/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric day", "26/03/abc/SLUG", 0, 0, 0, "", true},
		{"empty string", "", 0, 0, 0, "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			y, m, d, s, err := parseTicketPath(tt.path)
			if (err != nil) != tt.wantErr {
				t.Fatalf("parseTicketPath(%q) error = %v, wantErr %v", tt.path, err, tt.wantErr)
			}
			if !tt.wantErr {
				if y != tt.year || m != tt.month || d != tt.day || s != tt.slug {
					t.Errorf("parseTicketPath(%q) = (%d,%d,%d,%q), want (%d,%d,%d,%q)", tt.path, y, m, d, s, tt.year, tt.month, tt.day, tt.slug)
				}
			}
		})
	}
}

func TestToolDraftLifecycle(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()

	result := ToolDraftCreate("test-mcp-draft", nil)
	if result.Error != "" {
		t.Fatalf("ToolDraftCreate returned error: %s", result.Error)
	}

	listResult := ToolDraftList()
	if listResult.Error != "" {
		t.Fatalf("ToolDraftList returned error: %s", listResult.Error)
	}

	deleteResult := ToolDraftDelete("test-mcp-draft")
	if deleteResult.Error != "" {
		t.Fatalf("ToolDraftDelete returned error: %s", deleteResult.Error)
	}
}

func TestToolGoalUri(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok || len(goals) == 0 {
		t.Skip("no goals to verify URI")
	}
	for _, g := range goals {
		uri := g.GetURI()
		if uri == "" {
			t.Errorf("goal %s has empty URI", g.ID)
		}
		if !strings.HasPrefix(uri, "repo://goal/") {
			t.Errorf("goal %s URI %q should start with repo://goal/", g.ID, uri)
		}
	}
}

func TestParityDraftList(t *testing.T) {
	setupToolTest(t)

	t.Run("uses event rendering format", func(t *testing.T) {
		toolResult := ToolDraftList()
		if toolResult.Error != "" {
			t.Fatalf("ToolDraftList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)

		drafts := todos.ListDrafts(draftStore())
		if len(drafts) > 0 && mcpOut == "" {
			t.Error("ToolDraftList returned empty output despite having drafts")
		}
		if len(drafts) == 0 && mcpOut != "" {
			t.Error("ToolDraftList returned output despite having no drafts")
		}
	})

	t.Run("renders same as manual event rendering", func(t *testing.T) {
		drafts := todos.ListDrafts(draftStore())
		var events []Event
		for _, d := range drafts {
			data, _ := json.Marshal(map[string]interface{}{"draft": d})
			events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
		}
		expected := renderEventsToMarkdown(events)
		actual := toolOutputText(ToolDraftList())
		if expected != actual {
			t.Errorf("output mismatch:\nexpected:\n%s\nactual:\n%s", expected, actual)
		}
	})
}

func TestTreeCommandFlags(t *testing.T) {
	t.Run("builds filter from flags", func(t *testing.T) {
		cmd := &Command{}
		bindTreeFlags(cmd)
		cmd.Flags().Set("only-technology", "true")
		cmd.Flags().Set("no-folder", "true")
		cmd.Flags().Set("only-library", "true")
		cmd.Flags().Set("only-open", "true")
		cmd.Flags().Set("no-year", "2025")

		filter := buildTreeFilterFromFlags(cmd)

		if !filter.OnlyKinds[tree.TreeNodeTechnology] {
			t.Error("expected only-technology to be set")
		}
		if !filter.ExcludeKinds[tree.TreeNodeFolder] {
			t.Error("expected no-folder to be set")
		}
		if len(filter.OnlySubKinds[tree.TreeNodeBundle]) != 1 || filter.OnlySubKinds[tree.TreeNodeBundle][0] != string(model.BundleKindLibrary) {
			t.Error("expected only-library sub-kind")
		}
		if filter.OnlyStatus != "open" {
			t.Errorf("expected only-open status, got %q", filter.OnlyStatus)
		}
		if len(filter.ExcludeYears) != 1 || filter.ExcludeYears[0] != 2025 {
			t.Errorf("expected no-year 2025, got %v", filter.ExcludeYears)
		}
	})

	t.Run("empty flags produce empty filter", func(t *testing.T) {
		cmd := &Command{}
		bindTreeFlags(cmd)
		filter := buildTreeFilterFromFlags(cmd)
		if filter.HasOnlyKinds() {
			t.Error("empty flags should not set only-kinds")
		}
		if len(filter.ExcludeKinds) != 0 {
			t.Error("empty flags should not set exclude-kinds")
		}
		if filter.OnlyStatus != "" {
			t.Error("empty flags should not set status")
		}
	})
}

func TestSingleLineOutput(t *testing.T) {
	multiLineEntities := []struct {
		kind     string
		nodeKind tree.TreeNodeKind
		data     map[string]interface{}
	}{
		{"ticket", tree.TreeNodeTicket, map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Added folder renaming.\n\n1. MCP ticketReopen handler: reads the `title` parameter.\n2. MCP ticketClose handler: reads `title`.\n3. Goals: added `UpdateGoalTitle()` helper.\n\nAlso fixed a test bug.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"ticket", tree.TreeNodeTicket, map[string]interface{}{
			"slug": "T2", "title": "Open Ticket", "status": "open",
			"started": "2025-01-01T00:00:00Z",
			"prompt":  "Fix the `config` module.\nIt has multiple issues:\n- Issue 1\n- Issue 2",
			"year":    float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"goal", tree.TreeNodeGoal, map[string]interface{}{
			"id": "G1", "title": "Multi\nLine\nGoal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description with `code`\nand\r\nnewlines.",
		}},
		{"checkpoint", tree.TreeNodeCheckpoint, map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}},
		{"policy", tree.TreeNodePolicy, map[string]interface{}{
			"id": "p1", "name": "Policy", "description": "Rule 1\nRule 2\n`Rule 3`",
		}},
		{"technology", tree.TreeNodeTechnology, map[string]interface{}{
			"name": "proj1", "description": "Technology\nwith\nnewlines",
		}},
	}

	assertSingleLine := func(t *testing.T, label, output string) {
		t.Helper()
		lines := strings.Split(output, "\n")
		if len(lines) > 1 {
			t.Errorf("%s is multi-line (%d lines):\n%q", label, len(lines), output)
		}
		if strings.Contains(output, "\r") {
			t.Errorf("%s contains carriage return:\n%q", label, output)
		}
	}

	assertNoRawBackticks := func(t *testing.T, label, output string) {
		t.Helper()
		for _, p := range model.CollectEntityProps("ticket", multiLineEntities[0].data, false) {
			if strings.Contains(p, "`") {
				t.Errorf("%s prop contains backtick: %q", label, p)
			}
		}
		_ = output
	}

	for _, tt := range multiLineEntities {
		t.Run(tt.kind+"_renderEntityMarkdownLink_single_line", func(t *testing.T) {
			output := model.RenderEntityMarkdownLink(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdownLink("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityMarkdown_single_line", func(t *testing.T) {
			output := model.RenderEntityMarkdown(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_single_line", func(t *testing.T) {
			output := model.RenderEntityHuman(tt.kind, tt.data, false)
			assertSingleLine(t, "renderEntityHuman("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_tty_single_line", func(t *testing.T) {
			output := model.RenderEntityHuman(tt.kind, tt.data, true)
			assertSingleLine(t, "renderEntityHuman_tty("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_props_no_backticks", func(t *testing.T) {
			props := model.CollectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "`") {
					t.Errorf("prop contains backtick: %q", p)
				}
			}
			assertNoRawBackticks(t, tt.kind, "")
		})

		t.Run(tt.kind+"_monorepoTreeNodeMarkdown_single_line", func(t *testing.T) {
			treeNode := &tree.TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			tree.RenderTreeNodeMarkdown(&sb, treeNode, "", tree.DefaultEntityRenderer{})
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_monorepoTreeNodeText_single_line", func(t *testing.T) {
			treeNode := &tree.TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			tree.RenderTreeNodeText(&sb, treeNode, "", true, true, tree.DefaultEntityRenderer{})
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeText("+tt.kind+")", output)
		})
	}

	t.Run("goal_tree_with_multi_line_tickets_all_single_line", func(t *testing.T) {
		roots := []*model.GoalNode{{
			ID: "G1", Title: "Parent\nGoal", Status: "open",
			Tickets: []*model.TicketNode{
				{
					Slug: "T1", Title: "Ticket `One`", Status: "closed",
					Created:  "2025-01-01T00:00:00Z",
					Finished: "2025-01-02T00:00:00Z",
					Summary:  "Fixed things.\n\n1. First fix.\n2. Second fix with `code`.",
				},
				{
					Slug: "T2", Title: "Ticket Two", Status: "open",
					Created:     "2025-01-01T00:00:00Z",
					Description: "Please fix:\n- Item 1\n- Item 2",
				},
			},
			Children: []*model.GoalNode{{
				ID: "G2", Title: "Child Goal", Status: "open",
				Description: "Description\nwith\nnewlines.",
			}},
		}}
		for _, format := range []string{"md", "text"} {
			output := tree.RenderModelGoalTreeNodes(roots, format)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("goal tree (%s) line %d is empty (blank line in output)", format, i)
				}
			}
		}
	})

	t.Run("ticket_list_with_multi_line_summary_single_line", func(t *testing.T) {
		tickets := []interface{}{
			map[string]interface{}{
				"slug": "T1", "title": "Ticket", "status": "closed",
				"finished": "2025-01-02T00:00:00Z",
				"summary":  "Summary with\nnewlines and `backticks`.",
				"year":     float64(2025), "month": float64(1), "day": float64(1),
			},
		}
		for _, useMD := range []bool{true, false} {
			output := tree.RenderTicketList(tickets, false, useMD)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("ticket list (md=%v) line %d is empty", useMD, i)
				}
			}
		}
	})

	t.Run("formatMarkdownResult_ticket_list_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"tickets": []interface{}{
					map[string]interface{}{
						"slug": "T1", "title": "Ticket", "status": "closed",
						"finished": "2025-01-02T00:00:00Z",
						"summary":  "Line 1\nLine 2\n`code`",
						"year":     float64(2025), "month": float64(1), "day": float64(1),
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("ticket list", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult ticket list line %d is empty", i)
			}
		}
	})

	t.Run("formatMarkdownResult_goal_tree_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"goals": []interface{}{
					map[string]interface{}{
						"id": "G1", "title": "Goal", "status": "open",
						"dueDate": "2030-01-01", "createdAt": "2025-01-01T00:00:00Z",
						"description": "Desc\nwith\nnewlines",
					},
				},
				"tickets": []interface{}{
					map[string]interface{}{
						"id": "T1", "slug": "T1", "title": "Ticket", "status": "closed",
						"goal":    "G1",
						"date":    map[string]interface{}{"created": "2025-01-01T00:00:00Z", "finished": "2025-01-02T00:00:00Z"},
						"summary": "Summary\nwith `code`\nrefs.",
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("goal tree", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult goal tree line %d is empty", i)
			}
		}
	})
}

func TestRenderPromptTemplateUsesRepoMetaRoot(t *testing.T) {
	repoRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	templateDir := filepath.Join(repoRoot, ".🧬semio", "🦑️repo", "💬️prompts")
	if err := os.MkdirAll(templateDir, 0755); err != nil {
		t.Fatalf("mkdir template dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(templateDir, "enhance.tpl"), []byte("Prompt={{ .prompt }}"), 0644); err != nil {
		t.Fatalf("write template: %v", err)
	}
	otherDir := filepath.Join(repoRoot, "repo", "client")
	if err := os.MkdirAll(otherDir, 0755); err != nil {
		t.Fatalf("mkdir other dir: %v", err)
	}
	oldRoot := workspace.RootDir
	defer func() { workspace.SetRootDir(oldRoot) }()
	workspace.SetRootDir(repoRoot)
	oldWD, err := os.Getwd()
	if err != nil {
		t.Fatalf("getwd: %v", err)
	}
	defer func() { _ = os.Chdir(oldWD) }()
	if err := os.Chdir(otherDir); err != nil {
		t.Fatalf("chdir: %v", err)
	}
	out, err := renderPromptTemplate("enhance", map[string]string{"prompt": "hello"})
	if err != nil {
		t.Fatalf("renderPromptTemplate failed: %v", err)
	}
	if strings.TrimSpace(out) != "Prompt=hello" {
		t.Fatalf("unexpected rendered prompt: %q", out)
	}
}

func TestRunHookForRejectsGenericKind(t *testing.T) {
	err := RunHookFor(providers.McpClientGeneric, "stop", nil)
	if err == nil || !strings.Contains(strings.ToLower(err.Error()), "hooks are not available") {
		t.Fatalf("expected hooks-unavailable error for generic kind, got: %v", err)
	}
}

func TestHookCommandCLI(t *testing.T) {
	factory := func(cfg Config) (*Engine, error) {
		return nil, nil
	}
	config := &Config{Format: "json", Repo: t.TempDir()}
	cmd := hookCommand(factory, config)
	cases := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{"neutral agent starting", []string{"agent.started", "copilot-chat"}, false},
		{"neutral agent prompt submitting", []string{"agent.prompt.submitting", "cursor-chat"}, false},
		{"neutral agent tool terminal starting", []string{"agent.tool.terminal.starting", "windsurf-chat"}, false},
		{"native copilot SessionStart", []string{"SessionStart", "copilot-chat"}, false},
		{"native copilot PreToolUse", []string{"PreToolUse", "copilot-chat"}, false},
		{"native copilot PreCompact", []string{"PreCompact", "copilot-chat"}, false},
		{"native cursor sessionStart", []string{"sessionStart", "cursor-chat"}, false},
		{"native cursor beforeReadFile", []string{"beforeReadFile", "cursor-chat"}, false},
		{"native windsurf pre_user_prompt", []string{"pre_user_prompt", "windsurf-chat"}, false},
		{"native windsurf pre_read_code", []string{"pre_read_code", "windsurf-chat"}, false},
		{"native claude SessionStart", []string{"SessionStart", "claude-code"}, false},
		{"native claude PreToolUse", []string{"PreToolUse", "claude-code"}, false},
		{"native droid PreToolUse", []string{"PreToolUse", "droid"}, false},
		{"invalid event no client", []string{"invalid.event"}, true},
		{"invalid native no client", []string{"UnknownEvent", "copilot-chat"}, true},
		{"no args", []string{}, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			var buf bytes.Buffer
			cmd.SetOut(&buf)
			cmd.SetErr(&buf)
			cmd.SetIn(strings.NewReader(""))
			cmd.SetArgs(tc.args)
			err := cmd.Execute()
			if tc.wantErr && err == nil {
				t.Error("expected error")
			}
			if !tc.wantErr && err != nil {
				t.Errorf("unexpected error: %v", err)
			}
		})
	}
}

func TestConfigureCommandDoesNotGenerateConfigFiles(t *testing.T) {
	repoRoot := t.TempDir()
	hooksDir := filepath.Join(repoRoot, ".git", "hooks")
	hookSourcesDir := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks")
	if err := os.MkdirAll(hooksDir, 0o755); err != nil {
		t.Fatalf("mkdir hooks: %v", err)
	}
	if err := os.MkdirAll(hookSourcesDir, 0o755); err != nil {
		t.Fatalf("mkdir hook sources: %v", err)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		if err := os.WriteFile(filepath.Join(hookSourcesDir, hookName), []byte("#!/bin/sh\nexit 0\n"), 0o755); err != nil {
			t.Fatalf("write hook source %s: %v", hookName, err)
		}
	}
	for _, hookName := range []string{"pre-commit", "post-commit"} {
		hookPath := filepath.Join(hooksDir, hookName)
		if err := os.WriteFile(hookPath, []byte("#!/bin/sh\nexit 1\n"), 0o755); err != nil {
			t.Fatalf("write hook %s: %v", hookName, err)
		}
	}
	var out bytes.Buffer
	cmd := configureCommand(nil, &Config{Repo: repoRoot})
	cmd.SetOut(&out)
	cmd.SetArgs(nil)
	if err := cmd.Execute(); err != nil {
		t.Fatalf("configure should succeed: %v", err)
	}
	output := out.String()
	if !strings.Contains(output, "config generation is disabled") {
		t.Fatalf("expected no-operation message, got %q", output)
	}
	if !strings.Contains(output, "git hooks removed") {
		t.Fatalf("expected git hook removal message, got %q", output)
	}
	if !strings.Contains(output, "micro-commit hooks installed") {
		t.Fatalf("expected micro-commit hooks install message, got %q", output)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		hookPath := filepath.Join(hooksDir, hookName)
		if st, err := os.Stat(hookPath); err != nil || st.IsDir() {
			t.Fatalf("expected micro-commit hook at %s: %v", hookPath, err)
		}
	}
	if _, err := os.Stat(filepath.Join(hooksDir, "pre-commit")); err == nil {
		t.Fatal("expected legacy pre-commit hook removed")
	}
	for _, path := range []string{
		filepath.Join(repoRoot, ".github", "hooks", "repo.json"),
		filepath.Join(repoRoot, ".cursor", "hooks.json"),
		filepath.Join(repoRoot, ".claude", "settings.json"),
		filepath.Join(repoRoot, ".kiro", "agents", "repo.json"),
		filepath.Join(hooksDir, "pre-commit"),
	} {
		if _, err := os.Stat(path); err == nil || !os.IsNotExist(err) {
			t.Fatalf("configure unexpectedly left or created %s", path)
		}
	}
}

func testLoggingConfigFull() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func writeRepoLoggingConfig(t *testing.T, root string, lg workspace.LoggingConfig) {
	t.Helper()
	repoDir := filepath.Join(root, ".🧬semio", "🦑️repo")
	if err := os.MkdirAll(repoDir, 0o755); err != nil {
		t.Fatalf("mkdir .🦑️repo: %v", err)
	}
	content := fmt.Sprintf("[logging]\nsession = %t\noperations = %t\nplan = %t\ndetail = \"%s\"\n",
		lg.Session, lg.Operations, lg.Plan, lg.Detail)
	if err := os.WriteFile(filepath.Join(repoDir, workspace.ConfigFileName), []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", workspace.ConfigFileName, err)
	}
}

func TestHookCommandStdinPiped(t *testing.T) {
	tmpDir := t.TempDir()
	payload := `{"session_id":"sess1","tool_name":"Read","tool_input":{"file_path":"/tmp/x"}}`
	cmd := exec.Command("./cli", "hook", "agent.tool.ended", "claude-code", "--tool-name", "Read")
	cmd.Stdin = strings.NewReader(payload)
	cmd.Env = append(os.Environ(), fmt.Sprintf("COMPOSE_REPO=%s", tmpDir))
	cmd.Dir = filepath.Dir(os.Args[0])
	out, err := cmd.CombinedOutput()
	_ = out
	_ = err
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache", "🤖️generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"sess1")
	entries, readErr := os.ReadDir(logDir)
	if readErr != nil {
		t.Skip("cli binary not available for subprocess test")
	}
	if len(entries) == 0 {
		t.Fatal("expected at least one log file after piped hook invocation")
	}
	data, _ := os.ReadFile(filepath.Join(logDir, entries[0].Name()))
	if !strings.Contains(string(data), "sess1") {
		t.Errorf("expected stdin payload in log, got: %s", string(data))
	}
}

func TestNativeHookEventMappingWithRealData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow native hook event mapping test in short mode")
	}
	cases := []struct {
		name        string
		nativeEvent string
		client      string
		toolName    string
		input       string
		expectEvent model.HookEvent
		expectPar   string
	}{
		{"copilot/SessionStart", "SessionStart", "copilot-chat", "", `{"hookEventName":"SessionStart","sessionId":"d765d480","second":"2026-02-19T10:44:08.112Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentStarted, ""},
		{"copilot/Stop", "Stop", "copilot-chat", "", `{"hookEventName":"Stop","sessionId":"2f1e87c2","second":"2026-02-18T18:51:54.315Z","stop_hook_active":false}`, model.HookAgentEnded, ""},
		{"copilot/SubagentStart", "SubagentStart", "copilot-chat", "", `{"hookEventName":"SubagentStart","sessionId":"ab58fc89","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentStarted, "subagent"},
		{"copilot/SubagentStop", "SubagentStop", "copilot-chat", "", `{"hookEventName":"SubagentStop","sessionId":"ab58fc89","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentEnded, "subagent"},
		{"copilot/UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", `{"hookEventName":"UserPromptSubmit","sessionId":"d765d480","second":"2026-02-19T10:44:16.328Z","transcript_path":"/tmp/t.jsonl","cwd":"/workspaces/semio"}`, model.HookAgentPromptSubmitting, ""},
		{"copilot/PreCompact", "PreCompact", "copilot-chat", "", `{"second":"2026-02-18T18:41:24.718Z","hookEventName":"PreCompact","sessionId":"2f1e87c2","transcript_path":"/tmp/t.jsonl","trigger":"auto","cwd":"/workspaces/semio"}`, model.HookAgentCompacting, ""},
		{"copilot/PreToolUse/read_file", "PreToolUse", "copilot-chat", "read_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"read_file","second":"2026-02-19T12:30:31.702Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/grep_search", "PreToolUse", "copilot-chat", "grep_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"grep_search","second":"2026-02-19T10:44:35.056Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/file_search", "PreToolUse", "copilot-chat", "file_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"file_search","second":"2026-02-19T10:50:09.443Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_dir", "PreToolUse", "copilot-chat", "list_dir", `{"second":"2026-02-19T10:44:16.328Z","hookEventName":"PreToolUse","sessionId":"d765d480","transcript_path":"/tmp/t.jsonl","tool_name":"list_dir","tool_input":{"path":"/workspaces/semio"}}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_code_usages", "PreToolUse", "copilot-chat", "list_code_usages", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"list_code_usages","second":"2026-02-19T11:31:35.416Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/replace_string_in_file", "PreToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"replace_string_in_file","second":"2026-02-19T10:50:46.160Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/multi_replace_string_in_file", "PreToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:17.358Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/create_file", "PreToolUse", "copilot-chat", "create_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"create_file","second":"2026-02-19T12:25:00.000Z"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"run_in_terminal","second":"2026-02-18T18:42:59.593Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolTerminalStarting, ""},
		{"copilot/PreToolUse/manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:13.780Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingStarting, ""},
		{"copilot/PreToolUse/runSubagent", "PreToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"runSubagent","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolStarting, ""},
		{"copilot/PostToolUse/read_file", "PostToolUse", "copilot-chat", "read_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"read_file","second":"2026-02-18T18:38:51.951Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"copilot/PostToolUse/replace_string_in_file", "PostToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"replace_string_in_file","second":"2026-02-18T18:37:05.471Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/multi_replace_string_in_file", "PostToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:26.261Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"d765d480","hookEventName":"PostToolUse","tool_name":"run_in_terminal","second":"2026-02-19T10:43:56.761Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolTerminalEnded, ""},
		{"copilot/PostToolUse/manage_todo_list", "PostToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:20.586Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"copilot/PostToolUse/runSubagent", "PostToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"runSubagent","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolEnded, ""},
		{"copilot/PostToolUse/grep_search", "PostToolUse", "copilot-chat", "grep_search", `{"sessionId":"8a40542e","hookEventName":"PostToolUse","tool_name":"grep_search","second":"2026-02-18T18:58:48.393Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"cursor/sessionStart", "sessionStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:00Z"}`, model.HookAgentStarted, ""},
		{"cursor/sessionEnd", "sessionEnd", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/subagentStart", "subagentStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:01:00Z"}`, model.HookAgentStarted, "subagent"},
		{"cursor/subagentStop", "subagentStop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:29:00Z"}`, model.HookAgentEnded, "subagent"},
		{"cursor/stop", "stop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/beforeSubmitPrompt", "beforeSubmitPrompt", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:01Z","prompt":"Fix bug"}`, model.HookAgentPromptSubmitting, ""},
		{"cursor/preCompact", "preCompact", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, model.HookAgentCompacting, ""},
		{"cursor/preToolUse/read_file", "preToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:02:00Z","tool_name":"read_file"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/preToolUse/edit", "preToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:03:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditStarting, ""},
		{"cursor/preToolUse/terminal", "preToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:04:00Z","tool_name":"terminal"}`, model.HookAgentToolTerminalStarting, ""},
		{"cursor/preToolUse/task", "preToolUse", "cursor-chat", "task", `{"sessionId":"cur-001","second":"2026-02-19T10:05:00Z","tool_name":"task"}`, model.HookAgentToolPlanUpdatingStarting, ""},
		{"cursor/postToolUse/read_file", "postToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:06:00Z","tool_name":"read_file"}`, model.HookAgentToolSearchEnded, ""},
		{"cursor/postToolUse/editfile", "postToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:07:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/postToolUse/terminal", "postToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:08:00Z","tool_name":"terminal"}`, model.HookAgentToolTerminalEnded, ""},
		{"cursor/postToolUseFailure/edit", "postToolUseFailure", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:09:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:10:00Z"}`, model.HookAgentToolStarting, ""},
		{"cursor/afterMCPExecution", "afterMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:11:00Z"}`, model.HookAgentToolEnded, ""},
		{"cursor/beforeReadFile", "beforeReadFile", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:12:00Z"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/afterFileEdit", "afterFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:13:00Z"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeShellExecution", "beforeShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:14:00Z"}`, model.HookAgentToolTerminalStarting, ""},
		{"cursor/afterShellExecution", "afterShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, model.HookAgentToolTerminalEnded, ""},
		{"cursor/afterAgentResponse", "afterAgentResponse", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:16:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/afterAgentThought", "afterAgentThought", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:17:00Z"}`, model.HookAgentThinkingEnded, ""},
		{"cursor/beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:18:00Z"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:19:00Z"}`, model.HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", `{"second":"2026-02-18T18:46:41.123Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentPromptSubmitting, ""},
		{"windsurf/post_cascade_response", "post_cascade_response", "windsurf-chat", "", `{"second":"2026-02-18T19:00:12.032Z","agent_action_name":"post_cascade_response","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentEnded, ""},
		{"windsurf/post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", `{"second":"2026-02-18T18:45:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentStarted, ""},
		{"windsurf/pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", `{"agent_action_name":"pre_mcp_tool_use","trajectory_id":"23e6dcf5","second":"2026-02-18T18:54:57.304Z","execution_id":"d9b64466","tool_info":{"mcp_server_name":"repo","mcp_tool_name":"tree"}}`, model.HookAgentToolStarting, ""},
		{"windsurf/post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", `{"second":"2026-02-18T18:55:28.469Z","agent_action_name":"post_mcp_tool_use","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolEnded, ""},
		{"windsurf/pre_read_code", "pre_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:48.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolSearchStarting, ""},
		{"windsurf/post_read_code", "post_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:50.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolSearchEnded, ""},
		{"windsurf/pre_write_code", "pre_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:30.780Z","agent_action_name":"pre_write_code","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolCodeEditStarting, ""},
		{"windsurf/post_write_code", "post_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:35.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_run_command", "pre_run_command", "windsurf-chat", "", `{"second":"2026-02-18T18:54:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolTerminalStarting, ""},
		{"windsurf/post_run_command", "post_run_command", "windsurf-chat", "", `{"agent_action_name":"post_run_command","trajectory_id":"23e6dcf5","second":"2026-02-18T18:57:49.375Z","execution_id":"d9b64466","tool_info":{"command_line":"npm install","cwd":"/workspaces/semio"}}`, model.HookAgentToolTerminalEnded, ""},
		{"claude/SessionStart", "SessionStart", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionStart","source":"startup"}`, model.HookAgentStarted, ""},
		{"claude/SessionEnd", "SessionEnd", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionEnd"}`, model.HookAgentEnded, ""},
		{"claude/SubagentStart", "SubagentStart", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStart"}`, model.HookAgentStarted, "subagent"},
		{"claude/SubagentStop", "SubagentStop", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStop"}`, model.HookAgentEnded, "subagent"},
		{"claude/Stop", "Stop", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"Stop","stop_hook_active":false}`, model.HookAgentEnded, ""},
		{"claude/UserPromptSubmit", "UserPromptSubmit", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"UserPromptSubmit","prompt":"Fix the bug"}`, model.HookAgentPromptSubmitting, ""},
		{"claude/PreCompact", "PreCompact", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreCompact"}`, model.HookAgentCompacting, ""},
		{"claude/TaskCompleted", "TaskCompleted", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TaskCompleted"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"claude/Notification", "Notification", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"Notification"}`, model.HookAgentToolStarting, ""},
		{"claude/TeammateIdle", "TeammateIdle", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TeammateIdle"}`, model.HookAgentToolStarting, ""},
		{"claude/PermissionRequest", "PermissionRequest", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PermissionRequest"}`, model.HookAgentToolStarting, ""},
		{"claude/PreToolUse/Bash", "PreToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"go test -v"},"tool_use_id":"toolu_01WKqqc5y27LZu1KTB5GsGDu"}`, model.HookAgentToolTestStarting, ""},
		{"claude/PreToolUse/Read", "PreToolUse", "claude-code", "Read", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Read","tool_input":{"file_path":"/workspaces/semio/main.go"},"tool_use_id":"toolu_01Md976UFvJmmL5KaH4xzsx8"}`, model.HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/Edit", "PreToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl","tool_name":"Edit"}`, model.HookAgentToolCodeEditStarting, ""},
		{"claude/PreToolUse/Glob", "PreToolUse", "claude-code", "Glob", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"Glob","tool_input":{"pattern":"**/*.json"}}`, model.HookAgentToolStarting, ""},
		{"claude/PreToolUse/Grep", "PreToolUse", "claude-code", "Grep", `{"session_id":"e51a2976","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreToolUse","tool_name":"Grep","tool_input":{"pattern":"BlockedToolPatterns"}}`, model.HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/mcp_tree", "PreToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"mcp__repo__tree","tool_input":{"query":"hooks"}}`, model.HookAgentToolStarting, ""},
		{"claude/PostToolUse/Bash", "PostToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","tool_name":"Bash","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl"}`, model.HookAgentToolTerminalEnded, ""},
		{"claude/PostToolUse/Edit", "PostToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","tool_name":"Edit","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"claude/PostToolUse/Read", "PostToolUse", "claude-code", "Read", `{"session_id":"167906cd","tool_name":"Read","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"claude/PostToolUse/TodoWrite", "PostToolUse", "claude-code", "TodoWrite", `{"session_id":"167906cd","tool_name":"TodoWrite","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"claude/PostToolUse/mcp_tree", "PostToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd","tool_name":"mcp__repo__tree","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolEnded, ""},
		{"claude/PostToolUseFailure/Bash", "PostToolUseFailure", "claude-code", "Bash", `{"session_id":"167906cd","tool_name":"Bash","transcript_path":"/tmp/t.jsonl","error":"command failed"}`, model.HookAgentToolTerminalEnded, ""},
		{"droid/SessionStart", "SessionStart", "droid", "", `{"session_id":"droid-001","second":"2026-02-18T18:47:06.000Z"}`, model.HookAgentStarted, ""},
		{"droid/PreToolUse/Bash", "PreToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:06.000Z"}`, model.HookAgentToolTerminalStarting, ""},
		{"droid/PostToolUse/Bash", "PostToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:10.000Z"}`, model.HookAgentToolTerminalEnded, ""},
		{"codex/SessionStart", "SessionStart", "codex", "", `{"session_id":"codex-001","second":"2026-02-18T18:50:00.000Z"}`, model.HookAgentStarted, ""},
		{"codex/PreToolUse/Read", "PreToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:05.000Z"}`, model.HookAgentToolSearchStarting, ""},
		{"codex/PostToolUse/Read", "PostToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:10.000Z"}`, model.HookAgentToolSearchEnded, ""},
		{"antigravity/SessionStart", "SessionStart", "antigravity-chat", "", `{"session_id":"ag-001","second":"2026-02-18T18:55:00.000Z"}`, model.HookAgentStarted, ""},
		{"antigravity/PreToolUse/Task", "PreToolUse", "antigravity-chat", "Task", `{"session_id":"ag-001","tool_name":"Task","second":"2026-02-18T18:55:05.000Z"}`, model.HookAgentToolPlanUpdatingStarting, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			input := json.RawMessage(tc.input)
			event, parent, err := hooks.ResolveHookEvent(tc.nativeEvent, tc.client, tc.toolName, input)
			if err != nil {
				t.Fatalf("ResolveHookEvent error: %v", err)
			}
			if event != tc.expectEvent {
				t.Errorf("event: want %s, got %s", tc.expectEvent, event)
			}
			if parent != tc.expectPar {
				t.Errorf("parent: want %q, got %q", tc.expectPar, parent)
			}
			tmpDir := t.TempDir()
			writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
			inputSecond := extractSecondFromInput(string(input))
			var secondStr string
			if inputSecond == 0 {
				secondStr = time.Now().UTC().Format(time.RFC3339)
			} else {
				secondStr = fmt.Sprintf("%d", inputSecond)
			}
			hctx := model.HookContext{
				Event:      event,
				Client:     tc.client,
				Second:     secondStr,
				RepoRoot:   tmpDir,
				ToolName:   tc.toolName,
				Input:      input,
				ParentInfo: parent,
			}
			hooks.RunHook(hctx)
			sessionID := hooks.ExtractSessionIDFromInput(input)
			if sessionID == "" {
				sessionID = "unknown"
			}
			logNow := time.Now().UTC()
			logDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache", "🤖️generated",
				model.FormatYearDir(logNow.Year()%100),
				model.FormatMonthDir(int(logNow.Month())),
				model.FormatDayDir(logNow.Day()),
				sessionID)
			entries, err := os.ReadDir(logDir)
			if err != nil {
				t.Fatalf("log dir: %v", err)
			}
			if len(entries) != 1 || entries[0].Name() != "session.json" {
				names := make([]string, len(entries))
				for i, e := range entries {
					names[i] = e.Name()
				}
				t.Fatalf("want only session.json in session dir, got %v", names)
			}
			data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
			if err != nil {
				t.Fatalf("read session.json: %v", err)
			}
			var meta model.SessionMeta
			if err := json.Unmarshal(data, &meta); err != nil {
				t.Fatalf("invalid session JSON: %v", err)
			}
			if len(meta.Events) == 0 {
				t.Fatalf("expected at least one event in session.json")
			}
			var entry *model.HookLogEntry
			for i := range meta.Events {
				var probe map[string]interface{}
				if err := json.Unmarshal(meta.Events[i].Event, &probe); err == nil && probe["kind"] == string(event) {
					entry = &meta.Events[i]
					break
				}
			}
			if entry == nil {
				t.Fatalf("could not find event kind %s in session.json events (%d total)", event, len(meta.Events))
			}
			var evt map[string]interface{}
			if err := json.Unmarshal(entry.Event, &evt); err != nil {
				t.Fatalf("cannot unmarshal event: %v", err)
			}
			if evt["kind"] != string(event) {
				t.Errorf("log kind: want %s, got %v", event, evt["kind"])
			}
			if evt["client"] != tc.client {
				t.Errorf("log client: want %s, got %v", tc.client, evt["client"])
			}
			if entry.Native.Event == nil {
				t.Error("log native.event: want non-nil")
			}
			wantSession := hooks.ResolveEventSessionID(hooks.ExtractSessionIDFromInput(input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("log session: want %s, got %v", wantSession, evt["session"])
			}
			wantSecond := hooks.ResolveEventSecondID(secondStr)
			if wantSecond != "" && evt["second"] != wantSecond {
				t.Errorf("log second: want %s, got %v", wantSecond, evt["second"])
			}
			wantTranscript := hooks.ExtractTranscriptFromInput(input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("log transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if entry.Response.Blocked != nil {
				t.Error("log response.blocked: want nil for non-blocked event")
			}
		})
	}
}

// 🔢️TestLocWiring asserts the CLI wiring of the `loc` verb; every counting, parsing and rendering
// rule it used to duplicate now lives in 📊️metrics and is covered by that module's own tests and by
// the four language-agnostic cases under 📊️metrics/🧪️tests.
func TestLocWiring(t *testing.T) {
	t.Run("text no ansi for pipe", func(t *testing.T) {
		report := &metricspkg.LocReport{Snapshot: metricspkg.ComposeLocReportSnapshot(metricspkg.Cumulative{}, map[string]int64{"Go": 1}, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var out strings.Builder
		renderLocText(&out, report, false, false, false)
		if strings.ContainsRune(out.String(), 0x1b) {
			t.Fatal("unexpected ansi")
		}
	})
	t.Run("root has loc", func(t *testing.T) {
		root, _ := NewRootWithConfig(testEngineFactory)
		var found *Command
		for _, c := range root.Commands() {
			if c.Name() == "loc" {
				found = c
				break
			}
		}
		if found == nil {
			t.Fatal("no loc")
		}
		if f := found.Flags().Lookup("by-contributor"); f == nil {
			t.Fatal("missing by-contributor flag")
		}
	})
}

func TestMicroCommitCommandExists(t *testing.T) {
	root, _ := NewRootWithConfig(testEngineFactory)
	var found *Command
	for _, c := range root.Commands() {
		if c.Name() == "micro-commit" {
			found = c
			break
		}
	}
	if found == nil {
		t.Fatal("root command tree missing micro-commit")
	}
	if found.Use != "micro-commit [subcommand] [args...]" {
		t.Fatalf("unexpected micro-commit use: %q", found.Use)
	}
}

// 🚩️TestExtractEffortFromArgs tests extracting reasoning effort from command line arguments.
func TestExtractEffortFromArgs(t *testing.T) {
	cases := []struct {
		args     []string
		expected string
	}{
		{[]string{"--effort", "high"}, "high"},
		{[]string{"--effort=low"}, "low"},
		{[]string{"--llm", "opus-4-7", "--effort", "max"}, "max"},
		{[]string{"--other", "val"}, ""},
	}
	for _, tc := range cases {
		cmd := &Command{}
		addEffortFlags(cmd)
		got, _ := extractEffortFromArgs(cmd, tc.args)
		if got != tc.expected {
			t.Errorf("extractEffortFromArgs(%v) = %q, want %q", tc.args, got, tc.expected)
		}
	}
}

// 🧪️TestTicketEffortLifecycle tests full lifecycle preservation of reasoning effort on tickets.
func TestTicketEffortLifecycle(t *testing.T) {
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

	result := ToolTicketOpen("🎫️", "Test Effort Ticket", "Prompt with high effort", "opus-4-7", "high", "copilot-chat", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("expected valid ticket")
	}
	if ticket.GetEffort() != "high" {
		t.Errorf("ticket.GetEffort() = %q, want 'high'", ticket.GetEffort())
	}
	if ticket.GetLLM() != "opus-4-7" {
		t.Errorf("ticket.GetLLM() = %q, want 'opus-4-7'", ticket.GetLLM())
	}

	testFile := "work.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("done"), 0644); err != nil {
		t.Fatal(err)
	}
	run("git", "add", testFile)
	run("git", "commit", "-m", "work done")

	closeResult := ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Close effort ticket", []string{testFile}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose error: %s", closeResult.Error)
	}

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen with max effort", "gemini-3-7-pro", "max", "copilot-chat", "", "", "", "", true, providers.McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen error: %s", reopenResult.Error)
	}
	reopenedTicket, ok := reopenResult.Data.(*model.Ticket)
	if !ok || reopenedTicket == nil {
		t.Fatal("expected valid reopened ticket")
	}
	if reopenedTicket.GetEffort() != "max" {
		t.Errorf("reopenedTicket.GetEffort() = %q, want 'max'", reopenedTicket.GetEffort())
	}
	if reopenedTicket.GetLLM() != "gemini-3-7-pro" {
		t.Errorf("reopenedTicket.GetLLM() = %q, want 'gemini-3-7-pro'", reopenedTicket.GetLLM())
	}
}

type g1Fixture struct {
	Schema  string `json:"schema"`
	Command struct {
		InvalidArgs   []string `json:"invalidArgs"`
		ErrorContains string   `json:"errorContains"`
		HelpArgs      []string `json:"helpArgs"`
		HelpContains  string   `json:"helpContains"`
		DispatchArgs  []string `json:"dispatchArgs"`
	} `json:"command"`
	Glob struct {
		Pattern string   `json:"pattern"`
		Paths   []string `json:"paths"`
		Matches []string `json:"matches"`
	} `json:"glob"`
	Template struct {
		Invalid string `json:"invalid"`
		Error   bool   `json:"error"`
	} `json:"template"`
	Search struct {
		Query                  string            `json:"query"`
		Documents              map[string]string `json:"documents"`
		Matches                []string          `json:"matches"`
		ReindexInterruptPhases []string          `json:"reindexInterruptPhases"`
		MaxDocumentBytes       int               `json:"maxDocumentBytes"`
	} `json:"search"`
	YAML struct {
		Source  string   `json:"source"`
		Name    string   `json:"name"`
		Paths   []string `json:"paths"`
		Enabled bool     `json:"enabled"`
	} `json:"yaml"`
}

func loadG1Fixture(t *testing.T) g1Fixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "..", "📚️library", "🧪️tests", "1️⃣g1-contract", "🧫️fixtures", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture g1Fixture
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.cli.g1/1" {
		t.Fatalf("unexpected fixture schema %q", fixture.Schema)
	}
	return fixture
}

func TestG1InvalidFlagFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	root := &Command{Use: "repo"}
	root.AddCommand(&Command{Use: "inspect", Run: func(*Command, []string) {}})
	root.SetArgs(fixture.Command.InvalidArgs)
	err := root.Execute()
	if err == nil || !strings.Contains(err.Error(), fixture.Command.ErrorContains) {
		t.Fatalf("invalid flag error = %v", err)
	}
}

func TestG1CommandHelpAndDispatchFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	var output bytes.Buffer
	var dispatched []string
	root := &Command{Use: "repo"}
	root.SetOut(&output)
	root.AddCommand(&Command{
		Use:   "inspect target",
		Short: fixture.Command.HelpContains,
		Args:  ExactArgs(1),
		Run: func(_ *Command, args []string) {
			dispatched = append([]string(nil), args...)
		},
	})
	root.SetArgs(fixture.Command.HelpArgs)
	if err := root.Execute(); err != nil {
		t.Fatal(err)
	}
	if strings.Count(output.String(), fixture.Command.HelpContains) != 1 || dispatched != nil {
		t.Fatalf("help output = %q, dispatched = %v", output.String(), dispatched)
	}
	root.SetArgs(fixture.Command.DispatchArgs)
	if err := root.Execute(); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(dispatched, fixture.Command.DispatchArgs[1:]) {
		t.Fatalf("dispatch args = %v", dispatched)
	}
}

func TestG1RecursiveGlobFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	root := t.TempDir()
	for _, relative := range fixture.Glob.Paths {
		path := filepath.Join(root, filepath.FromSlash(relative))
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte(relative), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	matches, err := workspace.FilepathGlob(filepath.Join(root, filepath.FromSlash(fixture.Glob.Pattern)))
	if err != nil {
		t.Fatal(err)
	}
	for index := range matches {
		relative, err := filepath.Rel(root, matches[index])
		if err != nil {
			t.Fatal(err)
		}
		matches[index] = filepath.ToSlash(relative)
	}
	if !reflect.DeepEqual(matches, fixture.Glob.Matches) {
		t.Fatalf("glob matches = %v, want %v", matches, fixture.Glob.Matches)
	}
}

func TestG1BadTemplateFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	_, err := template.New("invalid").Funcs(model.TxtFuncMap()).Parse(fixture.Template.Invalid)
	if (err != nil) != fixture.Template.Error {
		t.Fatalf("template error = %v", err)
	}
}

func TestG1SearchFixtureAndCancellation(t *testing.T) {
	fixture := loadG1Fixture(t)
	indexValue, err := search.NewMemOnly(search.NewIndexMapping())
	if err != nil {
		t.Fatal(err)
	}
	for id, text := range fixture.Search.Documents {
		if err := indexValue.Index(id, map[string]interface{}{"text": text}); err != nil {
			t.Fatal(err)
		}
	}
	queries := make([]search.Query, 0)
	for _, term := range strings.Fields(fixture.Search.Query) {
		query := search.NewMatchQuery(term)
		query.SetFuzziness(0)
		queries = append(queries, query)
	}
	request := search.NewSearchRequest(search.NewConjunctionQuery(queries...))
	request.Size = 10
	result, err := indexValue.Search(request)
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	for _, hit := range result.Hits {
		ids = append(ids, hit.ID)
	}
	if !reflect.DeepEqual(ids, fixture.Search.Matches) {
		t.Fatalf("search ids = %v, want %v", ids, fixture.Search.Matches)
	}
	cancellable := indexValue.(interface {
		SearchContext(context.Context, *search.SearchRequest, func(int, int)) (*search.SearchResult, error)
	})
	cancelled, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := cancellable.SearchContext(cancelled, request, nil); !errors.Is(err, context.Canceled) {
		t.Fatalf("search cancellation = %v", err)
	}
}

func TestG1CorruptIndex(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "events.jsonl"), []byte("{broken\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := search.Open(root); err == nil || !strings.Contains(err.Error(), "corrupt index") {
		t.Fatalf("corrupt index error = %v", err)
	}
}

func TestG1ExportRetainsHistoryAndRejectsDuplicateSnapshot(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	firstContext := &testExportContext{technologies: []*model.Technology{{Name: "first", Root: "first", Kind: model.TechnologyKindUser}}}
	first, err := ExportToEventLog(path, firstContext)
	if err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := ExportToEventLog(path, firstContext); !errors.Is(err, eventspkg.ErrDuplicate) {
		t.Fatalf("duplicate snapshot = %v", err)
	}
	afterDuplicate, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, afterDuplicate) {
		t.Fatal("duplicate snapshot changed history")
	}
	secondContext := &testExportContext{technologies: []*model.Technology{{Name: "second", Root: "second", Kind: model.TechnologyKindUser}}}
	second, err := ExportToEventLog(path, secondContext)
	if err != nil {
		t.Fatal(err)
	}
	if first.Snapshot == second.Snapshot {
		t.Fatal("changed snapshots have the same identity")
	}
	events, err := (eventspkg.Store{Path: path}).Replay(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 2 || events[0].Sequence != 1 || events[1].Sequence != 2 {
		t.Fatalf("retained export history = %+v", events)
	}
}

func TestG1FailedAndInterruptedExportPreserveExistingLog(t *testing.T) {
	path := filepath.Join(t.TempDir(), "export.events.jsonl")
	base := &testExportContext{technologies: []*model.Technology{{Name: "base", Root: "base", Kind: model.TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, base); err != nil {
		t.Fatal(err)
	}
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	oversized := &testExportContext{technologies: []*model.Technology{{Name: "large", Root: strings.Repeat("x", eventspkg.MaxEventSize+1), Kind: model.TechnologyKindUser}}}
	if _, err := ExportToEventLog(path, oversized); !errors.Is(err, eventspkg.ErrTooLarge) {
		t.Fatalf("failed export = %v", err)
	}
	assertFileBytes(t, path, before)
	ctx, cancel := context.WithCancel(context.Background())
	interrupted := &testExportContext{technologies: []*model.Technology{{Name: "interrupted", Root: "interrupted", Kind: model.TechnologyKindUser}}}
	_, err = ExportToEventLogContext(ctx, path, interrupted, func(value eventspkg.Progress) {
		if value.Step == "appended" {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("interrupted export = %v", err)
	}
	assertFileBytes(t, path, before)
}

func assertFileBytes(t *testing.T, path string, expected []byte) {
	t.Helper()
	actual, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(actual, expected) {
		t.Fatalf("%s changed", path)
	}
}

func TestG1YAMLFixture(t *testing.T) {
	fixture := loadG1Fixture(t)
	var config struct {
		Name    string   `yaml:"name"`
		Paths   []string `yaml:"paths"`
		Enabled bool     `yaml:"enabled"`
	}
	if err := yaml.Unmarshal([]byte(fixture.YAML.Source), &config); err != nil {
		t.Fatal(err)
	}
	if config.Name != fixture.YAML.Name || !reflect.DeepEqual(config.Paths, fixture.YAML.Paths) || config.Enabled != fixture.YAML.Enabled {
		t.Fatalf("yaml config = %+v", config)
	}
}

// 🌱️newDispatchRoot returns a root carrying one recording child, plus the recorder.
func newDispatchRoot() (*Command, *[]string) {
	var seen []string
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(_ *Command, args []string) error {
			seen = append(seen, "child")
			seen = append(seen, args...)
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})
	return root, &seen
}

func TestExecuteFallsBackToProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "alpha"}

	root, seen := newDispatchRoot()
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 2 || (*seen)[0] != "child" || (*seen)[1] != "alpha" {
		t.Fatalf("process arguments were not dispatched, got %v", *seen)
	}
}

func TestSetArgsOverridesProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "from-process"}

	root, seen := newDispatchRoot()
	root.SetArgs([]string{"child", "from-caller"})
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 2 || (*seen)[1] != "from-caller" {
		t.Fatalf("installed arguments lost to the process vector, got %v", *seen)
	}
}

func TestSetArgsEmptyDoesNotFallBackToProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child"}

	root, seen := newDispatchRoot()
	root.SetArgs(nil)
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 0 {
		t.Fatalf("an explicitly empty argument vector fell back to the process vector, got %v", *seen)
	}
}

func TestSubcommandFlagsResolveInheritedPersistentFlags(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "--json"}

	var observed bool
	var lookupErr error
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	root.PersistentFlags().BoolP("json", "", false, "json output")
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(cmd *Command, _ []string) error {
			observed, lookupErr = cmd.Flags().GetBool("json")
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})

	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if lookupErr != nil {
		t.Fatalf("inherited persistent flag not visible to the subcommand: %v", lookupErr)
	}
	if !observed {
		t.Fatal("inherited persistent flag resolved but did not carry its parsed value")
	}
}

func TestSubcommandFlagsReportInheritedChanged(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "--format", "json"}

	var changed bool
	var value string
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	root.PersistentFlags().String("format", "md", "output format")
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(cmd *Command, _ []string) error {
			changed = cmd.Flags().Changed("format")
			value, _ = cmd.Flags().GetString("format")
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})

	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if !changed {
		t.Fatal("Changed did not report an inherited persistent flag that was set")
	}
	if value != "json" {
		t.Fatalf("inherited persistent flag value = %q, want %q", value, "json")
	}
}
