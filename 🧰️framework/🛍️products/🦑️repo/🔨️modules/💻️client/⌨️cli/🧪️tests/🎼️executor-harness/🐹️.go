// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	graphql "github.com/usalu/semio/repo/graphql"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 🛡️heldBindings names the client roots that must not point into the checkout the tests run in.
func heldBindings() map[string]string {
	executorMu.Lock()
	boundExecutorRoot := executorRoot
	executorMu.Unlock()
	return map[string]string{"rootDir": rootDir, "executor": boundExecutorRoot, "testRepoRootOverride": testRepoRootOverride}
}

// 🧷️bindRootForTest binds dir as the client root until the test ends and then restores the previous binding.
func bindRootForTest(t *testing.T, dir string) {
	t.Helper()
	previous := rootDir
	SetRootDir(dir)
	t.Cleanup(func() {
		if previous != "" {
			SetRootDir(previous)
		}
	})
}

func moduleFixturePath(parts ...string) string {
	return sandbox.OriginPath(parts...)
}

func TestMain(m *testing.M) {
	os.Exit(sandbox.Run("client", heldBindings, func() int {
		SetRootDir(sandbox.Root())
		return m.Run()
	}))
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

var testRepoRootOverride string

// 🧫️withTempRepo creates a throwaway git repository with an empty ticket store, points the repo root and the engine factory at it, and restores every root when the test ends.
func withTempRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	run := func(args ...string) {
		execCommandWithTimeout(t, 30*time.Second, root, nil, "git", args...)
	}
	run("init")
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("commit", "--allow-empty", "-m", "initial")
	if err := os.MkdirAll(filepath.Join(root, ".🧬semio", "🦑️repo", "🎫️tickets"), 0o755); err != nil {
		t.Fatal(err)
	}
	previousRoot := rootDir
	previousOverride := testRepoRootOverride
	previousWorkspace := workspace.GetRootDir()
	rootDir = root
	testRepoRootOverride = root
	InvalidateTechnologyCache()
	t.Cleanup(func() {
		rootDir = previousRoot
		testRepoRootOverride = previousOverride
		workspace.SetRootDir(previousWorkspace)
		InvalidateTechnologyCache()
	})
	return root
}

func testEngineFactory(config Config) (*Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		repoRoot = testRepoRootOverride
	}
	if repoRoot == "" {
		repoRoot = GetRootDir()
	}
	SetRootDir(repoRoot)
	executor, err := graphql.NewExecutor(repoRoot)
	if err != nil {
		return nil, err
	}
	return NewEngine(executor), nil
}

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

const sectionFixtureRelPath = "🗃️pkg/💻️index.ts"

const sectionFixtureContent = `// #region 🔖️Header
// [💻️pkg/index.ts](repo://file/💻️pkg)
// #endregion 🔖️Header

// #region 🧩️State
export const state = 1;

// #region 🏪️Store
export const store = 2;
// #endregion 🏪️Store
// #endregion 🧩️State
`

// 🧫️withMonorepoFixture materialises a throwaway monorepo (three technologies, three bundles and
// two sectioned sources) and points the repo root at it, so the codebase walk answers from a tree
// the test owns instead of from whatever this checkout happens to contain.
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
	previous := rootDir
	rootDir = root
	InvalidateTechnologyCache()
	t.Cleanup(func() {
		rootDir = previous
		InvalidateTechnologyCache()
	})
	return root
}

func withoutSelectors(value string) string {
	return strings.NewReplacer("\uFE0E", "", "\uFE0F", "").Replace(value)
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
	previousWorkspace := workspace.GetRootDir()
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
		execCommandWithTimeout(t, 30*time.Second, root, environment, "git", append([]string{"-c", "commit.gpgsign=false"}, arguments...)...)
	}
	SetRootDir(root)
	t.Cleanup(func() {
		workspace.SetRootDir(previousWorkspace)
		invalidateGitignore()
	})
	for _, title := range []string{"Sketchpad", "AI Optimized Repo"} {
		if result := ToolGoalCreate(title, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", ""); result.Error != "" {
			t.Fatalf("failed to seed goal %q: %s", title, result.Error)
		}
	}
	if result := ToolTicketOpen("🎫️", "Seed Ticket", "Seed prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", ""); result.Error != "" {
		t.Fatalf("failed to seed ticket: %s", result.Error)
	}
	return root
}

const statuteFixtureRoot = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures"

func copyTree(t *testing.T, source string, target string) {
	t.Helper()
	err := filepath.WalkDir(source, func(path string, entry os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		relative, err := filepath.Rel(source, path)
		if err != nil {
			return err
		}
		destination := filepath.Join(target, relative)
		if entry.IsDir() {
			return os.MkdirAll(destination, 0o755)
		}
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		return os.WriteFile(destination, data, 0o644)
	})
	if err != nil {
		t.Fatalf("failed to copy %s: %v", source, err)
	}
}

