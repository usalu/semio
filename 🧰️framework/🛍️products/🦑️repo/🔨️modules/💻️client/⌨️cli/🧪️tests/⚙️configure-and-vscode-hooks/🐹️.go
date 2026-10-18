// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Configure command, micro commit hooks, and VS Code hook output.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

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

func TestMicroCommitPostCommitHookResetsTemplates(t *testing.T) {
	repoRoot := t.TempDir()
	sourceRepoRoot := findTestRepoRoot(".")
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
	hookSourcesDir := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks")
	if err := os.MkdirAll(hookSourcesDir, 0o755); err != nil {
		t.Fatalf("mkdir repo hooks: %v", err)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		hookSrc := filepath.Join(sourceRepoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks", hookName)
		data, err := os.ReadFile(hookSrc)
		if err != nil {
			t.Fatalf("read hook source %s: %v", hookName, err)
		}
		if err := os.WriteFile(filepath.Join(hookSourcesDir, hookName), data, 0o755); err != nil {
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
