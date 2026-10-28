// 🧹️ The purge-generated law: real files in a throwaway git repository outside the checkout reproduce the plan that the independent Python model froze for every case, a dry run changes nothing, and an apply removes exactly the planned paths.
// @see ../🧫️fixtures/🧹️purge-generated/🧹️cases.json
// @see ./🐍️.py

package tickets

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	reflect "reflect"
	sort "sort"
	strings "strings"
	testing "testing"
	time "time"

	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

type purgeEntry struct {
	Path       string `json:"path"`
	Kind       string `json:"kind"`
	Bytes      int64  `json:"bytes"`
	AgeSeconds int64  `json:"ageSeconds"`
	Tracked    bool   `json:"tracked"`
	Target     string `json:"target"`
}

type purgeTicket struct {
	ID       string       `json:"id"`
	Manifest string       `json:"manifest"`
	Entries  []purgeEntry `json:"entries"`
}

type purgeRequest struct {
	Scope     string `json:"scope"`
	Ticket    string `json:"ticket"`
	Selection string `json:"selection"`
}

type purgeCase struct {
	ID      string        `json:"id"`
	Git     bool          `json:"git"`
	Request purgeRequest  `json:"request"`
	Tickets []purgeTicket `json:"tickets"`
	Expect  PurgePlan     `json:"expect"`
}

type purgeFixture struct {
	Policy struct {
		GeneratedDirectory   string `json:"generatedDirectory"`
		OversizedFileBytes   int64  `json:"oversizedFileBytes"`
		OversizedFolderBytes int64  `json:"oversizedFolderBytes"`
		LiveWriterSeconds    int64  `json:"liveWriterSeconds"`
	} `json:"policy"`
	Cases []purgeCase `json:"cases"`
}

type purgeWorld struct {
	root, ticketsDir, external string
}

func loadPurgeFixture(t *testing.T) purgeFixture {
	t.Helper()
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "🧪️tests", "🧫️fixtures", "🧹️purge-generated", "🧹️cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture purgeFixture
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	return fixture
}

func purgeGit(t *testing.T, root string, stdin []byte, args ...string) {
	t.Helper()
	command := workspace.ToolCommand("git", append([]string{"-C", root}, args...)...)
	command.Env = gitQueryEnvironment(command.Env, root)
	command.Stdin = bytes.NewReader(stdin)
	if output, err := command.CombinedOutput(); err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, output)
	}
}

func materializePurgeCase(t *testing.T, scratch string, c purgeCase, now time.Time) purgeWorld {
	t.Helper()
	root, err := os.MkdirTemp(scratch, "repo-")
	if err != nil {
		t.Fatal(err)
	}
	external, err := os.MkdirTemp(scratch, "external-")
	if err != nil {
		t.Fatal(err)
	}
	writePurgeFile(t, filepath.Join(external, "blob.bin"), 11<<20, now.Add(-24*time.Hour))
	if c.Git {
		purgeGit(t, root, nil, "init", "-q")
	}
	world := purgeWorld{root: root, ticketsDir: filepath.Join(root, ".🧬semio", "🦑️repo", "🎫️tickets"), external: external}
	var tracked []string
	for _, ticket := range c.Tickets {
		dir := filepath.Join(world.ticketsDir, filepath.FromSlash(ticket.ID))
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Fatal(err)
		}
		manifests := map[string]string{"open": `{"status":"open"}`, "closed": `{"status":"closed"}`, "closing": `{"status":"closing"}`, "invalid": `{`}
		if text, found := manifests[ticket.Manifest]; found {
			if err := os.WriteFile(filepath.Join(dir, "🎫️ticket.json"), []byte(text), 0o644); err != nil {
				t.Fatal(err)
			}
		}
		for _, entry := range ticket.Entries {
			path := filepath.Join(dir, filepath.FromSlash(entry.Path))
			if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
				t.Fatal(err)
			}
			switch entry.Kind {
			case "directory":
				if err := os.MkdirAll(path, 0o755); err != nil {
					t.Fatal(err)
				}
			case "symlink":
				if err := os.Symlink(filepath.Join(external, filepath.FromSlash(strings.TrimPrefix(entry.Target, "external"))), path); err != nil {
					t.Skipf("platform does not permit symbolic links: %v", err)
				}
			default:
				writePurgeFile(t, path, entry.Bytes, now.Add(-time.Duration(entry.AgeSeconds)*time.Second))
				if entry.Tracked {
					tracked = append(tracked, filepath.ToSlash(mustRelative(root, path)))
				}
			}
		}
	}
	if len(tracked) > 0 {
		purgeGit(t, root, []byte(strings.Join(tracked, "\x00")), "add", "-f", "--pathspec-from-file=-", "--pathspec-file-nul")
	}
	return world
}

func writePurgeFile(t *testing.T, path string, size int64, modified time.Time) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	file, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := file.Truncate(size); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	if err := os.Chtimes(path, modified, modified); err != nil {
		t.Fatal(err)
	}
}

