// 🔬️ Tests of the model domain, split out of the pre-split godfile suite.

package model

import (
	json "encoding/json"
	fmt "fmt"
	strings "strings"
	testing "testing"

	workspace "github.com/usalu/semio/repo/workspace"
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
		{"test ts", "🧪️index.test.ts", FileKindLab},
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
		{"dockerfile config", "Dockerfile", FileKindConfig},
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
		{"lab", "lab", EmojiText(EmojiFileLab)},
		{"script", "script", "\U0001F4DC"},
		{"docs", "docs", "\U0001F4C3"},
		{"config", "config", "\u2699\uFE0F"},
		{"resource", "resource", "\U0001F4BE"},
		{"template", "template", EmojiText(EmojiFileTemplate)},
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

// 🧪️#region 🕸️Test Command
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
	expected := EmojiText(EmojiDefinitionTest)
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

	labFileID := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiFileLab) + "maintest"

	id := BuildDefinitionID(labFileID, nil, "TestSomething", DefinitionKindImplementation)
	testEmoji := EmojiText(EmojiDefinitionTest)
	implEmoji := EmojiText(EmojiDefinitionImpl)

	if !strings.Contains(id, testEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should contain test emoji %q, got %q", testEmoji, id)
	}
	if strings.Contains(id, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should NOT contain impl emoji %q, got %q", implEmoji, id)
	}

	id2 := BuildDefinitionID(labFileID, nil, "helperFunc", DefinitionKindImplementation)
	if !strings.Contains(id2, implEmoji) {
		t.Errorf("buildDefinitionID for helperFunc in lab file should contain impl emoji %q, got %q", implEmoji, id2)
	}

	codeFileID := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiFileCode) + "main"
	id3 := BuildDefinitionID(codeFileID, nil, "TestSomething", DefinitionKindImplementation)
	if !strings.Contains(id3, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in code file should contain impl emoji %q, got %q", implEmoji, id3)
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

// 🏺️#region 🪨️Entity ID
func TestGetArtifactID_Root(t *testing.T) {
	id := GetArtifactID("root", map[string]interface{}{})
	if id != "" {
		t.Errorf("root id: expected empty, got %q", id)
	}
}

func TestGetArtifactID_Years(t *testing.T) {
	id := GetArtifactID("years", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiYear)
	if id != expected {
		t.Errorf("years id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Year(t *testing.T) {
	id := GetArtifactID("year", map[string]interface{}{"parentId": "", "yy": "26"})
	expected := EmojiText(EmojiYear) + "26"
	if id != expected {
		t.Errorf("year id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Months(t *testing.T) {
	yearId := EmojiText(EmojiYear) + "26"
	id := GetArtifactID("months", map[string]interface{}{"parentId": yearId})
	expected := yearId + EmojiText(EmojiMonth)
	if id != expected {
		t.Errorf("months id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Month(t *testing.T) {
	yearId := EmojiText(EmojiYear) + "26"
	id := GetArtifactID("month", map[string]interface{}{"parentId": yearId, "mm": "02"})
	expected := yearId + EmojiText(EmojiMonth) + "02"
	if id != expected {
		t.Errorf("month id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Days(t *testing.T) {
	monthId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02"
	id := GetArtifactID("days", map[string]interface{}{"parentId": monthId})
	expected := monthId + EmojiText(EmojiDay)
	if id != expected {
		t.Errorf("days id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Day(t *testing.T) {
	monthId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02"
	id := GetArtifactID("day", map[string]interface{}{"parentId": monthId, "dd": "15"})
	expected := monthId + EmojiText(EmojiDay) + "15"
	if id != expected {
		t.Errorf("day id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hours(t *testing.T) {
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	id := GetArtifactID("hours", map[string]interface{}{"parentId": dayId})
	expected := dayId + EmojiText(EmojiHour)
	if id != expected {
		t.Errorf("hours id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hour(t *testing.T) {
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	id := GetArtifactID("hour", map[string]interface{}{"parentId": dayId, "hh": "14"})
	expected := dayId + EmojiText(EmojiHour) + "14"
	if id != expected {
		t.Errorf("hour id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minutes(t *testing.T) {
	hourId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14"
	id := GetArtifactID("minutes", map[string]interface{}{"parentId": hourId})
	expected := hourId + EmojiText(EmojiMinute)
	if id != expected {
		t.Errorf("minutes id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minute(t *testing.T) {
	hourId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14"
	id := GetArtifactID("minute", map[string]interface{}{"parentId": hourId, "mm": "33"})
	expected := hourId + EmojiText(EmojiMinute) + "33"
	if id != expected {
		t.Errorf("minute id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Seconds(t *testing.T) {
	minuteId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14" + EmojiText(EmojiMinute) + "33"
	id := GetArtifactID("seconds", map[string]interface{}{"parentId": minuteId})
	expected := minuteId + EmojiText(EmojiSecond)
	if id != expected {
		t.Errorf("seconds id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Second(t *testing.T) {
	minuteId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14" + EmojiText(EmojiMinute) + "33"
	id := GetArtifactID("second", map[string]interface{}{"parentId": minuteId, "ss": "38"})
	expected := minuteId + EmojiText(EmojiSecond) + "38"
	if id != expected {
		t.Errorf("second id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Codebase(t *testing.T) {
	id := GetArtifactID("codebase", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiCodebase)
	if id != expected {
		t.Errorf("codebase id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technologies(t *testing.T) {
	id := GetArtifactID("technologies", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiTechnologies)
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
		{"user technology", map[string]interface{}{"name": "compose", "kind": "user"}, EmojiText(EmojiTechnologyUser) + "compose"},
		{"infra technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, EmojiText(EmojiTechnologyInfra) + "repo"},
		{"research technology", map[string]interface{}{"name": "coda", "kind": "research"}, EmojiText(EmojiTechnologyResearch) + "coda"},
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
	technologyId := EmojiText(EmojiTechnologyUser) + "compose"
	id := GetArtifactID("bundles", map[string]interface{}{"parentId": technologyId})
	expected := technologyId + EmojiText(EmojiBundles)
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
		{"library bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle", map[string]interface{}{"name": "repo/graphql", "kind": "schema"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"},
		{"ui bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleUI) + "vscode"},
		{"example bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleExample) + "example"},
		{"site bundle", map[string]interface{}{"name": "compose/desktop", "kind": "site"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSite) + "desktop"},
		{"assets bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, EmojiText(EmojiTechnologyUser) + "asset" + EmojiText(EmojiBundleAssets) + "asset"},
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
		{"root folders", "", EmojiText(EmojiFolders)},
		{"bundle folders", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolders)},
		{"required folder folders", EmojiText(EmojiFolderRequired) + "github", EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFolders)},
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
	bundleId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"org folder under bundle", map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization", "parentId": bundleId}, bundleId + EmojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder at root", map[string]interface{}{"path": ".devcontainer", "name": ".devcontainer", "kind": "required", "parentId": ""}, EmojiText(EmojiFolderRequired) + "devcontainer"},
		{"nested folder", map[string]interface{}{"path": "compose/js/sketchpad/pages", "name": "pages", "kind": "organization", "parentId": bundleId + EmojiText(EmojiFolderOrg) + "sketchpad"}, bundleId + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages"},
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
		{"root files", "", EmojiText(EmojiFiles)},
		{"folder files", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFiles)},
		{"required folder files", EmojiText(EmojiFolderRequired) + "github", EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFiles)},
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
	folderId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"code file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code", "parentId": folderId}, folderId + EmojiText(EmojiFileCode) + "design"},
		{"test file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "name": "sketchpad.test.ts", "kind": "lab", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileLab) + "sketchpadtest"},
		{"config file at root", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "name": "devcontainer.json", "kind": "config", "parentId": EmojiText(EmojiFolderRequired) + "devcontainer"}, EmojiText(EmojiFolderRequired) + "devcontainer" + EmojiText(EmojiFileConfig) + "devcontainer"},
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
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("line", map[string]interface{}{"parentId": fileId, "line": float64(3872)})
	expected := fileId + EmojiText(EmojiLine) + "3872"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Range(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("range", map[string]interface{}{"parentId": fileId, "startLine": float64(3872), "endLine": float64(3875)})
	expected := fileId + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Sections(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("sections", map[string]interface{}{"parentId": fileId})
	expected := fileId + EmojiText(EmojiSections)
	if id != expected {
		t.Errorf("sections id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Section(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level section", map[string]interface{}{"name": "State Managment", "parentId": fileId}, fileId + EmojiText(EmojiSection) + "statemanagment"},
		{"nested section", map[string]interface{}{"name": "Store", "parentId": fileId + EmojiText(EmojiSection) + "statemanagment"}, fileId + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"},
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
	sectionId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"
	id := GetArtifactID("definitions", map[string]interface{}{"parentId": sectionId})
	expected := sectionId + EmojiText(EmojiDefinitions)
	if id != expected {
		t.Errorf("definitions id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Definition(t *testing.T) {
	sectionId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"
	id := GetArtifactID("definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": sectionId})
	expected := sectionId + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"
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
		{"root goals", "", EmojiText(EmojiGoals)},
		{"nested goals", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiGoals)},
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
		{"bundles", "bundles", map[string]interface{}{"parentId": "👤compose"}, "👤compose📦"},
		{"bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, "👤compose📚js"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, "📁"},
		{"bundle folders", "folders", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad📁"},
		{"required folder folders", "folders", map[string]interface{}{"parentId": "🛅github"}, "🛅github📁"},
		{"folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": "👤compose📚js"}, "👤compose📚js🗃️sketchpad"},
		{"folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, "🛅devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, "📄"},
		{"folder files", "files", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad📄"},
		{"required folder files", "files", map[string]interface{}{"parentId": "🛅github"}, "🛅github📄"},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad💻design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": "🛅devcontainer"}, "🛅devcontainer⚙️devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design", "line": float64(3872)}, "👤compose📚js🗃️sketchpad💻design📌3872"},
		{"sections in file", "sections", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design"}, "👤compose📚js🗃️sketchpad💻design🔖"},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": "👤compose📚js🗃️sketchpad💻design"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment"},
		{"section Store nested", "section", map[string]interface{}{"name": "Store", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🏷️"},
		{"definition createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, "🎯"},
		{"nested goals", "goals", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎯"},
		{"goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": "🎯r26021"}, "🎯r26021🎯runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, "🎫"},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫"},
		{"section tickets", "tickets", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🎫"},
		{"ticket", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism"},
		{"draft", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": "🧰repo⌨️client"}, "🧰repo⌨️client📝newarchitecture"},
		{"todo", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore📝introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": EmojiText(EmojiFileCode)}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", "policy", map[string]interface{}{"id": "only-one-store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": "💻👮️godfiles",
			"affected": "👤compose📚js🗃️sketchpad💻designstore",
			"lineId":   "📌3872📌3875",
			"secondId": "🎆26🌙02☀️14⏰19⌚07⏱️12",
		}, "💻👮️godfiles🚫👤compose📚js🗃️sketchpad💻designstore🔍📌3872📌3875🎆26🌙02☀️14⏰19⌚07⏱️12"},
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
		{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay)},
		{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15"},
		{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F0"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014"},
		{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute)},
		{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33"},
		{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33" + EmojiText(EmojiSecond)},
		{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33", "ss": "38"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33" + EmojiText(EmojiSecond) + "38"},
		{"technologies under root", "technologies", map[string]interface{}{"parentId": ""}, EmojiText(EmojiTechnologies)},
		{"infra technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, EmojiText(EmojiTechnologyInfra) + "repo"},
		{"user technology compose", "technology", map[string]interface{}{"name": "compose", "kind": "user"}, EmojiText(EmojiTechnologyUser) + "compose"},
		{"research technology coda", "technology", map[string]interface{}{"name": "coda", "kind": "research"}, EmojiText(EmojiTechnologyResearch) + "coda"},
		{"bundles under technology", "bundles", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles)},
		{"library bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle compose/graphql", "bundle", map[string]interface{}{"name": "compose/graphql", "kind": "schema"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle repo/client", "bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"},
		{"ui bundle repo/vscode", "bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleUI) + "vscode"},
		{"example bundle coda/example", "bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleExample) + "example"},
		{"assets bundle asset", "bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, EmojiText(EmojiTechnologyUser) + "asset" + EmojiText(EmojiBundleAssets) + "asset"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, EmojiText(EmojiFolders)},
		{"bundle sketchpad folders", "folders", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolders)},
		{"required folder .github folders", "folders", map[string]interface{}{"parentId": EmojiText(EmojiFolderRequired) + "github"}, EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFolders)},
		{"org folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, EmojiText(EmojiFolderRequired) + "devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, EmojiText(EmojiFiles)},
		{"sketchpad files", "files", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFiles)},
		{"github files", "files", map[string]interface{}{"parentId": EmojiText(EmojiFolderRequired) + "github"}, EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFiles)},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": EmojiText(EmojiFolderRequired) + "devcontainer"}, EmojiText(EmojiFolderRequired) + "devcontainer" + EmojiText(EmojiFileConfig) + "devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design", "line": float64(3872)}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiLine) + "3872"},
		{"range 3872-3875", "range", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore", "startLine": float64(3872), "endLine": float64(3875)}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore" + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875"},
		{"sections in file", "sections", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSections)},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment"},
		{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitions)},
		{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
		{"definition interface", "definition", map[string]interface{}{"name": "IStore", "kind": "interface", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionInterface) + "istore"},
		{"definition constant", "definition", map[string]interface{}{"name": "MAX_SIZE", "kind": "constant", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionConstant) + "maxsize"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, EmojiText(EmojiGoals)},
		{"nested goals under parent", "goals", map[string]interface{}{"parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiGoals)},
		{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, EmojiText(EmojiGoal) + "r26021"},
		{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": EmojiText(EmojiGoal) + "r26021"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, EmojiText(EmojiTickets)},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTickets)},
		{"section tickets", "tickets", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiTickets)},
		{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiDraft) + "newarchitecture"},
		{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore" + EmojiText(EmojiTodo) + "introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": EmojiText(EmojiFileCode)}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"},
		{"specific policy Only One Store", "policy", map[string]interface{}{"id": "only-one-store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles",
			"affected": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore",
			"lineId":   EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875",
			"secondId": EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
		}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles" + EmojiText(EmojiBreach) + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore" + EmojiText(EmojiBreachScope) + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875" + EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12"},
		{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": EmojiText(EmojiContributor) + "usalu"}, EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiCheckpoint) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "started",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionStarted)},
		{"interaction edited", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "edited",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionEdited)},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "finished",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionFinished)},
		{"interaction restarted", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "restarted",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionRestarted)},
		{"interaction deleted", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "deleted",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionDeleted)},
		{"file test kind", "file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "kind": "lab", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileLab) + "sketchpadtest"},
		{"file script kind", "file", map[string]interface{}{"path": "compose/engine/build.ts", "kind": "script", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "engine"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "engine" + EmojiText(EmojiFileScript) + "build"},
		{"file docs kind", "file", map[string]interface{}{"path": "compose/js/README.md", "kind": "docs", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileDocs) + "readme"},
		{"file asset kind", "file", map[string]interface{}{"path": "compose/js/sketchpad/page/showcase/metabolism.mdx", "kind": "resource", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages" + EmojiText(EmojiFolderOrg) + "showcases"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages" + EmojiText(EmojiFolderOrg) + "showcases" + EmojiText(EmojiFileResource) + "metabolism"},
		{"file license kind", "file", map[string]interface{}{"path": "compose/go/LICENSE.md", "kind": "license", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "go"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "go" + EmojiText(EmojiFileLicense) + "license"},
		{"site bundle compose/docs", "bundle", map[string]interface{}{"name": "compose/docs", "kind": "site"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSite) + "docs"},
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
			props := CollectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "\n") || strings.Contains(p, "\r") {
					t.Errorf("property contains newline: %q", p)
				}
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
			output := RenderEntityMarkdownLink(tt.kind, tt.data)
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
			got := InferEntityKind(tt.key)
			if got != tt.expected {
				t.Errorf("inferEntityKind(%q) = %q, want %q", tt.key, got, tt.expected)
			}
		})
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
			wantID:  EmojiText(EmojiTechnologies),
			wantURI: "repo://technologies/" + EmojiText(EmojiTechnologies),
		},
		{
			name:    "technology user",
			kind:    "technology",
			data:    map[string]interface{}{"name": "compose", "kind": "user"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose",
		},
		{
			name:    "technology infrastructure",
			kind:    "technology",
			data:    map[string]interface{}{"name": "repo", "kind": "infrastructure"},
			wantID:  EmojiText(EmojiTechnologyInfra) + "repo",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo",
		},
		{
			name:    "technology research",
			kind:    "technology",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  EmojiText(EmojiTechnologyResearch) + "coda",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyResearch) + "coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"technologyCode": "compose", "parentId": EmojiText(EmojiTechnologyUser) + "compose"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles),
			wantURI: "repo://bundles/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles),
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/js", "kind": "library"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/example", "kind": "library"},
			wantID:  EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleLibrary) + "example",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleLibrary) + "example",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/desktop", "kind": "ui"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleUI) + "desktop",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleUI) + "desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiFolders),
			wantURI: "repo://folders/" + EmojiText(EmojiFolders),
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "compose/js/src", "parentId": EmojiText(EmojiFolderOrg) + "src"},
			wantID:  EmojiText(EmojiFolderOrg) + "src" + EmojiText(EmojiFolders),
			wantURI: "repo://folders/" + EmojiText(EmojiFolderOrg) + "src" + EmojiText(EmojiFolders),
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/src", "kind": "required"},
			wantID:  EmojiText(EmojiFolderRequired) + "src",
			wantURI: "repo://folder/" + EmojiText(EmojiFolderRequired) + "src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/utils", "kind": "organization"},
			wantID:  EmojiText(EmojiFolderOrg) + "utils",
			wantURI: "repo://folder/" + EmojiText(EmojiFolderOrg) + "utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiFiles),
			wantURI: "repo://files/" + EmojiText(EmojiFiles),
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  EmojiText(EmojiFileDocs) + "test",
			wantURI: "repo://file/" + EmojiText(EmojiFileDocs) + "test",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  EmojiText(EmojiFileCode) + "main",
			wantURI: "repo://file/" + EmojiText(EmojiFileCode) + "main",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "compose/js/src/🧪️index.test.ts", "kind": "lab"},
			wantID:  EmojiText(EmojiFileLab) + workspace.Flat("🧪️index.test"),
			wantURI: "repo://file/" + EmojiText(EmojiFileLab) + workspace.Flat("🧪️index.test"),
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  EmojiText(EmojiFileConfig) + "tsconfig",
			wantURI: "repo://file/" + EmojiText(EmojiFileConfig) + "tsconfig",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  EmojiText(EmojiFileScript) + "build",
			wantURI: "repo://file/" + EmojiText(EmojiFileScript) + "build",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "🖼️logo.png", "kind": "resource"},
			wantID:  EmojiText(EmojiFileResource) + workspace.Flat("🖼️logo"),
			wantURI: "repo://file/" + EmojiText(EmojiFileResource) + workspace.Flat("🖼️logo"),
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  EmojiText(EmojiFileLicense) + "license",
			wantURI: "repo://file/" + EmojiText(EmojiFileLicense) + "license",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": EmojiText(EmojiFileCode) + "index"},
			wantID:  EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSections),
			wantURI: "repo://sections/" + EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSections),
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/Design.tsx#State Management#Design Store"},
			wantID:  BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
			wantURI: "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/file.ts#Imports"},
			wantID:  BuildSectionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
			wantURI: "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": EmojiText(EmojiSection) + "types"},
			wantID:  EmojiText(EmojiSection) + "types" + EmojiText(EmojiDefinitions),
			wantURI: "repo://definitions/" + EmojiText(EmojiSection) + "types" + EmojiText(EmojiDefinitions),
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "compose/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "compose/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
		},
		{
			name:    "definition go type treated as interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "type", "filePath": "repo/client/main.go", "sectionPath": "GraphQL Types#GraphQL Input Types", "name": "TicketCloseInput"},
			wantID:  BuildDefinitionID(BuildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "compose/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiTickets),
			wantURI: "repo://tickets/" + EmojiText(EmojiTickets),
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
			wantID:  EmojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + EmojiText(EmojiTicket) + "testticket",
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
			wantID:  EmojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + EmojiText(EmojiTicket) + "testticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiGoals),
			wantURI: "repo://goals/" + EmojiText(EmojiGoals),
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "RUNNING-SKETCHPAD", "parentId": ""},
			wantID:  EmojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + EmojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "goal nested",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD", "parentId": EmojiText(EmojiGoal) + "r2602"},
			wantID:  EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiDrafts),
			wantURI: "repo://drafts/" + EmojiText(EmojiDrafts),
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  EmojiText(EmojiDraft) + "mydraft",
			wantURI: "repo://draft/" + EmojiText(EmojiDraft) + "mydraft",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiTodos),
			wantURI: "repo://todos/" + EmojiText(EmojiTodos),
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  EmojiText(EmojiTodo) + "mytodo",
			wantURI: "repo://todo/" + EmojiText(EmojiTodo) + "mytodo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiPolicies),
			wantURI: "repo://policies/" + EmojiText(EmojiPolicies),
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  EmojiText(EmojiPolicy) + "codehygiene",
			wantURI: "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene",
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
			wantID:  EmojiText(EmojiContributors),
			wantURI: "repo://contributors/" + EmojiText(EmojiContributors),
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  EmojiText(EmojiContributor) + "usalu",
			wantURI: "repo://contributor/" + EmojiText(EmojiContributor) + "usalu",
		},
		{
			name:    "checkpoints collection",
			kind:    "checkpoints",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiCheckpoints),
			wantURI: "repo://checkpoints/" + EmojiText(EmojiCheckpoints),
		},
		{
			name:    "checkpoint",
			kind:    "checkpoint",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  EmojiText(EmojiCheckpoint) + "abc123",
			wantURI: "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123",
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
			data:    map[string]interface{}{"kind": "started", "entityId": EmojiText(EmojiTicket) + "introduceinteractionmechanism"},
			wantID:  EmojiText(EmojiTicket) + "introduceinteractionmechanism" + EmojiText(EmojiInteractionStarted),
			wantURI: "repo://interaction/" + EmojiText(EmojiTicket) + "introduceinteractionmechanism" + EmojiText(EmojiInteractionStarted),
		},
		{
			name:    "interaction finished goal",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "finished", "entityId": EmojiText(EmojiGoal) + "r2602"},
			wantID:  EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished),
			wantURI: "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished),
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
		{"technology user", EmojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", EmojiText(EmojiTechnologyInfra) + "repo", "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"folder required", EmojiText(EmojiFolderRequired) + "src", "repo://folder/" + EmojiText(EmojiFolderRequired) + "src"},
		{"folder org", EmojiText(EmojiFolderOrg) + "utils", "repo://folder/" + EmojiText(EmojiFolderOrg) + "utils"},
		{"file docs", EmojiText(EmojiFileDocs) + "test", "repo://file/" + EmojiText(EmojiFileDocs) + "test"},
		{"file code", EmojiText(EmojiFileCode) + "main", "repo://file/" + EmojiText(EmojiFileCode) + "main"},
		{"section", EmojiText(EmojiSection), "repo://section/" + EmojiText(EmojiSection)},
		{"section nested", BuildSectionID(BuildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"}), "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"})},
		{"definition impl", BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation), "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation)},
		{"ticket", EmojiText(EmojiTicket) + "testticket", "repo://ticket/" + EmojiText(EmojiTicket) + "testticket"},
		{"goal", EmojiText(EmojiGoal) + "r2602runningsketchpad", "repo://goal/" + EmojiText(EmojiGoal) + "r2602runningsketchpad"},
		{"goal nested", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad", "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", EmojiText(EmojiDraft) + "mydraft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft"},
		{"policy", EmojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", EmojiText(EmojiContributor) + "usalu", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", EmojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123"},
		{"interaction started ticket", EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted), "repo://interaction/" + EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted)},
		{"interaction finished goal", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished), "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished)},
		{"session", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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
		{"technologies", "repo://technologies/" + EmojiText(EmojiTechnologies), EmojiText(EmojiTechnologies)},
		{"technology", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose", EmojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo", EmojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"folder", "repo://folder/" + EmojiText(EmojiFolderOrg) + "src", EmojiText(EmojiFolderOrg) + "src"},
		{"file", "repo://file/" + EmojiText(EmojiFileCode) + "test", EmojiText(EmojiFileCode) + "test"},
		{"section", "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}), BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"})},
		{"definition", "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation), BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation)},
		{"definition with section", "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation), BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation)},
		{"ticket", "repo://ticket/" + EmojiText(EmojiTicket) + "testticket", EmojiText(EmojiTicket) + "testticket"},
		{"goal", "repo://goal/" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "runningsketchpad"},
		{"goal nested", "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft", EmojiText(EmojiDraft) + "mydraft"},
		{"policy", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene", EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu", EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123", EmojiText(EmojiCheckpoint) + "abc123"},
		{"interaction", "repo://interaction/" + EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted), EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted)},
		{"session", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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

func TestIdUriRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		id   string
		uri  string
	}{
		{"policy", EmojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", EmojiText(EmojiContributor) + "usalu", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", EmojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123"},
		{"draft", EmojiText(EmojiDraft) + "mydraft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft"},
		{"section", EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSection) + "imports", "repo://section/" + EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSection) + "imports"},
		{"file", EmojiText(EmojiFileCode) + "index", "repo://file/" + EmojiText(EmojiFileCode) + "index"},
		{"ticket", EmojiText(EmojiTicket) + "20260115someticket", "repo://ticket/" + EmojiText(EmojiTicket) + "20260115someticket"},
		{"goal", EmojiText(EmojiGoal) + "r2602running", "repo://goal/" + EmojiText(EmojiGoal) + "r2602running"},
		{"interaction goal", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionStarted), "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionStarted)},
		{"technology", EmojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose"},
		{"bundle", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
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
		props := CollectEntityProps("goal", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("section", data, false)
		if len(props) < 1 || !strings.Contains(props[0], ":10-20") {
			t.Errorf("section props should contain :10-20, got: %v", props)
		}
	})

	t.Run("definition props include name and line range", func(t *testing.T) {
		data := map[string]interface{}{
			"name": "myFunc", "startLine": float64(5), "endLine": float64(15),
		}
		props := CollectEntityProps("definition", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("ticket", data, false)
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
		props := CollectEntityProps("goal", data, false)
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
		props := CollectEntityProps("checkpoint", data, false)
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
		props := CollectEntityProps("policy", data, false)
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

// 📡️#region 🗂️Hook
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

func TestCurrentMcpLLMsAllowed(t *testing.T) {
	for _, llm := range []string{"opus-4-7", "gpt-5-5"} {
		if got, err := ResolveAllowedLLM(llm); err != nil || got != llm {
			t.Fatalf("ResolveAllowedLLM(%q) = %q, %v", llm, got, err)
		}
	}
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
		{"ls piped", "ls .🦑️repo/ | head -20", ToolKindCodeSearch},
		{"ls with redirect", "ls .🦑️repo/🎯️/ 2>/dev/null | head -20 || ls .🦑️repo/ | head -20", ToolKindCodeSearch},
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
			result := ClassifyCommandKind(tc.command)
			if result != tc.expect {
				t.Errorf("classifyCommandKind(%q) = %s, want %s", tc.command, result, tc.expect)
			}
		})
	}
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
