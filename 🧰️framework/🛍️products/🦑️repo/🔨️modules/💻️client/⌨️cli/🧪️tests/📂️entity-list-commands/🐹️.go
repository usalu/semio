// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Bundle, folder, file, section, definition, ticket, and contributor list commands.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"testing"
	"time"
)

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
	bundles, ok := result.Data.([]Bundle)
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

func TestExhaustiveBundleTreeCommand(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle tree test in short mode")
	}
	result := ToolTechnologyTree()
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
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

func TestFolderCreateMoveDelete(t *testing.T) {
	testFolder := "temp/test-folder-cli"
	createResult := ToolFolderCreate(testFolder)
	if createResult.Error != "" {
		t.Errorf("ToolFolderCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFolderMove(testFolder, testFolder+"-moved")
	if moveResult.Error != "" {
		t.Errorf("ToolFolderMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFolderDelete(testFolder + "-moved")
	if deleteResult.Error != "" {
		t.Errorf("ToolFolderDelete returned error: %s", deleteResult.Error)
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

func TestFileCreateMoveDelete(t *testing.T) {
	testFile := "temp/test-file-cli.txt"
	createResult := ToolFileCreate(testFile)
	if createResult.Error != "" {
		t.Errorf("ToolFileCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFileMove(testFile, "temp/test-file-cli-moved.txt")
	if moveResult.Error != "" {
		t.Errorf("ToolFileMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFileDelete("temp/test-file-cli-moved.txt")
	if deleteResult.Error != "" {
		t.Errorf("ToolFileDelete returned error: %s", deleteResult.Error)
	}
}

// 📑️#region 🖲️Section
func TestSectionListCommand(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolSectionList returned nil data")
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Error("ToolSectionList data is not []SectionInfo")
		return
	}
	if len(sections) == 0 {
		t.Error("ToolSectionList returned no sections")
	}
	foundHeader := false
	for _, s := range sections {
		if s.Name == "Header" {
			foundHeader = true
			break
		}
	}
	if !foundHeader {
		t.Errorf("Expected to find 'Header' section in %s", sectionFixtureRelPath)
	}
}

func TestSectionTreeCommand(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolSectionTree("compose/js/index.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// 📖️#region 🐍️Definition
func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
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

func TestTicketOpenNoticketKeyword(t *testing.T) {
	result := ToolTicketOpen("🎫️", "Skip Ticket", "NOTICKET skip ticket creation", "gpt-5-mini", "", "codex", "", true, "", "", false, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	if result.Data != nil {
		t.Fatalf("expected no ticket data for NOTICKET keyword")
	}
}

func TestTicketOpenContinueKeyword(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	first := ToolTicketOpen("🌱️", "Seed Ticket", "Seed prompt", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", McpClientGeneric, "", "")
	if first.Error != "" {
		t.Fatalf("failed to seed ticket: %s", first.Error)
	}
	seed, ok := first.Data.(*Ticket)
	if !ok || seed == nil {
		t.Fatalf("expected seeded ticket data")
	}
	second := ToolTicketOpen("🎫️", "Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", McpClientGeneric, "", "")
	if second.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", second.Error)
	}
	continued, ok := second.Data.(*Ticket)
	if !ok || continued == nil {
		t.Fatalf("expected continued ticket data")
	}
	if continued.Slug != seed.Slug {
		t.Fatalf("expected continued ticket %s, got %s", seed.Slug, continued.Slug)
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