// 📜️withStatuteFixtures copies the language-agnostic statute fixtures owned by the statutes module into a throwaway monorepo at their checked-in relative location, so the policy engine answers from a repository the test owns.
func withStatuteFixtures(t *testing.T) string {
	t.Helper()
	root := withExhaustiveRepo(t)
	copyTree(t, moduleFixturePath("..", "..", "📜️statutes", "🧪️tests", "🧫️fixtures", "📁️some"), filepath.Join(root, filepath.FromSlash(statuteFixtureRoot), "📁️some"))
	return root
}

const scriptProcessInvocationEnv = "SEMIO_SCRIPT_PROCESS_INVOCATION"

type scriptProcessPolicy struct {
	Version                    int    `json:"version"`
	Owner                      string `json:"owner"`
	MaximumElapsedMilliseconds int64  `json:"maximumElapsedMilliseconds"`
}

type scriptProcessEnvelope struct {
	Version                   int                 `json:"version"`
	Policy                    scriptProcessPolicy `json:"policy"`
	DeadlineEpochMilliseconds int64               `json:"deadlineEpochMilliseconds"`
	Capabilities              map[string]any      `json:"capabilities"`
}

// 📨️provideScriptEnvelope gives the Script processes this test spawns the original process envelope a parent `bun repo` run would have handed down: the same wire the TypeScript suites build with createScriptProcessEnvelope, finite and starting now. An envelope inherited from a real parent is kept.
func provideScriptEnvelope(t *testing.T, owner string, budget time.Duration) {
	t.Helper()
	if os.Getenv(scriptProcessInvocationEnv) != "" {
		return
	}
	maximum := budget.Milliseconds()
	wire, err := json.Marshal(scriptProcessEnvelope{Version: 1, Policy: scriptProcessPolicy{Version: 1, Owner: owner, MaximumElapsedMilliseconds: maximum}, DeadlineEpochMilliseconds: time.Now().UnixMilli() + maximum, Capabilities: map[string]any{}})
	if err != nil {
		t.Fatal(err)
	}
	t.Setenv(scriptProcessInvocationEnv, string(wire))
}

// 🧰️requireToolchainRoot names the toolchain checkout that owns the canonical TypeScript dispatcher (SEMIO_TOOLCHAIN_ROOT when the caller exports it, else the checkout that contains this module), proves the dispatcher script and bun are there, exports the root and the script process envelope to the child processes of this test, and never binds the checkout as the repository.
func requireToolchainRoot(t *testing.T) string {
	t.Helper()
	root := os.Getenv("SEMIO_TOOLCHAIN_ROOT")
	if root == "" {
		root = sandbox.OriginPath("..", "..", "..", "..", "..", "..")
	}
	script := filepath.Join(root, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "📦️packages", "🟦️typescript", "📜️script.ts")
	if _, err := os.Stat(script); err != nil {
		t.Fatalf("the toolchain checkout %s has no canonical dispatcher script: %v", root, err)
	}
	if _, err := exec.LookPath("bun"); err != nil {
		t.Fatalf("the canonical dispatcher needs bun on PATH: %v", err)
	}
	t.Setenv("SEMIO_TOOLCHAIN_ROOT", root)
	provideScriptEnvelope(t, "client-test:canonical-go-test-dispatch", 2*time.Minute)
	return root
}

// 🎯️withGoalPath seeds the exhaustive repository and the goal chain below its seeded top-level goal, so goal-path conversions answer from goals the test owns.
func withGoalPath(t *testing.T, path string) string {
	t.Helper()
	root := withExhaustiveRepo(t)
	segments := strings.Split(path, "/")
	for index := 1; index < len(segments); index++ {
		words := strings.Split(strings.ToLower(segments[index]), "-")
		for position, word := range words {
			words[position] = strings.ToUpper(word[:1]) + word[1:]
		}
		parent := strings.Join(segments[:index], "/")
		if result := ToolGoalCreate(strings.Join(words, " "), "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent, ""); result.Error != "" {
			t.Fatalf("failed to seed goal %q below %q: %s", segments[index], parent, result.Error)
		}
	}
	return root
}

// 🧫️withSectionFixture points the repo root at a throwaway tree holding one sectioned file.
func withSectionFixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	abs := filepath.Join(root, filepath.FromSlash(sectionFixtureRelPath))
	if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
		t.Fatalf("mkdir fixture dir: %v", err)
	}
	if err := os.WriteFile(abs, []byte(sectionFixtureContent), 0o644); err != nil {
		t.Fatalf("write fixture file: %v", err)
	}
	previous := rootDir
	rootDir = root
	t.Cleanup(func() { rootDir = previous })
	InvalidateTechnologyCache()
	return sectionFixtureRelPath
}

// 🕸️newExecutorFor binds a GraphQL executor to one repository root and restores the shared workspace root when the test ends.
func newExecutorFor(t *testing.T, root string) *graphql.Executor {
	t.Helper()
	previous := workspace.GetRootDir()
	t.Cleanup(func() { workspace.SetRootDir(previous) })
	ex, err := graphql.NewExecutor(root)
	if err != nil {
		t.Fatalf("failed to create executor: %v", err)
	}
	return ex
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

// 🔧️setupToolTest binds the tool layer to a fixture repository the test owns and returns its root.
func setupToolTest(t *testing.T) string {
	t.Helper()
	return withExhaustiveRepo(t)
}
