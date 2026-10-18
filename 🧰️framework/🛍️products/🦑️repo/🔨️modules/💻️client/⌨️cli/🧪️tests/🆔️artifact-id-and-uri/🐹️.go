// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact ID and URI conversions.

// #endregion 🧲️Header

package client

import (
	"testing"
)

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
			wantID:  emojiText(EmojiTechnologies),
			wantURI: "repo://technologies/" + emojiText(EmojiTechnologies),
		},
		{
			name:    "technology user",
			kind:    "technology",
			data:    map[string]interface{}{"name": "compose", "kind": "user"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose",
		},
		{
			name:    "technology infrastructure",
			kind:    "technology",
			data:    map[string]interface{}{"name": "repo", "kind": "infrastructure"},
			wantID:  emojiText(EmojiTechnologyInfra) + "repo",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo",
		},
		{
			name:    "technology research",
			kind:    "technology",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  emojiText(EmojiTechnologyResearch) + "coda",
			wantURI: "repo://technology/" + emojiText(EmojiTechnologyResearch) + "coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"technologyCode": "compose", "parentId": emojiText(EmojiTechnologyUser) + "compose"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles),
			wantURI: "repo://bundles/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles),
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/js", "kind": "library"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/example", "kind": "library"},
			wantID:  emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleLibrary) + "example",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleLibrary) + "example",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/desktop", "kind": "ui"},
			wantID:  emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleUI) + "desktop",
			wantURI: "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleUI) + "desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFolders),
			wantURI: "repo://folders/" + emojiText(EmojiFolders),
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "compose/js/src", "parentId": emojiText(EmojiFolderOrg) + "src"},
			wantID:  emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFolders),
			wantURI: "repo://folders/" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFolders),
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/src", "kind": "required"},
			wantID:  emojiText(EmojiFolderRequired) + "src",
			wantURI: "repo://folder/" + emojiText(EmojiFolderRequired) + "src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/utils", "kind": "organization"},
			wantID:  emojiText(EmojiFolderOrg) + "utils",
			wantURI: "repo://folder/" + emojiText(EmojiFolderOrg) + "utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFiles),
			wantURI: "repo://files/" + emojiText(EmojiFiles),
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  emojiText(EmojiFileDocs) + "test",
			wantURI: "repo://file/" + emojiText(EmojiFileDocs) + "test",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  emojiText(EmojiFileCode) + "main",
			wantURI: "repo://file/" + emojiText(EmojiFileCode) + "main",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "compose/js/src/🧪️index.test.ts", "kind": "lab"},
			wantID:  emojiText(EmojiFileLab) + Flat("🧪️index.test"),
			wantURI: "repo://file/" + emojiText(EmojiFileLab) + Flat("🧪️index.test"),
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  emojiText(EmojiFileConfig) + "tsconfig",
			wantURI: "repo://file/" + emojiText(EmojiFileConfig) + "tsconfig",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  emojiText(EmojiFileScript) + "build",
			wantURI: "repo://file/" + emojiText(EmojiFileScript) + "build",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "🖼️logo.png", "kind": "resource"},
			wantID:  emojiText(EmojiFileResource) + Flat("🖼️logo"),
			wantURI: "repo://file/" + emojiText(EmojiFileResource) + Flat("🖼️logo"),
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  emojiText(EmojiFileLicense) + "license",
			wantURI: "repo://file/" + emojiText(EmojiFileLicense) + "license",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": emojiText(EmojiFileCode) + "index"},
			wantID:  emojiText(EmojiFileCode) + "index" + emojiText(EmojiSections),
			wantURI: "repo://sections/" + emojiText(EmojiFileCode) + "index" + emojiText(EmojiSections),
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/Design.tsx#State Management#Design Store"},
			wantID:  buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
			wantURI: "repo://section/" + buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/file.ts#Imports"},
			wantID:  buildSectionID(buildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
			wantURI: "repo://section/" + buildSectionID(buildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": emojiText(EmojiSection) + "types"},
			wantID:  emojiText(EmojiSection) + "types" + emojiText(EmojiDefinitions),
			wantURI: "repo://definitions/" + emojiText(EmojiSection) + "types" + emojiText(EmojiDefinitions),
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "compose/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "compose/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
		},
		{
			name:    "definition go type treated as interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "type", "filePath": "repo/client/main.go", "sectionPath": "GraphQL Types#GraphQL Input Types", "name": "TicketCloseInput"},
			wantID:  buildDefinitionID(buildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "compose/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
			wantURI: "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTickets),
			wantURI: "repo://tickets/" + emojiText(EmojiTickets),
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
			wantID:  emojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + emojiText(EmojiTicket) + "testticket",
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
			wantID:  emojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + emojiText(EmojiTicket) + "testticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiGoals),
			wantURI: "repo://goals/" + emojiText(EmojiGoals),
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "RUNNING-SKETCHPAD", "parentId": ""},
			wantID:  emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + emojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "goal nested",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiDrafts),
			wantURI: "repo://drafts/" + emojiText(EmojiDrafts),
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  emojiText(EmojiDraft) + "mydraft",
			wantURI: "repo://draft/" + emojiText(EmojiDraft) + "mydraft",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTodos),
			wantURI: "repo://todos/" + emojiText(EmojiTodos),
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  emojiText(EmojiTodo) + "mytodo",
			wantURI: "repo://todo/" + emojiText(EmojiTodo) + "mytodo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiPolicies),
			wantURI: "repo://policies/" + emojiText(EmojiPolicies),
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  emojiText(EmojiPolicy) + "codehygiene",
			wantURI: "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene",
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
			wantID:  emojiText(EmojiContributors),
			wantURI: "repo://contributors/" + emojiText(EmojiContributors),
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  emojiText(EmojiContributor) + "usalu",
			wantURI: "repo://contributor/" + emojiText(EmojiContributor) + "usalu",
		},
		{
			name:    "checkpoints collection",
			kind:    "checkpoints",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiCheckpoints),
			wantURI: "repo://checkpoints/" + emojiText(EmojiCheckpoints),
		},
		{
			name:    "checkpoint",
			kind:    "checkpoint",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  emojiText(EmojiCheckpoint) + "abc123",
			wantURI: "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123",
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
			data:    map[string]interface{}{"kind": "started", "entityId": emojiText(EmojiTicket) + "introduceinteractionmechanism"},
			wantID:  emojiText(EmojiTicket) + "introduceinteractionmechanism" + emojiText(EmojiInteractionStarted),
			wantURI: "repo://interaction/" + emojiText(EmojiTicket) + "introduceinteractionmechanism" + emojiText(EmojiInteractionStarted),
		},
		{
			name:    "interaction finished goal",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "finished", "entityId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished),
			wantURI: "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished),
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
