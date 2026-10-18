// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Goal and ticket compose-ID JSON marshalling round trips.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"testing"
)

// 📌️#region 🔑️Compose Repo ID Conversion
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
		Emoji:    "🔧️",
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

	if raw["emoji"] != "🔧️" {
		t.Errorf("expected emoji %q, got %q", "🔧️", raw["emoji"])
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
		"emoji": "🧪️",
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

	if ticket.Emoji != "🧪️" {
		t.Errorf("expected emoji %q, got %q", "🧪️", ticket.Emoji)
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
