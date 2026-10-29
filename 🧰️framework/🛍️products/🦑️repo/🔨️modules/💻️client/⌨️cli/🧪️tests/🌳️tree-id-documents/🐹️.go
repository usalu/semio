// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Tree ID documents and spec ID examples.

// #endregion 🧲️Header

package client

import (
	"context"
	"strings"
	"testing"
)

func TestExhaustiveMonorepoTreeFullIDDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
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
	var composeTechnology, composeRepoTechnology, codaTechnology *TreeNode
	for _, c := range codebaseNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		if id == emojiText(EmojiTechnologyUser)+"compose" {
			composeTechnology = c
		} else if id == emojiText(EmojiTechnologyInfra)+"repo" {
			composeRepoTechnology = c
		} else if id == emojiText(EmojiTechnologyResearch)+"coda" {
			codaTechnology = c
		}
	}
	if composeTechnology == nil {
		t.Fatal("compose technology not found with expected id " + emojiText(EmojiTechnologyUser) + "compose")
	}
	if composeRepoTechnology == nil {
		t.Fatal("repo technology not found with expected id " + emojiText(EmojiTechnologyInfra) + "repo")
	}
	if codaTechnology == nil {
		t.Fatal("coda technology not found with expected id " + emojiText(EmojiTechnologyResearch) + "coda")
	}
	expectedComposeId := emojiText(EmojiTechnologyUser) + "compose"
	actualComposeId := GetArtifactID("technology", composeTechnology.Data)
	if actualComposeId != expectedComposeId {
		t.Errorf("compose technology id: expected %q, got %q", expectedComposeId, actualComposeId)
	}
	expectedRepoId := emojiText(EmojiTechnologyInfra) + "repo"
	actualRepoId := GetArtifactID("technology", composeRepoTechnology.Data)
	if actualRepoId != expectedRepoId {
		t.Errorf("repo technology id: expected %q, got %q", expectedRepoId, actualRepoId)
	}
	expectedCodaId := emojiText(EmojiTechnologyResearch) + "coda"
	actualCodaId := GetArtifactID("technology", codaTechnology.Data)
	if actualCodaId != expectedCodaId {
		t.Errorf("coda technology id: expected %q, got %q", expectedCodaId, actualCodaId)
	}
	bundleChecks := map[string]string{
		"compose/js":      emojiText(EmojiTechnologyUser) + "compose" + emojiText("📜️") + "js",
		"compose/go":      emojiText(EmojiTechnologyUser) + "compose" + emojiText("🐹️") + "go",
		"compose/engine":  emojiText(EmojiTechnologyUser) + "compose" + emojiText("⚙️") + "engine",
		"compose/desktop": emojiText(EmojiTechnologyUser) + "compose" + emojiText("🖥️") + "desktop",
		"repo/client":     emojiText(EmojiTechnologyInfra) + "repo" + emojiText("⌨️") + "client",
		"repo/server":     emojiText(EmojiTechnologyInfra) + "repo" + emojiText("🌍️") + "server",
	}
	allBundles := []*TreeNode{}
	for _, technology := range []*TreeNode{composeTechnology, composeRepoTechnology, codaTechnology} {
		for _, child := range technology.Children {
			if child.Kind == TreeNodeBundle {
				allBundles = append(allBundles, child)
			}
		}
	}
	for _, b := range allBundles {
		bundleId := GetArtifactID("bundle", b.Data)
		name, _ := b.Data["name"].(string)
		if expected, ok := bundleChecks[name]; ok {
			if bundleId != expected {
				t.Errorf("bundle %q id: expected %q, got %q", name, expected, bundleId)
			}
			delete(bundleChecks, name)
		}
		for _, child := range b.Children {
			childEK := treeNodeKindToEntityKind(child.Kind)
			if childEK == "" {
				continue
			}
			childId := GetArtifactID(childEK, child.Data)
			if !strings.HasPrefix(childId, bundleId) {
				t.Errorf("bundle %q child %s %q: id %q should start with bundle id %q", name, childEK, child.Label, childId, bundleId)
			}
		}
	}
	for name := range bundleChecks {
		t.Errorf("expected bundle %q not found in tree", name)
	}
	verifyTreeDocument(t, tree, "")
}

func TestExhaustiveGoalTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	goalsNode := tree.Children[2]
	goalCount := 0
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			goalCount++
			goalId := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(goalId, emojiText(EmojiGoal)) {
				t.Errorf("goal id %q should start with %q", goalId, emojiText(EmojiGoal))
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childGoalId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childGoalId, goalId) {
						t.Errorf("child goal id %q should start with parent goal id %q", childGoalId, goalId)
					}
					if !strings.Contains(childGoalId, emojiText(EmojiGoal)) {
						t.Errorf("child goal id %q should contain goal emoji", childGoalId)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, goalId) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, goalId)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
	if goalCount == 0 {
		t.Log("no goals found in tree (may be expected for fresh repos)")
	}
}

func TestExhaustiveContributorTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
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
	foundUeli := false
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id %q should start with %q", id, emojiText(EmojiContributor))
			}
			alias, _ := c.Data["alias"].(string)
			expectedID := emojiText(EmojiContributor) + Flat(alias)
			if id != expectedID {
				t.Errorf("contributor %q id: expected %q, got %q", alias, expectedID, id)
			}
			if alias == "ueli" {
				foundUeli = true
				if id != emojiText(EmojiContributor)+"ueli" {
					t.Errorf("ueli contributor id: expected %q, got %q", emojiText(EmojiContributor)+"ueli", id)
				}
			}
		}
	}
	if !foundUeli {
		t.Error("expected to find contributor 'ueli' in tree")
	}
}

func TestExhaustiveCheckpointTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	withExhaustiveRepo(t)
	tree, err := BuildMonorepoTreeCached(context.Background())
	if err != nil {
		t.Fatal(err)
	}
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
	checkpointCount := 0
	for _, c := range checkpointsNode.Children {
		if c.Kind == TreeNodeCheckpoint {
			checkpointCount++
			id := GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, emojiText(EmojiCheckpoint)) {
				t.Errorf("checkpoint id %q should contain %q", id, emojiText(EmojiCheckpoint))
			}
			sha, _ := c.Data["sha"].(string)
			if sha != "" && !strings.HasSuffix(id, sha) {
				t.Errorf("checkpoint id %q should end with sha %q", id, sha)
			}
			contributorId, _ := c.Data["contributorId"].(string)
			if contributorId != "" && !strings.HasPrefix(id, contributorId) {
				t.Errorf("checkpoint id %q should start with contributor id %q", id, contributorId)
			}
		}
	}
	if checkpointCount == 0 {
		t.Error("no checkpoints found in tree")
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
		{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay)},
		{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"},
		{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F0"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"},
		{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute)},
		{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33"},
		{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33" + emojiText(EmojiSecond)},
		{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33", "ss": "38"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014" + emojiText(EmojiMinute) + "33" + emojiText(EmojiSecond) + "38"},
		{"technologies under root", "technologies", map[string]interface{}{"parentId": ""}, emojiText(EmojiTechnologies)},
		{"infra technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, emojiText(EmojiTechnologyInfra) + "repo"},
		{"user technology compose", "technology", map[string]interface{}{"name": "compose", "kind": "user"}, emojiText(EmojiTechnologyUser) + "compose"},
		{"research technology coda", "technology", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiTechnologyResearch) + "coda"},
		{"bundles under technology", "bundles", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundles)},
		{"library bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle compose/graphql", "bundle", map[string]interface{}{"name": "compose/graphql", "kind": "schema"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle repo/client", "bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"},
		{"ui bundle repo/vscode", "bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle coda/example", "bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleExample) + "example"},
		{"assets bundle asset", "bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, emojiText(EmojiTechnologyUser) + "asset" + emojiText(EmojiBundleAssets) + "asset"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, emojiText(EmojiFolders)},
		{"bundle sketchpad folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder .github folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
		{"org folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, emojiText(EmojiFiles)},
		{"sketchpad files", "files", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"github files", "files", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design", "line": float64(3872)}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiLine) + "3872"},
		{"range 3872-3875", "range", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore", "startLine": float64(3872), "endLine": float64(3875)}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore" + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"},
		{"sections in file", "sections", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSections)},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment"},
		{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitions)},
		{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
		{"definition interface", "definition", map[string]interface{}{"name": "IStore", "kind": "interface", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionInterface) + "istore"},
		{"definition constant", "definition", map[string]interface{}{"name": "MAX_SIZE", "kind": "constant", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionConstant) + "maxsize"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, emojiText(EmojiGoals)},
		{"nested goals under parent", "goals", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiGoals)},
		{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
		{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, emojiText(EmojiTickets)},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTickets)},
		{"section tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiTickets)},
		{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client" + emojiText(EmojiDraft) + "newarchitecture"},
		{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore" + emojiText(EmojiTodo) + "introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy Only One Store", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles",
			"affected": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore",
			"lineId":   emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875",
			"secondId": emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
		}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles" + emojiText(EmojiBreach) + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstore" + emojiText(EmojiBreachScope) + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875" + emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"},
		{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCheckpoint) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "started",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionStarted)},
		{"interaction edited", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "edited",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionEdited)},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "finished",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionFinished)},
		{"interaction restarted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "restarted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionRestarted)},
		{"interaction deleted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "deleted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionDeleted)},
		{"file test kind", "file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "kind": "lab", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileLab) + "sketchpadtest"},
		{"file script kind", "file", map[string]interface{}{"path": "compose/engine/build.ts", "kind": "script", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "engine"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "engine" + emojiText(EmojiFileScript) + "build"},
		{"file docs kind", "file", map[string]interface{}{"path": "compose/js/README.md", "kind": "docs", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileDocs) + "readme"},
		{"file asset kind", "file", map[string]interface{}{"path": "compose/js/sketchpad/page/showcase/metabolism.mdx", "kind": "resource", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases" + emojiText(EmojiFileResource) + "metabolism"},
		{"file license kind", "file", map[string]interface{}{"path": "compose/go/LICENSE.md", "kind": "license", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "go"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "go" + emojiText(EmojiFileLicense) + "license"},
		{"site bundle compose/docs", "bundle", map[string]interface{}{"name": "compose/docs", "kind": "site"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "docs"},
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
