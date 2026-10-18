// 🔢️ Tests of the artifact identifiers of the work, policy, contributor and session trees.

package model

import (
	testing "testing"
)

func TestGetArtifactID_Goal(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, EmojiText(EmojiGoal) + "r26021"},
		{"nested goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": EmojiText(EmojiGoal) + "r26021"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"},
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
		{"root tickets", "", EmojiText(EmojiTickets)},
		{"goal tickets", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTickets)},
		{"section tickets", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiTickets)},
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
	goalId := EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"ticket with parentId", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": goalId}, goalId + EmojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"ticket with goalId fallback", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "goalId": "R26-02-1/RUNNING-SKETCHPAD"}, goalId + EmojiText(EmojiTicket) + "introducekeyguidurimechanism"},
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
	parentId := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"
	id := GetArtifactID("drafts", map[string]interface{}{"parentId": parentId})
	expected := parentId + EmojiText(EmojiDrafts)
	if id != expected {
		t.Errorf("drafts id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Draft(t *testing.T) {
	parentId := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"
	id := GetArtifactID("draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": parentId})
	expected := parentId + EmojiText(EmojiDraft) + "newarchitecture"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todos(t *testing.T) {
	parentId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todos", map[string]interface{}{"parentId": parentId})
	expected := parentId + EmojiText(EmojiTodos)
	if id != expected {
		t.Errorf("todos id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todo(t *testing.T) {
	parentId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": parentId})
	expected := parentId + EmojiText(EmojiTodo) + "introducepropersyncmechanism"
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
		{"root policies", "", EmojiText(EmojiPolicies)},
		{"file kind policies", EmojiText(EmojiFileCode), EmojiText(EmojiFileCode) + EmojiText(EmojiPolicies)},
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
		{"general policy on file kind", map[string]interface{}{"id": "godfiles", "parentId": EmojiText(EmojiFileCode)}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", map[string]interface{}{"id": "only-one-store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiPolicy) + "onlyonestore"},
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
	expected := EmojiText(EmojiContributors)
	if id != expected {
		t.Errorf("contributors id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Contributor(t *testing.T) {
	id := GetArtifactID("contributor", map[string]interface{}{"github": "usalu"})
	expected := EmojiText(EmojiContributor) + "usalu"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Checkpoints(t *testing.T) {
	id := GetArtifactID("checkpoints", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiCheckpoints)
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
		{"with contributorId", map[string]interface{}{"sha": sha, "contributorId": EmojiText(EmojiContributor) + "usalu"}, EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiCheckpoint) + sha},
		{"with authorId fallback", map[string]interface{}{"sha": sha, "authorId": "usalu"}, EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiCheckpoint) + sha},
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
	secondId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12"
	contributorId := EmojiText(EmojiContributor) + "usalu"
	entityID := EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"started", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "started"}, secondId + contributorId + entityID + EmojiText(EmojiInteractionStarted)},
		{"finished", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "finished"}, secondId + contributorId + entityID + EmojiText(EmojiInteractionFinished)},
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
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root sessions", "", EmojiText(EmojiSessions)},
		{"day sessions", dayId, dayId + EmojiText(EmojiSessions)},
		{"checkpoint sessions", checkpointId, checkpointId + EmojiText(EmojiSessions)},
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
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	sessionsId := dayId + EmojiText(EmojiSessions)
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	checkpointSessionsId := checkpointId + EmojiText(EmojiSessions)
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"session with uuid under day", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": sessionsId}, sessionsId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with id fallback under day", map[string]interface{}{"id": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": sessionsId}, sessionsId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session no parent", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"}, EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with uuid under checkpoint", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": checkpointId}, checkpointId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with uuid under checkpoint sessions", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": checkpointSessionsId}, checkpointSessionsId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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
	expected := "repo://sessions/" + EmojiText(EmojiSessions)
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
		{"session with uuid", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"}, "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"session with id fallback", map[string]interface{}{"id": "abc123"}, "repo://session/" + EmojiText(EmojiSession) + "abc123"},
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
		{"session", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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
		{"session", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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

func TestNormalizeTicketSessionID(t *testing.T) {
	checkpointId := GetArtifactID("checkpoint", map[string]interface{}{"sha": "abc123sha"})
	cases := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", "", ""},
		{"raw uuid", "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"already normalized", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"running prefix stripped", EmojiText(EmojiSessionRunning) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"completed prefix stripped", EmojiText(EmojiSessionCompleted) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"checkpoint prefixed", checkpointId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", checkpointId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"checkpoint prefixed raw uuid", checkpointId + EmojiText(EmojiSession) + "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", checkpointId + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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
	policyId := EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"
	affected := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore"
	lineId := EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875"
	secondId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12"
	id := GetArtifactID("breach", map[string]interface{}{"parentId": policyId, "affected": affected, "lineId": lineId, "secondId": secondId})
	expected := policyId + EmojiText(EmojiBreach) + affected + EmojiText(EmojiBreachScope) + lineId + secondId
	if id != expected {
		t.Errorf("breach id: expected %q, got %q", expected, id)
	}
}

func TestGoalArtifactID(t *testing.T) {
	cases := []struct {
		rawID    string
		expected string
	}{
		{"R26-02-1", EmojiText(EmojiGoal) + "r26021"},
		{"R26-02-1/RUNNING-SKETCHPAD", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"AI-OPTIMIZED-REPO", EmojiText(EmojiGoal) + "aioptimizedrepo"},
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
