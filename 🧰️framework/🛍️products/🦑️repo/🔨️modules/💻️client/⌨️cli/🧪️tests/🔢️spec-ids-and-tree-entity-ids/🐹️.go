// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exact spec IDs, parent ID propagation, and tree entity IDs.

// #endregion 🧲️Header

package client

import (
	"context"
	"strings"
	"testing"
)

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
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
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

func TestPropagateParentIDs(t *testing.T) {
	root := &TreeNode{Kind: TreeNodeCategory, Data: map[string]interface{}{}, Children: []*TreeNode{
		{Kind: TreeNodeTechnology, Data: map[string]interface{}{"name": "compose", "kind": "user"}, Children: []*TreeNode{
			{Kind: TreeNodeBundle, Data: map[string]interface{}{"name": "compose/js", "kind": "library"}, Children: []*TreeNode{
				{Kind: TreeNodeFolder, Data: map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization"}, Children: []*TreeNode{
					{Kind: TreeNodeFile, Data: map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code"}, Children: []*TreeNode{
						{Kind: TreeNodeSection, Data: map[string]interface{}{"name": "Store"}, Children: []*TreeNode{
							{Kind: TreeNodeDefinition, Data: map[string]interface{}{"name": "createStore", "kind": "implementation"}},
						}},
					}},
				}},
			}},
		}},
	}}
	PropagateParentIDs(root, "")
	technologyId := emojiText(EmojiTechnologyUser) + "compose"
	bundleId := technologyId + emojiText(EmojiBundleLibrary) + "js"
	folderId := bundleId + emojiText(EmojiFolderOrg) + "sketchpad"
	fileId := folderId + emojiText(EmojiFileCode) + "design"
	sectionId := fileId + emojiText(EmojiSection) + "store"
	defId := sectionId + emojiText(EmojiDefinitionImpl) + "createstore"
	checks := []struct {
		label    string
		node     *TreeNode
		expected string
	}{
		{"technology", root.Children[0], technologyId},
		{"bundle", root.Children[0].Children[0], bundleId},
		{"folder", root.Children[0].Children[0].Children[0], folderId},
		{"file", root.Children[0].Children[0].Children[0].Children[0], fileId},
		{"section", root.Children[0].Children[0].Children[0].Children[0].Children[0], sectionId},
		{"definition", root.Children[0].Children[0].Children[0].Children[0].Children[0].Children[0], defId},
	}
	for _, c := range checks {
		t.Run(c.label, func(t *testing.T) {
			entityKind := treeNodeKindToEntityKind(c.node.Kind)
			got := GetArtifactID(entityKind, c.node.Data)
			if got != c.expected {
				t.Errorf("expected %q, got %q", c.expected, got)
			}
		})
	}
}

func TestFlat(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{"repo", "repo"},
		{"Design.tsx", "designtsx"},
		{".devcontainer", "devcontainer"},
		{"devcontainer.json", "devcontainerjson"},
		{"RUNNING-SKETCHPAD", "runningsketchpad"},
		{"R26-02-1", "r26021"},
		{"compose.ts", "composets"},
		{"State Managment", "statemanagment"},
		{"createSketchpadStore", "createsketchpadstore"},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			got := Flat(tc.input)
			if got != tc.expected {
				t.Errorf("Flat(%q): expected %q, got %q", tc.input, tc.expected, got)
			}
		})
	}
}

func verifyTreeDocument(t *testing.T, node *TreeNode, parentPrefix string) {
	t.Helper()
	entityKind := treeNodeKindToEntityKind(node.Kind)
	if entityKind == "" {
		for _, child := range node.Children {
			verifyTreeDocument(t, child, parentPrefix)
		}
		return
	}
	id := GetArtifactID(entityKind, node.Data)
	if id == "" {
		return
	}
	if parentPrefix != "" && !strings.HasPrefix(id, parentPrefix) {
		t.Errorf("%s %q: id %q should start with parent prefix %q", entityKind, node.Label, id, parentPrefix)
	}
	for _, child := range node.Children {
		verifyTreeDocument(t, child, id)
	}
}

func TestExhaustiveMonorepoTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree := BuildMonorepoTreeCached(context.Background())
	var codebaseNode *TreeNode
	for _, child := range tree.Children {
		if child.ID == "codebase" {
			codebaseNode = child
			break
		}
	}
	if codebaseNode == nil {
		t.Fatal("codebase node not found")
	}
	var composeTechnology, composeRepoTechnology *TreeNode
	for _, c := range codebaseNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		t.Logf("Found technology: %s", id)
		if strings.Contains(id, "compose") && !strings.Contains(id, "repo") && !strings.Contains(id, "coda") {
			composeTechnology = c
		}
		if strings.Contains(id, "repo") {
			composeRepoTechnology = c
		}
	}
	if composeTechnology == nil {
		t.Fatal("compose technology not found")
	}
	if composeRepoTechnology == nil {
		t.Fatal("repo technology not found")
	}
	composeId := GetArtifactID("technology", composeTechnology.Data)
	if composeId != emojiText(EmojiTechnologyUser)+"compose" {
		t.Errorf("compose technology id: expected %q, got %q", emojiText(EmojiTechnologyUser)+"compose", composeId)
	}
	composeRepoId := GetArtifactID("technology", composeRepoTechnology.Data)
	if composeRepoId != emojiText(EmojiTechnologyInfra)+"repo" {
		t.Errorf("repo technology id: expected %q, got %q", emojiText(EmojiTechnologyInfra)+"repo", composeRepoId)
	}
	var antlrBundle *TreeNode
	for _, c := range composeTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			t.Logf("Found bundle in composeTechnology: %s", bId)
			if strings.HasSuffix(bId, emojiText("🔤️")+"antlr") {
				antlrBundle = c
				break
			}
		}
	}
	if antlrBundle == nil {
		t.Fatal("compose/antlr bundle not found")
	}
	antlrBundleId := GetArtifactID("bundle", antlrBundle.Data)
	expectedBundleId := emojiText(EmojiTechnologyUser) + "compose" + emojiText("🔤️") + "antlr"
	if antlrBundleId != expectedBundleId {
		t.Errorf("compose/antlr bundle id: expected %q, got %q", expectedBundleId, antlrBundleId)
	}
	for _, c := range antlrBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedBundleId)
		}
	}
	var clientBundle *TreeNode
	for _, c := range composeRepoTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			if strings.HasSuffix(bId, emojiText(EmojiBundleBinary)+"client") {
				clientBundle = c
				break
			}
		}
	}
	if clientBundle == nil {
		t.Fatal("repo/client bundle not found")
	}
	clientBundleId := GetArtifactID("bundle", clientBundle.Data)
	expectedClientBundleId := emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"
	if clientBundleId != expectedClientBundleId {
		t.Errorf("repo/client bundle id: expected %q, got %q", expectedClientBundleId, clientBundleId)
	}
	for _, c := range clientBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedClientBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedClientBundleId)
		}
	}
	verifyTreeDocument(t, tree, "")
}

func TestExhaustiveGoalTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree := BuildMonorepoTreeCached(context.Background())
	goalsNode := tree.Children[2]
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			id := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiGoal)) {
				t.Errorf("goal id should start with goal emoji, got %q", id)
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childId, id) {
						t.Errorf("child goal id %q should start with parent goal id %q", childId, id)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, id) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, id)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
}

func TestExhaustiveContributorTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree := BuildMonorepoTreeCached(context.Background())
	var contributorsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "contributors" {
			contributorsNode = c
			break
		}
	}
	if contributorsNode == nil {
		t.Fatal("contributors node not found")
	}
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id should start with contributor emoji, got %q", id)
			}
		}
	}
}

func TestExhaustiveCheckpointTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree := BuildMonorepoTreeCached(context.Background())
	var checkpointsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "checkpoints" {
			checkpointsNode = c
			break
		}
	}
	if checkpointsNode == nil {
		t.Fatal("checkpoints node not found")
	}
	for _, c := range checkpointsNode.Children {
		if c.Kind == TreeNodeCheckpoint {
			id := GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, emojiText(EmojiCheckpoint)) {
				t.Errorf("checkpoint id should contain checkpoint emoji, got %q", id)
			}
		}
	}
}
