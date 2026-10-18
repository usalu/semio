// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Event log export schema and Postgres schema.

// #endregion 🧲️Header

package client

import (
	"context"
	eventstore "github.com/usalu/semio/repo/client/internal/eventstore"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 📤️#region 📤️Event Export
// 📤️testExportContext is a mock RepoContext for testing event export.
type testExportContext struct {
	rootDir      string
	technologies []*Technology
	bundles      []*Bundle
	folders      []*Folder
	files        []*File
	sections     []*Section
	definitions  []*Definition
}

func (c *testExportContext) GetRootDir() string                               { return c.rootDir }

func (c *testExportContext) GetTechnologies() []*Technology                   { return c.technologies }

func (c *testExportContext) GetBundles() []*Bundle                            { return c.bundles }

func (c *testExportContext) GetFolders() []*Folder                            { return c.folders }

func (c *testExportContext) GetFiles() []*File                                { return c.files }

func (c *testExportContext) GetSections() []*Section                          { return c.sections }

func (c *testExportContext) GetDefinitions() []*Definition                    { return c.definitions }

func (c *testExportContext) GetCheckpoints(limit *int) ([]*Checkpoint, error) { return nil, nil }

func (c *testExportContext) GetContributors() ([]*Contributor, error)         { return nil, nil }

func (c *testExportContext) GetGoals() ([]*Goal, error)                       { return nil, nil }

func (c *testExportContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return nil, nil
}

func (c *testExportContext) GetPolicies() []*Policy                        { return nil }

func (c *testExportContext) GetDrafts() ([]*Draft, error)                  { return nil, nil }

func (c *testExportContext) GetTodos(filter *FilterInput) ([]*Todo, error) { return nil, nil }

func (c *testExportContext) GetStatutes() []*StatuteMeta                   { return nil }

func (c *testExportContext) Analyze(scope *string) (*AnalyzeResult, error) {
	return &AnalyzeResult{}, nil
}

func (c *testExportContext) Fix(scope *string) (*FixResult, error)                 { return &FixResult{}, nil }

func (c *testExportContext) GoalCreate(input GoalCreateInput) (*Goal, error)       { return nil, nil }

func (c *testExportContext) GoalChange(input GoalChangeInput) (*Goal, error)       { return nil, nil }

func (c *testExportContext) GoalClose(input GoalCloseInput) (*Goal, error)         { return nil, nil }

func (c *testExportContext) GoalReopen(input GoalReopenInput) (*Goal, error)       { return nil, nil }

func (c *testExportContext) GoalDelete(input GoalDeleteInput) (bool, error)        { return false, nil }

func (c *testExportContext) TodoCreate(input TodoCreateInput) (*Todo, error)       { return nil, nil }

func (c *testExportContext) TodoChange(input TodoChangeInput) (*Todo, error)       { return nil, nil }

func (c *testExportContext) TodoDelete(id string) (bool, error)                    { return false, nil }

func (c *testExportContext) DraftCreate(input DraftCreateInput) (*Draft, error)    { return nil, nil }

func (c *testExportContext) DraftDelete(id string) (bool, error)                   { return false, nil }

func (c *testExportContext) TicketOpen(input TicketOpenInput) (*Ticket, error)     { return nil, nil }

func (c *testExportContext) TicketClose(input TicketCloseInput) (*Ticket, error)   { return nil, nil }

func (c *testExportContext) TicketReopen(input TicketReopenInput) (*Ticket, error) { return nil, nil }

func (c *testExportContext) TicketChange(input TicketChangeInput) (*Ticket, error) { return nil, nil }

func (c *testExportContext) TicketDelete(input TicketDeleteInput) (bool, error)    { return false, nil }

func (c *testExportContext) FolderCreate(path string) (*Folder, error)             { return nil, nil }

func (c *testExportContext) FolderMove(src, dst string) (*Folder, error)           { return nil, nil }

func (c *testExportContext) FolderDelete(path string) error                        { return nil }

func (c *testExportContext) FileCreate(path string) (*File, error)                 { return nil, nil }

func (c *testExportContext) FileMove(src, dst string) (*File, error)               { return nil, nil }

func (c *testExportContext) FileDelete(path string) error                          { return nil }

func (c *testExportContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}

func (c *testExportContext) SectionDelete(file, name string) error { return nil }

func (c *testExportContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	return nil, nil
}

func (c *testExportContext) Extract(sourceFile, sourceSection, targetFile *string) (*File, error) {
	return nil, nil
}

func (c *testExportContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *testExportContext) ContributorRemove(github string) error { return nil }

func (c *testExportContext) SyncManagement() (bool, error)         { return false, nil }

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

// #endregion 📤️Event Export

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
		technologies: []*Technology{
			{Name: "mytechnology", Root: "mytechnology", Kind: TechnologyKindUser},
		},
		bundles: []*Bundle{
			{Name: "mytechnology/mybundle", Root: "mytechnology/mybundle", TechnologyName: "mytechnology", Kind: BundleKindLibrary},
		},
		folders: []*Folder{
			{Path: "mytechnology", Name: "mytechnology", Kind: FolderKindOrganization},
			{Path: "mytechnology/mybundle", Name: "mybundle", Kind: FolderKindOrganization},
			{Path: "mytechnology/mybundle/src", Name: "src", Kind: FolderKindOrganization},
		},
		files: []*File{
			{Path: "mytechnology/mybundle/src/app.ts", Name: "app.ts", Extension: "ts", Kind: FileKindCode},
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
	events, err := (eventstore.Store{Path: outputPath}).Replay(context.Background(), nil)
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

func TestPostgresSchemaIncludesKitVersionControlTables(t *testing.T) {
	rootDir := findTestRepoRoot(".")
	schemaPath := filepath.Join(rootDir, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🖥️server", "🧬️schema", "🐘️postgres", "🗄️.sql")
	data, err := os.ReadFile(schemaPath)
	if err != nil {
		t.Fatalf("failed to read postgres schema: %v", err)
	}
	schema := string(data)
	requiredSnippets := []string{
		"CREATE TABLE IF NOT EXISTS kits (",
		"CREATE TABLE IF NOT EXISTS kit_snapshots (",
		"CREATE TABLE IF NOT EXISTS kit_checkpoints (",
		"CREATE TABLE IF NOT EXISTS kit_alternatives (",
		"CREATE TABLE IF NOT EXISTS kit_sessions (",
		"CREATE TABLE IF NOT EXISTS kit_drafts (",
		"CREATE TABLE IF NOT EXISTS kit_transactions (",
		"CREATE TABLE IF NOT EXISTS kit_releases (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_families (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_kind_entities (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layouts (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layout_pieces (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_layout_connections (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_properties (",
		"CREATE TABLE IF NOT EXISTS kit_snapshot_attributes (",
		"CREATE UNIQUE INDEX IF NOT EXISTS idx_kit_transactions_one_open_per_draft ON kit_transactions(draft_id) WHERE state = 'open';",
		"source_json          JSONB NOT NULL DEFAULT '{}'",
		"before_snapshot_json  JSONB NOT NULL DEFAULT '{}'",
	}
	for _, snippet := range requiredSnippets {
		if !strings.Contains(schema, snippet) {
			t.Errorf("postgres schema missing snippet %q", snippet)
		}
	}

	if !strings.Contains(schema, "snapshot_kind        TEXT NOT NULL CHECK (snapshot_kind IN ('initial', 'materialized', 'draft-base', 'session-cache'))") {
		t.Error("expected snapshot kind check constraint in postgres schema")
	}
	if !strings.Contains(schema, "state       TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'finalized', 'aborted'))") {
		t.Error("expected transaction state check constraint in postgres schema")
	}
}

func TestExportToEventLogEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	ctx := &testExportContext{
		rootDir:      tmpDir,
		technologies: []*Technology{},
		bundles:      []*Bundle{},
		folders:      []*Folder{},
		files:        []*File{},
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
