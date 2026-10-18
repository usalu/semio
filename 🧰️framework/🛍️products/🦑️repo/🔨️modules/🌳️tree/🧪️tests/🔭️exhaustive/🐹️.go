//go:build exhaustive

// 🔬️ Tests of the tree domain, split out of the pre-split godfile suite.

package tree

import (
	context "context"
	os "os"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveMonorepoTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
		entityKind := TreeNodeKindToEntityKind(c.Kind)
		id := model.GetArtifactID(entityKind, c.Data)
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
	composeId := model.GetArtifactID("technology", composeTechnology.Data)
	if composeId != model.EmojiText(model.EmojiTechnologyUser)+"compose" {
		t.Errorf("compose technology id: expected %q, got %q", model.EmojiText(model.EmojiTechnologyUser)+"compose", composeId)
	}
	composeRepoId := model.GetArtifactID("technology", composeRepoTechnology.Data)
	if composeRepoId != model.EmojiText(model.EmojiTechnologyInfra)+"repo" {
		t.Errorf("repo technology id: expected %q, got %q", model.EmojiText(model.EmojiTechnologyInfra)+"repo", composeRepoId)
	}
	var antlrBundle *TreeNode
	for _, c := range composeTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := model.GetArtifactID("bundle", c.Data)
			t.Logf("Found bundle in composeTechnology: %s", bId)
			if strings.HasSuffix(bId, model.EmojiText("🔤️")+"antlr") {
				antlrBundle = c
				break
			}
		}
	}
	if antlrBundle == nil {
		t.Fatal("compose/antlr bundle not found")
	}
	antlrBundleId := model.GetArtifactID("bundle", antlrBundle.Data)
	expectedBundleId := model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("🔤️") + "antlr"
	if antlrBundleId != expectedBundleId {
		t.Errorf("compose/antlr bundle id: expected %q, got %q", expectedBundleId, antlrBundleId)
	}
	for _, c := range antlrBundle.Children {
		ek := TreeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := model.GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedBundleId)
		}
	}
	var clientBundle *TreeNode
	for _, c := range composeRepoTechnology.Children {
		if c.Kind == TreeNodeBundle {
			bId := model.GetArtifactID("bundle", c.Data)
			if strings.HasSuffix(bId, model.EmojiText(model.EmojiBundleBinary)+"client") {
				clientBundle = c
				break
			}
		}
	}
	if clientBundle == nil {
		t.Fatal("repo/client bundle not found")
	}
	clientBundleId := model.GetArtifactID("bundle", clientBundle.Data)
	expectedClientBundleId := model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText(model.EmojiBundleBinary) + "client"
	if clientBundleId != expectedClientBundleId {
		t.Errorf("repo/client bundle id: expected %q, got %q", expectedClientBundleId, clientBundleId)
	}
	for _, c := range clientBundle.Children {
		ek := TreeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := model.GetArtifactID(ek, c.Data)
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
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	goalsNode := tree.Children[2]
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			id := model.GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(id, model.EmojiText(model.EmojiGoal)) {
				t.Errorf("goal id should start with goal emoji, got %q", id)
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childId := model.GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childId, id) {
						t.Errorf("child goal id %q should start with parent goal id %q", childId, id)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := model.GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, id) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, id)
					}
					if !strings.Contains(ticketId, model.EmojiText(model.EmojiTicket)) {
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
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
			id := model.GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, model.EmojiText(model.EmojiContributor)) {
				t.Errorf("contributor id should start with contributor emoji, got %q", id)
			}
		}
	}
}

func TestExhaustiveCheckpointTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
			id := model.GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, model.EmojiText(model.EmojiCheckpoint)) {
				t.Errorf("checkpoint id should contain checkpoint emoji, got %q", id)
			}
		}
	}
}

