package launcher

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"testing"
	"unicode/utf16"
)

func TestWorkspaceLinksPublishNeverReplacesAnotherTarget(t *testing.T) {
	root := launcherSandbox(t)
	one, two := filepath.Join(root, "one"), filepath.Join(root, "two")
	for _, path := range []string{one, two} {
		if err := os.Mkdir(path, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	stage, destination := filepath.Join(root, "stage"), filepath.Join(root, "destination")
	if err := createWorkspaceDirectoryLink(one, stage); err != nil {
		t.Fatal(err)
	}
	if err := createWorkspaceDirectoryLink(two, destination); err != nil {
		t.Fatal(err)
	}
	if err := publishWorkspaceDirectoryLink(stage, destination); err == nil {
		t.Fatal("racing destination was overwritten")
	}
	actual, err := os.Stat(destination)
	if err != nil {
		t.Fatal(err)
	}
	expected, err := os.Stat(two)
	if err != nil {
		t.Fatal(err)
	}
	if !os.SameFile(actual, expected) {
		t.Fatal("racing destination changed")
	}
}

func TestWorkspaceLinksRejectLinkedManifest(t *testing.T) {
	root, outside := launcherSandbox(t), launcherSandbox(t)
	source := filepath.Join(outside, "package.json")
	if err := os.WriteFile(source, []byte(`{"name":"outside"}`), 0o644); err != nil {
		t.Fatal(err)
	}
	link := filepath.Join(root, "package.json")
	if err := os.Symlink(source, link); err != nil {
		if runtime.GOOS == "windows" {
			t.Skip("file symlink creation requires developer mode on this host")
		}
		t.Fatal(err)
	}
	var manifest any
	if err := readWorkspaceManifest(link, 4096, &manifest); err == nil {
		t.Fatal("linked manifest accepted")
	}
}

type workspaceMutationContext struct {
	context.Context
	probe func() error
}

func (c workspaceMutationContext) Err() error { return c.probe() }

func TestWorkspaceLinksRecheckParentBeforePublication(t *testing.T) {
	root, outside := launcherSandbox(t), launcherSandbox(t)
	policy := workspaceLinksPolicy(t)
	workspaceLinksFixture(t, root, "p/one", "@example/one")
	if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["p/one"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	parent, moved := filepath.Join(root, "node_modules/@example"), filepath.Join(outside, "scope")
	mutated := false
	ctx := workspaceMutationContext{context.Background(), func() error {
		if mutated {
			return nil
		}
		entries, _ := os.ReadDir(parent)
		for _, entry := range entries {
			if !strings.HasPrefix(entry.Name(), ".repo-link-") {
				continue
			}
			if err := os.Rename(parent, moved); err != nil {
				return err
			}
			if err := createWorkspaceDirectoryLink(moved, parent); err != nil {
				return err
			}
			mutated = true
			break
		}
		return nil
	}}
	if err := PrepareWorkspacePackageLinks(ctx, root, policy); err == nil || !mutated {
		t.Fatalf("parent swap accepted: mutated=%v error=%v", mutated, err)
	}
	if _, err := os.Lstat(filepath.Join(moved, "one")); !os.IsNotExist(err) {
		t.Fatalf("publication escaped the root: %v", err)
	}
}

const workspaceLinksBase = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️workspaces/"

func TestWorkspaceExistingAlias(t *testing.T) {
	root, policy := testRoot(t), workspaceLinksPolicy(t)
	plan, err := PlanWorkspacePackageLink(root, "🧰️framework/📦️packages/🟦️typescript", "@semio-tech/framework", policy)
	if err != nil {
		t.Fatal(err)
	}
	before, err := os.Lstat(plan.Destination)
	if err != nil {
		t.Fatal(err)
	}
	parentBefore, err := os.Stat(filepath.Dir(plan.Destination))
	if err != nil {
		t.Fatal(err)
	}
	namespace, err := ClassifyWorkspaceNamespace(root, policy)
	if err != nil {
		t.Fatal(err)
	}
	if namespace.Kind == "managed" {
		if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err != nil {
			t.Fatal(err)
		}
	} else {
		matched, err := workspaceLinkMatches(plan)
		if err != nil || !matched {
			t.Fatalf("existing native Source alias is not recognized: %v", err)
		}
		if err := publishWorkspacePackageLink(context.Background(), root, plan); err != nil {
			t.Fatal(err)
		}
	}
	after, err := os.Lstat(plan.Destination)
	if err != nil {
		t.Fatal(err)
	}
	parentAfter, err := os.Stat(filepath.Dir(plan.Destination))
	if err != nil {
		t.Fatal(err)
	}
	if !os.SameFile(before, after) || before.Mode() != after.Mode() || before.Size() != after.Size() || !before.ModTime().Equal(after.ModTime()) || !parentBefore.ModTime().Equal(parentAfter.ModTime()) {
		t.Fatal("existing native Source alias was changed")
	}
}

func TestWorkspacePathUnits(t *testing.T) {
	var corpus struct {
		MaximumUnits int
		Cases        []struct {
			ID, Unit string
			Units    int
		}
	}
	readFixture(t, workspaceLinksBase+"🧫️fixtures/📏️paths.json", &corpus)
	policy := workspaceLinksPolicy(t)
	for _, row := range corpus.Cases {
		t.Run(row.ID, func(t *testing.T) {
			root := launcherSandbox(t)
			if len(utf16.Encode([]rune(row.Unit))) != row.Units {
				t.Fatal("neutral UTF-16 unit count differs")
			}
			base := len(utf16.Encode([]rune(filepath.Join(root, "p/x/package.json")))) - 1
			remaining := corpus.MaximumUnits - base
			workspace := "p/" + strings.Repeat(row.Unit, remaining/row.Units) + strings.Repeat("x", remaining%row.Units)
			plan, err := PlanWorkspacePackageLink(root, workspace, "one", policy)
			if err != nil {
				t.Fatal(err)
			}
			if len(utf16.Encode([]rune(filepath.Join(plan.Source, "package.json")))) != corpus.MaximumUnits {
				t.Fatal("boundary fixture is not exact")
			}
			if _, err := PlanWorkspacePackageLink(root, workspace+row.Unit, "one", policy); err == nil {
				t.Fatal("over-limit supplementary path admitted")
			}
		})
	}
}

func workspaceLinksPolicy(t *testing.T) WorkspaceLinksPolicy {
	t.Helper()
	policy, err := ReadWorkspaceLinksPolicy(testRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	return policy
}

func workspaceLinksFixture(t *testing.T, root, workspace, name string) {
	t.Helper()
	directory := filepath.Join(root, filepath.FromSlash(workspace))
	if err := os.MkdirAll(directory, 0o755); err != nil {
		t.Fatal(err)
	}
	manifest, _ := json.Marshal(map[string]any{"name": name, "type": "module", "exports": "./index.js"})
	if err := os.WriteFile(filepath.Join(directory, "package.json"), manifest, 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(directory, "index.js"), []byte("export default 42;\n"), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestWorkspaceLinksNeutralCorpus(t *testing.T) {
	var rows []struct {
		ID, Workspace, Name string
		Valid               bool
	}
	readFixture(t, workspaceLinksBase+"🧫️fixtures/🔣️.json", &rows)
	policy := workspaceLinksPolicy(t)
	for _, row := range rows {
		t.Run(row.ID, func(t *testing.T) {
			_, err := PlanWorkspacePackageLink(launcherSandbox(t), row.Workspace, row.Name, policy)
			if (err == nil) != row.Valid {
				t.Fatalf("valid=%v error=%v", row.Valid, err)
			}
		})
	}
}

func TestWorkspaceLinksColdImportAndConcurrentIdempotence(t *testing.T) {
	root, policy := launcherSandbox(t), workspaceLinksPolicy(t)
	workspaceLinksFixture(t, root, "p/one", "@example/one")
	if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["p/one"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(root, "node_modules")); !os.IsNotExist(err) {
		t.Fatalf("fixture is not cold: %v", err)
	}
	var peers sync.WaitGroup
	for i := 0; i < 4; i++ {
		peers.Add(1)
		go func() {
			defer peers.Done()
			if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err != nil {
				t.Error(err)
			}
		}()
	}
	peers.Wait()
	registry := loadRegistry(t)
	platform, arch, musl := HostTarget()
	rt := registry.Runtimes[registry.Selected]
	request := ProvisionRequest{Root: testRoot(t), Name: registry.Selected, Registry: registry, Runtime: rt, Platform: platform, Arch: arch, Musl: musl, Env: os.Getenv}
	installed, present, err := request.Locate()
	if err != nil || !present {
		t.Fatalf("registered test runtime required: present=%v error=%v", present, err)
	}
	script := filepath.Join(root, "cold.mjs")
	if err := os.WriteFile(script, []byte(`import value from "@example/one";console.log(value);`), 0o644); err != nil {
		t.Fatal(err)
	}
	child := exec.Command(installed.Executable, script)
	child.Dir = root
	output, err := child.CombinedOutput()
	if err != nil || strings.TrimSpace(string(output)) != "42" {
		t.Fatalf("cold workspace import: %v %s", err, output)
	}
	if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err != nil {
		t.Fatal(err)
	}
}

func TestWorkspaceLinksRefuseCollisionsAndCancellation(t *testing.T) {
	policy := workspaceLinksPolicy(t)
	for _, duplicate := range []bool{false, true} {
		root := launcherSandbox(t)
		workspaceLinksFixture(t, root, "p/one", "one")
		manifest := `{"workspaces":["p/one"]}`
		if duplicate {
			workspaceLinksFixture(t, root, "p/two", "one")
			manifest = `{"workspaces":["p/one","p/two"]}`
		} else {
			if err := os.MkdirAll(filepath.Join(root, "node_modules/one"), 0o755); err != nil {
				t.Fatal(err)
			}
		}
		if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(manifest), 0o644); err != nil {
			t.Fatal(err)
		}
		if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
			t.Fatal("occupied or duplicated workspace package accepted")
		}
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if err := PrepareWorkspacePackageLinks(ctx, launcherSandbox(t), policy); err != context.Canceled {
		t.Fatalf("cancellation: %v", err)
	}
}

func TestWorkspaceLinksPreparedBeforeRuntimeHandoff(t *testing.T) {
	setup := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "en"}, true)
	prepared := false
	setup.host.Prepare = func(_ context.Context, root string) error { prepared = root == setup.root; return nil }
	setup.host.HandOff = func(string, []string, []string) error {
		if !prepared {
			t.Fatal("runtime imported before workspace links")
		}
		return nil
	}
	if code := setup.host.Execute(context.Background(), setup.root, "build", []string{"--help"}); code != 0 {
		t.Fatalf("code=%d notices=%q", code, setup.log.err)
	}
	setup.host.Prepare = func(context.Context, string) error { return errors.New("workspace collision") }
	setup.host.HandOff = func(string, []string, []string) error { t.Fatal("runtime started after workspace refusal"); return nil }
	if code := setup.host.Execute(context.Background(), setup.root, "build", []string{"--help"}); code == 0 {
		t.Fatal("workspace collision accepted")
	}
	setup.host.Prepare = func(context.Context, string) error { return context.Canceled }
	if code := setup.host.Execute(context.Background(), setup.root, "build", []string{"--help"}); code != 130 {
		t.Fatalf("workspace cancellation code=%d", code)
	}
}

func TestWorkspaceLinksValidateWholeRosterBeforePublishing(t *testing.T) {
	policy := workspaceLinksPolicy(t)
	for _, roster := range [][]string{{"p/one", "p/one"}, {"p/one", "p/two"}} {
		root := launcherSandbox(t)
		workspaceLinksFixture(t, root, "p/one", "one")
		workspaceLinksFixture(t, root, "p/two", "ONE")
		manifest, _ := json.Marshal(map[string]any{"workspaces": roster})
		if err := os.WriteFile(filepath.Join(root, "package.json"), manifest, 0o644); err != nil {
			t.Fatal(err)
		}
		if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
			t.Fatal("colliding roster accepted")
		}
		if _, err := os.Lstat(filepath.Join(root, "node_modules")); !os.IsNotExist(err) {
			t.Fatalf("invalid roster wrote links: %v", err)
		}
	}
	root := launcherSandbox(t)
	policy.MaximumWorkspaces = 1
	if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["a","b"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
		t.Fatal("unbounded workspace roster accepted")
	}
}

func TestWorkspaceLinksRefuseEscapedTargetsAndExistingLinks(t *testing.T) {
	policy := workspaceLinksPolicy(t)
	for _, kind := range []string{"source", "scope", "wrong", "dangling"} {
		root, outside := launcherSandbox(t), launcherSandbox(t)
		workspaceLinksFixture(t, root, "p/one", "@example/one")
		if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["p/one"]}`), 0o644); err != nil {
			t.Fatal(err)
		}
		switch kind {
		case "source":
			if err := os.RemoveAll(filepath.Join(root, "p")); err != nil {
				t.Fatal(err)
			}
			workspaceLinksFixture(t, outside, "one", "@example/one")
			if err := createWorkspaceDirectoryLink(outside, filepath.Join(root, "p")); err != nil {
				t.Fatal(err)
			}
		case "scope":
			if err := os.MkdirAll(filepath.Join(root, "node_modules"), 0o755); err != nil {
				t.Fatal(err)
			}
			if err := createWorkspaceDirectoryLink(outside, filepath.Join(root, "node_modules/@example")); err != nil {
				t.Fatal(err)
			}
		default:
			if err := os.MkdirAll(filepath.Join(root, "node_modules/@example"), 0o755); err != nil {
				t.Fatal(err)
			}
			if err := createWorkspaceDirectoryLink(outside, filepath.Join(root, "node_modules/@example/one")); err != nil {
				t.Fatal(err)
			}
			if kind == "dangling" {
				if err := os.Remove(outside); err != nil {
					t.Fatal(err)
				}
			}
		}
		if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
			t.Fatalf("%s escape accepted", kind)
		}
	}
}

type workspaceCancelAfterLinkContext struct {
	context.Context
	link string
}

func (c workspaceCancelAfterLinkContext) Err() error {
	if _, err := os.Lstat(c.link); err == nil {
		return context.Canceled
	}
	return nil
}

func TestWorkspaceLinksCancelBetweenPublicationsAndBoundManifestReads(t *testing.T) {
	root, policy := launcherSandbox(t), workspaceLinksPolicy(t)
	workspaceLinksFixture(t, root, "p/one", "one")
	workspaceLinksFixture(t, root, "p/two", "two")
	if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["p/one","p/two"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	ctx := workspaceCancelAfterLinkContext{context.Background(), filepath.Join(root, "node_modules/one")}
	if err := PrepareWorkspacePackageLinks(ctx, root, policy); err != context.Canceled {
		t.Fatalf("mid-roster cancellation: %v", err)
	}
	if _, err := os.Lstat(filepath.Join(root, "node_modules/two")); !os.IsNotExist(err) {
		t.Fatalf("link published after cancellation: %v", err)
	}
	policy.MaximumManifestBytes = 8
	if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
		t.Fatal("unbounded manifest read accepted")
	}
}

func TestWorkspaceLinksRequireDeclaredArrayRoster(t *testing.T) {
	policy := workspaceLinksPolicy(t)
	for _, manifest := range []string{`{}`, `{"workspaces":null}`, `{"workspaces":"p"}`} {
		root := launcherSandbox(t)
		if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(manifest), 0o644); err != nil {
			t.Fatal(err)
		}
		if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
			t.Fatalf("undeclared roster accepted: %s", manifest)
		}
	}
}
