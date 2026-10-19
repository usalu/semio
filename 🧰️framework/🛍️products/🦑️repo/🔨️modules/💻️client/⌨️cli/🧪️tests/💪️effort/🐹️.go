// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Allowed LLMs, effort resolution, and ticket effort lifecycle.

// #endregion 🧲️Header

package client

import (
	"context"
	command "github.com/usalu/semio/repo/client/internal/command"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// 🧠️TestAllowedLLMsCoverage tests resolution of all latest LLMs across providers.
func TestAllowedLLMsCoverage(t *testing.T) {
	testModels := []string{
		"opus-4-7", "opus-4-6", "opus-4-5", "opus-4-1", "opus-4", "opus-3",
		"sonnet-5", "sonnet-4-6", "sonnet-4-5", "sonnet-4", "sonnet-3-7", "sonnet-3-5",
		"haiku-4-5", "haiku-4", "haiku-3-5",
		"gemini-3-7-pro", "gemini-3-7-flash", "gemini-3-1-pro", "gemini-3-pro", "gemini-3-flash",
		"gemini-2-5-pro", "gemini-2-5-flash", "gemini-2-5-flash-lite", "gemini-2-pro", "gemini-2-flash", "gemini-2-flash-lite",
		"gpt-5-5", "gpt-5-4", "gpt-5-3-codex", "gpt-5-2-codex", "gpt-5-2", "gpt-5-1-codex", "gpt-5-1", "gpt-5-codex", "gpt-5", "gpt-5-mini",
		"o3-pro", "o3-mini", "o3", "o1-pro", "o1-mini", "o1-preview", "o1",
		"grok-4-5", "grok-4", "grok-3-mini", "grok-3", "grok-2", "cursor-grok-4-5", "cursor-grok-4",
		"composer-2-5", "composer-2", "composer-1-5", "composer",
		"deepseek-r1", "deepseek-v3", "deepseek-coder",
		"llama-4", "llama-3-3", "llama-3-2", "llama-3-1", "llama-3",
		"qwen-3", "qwen-2-5", "swe-1-5", "swe-1",
	}
	for _, model := range testModels {
		slug, err := ResolveAllowedLLM(model)
		if err != nil {
			t.Errorf("ResolveAllowedLLM(%q) failed: %v", model, err)
		}
		if slug == "" {
			t.Errorf("ResolveAllowedLLM(%q) returned empty slug", model)
		}
	}
}

// 🏋️TestResolveAllowedEffort tests reasoning effort resolution and normalization.
func TestResolveAllowedEffort(t *testing.T) {
	cases := []struct {
		input    string
		expected string
		wantErr  bool
	}{
		{"low", "low", false},
		{"Low", "low", false},
		{"MEDIUM", "medium", false},
		{"high", "high", false},
		{"Max", "max", false},
		{"", "", false},
		{"invalid-effort", "", true},
		{"extreme", "", true},
	}
	for _, tc := range cases {
		got, err := ResolveAllowedEffort(tc.input)
		if (err != nil) != tc.wantErr {
			t.Errorf("ResolveAllowedEffort(%q) error = %v, wantErr %v", tc.input, err, tc.wantErr)
		}
		if got != tc.expected {
			t.Errorf("ResolveAllowedEffort(%q) = %q, want %q", tc.input, got, tc.expected)
		}
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
		cmd := &command.Command{}
		addEffortFlags(cmd)
		got, _ := extractEffortFromArgs(cmd, tc.args)
		if got != tc.expected {
			t.Errorf("extractEffortFromArgs(%v) = %q, want %q", tc.args, got, tc.expected)
		}
	}
}

// 🎫️TestTicketGetEffort tests retrieving effort from Ticket struct interactions.
func TestTicketGetEffort(t *testing.T) {
	ticket := &Ticket{
		Interactions: []Interaction{
			{Kind: "ticket.open", LLM: "opus-4-7", Effort: "high"},
		},
	}
	if got := ticket.GetEffort(); got != "high" {
		t.Errorf("ticket.GetEffort() = %q, want 'high'", got)
	}

	emptyTicket := &Ticket{}
	if got := emptyTicket.GetEffort(); got != "" {
		t.Errorf("emptyTicket.GetEffort() = %q, want ''", got)
	}
}

// 🧪️TestTicketEffortLifecycle tests full lifecycle preservation of reasoning effort on tickets.
func TestTicketEffortLifecycle(t *testing.T) {
	tmpDir := withTempRepo(t)
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}

	result := ToolTicketOpen("🎫️", "Test Effort Ticket", "Prompt with high effort", "opus-4-7", "high", "copilot-chat", "", true, "TEST-GOAL", "", false, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
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

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen with max effort", "gemini-3-7-pro", "max", "copilot-chat", "", "", "", "", true, McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen error: %s", reopenResult.Error)
	}
	reopenedTicket, ok := reopenResult.Data.(*Ticket)
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

// 🌐️TestGraphQLEffortMutationsAndQueries tests that GraphQL mutations and queries support reasoning effort.
func TestGraphQLEffortMutationsAndQueries(t *testing.T) {
	executor := newExecutorFor(t, withTempRepo(t))

	goalMutation := `
		mutation {
			goalCreate(input: {
				title: "GQL Goal",
				description: "GQL Goal Desc",
				prompt: "GQL Goal Prompt",
				dueDate: "2026-12-31",
				client: "copilot-chat",
				llm: "gemini-3-7-pro",
				effort: "high",
				noManagement: true
			}) {
				id
				title
				llm
				effort
			}
		}
	`
	result, err := executor.Execute(context.Background(), goalMutation, nil)
	if err != nil {
		t.Fatalf("goalCreate GraphQL errors: %v", err)
	}
	goalData := result.(map[string]interface{})["goalCreate"].(map[string]interface{})
	if goalData["effort"] != "high" {
		t.Errorf("goal.effort = %v, want 'high'", goalData["effort"])
	}
	if goalData["llm"] != "gemini-3-7-pro" {
		t.Errorf("goal.llm = %v, want 'gemini-3-7-pro'", goalData["llm"])
	}

	ticketMutation := `
		mutation {
			ticketOpen(input: {
				emoji: "🎫️",
				title: "GQL Ticket",
				prompt: "GQL Ticket Prompt",
				client: COPILOT_CHAT,
				llm: "opus-4-7",
				effort: "max",
				goal: "GQL-GOAL",
				noManagement: true
			}) {
				year
				month
				day
				slug
				effort
				llm
			}
		}
	`
	tResult, err := executor.Execute(context.Background(), ticketMutation, nil)
	if err != nil {
		t.Fatalf("ticketOpen GraphQL errors: %v", err)
	}
	ticketData := tResult.(map[string]interface{})["ticketOpen"].(map[string]interface{})
	if ticketData["effort"] != "max" {
		t.Errorf("ticket.effort = %v, want 'max'", ticketData["effort"])
	}
	if ticketData["llm"] != "opus-4-7" {
		t.Errorf("ticket.llm = %v, want 'opus-4-7'", ticketData["llm"])
	}
}
