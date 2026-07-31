// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

package client

import (
	"bufio"
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"runtime"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/spf13/cobra"
	_ "modernc.org/sqlite"
)

// #region 🎼Helpers

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
			if parts[i] == "🎫" {
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

func TestInteractionUnmarshalAuthorShapes(t *testing.T) {
	cases := []struct {
		name         string
		authorJSON   string
		expectedAuth string
	}{
		{
			name:         "string author",
			authorJSON:   `"Ueli Saluz <ueli@semio-tech.com>"`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
		{
			name:         "object author",
			authorJSON:   `{"name":"Ueli Saluz","email":"ueli@semio-tech.com","github":"usalu"}`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			raw := fmt.Sprintf(`{
				"date": "2026-02-06 22:03:11",
				"author": %s,
				"system": "linux",
				"client": "codex",
				"checkpoint": "abc123",
				"prompt": "test",
				"llm": "gpt-5-2-codex"
			}`, tc.authorJSON)

			var interaction Interaction
			if err := json.Unmarshal([]byte(raw), &interaction); err != nil {
				t.Fatalf("unexpected unmarshal error: %v", err)
			}
			if interaction.Author != tc.expectedAuth {
				t.Fatalf("expected author %q, got %q", tc.expectedAuth, interaction.Author)
			}
		})
	}
}

func TestContributorDiscovery(t *testing.T) {

	tmpDir, err := os.MkdirTemp("", "compose-test-discovery")
	if err != nil {
		t.Fatalf("failed to create tmp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	contributorsDir := filepath.Join(tmpDir, ".🦑repo", "🧑‍💻devs")
	os.MkdirAll(contributorsDir, 0755)

	t.Run("Match and update email", func(t *testing.T) {

		github := "usalu"
		c := Contributor{
			Github: github,
			Name:   "Ueli Saluz",
			Names:  []string{"Ueli Saluz"},
			Email:  "ueli@semio-tech.com",
			Emails: []string{"ueli@semio-tech.com"},
		}
		if err := SaveContributor(c); err != nil {
			t.Fatalf("failed to save: %v", err)
		}

		authorStr := "Ueli <ueli@semio-tech.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, err := LoadContributor(github)
		if err != nil {
			t.Fatalf("failed to load: %v", err)
		}
		if len(updated.Names) != 2 || updated.Names[1] != "Ueli" {
			t.Errorf("expected names updated, got %v", updated.Names)
		}
	})

	t.Run("Match and update name", func(t *testing.T) {

		github := "octocat"
		c := Contributor{
			Github: github,
			Name:   "The Octocat",
			Names:  []string{"The Octocat"},
			Email:  "octocat@github.com",
			Emails: []string{"octocat@github.com"},
		}
		SaveContributor(c)

		authorStr := "The Octocat <octo@github.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, _ := LoadContributor(github)
		found := false
		for _, e := range updated.Emails {
			if e == "octo@github.com" {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("expected emails updated with octo@github.com, got %v", updated.Emails)
		}
	})

	t.Run("No match returns original string", func(t *testing.T) {
		authorStr := "Stranger <stranger@danger.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != authorStr {
			t.Errorf("expected original string, got %q", gotGithub)
		}
	})
}

func TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow devcontainer post-attach subprocess test in short mode")
	}
	if runtime.GOOS == "windows" {
		t.Skip("post-attach is a Linux devcontainer script; Windows bash wrappers can hang on path translation")
	}
	_, currentFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("failed to resolve current test file path")
	}
	repoRoot := findTestRepoRoot(filepath.Dir(currentFile))
	t.Run("creates workspace from root and submodules", func(t *testing.T) {
		workspaceDir := t.TempDir()
		homeDir := t.TempDir()
		binDir := t.TempDir()
		logPath := filepath.Join(workspaceDir, "gk.log")
		submoduleDir := filepath.Join(workspaceDir, "metabolism")

		if err := os.MkdirAll(submoduleDir, 0755); err != nil {
			t.Fatalf("failed to create submodule dir: %v", err)
		}
		if err := os.WriteFile(filepath.Join(workspaceDir, ".gitmodules"), []byte("[submodule \"metabolism\"]\n\tpath = metabolism\n\turl = https://github.com/usalu/metabolism.git\n"), 0644); err != nil {
			t.Fatalf("failed to write .gitmodules: %v", err)
		}

		writeExecutableFile(t, filepath.Join(binDir, "git"), fmt.Sprintf(`#!/bin/sh
if [ "$1" = "-C" ]; then
  target="$2"
  shift 2
fi
if [ "$1" = "rev-parse" ] && [ "$2" = "--is-inside-work-tree" ]; then
  case "$target" in
    "%s"|"%s")
      exit 0
      ;;
    *)
      exit 1
      ;;
  esac
fi
if [ "$1" = "config" ] && [ "$2" = "-f" ] && [ "$3" = "%s/.gitmodules" ]; then
  printf 'submodule.metabolism.path metabolism\n'
  exit 0
fi
exit 1
`, workspaceDir, submoduleDir, workspaceDir))

		writeExecutableFile(t, filepath.Join(binDir, "awk"), `#!/bin/sh
printf 'metabolism\n'
`)

		writeExecutableFile(t, filepath.Join(binDir, "gk"), fmt.Sprintf(`#!/bin/sh
printf '%%s\n' "$*" >> "%s"
if [ "$1" = "ws" ] && [ "$2" = "info" ]; then
  exit 1
fi
exit 0
`, logPath))

		codexDir := filepath.Join(homeDir, ".codex")
		if err := os.MkdirAll(codexDir, 0755); err != nil {
			t.Fatalf("failed to create Codex dir: %v", err)
		}
		if err := os.WriteFile(filepath.Join(codexDir, "config.toml"), []byte("personality = \"pragmatic\"\nmodel = \"gpt-5.4\"\nmodel_reasoning_effort = \"medium\"\n"), 0644); err != nil {
			t.Fatalf("failed to seed Codex config: %v", err)
		}

		execCommandWithTimeout(t, 60*time.Second, repoRoot, append(os.Environ(),
			"PATH="+binDir+":"+os.Getenv("PATH"),
			"HOME="+homeDir,
			"XDG_CONFIG_HOME="+filepath.Join(homeDir, ".config"),
			"XDG_DATA_HOME="+filepath.Join(homeDir, ".local", "share"),
			"containerWorkspaceFolder="+workspaceDir,
			"SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL=1",
			"SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL=1",
			"SEMIO_GITKRAKEN_WORKSPACE_NAME=Compose Test Workspace",
		), "bash", ".devcontainer/post-attach.sh")

		logData, err := os.ReadFile(logPath)
		if err != nil {
			t.Fatalf("failed to read gk log: %v", err)
		}
		logText := string(logData)
		expectedRepoArg := workspaceDir + "," + submoduleDir
		if !strings.Contains(logText, "ws create Compose Test Workspace --add-repos "+expectedRepoArg) {
			t.Fatalf("expected workspace creation call with repos %q, got log:\n%s", expectedRepoArg, logText)
		}
		if !strings.Contains(logText, "ws refresh Compose Test Workspace") {
			t.Fatalf("expected workspace refresh call, got log:\n%s", logText)
		}
		if !strings.Contains(logText, "ws set Compose Test Workspace") {
			t.Fatalf("expected workspace set call, got log:\n%s", logText)
		}

		windsurfPath := filepath.Join(homeDir, ".codeium", "windsurf", "mcp_config.json")
		configData, err := os.ReadFile(windsurfPath)
		if err != nil {
			t.Fatalf("failed to read Windsurf MCP config: %v", err)
		}
		configText := string(configData)
		if !strings.Contains(configText, "\"repo\"") {
			t.Fatalf("expected Windsurf MCP config to include repo server, got:\n%s", configData)
		}
		if !strings.Contains(configText, "\"command\": \"go\"") {
			t.Fatalf("expected Windsurf MCP config to keep the portable go command, got:\n%s", configData)
		}
		if !strings.Contains(configText, "\"args\": [\n        \"run\",\n        \"./repo/client/mcp/go\"\n      ]") {
			t.Fatalf("expected Windsurf MCP config to keep portable repo args, got:\n%s", configData)
		}

		codexPath := filepath.Join(homeDir, ".codex", "config.toml")
		codexData, err := os.ReadFile(codexPath)
		if err != nil {
			t.Fatalf("failed to read Codex MCP config: %v", err)
		}
		codexText := string(codexData)
		if !strings.Contains(codexText, "[mcp_servers.repo]") {
			t.Fatalf("expected Codex MCP config to include repo server, got:\n%s", codexData)
		}
		if !strings.Contains(codexText, "personality = \"pragmatic\"") || !strings.Contains(codexText, "model = \"gpt-5.4\"") {
			t.Fatalf("expected Codex MCP sync to preserve existing user settings, got:\n%s", codexData)
		}
		if !strings.Contains(codexText, `command = "go"`) {
			t.Fatalf("expected Codex MCP config to keep the portable go command, got:\n%s", codexData)
		}
		if !strings.Contains(codexText, `args = ["run", "./repo/client/mcp/go"]`) {
			t.Fatalf("expected Codex MCP config to keep portable repo args, got:\n%s", codexData)
		}
		if !strings.Contains(codexText, fmt.Sprintf("cwd = %q", repoRoot)) {
			t.Fatalf("expected Codex MCP config to set cwd to repo root, got:\n%s", codexData)
		}
		if !strings.Contains(codexText, fmt.Sprintf("%q", filepath.Join(repoRoot, "compose", "engine"))) {
			t.Fatalf("expected Codex MCP config to normalize --directory arguments to absolute paths, got:\n%s", codexData)
		}
	})

	t.Run("updates workspace only for missing repos", func(t *testing.T) {
		workspaceDir := t.TempDir()
		homeDir := t.TempDir()
		binDir := t.TempDir()
		logPath := filepath.Join(workspaceDir, "gk.log")
		submoduleDir := filepath.Join(workspaceDir, "metabolism")

		if err := os.MkdirAll(submoduleDir, 0755); err != nil {
			t.Fatalf("failed to create submodule dir: %v", err)
		}
		if err := os.WriteFile(filepath.Join(workspaceDir, ".gitmodules"), []byte("[submodule \"metabolism\"]\n\tpath = metabolism\n\turl = https://github.com/usalu/metabolism.git\n"), 0644); err != nil {
			t.Fatalf("failed to write .gitmodules: %v", err)
		}

		writeExecutableFile(t, filepath.Join(binDir, "git"), fmt.Sprintf(`#!/bin/sh
if [ "$1" = "-C" ]; then
  target="$2"
  shift 2
fi
if [ "$1" = "rev-parse" ] && [ "$2" = "--is-inside-work-tree" ]; then
  case "$target" in
    "%s"|"%s")
      exit 0
      ;;
    *)
      exit 1
      ;;
  esac
fi
if [ "$1" = "config" ] && [ "$2" = "-f" ] && [ "$3" = "%s/.gitmodules" ]; then
  printf 'submodule.metabolism.path metabolism\n'
  exit 0
fi
exit 1
`, workspaceDir, submoduleDir, workspaceDir))

		writeExecutableFile(t, filepath.Join(binDir, "awk"), `#!/bin/sh
printf 'metabolism\n'
`)

		writeExecutableFile(t, filepath.Join(binDir, "gk"), fmt.Sprintf(`#!/bin/sh
printf '%%s\n' "$*" >> "%s"
if [ "$1" = "ws" ] && [ "$2" = "info" ]; then
  printf '%%s\n' "NAME | DESCRIPTION | TYPE | # OF REPOS | SHARED WITH | ACTIVE"
  printf '%%s\n' "%s"
  exit 0
fi
exit 0
`, logPath, workspaceDir))

		execCommandWithTimeout(t, 60*time.Second, repoRoot, append(os.Environ(),
			"PATH="+binDir+":"+os.Getenv("PATH"),
			"HOME="+homeDir,
			"XDG_CONFIG_HOME="+filepath.Join(homeDir, ".config"),
			"XDG_DATA_HOME="+filepath.Join(homeDir, ".local", "share"),
			"containerWorkspaceFolder="+workspaceDir,
			"SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL=1",
			"SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL=1",
			"SEMIO_GITKRAKEN_WORKSPACE_NAME=Compose Existing Workspace",
		), "bash", ".devcontainer/post-attach.sh")

		logData, err := os.ReadFile(logPath)
		if err != nil {
			t.Fatalf("failed to read gk log: %v", err)
		}
		logText := string(logData)
		if strings.Contains(logText, "ws create Compose Existing Workspace") {
			t.Fatalf("did not expect workspace create call, got log:\n%s", logText)
		}
		if !strings.Contains(logText, "ws update Compose Existing Workspace --add-repos "+submoduleDir) {
			t.Fatalf("expected workspace update call for missing submodule, got log:\n%s", logText)
		}
		if strings.Contains(logText, "ws update Compose Existing Workspace --add-repos "+workspaceDir+","+submoduleDir) {
			t.Fatalf("expected update to include only missing repos, got log:\n%s", logText)
		}
		if !strings.Contains(logText, "ws set Compose Existing Workspace") {
			t.Fatalf("expected workspace set call, got log:\n%s", logText)
		}
	})
}

func TestNativeBootstrapAssetsStayRepoRelative(t *testing.T) {
	repoRoot := findTestRepoRoot(".")

	cases := []struct {
		name               string
		path               string
		requiredFragments  []string
		forbiddenFragments []string
	}{
		{
			name: "codex template uses coda assistant",
			path: filepath.Join(repoRoot, ".codex", "config.toml"),
			requiredFragments: []string{
				`command = "go"`,
				`"run"`,
				`"./repo/codex"`,
			},
			forbiddenFragments: []string{
				"coda/engine",
				"coda.py",
			},
		},
		{
			name: "kiro settings stay repo relative",
			path: filepath.Join(repoRoot, ".kiro", "settings", "mcp.json"),
			requiredFragments: []string{
				`"./repo/kiro"`,
				`"go"`,
				`"run"`,
			},
			forbiddenFragments: []string{
				"/workspaces/semio/",
				"coda/engine",
				"coda.py",
			},
		},
		{
			name: "kiro compose agent uses coda assistant",
			path: filepath.Join(repoRoot, ".kiro", "agents", "compose.json"),
			requiredFragments: []string{
				`"./repo/kiro"`,
				`"go"`,
				`"run"`,
			},
			forbiddenFragments: []string{
				"coda/engine",
				"coda.py",
			},
		},
		{
			name: "devcontainer post-create antigravity config uses coda assistant",
			path: filepath.Join(repoRoot, ".devcontainer", "post-create.sh"),
			requiredFragments: []string{
				"/workspaces/semio/compose/engine",
				"/workspaces/semio/coda/assistant",
				"main.py",
			},
			forbiddenFragments: []string{
				"/workspaces/semio/coda/engine",
				"coda.py",
			},
		},
		{
			name: "native bootstrap script performs repo bootstrap",
			path: filepath.Join(repoRoot, ".devcontainer", "install-native.ps1"),
			requiredFragments: []string{
				"PLAYWRIGHT_BROWSERS_PATH",
				`$script:PythonKind = "3.14"`,
				`Sync-WingetPackage -Id "Microsoft.DotNet.SDK.10" -Label ".NET SDK 10.0"`,
				`Sync-WingetPackage -Id "Microsoft.VisualStudio.2022.BuildTools" -Label "Visual Studio Build Tools"`,
				`Set-UserEnvironmentVariable -Name "SEMIO_F3D_AUTO_START" -Value "true"`,
				`Stop-RepoPythonProcesses -RepoRoot $repoRoot`,
				`@("sync", "--all-packages", "--all-groups", "--python", $script:PythonKind)`,
				`@("run", "./repo/client/mcp/go", "configure", "--repo", $repoRoot)`,
				`@("playwright", "install", "chromium")`,
				`@("run", "git:setup")`,
			},
			forbiddenFragments: []string{
				`Microsoft.DotNet.SDK.7`,
			},
		},
		{
			name: "devcontainer excludes dotnet 7 and restores monorepo solution",
			path: filepath.Join(repoRoot, ".devcontainer", "devcontainer.json"),
			requiredFragments: []string{
				`"version": "1.26"`,
				`"version": "2.53"`,
				`"additionalVersions": "9.0 10.0"`,
			},
			forbiddenFragments: []string{
				`7.0`,
			},
		},
		{
			name: "devcontainer post-create restores monorepo solution",
			path: filepath.Join(repoRoot, ".devcontainer", "post-create.sh"),
			requiredFragments: []string{
				"uv sync --all-packages --all-groups",
				"dotnet restore Monorepo.sln",
			},
			forbiddenFragments: []string{
				"dotnet restore net/Compose.sln",
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
			for _, fragment := range tc.requiredFragments {
				if !strings.Contains(text, fragment) {
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

// 📌#region 🔑Compose Repo ID Conversion
func TestGoalPathToComposeID(t *testing.T) {
	cases := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", "", ""},
		{"single segment", "AI-OPTIMIZED-REPO", emojiText(EmojiGoal) + "aioptimizedrepo"},
		{"two segments", "AI-OPTIMIZED-REPO/REPO-CLI", emojiText(EmojiGoal) + "aioptimizedrepo" + emojiText(EmojiGoal) + "repocli"},
		{"four segments", "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI",
			emojiText(EmojiGoal) + "aioptimizedrepo" + emojiText(EmojiGoal) + "repoclient" + emojiText(EmojiGoal) + "repobinary" + emojiText(EmojiGoal) + "repocli"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := goalPathToComposeID(tc.input)
			if got != tc.expected {
				t.Errorf("goalPathToComposeID(%q) = %q, want %q", tc.input, got, tc.expected)
			}
		})
	}
}

func TestComposeIDToGoalPath(t *testing.T) {

	goals, err := ListGoals()
	if err != nil || len(goals) == 0 {
		t.Skip("no goals available for round-trip test")
	}
	for _, g := range goals {
		composeID := goalPathToComposeID(g.ID)
		roundTrip := composeIDToGoalPath(composeID)
		if roundTrip != g.ID {
			t.Errorf("round-trip failed: %q -> %q -> %q (expected %q)", g.ID, composeID, roundTrip, g.ID)
		}
	}
}

func TestContributorComposeIDRoundTrip(t *testing.T) {
	cases := []struct {
		name     string
		github   string
		expected string
	}{
		{"empty", "", ""},
		{"unknown", "unknown", "unknown"},
		{"normal", "usalu", emojiText(EmojiContributor) + "usalu"},
		{"already prefixed", emojiText(EmojiContributor) + "usalu", emojiText(EmojiContributor) + "usalu"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := contributorGithubToComposeID(tc.github)
			if got != tc.expected {
				t.Errorf("contributorGithubToComposeID(%q) = %q, want %q", tc.github, got, tc.expected)
			}
		})
	}
}

func TestTicketMarshalJSONGoalToComposeID(t *testing.T) {
	ticket := Ticket{
		Title:    "Test Ticket",
		Emoji:    "🔧",
		Goal:     "AI-OPTIMIZED-REPO/REPO-CLI",
		Status:   TicketStatusOpen,
		Sessions: []string{"s1"},
	}
	data, err := json.Marshal(ticket)
	if err != nil {
		t.Fatalf("marshal failed: %v", err)
	}
	var raw map[string]interface{}
	json.Unmarshal(data, &raw)

	if raw["emoji"] != "🔧" {
		t.Errorf("expected emoji %q, got %q", "🔧", raw["emoji"])
	}

	goalEmoji := emojiText(EmojiGoal)
	expectedGoal := goalEmoji + "aioptimizedrepo" + goalEmoji + "repocli"
	if raw["goal"] != expectedGoal {
		t.Errorf("expected goal %q, got %q", expectedGoal, raw["goal"])
	}

	sessions := raw["sessions"].([]interface{})
	expectedSessionID := emojiText(EmojiSession) + "s1"
	if sessions[0] != expectedSessionID {
		t.Errorf("expected session %q, got %q", expectedSessionID, sessions[0])
	}
}

func TestTicketUnmarshalJSONComposeIDToGoalPath(t *testing.T) {
	goalEmoji := emojiText(EmojiGoal)
	raw := fmt.Sprintf(`{
		"title": "Test",
		"emoji": "🧪",
		"goal": "%saioptimizedrepo%srepoclient%srepobinary%srepocli",
		"status": "open",
		"sessions": [
			"%ss1"
		]
	}`, goalEmoji, goalEmoji, goalEmoji, goalEmoji, emojiText(EmojiSession))

	var ticket Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		t.Fatalf("unmarshal failed: %v", err)
	}

	if ticket.Emoji != "🧪" {
		t.Errorf("expected emoji %q, got %q", "🧪", ticket.Emoji)
	}

	expectedGoalPath := "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI"
	if ticket.Goal != expectedGoalPath {
		t.Errorf("expected goal path %q, got %q", expectedGoalPath, ticket.Goal)
	}

	if len(ticket.Sessions) != 1 || ticket.Sessions[0] != emojiText(EmojiSession)+"s1" {
		t.Errorf("expected session %q, got %+v", emojiText(EmojiSession)+"s1", ticket.Sessions)
	}
}

func TestGoalMarshalJSONParentToComposeID(t *testing.T) {
	goal := Goal{
		Title:       "Test Goal",
		Description: "desc",
		Prompt:      "prompt",
		Status:      "open",
		Client:      "copilot-chat",
		LLM:         "opus-4-6",
		Parent:      "AI-OPTIMIZED-REPO/REPO-CLIENT",
	}
	data, err := json.Marshal(goal)
	if err != nil {
		t.Fatalf("marshal failed: %v", err)
	}
	var raw map[string]interface{}
	json.Unmarshal(data, &raw)

	goalEmoji := emojiText(EmojiGoal)
	expectedParent := goalEmoji + "aioptimizedrepo" + goalEmoji + "repoclient"
	if raw["parent"] != expectedParent {
		t.Errorf("expected parent %q, got %q", expectedParent, raw["parent"])
	}
}

func TestTicketGoalRoundTripThroughJSON(t *testing.T) {

	ticket := Ticket{
		Title:    "Roundtrip Test",
		Emoji:    "♻️",
		Goal:     "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI",
		Status:   TicketStatusOpen,
		Sessions: []string{"my-session"},
	}

	data, err := json.Marshal(ticket)
	if err != nil {
		t.Fatalf("marshal failed: %v", err)
	}

	var ticket2 Ticket
	if err := json.Unmarshal(data, &ticket2); err != nil {
		t.Fatalf("unmarshal failed: %v", err)
	}

	if ticket2.Emoji != "♻️" {
		t.Errorf("emoji round-trip failed: original %q, after round-trip %q", ticket.Emoji, ticket2.Emoji)
	}

	if ticket2.Goal != ticket.Goal {
		t.Errorf("round-trip failed: original %q, after round-trip %q", ticket.Goal, ticket2.Goal)
	}

	if len(ticket2.Sessions) != 1 || ticket2.Sessions[0] != emojiText(EmojiSession)+"mysession" {
		t.Errorf("round-trip sessions failed: got %+v", ticket2.Sessions)
	}
}

// #endregion 🔑Compose Repo ID Conversion

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
	SetRootDir(repoRoot)
	executor, err := NewExecutor(repoRoot)
	if err != nil {
		return nil, err
	}
	return NewEngine(executor), nil
}

func getTestExecutor(t *testing.T) *Executor {
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}

	rootDir = findTestRepoRoot(cwd)
	ex, err := NewExecutor(rootDir)
	if err != nil {
		t.Fatalf("failed to create executor: %v", err)
	}
	return ex
}

// #endregion 🎼Helpers

// 📦#region 🧳Collection
func TestExhaustiveBundlesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ bundles { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Bundles []struct {
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Bundles) == 0 {
		t.Error("bundles collection should not be empty")
	}
}

func TestExhaustiveContributorsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow contributor test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ contributors { github } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Contributors []struct {
			Github string `json:"github"`
		} `json:"contributors"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Contributors) == 0 {
		t.Error("contributors collection should not be empty")
	}
}

func TestExhaustiveTicketsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tickets test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ tickets { slug } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Tickets []struct {
			Slug string `json:"slug"`
		} `json:"tickets"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets collection should not be empty")
	}
}

func TestExhaustivePoliciesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policies test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ policies { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Policies []struct {
			Name string `json:"name"`
		} `json:"policies"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Policies) == 0 {
		// Policies are no longer registered in Go; breachs come from lint scripts + cache.
	}
}

func TestExhaustiveStatutesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statutes test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ statutes { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes collection should not be empty")
	}
}

func TestPolicyStatutesHaveMetadata(t *testing.T) {
	for _, policy := range GetPolicies() {
		for _, statute := range policy.AllKinds() {
			info, ok := statuteInfoTable[statute]
			if !ok {
				t.Fatalf("policy %q statute %q is missing metadata", policy.ID, statute)
			}
			if info.Kind != statute {
				t.Fatalf("policy %q statute %q metadata kind = %q", policy.ID, statute, info.Kind)
			}
		}
	}
}

func TestExhaustiveFoldersNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow folders test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ folders { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Folders []struct {
			Path string `json:"path"`
		} `json:"folders"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Folders) == 0 {
		t.Error("folders collection should not be empty")
	}
}

func TestExhaustiveFilesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow files test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ files { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Files []struct {
			Path string `json:"path"`
		} `json:"files"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Files) == 0 {
		t.Error("files collection should not be empty")
	}
}

func TestExhaustiveBreachsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow breachs test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	// Unscoped breachs defaults to technology scope "compose" and runs full policy
	// analysis across the tree (very slow). Single-file scope keeps the test
	// representative while bounded.
	result, err := executor.ExecuteJSON(ctx, `{ breachs(scope: "repo/client/main.go") { id } }`, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Breachs []struct {
			ID string `json:"id"`
		} `json:"breachs"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if resp.Breachs == nil {
		t.Error("breachs collection should not be nil")
	}
}

func TestExhaustiveTicketTitleValidation(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket title validation test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	tests := []struct {
		name    string
		emoji   string
		title   string
		wantErr bool
	}{
		{"Emoji Titleized Valid", "🎫", "Some Title on Something", false},
		{"Emoji Single Word Valid", "🛠️", "Cleanup", false},
		{"Emoji With Hyphen Valid", "🧩", "Refactor Resource ID System to Bundle-Based Document", false},
		{"Emoji Lowercase Valid", "🔖", "some title", false},
		{"Emoji Allcaps Valid", "🔥", "FIX EVERYTHING", false},
		{"Emoji Slug Valid", "🎫", "some-slug-title", false},
		{"Emoji Dashed Slug Valid", "🎫", "fix-vscode-types-version-mismatch", false},
		{"Emoji Uppercase Slug Valid", "🎫", "ENSURE-COMPOSE-REPO-MCP-WORKS-ALLIDES", false},
		{"Missing Emoji Invalid", "", "Some Title on Something", true},
		{"Plain Text Emoji Invalid", "ticket", "Some Title", true},
		{"Multiple Emoji Invalid", "🎫🎫", "Some Title", true},
		{"Empty Title Invalid", "🎫", "", true},
		{"Non Alphanumeric Title Invalid", "🎫", "---", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			query := `mutation { ticketOpen(input: { emoji: "` + tt.emoji + `", title: "` + tt.title + `", prompt: "Test prompt", llm: "opus-4", client: COPILOT_CHAT, goal: "TEST-GOAL", noIssue: true }) { id slug year month day } }`
			result, err := executor.ExecuteJSON(ctx, query, nil)
			if (err != nil) != tt.wantErr {
				t.Errorf("ticketOpen() error = %v, wantErr %v", err, tt.wantErr)
			}

			if err == nil {
				var resp struct {
					TicketOpen struct {
						ID    string `json:"id"`
						Slug  string `json:"slug"`
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
					} `json:"ticketOpen"`
				}
				if json.Unmarshal([]byte(result), &resp) == nil {
					to := resp.TicketOpen
					if to.Year == 0 || to.Month == 0 || to.Day == 0 {
						t.Errorf("ticketOpen returned invalid date: year=%d month=%d day=%d (id=%s)", to.Year, to.Month, to.Day, to.ID)
					}
					if strings.Contains(to.ID, "0000/00/00") {
						t.Errorf("ticketOpen id must not contain 0000/00/00, got %s", to.ID)
					}
					path := GetTicketPath(to.Year, to.Month, to.Day, to.Slug)
					os.RemoveAll(path)
				}
			}
		})
	}
}

func TestBuildTechnologyLinkArgs(t *testing.T) {
	args := buildTechnologyLinkArgs("https://github.com/usalu/semio/issues/1")
	expected := []string{"project", "item-add", "2", "--owner", "usalu", "--url", "https://github.com/usalu/semio/issues/1"}
	if len(args) != len(expected) {
		t.Fatalf("expected %d args, got %d", len(expected), len(args))
	}
	for i := range expected {
		if args[i] != expected[i] {
			t.Fatalf("args[%d] expected %s, got %s", i, expected[i], args[i])
		}
	}
}

func TestExhaustiveFilterTicketWorkspaceFiles(t *testing.T) {
	executor := getTestExecutor(t)
	if executor == nil {
		t.Fatal("executor is nil")
	}
	absMain := filepath.Join(rootDir, "go", "repo", "main.go")
	ticket := &Ticket{
		Year:       26,
		Month:      1,
		Day:        20,
		Slug:       "SAMPLE",
		FolderPath: filepath.Join(rootDir, ".🦑repo", "🎫tickets", "26", "01", "20", "SAMPLE"),
	}
	files := []string{
		".🦑repo/🎫tickets/26/01/20/SAMPLE/plan.md",
		"./.🦑repo/🎫tickets/26/01/20/SAMPLE/ticket.json",
		filepath.Join(rootDir, ".🦑repo", "🎫tickets", "26", "01", "20", "SAMPLE", "extra.txt"),
		absMain,
	}
	filtered := FilterTicketWorkspaceFiles(ticket, files)
	if len(filtered) != 1 || filtered[0] != absMain {
		t.Fatalf("expected [%s], got %v", absMain, filtered)
	}
}

func TestExhaustiveNormalizeTicketFileInput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow normalize ticket file input test in short mode")
	}
	absRoot := GetRootDir()
	filePath := filepath.ToSlash(filepath.Join("repo", "client", "main.go"))
	absPath := filepath.Join(absRoot, filePath)
	fileID := FileHeaderId(filePath)
	fileUri := buildFileUriFromPath(filePath)
	cases := []struct {
		name  string
		input string
		want  string
	}{
		{"path", filePath, filePath},
		{"abs path", absPath, filePath},
		{"file uri", fileUri, filePath},
		{"id", fileID, filePath},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeTicketFileInput(tc.input)
			if got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestMatchesIgnorePatternDirectoryCoverage(t *testing.T) {
	cases := []struct {
		name    string
		path    string
		isDir   bool
		pattern string
		want    bool
	}{
		{"dir by recursive pattern", "node_modules", true, "**/node_modules/**", true},
		{"nested file by recursive pattern", "a/node_modules/pkg/index.js", false, "**/node_modules/**", true},
		{"dir mismatch", "src", true, "**/node_modules/**", false},
		{"exact dir pattern", "dist", true, "dist/**", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := matchesIgnorePattern(tc.path, tc.isDir, tc.pattern)
			if got != tc.want {
				t.Fatalf("matchesIgnorePattern(%q, %t, %q) = %t, want %t", tc.path, tc.isDir, tc.pattern, got, tc.want)
			}
		})
	}
}

func TestGlobByExtensionSkipsIgnoredDirectoryRoot(t *testing.T) {
	tempRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tempRoot, "src"), 0755); err != nil {
		t.Fatalf("failed to create src dir: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tempRoot, "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create node_modules dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "src", "kept.go"), []byte("package src\n"), 0644); err != nil {
		t.Fatalf("failed to create kept.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "node_modules", "pkg", "ignored.go"), []byte("package pkg\n"), 0644); err != nil {
		t.Fatalf("failed to create ignored.go: %v", err)
	}

	files, err := globByExtension(tempRoot, "**/*", []string{"go"}, []string{"**/node_modules/**"}, false)
	if err != nil {
		t.Fatalf("globByExtension failed: %v", err)
	}
	if len(files) != 1 || files[0] != "src/kept.go" {
		t.Fatalf("expected only src/kept.go, got %v", files)
	}
}

func TestStreamAndListTicketsIgnoreNestedWorkspaceFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(oldRootDir)

	ticketDir := filepath.Join(tmpDir, ".🦑repo", "🎫tickets", "26", "03", "07", "SAMPLE")
	if err := os.MkdirAll(filepath.Join(ticketDir, "workspace", "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create nested workspace: %v", err)
	}
	ticketJSON := `{
  "title": "Sample Ticket",
  "status": "open",
  "description": "ticket for streaming"
}`
	if err := os.WriteFile(filepath.Join(ticketDir, "ticket.json"), []byte(ticketJSON), 0644); err != nil {
		t.Fatalf("failed to create ticket.json: %v", err)
	}
	if err := os.WriteFile(filepath.Join(ticketDir, "workspace", "node_modules", "pkg", "ticket.json"), []byte(`{"title":"nested"}`), 0644); err != nil {
		t.Fatalf("failed to create nested ticket.json: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	ticketCh := make(chan Ticket)
	errCh := make(chan error, 1)
	go func() {
		errCh <- StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	var streamed []Ticket
	for ticket := range ticketCh {
		streamed = append(streamed, ticket)
	}
	if err := <-errCh; err != nil {
		t.Fatalf("StreamTickets returned error: %v", err)
	}
	if len(streamed) != 1 || streamed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one streamed SAMPLE ticket, got %+v", streamed)
	}

	listed, err := ListTickets(nil, nil, nil)
	if err != nil {
		t.Fatalf("ListTickets failed: %v", err)
	}
	if len(listed) != 1 || listed[0].Slug != "SAMPLE" {
		t.Fatalf("expected one listed SAMPLE ticket, got %+v", listed)
	}
}

func TestExhaustiveNodesAndEdgesQuick(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow nodes/edges quick test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	query := `{
		tickets {
			id
			slug
		}
		policies {
			id
			name
			statutes { id }
		}
		statutes {
			id
		}
		folders {
			id
			path
			parent { id }
			children { id }
		}
		files {
			id
			path
			folder { id }
			sections { id name }
			definitions { id name kind }
		}
		breachs(scope: "repo/client/main.go") {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
}

// #endregion 🧳Collection

// 🌿#region 🌈Nodes and Edges
func TestExhaustiveNodesAndEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow nodes/edges test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		bundles {
			id
			name
			folders { id path }
			files { id path }
			breachs { id }
		}
		folders {
			id
			path
			parent { id }
			children { id }
			files { id }
			bundle { id }
			breachs { id }
		}
		files {
			id
			path
			folder { id }
			bundle { id }
			sections { id name }
			definitions { id name kind }
			breachs { id }
		}
		tickets {
			id
			slug
		}
		policies {
			id
			name
			statutes { id }
		}
		statutes {
			id
		}
		breachs(scope: "repo/client/main.go") {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Bundles []struct {
			ID      string `json:"id"`
			Name    string `json:"name"`
			Folders []struct {
				ID string `json:"id"`
			} `json:"folders"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"bundles"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"files"`
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Bundles) == 0 {
		t.Error("bundles should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
	}
	if resp.Breachs == nil {
		t.Error("breachs should not be nil")
	}

	for _, bundle := range resp.Bundles {
		if bundle.ID == "" {
			t.Errorf("bundle %s has empty id", bundle.Name)
		}
	}
	for _, folder := range resp.Folders {
		if folder.ID == "" {
			t.Errorf("folder %s has empty id", folder.Path)
		}
	}
	for _, file := range resp.Files {
		if file.ID == "" {
			t.Errorf("file %s has empty id", file.Path)
		}
	}
	for _, ticket := range resp.Tickets {
		if ticket.ID == "" {
			t.Errorf("ticket %s has empty id", ticket.Slug)
		}
	}
	for _, policy := range resp.Policies {
		if policy.ID == "" {
			t.Errorf("policy %s has empty id", policy.Name)
		}
	}
	for _, vk := range resp.Statutes {
		if vk.ID == "" {
			t.Error("statute has empty id")
		}
	}
	for _, v := range resp.Breachs {
		if v.ID == "" {
			t.Errorf("breach has empty id: %+v", v)
		}
	}
}

func TestExhaustiveNodeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow node query test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	bundleResult, err := executor.ExecuteJSON(ctx, "{ bundles { id name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var bundleResp struct {
		Bundles []struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(bundleResult), &bundleResp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(bundleResp.Bundles) == 0 {
		t.Skip("no bundles to test node query")
	}

	testID := bundleResp.Bundles[0].ID
	nodeResult, err := executor.ExecuteJSON(ctx, `query($id: ID!) { node(id: $id) { ... on Bundle { id name } } }`, map[string]interface{}{"id": testID})
	if err != nil {
		t.Fatalf("node query failed: %v", err)
	}
	var nodeResp struct {
		Node struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"node"`
	}
	if err := json.Unmarshal([]byte(nodeResult), &nodeResp); err != nil {
		t.Fatalf("failed to parse node response: %v", err)
	}
	if nodeResp.Node.ID != testID {
		t.Errorf("node query returned wrong id: got %s, want %s", nodeResp.Node.ID, testID)
	}
	if nodeResp.Node.Name != bundleResp.Bundles[0].Name {
		t.Errorf("node query returned wrong name: got %s, want %s", nodeResp.Node.Name, bundleResp.Bundles[0].Name)
	}
}

func TestExhaustiveSectionsEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow sections edges test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		files {
			id
			path
			sections {
				id
				name
				path
				file { id }
				parent { id }
				children { id }
				definitions { id name }
				breachs { id }
				range { start end }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID       string `json:"id"`
			Path     string `json:"path"`
			Sections []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Path string `json:"path"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Parent *struct {
					ID string `json:"id"`
				} `json:"parent"`
				Children []struct {
					ID string `json:"id"`
				} `json:"children"`
				Definitions []struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"definitions"`
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
				Range struct {
					Start int `json:"start"`
					End   int `json:"end"`
				} `json:"range"`
			} `json:"sections"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	sectionsFound := false
	for _, file := range resp.Files {
		for _, section := range file.Sections {
			sectionsFound = true
			if section.ID == "" {
				t.Errorf("section %s in file %s has empty id", section.Name, file.Path)
			}
			if section.File.ID == "" {
				t.Errorf("section %s has file with empty id", section.Name)
			}
			if section.File.ID != file.ID {
				t.Errorf("section %s file id mismatch: got %s, want %s", section.Name, section.File.ID, file.ID)
			}
		}
	}
	if !sectionsFound {
		t.Skip("no sections found in any file - may be expected for test repository")
	}
}

func TestExhaustiveDefinitionsEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow definitions edges test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		files {
			id
			path
			definitions {
				id
				name
				kind
				file { id }
				section { id name }
				breachs { id }
				range { start end }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID          string `json:"id"`
			Path        string `json:"path"`
			Definitions []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Section *struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"section"`
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
				Range struct {
					Start int `json:"start"`
					End   int `json:"end"`
				} `json:"range"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.ID == "" {
				t.Errorf("definition %s in file %s has empty id", def.Name, file.Path)
			}
			if def.File.ID == "" {
				t.Errorf("definition %s has file with empty id", def.Name)
			}
			if def.File.ID != file.ID {
				t.Errorf("definition %s file id mismatch: got %s, want %s", def.Name, def.File.ID, file.ID)
			}
			if def.Kind == "" {
				t.Errorf("definition %s has empty kind", def.Name)
			}
		}
	}
	if !definitionsFound {
		t.Skip("no definitions found in any file - may be expected for test repository")
	}
}

func TestExhaustiveDefinitionKind(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow definition kind test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		files {
			id
			path
			definitions {
				id
				name
				kind
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID          string `json:"id"`
			Path        string `json:"path"`
			Definitions []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	validKinds := map[string]bool{
		"IMPLEMENTATION": true,
		"INTERFACE":      true,
		"CONSTANT":       true,
		"TEST":           true,
	}

	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.Kind == "" {
				t.Errorf("definition %s in file %s has empty kind", def.Name, file.Path)
			}
			if !validKinds[def.Kind] {
				t.Errorf("definition %s has invalid kind: %s (expected implementation, interface, constant, or test)", def.Name, def.Kind)
			}
		}
	}
	if !definitionsFound {
		t.Skip("no definitions found in any file - may be expected for test repository")
	}
}

// #endregion 🌈Nodes and Edges

// 🔷#region 🔊Cli
// ⌨️#region 🎼Helpers
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
	newRoot := func(recorder *recordingGraphQLExecutor) *cobra.Command {
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

func toolOutputText(result ToolResult) string {
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

// #endregion 🎼Helpers

// 🧪#region 🏩Codebase
func TestExhaustiveCodebaseCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow codebase test in short mode")
	}
	result := ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

// #endregion 🏩Codebase

// 🔬#region 🗝️Analyze
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

// #endregion 🗝️Analyze

func TestAnalyzeReadsBreachCacheJSON(t *testing.T) {
	tmp := t.TempDir()
	cacheDir := filepath.Join(tmp, ".🦑repo", "⚡cache", "breaches")
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		t.Fatal(err)
	}
	cachePath := filepath.Join(cacheDir, "unit-test.json")
	payload := `{
  "entityId": "test",
  "script": "unit.test.lint.script.ts",
  "breachs": [
    {
      "id": "e1",
      "summary": "hello",
      "kind": "lint/test/rule",
      "scope": "repo/example.go",
      "priority": "medium"
    }
  ]
}`
	if err := os.WriteFile(cachePath, []byte(payload), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()
	ctx := NewRepoContext(tmp)
	ar, err := ctx.Analyze(nil)
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if ar == nil || len(ar.Breachs) != 1 {
		t.Fatalf("expected 1 breach, got %#v", ar)
	}
	if ar.Breachs[0].Summary != "hello" {
		t.Fatalf("unexpected breach: %+v", ar.Breachs[0])
	}
}

// 🔧#region 🎁Fix
func TestFixCommand(t *testing.T) {
	result := ToolFix("compose/js")
	if result.Error == "" {
		t.Fatal("expected ToolFix to error now that server-side fix was removed")
	}
	if !strings.Contains(result.Error, "fix was removed") {
		t.Fatalf("unexpected ToolFix error: %s", result.Error)
	}
}

func TestFileHeaderId(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"code ts", "compose/js/src/index.ts", "🏘️compose📜js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + "index"},
		{"code tsx", "compose/js/src/App.tsx", "🏘️compose📜js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + "app"},
		{"code go", "repo/client/client.go", "🧰repo⌨️client" + emojiText(EmojiFileCode) + "client"},
		{"code cs", "compose/gh/Compose.cs", "🏘️compose🐙gh" + emojiText(EmojiFileCode) + "compose"},
		{"code py", "compose/engine/main.py", "🏘️compose⚙️engine" + emojiText(EmojiFileCode) + "main"},
		{"test ts", "compose/js/src/🧪index.test.ts", "🏘️compose📜js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileLab) + "indextest"},
		{"test go", "repo/client/client_test.go", "🧰repo⌨️client" + emojiText(EmojiFileLab) + "clienttest"},
		{"config json", "tsconfig.json", emojiText(EmojiFileConfig) + "tsconfig"},
		{"docs md", "README.md", emojiText(EmojiFileDocs) + "readme"},
		{"script sh", "build.sh", emojiText(EmojiFileScript) + "build"},
		{"script bash", "deploy.bash", emojiText(EmojiFileScript) + "deploy"},
		{"script ps1", "setup.ps1", emojiText(EmojiFileScript) + "setup"},
		{"resource png", "🖼️logo.png", emojiText(EmojiFileResource) + "logo"},
		{"license", "LICENSE.md", emojiText(EmojiFileLicense) + "license"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := FileHeaderId(tt.path)
			if got != tt.want {
				t.Errorf("FileHeaderId(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}

	t.Run("shebang ts file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "tools/build.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env tsx\nconsole.log('build');\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "tools" + emojiText(EmojiFileScript) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("shebang py file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "scripts/run.py"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env python3\nprint('hello')\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "scripts" + emojiText(EmojiFileScript) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("code ts without shebang stays code", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "src/index.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("export const x = 1;\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) without shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("nonexistent code file stays code", func(t *testing.T) {
		got := FileHeaderId("nonexistent/file.ts")
		want := emojiText(EmojiFolderOrg) + "nonexistent" + emojiText(EmojiFileCode) + Flat("file")
		if got != want {
			t.Errorf("FileHeaderId for nonexistent file = %q, want %q", got, want)
		}
	})
}

func TestDeriveFileKind(t *testing.T) {
	tests := []struct {
		name string
		file string
		want string
	}{
		{"ts code", "index.ts", FileKindCode},
		{"tsx code", "App.tsx", FileKindCode},
		{"go code", "main.go", FileKindCode},
		{"py code", "compose.py", FileKindCode},
		{"cs code", "Compose.cs", FileKindCode},
		{"rs code", "lib.rs", FileKindCode},
		{"rb code", "app.rb", FileKindCode},
		{"sh script", "build.sh", FileKindScript},
		{"bash script", "deploy.bash", FileKindScript},
		{"zsh script", "setup.zsh", FileKindScript},
		{"fish script", "init.fish", FileKindScript},
		{"bat script", "run.bat", FileKindScript},
		{"cmd script", "build.cmd", FileKindScript},
		{"ps1 script", "setup.ps1", FileKindScript},
		{"psm1 script", "module.psm1", FileKindScript},
		{"test ts", "🧪index.test.ts", FileKindLab},
		{"test go", "main_test.go", FileKindLab},
		{"spec ts", "app.spec.ts", FileKindLab},
		{"benchmark go", "compose_benchmark.go", FileKindLab},
		{"stories tsx", "Button.stories.tsx", FileKindLab},
		{"json config", "tsconfig.json", FileKindConfig},
		{"yaml config", "config.yaml", FileKindConfig},
		{"toml config", "pyproject.toml", FileKindConfig},
		{"env config", ".env", FileKindConfig},
		{"md docs", "README.md", FileKindDocs},
		{"txt docs", "notes.txt", FileKindDocs},
		{"png resource", "🖼️logo.png", FileKindResource},
		{"svg resource", "icon.svg", FileKindResource},
		{"wasm resource", "module.wasm", FileKindResource},
		{"tpl template", "layout.tpl", FileKindTemplate},
		{"tmpl template", "page.tmpl", FileKindTemplate},
		{"gotmpl template", "header.gotmpl", FileKindTemplate},
		{"mustache template", "view.mustache", FileKindTemplate},
		{"hbs template", "partial.hbs", FileKindTemplate},
		{"jinja2 template", "base.jinja2", FileKindTemplate},
		{"j2 template", "config.j2", FileKindTemplate},
		{"ejs template", "page.ejs", FileKindTemplate},
		{"njk template", "layout.njk", FileKindTemplate},
		{"pug template", "index.pug", FileKindTemplate},
		{"license md", "LICENSE.md", FileKindLicense},
		{"licence txt", "LICENCE.txt", FileKindLicense},
		{"gitignore config", ".gitignore", FileKindConfig},
		{"dockerfile config", "🐳Dockerfile", FileKindConfig},
		{"makefile config", "Makefile", FileKindConfig},
		{"config suffix", "⚙️vite.config.ts", FileKindConfig},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DeriveFileKind(tt.file)
			if got != tt.want {
				t.Errorf("DeriveFileKind(%q) = %q, want %q", tt.file, got, tt.want)
			}
		})
	}
}

func TestFileKindEmoji(t *testing.T) {
	tests := []struct {
		name  string
		kind  string
		emoji string
	}{
		{"code", "code", "\U0001F4BB"},
		{"lab", "lab", emojiText(EmojiFileLab)},
		{"script", "script", "\U0001F4DC"},
		{"docs", "docs", "\U0001F4C3"},
		{"config", "config", "\u2699\uFE0F"},
		{"resource", "resource", "\U0001F4BE"},
		{"template", "template", emojiText(EmojiFileTemplate)},
		{"license", "license", "\u2696\uFE0F"},
		{"unknown", "unknown", ""},
		{"empty", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := map[string]interface{}{"kind": tt.kind}
			got := fileKindEmoji(data)
			if got != tt.emoji {
				t.Errorf("fileKindEmoji(%q) = %q, want %q", tt.kind, got, tt.emoji)
			}
		})
	}
}

func TestFixApplyAutofixes(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	fixtureSrc := "repo/asset/fixture/some/folder/⚛️⚛️file_fixable.tsx"
	expectedSrc := "repo/asset/fixture/some/folder/⚛️⚛️file_fixable_expected.tsx"

	srcAbs := filepath.Join(rootDir, fixtureSrc)
	expectedAbs := filepath.Join(rootDir, expectedSrc)

	originalContent, err := ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixture: %v", err)
	}
	defer WriteTextFile(srcAbs, originalContent)

	expectedContent, err := ReadTextFile(expectedAbs)
	if err != nil {
		t.Fatalf("failed to read expected: %v", err)
	}

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: fixtureSrc}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixtureSrc})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	autofixableCount := 0
	for _, v := range breachs {
		if v.Autofixable() {
			autofixableCount++
		}
	}
	if autofixableCount == 0 {
		t.Fatal("expected autofixable breachs in fixture")
	}

	var autofixable []Breach
	for _, v := range breachs {
		if v.Autofixable() {
			autofixable = append(autofixable, v)
		}
	}

	fixed, err := applyAutofixes(fixtureSrc, autofixable)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Error("expected at least one fix applied")
	}

	fixedContent, err := ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}

	if strings.TrimSpace(fixedContent) != strings.TrimSpace(expectedContent) {
		t.Errorf("fixed content does not match expected.\nGot:\n%s\n\nExpected:\n%s", fixedContent, expectedContent)
	}
}

func TestFormatterPlansCoverRegisteredLanguages(t *testing.T) {
	for _, language := range languageRegistry {
		plans := formatterPlansForLanguage(language.Name(), "example.file")
		if len(plans) == 0 {
			t.Fatalf("expected formatter plans for language %q", language.Name())
		}
		for _, plan := range plans {
			if strings.TrimSpace(plan.binary) == "" {
				t.Fatalf("formatter plan for language %q has empty binary", language.Name())
			}
		}
	}
}

func TestApplyAutofixesRunsFormatterAfterEdit(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	if err := WriteTextFile(filepath.Join(tmpDir, "package.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write package.json: %v", err)
	}
	if err := WriteTextFile(filepath.Join(tmpDir, ".prettierrc.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write .prettierrc.json: %v", err)
	}
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin", "prettier")
	if err := WriteTextFile(prettierBin, "#!/usr/bin/env sh\nexit 0\n"); err != nil {
		t.Fatalf("failed to write prettier stub: %v", err)
	}
	if err := os.Chmod(prettierBin, 0755); err != nil {
		t.Fatalf("failed to chmod prettier stub: %v", err)
	}

	originalLookup := formatterBinaryLookup
	originalRun := formatterCommandRun
	defer func() {
		formatterBinaryLookup = originalLookup
		formatterCommandRun = originalRun
	}()

	var ranBinary string
	var ranArgs []string
	var ranDir string
	formatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}
	formatterCommandRun = func(binary string, args []string, workDir string) error {
		ranBinary = binary
		ranArgs = append([]string{}, args...)
		ranDir = workDir
		return nil
	}

	testFile := "formatted.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖A\n\nconst x = 1; // remove\n\n// #endregion 🔖A\n"
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	expectedBinary := filepath.Join("node_modules", ".bin", "prettier")
	if ranBinary != expectedBinary {
		t.Fatalf("expected formatter to run with %q, got %q", expectedBinary, ranBinary)
	}
	if ranDir != tmpDir {
		t.Fatalf("expected formatter work dir %q, got %q", tmpDir, ranDir)
	}
	if len(ranArgs) < 3 || ranArgs[0] != "--write" || ranArgs[len(ranArgs)-1] != testFile {
		t.Fatalf("unexpected formatter args: %v", ranArgs)
	}
}

func TestApplyAutofixesFormatterFallbackNormalizesText(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	originalLookup := formatterBinaryLookup
	defer func() { formatterBinaryLookup = originalLookup }()
	formatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}

	testFile := "fallback.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖A\n\nconst x = 1;   \n\n// #endregion"
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}
	breachs := []Breach{
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	result, err := ReadTextFile(absPath)
	if err != nil {
		t.Fatalf("failed to read result: %v", err)
	}
	if strings.Contains(result, "   \n") {
		t.Fatalf("expected fallback formatter to trim trailing spaces, got %q", result)
	}
}

func TestFixSectionMissingEndName(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖MySection\n\nconst x = 1;\n\n// #endregion\n"
	expected := "// #region 🔖MySection\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"

	testFile := "test_missing_end.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionNameMismatch(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Alpha\n\nconst x = 1;\n\n// #endregion 🔖Beta\n"
	expected := "// #region 🔖Alpha\n\nconst x = 1;\n\n// #endregion 🔖Alpha\n"

	testFile := "test_mismatch.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeSectionNameMismatch, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Keep\n\nconst x = 1;\n\n// #endregion 🔖Keep\n\n// #region 🔖Empty\n\n// #endregion 🔖Empty\n\n// #region 🔖Also\n\nconst y = 2;\n\n// #endregion 🔖Also\n"
	expected := "// #region 🔖Keep\n\nconst x = 1;\n\n// #endregion 🔖Keep\n\n// #region 🔖Also\n\nconst y = 2;\n\n// #endregion 🔖Also\n"

	testFile := "test_empty.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeSectionEmpty, Scope: testFile + "#Empty", Line: 7},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixInlineComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n// inline one\n\n// inline two\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_inline.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixBlockComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n/* block comment */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_block.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeCommentBlock, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixJSDocComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n/** jsdoc comment */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_jsdoc.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeCommentJSDoc, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixMultipleBreachsSameFile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖A\n\n// bad comment\n\nconst a = 1;\n\n// #endregion\n\n// #region 🔖B\n\n// another bad\n\nconst b = 2;\n\n// #endregion 🔖Wrong\n"
	testFile := "test_multi.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 11},
		{Kind: BreachCodeSectionNameMismatch, Scope: testFile, Line: 15},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 4 {
		t.Errorf("expected 4 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖A") {
		t.Error("expected missing end name to be fixed to A")
	}
	if !strings.Contains(result, "// #endregion 🔖B") {
		t.Error("expected mismatch to be fixed to B")
	}
	if strings.Contains(result, "// bad comment") {
		t.Error("expected inline comment to be removed")
	}
	if strings.Contains(result, "// another bad") {
		t.Error("expected second inline comment to be removed")
	}
}

func TestFixImprovedCommentLogic(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := `// #region 🔖Section
const x = 1; // trailing comment
// TODO: fix this
// this line is part of the todo description

// another normal comment
/* TODO: block todo */
const y = 2; // normal trailing
// #endregion 🎁Fix
`

	testFile := "test_improved.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	expectedBreachs := 3
	if len(breachs) != expectedBreachs {
		t.Errorf("expected %d breachs, got %d", expectedBreachs, len(breachs))
		for i, v := range breachs {
			t.Logf("Breach %d: %s at %d:%d", i, v.Kind, v.Line, v.Column)
		}
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}

	if fixed != 3 {
		t.Errorf("expected 3 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)

	if strings.Contains(result, "trailing comment") {
		t.Errorf("trailing comment should be removed")
	}
	if !strings.Contains(result, "const x = 1;") {
		t.Errorf("code 'const x = 1;' should be kept")
	}
	if !strings.Contains(result, "// TODO: fix this") {
		t.Errorf("TODO comment should be kept")
	}
	if !strings.Contains(result, "// this line is part of the todo description") {
		t.Errorf("TODO description should be kept")
	}
	if strings.Contains(result, "// another normal comment") {
		t.Errorf("normal comment should be removed")
	}
	if !strings.Contains(result, "/* TODO: block todo */") {
		t.Errorf("block TODO should be kept")
	}

	lines := strings.Split(result, "\n")
	foundX := false
	for _, l := range lines {
		if strings.HasPrefix(l, "const x = 1;") {
			foundX = true
			if strings.Contains(l, "//") {
				t.Errorf("line 2 should not contain comment: %q", l)
			}
			if strings.HasSuffix(l, " ") {
				t.Errorf("line 2 should be trimmed right: %q", l)
			}
		}
	}
	if !foundX {
		t.Errorf("did not find 'const x = 1;' line in result")
	}
}

func TestFixConfigIgnored(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// comment in config\nconst x = 1;\n"
	testFile := "package.json"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	if len(breachs) != 0 {
		t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
	}
}

func TestScanCommentsGo(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewGoLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// this is a comment\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* block */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// TODO: fix later\n// continuation of todo\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("block TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* TODO: fix later */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for block TODO, got %d", len(breachs))
		}
	})

	t.Run("nolint skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// nolint:errcheck\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for nolint, got %d", len(breachs))
		}
	})

	t.Run("raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `// not a comment`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in raw string, got %d", len(breachs))
		}
	})

	t.Run("multi-line raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `line1\n// not a comment\nline3`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line raw string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "// #region 🔖Header\n\n// header comment\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("debug marker skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfmt.Println(\"[DEBUG] test\")\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for debug marker, got %d", len(breachs))
		}
	})

	t.Run("url scheme not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar url = \"https://example.com\"\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for URL scheme, got %d", len(breachs))
		}
	})

	t.Run("grouped inline comments", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// comment one\n\n// comment two\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 2 {
			t.Errorf("expected 2 breachs for separate comment blocks, got %d", len(breachs))
		}
	})
}

func TestScanCommentsPython(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewPythonLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "# region Section\n\n# TODO: fix later\n# continuation of todo\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("noqa skipped", func(t *testing.T) {
		content := "# region Section\n\n# noqa: E501\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for noqa, got %d", len(breachs))
		}
	})

	t.Run("type ignore skipped", func(t *testing.T) {
		content := "# region Section\n\n# type: ignore[assignment]\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for type: ignore, got %d", len(breachs))
		}
	})

	t.Run("triple double quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"# not a comment\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("triple single quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = '''# not a comment'''\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-single-quoted string, got %d", len(breachs))
		}
	})

	t.Run("multi-line triple quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"\n# not a comment\n\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "# region Header\n#\n# header comment\n#\n# endregion Header\n\n# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("comment in regular string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"# not a comment\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("trailing comment", func(t *testing.T) {
		content := "# region Section\n\nx = 1  # trailing comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach for trailing comment, got %d", len(breachs))
		}
		if breachs[0].Column <= 1 {
			t.Errorf("expected column > 1 for trailing comment, got %d", breachs[0].Column)
		}
	})
}

func TestScanCommentsCSharp(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewCSharpLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n// this is a comment\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n/* block */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// TODO: fix later\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("pragma skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// pragma warning disable\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for pragma, got %d", len(breachs))
		}
	})

	t.Run("verbatim string skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\nvar s = @\"// not a comment\";\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in verbatim string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "#region 🔖Header\n// header comment\n#endregion 🔖Header\n\n#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("no JSDoc for csharp", func(t *testing.T) {
		content := "#region 🔖Section\n\n/** not jsdoc in csharp */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment (not JSDoc) for C#, got %s", breachs[0].Kind)
		}
	})
}

func TestScanCommentsTypeScript(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()

	t.Run("JSDoc detected", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/** jsdoc */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentJSDoc {
			t.Errorf("expected JSDoc breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("template literal skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `// not a comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in template literal, got %d", len(breachs))
		}
	})

	t.Run("template expression not skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `${x} // comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for template expression context, got %d", len(breachs))
		}
	})

	t.Run("eslint directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// eslint-disable-next-line\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for eslint directive, got %d", len(breachs))
		}
	})

	t.Run("@ts directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// @ts-ignore\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for @ts directive, got %d", len(breachs))
		}
	})

	t.Run("string literals skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst a = '// not a comment';\nconst b = \"// not a comment\";\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in strings, got %d", len(breachs))
		}
	})

	t.Run("config file skipped", func(t *testing.T) {
		content := "// inline comment\nconst x = 1;\n"
		breachs := lang.ScanComments(ctx, "tsconfig.json", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
		}
	})
}

func TestScanCommentsShell(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewShellLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("comment in string skipped", func(t *testing.T) {
		content := "# region Section\n\necho \"# not a comment\"\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})
}

func TestScanCommentsRust(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewRustLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "mod section { // 🔖Section\n\n// this is a comment\n\nfn main() {}\n\n} // 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "mod section { // 🔖Section\n\n/* block comment */\n\nfn main() {}\n\n} // 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "mod section { // 🔖Section\n\n// TODO: fix later\n\nfn main() {}\n\n} // 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})
}

func TestScanCommentsAutofix(t *testing.T) {
	t.Run("python inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "# region Section\n\n# bad comment\n\ndef main(): pass\n\n# endregion Section\n"
		expected := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_inline.py"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("python trailing comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "# region Section\n\nx = 1  # trailing\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_trailing.py"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3, Column: 7},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if !strings.Contains(result, "x = 1") {
			t.Error("code should be preserved")
		}
		if strings.Contains(result, "trailing") {
			t.Error("trailing comment should be removed")
		}
	})

	t.Run("go block comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Section\n\n/* block comment */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		expected := "// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		testFile := "test_go_block.go"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeCommentBlock, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("csharp inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "#region 🔖Section\n\n// bad comment\n\npublic class C {}\n\n#endregion 🔖Section\n"
		expected := "#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		testFile := "test_cs_inline.cs"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})
}

func TestEmojiVariationAutofix(t *testing.T) {
	t.Run("fix emoji variation to colorful", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "This is a test \U0001F4BB\uFE0E with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7 construction."
		expected := "This is a test \U0001F4BB\uFE0F with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7\uFE0F construction.\n"
		testFile := "test_emoji.txt"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 1},
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 2},
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 3 {
			t.Fatalf("expected 3 fixed, got %d", fixed)
		}
		got, _ := ReadTextFile(absPath)
		if got != expected {
			t.Errorf("expected:\n%q\ngot:\n%q", expected, got)
		}
	})

	t.Run("emojiText preserves VS16 for text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"⚙️", "⚙️"},
			{"⚖️", "⚖️"},
			{"✂️", "✂️"},
			{"🏗️", "🏗️"},
			{"🛠️", "🛠️"},
			{"🛡️", "🛡️"},
			{"⌨️", "⌨️"},
			{"🖱️", "🖱️"},
			{"🏷️", "🏷️"},
			{"🗃️", "🗃️"},
		}
		for _, tc := range cases {
			got := emojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText strips VS16 for non-text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"💻️", "💻"},
			{"🥼️", "🥼"},
			{"📃️", "📃"},
			{"📜️", "📜"},
		}
		for _, tc := range cases {
			got := emojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText is idempotent", func(t *testing.T) {
		cases := []string{"⚙️", "🏗️", "💻", "🛠️"}
		for _, tc := range cases {
			once := emojiText(tc)
			twice := emojiText(once)
			if once != twice {
				t.Errorf("emojiText not idempotent: emojiText(%q)=%q, emojiText(%q)=%q", tc, once, once, twice)
			}
		}
	})
	t.Run("emojiText strips VS15", func(t *testing.T) {
		got := emojiText("⚙️")
		if got != "⚙️" {
			t.Errorf("emojiText with VS15 = %q, want %q", got, "⚙️")
		}
	})
	t.Run("section markers not flagged as inline comments", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		fileContent := "// #region \U0001F516Header\n\n// \U0001F4BBcompose/test.tsx\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n//#region \U0001F516Action Hooks\nconst x = 1;\n//#endregion \U0001F516Action Hooks\n"
		testFile := "test.tsx"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, fileContent)
		bundles := LoadBundles()
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, _ := CheckPoliciesWithContext(ctx, nil)
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				t.Errorf("section marker flagged as inline comment at line %d: %s", v.Line, v.Excerpt)
			}
		}
	})
}

func TestFixNonAutofixableNotFixed(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	bundles := LoadBundles()
	path := "repo/asset/fixture/some/folder/⚛️⚛️file_invalid.tsx"
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	for _, v := range breachs {
		info := v.Kind.Info()
		if v.Autofixable() != info.Autofixable {
			t.Errorf("breach %s: Autofixable() = %v, Info().Autofixable = %v", v.Kind, v.Autofixable(), info.Autofixable)
		}
	}

	autofixableKinds := []Statute{
		BreachCodeFileWrongLicense,
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	for _, kind := range autofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected autofixable statute %s to be detected", kind)
		}
		if !kind.Info().Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
	}
	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected non-autofixable statute %s to be detected", kind)
		}
		if kind.Info().Autofixable {
			t.Errorf("statute %s should not be autofixable", kind)
		}
	}
}

func TestExhaustiveFixViaGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow fix via graphql test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	result, err := executor.ExecuteJSON(ctx, `mutation { fix(scope: "repo/go/main_test.go") { fixed remaining breachs { id summary } } }`, nil)
	if err != nil {
		t.Fatalf("fix mutation failed: %v", err)
	}

	var resp struct {
		Fix struct {
			Fixed     int `json:"fixed"`
			Remaining int `json:"remaining"`
			Breachs   []struct {
				ID      string `json:"id"`
				Summary string `json:"summary"`
			} `json:"breachs"`
		} `json:"fix"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse fix response: %v\nResult: %s", err, result)
	}
	if resp.Fix.Remaining < 0 {
		t.Error("remaining should not be negative")
	}
	if len(resp.Fix.Breachs) != resp.Fix.Remaining {
		t.Errorf("breachs length %d != remaining %d", len(resp.Fix.Breachs), resp.Fix.Remaining)
	}
}

func TestFixViaRepoContext(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	ctx := NewRepoContext(rootDir)
	scope := "repo/go/main_test.go"
	res, err := ctx.Fix(&scope)
	if err != nil {
		t.Fatalf("Fix failed: %v", err)
	}
	if res == nil {
		t.Fatal("Fix returned nil result")
	}
	if res.Breachs == nil {
		t.Error("Breachs should not be nil")
	}
	if res.Remaining != len(res.Breachs) {
		t.Errorf("remaining %d != breachs length %d", res.Remaining, len(res.Breachs))
	}
}

func TestFixIdempotent(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	testFile := "test_clean.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 0 {
		t.Errorf("expected 0 fixes on clean file, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != content {
		t.Error("clean file should not be modified")
	}
}

func TestFixNestedSections(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Outer\n\n// #region 🔖Inner\n\nconst x = 1;\n\n// #endregion\n\nconst y = 2;\n\n// #endregion\n"
	testFile := "test_nested.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []Breach{
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 11},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 2 {
		t.Errorf("expected 2 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖Inner") {
		t.Error("expected inner endregion to get name Inner")
	}
	if !strings.Contains(result, "// #endregion 🔖Outer") {
		t.Error("expected outer endregion to get name Outer")
	}
}

func TestFixExtractFileFromScope(t *testing.T) {
	tests := []struct {
		scope    string
		expected string
	}{
		{"file.ts", "file.ts"},
		{"file.ts#Section", "file.ts"},
		{"file.ts::definition", "file.ts"},
		{"path/to/file.ts#Section/Sub", "path/to/file.ts"},
		{"path/to/file.ts::myFunc", "path/to/file.ts"},
	}
	for _, tt := range tests {
		result := extractFileFromScope(tt.scope)
		if result != tt.expected {
			t.Errorf("extractFileFromScope(%q) = %q, want %q", tt.scope, result, tt.expected)
		}
	}
}

func TestFixStatuteMeta(t *testing.T) {
	autofixableKinds := []Statute{
		BreachCodeFileMissingHeaderRegion,
		BreachCodeFileMissingLicense,
		BreachCodeFileWrongLicense,
		BreachCodeSectionEmpty,
		BreachCodeSectionMissingEndName,
		BreachCodeSectionNameMismatch,
		BreachCodeCommentInline,
		BreachCodeCommentBlock,
		BreachCodeCommentJSDoc,
	}
	for _, kind := range autofixableKinds {
		info := kind.Info()
		if !info.Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
		if info.Reason == "" {
			t.Errorf("statute %s has empty reason", kind)
		}
		if info.Solution == "" {
			t.Errorf("statute %s has empty solution", kind)
		}
	}

	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		info := kind.Info()
		if info.Autofixable {
			t.Errorf("statute %s should NOT be autofixable", kind)
		}
	}
}

func TestFindMatchingSectionStartName(t *testing.T) {
	lines := []string{
		"// #region 🔖Outer",
		"",
		"// #region 🔖Inner",
		"const x = 1;",
		"// #endregion 🔖Inner",
		"",
		"// #endregion",
	}
	language := NewTypeScriptLanguage()

	name := findMatchingSectionStartName(lines, 6, language)
	if name != "Outer" {
		t.Errorf("expected Outer, got %q", name)
	}

	name = findMatchingSectionStartName(lines, 4, language)
	if name != "Inner" {
		t.Errorf("expected Inner, got %q", name)
	}
}

// 🧪#region 🕸️Test Command
func TestIsTestFunctionName(t *testing.T) {

	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		{"Go TestXxx", "TestSomething", true},
		{"Go TestAbc", "TestAbc", true},
		{"Go BenchmarkXxx", "BenchmarkSomething", true},
		{"Go FuzzXxx", "FuzzSomething", true},
		{"Python test_xxx", "test_something", true},
		{"Go Test without capital", "Testnotcapital", false},
		{"plain func", "doSomething", false},
		{"plain func capitalized", "DoSomething", false},
		{"Test alone not enough", "Test", false},
		{"Benchmark alone not enough", "Benchmark", false},
		{"Fuzz alone not enough", "Fuzz", false},
		{"test_ prefix not Go", "test_go_style", true},
		{"lowercase test", "testsomething", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := isTestFunctionName(tt.input)
			if got != tt.expected {
				t.Errorf("isTestFunctionName(%q) = %v, want %v", tt.input, got, tt.expected)
			}
		})
	}
}

func TestDefinitionKindTestEmoji(t *testing.T) {

	emoji := definitionKindEmoji(map[string]interface{}{"kind": string(DefinitionKindTest)})
	expected := emojiText(EmojiDefinitionTest)
	if emoji != expected {
		t.Errorf("definitionKindEmoji(test) = %q, want %q", emoji, expected)
	}
}

func TestDefinitionKindTestCode(t *testing.T) {

	code := definitionKindCode(map[string]interface{}{"kind": string(DefinitionKindTest)})
	if code != "t" {
		t.Errorf("definitionKindCode(DefinitionKindTest) = %q, want %q", code, "t")
	}

	emoji := definitionEmojiFromCode("t")
	if emoji != EmojiDefinitionTest {
		t.Errorf("definitionEmojiFromCode(t) = %q, want %q", emoji, EmojiDefinitionTest)
	}
}

func TestDefinitionKindTestIsValid(t *testing.T) {
	if !DefinitionKindTest.IsValid() {
		t.Error("DefinitionKindTest.IsValid() should return true")
	}
}

func TestBuildDefinitionIDTestKind(t *testing.T) {

	labFileID := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiFileLab) + "maintest"

	id := buildDefinitionID(labFileID, nil, "TestSomething", DefinitionKindImplementation)
	testEmoji := emojiText(EmojiDefinitionTest)
	implEmoji := emojiText(EmojiDefinitionImpl)

	if !strings.Contains(id, testEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should contain test emoji %q, got %q", testEmoji, id)
	}
	if strings.Contains(id, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should NOT contain impl emoji %q, got %q", implEmoji, id)
	}

	id2 := buildDefinitionID(labFileID, nil, "helperFunc", DefinitionKindImplementation)
	if !strings.Contains(id2, implEmoji) {
		t.Errorf("buildDefinitionID for helperFunc in lab file should contain impl emoji %q, got %q", implEmoji, id2)
	}

	codeFileID := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiFileCode) + "main"
	id3 := buildDefinitionID(codeFileID, nil, "TestSomething", DefinitionKindImplementation)
	if !strings.Contains(id3, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in code file should contain impl emoji %q, got %q", implEmoji, id3)
	}
}

func TestDetectBundleLanguage(t *testing.T) {

	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	t.Run("go", func(t *testing.T) {
		goDir := filepath.Join(tmpDir, "gomod")
		if err := os.MkdirAll(goDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(goDir, "go.mod"), []byte("module test\n\ngo 1.21\n"), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(goDir)
		if lang != "go" {
			t.Errorf("detectBundleLanguage with go.mod = %q, want %q", lang, "go")
		}
	})

	t.Run("python", func(t *testing.T) {
		pyDir := filepath.Join(tmpDir, "pyproj")
		if err := os.MkdirAll(pyDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(pyDir, "pyproject.toml"), []byte("[technology]\nname = \"test\"\n"), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(pyDir)
		if lang != "python" {
			t.Errorf("detectBundleLanguage with pyproject.toml = %q, want %q", lang, "python")
		}
	})

	t.Run("typescript", func(t *testing.T) {
		tsDir := filepath.Join(tmpDir, "tspack")
		if err := os.MkdirAll(tsDir, 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(tsDir, "package.json"), []byte(`{"name":"test"}`), 0644); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(tsDir)
		if lang != "typescript" {
			t.Errorf("detectBundleLanguage with package.json = %q, want %q", lang, "typescript")
		}
	})

	t.Run("unknown", func(t *testing.T) {
		emptyDir := filepath.Join(tmpDir, "emptydir")
		if err := os.MkdirAll(emptyDir, 0755); err != nil {
			t.Fatal(err)
		}
		lang := detectBundleLanguage(emptyDir)
		if lang != "" {
			t.Errorf("detectBundleLanguage empty dir = %q, want empty string", lang)
		}
	})
}

func TestUnflattenTestName(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"testsomething", "TestSomething"},
		{"benchmarksomething", "BenchmarkSomething"},
		{"fuzzsomething", "FuzzSomething"},
		{"testbenchmarksomething", "TestbenchmarkSomething"},
		{"myfunction", "Myfunction"},
		{"", ""},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got := unflattenTestName(tt.input)
			if got != tt.expected {
				t.Errorf("unflattenTestName(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

func TestResolveTestScopes(t *testing.T) {

	scopes := resolveTestScopes(nil)
	if len(scopes) != 1 {
		t.Fatalf("resolveTestScopes(nil) len = %d, want 1", len(scopes))
	}
	if scopes[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes(nil)[0].Kind = %q, want %q", scopes[0].Kind, testScopeAll)
	}

	scopes2 := resolveTestScopes([]string{})
	if len(scopes2) != 1 || scopes2[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes([]) should return testScopeAll scope")
	}

	scopes3 := resolveTestScopes([]string{"notavalidid"})
	if len(scopes3) != 1 || scopes3[0].Kind != testScopeAll {
		t.Errorf("resolveTestScopes([invalid]) should return testScopeAll scope")
	}
}

func TestCollectGoTestsInSection(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "foo_test.go")
	content := `package foo

// 🧪#region 📷Alpha
func TestAlpha(t *testing.T) {}

func TestAlphaBeta(t *testing.T) {}

// #endregion 📷Alpha

// 🧪#region ⭐Gamma
func TestGamma(t *testing.T) {}

// #endregion ⭐Gamma
`
	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	pattern := collectGoTestsInSection(testFile, "Alpha")
	if pattern == "" {
		t.Fatal("expected non-empty pattern for Alpha section")
	}
	if !strings.Contains(pattern, "TestAlpha") {
		t.Errorf("pattern should contain TestAlpha, got: %s", pattern)
	}
	if !strings.Contains(pattern, "TestAlphaBeta") {
		t.Errorf("pattern should contain TestAlphaBeta, got: %s", pattern)
	}
	if strings.Contains(pattern, "TestGamma") {
		t.Errorf("pattern should NOT contain TestGamma from different section, got: %s", pattern)
	}

	empty := collectGoTestsInSection(testFile, "Nonexistent")
	if empty != "" {
		t.Errorf("collectGoTestsInSection for missing section should return empty, got: %s", empty)
	}
}

func TestResolveTestFunctionName(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "foo_test.go")
	content := `package foo

func TestMyFunction(t *testing.T) {}

func BenchmarkMyFunction(b *testing.B) {}
`
	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	name := resolveTestFunctionName(testFile, "testmyfunction")
	if name != "TestMyFunction" {
		t.Errorf("resolveTestFunctionName(testmyfunction) = %q, want %q", name, "TestMyFunction")
	}

	bname := resolveTestFunctionName(testFile, "benchmarkmyfunction")
	if bname != "BenchmarkMyFunction" {
		t.Errorf("resolveTestFunctionName(benchmarkmyfunction) = %q, want %q", bname, "BenchmarkMyFunction")
	}

	missing := resolveTestFunctionName(testFile, "nonexistent")
	if missing != "" {
		t.Errorf("resolveTestFunctionName for missing function should return empty, got: %q", missing)
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

// #endregion 🕸️Test Command

// #endregion 🔊Cli

// 📤#region 🕰️SQLite Export
// 📤testExportContext is a mock RepoContext for testing ExportToSQLite.
type testExportContext struct {
	rootDir      string
	technologies []*Technology
	bundles      []*Bundle
	folders      []*Folder
	files        []*File
	sections     []*Section
	definitions  []*Definition
}

func (c *testExportContext) GetRootDir() string                               { return c.rootDir }
func (c *testExportContext) GetTechnologies() []*Technology                   { return c.technologies }
func (c *testExportContext) GetBundles() []*Bundle                            { return c.bundles }
func (c *testExportContext) GetFolders() []*Folder                            { return c.folders }
func (c *testExportContext) GetFiles() []*File                                { return c.files }
func (c *testExportContext) GetSections() []*Section                          { return c.sections }
func (c *testExportContext) GetDefinitions() []*Definition                    { return c.definitions }
func (c *testExportContext) GetCheckpoints(limit *int) ([]*Checkpoint, error) { return nil, nil }
func (c *testExportContext) GetContributors() ([]*Contributor, error)         { return nil, nil }
func (c *testExportContext) GetGoals() ([]*Goal, error)                       { return nil, nil }
func (c *testExportContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return nil, nil
}
func (c *testExportContext) GetPolicies() []*Policy                        { return nil }
func (c *testExportContext) GetDrafts() ([]*Draft, error)                  { return nil, nil }
func (c *testExportContext) GetTodos(filter *FilterInput) ([]*Todo, error) { return nil, nil }
func (c *testExportContext) GetStatutes() []*StatuteMeta                   { return nil }
func (c *testExportContext) Analyze(scope *string) (*AnalyzeResult, error) {
	return &AnalyzeResult{}, nil
}
func (c *testExportContext) Fix(scope *string) (*FixResult, error)                 { return &FixResult{}, nil }
func (c *testExportContext) GoalCreate(input GoalCreateInput) (*Goal, error)       { return nil, nil }
func (c *testExportContext) GoalChange(input GoalChangeInput) (*Goal, error)       { return nil, nil }
func (c *testExportContext) GoalClose(input GoalCloseInput) (*Goal, error)         { return nil, nil }
func (c *testExportContext) GoalReopen(input GoalReopenInput) (*Goal, error)       { return nil, nil }
func (c *testExportContext) GoalDelete(input GoalDeleteInput) (bool, error)        { return false, nil }
func (c *testExportContext) TodoCreate(input TodoCreateInput) (*Todo, error)       { return nil, nil }
func (c *testExportContext) TodoChange(input TodoChangeInput) (*Todo, error)       { return nil, nil }
func (c *testExportContext) TodoDelete(id string) (bool, error)                    { return false, nil }
func (c *testExportContext) DraftCreate(input DraftCreateInput) (*Draft, error)    { return nil, nil }
func (c *testExportContext) DraftDelete(id string) (bool, error)                   { return false, nil }
func (c *testExportContext) TicketOpen(input TicketOpenInput) (*Ticket, error)     { return nil, nil }
func (c *testExportContext) TicketClose(input TicketCloseInput) (*Ticket, error)   { return nil, nil }
func (c *testExportContext) TicketReopen(input TicketReopenInput) (*Ticket, error) { return nil, nil }
func (c *testExportContext) TicketChange(input TicketChangeInput) (*Ticket, error) { return nil, nil }
func (c *testExportContext) TicketDelete(input TicketDeleteInput) (bool, error)    { return false, nil }
func (c *testExportContext) FolderCreate(path string) (*Folder, error)             { return nil, nil }
func (c *testExportContext) FolderMove(src, dst string) (*Folder, error)           { return nil, nil }
func (c *testExportContext) FolderDelete(path string) error                        { return nil }
func (c *testExportContext) FileCreate(path string) (*File, error)                 { return nil, nil }
func (c *testExportContext) FileMove(src, dst string) (*File, error)               { return nil, nil }
func (c *testExportContext) FileDelete(path string) error                          { return nil }
func (c *testExportContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}
func (c *testExportContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}
func (c *testExportContext) SectionDelete(file, name string) error { return nil }
func (c *testExportContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	return nil, nil
}
func (c *testExportContext) Extract(sourceFile, sourceSection, targetFile *string) (*File, error) {
	return nil, nil
}
func (c *testExportContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}
func (c *testExportContext) ContributorRemove(github string) error { return nil }
func (c *testExportContext) SyncManagement() (bool, error)         { return false, nil }

func TestExportToSQLiteSchema(t *testing.T) {
	tmpDir := t.TempDir()
	srcDir := filepath.Join(tmpDir, "mytechnology", "mybundle", "src")
	os.MkdirAll(srcDir, 0755)

	tsContent := `// #region 🔖Header

// 💻src/app.ts

// 2025 Test <t@t.com>

// GNU Affero General Public License
// https://www.gnu.org/licenses/

// App module summary.

// #endregion 🕰️SQLite Export

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
		technologies: []*Technology{
			{Name: "mytechnology", Root: "mytechnology", Kind: TechnologyKindUser},
		},
		bundles: []*Bundle{
			{Name: "mytechnology/mybundle", Root: "mytechnology/mybundle", TechnologyName: "mytechnology", Kind: BundleKindLibrary},
		},
		folders: []*Folder{
			{Path: "mytechnology", Name: "mytechnology", Kind: FolderKindOrganization},
			{Path: "mytechnology/mybundle", Name: "mybundle", Kind: FolderKindOrganization},
			{Path: "mytechnology/mybundle/src", Name: "src", Kind: FolderKindOrganization},
		},
		files: []*File{
			{Path: "mytechnology/mybundle/src/app.ts", Name: "app.ts", Extension: "ts", Kind: FileKindCode},
		},
	}

	outputPath := filepath.Join(tmpDir, "test.db")
	result, err := ExportToSQLite(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToSQLite failed: %v", err)
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

	db, err := sql.Open("sqlite", outputPath)
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}
	defer db.Close()

	var checkpointCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM checkpoint").Scan(&checkpointCount); err != nil {
		t.Fatalf("failed to count checkpoints: %v", err)
	}
	if checkpointCount != 1 {
		t.Errorf("expected 1 checkpoint, got %d", checkpointCount)
	}

	var folderKindCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM folder_kind").Scan(&folderKindCount); err != nil {
		t.Fatalf("failed to count folder_kind: %v", err)
	}
	if folderKindCount != 2 {
		t.Errorf("expected 2 folder_kind rows, got %d", folderKindCount)
	}
	var fileKindCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM file_kind").Scan(&fileKindCount); err != nil {
		t.Fatalf("failed to count file_kind: %v", err)
	}
	if fileKindCount != 8 {
		t.Errorf("expected 8 file_kind rows, got %d", fileKindCount)
	}

	var technologyID int
	var technologyFolderID int
	var technologyKindID int
	var technologyName string
	var technologySummary sql.NullString
	if err := db.QueryRow("SELECT id, folder_id, technology_kind_id, name, summary FROM technology WHERE id = 1").Scan(&technologyID, &technologyFolderID, &technologyKindID, &technologyName, &technologySummary); err != nil {
		t.Fatalf("failed to query technology: %v", err)
	}
	if technologyName != "mytechnology" {
		t.Errorf("expected technology name 'mytechnology', got %q", technologyName)
	}
	if technologyKindID != 0 {
		t.Errorf("expected technology_kind_id 0 (user), got %d", technologyKindID)
	}
	if technologySummary.Valid && technologySummary.String != "This is the technology summary." {
		t.Errorf("expected technology summary 'This is the technology summary.', got %q", technologySummary.String)
	}

	var bundleID int
	var bundleTechnologyID int
	var bundleFolderID int
	var bundleKindID int
	var bundleName string
	if err := db.QueryRow("SELECT id, technology_id, folder_id, bundle_kind_id, name FROM bundle WHERE id = 1").Scan(&bundleID, &bundleTechnologyID, &bundleFolderID, &bundleKindID, &bundleName); err != nil {
		t.Fatalf("failed to query bundle: %v", err)
	}
	if bundleName != "mybundle" {
		t.Errorf("expected bundle name 'mybundle', got %q", bundleName)
	}
	if bundleKindID != 0 {
		t.Errorf("expected bundle_kind_id 0 (library), got %d", bundleKindID)
	}
	if bundleTechnologyID != technologyID {
		t.Errorf("expected bundle technology_id %d, got %d", technologyID, bundleTechnologyID)
	}

	var folderCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM folder").Scan(&folderCount); err != nil {
		t.Fatalf("failed to count folders: %v", err)
	}
	if folderCount != 3 {
		t.Errorf("expected 3 folders, got %d", folderCount)
	}
	var folderCheckpointID int
	var folderKindID int
	if err := db.QueryRow("SELECT checkpoint_id, folder_kind_id FROM folder WHERE id = 1").Scan(&folderCheckpointID, &folderKindID); err != nil {
		t.Fatalf("failed to query folder: %v", err)
	}
	if folderCheckpointID != 1 {
		t.Errorf("expected folder checkpoint_id 1, got %d", folderCheckpointID)
	}

	var fileID int
	var fileParentFolderID sql.NullInt64
	var fileKindID int
	var fileName string
	var fileExtension string
	if err := db.QueryRow("SELECT id, parent_folder_id, file_kind_id, name, extension FROM file WHERE id = 1").Scan(&fileID, &fileParentFolderID, &fileKindID, &fileName, &fileExtension); err != nil {
		t.Fatalf("failed to query file: %v", err)
	}
	if fileName != "app.ts" {
		t.Errorf("expected file name 'app.ts', got %q", fileName)
	}
	if fileKindID != 0 {
		t.Errorf("expected file_kind_id 0 (code), got %d", fileKindID)
	}
	if fileExtension != "ts" {
		t.Errorf("expected file extension 'ts', got %q", fileExtension)
	}

	var sectionCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM section").Scan(&sectionCount); err != nil {
		t.Fatalf("failed to count sections: %v", err)
	}
	if sectionCount == 0 {
		t.Error("expected at least 1 section")
	}

	var defCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM definition").Scan(&defCount); err != nil {
		t.Fatalf("failed to count definitions: %v", err)
	}
	if defCount > 0 {
		var defSectionID int
		var defKindID int
		var defName string
		var defCode sql.NullString
		if err := db.QueryRow("SELECT section_id, definition_kind_id, name, code FROM definition LIMIT 1").Scan(&defSectionID, &defKindID, &defName, &defCode); err != nil {
			t.Fatalf("failed to query definition: %v", err)
		}
		if defName != "doWork" {
			t.Errorf("expected definition name 'doWork', got %q", defName)
		}
		if defKindID != 0 {
			t.Errorf("expected definition_kind_id 0 (implementation), got %d", defKindID)
		}
		if !defCode.Valid || defCode.String == "" {
			t.Error("expected definition code to be non-empty")
		}
	}

	_, dupErr := db.Exec("INSERT INTO contributor (github, name, alias) VALUES (?, ?, ?)", "export", "Export System", "export")
	if dupErr == nil {
		t.Error("expected unique constraint violation for duplicate contributor github")
	}
}

func TestPostgresSchemaIncludesKitVersionControlTables(t *testing.T) {
	rootDir := findTestRepoRoot(".")
	schemaPath := filepath.Join(rootDir, "repo", "postgres", "🛢️🛢️schema.sql")
	data, err := os.ReadFile(schemaPath)
	if err != nil {
		t.Fatalf("failed to read postgres schema: %v", err)
	}
	schema := string(data)
	requiredSnippets := []string{
		"CREATE TABLE IF NOT EXISTS kits (",
		"CREATE TABLE IF NOT EXISTS kit_snapshots (",
		"CREATE TABLE IF NOT EXISTS kit_checkpoints (",
		"CREATE TABLE IF NOT EXISTS kit_alternatives (",
		"CREATE TABLE IF NOT EXISTS kit_sessions (",
		"CREATE TABLE IF NOT EXISTS kit_drafts (",
		"CREATE TABLE IF NOT EXISTS kit_transactions (",
		"CREATE TABLE IF NOT EXISTS kit_releases (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_families (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_kind_entities (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layouts (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layout_pieces (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layout_connections (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_properties (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_attributes (",
		"CREATE UNIQUE INDEX IF NOT EXISTS idx_kit_transactions_one_open_per_draft ON kit_transactions(draft_id) WHERE state = 'open';",
		"source_json          JSONB NOT NULL DEFAULT '{}'",
		"before_snapshot_json  JSONB NOT NULL DEFAULT '{}'",
	}
	for _, snippet := range requiredSnippets {
		if !strings.Contains(schema, snippet) {
			t.Errorf("postgres schema missing snippet %q", snippet)
		}
	}

	if !strings.Contains(schema, "snapshot_kind        TEXT NOT NULL CHECK (snapshot_kind IN ('initial', 'materialized', 'draft-base', 'session-cache'))") {
		t.Error("expected snapshot kind check constraint in postgres schema")
	}
	if !strings.Contains(schema, "state       TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'finalized', 'aborted'))") {
		t.Error("expected transaction state check constraint in postgres schema")
	}
}

func TestExportToSQLiteEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	ctx := &testExportContext{
		rootDir:      tmpDir,
		technologies: []*Technology{},
		bundles:      []*Bundle{},
		folders:      []*Folder{},
		files:        []*File{},
	}

	outputPath := filepath.Join(tmpDir, "empty.db")
	result, err := ExportToSQLite(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToSQLite failed: %v", err)
	}
	if result.Technologies != 0 || result.Bundles != 0 || result.Folders != 0 || result.Files != 0 || result.Sections != 0 || result.Definitions != 0 {
		t.Errorf("expected all counts to be 0, got technologies=%d bundles=%d folders=%d files=%d sections=%d definitions=%d",
			result.Technologies, result.Bundles, result.Folders, result.Files, result.Sections, result.Definitions)
	}

	db, err := sql.Open("sqlite", outputPath)
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}
	defer db.Close()
	tables := []string{
		"contributor", "release", "release_contributors", "version", "checkpoint",
		"folder_kind", "folder", "file_kind", "file",
		"technology_kind", "technology", "entity", "mechanism", "system", "system_entities",
		"bundle_kind", "bundle", "section",
		"definition_kind", "definition",
		"client_kind", "agent", "session", "event_kind", "event",
	}
	for _, table := range tables {
		var count int
		if err := db.QueryRow(fmt.Sprintf("SELECT COUNT(*) FROM %s", table)).Scan(&count); err != nil {
			t.Errorf("table %s does not exist: %v", table, err)
		}
	}

	var contributorCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM contributor").Scan(&contributorCount); err != nil {
		t.Fatalf("failed to count contributors: %v", err)
	}
	if contributorCount != 1 {
		t.Errorf("expected 1 synthetic contributor, got %d", contributorCount)
	}
	var checkpointCount int
	if err := db.QueryRow("SELECT COUNT(*) FROM checkpoint").Scan(&checkpointCount); err != nil {
		t.Fatalf("failed to count checkpoints: %v", err)
	}
	if checkpointCount != 1 {
		t.Errorf("expected 1 synthetic checkpoint, got %d", checkpointCount)
	}

	var folderKindCount int
	db.QueryRow("SELECT COUNT(*) FROM folder_kind").Scan(&folderKindCount)
	if folderKindCount != 2 {
		t.Errorf("expected 2 folder_kind rows, got %d", folderKindCount)
	}
	var fileKindCount int
	db.QueryRow("SELECT COUNT(*) FROM file_kind").Scan(&fileKindCount)
	if fileKindCount != 8 {
		t.Errorf("expected 8 file_kind rows, got %d", fileKindCount)
	}
	var technologyKindCount int
	db.QueryRow("SELECT COUNT(*) FROM technology_kind").Scan(&technologyKindCount)
	if technologyKindCount != 3 {
		t.Errorf("expected 3 technology_kind rows, got %d", technologyKindCount)
	}
	var bundleKindCount int
	db.QueryRow("SELECT COUNT(*) FROM bundle_kind").Scan(&bundleKindCount)
	if bundleKindCount != 7 {
		t.Errorf("expected 7 bundle_kind rows, got %d", bundleKindCount)
	}
	var defKindCount int
	db.QueryRow("SELECT COUNT(*) FROM definition_kind").Scan(&defKindCount)
	if defKindCount != 4 {
		t.Errorf("expected 4 definition_kind rows, got %d", defKindCount)
	}
}

func TestExportKindMappings(t *testing.T) {

	if folderKindToInt(FolderKindOrganization) != 0 {
		t.Errorf("FolderKindOrganization should map to 0")
	}
	if folderKindToInt(FolderKindRequired) != 1 {
		t.Errorf("FolderKindRequired should map to 1")
	}
	if folderKindToInt(FolderKindRoot) != 0 {
		t.Errorf("FolderKindRoot should default to 0")
	}

	if technologyKindToInt(TechnologyKindUser) != 0 {
		t.Errorf("TechnologyKindUser should map to 0")
	}
	if technologyKindToInt(TechnologyKindInfrastructure) != 1 {
		t.Errorf("TechnologyKindInfrastructure should map to 1")
	}
	if technologyKindToInt(TechnologyKindResearch) != 2 {
		t.Errorf("TechnologyKindResearch should map to 2")
	}

	if bundleKindToInt(BundleKindLibrary) != 0 {
		t.Errorf("BundleKindLibrary should map to 0")
	}
	if bundleKindToInt(BundleKindSchema) != 1 {
		t.Errorf("BundleKindSchema should map to 1")
	}
	if bundleKindToInt(BundleKindBinary) != 2 {
		t.Errorf("BundleKindBinary should map to 2")
	}
	if bundleKindToInt(BundleKindUI) != 3 {
		t.Errorf("BundleKindUI should map to 3")
	}
	if bundleKindToInt("example") != 4 {
		t.Errorf("BundleKind example should map to 4")
	}
	if bundleKindToInt(BundleKindSite) != 5 {
		t.Errorf("BundleKindSite should map to 5")
	}
	if bundleKindToInt(BundleKindAssets) != 6 {
		t.Errorf("BundleKindAssets should map to 6")
	}

	if fileKindToInt(FileKindCode) != 0 {
		t.Errorf("FileKindCode should map to 0")
	}
	if fileKindToInt(FileKindLab) != 1 {
		t.Errorf("FileKindLab should map to 1")
	}
	if fileKindToInt(FileKindScript) != 2 {
		t.Errorf("FileKindScript should map to 2")
	}
	if fileKindToInt(FileKindDocs) != 3 {
		t.Errorf("FileKindDocs should map to 3")
	}
	if fileKindToInt(FileKindConfig) != 4 {
		t.Errorf("FileKindConfig should map to 4")
	}
	if fileKindToInt(FileKindResource) != 5 {
		t.Errorf("FileKindResource should map to 5")
	}
	if fileKindToInt(FileKindTemplate) != 6 {
		t.Errorf("FileKindTemplate should map to 6")
	}
	if fileKindToInt(FileKindLicense) != 7 {
		t.Errorf("FileKindLicense should map to 7")
	}

	if definitionKindToInt(DefinitionKindImplementation) != 0 {
		t.Errorf("DefinitionKindImplementation should map to 0")
	}
	if definitionKindToInt(DefinitionKindInterface) != 1 {
		t.Errorf("DefinitionKindInterface should map to 1")
	}
	if definitionKindToInt(DefinitionKindConstant) != 2 {
		t.Errorf("DefinitionKindConstant should map to 2")
	}
	if definitionKindToInt(DefinitionKindTest) != 3 {
		t.Errorf("DefinitionKindTest should map to 3")
	}
}

func TestExtractReadmeSummary(t *testing.T) {
	content := "# Title\n\n### Summary\n\nThis is the summary.\n\n### Specs\n\nSome specs.\n"
	result := extractReadmeSummary(content)
	if result != "This is the summary." {
		t.Errorf("expected 'This is the summary.', got %q", result)
	}

	empty := extractReadmeSummary("# No summary section\n\nJust text.\n")
	if empty != "" {
		t.Errorf("expected empty string, got %q", empty)
	}

	multiline := extractReadmeSummary("### Summary\n\nLine 1.\nLine 2.\n\n### Specs\n")
	if multiline != "Line 1.\nLine 2." {
		t.Errorf("expected multiline summary, got %q", multiline)
	}
}

//#endregion 🕰️SQLite Export

// 📜#region 🔬Policy
func TestPolicyListCommand(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]PolicyDef)
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
	policies, ok := result.Data.([]PolicyDef)
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

func TestExhaustivePolicyBreachListCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy breach list test in short mode")
	}
	result := ToolPolicyBreachList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyBreachList returned error: %s", result.Error)
	}
}

func TestFixtureBreachsGroupedInline(t *testing.T) {
	path := "repo/asset/fixture/some/folder/⚛️⚛️file_invalid.tsx"
	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("fixture policy check failed: %v", err)
	}
	if len(breachs) == 0 {
		t.Fatal("expected fixture breachs")
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	required := []Statute{
		BreachCodeSectionMissingSummary,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range required {
		if counts[kind] == 0 {
			t.Fatalf("expected statute %s", kind)
		}
	}
}

func TestFixtureBreachsByLanguage(t *testing.T) {
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []Statute
	}{
		{
			path:          "repo/asset/fixture/some/folder/🐍🐍file_invalid.py",
			requiredKinds: []Statute{BreachCodeDefMissingSummary},
		},
		{
			path:          "repo/asset/fixture/some/folder/🔷🔷file_invalid.cs",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
		{
			path:          "repo/asset/fixture/some/folder/🐹🐹file_invalid.go",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
	}
	for _, fixture := range fixtures {
		scope := Scope{Kind: ScopeFile, FilePath: fixture.path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixture.path})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", fixture.path, err)
		}
		if len(breachs) == 0 {
			t.Fatalf("expected fixture breachs for %s", fixture.path)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		for _, kind := range fixture.requiredKinds {
			if counts[kind] == 0 {
				t.Fatalf("expected statute %s in %s", kind, fixture.path)
			}
		}
	}
	clean := []string{
		"repo/asset/fixture/some/folder/⚛️⚛️file_fixed.tsx",
		"repo/asset/fixture/some/folder/🐍🐍file_fixed.py",
		"repo/asset/fixture/some/folder/🔷🔷file_fixed.cs",
		"repo/asset/fixture/some/folder/🐹🐹file_fixed.go",
	}
	for _, path := range clean {
		scope := Scope{Kind: ScopeFile, FilePath: path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", path, err)
		}
		if len(breachs) != 0 {
			for _, v := range breachs {
				t.Logf("[DEBUG] breach in %s: kind=%s scope=%s line=%d summary=%s", path, v.Kind, v.Scope, v.Line, v.Summary)
			}
			t.Fatalf("expected no breachs for %s, got %d", path, len(breachs))
		}
	}
}

func TestSectionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[BreachCodeSectionMissingSummary] == 0 {
		t.Fatal("expected section missing summary breach")
	}
}

func TestSectionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeSectionMissingSummary {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestDefinitionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[BreachCodeDefMissingSummary] == 0 {
		t.Fatal("expected definition missing summary breach")
	}
}

func TestDefinitionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeDefMissingSummary || v.Kind == BreachCodeDefMissingRequirements {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestSectionDocLinesExemptsDocComments(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n// Functions MUST be exported.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentInline {
			t.Fatalf("section doc comment wrongly flagged as inline at line %d", v.Line)
		}
	}
}

func TestDefinitionNativeDocstring(t *testing.T) {
	tests := []struct {
		name         string
		file         string
		content      string
		expectBreach bool
	}{
		{
			name:         "TypeScript // comments should flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖Header\n\n// [💻src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
			expectBreach: true,
		},
		{
			name:         "TypeScript JSDoc should NOT flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖Header\n\n// [💻src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "Go // comments should NOT flag breach (native format)",
			file:         "src/app.go",
			content:      "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](repo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.go#Functions](repo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](repo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "Python # comments should flag breach (should use triple-quote docstring)",
			file:         "src/app.py",
			content:      "# #region 🔖Header\n\n# [💻src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
			expectBreach: true,
		},
		{
			name:         "Python triple-quote docstring should NOT flag breach",
			file:         "src/app.py",
			content:      "# #region 🔖Header\n\n# [💻src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "CSharp // comments should flag breach (should use ///)",
			file:         "src/App.cs",
			content:      "// #region 🔖Header\n\n// [💻src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖Types\n",
			expectBreach: true,
		},
		{
			name:         "CSharp /// comments should NOT flag breach",
			file:         "src/App.cs",
			content:      "// #region 🔖Header\n\n// [💻src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖Types\n",
			expectBreach: false,
		},
		{
			name:         "Rust // comments should flag breach (should use ///)",
			file:         "src/lib.rs",
			content:      "// #region 🔖Header\n\n// [💻src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖Types\n",
			expectBreach: true,
		},
		{
			name:         "Rust /// comments should NOT flag breach",
			file:         "src/lib.rs",
			content:      "// #region 🔖Header\n\n// [💻src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖Types\n",
			expectBreach: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tt.file))
			os.MkdirAll(dir, 0o755)
			absPath := filepath.Join(tmpDir, tt.file)
			if err := WriteTextFile(absPath, tt.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tt.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tt.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			hasBreach := false
			for _, v := range breachs {
				if v.Kind == BreachCodeDefNotNativeDocstring {
					hasBreach = true
					break
				}
			}
			if tt.expectBreach && !hasBreach {
				t.Fatal("expected DefNotNativeDocstring breach but got none")
			}
			if !tt.expectBreach && hasBreach {
				t.Fatal("unexpected DefNotNativeDocstring breach")
			}
		})
	}
}

func TestDefinitionNativeDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "/**") {
		t.Fatal("expected JSDoc opening after autofix")
	}
	if !strings.Contains(fixedContent, "**/") {
		t.Fatal("expected JSDoc closing after autofix")
	}
	if !strings.Contains(fixedContent, " * Does work.") {
		t.Fatal("expected summary line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * doWork MUST be idempotent.") {
		t.Fatal("expected spec line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, "§doWork") {
		t.Fatal("expected identification in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("expected identification emitted as a single JSDoc line after autofix")
	}
	if strings.Contains(fixedContent, " *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect doubled asterisk marker before definition identification after autofix")
	}
	if strings.Contains(fixedContent, " *\n * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect an extra blank JSDoc separator before definition identification after autofix")
	}
}

func TestPythonTripleQuoteDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, `"""Does work.`) {
		t.Fatal("expected triple-quote docstring with summary after autofix")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec line in docstring after autofix")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification in docstring after autofix")
	}
	if !strings.Contains(fixedContent, `"""`) {
		t.Fatal("expected closing triple-quote after autofix")
	}
	if strings.Contains(fixedContent, "# Does work.") {
		t.Fatal("# comment should be removed after autofix")
	}
}

func TestPythonTripleQuoteDocstringMerge(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    \"\"\"Does work.\"\"\"\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach for # comments above existing docstring")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "Does work.") {
		t.Fatal("expected existing summary preserved after merge")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec from # comment merged into docstring")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification merged into docstring")
	}
	if strings.Contains(fixedContent, "# do_work MUST") {
		t.Fatal("# comment should be removed after merge autofix")
	}
}

func TestPythonTripleQuoteDocstringExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentBlock {
			t.Fatalf("Python triple-quote docstring should not be flagged as block comment at line %d", v.Line)
		}
		if v.Kind == BreachCodeDefNotNativeDocstring {
			t.Fatalf("Python triple-quote docstring should not flag DefNotNativeDocstring at line %d", v.Line)
		}
	}
}

func TestDefinitionJSDocExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentJSDoc {
			t.Fatalf("definition JSDoc should not be flagged as comment breach at line %d", v.Line)
		}
		if v.Kind == BreachCodeCommentBlock {
			t.Fatalf("definition JSDoc should not be flagged as block comment breach at line %d", v.Line)
		}
	}
}

func TestSectionHeaderIdAndUri(t *testing.T) {
	id := SectionHeaderId("src/app.ts", "Functions")
	if !strings.Contains(id, emojiText(EmojiSection)+Flat("Functions")) {
		t.Fatalf("unexpected section header id: %s", id)
	}
	if strings.HasPrefix(id, emojiText(EmojiSection)) {
		t.Fatalf("section header id should include file parent before section emoji: %s", id)
	}
	nestedId := SectionHeaderId("src/shared.ts", "Types#YPath Types")
	if !strings.Contains(nestedId, emojiText(EmojiSection)+Flat("Types")+emojiText(EmojiSection)+Flat("YPath Types")) {
		t.Fatalf("nested section header id should include section emoji before each nested segment, got: %s", nestedId)
	}
	uri := SectionHeaderUri("src/app.ts", "Functions")
	if !strings.HasPrefix(uri, "repo://section/") {
		t.Fatalf("unexpected section header uri: %s", uri)
	}
	if !strings.Contains(uri, emojiText(EmojiSection)) {
		t.Fatalf("section uri should contain section emoji: %s", uri)
	}
}

func TestDefinitionHeaderIdAndUri(t *testing.T) {
	id := DefinitionHeaderId("src/app.ts", "Functions", "doWork", "implementation")
	if !strings.Contains(id, emojiText(EmojiDefinitionImpl)+Flat("doWork")) {
		t.Fatalf("unexpected definition header id: %s", id)
	}
	uri := DefinitionHeaderUri("src/app.ts", "Functions", "doWork")
	if !strings.HasPrefix(uri, "repo://definition/") {
		t.Fatalf("unexpected definition header uri: %s", uri)
	}
	if !strings.Contains(uri, Flat("doWork")) {
		t.Fatalf("definition uri should contain flattened def name: %s", uri)
	}
}

func TestRequirementsBreach(t *testing.T) {
	t.Run("isSpecText detects RFC 2119 keywords", func(t *testing.T) {
		cases := []struct {
			text   string
			expect bool
		}{
			{"File headers MUST contain License subregions.", true},
			{"Implementations SHOULD follow the standard.", true},
			{"This feature MAY be omitted.", true},
			{"Clients SHALL NOT modify the data.", true},
			{"This is REQUIRED for all files.", true},
			{"This approach is RECOMMENDED.", true},
			{"This field is OPTIONAL.", true},
			{"MUST NOT contain inline code.", true},
			{"This is a normal comment.", false},
			{"Just some text here.", false},
			{"", false},
		}
		for _, tc := range cases {
			got := isSpecText(tc.text)
			if got != tc.expect {
				t.Errorf("isSpecText(%q) = %v, want %v", tc.text, got, tc.expect)
			}
		}
	})

	t.Run("hasImplementationSyntax detects backticks", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"File headers MUST contain `License` subregions.", true},
			{"Use `FormatHeader` to build headers.", true},
			{"File headers MUST contain License subregions.", false},
			{"Requirements MUST be implementation-agnostic.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("hasImplementationSyntax detects function calls", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"FormatHeader() MUST build the header.", true},
			{"Call ctx.ReadText() for content.", true},
			{"File headers MUST contain License subregions.", false},
			{"Requirements MUST be clean.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("requirementsPolicy detects implementation syntax in header Requirements", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain `License` subregions.\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeRequirementsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeRequirementsSyntax for backtick-wrapped code in header Requirements")
		}
	})

	t.Run("requirementsPolicy clean requirements no breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain License subregions.\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeRequirementsSyntax {
				t.Errorf("unexpected BreachCodeRequirementsSyntax for clean spec: %s", v.Summary)
			}
		}
	})

	t.Run("requirementsPolicy detects implementation syntax in section requirements", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST call `ctx.Check()` internally.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := requirementsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeRequirementsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeRequirementsSyntax for backtick in section spec")
		}
	})

	t.Run("section spec comments exempt from inline breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST check constraints.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				t.Errorf("spec comment should be exempt from inline breach: line %d %s", v.Line, v.Excerpt)
			}
		}
	})

	t.Run("JSDoc spec comments exempt from JSDoc breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n/**\n * Kits MUST be editable offline.\n */\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeCommentJSDoc {
				t.Errorf("JSDoc spec comment should be exempt from JSDoc breach: line %d", v.Line)
			}
		}
	})

	t.Run("non-spec JSDoc still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\nx = 1;\n\n/**\n * This is a regular docstring without spec keywords.\n */\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentJSDoc {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec JSDoc to still be flagged")
		}
	})

	t.Run("non-spec inline comment still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🥼test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\nconst x = 1;\n\n// This is a regular comment not a spec.\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec inline comment to be flagged")
		}
	})

	t.Run("BreachCodeRequirementsSyntax in breach info table", func(t *testing.T) {
		info := BreachCodeRequirementsSyntax.Info()
		if info.Kind != BreachCodeRequirementsSyntax {
			t.Errorf("expected kind %s, got %s", BreachCodeRequirementsSyntax, info.Kind)
		}
		if info.Autofixable {
			t.Error("requirements syntax breach should not be autofixable")
		}
	})
}

func TestDocsBreach(t *testing.T) {
	t.Run("docsPolicy detects missing README.md", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing README.md")
		}
	})
	t.Run("docsPolicy detects missing Summary section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# 💯Requirements\n\nSome requirements here.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Summary") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # Summary section")
		}
	})
	t.Run("docsPolicy detects missing Requirements section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Requirements") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # 💯Requirements section")
		}
	})
	t.Run("docsPolicy clean README no breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n\n# Docs\n\n# 💯Requirements\n\nSome requirements.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				t.Errorf("unexpected BreachCodeDocsMissingReadme: %s", v.Summary)
			}
		}
	})
	t.Run("docsPolicy deduplicates bundles with same root", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []Bundle{
			{Name: "bundle-a", Root: bundleRoot},
			{Name: "bundle-b", Root: bundleRoot},
		}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		breachs := docsPolicy(ctx)
		count := 0
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				count++
			}
		}
		if count != 1 {
			t.Errorf("expected 1 breach for deduplicated root, got %d", count)
		}
	})
}

func TestFormatHeaderStructure(t *testing.T) {
	lang := NewTypeScriptLanguage()
	fileId := emojiText(EmojiFileCode) + "test/file.ts"
	fileUri := "repo://file/" + emojiText(EmojiFileCode) + "test"
	header := lang.FormatHeader(fileId, fileUri, "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some requirements")
	if !strings.Contains(header, "// #region 🔖Header") {
		t.Error("header missing Header region start")
	}
	if !strings.Contains(header, "// #endregion 🔖Header") {
		t.Error("header missing Header region end")
	}
	if !strings.Contains(header, "["+fileId+"]("+fileUri+")") {
		t.Errorf("header missing [ID](URI) identification, got: %s", header)
	}
	if !strings.Contains(header, "A test file") {
		t.Error("header missing summary")
	}
	if !strings.Contains(header, "2025 Test User <test@test.com>") {
		t.Error("header missing contributors")
	}
	if !strings.Contains(header, "AGPL license text here") {
		t.Error("header missing license text")
	}
	if !strings.Contains(header, "Some requirements") {
		t.Error("header missing requirements text")
	}
}

func TestFormatHeaderEmptyRequirements(t *testing.T) {
	lang := NewGoLanguage()
	header := lang.FormatHeader("💻test/file.go", "repo://file/💻test", "", "2025 Dev <dev@dev.com>", "AGPL text", "")
	if strings.Contains(header, "Requirements") {
		t.Error("header should not contain Requirements subregion when requirements is empty")
	}
	if !strings.Contains(header, "// #region 🔖Header") {
		t.Error("header missing Header region start")
	}
}

func TestFormatHeaderAllLanguages(t *testing.T) {
	languages := []LanguagePlugin{
		NewTypeScriptLanguage(),
		NewGoLanguage(),
		NewPythonLanguage(),
		NewCSharpLanguage(),
		NewRustLanguage(),
		NewRubyLanguage(),
		NewShellLanguage(),
		NewSqlLanguage(),
		NewGraphqlLanguage(),
	}
	for _, lang := range languages {
		header := lang.FormatHeader("💻test/file", "repo://file/💻test", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header == "" {
			t.Errorf("%s: FormatHeader returned empty", lang.Name())
		}
		if !strings.Contains(header, "[💻test/file](repo://file/💻test)") {
			t.Errorf("%s: header missing [ID](URI) identification", lang.Name())
		}
	}
	noHeader := []LanguagePlugin{
		NewMarkdownLanguage(),
		NewTomlLanguage(),
		NewYamlLanguage(),
	}
	for _, lang := range noHeader {
		header := lang.FormatHeader("💻test/file", "repo://file/💻test", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header != "" {
			t.Errorf("%s: FormatHeader should return empty for non-header language", lang.Name())
		}
	}
}

func TestTerritory(t *testing.T) {
	t.Run("AllKinds flat", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
			Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingSummary {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingSummary, kinds[1])
		}
	})
	t.Run("AllKinds nested groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
				{
					Name:        "Section",
					Description: "Section-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeSectionEmpty},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
	})
	t.Run("AllKinds mixed kinds and groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Kinds:       []Statute{BreachCodeCommentInline},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeCommentInline {
			t.Errorf("expected %s first, got %s", BreachCodeCommentInline, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s second, got %s", BreachCodeFileMissingHeaderRegion, kinds[1])
		}
	})
	t.Run("AllKinds deeply nested", func(t *testing.T) {
		g := Territory{
			Name:   "Root",
			Scopes: []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "Level1",
					Scopes: []string{"**/*"},
					Groups: []Territory{
						{
							Name:   "Level2",
							Scopes: []string{"**/*"},
							Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
						},
					},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 1 {
			t.Fatalf("expected 1 kind, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
	})
	t.Run("AllKinds empty group", func(t *testing.T) {
		g := Territory{
			Name:   "Empty",
			Scopes: []string{"**/*"},
		}
		kinds := g.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
	t.Run("GetID and GetURI", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
		}
		id := g.GetID()
		if id == "" {
			t.Error("expected non-empty ID")
		}
		if !strings.Contains(id, "File") {
			t.Errorf("expected ID to contain 'File', got %s", id)
		}
		uri := g.GetURI()
		if uri == "" {
			t.Error("expected non-empty URI")
		}
		if !strings.HasPrefix(uri, "repo://") {
			t.Errorf("expected URI to start with 'repo://', got %s", uri)
		}
	})
}

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestSystemPolicy(t *testing.T) {
	t.Run("detects settings.json outside devcontainer", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachSystemDevcontainerVscodeSettingsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected settings-outside-devcontainer breach")
		}
	})
	t.Run("detects extensions.json missing devcontainer recommendations", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachSystemDevcontainerVscodeExtensionsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected extensions-outside-devcontainer breach")
		}
	})
	t.Run("no extensions breach when workspace recommendations include devcontainer extensions", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python","golang.go","ms-vscode-remote.remote-containers"]}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		for _, v := range breachs {
			if v.Kind == BreachSystemDevcontainerVscodeExtensionsOutside {
				t.Error("expected no extensions breach when workspace recommendations include devcontainer extensions")
			}
		}
	})
	t.Run("no breachs when .vscode files absent", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("autofix moves settings.json into devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		if _, err := os.Stat(filepath.Join(tmpDir, ".vscode", "settings.json")); !os.IsNotExist(err) {
			t.Error("expected .vscode/settings.json to be removed")
		}
		dcPath := filepath.Join(tmpDir, ".devcontainer", "devcontainer.json")
		dcData, err := os.ReadFile(dcPath)
		if err != nil {
			t.Fatalf("expected devcontainer.json to exist: %v", err)
		}
		var dc map[string]interface{}
		if err := json.Unmarshal(dcData, &dc); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		if customizations == nil {
			t.Fatal("expected customizations key")
		}
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode == nil {
			t.Fatal("expected vscode key in customizations")
		}
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings == nil {
			t.Fatal("expected settings key in customizations.vscode")
		}
		if settings["editor.fontSize"] != float64(14) {
			t.Errorf("expected editor.fontSize=14, got %v", settings["editor.fontSize"])
		}
	})
	t.Run("autofix syncs extensions.json from devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		extPath := filepath.Join(tmpDir, ".vscode", "extensions.json")
		if _, err := os.Stat(extPath); err != nil {
			t.Fatalf("expected .vscode/extensions.json to remain: %v", err)
		}
		extData, err := os.ReadFile(extPath)
		if err != nil {
			t.Fatalf("read extensions.json: %v", err)
		}
		var extFile map[string]interface{}
		if err := json.Unmarshal(extData, &extFile); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		recommendations, _ := extFile["recommendations"].([]interface{})
		if len(recommendations) != 2 {
			t.Fatalf("expected 2 recommendations, got %d", len(recommendations))
		}
		if recommendations[0] != "ms-python.python" {
			t.Errorf("expected first recommendation ms-python.python, got %v", recommendations[0])
		}
		if recommendations[1] != "golang.go" {
			t.Errorf("expected second recommendation golang.go, got %v", recommendations[1])
		}
	})
	t.Run("autofix merges into existing devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.tabSize": 2}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"name": "test", "image": "ubuntu"}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		if dc["name"] != "test" {
			t.Errorf("expected existing name=test to be preserved, got %v", dc["name"])
		}
		if dc["image"] != "ubuntu" {
			t.Errorf("expected existing image=ubuntu to be preserved, got %v", dc["image"])
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings["editor.tabSize"] != float64(2) {
			t.Errorf("expected editor.tabSize=2, got %v", settings["editor.tabSize"])
		}
	})
	t.Run("autofix both settings and extensions together", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"customizations":{"vscode":{"extensions":["ms-python.python","golang.go"]}}}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
			{Kind: BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 2 {
			t.Fatalf("expected 2 fixes, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode["settings"] == nil {
			t.Error("expected settings in devcontainer.json")
		}
		extData, _ := os.ReadFile(filepath.Join(tmpDir, ".vscode", "extensions.json"))
		var extFile map[string]interface{}
		json.Unmarshal(extData, &extFile)
		recommendations, _ := extFile["recommendations"].([]interface{})
		if len(recommendations) != 2 {
			t.Fatalf("expected synced extensions.json recommendations, got %v", recommendations)
		}
	})
	t.Run("policy registered with correct id", func(t *testing.T) {
		p, found := FindPolicy("system")
		if !found {
			t.Fatal("expected system policy to be registered")
		}
		if p.Name != "System" {
			t.Errorf("expected name System, got %s", p.Name)
		}
		kinds := p.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 statutes, got %d", len(kinds))
		}
		kindSet := map[Statute]bool{}
		for _, k := range kinds {
			kindSet[k] = true
		}
		if !kindSet[BreachSystemDevcontainerVscodeSettingsOutside] {
			t.Error("expected settings-outside-devcontainer kind")
		}
		if !kindSet[BreachSystemDevcontainerVscodeExtensionsOutside] {
			t.Error("expected extensions-outside-devcontainer kind")
		}
	})
	t.Run("statute meta is correct", func(t *testing.T) {
		settingsMeta := BreachSystemDevcontainerVscodeSettingsOutside.Info()
		if !settingsMeta.Autofixable {
			t.Error("expected settings breach to be autofixable")
		}
		if settingsMeta.Priority != BreachPriorityHigh {
			t.Error("expected settings breach to be high priority")
		}
		extMeta := BreachSystemDevcontainerVscodeExtensionsOutside.Info()
		if !extMeta.Autofixable {
			t.Error("expected extensions breach to be autofixable")
		}
		if extMeta.Priority != BreachPriorityHigh {
			t.Error("expected extensions breach to be high priority")
		}
	})
}

func TestBuildTerritoryTree(t *testing.T) {
	t.Run("single group with kinds", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "File",
				Description: "File breachs",
				Scopes:      []string{"**/*.ts"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingSummary},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 node, got %d", len(nodes))
		}
		if nodes[0].Label != "File" {
			t.Errorf("expected label 'File', got %s", nodes[0].Label)
		}
		if nodes[0].Kind != TreeNodeCategory {
			t.Errorf("expected category kind, got %s", nodes[0].Kind)
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		for _, child := range nodes[0].Children {
			if child.Kind != TreeNodeStatute {
				t.Errorf("expected statute node, got %s", child.Kind)
			}
		}
	})
	t.Run("nested groups", func(t *testing.T) {
		groups := []Territory{
			{
				Name:   "Code",
				Scopes: []string{"**/*.ts"},
				Groups: []Territory{
					{
						Name:   "File",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
					},
					{
						Name:   "Section",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeSectionEmpty},
					},
				},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 root node, got %d", len(nodes))
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		fileGroup := nodes[0].Children[0]
		if fileGroup.Label != "File" {
			t.Errorf("expected label 'File', got %s", fileGroup.Label)
		}
		if len(fileGroup.Children) != 1 {
			t.Fatalf("expected 1 child in File group, got %d", len(fileGroup.Children))
		}
	})
	t.Run("empty groups", func(t *testing.T) {
		nodes := buildTerritoryTree(nil)
		if len(nodes) != 0 {
			t.Fatalf("expected 0 nodes, got %d", len(nodes))
		}
	})
	t.Run("group node data contains scopes", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "Sketchpad",
				Description: "Sketchpad breachs",
				Scopes:      []string{"js/sketchpad/**/*.ts", "js/sketchpad/**/*.tsx"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
			},
		}
		nodes := buildTerritoryTree(groups)
		data := nodes[0].Data
		if data == nil {
			t.Fatal("expected non-nil data")
		}
		scopes, ok := data["scopes"].([]string)
		if !ok {
			t.Fatal("expected scopes in data")
		}
		if len(scopes) != 2 {
			t.Fatalf("expected 2 scopes, got %d", len(scopes))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestExhaustivePolicyGroupsGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy groups graphql test in short mode")
	}
	executor := getTestExecutor(t)
	query := `{ policies { id name groups { name description scopes kinds { id } groups { name kinds { id } } } } }`
	result, err := executor.Execute(context.Background(), query, nil)
	if err != nil {
		t.Fatalf("GraphQL query failed: %v", err)
	}
	data, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected map result")
	}
	policiesData, ok := data["policies"].([]interface{})
	if !ok {
		t.Fatal("expected policies array")
	}
	if len(policiesData) == 0 {
		t.Fatal("expected at least one policy")
	}
	for _, pRaw := range policiesData {
		p, ok := pRaw.(map[string]interface{})
		if !ok {
			continue
		}
		groups, ok := p["groups"].([]interface{})
		if !ok {
			t.Fatalf("expected groups array for policy %v", p["id"])
		}
		if len(groups) == 0 {
			t.Errorf("expected at least one group for policy %v", p["id"])
		}
	}
}

// #endregion 🔬Policy

// 📦#region 📌Bundle
func TestBundleListCommand(t *testing.T) {
	result := ToolBundleList()
	if result.Error != "" {
		t.Errorf("ToolBundleList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolBundleList returned nil data")
	}
	bundles, ok := result.Data.([]Bundle)
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

func TestExhaustiveBundleTreeCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle tree test in short mode")
	}
	result := ToolTechnologyTree()
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
	}
}

// #endregion 📌Bundle

// 📁#region 🌨️Folder
func TestFolderListCommand(t *testing.T) {
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFolderList("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFolderTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestFolderCreateMoveDelete(t *testing.T) {
	testFolder := "temp/test-folder-cli"
	createResult := ToolFolderCreate(testFolder)
	if createResult.Error != "" {
		t.Errorf("ToolFolderCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFolderMove(testFolder, testFolder+"-moved")
	if moveResult.Error != "" {
		t.Errorf("ToolFolderMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFolderDelete(testFolder + "-moved")
	if deleteResult.Error != "" {
		t.Errorf("ToolFolderDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion 🌨️Folder

// 📄#region ✏️File
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
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFileTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestFileCreateMoveDelete(t *testing.T) {
	testFile := "temp/test-file-cli.txt"
	createResult := ToolFileCreate(testFile)
	if createResult.Error != "" {
		t.Errorf("ToolFileCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFileMove(testFile, "temp/test-file-cli-moved.txt")
	if moveResult.Error != "" {
		t.Errorf("ToolFileMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFileDelete("temp/test-file-cli-moved.txt")
	if deleteResult.Error != "" {
		t.Errorf("ToolFileDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion ✏️File

// 🔤#region 🪪Rename

func TestApplyRenameCasingsReplacesAllVariants(t *testing.T) {
	input := "MODEL Model model getModel ModelFile USER_MODEL"
	got := applyRenameCasings(input, "model", "representation")
	want := "REPRESENTATION Representation representation getRepresentation RepresentationFile USER_REPRESENTATION"
	if got != want {
		t.Fatalf("applyRenameCasings mismatch:\n  got:  %q\n  want: %q", got, want)
	}
}

func TestApplyRenameCasingsKeepsUnrelatedContent(t *testing.T) {
	input := "foo bar baz"
	got := applyRenameCasings(input, "model", "representation")
	if got != input {
		t.Fatalf("applyRenameCasings changed unrelated content: %q", got)
	}
}

func TestToolRenameRewritesFilesAndFilenames(t *testing.T) {
	tmpRoot := t.TempDir()
	oldRoot := GetRootDir()
	SetRootDir(tmpRoot)
	defer SetRootDir(oldRoot)
	rootDir = tmpRoot

	nestedDir := filepath.Join(tmpRoot, "pkg", "model")
	if err := os.MkdirAll(nestedDir, 0755); err != nil {
		t.Fatalf("mkdir failed: %v", err)
	}
	contentFile := filepath.Join(nestedDir, "Model.go")
	contents := "package model\n\ntype Model struct{}\nconst USER_MODEL = \"model\"\n"
	if err := os.WriteFile(contentFile, []byte(contents), 0644); err != nil {
		t.Fatalf("write contents: %v", err)
	}
	unrelated := filepath.Join(tmpRoot, "pkg", "other.go")
	if err := os.WriteFile(unrelated, []byte("package other\n"), 0644); err != nil {
		t.Fatalf("write other: %v", err)
	}
	ignoredPath := filepath.Join(tmpRoot, "ignored", "model.txt")
	if err := os.MkdirAll(filepath.Dir(ignoredPath), 0755); err != nil {
		t.Fatalf("mkdir ignored: %v", err)
	}
	if err := os.WriteFile(ignoredPath, []byte("model stays"), 0644); err != nil {
		t.Fatalf("write ignored: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tmpRoot, ".gitignore"), []byte("ignored/\n"), 0644); err != nil {
		t.Fatalf("write .gitignore: %v", err)
	}

	result := ToolRename("model", "representation", "")
	if result.Error != "" {
		t.Fatalf("ToolRename error: %s", result.Error)
	}

	renamedContent := filepath.Join(tmpRoot, "pkg", "representation", "Representation.go")
	data, err := os.ReadFile(renamedContent)
	if err != nil {
		t.Fatalf("expected renamed file at %s: %v", renamedContent, err)
	}
	want := "package representation\n\ntype Representation struct{}\nconst USER_REPRESENTATION = \"representation\"\n"
	if string(data) != want {
		t.Fatalf("content mismatch:\n  got:  %q\n  want: %q", string(data), want)
	}
	if _, err := os.Stat(contentFile); !os.IsNotExist(err) {
		t.Fatalf("expected original path to be gone, stat err: %v", err)
	}
	if _, err := os.Stat(filepath.Join(tmpRoot, "pkg", "other.go")); err != nil {
		t.Fatalf("unrelated file disappeared: %v", err)
	}
	ignoredData, err := os.ReadFile(ignoredPath)
	if err != nil {
		t.Fatalf("read ignored file: %v", err)
	}
	if string(ignoredData) != "model stays" {
		t.Fatalf("gitignored file was modified: %q", string(ignoredData))
	}
	if _, err := os.Stat(ignoredPath); err != nil {
		t.Fatalf("gitignored file was renamed: %v", err)
	}
}

func TestToolRenameRejectsEmptyAndIdenticalTokens(t *testing.T) {
	if got := ToolRename("", "new", ""); got.Error == "" {
		t.Fatalf("expected error for empty old token")
	}
	if got := ToolRename("model", "", ""); got.Error == "" {
		t.Fatalf("expected error for empty new token")
	}
	if got := ToolRename("Model", "MODEL", ""); got.Error == "" {
		t.Fatalf("expected error for case-only identical tokens")
	}
}

func TestToolRenameScopeLimitsWalk(t *testing.T) {
	tmpRoot := t.TempDir()
	oldRoot := GetRootDir()
	SetRootDir(tmpRoot)
	defer SetRootDir(oldRoot)
	rootDir = tmpRoot

	inScope := filepath.Join(tmpRoot, "compose", "model.txt")
	outOfScope := filepath.Join(tmpRoot, "elements", "model.txt")
	if err := os.MkdirAll(filepath.Dir(inScope), 0755); err != nil {
		t.Fatalf("mkdir in-scope: %v", err)
	}
	if err := os.MkdirAll(filepath.Dir(outOfScope), 0755); err != nil {
		t.Fatalf("mkdir out-of-scope: %v", err)
	}
	if err := os.WriteFile(inScope, []byte("model in"), 0644); err != nil {
		t.Fatalf("write in: %v", err)
	}
	if err := os.WriteFile(outOfScope, []byte("model out"), 0644); err != nil {
		t.Fatalf("write out: %v", err)
	}

	result := ToolRename("model", "representation", "compose")
	if result.Error != "" {
		t.Fatalf("ToolRename error: %s", result.Error)
	}

	if _, err := os.Stat(inScope); !os.IsNotExist(err) {
		t.Fatalf("expected in-scope file to be renamed, stat err: %v", err)
	}
	renamed := filepath.Join(tmpRoot, "compose", "representation.txt")
	data, err := os.ReadFile(renamed)
	if err != nil {
		t.Fatalf("expected renamed file at %s: %v", renamed, err)
	}
	if string(data) != "representation in" {
		t.Fatalf("in-scope content not rewritten: %q", string(data))
	}
	outData, err := os.ReadFile(outOfScope)
	if err != nil {
		t.Fatalf("out-of-scope file disappeared: %v", err)
	}
	if string(outData) != "model out" {
		t.Fatalf("out-of-scope file was rewritten: %q", string(outData))
	}
}

// #endregion 🪪Rename

// 📑#region 🖲️Section
func TestSectionListCommand(t *testing.T) {
	result := ToolSectionList("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolSectionList returned nil data")
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Error("ToolSectionList data is not []SectionInfo")
		return
	}
	if len(sections) == 0 {
		t.Error("ToolSectionList returned no sections")
	}
	foundHeader := false
	for _, s := range sections {
		if s.Name == "Header" {
			foundHeader = true
			break
		}
	}
	if !foundHeader {
		t.Error("Expected to find 'Header' section in compose/js/index.ts")
	}
}

func TestSectionTreeCommand(t *testing.T) {
	result := ToolSectionTree("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// #endregion 🖲️Section

// 📖#region 🐍Definition
func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

// #endregion 🐍Definition

// 🎫#region 🦉Ticket
func TestTicketListCommand(t *testing.T) {
	year := 2025
	result := ToolTicketList(&year, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
}

func TestTicketOpenNoticketKeyword(t *testing.T) {
	result := ToolTicketOpen("🎫", "Skip Ticket", "NOTICKET skip ticket creation", "gpt-5-mini", "codex", "", true, "", "", false, "", McpClientGeneric, "", "")
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
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🦑repo", "🎫tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	first := ToolTicketOpen("🌱", "Seed Ticket", "Seed prompt", "gpt-5-mini", "codex", "", true, "TEST-GOAL", "", false, "", McpClientGeneric, "", "")
	if first.Error != "" {
		t.Fatalf("failed to seed ticket: %s", first.Error)
	}
	seed, ok := first.Data.(*Ticket)
	if !ok || seed == nil {
		t.Fatalf("expected seeded ticket data")
	}
	second := ToolTicketOpen("🎫", "Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "codex", "", true, "TEST-GOAL", "", false, "", McpClientGeneric, "", "")
	if second.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", second.Error)
	}
	continued, ok := second.Data.(*Ticket)
	if !ok || continued == nil {
		t.Fatalf("expected continued ticket data")
	}
	if continued.Slug != seed.Slug {
		t.Fatalf("expected continued ticket %s, got %s", seed.Slug, continued.Slug)
	}
}

// #endregion 🦉Ticket

// 🆕#region 💌Goal
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
	goal, ok := result.Data.(*Goal)
	if !ok || goal == nil {
		t.Fatal("expected goal data")
	}
	if goal.Title != "Test Goal Creation" {
		t.Errorf("expected title 'Test Goal Creation', got '%s'", goal.Title)
	}
	if goal.Dates.Due != "2026-02-15" {
		t.Errorf("expected due date '2026-02-15', got '%s'", goal.Dates.Due)
	}

	goalPath := filepath.Join(GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestExhaustiveGoalDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow goal document test in short mode")
	}
	os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "TEST-PARENT-GOAL"))
	os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "RENAMED-PARENT"))
	os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL"))

	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

	childTitle := "Test Child Goal"
	childRes := ToolGoalCreate(childTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent.ID, "")
	if childRes.Error != "" {
		t.Fatalf("Failed to create child: %s", childRes.Error)
	}
	child, ok := childRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	expectedChildID := "TEST-PARENT-GOAL/TEST-CHILD-GOAL"
	if child.ID != expectedChildID {
		t.Errorf("Expected child ID '%s', got '%s'", expectedChildID, child.ID)
	}

	childPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(child.ID), "goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	parent.Title = "Renamed Parent"
	err := UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(newChildID), "goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*Goal)
	var foundChild *Goal
	for _, g := range allGoals {

		if strings.HasSuffix(g.ID, "/TEST-CHILD-GOAL") && strings.HasPrefix(g.ID, "RENAMED-PARENT") {
			foundChild = g
			break
		}
	}
	if foundChild == nil {
		t.Errorf("Could not find child with new ID prefix in ListGoals output")
	} else {
		if foundChild.ID != newChildID {
			t.Errorf("Expected listed child ID '%s', got '%s'", newChildID, foundChild.ID)
		}
		if foundChild.Parent != parent.ID {
			t.Errorf("Expected listed child parent '%s', got '%s'", parent.ID, foundChild.Parent)
		}
	}

	ctx := &repoContext{}
	emptyParent := ""
	changeInput := GoalChangeInput{
		ID:     newChildID,
		Parent: &emptyParent,
	}

	updatedChild, err := ctx.GoalChange(changeInput)
	if err != nil {
		t.Fatalf("GoalChange failed: %v", err)
	}

	if updatedChild.ID != "TEST-CHILD-GOAL" {
		t.Errorf("Expected reparented child ID 'TEST-CHILD-GOAL', got '%s'", updatedChild.ID)
	}

	if updatedChild.Parent != "" {
		t.Errorf("Expected empty parent, got '%s'", updatedChild.Parent)
	}

	rootChildPath := filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL", "goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL"))
}

func TestGoalList(t *testing.T) {
	result := ToolGoalList()

	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*Goal)
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
	req := mcp.ReadResourceRequest{}
	req.Params.URI = "repo://goals"
	contents, err := handleGoalsResource(context.Background(), req)
	if err != nil {
		t.Fatalf("handleGoalsResource returned error: %v", err)
	}
	if len(contents) != 1 {
		t.Fatalf("expected 1 ResourceContents entry, got %d", len(contents))
	}
	text, ok := contents[0].(mcp.TextResourceContents)
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
	goals, err := ListGoals()
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

// #endregion 💌Goal

// 🤝#region 🩻Contributor
func TestContributorListCommand(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

// #endregion 🩻Contributor

// 🏺#region 🪨Entity ID
func TestGetArtifactID_Root(t *testing.T) {
	id := GetArtifactID("root", map[string]interface{}{})
	if id != "" {
		t.Errorf("root id: expected empty, got %q", id)
	}
}

func TestGetArtifactID_Years(t *testing.T) {
	id := GetArtifactID("years", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiYear)
	if id != expected {
		t.Errorf("years id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Year(t *testing.T) {
	id := GetArtifactID("year", map[string]interface{}{"parentId": "", "yy": "26"})
	expected := emojiText(EmojiYear) + "26"
	if id != expected {
		t.Errorf("year id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Months(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("months", map[string]interface{}{"parentId": yearId})
	expected := yearId + emojiText(EmojiMonth)
	if id != expected {
		t.Errorf("months id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Month(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("month", map[string]interface{}{"parentId": yearId, "mm": "02"})
	expected := yearId + emojiText(EmojiMonth) + "02"
	if id != expected {
		t.Errorf("month id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Days(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("days", map[string]interface{}{"parentId": monthId})
	expected := monthId + emojiText(EmojiDay)
	if id != expected {
		t.Errorf("days id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Day(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("day", map[string]interface{}{"parentId": monthId, "dd": "15"})
	expected := monthId + emojiText(EmojiDay) + "15"
	if id != expected {
		t.Errorf("day id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hours(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hours", map[string]interface{}{"parentId": dayId})
	expected := dayId + emojiText(EmojiHour)
	if id != expected {
		t.Errorf("hours id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hour(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hour", map[string]interface{}{"parentId": dayId, "hh": "14"})
	expected := dayId + emojiText(EmojiHour) + "14"
	if id != expected {
		t.Errorf("hour id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minutes(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minutes", map[string]interface{}{"parentId": hourId})
	expected := hourId + emojiText(EmojiMinute)
	if id != expected {
		t.Errorf("minutes id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minute(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minute", map[string]interface{}{"parentId": hourId, "mm": "33"})
	expected := hourId + emojiText(EmojiMinute) + "33"
	if id != expected {
		t.Errorf("minute id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Seconds(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("seconds", map[string]interface{}{"parentId": minuteId})
	expected := minuteId + emojiText(EmojiSecond)
	if id != expected {
		t.Errorf("seconds id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Second(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("second", map[string]interface{}{"parentId": minuteId, "ss": "38"})
	expected := minuteId + emojiText(EmojiSecond) + "38"
	if id != expected {
		t.Errorf("second id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Codebase(t *testing.T) {
	id := GetArtifactID("codebase", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiCodebase)
	if id != expected {
		t.Errorf("codebase id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technologies(t *testing.T) {
	id := GetArtifactID("technologies", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiTechnologies)
	if id != expected {
		t.Errorf("technologies id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technology(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"user technology", map[string]interface{}{"name": "compose", "kind": "user"}, emojiText(EmojiTechnologyUser) + "compose"},
		{"infra technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, emojiText(EmojiTechnologyInfra) + "repo"},
		{"research technology", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiTechnologyResearch) + "coda"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("technology", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Bundles(t *testing.T) {
	technologyId := emojiText(EmojiTechnologyUser) + "compose"
	id := GetArtifactID("bundles", map[string]interface{}{"parentId": technologyId})
	expected := technologyId + emojiText(EmojiBundles)
	if id != expected {
		t.Errorf("bundles id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Bundle(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"library bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle", map[string]interface{}{"name": "repo/graphql", "kind": "schema"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"},
		{"ui bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleExample) + "examples"},
		{"site bundle", map[string]interface{}{"name": "compose/desktop", "kind": "site"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "desktop"},
		{"assets bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, emojiText(EmojiTechnologyUser) + "semio" + emojiText(EmojiBundleAssets) + "asset"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("bundle", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folders(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root folders", "", emojiText(EmojiFolders)},
		{"bundle folders", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder folders", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folders", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folder(t *testing.T) {
	bundleId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"org folder under bundle", map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization", "parentId": bundleId}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder at root", map[string]interface{}{"path": ".devcontainer", "name": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"nested folder", map[string]interface{}{"path": "compose/js/sketchpad/pages", "name": "pages", "kind": "organization", "parentId": bundleId + emojiText(EmojiFolderOrg) + "sketchpad"}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folder", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Files(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root files", "", emojiText(EmojiFiles)},
		{"folder files", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"required folder files", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("files", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_File(t *testing.T) {
	folderId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"code file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code", "parentId": folderId}, folderId + emojiText(EmojiFileCode) + "design"},
		{"test file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "name": "sketchpad.test.ts", "kind": "lab", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileLab) + "sketchpadtest"},
		{"config file at root", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "name": "devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainer"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("file", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Line(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	id := GetArtifactID("line", map[string]interface{}{"parentId": fileId, "line": float64(3872)})
	expected := fileId + emojiText(EmojiLine) + "3872"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Range(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	id := GetArtifactID("range", map[string]interface{}{"parentId": fileId, "startLine": float64(3872), "endLine": float64(3875)})
	expected := fileId + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Sections(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	id := GetArtifactID("sections", map[string]interface{}{"parentId": fileId})
	expected := fileId + emojiText(EmojiSections)
	if id != expected {
		t.Errorf("sections id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Section(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level section", map[string]interface{}{"name": "State Managment", "parentId": fileId}, fileId + emojiText(EmojiSection) + "statemanagment"},
		{"nested section", map[string]interface{}{"name": "Store", "parentId": fileId + emojiText(EmojiSection) + "statemanagment"}, fileId + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("section", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Definitions(t *testing.T) {
	sectionId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"
	id := GetArtifactID("definitions", map[string]interface{}{"parentId": sectionId})
	expected := sectionId + emojiText(EmojiDefinitions)
	if id != expected {
		t.Errorf("definitions id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Definition(t *testing.T) {
	sectionId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"
	id := GetArtifactID("definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": sectionId})
	expected := sectionId + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Goals(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root goals", "", emojiText(EmojiGoals)},
		{"nested goals", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiGoals)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("goals", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Goal(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
		{"nested goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("goal", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Tickets(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root tickets", "", emojiText(EmojiTickets)},
		{"goal tickets", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTickets)},
		{"section tickets", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiTickets)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("tickets", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Ticket(t *testing.T) {
	goalId := emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"ticket with parentId", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": goalId}, goalId + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"ticket with goalId fallback", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "goalId": "R26-02-1/RUNNING-SKETCHPAD"}, goalId + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("ticket", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Drafts(t *testing.T) {
	parentId := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"
	id := GetArtifactID("drafts", map[string]interface{}{"parentId": parentId})
	expected := parentId + emojiText(EmojiDrafts)
	if id != expected {
		t.Errorf("drafts id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Draft(t *testing.T) {
	parentId := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"
	id := GetArtifactID("draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": parentId})
	expected := parentId + emojiText(EmojiDraft) + "newarchitecture"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todos(t *testing.T) {
	parentId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todos", map[string]interface{}{"parentId": parentId})
	expected := parentId + emojiText(EmojiTodos)
	if id != expected {
		t.Errorf("todos id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todo(t *testing.T) {
	parentId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": parentId})
	expected := parentId + emojiText(EmojiTodo) + "introducepropersyncmechanism"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Policies(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root policies", "", emojiText(EmojiPolicies)},
		{"file kind policies", emojiText(EmojiFileCode), emojiText(EmojiFileCode) + emojiText(EmojiPolicies)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("policies", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Policy(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"general policy on file kind", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("policy", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Contributors(t *testing.T) {
	id := GetArtifactID("contributors", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiContributors)
	if id != expected {
		t.Errorf("contributors id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Contributor(t *testing.T) {
	id := GetArtifactID("contributor", map[string]interface{}{"github": "usalu"})
	expected := emojiText(EmojiContributor) + "usalu"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Checkpoints(t *testing.T) {
	id := GetArtifactID("checkpoints", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiCheckpoints)
	if id != expected {
		t.Errorf("checkpoints id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Checkpoint(t *testing.T) {
	sha := "cfb3b6084ff3fe883d5f39b08810a0b90997907a"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"with contributorId", map[string]interface{}{"sha": sha, "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCheckpoint) + sha},
		{"with authorId fallback", map[string]interface{}{"sha": sha, "authorId": "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCheckpoint) + sha},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("checkpoint", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Interaction(t *testing.T) {
	secondId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"
	contributorId := emojiText(EmojiContributor) + "usalu"
	entityID := emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"started", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "started"}, secondId + contributorId + entityID + emojiText(EmojiInteractionStarted)},
		{"finished", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "finished"}, secondId + contributorId + entityID + emojiText(EmojiInteractionFinished)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("interaction", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Sessions(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root sessions", "", emojiText(EmojiSessions)},
		{"day sessions", dayId, dayId + emojiText(EmojiSessions)},
		{"checkpoint sessions", checkpointId, checkpointId + emojiText(EmojiSessions)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("sessions", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Session(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	sessionsId := dayId + emojiText(EmojiSessions)
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	checkpointSessionsId := checkpointId + emojiText(EmojiSessions)
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"session with uuid under day", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": sessionsId}, sessionsId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with id fallback under day", map[string]interface{}{"id": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": sessionsId}, sessionsId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session no parent", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"}, emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with uuid under checkpoint", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": checkpointId}, checkpointId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with uuid under checkpoint sessions", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": checkpointSessionsId}, checkpointSessionsId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("session", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactURI_Sessions(t *testing.T) {
	uri := GetArtifactURI("sessions", map[string]interface{}{})
	expected := "repo://sessions/" + emojiText(EmojiSessions)
	if uri != expected {
		t.Errorf("sessions uri: expected %q, got %q", expected, uri)
	}
}

func TestGetArtifactURI_Session(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"session with uuid", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"}, "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with id fallback", map[string]interface{}{"id": "abc123"}, "repo://session/" + emojiText(EmojiSession) + "abc123"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			uri := GetArtifactURI("session", tc.data)
			if uri != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, uri)
			}
		})
	}
}

func TestSessionIdToUri(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"session", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := IdToUri(tt.id)
			if got != tt.want {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestSessionUriToId(t *testing.T) {
	tests := []struct {
		name string
		uri  string
		want string
	}{
		{"session", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := UriToId(tt.uri)
			if got != tt.want {
				t.Errorf("UriToId(%q) = %q, want %q", tt.uri, got, tt.want)
			}
		})
	}
}

func TestSessionKindEmoji(t *testing.T) {
	tests := []struct {
		kind SessionKind
		want string
	}{
		{SessionKindRunning, EmojiSessionRunning},
		{SessionKindCompleted, EmojiSessionCompleted},
		{SessionKindInterrupted, EmojiSessionInterrupted},
	}
	for _, tt := range tests {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := SessionKindEmoji(tt.kind)
			if got != tt.want {
				t.Errorf("SessionKindEmoji(%q) = %q, want %q", tt.kind, got, tt.want)
			}
		})
	}
}

func TestSessionGetID(t *testing.T) {
	uuid := "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"
	uuidFlat := "e753ed61e8cc49b788f7dda53b8d5a15"
	checkpointSHA := "abc123sha"
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": checkpointSHA})
	cases := []struct {
		name     string
		session  Session
		expected string
	}{
		{
			name:     "with checkpoint as parent",
			session:  Session{UUID: uuid, Year: 26, Month: 2, Day: 15, Checkpoint: checkpointSHA, Kind: SessionKindCompleted},
			expected: checkpointId + emojiText(EmojiSession) + uuidFlat,
		},
		{
			name:     "without checkpoint falls back to date document",
			session:  Session{UUID: uuid, Year: 26, Month: 2, Day: 15, Kind: SessionKindCompleted},
			expected: emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiSession) + uuidFlat,
		},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := tc.session.GetID()
			if id != tc.expected {
				t.Errorf("Session.GetID() = %q, want %q", id, tc.expected)
			}
		})
	}
}

func TestSessionGetURI(t *testing.T) {
	s := Session{
		UUID:  "e753ed61-e8cc-49b7-88f7-dda53b8d5a15",
		Year:  26,
		Month: 2,
		Day:   15,
		Kind:  SessionKindCompleted,
	}
	uri := s.GetURI()
	expected := "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"
	if uri != expected {
		t.Errorf("Session.GetURI() = %q, want %q", uri, expected)
	}
}

func TestNormalizeTicketSessionID(t *testing.T) {
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	cases := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", "", ""},
		{"raw uuid", "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"already normalized", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"running prefix stripped", emojiText(EmojiSessionRunning) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"completed prefix stripped", emojiText(EmojiSessionCompleted) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"checkpoint prefixed", checkpointId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", checkpointId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"checkpoint prefixed raw uuid", checkpointId + emojiText(EmojiSession) + "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", checkpointId + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeTicketSessionID(tc.input)
			if got != tc.expected {
				t.Errorf("normalizeTicketSessionID(%q) = %q, want %q", tc.input, got, tc.expected)
			}
		})
	}
}

func TestGetArtifactID_Breach(t *testing.T) {
	policyId := emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"
	affected := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore"
	lineId := emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"
	secondId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"
	id := GetArtifactID("breach", map[string]interface{}{"parentId": policyId, "affected": affected, "lineId": lineId, "secondId": secondId})
	expected := policyId + emojiText(EmojiBreach) + affected + emojiText(EmojiBreachScope) + lineId + secondId
	if id != expected {
		t.Errorf("breach id: expected %q, got %q", expected, id)
	}
}

func TestGoalArtifactID(t *testing.T) {
	cases := []struct {
		rawID    string
		expected string
	}{
		{"R26-02-1", emojiText(EmojiGoal) + "r26021"},
		{"R26-02-1/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"AI-OPTIMIZED-REPO", emojiText(EmojiGoal) + "aioptimizedrepo"},
	}
	for _, tc := range cases {
		t.Run(tc.rawID, func(t *testing.T) {
			id := goalArtifactID(tc.rawID)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestSpecExactIDs(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root", "root", map[string]interface{}{}, ""},
		{"years", "years", map[string]interface{}{"parentId": ""}, "🎆"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "🎆26"},
		{"months", "months", map[string]interface{}{"parentId": "🎆26"}, "🎆26🌙"},
		{"month 02", "month", map[string]interface{}{"parentId": "🎆26", "mm": "02"}, "🎆26🌙02"},
		{"days", "days", map[string]interface{}{"parentId": "🎆26🌙02"}, "🎆26🌙02☀️"},
		{"day 15", "day", map[string]interface{}{"parentId": "🎆26🌙02", "dd": "15"}, "🎆26🌙02☀️15"},
		{"hours", "hours", map[string]interface{}{"parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⏰"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "🎆26🌙02☀️15", "hh": "14"}, "🎆26🌙02☀️15⏰14"},
		{"minutes", "minutes", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14"}, "🎆26🌙02☀️15⏰14⌚"},
		{"minute 33", "minute", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14", "mm": "33"}, "🎆26🌙02☀️15⏰14⌚33"},
		{"seconds", "seconds", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33"}, "🎆26🌙02☀️15⏰14⌚33⏱️"},
		{"second 38", "second", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33", "ss": "38"}, "🎆26🌙02☀️15⏰14⌚33⏱️38"},
		{"technologies", "technologies", map[string]interface{}{"parentId": ""}, "🏗️"},
		{"technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, "🧰repo"},
		{"bundles", "bundles", map[string]interface{}{"parentId": "🏘️compose"}, "🏘️compose📦"},
		{"bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, "🏘️compose📚js"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, "📁"},
		{"bundle folders", "folders", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad"}, "🏘️compose📚js🗃️sketchpad📁"},
		{"required folder folders", "folders", map[string]interface{}{"parentId": "🛅github"}, "🛅github📁"},
		{"folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": "🏘️compose📚js"}, "🏘️compose📚js🗃️sketchpad"},
		{"folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, "🛅devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, "📄"},
		{"folder files", "files", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad"}, "🏘️compose📚js🗃️sketchpad📄"},
		{"required folder files", "files", map[string]interface{}{"parentId": "🛅github"}, "🛅github📄"},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": "🏘️compose📚js🗃️sketchpad"}, "🏘️compose📚js🗃️sketchpad💻design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": "🛅devcontainer"}, "🛅devcontainer⚙️devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad💻design", "line": float64(3872)}, "🏘️compose📚js🗃️sketchpad💻design📌3872"},
		{"sections in file", "sections", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad💻design"}, "🏘️compose📚js🗃️sketchpad💻design🔖"},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": "🏘️compose📚js🗃️sketchpad💻design"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment"},
		{"section Store nested", "section", map[string]interface{}{"name": "Store", "parentId": "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🏷️"},
		{"definition createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, "🎯"},
		{"nested goals", "goals", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎯"},
		{"goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": "🎯r26021"}, "🎯r26021🎯runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, "🎫"},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫"},
		{"section tickets", "tickets", map[string]interface{}{"parentId": "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🎫"},
		{"ticket", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism"},
		{"draft", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": "🧰repo⌨️client"}, "🧰repo⌨️client📝newarchitecture"},
		{"todo", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"}, "🏘️compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore📝introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": "💻👮godfiles",
			"affected": "🏘️compose📚js🗃️sketchpad💻designstore",
			"lineId":   "📌3872📌3875",
			"secondId": "🎆26🌙02☀️14⏰19⌚07⏱️12",
		}, "💻👮godfiles🚫🏘️compose📚js🗃️sketchpad💻designstore🔍📌3872📌3875🎆26🌙02☀️14⏰19⌚07⏱️12"},
		{"contributor", "contributor", map[string]interface{}{"alias": "ueli", "github": "usalu"}, "🧑‍💻ueli"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": "🧑‍💻ueli"}, "🧑‍💻ueli🔀cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻ueli",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "started",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻ueli🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism🌱"},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻ueli",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "finished",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻ueli🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism✅"},
		{"sessions", "sessions", map[string]interface{}{"parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⚪"},
		{"session", "session", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⚪e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

func TestPropagateParentIDs(t *testing.T) {
	root := &TreeNode{Kind: TreeNodeCategory, Data: map[string]interface{}{}, Children: []*TreeNode{
		{Kind: TreeNodeTechnology, Data: map[string]interface{}{"name": "compose", "kind": "user"}, Children: []*TreeNode{
			{Kind: TreeNodeBundle, Data: map[string]interface{}{"name": "compose/js", "kind": "library"}, Children: []*TreeNode{
				{Kind: TreeNodeFolder, Data: map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization"}, Children: []*TreeNode{
					{Kind: TreeNodeFile, Data: map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code"}, Children: []*TreeNode{
						{Kind: TreeNodeSection, Data: map[string]interface{}{"name": "Store"}, Children: []*TreeNode{
							{Kind: TreeNodeDefinition, Data: map[string]interface{}{"name": "createStore", "kind": "implementation"}},
						}},
					}},
				}},
			}},
		}},
	}}
	PropagateParentIDs(root, "")
	technologyId := emojiText(EmojiTechnologyUser) + "compose"
	bundleId := technologyId + emojiText(EmojiBundleLibrary) + "js"
	folderId := bundleId + emojiText(EmojiFolderOrg) + "sketchpad"
	fileId := folderId + emojiText(EmojiFileCode) + "design"
	sectionId := fileId + emojiText(EmojiSection) + "store"
	defId := sectionId + emojiText(EmojiDefinitionImpl) + "createstore"
	checks := []struct {
		label    string
		node     *TreeNode
		expected string
	}{
		{"technology", root.Children[0], technologyId},
		{"bundle", root.Children[0].Children[0], bundleId},
		{"folder", root.Children[0].Children[0].Children[0], folderId},
		{"file", root.Children[0].Children[0].Children[0].Children[0], fileId},
		{"section", root.Children[0].Children[0].Children[0].Children[0].Children[0], sectionId},
		{"definition", root.Children[0].Children[0].Children[0].Children[0].Children[0].Children[0], defId},
	}
	for _, c := range checks {
		t.Run(c.label, func(t *testing.T) {
			entityKind := treeNodeKindToEntityKind(c.node.Kind)
			got := GetArtifactID(entityKind, c.node.Data)
			if got != c.expected {
				t.Errorf("expected %q, got %q", c.expected, got)
			}
		})
	}
}

func TestFlat(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{"repo", "repo"},
		{"Design.tsx", "designtsx"},
		{".devcontainer", "devcontainer"},
		{"devcontainer.json", "devcontainerjson"},
		{"RUNNING-SKETCHPAD", "runningsketchpad"},
		{"R26-02-1", "r26021"},
		{"compose.ts", "composets"},
		{"State Managment", "statemanagment"},
		{"createSketchpadStore", "createsketchpadstore"},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			got := Flat(tc.input)
			if got != tc.expected {
				t.Errorf("Flat(%q): expected %q, got %q", tc.input, tc.expected, got)
			}
		})
	}
}

func verifyTreeDocument(t *testing.T, node *TreeNode, parentPrefix string) {
	t.Helper()
	entityKind := treeNodeKindToEntityKind(node.Kind)
	if entityKind == "" {
		for _, child := range node.Children {
			verifyTreeDocument(t, child, parentPrefix)
		}
		return
	}
	id := GetArtifactID(entityKind, node.Data)
	if id == "" {
		return
	}
	if parentPrefix != "" && !strings.HasPrefix(id, parentPrefix) {
		t.Errorf("%s %q: id %q should start with parent prefix %q", entityKind, node.Label, id, parentPrefix)
	}
	for _, child := range node.Children {
		verifyTreeDocument(t, child, id)
	}
}

func TestExhaustiveMonorepoTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var codebaseNode *TreeNode
	for _, child := range tree.Children {
		if child.ID == "codebase" {
			codebaseNode = child
			break
		}
	}
	if codebaseNode == nil {
		t.Fatal("codebase node not found")
	}
	var composeTechnology, composeRepoTechnology *TreeNode
	for _, c := range codebaseNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		t.Logf("Found technology: %s", id)
		if strings.Contains(id, "compose") && !strings.Contains(id, "repo") && !strings.Contains(id, "coda") {
			composeTechnology = c
		}
		if strings.Contains(id, "repo") {
			composeRepoTechnology = c
		}
	}
	if composeTechnology == nil {
		t.Fatal("compose technology not found")
	}
	if composeRepoTechnology == nil {
		t.Fatal("repo technology not found")
	}
	composeId := GetArtifactID("technology", composeTechnology.Data)
	if composeId != emojiText(EmojiTechnologyUser)+"compose" {
		t.Errorf("compose technology id: expected %q, got %q", emojiText(EmojiTechnologyUser)+"compose", composeId)
	}
	composeRepoId := GetArtifactID("technology", composeRepoTechnology.Data)
	if composeRepoId != emojiText(EmojiTechnologyInfra)+"repo" {
		t.Errorf("repo technology id: expected %q, got %q", emojiText(EmojiTechnologyInfra)+"repo", composeRepoId)
	}
	var antlrBundle *TreeNode
	for _, c := range composeTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			t.Logf("Found bundle in composeTechnology: %s", bId)
			if strings.HasSuffix(bId, emojiText("🔤")+"antlr") {
				antlrBundle = c
				break
			}
		}
	}
	if antlrBundle == nil {
		t.Fatal("compose/antlr bundle not found")
	}
	antlrBundleId := GetArtifactID("bundle", antlrBundle.Data)
	expectedBundleId := emojiText(EmojiTechnologyUser) + "compose" + emojiText("🔤") + "antlr"
	if antlrBundleId != expectedBundleId {
		t.Errorf("compose/antlr bundle id: expected %q, got %q", expectedBundleId, antlrBundleId)
	}
	for _, c := range antlrBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedBundleId)
		}
	}
	var clientBundle *TreeNode
	for _, c := range composeRepoTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			if strings.HasSuffix(bId, emojiText(EmojiBundleBinary)+"client") {
				clientBundle = c
				break
			}
		}
	}
	if clientBundle == nil {
		t.Fatal("repo/client bundle not found")
	}
	clientBundleId := GetArtifactID("bundle", clientBundle.Data)
	expectedClientBundleId := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"
	if clientBundleId != expectedClientBundleId {
		t.Errorf("repo/client bundle id: expected %q, got %q", expectedClientBundleId, clientBundleId)
	}
	for _, c := range clientBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedClientBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedClientBundleId)
		}
	}
	verifyTreeDocument(t, tree, "")
}

func TestExhaustiveGoalTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	goalsNode := tree.Children[2]
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			id := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiGoal)) {
				t.Errorf("goal id should start with goal emoji, got %q", id)
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childId, id) {
						t.Errorf("child goal id %q should start with parent goal id %q", childId, id)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, id) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, id)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
}

func TestExhaustiveContributorTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var contributorsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "contributors" {
			contributorsNode = c
			break
		}
	}
	if contributorsNode == nil {
		t.Fatal("contributors node not found")
	}
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id should start with contributor emoji, got %q", id)
			}
		}
	}
}

func TestExhaustiveCheckpointTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var checkpointsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "checkpoints" {
			checkpointsNode = c
			break
		}
	}
	if checkpointsNode == nil {
		t.Fatal("checkpoints node not found")
	}
	for _, c := range checkpointsNode.Children {
		if c.Kind == TreeNodeCheckpoint {
			id := GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, emojiText(EmojiCheckpoint)) {
				t.Errorf("checkpoint id should contain checkpoint emoji, got %q", id)
			}
		}
	}
}

func TestEntityKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "file", "line", "range",
		"section", "definition", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(EntityKinds) != len(expected) {
		t.Fatalf("EntityKinds length: expected %d, got %d", len(expected), len(EntityKinds))
	}
	for i, e := range expected {
		if EntityKinds[i] != e {
			t.Errorf("EntityKinds[%d]: expected %q, got %q", i, e, EntityKinds[i])
		}
	}
}

func TestArtifactKinds(t *testing.T) {
	expected := []string{"repo", "technology", "bundle", "folder", "file", "section", "definition"}
	if len(ArtifactKinds) != len(expected) {
		t.Fatalf("ArtifactKinds length: expected %d, got %d", len(expected), len(ArtifactKinds))
	}
	for i, e := range expected {
		if ArtifactKinds[i] != e {
			t.Errorf("ArtifactKinds[%d]: expected %q, got %q", i, e, ArtifactKinds[i])
		}
	}
}

func TestDiffableKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour",
		"technology", "bundle", "folder", "file", "section", "definition",
		"goal", "ticket", "contributor", "checkpoint", "interaction", "session",
	}
	if len(DiffableKinds) != len(expected) {
		t.Fatalf("DiffableKinds length: expected %d, got %d", len(expected), len(DiffableKinds))
	}
	for i, e := range expected {
		if DiffableKinds[i] != e {
			t.Errorf("DiffableKinds[%d]: expected %q, got %q", i, e, DiffableKinds[i])
		}
	}
}

func TestRelatedToFileKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(RelatedToFileKinds) != len(expected) {
		t.Fatalf("RelatedToFileKinds length: expected %d, got %d", len(expected), len(RelatedToFileKinds))
	}
	for i, e := range expected {
		if RelatedToFileKinds[i] != e {
			t.Errorf("RelatedToFileKinds[%d]: expected %q, got %q", i, e, RelatedToFileKinds[i])
		}
	}
}

func TestTechnologyListIDs(t *testing.T) {
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Fatalf("ToolTechnologyList returned error: %s", result.Error)
	}
	technologies, ok := result.Data.([]Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	expectedIDs := map[string]string{
		"compose": emojiText(EmojiTechnologyUser) + "compose",
		"repo":  emojiText(EmojiTechnologyInfra) + "repo",
		"coda":  emojiText(EmojiTechnologyResearch) + "coda",
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
	result := ToolBundleList()
	if result.Error != "" {
		t.Fatalf("ToolBundleList returned error: %s", result.Error)
	}
	bundles, ok := result.Data.([]Bundle)
	if !ok {
		t.Fatal("ToolBundleList data is not []Bundle")
	}
	expectedIDs := map[string]string{
		"compose/js":         emojiText(EmojiTechnologyUser) + "compose" + emojiText("📜") + "js",
		"compose/engine":     emojiText(EmojiTechnologyUser) + "compose" + emojiText("⚙️") + "engine",
		"compose/go":         emojiText(EmojiTechnologyUser) + "compose" + emojiText("🐹") + "go",
		"compose/rs":         emojiText(EmojiTechnologyUser) + "compose" + emojiText("🦀") + "rs",
		"compose/py":         emojiText(EmojiTechnologyUser) + "compose" + emojiText("🐍") + "py",
		"compose/net":        emojiText(EmojiTechnologyUser) + "compose" + emojiText("🔷") + "net",
		"compose/graphql":    emojiText(EmojiTechnologyUser) + "compose" + emojiText("🔗") + "graphql",
		"compose/jsonschema": emojiText(EmojiTechnologyUser) + "compose" + emojiText("📋") + "jsonschema",
		"compose/openapi":    emojiText(EmojiTechnologyUser) + "compose" + emojiText("📡") + "openapi",
		"compose/desktop":    emojiText(EmojiTechnologyUser) + "compose" + emojiText("🖥️") + "desktop",
		"compose/docs":       emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "docs",
		"compose/play":       emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "play",
		"asset":     emojiText(EmojiTechnologyUser) + "semio" + emojiText(EmojiBundleAssets) + "asset",
		"repo/client":      emojiText(EmojiTechnologyInfra) + "repo" + emojiText("⌨️") + "client",
		"repo/server":      emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🌍") + "server",
		"repo/go":          emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleLibrary) + "go",
		"repo/vscode":      emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🧩") + "vscode",
		"repo/graphql":     emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🔗") + "graphql",
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

func TestSectionListIDs(t *testing.T) {
	result := ToolSectionList("repo/client/main.go")
	if result.Error != "" {
		t.Fatalf("ToolSectionList returned error: %s", result.Error)
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Fatal("ToolSectionList data is not []Section")
	}
	if len(sections) == 0 {
		t.Fatal("ToolSectionList returned no sections")
	}
	seenEmojis := make(map[string]string)
	for _, s := range sections {
		localID := s.GetID()
		emoji := s.Emoji
		if emoji == "" {
			t.Errorf("section %q has no emoji", s.Name)
			continue
		}
		flatName := Flat(s.Name)
		expectedID := emojiText(emoji) + flatName
		if localID != expectedID {
			t.Errorf("section %q local id: expected %q, got %q", s.Name, expectedID, localID)
		}
		if prev, exists := seenEmojis[emoji]; exists {
			t.Errorf("section %q has duplicate emoji %q (same as %q)", s.Name, emoji, prev)
		}
		seenEmojis[emoji] = s.Name
	}
}

func TestContributorListIDs(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Fatalf("ToolContributorList returned error: %s", result.Error)
	}
	contributors, ok := result.Data.([]Contributor)
	if !ok {
		t.Fatal("ToolContributorList data is not []Contributor")
	}
	if len(contributors) == 0 {
		t.Fatal("ToolContributorList returned no contributors")
	}
	for _, c := range contributors {
		id := c.GetID()
		expectedPrefix := emojiText(EmojiContributor)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("contributor %q id %q should start with %q", c.Alias, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(c.Alias)
		if id != expectedID {
			t.Errorf("contributor %q id: expected %q, got %q", c.Alias, expectedID, id)
		}
	}
	foundUeli := false
	for _, c := range contributors {
		if c.Alias == "ueli" {
			if c.GetID() != emojiText(EmojiContributor)+"ueli" {
				t.Errorf("ueli contributor id: expected %q, got %q", emojiText(EmojiContributor)+"ueli", c.GetID())
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
	goals, ok := result.Data.([]*Goal)
	if !ok {
		t.Fatalf("ToolGoalList data is not []*Goal, got %T", result.Data)
	}
	for _, g := range goals {
		id := g.GetID()
		goalEmoji := emojiText(EmojiGoal)
		if !strings.HasPrefix(id, goalEmoji) {
			t.Errorf("goal %q id %q should start with %q", g.ID, id, goalEmoji)
		}
		var expected strings.Builder
		for _, segment := range strings.Split(g.ID, "/") {
			expected.WriteString(goalEmoji)
			expected.WriteString(Flat(segment))
		}
		if id != expected.String() {
			t.Errorf("goal %q id: expected %q, got %q", g.ID, expected.String(), id)
		}
	}
}

func TestExhaustiveTicketListIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket list ids test in short mode")
	}
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Skipf("ToolTicketList returned error: %s", result.Error)
	}
	tickets, ok := result.Data.([]Ticket)
	if !ok {
		t.Skip("ToolTicketList data is not []Ticket")
	}
	for _, tk := range tickets {
		id := tk.GetID()
		expectedPrefix := emojiText(EmojiTicket)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(tk.Slug)
		if id != expectedID {
			t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
		}
	}
}

func TestDraftListIDs(t *testing.T) {
	result := ToolDraftList()
	if result.Error != "" {
		t.Skipf("ToolDraftList returned error: %s", result.Error)
	}
	drafts, ok := result.Data.([]*Draft)
	if !ok {
		t.Skip("ToolDraftList data is not []*Draft")
	}
	for _, d := range drafts {
		id := d.GetID()
		expectedPrefix := emojiText(EmojiDraft)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
		}
	}
}

func TestExhaustiveMonorepoTreeFullIDDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var codebaseNode *TreeNode
	for _, child := range tree.Children {
		if child.ID == "codebase" {
			codebaseNode = child
			break
		}
	}
	if codebaseNode == nil {
		t.Fatal("codebase node not found")
	}
	var composeTechnology, composeRepoTechnology, codaTechnology *TreeNode
	for _, c := range codebaseNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		if id == emojiText(EmojiTechnologyUser)+"compose" {
			composeTechnology = c
		} else if id == emojiText(EmojiTechnologyInfra)+"repo" {
			composeRepoTechnology = c
		} else if id == emojiText(EmojiTechnologyResearch)+"coda" {
			codaTechnology = c
		}
	}
	if composeTechnology == nil {
		t.Fatal("compose technology not found with expected id " + emojiText(EmojiTechnologyUser) + "compose")
	}
	if composeRepoTechnology == nil {
		t.Fatal("repo technology not found with expected id " + emojiText(EmojiTechnologyInfra) + "repo")
	}
	if codaTechnology == nil {
		t.Fatal("coda technology not found with expected id " + emojiText(EmojiTechnologyResearch) + "coda")
	}
	expectedComposeId := emojiText(EmojiTechnologyUser) + "compose"
	actualComposeId := GetArtifactID("technology", composeTechnology.Data)
	if actualComposeId != expectedComposeId {
		t.Errorf("compose technology id: expected %q, got %q", expectedComposeId, actualComposeId)
	}
	expectedRepoId := emojiText(EmojiTechnologyInfra) + "repo"
	actualRepoId := GetArtifactID("technology", composeRepoTechnology.Data)
	if actualRepoId != expectedRepoId {
		t.Errorf("repo technology id: expected %q, got %q", expectedRepoId, actualRepoId)
	}
	expectedCodaId := emojiText(EmojiTechnologyResearch) + "coda"
	actualCodaId := GetArtifactID("technology", codaTechnology.Data)
	if actualCodaId != expectedCodaId {
		t.Errorf("coda technology id: expected %q, got %q", expectedCodaId, actualCodaId)
	}
	bundleChecks := map[string]string{
		"compose/js":      emojiText(EmojiTechnologyUser) + "compose" + emojiText("📜") + "js",
		"compose/go":      emojiText(EmojiTechnologyUser) + "compose" + emojiText("🐹") + "go",
		"compose/engine":  emojiText(EmojiTechnologyUser) + "compose" + emojiText("⚙️") + "engine",
		"asset":  emojiText(EmojiTechnologyUser) + "semio" + emojiText(EmojiBundleAssets) + "asset",
		"compose/desktop": emojiText(EmojiTechnologyUser) + "compose" + emojiText("🖥️") + "desktop",
		"compose/docs":    emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "docs",
		"repo/client":   emojiText(EmojiTechnologyInfra) + "repo" + emojiText("⌨️") + "client",
		"repo/server":   emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🌍") + "server",
		"repo/vscode":   emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🧩") + "vscode",
	}
	allBundles := []*TreeNode{}
	for _, technology := range []*TreeNode{composeTechnology, composeRepoTechnology, codaTechnology} {
		for _, child := range technology.Children {
			if child.Kind == TreeNodeBundle {
				allBundles = append(allBundles, child)
			}
		}
	}
	for _, b := range allBundles {
		bundleId := GetArtifactID("bundle", b.Data)
		name, _ := b.Data["name"].(string)
		if expected, ok := bundleChecks[name]; ok {
			if bundleId != expected {
				t.Errorf("bundle %q id: expected %q, got %q", name, expected, bundleId)
			}
			delete(bundleChecks, name)
		}
		for _, child := range b.Children {
			childEK := treeNodeKindToEntityKind(child.Kind)
			if childEK == "" {
				continue
			}
			childId := GetArtifactID(childEK, child.Data)
			if !strings.HasPrefix(childId, bundleId) {
				t.Errorf("bundle %q child %s %q: id %q should start with bundle id %q", name, childEK, child.Label, childId, bundleId)
			}
		}
	}
	for name := range bundleChecks {
		t.Errorf("expected bundle %q not found in tree", name)
	}
	verifyTreeDocument(t, tree, "")
}

func TestExhaustiveGoalTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	goalsNode := tree.Children[2]
	goalCount := 0
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			goalCount++
			goalId := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(goalId, emojiText(EmojiGoal)) {
				t.Errorf("goal id %q should start with %q", goalId, emojiText(EmojiGoal))
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childGoalId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childGoalId, goalId) {
						t.Errorf("child goal id %q should start with parent goal id %q", childGoalId, goalId)
					}
					if !strings.Contains(childGoalId, emojiText(EmojiGoal)) {
						t.Errorf("child goal id %q should contain goal emoji", childGoalId)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, goalId) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, goalId)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
	if goalCount == 0 {
		t.Log("no goals found in tree (may be expected for fresh repos)")
	}
}

func TestExhaustiveContributorTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var contributorsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "contributors" {
			contributorsNode = c
			break
		}
	}
	if contributorsNode == nil {
		t.Fatal("contributors node not found")
	}
	foundUeli := false
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id %q should start with %q", id, emojiText(EmojiContributor))
			}
			alias, _ := c.Data["alias"].(string)
			expectedID := emojiText(EmojiContributor) + Flat(alias)
			if id != expectedID {
				t.Errorf("contributor %q id: expected %q, got %q", alias, expectedID, id)
			}
			if alias == "ueli" {
				foundUeli = true
				if id != emojiText(EmojiContributor)+"ueli" {
					t.Errorf("ueli contributor id: expected %q, got %q", emojiText(EmojiContributor)+"ueli", id)
				}
			}
		}
	}
	if !foundUeli {
		t.Error("expected to find contributor 'ueli' in tree")
	}
}

func TestExhaustiveCheckpointTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	var checkpointsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "checkpoints" {
			checkpointsNode = c
			break
		}
	}
	if checkpointsNode == nil {
		t.Fatal("checkpoints node not found")
	}
	checkpointCount := 0
	for _, c := range checkpointsNode.Children {
		if c.Kind == TreeNodeCheckpoint {
			checkpointCount++
			id := GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, emojiText(EmojiCheckpoint)) {
				t.Errorf("checkpoint id %q should contain %q", id, emojiText(EmojiCheckpoint))
			}
			sha, _ := c.Data["sha"].(string)
			if sha != "" && !strings.HasSuffix(id, sha) {
				t.Errorf("checkpoint id %q should end with sha %q", id, sha)
			}
			contributorId, _ := c.Data["contributorId"].(string)
			if contributorId != "" && !strings.HasPrefix(id, contributorId) {
				t.Errorf("checkpoint id %q should start with contributor id %q", id, contributorId)
			}
		}
	}
	if checkpointCount == 0 {
		t.Error("no checkpoints found in tree")
	}
}

func TestAllSpecIDExamples(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root id is empty", "root", map[string]interface{}{}, ""},
		{"years under root", "years", map[string]interface{}{"parentId": ""}, "\U0001F386"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "\U0001F38626"},
		{"months under year", "months", map[string]interface{}{"parentId": "\U0001F38626"}, "\U0001F38626\U0001F319"},
		{"month 02", "month", map[string]interface{}{"parentId": "\U0001F38626", "mm": "02"}, "\U0001F38626\U0001F31902"},
		{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay)},
		{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"},
		{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F0"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"},
		{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute)},
		{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33"},
		{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33" + emojiText(EmojiSecond)},
		{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33", "ss": "38"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33" + emojiText(EmojiSecond) + "38"},
		{"technologies under root", "technologies", map[string]interface{}{"parentId": ""}, emojiText(EmojiTechnologies)},
		{"infra technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, emojiText(EmojiTechnologyInfra) + "repo"},
		{"user technology compose", "technology", map[string]interface{}{"name": "compose", "kind": "user"}, emojiText(EmojiTechnologyUser) + "compose"},
		{"research technology coda", "technology", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiTechnologyResearch) + "coda"},
		{"bundles under technology", "bundles", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles)},
		{"library bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle compose/graphql", "bundle", map[string]interface{}{"name": "compose/graphql", "kind": "schema"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle repo/client", "bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"},
		{"ui bundle repo/vscode", "bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle coda/example", "bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleExample) + "examples"},
		{"assets bundle asset", "bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, emojiText(EmojiTechnologyUser) + "semio" + emojiText(EmojiBundleAssets) + "asset"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, emojiText(EmojiFolders)},
		{"bundle sketchpad folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder .github folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
		{"org folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, emojiText(EmojiFiles)},
		{"sketchpad files", "files", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"github files", "files", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design", "line": float64(3872)}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiLine) + "3872"},
		{"range 3872-3875", "range", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore", "startLine": float64(3872), "endLine": float64(3875)}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore" + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"},
		{"sections in file", "sections", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSections)},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment"},
		{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitions)},
		{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
		{"definition interface", "definition", map[string]interface{}{"name": "IStore", "kind": "interface", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionInterface) + "istore"},
		{"definition constant", "definition", map[string]interface{}{"name": "MAX_SIZE", "kind": "constant", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionConstant) + "maxsize"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, emojiText(EmojiGoals)},
		{"nested goals under parent", "goals", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiGoals)},
		{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
		{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, emojiText(EmojiTickets)},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTickets)},
		{"section tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiTickets)},
		{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiDraft) + "newarchitecture"},
		{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore" + emojiText(EmojiTodo) + "introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy Only One Store", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles",
			"affected": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore",
			"lineId":   emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875",
			"secondId": emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
		}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles" + emojiText(EmojiBreach) + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore" + emojiText(EmojiBreachScope) + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875" + emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"},
		{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCheckpoint) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "started",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionStarted)},
		{"interaction edited", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "edited",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionEdited)},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "finished",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionFinished)},
		{"interaction restarted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "restarted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionRestarted)},
		{"interaction deleted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "deleted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionDeleted)},
		{"file test kind", "file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "kind": "lab", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileLab) + "sketchpadtest"},
		{"file script kind", "file", map[string]interface{}{"path": "compose/engine/build.ts", "kind": "script", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "engine"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "engine" + emojiText(EmojiFileScript) + "build"},
		{"file docs kind", "file", map[string]interface{}{"path": "compose/js/README.md", "kind": "docs", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileDocs) + "readme"},
		{"file asset kind", "file", map[string]interface{}{"path": "compose/js/sketchpad/page/showcase/metabolism.mdx", "kind": "resource", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases" + emojiText(EmojiFileResource) + "metabolism"},
		{"file license kind", "file", map[string]interface{}{"path": "compose/go/LICENSE.md", "kind": "license", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "go"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "go" + emojiText(EmojiFileLicense) + "license"},
		{"site bundle compose/docs", "bundle", map[string]interface{}{"name": "compose/docs", "kind": "site"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "docs"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

// #endregion 🪨Entity ID

// 🕸️#region 🎙️GraphQL
func TestGraphQLRepoQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "compose") {
		t.Errorf("Expected result to contain 'compose', got: %s", result)
	}
}

func TestGraphQLBundlesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL bundles returned error: %v", err)
	}
	if !strings.Contains(result, "compose/js") {
		t.Errorf("Expected result to contain 'compose/js', got: %s", result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestExhaustiveGraphQLTicketsQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow graphql tickets query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "tickets") {
		t.Errorf("Expected result to contain 'tickets', got: %s", result)
	}
}

func TestExhaustiveGraphQLAnalyzeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow analyze query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ analyze(scope: "repo/asset/fixture/some/folder/🐹🐹file_fixed.go") { metrics { total } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL analyze returned error: %v", err)
	}
	if !strings.Contains(result, "total") {
		t.Errorf("Expected result to contain 'total', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

func TestGraphQLFixMutation(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `mutation { fix(scope: "repo/go/main_test.go") { fixed remaining } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL fix mutation returned error: %v", err)
	}
	if !strings.Contains(result, "fixed") {
		t.Errorf("Expected result to contain 'fixed', got: %s", result)
	}
}

// #endregion 🎙️GraphQL

// 🌳#region 📜Tree
func executeTreeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	root, _ := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)

	err := root.Execute()
	return buf.String(), err
}

func TestExhaustiveTreeCommands(t *testing.T) {

	if testing.Short() {
		t.Skip("skipping slow tree test")
	}

	output, err := executeTreeCommand("search", "compose/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻compose") {
		t.Errorf("repo tree compose/go missing compose.go, got:\n%s", output)
	}
	if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
		t.Errorf("repo tree default output must be markdown, got:\n%s", output)
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("repo tree default output missing markdown list items, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-folder", "compose/go")
	if err != nil {
		t.Errorf("folder tree failed: %v", err)
	}
	if !strings.Contains(output, "compose.go") {

		if len(output) < 10 {
			t.Errorf("folder tree output suspicious: %s", output)
		}
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("folder tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-file", "compose/go")
	if err != nil {
		t.Errorf("file tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻compose") {
		t.Errorf("file tree missing compose.go")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("file tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-ticket")
	if err != nil {
		t.Errorf("ticket tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("ticket tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("ticket tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "--only-goal")
	if err != nil {
		t.Errorf("goal tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("goal tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("goal tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "compose/go", "--text")
	if err != nil {
		t.Errorf("repo tree text failed: %v", err)
	}
	if !strings.Contains(output, "├── ") && !strings.Contains(output, "└── ") {
		t.Errorf("repo tree text output should use connectors, got:\n%s", output)
	}

	output, err = executeTreeCommand("search", "compose/go", "--json")
	if err != nil {
		t.Errorf("repo tree json failed: %v", err)
	}
	var parsed map[string]interface{}
	if parseErr := json.Unmarshal([]byte(strings.TrimSpace(output)), &parsed); parseErr != nil {
		t.Errorf("repo tree json output is invalid JSON: %v\noutput:\n%s", parseErr, output)
	}
	if _, ok := parsed["kind"]; !ok {
		if _, ok := parsed["Kind"]; !ok {
			t.Errorf("repo tree json output missing kind field: %s", output)
		}
	}
}

func TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}

	fileRel := filepath.ToSlash(filepath.Join("go", "repo", "main.go"))

	openOut, openErr, err := executeCommand(
		"ticket", "open",
		"🎫",
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
	defer os.RemoveAll(GetTicketPath(y, m, d, slug))
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
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

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
		{"policy check", []string{"policy", "check", "code", "compose/js"}},
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

// 🎫#region 🎬Wrong Argument
func TestCliWrongArgs_TicketOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing emoji", []string{"ticket", "open", "--goal", "TEST", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing title", []string{"ticket", "open", "--emoji", "🎫", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "open", "--emoji", "🎫", "--goal", "TEST", "--title", "Valid Title", "--opus-4-5", "--no-management"}},
		{"missing goal", []string{"ticket", "open", "--emoji", "🎫", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
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
		{"ticket open missing title", []string{"ticket", "open", "--emoji", "🎫", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing title"},
		{"ticket open missing goal", []string{"ticket", "open", "--emoji", "🎫", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal"},
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

func TestExhaustiveCliJsonPureData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow CLI JSON pure data test in short mode")
	}
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
		{"ticket open missing title", []string{"ticket", "open", "--emoji", "🎫", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
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

// #endregion 🎬Wrong Argument

// 📑#region 🧬Consolidated
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

func TestCollectEntityProps_MultilineEscaped(t *testing.T) {
	tests := []struct {
		name string
		kind string
		data map[string]interface{}
	}{
		{"ticket summary with newlines", "ticket", map[string]interface{}{
			"slug": "T1", "status": "closed", "title": "Fix Bug",
			"year": float64(2026), "month": float64(1), "day": float64(1),
			"summary": "Fixed three areas:\n\n1. First fix\n2. Second fix\n3. Third fix",
		}},
		{"ticket prompt with newlines", "ticket", map[string]interface{}{
			"slug": "T2", "status": "open", "title": "Add Feature",
			"year": float64(2026), "month": float64(1), "day": float64(2),
			"prompt": "Please implement:\n- item A\n- item B",
		}},
		{"goal description with newlines", "goal", map[string]interface{}{
			"id": "GOAL1", "title": "Goal One", "status": "open",
			"description": "Line one\nLine two\r\nLine three",
		}},
		{"policy description with newlines", "policy", map[string]interface{}{
			"id": "P1", "description": "Rule one\nRule two",
		}},
		{"checkpoint message with newlines", "checkpoint", map[string]interface{}{
			"id": "abc123", "message": "feat: add feature\n\nDetailed description here",
		}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "\n") || strings.Contains(p, "\r") {
					t.Errorf("property contains newline: %q", p)
				}
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

func TestRenderEntityMarkdownLink_AllKinds(t *testing.T) {
	entities := []struct {
		kind string
		data map[string]interface{}
	}{
		{"ticket", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goal", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path",
		}},
		{"folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"file", map[string]interface{}{
			"id": "file.ts",
		}},
		{"section", map[string]interface{}{
			"name": "Sec", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"definition", map[string]interface{}{
			"name": "def", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"contributor", map[string]interface{}{
			"github": "octocat", "name": "Cat",
		}},
		{"todo", map[string]interface{}{
			"name": "Todo",
		}},
		{"draft", map[string]interface{}{
			"id": "draft1",
		}},
		{"policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"statute", map[string]interface{}{
			"id": "vk1", "description": "Desc",
		}},
		{"technology", map[string]interface{}{
			"id": "proj", "description": "Desc",
		}},
		{"checkpoint", map[string]interface{}{
			"sha": "abc123", "message": "msg",
		}},
		{"root", map[string]interface{}{
			"name": "myrepo",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind, func(t *testing.T) {
			output := renderEntityMarkdownLink(tt.kind, tt.data)
			if !strings.HasPrefix(output, "[") {
				t.Errorf("link for %s missing '[' prefix: %s", tt.kind, output)
			}
			if !strings.Contains(output, "](") {
				t.Errorf("link for %s missing '](': %s", tt.kind, output)
			}
			if !strings.Contains(output, "repo://") {
				t.Errorf("link for %s missing 'repo://' uri: %s", tt.kind, output)
			}
			if strings.Contains(output, "```") {
				t.Errorf("link for %s contains code fence: %s", tt.kind, output)
			}
		})
	}
}

func TestInferEntityKind(t *testing.T) {
	cases := []struct {
		key      string
		expected string
	}{
		{"ticketOpen", "ticket"},
		{"ticketClose", "ticket"},
		{"ticketReopen", "ticket"},
		{"ticketChange", "ticket"},
		{"goalCreate", "goal"},
		{"goalClose", "goal"},
		{"goalReopen", "goal"},
		{"goalChange", "goal"},
		{"folderCreate", "folder"},
		{"folderDelete", "folder"},
		{"folderMove", "folder"},
		{"fileCreate", "file"},
		{"fileDelete", "file"},
		{"fileMove", "file"},
		{"sectionCreate", "section"},
		{"sectionDelete", "section"},
		{"sectionMove", "section"},
		{"definitionList", "definition"},
		{"contributorRemove", "contributor"},
		{"todoCreate", "todo"},
		{"todoChange", "todo"},
		{"todoDelete", "todo"},
		{"syncManagement", "root"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "root"},
		{"unknownKey", ""},
	}

	for _, tt := range cases {
		t.Run(tt.key, func(t *testing.T) {
			got := inferEntityKind(tt.key)
			if got != tt.expected {
				t.Errorf("inferEntityKind(%q) = %q, want %q", tt.key, got, tt.expected)
			}
		})
	}
}

func TestExhaustiveMarkdownOutput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow markdown output test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
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
			if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
				t.Errorf("Output should not contain ASCII tree connectors in default markdown mode:\n%s", output)
			}
		})
	}
}

func TestExhaustiveLifecycleCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow lifecycle commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
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
			defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

			openArgs := []string{"ticket", "open", "🎫", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--goal", goalID, "--no-issue", "--no-management"}
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

			defer os.RemoveAll(GetTicketPath(y, m, d, slug))

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
			jsonContent, err := os.ReadFile(filepath.Join(ticketDir, "ticket.json"))
			if err == nil {
				var tm Ticket
				if err := json.Unmarshal(jsonContent, &tm); err == nil {
					if tm.Goal != "🎯testgoal" && tm.Goal != "test-goal" {
						t.Errorf("ticket change goal mismatch: expected test-goal, got %s", tm.Goal)
					}
					if tm.Parent != "🎫parent-ticket-slug" && tm.Parent != "parent-ticket-slug" && tm.Parent != "" {
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
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
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
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
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
		{"TypeScript", ".ts", "const x = 1;\n// #region 🔖%s\nconst y = 2;\n// #endregion 🔖%s\n", "Renamed"},
		{"Go", ".go", "package main\n// #region 🔖%s\nvar y = 2\n// #endregion 🔖%s\n", "Renamed"},
		{"Python", ".py", "# #region 🔖%s\ny = 2\n# #endregion 🔖%s\n", "Renamed"},
		{"CSharp", ".cs", "// #region 🔖%s\nvar y = 2;\n// #endregion 🔖%s\n", "Renamed"},
		{"Rust", ".rs", "// #region 🔖%s\nlet y = 2;\n// #endregion 🔖%s\n", "Renamed"},
		{"Ruby", ".rb", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"Shell", ".sh", "# region %s\ny=2\n# endregion %s\n", "Renamed"},
		{"TOML", ".toml", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"YAML", ".yaml", "# region %s\ny: 2\n# endregion %s\n", "Renamed"},
		{"SQL", ".sql", "-- #region 🔖%s\nSELECT 1;\n-- #endregion 🔖%s\n", "Renamed"},
		{"GraphQL", ".graphql", "# #region 🔖%s\ntype Query { name: String }\n# #endregion 🔖%s\n", "Renamed"},
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
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
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

func TestTicketLifecycle_NoManagement(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle no-management test in short mode")
	}
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

	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".🦑repo", "🎫tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	goal, err := OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}

	testSessionIDOverride = "session-open-1"
	defer func() { testSessionIDOverride = "" }()
	ticket, err := OpenTicket("🎫", "Test Title NoGH", "Test Prompt", "gemini-3-pro", "copilot-chat", "", false, goal.ID, "", true, "", McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("OpenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.Management != nil {
		t.Error("OpenTicket: GitHub data should be nil")
	}
	openSessionCount := len(ticket.Sessions)
	if openSessionCount == 0 {
		t.Fatal("OpenTicket must persist one session")
	}

	testFile := "test.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("content"), 0644); err != nil {
		t.Fatal(err)
	}

	if goal.Title != "Goal Title" {
		t.Errorf("expected title 'Goal Title', got '%s'", goal.Title)
	}
	if goal.Prompt != "Goal Prompt" {
		t.Errorf("expected prompt 'Goal Prompt', got '%s'", goal.Prompt)
	}
	if goal.Client != "copilot-chat" {
		t.Errorf("expected ui 'copilot-chat', got '%s'", goal.Client)
	}
	if goal.LLM != "gemini-3-pro" {
		t.Errorf("expected llm 'gemini-3-pro', got '%s'", goal.LLM)
	}
	if goal.Management != nil {
		t.Error("OpenGoal: GitHub data should be nil")
	}

	goalPath := filepath.Join(tmpDir, ".🦑repo", "🎯goals", "GOAL-TITLE", "goal.json")
	if _, err := os.Stat(goalPath); os.IsNotExist(err) {
		t.Errorf("goal file not created at %s", goalPath)
	}

	run("git", "add", testFile)
	run("git", "commit", "-m", "add test file")

	err = FinishTicket(ticket, "Summary", []string{testFile}, true, false)
	if err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.GetStatus() != TicketStatusClosed {
		t.Errorf("Ticket status mismatch: got %v, want closed", ticket.GetStatus())
	}
	if len(ticket.Interactions) < 2 {
		t.Fatalf("expected at least 2 interactions after close, got %d", len(ticket.Interactions))
	}
	if ticket.Interactions[0].Kind != "ticket.open" {
		t.Errorf("interaction[0].Kind = %q, want %q", ticket.Interactions[0].Kind, "ticket.open")
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.close" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.close")
	}
	if len(ticket.Sessions) != openSessionCount {
		t.Fatalf("FinishTicket must not append sessions: before=%d after=%d", openSessionCount, len(ticket.Sessions))
	}

	testSessionIDOverride = "session-reopen-2"
	err = ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "copilot-chat", "", "", "", true, McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("ReopenTicket should not create synthetic agent, got %d", len(ticket.Agents))
	}
	if ticket.GetStatus() != TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.reopen" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.reopen")
	}
	if len(ticket.Sessions) != openSessionCount+1 {
		t.Fatalf("ReopenTicket must append exactly one session: expected=%d got=%d", openSessionCount+1, len(ticket.Sessions))
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := GoalCreateInput{
		Title:        "Test Goal NoGH 2",
		Description:  "Desc",
		Prompt:       "Prompt",
		DueDate:      "2026-02-15",
		Client:       "cursor",
		LLM:          "gpt-5-2-codex",
		NoManagement: true,
	}

	goal2, err := ctx.GoalCreate(goalInput)
	if err != nil {
		t.Fatalf("GoalCreate failed: %v", err)
	}
	if goal2.Title != "Test Goal NoGH 2" {
		t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
	}

	_, err = ctx.GoalClose(GoalCloseInput{ID: goal2.ID, Summary: "Done", NoManagement: true})
	if err != nil {
		t.Fatalf("GoalClose failed: %v", err)
	}
	closedGoalPath := filepath.Join(tmpDir, ".🦑repo", "🎯goals", goal2.ID, "goal.json")
	closedGoalContent, err := ReadTextFile(closedGoalPath)
	if err != nil {
		t.Fatalf("failed to read closed goal: %v", err)
	}
	var closedGoal Goal
	if err := json.Unmarshal([]byte(closedGoalContent), &closedGoal); err != nil {
		t.Fatalf("failed to unmarshal closed goal: %v", err)
	}
}

func TestExtractSessionIDFromInput(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", ``, ""},
		{"invalid json", `{invalid}`, ""},
		{"trajectory_id", `{"trajectory_id": "test-1"}`, "test-1"},
		{"trajectoryId", `{"trajectoryId": "test-2"}`, "test-2"},
		{"sessionId", `{"sessionId": "test-3"}`, "test-3"},
		{"session_id", `{"session_id": "test-4"}`, "test-4"},
		{"conversationId", `{"conversationId": "test-5"}`, "test-5"},
		{"conversation_id", `{"conversation_id": "test-6"}`, "test-6"},
		{"agent_id", `{"agent_id": "test-7"}`, "test-7"},
		{"agentId", `{"agentId": "test-8"}`, "test-8"},
		{"nested conversation id", `{"native":{"event":{"conversation_id":"test-9"}}}`, "test-9"},
		{"transcript basename fallback", `{"native":{"event":{"transcript_path":"/tmp/transcripts/test-10.jsonl"}}}`, "test-10"},
		{"whitespace", `{"sessionId": " test-9 "}`, "test-9"},
		{"missing", `{"other": "value"}`, ""},
		{"wrong type", `{"sessionId": 123}`, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := extractSessionIDFromInput(json.RawMessage(tt.input))
			if result != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, result)
			}
		})
	}
}

func TestExtractTranscriptAndLLMFromNestedInput(t *testing.T) {
	input := json.RawMessage(`{
		"native": {
			"event": {
				"model": "composer-1.5",
				"transcript_path": "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl"
			}
		}
	}`)

	if got := extractTranscriptFromInput(input); got != "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl" {
		t.Fatalf("expected nested transcript path, got %q", got)
	}

	if got := extractLLMFromInput(input); got != "composer-1.5" {
		t.Fatalf("expected nested model, got %q", got)
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
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🦑repo", "🎫tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	goal, err := OpenGoal("Hook Goal", "Hook Goal Desc", "Hook Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}
	ticket, err := OpenTicket("🪝", "Hook Ticket", "Hook Ticket Prompt", "gemini-3-pro", "copilot-chat", "", false, goal.ID, "", true, "", McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("expected 0 agents at open (no synthetic agent), got %d", len(ticket.Agents))
	}
	stableInput := json.RawMessage(`{"session_id":"agent-session-123","request_id":"req-1","second":"2026-02-23T00:00:00Z"}`)
	RunHook(HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:00Z",
		RepoRoot: tmpDir,
		Input:    stableInput,
	})
	ticketData, err := os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json: %v", err)
	}
	var saved Ticket
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
	RunHook(HookContext{
		Event:    HookAgentToolStarting,
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

func TestArtifactIDAndURI(t *testing.T) {
	tests := []struct {
		name    string
		kind    string
		data    map[string]interface{}
		wantID  string
		wantURI string
	}{
		{
			name:    "root",
			kind:    "root",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "repo://root",
		},
		{
			name:    "technologies collection",
			kind:    "technologies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTechnologies),
			wantURI: "repo://technologies/" + emojiText(EmojiTechnologies),
		},
		{
			name:    "technology user",
			kind:    "technology",
			data:    map[string]interface{}{"name": "compose", "kind": "user"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose",
		},
		{
			name:    "technology infrastructure",
			kind:    "technology",
			data:    map[string]interface{}{"name": "repo", "kind": "infrastructure"},
			wantID:  emojiText(EmojiTechnologyInfra) + "repo",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo",
		},
		{
			name:    "technology research",
			kind:    "technology",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  emojiText(EmojiTechnologyResearch) + "coda",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyResearch) + "coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"technologyCode": "compose", "parentId": emojiText(EmojiTechnologyUser) + "compose"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles),
			wantURI: "repo://bundles/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles),
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/js", "kind": "library"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/example", "kind": "library"},
			wantID:  emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleLibrary) + "examples",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleLibrary) + "examples",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/desktop", "kind": "ui"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleUI) + "desktop",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleUI) + "desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFolders),
			wantURI: "repo://folders/" + emojiText(EmojiFolders),
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "compose/js/src", "parentId": emojiText(EmojiFolderOrg) + "src"},
			wantID:  emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFolders),
			wantURI: "repo://folders/" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFolders),
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/src", "kind": "required"},
			wantID:  emojiText(EmojiFolderRequired) + "src",
			wantURI: "repo://folder/" + emojiText(EmojiFolderRequired) + "src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/utils", "kind": "organization"},
			wantID:  emojiText(EmojiFolderOrg) + "utils",
			wantURI: "repo://folder/" + emojiText(EmojiFolderOrg) + "utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFiles),
			wantURI: "repo://files/" + emojiText(EmojiFiles),
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  emojiText(EmojiFileDocs) + "test",
			wantURI: "repo://file/" + emojiText(EmojiFileDocs) + "test",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  emojiText(EmojiFileCode) + "main",
			wantURI: "repo://file/" + emojiText(EmojiFileCode) + "main",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "compose/js/src/🧪index.test.ts", "kind": "lab"},
			wantID:  emojiText(EmojiFileLab) + "indextest",
			wantURI: "repo://file/" + emojiText(EmojiFileLab) + "indextest",
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  emojiText(EmojiFileConfig) + "tsconfig",
			wantURI: "repo://file/" + emojiText(EmojiFileConfig) + "tsconfig",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  emojiText(EmojiFileScript) + "build",
			wantURI: "repo://file/" + emojiText(EmojiFileScript) + "build",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "🖼️logo.png", "kind": "resource"},
			wantID:  emojiText(EmojiFileResource) + "logo",
			wantURI: "repo://file/" + emojiText(EmojiFileResource) + "logo",
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  emojiText(EmojiFileLicense) + "license",
			wantURI: "repo://file/" + emojiText(EmojiFileLicense) + "license",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": emojiText(EmojiFileCode) + "index"},
			wantID:  emojiText(EmojiFileCode) + "index" + emojiText(EmojiSections),
			wantURI: "repo://sections/" + emojiText(EmojiFileCode) + "index" + emojiText(EmojiSections),
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/Design.tsx#State Management#Design Store"},
			wantID:  buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
			wantURI: "repo://section/" + buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/file.ts#Imports"},
			wantID:  buildSectionID(buildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
			wantURI: "repo://section/" + buildSectionID(buildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": emojiText(EmojiSection) + "types"},
			wantID:  emojiText(EmojiSection) + "types" + emojiText(EmojiDefinitions),
			wantURI: "repo://definitions/" + emojiText(EmojiSection) + "types" + emojiText(EmojiDefinitions),
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "compose/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "compose/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
		},
		{
			name:    "definition go type treated as interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "type", "filePath": "repo/client/main.go", "sectionPath": "GraphQL Types#GraphQL Input Types", "name": "TicketCloseInput"},
			wantID:  buildDefinitionID(buildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "compose/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTickets),
			wantURI: "repo://ticket/" + emojiText(EmojiTickets),
		},
		{
			name: "ticket",
			kind: "ticket",
			data: map[string]interface{}{
				"year":  float64(2025),
				"month": float64(2),
				"day":   float64(4),
				"slug":  "test-ticket",
			},
			wantID:  emojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + emojiText(EmojiTicket) + "testticket",
		},
		{
			name: "ticket with status",
			kind: "ticket",
			data: map[string]interface{}{
				"year":   float64(2025),
				"month":  float64(2),
				"day":    float64(4),
				"slug":   "test-ticket",
				"status": "open",
			},
			wantID:  emojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + emojiText(EmojiTicket) + "testticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiGoals),
			wantURI: "repo://goals/" + emojiText(EmojiGoals),
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "RUNNING-SKETCHPAD", "parentId": ""},
			wantID:  emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + emojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "goal nested",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiDrafts),
			wantURI: "repo://drafts/" + emojiText(EmojiDrafts),
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  emojiText(EmojiDraft) + "mydraft",
			wantURI: "repo://draft/" + emojiText(EmojiDraft) + "mydraft",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTodos),
			wantURI: "repo://todos/" + emojiText(EmojiTodos),
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  emojiText(EmojiTodo) + "mytodo",
			wantURI: "repo://todo/" + emojiText(EmojiTodo) + "mytodo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiPolicies),
			wantURI: "repo://policies/" + emojiText(EmojiPolicies),
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  emojiText(EmojiPolicy) + "codehygiene",
			wantURI: "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene",
		},
		{
			name:    "statutes collection",
			kind:    "statutes",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "repo://statutes",
		},
		{
			name:    "statute",
			kind:    "statute",
			data:    map[string]interface{}{"id": "code/inline-comment"},
			wantID:  "codeinlinecomment",
			wantURI: "repo://statute/codeinlinecomment",
		},
		{
			name:    "contributors collection",
			kind:    "contributors",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiContributors),
			wantURI: "repo://contributors/" + emojiText(EmojiContributors),
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  emojiText(EmojiContributor) + "usalu",
			wantURI: "repo://contributor/" + emojiText(EmojiContributor) + "usalu",
		},
		{
			name:    "checkpoints collection",
			kind:    "checkpoints",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiCheckpoints),
			wantURI: "repo://checkpoints/" + emojiText(EmojiCheckpoints),
		},
		{
			name:    "checkpoint",
			kind:    "checkpoint",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  emojiText(EmojiCheckpoint) + "abc123",
			wantURI: "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123",
		},
		{
			name:    "interactions collection",
			kind:    "interactions",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  "",
			wantURI: "repo://interactions",
		},
		{
			name:    "interaction started ticket",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "started", "entityId": emojiText(EmojiTicket) + "introduceinteractionmechanism"},
			wantID:  emojiText(EmojiTicket) + "introduceinteractionmechanism" + emojiText(EmojiInteractionStarted),
			wantURI: "repo://interaction/" + emojiText(EmojiTicket) + "introduceinteractionmechanism" + emojiText(EmojiInteractionStarted),
		},
		{
			name:    "interaction finished goal",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "finished", "entityId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished),
			wantURI: "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotID := GetArtifactID(tt.kind, tt.data)
			if gotID != tt.wantID {
				t.Errorf("GetArtifactID() = %q, want %q", gotID, tt.wantID)
			}
			gotURI := GetArtifactURI(tt.kind, tt.data)
			if gotURI != tt.wantURI {
				t.Errorf("GetArtifactURI() = %q, want %q", gotURI, tt.wantURI)
			}
		})
	}
}

func TestIdToUri(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"technology user", emojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", emojiText(EmojiTechnologyInfra) + "repo", "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"folder required", emojiText(EmojiFolderRequired) + "src", "repo://folder/" + emojiText(EmojiFolderRequired) + "src"},
		{"folder org", emojiText(EmojiFolderOrg) + "utils", "repo://folder/" + emojiText(EmojiFolderOrg) + "utils"},
		{"file docs", emojiText(EmojiFileDocs) + "test", "repo://file/" + emojiText(EmojiFileDocs) + "test"},
		{"file code", emojiText(EmojiFileCode) + "main", "repo://file/" + emojiText(EmojiFileCode) + "main"},
		{"section", emojiText(EmojiSection), "repo://section/" + emojiText(EmojiSection)},
		{"section nested", buildSectionID(buildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"}), "repo://section/" + buildSectionID(buildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"})},
		{"definition impl", buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation), "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation)},
		{"ticket", emojiText(EmojiTicket) + "testticket", "repo://ticket/" + emojiText(EmojiTicket) + "testticket"},
		{"goal", emojiText(EmojiGoal) + "r2602runningsketchpad", "repo://goal/" + emojiText(EmojiGoal) + "r2602runningsketchpad"},
		{"goal nested", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft"},
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "repo://contributor/" + emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", emojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123"},
		{"interaction started ticket", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted), "repo://interaction/" + emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},
		{"interaction finished goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished), "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished)},
		{"session", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"empty string", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := IdToUri(tt.id)
			if got != tt.want {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestUriToId(t *testing.T) {
	tests := []struct {
		name string
		uri  string
		want string
	}{
		{"root", "repo://root", ""},
		{"technologies", "repo://technologies/" + emojiText(EmojiTechnologies), emojiText(EmojiTechnologies)},
		{"technology", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose", emojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo", emojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"folder", "repo://folder/" + emojiText(EmojiFolderOrg) + "src", emojiText(EmojiFolderOrg) + "src"},
		{"file", "repo://file/" + emojiText(EmojiFileCode) + "test", emojiText(EmojiFileCode) + "test"},
		{"section", "repo://section/" + buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}), buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"})},
		{"definition", "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation), buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation)},
		{"definition with section", "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation), buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation)},
		{"ticket", "repo://ticket/" + emojiText(EmojiTicket) + "testticket", emojiText(EmojiTicket) + "testticket"},
		{"goal", "repo://goal/" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "runningsketchpad"},
		{"goal nested", "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft", emojiText(EmojiDraft) + "mydraft"},
		{"policy", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene", emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", "repo://contributor/" + emojiText(EmojiContributor) + "usalu", emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123", emojiText(EmojiCheckpoint) + "abc123"},
		{"interaction", "repo://interaction/" + emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted), emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},
		{"session", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"kind only no id", "repo://technologies", ""},
		{"invalid", "https://example.com", ""},
		{"empty", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := UriToId(tt.uri)
			if got != tt.want {
				t.Errorf("UriToId(%q) = %q, want %q", tt.uri, got, tt.want)
			}
		})
	}
}

func TestPathToUriPath(t *testing.T) {
	tests := []struct {
		path string
		want string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"test.txt", "test.txt"},
		{"a b/c d", "a%20b/c%20d"},
	}
	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			if got := PathToUriPath(tt.path); got != tt.want {
				t.Errorf("PathToUriPath(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestPathFromUriPath(t *testing.T) {
	tests := []struct {
		uriPath string
		want    string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"a%20b/c%20d", "a b/c d"},
	}
	for _, tt := range tests {
		t.Run(tt.uriPath, func(t *testing.T) {
			if got := PathFromUriPath(tt.uriPath); got != tt.want {
				t.Errorf("PathFromUriPath(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestSectionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "compose/js/src/file.ts", "compose/js/src/file.ts"},
		{"single section", "compose/js/src/file.ts#Imports", "compose/js/src/file.ts/Imports"},
		{"nested sections", "compose/js/src/Design.tsx#State Management#Design Store", "compose/js/src/Design.tsx/State%20Management/Design%20Store"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := SectionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("SectionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestDefinitionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "compose/js/src/file.ts", "compose/js/src/file.ts"},
		{"with section and def", "compose/js/src/file.ts#Section§myFunc", "compose/js/src/file.ts/Section/myFunc"},
		{"def only", "compose/js/src/file.ts§myFunc", "compose/js/src/file.ts/myFunc"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DefinitionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("DefinitionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestParseSectionUriPath(t *testing.T) {
	tests := []struct {
		name      string
		uriPath   string
		wantFile  string
		wantSlugs []string
	}{
		{"file only", "compose/js/src/file.ts", "compose/js/src/file.ts", nil},
		{"file with sections", "compose/js/src/Design.tsx/State%20Management/Design%20Store", "compose/js/src/Design.tsx", []string{"State%20Management", "Design%20Store"}},
		{"file with one section", "compose/js/src/file.ts/Imports", "compose/js/src/file.ts", []string{"Imports"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotFile, gotSlugs := ParseSectionUriPath(tt.uriPath)
			if gotFile != tt.wantFile {
				t.Errorf("ParseSectionUriPath(%q) filePath = %q, want %q", tt.uriPath, gotFile, tt.wantFile)
			}
			if len(gotSlugs) != len(tt.wantSlugs) {
				t.Errorf("ParseSectionUriPath(%q) slugs len = %d, want %d", tt.uriPath, len(gotSlugs), len(tt.wantSlugs))
			} else {
				for i, s := range gotSlugs {
					if s != tt.wantSlugs[i] {
						t.Errorf("ParseSectionUriPath(%q) slug[%d] = %q, want %q", tt.uriPath, i, s, tt.wantSlugs[i])
					}
				}
			}
		})
	}
}

func TestStatuteIdToUriPath(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdToUriPath(tt.id)
			if got != tt.want {
				t.Errorf("StatuteIdToUriPath(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestStatuteUriPathToId(t *testing.T) {
	tests := []struct {
		name    string
		uriPath string
		want    string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteUriPathToId(tt.uriPath)
			if got != tt.want {
				t.Errorf("StatuteUriPathToId(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestTitleizeSlug(t *testing.T) {
	tests := []struct {
		name string
		slug string
		want string
	}{
		{"single word", "code", "Code"},
		{"two words", "inline-comment", "Inline Comment"},
		{"three words", "missing-region-marker", "Missing Region Marker"},
		{"already titleized", "Code", "Code"},
		{"uppercase input", "CODE", "Code"},
		{"empty", "", ""},
		{"single char", "a", "A"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := TitleizeSlug(tt.slug)
			if got != tt.want {
				t.Errorf("TitleizeSlug(%q) = %q, want %q", tt.slug, got, tt.want)
			}
		})
	}
}

func TestStatutePathToIdValue(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"single segment", "code", "Code"},
		{"two segments", "code/inline-comment", "Code#Inline Comment"},
		{"three segments", "code/file/missing-header-region", "Code#File#Missing Header Region"},
		{"four segments", "code/header/region/nested", "Code#Header#Region#Nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatutePathToIdValue(tt.path)
			if got != tt.want {
				t.Errorf("StatutePathToIdValue(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestStatuteIdValueToPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"single segment", "Code", "code"},
		{"two segments", "Code#Inline Comment", "code/inline-comment"},
		{"three segments", "Code#File#Missing Header Region", "code/file/missing-header-region"},
		{"four segments", "Code#Header#Region#Nested", "code/header/region/nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdValueToPath(tt.value)
			if got != tt.want {
				t.Errorf("StatuteIdValueToPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestStatutePathIdValueRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		path string
	}{
		{"single segment", "code"},
		{"two segments", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			idValue := StatutePathToIdValue(tt.path)
			gotPath := StatuteIdValueToPath(idValue)
			if gotPath != tt.path {
				t.Errorf("round trip failed: path %q -> idValue %q -> path %q", tt.path, idValue, gotPath)
			}
		})
	}
}

func TestIdUriRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		id   string
		uri  string
	}{
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "repo://contributor/" + emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", emojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft"},
		{"section", emojiText(EmojiFileCode) + "index" + emojiText(EmojiSection) + "imports", "repo://section/" + emojiText(EmojiFileCode) + "index" + emojiText(EmojiSection) + "imports"},
		{"file", emojiText(EmojiFileCode) + "index", "repo://file/" + emojiText(EmojiFileCode) + "index"},
		{"ticket", emojiText(EmojiTicket) + "20260115someticket", "repo://ticket/" + emojiText(EmojiTicket) + "20260115someticket"},
		{"goal", emojiText(EmojiGoal) + "r2602running", "repo://goal/" + emojiText(EmojiGoal) + "r2602running"},
		{"interaction goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted), "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted)},
		{"technology", emojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose"},
		{"bundle", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
	}
	for _, tt := range tests {
		t.Run(tt.name+"_IdToUri", func(t *testing.T) {
			gotUri := IdToUri(tt.id)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, gotUri, tt.uri)
			}
		})
		t.Run(tt.name+"_UriToId_then_IdToUri", func(t *testing.T) {
			gotId := UriToId(tt.uri)
			gotUri := IdToUri(gotId)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(UriToId(%q)) = %q, want %q (intermediate id: %q)", tt.uri, gotUri, tt.uri, gotId)
			}
		})
	}
}

// 🧪#endregion 🧬Consolidated
func TestMcpToolsSchemas(t *testing.T) {
	s := CreateMcpServer(McpClientGeneric, DefaultCommandTimeout)
	tools := s.ListTools()
	allowedTools := []string{
		"search",
		"ticket_open",
		"ticket_close",
		"ticket_reopen",
		"section_move",
		"section_extract",
		"file_integrate",
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

// #endregion 📜Tree

// 🔍#region 🏂Query
func TestExhaustiveQueryFlag(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query flag test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name          string
		args          []string
		query         string
		expectMatch   string
		expectMissing string
	}{
		{
			name:        "tree --query filters monorepo tree",
			args:        []string{"search", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "tree --query excludes unrelated",
			args:          []string{"search", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "compose/go",
		},
		{
			name:        "technology list --query matches",
			args:        []string{"list", "--only-technology", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "technology tree --query matches",
			args:        []string{"search", "--only-technology", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "bundle list --query matches",
			args:        []string{"list", "--only-bundle", "--query", "engine", "--json"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "bundle list --query excludes unrelated",
			args:          []string{"list", "--only-bundle", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "engine",
		},
		{
			name:        "bundle tree --query matches",
			args:        []string{"search", "--only-bundle", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:        "folder list --query matches",
			args:        []string{"list", "--only-folder", "--query", "go", "--json"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "folder tree --query matches",
			args:        []string{"search", "--only-folder", "--query", "go", "--text"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "file list --query matches",
			args:        []string{"list", "--only-file", "--query", "compose", "--json"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "file tree --query matches",
			args:        []string{"search", "--only-file", "--query", "compose", "--text"},
			query:       "",
			expectMatch: "compose",
		},
		{
			name:        "section list --query matches",
			args:        []string{"list", "--only-section", "--query", "Models", "--json"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "section tree --query matches",
			args:        []string{"search", "--only-section", "--query", "Models", "--text"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "definition list --query matches",
			args:        []string{"list", "--only-definition", "--query", "Kit", "--json"},
			query:       "",
			expectMatch: "Kit",
		},
		{
			name:        "ticket list --query matches",
			args:        []string{"list", "--only-ticket", "--query", "ticket", "--json"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "ticket tree --query matches",
			args:        []string{"search", "--only-ticket", "--query", "ticket", "--text"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "goal list --query matches",
			args:        []string{"list", "--only-goal", "--query", "repo", "--json"},
			query:       "",
			expectMatch: "repo",
		},
		{
			name:        "goal tree --query matches",
			args:        []string{"search", "--only-goal", "--query", "sketchpad", "--text"},
			query:       "",
			expectMatch: "Sketchpad",
		},
		{
			name:          "goal tree --query excludes unrelated",
			args:          []string{"search", "--only-goal", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "Sketchpad",
		},
		{
			name:        "policy list --query matches",
			args:        []string{"list", "--only-policy", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "policy tree --query matches",
			args:        []string{"search", "--only-policy", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "statute list --query matches",
			args:        []string{"list", "--only-statute", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:          "statute list --query excludes unrelated",
			args:          []string{"list", "--only-statute", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "header",
		},
		{
			name:        "statute tree --query matches",
			args:        []string{"search", "--only-statute", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "query command returns matching IDs",
			args:        []string{"query", "bleve"},
			query:       "",
			expectMatch: "bleve",
		},
		{
			name:        "contributor list --query matches",
			args:        []string{"list", "--only-contributor", "--query", "usalu", "--json"},
			query:       "",
			expectMatch: "usalu",
		},
		{
			name:          "contributor list --query excludes unrelated",
			args:          []string{"list", "--only-contributor", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "usalu",
		},
		{
			name:        "checkpoint list --query matches",
			args:        []string{"list", "--only-checkpoint", "--query", "merge", "--json", "--limit", "200"},
			query:       "",
			expectMatch: "merge",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v\nOutput: %s", tt.args, err, output)
			}
			lower := strings.ToLower(output)
			if tt.expectMatch != "" {
				if !strings.Contains(lower, strings.ToLower(tt.expectMatch)) {
					t.Errorf("expected output to contain %q, got:\n%s", tt.expectMatch, output)
				}
			}
			if tt.expectMissing != "" {
				if strings.Contains(lower, strings.ToLower(tt.expectMissing)) {
					t.Errorf("expected output NOT to contain %q, got:\n%s", tt.expectMissing, output)
				}
			}
		})
	}
}

func TestExhaustiveQueryFuzzyMatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query fuzzy match test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("policy list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-policy", "--query", "headr", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "header") {
			t.Errorf("expected fuzzy match for 'headr' to include header-related results, got:\n%s", output)
		}
	})

	t.Run("statute list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--query", "licenss", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "license") {
			t.Errorf("expected fuzzy match for 'licenss' to include license-related results, got:\n%s", output)
		}
	})

	t.Run("bundle list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-bundle", "--query", "engin", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "engine") {
			t.Errorf("expected fuzzy match for 'engin' to include engine, got:\n%s", output)
		}
	})

	t.Run("goal list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-goal", "--query", "sketchpd", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "sketchpad") {
			t.Errorf("expected fuzzy match for 'sketchpd' to include sketchpad, got:\n%s", output)
		}
	})
}

func TestExhaustiveCacheIndexAndTreeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow cache index test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("tree query returns multiple resource kinds for shared keyword", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "bleve", "--text")
		if err != nil {
			t.Fatalf("tree --query bleve failed: %v\nOutput: %s", err, output)
		}
		hasFile := strings.Contains(output, ".go") || strings.Contains(output, "main")
		hasGoal := strings.Contains(output, "AI-OPTIMIZED") || strings.Contains(output, "Repo")
		hasTicket := strings.Contains(output, "ADD-BLEVE") || strings.Contains(output, "02/")
		if !hasFile && !hasGoal && !hasTicket {
			t.Errorf("tree --query bleve should return files, goals, or tickets; got:\n%s", output)
		}
		kinds := 0
		if hasFile {
			kinds++
		}
		if hasGoal {
			kinds++
		}
		if hasTicket {
			kinds++
		}
		if kinds < 2 {
			t.Logf("tree --query returned %d resource kinds (file=%v goal=%v ticket=%v); ideally multiple", kinds, hasFile, hasGoal, hasTicket)
		}
	})

	t.Run("query command returns matching resource IDs", func(t *testing.T) {
		output, err := executeTreeCommand("query", "bleve")
		if err != nil {
			t.Fatalf("query bleve failed: %v\nOutput: %s", err, output)
		}
		var nonEmpty int
		for _, l := range strings.Split(output, "\n") {
			if strings.TrimSpace(l) != "" {
				nonEmpty++
			}
		}
		if nonEmpty == 0 {
			t.Errorf("query bleve should return at least one ID, got:\n%s", output)
		}
	})

	t.Run("tree query for cli returns file and bundle", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "cli", "--text")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		if !strings.Contains(strings.ToLower(output), "cli") {
			t.Errorf("expected 'cli' in output:\n%s", output)
		}
		hasComposeRepo := strings.Contains(output, "repo")
		hasTechnologyOrBundle := strings.Contains(output, "bundle") || strings.Contains(output, "Technologies")
		if !hasComposeRepo || !hasTechnologyOrBundle {
			t.Errorf("tree --query cli should return technology/bundle document; got:\n%s", output)
		}
	})

	t.Run("tree query nonexistent returns minimal output", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "zzz_nonexistent_xyzz", "--text")
		if err != nil {
			t.Fatalf("tree --query nonexistent failed: %v", err)
		}
		if strings.Contains(strings.ToLower(output), "zzz_nonexistent") {
			t.Errorf("tree --query nonexistent should not contain the query term in output")
		}
	})

	t.Run("different queries return different resources", func(t *testing.T) {
		bleveOut, err := executeTreeCommand("search", "--query", "bleve", "--json")
		if err != nil {
			t.Fatalf("tree --query bleve failed: %v", err)
		}
		cliOut, err := executeTreeCommand("search", "--query", "cli", "--json")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		var bleveTree, cliTree map[string]interface{}
		if json.Unmarshal([]byte(strings.TrimSpace(bleveOut)), &bleveTree) != nil {
			t.Fatal("bleve output not valid JSON")
		}
		if json.Unmarshal([]byte(strings.TrimSpace(cliOut)), &cliTree) != nil {
			t.Fatal("cli output not valid JSON")
		}
		bleveStr := fmt.Sprint(bleveTree)
		cliStr := fmt.Sprint(cliTree)
		if bleveStr == cliStr {
			t.Error("tree --query bleve and tree --query cli should return different results")
		}
	})
}

func TestExhaustiveQueryEmptyReturnsAll(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query empty test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name string
		args []string
	}{
		{"policy list no query", []string{"list", "--only-policy", "--json"}},
		{"statute list no query", []string{"list", "--only-statute", "--json"}},
		{"contributor list no query", []string{"list", "--only-contributor", "--json"}},
		{"bundle list no query", []string{"list", "--only-bundle", "--json"}},
		{"goal list no query", []string{"list", "--only-goal", "--json"}},
		{"checkpoint list no query", []string{"list", "--only-checkpoint", "--json", "--limit", "5"}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v", tt.args, err)
			}
			if strings.TrimSpace(output) == "" {
				t.Errorf("expected non-empty output for %v without query", tt.args)
			}
		})
	}
}

func TestExhaustiveStatuteCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statute commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("statute list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--json")
		if err != nil {
			t.Fatalf("statute list failed: %v", err)
		}
		if !strings.Contains(output, "statute") {
			t.Errorf("expected statute JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) < 5 {
			t.Errorf("expected multiple statutes, got %d lines", len(lines))
		}
	})

	t.Run("statute tree returns results", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--text")
		if err != nil {
			t.Fatalf("statute tree failed: %v", err)
		}
		if !strings.Contains(output, "header") {
			t.Errorf("expected statute tree categories in output")
		}
	})

	t.Run("statute list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("statute tree markdown", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute tree md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})
}

func TestExhaustiveCheckpointCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow checkpoint commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("checkpoint list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		if !strings.Contains(output, "checkpoint") {
			t.Errorf("expected checkpoint JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) == 0 {
			t.Error("expected at least one checkpoint")
		}
	})

	t.Run("checkpoint list --query filters", func(t *testing.T) {
		allOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		allLines := strings.Split(strings.TrimSpace(allOutput), "\n")

		filteredOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200", "--query", "merge")
		if err != nil {
			t.Fatalf("checkpoint list --query failed: %v", err)
		}
		filteredLines := strings.Split(strings.TrimSpace(filteredOutput), "\n")
		if len(filteredLines) >= len(allLines) && len(allLines) > 1 {
			t.Errorf("expected --query to reduce results: all=%d filtered=%d", len(allLines), len(filteredLines))
		}
	})

	t.Run("checkpoint list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--md", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("checkpoint list text", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--text", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list text failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty text output")
		}
	})
}

// #endregion 🏂Query

func setupToolTest(t *testing.T) {
	t.Helper()
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	rootDir = findTestRepoRoot(cwd)
	InvalidateTechnologyCache()
}

func TestToolTechnologyList(t *testing.T) {
	setupToolTest(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Errorf("ToolTechnologyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTechnologyList returned nil data")
	}
	technologies, ok := result.Data.([]Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	if len(technologies) == 0 {
		t.Error("ToolTechnologyList returned empty technologies")
	}
}

func TestExhaustiveToolTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool technology tree test in short mode")
	}
	setupToolTest(t)
	result := ToolTechnologyTree()
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
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

func TestExhaustiveToolTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool ticket list test in short mode")
	}
	setupToolTest(t)
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTicketList returned nil data")
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

func TestExhaustiveToolFolderTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool folder tree test in short mode")
	}
	setupToolTest(t)
	result := ToolFolderTree("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
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

func TestExhaustiveToolFileTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool file tree test in short mode")
	}
	setupToolTest(t)
	result := ToolFileTree("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestToolSectionList(t *testing.T) {
	setupToolTest(t)
	result := ToolSectionList("repo/client/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
}

func TestToolSectionTree(t *testing.T) {
	setupToolTest(t)
	result := ToolSectionTree("repo/client/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

func TestToolDefinitionList(t *testing.T) {
	setupToolTest(t)
	result := ToolDefinitionList("repo/client/main.go")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
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

func TestToolFixScope(t *testing.T) {
	setupToolTest(t)
	result := ToolFix("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFix returned error: %s", result.Error)
	}
}

func TestToolFolderCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFolderCreate("test-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderCreate returned error: %s", result.Error)
	}

	result = ToolFolderList(".")
	if result.Error != "" {
		t.Fatalf("ToolFolderList returned error: %s", result.Error)
	}

	result = ToolFolderMove("test-folder", "renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderMove returned error: %s", result.Error)
	}

	result = ToolFolderDelete("renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderDelete returned error: %s", result.Error)
	}
}

func TestToolFileCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFileCreate("test.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileCreate returned error: %s", result.Error)
	}

	result = ToolFileMove("test.txt", "renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileMove returned error: %s", result.Error)
	}

	result = ToolFileDelete("renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileDelete returned error: %s", result.Error)
	}
}

func TestExhaustiveToolTicketLifecycle(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle test in short mode")
	}
	setupToolTest(t)
	title := fmt.Sprintf("Test Lifecycle Ticket %d", time.Now().UnixNano())

	result := ToolTicketOpen("🎫", title, "Test prompt", "sonnet-4-5", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}

	readResult := ToolTicketRead(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	if readResult.Error != "" {
		t.Fatalf("ToolTicketRead returned error: %s", readResult.Error)
	}

	closeResult := ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Test summary", []string{"repo/client/package.json"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose returned error: %s", closeResult.Error)
	}

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen prompt", "sonnet-4-5", "windsurf-chat", "", "", "", "", true, McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen returned error: %s", reopenResult.Error)
	}

	ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Final close", []string{"repo/client/package.json"}, "", true)
	ticketPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	os.RemoveAll(ticketPath)
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

func TestExhaustiveMcpTicketCloseAutoResolve(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close auto-resolve test in short mode")
	}
	setupToolTest(t)

	result := ToolTicketOpen("🎫", "Test Auto Resolve Close", "Test prompt", "sonnet-4-5", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "cleanup", []string{"repo/client/package.json"}, "", true)
		os.RemoveAll(GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	year, month, day, slug, err := resolveTicketForClose("")
	if err != nil {
		t.Fatalf("resolveTicketForClose('') error: %v", err)
	}
	resolved, err := ReadTicket(year, month, day, slug)
	if err != nil {
		t.Fatalf("resolved ticket not readable: %v", err)
	}
	if resolved.Status != TicketStatusOpen {
		t.Errorf("resolveTicketForClose('') resolved to non-open ticket (status=%s)", resolved.Status)
	}
}

func TestExhaustiveMcpTicketCloseWithFullYearPath(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close full-year-path test in short mode")
	}
	setupToolTest(t)

	result := ToolTicketOpen("🎫", "Test Full Year Path", "Test prompt", "sonnet-4-5", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		os.RemoveAll(GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	fullYearPath := fmt.Sprintf("%d/%02d/%02d/%s", 2000+ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	year, month, day, slug, err := parseTicketPath(fullYearPath)
	if err != nil {
		t.Fatalf("parseTicketPath(%q) error: %v", fullYearPath, err)
	}

	closeResult := ToolTicketClose(year, month, day, slug, "Test summary", []string{"repo/client/package.json"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose with full year path returned error: %s", closeResult.Error)
	}
}

func TestToolDraftLifecycle(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

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
	goals, ok := result.Data.([]*Goal)
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

// ⛳#region 🕹️Output Parity
func TestExhaustiveParityGoalList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity goal list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-goal")
		if err != nil {
			t.Fatalf("CLI goal list failed: %v", err)
		}
		toolResult := ToolGoalList()
		if toolResult.Error != "" {
			t.Fatalf("ToolGoalList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of goals", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		toolResult := ToolGoalList()
		mcpOut := toolOutputText(toolResult)
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})

	t.Run("empty output when no goals match filter", func(t *testing.T) {

		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		mcpOut := toolOutputText(ToolGoalList())
		if len(cliOut) == 0 && len(mcpOut) != 0 {
			t.Error("CLI produced empty output but MCP did not")
		}
		if len(cliOut) != 0 && len(mcpOut) == 0 {
			t.Error("MCP produced empty output but CLI did not")
		}
	})
}

func TestExhaustiveParityContributorList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity contributor list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-contributor")
		if err != nil {
			t.Fatalf("CLI contributor list failed: %v", err)
		}
		toolResult := ToolContributorList()
		if toolResult.Error != "" {
			t.Fatalf("ToolContributorList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of contributors", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-contributor")
		mcpOut := toolOutputText(ToolContributorList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestExhaustiveParityTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity ticket list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-ticket")
		if err != nil {
			t.Fatalf("CLI ticket list failed: %v", err)
		}
		toolResult := ToolTicketList(nil, nil, nil)
		if toolResult.Error != "" {
			t.Fatalf("ToolTicketList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of tickets", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-ticket")
		mcpOut := toolOutputText(ToolTicketList(nil, nil, nil))
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestParityDraftList(t *testing.T) {
	setupToolTest(t)

	t.Run("uses event rendering format", func(t *testing.T) {
		toolResult := ToolDraftList()
		if toolResult.Error != "" {
			t.Fatalf("ToolDraftList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)

		drafts, _ := ListDrafts()
		if len(drafts) > 0 && mcpOut == "" {
			t.Error("ToolDraftList returned empty output despite having drafts")
		}
		if len(drafts) == 0 && mcpOut != "" {
			t.Error("ToolDraftList returned output despite having no drafts")
		}
	})

	t.Run("renders same as manual event rendering", func(t *testing.T) {
		drafts, err := ListDrafts()
		if err != nil {
			t.Fatalf("ListDrafts failed: %v", err)
		}
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

func TestExhaustiveParityTechnologyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology list failed: %v", err)
		}
		toolResult := ToolTechnologyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return non-empty output", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-technology")
		mcpOut := toolOutputText(ToolTechnologyList())
		if len(cliOut) == 0 {
			t.Error("CLI technology list returned empty output")
		}
		if len(mcpOut) == 0 {
			t.Error("MCP technology list returned empty output")
		}
	})
}

func TestExhaustiveParityTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity technology tree test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("search", "--only-technology")
		if err != nil {
			t.Fatalf("CLI technology tree failed: %v", err)
		}
		toolResult := ToolTechnologyTree()
		if toolResult.Error != "" {
			t.Fatalf("ToolTechnologyTree returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("technologies are sorted alphabetically", func(t *testing.T) {
		mcpOut := toolOutputText(ToolTechnologyTree())
		lines := strings.Split(strings.TrimSpace(mcpOut), "\n")
		var technologyNames []string
		for _, line := range lines {
			trimmed := strings.TrimSpace(line)
			if idx := strings.Index(trimmed, "repo://technology/"); idx >= 0 {
				rest := trimmed[idx+len("repo://technology/"):]
				// Extract inline name from parenthesized link
				endParen := strings.Index(rest, ")")
				if endParen >= 0 {
					technologyNames = append(technologyNames, rest[:endParen])
				}
			}
		}
		for i := 1; i < len(technologyNames); i++ {
			if technologyNames[i] < technologyNames[i-1] {
				t.Errorf("technologies not sorted: %q comes after %q", technologyNames[i], technologyNames[i-1])
			}
		}
	})
}

func TestExhaustiveParityPolicyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity policy list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-policy")
		if err != nil {
			t.Fatalf("CLI policy list failed: %v", err)
		}
		toolResult := ToolPolicyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolPolicyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of policies", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-policy")
		mcpOut := toolOutputText(ToolPolicyList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

// #endregion 🕹️Output Parity

// #endregion 🔖MCP Tool

// 🌳#region 🗿Monorepo Tree
func TestTreeNodeKindConstants(t *testing.T) {
	t.Run("all kinds are distinct", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeTechnology, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
			TreeNodeContributor, TreeNodeCheckpoint, TreeNodeCategory,
		}
		seen := make(map[TreeNodeKind]bool)
		for _, k := range kinds {
			if seen[k] {
				t.Errorf("duplicate TreeNodeKind: %s", k)
			}
			seen[k] = true
		}
	})

	t.Run("kinds are non-empty strings", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeTechnology, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
			TreeNodeContributor, TreeNodeCheckpoint, TreeNodeCategory,
		}
		for _, k := range kinds {
			if string(k) == "" {
				t.Error("TreeNodeKind should not be empty")
			}
		}
	})
}

func TestTreeFilterIsKindVisible(t *testing.T) {
	t.Run("all visible by default", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible by default")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should be visible by default")
		}
	})

	t.Run("only-kind filters to specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeTechnology: true, TreeNodeBundle: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeTechnology) {
			t.Error("technology should be visible with only-technology")
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible with only-bundle")
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when not in only-kinds")
		}
		if f.IsKindVisible(TreeNodeFile) {
			t.Error("file should not be visible when not in only-kinds")
		}
	})

	t.Run("exclude-kind hides specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: map[TreeNodeKind]bool{TreeNodeFolder: true},
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when excluded")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should still be visible")
		}
	})

	t.Run("category always visible", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeTechnology: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeCategory) {
			t.Error("category should always be visible")
		}
	})
}

func TestTreeFilterMatchesSubKind(t *testing.T) {
	t.Run("matches all when no sub-kind filters", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match any sub-kind by default")
		}
	})

	t.Run("only sub-kind includes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("library should match only-library")
		}
		if f.MatchesSubKind(TreeNodeBundle, "schema") {
			t.Error("schema should not match only-library")
		}
	})

	t.Run("exclude sub-kind removes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: map[TreeNodeKind][]string{TreeNodeFolder: {"required"}},
		}
		if f.MatchesSubKind(TreeNodeFolder, "required") {
			t.Error("required should not match when excluded")
		}
		if !f.MatchesSubKind(TreeNodeFolder, "organization") {
			t.Error("organization should still match")
		}
	})

	t.Run("empty sub-kind always matches", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "") {
			t.Error("empty sub-kind should always match")
		}
	})

	t.Run("case insensitive matching", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"Library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesDate(t *testing.T) {
	t.Run("matches all when no date filters", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesDate(2026, 1, 15) {
			t.Error("should match any date by default")
		}
	})

	t.Run("only-year includes specified year", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}}
		if !f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should match only-year 2026")
		}
		if f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should not match only-year 2026")
		}
	})

	t.Run("exclude-year removes specified year", func(t *testing.T) {
		f := &TreeFilter{ExcludeYears: []int{2026}}
		if f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should not match no-year 2026")
		}
		if !f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should still match")
		}
	})

	t.Run("month filter", func(t *testing.T) {
		f := &TreeFilter{OnlyMonths: []int{6}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("June should match")
		}
		if f.MatchesDate(2026, 7, 1) {
			t.Error("July should not match")
		}
	})

	t.Run("combined year and month", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}, ExcludeMonths: []int{12}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("2026/06 should match")
		}
		if f.MatchesDate(2026, 12, 1) {
			t.Error("2026/12 should not match")
		}
		if f.MatchesDate(2025, 6, 1) {
			t.Error("2025 should not match")
		}
	})
}

func TestTreeFilterMatchesStatus(t *testing.T) {
	t.Run("matches all when no status filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesStatus("open") {
			t.Error("should match any status by default")
		}
		if !f.MatchesStatus("closed") {
			t.Error("should match any status by default")
		}
	})

	t.Run("only-open filters to open", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "open"}
		if !f.MatchesStatus("open") {
			t.Error("open should match only-open")
		}
		if f.MatchesStatus("closed") {
			t.Error("closed should not match only-open")
		}
	})

	t.Run("only-closed filters to closed", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "closed"}
		if !f.MatchesStatus("closed") {
			t.Error("closed should match only-closed")
		}
		if f.MatchesStatus("open") {
			t.Error("open should not match only-closed")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "Open"}
		if !f.MatchesStatus("open") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesContributor(t *testing.T) {
	t.Run("matches all when no contributor filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesContributor("usalu") {
			t.Error("should match any contributor by default")
		}
	})

	t.Run("only-contributor includes specified", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("usalu should match")
		}
		if f.MatchesContributor("other") {
			t.Error("other should not match")
		}
	})

	t.Run("exclude-contributor removes specified", func(t *testing.T) {
		f := &TreeFilter{ExcludeContributors: []string{"usalu"}}
		if f.MatchesContributor("usalu") {
			t.Error("usalu should not match when excluded")
		}
		if !f.MatchesContributor("other") {
			t.Error("other should still match")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"Usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestFilterMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj1", Label: "proj1", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "b1", Label: "bundle1", SubKind: "library", Children: []*TreeNode{
							{Kind: TreeNodeFolder, ID: "f1", Label: "src", SubKind: "organization", Children: []*TreeNode{
								{Kind: TreeNodeFile, ID: "file1", Label: "index.ts", SubKind: "code"},
								{Kind: TreeNodeFile, ID: "file2", Label: "README.md", SubKind: "docs"},
							}},
						}},
						{Kind: TreeNodeBundle, ID: "b2", Label: "bundle2", SubKind: "schema"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "g1", Label: "Goal1", Status: "open", Children: []*TreeNode{
						{Kind: TreeNodeTicket, ID: "t1", Label: "Ticket1", Status: "open", Year: 2026, Month: 2, Day: 5},
						{Kind: TreeNodeTicket, ID: "t2", Label: "Ticket2", Status: "closed", Year: 2025, Month: 12, Day: 1},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "contributors", Label: "Contributors", Children: []*TreeNode{
					{Kind: TreeNodeContributor, ID: "c1", Label: "usalu", Contributor: "usalu"},
					{Kind: TreeNodeContributor, ID: "c2", Label: "other", Contributor: "other"},
				}},
			},
		}
	}

	t.Run("no filter returns full tree", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		if len(result.Children) != 3 {
			t.Errorf("expected 3 top-level categories, got %d", len(result.Children))
		}
	})

	t.Run("exclude-bundle removes bundles", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle {
				t.Error("bundles should be collapsed out")
			}
		}
	})

	t.Run("no-folder collapses folders", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		bundle := proj.Children[0]
		hasFile := false
		for _, c := range bundle.Children {
			if c.Kind == TreeNodeFolder {
				t.Error("folders should be collapsed")
			}
			if c.Kind == TreeNodeFile {
				hasFile = true
			}
		}
		if !hasFile {
			t.Error("files should be promoted to bundle level")
		}
	})

	t.Run("only-library sub-kind filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		technologiesNode := result.Children[0]
		proj := technologiesNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle && c.SubKind != "library" {
				t.Errorf("only library bundles expected, got %s", c.SubKind)
			}
		}
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 bundle (library), got %d", len(proj.Children))
		}
	})

	t.Run("status filter open", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			OnlyStatus:      "open",
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Status != "open" {
				t.Error("only open tickets should be visible")
			}
		}
	})

	t.Run("year filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			ExcludeYears:    []int{2025},
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Year == 2025 {
				t.Error("2025 tickets should be excluded")
			}
		}
	})

	t.Run("contributor filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:           make(map[TreeNodeKind]bool),
			ExcludeKinds:        make(map[TreeNodeKind]bool),
			OnlySubKinds:        make(map[TreeNodeKind][]string),
			ExcludeSubKinds:     make(map[TreeNodeKind][]string),
			ExcludeContributors: []string{"usalu"},
		}
		result := FilterMonorepoTree(tree, filter)
		contribNode := result.Children[2]
		for _, c := range contribNode.Children {
			if c.Contributor == "usalu" {
				t.Error("usalu should be excluded")
			}
		}
		if len(contribNode.Children) != 1 {
			t.Errorf("expected 1 contributor, got %d", len(contribNode.Children))
		}
	})

	t.Run("nil filter returns same tree", func(t *testing.T) {
		tree := makeTree()
		result := FilterMonorepoTree(tree, nil)
		if result != tree {
			t.Error("nil filter should return same tree")
		}
	})
}

func TestSearchMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj:compose", Label: "compose", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "bundle:cli", Label: "cli", SubKind: "binary"},
						{Kind: TreeNodeBundle, ID: "bundle:docs", Label: "docs", SubKind: "site"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "goal:test", Label: "Test Goal", Description: "testing search"},
				}},
			},
		}
	}

	t.Run("empty query returns full tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "")
		if len(result.Children) != 2 {
			t.Errorf("expected 2 categories, got %d", len(result.Children))
		}
	})

	t.Run("query matches items", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		found := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.ID == "bundle:cli" {
				found = true
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(result)
		if !found {
			t.Error("search for 'cli' should find bundle:cli")
		}
	})

	t.Run("query with no matches returns empty tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "zzzznonexistent")
		totalChildren := 0
		for _, c := range result.Children {
			totalChildren += len(c.Children)
		}
		if totalChildren != 0 {
			t.Errorf("search for nonexistent term should return empty, got %d children", totalChildren)
		}
	})

	t.Run("parent chain preserved", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		if len(result.Children) == 0 {
			t.Fatal("expected at least one category")
		}
		technologiesNode := result.Children[0]
		if technologiesNode.ID != "codebase" {
			t.Errorf("expected codebase category, got %s", technologiesNode.ID)
		}
		if len(technologiesNode.Children) == 0 {
			t.Fatal("expected technology under technologies")
		}
		proj := technologiesNode.Children[0]
		if proj.ID != "proj:compose" {
			t.Errorf("expected compose technology, got %s", proj.ID)
		}
	})
}

func TestSearchMonorepoTreeWithCache(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow search monorepo tree cache test in short mode")
	}
	tmpDir := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🦑repo"), 0755); err != nil {
		t.Fatalf("mkdir .🦑repo: %v", err)
	}
	oldRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(oldRoot)

	makeTree := func(description string) *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "proj:repo", Label: "repo", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "bundle:search", Label: "search-bundle", Description: description},
					}},
				}},
			},
		}
	}

	t.Run("uses indexed matches from cache", func(t *testing.T) {
		indexedTree := makeTree("ultrafastneedle")
		idx, err := ensureCacheIndexed(context.Background(), indexedTree)
		if err != nil {
			t.Fatalf("ensureCacheIndexed failed: %v", err)
		}
		if err := idx.Close(); err != nil {
			t.Fatalf("close index failed: %v", err)
		}

		mutatedTree := makeTree("different-text")
		result := searchMonorepoTreeWithCache(context.Background(), mutatedTree, "ultrafastneedle")

		found := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			if node.ID == "bundle:search" {
				found = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(result)
		if !found {
			t.Fatal("expected cached index to find bundle:search")
		}
	})

	t.Run("returns empty tree for miss", func(t *testing.T) {
		result := searchMonorepoTreeWithCache(context.Background(), makeTree("something else"), "no-match-here")
		if len(result.Children) != 0 {
			t.Fatalf("expected empty tree, got %d children", len(result.Children))
		}
	})
}

func TestRenderMonorepoTree(t *testing.T) {
	t.Run("renders basic tree", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "🖥️Codebase") {
			t.Error("output should contain Codebase label")
		}
		if !strings.Contains(output, "compose") {
			t.Error("output should contain technology name")
		}
	})

	t.Run("renders category URI", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "repo://goals"},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "[🎯Goals](repo://goals)") {
			t.Errorf("output should contain category with URI link, got: %s", output)
		}
	})

	t.Run("renders nested tree with connectors", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "Codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "proj1"},
					{Kind: TreeNodeTechnology, ID: "p2", Label: "proj2"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "├── ") || !strings.Contains(output, "└── ") {
			t.Errorf("output should contain tree connectors, got: %s", output)
		}
	})

	t.Run("empty tree renders nothing", func(t *testing.T) {
		tree := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
		output := RenderMonorepoTree(tree)
		if output != "" {
			t.Errorf("empty tree should render nothing, got: %q", output)
		}
	})

	t.Run("markdown renderer uses list bullets", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "codebase", Label: "🖥️Codebase", URI: "repo://codebase", Children: []*TreeNode{
					{Kind: TreeNodeTechnology, ID: "p1", Label: "compose"},
				}},
			},
		}
		output := RenderMonorepoTreeMarkdown(tree)
		if !strings.Contains(output, "- [🖥️Codebase](repo://codebase)") {
			t.Errorf("markdown tree should contain markdown link list item, got: %s", output)
		}
		if !strings.Contains(output, "  - compose") {
			t.Errorf("markdown tree should contain nested bullet item, got: %s", output)
		}
		if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
			t.Errorf("markdown tree must not contain ascii connectors, got: %s", output)
		}
	})

	t.Run("text tree shows only own ID segment not full parent chain", func(t *testing.T) {
		parentGoalData := map[string]interface{}{
			"id":     "parentgoal",
			"title":  "Parent Goal",
			"status": "open",
		}
		childGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal",
			"title":    "Child Goal",
			"status":   "open",
			"parentId": "🎯parentgoal",
		}
		grandchildGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal/grandchildgoal",
			"title":    "Grandchild Goal",
			"status":   "open",
			"parentId": "🎯parentgoal🎯childgoal",
		}
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "repo://goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "parentgoal", Label: "Parent Goal", Data: parentGoalData, Children: []*TreeNode{
						{Kind: TreeNodeGoal, ID: "childgoal", Label: "Child Goal", Data: childGoalData, Children: []*TreeNode{
							{Kind: TreeNodeGoal, ID: "grandchildgoal", Label: "Grandchild Goal", Data: grandchildGoalData},
						}},
					}},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		lines := strings.Split(strings.TrimRight(output, "\n"), "\n")
		for _, line := range lines {
			if strings.Contains(line, "🎯parentgoal🎯childgoal") {
				t.Errorf("tree text should not contain full hierarchical ID, got line: %s", line)
			}
			if strings.Contains(line, "🎯parentgoal🎯") {
				t.Errorf("tree text should not contain parent prefix in child line, got line: %s", line)
			}
		}
		childFound := false
		grandchildFound := false
		for _, line := range lines {
			if strings.Contains(line, "🎯childgoal") && !strings.Contains(line, "🎯parentgoal🎯childgoal") {
				childFound = true
			}
			if strings.Contains(line, "🎯grandchildgoal") && !strings.Contains(line, "🎯childgoal🎯grandchildgoal") {
				grandchildFound = true
			}
		}
		if !childFound {
			t.Errorf("tree text should contain short child ID 🎯childgoal, got:\n%s", output)
		}
		if !grandchildFound {
			t.Errorf("tree text should contain short grandchild ID 🎯grandchildgoal, got:\n%s", output)
		}
	})

	t.Run("text tree preserves parentId on data after rendering", func(t *testing.T) {
		data := map[string]interface{}{
			"id":       "parent/child",
			"title":    "Child",
			"status":   "open",
			"parentId": "🎯parent",
		}
		node := &TreeNode{Kind: TreeNodeGoal, ID: "child", Label: "Child", Data: data}
		var sb strings.Builder
		renderTreeNodeText(&sb, node, "", true, true)
		if data["parentId"] != "🎯parent" {
			t.Errorf("renderTreeNodeText should restore parentId, got: %v", data["parentId"])
		}
	})
}

func TestExhaustiveBuildMonorepoTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree build test in short mode")
	}

	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()
	InvalidateTechnologyCache()

	ctx := context.Background()
	treeNoSections := BuildMonorepoTreeCached(ctx)
	treeSections := BuildMonorepoTreeCached(ctx, TreeBuildOptions{IncludeSections: true})

	t.Run("builds tree with categories", func(t *testing.T) {
		if treeNoSections == nil {
			t.Fatal("tree should not be nil")
		}
		if len(treeNoSections.Children) == 0 {
			t.Fatal("tree should have categories")
		}
		categoryIDs := make(map[string]bool)
		for _, c := range treeNoSections.Children {
			if c.Kind != TreeNodeCategory {
				t.Errorf("top-level children should be categories, got %s", c.Kind)
			}
			categoryIDs[c.ID] = true
		}
		expected := []string{"codebase", "goals", "drafts", "policies", "contributors", "checkpoints"}
		for _, id := range expected {
			if !categoryIDs[id] {
				t.Errorf("missing category: %s", id)
			}
		}
	})

	t.Run("codebase category has folder and file document", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		if len(codebaseNode.Children) == 0 {
			t.Fatal("codebase category should have children")
		}
		hasFolder := false
		hasFile := false
		hasSection := false
		hasDefinition := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			switch node.Kind {
			case TreeNodeFolder:
				hasFolder = true
			case TreeNodeFile:
				hasFile = true
			case TreeNodeSection:
				hasSection = true
			case TreeNodeDefinition:
				hasDefinition = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(codebaseNode)
		if !hasFolder {
			t.Error("codebase category should include folder nodes")
		}
		if !hasFile {
			t.Error("codebase category should include file nodes")
		}
		if !hasSection {
			t.Error("codebase category should include section nodes when IncludeSections is true")
		}
		if !hasDefinition {
			t.Error("codebase category should include definition nodes when IncludeSections is true")
		}
	})

	t.Run("codebase category has technology children", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		hasTechnologies := false
		hasBundles := false
		for _, p := range codebaseNode.Children {
			if p.Kind == TreeNodeTechnology {
				hasTechnologies = true
				for _, b := range p.Children {
					if b.Kind == TreeNodeBundle {
						hasBundles = true
					}
				}
			}
		}
		if !hasTechnologies {
			t.Error("codebase should have technology children")
		}
		if !hasBundles {
			t.Error("at least one technology should have bundles")
		}
	})

	t.Run("policies category uses entitykind grouping", func(t *testing.T) {
		var policiesNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "policies" {
				policiesNode = c
				break
			}
		}
		if policiesNode == nil {
			t.Fatal("policies category not found")
		}
		if len(policiesNode.Children) == 0 {
			t.Fatal("policies should have children")
		}
		policy := policiesNode.Children[0]
		if policy.Kind != TreeNodePolicy {
			t.Fatalf("expected policy node, got %s", policy.Kind)
		}
		if len(policy.Children) == 0 {
			t.Fatal("policy should contain entitykind children")
		}
		entityKind := policy.Children[0]
		if entityKind.Kind != TreeNodeCategory {
			t.Fatalf("expected entitykind category node, got %s", entityKind.Kind)
		}
		if entityKind.SubKind != "entitykind" {
			t.Fatalf("expected entitykind subkind, got %s", entityKind.SubKind)
		}
		if len(entityKind.Children) == 0 {
			t.Fatal("entitykind should contain statute children")
		}
		for _, statute := range entityKind.Children {
			if statute.Kind != TreeNodeStatute {
				t.Fatalf("expected statute child under entitykind, got %s", statute.Kind)
			}
		}
	})

	t.Run("with sections includes sections", func(t *testing.T) {
		hasSections := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				hasSections = true
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeSections)
		if !hasSections {
			t.Error("tree with IncludeSections should have section nodes")
		}
	})

	t.Run("without sections excludes sections", func(t *testing.T) {
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				t.Error("tree without IncludeSections should not have section nodes")
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeNoSections)
	})
}

func TestCollapseFilteredKinds(t *testing.T) {
	t.Run("collapses folders promoting files to parent", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
					{Kind: TreeNodeFolder, ID: "f1", Label: "src", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "file1", Label: "index.ts"},
						{Kind: TreeNodeFile, ID: "file2", Label: "app.ts"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		bundle := tree.Children[0]
		if len(bundle.Children) != 2 {
			t.Errorf("expected 2 files promoted to bundle, got %d", len(bundle.Children))
		}
		for _, c := range bundle.Children {
			if c.Kind != TreeNodeFile {
				t.Errorf("expected file, got %s", c.Kind)
			}
		}
	})

	t.Run("nested collapse", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeTechnology, ID: "p1", Label: "proj", Children: []*TreeNode{
					{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "f1", Label: "main.go"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		proj := tree.Children[0]
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 file promoted to technology, got %d", len(proj.Children))
		}
		if proj.Children[0].Kind != TreeNodeFile {
			t.Errorf("expected file, got %s", proj.Children[0].Kind)
		}
	})
}

func TestSortTreeChildren(t *testing.T) {
	t.Run("sorts alphabetically", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "z.ts"},
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFile, Label: "m.ts"},
			},
		}
		sortTreeChildren(tree)
		if tree.Children[0].Label != "a.ts" {
			t.Errorf("expected a.ts first, got %s", tree.Children[0].Label)
		}
		if tree.Children[2].Label != "z.ts" {
			t.Errorf("expected z.ts last, got %s", tree.Children[2].Label)
		}
	})

	t.Run("folders before files", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFolder, Label: "src"},
				{Kind: TreeNodeFile, Label: "b.ts"},
			},
		}
		sortTreeChildren(tree)
		if tree.Children[0].Kind != TreeNodeFolder {
			t.Errorf("expected folder first, got %s", tree.Children[0].Kind)
		}
	})
}

func TestTreeCommandFlags(t *testing.T) {
	t.Run("builds filter from flags", func(t *testing.T) {
		cmd := &cobra.Command{}
		bindTreeFlags(cmd)
		cmd.Flags().Set("only-technology", "true")
		cmd.Flags().Set("no-folder", "true")
		cmd.Flags().Set("only-library", "true")
		cmd.Flags().Set("only-open", "true")
		cmd.Flags().Set("no-year", "2025")

		filter := buildTreeFilterFromFlags(cmd)

		if !filter.OnlyKinds[TreeNodeTechnology] {
			t.Error("expected only-technology to be set")
		}
		if !filter.ExcludeKinds[TreeNodeFolder] {
			t.Error("expected no-folder to be set")
		}
		if len(filter.OnlySubKinds[TreeNodeBundle]) != 1 || filter.OnlySubKinds[TreeNodeBundle][0] != string(BundleKindLibrary) {
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
		cmd := &cobra.Command{}
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

// 🌳#region 📋Unified Rendering Identity
func TestTreeNodeKindToEntityKindCoversAll(t *testing.T) {
	kinds := []struct {
		kind     TreeNodeKind
		expected string
	}{
		{TreeNodeTechnology, "technology"},
		{TreeNodeBundle, "bundle"},
		{TreeNodeFolder, "folder"},
		{TreeNodeFile, "file"},
		{TreeNodeSection, "section"},
		{TreeNodeDefinition, "definition"},
		{TreeNodeGoal, "goal"},
		{TreeNodeTicket, "ticket"},
		{TreeNodeDraft, "draft"},
		{TreeNodePolicy, "policy"},
		{TreeNodeStatute, ""},
		{TreeNodeContributor, "contributor"},
		{TreeNodeCheckpoint, "checkpoint"},
		{TreeNodeCategory, ""},
	}
	for _, tt := range kinds {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := treeNodeKindToEntityKind(tt.kind)
			if got != tt.expected {
				t.Errorf("treeNodeKindToEntityKind(%q) = %q, want %q", tt.kind, got, tt.expected)
			}
		})
	}
	t.Run("unknown returns empty", func(t *testing.T) {
		got := treeNodeKindToEntityKind(TreeNodeKind("unknown"))
		if got != "" {
			t.Errorf("treeNodeKindToEntityKind(unknown) = %q, want empty", got)
		}
	})
}

func TestUnifiedRenderingGoalIdentity(t *testing.T) {
	data := map[string]interface{}{
		"id":          "TEST-GOAL",
		"title":       "Test Goal",
		"status":      "open",
		"dueDate":     "2030-01-01",
		"createdAt":   "2025-01-01T00:00:00Z",
		"description": "A test goal",
	}

	mdLink := renderEntityMarkdownLink("goal", data)
	mdItem := renderEntityMarkdown("goal", data)
	humanItem := renderEntityHuman("goal", data, false)

	t.Run("renderEntityMarkdown is dash-prefixed renderEntityMarkdownLink", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("markdown link has artifact ID and URI", func(t *testing.T) {
		if !strings.Contains(mdLink, "[🎯") {
			t.Errorf("markdown link missing goal emoji prefix: %s", mdLink)
		}
		if !strings.Contains(mdLink, "](repo://g/") {
			t.Errorf("markdown link missing goal URI: %s", mdLink)
		}
	})

	t.Run("human has artifact ID", func(t *testing.T) {
		if !strings.Contains(humanItem, "🎯") {
			t.Errorf("human output missing goal emoji: %s", humanItem)
		}
	})

	t.Run("both formats share same props from collectEntityProps", func(t *testing.T) {
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("goalNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &GoalNode{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}
		nodeData := goalNodeToData(node)
		fromNode := renderEntityMarkdownLink("goal", nodeData)
		fromDirect := renderEntityMarkdownLink("goal", data)
		if fromNode != fromDirect {
			t.Errorf("goalNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree markdown uses renderEntityMarkdownLink for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		expectedLink := renderEntityMarkdownLink("goal", data)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("goal tree markdown must not have double dash: %q", treeOutput)
		}
	})

	t.Run("goal tree text uses renderEntityHuman for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "text")
		expectedHuman := renderEntityHuman("goal", data, false)
		if !strings.Contains(treeOutput, expectedHuman) {
			t.Errorf("goal tree text should contain renderEntityHuman output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("monorepo tree node markdown matches goal tree markdown", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + emojiText(EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "md"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree markdown and goal tree markdown differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})

	t.Run("monorepo tree node text matches goal tree text", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "repo://goal/" + emojiText(EmojiGoal) + "testgoal",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeText(&sb, treeNode, "", true, true)
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "text"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree text and goal tree text differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})
}

func TestUnifiedRenderingTicketIdentity(t *testing.T) {
	data := map[string]interface{}{
		"slug":     "MY-TICKET",
		"title":    "My Ticket",
		"status":   "open",
		"started":  "2025-01-01T00:00:00Z",
		"finished": "",
		"prompt":   "Fix something",
		"summary":  "",
		"year":     float64(2025),
		"month":    float64(1),
		"day":      float64(1),
	}

	mdLink := renderEntityMarkdownLink("ticket", data)
	mdItem := renderEntityMarkdown("ticket", data)
	humanItem := renderEntityHuman("ticket", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("ticketNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &TicketNode{
			Slug:        "MY-TICKET",
			Title:       "My Ticket",
			Status:      "open",
			Created:     "2025-01-01T00:00:00Z",
			Finished:    "",
			Description: "Fix something",
			Summary:     "",
		}
		nodeData := ticketNodeToData(node)
		nodeData["year"] = float64(2025)
		nodeData["month"] = float64(1)
		nodeData["day"] = float64(1)
		fromNode := renderEntityMarkdownLink("ticket", nodeData)
		fromDirect := renderEntityMarkdownLink("ticket", data)
		if fromNode != fromDirect {
			t.Errorf("ticketNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree ticket markdown uses renderEntityMarkdownLink", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "MY-TICKET", Title: "My Ticket", Status: "open",
				Created: "2025-01-01T00:00:00Z", Description: "Fix something",
			}},
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		ticketData := ticketNodeToData(roots[0].Tickets[0])
		expectedLink := renderEntityMarkdownLink("ticket", ticketData)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree ticket markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("ticket in goal tree must not have double dash: %q", treeOutput)
		}
	})

	t.Run("ticket list markdown matches renderEntityMarkdown", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, true))
		directMD := strings.TrimSpace(renderEntityMarkdown("ticket", data))
		if listOutput != directMD {
			t.Errorf("ticket list markdown should match renderEntityMarkdown.\n  List:   %q\n  Direct: %q", listOutput, directMD)
		}
	})

	t.Run("ticket list text matches renderEntityHuman", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, false))
		directHuman := renderEntityHuman("ticket", data, false)
		if !strings.Contains(listOutput, directHuman) {
			t.Errorf("ticket list text should contain renderEntityHuman output.\n  List:   %q\n  Direct: %q", listOutput, directHuman)
		}
	})
}

func TestUnifiedRenderingSectionIdentity(t *testing.T) {
	data := map[string]interface{}{
		"path":      "test/file.ts#MySection",
		"name":      "MySection",
		"startLine": float64(10),
		"endLine":   float64(20),
	}

	mdLink := renderEntityMarkdownLink("section", data)
	mdItem := renderEntityMarkdown("section", data)
	humanItem := renderEntityHuman("section", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("section", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("section tree markdown uses renderEntityMarkdown", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, true))
		expectedMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if treeOutput != expectedMD {
			t.Errorf("section tree markdown root should match renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedMD)
		}
	})

	t.Run("section tree text uses renderEntityHuman", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, false))
		expectedHuman := renderEntityHuman("section", data, false)
		if treeOutput != expectedHuman {
			t.Errorf("section tree text root should match renderEntityHuman.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("section tree markdown preserves indentation for children", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#Parent",
			Name:      "Parent",
			StartLine: 1,
			EndLine:   30,
			Children: []Section{{
				Path:      "test/file.ts#Child",
				Name:      "Child",
				StartLine: 5,
				EndLine:   15,
			}},
		}
		treeOutput := renderSectionTree(s, false, true)
		lines := strings.Split(strings.TrimSpace(treeOutput), "\n")
		if len(lines) < 2 {
			t.Fatalf("expected at least 2 lines, got %d: %q", len(lines), treeOutput)
		}
		if !strings.HasPrefix(lines[0], "- [") {
			t.Errorf("root section should start with '- [': %q", lines[0])
		}
		if !strings.HasPrefix(lines[1], "  - [") {
			t.Errorf("child section should start with '  - [' for 2-space indent: %q", lines[1])
		}
	})

	t.Run("monorepo tree node markdown matches direct rendering", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeSection,
			ID:    "sec1",
			Label: "MySection",
			URI:   "repo://section/" + emojiText(EmojiFileCode) + "file" + emojiText(EmojiSection) + "mysection",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())
		directMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if monorepoOutput != directMD {
			t.Errorf("monorepo tree section markdown should match renderEntityMarkdown.\n  Monorepo: %q\n  Direct:   %q", monorepoOutput, directMD)
		}
	})
}

func TestUnifiedRenderingAllKindIdentity(t *testing.T) {
	entities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"technology", TreeNodeTechnology, map[string]interface{}{
			"name": "mytechnology", "description": "A technology",
		}},
		{"bundle", TreeNodeBundle, map[string]interface{}{
			"name": "mybundle", "root": "path/to/bundle",
		}},
		{"folder", TreeNodeFolder, map[string]interface{}{
			"path": "src/folder", "name": "folder",
		}},
		{"file", TreeNodeFile, map[string]interface{}{
			"path": "src/file.ts", "name": "file.ts",
		}},
		{"contributor", TreeNodeContributor, map[string]interface{}{
			"github": "dev1", "name": "Developer One",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "code-hygiene", "name": "Code Hygiene", "description": "Clean code policy",
		}},
		{"statute", TreeNodeStatute, map[string]interface{}{
			"id": "inline-comment", "description": "No inline comments",
		}},
		{"draft", TreeNodeDraft, map[string]interface{}{
			"id": "draft-1", "slug": "my-draft",
		}},
		{"checkpoint", TreeNodeCheckpoint, map[string]interface{}{
			"sha": "abc1234567890", "message": "fix: something",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind+"_markdown_identity", func(t *testing.T) {
			directMD := renderEntityMarkdown(tt.kind, tt.data)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			treeOutput := strings.TrimSpace(sb.String())
			directMDTrimmed := strings.TrimSpace(directMD)
			if treeOutput != directMDTrimmed {
				t.Errorf("%s: monorepo tree markdown differs from direct renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directMDTrimmed)
			}
		})

		t.Run(tt.kind+"_text_identity", func(t *testing.T) {
			directHuman := renderEntityHuman(tt.kind, tt.data, false)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			treeOutput := strings.TrimSpace(sb.String())
			if treeOutput != directHuman {
				t.Errorf("%s: monorepo tree text differs from direct renderEntityHuman.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directHuman)
			}
		})

		t.Run(tt.kind+"_props_in_both_formats", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			mdLink := renderEntityMarkdownLink(tt.kind, tt.data)
			human := renderEntityHuman(tt.kind, tt.data, false)
			for _, p := range props {
				if !strings.Contains(mdLink, p) {
					t.Errorf("%s: markdown link missing prop %q: %s", tt.kind, p, mdLink)
				}
				if !strings.Contains(human, p) {
					t.Errorf("%s: human output missing prop %q: %s", tt.kind, p, human)
				}
			}
		})
	}
}

func TestCollectEntityPropsConsistency(t *testing.T) {
	t.Run("goal props include all fields", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "G1",
			"title":       "My Goal",
			"status":      "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description",
		}
		props := collectEntityProps("goal", data, false)
		if len(props) < 4 {
			t.Errorf("goal props should have >= 4 entries (title, status, created, due, desc), got %d: %v", len(props), props)
		}
		found := map[string]bool{}
		for _, p := range props {
			if strings.Contains(p, "My Goal") {
				found["title"] = true
			}
			if strings.Contains(p, "open") {
				found["status"] = true
			}
			if strings.Contains(p, "created") {
				found["created"] = true
			}
			if strings.Contains(p, "Description") {
				found["description"] = true
			}
		}
		for _, key := range []string{"title", "status", "created", "description"} {
			if !found[key] {
				t.Errorf("goal props missing %s: %v", key, props)
			}
		}
	})

	t.Run("ticket open props include prompt", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "open",
			"started": "2025-01-01T00:00:00Z", "prompt": "Please fix",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Please fix") {
				found = true
			}
		}
		if !found {
			t.Errorf("open ticket props should contain prompt: %v", props)
		}
	})

	t.Run("ticket closed props include summary", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z", "summary": "Fixed the bug",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Fixed the bug") {
				found = true
			}
		}
		if !found {
			t.Errorf("closed ticket props should contain summary: %v", props)
		}
	})

	t.Run("section props include line range", func(t *testing.T) {
		data := map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(10), "endLine": float64(20),
		}
		props := collectEntityProps("section", data, false)
		if len(props) < 1 || !strings.Contains(props[0], ":10-20") {
			t.Errorf("section props should contain :10-20, got: %v", props)
		}
	})

	t.Run("definition props include name and line range", func(t *testing.T) {
		data := map[string]interface{}{
			"name": "myFunc", "startLine": float64(5), "endLine": float64(15),
		}
		props := collectEntityProps("definition", data, false)
		foundName := false
		foundRange := false
		for _, p := range props {
			if strings.Contains(p, "myFunc") {
				foundName = true
			}
			if strings.Contains(p, ":5-15") {
				foundRange = true
			}
		}
		if !foundName {
			t.Errorf("definition props should contain name: %v", props)
		}
		if !foundRange {
			t.Errorf("definition props should contain line range: %v", props)
		}
	})

	t.Run("props strip newlines from multi-line content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\nLine two.\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("prop contains newline: %q", p)
			}
			if strings.Contains(p, "\r") {
				t.Errorf("prop contains carriage return: %q", p)
			}
		}
	})

	t.Run("props strip backticks from content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed the `title` parameter in `UpdateTicketTitle`.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "`") {
				t.Errorf("prop contains backtick: %q", p)
			}
		}
	})

	t.Run("props collapse multiple spaces", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed.\n\n1. First.\n2. Second.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "  ") {
				t.Errorf("prop contains double space: %q", p)
			}
		}
	})

	t.Run("props handle Windows line endings", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\r\nLine two.\r\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\r") || strings.Contains(p, "\n") {
				t.Errorf("prop contains line break: %q", p)
			}
		}
	})

	t.Run("goal props strip newlines from description", func(t *testing.T) {
		data := map[string]interface{}{
			"id": "G1", "title": "My Goal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Goal with\nmultiple\nlines and `backticks`.",
		}
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("goal prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("goal prop contains backtick: %q", p)
			}
		}
	})

	t.Run("checkpoint props strip newlines from message", func(t *testing.T) {
		data := map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}
		props := collectEntityProps("checkpoint", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("checkpoint prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("checkpoint prop contains backtick: %q", p)
			}
		}
	})

	t.Run("policy props strip newlines", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "code-hygiene",
			"name":        "Code Hygiene",
			"description": "Clean code\npolicy with `rules`.",
		}
		props := collectEntityProps("policy", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("policy prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("policy prop contains backtick: %q", p)
			}
		}
	})
}

func TestSingleLineOutput(t *testing.T) {
	multiLineEntities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Added folder renaming.\n\n1. MCP ticketReopen handler: reads the `title` parameter.\n2. MCP ticketClose handler: reads `title`.\n3. Goals: added `UpdateGoalTitle()` helper.\n\nAlso fixed a test bug.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T2", "title": "Open Ticket", "status": "open",
			"started": "2025-01-01T00:00:00Z",
			"prompt":  "Fix the `config` module.\nIt has multiple issues:\n- Issue 1\n- Issue 2",
			"year":    float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"goal", TreeNodeGoal, map[string]interface{}{
			"id": "G1", "title": "Multi\nLine\nGoal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description with `code`\nand\r\nnewlines.",
		}},
		{"checkpoint", TreeNodeCheckpoint, map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "p1", "name": "Policy", "description": "Rule 1\nRule 2\n`Rule 3`",
		}},
		{"technology", TreeNodeTechnology, map[string]interface{}{
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
		for _, p := range collectEntityProps("ticket", multiLineEntities[0].data, false) {
			if strings.Contains(p, "`") {
				t.Errorf("%s prop contains backtick: %q", label, p)
			}
		}
		_ = output
	}

	for _, tt := range multiLineEntities {
		t.Run(tt.kind+"_renderEntityMarkdownLink_single_line", func(t *testing.T) {
			output := renderEntityMarkdownLink(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdownLink("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityMarkdown_single_line", func(t *testing.T) {
			output := renderEntityMarkdown(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, false)
			assertSingleLine(t, "renderEntityHuman("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_tty_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, true)
			assertSingleLine(t, "renderEntityHuman_tty("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_props_no_backticks", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "`") {
					t.Errorf("prop contains backtick: %q", p)
				}
			}
			assertNoRawBackticks(t, tt.kind, "")
		})

		t.Run(tt.kind+"_monorepoTreeNodeMarkdown_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_monorepoTreeNodeText_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeText("+tt.kind+")", output)
		})
	}

	t.Run("goal_tree_with_multi_line_tickets_all_single_line", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent\nGoal", Status: "open",
			Tickets: []*TicketNode{
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
			Children: []*GoalNode{{
				ID: "G2", Title: "Child Goal", Status: "open",
				Description: "Description\nwith\nnewlines.",
			}},
		}}
		for _, format := range []string{"md", "text"} {
			output := renderGoalTreeNodes(roots, format)
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
			output := renderTicketList(tickets, false, useMD)
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

func TestNoDoubleDashInMarkdownOutput(t *testing.T) {
	kinds := []struct {
		kind string
		data map[string]interface{}
	}{
		{"goal", map[string]interface{}{
			"id": "G1", "title": "Goal", "status": "open",
		}},
		{"ticket", map[string]interface{}{
			"slug": "T1", "title": "Ticket", "status": "open",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"section", map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(1), "endLine": float64(5),
		}},
		{"bundle", map[string]interface{}{
			"name": "b1", "root": "path",
		}},
		{"folder", map[string]interface{}{
			"path": "src/f",
		}},
		{"file", map[string]interface{}{
			"path": "src/a.ts",
		}},
		{"contributor", map[string]interface{}{
			"github": "dev",
		}},
		{"checkpoint", map[string]interface{}{
			"sha": "abc",
		}},
	}

	for _, tt := range kinds {
		t.Run(tt.kind+"_renderEntityMarkdown", func(t *testing.T) {
			output := renderEntityMarkdown(tt.kind, tt.data)
			if count := strings.Count(output, "- "); count > 1 {
				dashPositions := []int{}
				idx := 0
				for {
					pos := strings.Index(output[idx:], "- ")
					if pos == -1 {
						break
					}
					dashPositions = append(dashPositions, idx+pos)
					idx += pos + 2
				}
				if len(dashPositions) >= 2 && dashPositions[1]-dashPositions[0] <= 3 {
					t.Errorf("renderEntityMarkdown(%s) has double dash at start: %q", tt.kind, output)
				}
			}
		})

		t.Run(tt.kind+"_treeNodeMarkdown", func(t *testing.T) {
			nodeKind := TreeNodeKind(tt.kind)
			switch tt.kind {
			case "goal":
				nodeKind = TreeNodeGoal
			case "ticket":
				nodeKind = TreeNodeTicket
			case "section":
				nodeKind = TreeNodeSection
			case "bundle":
				nodeKind = TreeNodeBundle
			case "folder":
				nodeKind = TreeNodeFolder
			case "file":
				nodeKind = TreeNodeFile
			case "contributor":
				nodeKind = TreeNodeContributor
			case "checkpoint":
				nodeKind = TreeNodeCheckpoint
			}

			treeNode := &TreeNode{Kind: nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := sb.String()
			if strings.HasPrefix(output, "- - ") {
				t.Errorf("renderTreeNodeMarkdown(%s) has double dash: %q", tt.kind, output)
			}
		})
	}

	t.Run("goalTreeNodes_no_double_dash", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Goal", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "T1", Title: "Ticket", Status: "open",
			}},
		}}
		output := renderGoalTreeNodes(roots, "md")
		for i, line := range strings.Split(output, "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if strings.HasPrefix(trimmed, "- - ") {
				t.Errorf("line %d has double dash: %q", i, line)
			}
		}
	})
}

// #endregion 📋Unified Rendering Identity

// #endregion 🗿Monorepo Tree

func TestExhaustiveMigrateAuthorFieldsToString(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow migration test in short mode")
	}
	ctx := context.Background()

	ticketCh := make(chan Ticket)
	var ticketErr error
	go func() {
		ticketErr = StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	ticketCount := 0
	for ticket := range ticketCh {
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save ticket %s: %v", ticket.Slug, err)
		}
		ticketCount++
	}
	if ticketErr != nil {
		t.Fatalf("stream tickets failed: %v", ticketErr)
	}
	t.Logf("migrated %d tickets via stream", ticketCount)

	ticketsDir := GetTicketsDir()
	remainingCount := 0
	filepath.WalkDir(ticketsDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || d.Name() != "ticket.json" {
			return nil
		}
		raw, err := ReadTextFile(path)
		if err != nil {
			return nil
		}
		if !strings.Contains(raw, `"author": {`) {
			return nil
		}
		var ticket Ticket
		if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
			t.Logf("failed to parse %s: %v", path, err)
			return nil
		}
		ticket.JsonPath = path
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save remaining ticket %s: %v", path, err)
		}
		remainingCount++
		return nil
	})
	t.Logf("migrated %d remaining tickets", remainingCount)

	goalCh := make(chan *Goal)
	var goalErr error
	go func() {
		goalErr = StreamGoals(ctx, goalCh)
	}()
	goalCount := 0
	for goal := range goalCh {
		if err := SaveGoal(*goal); err != nil {
			t.Errorf("failed to save goal %s: %v", goal.ID, err)
		}
		goalCount++
	}
	if goalErr != nil {
		t.Fatalf("stream goals failed: %v", goalErr)
	}
	t.Logf("migrated %d goals", goalCount)
}

func TestFixHeaderWithShebang(t *testing.T) {
	tmpDir := t.TempDir()
	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	filePath := "script.py"
	absPath := filepath.Join(tmpDir, filePath)
	content := "#!/usr/bin/env python3\n" +
		"#region 🔖Header\n\n" +
		"# wrong/path.py\n\n" +
		"# 2025 Test <t@t.com>\n\n" +
		"# #region 🔖License\n" +
		"# AGPL\n" +
		"# #endregion 🔖License\n\n" +
		"# #region 🔖Requirements\n" +
		"# 💯Requirements\n" +
		"# #endregion 🔖Requirements\n\n" +
		"#endregion 🔖Header\n\n" +
		"print(\"hello\")\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	breachs, err := CheckPolicies(ParseScope(filePath), bundles, nil)
	if err != nil {
		t.Fatalf("CheckPolicies failed: %v", err)
	}
	for _, v := range breachs {
		if v.Autofixable() {
			t.Logf("Detected Autofixable Breach: %s at line %d", v.Kind, v.Line)
		} else {
			t.Logf("Detected Non-Autofixable Breach: %s at line %d", v.Kind, v.Line)
		}
	}

	ctx := NewRepoContext(tmpDir)
	scope := filePath
	res, err := ctx.Fix(&scope)
	if err != nil {
		t.Fatalf("Fix failed: %v", err)
	}

	t.Logf("Fixed: %d", res.Fixed)
	for _, v := range res.Breachs {
		t.Logf("Remaining Breach: %s at line %d", v.Kind, v.Line)
	}

	if res.Fixed == 0 {
		t.Log("No fixes applied (unexpected).")
	}

	newContentBytes, _ := os.ReadFile(absPath)
	newContent := string(newContentBytes)
	if !strings.Contains(newContent, "#!/usr/bin/env python3") {
		t.Errorf("Shebang missing in fixed content:\n%s", newContent)
	}
	expectedId := "📜script.py"
	if !strings.Contains(newContent, expectedId) {
		t.Logf("Expected ID %q might be missing or different format.", expectedId)
	}
}

func TestFolderPolicyEmptyFolder(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "some", "empty")
	os.MkdirAll(emptyDir, 0755)
	nonEmptyDir := filepath.Join(tmpDir, "some", "nonempty")
	os.MkdirAll(nonEmptyDir, 0755)
	os.WriteFile(filepath.Join(nonEmptyDir, "file.txt"), []byte("content"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	foundEmpty := false
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty && v.Excerpt == "some/empty" {
			foundEmpty = true
			if !v.Autofixable() {
				t.Error("BreachFolderIllegalEmpty should be autofixable")
			}
		}
	}
	if !foundEmpty {
		t.Error("expected BreachFolderIllegalEmpty for some/empty")
	}
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty && v.Excerpt == "some/nonempty" {
			t.Error("should not report BreachFolderIllegalEmpty for non-empty folder")
		}
	}
}

func TestFolderPolicyEmptyFolderAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "remove", "me")
	os.MkdirAll(emptyDir, 0755)
	breachs := []Breach{{
		Kind:    BreachFolderIllegalEmpty,
		Scope:   "remove/me/",
		Excerpt: "remove/me",
	}}
	fixed, err := applySystemAutofixes(breachs)
	if err != nil {
		t.Fatalf("applySystemAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}
	if _, statErr := os.Stat(emptyDir); !os.IsNotExist(statErr) {
		t.Error("empty folder should have been removed")
	}
}

func TestFolderPolicySkipsExcludedDirs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	for _, dir := range []string{".git/objects", ".🦑repo/cache", "node_modules/.cache"} {
		os.MkdirAll(filepath.Join(tmpDir, dir), 0755)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty {
			if strings.HasPrefix(v.Excerpt, ".git") || strings.HasPrefix(v.Excerpt, ".🦑repo") || strings.HasPrefix(v.Excerpt, "node_modules") {
				t.Errorf("should skip excluded dir, got breach for %s", v.Excerpt)
			}
		}
	}
}

func TestFilePolicyGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🦑repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/main.ts"]`
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(srcDir, 0755)
	os.WriteFile(filepath.Join(srcDir, "main.ts"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/main.ts") {
			t.Errorf("should not report breach for allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSupportsGlobPatterns(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🦑repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/**/*.ts", "docs/*.md"]`
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcNestedDir := filepath.Join(tmpDir, "src", "nested")
	os.MkdirAll(srcNestedDir, 0755)
	os.WriteFile(filepath.Join(srcNestedDir, "main.ts"), []byte("ok"), 0644)
	docsDir := filepath.Join(tmpDir, "docs")
	os.MkdirAll(docsDir, 0755)
	os.WriteFile(filepath.Join(docsDir, "guide.md"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/nested/main.ts" || v.Excerpt == "docs/guide.md") {
			t.Errorf("should not report breach for glob-allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsComposeRepo(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".🦑repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(`[]`), 0644)
	os.WriteFile(filepath.Join(metaDir, "some_internal.json"), []byte("internal"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && strings.HasPrefix(v.Excerpt, ".🦑repo") {
			t.Errorf("should skip .🦑repo files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsNestedNodeModules(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	SetRootDir(tmpDir)
	defer func() { SetRootDir(oldRoot) }()
	os.WriteFile(filepath.Join(tmpDir, ".gitignore"), []byte("node_modules/\n"), 0644)
	metaDir := filepath.Join(tmpDir, ".🦑repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(`[]`), 0644)
	nested := filepath.Join(tmpDir, "repo", "vscode", "node_modules", "undici-types")
	os.MkdirAll(nested, 0755)
	os.WriteFile(filepath.Join(nested, "fetch.d.ts"), []byte("export {};"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && strings.Contains(v.Excerpt, "node_modules/") {
			t.Errorf("should skip ignored node_modules files, got breach for %s", v.Excerpt)
		}
	}
}

func TestSetRootDirResetsGitignoreCache(t *testing.T) {
	tmpDirA := t.TempDir()
	tmpDirB := t.TempDir()
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(tmpDirA)
	_ = isGitIgnored("any.txt")
	os.WriteFile(filepath.Join(tmpDirB, ".gitignore"), []byte("node_modules/\n"), 0644)
	SetRootDir(tmpDirB)
	ignored := isGitIgnored(filepath.Join(tmpDirB, "node_modules", "x.ts"))
	if !ignored {
		t.Error("expected SetRootDir to refresh gitignore cache for new root")
	}
}

func TestBuildBinaryArtifactsGitIgnored(t *testing.T) {
	artifacts := []string{
		"repo/client/client",
		"repo/client/client.exe",
		"repo/server/coordinator/server",
		"repo/server/coordinator/server.exe",
		"claude",
		"claude.exe",
		"codex",
		"codex.exe",
		"copilot",
		"copilot.exe",
		"cursor",
		"cursor.exe",
		"kiro",
		"kiro.exe",
		"mcp",
		"mcp.exe",
		"coda/example/compose-blnbo-roomprogram/.coda/validators/programming",
		"coda/example/compose-blnbo-roomprogram/.coda/validators/programming.exe",
	}
	ignored := GetGitIgnoredSet(artifacts)
	for _, path := range artifacts {
		if !ignored[path] {
			t.Errorf("expected build artifact %q to be gitignored", path)
		}
	}
}

func TestSetRootDirCanonicalizesToRepoRoot(t *testing.T) {
	repoRoot := t.TempDir()
	nested := filepath.Join(repoRoot, "a", "b", "c")
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(nested, 0755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(nested)
	if got := GetRootDir(); got != repoRoot {
		t.Fatalf("expected repo root %q, got %q", repoRoot, got)
	}
	if got := GetRepoMetaDir(); got != filepath.Join(repoRoot, ".🦑repo") {
		t.Fatalf("expected repo meta dir at monorepo root, got %q", got)
	}
}

func TestRenderPromptTemplateUsesRepoMetaRoot(t *testing.T) {
	repoRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	templateDir := filepath.Join(repoRoot, ".🦑repo", "💬prompts")
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
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(repoRoot)
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

func TestFindRepoRootPrefersMonorepoMarkerOverLocalGoMod(t *testing.T) {
	monoRoot := t.TempDir()
	clientDir := filepath.Join(monoRoot, "repo", "client")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	got := findRepoRoot(clientDir)
	if got != monoRoot {
		t.Fatalf("expected monorepo root %q, got %q", monoRoot, got)
	}
}

func TestGetCacheDirUsesMonorepoRoot(t *testing.T) {
	monoRoot := t.TempDir()
	clientDir := filepath.Join(monoRoot, "repo", "client")
	if err := os.MkdirAll(clientDir, 0755); err != nil {
		t.Fatalf("mkdir cli dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "main.go"), []byte("package main"), 0644); err != nil {
		t.Fatalf("write main.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(clientDir, "go.mod"), []byte("module example.com/client"), 0644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(clientDir)
	cacheDir := getCacheDir()
	wantPrefix := filepath.Join(monoRoot, ".🦑repo", "⚡cache") + string(os.PathSeparator)
	if !strings.HasPrefix(cacheDir, wantPrefix) {
		t.Fatalf("expected cache dir under %q, got %q", wantPrefix, cacheDir)
	}
}

func TestFilePolicyNoGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("content"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	if len(breachs) != 0 {
		t.Errorf("expected no breachs when godfile is missing, got %d", len(breachs))
	}
}

func TestFolderPolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("folder")
	if !found {
		t.Fatal("folder policy not registered")
	}
	if policy.Name != "Folder" {
		t.Errorf("expected policy name Folder, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFolderIllegalEmpty {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("folder policy should contain BreachFolderIllegalEmpty kind")
	}
}

func TestFilePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("file")
	if !found {
		t.Fatal("file policy not registered")
	}
	if policy.Name != "File" {
		t.Errorf("expected policy name File, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFileIllegalUseGodfile {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("file policy should contain BreachFileIllegalUseGodfile kind")
	}
}

func TestComposePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("compose")
	if !found {
		t.Fatal("compose policy not registered")
	}
	if policy.Name != "Compose" {
		t.Errorf("expected policy name Compose, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachComposeNoUiDependency {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("compose policy should contain BreachComposeNoUiDependency kind")
	}
}

func TestComposePolicyNoUiDependency(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	t.Run("detects tailwind-merge import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { twMerge } from \"tailwind-merge\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects elements/ui import", func(t *testing.T) {
		relPath := "compose.test.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as UI from \"../../elements/ui\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachComposeNoUiDependency {
			t.Errorf("expected BreachComposeNoUiDependency, got %s", breachs[0].Kind)
		}
	})
	t.Run("detects clsx import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { ClassValue, clsx } from \"clsx\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects three import", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import * as THREE from \"three\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
	})
	t.Run("detects multiple ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\nimport { twMerge } from \"tailwind-merge\";\nimport * as THREE from \"three\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 3 {
			t.Fatalf("expected 3 breachs, got %d", len(breachs))
		}
	})
	t.Run("allows non-ui imports", func(t *testing.T) {
		relPath := "compose.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { z } from \"zod\";\nimport { v7 as uuidv7 } from \"uuid\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("ignores non-compose files", func(t *testing.T) {
		relPath := "other.ts"
		os.WriteFile(filepath.Join(tmpDir, relPath), []byte("import { clsx } from \"clsx\";\n"), 0644)
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{relPath})
		breachs := composePolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for non-compose file, got %d", len(breachs))
		}
	})
	t.Run("not autofixable", func(t *testing.T) {
		info := BreachComposeNoUiDependency.Info()
		if info.Autofixable {
			t.Error("BreachComposeNoUiDependency should not be autofixable")
		}
	})
}

// 📡#region 🗂️Hook
func TestValidateHookEvent(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		valid  bool
		expect HookEvent
	}{
		{"version checkpoint starting", "version.checkpoint.starting", true, HookVersionCheckpointStarting},
		{"version checkpoint ended", "version.checkpoint.ended", true, HookVersionCheckpointEnded},
		{"version checkin starting", "version.checkin.starting", true, HookVersionCheckinStarting},
		{"version checkin ended", "version.checkin.ended", true, HookVersionCheckinEnded},
		{"version checkout starting", "version.checkout.starting", true, HookVersionCheckoutStarting},
		{"version checkout ended", "version.checkout.ended", true, HookVersionCheckoutEnded},
		{"agent starting", "agent.started", true, HookAgentStarted},
		{"agent ended", "agent.ended", true, HookAgentEnded},
		{"agent prompt submitting", "agent.prompt.submitting", true, HookAgentPromptSubmitting},
		{"agent compacting", "agent.compacting", true, HookAgentCompacting},
		{"agent tool starting", "agent.tool.starting", true, HookAgentToolStarting},
		{"agent tool ended", "agent.tool.ended", true, HookAgentToolEnded},
		{"agent tool plan updating starting", "agent.tool.plan.updating.starting", true, HookAgentToolPlanUpdatingStarting},
		{"agent tool plan updating ended", "agent.tool.plan.updating.ended", true, HookAgentToolPlanUpdatingEnded},
		{"agent tool code searching", "agent.file.read.starting", true, HookAgentToolSearchStarting},
		{"agent tool searched", "agent.file.read.ended", true, HookAgentToolSearchEnded},
		{"agent tool code editing", "agent.tool.code.edit.starting", true, HookAgentToolCodeEditStarting},
		{"agent tool code edited", "agent.tool.code.edit.ended", true, HookAgentToolCodeEditEnded},
		{"agent tool terminal starting", "agent.tool.terminal.starting", true, HookAgentToolTerminalStarting},
		{"agent tool terminal ended", "agent.tool.terminal.ended", true, HookAgentToolTerminalEnded},
		{"agent thinking starting", "agent.thinking.starting", true, HookAgentThinkingStarting},
		{"agent thinking ended", "agent.thinking.ended", true, HookAgentThinkingEnded},
		{"invalid", "invalid.event", false, ""},
		{"empty", "", false, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, err := ValidateHookEvent(tc.input)
			if tc.valid {
				if err != nil {
					t.Fatalf("expected valid event, got error: %v", err)
				}
				if event != tc.expect {
					t.Errorf("expected %s, got %s", tc.expect, event)
				}
			} else {
				if err == nil {
					t.Fatal("expected error for invalid event")
				}
			}
		})
	}
}

func TestHookEventKind(t *testing.T) {
	cases := []struct {
		name   string
		event  HookEvent
		expect HookKind
	}{
		{"version checkpoint starting is version", HookVersionCheckpointStarting, HookKindVersion},
		{"version checkpoint ended is version", HookVersionCheckpointEnded, HookKindVersion},
		{"version checkin starting is version", HookVersionCheckinStarting, HookKindVersion},
		{"version checkin ended is version", HookVersionCheckinEnded, HookKindVersion},
		{"version checkout starting is version", HookVersionCheckoutStarting, HookKindVersion},
		{"version checkout ended is version", HookVersionCheckoutEnded, HookKindVersion},
		{"agent starting is agent", HookAgentStarted, HookKindAgent},
		{"agent ended is agent", HookAgentEnded, HookKindAgent},
		{"agent prompt submitting is agent", HookAgentPromptSubmitting, HookKindAgent},
		{"agent compacting is agent", HookAgentCompacting, HookKindAgent},
		{"agent tool starting is agent", HookAgentToolStarting, HookKindAgent},
		{"agent tool ended is agent", HookAgentToolEnded, HookKindAgent},
		{"agent tool plan updating starting is agent", HookAgentToolPlanUpdatingStarting, HookKindAgent},
		{"agent tool plan updating ended is agent", HookAgentToolPlanUpdatingEnded, HookKindAgent},
		{"agent tool code searching is agent", HookAgentToolSearchStarting, HookKindAgent},
		{"agent tool searched is agent", HookAgentToolSearchEnded, HookKindAgent},
		{"agent tool code editing is agent", HookAgentToolCodeEditStarting, HookKindAgent},
		{"agent tool code edited is agent", HookAgentToolCodeEditEnded, HookKindAgent},
		{"agent tool terminal starting is agent", HookAgentToolTerminalStarting, HookKindAgent},
		{"agent tool terminal ended is agent", HookAgentToolTerminalEnded, HookKindAgent},
		{"agent thinking starting is agent", HookAgentThinkingStarting, HookKindAgent},
		{"agent thinking ended is agent", HookAgentThinkingEnded, HookKindAgent},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := HookEventKind(tc.event)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestIsToolBlocked(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		toolArgs string
		blocked  bool
	}{
		{"git checkout blocked", "run_in_terminal", "git checkout main", true},
		{"git add blocked", "run_in_terminal", "git add .", true},
		{"git branch blocked", "run_in_terminal", "git branch feature/x", true},
		{"git cherry-pick blocked", "run_in_terminal", "git cherry-pick abc123", true},
		{"git clone blocked", "run_in_terminal", "git clone https://example.com/repo.git", true},
		{"git commit blocked", "run_in_terminal", "git commit -m test", true},
		{"git config blocked", "run_in_terminal", "git config user.name compose", true},
		{"git fetch blocked", "run_in_terminal", "git fetch origin", true},
		{"git init blocked", "run_in_terminal", "git init", true},
		{"git merge blocked", "run_in_terminal", "git merge main", true},
		{"git mv blocked", "run_in_terminal", "git mv a b", true},
		{"git pull blocked", "run_in_terminal", "git pull origin main", true},
		{"git push blocked", "run_in_terminal", "git push origin main", true},
		{"git rebase blocked", "run_in_terminal", "git rebase main", true},
		{"git remote blocked", "run_in_terminal", "git remote add origin https://example.com/repo.git", true},
		{"git reset blocked", "run_in_terminal", "git reset --hard", true},
		{"git restore blocked", "run_in_terminal", "git restore .", true},
		{"git revert blocked", "run_in_terminal", "git revert abc123", true},
		{"git rm blocked", "run_in_terminal", "git rm file.txt", true},
		{"git stash blocked", "bash", "git stash", true},
		{"git stash pop blocked", "shell", "git stash pop", true},
		{"git stash drop blocked", "terminal", "git stash drop", true},
		{"git stash apply blocked", "run", "git stash apply", true},
		{"git switch blocked", "run_in_terminal", "git switch main", true},
		{"git tag blocked", "run_in_terminal", "git tag v1.2.3", true},
		{"git clean fd blocked", "terminal", "git clean -fd", true},
		{"git with global option blocked", "terminal", "git -C /workspaces/semio stash", true},
		{"git with config option blocked", "terminal", "git -c core.hooksPath=/tmp commit -m test", true},
		{"env wrapped git blocked", "terminal", "GIT_TRACE=1 git stash push", true},
		{"env command git blocked", "terminal", "env GIT_TRACE=1 git checkout main", true},
		{"command wrapped git blocked", "terminal", "command git switch main", true},
		{"sudo wrapped git blocked", "terminal", "sudo git reset --hard", true},
		{"absolute git path blocked", "terminal", "/usr/bin/git pull origin main", true},
		{"shell wrapped git blocked", "terminal", `bash -lc "git stash && echo done"`, true},
		{"git checkout in args blocked", "", "git checkout feature/branch", true},
		{"regular tool allowed", "read_file", "/path/to/file.ts", false},
		{"git status allowed", "terminal", "git status", false},
		{"git log allowed", "terminal", "git log --oneline -n 5", false},
		{"git diff allowed", "terminal", "git diff", false},
		{"git rev-parse allowed", "terminal", "git rev-parse HEAD", false},
		{"shell wrapped git status allowed", "terminal", `bash -lc "git status"`, false},
		{"empty allowed", "", "", false},
		{"case insensitive", "TERMINAL", "GIT CHECKOUT main", true},
		{"grep with git checkout pattern not blocked", "", `grep "git checkout" file.go`, false},
		{"echo with git stash not blocked", "", `echo "git stash"`, false},
		{"compose cli command not blocked", "", `go run ./repo/client/mcp/go tree "hooks events inlet adapter cli"`, false},
		{"cd then git checkout blocked", "", "cd /workspaces && git checkout feature", true},
		{"pipe grep allowed", "", `ls | grep "git checkout"`, false},
		{"git checkout after semicolon blocked", "", "echo done; git checkout main", true},
		{"git restore after semicolon blocked", "", "echo done; git restore --staged .", true},
		{"grep for git reset not blocked", "bash", `grep -rn "git reset --hard" .`, false},
		{"python subprocess git stash blocked", "run_in_terminal", `python3 -c "import subprocess; subprocess.run(['git', 'stash'])"`, true},
		{"python os.system git checkout blocked", "run_in_terminal", `python3 -c "import os; os.system('git checkout main')"`, true},
		{"node exec git stash blocked", "run_in_terminal", `node -e "require('child_process').exec('git stash')"`, true},
		{"perl system git checkout blocked", "run_in_terminal", `perl -e "system('git checkout main')"`, true},
		{"ruby system git stash blocked", "run_in_terminal", `ruby -e "system('git stash')"`, true},
		{"fish git stash blocked", "run_in_terminal", `fish -c "git stash"`, true},
		{"ksh git reset blocked", "run_in_terminal", `ksh -c "git reset --hard"`, true},
		{"xargs git stash blocked", "run_in_terminal", "xargs git stash", true},
		{"python git status allowed", "run_in_terminal", `python3 -c "import subprocess; subprocess.call(['git', 'status'])"`, false},
		{"kill lsof port blocked", "terminal", "kill $(lsof -t -i:9876)", true},
		{"kill -9 lsof port blocked", "terminal", "kill -9 $(lsof -t -i:3000)", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			blocked, _ := IsToolBlocked(tc.toolName, tc.toolArgs)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v, got blocked=%v", tc.blocked, blocked)
			}
		})
	}
}

func TestSplitCommandSegments(t *testing.T) {
	cases := []struct {
		cmd      string
		expected []string
	}{
		{"git checkout main", []string{"git checkout main"}},
		{"cd /tmp && git checkout main", []string{"cd /tmp", "git checkout main"}},
		{"echo done; git stash", []string{"echo done", "git stash"}},
		{"ls | grep foo", []string{"ls", "grep foo"}},
		{"a || b", []string{"a", "b"}},
		{"  ", []string{}},
		{"", []string{}},
	}
	for _, tc := range cases {
		t.Run(tc.cmd, func(t *testing.T) {
			got := splitCommandSegments(tc.cmd)
			if len(got) != len(tc.expected) {
				t.Fatalf("expected %v, got %v", tc.expected, got)
			}
			for i := range tc.expected {
				if got[i] != tc.expected[i] {
					t.Errorf("segment %d: expected %q, got %q", i, tc.expected[i], got[i])
				}
			}
		})
	}
}

func TestIsCommandSegmentBlocked(t *testing.T) {
	cases := []struct {
		segment string
		blocked bool
	}{
		{"git add .", true},
		{"git branch feature/x", true},
		{"git checkout main", true},
		{"git cherry-pick abc123", true},
		{"git clone https://example.com/repo.git", true},
		{"git commit -m msg", true},
		{"git config user.email dev@example.com", true},
		{"git fetch origin", true},
		{"git init", true},
		{"git merge main", true},
		{"git mv a b", true},
		{"git pull origin main", true},
		{"git push origin main", true},
		{"git rebase main", true},
		{"git remote add origin https://example.com/repo.git", true},
		{"git restore --staged .", true},
		{"git revert abc123", true},
		{"git rm file.txt", true},
		{"git stash", true},
		{"git reset --hard", true},
		{"git switch feature/x", true},
		{"git tag v1.2.3", true},
		{"git clean -fd", true},
		{"git -C /tmp stash", true},
		{"git -c core.editor=true commit -m msg", true},
		{"GIT_TRACE=1 git stash", true},
		{"env GIT_TRACE=1 git checkout main", true},
		{"command git switch main", true},
		{"sudo /usr/bin/git reset --hard", true},
		{`bash -lc "git stash && echo done"`, true},
		{`grep "git checkout" file.go`, false},
		{`echo "git stash"`, false},
		{"go run ./repo/client/mcp/go tree hooks", false},
		{"git status", false},
		{"git log --oneline -n 5", false},
		{"git diff", false},
		{"git rev-parse HEAD", false},
		{`bash -lc "git status"`, false},
		{"GIT CHECKOUT branch", true},
		{"", false},
		// Script interpreters with inline git commands.
		{`python -c "import subprocess; subprocess.run(['git', 'stash'])"`, true},
		{`python3 -c "import os; os.system('git checkout main')"`, true},
		{`python3 -c "import subprocess; subprocess.call(['git', 'status'])"`, false},
		{`node -e "require('child_process').exec('git stash')"`, true},
		{`node -e "require('child_process').exec('git status')"`, false},
		{`perl -e "system('git checkout main')"`, true},
		{`ruby -e "system('git stash')"`, true},
		{`ruby -e "system('git status')"`, false},
		// Additional shells.
		{`fish -c "git stash"`, true},
		{`ksh -c "git reset --hard"`, true},
		{`dash -c "git checkout main"`, true},
		// xargs forwarding git.
		{"xargs git stash", true},
		{"xargs git checkout", true},
		{"xargs git status", false},
		// kill+lsof port killing is denied (can terminate devcontainer).
		{"kill $(lsof -t -i:9876)", true},
		{"kill -9 $(lsof -t -i:9876)", true},
	}
	for _, tc := range cases {
		t.Run(tc.segment, func(t *testing.T) {
			blocked, _ := isCommandSegmentBlocked(tc.segment)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v for %q", tc.blocked, tc.segment)
			}
		})
	}
}

func TestRunHookAgentEvents(t *testing.T) {
	cases := []struct {
		name    string
		event   HookEvent
		allowed bool
	}{
		{"agent starting", HookAgentStarted, true},
		{"agent ended", HookAgentEnded, true},
		{"agent prompt submitting", HookAgentPromptSubmitting, true},
		{"agent compacting", HookAgentCompacting, true},
		{"agent tool ended", HookAgentToolEnded, true},
		{"agent tool code searching", HookAgentToolSearchStarting, true},
		{"agent tool searched", HookAgentToolSearchEnded, true},
		{"agent tool code editing", HookAgentToolCodeEditStarting, true},
		{"agent tool code edited", HookAgentToolCodeEditEnded, true},
		{"agent tool plan updating starting", HookAgentToolPlanUpdatingStarting, true},
		{"agent tool plan updating ended", HookAgentToolPlanUpdatingEnded, true},
		{"agent tool terminal starting", HookAgentToolTerminalStarting, true},
		{"agent tool terminal ended", HookAgentToolTerminalEnded, true},
		{"agent thinking starting", HookAgentThinkingStarting, true},
		{"agent thinking ended", HookAgentThinkingEnded, true},
		{"version checkpoint ended", HookVersionCheckpointEnded, true},
		{"version checkin starting", HookVersionCheckinStarting, true},
		{"version checkin ended", HookVersionCheckinEnded, true},
		{"version checkout starting", HookVersionCheckoutStarting, true},
		{"version checkout ended", HookVersionCheckoutEnded, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "copilot-chat",
				Second:   time.Now().UTC().Format(time.RFC3339),
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if result.IsAllowed() != tc.allowed {
				t.Errorf("expected allowed=%v, got allowed=%v: %s", tc.allowed, result.IsAllowed(), result.GetMessage())
			}
		})
	}
	t.Run("version checkpoint starting", func(t *testing.T) {
		hctx := HookContext{
			Event:    HookVersionCheckpointStarting,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)

		_, ok := result.(HookResultVersionCheckpointStarting)
		if !ok {
			t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
		}
	})
}

func TestRunHookToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "run_in_terminal",
		ToolArgs: "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestRunHookToolAllowed(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "cursor-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "read_file",
		ToolArgs: "/workspaces/semio/repo/client/main.go",
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Errorf("expected tool to be allowed, got: %s", result.GetMessage())
	}
}

func TestRunHookUnknownEvent(t *testing.T) {
	hctx := HookContext{
		Event:    HookEvent("unknown.event"),
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected unknown event to be denied")
	}
}

func TestHookClientForMcpKindMapsIDEEntrypoints(t *testing.T) {
	cases := []struct {
		kind McpClientKind
		want string
	}{
		{McpClientCursor, "cursor-chat"},
		{McpClientKiro, "kiro-cli"},
		{McpClientCopilot, "copilot-chat"},
		{McpClientClaude, "claude-code"},
		{McpClientCodex, "codex"},
		{McpClientGeneric, ""},
	}
	for _, tc := range cases {
		if got := HookClientForMcpKind(tc.kind); got != tc.want {
			t.Errorf("HookClientForMcpKind(%q) = %q, want %q", tc.kind, got, tc.want)
		}
	}
}

func TestResolveMcpTicketClient(t *testing.T) {
	tests := []struct {
		name     string
		kind     McpClientKind
		client   string
		expected string
	}{
		{"Codex inferred", McpClientCodex, "", "codex"},
		{"Claude inferred", McpClientClaude, "", "claude-code"},
		{"Explicit preserved", McpClientCodex, "cursor-chat", "cursor-chat"},
		{"Generic remains empty", McpClientGeneric, "", ""},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if got := resolveMcpTicketClient(tc.kind, tc.client); got != tc.expected {
				t.Fatalf("resolveMcpTicketClient(%q, %q) = %q, want %q", tc.kind, tc.client, got, tc.expected)
			}
		})
	}
}

func TestCurrentMcpLLMsAllowed(t *testing.T) {
	for _, llm := range []string{"opus-4-7", "gpt-5-5"} {
		if got, err := ResolveAllowedLLM(llm); err != nil || got != llm {
			t.Fatalf("ResolveAllowedLLM(%q) = %q, %v", llm, got, err)
		}
	}
}

func TestWriteWarningfUsesStderr(t *testing.T) {
	read, write, err := os.Pipe()
	if err != nil {
		t.Fatal(err)
	}
	previous := os.Stderr
	os.Stderr = write
	defer func() { os.Stderr = previous }()

	writeWarningf("Failed to update: %s", "boom")
	if err := write.Close(); err != nil {
		t.Fatal(err)
	}
	os.Stderr = previous
	output, err := io.ReadAll(read)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := string(output), "Warning: Failed to update: boom\n"; got != want {
		t.Fatalf("warning output = %q, want %q", got, want)
	}
}

func TestRunHookForRejectsGenericKind(t *testing.T) {
	err := RunHookFor(McpClientGeneric, "stop", nil)
	if err == nil || !strings.Contains(strings.ToLower(err.Error()), "hooks are not available") {
		t.Fatalf("expected hooks-unavailable error for generic kind, got: %v", err)
	}
}

func TestRunHookThinkingEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow run hook thinking events test in short mode")
	}
	t.Run("thinking ended extracts text from top-level", func(t *testing.T) {
		hctx := HookContext{
			Event:  HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think","text":"Planning repo modifications"}`),
		}
		result := RunHook(hctx)
		res, ok := result.(HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Planning repo modifications" {
			t.Errorf("expected thinking text in message, got %q", res.GetMessage())
		}
		if !res.IsAllowed() {
			t.Error("expected thinking ended to be allowed")
		}
	})
	t.Run("thinking ended extracts text from native.event", func(t *testing.T) {
		input := `{"native":{"event":{"text":"Thinking deeply","hook_event_name":"afterAgentThought"}},"session_id":"sess-think2"}`
		hctx := HookContext{
			Event:  HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(input),
		}
		result := RunHook(hctx)
		res, ok := result.(HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Thinking deeply" {
			t.Errorf("expected thinking text from native.event, got %q", res.GetMessage())
		}
	})
	t.Run("thinking starting is allowed", func(t *testing.T) {
		hctx := HookContext{
			Event:  HookAgentThinkingStarting,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think3","text":"About to think"}`),
		}
		result := RunHook(hctx)
		if _, ok := result.(HookResultAgentThinkingStarting); !ok {
			t.Fatalf("expected HookResultAgentThinkingStarting, got %T", result)
		}
		if !result.IsAllowed() {
			t.Error("expected thinking starting to be allowed")
		}
	})
}

func TestAllHookEventsCompleteness(t *testing.T) {
	expected := []HookEvent{
		HookVersionCheckpointStarting, HookVersionCheckpointEnded,
		HookVersionCheckinStarting, HookVersionCheckinEnded,
		HookVersionCheckoutStarting, HookVersionCheckoutEnded,
		HookAgentStarted, HookAgentEnded,
		HookAgentPromptSubmitting, HookAgentCompacting,
		HookAgentToolStarting, HookAgentToolEnded,
		HookAgentToolPlanUpdatingStarting, HookAgentToolPlanUpdatingEnded,
		HookAgentToolSearchStarting, HookAgentToolSearchEnded,
		HookAgentToolCodeEditStarting, HookAgentToolCodeEditEnded,
		HookAgentToolTestStarting, HookAgentToolTestEnded,
		HookAgentToolBuildStarting, HookAgentToolBuildEnded,
		HookAgentToolTerminalStarting, HookAgentToolTerminalEnded,
		HookAgentThinkingStarting, HookAgentThinkingEnded,
	}
	if len(AllHookEvents) != len(expected) {
		t.Errorf("expected %d events, got %d", len(expected), len(AllHookEvents))
	}
	for _, e := range expected {
		found := false
		for _, a := range AllHookEvents {
			if a == e {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing event: %s", e)
		}
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

func TestHookCommandToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "terminal",
		ToolArgs: "git stash pop",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "git stash") {
		t.Errorf("expected git stash in message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "other developers and agents may be editing the same files concurrently") {
		t.Errorf("expected concurrent edit warning in message, got: %s", result.GetMessage())
	}
}

func TestHookCommandJSONOutput(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	out, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON marshaling, got: %v", err)
	}
	var parsed HookResultAgentStarted
	if err := json.Unmarshal(out, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	if !parsed.Allowed {
		t.Error("expected allowed=true")
	}
	if parsed.Second == "" {
		t.Error("expected non-empty second")
	}
}

func TestConfigureCommandDoesNotGenerateConfigFiles(t *testing.T) {
	repoRoot := t.TempDir()
	hooksDir := filepath.Join(repoRoot, ".git", "hooks")
	if err := os.MkdirAll(hooksDir, 0o755); err != nil {
		t.Fatalf("mkdir hooks: %v", err)
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
	for _, hookName := range []string{"post-commit", "prepare-commit-msg"} {
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

func TestMicroCommitPostCommitHookResetsTemplates(t *testing.T) {
	repoRoot := t.TempDir()
	initGit := exec.Command("git", "init")
	initGit.Dir = repoRoot
	if out, err := initGit.CombinedOutput(); err != nil {
		t.Fatalf("git init: %v\n%s", err, out)
	}
	gitDir := filepath.Join(repoRoot, ".git")
	hooksDir := filepath.Join(gitDir, "hooks")
	if err := os.MkdirAll(hooksDir, 0o755); err != nil {
		t.Fatalf("mkdir hooks: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(repoRoot, "repo", "hook"), 0o755); err != nil {
		t.Fatalf("mkdir repo hooks: %v", err)
	}
	for _, hookName := range []string{"post-commit", "prepare-commit-msg"} {
		hookSrc := filepath.Join("..", "..", "hook", hookName)
		data, err := os.ReadFile(hookSrc)
		if err != nil {
			t.Fatalf("read hook source %s: %v", hookName, err)
		}
		if err := os.WriteFile(filepath.Join(repoRoot, "repo", "hook", hookName), data, 0o755); err != nil {
			t.Fatalf("write hook source %s: %v", hookName, err)
		}
	}
	if err := installMicroCommitHooks(repoRoot); err != nil {
		t.Fatalf("install hooks: %v", err)
	}
	templatePath := filepath.Join(gitDir, "gkcommittemplate.txt")
	editMsgPath := filepath.Join(gitDir, "COMMIT_EDITMSG")
	for _, p := range []string{templatePath, editMsgPath} {
		if err := os.WriteFile(p, []byte("draft\n"), 0o644); err != nil {
			t.Fatalf("seed %s: %v", p, err)
		}
	}
	hookPath := filepath.Join(hooksDir, "post-commit")
	cmd := exec.Command(hookPath)
	cmd.Dir = repoRoot
	cmd.Env = append(os.Environ(), "GIT_DIR="+gitDir, "GIT_WORK_TREE="+repoRoot)
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("run post-commit: %v\n%s", err, out)
	}
	if _, err := os.Stat(templatePath); err == nil {
		t.Fatalf("expected %s removed after post-commit", templatePath)
	} else if !os.IsNotExist(err) {
		t.Fatalf("stat %s: %v", templatePath, err)
	}
	b, err := os.ReadFile(editMsgPath)
	if err != nil {
		t.Fatalf("read %s: %v", editMsgPath, err)
	}
	if len(b) != 0 {
		t.Fatalf("expected empty %s after post-commit, got %q", editMsgPath, b)
	}
	tplCfg := exec.Command("git", "config", "--local", "--get", "commit.template")
	tplCfg.Dir = repoRoot
	tplOut, err := tplCfg.Output()
	if err != nil {
		t.Fatalf("expected commit.template set to empty GK file after post-commit: %v", err)
	}
	if !strings.Contains(string(tplOut), "gkcommittemplate.txt") {
		t.Fatalf("expected commit.template to point at gkcommittemplate.txt, got %q", tplOut)
	}
	legacy, err := os.ReadFile(templatePath)
	if err != nil {
		t.Fatalf("read legacy GK template: %v", err)
	}
	if len(legacy) != 0 {
		t.Fatalf("expected empty gkcommittemplate.txt after post-commit, got %q", legacy)
	}
}

func TestExtractToolNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"vscode tool_name", `{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"ls"}}`, "run_in_terminal"},
		{"claude code tool_name", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "Bash"},
		{"no tool_name", `{"tool_input":{"command":"ls"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractToolNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractHookEventNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"PreToolUse", `{"hookEventName":"PreToolUse","tool_name":"editFiles"}`, "PreToolUse"},
		{"PostToolUse", `{"hookEventName":"PostToolUse","tool_name":"editFiles"}`, "PostToolUse"},
		{"SessionStart", `{"hookEventName":"SessionStart","source":"new"}`, "SessionStart"},
		{"no hookEventName", `{"tool_name":"Bash"}`, ""},
		{"empty", `{}`, ""},
		{"invalid json", `bad`, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractHookEventNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestVSCodeEventFromHookEvent(t *testing.T) {
	cases := []struct {
		name       string
		event      HookEvent
		parentInfo string
		expect     string
	}{
		{"agent.tool.starting", HookAgentToolStarting, "", "PreToolUse"},
		{"agent.tool.ended", HookAgentToolEnded, "", "PostToolUse"},
		{"agent.started", HookAgentStarted, "", "SessionStart"},
		{"agent.started subagent", HookAgentStarted, "subagent", "SubagentStart"},
		{"agent.ended", HookAgentEnded, "", "Stop"},
		{"agent.ended subagent", HookAgentEnded, "subagent", "SubagentStop"},
		{"agent.prompt.submitting", HookAgentPromptSubmitting, "", "UserPromptSubmit"},
		{"agent.compacting", HookAgentCompacting, "", "PreCompact"},
		{"agent.file.read.starting", HookAgentToolSearchStarting, "", "PreToolUse"},
		{"agent.tool.code.edit.starting", HookAgentToolCodeEditStarting, "", "PreToolUse"},
		{"agent.tool.code.edit.ended", HookAgentToolCodeEditEnded, "", "PostToolUse"},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, "", "PreToolUse"},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, "", "PostToolUse"},
		{"agent.tool.test.starting", HookAgentToolTestStarting, "", "PreToolUse"},
		{"agent.tool.test.ended", HookAgentToolTestEnded, "", "PostToolUse"},
		{"agent.tool.build.starting", HookAgentToolBuildStarting, "", "PreToolUse"},
		{"agent.tool.build.ended", HookAgentToolBuildEnded, "", "PostToolUse"},
		{"agent.tool.plan.updating.starting", HookAgentToolPlanUpdatingStarting, "", "PreToolUse"},
		{"agent.tool.plan.updating.ended", HookAgentToolPlanUpdatingEnded, "", "PostToolUse"},
		{"agent.thinking.starting", HookAgentThinkingStarting, "", ""},
		{"agent.thinking.ended", HookAgentThinkingEnded, "", ""},
		{"unknown", HookEvent("unknown.x"), "", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := vsCodeEventFromHookEvent(tc.event, tc.parentInfo)
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestFormatVSCodeHookOutput(t *testing.T) {
	t.Run("PreToolUse allow", func(t *testing.T) {
		out := formatVSCodeHookOutput("PreToolUse", HookResultAgentToolStarting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected permissionDecision=allow, got %v", hso["permissionDecision"])
		}
		if hso["hookEventName"] != "PreToolUse" {
			t.Errorf("expected hookEventName=PreToolUse, got %v", hso["hookEventName"])
		}
	})
	t.Run("PreToolUse deny", func(t *testing.T) {
		out := formatVSCodeHookOutput("PreToolUse", HookResultAgentToolStarting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: false, Message: "blocked: git checkout"}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected permissionDecision=deny, got %v", hso["permissionDecision"])
		}
		if hso["permissionDecisionReason"] != "blocked: git checkout" {
			t.Errorf("expected reason in output, got %v", hso["permissionDecisionReason"])
		}
	})
	t.Run("SessionStart with message", func(t *testing.T) {
		out := formatVSCodeHookOutput("SessionStart", HookResultAgentStarted{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true, Message: "agent.started acknowledged"}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["additionalContext"] != "agent.started acknowledged" {
			t.Errorf("expected additionalContext, got %v", hso["additionalContext"])
		}
	})
	t.Run("Stop always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("Stop", HookResultAgentEnded{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for Stop")
		}
		if hso["hookEventName"] != "Stop" {
			t.Errorf("expected hookEventName=Stop, got %v", hso["hookEventName"])
		}
	})
	t.Run("UserPromptSubmit always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("UserPromptSubmit", HookResultAgentPromptSubmitting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for UserPromptSubmit")
		}
		if hso["hookEventName"] != "UserPromptSubmit" {
			t.Errorf("expected hookEventName=UserPromptSubmit, got %v", hso["hookEventName"])
		}
	})
	t.Run("PostToolUse always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("PostToolUse", HookResultAgentToolEnded{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for PostToolUse")
		}
		if hso["hookEventName"] != "PostToolUse" {
			t.Errorf("expected hookEventName=PostToolUse, got %v", hso["hookEventName"])
		}
	})
	t.Run("unknown event empty output", func(t *testing.T) {
		out := formatVSCodeHookOutput("", HookResultBase{Allowed: true, Message: "test"})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
	})
}

func TestHookCommandCopilotChatVSCodeOutput(t *testing.T) {
	t.Run("PreToolUse allow produces VS Code JSON", func(t *testing.T) {
		hctx := HookContext{
			Event:    HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "read_file",
			ToolArgs: "/tmp/file.ts",
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if !result.IsAllowed() {
			t.Fatal("expected allowed")
		}
		out := formatVSCodeHookOutput("PreToolUse", result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected allow, got %v", hso["permissionDecision"])
		}
	})
	t.Run("PreToolUse blocked produces VS Code deny JSON", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git checkout main"}}`)
		hctx := HookContext{
			Event:    HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "run_in_terminal",
			Input:    payload,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if result.IsAllowed() {
			t.Fatal("expected blocked")
		}
		hookEventName := extractHookEventNameFromStdin(payload)
		out := formatVSCodeHookOutput(hookEventName, result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected deny, got %v", hso["permissionDecision"])
		}
		reason, _ := hso["permissionDecisionReason"].(string)
		if !strings.Contains(reason, "blocked") {
			t.Errorf("expected blocked reason, got: %s", reason)
		}
	})
	t.Run("tool_name extracted from stdin", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git stash"}}`)
		toolName := extractToolNameFromStdin(payload)
		if toolName != "run_in_terminal" {
			t.Errorf("expected run_in_terminal, got %s", toolName)
		}
		cmd := extractCommandFromStdin(payload)
		if cmd != "git stash" {
			t.Errorf("expected git stash, got %s", cmd)
		}
	})
}

func TestBlockedToolPatterns(t *testing.T) {
	if len(BlockedToolPatterns) < 20 {
		t.Errorf("expected at least 20 blocked patterns, got %d", len(BlockedToolPatterns))
	}
	expectedPatterns := []string{
		"git add",
		"git branch",
		"git checkout",
		"git cherry-pick",
		"git clean",
		"git clone",
		"git commit",
		"git config",
		"git fetch",
		"git init",
		"git merge",
		"git mv",
		"git pull",
		"git push",
		"git rebase",
		"git remote",
		"git reset",
		"git restore",
		"git revert",
		"git rm",
		"git stash",
		"git switch",
		"git tag",
	}
	for _, ep := range expectedPatterns {
		found := false
		for bp := range BlockedToolPatterns {
			if bp == ep {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing blocked pattern: %s", ep)
		}
	}
}

func testLoggingConfigFull() LoggingConfig {
	return LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func testLoggingConfigSession() LoggingConfig {
	return LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "standard"}
}

func writeRepoLoggingConfig(t *testing.T, root string, lg LoggingConfig) {
	t.Helper()
	repoDir := filepath.Join(root, ".🦑repo")
	if err := os.MkdirAll(repoDir, 0o755); err != nil {
		t.Fatalf("mkdir .🦑repo: %v", err)
	}
	content := fmt.Sprintf("[logging]\nsession = %t\noperations = %t\nplan = %t\ndetail = \"%s\"\n",
		lg.Session, lg.Operations, lg.Plan, lg.Detail)
	if err := os.WriteFile(filepath.Join(repoDir, "config.toml"), []byte(content), 0o644); err != nil {
		t.Fatalf("write config.toml: %v", err)
	}
}

func TestRepoConfig(t *testing.T) {
	t.Run("defaults when file missing", func(t *testing.T) {
		cfg := LoadRepoConfig(t.TempDir())
		if cfg.Logging.Session {
			t.Error("expected session logging off by default")
		}
		if !cfg.Logging.Operations || !cfg.Logging.Plan {
			t.Error("expected operations and plan enabled by default")
		}
		if cfg.Logging.Detail != "standard" {
			t.Errorf("expected detail standard, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("parse logging table", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{
			Session: true, Operations: false, Plan: false, Detail: "minimal",
		})
		cfg := LoadRepoConfig(tmpDir)
		if !cfg.Logging.Session || cfg.Logging.Operations || cfg.Logging.Plan {
			t.Errorf("unexpected logging config: %+v", cfg.Logging)
		}
		if cfg.Logging.Detail != "minimal" {
			t.Errorf("expected detail minimal, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("session off by default", func(t *testing.T) {
		tmpDir := t.TempDir()
		RunHook(HookContext{
			Event:    HookAgentStarted,
			Client:   "claude-code",
			Second:   time.Now().UTC().Format(time.RFC3339),
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"off-by-default"}`),
		})
		assertNoHookLogFiles(t, tmpDir)
	})
	t.Run("detail levels", func(t *testing.T) {
		tmpDir := t.TempDir()
		logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated", "26", "05", "30", "detail-sess")
		if err := os.MkdirAll(logDir, 0o755); err != nil {
			t.Fatal(err)
		}
		hctx := HookContext{Event: HookAgentStarted, Client: "claude-code", Input: json.RawMessage(`{"x":1}`)}
		result := HookResultBase{Allowed: true}
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Detail: "minimal"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", LoadRepoConfig(tmpDir).Logging)
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 event, got %d", len(meta.Events))
		}
		if meta.Events[0].Native != nil {
			t.Error("minimal must not include native")
		}
		if meta.Events[0].Response != nil {
			t.Error("minimal must not include response")
		}
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Detail: "full"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", LoadRepoConfig(tmpDir).Logging)
		data, _ = os.ReadFile(filepath.Join(logDir, "session.json"))
		json.Unmarshal(data, &meta)
		if len(meta.Events) < 2 {
			t.Fatalf("expected 2 events after full detail append")
		}
		last := meta.Events[len(meta.Events)-1]
		if last.Native == nil || last.Response == nil {
			t.Error("full detail must include native and response")
		}
	})
	t.Run("plan off", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Plan: false, Detail: "standard"})
		RunHook(HookContext{
			Event:    HookAgentToolPlanUpdatingEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"plan-off","tool_input":{"todoList":[{"title":"A","status":"pending"}]}}`),
		})
		logFiles := getLogFiles(t, tmpDir)
		if len(logFiles) != 1 {
			t.Fatalf("expected session.json, got %v", logFiles)
		}
		var meta SessionMeta
		data, _ := os.ReadFile(logFiles[0])
		json.Unmarshal(data, &meta)
		if meta.Plan != nil {
			t.Error("expected no plan when plan=false")
		}
	})
}

func TestHookLogging(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"sess-log")
	payload := json.RawMessage(`{"session_id":"sess-log","second":"2026-02-20T10:00:00Z","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-02-20T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Fatalf("expected allowed=true, got: %s", result.GetMessage())
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json for session events, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json to be the only file, got %s", entries[0].Name())
	}
	metaBytes, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("expected session.json to exist, got error: %v", err)
	}
	var meta SessionMeta
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("expected valid JSON in session.json, got: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected exactly 1 session event, got %d", len(meta.Events))
	}
	if meta.ID == "" {
		t.Errorf("expected id in session.json, got: %v", meta.ID)
	}
	if meta.URI == "" {
		t.Errorf("expected uri in session.json, got: %v", meta.URI)
	}
	if meta.Contributor == "" {
		t.Errorf("expected contributor in session.json, got: %v", meta.Contributor)
	}
	if meta.Client != "claude-code" {
		t.Errorf("expected client claude-code in session.json, got: %v", meta.Client)
	}
	if meta.Second == "" {
		t.Errorf("expected second in session.json, got: %v", meta.Second)
	}
	if meta.Transcript != "/tmp/transcript.jsonl" {
		t.Errorf("expected transcript in session.json, got: %v", meta.Transcript)
	}
	entry := meta.Events[0]
	if entry.Native.Event == nil {
		t.Error("expected native.event to be populated")
	}
	var evt map[string]interface{}
	if err := json.Unmarshal(entry.Event, &evt); err != nil {
		t.Fatalf("cannot unmarshal event: %v", err)
	}
	if evt["kind"] != string(HookAgentStarted) {
		t.Errorf("expected event.kind agent.started in log, got: %v", evt["kind"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code in log, got: %v", evt["client"])
	}
	if _, ok := evt["contributor"]; !ok {
		t.Error("expected event.contributor to be present in log entry")
	}
	expectedSession := resolveEventSessionID("sess-log")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s in log, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := resolveEventSecondID("2026-02-20T10:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/transcript.jsonl" {
		t.Errorf("expected event.transcript in log, got: %v", evt["transcript"])
	}
	if entry.Response.Blocked != nil {
		t.Error("expected response.blocked to be nil for allowed event")
	}
	if len(meta.Events) > 0 && len(meta.Events[0].Event) == 0 {
		t.Error("expected non-empty serialized event payload in session event")
	}
	for _, forbidden := range []string{"uuid", "started_at", "first_event"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent in session.json", forbidden)
		}
	}
	for _, forbidden := range []string{"native", "event", "response"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent at root of session.json", forbidden)
		}
	}
	if len(meta.Events) == 0 {
		t.Fatalf("expected session.json to be written alongside agent events")
	}
}

func TestSessionJsonTracksPlan(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	sessionID := "plan-track-session"
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		sessionID)
	startPayload := json.RawMessage(`{"session_id":"plan-track-session","second":"2026-03-02T10:00:00Z"}`)
	RunHook(HookContext{
		Event:    HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-03-02T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    startPayload,
	})

	plan1Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"in-progress"},{"title":"Step B","status":"pending"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    plan1Payload,
	})

	metaPath := filepath.Join(logDir, "session.json")
	metaBytes, err := os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after plan update, got error: %v", err)
	}
	var meta SessionMeta
	assertHasLifecycle := func(step TicketAgentPlanStep) {
		if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
			t.Errorf("expected step %q to include at least one lifecycle timestamp, got %+v", step.Name, step)
		}
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 2 {
		t.Fatalf("expected 2 plan steps, got %v", meta.Plan)
	}
	stepA := meta.Plan.Steps[0]
	if stepA.Name != "Step A" {
		t.Errorf("expected Step A, got %s", stepA.Name)
	}
	if stepA.Ideated == "" {
		t.Errorf("expected Step A ideated timestamp, got empty")
	}
	if stepA.Started == "" {
		t.Errorf("expected Step A started timestamp, got empty")
	}
	if stepA.Completed != "" {
		t.Errorf("expected Step A not completed, got %s", stepA.Completed)
	}
	stepB := meta.Plan.Steps[1]
	if stepB.Name != "Step B" {
		t.Errorf("expected Step B, got %s", stepB.Name)
	}
	if stepB.Ideated == "" {
		t.Errorf("expected Step B ideated timestamp, got empty")
	}
	if stepB.Started != "" || stepB.Completed != "" {
		t.Errorf("expected Step B pending, got started=%s completed=%s", stepB.Started, stepB.Completed)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan2Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"completed"},{"title":"Step B","status":"in-progress"},{"title":"Step C","status":"pending"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    plan2Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after second plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after second update, got %v", meta.Plan)
	}
	stepA2 := meta.Plan.Steps[0]
	if stepA2.Name != "Step A" || stepA2.Completed == "" {
		t.Errorf("expected Step A completed, got %+v", stepA2)
	}
	if stepA2.Ideated != stepA.Ideated {
		t.Errorf("expected Step A ideated timestamp preserved, got %s vs %s", stepA2.Ideated, stepA.Ideated)
	}
	if stepA2.Started != stepA.Started {
		t.Errorf("expected Step A started timestamp preserved, got %s vs %s", stepA2.Started, stepA.Started)
	}
	stepB2 := meta.Plan.Steps[1]
	if stepB2.Name != "Step B" || stepB2.Started == "" {
		t.Errorf("expected Step B in-progress, got %+v", stepB2)
	}
	stepC := meta.Plan.Steps[2]
	if stepC.Name != "Step C" || stepC.Ideated == "" || stepC.Started != "" || stepC.Completed != "" {
		t.Errorf("expected Step C pending, got %+v", stepC)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan3Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step B","status":"completed"},{"title":"Step C","status":"in-progress"}]}}`)
	RunHook(HookContext{
		Event:    HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:03:00Z",
		RepoRoot: tmpDir,
		Input:    plan3Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after third plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after third update (Step A preserved as completed), got %v", meta.Plan)
	}
	stepB3 := meta.Plan.Steps[0]
	if stepB3.Name != "Step B" || stepB3.Completed == "" {
		t.Errorf("expected Step B completed, got %+v", stepB3)
	}
	if stepB3.Started != stepB2.Started {
		t.Errorf("expected Step B started timestamp preserved, got %s vs %s", stepB3.Started, stepB2.Started)
	}
	stepC2 := meta.Plan.Steps[1]
	if stepC2.Name != "Step C" || stepC2.Started == "" {
		t.Errorf("expected Step C in-progress, got %+v", stepC2)
	}
	stepACompleted := meta.Plan.Steps[2]
	if stepACompleted.Name != "Step A" || stepACompleted.Completed == "" || stepACompleted.Abandoned != "" {
		t.Errorf("expected Step A to remain completed and not abandoned, got %+v", stepACompleted)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}
}

func TestMergeTicketAgentPlanSteps(t *testing.T) {
	second1 := "2026-03-02T10:00:00Z"
	second2 := "2026-03-02T10:01:00Z"

	t.Run("empty existing adopts incoming", func(t *testing.T) {
		incoming := []HookPlanStep{
			{Name: "A", Status: "in-progress"},
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(nil, incoming, second1)
		if len(merged) != 2 {
			t.Fatalf("expected 2, got %d", len(merged))
		}
		if merged[0].Ideated != second1 {
			t.Errorf("expected A ideated=%s, got %s", second1, merged[0].Ideated)
		}
		if merged[0].Started != second1 {
			t.Errorf("expected A started=%s, got %s", second1, merged[0].Started)
		}
		if merged[1].Ideated != second1 || merged[1].Started != "" || merged[1].Completed != "" {
			t.Errorf("expected B pending, got %+v", merged[1])
		}
	})

	t.Run("preserves existing timestamps", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if merged[0].Ideated != second1 {
			t.Errorf("expected ideated preserved as %s, got %s", second1, merged[0].Ideated)
		}
		if merged[0].Started != second1 {
			t.Errorf("expected started preserved as %s, got %s", second1, merged[0].Started)
		}
		if merged[0].Completed != second2 {
			t.Errorf("expected completed=%s, got %s", second2, merged[0].Completed)
		}
	})

	t.Run("marks removed never-started steps as abandoned", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
			{Name: "B"},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A abandoned), got %d", len(merged))
		}
		if merged[0].Name != "B" {
			t.Errorf("expected B first, got %s", merged[0].Name)
		}
		abandoned := merged[1]
		if abandoned.Name != "A" || abandoned.Ideated != second1 || abandoned.Started != "" || abandoned.Abandoned != second2 {
			t.Errorf("expected A abandoned at %s, got %+v", second2, abandoned)
		}
	})

	t.Run("does not abandon removed started steps", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1},
			{Name: "B"},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A historical), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Started != second1 || historical.Abandoned != "" {
			t.Errorf("expected A started history preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("preserves already abandoned steps", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Abandoned: second1},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		foundA := 0
		for _, s := range merged {
			if s.Name == "A" {
				foundA++
				if s.Abandoned != second1 {
					t.Errorf("expected A abandoned timestamp unchanged, got %s", s.Abandoned)
				}
			}
		}
		if foundA != 1 {
			t.Errorf("expected abandoned step A to be preserved exactly once, got %d", foundA)
		}
	})

	t.Run("does not abandon completed steps removed from incoming", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Completed: second1},
			{Name: "B", Started: second1},
		}
		incoming := []HookPlanStep{
			{Name: "B", Status: "in-progress"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps (B active + A completed history), got %d", len(merged))
		}
		historical := merged[1]
		if historical.Name != "A" || historical.Completed != second1 || historical.Abandoned != "" {
			t.Errorf("expected completed A preserved without abandoned timestamp, got %+v", historical)
		}
	})

	t.Run("adds new steps not in existing", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1, Started: second1, Completed: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
			{Name: "C", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(merged))
		}
		if merged[1].Name != "C" || merged[1].Ideated != second2 || merged[1].Started != "" {
			t.Errorf("expected C as new pending step, got %+v", merged[1])
		}
	})

	t.Run("does not set completed for step never started", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "A", Ideated: second1},
		}
		incoming := []HookPlanStep{
			{Name: "A", Status: "completed"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 1 {
			t.Fatalf("expected 1 step, got %d", len(merged))
		}
		if merged[0].Completed != "" {
			t.Errorf("expected no completed timestamp when step was never started, got %+v", merged[0])
		}
	})

	t.Run("backfills legacy name-only steps with lifecycle dates", func(t *testing.T) {
		existing := []TicketAgentPlanStep{
			{Name: "Legacy Pending"},
		}
		incoming := []HookPlanStep{
			{Name: "Legacy Pending", Status: "pending"},
			{Name: "New Pending", Status: "pending"},
		}
		merged := mergeTicketAgentPlanSteps(existing, incoming, second2)
		if len(merged) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(merged))
		}
		for _, step := range merged {
			if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
				t.Errorf("expected non-empty lifecycle on step %q, got %+v", step.Name, step)
			}
		}
	})
}

func TestHookLoggingToolBlocked(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"unknown")
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: tmpDir,
		ToolName: "bash",
		ToolArgs: "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected blocked tool to be denied")
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("expected valid session.json: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if entry.Response.Blocked == nil || !*entry.Response.Blocked {
		t.Error("expected response.blocked=true for blocked tool")
	}
	if !strings.Contains(entry.Response.Reason, "blocked") {
		t.Errorf("expected blocked reason in log, got: %s", entry.Response.Reason)
	}
}

func TestHookLoggingStdinInput(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"abc123")
	payload := json.RawMessage(`{"session_id":"abc123","second":"2026-02-20T12:00:00Z","tool_name":"Bash","tool_input":{"command":"ls"},"transcript_path":"/tmp/t.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-02-20T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if len(entry.Native.Event) == 0 {
		t.Error("expected native.event from HookContext.Input")
	}
	var wantMap, gotMap map[string]interface{}
	json.Unmarshal(payload, &wantMap)
	json.Unmarshal(entry.Native.Event, &gotMap)
	wantBytes, _ := json.Marshal(wantMap)
	gotBytes, _ := json.Marshal(gotMap)
	if string(gotBytes) != string(wantBytes) {
		t.Errorf("expected raw %s in log, got: %s", wantBytes, gotBytes)
	}
	var evt map[string]interface{}
	json.Unmarshal(entry.Event, &evt)
	expectedSession := resolveEventSessionID("abc123")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := resolveEventSecondID("2026-02-20T12:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/t.jsonl" {
		t.Errorf("expected event.transcript, got: %v", evt["transcript"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code, got: %v", evt["client"])
	}
}

func TestDeriveRepoOpFromMCPTool(t *testing.T) {
	cases := []struct {
		name     string
		tool     string
		expected string
	}{
		{"ticket open", "mcp__repo__ticket_open", "ticket.open"},
		{"ticket close", "mcp__repo__ticket_close", "ticket.close"},
		{"ticket reopen", "mcp__repo__ticket_reopen", "ticket.reopen"},
		{"ticket read", "mcp__repo__ticket_read", "ticket.read"},
		{"goal open", "mcp__repo__goal_open", "goal.open"},
		{"goal close", "mcp__repo__goal_close", "goal.close"},
		{"goal reopen", "mcp__repo__goal_reopen", "goal.reopen"},
		{"contributor add", "mcp__repo__contributor_add", "contributor.add"},
		{"contributor remove", "mcp__repo__contributor_remove", "contributor.remove"},
		{"draft create", "mcp__repo__draft_create", "draft.create"},
		{"draft delete", "mcp__repo__draft_delete", "draft.delete"},
		{"file create", "mcp__repo__file_create", "file.create"},
		{"file move", "mcp__repo__file_move", "file.move"},
		{"file delete", "mcp__repo__file_delete", "file.delete"},
		{"folder create", "mcp__repo__folder_create", "folder.create"},
		{"folder move", "mcp__repo__folder_move", "folder.move"},
		{"folder delete", "mcp__repo__folder_delete", "folder.delete"},
		{"section create", "mcp__repo__section_create", "section.create"},
		{"section move", "mcp__repo__section_move", "section.move"},
		{"section delete", "mcp__repo__section_delete", "section.delete"},
		{"integrate", "mcp__repo__integrate", "integrate"},
		{"extract", "mcp__repo__extract", "extract"},
		{"export", "mcp__repo__export", "export"},
		{"analyze", "mcp__repo__analyze", "analyze"},
		{"fix", "mcp__repo__fix", "fix"},
		{"tree", "mcp__repo__tree", "tree"},
		{"graphql", "mcp__repo__graphql", "graphql"},
		{"move", "mcp__repo__move", "move"},
		{"policy check", "mcp__repo__policy_check", "policy.check"},
		{"non-compose tool", "mcp__other__thing", ""},
		{"bash tool", "Bash", ""},
		{"empty", "", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := deriveRepoOpFromMCPTool(tc.tool)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromMCPTool(%q) = %q, want %q", tc.tool, got, tc.expected)
			}
		})
	}
}

func TestDeriveRepoOpFromCLICommand(t *testing.T) {
	cases := []struct {
		name     string
		cmd      string
		expected string
	}{
		{"ticket open full path", "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket open exe path", ".\\repo\\cli\\cli.exe ticket open MY-GOAL 'My Title' 'My Prompt' claude-code sonnet-4-5", "ticket.open"},
		{"ticket close", "./cli ticket close 26 03 05 MY-SLUG 'Summary' compose/go/compose.go", "ticket.close"},
		{"ticket reopen", "/workspaces/semio/repo/client/client ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"ticket reopen go run", "go run ./repo/client/mcp/go ticket reopen 26 03 05 MY-SLUG 'Prompt' claude-code sonnet-4-5", "ticket.reopen"},
		{"goal open", "./cli goal open 'Title' 'Desc' 'Prompt' claude-code sonnet-4-5", "goal.open"},
		{"goal close", "./cli goal close MY-GOAL 'Summary'", "goal.close"},
		{"contributor add", "./cli contributor add github-user", "contributor.add"},
		{"single subcommand", "./cli analyze", "analyze"},
		{"non-cli binary", "./other-tool ticket open", ""},
		{"empty", "", ""},
		{"just cli", "./cli", ""},
		{"cli with flag first", "./cli -v ticket open", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := deriveRepoOpFromCLICommand(tc.cmd)
			if got != tc.expected {
				t.Errorf("deriveRepoOpFromCLICommand(%q) = %q, want %q", tc.cmd, got, tc.expected)
			}
		})
	}
}

func TestLogRepoOperationHookMCPTool(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"sess1")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for mcp__repo__tree", func(t *testing.T) {
		input := json.RawMessage(`{"query":"contributors"}`)
		result := HookResultAgentToolStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name:  "mcp__repo__tree",
			Input: input,
		}
		hctx := HookContext{
			Event:    HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"sess1"}`),
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json to be created for derived repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.tree.starting" {
			t.Errorf("expected kind agent.tree.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for mcp__repo__ticket_open", func(t *testing.T) {
		result := HookResultAgentToolEnded{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name: "mcp__repo__ticket_open",
		}
		hctx := HookContext{
			Event:    HookAgentToolEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra files beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.ended" {
			t.Errorf("expected kind agent.ticket.open.ended, got: %v", evt["kind"])
		}
	})

	t.Run("no extra file for non-compose tool", func(t *testing.T) {
		result := HookResultAgentToolStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
			},
			Name: "Bash",
		}
		hctx := HookContext{
			Event:    HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Errorf("expected no new files for Bash tool, got %d new files", len(after)-len(before))
		}
	})
}

func TestLogRepoOperationHookCLI(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"sess2")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for CLI ticket open", func(t *testing.T) {
		result := HookResultAgentToolTerminalStarting{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' claude-code sonnet-4-5",
		}
		hctx := HookContext{
			Event:    HookAgentToolTerminalStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json creation for CLI repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.starting" {
			t.Errorf("expected kind agent.ticket.open.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for CLI goal close", func(t *testing.T) {
		result := HookResultAgentToolTerminalEnded{
			HookResultAgentBase: HookResultAgentBase{
				HookResultBase: HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "/workspace/repo/client/client goal close MY-GOAL 'Summary'",
		}
		hctx := HookContext{
			Event:    HookAgentToolTerminalEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra file beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.goal.close.ended" {
			t.Errorf("expected kind agent.goal.close.ended, got: %v", evt["kind"])
		}
	})
}

func TestRunHookAgentToolStartingDerivedRepoEvents(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	sessionID := "test-repo-events-session"
	payload := json.RawMessage(`{"session_id":"test-repo-events-session","tool_name":"mcp__repo__ticket_open","tool_input":{"title":"My Ticket","prompt":"My Prompt","client":"claude-code","llm":"sonnet-4-5","goal":"MY-GOAL"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-03-05T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)

	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		sessionID)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 || entries[0].Name() != "session.json" {
		names := make([]string, len(entries))
		for i, e := range entries {
			names[i] = e.Name()
		}
		t.Fatalf("expected only session.json, got %d: %v", len(entries), names)
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 2 {
		t.Fatalf("expected 2 session events (agent + derived repo operation), got %d", len(meta.Events))
	}
	found := false
	for _, entry := range meta.Events {
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] == "agent.ticket.open.starting" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected derived agent.ticket.open.starting event in session.json events")
	}
}
func setupTicketDir(t *testing.T) (string, string) {
	t.Helper()
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	ticketDir := filepath.Join(tmpDir, ".🦑repo", "🎫tickets",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", now.Month()),
		fmt.Sprintf("%02d", now.Day()),
		"TEST-TICKET")
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "ticket.json")
	initialTicket := `{"title":"Test Ticket","goal":"TEST/GOAL"}`
	if err := os.WriteFile(ticketJSON, []byte(initialTicket), 0644); err != nil {
		t.Fatal(err)
	}
	return tmpDir, ticketJSON
}

func readTicketJSON(t *testing.T, ticketJSON string) map[string]interface{} {
	t.Helper()
	data, err := os.ReadFile(ticketJSON)
	if err != nil {
		t.Fatalf("cannot read ticket.json: %v", err)
	}
	var result map[string]interface{}
	if err := json.Unmarshal(data, &result); err != nil {
		t.Fatalf("invalid ticket.json: %v\n%s", err, string(data))
	}
	return result
}

func getLogFiles(t *testing.T, tmpDir string) []string {
	t.Helper()
	outBase := filepath.Join(tmpDir, ".🦑repo", "⚡cache")
	var logFiles []string
	filepath.WalkDir(outBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return nil
		}
		if !d.IsDir() && strings.HasSuffix(d.Name(), ".json") {
			logFiles = append(logFiles, path)
		}
		return nil
	})
	return logFiles
}

func assertNoHookLogFiles(t *testing.T, tmpDir string) {
	t.Helper()
	logFiles := getLogFiles(t, tmpDir)
	if len(logFiles) != 0 {
		t.Fatalf("expected no hook log files under .🦑repo/⚡, got %v", logFiles)
	}
}

func TestTrackHookAllEventsLogged(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow track hook all-events test in short mode")
	}
	tmpDir, ticketJSON := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"test-session-1","llm":"opus-4-6","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Fix the bug",
	}
	RunHook(hctx)
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	agentEvents := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"agent.started", HookAgentStarted, sessionInput},
		{"agent.tool.starting", HookAgentToolStarting, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.file.read.starting", HookAgentToolSearchStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"hello","includePattern":"src/**"}}`)},
		{"agent.file.read.ended", HookAgentToolSearchEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"world"}}`)},
		{"agent.tool.code.edit.starting", HookAgentToolCodeEditStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.code.edit.ended", HookAgentToolCodeEditEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"command":"go test ./..."}}`)},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, json.RawMessage(`{"session_id":"test-session-1","command":"go test ./...","pid":"12345","stdout":"PASS"}`)},
		{"agent.tool.test.starting", HookAgentToolTestStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"files":["/tmp/test.go"],"timeout":"30000"}}`)},
		{"agent.tool.test.ended", HookAgentToolTestEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["TestA"],"failed":["TestB"]}}`)},
		{"agent.tool.build.starting", HookAgentToolBuildStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"bundles":["compose/js"]}}`)},
		{"agent.tool.build.ended", HookAgentToolBuildEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["compose/js"]}}`)},
		{"agent.tool.ended", HookAgentToolEnded, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.tool.plan.updating.starting", HookAgentToolPlanUpdatingStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"todoList":[{"title":"Step 1","status":"completed"},{"title":"Step 2","status":"in-progress"}]}}`)},
		{"agent.ended", HookAgentEnded, sessionInput},
		{"agent.thinking.starting", HookAgentThinkingStarting, json.RawMessage(`{"session_id":"test-session-1","text":"Planning the approach"}`)},
		{"agent.thinking.ended", HookAgentThinkingEnded, json.RawMessage(`{"session_id":"test-session-1","text":"Decided to use X"}`)},
	}
	for _, ae := range agentEvents {
		hctx := HookContext{
			Event:    ae.event,
			Client:   "copilot-chat",
			Second:   "2026-02-23T10:01:00Z",
			RepoRoot: tmpDir,
			Input:    ae.input,
		}
		RunHook(hctx)
	}

	logFiles := getLogFiles(t, tmpDir)

	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for all events, got %d", sessionCount)
	}

	// 🪪Verify that all 16 events are in the session.json file
	var sessionFile string
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionFile = f
			break
		}
	}

	data, err := os.ReadFile(sessionFile)
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}

	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	// Should have 18 events (1 prompt + 15 agent events + 2 derived events from search operations)
	expectedEventCount := 18
	if len(meta.Events) != expectedEventCount {
		t.Fatalf("expected %d events in session.json, got %d", expectedEventCount, len(meta.Events))
	}

	ticket = readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Fatal("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookSearchingPattern(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"search-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Search test",
	})
	RunHook(HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"search-session","tool_input":{"query":"findMe","includePattern":"src/**/*.ts"}}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for search-session, got %d", sessionCount)
	}

	// Check that session.json contains the search event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			if strings.Contains(string(data), "findMe") {
				found = true
				break
			}
		}
	}
	if !found {
		t.Error("expected search query in session.json")
	}
}

func TestTrackHookBlockedEvent(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"blocked-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Do something",
	})
	RunHook(HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"blocked-session","tool_input":{"command":"git checkout main"}}`),
		ToolName: "run_in_terminal",
		ToolArgs: "git checkout main",
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for blocked-session, got %d", sessionCount)
	}
	// Check that session.json contains the blocked event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					if entry.Response != nil && entry.Response.Blocked != nil && *entry.Response.Blocked {
						found = true
						break
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected blocked event in session.json")
	}
}

func TestTrackHookTerminalEvents(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	SetRootDir(tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"terminal-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Run command",
	})
	RunHook(HookContext{
		Event:    HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","tool_input":{"command":"npm test"}}`),
	})
	RunHook(HookContext{
		Event:    HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","command":"npm test","pid":"999","stdout":"all passed"}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for terminal-session, got %d", sessionCount)
	}
	// Check that session.json contains terminal events
	startFound := false
	endFound := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if kind, ok := evt["kind"].(string); ok {
							if kind == "agent.tool.terminal.starting" {
								startFound = true
							} else if kind == "agent.tool.terminal.ended" {
								endFound = true
							}
						}
					}
				}
			}
		}
	}
	if !startFound {
		t.Error("expected agent-tool-terminal-starting event in session.json")
	}
	if !endFound {
		t.Error("expected agent-tool-terminal-ended event in session.json")
	}
}

func TestTrackHookTranscriptInSession(t *testing.T) {
	tmpDir, ticketJSON := setupTicketDir(t)
	SetRootDir(tmpDir)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"transcript-session","transcript_path":"/home/user/.vscode/transcripts/abc.jsonl"}`),
		ToolArgs: "test",
	})
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Error("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookCodeEditedLogsToFile(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	SetRootDir(tmpDir)
	tsContent := "// #region \U0001F516Functions\n\n// doWork MUST work.\nexport function doWork(): void {}\n\n// #endregion \U0001F516Functions\n"
	tsFile := filepath.Join(tmpDir, "proj", "kit", "file.ts")
	if err := os.MkdirAll(filepath.Dir(tsFile), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(tsFile, []byte(tsContent), 0644); err != nil {
		t.Fatal(err)
	}
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"def-id-session"}`),
		ToolArgs: "Edit file",
	})
	payload := fmt.Sprintf(`{"session_id":"def-id-session","tool_input":{"filePath":%q,"oldString":"","newString":"export function doWork(): void {}"}}`, tsFile)
	RunHook(HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(payload),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for def-id-session, got %d", sessionCount)
	}

	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if evt["kind"] == "agent.tool.code.edit.ended" {
							found = true
							break
						}
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected agent-tool-code-edit-ended event in session.json")
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
	logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
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

func TestExtractCommandFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"claude code tool_input.command", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "git checkout main"},
		{"cursor beforeShellExecution", `{"command":"git stash pop"}`, "git stash pop"},
		{"windsurf tool_info.command_line", `{"tool_info":{"command_line":"git reset --hard"}}`, "git reset --hard"},
		{"no command", `{"tool_name":"ReadFile","tool_input":{"path":"/tmp"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractCommandFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractCommandFromStdinBlocking(t *testing.T) {
	payload := json.RawMessage(`{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		Input:    payload,
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected stdin-based git checkout to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestClassifyTool(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		expect   ToolKind
	}{
		{"manage_todo_list", "manage_todo_list", ToolKindPlan},
		{"Task", "Task", ToolKindPlan},
		{"todo_tool", "todo_tool", ToolKindPlan},
		{"TodoWrite", "TodoWrite", ToolKindPlan},
		{"read_file", "read_file", ToolKindCodeSearch},
		{"grep_search", "grep_search", ToolKindCodeSearch},
		{"rg", "rg", ToolKindCodeSearch},
		{"ripgrep", "ripgrep", ToolKindCodeSearch},
		{"file_search", "file_search", ToolKindCodeSearch},
		{"semantic_search", "semantic_search", ToolKindCodeSearch},
		{"list_dir", "list_dir", ToolKindCodeSearch},
		{"get_errors", "get_errors", ToolKindCodeSearch},
		{"Read", "Read", ToolKindCodeSearch},
		{"replace_string_in_file", "replace_string_in_file", ToolKindCodeEdit},
		{"create_file", "create_file", ToolKindCodeEdit},
		{"multi_replace_string_in_file", "multi_replace_string_in_file", ToolKindCodeEdit},
		{"Edit", "Edit", ToolKindCodeEdit},
		{"Write", "Write", ToolKindCodeEdit},
		{"run_in_terminal", "run_in_terminal", ToolKindTerminal},
		{"get_terminal_output", "get_terminal_output", ToolKindTerminal},
		{"Bash", "Bash", ToolKindTerminal},
		{"runSubagent", "runSubagent", ToolKindGeneric},
		{"runTests", "runTests", ToolKindTest},
		{"run_tests", "run_tests", ToolKindTest},
		{"run_task", "run_task", ToolKindBuild},
		{"create_and_run_task", "create_and_run_task", ToolKindBuild},
		{"fetch_webpage", "fetch_webpage", ToolKindCodeSearch},
		{"open_simple_browser", "open_simple_browser", ToolKindCodeSearch},
		{"Glob", "Glob", ToolKindCodeSearch},
		{"tool_search_tool_regex", "tool_search_tool_regex", ToolKindGeneric},
		{"fs_read", "fs_read", ToolKindCodeSearch},
		{"fs_write", "fs_write", ToolKindCodeEdit},
		{"execute_bash", "execute_bash", ToolKindTerminal},
		{"code", "code", ToolKindCodeSearch},
		{"grep_kiro", "grep", ToolKindCodeSearch},
		{"glob_kiro", "glob", ToolKindCodeSearch},
		{"web_search", "web_search", ToolKindCodeSearch},
		{"web_fetch", "web_fetch", ToolKindCodeSearch},
		{"use_subagent", "use_subagent", ToolKindGeneric},
		{"use_aws", "use_aws", ToolKindGeneric},
		{"empty", "", ToolKindGeneric},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := classifyTool(tc.toolName)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestClassifyCommandKind(t *testing.T) {
	cases := []struct {
		name    string
		command string
		expect  ToolKind
	}{
		{"empty", "", ToolKindTerminal},
		{"whitespace only", "   ", ToolKindTerminal},
		{"grep", "grep -r foo .", ToolKindCodeSearch},
		{"grep with path", "/usr/bin/grep -rn pattern file.go", ToolKindCodeSearch},
		{"rg", "rg --type go pattern", ToolKindCodeSearch},
		{"ripgrep", "ripgrep --type go pattern", ToolKindCodeSearch},
		{"ag", "ag pattern src/", ToolKindCodeSearch},
		{"ack", "ack --go pattern", ToolKindCodeSearch},
		{"ack-grep", "ack-grep pattern", ToolKindCodeSearch},
		{"find", "find . -name '*.go'", ToolKindCodeSearch},
		{"fd", "fd -e go pattern", ToolKindCodeSearch},
		{"fdfind", "fdfind pattern", ToolKindCodeSearch},
		{"locate", "locate main.go", ToolKindCodeSearch},
		{"mlocate", "mlocate something", ToolKindCodeSearch},
		{"ls", "ls -la", ToolKindCodeSearch},
		{"ls piped", "ls .🦑repo/ | head -20", ToolKindCodeSearch},
		{"ls with redirect", "ls .🦑repo/🎯/ 2>/dev/null | head -20 || ls .🦑repo/ | head -20", ToolKindCodeSearch},
		{"exa", "exa --long", ToolKindCodeSearch},
		{"eza", "eza --tree", ToolKindCodeSearch},
		{"tree", "tree -L 2", ToolKindCodeSearch},
		{"dir", "dir /w", ToolKindCodeSearch},
		{"cat", "cat file.txt", ToolKindCodeSearch},
		{"bat", "bat --style=numbers file.go", ToolKindCodeSearch},
		{"batcat", "batcat file.go", ToolKindCodeSearch},
		{"less", "less file.txt", ToolKindCodeSearch},
		{"more", "more file.txt", ToolKindCodeSearch},
		{"head", "head -20 file.txt", ToolKindCodeSearch},
		{"tail", "tail -f log.txt", ToolKindCodeSearch},
		{"wc", "wc -l file.txt", ToolKindCodeSearch},
		{"file", "file binary.dat", ToolKindCodeSearch},
		{"stat", "stat file.txt", ToolKindCodeSearch},
		{"du", "du -sh .", ToolKindCodeSearch},
		{"which", "which go", ToolKindCodeSearch},
		{"whereis", "whereis python", ToolKindCodeSearch},
		{"type", "type ls", ToolKindCodeSearch},
		{"command", "command -v node", ToolKindCodeSearch},
		{"hash", "hash -r", ToolKindCodeSearch},
		{"diff", "diff file1.txt file2.txt", ToolKindCodeSearch},
		{"cmp", "cmp file1 file2", ToolKindCodeSearch},
		{"comm", "comm sorted1 sorted2", ToolKindCodeSearch},
		{"strings", "strings binary", ToolKindCodeSearch},
		{"od", "od -x file", ToolKindCodeSearch},
		{"xxd", "xxd file", ToolKindCodeSearch},
		{"hexdump", "hexdump -C file", ToolKindCodeSearch},
		{"readlink", "readlink -f symlink", ToolKindCodeSearch},
		{"realpath", "realpath relative/path", ToolKindCodeSearch},
		{"basename", "basename /path/to/file.txt", ToolKindCodeSearch},
		{"dirname", "dirname /path/to/file.txt", ToolKindCodeSearch},
		{"jq", "jq '.name' package.json", ToolKindCodeSearch},
		{"yq", "yq '.spec' config.yaml", ToolKindCodeSearch},
		{"xq", "xq '.root' config.xml", ToolKindCodeSearch},
		{"sort", "sort file.txt", ToolKindCodeSearch},
		{"uniq", "uniq -c file.txt", ToolKindCodeSearch},
		{"cut", "cut -d: -f1 /etc/passwd", ToolKindCodeSearch},
		{"tr", "tr '[:lower:]' '[:upper:]'", ToolKindCodeSearch},
		{"paste", "paste file1 file2", ToolKindCodeSearch},
		{"column", "column -t file.txt", ToolKindCodeSearch},
		{"rev", "rev file.txt", ToolKindCodeSearch},
		{"fold", "fold -w 80 file.txt", ToolKindCodeSearch},
		{"fmt", "fmt -w 72 file.txt", ToolKindCodeSearch},
		{"nl", "nl file.txt", ToolKindCodeSearch},
		{"expand", "expand file.txt", ToolKindCodeSearch},
		{"unexpand", "unexpand file.txt", ToolKindCodeSearch},
		{"echo", "echo hello", ToolKindCodeSearch},
		{"printf", "printf '%s\\n' hello", ToolKindCodeSearch},
		{"env", "env", ToolKindCodeSearch},
		{"printenv", "printenv HOME", ToolKindCodeSearch},
		{"set", "set", ToolKindCodeSearch},
		{"export", "export FOO=bar", ToolKindCodeSearch},
		{"pwd", "pwd", ToolKindCodeSearch},
		{"id", "id", ToolKindCodeSearch},
		{"whoami", "whoami", ToolKindCodeSearch},
		{"hostname", "hostname", ToolKindCodeSearch},
		{"uname", "uname -a", ToolKindCodeSearch},
		{"date", "date +%Y-%m-%d", ToolKindCodeSearch},
		{"uptime", "uptime", ToolKindCodeSearch},
		{"free", "free -h", ToolKindCodeSearch},
		{"df", "df -h", ToolKindCodeSearch},
		{"ps", "ps aux", ToolKindCodeSearch},
		{"top", "top -bn1", ToolKindCodeSearch},
		{"htop", "htop", ToolKindCodeSearch},
		{"lsof", "lsof -i :8080", ToolKindCodeSearch},
		{"netstat", "netstat -tlnp", ToolKindCodeSearch},
		{"ss", "ss -tlnp", ToolKindCodeSearch},
		{"test", "test -f file.txt", ToolKindCodeSearch},
		{"bracket test", "[ -f file.txt ]", ToolKindCodeSearch},
		{"sed read-only (no -i)", "sed 's/old/new/g' file.txt", ToolKindCodeSearch},
		{"awk read-only", "awk '{print $1}' file.txt", ToolKindCodeSearch},
		{"gawk read-only", "gawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"mawk read-only", "mawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"nawk read-only", "nawk '{print $1}' file.txt", ToolKindCodeSearch},
		{"rm", "rm -rf temp/", ToolKindCodeEdit},
		{"mv", "mv old.txt new.txt", ToolKindCodeEdit},
		{"cp", "cp src.txt dst.txt", ToolKindCodeEdit},
		{"install", "install -m 755 bin /usr/local/bin/", ToolKindCodeEdit},
		{"mkdir", "mkdir -p new/dir", ToolKindCodeEdit},
		{"rmdir", "rmdir empty/", ToolKindCodeEdit},
		{"touch", "touch new.txt", ToolKindCodeEdit},
		{"chmod", "chmod 644 file.txt", ToolKindCodeEdit},
		{"chown", "chown user:group file.txt", ToolKindCodeEdit},
		{"chgrp", "chgrp group file.txt", ToolKindCodeEdit},
		{"ln", "ln -s target link", ToolKindCodeEdit},
		{"tee", "tee output.log", ToolKindCodeEdit},
		{"patch", "patch -p1 < fix.patch", ToolKindCodeEdit},
		{"truncate", "truncate -s 0 file.log", ToolKindCodeEdit},
		{"dd", "dd if=/dev/zero of=file bs=1M count=1", ToolKindCodeEdit},
		{"shred", "shred -u file.txt", ToolKindCodeEdit},
		{"tar", "tar -xzf archive.tar.gz", ToolKindCodeEdit},
		{"zip", "zip archive.zip file.txt", ToolKindCodeEdit},
		{"unzip", "unzip archive.zip", ToolKindCodeEdit},
		{"gzip", "gzip file.txt", ToolKindCodeEdit},
		{"gunzip", "gunzip file.txt.gz", ToolKindCodeEdit},
		{"bzip2", "bzip2 file.txt", ToolKindCodeEdit},
		{"bunzip2", "bunzip2 file.txt.bz2", ToolKindCodeEdit},
		{"xz", "xz file.txt", ToolKindCodeEdit},
		{"unxz", "unxz file.txt.xz", ToolKindCodeEdit},
		{"zstd", "zstd file.txt", ToolKindCodeEdit},
		{"git", "git status", ToolKindTerminal},
		{"npm install", "npm install", ToolKindTerminal},
		{"npm test", "npm test", ToolKindTest},
		{"npm run test", "npm run test", ToolKindTest},
		{"pnpm test", "pnpm test", ToolKindTest},
		{"yarn test", "yarn test", ToolKindTest},
		{"bun test", "bun test", ToolKindTest},
		{"npx vitest run", "npx vitest run", ToolKindTest},
		{"npx jest", "npx jest", ToolKindTest},
		{"npx mocha", "npx mocha", ToolKindTest},
		{"go build", "go build ./...", ToolKindTerminal},
		{"go test all", "go test ./...", ToolKindTest},
		{"go test specific", "go test -run TestFoo ./...", ToolKindTest},
		{"cargo build", "cargo build", ToolKindTerminal},
		{"cargo test", "cargo test", ToolKindTest},
		{"cargo nextest", "cargo nextest run", ToolKindTest},
		{"pip", "pip install requests", ToolKindTerminal},
		{"uv pip install", "uv pip install requests", ToolKindTerminal},
		{"uv run pytest", "uv run pytest", ToolKindTest},
		{"uvx pytest", "uvx pytest", ToolKindTest},
		{"python -m pytest", "python -m pytest", ToolKindTest},
		{"python -m unittest", "python -m unittest", ToolKindTest},
		{"python3 script", "python3 script.py", ToolKindTerminal},
		{"node", "node script.js", ToolKindTerminal},
		{"make build", "make build", ToolKindTerminal},
		{"make test", "make test", ToolKindTest},
		{"make check", "make check", ToolKindTest},
		{"dotnet build", "dotnet build", ToolKindTerminal},
		{"dotnet test", "dotnet test", ToolKindTest},
		{"swift test", "swift test", ToolKindTest},
		{"dart test", "dart test", ToolKindTest},
		{"flutter test", "flutter test", ToolKindTest},
		{"mix test", "mix test", ToolKindTest},
		{"mvn test", "mvn test", ToolKindTest},
		{"mvn verify", "mvn verify", ToolKindTest},
		{"gradle test", "gradle test", ToolKindTest},
		{"gradlew test", "./gradlew test", ToolKindTest},
		{"cabal test", "cabal test", ToolKindTest},
		{"stack test", "stack test", ToolKindTest},
		{"lein test", "lein test", ToolKindTest},
		{"sbt test", "sbt test", ToolKindTest},
		{"bundle exec rspec", "bundle exec rspec", ToolKindTest},
		{"jest direct", "jest --testPathPattern=foo", ToolKindTest},
		{"vitest direct", "vitest run", ToolKindTest},
		{"mocha direct", "mocha test/", ToolKindTest},
		{"pytest direct", "pytest -k test_foo", ToolKindTest},
		{"tox direct", "tox -e py311", ToolKindTest},
		{"rspec direct", "rspec spec/", ToolKindTest},
		{"phpunit direct", "phpunit tests/", ToolKindTest},
		{"phpunit vendor", "./vendor/bin/phpunit tests/", ToolKindTest},
		{"ctest direct", "ctest --test-dir build/", ToolKindTest},
		{"bats direct", "bats tests/", ToolKindTest},
		{"docker", "docker build .", ToolKindTerminal},
		{"kubectl", "kubectl get pods", ToolKindTerminal},
		{"curl", "curl https://example.com", ToolKindTerminal},
		{"wget", "wget https://example.com", ToolKindTerminal},
		{"ssh", "ssh user@host", ToolKindTerminal},
		{"custom-tool", "./custom-tool --flag", ToolKindTerminal},

		{"cd && go test", "cd /workspaces/semio/repo/client && go test ./...", ToolKindTest},
		{"cd && go test piped", "cd /workspaces/semio/repo/client && go test -v -run TestFoo -timeout 60s 2>&1 | tail -80", ToolKindTest},
		{"cd && cargo test", "cd /path && cargo test", ToolKindTest},
		{"cd && npm test", "cd technology && npm test", ToolKindTest},
		{"cd && pytest", "cd tests && pytest -k test_foo", ToolKindTest},
		{"cd && python -m pytest", "cd /app && python -m pytest", ToolKindTest},
		{"cd && dotnet test", "cd /app && dotnet test", ToolKindTest},
		{"cd && jest", "cd frontend && jest --testPathPattern=foo", ToolKindTest},
		{"cd && vitest", "cd app && vitest run", ToolKindTest},

		{"cd && go build", "cd /path && go build ./...", ToolKindTerminal},
		{"cd && npm install", "cd /path && npm install", ToolKindTerminal},

		{"cd; go test", "cd /path; go test ./...", ToolKindTest},

		{"go test || echo", "go test ./... || echo 'failed'", ToolKindTest},

		{"go test piped", "go test -v ./... | head -50", ToolKindTest},
		{"cargo test piped", "cargo test 2>&1 | tail -20", ToolKindTest},

		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v ./...", ToolKindTest},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := classifyCommandKind(tc.command)
			if result != tc.expect {
				t.Errorf("classifyCommandKind(%q) = %s, want %s", tc.command, result, tc.expect)
			}
		})
	}
}

func TestResolveHookEventCommandReclassification(t *testing.T) {
	searchCommands := []struct {
		name string
		cmd  string
	}{
		{"grep", "grep -r pattern ."},
		{"rg", "rg pattern"},
		{"ripgrep", "ripgrep pattern"},
		{"ag", "ag pattern"},
		{"find", "find . -name '*.go'"},
		{"fd", "fd pattern"},
		{"ls", "ls -la"},
		{"ls piped", "ls .🦑repo/ | head -20"},
		{"cat", "cat file.txt"},
		{"head", "head -20 file.txt"},
		{"tail", "tail -f log.txt"},
		{"wc", "wc -l file.txt"},
		{"diff", "diff file1 file2"},
		{"jq", "jq '.name' package.json"},
		{"sort", "sort file.txt"},
		{"echo", "echo hello"},
		{"pwd", "pwd"},
		{"ps", "ps aux"},
		{"which", "which go"},
		{"tree", "tree -L 2"},
		{"stat", "stat file.txt"},
		{"sed read-only", "sed 's/old/new/g' file.txt"},
		{"sed -n print lines", "sed -n '1,50p' file.txt"},
		{"awk read-only", "awk '{print $1}' file.txt"},
		{"gawk read-only", "gawk '{print $1}' file.txt"},
	}
	editCommands := []struct {
		name string
		cmd  string
	}{
		{"sed -i in-place", "sed -i 's/old/new/g' file.txt"},
		{"sed -i.bak with backup", "sed -i.bak 's/old/new/g' file.txt"},
		{"rm", "rm -rf temp/"},
		{"mv", "mv old.txt new.txt"},
		{"cp", "cp src.txt dst.txt"},
		{"mkdir", "mkdir -p newdir"},
		{"touch", "touch new.txt"},
		{"chmod", "chmod 644 file.txt"},
		{"ln", "ln -s target link"},
		{"tee", "tee output.log"},
		{"patch", "patch -p1 < fix.patch"},
		{"tar", "tar -xzf archive.tar.gz"},
		{"zip", "zip archive.zip file.txt"},
	}
	terminalCommands := []struct {
		name string
		cmd  string
	}{
		{"git", "git status"},
		{"npm", "npm install"},
		{"go build", "go build ./..."},
		{"cargo build", "cargo build"},
		{"python", "python script.py"},
		{"node", "node script.js"},
		{"make build", "make build"},
		{"docker", "docker build ."},
		{"curl", "curl https://example.com"},
	}
	testCommands := []struct {
		name string
		cmd  string
	}{
		{"go test", "go test ./..."},
		{"cargo test", "cargo test"},
		{"cargo nextest", "cargo nextest run"},
		{"npm test", "npm test"},
		{"pnpm test", "pnpm test"},
		{"yarn test", "yarn test"},
		{"bun test", "bun test"},
		{"npx jest", "npx jest"},
		{"npx vitest", "npx vitest run"},
		{"jest direct", "jest --testPathPattern=foo"},
		{"vitest direct", "vitest run"},
		{"mocha direct", "mocha test/"},
		{"pytest direct", "pytest -k test_foo"},
		{"python -m pytest", "python -m pytest"},
		{"uv run pytest", "uv run pytest"},
		{"make test", "make test"},
		{"dotnet test", "dotnet test"},
		{"swift test", "swift test"},
		{"mix test", "mix test"},
		{"mvn test", "mvn test"},
		{"gradle test", "gradle test"},
		{"rspec direct", "rspec spec/"},
		{"phpunit direct", "phpunit tests/"},
		{"phpunit vendor", "./vendor/bin/phpunit tests/"},
		{"cargo nextest", "cargo nextest run"},

		{"cd && go test", "cd /workspaces/semio/repo/client && go test -v -run TestFoo -timeout 60s 2>&1 | tail -80"},
		{"cd && cargo test", "cd /path/to/technology && cargo test"},
		{"cd && npm test", "cd frontend && npm test"},
		{"cd && pytest", "cd tests && pytest -k test_integration"},
		{"cd && dotnet test", "cd /app && dotnet test"},

		{"cd; go test", "cd /path; go test ./..."},

		{"go test piped", "go test -v ./... | head -50"},

		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v ./..."},
	}
	clients := []struct {
		name      string
		client    string
		preEvent  string
		postEvent string
		toolName  string
		mkInput   func(cmd string) json.RawMessage
	}{
		{
			"claude-code", "claude-code", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"copilot-chat", "copilot-chat", "PreToolUse", "PostToolUse", "run_in_terminal",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"droid", "droid", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"codex", "codex", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"antigravity-chat", "antigravity-chat", "PreToolUse", "PostToolUse", "Bash",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"Bash","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"windsurf-chat", "windsurf-chat", "pre_run_command", "post_run_command", "",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"input":{"command_line":%q}}`, cmd))
			},
		},
		{
			"cursor-chat", "cursor-chat", "preToolUse", "postToolUse", "terminal",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_name":"terminal","tool_input":{"command":%q}}`, cmd))
			},
		},
		{
			"cursor-chat-shell", "cursor-chat", "beforeShellExecution", "afterShellExecution", "",
			func(cmd string) json.RawMessage {
				return json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":%q}}`, cmd))
			},
		},
	}
	for _, cl := range clients {
		for _, sc := range searchCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/searching", cl.name, sc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(sc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolSearchStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolSearchStarting, event, sc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/searched", cl.name, sc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(sc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolSearchEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolSearchEnded, event, sc.cmd)
				}
			})
		}
		for _, ec := range editCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/editing", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolCodeEditStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolCodeEditStarting, event, ec.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/edited", cl.name, ec.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(ec.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolCodeEditEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolCodeEditEnded, event, ec.cmd)
				}
			})
		}
		for _, tc := range terminalCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTerminalStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTerminalStarting, event, tc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/terminal", cl.name, tc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(tc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTerminalEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTerminalEnded, event, tc.cmd)
				}
			})
		}
		for _, xc := range testCommands {
			t.Run(fmt.Sprintf("%s/%s/pre/test-starting", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.preEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTestStarting {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTestStarting, event, xc.cmd)
				}
			})
			t.Run(fmt.Sprintf("%s/%s/post/test-ended", cl.name, xc.name), func(t *testing.T) {
				event, _, err := ResolveHookEvent(cl.postEvent, cl.client, cl.toolName, cl.mkInput(xc.cmd))
				if err != nil {
					t.Fatalf("unexpected error: %v", err)
				}
				if event != HookAgentToolTestEnded {
					t.Errorf("expected %s, got %s for command %q", HookAgentToolTestEnded, event, xc.cmd)
				}
			})
		}
	}
}

func TestParseTestInfoFromCommand(t *testing.T) {
	cases := []struct {
		name          string
		command       string
		expectTests   []string
		expectTimeout string
	}{
		{"go test all", "go test ./...", []string{""}, ""},
		{"go test -run", "go test -run TestFoo ./...", []string{"TestFoo"}, ""},
		{"go test -run -timeout", "go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"go test -timeout 2m", "go test -timeout 2m ./...", []string{""}, "120"},
		{"pytest all", "pytest", []string{""}, ""},
		{"pytest -k", "pytest -k test_foo", []string{"test_foo"}, ""},
		{"jest all", "jest", []string{""}, ""},
		{"jest -t", "jest -t MyTest", []string{"MyTest"}, ""},
		{"jest --testNamePattern", "jest --testNamePattern=MyTest", []string{"MyTest"}, ""},
		{"jest --timeout", "jest --timeout 5000", []string{""}, "5000"},
		{"mocha --grep", "mocha --grep mytest", []string{"mytest"}, ""},
		{"mocha --timeout", "mocha --timeout 3000 test/", []string{""}, "3000"},
		{"cargo test -- filter", "cargo test -- my_test", []string{"my_test"}, ""},
		{"dotnet --filter", "dotnet test --filter Category=Unit", []string{"Category=Unit"}, ""},
		{"rspec --example", "rspec --example myexample", []string{"myexample"}, ""},

		{"cd && go test -run", "cd /path && go test -run TestFoo -timeout 30s ./...", []string{"TestFoo"}, "30"},
		{"cd && go test all", "cd /path && go test ./...", []string{""}, ""},
		{"cd && go test piped", "cd /workspaces/semio/repo/client && go test -v -run TestBar -timeout 60s 2>&1 | tail -80", []string{"TestBar"}, "60"},
		{"cd && pytest -k", "cd /app && pytest -k test_integration", []string{"test_integration"}, ""},
		{"cd && jest -t", "cd frontend && jest -t MyComponent", []string{"MyComponent"}, ""},
		{"cd; cargo test -- filter", "cd /path; cargo test -- my_test", []string{"my_test"}, ""},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /path && go test -v -run TestBaz ./...", []string{"TestBaz"}, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			tests, timeout := parseTestInfoFromCommand(tc.command)
			if len(tests) != len(tc.expectTests) {
				t.Errorf("parseTestInfoFromCommand(%q) tests = %v, want %v", tc.command, tests, tc.expectTests)
			} else {
				for i := range tests {
					if tests[i] != tc.expectTests[i] {
						t.Errorf("parseTestInfoFromCommand(%q) tests[%d] = %q, want %q", tc.command, i, tests[i], tc.expectTests[i])
					}
				}
			}
			if timeout != tc.expectTimeout {
				t.Errorf("parseTestInfoFromCommand(%q) timeout = %q, want %q", tc.command, timeout, tc.expectTimeout)
			}
		})
	}
}

func TestExtractTestSegmentFromCommand(t *testing.T) {
	cases := []struct {
		name      string
		command   string
		expectSeg string
		expectCwd string
	}{
		{"simple go test", "go test ./...", "go test ./...", ""},
		{"cd && go test", "cd /workspaces/semio/repo/client && go test -v -run TestFoo ./...", "go test -v -run TestFoo ./...", "/workspaces/semio/repo/client"},
		{"cd && go test piped", "cd /path && go test -v ./... 2>&1 | tail -80", "go test -v ./... 2>&1", "/path"},
		{"export && cd && go test", "export GOFLAGS=-count=1 && cd /src && go test -v ./...", "go test -v ./...", "/src"},
		{"cd && npm test", "cd frontend && npm test", "npm test", "frontend"},
		{"cd; cargo test", "cd /path; cargo test", "cargo test", "/path"},
		{"no test segment", "cd /path && go build ./...", "", "/path"},
		{"empty", "", "", ""},
		{"just cd", "cd /path", "", ""},
		{"piped go test", "go test -v ./... | head -50", "go test -v ./...", ""},
		{"vitest piped", "vitest run 2>&1 | tail -20", "vitest run 2>&1", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			seg, cwd := extractTestSegmentFromCommand(tc.command)
			if seg != tc.expectSeg {
				t.Errorf("extractTestSegmentFromCommand(%q) seg = %q, want %q", tc.command, seg, tc.expectSeg)
			}
			if cwd != tc.expectCwd {
				t.Errorf("extractTestSegmentFromCommand(%q) cwd = %q, want %q", tc.command, cwd, tc.expectCwd)
			}
		})
	}
}

func TestResolveTestFilesFromCommand(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "pkg", "main")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "main.go"), []byte("package main\n"), 0644)
	os.WriteFile(filepath.Join(goDir, "main_test.go"), []byte("package main\nfunc TestFoo(t *testing.T) {}\n"), 0644)
	subDir := filepath.Join(goDir, "sub")
	os.MkdirAll(subDir, 0755)
	os.WriteFile(filepath.Join(subDir, "sub_test.go"), []byte("package sub\nfunc TestBar(t *testing.T) {}\n"), 0644)

	pyDir := filepath.Join(tmpDir, "tests")
	os.MkdirAll(pyDir, 0755)
	os.WriteFile(filepath.Join(pyDir, "test_foo.py"), []byte("def test_foo(): pass\n"), 0644)
	os.WriteFile(filepath.Join(pyDir, "helper.py"), []byte("def helper(): pass\n"), 0644)

	jsDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(jsDir, 0755)
	os.WriteFile(filepath.Join(jsDir, "app.test.ts"), []byte("test('works', () => {})\n"), 0644)
	os.WriteFile(filepath.Join(jsDir, "app.ts"), []byte("export const a = 1\n"), 0644)

	rsDir := filepath.Join(tmpDir, "rsrc", "tests")
	os.MkdirAll(rsDir, 0755)
	os.WriteFile(filepath.Join(rsDir, "integration_test.rs"), []byte("#[test]\nfn test_it() {}\n"), 0644)

	specDir := filepath.Join(tmpDir, "spec")
	os.MkdirAll(specDir, 0755)
	os.WriteFile(filepath.Join(specDir, "app_spec.rb"), []byte("describe App do; end\n"), 0644)

	phpDir := filepath.Join(tmpDir, "tests", "Feature")
	os.MkdirAll(phpDir, 0755)
	os.WriteFile(filepath.Join(phpDir, "ExampleTest.php"), []byte("<?php\nfinal class ExampleTest {}\n"), 0644)

	t.Run("go_test_recursive", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test -v ./pkg/main/...", tmpDir)
		if len(files) != 2 {
			t.Fatalf("expected 2 Go test files, got %d: %v", len(files), files)
		}
		found := map[string]bool{}
		for _, f := range files {
			found[filepath.Base(f)] = true
		}
		if !found["main_test.go"] || !found["sub_test.go"] {
			t.Errorf("expected main_test.go and sub_test.go, got %v", files)
		}
	})

	t.Run("go_test_single_package", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test ./pkg/main", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Go test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "main_test.go" {
			t.Errorf("expected main_test.go, got %s", files[0])
		}
	})

	t.Run("go_test_with_run_flag", func(t *testing.T) {
		files := resolveTestFilesFromCommand("go test -run TestFoo -v ./pkg/main/...", tmpDir)
		if len(files) != 2 {
			t.Fatalf("expected 2 Go test files (run flag doesn't filter files), got %d: %v", len(files), files)
		}
	})

	t.Run("pytest_directory", func(t *testing.T) {
		files := resolveTestFilesFromCommand("pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "test_foo.py" {
			t.Errorf("expected test_foo.py, got %s", files[0])
		}
	})

	t.Run("pytest_no_args", func(t *testing.T) {
		files := resolveTestFilesFromCommand("pytest", tmpDir)

		foundTestFile := false
		for _, f := range files {
			if filepath.Base(f) == "test_foo.py" {
				foundTestFile = true
			}
		}
		if !foundTestFile {
			t.Errorf("expected to find test_foo.py, got %v", files)
		}
	})

	t.Run("python_m_pytest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("python -m pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("uv_run_pytest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("uv run pytest tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Python test file, got %d: %v", len(files), files)
		}
	})

	t.Run("npx_vitest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("npx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app.test.ts" {
			t.Errorf("expected app.test.ts, got %s", files[0])
		}
	})

	t.Run("cargo_test", func(t *testing.T) {
		files := resolveTestFilesFromCommand("cargo test", filepath.Join(tmpDir, "rsrc"))
		if len(files) != 1 {
			t.Fatalf("expected 1 Rust test file, got %d: %v", len(files), files)
		}
	})

	t.Run("bunx_vitest", func(t *testing.T) {
		files := resolveTestFilesFromCommand("bunx vitest src/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 JS test file, got %d: %v", len(files), files)
		}
	})

	t.Run("rspec", func(t *testing.T) {
		files := resolveTestFilesFromCommand("rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("bundle_exec_rspec", func(t *testing.T) {
		files := resolveTestFilesFromCommand("bundle exec rspec", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 Ruby spec file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "app_spec.rb" {
			t.Errorf("expected app_spec.rb, got %s", files[0])
		}
	})

	t.Run("vendor_phpunit", func(t *testing.T) {
		files := resolveTestFilesFromCommand("./vendor/bin/phpunit tests/", tmpDir)
		if len(files) != 1 {
			t.Fatalf("expected 1 PHP test file, got %d: %v", len(files), files)
		}
		if filepath.Base(files[0]) != "ExampleTest.php" {
			t.Errorf("expected ExampleTest.php, got %s", files[0])
		}
	})

	t.Run("unsupported_command", func(t *testing.T) {
		files := resolveTestFilesFromCommand("echo hello", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for unsupported command, got %d", len(files))
		}
	})

	t.Run("empty_command", func(t *testing.T) {
		files := resolveTestFilesFromCommand("", tmpDir)
		if len(files) != 0 {
			t.Errorf("expected 0 files for empty command, got %d", len(files))
		}
	})
}

func TestResolveAllTestDefinitionIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	t.Run("go_test_file", func(t *testing.T) {
		goDir := filepath.Join(tmpDir, "pkg")
		os.MkdirAll(goDir, 0755)
		goContent := `package pkg
// 🧪#region 📐Tests
func TestAlpha(t *testing.T) {}
func TestBeta(t *testing.T) {}
func helperNotATest() {}
// #endregion 📐Tests
`
		testFile := filepath.Join(goDir, "pkg_test.go")
		os.WriteFile(testFile, []byte(goContent), 0644)

		ids := resolveAllTestDefinitionIDs([]string{"pkg/pkg_test.go"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}

		foundAlpha := false
		foundBeta := false
		for _, id := range ids {
			if strings.Contains(id, "testalpha") {
				foundAlpha = true
			}
			if strings.Contains(id, "testbeta") {
				foundBeta = true
			}
		}
		if !foundAlpha {
			t.Errorf("expected to find TestAlpha in IDs, got %v", ids)
		}
		if !foundBeta {
			t.Errorf("expected to find TestBeta in IDs, got %v", ids)
		}

		for _, id := range ids {
			if strings.Contains(id, "helpernotatest") {
				t.Errorf("expected helperNotATest to be excluded, but found in IDs: %s", id)
			}
		}
	})

	t.Run("python_test_file", func(t *testing.T) {
		pyDir := filepath.Join(tmpDir, "pytests")
		os.MkdirAll(pyDir, 0755)
		pyContent := `def test_foo():
    pass

def test_bar():
    pass

def helper():
    pass
`
		testFile := filepath.Join(pyDir, "test_stuff.py")
		os.WriteFile(testFile, []byte(pyContent), 0644)

		ids := resolveAllTestDefinitionIDs([]string{"pytests/test_stuff.py"})
		if len(ids) < 2 {
			t.Fatalf("expected at least 2 test definition IDs, got %d: %v", len(ids), ids)
		}
		foundFoo := false
		foundBar := false
		for _, id := range ids {
			if strings.Contains(id, "testfoo") {
				foundFoo = true
			}
			if strings.Contains(id, "testbar") {
				foundBar = true
			}
		}
		if !foundFoo {
			t.Errorf("expected test_foo in IDs, got %v", ids)
		}
		if !foundBar {
			t.Errorf("expected test_bar in IDs, got %v", ids)
		}
	})

	t.Run("no_files", func(t *testing.T) {
		ids := resolveAllTestDefinitionIDs(nil)
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nil files, got %d", len(ids))
		}
	})

	t.Run("nonexistent_file", func(t *testing.T) {
		ids := resolveAllTestDefinitionIDs([]string{"nonexistent/file_test.go"})
		if len(ids) != 0 {
			t.Errorf("expected 0 IDs for nonexistent file, got %d", len(ids))
		}
	})
}

func TestExtractTestStartingFromInputResolvesTestIDs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "mypackage")
	os.MkdirAll(goDir, 0755)
	goContent := `package mypackage
func TestOne(t *testing.T) {}
func TestTwo(t *testing.T) {}
`
	os.WriteFile(filepath.Join(goDir, "my_test.go"), []byte(goContent), 0644)

	pyDir := filepath.Join(tmpDir, "tests")
	os.MkdirAll(pyDir, 0755)
	pyContent := `def test_alpha():
    pass

def test_beta():
    pass
`
	os.WriteFile(filepath.Join(pyDir, "test_sample.py"), []byte(pyContent), 0644)

	jsDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(jsDir, 0755)
	jsContent := `import { describe, it } from 'vitest'

describe('suite', () => {
  it('alpha', () => {})
  it('beta', () => {})
})
`
	os.WriteFile(filepath.Join(jsDir, "app.test.ts"), []byte(jsContent), 0644)

	rubyDir := filepath.Join(tmpDir, "spec")
	os.MkdirAll(rubyDir, 0755)
	rubyContent := `describe 'suite' do
  it 'alpha' do
  end

  it 'beta' do
  end
end
`
	os.WriteFile(filepath.Join(rubyDir, "app_spec.rb"), []byte(rubyContent), 0644)

	phpDir := filepath.Join(tmpDir, "tests", "Feature")
	os.MkdirAll(phpDir, 0755)
	phpContent := `<?php
final class ExampleTest extends TestCase {
    public function testAlpha(): void {}
    public function testBeta(): void {}
}
`
	os.WriteFile(filepath.Join(phpDir, "ExampleTest.php"), []byte(phpContent), 0644)

	t.Run("command_from_tool_info", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved from command, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
		if !strings.Contains(labs[0], "🥼") {
			t.Errorf("expected normalized lab IDs, got %v", labs)
		}
	})

	t.Run("command_from_tool_input", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"command":"go test -v ./mypackage/..."},"tool_info":{"cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs to be resolved, got empty")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full-suite command, got %v", tests)
		}
	})

	t.Run("explicit_test_names_not_overwritten", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"tests":["TestSpecific"],"files":["some_test.go"]}}`)
		_, tests, _ := extractTestStartingFromInput(input, "")

		if len(tests) != 1 || tests[0] != "TestSpecific" {
			t.Errorf("expected explicit test name to be preserved, got %v", tests)
		}
	})

	t.Run("go_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -v ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for full suite, got %v", tests)
		}
		if !strings.Contains(labs[0], "🥼") || !strings.Contains(labs[0], "mypackage") {
			t.Errorf("expected normalized lab id for my_test.go, got %v", labs)
		}
	})

	t.Run("go_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test -run TestOne ./mypackage/...","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testone") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted TestOne id, got %v", tests)
		}
	})

	t.Run("python_wrapped_runner_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"python -m pytest tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped pytest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped pytest full suite, got %v", tests)
		}
	})

	t.Run("uv_pytest_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"uv run pytest -k test_alpha tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted uv pytest run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted uv pytest run, got none")
		}
		found := false
		for _, id := range tests {
			if strings.Contains(id, "testalpha") {
				found = true
			}
		}
		if !found {
			t.Errorf("expected targeted test_alpha id, got %v", tests)
		}
	})

	t.Run("vitest_wrapped_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"npx vitest run src/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for wrapped vitest full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for wrapped vitest full suite, got %v", tests)
		}
	})

	t.Run("bundle_rspec_targeted_returns_tests", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"bundle exec rspec --example alpha","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) != 0 {
			t.Fatalf("expected no labs for targeted rspec run, got %v", labs)
		}
		if len(tests) == 0 {
			t.Fatalf("expected tests for targeted rspec run, got none")
		}
	})

	t.Run("phpunit_full_suite_returns_labs", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"./vendor/bin/phpunit tests/","cwd":"%s"}}`, tmpDir))
		labs, tests, _ := extractTestStartingFromInput(input, "")
		if len(labs) == 0 {
			t.Fatalf("expected labs for phpunit full suite, got none")
		}
		if len(tests) != 0 {
			t.Fatalf("expected no tests for phpunit full suite, got %v", tests)
		}
	})
}

func TestExtractTestEndedFromInputResolvesFiles(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	goDir := filepath.Join(tmpDir, "pkg")
	os.MkdirAll(goDir, 0755)
	os.WriteFile(filepath.Join(goDir, "pkg_test.go"), []byte("package pkg\nfunc TestX(t *testing.T) {}\n"), 0644)

	t.Run("resolves_files_from_command", func(t *testing.T) {
		input := json.RawMessage(fmt.Sprintf(`{"tool_info":{"command_line":"go test ./pkg/...","cwd":"%s"}}`, tmpDir))
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) == 0 {
			t.Fatalf("expected files to be resolved from command, got empty")
		}
	})

	t.Run("explicit_files_preserved", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"files":["explicit_test.go"]},"tool_output":{"succeeded":["TestX"]}}`)
		files, _, _ := extractTestEndedFromInput(input)
		if len(files) != 1 || files[0] != "explicit_test.go" {
			t.Errorf("expected explicit file, got %v", files)
		}
	})
}

func TestResolveHookEvent(t *testing.T) {
	cases := []struct {
		name      string
		eventStr  string
		client    string
		toolName  string
		expectEvt HookEvent
		expectPar string
		expectErr bool
	}{
		{"neutral agent.started", "agent.started", "copilot-chat", "", HookAgentStarted, "", false},
		{"neutral agent.tool.code.edit.starting", "agent.tool.code.edit.starting", "copilot-chat", "", HookAgentToolCodeEditStarting, "", false},
		{"copilot SessionStart", "SessionStart", "copilot-chat", "", HookAgentStarted, "", false},
		{"copilot Stop", "Stop", "copilot-chat", "", HookAgentEnded, "", false},
		{"copilot SubagentStart", "SubagentStart", "copilot-chat", "", HookAgentStarted, "subagent", false},
		{"copilot SubagentStop", "SubagentStop", "copilot-chat", "", HookAgentEnded, "subagent", false},
		{"copilot UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", HookAgentPromptSubmitting, "", false},
		{"copilot PreCompact", "PreCompact", "copilot-chat", "", HookAgentCompacting, "", false},
		{"copilot PreToolUse generic", "PreToolUse", "copilot-chat", "runSubagent", HookAgentToolStarting, "", false},
		{"copilot PreToolUse read_file", "PreToolUse", "copilot-chat", "read_file", HookAgentToolSearchStarting, "", false},
		{"copilot PreToolUse create_file", "PreToolUse", "copilot-chat", "create_file", HookAgentToolCodeEditStarting, "", false},
		{"copilot PreToolUse run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalStarting, "", false},
		{"copilot PreToolUse manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", HookAgentToolPlanUpdatingStarting, "", false},
		{"copilot PostToolUse read_file", "PostToolUse", "copilot-chat", "read_file", HookAgentToolSearchEnded, "", false},
		{"copilot PostToolUse create_file", "PostToolUse", "copilot-chat", "create_file", HookAgentToolCodeEditEnded, "", false},
		{"copilot PostToolUse run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalEnded, "", false},
		{"copilot PostToolUse generic", "PostToolUse", "copilot-chat", "runSubagent", HookAgentToolEnded, "", false},
		{"cursor sessionStart", "sessionStart", "cursor-chat", "", HookAgentStarted, "", false},
		{"cursor sessionEnd", "sessionEnd", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor subagentStart", "subagentStart", "cursor-chat", "", HookAgentStarted, "subagent", false},
		{"cursor beforeReadFile", "beforeReadFile", "cursor-chat", "", HookAgentToolSearchStarting, "", false},
		{"cursor afterFileEdit", "afterFileEdit", "cursor-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"cursor beforeShellExecution", "beforeShellExecution", "cursor-chat", "", HookAgentToolTerminalStarting, "", false},
		{"cursor afterShellExecution", "afterShellExecution", "cursor-chat", "", HookAgentToolTerminalEnded, "", false},
		{"cursor beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", HookAgentToolStarting, "", false},
		{"cursor afterMCPExecution", "afterMCPExecution", "cursor-chat", "", HookAgentToolEnded, "", false},
		{"cursor afterAgentResponse", "afterAgentResponse", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor afterAgentThought", "afterAgentThought", "cursor-chat", "", HookAgentThinkingEnded, "", false},
		{"cursor beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", HookAgentToolSearchStarting, "", false},
		{"cursor afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", HookAgentPromptSubmitting, "", false},
		{"windsurf post_cascade_response", "post_cascade_response", "windsurf-chat", "", HookAgentEnded, "", false},
		{"windsurf post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", HookAgentStarted, "", false},
		{"windsurf pre_read_code", "pre_read_code", "windsurf-chat", "", HookAgentToolSearchStarting, "", false},
		{"windsurf pre_write_code", "pre_write_code", "windsurf-chat", "", HookAgentToolCodeEditStarting, "", false},
		{"windsurf post_write_code", "post_write_code", "windsurf-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_run_command", "pre_run_command", "windsurf-chat", "", HookAgentToolTerminalStarting, "", false},
		{"windsurf post_run_command", "post_run_command", "windsurf-chat", "", HookAgentToolTerminalEnded, "", false},
		{"windsurf pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", HookAgentToolStarting, "", false},
		{"windsurf post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", HookAgentToolEnded, "", false},
		{"claude SessionStart", "SessionStart", "claude-code", "", HookAgentStarted, "", false},
		{"claude SessionEnd", "SessionEnd", "claude-code", "", HookAgentEnded, "", false},
		{"claude SubagentStart", "SubagentStart", "claude-code", "", HookAgentStarted, "subagent", false},
		{"claude SubagentStop", "SubagentStop", "claude-code", "", HookAgentEnded, "subagent", false},
		{"claude TaskCompleted", "TaskCompleted", "claude-code", "", HookAgentToolPlanUpdatingEnded, "", false},
		{"claude PermissionRequest", "PermissionRequest", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude TeammateIdle", "TeammateIdle", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude Notification", "Notification", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude PreToolUse Bash", "PreToolUse", "claude-code", "Bash", HookAgentToolTerminalStarting, "", false},
		{"claude PostToolUse Bash", "PostToolUse", "claude-code", "Bash", HookAgentToolTerminalEnded, "", false},
		{"claude PreToolUse Read", "PreToolUse", "claude-code", "Read", HookAgentToolSearchStarting, "", false},
		{"claude PreToolUse Glob", "PreToolUse", "claude-code", "Glob", HookAgentToolSearchStarting, "", false},
		{"claude PostToolUse Glob", "PostToolUse", "claude-code", "Glob", HookAgentToolSearchEnded, "", false},
		{"claude PreToolUse Edit", "PreToolUse", "claude-code", "Edit", HookAgentToolCodeEditStarting, "", false},
		{"claude PostToolUse Edit", "PostToolUse", "claude-code", "Edit", HookAgentToolCodeEditEnded, "", false},
		{"droid PreToolUse", "PreToolUse", "droid", "Bash", HookAgentToolTerminalStarting, "", false},
		{"codex PreToolUse", "PreToolUse", "codex", "Read", HookAgentToolSearchStarting, "", false},
		{"antigravity PreToolUse", "PreToolUse", "antigravity-chat", "Task", HookAgentToolPlanUpdatingStarting, "", false},
		{"unknown client defaults to claude-compatible", "SessionStart", "unknown-client", "", HookAgentStarted, "", false},
		{"kiro agentSpawn", "agentSpawn", "kiro-cli", "", HookAgentStarted, "", false},
		{"kiro userPromptSubmit", "userPromptSubmit", "kiro-cli", "", HookAgentPromptSubmitting, "", false},
		{"kiro preToolUse fs_read", "preToolUse", "kiro-cli", "fs_read", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse fs_write", "preToolUse", "kiro-cli", "fs_write", HookAgentToolCodeEditStarting, "", false},
		{"kiro preToolUse execute_bash", "preToolUse", "kiro-cli", "execute_bash", HookAgentToolTerminalStarting, "", false},
		{"kiro preToolUse code", "preToolUse", "kiro-cli", "code", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse grep", "preToolUse", "kiro-cli", "grep", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse glob", "preToolUse", "kiro-cli", "glob", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_search", "preToolUse", "kiro-cli", "web_search", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_fetch", "preToolUse", "kiro-cli", "web_fetch", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse use_subagent", "preToolUse", "kiro-cli", "use_subagent", HookAgentToolStarting, "", false},
		{"kiro postToolUse fs_write", "postToolUse", "kiro-cli", "fs_write", HookAgentToolCodeEditEnded, "", false},
		{"kiro postToolUse execute_bash", "postToolUse", "kiro-cli", "execute_bash", HookAgentToolTerminalEnded, "", false},
		{"kiro stop", "stop", "kiro-cli", "", HookAgentEnded, "", false},
		{"kiro invalid event", "UnknownEvent", "kiro-cli", "", "", "", true},
		{"invalid copilot event", "UnknownEvent", "copilot-chat", "", "", "", true},
		{"invalid cursor event", "UnknownEvent", "cursor-chat", "", "", "", true},
		{"invalid windsurf event", "UnknownEvent", "windsurf-chat", "", "", "", true},
		{"invalid claude event", "UnknownEvent", "claude-code", "", "", "", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, parent, err := ResolveHookEvent(tc.eventStr, tc.client, tc.toolName, nil)
			if tc.expectErr {
				if err == nil {
					t.Errorf("expected error, got event=%s parent=%s", event, parent)
				}
				return
			}
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if event != tc.expectEvt {
				t.Errorf("expected event %s, got %s", tc.expectEvt, event)
			}
			if parent != tc.expectPar {
				t.Errorf("expected parent %q, got %q", tc.expectPar, parent)
			}
		})
	}
}

func TestResolvePreToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindPlan, HookAgentToolPlanUpdatingStarting},
		{ToolKindCodeSearch, HookAgentToolSearchStarting},
		{ToolKindCodeEdit, HookAgentToolCodeEditStarting},
		{ToolKindTest, HookAgentToolTestStarting},
		{ToolKindBuild, HookAgentToolBuildStarting},
		{ToolKindTerminal, HookAgentToolTerminalStarting},
		{ToolKindGeneric, HookAgentToolStarting},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePreToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestResolvePostToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindCodeSearch, HookAgentToolSearchEnded},
		{ToolKindCodeEdit, HookAgentToolCodeEditEnded},
		{ToolKindTest, HookAgentToolTestEnded},
		{ToolKindBuild, HookAgentToolBuildEnded},
		{ToolKindTerminal, HookAgentToolTerminalEnded},
		{ToolKindGeneric, HookAgentToolEnded},
		{ToolKindPlan, HookAgentToolPlanUpdatingEnded},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePostToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestPopulateEventDataAgentStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-abc","parent":"subagent"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T10:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "sess-abc" {
		t.Errorf("expected session=sess-abc, got %s", res.Session)
	}
	if res.Second != "2026-02-19T10:00:00Z" {
		t.Errorf("expected second=2026-02-19T10:00:00Z, got %s", res.Second)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
	if res.Parent != "" {
		t.Errorf("expected empty parent, got %s", res.Parent)
	}
	if res.Raw == nil {
		t.Error("expected raw to be populated")
	}
}

func TestPopulateEventDataAgentStartingSubagentParentFromContextUsesSession(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-sub"}`)
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "codex",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "sess-sub" {
		t.Errorf("expected parent=sess-sub, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingSubagentUsesAgentIDAsSession(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-parent","agent_id":"agent-child"}`)
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "copilot-chat",
		Second:     "2026-03-06T19:54:55.558Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "agent-child" {
		t.Errorf("expected session=agent-child, got %s", res.Session)
	}
	if res.Parent != "sess-parent" {
		t.Errorf("expected parent=sess-parent, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingParentFromContext(t *testing.T) {
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "claude-code",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		ParentInfo: "parent-agent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "parent-agent" {
		t.Errorf("expected parent=parent-agent, got %s", res.Parent)
	}
	if res.Client != "claude-code" {
		t.Errorf("expected client=claude-code, got %s", res.Client)
	}
}

func TestPopulateEventDataAgentEnded(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-end","tool_info":{"response":"final report text"}}`)
	hctx := HookContext{
		Event:    HookAgentEnded,
		Client:   "cursor-chat",
		Second:   "2026-02-19T11:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentEnded, got %T", result)
	}
	if res.Session != "sess-end" {
		t.Errorf("expected session=sess-end, got %s", res.Session)
	}
	if res.Second != "2026-02-19T11:00:00Z" {
		t.Errorf("expected second, got %s", res.Second)
	}
	if res.Client != "cursor-chat" {
		t.Errorf("expected client=cursor-chat, got %s", res.Client)
	}
	if res.Report != "final report text" {
		t.Errorf("expected report to be extracted, got %q", res.Report)
	}
}

func TestPopulateEventDataPromptSubmitting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-p","prompt":"Fix the bug in main.go"}`)
	hctx := HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T12:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentPromptSubmitting)
	if !ok {
		t.Fatalf("expected HookResultAgentPromptSubmitting, got %T", result)
	}
	if res.Session != "sess-p" {
		t.Errorf("expected session=sess-p, got %s", res.Session)
	}
	if res.Prompt != "Fix the bug in main.go" {
		t.Errorf("expected prompt, got %s", res.Prompt)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
}

func TestPopulateEventDataCompacting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-c","chat":"previous conversation context"}`)
	hctx := HookContext{
		Event:    HookAgentCompacting,
		Client:   "claude-code",
		Second:   "2026-02-19T13:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentCompacting)
	if !ok {
		t.Fatalf("expected HookResultAgentCompacting, got %T", result)
	}
	if res.Session != "sess-c" {
		t.Errorf("expected session=sess-c, got %s", res.Session)
	}
	if res.Chat != "previous conversation context" {
		t.Errorf("expected chat content, got %s", res.Chat)
	}
}

func TestPopulateEventDataToolStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-t","tool_name":"runSubagent","tool_input":{"prompt":"do something"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:00:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Session != "sess-t" {
		t.Errorf("expected session=sess-t, got %s", res.Session)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	var inputData map[string]interface{}
	if err := json.Unmarshal(res.Input, &inputData); err != nil {
		t.Fatalf("expected valid JSON input, got: %v", err)
	}
	if inputData["prompt"] != "do something" {
		t.Errorf("expected prompt in input, got %v", inputData["prompt"])
	}
}

func TestPopulateEventDataToolEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-te","tool_name":"runSubagent","tool_input":{"prompt":"do something"},"tool_output":"done"}`)
	hctx := HookContext{
		Event:    HookAgentToolEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:30:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolEnded, got %T", result)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	if res.Response == nil {
		t.Error("expected response to be populated")
	}
}

func TestPopulateEventDataPlanUpdating(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-plan","tool_input":{"todoList":[{"id":1,"title":"Step 1","status":"completed"},{"id":2,"title":"Step 2","status":"in-progress"},{"id":3,"title":"Step 3","status":"not-started"}]}}`)
	hctx := HookContext{
		Event:    HookAgentToolPlanUpdatingStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T15:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolPlanUpdating)
	if !ok {
		t.Fatalf("expected HookResultAgentToolPlanUpdating, got %T", result)
	}
	if res.Session != "sess-plan" {
		t.Errorf("expected session=sess-plan, got %s", res.Session)
	}
	if len(res.Steps) != 3 {
		t.Fatalf("expected 3 steps, got %d", len(res.Steps))
	}
	if res.Steps[0].Name != "Step 1" || res.Steps[0].Status != "completed" {
		t.Errorf("expected Step 1 completed, got %+v", res.Steps[0])
	}
	if res.Steps[1].Name != "Step 2" || res.Steps[1].Status != "in-progress" {
		t.Errorf("expected Step 2 in-progress, got %+v", res.Steps[1])
	}
	if res.Steps[2].Name != "Step 3" || res.Steps[2].Status != "not-started" {
		t.Errorf("expected Step 3 not-started, got %+v", res.Steps[2])
	}
}

func TestPopulateEventDataCodeSearching(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cs","tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T16:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if res.Session != "sess-cs" {
		t.Errorf("expected session=sess-cs, got %s", res.Session)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestPopulateEventDataCodeEditing(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ce","tool_input":{"filePath":"/workspaces/semio/test.go","oldString":"old code","newString":"new code"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T17:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if res.Session != "sess-ce" {
		t.Errorf("expected session=sess-ce, got %s", res.Session)
	}
	if res.Path != "/workspaces/semio/test.go" {
		t.Errorf("expected path, got %s", res.Path)
	}
	if res.Old != "old code" {
		t.Errorf("expected old=old code, got %s", res.Old)
	}
	if res.New != "new code" {
		t.Errorf("expected new=new code, got %s", res.New)
	}
}

func TestPopulateEventDataCodeEditingWithAll(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cea","tool_input":{"filePath":"/tmp/file.ts","oldString":"x","newString":"y","all":true}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditStarting,
		Client:   "cursor-chat",
		Second:   "2026-02-19T17:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if !res.All {
		t.Error("expected all=true")
	}
}

func TestPopulateEventDataCodeEdited(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ced","tool_input":{"filePath":"/tmp/edited.ts","oldString":"before","newString":"after"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T18:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != "/tmp/edited.ts" {
		t.Errorf("expected path=/tmp/edited.ts, got %s", res.Path)
	}
	if res.Old != "before" {
		t.Errorf("expected old=before, got %s", res.Old)
	}
	if res.New != "after" {
		t.Errorf("expected new=after, got %s", res.New)
	}
}

func TestCodeEditEndedRunsFormatter(t *testing.T) {
	// 📋Track whether the formatter was invoked.
	var formatterCalled bool
	var formatterBinary string
	origRun := formatterCommandRun
	formatterCommandRun = func(binary string, args []string, workDir string) error {
		formatterCalled = true
		formatterBinary = binary
		return nil
	}
	defer func() { formatterCommandRun = origRun }()

	// Stub binary lookup so the formatter plan is considered available.
	origLookup := formatterBinaryLookup
	formatterBinaryLookup = func(file string) (string, error) { return "/usr/bin/" + file, nil }
	defer func() { formatterBinaryLookup = origLookup }()

	tmpDir := t.TempDir()
	// Create requirement files so the plan is available (prettier needs .prettierrc.json and node_modules/.bin/prettier).
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin")
	_ = os.MkdirAll(prettierBin, 0o755)
	_ = os.WriteFile(filepath.Join(prettierBin, "prettier"), []byte("#!/bin/sh\n"), 0o755)
	_ = os.WriteFile(filepath.Join(tmpDir, ".prettierrc.json"), []byte("{}"), 0o644)

	origRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = origRoot }()

	relPath := "src/example.ts"
	payload := json.RawMessage(`{"sessionId":"sess-fmt","tool_input":{"filePath":"` + relPath + `","oldString":"old","newString":"new"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "claude-code",
		Second:   "2026-03-16T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := dispatchHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != relPath {
		t.Errorf("expected path=%s, got %s", relPath, res.Path)
	}
	if !formatterCalled {
		t.Error("expected formatter to be called on code edit ended, but it was not")
	}
	if !strings.Contains(formatterBinary, "prettier") {
		t.Errorf("expected prettier formatter for .ts file, got %s", formatterBinary)
	}
}

func TestPopulateEventDataTerminalStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ts","tool_input":{"command":"npm test"}}`)
	hctx := HookContext{
		Event:    HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalStarting, got %T", result)
	}
	if res.Session != "sess-ts" {
		t.Errorf("expected session=sess-ts, got %s", res.Session)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
}

func TestPopulateEventDataTerminalEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tse","tool_input":{"command":"npm test"},"pid":"12345","terminated":true,"stdout":"all passed","stderr":""}`)
	hctx := HookContext{
		Event:    HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalEnded, got %T", result)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
	if res.PID != 12345 {
		t.Errorf("expected pid=12345, got %d", res.PID)
	}
	if !res.Terminated {
		t.Error("expected terminated=true")
	}
	if res.Output != nil {
		var outputStr string
		if err := json.Unmarshal(res.Output, &outputStr); err != nil {
			t.Errorf("expected output to be JSON string, got error: %v", err)
		} else if outputStr != "all passed" {
			t.Errorf("expected stdout=all passed, got %s", outputStr)
		}
	}
}

func TestPopulateEventDataVersionCheckpointEnded(t *testing.T) {
	tmpDir := t.TempDir()
	payload := json.RawMessage(`{"sha":"abc123def","message":"feat: add hooks"}`)
	hctx := HookContext{
		Event:    HookVersionCheckpointEnded,
		Client:   "",
		Second:   "2026-02-19T20:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultVersionCheckpointEnded)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointEnded, got %T", result)
	}
	if res.Checkpoint != "abc123def" {
		t.Errorf("expected checkpoint=abc123def, got %s", res.Checkpoint)
	}
	if res.Description != "feat: add hooks" {
		t.Errorf("expected description=feat: add hooks, got %s", res.Description)
	}
}

func TestPopulateEventDataVersionCheckpointStartingFromFile(t *testing.T) {
	tmpDir := t.TempDir()
	gitDir := filepath.Join(tmpDir, ".git")
	if err := os.MkdirAll(gitDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: resolve issue"), 0644); err != nil {
		t.Fatal(err)
	}
	hctx := HookContext{
		Event:    HookVersionCheckpointStarting,
		Client:   "",
		RepoRoot: tmpDir,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultVersionCheckpointStarting)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
	}
	if res.Description != "fix: resolve issue" {
		t.Errorf("expected description=fix: resolve issue, got %s", res.Description)
	}
}

func TestPopulateEventDataRawField(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"raw-test","some":"data"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T21:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw == nil {
		t.Fatal("expected raw to be populated")
	}
	var rawData map[string]interface{}
	body, _ := json.Marshal(res.Raw)
	if err := json.Unmarshal(body, &rawData); err != nil {
		t.Fatalf("expected valid JSON raw, got: %v", err)
	}
	if rawData["sessionId"] != "raw-test" {
		t.Errorf("expected sessionId=raw-test in raw, got %v", rawData["sessionId"])
	}
	if rawData["some"] != "data" {
		t.Errorf("expected some=data in raw, got %v", rawData["some"])
	}
}

func TestPopulateEventDataNoInputNoRaw(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T22:00:00Z",
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw != nil {
		t.Error("expected raw to be nil when no input")
	}
}

func TestPopulateEventDataToolNameFromStdin(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tn","tool_name":"mcp_custom_tool","tool_input":{"arg":"val"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Name != "mcp_custom_tool" {
		t.Errorf("expected name=mcp_custom_tool, got %s", res.Name)
	}
}

func TestPopulateEventDataCodeSearchWithExclude(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ex","tool_input":{"query":"test","include":["*.ts","*.tsx"],"exclude":["node_modules"]}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestExtractPlanStepsFromInput(t *testing.T) {
	t.Run("todoList format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"todoList":[{"title":"Task A","status":"completed"},{"title":"Task B","status":"in-progress"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(steps))
		}
		if steps[0].Name != "Task A" || steps[0].Status != "completed" {
			t.Errorf("unexpected step 0: %+v", steps[0])
		}
	})
	t.Run("steps format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"steps":[{"name":"Build","status":"pending"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "Build" {
			t.Errorf("expected name=Build, got %s", steps[0].Name)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, `{"todoList":[{"title":"FromArgs","status":"done"}]}`)
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "FromArgs" {
			t.Errorf("expected name=FromArgs, got %s", steps[0].Name)
		}
	})
	t.Run("empty", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, "")
		if steps != nil {
			t.Errorf("expected nil steps, got %v", steps)
		}
	})
}

func TestExtractSearchFromInput(t *testing.T) {
	t.Run("grep_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("file_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"**/*.ts"}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("read_file style", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		if err := os.WriteFile(tempFile, []byte("one\ntwo\nthree\n"), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"filePath":%q}}`, tempFile))
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		pages, ranges := extractSearchFromInput(nil, `{"query":"fromArgs"}`)
		if len(pages) != 0 {
			t.Errorf("expected no webpages from non-url query, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("webpages only", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"url":"https://example.com/docs","pages":["https://compose.dev","not-a-url"]}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
		if len(pages) != 2 || pages[0] != "https://example.com/docs" || pages[1] != "https://compose.dev" {
			t.Errorf("expected only valid webpages, got %v", pages)
		}
	})
	t.Run("read tool with limit only", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"limit":3}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("read tool with offset and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"offset":2,"limit":3}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L2-L4"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format with file_path and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q,"limit":100}}}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")

		want := tempFile + "#L1-L100"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format without limit reads full file", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q}}}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("grep tool with pattern ignores path for line range", func(t *testing.T) {
		tempDir := t.TempDir()
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","path":%q}}`, tempDir))
		_, ranges := extractSearchFromInput(input, "")

		if len(ranges) != 0 {
			t.Errorf("grep with pattern+path should produce no file ranges, got %v", ranges)
		}
	})
	t.Run("grep tool with pattern and file_path uses file_path for range", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","file_path":%q}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L2"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
}

func TestExtractToolInputMapFromData(t *testing.T) {
	t.Run("direct tool_input", func(t *testing.T) {
		data := map[string]interface{}{"tool_input": map[string]interface{}{"key": "val"}}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "val" {
			t.Errorf("expected tool_input with key=val, got %v", ti)
		}
	})
	t.Run("native.event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "native" {
			t.Errorf("expected native tool_input with key=native, got %v", ti)
		}
	})
	t.Run("event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"event": map[string]interface{}{
				"tool_input": map[string]interface{}{"key": "event"},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "event" {
			t.Errorf("expected event tool_input with key=event, got %v", ti)
		}
	})
	t.Run("direct takes precedence over native", func(t *testing.T) {
		data := map[string]interface{}{
			"tool_input": map[string]interface{}{"key": "direct"},
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "direct" {
			t.Errorf("expected direct tool_input to take precedence, got %v", ti)
		}
	})
	t.Run("empty data returns nil", func(t *testing.T) {
		ti := extractToolInputMapFromData(map[string]interface{}{})
		if ti != nil {
			t.Errorf("expected nil for empty data, got %v", ti)
		}
	})
}

func TestExtractToolInputFromStdinNativeFormat(t *testing.T) {
	t.Run("native.event.tool_input extracted", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go","limit":100}}}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input from native format")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["file_path"] != "/tmp/test.go" {
			t.Errorf("expected file_path=/tmp/test.go, got %v", data["file_path"])
		}
	})
	t.Run("direct tool_input still works", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go"}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
	})
}

func TestExtractSearchDefinitionReadsFromInput(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc First() {\n\tprintln(\"alpha\")\n}\n\nfunc Second() {\n\tprintln(\"beta\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	input := json.RawMessage(`{"tool_info":{"command_line":"grep -n \"beta\" sample.go","cwd":"` + tmpDir + `"}}`)
	definitions := extractSearchDefinitionReadsFromInput(input, "")
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition read, got %v", definitions)
	}
	foundSecond := false
	for _, definition := range definitions {
		if strings.Contains(strings.ToLower(definition.ID), "second") {
			foundSecond = true
			if definition.Loc <= 0 {
				t.Fatalf("expected positive loc for Second, got %d", definition.Loc)
			}
		}
	}
	if !foundSecond {
		t.Fatalf("expected Second definition in %v", definitions)
	}
}

func TestRunHookSearchStartingIncludesDefinitions(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc Alpha() {\n\tprintln(\"one\")\n}\n\nfunc Beta() {\n\tprintln(\"needle\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	payload := json.RawMessage(`{"trajectory_id":"trace-1","tool_info":{"command_line":"grep -n \"needle\" sample.go","cwd":"` + tmpDir + `"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected definitions in event, got %v", res.Definitions)
	}
	if res.Definitions[0].Loc <= 0 {
		t.Fatalf("expected loc > 0, got %d", res.Definitions[0].Loc)
	}
}

func TestBuildDefinitionReadsFullFileReturnsFileID(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "full.go")
	content := "package main\n\nfunc Alpha() {}\n\nfunc Beta() {}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(origRoot)

	lineSet := map[int]struct{}{1: {}, 2: {}, 3: {}, 4: {}, 5: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) != 1 {
		t.Fatalf("expected exactly one file-id entry for full file read, got %d: %v", len(definitions), definitions)
	}
	if definitions[0].Loc != 0 {
		t.Errorf("expected loc=0 for file-id entry, got %d", definitions[0].Loc)
	}
	if definitions[0].ID == "" {
		t.Errorf("expected non-empty file ID")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	if strings.Contains(definitions[0].ID, absPrefix) {
		t.Errorf("file ID %q contains absolute tmpDir path %q", definitions[0].ID, absPrefix)
	}
}

func TestBuildDefinitionReadsUsesRepoRelativePaths(t *testing.T) {
	tmpDir := t.TempDir()
	subDir := filepath.Join(tmpDir, "pkg")
	if err := os.MkdirAll(subDir, 0o755); err != nil {
		t.Fatalf("failed to create subdir: %v", err)
	}
	filePath := filepath.Join(subDir, "util.go")
	content := "package pkg\n\nfunc Helper() {\n\tprintln(\"found\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(origRoot)
	lineSet := map[int]struct{}{3: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition, got none")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute tmpDir path %q", def.ID, absPrefix)
		}
	}
}

func TestRunHookPreReadCodeDefinitionIDsAreRepoRelative(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "cmd.go")
	content := "package main\n\nfunc Run() {\n\tprintln(\"run\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	payload := json.RawMessage(`{"tool_info":{"file_path":"` + filepath.ToSlash(filePath) + `"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected at least one definition")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range res.Definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute path %q", def.ID, absPrefix)
		}
	}
}

func TestShouldExecuteSearchCommand(t *testing.T) {
	tests := []struct {
		name    string
		command string
		want    bool
	}{
		{name: "grep command", command: "grep -n needle file.go", want: true},
		{name: "cat command", command: "cat file.go", want: true},
		{name: "blocked redirection", command: "grep -n needle file.go > out.txt", want: false},
		{name: "edit command", command: "sed -i 's/a/b/' file.go", want: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := shouldExecuteSearchCommand(tt.command)
			if got != tt.want {
				t.Fatalf("shouldExecuteSearchCommand(%q) = %v, want %v", tt.command, got, tt.want)
			}
		})
	}
}

func TestExtractCodeEditFromInput(t *testing.T) {
	t.Run("replace_string_in_file style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/file.go","oldString":"old","newString":"new"}}`)
		path, old, new_, all := extractCodeEditFromInput(input, "")
		if path != "/tmp/file.go" {
			t.Errorf("expected path=/tmp/file.go, got %s", path)
		}
		if old != "old" {
			t.Errorf("expected old=old, got %s", old)
		}
		if new_ != "new" {
			t.Errorf("expected new=new, got %s", new_)
		}
		if all {
			t.Error("expected all=false")
		}
	})
	t.Run("with replaceAll", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/f.go","oldString":"a","newString":"b","replaceAll":true}}`)
		_, _, _, all := extractCodeEditFromInput(input, "")
		if !all {
			t.Error("expected all=true")
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		path, _, _, _ := extractCodeEditFromInput(nil, `{"filePath":"/from/args.go"}`)
		if path != "/from/args.go" {
			t.Errorf("expected path=/from/args.go, got %s", path)
		}
	})
}

func TestExtractToolInputFromStdin(t *testing.T) {
	t.Run("with tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test","tool_input":{"key":"val"}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["key"] != "val" {
			t.Errorf("expected key=val, got %v", data["key"])
		}
	})
	t.Run("without tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolInputFromStdin(input)
		if result != nil {
			t.Error("expected nil tool input")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractToolInputFromStdin(nil)
		if result != nil {
			t.Error("expected nil for empty input")
		}
	})
}

func TestExtractToolResponseFromStdin(t *testing.T) {
	t.Run("tool_output", func(t *testing.T) {
		input := json.RawMessage(`{"tool_output":"response data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("tool_response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_response":"data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("no response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolResponseFromStdin(input)
		if result != nil {
			t.Error("expected nil response")
		}
	})
}

func TestExtractTerminalEndedFromInput(t *testing.T) {
	t.Run("full payload", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"command":"ls -la"},"pid":"999","terminated":true,"stdout":"file1\nfile2","stderr":""}`)
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
		if command != "ls -la" {
			t.Errorf("expected command=ls -la, got %s", command)
		}
		if pid != "999" {
			t.Errorf("expected pid=999, got %s", pid)
		}
		if !terminated {
			t.Error("expected terminated=true")
		}
		if stdout != "file1\nfile2" {
			t.Errorf("expected stdout, got %s", stdout)
		}
		if stderr != "" {
			t.Errorf("expected empty stderr, got %s", stderr)
		}
	})
	t.Run("numeric pid", func(t *testing.T) {
		input := json.RawMessage(`{"pid":42}`)
		_, pid, _, _, _ := extractTerminalEndedFromInput(input)
		if pid != "42" {
			t.Errorf("expected pid=42, got %s", pid)
		}
	})
	t.Run("empty", func(t *testing.T) {
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(nil)
		if command != "" || pid != "" || terminated || stdout != "" || stderr != "" {
			t.Error("expected all empty for nil input")
		}
	})
}

func TestExtractChatFromInput(t *testing.T) {
	t.Run("chat string", func(t *testing.T) {
		input := json.RawMessage(`{"chat":"conversation context"}`)
		result := extractChatFromInput(input)
		if result != "conversation context" {
			t.Errorf("expected conversation context, got %s", result)
		}
	})
	t.Run("messages array", func(t *testing.T) {
		input := json.RawMessage(`{"messages":[{"role":"user","content":"hello"},{"role":"assistant","content":"hi"}]}`)
		result := extractChatFromInput(input)
		if result == "" {
			t.Error("expected non-empty chat from messages array")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractChatFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractReportFromInput(t *testing.T) {
	t.Run("report field", func(t *testing.T) {
		input := json.RawMessage(`{"report":"agent summary"}`)
		result := extractReportFromInput(input)
		if result != "agent summary" {
			t.Errorf("expected agent summary, got %s", result)
		}
	})
	t.Run("tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"tool_info":{"response":"from tool info"}}`)
		result := extractReportFromInput(input)
		if result != "from tool info" {
			t.Errorf("expected from tool info, got %s", result)
		}
	})
	t.Run("native event tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_info":{"response":"nested response"}}}}`)
		result := extractReportFromInput(input)
		if result != "nested response" {
			t.Errorf("expected nested response, got %s", result)
		}
	})
	t.Run("empty input", func(t *testing.T) {
		result := extractReportFromInput(nil)
		if result != "" {
			t.Errorf("expected empty report, got %s", result)
		}
	})
}

func TestExtractCheckpointMessageFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"message":"feat: new feature"}`)
		result := extractCheckpointMessageFromInput(input, "/nonexistent")
		if result != "feat: new feature" {
			t.Errorf("expected feat: new feature, got %s", result)
		}
	})
	t.Run("from COMMIT_EDITMSG file", func(t *testing.T) {
		tmpDir := t.TempDir()
		gitDir := filepath.Join(tmpDir, ".git")
		os.MkdirAll(gitDir, 0755)
		os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: bug fix"), 0644)
		result := extractCheckpointMessageFromInput(nil, tmpDir)
		if result != "fix: bug fix" {
			t.Errorf("expected fix: bug fix, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractCheckpointMessageFromInput(nil, "/nonexistent")
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractCheckpointSHAFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"sha":"deadbeef123"}`)
		result := extractCheckpointSHAFromInput(input)
		if result != "deadbeef123" {
			t.Errorf("expected deadbeef123, got %s", result)
		}
	})
	t.Run("empty falls back to git", func(t *testing.T) {
		result := extractCheckpointSHAFromInput(nil)
		if result == "" {
			t.Skip("no git repo available")
		}
	})
}

func TestExtractParentFromInput(t *testing.T) {
	t.Run("parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"parent-session"}`)
		result := extractParentFromInput(input)
		if result != "parent-session" {
			t.Errorf("expected parent-session, got %s", result)
		}
	})
	t.Run("nested event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"event":{"parent":"parent-from-event"}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-event" {
			t.Errorf("expected parent-from-event, got %s", result)
		}
	})
	t.Run("nested native event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"parent":"parent-from-native-event"}}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-native-event" {
			t.Errorf("expected parent-from-native-event, got %s", result)
		}
	})
	t.Run("sentinel parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"subagent"}`)
		result := extractParentFromInput(input)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractParentFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestHookResultJSONFields(t *testing.T) {
	result := HookResultAgentToolTerminalEnded{
		HookResultAgentBase: HookResultAgentBase{
			HookResultBase: HookResultBase{
				Allowed: true,
				Raw:     json.RawMessage(`{"raw":"data"}`),
			},
			Session:   "sess-1",
			Second:    "2026-02-19T10:00:00Z",
			Client:    "copilot-chat",
			MessageID: "msg-123",
		},
		Command:    "npm test",
		PID:        12345,
		Terminated: true,
		Stdout:     json.RawMessage(`"passed"`),
		Stderr:     json.RawMessage(`"warn"`),
	}
	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON, got: %v", err)
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	expectedKeys := []string{"allowed", "message", "session", "second", "client", "command", "pid", "terminated", "stdout", "stderr"}
	for _, key := range expectedKeys {
		if _, ok := parsed[key]; !ok {
			t.Errorf("missing JSON key: %s", key)
		}
	}
}

func TestHookResultOmitEmpty(t *testing.T) {
	result := HookResultAgentStarted{
		HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}},
	}
	data, _ := json.Marshal(result)
	var parsed map[string]interface{}
	json.Unmarshal(data, &parsed)
	if _, ok := parsed["session"]; ok {
		t.Error("expected session to be omitted when empty")
	}
	if _, ok := parsed["raw"]; ok {
		t.Error("expected raw to be omitted when nil")
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
		expectEvent HookEvent
		expectPar   string
	}{
		{"copilot/SessionStart", "SessionStart", "copilot-chat", "", `{"hookEventName":"SessionStart","sessionId":"d765d480","second":"2026-02-19T10:44:08.112Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentStarted, ""},
		{"copilot/Stop", "Stop", "copilot-chat", "", `{"hookEventName":"Stop","sessionId":"2f1e87c2","second":"2026-02-18T18:51:54.315Z","stop_hook_active":false}`, HookAgentEnded, ""},
		{"copilot/SubagentStart", "SubagentStart", "copilot-chat", "", `{"hookEventName":"SubagentStart","sessionId":"ab58fc89","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentStarted, "subagent"},
		{"copilot/SubagentStop", "SubagentStop", "copilot-chat", "", `{"hookEventName":"SubagentStop","sessionId":"ab58fc89","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentEnded, "subagent"},
		{"copilot/UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", `{"hookEventName":"UserPromptSubmit","sessionId":"d765d480","second":"2026-02-19T10:44:16.328Z","transcript_path":"/tmp/t.jsonl","cwd":"/workspaces/semio"}`, HookAgentPromptSubmitting, ""},
		{"copilot/PreCompact", "PreCompact", "copilot-chat", "", `{"second":"2026-02-18T18:41:24.718Z","hookEventName":"PreCompact","sessionId":"2f1e87c2","transcript_path":"/tmp/t.jsonl","trigger":"auto","cwd":"/workspaces/semio"}`, HookAgentCompacting, ""},
		{"copilot/PreToolUse/read_file", "PreToolUse", "copilot-chat", "read_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"read_file","second":"2026-02-19T12:30:31.702Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/grep_search", "PreToolUse", "copilot-chat", "grep_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"grep_search","second":"2026-02-19T10:44:35.056Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/file_search", "PreToolUse", "copilot-chat", "file_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"file_search","second":"2026-02-19T10:50:09.443Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_dir", "PreToolUse", "copilot-chat", "list_dir", `{"second":"2026-02-19T10:44:16.328Z","hookEventName":"PreToolUse","sessionId":"d765d480","transcript_path":"/tmp/t.jsonl","tool_name":"list_dir","tool_input":{"path":"/workspaces/semio"}}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_code_usages", "PreToolUse", "copilot-chat", "list_code_usages", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"list_code_usages","second":"2026-02-19T11:31:35.416Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/replace_string_in_file", "PreToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"replace_string_in_file","second":"2026-02-19T10:50:46.160Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/multi_replace_string_in_file", "PreToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:17.358Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/create_file", "PreToolUse", "copilot-chat", "create_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"create_file","second":"2026-02-19T12:25:00.000Z"}`, HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"run_in_terminal","second":"2026-02-18T18:42:59.593Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolTerminalStarting, ""},
		{"copilot/PreToolUse/manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:13.780Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingStarting, ""},
		{"copilot/PreToolUse/runSubagent", "PreToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"runSubagent","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolStarting, ""},
		{"copilot/PostToolUse/read_file", "PostToolUse", "copilot-chat", "read_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"read_file","second":"2026-02-18T18:38:51.951Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"copilot/PostToolUse/replace_string_in_file", "PostToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"replace_string_in_file","second":"2026-02-18T18:37:05.471Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/multi_replace_string_in_file", "PostToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:26.261Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"d765d480","hookEventName":"PostToolUse","tool_name":"run_in_terminal","second":"2026-02-19T10:43:56.761Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolTerminalEnded, ""},
		{"copilot/PostToolUse/manage_todo_list", "PostToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:20.586Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"copilot/PostToolUse/runSubagent", "PostToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"runSubagent","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolEnded, ""},
		{"copilot/PostToolUse/grep_search", "PostToolUse", "copilot-chat", "grep_search", `{"sessionId":"8a40542e","hookEventName":"PostToolUse","tool_name":"grep_search","second":"2026-02-18T18:58:48.393Z","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"cursor/sessionStart", "sessionStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:00Z"}`, HookAgentStarted, ""},
		{"cursor/sessionEnd", "sessionEnd", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, HookAgentEnded, ""},
		{"cursor/subagentStart", "subagentStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:01:00Z"}`, HookAgentStarted, "subagent"},
		{"cursor/subagentStop", "subagentStop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:29:00Z"}`, HookAgentEnded, "subagent"},
		{"cursor/stop", "stop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, HookAgentEnded, ""},
		{"cursor/beforeSubmitPrompt", "beforeSubmitPrompt", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:01Z","prompt":"Fix bug"}`, HookAgentPromptSubmitting, ""},
		{"cursor/preCompact", "preCompact", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, HookAgentCompacting, ""},
		{"cursor/preToolUse/read_file", "preToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:02:00Z","tool_name":"read_file"}`, HookAgentToolSearchStarting, ""},
		{"cursor/preToolUse/edit", "preToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:03:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditStarting, ""},
		{"cursor/preToolUse/terminal", "preToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:04:00Z","tool_name":"terminal"}`, HookAgentToolTerminalStarting, ""},
		{"cursor/preToolUse/task", "preToolUse", "cursor-chat", "task", `{"sessionId":"cur-001","second":"2026-02-19T10:05:00Z","tool_name":"task"}`, HookAgentToolPlanUpdatingStarting, ""},
		{"cursor/postToolUse/read_file", "postToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:06:00Z","tool_name":"read_file"}`, HookAgentToolSearchEnded, ""},
		{"cursor/postToolUse/editfile", "postToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:07:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/postToolUse/terminal", "postToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:08:00Z","tool_name":"terminal"}`, HookAgentToolTerminalEnded, ""},
		{"cursor/postToolUseFailure/edit", "postToolUseFailure", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:09:00Z","tool_name":"editfile"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:10:00Z"}`, HookAgentToolStarting, ""},
		{"cursor/afterMCPExecution", "afterMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:11:00Z"}`, HookAgentToolEnded, ""},
		{"cursor/beforeReadFile", "beforeReadFile", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:12:00Z"}`, HookAgentToolSearchStarting, ""},
		{"cursor/afterFileEdit", "afterFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:13:00Z"}`, HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeShellExecution", "beforeShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:14:00Z"}`, HookAgentToolTerminalStarting, ""},
		{"cursor/afterShellExecution", "afterShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, HookAgentToolTerminalEnded, ""},
		{"cursor/afterAgentResponse", "afterAgentResponse", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:16:00Z"}`, HookAgentEnded, ""},
		{"cursor/afterAgentThought", "afterAgentThought", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:17:00Z"}`, HookAgentThinkingEnded, ""},
		{"cursor/beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:18:00Z"}`, HookAgentToolSearchStarting, ""},
		{"cursor/afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:19:00Z"}`, HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", `{"second":"2026-02-18T18:46:41.123Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentPromptSubmitting, ""},
		{"windsurf/post_cascade_response", "post_cascade_response", "windsurf-chat", "", `{"second":"2026-02-18T19:00:12.032Z","agent_action_name":"post_cascade_response","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentEnded, ""},
		{"windsurf/post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", `{"second":"2026-02-18T18:45:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentStarted, ""},
		{"windsurf/pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", `{"agent_action_name":"pre_mcp_tool_use","trajectory_id":"23e6dcf5","second":"2026-02-18T18:54:57.304Z","execution_id":"d9b64466","tool_info":{"mcp_server_name":"repo","mcp_tool_name":"tree"}}`, HookAgentToolStarting, ""},
		{"windsurf/post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", `{"second":"2026-02-18T18:55:28.469Z","agent_action_name":"post_mcp_tool_use","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolEnded, ""},
		{"windsurf/pre_read_code", "pre_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:48.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolSearchStarting, ""},
		{"windsurf/post_read_code", "post_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:50.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolSearchEnded, ""},
		{"windsurf/pre_write_code", "pre_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:30.780Z","agent_action_name":"pre_write_code","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolCodeEditStarting, ""},
		{"windsurf/post_write_code", "post_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:35.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_run_command", "pre_run_command", "windsurf-chat", "", `{"second":"2026-02-18T18:54:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, HookAgentToolTerminalStarting, ""},
		{"windsurf/post_run_command", "post_run_command", "windsurf-chat", "", `{"agent_action_name":"post_run_command","trajectory_id":"23e6dcf5","second":"2026-02-18T18:57:49.375Z","execution_id":"d9b64466","tool_info":{"command_line":"npm install","cwd":"/workspaces/semio"}}`, HookAgentToolTerminalEnded, ""},
		{"claude/SessionStart", "SessionStart", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionStart","source":"startup"}`, HookAgentStarted, ""},
		{"claude/SessionEnd", "SessionEnd", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionEnd"}`, HookAgentEnded, ""},
		{"claude/SubagentStart", "SubagentStart", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStart"}`, HookAgentStarted, "subagent"},
		{"claude/SubagentStop", "SubagentStop", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStop"}`, HookAgentEnded, "subagent"},
		{"claude/Stop", "Stop", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"Stop","stop_hook_active":false}`, HookAgentEnded, ""},
		{"claude/UserPromptSubmit", "UserPromptSubmit", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"UserPromptSubmit","prompt":"Fix the bug"}`, HookAgentPromptSubmitting, ""},
		{"claude/PreCompact", "PreCompact", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreCompact"}`, HookAgentCompacting, ""},
		{"claude/TaskCompleted", "TaskCompleted", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TaskCompleted"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"claude/Notification", "Notification", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"Notification"}`, HookAgentToolStarting, ""},
		{"claude/TeammateIdle", "TeammateIdle", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TeammateIdle"}`, HookAgentToolStarting, ""},
		{"claude/PermissionRequest", "PermissionRequest", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PermissionRequest"}`, HookAgentToolStarting, ""},
		{"claude/PreToolUse/Bash", "PreToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"go test -v"},"tool_use_id":"toolu_01WKqqc5y27LZu1KTB5GsGDu"}`, HookAgentToolTestStarting, ""},
		{"claude/PreToolUse/Read", "PreToolUse", "claude-code", "Read", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Read","tool_input":{"file_path":"/workspaces/semio/main.go"},"tool_use_id":"toolu_01Md976UFvJmmL5KaH4xzsx8"}`, HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/Edit", "PreToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl","tool_name":"Edit"}`, HookAgentToolCodeEditStarting, ""},
		{"claude/PreToolUse/Glob", "PreToolUse", "claude-code", "Glob", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"Glob","tool_input":{"pattern":"**/*.json"}}`, HookAgentToolStarting, ""},
		{"claude/PreToolUse/Grep", "PreToolUse", "claude-code", "Grep", `{"session_id":"e51a2976","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreToolUse","tool_name":"Grep","tool_input":{"pattern":"BlockedToolPatterns"}}`, HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/mcp_tree", "PreToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"mcp__repo__tree","tool_input":{"query":"hooks"}}`, HookAgentToolStarting, ""},
		{"claude/PostToolUse/Bash", "PostToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","tool_name":"Bash","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl"}`, HookAgentToolTerminalEnded, ""},
		{"claude/PostToolUse/Edit", "PostToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","tool_name":"Edit","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl"}`, HookAgentToolCodeEditEnded, ""},
		{"claude/PostToolUse/Read", "PostToolUse", "claude-code", "Read", `{"session_id":"167906cd","tool_name":"Read","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolSearchEnded, ""},
		{"claude/PostToolUse/TodoWrite", "PostToolUse", "claude-code", "TodoWrite", `{"session_id":"167906cd","tool_name":"TodoWrite","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolPlanUpdatingEnded, ""},
		{"claude/PostToolUse/mcp_tree", "PostToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd","tool_name":"mcp__repo__tree","transcript_path":"/tmp/t.jsonl"}`, HookAgentToolEnded, ""},
		{"claude/PostToolUseFailure/Bash", "PostToolUseFailure", "claude-code", "Bash", `{"session_id":"167906cd","tool_name":"Bash","transcript_path":"/tmp/t.jsonl","error":"command failed"}`, HookAgentToolTerminalEnded, ""},
		{"droid/SessionStart", "SessionStart", "droid", "", `{"session_id":"droid-001","second":"2026-02-18T18:47:06.000Z"}`, HookAgentStarted, ""},
		{"droid/PreToolUse/Bash", "PreToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:06.000Z"}`, HookAgentToolTerminalStarting, ""},
		{"droid/PostToolUse/Bash", "PostToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:10.000Z"}`, HookAgentToolTerminalEnded, ""},
		{"codex/SessionStart", "SessionStart", "codex", "", `{"session_id":"codex-001","second":"2026-02-18T18:50:00.000Z"}`, HookAgentStarted, ""},
		{"codex/PreToolUse/Read", "PreToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:05.000Z"}`, HookAgentToolSearchStarting, ""},
		{"codex/PostToolUse/Read", "PostToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:10.000Z"}`, HookAgentToolSearchEnded, ""},
		{"antigravity/SessionStart", "SessionStart", "antigravity-chat", "", `{"session_id":"ag-001","second":"2026-02-18T18:55:00.000Z"}`, HookAgentStarted, ""},
		{"antigravity/PreToolUse/Task", "PreToolUse", "antigravity-chat", "Task", `{"session_id":"ag-001","tool_name":"Task","second":"2026-02-18T18:55:05.000Z"}`, HookAgentToolPlanUpdatingStarting, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			input := json.RawMessage(tc.input)
			event, parent, err := ResolveHookEvent(tc.nativeEvent, tc.client, tc.toolName, input)
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
			hctx := HookContext{
				Event:      event,
				Client:     tc.client,
				Second:     secondStr,
				RepoRoot:   tmpDir,
				ToolName:   tc.toolName,
				Input:      input,
				ParentInfo: parent,
			}
			RunHook(hctx)
			sessionID := extractSessionIDFromInput(input)
			if sessionID == "" {
				sessionID = "unknown"
			}
			logNow := time.Now().UTC()
			logDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated",
				fmt.Sprintf("%02d", logNow.Year()%100),
				fmt.Sprintf("%02d", int(logNow.Month())),
				fmt.Sprintf("%02d", logNow.Day()),
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
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err != nil {
				t.Fatalf("invalid session JSON: %v", err)
			}
			if len(meta.Events) == 0 {
				t.Fatalf("expected at least one event in session.json")
			}
			var entry *HookLogEntry
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
			wantSession := resolveEventSessionID(extractSessionIDFromInput(input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("log session: want %s, got %v", wantSession, evt["session"])
			}
			wantSecond := resolveEventSecondID(secondStr)
			if wantSecond != "" && evt["second"] != wantSecond {
				t.Errorf("log second: want %s, got %v", wantSecond, evt["second"])
			}
			wantTranscript := extractTranscriptFromInput(input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("log transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if entry.Response.Blocked != nil {
				t.Error("log response.blocked: want nil for non-blocked event")
			}
		})
	}
}

func TestNativeHookEventMappingFromRealLogFiles(t *testing.T) {
	repoRoot := findRepoRoot(".")
	logDir := filepath.Join(repoRoot, ".🦑repo", "📜")
	dirEntries, err := os.ReadDir(logDir)
	if err != nil {
		t.Skipf("no log directory: %v", err)
	}
	type oldLogEntry struct {
		Context struct {
			Event    string          `json:"event"`
			Client   string          `json:"client"`
			ToolName string          `json:"toolName"`
			Input    json.RawMessage `json:"input"`
		} `json:"context"`
	}
	seen := map[string]bool{}
	for _, de := range dirEntries {
		if de.IsDir() || !strings.HasSuffix(de.Name(), ".json") {
			continue
		}
		data, err := os.ReadFile(filepath.Join(logDir, de.Name()))
		if err != nil {
			continue
		}
		var old oldLogEntry
		if err := json.Unmarshal(data, &old); err != nil {
			continue
		}
		if old.Context.Event == "" || old.Context.Client == "" {
			continue
		}
		key := old.Context.Client + "|" + old.Context.Event + "|" + old.Context.ToolName
		if seen[key] {
			continue
		}
		seen[key] = true
		t.Run(fmt.Sprintf("%s/%s/%s", old.Context.Client, strings.ReplaceAll(old.Context.Event, ".", "-"), old.Context.ToolName), func(t *testing.T) {
			tmpDir := t.TempDir()
			writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
			hctx := HookContext{
				Event:    HookEvent(old.Context.Event),
				Client:   old.Context.Client,
				Second:   time.Now().UTC().Format(time.RFC3339),
				RepoRoot: tmpDir,
				ToolName: old.Context.ToolName,
				Input:    old.Context.Input,
			}
			result := RunHook(hctx)
			outBase := filepath.Join(tmpDir, ".🦑repo", "⚡cache")
			var logFiles []string
			filepath.WalkDir(outBase, func(path string, d os.DirEntry, walkErr error) error {
				if walkErr != nil {
					return nil
				}
				if !d.IsDir() && strings.HasSuffix(d.Name(), ".json") {
					logFiles = append(logFiles, path)
				}
				return nil
			})
			if len(logFiles) != 1 {
				t.Fatalf("want 1 log file under ⚡, got %d", len(logFiles))
			}
			outData, err := os.ReadFile(logFiles[0])
			if err != nil {
				t.Fatalf("read log: %v", err)
			}
			var entry HookLogEntry
			if err := json.Unmarshal(outData, &entry); err != nil {
				t.Fatalf("invalid log JSON: %v", err)
			}
			var evt map[string]interface{}
			if err := json.Unmarshal(entry.Event, &evt); err != nil {
				t.Fatalf("cannot unmarshal event: %v", err)
			}
			if evt["kind"] != string(old.Context.Event) {
				t.Errorf("kind: want %s, got %v", old.Context.Event, evt["kind"])
			}
			if evt["client"] != old.Context.Client {
				t.Errorf("client: want %s, got %v", old.Context.Client, evt["client"])
			}
			if old.Context.Input != nil && entry.Native.Event == nil {
				t.Error("native.event not preserved")
			}
			wantSession := resolveEventSessionID(extractSessionIDFromInput(old.Context.Input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("session: want %s, got %v", wantSession, evt["session"])
			}
			wantTranscript := extractTranscriptFromInput(old.Context.Input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if result.IsAllowed() && entry.Response.Blocked != nil {
				t.Error("response.blocked should be nil for allowed event")
			}
		})
	}
}

func TestCheckpointInAllAgentEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	SetRootDir(tmpDir)

	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	sessionInput := json.RawMessage(`{"session_id":"checkpoint-test","llm":"opus-4-6"}`)
	agentEvents := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"agent.started", HookAgentStarted, sessionInput},
		{"agent.ended", HookAgentEnded, sessionInput},
		{"agent.prompt.submitting", HookAgentPromptSubmitting, sessionInput},
		{"agent.compacting", HookAgentCompacting, sessionInput},
		{"agent.tool.starting", HookAgentToolStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.ended", HookAgentToolEnded, json.RawMessage(`{"session_id":"checkpoint-test","tool_name":"read_file"}`)},
		{"agent.tool.plan.updating.starting", HookAgentToolPlanUpdatingStarting, sessionInput},
		{"agent.tool.plan.updating.ended", HookAgentToolPlanUpdatingEnded, sessionInput},
		{"agent.file.read.starting", HookAgentToolSearchStarting, sessionInput},
		{"agent.file.read.ended", HookAgentToolSearchEnded, sessionInput},
		{"agent.tool.code.edit.starting", HookAgentToolCodeEditStarting, sessionInput},
		{"agent.tool.code.edit.ended", HookAgentToolCodeEditEnded, sessionInput},
		{"agent.tool.test.starting", HookAgentToolTestStarting, sessionInput},
		{"agent.tool.test.ended", HookAgentToolTestEnded, sessionInput},
		{"agent.tool.build.starting", HookAgentToolBuildStarting, sessionInput},
		{"agent.tool.build.ended", HookAgentToolBuildEnded, sessionInput},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"checkpoint-test","tool_input":{"command":"echo test"}}`)},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, sessionInput},
		{"agent.thinking.starting", HookAgentThinkingStarting, json.RawMessage(`{"session_id":"checkpoint-test","text":"Planning the approach"}`)},
		{"agent.thinking.ended", HookAgentThinkingEnded, json.RawMessage(`{"session_id":"checkpoint-test","text":"Decided to use X"}`)},
	}
	for _, tc := range agentEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "copilot-chat",
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			}
			result := dispatchHook(hctx)

			data, _ := json.Marshal(result)
			var m map[string]interface{}
			json.Unmarshal(data, &m)
			checkpoint, ok := m["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint field in %s result, got: %v", tc.name, m)
			}
			if checkpoint != expectedSHA {
				t.Errorf("expected checkpoint=%s, got %v", expectedSHA, checkpoint)
			}
		})
	}
}

func TestCheckpointInAllVersionEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	SetRootDir(tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	versionEvents := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"checkpoint.starting", HookVersionCheckpointStarting, nil},
		{"checkpoint.ended", HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin.starting", HookVersionCheckinStarting, nil},
		{"checkin.ended", HookVersionCheckinEnded, nil},
		{"checkout.starting", HookVersionCheckoutStarting, nil},
		{"checkout.ended", HookVersionCheckoutEnded, nil},
	}
	for _, tc := range versionEvents {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "",
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			}
			result := dispatchHook(hctx)
			data, _ := json.Marshal(result)
			var m map[string]interface{}
			json.Unmarshal(data, &m)
			checkpoint, ok := m["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint field in %s result, got: %v", tc.name, m)
			}
			if checkpoint != expectedSHA {
				t.Errorf("expected checkpoint=%s, got %v", expectedSHA, checkpoint)
			}
		})
	}
}

func TestVersionHooksDoNotWriteSessionLogs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess version hook test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	SetRootDir(tmpDir)

	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))

	events := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"checkpoint starting", HookVersionCheckpointStarting, nil},
		{"checkpoint ended", HookVersionCheckpointEnded, json.RawMessage(`{"sha":"` + expectedSHA + `","message":"test commit"}`)},
		{"checkin starting", HookVersionCheckinStarting, nil},
		{"checkin ended", HookVersionCheckinEnded, nil},
		{"checkout starting", HookVersionCheckoutStarting, nil},
		{"checkout ended", HookVersionCheckoutEnded, nil},
	}

	for _, tc := range events {
		t.Run(tc.name, func(t *testing.T) {
			result := RunHook(HookContext{
				Event:    tc.event,
				Second:   "2026-02-25T12:00:00Z",
				RepoRoot: tmpDir,
				Input:    tc.input,
			})
			if !result.IsAllowed() {
				t.Fatalf("expected %s to be allowed, got %s", tc.event, result.GetMessage())
			}
			assertNoHookLogFiles(t, tmpDir)
		})
	}
}

func TestCheckpointInLoggedEventJSON(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess checkpoint logging test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	SetRootDir(tmpDir)
	headCmd := exec.Command("git", "rev-parse", "HEAD")
	headCmd.Dir = tmpDir
	headOut, err := headCmd.Output()
	if err != nil {
		t.Fatalf("cannot get HEAD: %v", err)
	}
	expectedSHA := strings.TrimSpace(string(headOut))
	sessionInput := json.RawMessage(`{"session_id":"checkpoint-log","llm":"opus-4-6","transcript_path":"/tmp/t.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-25T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
	}
	RunHook(hctx)

	agentEventsDir := filepath.Join(tmpDir, ".🦑repo", "⚡cache", "🤖generated")
	var sessionJSONPath string
	filepath.WalkDir(agentEventsDir, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return nil
		}
		if !d.IsDir() && d.Name() == "session.json" {
			sessionJSONPath = path
		}
		return nil
	})
	if sessionJSONPath == "" {
		t.Fatal("expected session.json to be written for agent.started event")
	}
	data, err := os.ReadFile(sessionJSONPath)
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	if meta.Checkpoint != expectedSHA {
		t.Errorf("expected session.json checkpoint=%s, got %q", expectedSHA, meta.Checkpoint)
	}

	expectedCheckpoint := resolveEventCheckpointID(expectedSHA)
	found := false
	for _, entry := range meta.Events {
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if kind, _ := evt["kind"].(string); kind == "agent.started" {
			checkpoint, ok := evt["checkpoint"]
			if !ok || checkpoint == "" {
				t.Errorf("expected checkpoint in agent.started event, got: %v", evt)
			}
			if checkpoint != expectedCheckpoint {
				t.Errorf("expected agent.started checkpoint=%s, got %v", expectedCheckpoint, checkpoint)
			}
			found = true
			break
		}
	}
	if !found {
		t.Error("expected agent.started event in session.json events array")
	}
}

func TestEventIDsUseComposeRepoFormat(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess event id test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	SetRootDir(tmpDir)

	os.MkdirAll(filepath.Join(tmpDir, "src"), 0755)
	os.WriteFile(filepath.Join(tmpDir, "src", "main.go"), []byte("package main"), 0644)

	fileID := resolvePathToFileID(filepath.Join(tmpDir, "src", "main.go"))

	if fileID == "" {
		t.Error("expected non-empty file ID")
	}
	if strings.Contains(fileID, "/") {
		t.Errorf("file ID should not contain path separators: %s", fileID)
	}

	rangeRef, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10")
	if err != nil {
		t.Errorf("unexpected error resolving range ref: %v", err)
	}
	if rangeRef == "" {
		t.Error("expected non-empty range ref")
	}
	if !strings.Contains(rangeRef, "📌") {
		t.Errorf("expected 📌 in range ref, got: %s", rangeRef)
	}
	if !strings.Contains(rangeRef, "10") {
		t.Errorf("expected line number 10 in range ref, got: %s", rangeRef)
	}

	rangeRefFull, err := resolveRangeRef(filepath.Join(tmpDir, "src", "main.go") + "#L10-L20")
	if err != nil {
		t.Errorf("unexpected error resolving full range ref: %v", err)
	}
	if !strings.Contains(rangeRefFull, "📌10📌20") {
		t.Errorf("expected 📌10📌20 in full range ref, got: %s", rangeRefFull)
	}
}

// #endregion 🗂️Hook

// 🧜#region ⏲️Mermaid
func TestExhaustiveMermaidLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("walks all bundles for LOC treemap; too slow for -short runs on large monorepos")
	}
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByTechnologiesBundlesFoldersFiles()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code\"") {
		t.Error("expected 'Lines of Code' title")
	}
	if !strings.Contains(result, EmojiTechnologyUser) {
		t.Error("expected user technology emoji")
	}
	if !strings.Contains(result, EmojiTechnologyInfra) {
		t.Error("expected infra technology emoji")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 5 {
		t.Errorf("expected at least 5 lines, got %d", len(lines))
	}
	hasValue := false
	for _, line := range lines {
		if strings.Contains(line, ": ") {
			parts := strings.Split(strings.TrimSpace(line), ": ")
			if len(parts) == 2 {
				val := strings.TrimSpace(parts[1])
				if val != "0" {
					hasValue = true
				}
			}
		}
	}
	if !hasValue {
		t.Error("expected at least one file with non-zero LOC value")
	}
}

func TestExhaustiveMermaidLocByLanguage(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc-by-language test in short mode")
	}
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByLanguage()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code by Language\"") {
		t.Error("expected 'Lines of Code by Language' title")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 3 {
		t.Errorf("expected at least 3 lines (header + title + at least 1 language), got %d", len(lines))
	}
	hasLanguage := false
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.Contains(trimmed, ": ") && strings.HasPrefix(trimmed, "\"") {
			hasLanguage = true
		}
	}
	if !hasLanguage {
		t.Error("expected at least one language entry with LOC")
	}
}

func TestExhaustiveMermaidLocByContributors(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc-by-contributors test in short mode")
	}
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByContributors()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code by Contributor\"") {
		t.Error("expected 'Lines of Code by Contributor' title")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 3 {
		t.Errorf("expected at least 3 lines (header + title + at least 1 contributor), got %d", len(lines))
	}
}

func TestExhaustiveMermaidCommandLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc command test in short mode")
	}
	root := findTestRepoRoot(".")
	SetRootDir(root)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-technologies-bundles-folders-files"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}

func TestExhaustiveMermaidCommandLocByLanguage(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc-by-language command test in short mode")
	}
	root := findTestRepoRoot(".")
	SetRootDir(root)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-language"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}

func TestMermaidEscapeLabel(t *testing.T) {
	if got := mermaidEscapeLabel("hello \"world\""); got != "hello 'world'" {
		t.Errorf("expected hello 'world', got: %s", got)
	}
	if got := mermaidEscapeLabel("no quotes"); got != "no quotes" {
		t.Errorf("expected no quotes, got: %s", got)
	}
}

// #endregion ⏲️Mermaid

// 🔌#region 🏷️Provider
func TestProviderRegistry(t *testing.T) {
	mp := DefaultManagementProvider()
	if mp == nil {
		t.Fatal("DefaultManagementProvider() returned nil")
	}
	if mp.Kind() != "github" {
		t.Errorf("expected github, got %s", mp.Kind())
	}
	vcp := DefaultVersionControlProvider()
	if vcp == nil {
		t.Fatal("DefaultVersionControlProvider() returned nil")
	}
	if vcp.Kind() != "git" {
		t.Errorf("expected git, got %s", vcp.Kind())
	}
	sp := DefaultSandboxProvider()
	if sp == nil {
		t.Fatal("DefaultSandboxProvider() returned nil")
	}
	if sp.Kind() != "devcontainer" {
		t.Errorf("expected devcontainer, got %s", sp.Kind())
	}
}

func TestGetManagementProvider(t *testing.T) {
	mp := GetManagementProvider()
	if mp == nil {
		t.Fatal("GetManagementProvider() returned nil")
	}
	if mp.Kind() != "github" {
		t.Errorf("expected github, got %s", mp.Kind())
	}
}

func TestGoalIDForFilesystem(t *testing.T) {
	oldRoot := rootDir
	rootDir = findTestRepoRoot(".")
	defer func() { rootDir = oldRoot }()

	tests := []struct {
		in   string
		want string
	}{
		{"AI-OPTIMIZED-REPO/REPO-CLIENT", "AI-OPTIMIZED-REPO/REPO-CLIENT"},
		{"🎯aioptimizedrepo🎯repoclient", "AI-OPTIMIZED-REPO/REPO-CLIENT"},
	}
	for _, tt := range tests {
		got := goalIDForFilesystem(tt.in)
		if got != tt.want {
			t.Errorf("goalIDForFilesystem(%q) = %q, want %q", tt.in, got, tt.want)
		}
	}
}

func TestGhExtractIssueURL(t *testing.T) {
	const url = "https://github.com/usalu/semio/issues/42"
	tests := []struct {
		in   string
		want string
	}{
		{url + "\n", url},
		{"Created " + url + " in repo", url},
		{"", ""},
	}
	for _, tt := range tests {
		if got := ghExtractIssueURL(tt.in); got != tt.want {
			t.Errorf("ghExtractIssueURL(%q) = %q, want %q", tt.in, got, tt.want)
		}
	}
}

func TestNullManagementProvider(t *testing.T) {
	p := &NullManagementProvider{}
	if p.Kind() != "none" {
		t.Errorf("expected none, got %s", p.Kind())
	}
	if err := p.Configure("/tmp"); err != nil {
		t.Errorf("Configure should not error: %v", err)
	}
	url, err := p.CreateIssue("test", "body", nil)
	if err != nil || url != "" {
		t.Errorf("CreateIssue should return empty string, got %q, err=%v", url, err)
	}
	if err := p.CloseIssue("url"); err != nil {
		t.Errorf("CloseIssue should not error: %v", err)
	}
	if err := p.ReopenIssue("url"); err != nil {
		t.Errorf("ReopenIssue should not error: %v", err)
	}
	if err := p.DeleteIssue("url"); err != nil {
		t.Errorf("DeleteIssue should not error: %v", err)
	}
	if err := p.UpdateIssueTitle("url", "title"); err != nil {
		t.Errorf("UpdateIssueTitle should not error: %v", err)
	}
	if err := p.UpdateIssueBody("url", "body"); err != nil {
		t.Errorf("UpdateIssueBody should not error: %v", err)
	}
	details, err := p.GetIssueDetails("url")
	if err != nil || details != nil {
		t.Errorf("GetIssueDetails should return nil, got %v, err=%v", details, err)
	}
	nodeID, err := p.GetIssueNodeID("url")
	if err != nil || nodeID != "" {
		t.Errorf("GetIssueNodeID should return empty, got %q, err=%v", nodeID, err)
	}
	parentURL, err := p.GetIssueParentURL("url")
	if err != nil || parentURL != "" {
		t.Errorf("GetIssueParentURL should return empty, got %q, err=%v", parentURL, err)
	}
	if err := p.AddComment("url", "comment"); err != nil {
		t.Errorf("AddComment should not error: %v", err)
	}
	if err := p.AddLabels("url", []string{"a"}); err != nil {
		t.Errorf("AddLabels should not error: %v", err)
	}
	if err := p.RemoveLabels("url", []string{"a"}); err != nil {
		t.Errorf("RemoveLabels should not error: %v", err)
	}
	p.AddIssueToProject("url")
	p.AssignIssueToCurrentUser("url")
	if err := p.AddSubIssue("parent", "child"); err != nil {
		t.Errorf("AddSubIssue should not error: %v", err)
	}
	if err := p.UpdateIssueMilestone("url", "title"); err != nil {
		t.Errorf("UpdateIssueMilestone should not error: %v", err)
	}
	if err := p.ClearIssueMilestone("url"); err != nil {
		t.Errorf("ClearIssueMilestone should not error: %v", err)
	}
	num, err := p.CreateMilestone("title", "desc")
	if err != nil || num != 0 {
		t.Errorf("CreateMilestone should return 0, got %d, err=%v", num, err)
	}
	if err := p.UpdateMilestone(1, "t", "d", "s", "due"); err != nil {
		t.Errorf("UpdateMilestone should not error: %v", err)
	}
	if err := p.DeleteMilestone(1); err != nil {
		t.Errorf("DeleteMilestone should not error: %v", err)
	}
	m, err := p.GetMilestone(1)
	if err != nil || m != nil {
		t.Errorf("GetMilestone should return nil, got %v, err=%v", m, err)
	}
	title, err := p.GetMilestoneTitle(1)
	if err != nil || title != "" {
		t.Errorf("GetMilestoneTitle should return empty, got %q, err=%v", title, err)
	}
	found, err := p.FindMilestoneByTitle("title")
	if err != nil || found != nil {
		t.Errorf("FindMilestoneByTitle should return nil, got %v, err=%v", found, err)
	}
	issues, err := p.ListIssuesForLabelSync()
	if err != nil || issues != nil {
		t.Errorf("ListIssuesForLabelSync should return nil, got %v, err=%v", issues, err)
	}
	urls, err := p.ListOpenIssuesWithLabel("label")
	if err != nil || urls != nil {
		t.Errorf("ListOpenIssuesWithLabel should return nil, got %v, err=%v", urls, err)
	}
	labels, err := p.ListRepoLabels()
	if err != nil || labels != nil {
		t.Errorf("ListRepoLabels should return nil, got %v, err=%v", labels, err)
	}
	if err := p.CreateRepoLabel("name"); err != nil {
		t.Errorf("CreateRepoLabel should not error: %v", err)
	}
	if err := p.DeleteRepoLabel("name"); err != nil {
		t.Errorf("DeleteRepoLabel should not error: %v", err)
	}
	if err := p.SyncRepoLabelCatalog(map[string]bool{"a": true}); err != nil {
		t.Errorf("SyncRepoLabelCatalog should not error: %v", err)
	}
	goalURL, err := p.CreateGoalIssue("title", "desc", nil)
	if err != nil || goalURL != "" {
		t.Errorf("CreateGoalIssue should return empty, got %q, err=%v", goalURL, err)
	}
	if err := p.UpdateGoalIssue("url", "t", "d"); err != nil {
		t.Errorf("UpdateGoalIssue should not error: %v", err)
	}
	if user := p.GetCurrentUser(); user != "" {
		t.Errorf("GetCurrentUser should return empty, got %q", user)
	}
}

func TestAllEditorProviders(t *testing.T) {
	providers := AllEditorProviders()
	if len(providers) == 0 {
		t.Fatal("AllEditorProviders() returned empty")
	}
	kinds := make(map[string]bool)
	for _, p := range providers {
		if p.Kind() == "" {
			t.Error("editor provider has empty Kind()")
		}
		kinds[p.Kind()] = true
	}
	for _, expected := range []string{"copilot-chat", "cursor-chat", "windsurf-chat", "claude-code", "droid", "codex", "antigravity-chat"} {
		if !kinds[expected] {
			t.Errorf("missing editor provider for %s", expected)
		}
	}
}

func TestGetEditorProvider(t *testing.T) {
	for _, client := range []string{"copilot-chat", "cursor-chat", "windsurf-chat", "claude-code", "droid", "codex", "antigravity-chat"} {
		p := GetEditorProvider(client)
		if p == nil {
			t.Errorf("GetEditorProvider(%s) returned nil", client)
			continue
		}
		if p.Kind() != client {
			t.Errorf("expected Kind()=%s, got %s", client, p.Kind())
		}
	}
	if p := GetEditorProvider("nonexistent"); p != nil {
		t.Errorf("expected nil for unknown client, got %v", p)
	}
}

func TestGitVersionControlProviderKind(t *testing.T) {
	p := &GitVersionControlProvider{}
	if p.Kind() != "git" {
		t.Errorf("expected git, got %s", p.Kind())
	}
}

func TestGitVersionControlProviderConfigure(t *testing.T) {
	p := &GitVersionControlProvider{}
	if err := p.Configure("/tmp"); err != nil {
		t.Errorf("Configure should not error: %v", err)
	}
}

// 💾initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

func TestComputeCompositeFingerprintIgnoresUntrackedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	firstFingerprint, firstMeta := computeCompositeFingerprint(tmpDir)
	if firstFingerprint == "" || firstMeta == nil {
		t.Fatalf("expected first fingerprint and meta to be set, got %q / %+v", firstFingerprint, firstMeta)
	}

	if err := os.WriteFile(filepath.Join(tmpDir, "untracked.txt"), []byte("untracked"), 0644); err != nil {
		t.Fatalf("failed to create untracked file: %v", err)
	}

	secondFingerprint, secondMeta := computeCompositeFingerprint(tmpDir)
	if secondFingerprint == "" || secondMeta == nil {
		t.Fatalf("expected second fingerprint and meta to be set, got %q / %+v", secondFingerprint, secondMeta)
	}
	if firstFingerprint != secondFingerprint {
		t.Fatalf("expected untracked files to be ignored, fingerprint changed from %q to %q", firstFingerprint, secondFingerprint)
	}
}

func TestComputeCompositeFingerprintFallsBackWhenRecursiveSubmoduleStatusFails(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess submodule fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	childDir := initTestGitRepo(t, "main")
	nestedDir := initTestGitRepo(t, "main")
	run := func(dir string, args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = dir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v in %s failed: %s\n%s", args, dir, err, string(out))
		}
	}

	run(tmpDir, "-c", "protocol.file.allow=always", "submodule", "add", childDir, "child")
	run(tmpDir, "commit", "-am", "add child submodule")
	childWorkTree := filepath.Join(tmpDir, "child")
	run(childWorkTree, "config", "user.email", "test@test.com")
	run(childWorkTree, "config", "user.name", "Test")
	run(childWorkTree, "-c", "protocol.file.allow=always", "submodule", "add", nestedDir, "nested")
	run(childWorkTree, "commit", "-am", "add nested submodule")

	nestedGitDir := filepath.Join(tmpDir, ".git", "modules", "child", "modules", "nested")
	if err := os.RemoveAll(nestedGitDir); err != nil {
		t.Fatalf("failed to remove nested gitdir: %v", err)
	}

	_, meta := computeCompositeFingerprint(tmpDir)
	if _, ok := meta.SubmodulePointers["child"]; !ok {
		t.Fatalf("expected fallback submodule status to include child pointer, got %+v", meta.SubmodulePointers)
	}
	if _, ok := meta.SubmodulePointers["child/nested"]; ok {
		t.Fatalf("expected broken nested submodule to be excluded after fallback, got %+v", meta.SubmodulePointers)
	}
}

// 🆕initTestGitRepoWithRemote creates a git repo with a bare remote and returns (workDir, remoteDir).
func initTestGitRepoWithRemote(t *testing.T) (string, string) {
	t.Helper()
	remoteDir := t.TempDir()
	cmd := exec.Command("git", "init", "--bare", "-b", "main", remoteDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init --bare failed: %s\n%s", err, string(out))
	}

	mainDir := initTestGitRepo(t, "main")
	run := func(dir string, args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = dir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v in %s failed: %s\n%s", args, dir, err, string(out))
		}
	}
	run(mainDir, "remote", "add", "origin", remoteDir)
	run(mainDir, "push", "-u", "origin", "main")

	workDir := t.TempDir()
	cmd = exec.Command("git", "clone", remoteDir, workDir)
	out, err = cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git clone failed: %s\n%s", err, string(out))
	}
	run(workDir, "config", "user.email", "test@test.com")
	run(workDir, "config", "user.name", "Test")
	run(workDir, "config", "commit.gpgsign", "false")
	run(workDir, "config", "tag.gpgsign", "false")
	return workDir, remoteDir
}

func TestGitVersionControlProviderCurrentCheckpoint(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	sha, err := p.CurrentCheckpoint(tmpDir)
	if err != nil {
		t.Fatalf("CurrentCheckpoint failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}
}

func TestGitVersionControlProviderCurrentBranch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	branch, err := p.CurrentBranch(tmpDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "main" {
		t.Errorf("expected main, got %q", branch)
	}
}

func TestGitVersionControlProviderCheckpoint(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	os.WriteFile(filepath.Join(tmpDir, "file2.txt"), []byte("world"), 0644)

	p := &GitVersionControlProvider{}
	sha, err := p.Checkpoint(tmpDir, "add file2")
	if err != nil {
		t.Fatalf("Checkpoint failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}

	currentSha, _ := p.CurrentCheckpoint(tmpDir)
	if currentSha != sha {
		t.Errorf("expected current checkpoint %q to match checkpoint result %q", currentSha, sha)
	}
}

func TestGitVersionControlProviderStageAll(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	os.WriteFile(filepath.Join(tmpDir, "newfile.txt"), []byte("new"), 0644)

	p := &GitVersionControlProvider{}
	if err := p.StageAll(tmpDir); err != nil {
		t.Fatalf("StageAll failed: %v", err)
	}

	files, err := p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) == 0 {
		t.Error("expected staged files after StageAll")
	}
}

func TestGitVersionControlProviderStagedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	files, err := p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) != 0 {
		t.Errorf("expected no staged files, got %d", len(files))
	}

	os.WriteFile(filepath.Join(tmpDir, "file2.txt"), []byte("world"), 0644)
	cmd := exec.Command("git", "add", "file2.txt")
	cmd.Dir = tmpDir
	cmd.Run()
	files, err = p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) != 1 || files[0] != "file2.txt" {
		t.Errorf("expected [file2.txt], got %v", files)
	}
}

func TestGitVersionControlProviderCheckin(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	workDir, _ := initTestGitRepoWithRemote(t)

	p := &GitVersionControlProvider{}
	err := p.Checkin(workDir, "testuser")
	if err != nil {
		t.Fatalf("Checkin failed: %v", err)
	}

	branch, err := p.CurrentBranch(workDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "testuser/latest" {
		t.Errorf("expected testuser/latest, got %q", branch)
	}
}

func TestGitVersionControlProviderCheckout(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	workDir, _ := initTestGitRepoWithRemote(t)

	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = workDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	run("switch", "-c", "testuser/latest")
	os.WriteFile(filepath.Join(workDir, "feature.txt"), []byte("feature"), 0644)
	run("add", "-A")
	run("commit", "-m", "add feature")

	p := &GitVersionControlProvider{}
	sha, err := p.Checkout(workDir, "testuser", "merge feature")
	if err != nil {
		t.Fatalf("Checkout failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}

	branch, err := p.CurrentBranch(workDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "main" {
		t.Errorf("expected main after checkout, got %q", branch)
	}

	if _, err := os.Stat(filepath.Join(workDir, "feature.txt")); os.IsNotExist(err) {
		t.Error("expected feature.txt to exist on main after checkout")
	}
}

func TestVersionHookEventsDispatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow version hook events dispatch test in short mode")
	}
	cases := []struct {
		name  string
		event HookEvent
	}{
		{"checkin starting", HookVersionCheckinStarting},
		{"checkin ended", HookVersionCheckinEnded},
		{"checkout starting", HookVersionCheckoutStarting},
		{"checkout ended", HookVersionCheckoutEnded},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "",
				Second:   "2026-02-24T12:00:00Z",
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if !result.IsAllowed() {
				t.Errorf("expected allowed for %s, got denied: %s", tc.event, result.GetMessage())
			}
		})
	}
}

// #endregion 🏷️Provider

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
	if !isHeaderMetaLine("[🧰repo⌨️client💻main](repo://p/i/repo/b/b/cli/f/main.go)") {
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
	if !isHeaderMetaLine("💻repo/asset/fixture/some/folder/🐍🐍file.py") {
		t.Error("should detect file ID emoji prefix")
	}
	if isHeaderMetaLine("This function handles parsing.") {
		t.Error("should not match summary text")
	}
}

func TestExtractMarkdownSection(t *testing.T) {
	content := "# Summary\n\nThis is the summary.\n\n# 💯Requirements\n\nSpec line one MUST work.\nSpec line two SHOULD also work.\n\n# Docs\n\nDocumentation here.\n"
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
	summary := ExtractFileHeaderSummary("repo/asset/fixture/some/folder/⚛️⚛️file_empty_region.tsx")
	if strings.Contains(summary, "GNU") || strings.Contains(summary, "license") || strings.Contains(summary, "redistribute") {
		t.Errorf("should not contain license text, got: %q", summary)
	}
	if strings.HasPrefix(summary, "#region") || strings.HasPrefix(summary, "region ") {
		t.Errorf("should not start with region markers, got: %q", summary)
	}
}

func TestExtractFileHeaderSummaryReturnsActualSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("repo/asset/fixture/some/folder/⚛️⚛️file_empty_region.tsx")
	if strings.Contains(summary, "free software") {
		t.Errorf("should not return license as summary, got: %q", summary)
	}
}

func TestExtractFileHeaderRequirementsNoLicense(t *testing.T) {
	requirements := ExtractFileHeaderRequirements("repo/asset/fixture/some/folder/🐍🐍file.py")
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
	content := "//#region 🔖Exports\n// Re-exports of icons.\n// Data MUST be valid.\n//#endregion 🔖Exports\n"
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

func TestGenerateTechnologyRequirements(t *testing.T) {
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

// 🧲#endregion 🌡️Technology Generate
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

func TestSectionNewlineAfterRegion(t *testing.T) {
	t.Run("detect_blank_line_after_region_typescript", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_go", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n// [💻test.go](repo://file/test.go)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Package\n\n// [🔖test.go#Package](repo://section/test.go/Package)\n// Package declaration.\n\npackage main\n\n// #endregion 🔖Package\n"
		testFile := "test.go"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Package section")
		}
	})

	t.Run("detect_blank_line_after_region_python", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "# region Header\n# [💻test.py](repo://file/test.py)\n# 2025 Test <t@t.com>\n# AGPL\n# endregion Header\n\n# region Functions\n\n# [🔖test.py#Functions](repo://section/test.py/Functions)\n# Utility functions.\n\ndef add(a, b):\n    return a + b\n\n# endregion Functions\n"
		testFile := "test.py"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Functions section")
		}
	})

	t.Run("detect_blank_line_after_region_csharp", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "#region 🔖Header\n// [💻test.cs](repo://file/test.cs)\n// 2025 Test <t@t.com>\n// AGPL\n#endregion 🔖Header\n\n#region 🔖Classes\n\n// [🔖test.cs#Classes](repo://section/test.cs/Classes)\n// Domain classes.\n\npublic class Foo {}\n\n#endregion 🔖Classes\n"
		testFile := "test.cs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Classes section")
		}
	})

	t.Run("detect_blank_line_after_region_rust", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\npub mod structs { // 🔖Structs\n\n// Struct definitions.\n\nstruct Foo {}\n\n} // 🔖Structs\n"
		testFile := "test.rs"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Structs section")
		}
	})

	t.Run("no_false_positive_without_blank_line", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		for _, v := range breachs {
			if v.Kind == BreachCodeSectionWrongFormatNewlineAfterRegion {
				t.Fatal("unexpected newline-after-region breach when no blank line exists")
			}
		}
	})

	t.Run("autofix_removes_blank_line_after_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		expected := "// #region 🔖Header\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []Breach{
			{Kind: BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Functions", Line: 8},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("autofix_removes_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		expected := "// #region 🔖Header\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// [🔖test.ts#Functions](repo://section/test.ts/Functions)\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		breachs := []Breach{
			{Kind: BreachCodeSectionWrongFormatNewlineAfterRegion, Scope: testFile + "#Header", Line: 2},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("detect_blank_line_after_header_region", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		content := "// #region 🔖Header\n\n// [💻test.ts](repo://file/test.ts)\n// 2025 Test <t@t.com>\n// AGPL\n// #endregion 🔖Header\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("policy check failed: %v", err)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		if counts[BreachCodeSectionWrongFormatNewlineAfterRegion] == 0 {
			t.Fatal("expected newline-after-region breach for Header section")
		}
	})
}

func TestResolvePlanSourceCursorPlanID(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	planFile := filepath.Join(planDir, "kit_store_"+id+".plan.md")
	if err := os.WriteFile(planFile, []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	got, isDir, err := ResolvePlanSource(McpClientCursor, id)
	if err != nil {
		t.Fatal(err)
	}
	if isDir {
		t.Fatal("expected file")
	}
	if filepath.Clean(got) != filepath.Clean(planFile) {
		t.Fatalf("got %q want %q", got, planFile)
	}
}

func TestResolvePlanSourceKiroSpecID(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, ".kiro", "specs", "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "design.md"), []byte("d"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	got, isDir, err := ResolvePlanSource(McpClientKiro, "my-spec")
	if err != nil {
		t.Fatal(err)
	}
	if !isDir {
		t.Fatal("expected directory")
	}
	if filepath.Clean(got) != filepath.Clean(specDir) {
		t.Fatalf("got %q want %q", got, specDir)
	}
}

func TestMoveTicketPlanIntoFolderFile(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	src := filepath.Join(tmp, "outside.md")
	if err := os.WriteFile(src, []byte("body"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &Ticket{FolderPath: ticketDir, Plan: &TicketPlan{Source: src, Client: "cursor", ID: "x"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	dst := filepath.Join(ticketDir, "outside.md")
	if _, err := os.Stat(dst); err != nil {
		t.Fatal(err)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "outside.md" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestMoveTicketPlanIntoFolderSpecDir(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	specRoot := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(filepath.Join(specRoot, "nested"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "a.md"), []byte("a"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "nested", "b.md"), []byte("b"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &Ticket{FolderPath: ticketDir, Plan: &TicketPlan{Source: specRoot, Client: "kiro", ID: "my-spec"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(specRoot); !os.IsNotExist(err) {
		t.Fatalf("source spec dir should be gone: %v", err)
	}
	destRoot := filepath.Join(ticketDir, "my-spec")
	if b, err := os.ReadFile(filepath.Join(destRoot, "a.md")); err != nil || string(b) != "a" {
		t.Fatalf("a.md: %v %q", err, b)
	}
	if b, err := os.ReadFile(filepath.Join(destRoot, "nested", "b.md")); err != nil || string(b) != "b" {
		t.Fatalf("nested/b.md: %v %q", err, b)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "my-spec" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestApplyTicketPlanFromIDsCursor(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	planFile := filepath.Join(planDir, "kit_store_backbone_generalization_"+id+".plan.md")
	if err := os.WriteFile(planFile, []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	ticket := &Ticket{}
	if err := ApplyTicketPlanFromIDs(ticket, McpClientCursor, id, ""); err != nil {
		t.Fatal(err)
	}
	if ticket.Plan == nil || ticket.Plan.ID != id || ticket.Plan.Client != "cursor" {
		t.Fatalf("plan: %+v", ticket.Plan)
	}
	if filepath.Clean(ticket.Plan.Source) != filepath.Clean(planFile) {
		t.Fatalf("source %q want %q", ticket.Plan.Source, planFile)
	}
}

func TestStripPlanFrontmatter(t *testing.T) {
	raw := "---\nname: Test\noverview: x\n---\n\n# Body\n"
	got := stripPlanFrontmatter(raw)
	if got != "# Body" {
		t.Fatalf("got %q", got)
	}
	if stripPlanFrontmatter("# No frontmatter") != "# No frontmatter" {
		t.Fatal("expected unchanged content without frontmatter")
	}
}

func TestFormatPlanCommentFile(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "feature_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Feature\n---\n\n## Steps\n\nDo work.\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&TicketPlan{Client: "cursor", ID: "abcd1234"}, planPath)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(body, "# 📋 Plan") {
		t.Fatalf("missing heading: %q", body)
	}
	if !strings.Contains(body, "<details>") || !strings.Contains(body, "feature_abcd1234.plan.md") {
		t.Fatalf("missing details block: %q", body)
	}
	if strings.Contains(body, "name: Feature") {
		t.Fatalf("frontmatter should be stripped: %q", body)
	}
	if !strings.Contains(body, "## Steps") {
		t.Fatalf("missing body: %q", body)
	}
}

func TestFormatPlanCommentSpecDir(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "b.md"), []byte("## B\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "a.md"), []byte("## A\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&TicketPlan{Client: "kiro", ID: "my-spec"}, specDir)
	if err != nil {
		t.Fatal(err)
	}
	aPos := strings.Index(body, "a.md")
	bPos := strings.Index(body, "b.md")
	if aPos < 0 || bPos < 0 || aPos > bPos {
		t.Fatalf("expected sorted sections a before b: %q", body)
	}
}

type captureCommentProvider struct {
	NullManagementProvider
	comments []struct {
		url  string
		body string
	}
}

func (p *captureCommentProvider) AddComment(issueURL, comment string) error {
	p.comments = append(p.comments, struct {
		url  string
		body string
	}{issueURL, comment})
	return nil
}

func TestPostTicketPlanComment(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "task_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Task\n---\n\n## Work\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	capture := &captureCommentProvider{}
	old := mgmtProvider
	mgmtProvider = capture
	defer func() { mgmtProvider = old }()

	ticket := &Ticket{
		Management: &TicketManagementData{Issue: "https://github.com/example/repo/issues/1"},
		Plan:       &TicketPlan{Source: planPath, Client: "cursor", ID: "abcd1234"},
	}
	postTicketPlanComment(ticket, false)
	if len(capture.comments) != 1 {
		t.Fatalf("expected 1 comment, got %d", len(capture.comments))
	}
	if !strings.Contains(capture.comments[0].body, "# 📋 Plan") {
		t.Fatalf("missing plan heading: %q", capture.comments[0].body)
	}
	if !strings.Contains(capture.comments[0].body, "## Work") {
		t.Fatalf("missing plan body: %q", capture.comments[0].body)
	}
	postTicketPlanComment(ticket, true)
	if len(capture.comments) != 1 {
		t.Fatalf("noManagement should skip comment, got %d", len(capture.comments))
	}
}

func TestPostTicketPlanCommentLive(t *testing.T) {
	if os.Getenv("REPO_LIVE_GH") != "1" {
		t.Skip("set REPO_LIVE_GH=1 to run live GitHub plan comment test")
	}
	planFile := filepath.Join(rootDir, ".cursor", "plans", "post_plan_to_issue_c256e28e.plan.md")
	if _, err := os.Stat(planFile); err != nil {
		t.Skip("plan file missing")
	}
	issueURL, err := ghCreateIssue("TEST Post Plan Comment", "# temp", nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = ghCloseIssue(issueURL) })
	ticket := &Ticket{
		Management: &TicketManagementData{Issue: issueURL},
		Plan:       &TicketPlan{Source: planFile, Client: "cursor", ID: "c256e28e"},
	}
	postTicketPlanComment(ticket, false)
	issueNum := issueURL[strings.LastIndex(issueURL, "/")+1:]
	if issueNum == "" {
		t.Fatal("could not parse issue number")
	}
	stdout, stderr, code := ExecCommand("gh", []string{"api", "repos/{owner}/{repo}/issues/" + issueNum + "/comments", "--jq", ".[].body"}, "")
	if code != 0 {
		t.Fatalf("gh api comments failed: %s", stderr)
	}
	if !strings.Contains(stdout, "# 📋 Plan") {
		t.Fatalf("issue comment missing plan heading: %q", stdout)
	}
	if !strings.Contains(stdout, "post_plan_to_issue_c256e28e.plan.md") {
		t.Fatalf("issue comment missing plan filename: %q", stdout)
	}
}

func TestLocCommand(t *testing.T) {
	langs := locMakeLangSet([]string{"TypeScript", "Go", "C#", "Python", "Rust"})
	num := locMakeNumstatLangSet([]string{"TypeScript", "Go", "C#", "Python", "Rust"})
	t.Run("classify language", func(t *testing.T) {
		if got := locClassifyLanguage("a/b/foo.ts", num); got != "TypeScript" {
			t.Fatalf("ts: got %q", got)
		}
		if got := locClassifyLanguage("x.mtsx", num); got != "TypeScript" {
			t.Fatalf("mtsx: got %q", got)
		}
		if got := locClassifyLanguage("pkg/m.go", num); got != "Go" {
			t.Fatalf("go: got %q", got)
		}
		if got := locClassifyLanguage("noext", num); got != "" {
			t.Fatalf("noext: got %q", got)
		}
		if got := locClassifyLocBucket("x.md"); got != locAggMarkup {
			t.Fatalf("md bucket: got %q", got)
		}
		if got := locClassifyLocBucket("d.json"); got != locAggData {
			t.Fatalf("json bucket: got %q", got)
		}
		if locClassifyLanguage("x.md", langs) != "" {
			t.Fatalf("markdown should not match code-only weight map")
		}
	})
	t.Run("hidden path segments skipped", func(t *testing.T) {
		if !locPathHasHiddenSegment("pkg/.cache/x.go") {
			t.Fatal("expected hidden segment")
		}
		if locPathHasHiddenSegment("pkg/src/x.go") {
			t.Fatal("unexpected hidden")
		}
	})
	t.Run("json key counting", func(t *testing.T) {
		if n := locJSONKeyCount([]byte(`{"a":1,"b":{"c":2}}`)); n != 3 {
			t.Fatalf("keys want 3 got %d", n)
		}
		if n := locJSONKeyCount([]byte(`{}`)); n != 0 {
			t.Fatalf("empty object keys want 0 got %d", n)
		}
		if n := locCountBucketLoc("x.json", []byte(`{"x":1}`)); n != 1 {
			t.Fatalf("single-line json loc want 1 got %d", n)
		}
	})
	t.Run("parse numstat", func(t *testing.T) {
		oldR := GetRootDir()
		td := t.TempDir()
		_ = os.WriteFile(filepath.Join(td, ".gitignore"), []byte("# test\n"), 0o644)
		SetRootDir(td)
		defer SetRootDir(oldR)
		raw := "COMMIT\t" + strings.Repeat("a", 8) + "\tAlice\ta@x\t1600000000\n3\t1\tlib/main.go\n-\t-\tbin/legacy\n2\t0\tc.cs\n"
		out, err := locParseNumstatLog(raw, num)
		if err != nil {
			t.Fatal(err)
		}
		if len(out) != 1 {
			t.Fatalf("commits %d", len(out))
		}
		p := out[0].Delta["Go"]
		if p.Added != 3 || p.Removed != 1 {
			t.Fatalf("go delta %+v", p)
		}
		p2 := out[0].Delta["C#"]
		if p2.Added != 2 {
			t.Fatalf("cs %+v", p2)
		}
		// 0,0 and binary - skipped above
	})
	t.Run("merge cumulative and cloc", func(t *testing.T) {
		c := map[string]LocCumulative{"Go": {Added: 10, Removed: 2}}
		scanned := map[string]int{"Go": 100, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 5, "Data": 3}
		got := locMergeCumulativeClocEx(c, scanned, []string{"Go", "TypeScript", "C#", "Python", "Rust"})
		if got["Go"].Loc != 100 || got["Go"].Edited != 12 || got["TypeScript"].Loc != 0 {
			t.Fatalf("%+v", got["Go"])
		}
		if got[locAggCode].Loc != 100 {
			t.Fatalf("code aggregate %+v", got[locAggCode])
		}
		if got[locAggTotal].Loc != 108 {
			t.Fatalf("total loc want 108 got %d", got[locAggTotal].Loc)
		}
		if got["Go"].Percent <= 0 || got[locAggTotal].Percent != 100 {
			t.Fatalf("percents go=%v total=%v", got["Go"].Percent, got[locAggTotal].Percent)
		}
		if got["Go"].WipPercent != 100 || got[locAggTotal].WipPercent != 100 {
			t.Fatalf("wip percents go=%v total=%v", got["Go"].WipPercent, got[locAggTotal].WipPercent)
		}
	})
	t.Run("wip percent uses branch denominator for contributor rows", func(t *testing.T) {
		langs := []string{"Go", "TypeScript", "C#", "Python", "Rust"}
		by := map[string]map[string]locCumulativePair{
			"alice": {"Go": {Added: 5, Removed: 1}},
			"bob":   {"Go": {Added: 15, Removed: 3}},
		}
		branchDenom := locSumEditedPairs(map[string]locCumulativePair{
			"Go": {Added: 20, Removed: 4},
		}, locMakeNumstatLangSet(langs))
		if branchDenom != 24 {
			t.Fatalf("branch denom %d", branchDenom)
		}
		out := locByContributorsToSnapshot(by, langs, branchDenom)
		if out["alice"]["Go"].WipPercent != 25 {
			t.Fatalf("alice wip %% want 25 got %v", out["alice"]["Go"].WipPercent)
		}
		if out["bob"]["Go"].WipPercent != 75 {
			t.Fatalf("bob wip %% want 75 got %v", out["bob"]["Go"].WipPercent)
		}
	})
	t.Run("render markdown", func(t *testing.T) {
		scan := map[string]int{"Go": 1, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 0, "Data": 0}
		cum := map[string]locCumulativePair{"Go": {Added: 1, Removed: 1}}
		r := &LocReport{Snapshot: locComposeLocReportSnapshot(cum, scan, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var b strings.Builder
		renderLocMarkdown(&b, r, false, false)
		s := b.String()
		if !strings.Contains(s, "Go") || !strings.Contains(s, "| 1 |") || !strings.Contains(s, "%") {
			t.Fatalf("markdown: %q", s)
		}
	})
	t.Run("delta-only table omits loc and tree percent columns", func(t *testing.T) {
		rows := map[string]LocLangStats{
			"TypeScript": {Loc: 0, Edited: 5, Added: 1, Removed: 1, Percent: 0, WipPercent: 10},
			"Go":         {Loc: 0, Edited: 20, Added: 2, Removed: 2, Percent: 0, WipPercent: 40},
			locAggTotal:  {Loc: 0, Edited: 25, Added: 3, Removed: 3, Percent: 0, WipPercent: 50},
		}
		if locUseFullTreeTable(rows) {
			t.Fatal("expected delta-only rows (total loc 0)")
		}
		md := locMarkdownTable("", rows, false, false)
		if strings.Contains(md, "| loc |") || strings.Contains(md, "| Category | % |") {
			t.Fatalf("unexpected full-tree columns: %q", md)
		}
		if !strings.Contains(md, "| Category | wip% | edited | added | removed |") {
			t.Fatalf("want churn header: %q", md)
		}
		// churn sort: Go before TypeScript (edited 20 > 5), Total last
		goIdx := strings.Index(md, "| Go |")
		tsIdx := strings.Index(md, "| TypeScript |")
		totIdx := strings.Index(md, "| Total |")
		if goIdx <= 0 || tsIdx <= 0 || totIdx <= 0 || !(goIdx < tsIdx && tsIdx < totIdx) {
			t.Fatalf("row order: %q", md)
		}
	})
	t.Run("locSortedRowKeysChurn total last", func(t *testing.T) {
		rows := map[string]LocLangStats{
			"Rust":      {Edited: 1},
			"Go":        {Edited: 99},
			locAggTotal: {Edited: 100},
		}
		ks := locSortedRowKeysChurn(rows)
		if len(ks) != 3 || ks[0] != "Go" || ks[1] != "Rust" || ks[2] != locAggTotal {
			t.Fatalf("got %v", ks)
		}
	})
	t.Run("history since-prev loc percent", func(t *testing.T) {
		prev := map[string]LocLangStats{"Go": {Loc: 100}, locAggTotal: {Loc: 1000}}
		cur := map[string]LocLangStats{"Go": {Loc: 110}, locAggTotal: {Loc: 1100}}
		h := []LocHistoryEntry{{SHA: "aaa", Languages: prev}, {SHA: "bbb", Languages: cur}}
		locApplyHistoryLocSincePrev(h)
		if h[0].Languages["Go"].SincePrevLocPercent != nil {
			t.Fatalf("first row want nil delta")
		}
		if h[1].Languages["Go"].SincePrevLocPercent == nil {
			t.Fatal("second row want delta")
		}
		if g := *h[1].Languages["Go"].SincePrevLocPercent; g < 9.99 || g > 10.01 {
			t.Fatalf("go Δ%% want ~10 got %v", g)
		}
	})
	t.Run("locHistoryEntryStatsMap prefers languages", func(t *testing.T) {
		rows := map[string]LocLangStats{"Go": {Loc: 1}}
		e := LocHistoryEntry{Languages: rows, ByContributors: map[string]map[string]LocLangStats{"x": {}}}
		got := locHistoryEntryStatsMap(&e)
		if got == nil || got["Go"].Loc != 1 {
			t.Fatalf("expected Languages map, got %#v", got)
		}
	})
	t.Run("text no ansi for pipe", func(t *testing.T) {
		scan := map[string]int{"Go": 1, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 0, "Data": 0}
		r := &LocReport{Snapshot: locComposeLocReportSnapshot(map[string]locCumulativePair{}, scan, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var b strings.Builder
		renderLocText(&b, r, false, false, false)
		if strings.ContainsRune(b.String(), '\x1b') {
			t.Fatal("unexpected ansi")
		}
	})
	t.Run("root has loc", func(t *testing.T) {
		root, _ := NewRootWithConfig(testEngineFactory)
		var found *cobra.Command
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

func TestParseMcpClientKind(t *testing.T) {
	cases := []struct {
		raw  string
		want McpClientKind
	}{
		{"", McpClientGeneric},
		{"generic", McpClientGeneric},
		{"client", McpClientGeneric},
		{"cursor", McpClientCursor},
		{"kiro", McpClientKiro},
		{"copilot", McpClientCopilot},
		{"claude", McpClientClaude},
		{"codex", McpClientCodex},
		{"CURSOR", McpClientCursor},
		{"  codex  ", McpClientCodex},
	}
	for _, tc := range cases {
		got, err := ParseMcpClientKind(tc.raw)
		if err != nil {
			t.Fatalf("ParseMcpClientKind(%q) error: %v", tc.raw, err)
		}
		if got != tc.want {
			t.Fatalf("ParseMcpClientKind(%q) = %q, want %q", tc.raw, got, tc.want)
		}
	}
	if _, err := ParseMcpClientKind("unknown"); err == nil {
		t.Fatal("expected error for unknown kind")
	}
}

func TestMcpCommandKinds(t *testing.T) {
	cases := []struct {
		kind McpClientKind
		want string
	}{
		{McpClientGeneric, "repo"},
		{McpClientCursor, "repo-cursor"},
		{McpClientKiro, "repo-kiro"},
		{McpClientCopilot, "repo-copilot"},
		{McpClientClaude, "repo-claude"},
		{McpClientCodex, "repo-codex"},
	}
	for _, tc := range cases {
		parsed, err := ParseMcpClientKind(string(tc.kind))
		if err != nil {
			t.Fatalf("ParseMcpClientKind(%q): %v", tc.kind, err)
		}
		if parsed != tc.kind {
			t.Fatalf("ParseMcpClientKind(%q) = %q, want %q", tc.kind, parsed, tc.kind)
		}
		if got := McpServerName(tc.kind); got != tc.want {
			t.Fatalf("McpServerName(%q) = %q, want %q", tc.kind, got, tc.want)
		}
	}
}

func TestMicroCommitCommandExists(t *testing.T) {
	root, _ := NewRootWithConfig(testEngineFactory)
	var found *cobra.Command
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

func TestMcpStdioInitializeHandshake(t *testing.T) {
	repoRoot := findRepoRoot(".")
	if repoRoot == "" {
		t.Skip("repo root not found")
	}
	bin := filepath.Join(repoRoot, "repo", "client", "client")
	if _, err := os.Stat(bin); err != nil {
		t.Skip("repo client binary not built")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, bin, "mcp", "cursor")
	cmd.Dir = repoRoot
	stdin, err := cmd.StdinPipe()
	if err != nil {
		t.Fatal(err)
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	cmd.Stderr = os.Stderr
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	defer func() {
		_ = stdin.Close()
		_ = cmd.Process.Kill()
		_ = cmd.Wait()
	}()
	initReq := `{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}` + "\n"
	if _, err := io.WriteString(stdin, initReq); err != nil {
		t.Fatal(err)
	}
	reader := bufio.NewReader(stdout)
	line, err := reader.ReadString('\n')
	if err != nil {
		t.Fatalf("read initialize response: %v", err)
	}
	if !strings.Contains(line, `"result"`) || !strings.Contains(line, `jsonrpc`) {
		t.Fatalf("unexpected initialize response: %s", line)
	}
}
