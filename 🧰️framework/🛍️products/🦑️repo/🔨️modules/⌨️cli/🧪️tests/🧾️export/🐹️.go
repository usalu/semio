// 🧾️ Tests of the cli domain: event-log export.

package cli

import (
	context "context"
	os "os"
	filepath "path/filepath"
	testing "testing"

	eventspkg "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
)

// 📤️#region 📤️Event Export
// 📤️testExportContext is a mock RepoContext for testing event export.
type testExportContext struct {
	rootDir      string
	technologies []*model.Technology
	bundles      []*model.Bundle
	folders      []*model.Folder
	files        []*model.File
	sections     []*model.Section
	definitions  []*model.Definition
}

func (c *testExportContext) GetRootDir() string { return c.rootDir }

func (c *testExportContext) GetTechnologies() []*model.Technology { return c.technologies }

func (c *testExportContext) GetBundles() []*model.Bundle { return c.bundles }

func (c *testExportContext) GetFolders() []*model.Folder { return c.folders }

func (c *testExportContext) GetFiles() []*model.File { return c.files }

func (c *testExportContext) GetSections() []*model.Section { return c.sections }

func (c *testExportContext) GetDefinitions() []*model.Definition { return c.definitions }

func (c *testExportContext) GetCheckpoints(limit *int) ([]*model.Checkpoint, error) { return nil, nil }

func (c *testExportContext) GetContributors() ([]*model.Contributor, error) { return nil, nil }

func (c *testExportContext) GetGoals() ([]*model.Goal, error) { return nil, nil }

func (c *testExportContext) GetTickets(year, month, day *int, status *model.TicketStatus) ([]*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) GetPolicies() []*model.Policy { return nil }

func (c *testExportContext) GetDrafts() ([]*model.Draft, error) { return nil, nil }

func (c *testExportContext) GetTodos(filter *model.FilterInput) ([]*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) GetStatutes() []*model.StatuteMeta { return nil }

func (c *testExportContext) GetInteractions() ([]model.InteractionResource, error) {
	return nil, nil
}

func (c *testExportContext) Analyze(scope *string) (*model.AnalyzeResult, error) {
	return &model.AnalyzeResult{}, nil
}

func (c *testExportContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	return nil, nil
}

func (c *testExportContext) GoalDelete(input model.GoalDeleteInput) (bool, error) { return false, nil }

func (c *testExportContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	return nil, nil
}

func (c *testExportContext) TodoDelete(id string) (bool, error) { return false, nil }

func (c *testExportContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	return nil, nil
}

func (c *testExportContext) DraftDelete(id string) (bool, error) { return false, nil }

func (c *testExportContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	return nil, nil
}

func (c *testExportContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	return false, nil
}

func (c *testExportContext) FolderCreate(path string) (*model.Folder, error) { return nil, nil }

func (c *testExportContext) FolderMove(src, dst string) (*model.Folder, error) { return nil, nil }

func (c *testExportContext) FolderDelete(path string) error { return nil }

func (c *testExportContext) FileCreate(path string) (*model.File, error) { return nil, nil }

func (c *testExportContext) FileMove(src, dst string) (*model.File, error) { return nil, nil }

func (c *testExportContext) FileDelete(path string) error { return nil }