func purgeTree(t *testing.T, root string) map[string]string {
	t.Helper()
	tree := map[string]string{}
	err := filepath.WalkDir(root, func(path string, entry fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		relative := filepath.ToSlash(mustRelative(root, path))
		if relative == ".git" || strings.HasPrefix(relative, ".git/") {
			if entry.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		info, err := entry.Info()
		if err != nil {
			return err
		}
		size := int64(0)
		if info.Mode().IsRegular() {
			size = info.Size()
		}
		tree[relative] = fmt.Sprintf("%s|%d|%s", info.Mode().Type(), size, info.ModTime().UTC().Format(time.RFC3339Nano))
		return nil
	})
	if err != nil {
		t.Fatal(err)
	}
	return tree
}

func purgeRequestOptions(c purgeCase, world purgeWorld, now time.Time, apply bool) PurgeOptions {
	return PurgeOptions{Root: world.root, TicketsDir: world.ticketsDir, Selection: PurgeSelection(c.Request.Selection), Apply: apply, Now: now}
}

func runPurgeCase(c purgeCase, world purgeWorld, now time.Time, apply bool) (PurgePlan, error) {
	options := purgeRequestOptions(c, world, now, apply)
	if c.Request.Scope == "ticket" {
		return PurgeTicketGeneratedOutput(options, []string{filepath.Join(world.ticketsDir, filepath.FromSlash(c.Request.Ticket))})
	}
	return PurgeClosedTicketGeneratedOutput(options)
}

func normalizePurgePlan(plan PurgePlan) PurgePlan {
	if plan.Removals == nil {
		plan.Removals = []PurgeRemoval{}
	}
	if plan.Skips == nil {
		plan.Skips = []PurgeSkip{}
	}
	return plan
}

func TestPurgePolicyMatchesTheSchemaFixture(t *testing.T) {
	fixture := loadPurgeFixture(t)
	if fixture.Policy.GeneratedDirectory != TicketGeneratedDirectory || fixture.Policy.OversizedFileBytes != OversizedFileBytes || fixture.Policy.OversizedFolderBytes != OversizedFolderBytes || time.Duration(fixture.Policy.LiveWriterSeconds)*time.Second != LiveWriterWindow {
		t.Fatalf("purge policy differs from the frozen fixture policy: %+v", fixture.Policy)
	}
	if len(fixture.Cases) == 0 {
		t.Fatal("no purge cases")
	}
}

func TestPurgeReproducesEveryFrozenPlanFromRealFiles(t *testing.T) {
	fixture := loadPurgeFixture(t)
	scratch, err := os.MkdirTemp("", "purge-generated-")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = os.RemoveAll(scratch) })
	now := time.Now().Truncate(time.Second)
	for _, c := range fixture.Cases {
		t.Run(c.ID, func(t *testing.T) {
			world := materializePurgeCase(t, scratch, c, now)
			before := purgeTree(t, world.root)
			externalBefore := purgeTree(t, world.external)
			want := normalizePurgePlan(c.Expect)
			dry, err := runPurgeCase(c, world, now, false)
			if err != nil {
				t.Fatal(err)
			}
			if got := normalizePurgePlan(dry); !reflect.DeepEqual(got, want) {
				t.Fatalf("dry-run plan differs\n got: %+v\nwant: %+v", got, want)
			}
			if after := purgeTree(t, world.root); !reflect.DeepEqual(after, before) {
				t.Fatalf("a dry run changed the repository")
			}
			applied, err := runPurgeCase(c, world, now, true)
			if err != nil {
				t.Fatal(err)
			}
			if got := normalizePurgePlan(applied); !reflect.DeepEqual(got, want) {
				t.Fatalf("applied plan differs\n got: %+v\nwant: %+v", got, want)
			}
			remaining := map[string]string{}
			for path, state := range before {
				remaining[path] = state
			}
			for _, removal := range want.Removals {
				prefix := filepath.ToSlash(mustRelative(world.root, filepath.Join(world.ticketsDir, filepath.FromSlash(removal.Path))))
				for path := range remaining {
					if path == prefix || strings.HasPrefix(path, prefix+"/") {
						delete(remaining, path)
					}
				}
			}
			after := purgeTree(t, world.root)
			var missing, extra []string
			for path := range remaining {
				if _, found := after[path]; !found {
					missing = append(missing, path)
				}
			}
			for path := range after {
				if _, found := remaining[path]; !found {
					extra = append(extra, path)
				}
			}
			sort.Strings(missing)
			sort.Strings(extra)
			if len(missing) > 0 || len(extra) > 0 {
				t.Fatalf("apply removed or kept the wrong paths\n vanished: %v\n survived: %v", missing, extra)
			}
			if !reflect.DeepEqual(purgeTree(t, world.external), externalBefore) {
				t.Fatalf("a purge reached through a link into the external folder")
			}
		})
	}
}

func TestPurgeNeverAppliesWithoutAnExplicitApply(t *testing.T) {
	fixture := loadPurgeFixture(t)
	scratch, err := os.MkdirTemp("", "purge-default-")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = os.RemoveAll(scratch) })
	c := fixture.Cases[2]
	now := time.Now().Truncate(time.Second)
	world := materializePurgeCase(t, scratch, c, now)
	before := purgeTree(t, world.root)
	if _, err := PurgeClosedTicketGeneratedOutput(PurgeOptions{Root: world.root, TicketsDir: world.ticketsDir, Now: now}); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(purgeTree(t, world.root), before) {
		t.Fatal("the default purge changed the repository")
	}
}

func TestPurgeLeavesAMissingTicketsFolderAlone(t *testing.T) {
	plan, err := PurgeClosedTicketGeneratedOutput(PurgeOptions{Root: t.TempDir(), TicketsDir: filepath.Join(t.TempDir(), "absent"), Apply: true})
	if err != nil || len(plan.Removals) != 0 || len(plan.Skips) != 0 {
		t.Fatalf("plan %+v err %v", plan, err)
	}
}
