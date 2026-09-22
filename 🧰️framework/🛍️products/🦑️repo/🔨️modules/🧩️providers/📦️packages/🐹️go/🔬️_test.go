// 🔬️ Tests of the providers domain, split out of the pre-split godfile suite.

package providers

import (
	json "encoding/json"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

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
			if got := ResolveMcpTicketClient(tc.kind, tc.client); got != tc.expected {
				t.Fatalf("resolveMcpTicketClient(%q, %q) = %q, want %q", tc.kind, tc.client, got, tc.expected)
			}
		})
	}
}

func TestVSCodeEventFromHookEvent(t *testing.T) {
	cases := []struct {
		name       string
		event      model.HookEvent
		parentInfo string
		expect     string
	}{
		{"agent.tool.starting", model.HookAgentToolStarting, "", "PreToolUse"},
		{"agent.tool.ended", model.HookAgentToolEnded, "", "PostToolUse"},
		{"agent.started", model.HookAgentStarted, "", "SessionStart"},
		{"agent.started subagent", model.HookAgentStarted, "subagent", "SubagentStart"},
		{"agent.ended", model.HookAgentEnded, "", "Stop"},
		{"agent.ended subagent", model.HookAgentEnded, "subagent", "SubagentStop"},
		{"agent.prompt.submitting", model.HookAgentPromptSubmitting, "", "UserPromptSubmit"},
		{"agent.compacting", model.HookAgentCompacting, "", "PreCompact"},
		{"agent.file.read.starting", model.HookAgentToolSearchStarting, "", "PreToolUse"},
		{"agent.tool.code.edit.starting", model.HookAgentToolCodeEditStarting, "", "PreToolUse"},
		{"agent.tool.code.edit.ended", model.HookAgentToolCodeEditEnded, "", "PostToolUse"},
		{"agent.tool.terminal.starting", model.HookAgentToolTerminalStarting, "", "PreToolUse"},
		{"agent.tool.terminal.ended", model.HookAgentToolTerminalEnded, "", "PostToolUse"},
		{"agent.tool.test.starting", model.HookAgentToolTestStarting, "", "PreToolUse"},
		{"agent.tool.test.ended", model.HookAgentToolTestEnded, "", "PostToolUse"},
		{"agent.tool.build.starting", model.HookAgentToolBuildStarting, "", "PreToolUse"},
		{"agent.tool.build.ended", model.HookAgentToolBuildEnded, "", "PostToolUse"},
		{"agent.tool.plan.updating.starting", model.HookAgentToolPlanUpdatingStarting, "", "PreToolUse"},
		{"agent.tool.plan.updating.ended", model.HookAgentToolPlanUpdatingEnded, "", "PostToolUse"},
		{"agent.thinking.starting", model.HookAgentThinkingStarting, "", ""},
		{"agent.thinking.ended", model.HookAgentThinkingEnded, "", ""},
		{"unknown", model.HookEvent("unknown.x"), "", ""},
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
		out := FormatVSCodeHookOutput("PreToolUse", model.HookResultAgentToolStarting{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: true}}})
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
		out := FormatVSCodeHookOutput("PreToolUse", model.HookResultAgentToolStarting{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: false, Message: "blocked: git checkout"}}})
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
		out := FormatVSCodeHookOutput("SessionStart", model.HookResultAgentStarted{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: true, Message: "agent.started acknowledged"}}})
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
		out := FormatVSCodeHookOutput("Stop", model.HookResultAgentEnded{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: true}}})
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
		out := FormatVSCodeHookOutput("UserPromptSubmit", model.HookResultAgentPromptSubmitting{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: true}}})
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
		out := FormatVSCodeHookOutput("PostToolUse", model.HookResultAgentToolEnded{HookResultAgentBase: model.HookResultAgentBase{HookResultBase: model.HookResultBase{Allowed: true}}})
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
		out := FormatVSCodeHookOutput("", model.HookResultBase{Allowed: true, Message: "test"})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
	})
}

func TestResolvePreToolUse(t *testing.T) {
	cases := []struct {
		kind   model.ToolKind
		expect model.HookEvent
	}{
		{model.ToolKindPlan, model.HookAgentToolPlanUpdatingStarting},
		{model.ToolKindCodeSearch, model.HookAgentToolSearchStarting},
		{model.ToolKindCodeEdit, model.HookAgentToolCodeEditStarting},
		{model.ToolKindTest, model.HookAgentToolTestStarting},
		{model.ToolKindBuild, model.HookAgentToolBuildStarting},
		{model.ToolKindTerminal, model.HookAgentToolTerminalStarting},
		{model.ToolKindGeneric, model.HookAgentToolStarting},
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
		kind   model.ToolKind
		expect model.HookEvent
	}{
		{model.ToolKindCodeSearch, model.HookAgentToolSearchEnded},
		{model.ToolKindCodeEdit, model.HookAgentToolCodeEditEnded},
		{model.ToolKindTest, model.HookAgentToolTestEnded},
		{model.ToolKindBuild, model.HookAgentToolBuildEnded},
		{model.ToolKindTerminal, model.HookAgentToolTerminalEnded},
		{model.ToolKindGeneric, model.HookAgentToolEnded},
		{model.ToolKindPlan, model.HookAgentToolPlanUpdatingEnded},
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

// 🔌️#region 🏷️Provider
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
		if got := ExtractIssueURL(tt.in); got != tt.want {
			t.Errorf("ExtractIssueURL(%q) = %q, want %q", tt.in, got, tt.want)
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

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
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

// 🆕️initTestGitRepoWithRemote creates a git repo with a bare remote and returns (workDir, remoteDir).
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
		{McpClientCursor, "repo"},
		{McpClientKiro, "repo"},
		{McpClientCopilot, "repo"},
		{McpClientClaude, "repo"},
		{McpClientCodex, "repo"},
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
