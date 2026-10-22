// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket plan resolution, movement, and comment posting.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestResolvePlanSourceRefusesCursorPlanID(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	if err := os.WriteFile(filepath.Join(planDir, "kit_store_"+id+".plan.md"), []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	if got, _, err := ResolvePlanSource(McpClientCursor, id); err == nil {
		t.Fatalf("a client directory of the repository must not resolve a plan, got %q", got)
	}
}

func TestResolvePlanSourceKiroSpecID(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, ".kiro", "specs", "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "design.md"), []byte("d"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	got, isDir, err := ResolvePlanSource(McpClientKiro, "my-spec")
	if err != nil {
		t.Fatal(err)
	}
	if !isDir {
		t.Fatal("expected directory")
	}
	if filepath.Clean(got) != filepath.Clean(specDir) {
		t.Fatalf("got %q want %q", got, specDir)
	}
}

func TestMoveTicketPlanIntoFolderFile(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	src := filepath.Join(tmp, "outside.md")
	if err := os.WriteFile(src, []byte("body"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &Ticket{FolderPath: ticketDir, Plan: &TicketPlan{Source: src, Client: "cursor", ID: "x"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	dst := filepath.Join(ticketDir, "outside.md")
	if _, err := os.Stat(dst); err != nil {
		t.Fatal(err)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "outside.md" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestMoveTicketPlanIntoFolderSpecDir(t *testing.T) {
	tmp := t.TempDir()
	ticketDir := filepath.Join(tmp, "ticket")
	if err := os.MkdirAll(ticketDir, 0o755); err != nil {
		t.Fatal(err)
	}
	specRoot := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(filepath.Join(specRoot, "nested"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "a.md"), []byte("a"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specRoot, "nested", "b.md"), []byte("b"), 0o644); err != nil {
		t.Fatal(err)
	}
	ticket := &Ticket{FolderPath: ticketDir, Plan: &TicketPlan{Source: specRoot, Client: "kiro", ID: "my-spec"}}
	if err := moveTicketPlanIntoFolder(ticket); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(specRoot); !os.IsNotExist(err) {
		t.Fatalf("source spec dir should be gone: %v", err)
	}
	destRoot := filepath.Join(ticketDir, "my-spec")
	if b, err := os.ReadFile(filepath.Join(destRoot, "a.md")); err != nil || string(b) != "a" {
		t.Fatalf("a.md: %v %q", err, b)
	}
	if b, err := os.ReadFile(filepath.Join(destRoot, "nested", "b.md")); err != nil || string(b) != "b" {
		t.Fatalf("nested/b.md: %v %q", err, b)
	}
	if ticket.Plan.Source != "" {
		t.Fatalf("expected empty Source, got %q", ticket.Plan.Source)
	}
	if ticket.Plan.Local != "my-spec" {
		t.Fatalf("Local = %q", ticket.Plan.Local)
	}
}

func TestApplyTicketPlanFromIDsRefusesCursor(t *testing.T) {
	tmp := t.TempDir()
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	ticket := &Ticket{}
	if err := ApplyTicketPlanFromIDs(ticket, McpClientCursor, "fe75d494", ""); err == nil {
		t.Fatal("cursor carries no plan attachment")
	}
	if ticket.Plan != nil {
		t.Fatalf("plan: %+v", ticket.Plan)
	}
}

func TestStripPlanFrontmatter(t *testing.T) {
	raw := "---\nname: Test\noverview: x\n---\n\n# Body\n"
	got := stripPlanFrontmatter(raw)
	if got != "# Body" {
		t.Fatalf("got %q", got)
	}
	if stripPlanFrontmatter("# No frontmatter") != "# No frontmatter" {
		t.Fatal("expected unchanged content without frontmatter")
	}
}

func TestFormatPlanCommentFile(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "feature_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Feature\n---\n\n## Steps\n\nDo work.\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&TicketPlan{Client: "cursor", ID: "abcd1234"}, planPath)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(body, "# 📋️ Plan") {
		t.Fatalf("missing heading: %q", body)
	}
	if !strings.Contains(body, "<details>") || !strings.Contains(body, "feature_abcd1234.plan.md") {
		t.Fatalf("missing details block: %q", body)
	}
	if strings.Contains(body, "name: Feature") {
		t.Fatalf("frontmatter should be stripped: %q", body)
	}
	if !strings.Contains(body, "## Steps") {
		t.Fatalf("missing body: %q", body)
	}
}

func TestFormatPlanCommentSpecDir(t *testing.T) {
	tmp := t.TempDir()
	specDir := filepath.Join(tmp, "my-spec")
	if err := os.MkdirAll(specDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "b.md"), []byte("## B\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(specDir, "a.md"), []byte("## A\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	body, err := formatPlanComment(&TicketPlan{Client: "kiro", ID: "my-spec"}, specDir)
	if err != nil {
		t.Fatal(err)
	}
	aPos := strings.Index(body, "a.md")
	bPos := strings.Index(body, "b.md")
	if aPos < 0 || bPos < 0 || aPos > bPos {
		t.Fatalf("expected sorted sections a before b: %q", body)
	}
}

type captureCommentProvider struct {
	NullManagementProvider
	comments []struct {
		url  string
		body string
	}
}

func (p *captureCommentProvider) AddComment(issueURL, comment string) error {
	p.comments = append(p.comments, struct {
		url  string
		body string
	}{issueURL, comment})
	return nil
}

func TestPostTicketPlanComment(t *testing.T) {
	tmp := t.TempDir()
	planPath := filepath.Join(tmp, "task_abcd1234.plan.md")
	if err := os.WriteFile(planPath, []byte("---\nname: Task\n---\n\n## Work\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	capture := &captureCommentProvider{}
	old := mgmtProvider
	mgmtProvider = capture
	defer func() { mgmtProvider = old }()

	ticket := &Ticket{
		Management: &TicketManagementData{Issue: "https://github.com/example/repo/issues/1"},
		Plan:       &TicketPlan{Source: planPath, Client: "cursor", ID: "abcd1234"},
	}
	postTicketPlanComment(ticket, false)
	if len(capture.comments) != 1 {
		t.Fatalf("expected 1 comment, got %d", len(capture.comments))
	}
	if !strings.Contains(capture.comments[0].body, "# 📋️ Plan") {
		t.Fatalf("missing plan heading: %q", capture.comments[0].body)
	}
	if !strings.Contains(capture.comments[0].body, "## Work") {
		t.Fatalf("missing plan body: %q", capture.comments[0].body)
	}
	postTicketPlanComment(ticket, true)
	if len(capture.comments) != 1 {
		t.Fatalf("noManagement should skip comment, got %d", len(capture.comments))
	}
}

func TestPostTicketPlanCommentLive(t *testing.T) {
	if os.Getenv("REPO_LIVE_GH") != "1" {
		t.Skip("set REPO_LIVE_GH=1 to run live GitHub plan comment test")
	}
	planFile := filepath.Join(rootDir, filepath.FromSlash(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️22/ARCHIVED-CURSOR-PLANS-WITHOUT-TICKET/post_plan_to_issue_c256e28e.plan.md"))
	if _, err := os.Stat(planFile); err != nil {
		t.Skip("plan file missing")
	}
	issueURL, err := ghCreateIssue("TEST Post Plan Comment", "# temp", nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = ghCloseIssue(issueURL) })
	ticket := &Ticket{
		Management: &TicketManagementData{Issue: issueURL},
		Plan:       &TicketPlan{Source: planFile, Client: "cursor", ID: "c256e28e"},
	}
	postTicketPlanComment(ticket, false)
	issueNum := issueURL[strings.LastIndex(issueURL, "/")+1:]
	if issueNum == "" {
		t.Fatal("could not parse issue number")
	}
	stdout, stderr, code := ExecCommand("gh", []string{"api", "repos/{owner}/{repo}/issues/" + issueNum + "/comments", "--jq", ".[].body"}, "")
	if code != 0 {
		t.Fatalf("gh api comments failed: %s", stderr)
	}
	if !strings.Contains(stdout, "# 📋️ Plan") {
		t.Fatalf("issue comment missing plan heading: %q", stdout)
	}
	if !strings.Contains(stdout, "post_plan_to_issue_c256e28e.plan.md") {
		t.Fatalf("issue comment missing plan filename: %q", stdout)
	}
}