func TestExhaustiveMonorepoTreeFullIDDocument(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
	var composeTechnology, composeRepoTechnology, codaTechnology *TreeNode
	for _, c := range codebaseNode.Children {
		entityKind := TreeNodeKindToEntityKind(c.Kind)
		id := model.GetArtifactID(entityKind, c.Data)
		if id == model.EmojiText(model.EmojiTechnologyUser)+"compose" {
			composeTechnology = c
		} else if id == model.EmojiText(model.EmojiTechnologyInfra)+"repo" {
			composeRepoTechnology = c
		} else if id == model.EmojiText(model.EmojiTechnologyResearch)+"coda" {
			codaTechnology = c
		}
	}
	if composeTechnology == nil {
		t.Fatal("compose technology not found with expected id " + model.EmojiText(model.EmojiTechnologyUser) + "compose")
	}
	if composeRepoTechnology == nil {
		t.Fatal("repo technology not found with expected id " + model.EmojiText(model.EmojiTechnologyInfra) + "repo")
	}
	if codaTechnology == nil {
		t.Fatal("coda technology not found with expected id " + model.EmojiText(model.EmojiTechnologyResearch) + "coda")
	}
	expectedComposeId := model.EmojiText(model.EmojiTechnologyUser) + "compose"
	actualComposeId := model.GetArtifactID("technology", composeTechnology.Data)
	if actualComposeId != expectedComposeId {
		t.Errorf("compose technology id: expected %q, got %q", expectedComposeId, actualComposeId)
	}
	expectedRepoId := model.EmojiText(model.EmojiTechnologyInfra) + "repo"
	actualRepoId := model.GetArtifactID("technology", composeRepoTechnology.Data)
	if actualRepoId != expectedRepoId {
		t.Errorf("repo technology id: expected %q, got %q", expectedRepoId, actualRepoId)
	}
	expectedCodaId := model.EmojiText(model.EmojiTechnologyResearch) + "coda"
	actualCodaId := model.GetArtifactID("technology", codaTechnology.Data)
	if actualCodaId != expectedCodaId {
		t.Errorf("coda technology id: expected %q, got %q", expectedCodaId, actualCodaId)
	}
	bundleChecks := map[string]string{
		"compose/js":      model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("📜️") + "js",
		"compose/go":      model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("🐹️") + "go",
		"compose/engine":  model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("⚙️") + "engine",
		"asset":           model.EmojiText(model.EmojiTechnologyUser) + "semio" + model.EmojiText(model.EmojiBundleAssets) + "asset",
		"compose/desktop": model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("🖥️") + "desktop",
		"compose/docs":    model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText(model.EmojiBundleSite) + "docs",
		"repo/client":     model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText("⌨️") + "client",
		"repo/server":     model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText("🌍️") + "server",
		"repo/vscode":     model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText("🧩️") + "vscode",
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
		bundleId := model.GetArtifactID("bundle", b.Data)
		name, _ := b.Data["name"].(string)
		if expected, ok := bundleChecks[name]; ok {
			if bundleId != expected {
				t.Errorf("bundle %q id: expected %q, got %q", name, expected, bundleId)
			}
			delete(bundleChecks, name)
		}
		for _, child := range b.Children {
			childEK := TreeNodeKindToEntityKind(child.Kind)
			if childEK == "" {
				continue
			}
			childId := model.GetArtifactID(childEK, child.Data)
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
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTreeCached(context.Background())
	goalsNode := tree.Children[2]
	goalCount := 0
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			goalCount++
			goalId := model.GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(goalId, model.EmojiText(model.EmojiGoal)) {
				t.Errorf("goal id %q should start with %q", goalId, model.EmojiText(model.EmojiGoal))
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childGoalId := model.GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childGoalId, goalId) {
						t.Errorf("child goal id %q should start with parent goal id %q", childGoalId, goalId)
					}
					if !strings.Contains(childGoalId, model.EmojiText(model.EmojiGoal)) {
						t.Errorf("child goal id %q should contain goal emoji", childGoalId)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := model.GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, goalId) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, goalId)
					}
					if !strings.Contains(ticketId, model.EmojiText(model.EmojiTicket)) {
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
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
	foundUeli := false
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := model.GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, model.EmojiText(model.EmojiContributor)) {
				t.Errorf("contributor id %q should start with %q", id, model.EmojiText(model.EmojiContributor))
			}
			alias, _ := c.Data["alias"].(string)
			expectedID := model.EmojiText(model.EmojiContributor) + workspace.Flat(alias)
			if id != expectedID {
				t.Errorf("contributor %q id: expected %q, got %q", alias, expectedID, id)
			}
			if alias == "ueli" {
				foundUeli = true
				if id != model.EmojiText(model.EmojiContributor)+"ueli" {
					t.Errorf("ueli contributor id: expected %q, got %q", model.EmojiText(model.EmojiContributor)+"ueli", id)
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
	cwd, _ := os.Getwd()
	workspace.SetRootDir(findTestRepoRoot(cwd))
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
	checkpointCount := 0
	for _, c := range checkpointsNode.Children {
		if c.Kind == TreeNodeCheckpoint {
			checkpointCount++
			id := model.GetArtifactID("checkpoint", c.Data)
			if !strings.Contains(id, model.EmojiText(model.EmojiCheckpoint)) {
				t.Errorf("checkpoint id %q should contain %q", id, model.EmojiText(model.EmojiCheckpoint))
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

func TestExhaustiveBuildMonorepoTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree build test in short mode")
	}

	cwd, _ := os.Getwd()
	oldRoot := workspace.RootDir
	workspace.RootDir = findTestRepoRoot(cwd)
	defer func() { workspace.RootDir = oldRoot }()
	codebase.InvalidateTechnologyCache()

	ctx := context.Background()
	treeNoSections := BuildMonorepoTreeCached(ctx)
	treeSections := BuildMonorepoTreeCached(ctx, TreeBuildOptions{IncludeSections: true})

	t.Run("builds tree with categories", func(t *testing.T) {
		if treeNoSections == nil {
			t.Fatal("tree should not be nil")
		}
		if len(treeNoSections.Children) == 0 {
			t.Fatal("tree should have categories")
		}
		categoryIDs := make(map[string]bool)
		for _, c := range treeNoSections.Children {
			if c.Kind != TreeNodeCategory {
				t.Errorf("top-level children should be categories, got %s", c.Kind)
			}
			categoryIDs[c.ID] = true
		}
		expected := []string{"codebase", "goals", "drafts", "policies", "contributors", "checkpoints"}
		for _, id := range expected {
			if !categoryIDs[id] {
				t.Errorf("missing category: %s", id)
			}
		}
	})

	t.Run("codebase category has folder and file document", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		if len(codebaseNode.Children) == 0 {
			t.Fatal("codebase category should have children")
		}
		hasFolder := false
		hasFile := false
		hasSection := false
		hasDefinition := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			switch node.Kind {
			case TreeNodeFolder:
				hasFolder = true
			case TreeNodeFile:
				hasFile = true
			case TreeNodeSection:
				hasSection = true
			case TreeNodeDefinition:
				hasDefinition = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(codebaseNode)
		if !hasFolder {
			t.Error("codebase category should include folder nodes")
		}
		if !hasFile {
			t.Error("codebase category should include file nodes")
		}
		if !hasSection {
			t.Error("codebase category should include section nodes when IncludeSections is true")
		}
		if !hasDefinition {
			t.Error("codebase category should include definition nodes when IncludeSections is true")
		}
	})

	t.Run("codebase category has technology children", func(t *testing.T) {
		var codebaseNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "codebase" {
				codebaseNode = c
				break
			}
		}
		if codebaseNode == nil {
			t.Fatal("codebase category not found")
		}
		hasTechnologies := false
		hasBundles := false
		for _, p := range codebaseNode.Children {
			if p.Kind == TreeNodeTechnology {
				hasTechnologies = true
				for _, b := range p.Children {
					if b.Kind == TreeNodeBundle {
						hasBundles = true
					}
				}
			}
		}
		if !hasTechnologies {
			t.Error("codebase should have technology children")
		}
		if !hasBundles {
			t.Error("at least one technology should have bundles")
		}
	})

	t.Run("policies category uses entitykind grouping", func(t *testing.T) {
		var policiesNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "policies" {
				policiesNode = c
				break
			}
		}
		if policiesNode == nil {
			t.Fatal("policies category not found")
		}
		if len(policiesNode.Children) == 0 {
			t.Fatal("policies should have children")
		}
		policy := policiesNode.Children[0]
		if policy.Kind != TreeNodePolicy {
			t.Fatalf("expected policy node, got %s", policy.Kind)
		}
		if len(policy.Children) == 0 {
			t.Fatal("policy should contain entitykind children")
		}
		entityKind := policy.Children[0]
		if entityKind.Kind != TreeNodeCategory {
			t.Fatalf("expected entitykind category node, got %s", entityKind.Kind)
		}
		if entityKind.SubKind != "entitykind" {
			t.Fatalf("expected entitykind subkind, got %s", entityKind.SubKind)
		}
		if len(entityKind.Children) == 0 {
			t.Fatal("entitykind should contain statute children")
		}
		for _, statute := range entityKind.Children {
			if statute.Kind != TreeNodeStatute {
				t.Fatalf("expected statute child under entitykind, got %s", statute.Kind)
			}
		}
	})

	t.Run("with sections includes sections", func(t *testing.T) {
		hasSections := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				hasSections = true
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeSections)
		if !hasSections {
			t.Error("tree with IncludeSections should have section nodes")
		}
	})

	t.Run("without sections excludes sections", func(t *testing.T) {
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				t.Error("tree without IncludeSections should not have section nodes")
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(treeNoSections)
	})
}

// 🧜️#region ⏲️Mermaid
func TestExhaustiveMermaidLocByTechnologiesBundlesFoldersFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("walks all bundles for LOC treemap; too slow for -short runs on large monorepos")
	}
	root := findTestRepoRoot(".")
	workspace.SetRootDir(root)
	result := MermaidLocByTechnologiesBundlesFoldersFiles()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code\"") {
		t.Error("expected 'Lines of Code' title")
	}
	if !strings.Contains(result, model.EmojiTechnologyUser) {
		t.Error("expected user technology emoji")
	}
	if !strings.Contains(result, model.EmojiTechnologyInfra) {
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
	root := findTestRepoRoot(".")
	workspace.SetRootDir(root)
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
	root := findTestRepoRoot(".")
	workspace.SetRootDir(root)
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
