// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Mermaid LOC diagrams, provider registry, management and editor providers.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"strings"
	"testing"
)

// 🧜️#region ⏲️Mermaid
func TestExhaustiveMermaidLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("walks all bundles for LOC treemap; too slow for -short runs on large monorepos")
	}
	withExhaustiveRepo(t)
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
	withExhaustiveRepo(t)
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
	withExhaustiveRepo(t)
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
	withExhaustiveRepo(t)
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
	withExhaustiveRepo(t)
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

func TestGoalIDForFilesystem(t *testing.T) {
	withGoalPath(t, "AI-OPTIMIZED-REPO/REPO-CLIENT")

	tests := []struct {
		in   string
		want string
	}{
		{"AI-OPTIMIZED-REPO/REPO-CLIENT", "AI-OPTIMIZED-REPO/REPO-CLIENT"},
		{"🎯️aioptimizedrepo🎯️repoclient", "AI-OPTIMIZED-REPO/REPO-CLIENT"},
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
