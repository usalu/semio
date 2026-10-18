// 📋️ Tests of the cli domain: policy, list and tree commands and list identifiers.

package cli

import (
	bytes "bytes"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

// 🔬️#region 🗝️Analyze
func TestAnalyzeCommand(t *testing.T) {
	result := ToolAnalyze("compose/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := ToolAnalyze("compose/js/compose.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

// 📜️#region 🔬️Policy
func TestPolicyListCommand(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]statutes.PolicyDef)
	if !ok {
		t.Error("ToolPolicyList data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyList returned no policies")
	}
	foundCode := false
	for _, p := range policies {
		if p.ID == "code" {
			foundCode = true
			break
		}
	}
	if !foundCode {
		t.Error("Expected to find 'code' policy")
	}
}

func TestPolicyTreeCommand(t *testing.T) {
	result := ToolPolicyTree()
	if result.Error != "" {
		t.Errorf("ToolPolicyTree returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyTree returned nil data")
	}
	policies, ok := result.Data.([]statutes.PolicyDef)
	if !ok {
		t.Error("ToolPolicyTree data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyTree returned no policies")
	}
	text := result.Output.Lines[0].Text
	if !strings.Contains(text, "code") {
		t.Error("Expected policy tree output to contain 'code' policy")
	}
	if !strings.Contains(text, "code") && !strings.Contains(text, "Missing") {
		t.Error("Expected policy tree output to contain policy info")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "compose/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

// 📦️#region 📌️Bundle
func TestBundleListCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolBundleList()
	if result.Error != "" {
		t.Errorf("ToolBundleList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolBundleList returned nil data")
	}
	bundles, ok := result.Data.([]model.Bundle)
	if !ok {
		t.Error("ToolBundleList data is not []Bundle")
		return
	}
	if len(bundles) == 0 {
		t.Error("ToolTechnologyList returned no bundles")
	}
	foundJS := false
	for _, b := range bundles {
		if b.Name == "compose/js" {
			foundJS = true
			break
		}
	}
	if !foundJS {
		t.Error("Expected to find 'compose/js' bundle")
	}
}

// 📁️#region 🌨️Folder
func TestFolderListCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFolderList("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFolderTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

// 📄️#region ✏️File
func TestFileListCommand(t *testing.T) {
	result := ToolFileList("compose/js")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestFileTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolFileTree("compose/go")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestSectionTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolSectionTree("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// 🎫️#region 🦉️Ticket
func TestTicketListCommand(t *testing.T) {
	year := 2025
	result := ToolTicketList(&year, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
}

// 🤝️#region 🩻️Contributor
func TestContributorListCommand(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestTechnologyListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Fatalf("ToolTechnologyList returned error: %s", result.Error)
	}
	technologies, ok := result.Data.([]model.Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	expectedIDs := map[string]string{
		"compose": model.EmojiText(model.EmojiTechnologyUser) + "compose",
		"repo":    model.EmojiText(model.EmojiTechnologyInfra) + "repo",
		"coda":    model.EmojiText(model.EmojiTechnologyResearch) + "coda",
	}
	for _, p := range technologies {
		expected, ok := expectedIDs[p.Name]
		if !ok {
			continue
		}
		got := p.GetID()
		if got != expected {
			t.Errorf("technology %q id: expected %q, got %q", p.Name, expected, got)
		}
		delete(expectedIDs, p.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected technology %q not found in list", name)
	}
}

func TestBundleListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolBundleList()
	if result.Error != "" {
		t.Fatalf("ToolBundleList returned error: %s", result.Error)
	}
	bundles, ok := result.Data.([]model.Bundle)
	if !ok {
		t.Fatal("ToolBundleList data is not []Bundle")
	}
	expectedIDs := map[string]string{
		"compose/js":  model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("📜️") + "js",
		"compose/go":  model.EmojiText(model.EmojiTechnologyUser) + "compose" + model.EmojiText("🐹️") + "go",
		"repo/client": model.EmojiText(model.EmojiTechnologyInfra) + "repo" + model.EmojiText("⌨️") + "client",
	}
	for _, b := range bundles {
		expected, ok := expectedIDs[b.Name]
		if !ok {
			continue
		}
		got := b.GetID()
		if got != expected {
			t.Errorf("bundle %q id: expected %q, got %q", b.Name, expected, got)
		}
		delete(expectedIDs, b.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected bundle %q not found in list", name)
	}
}

func TestContributorListIDs(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Fatalf("ToolContributorList returned error: %s", result.Error)
	}
	contributors, ok := result.Data.([]model.Contributor)
	if !ok {
		t.Fatal("ToolContributorList data is not []Contributor")
	}
	if len(contributors) == 0 {
		t.Fatal("ToolContributorList returned no contributors")
	}
	for _, c := range contributors {
		id := c.GetID()
		expectedPrefix := model.EmojiText(model.EmojiContributor)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("contributor %q id %q should start with %q", c.Alias, id, expectedPrefix)
		}
		expectedID := expectedPrefix + workspace.Flat(c.Alias)
		if id != expectedID {
			t.Errorf("contributor %q id: expected %q, got %q", c.Alias, expectedID, id)
		}
	}
	foundUeli := false
	for _, c := range contributors {
		if c.Alias == "ueli" {
			if c.GetID() != model.EmojiText(model.EmojiContributor)+"ueli" {
				t.Errorf("ueli contributor id: expected %q, got %q", model.EmojiText(model.EmojiContributor)+"ueli", c.GetID())
			}
			foundUeli = true
		}
	}
	if !foundUeli {
		t.Error("expected to find contributor 'ueli'")
	}
}

func TestGoalListIDs(t *testing.T) {
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*model.Goal)
	if !ok {
		t.Fatalf("ToolGoalList data is not []*Goal, got %T", result.Data)
	}
	for _, g := range goals {
		id := g.GetID()
		goalEmoji := model.EmojiText(model.EmojiGoal)
		if !strings.HasPrefix(id, goalEmoji) {
			t.Errorf("goal %q id %q should start with %q", g.ID, id, goalEmoji)
		}
		var expected strings.Builder
		for _, segment := range strings.Split(g.ID, "/") {
			expected.WriteString(goalEmoji)
			expected.WriteString(workspace.Flat(segment))
		}
		if id != expected.String() {
			t.Errorf("goal %q id: expected %q, got %q", g.ID, expected.String(), id)
		}
	}
}

func TestDraftListIDs(t *testing.T) {
	result := ToolDraftList()
	if result.Error != "" {
		t.Skipf("ToolDraftList returned error: %s", result.Error)
	}
	drafts, ok := result.Data.([]*model.Draft)
	if !ok {
		t.Skip("ToolDraftList data is not []*Draft")
	}
	for _, d := range drafts {
		id := d.GetID()
		expectedPrefix := model.EmojiText(model.EmojiDraft)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
		}
	}
}

// 🌳️#region 📜️Tree
func executeTreeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	root, _ := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)

	err := root.Execute()
	return buf.String(), err
}
