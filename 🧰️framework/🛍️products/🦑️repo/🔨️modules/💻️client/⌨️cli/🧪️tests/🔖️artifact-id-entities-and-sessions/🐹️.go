// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact IDs and URIs for sections, goals, tickets, policies, and sessions.

// #endregion 🧲️Header

package client

import (
	"testing"
)

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
