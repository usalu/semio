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
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
	"time"
)

func TestMain(m *testing.M) {
	ensureExecutor()
	os.Exit(m.Run())
}

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
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = findTestRepoRoot(cwd)
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

func getTestExecutor(t *testing.T) *graphql.Executor {
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}

	rootDir = findTestRepoRoot(cwd)
	ex, err := graphql.NewExecutor(rootDir)
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

func setupToolTest(t *testing.T) {
	t.Helper()
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	rootDir = findTestRepoRoot(cwd)
	InvalidateTechnologyCache()
}
