// 🛠️ Tests of the cli domain: tool functions and their fixtures.

package cli

import (
	"bytes"
	"encoding/json"
	"github.com/usalu/semio/repo/codebase"
	"github.com/usalu/semio/repo/model"
	"github.com/usalu/semio/repo/move"
	"github.com/usalu/semio/repo/providers"
	"github.com/usalu/semio/repo/todos"
	"github.com/usalu/semio/repo/workspace"
	"github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

const fixtureSectionedTypeScript = `// #region 🔖️Header
// [💻️js/index.ts](repo://file/💻️index)

// #region 🧩️State
export const state = 1;

// #region 🏪️Store
export const store = 2;
// #endregion 🏪️Store
// #endregion 🧩️State
`

const fixtureSectionedGo = `// #region 🔖️Header
// [💻️client/main.go](repo://file/💻️main)

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
	previous := workspace.GetRootDir()
	workspace.SetRootDir(root)
	codebase.InvalidateTechnologyCache()
	t.Cleanup(func() {
		workspace.SetRootDir(previous)
		codebase.InvalidateTechnologyCache()
	})
	return root
}

const exhaustiveModelsGo = `// #region 🔖️Header
// [💻️compose/compose.go](repo://file/💻️compose)

package compose

// #region 🧩️Models
// Kit holds the parts of one assembly.
type Kit struct {
	Name string
}
// #endregion 🧩️Models
`

// 🧫️withExhaustiveRepo extends the monorepo fixture with the bundles, files, git history, goals and ticket the exhaustive
// suites query, so every exhaustive test answers from a repository it owns and never from the checkout it runs in.
func withExhaustiveRepo(t *testing.T) string {
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
	for name, emoji := range map[string]string{"compose/engine": "⚙️", "compose/antlr": "🔤️", "compose/desktop": "🖥️", "repo/server": "🌍️", "repo/go": "🐹️", "repo/cli": "⌨️", "coda/notes": "📝️"} {
		write(name+"/AGENTS.md", "---\nbundle:\n  emoji: "+emoji+"\n---\n")
		write(name+"/main.go", fixtureSectionedGo)
	}
	write("compose/go/compose.go", exhaustiveModelsGo)
	write("compose/go/search.go", fixtureSectionedGo)
	write(".🧬semio/🦑️repo/🧑️‍💻️devs/usalu/🧑️‍💻️contributor.json", "{\n  \"alias\": \"usalu\",\n  \"github\": \"usalu\",\n  \"name\": \"usalu\"\n}\n")
	write(".🧬semio/🦑️repo/🧑️‍💻️devs/ueli/🧑️‍💻️contributor.json", "{\n  \"alias\": \"ueli\",\n  \"github\": \"ueli\",\n  \"name\": \"ueli\"\n}\n")
	write("repo/client/package.json", "{\n  \"name\": \"client\"\n}\n")
	environment := append(os.Environ(), "GIT_AUTHOR_NAME=usalu", "GIT_AUTHOR_EMAIL=usalu@example.com", "GIT_COMMITTER_NAME=usalu", "GIT_COMMITTER_EMAIL=usalu@example.com", "GIT_CONFIG_GLOBAL="+os.DevNull, "GIT_CONFIG_SYSTEM="+os.DevNull)
	for _, arguments := range [][]string{
		{"init", "-q", "-b", "main"}, {"add", "-A"}, {"commit", "-q", "-m", "Initial fixture"},
		{"checkout", "-q", "-b", "side"}, {"commit", "-q", "--allow-empty", "-m", "Side change"},
		{"checkout", "-q", "main"}, {"commit", "-q", "--allow-empty", "-m", "Main change"},
		{"merge", "-q", "--no-ff", "-m", "Merge side into main", "side"},
	} {
		command := exec.Command("git", append([]string{"-c", "commit.gpgsign=false"}, arguments...)...)
		command.Dir = root
		command.Env = environment
		if output, err := command.CombinedOutput(); err != nil {
			t.Fatalf("git %v failed: %v\n%s", arguments, err, output)
		}
	}
	codebase.InvalidateTechnologyCache()
	for _, title := range []string{"Sketchpad", "AI Optimized Repo"} {
		if result := ToolGoalCreate(title, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", ""); result.Error != "" {
			t.Fatalf("failed to seed goal %q: %s", title, result.Error)
		}
	}
	if result := ToolTicketOpen("🎫️", "Seed Ticket", "Seed prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", ""); result.Error != "" {
		t.Fatalf("failed to seed ticket: %s", result.Error)
	}
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
	withExhaustiveRepo(t)
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestToolGoalList(t *testing.T) {
	withExhaustiveRepo(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Errorf("ToolGoalList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolGoalList returned nil data")
	}
}

func TestToolDraftList(t *testing.T) {
	withExhaustiveRepo(t)
	result := ToolDraftList()
	if result.Error != "" {
		t.Errorf("ToolDraftList returned error: %s", result.Error)
	}
}

func TestToolFolderList(t *testing.T) {
	withExhaustiveRepo(t)
	result := ToolFolderList(".")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestToolFileList(t *testing.T) {
	withExhaustiveRepo(t)
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
	withExhaustiveRepo(t)
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
}

func TestToolPolicyCheck(t *testing.T) {
	withExhaustiveRepo(t)
	result := ToolPolicyCheck("code", "repo/client")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestToolAnalyzeScope(t *testing.T) {
	withExhaustiveRepo(t)
	result := ToolAnalyze("repo/client", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestToolFolderCRUD(t *testing.T) {
	withExhaustiveRepo(t)
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

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

func TestToolDraftLifecycle(t *testing.T) {
	withExhaustiveRepo(t)
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

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
	withExhaustiveRepo(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok || len(goals) == 0 {
		t.Fatal("no goals to verify URI")
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
	withExhaustiveRepo(t)

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

func TestToolPolicyListIsSortedByIdentifier(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Fatalf("ToolPolicyList returned error: %s", result.Error)
	}
	policies := sortedRegisteredPolicies()
	if len(policies) < 2 {
		t.Fatalf("expected at least two registered policies, got %d", len(policies))
	}
	for index := 1; index < len(policies); index++ {
		if policies[index].ID < policies[index-1].ID {
			t.Errorf("policy %q is listed after %q", policies[index].ID, policies[index-1].ID)
		}
	}
}

func TestAutofixCommandRunsTheFixMutation(t *testing.T) {
	root := withMonorepoFixture(t)
	file := filepath.Join(root, "compose", "go", "bare.go")
	original := "package main\n\nfunc bare() {}\n"
	if err := os.WriteFile(file, []byte(original), 0o644); err != nil {
		t.Fatalf("failed to write the bare source: %v", err)
	}
	stdout, stderr, err := executeCommand("autofix", "compose/go/bare.go")
	if err != nil {
		t.Fatalf("autofix failed: %v\nStdout: %s\nStderr: %s", err, stdout, stderr)
	}
	content, err := os.ReadFile(file)
	if err != nil {
		t.Fatalf("failed to read the fixed source: %v", err)
	}
	if string(content) == original {
		t.Error("autofix must repair the header of a bare source")
	}
}

func TestDashboardAnalysisScopesAndPolicies(t *testing.T) {
	fixturePath := sandbox.OriginPath("..", "..", "..", "🎛️dashboard", "🧫️fixtures", "🏛️repo-action", "🔣️.json")
	data, err := os.ReadFile(fixturePath)
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Valid []struct {
			Action map[string]json.RawMessage `json:"action"`
			Argv   []string                   `json:"argv"`
		} `json:"valid"`
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, vector := range fixture.Valid {
		if action, present := vector.Action["Analyze"]; present {
			var selected struct {
				Scope  string  `json:"scope"`
				Policy *string `json:"policy"`
			}
			if err := json.Unmarshal(action, &selected); err != nil {
				t.Fatal(err)
			}
			extensions, found := analysisScopeExtensions(selected.Scope)
			if !found || len(extensions) == 0 {
				t.Fatalf("missing scope %s", selected.Scope)
			}
			if selected.Scope == "rust" && !reflect.DeepEqual(extensions, map[string]bool{".rs": true}) {
				t.Fatal(extensions)
			}
			if selected.Scope == "typescript" && !reflect.DeepEqual(extensions, map[string]bool{".ts": true, ".tsx": true}) {
				t.Fatal(extensions)
			}
		}
	}
	if result := ToolAnalyze("rust", []string{"NOT-A-POLICY"}); result.Error == "" {
		t.Fatal("unknown policy produced false success")
	}
	root := NewRoot(nil)
	root.SetArgs([]string{"analyze", "rust", "--policy", "NOT-A-POLICY"})
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})
	if err := root.Execute(); err == nil || !strings.Contains(err.Error(), "unknown analysis policy") {
		t.Fatalf("policy CLI dispatch: %v", err)
	}
}
