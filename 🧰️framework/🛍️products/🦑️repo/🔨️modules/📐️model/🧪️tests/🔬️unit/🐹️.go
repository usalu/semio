// 🔬️ Tests of the model domain, split out of the pre-split godfile suite.

package model

import (
	json "encoding/json"
	fmt "fmt"
	testing "testing"
)

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

// 📌️#region 🔑️Compose Repo ID Conversion
func TestGoalPathToComposeID(t *testing.T) {
	cases := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", "", ""},
		{"single segment", "AI-OPTIMIZED-REPO", EmojiText(EmojiGoal) + "aioptimizedrepo"},
		{"two segments", "AI-OPTIMIZED-REPO/REPO-CLI", EmojiText(EmojiGoal) + "aioptimizedrepo" + EmojiText(EmojiGoal) + "repocli"},
		{"four segments", "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI",
			EmojiText(EmojiGoal) + "aioptimizedrepo" + EmojiText(EmojiGoal) + "repoclient" + EmojiText(EmojiGoal) + "repobinary" + EmojiText(EmojiGoal) + "repocli"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := GoalPathToComposeID(tc.input)
			if got != tc.expected {
				t.Errorf("goalPathToComposeID(%q) = %q, want %q", tc.input, got, tc.expected)
			}
		})
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
		{"normal", "usalu", EmojiText(EmojiContributor) + "usalu"},
		{"already prefixed", EmojiText(EmojiContributor) + "usalu", EmojiText(EmojiContributor) + "usalu"},
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

func TestTicketMarshalJSONWritesEveryMemberVerbatim(t *testing.T) {
	goalReference := EmojiText(EmojiGoal) + "aioptimizedrepo" + EmojiText(EmojiGoal) + "repocli"
	ticket := Ticket{
		Title:    "Test Ticket",
		Emoji:    "🔧️",
		Goal:     goalReference,
		Status:   TicketStatusOpen,
		Sessions: []string{EmojiText(EmojiSession) + "s1"},
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

	if raw["goal"] != goalReference {
		t.Errorf("expected goal %q, got %q", goalReference, raw["goal"])
	}

	sessions := raw["sessions"].([]interface{})
	expectedSessionID := EmojiText(EmojiSession) + "s1"
	if sessions[0] != expectedSessionID {
		t.Errorf("expected session %q, got %q", expectedSessionID, sessions[0])
	}
}

func TestTicketUnmarshalJSONReadsEveryMemberVerbatim(t *testing.T) {
	goalEmoji := EmojiText(EmojiGoal)
	raw := fmt.Sprintf(`{
		"title": "Test",
		"emoji": "🧪️",
		"goal": "%saioptimizedrepo%srepoclient%srepobinary%srepocli",
		"status": "open",
		"sessions": [
			"%ss1"
		]
	}`, goalEmoji, goalEmoji, goalEmoji, goalEmoji, EmojiText(EmojiSession))

	var ticket Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		t.Fatalf("unmarshal failed: %v", err)
	}

	if ticket.Emoji != "🧪️" {
		t.Errorf("expected emoji %q, got %q", "🧪️", ticket.Emoji)
	}

	expectedGoal := fmt.Sprintf("%saioptimizedrepo%srepoclient%srepobinary%srepocli", goalEmoji, goalEmoji, goalEmoji, goalEmoji)
	if ticket.Goal != expectedGoal {
		t.Errorf("expected goal %q, got %q", expectedGoal, ticket.Goal)
	}

	if len(ticket.Sessions) != 1 || ticket.Sessions[0] != EmojiText(EmojiSession)+"s1" {
		t.Errorf("expected session %q, got %+v", EmojiText(EmojiSession)+"s1", ticket.Sessions)
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

	goalEmoji := EmojiText(EmojiGoal)
	expectedParent := goalEmoji + "aioptimizedrepo" + goalEmoji + "repoclient"
	if raw["parent"] != expectedParent {
		t.Errorf("expected parent %q, got %q", expectedParent, raw["parent"])
	}
}

func TestTicketGoalRoundTripThroughJSONIsIdentity(t *testing.T) {

	ticket := Ticket{
		Title:    "Roundtrip Test",
		Emoji:    "♻️",
		Goal:     EmojiText(EmojiGoal) + "aioptimizedrepo" + EmojiText(EmojiGoal) + "repocli",
		Status:   TicketStatusOpen,
		Sessions: []string{EmojiText(EmojiSession) + "mysession"},
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

	if len(ticket2.Sessions) != 1 || ticket2.Sessions[0] != EmojiText(EmojiSession)+"mysession" {
		t.Errorf("round-trip sessions failed: got %+v", ticket2.Sessions)
	}
}

func TestTicketUnmarshalRequiresExplicitValidStatus(t *testing.T) {
	for _, test := range []struct {
		name    string
		payload string
		wantErr bool
	}{
		{name: "open", payload: `{"title":"Ticket","status":"open"}`},
		{name: "closed", payload: `{"title":"Ticket","status":"closed"}`},
		{name: "missing", payload: `{"title":"Ticket"}`, wantErr: true},
		{name: "empty", payload: `{"title":"Ticket","status":""}`, wantErr: true},
		{name: "unknown", payload: `{"title":"Ticket","status":"paused"}`, wantErr: true},
	} {
		t.Run(test.name, func(t *testing.T) {
			var ticket Ticket
			err := json.Unmarshal([]byte(test.payload), &ticket)
			if (err != nil) != test.wantErr {
				t.Fatalf("json.Unmarshal() error = %v, wantErr %v", err, test.wantErr)
			}
		})
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

func TestTicketDateStartedFallsBackToTheTicketDay(t *testing.T) {
	ticket := &Ticket{Year: 26, Month: 10, Day: 18}
	started := ticket.GetDateStarted()
	if started.Year() != 2026 || int(started.Month()) != 10 || started.Day() != 18 {
		t.Fatalf("a ticket without interactions must start on its own day (two-digit year 26 is 2026), got %s", started)
	}
}
