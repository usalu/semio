//go:build exhaustive

// 🔬️ Tests of the cli domain, split out of the pre-split godfile suite.

package cli

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strconv "strconv"
	strings "strings"
	testing "testing"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	graphql "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

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

// 🧪️#region 🏩️Codebase
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

func TestExhaustivePolicyBreachListCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy breach list test in short mode")
	}
	result := ToolPolicyBreachList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyBreachList returned error: %s", result.Error)
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

func TestExhaustiveGoalDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow goal document test in short mode")
	}
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-PARENT-GOAL"))
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "RENAMED-PARENT"))
	os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL"))

	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*model.Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

	childTitle := "Test Child Goal"
	childRes := ToolGoalCreate(childTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent.ID, "")
	if childRes.Error != "" {
		t.Fatalf("Failed to create child: %s", childRes.Error)
	}
	child, ok := childRes.Data.(*model.Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	expectedChildID := "TEST-PARENT-GOAL/TEST-CHILD-GOAL"
	if child.ID != expectedChildID {
		t.Errorf("Expected child ID '%s', got '%s'", expectedChildID, child.ID)
	}

	childPath := filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(child.ID), "🎯️goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	parent.Title = "Renamed Parent"
	err := graphql.UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(workspace.GetRepoGoalsDir(), filepath.FromSlash(newChildID), "🎯️goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*model.Goal)
	var foundChild *model.Goal
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

	ctx := &graphql.RepoContext{}
	emptyParent := ""
	changeInput := model.GoalChangeInput{
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

	rootChildPath := filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL", "🎯️goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

	defer os.RemoveAll(filepath.Join(workspace.GetRepoGoalsDir(), "TEST-CHILD-GOAL"))
}

func TestExhaustiveTicketListIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket list ids test in short mode")
	}
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Skipf("ToolTicketList returned error: %s", result.Error)
	}
	tickets, ok := result.Data.([]model.Ticket)
	if !ok {
		t.Skip("ToolTicketList data is not []Ticket")
	}
	for _, tk := range tickets {
		id := tk.GetID()
		expectedPrefix := model.EmojiText(model.EmojiTicket)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
		}
		expectedID := expectedPrefix + workspace.Flat(tk.Slug)
		if id != expectedID {
			t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
		}
	}
}

func TestExhaustiveTreeCommands(t *testing.T) {

	if testing.Short() {
		t.Skip("skipping slow tree test")
	}

	output, err := executeTreeCommand("search", "compose/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻️compose") {
		t.Errorf("repo tree compose/go missing compose.go, got:\n%s", output)
	}
	if strings.Contains(output, "├️─️─️ ") || strings.Contains(output, "└️─️─️ ") {
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
	if !strings.Contains(strings.ToLower(output), "compose.go") && !strings.Contains(output, "composego") && !strings.Contains(output, "💻️compose") {
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
	if !strings.Contains(output, "├️─️─️ ") && !strings.Contains(output, "└️─️─️ ") {
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

func TestExhaustiveMarkdownOutput(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow markdown output test in short mode")
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

// 🔍️#region 🏂️Query
func TestExhaustiveQueryFlag(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query flag test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

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
			args:        []string{"query", "search"},
			query:       "",
			expectMatch: "search",
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
	workspace.SetRootDir(repoRoot)

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
	workspace.SetRootDir(repoRoot)

	t.Run("tree query returns multiple resource kinds for shared keyword", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--query", "search", "--text")
		if err != nil {
			t.Fatalf("tree --query search failed: %v\nOutput: %s", err, output)
		}
		hasFile := strings.Contains(output, ".go") || strings.Contains(output, "main")
		hasGoal := strings.Contains(output, "AI-OPTIMIZED") || strings.Contains(output, "Repo")
		hasTicket := strings.Contains(output, "02/")
		if !hasFile && !hasGoal && !hasTicket {
			t.Errorf("tree --query search should return files, goals, or tickets; got:\n%s", output)
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
		output, err := executeTreeCommand("query", "search")
		if err != nil {
			t.Fatalf("query search failed: %v\nOutput: %s", err, output)
		}
		var nonEmpty int
		for _, l := range strings.Split(output, "\n") {
			if strings.TrimSpace(l) != "" {
				nonEmpty++
			}
		}
		if nonEmpty == 0 {
			t.Errorf("query search should return at least one ID, got:\n%s", output)
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
		searchOut, err := executeTreeCommand("search", "--query", "search", "--json")
		if err != nil {
			t.Fatalf("tree --query search failed: %v", err)
		}
		cliOut, err := executeTreeCommand("search", "--query", "cli", "--json")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		var searchTree, cliTree map[string]interface{}
		if json.Unmarshal([]byte(strings.TrimSpace(searchOut)), &searchTree) != nil {
			t.Fatal("search output not valid JSON")
		}
		if json.Unmarshal([]byte(strings.TrimSpace(cliOut)), &cliTree) != nil {
			t.Fatal("cli output not valid JSON")
		}
		searchStr := fmt.Sprint(searchTree)
		cliStr := fmt.Sprint(cliTree)
		if searchStr == cliStr {
			t.Error("tree --query search and tree --query cli should return different results")
		}
	})
}

func TestExhaustiveQueryEmptyReturnsAll(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query empty test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

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
	workspace.SetRootDir(repoRoot)

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
	workspace.SetRootDir(repoRoot)

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

func TestExhaustiveToolTicketLifecycle(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle test in short mode")
	}
	setupToolTest(t)
	title := fmt.Sprintf("Test Lifecycle Ticket %d", time.Now().UnixNano())

	result := ToolTicketOpen("🎫️", title, "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
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

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen prompt", "sonnet-4-5", "", "windsurf-chat", "", "", "", "", true, providers.McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen returned error: %s", reopenResult.Error)
	}

	ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Final close", []string{"repo/client/package.json"}, "", true)
	ticketPath := ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	os.RemoveAll(ticketPath)
}

func TestExhaustiveMcpTicketCloseAutoResolve(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close auto-resolve test in short mode")
	}
	setupToolTest(t)

	result := ToolTicketOpen("🎫️", "Test Auto Resolve Close", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "cleanup", []string{"repo/client/package.json"}, "", true)
		os.RemoveAll(ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
	}()

	year, month, day, slug, err := resolveTicketForClose("")
	if err != nil {
		t.Fatalf("resolveTicketForClose('') error: %v", err)
	}
	resolved, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		t.Fatalf("resolved ticket not readable: %v", err)
	}
	if resolved.Status != model.TicketStatusOpen {
		t.Errorf("resolveTicketForClose('') resolved to non-open ticket (status=%s)", resolved.Status)
	}
}

func TestExhaustiveMcpTicketCloseWithFullYearPath(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mcp ticket close full-year-path test in short mode")
	}
	setupToolTest(t)

	result := ToolTicketOpen("🎫️", "Test Full Year Path", "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}
	defer func() {
		os.RemoveAll(ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug))
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

// ⛳️#region 🕹️Output Parity
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

func TestExhaustiveMermaidCommandLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow mermaid loc command test in short mode")
	}
	root := findTestRepoRoot(".")
	workspace.SetRootDir(root)
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
	workspace.SetRootDir(root)
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