func (c *testExportContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionDelete(file, name string) error { return nil }

func (c *testExportContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	return nil, nil
}

func (c *testExportContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	return nil, nil
}

func (c *testExportContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	return nil, nil
}

func (c *testExportContext) ContributorRemove(github string) error { return nil }

func (c *testExportContext) SyncManagement() (bool, error) { return false, nil }

func TestExportToEventLogSchema(t *testing.T) {
	tmpDir := t.TempDir()
	srcDir := filepath.Join(tmpDir, "mytechnology", "mybundle", "src")
	os.MkdirAll(srcDir, 0755)

	tsContent := `// #region 🔖️Header

// 💻️src/app.ts

// 2025 Test <t@t.com>

// GNU Affero General Public License
// https://www.gnu.org/licenses/

// App module summary.


// #region ⚗️Functions
// Processes work items.
export function doWork(): void {}

// #endregion ⚗️Functions
`
	tsFile := filepath.Join(srcDir, "app.ts")
	os.WriteFile(tsFile, []byte(tsContent), 0644)

	technologyReadme := "# My Technology\n\n### Summary\n\nThis is the technology summary.\n\n### Specs\n\nSome specs.\n"
	os.WriteFile(filepath.Join(tmpDir, "mytechnology", "README.md"), []byte(technologyReadme), 0644)

	bundleReadme := "# My Bundle\n\n### Summary\n\nThis is the bundle summary.\n"
	os.WriteFile(filepath.Join(tmpDir, "mytechnology", "mybundle", "README.md"), []byte(bundleReadme), 0644)

	ctx := &testExportContext{
		rootDir: tmpDir,
		technologies: []*model.Technology{
			{Name: "mytechnology", Root: "mytechnology", Kind: model.TechnologyKindUser},
		},
		bundles: []*model.Bundle{
			{Name: "mytechnology/mybundle", Root: "mytechnology/mybundle", TechnologyName: "mytechnology", Kind: model.BundleKindLibrary},
		},
		folders: []*model.Folder{
			{Path: "mytechnology", Name: "mytechnology", Kind: model.FolderKindOrganization},
			{Path: "mytechnology/mybundle", Name: "mybundle", Kind: model.FolderKindOrganization},
			{Path: "mytechnology/mybundle/src", Name: "src", Kind: model.FolderKindOrganization},
		},
		files: []*model.File{
			{Path: "mytechnology/mybundle/src/app.ts", Name: "app.ts", Extension: "ts", Kind: model.FileKindCode},
		},
	}

	outputPath := filepath.Join(tmpDir, "test.events.jsonl")
	result, err := ExportToEventLog(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToEventLog failed: %v", err)
	}

	if result.Technologies != 1 {
		t.Errorf("expected 1 technology, got %d", result.Technologies)
	}
	if result.Bundles != 1 {
		t.Errorf("expected 1 bundle, got %d", result.Bundles)
	}
	if result.Folders != 3 {
		t.Errorf("expected 3 folders, got %d", result.Folders)
	}
	if result.Files != 1 {
		t.Errorf("expected 1 file, got %d", result.Files)
	}
	events, err := (eventspkg.Store{Path: outputPath}).Replay(context.Background(), nil)
	if err != nil {
		t.Fatalf("event replay failed: %v", err)
	}
	if len(events) != result.Technologies+result.Bundles+result.Folders+result.Files+result.Sections+result.Definitions {
		t.Fatalf("event count = %d, result = %+v", len(events), result)
	}
	if events[0].Kind != "bundle.recorded" {
		t.Fatalf("first deterministic event kind = %q", events[0].Kind)
	}
}

func TestExportToEventLogEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	ctx := &testExportContext{
		rootDir:      tmpDir,
		technologies: []*model.Technology{},
		bundles:      []*model.Bundle{},
		folders:      []*model.Folder{},
		files:        []*model.File{},
	}

	outputPath := filepath.Join(tmpDir, "empty.events.jsonl")
	result, err := ExportToEventLog(outputPath, ctx)
	if err != nil {
		t.Fatalf("ExportToEventLog failed: %v", err)
	}
	if result.Technologies != 0 || result.Bundles != 0 || result.Folders != 0 || result.Files != 0 || result.Sections != 0 || result.Definitions != 0 {
		t.Errorf("expected all counts to be 0, got technologies=%d bundles=%d folders=%d files=%d sections=%d definitions=%d",
			result.Technologies, result.Bundles, result.Folders, result.Files, result.Sections, result.Definitions)
	}
	if _, err := os.Stat(outputPath); !os.IsNotExist(err) {
		t.Fatalf("empty export created an unnecessary log: %v", err)
	}
}
