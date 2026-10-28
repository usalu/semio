// 🏖️ Every test binary of this module runs against a repository root it owns, and the sandbox that enforces it is itself proven here.

package workspace_test

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"
	testing "testing"

	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestMain(m *testing.M) {
	os.Exit(sandbox.Run("workspace", nil, func() int {
		workspace.FixtureOrigin = sandbox.Origin()
		return m.Run()
	}))
}

func TestTestsRunAgainstAnOwnedRoot(t *testing.T) {
	for _, problem := range sandbox.Problems("🏖️sandbox") {
		t.Error(problem)
	}
}

func checkoutWithMarker(t *testing.T) string {
	t.Helper()
	checkout := t.TempDir()
	if err := os.MkdirAll(filepath.Join(checkout, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	return checkout
}

func TestRootGuardFlagsOnlyTheCheckoutRootItself(t *testing.T) {
	checkout := checkoutWithMarker(t)
	sibling := checkout + "-sibling"
	if err := os.MkdirAll(sibling, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { os.RemoveAll(sibling) })
	guard := sandbox.NewGuard(checkout)
	guard.Observe(sibling)
	guard.Observe(filepath.Join(os.TempDir(), "elsewhere"))
	if problems := guard.Report(nil); len(problems) != 0 {
		t.Fatalf("outside resolutions were flagged: %v", problems)
	}
	guard.Observe(checkout + string(filepath.Separator))
	problems := guard.Report(nil)
	if len(problems) != 1 || !strings.Contains(problems[0], "TestRootGuardFlagsOnlyTheCheckoutRootItself") {
		t.Fatalf("a resolution of the checkout root must be reported with the test that caused it, got %v", problems)
	}
}

func TestRootGuardIgnoresRootsNestedInTheCheckout(t *testing.T) {
	checkout := checkoutWithMarker(t)
	nested := filepath.Join(checkout, ".cache", "tests", "tmp", "rr-go-1-0")
	if err := os.MkdirAll(filepath.Join(nested, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	guard := sandbox.NewGuard(checkout)
	guard.Observe(nested)
	guard.Observe(filepath.Join(nested, "child"))
	if problems := guard.Report(map[string]string{"workspace root": nested, "alias": nested + string(filepath.Separator) + "." + string(filepath.Separator)}); len(problems) != 0 {
		t.Fatalf("a marked root nested in the checkout is a root of its own and must not be flagged: %v", problems)
	}
}

func TestRootGuardFlagsUnmarkedDirectoriesBelowTheCheckout(t *testing.T) {
	checkout := checkoutWithMarker(t)
	unmarked := filepath.Join(checkout, ".cache", "tests", "tmp", "foreign")
	if err := os.MkdirAll(unmarked, 0o755); err != nil {
		t.Fatal(err)
	}
	guard := sandbox.NewGuard(checkout)
	if problems := guard.Report(map[string]string{"other": unmarked}); len(problems) != 1 {
		t.Fatalf("a held directory below the checkout resolves to the checkout root and must be flagged, got %v", problems)
	}
}

func TestRootGuardFlagsHeldBindingsInsideTheCheckout(t *testing.T) {
	checkout := checkoutWithMarker(t)
	guard := sandbox.NewGuard(checkout)
	problems := guard.Report(map[string]string{"executor": checkout, "cache": filepath.Join(os.TempDir(), "owned")})
	if len(problems) != 1 || !strings.Contains(problems[0], "executor is bound to the real repository root") {
		t.Fatalf("only the held binding of the checkout must be reported, got %v", problems)
	}
}

func TestAbortingGuardPanicsBeforeAnythingIsWrittenToTheCheckout(t *testing.T) {
	checkout := checkoutWithMarker(t)
	nested := filepath.Join(checkout, "nested")
	if err := os.MkdirAll(filepath.Join(nested, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	guard := sandbox.NewGuard(checkout).AbortOnTouch()
	guard.Observe(nested)
	defer func() {
		message, _ := recover().(string)
		if !strings.Contains(message, "TestAbortingGuardPanicsBeforeAnythingIsWrittenToTheCheckout") {
			t.Fatalf("the guard must abort naming the test that resolved the checkout root, got %q", message)
		}
		if problems := guard.Report(nil); len(problems) != 1 {
			t.Fatalf("the aborting resolution must stay recorded, got %v", problems)
		}
	}()
	guard.Observe(checkout)
	t.Fatal("resolving the checkout root did not abort")
}

func TestBindRootMakesAGitRepositoryOfItsOwnBelowACheckout(t *testing.T) {
	checkout := t.TempDir()
	if output, err := workspace.ToolCommand("git", "-C", checkout, "init", "-q").CombinedOutput(); err != nil {
		t.Fatalf("git init: %v\n%s", err, output)
	}
	nested := filepath.Join(checkout, ".cache", "tests", "tmp", "case")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatal(err)
	}
	previous, _ := workspace.LookupRootDir()
	t.Cleanup(func() { workspace.SetRootDir(previous) })
	sandbox.BindRoot(t, nested)
	output, err := workspace.ToolCommand("git", "-C", nested, "rev-parse", "--show-toplevel").Output()
	if err != nil {
		t.Fatal(err)
	}
	if !sandbox.SameDirectory(strings.TrimSpace(string(output)), nested) {
		t.Fatalf("git must end in the bound root %s, it answered for %s", nested, output)
	}
	if bound, _ := workspace.LookupRootDir(); !sandbox.SameDirectory(bound, nested) {
		t.Fatalf("the workspace root must be %s, got %s", nested, bound)
	}
}

func realCheckout(t *testing.T) string {
	t.Helper()
	checkout := t.TempDir()
	if output, err := workspace.ToolCommand("git", "-C", checkout, "init", "-q").CombinedOutput(); err != nil {
		t.Fatalf("git init: %v\n%s", err, output)
	}
	return checkout
}

func TestRelocateScratchMovesTheScratchOutOfTheCheckout(t *testing.T) {
	checkout := realCheckout(t)
	issued := filepath.Join(checkout, ".cache", "tests", "tmp")
	if err := os.MkdirAll(issued, 0o755); err != nil {
		t.Fatal(err)
	}
	outside := t.TempDir()
	for _, variable := range []string{"TMPDIR", "TMP", "TEMP"} {
		t.Setenv(variable, issued)
	}
	restore := sandbox.RelocateScratch(checkout, filepath.Join(checkout, "inside"), outside)
	if restore == nil {
		t.Fatal("a scratch directory inside the checkout must be relocated")
	}
	moved := os.TempDir()
	if sandbox.PathWithin(checkout, moved) || !sandbox.PathWithin(outside, moved) {
		t.Fatalf("the scratch must lie below %s and outside %s, got %s", outside, checkout, moved)
	}
	fresh := t.TempDir()
	if !sandbox.SameDirectory(workspace.ResolveRepoRoot(fresh), fresh) {
		t.Fatalf("a test directory %s must be a root of its own once the scratch lies outside the checkout", fresh)
	}
	restore()
	if sandbox.PathWithin(outside, os.TempDir()) || !sandbox.SameDirectory(os.TempDir(), issued) {
		t.Fatalf("restoring must bring back the issued scratch %s, got %s", issued, os.TempDir())
	}
	if _, err := os.Stat(moved); !os.IsNotExist(err) {
		t.Fatalf("restoring must remove the relocated scratch %s", moved)
	}
}

func TestRelocateScratchLeavesAScratchOutsideTheCheckoutAlone(t *testing.T) {
	if restore := sandbox.RelocateScratch(realCheckout(t), t.TempDir()); restore != nil {
		restore()
		t.Fatal("a scratch directory outside the checkout must stay")
	}
}

func TestRelocateScratchSkipsBasesInsideARepository(t *testing.T) {
	checkout := realCheckout(t)
	issued := filepath.Join(checkout, ".cache", "tmp")
	if err := os.MkdirAll(issued, 0o755); err != nil {
		t.Fatal(err)
	}
	foreign := realCheckout(t)
	for _, variable := range []string{"TMPDIR", "TMP", "TEMP"} {
		t.Setenv(variable, issued)
	}
	if restore := sandbox.RelocateScratch(checkout, foreign, filepath.Join(t.TempDir(), "missing", "base")); restore != nil {
		restore()
		t.Fatal("a base below another repository would resolve test directories to that repository and must be skipped")
	}
	if !sandbox.SameDirectory(os.TempDir(), issued) {
		t.Fatalf("the issued scratch must stay when no base qualifies, got %s", os.TempDir())
	}
}

type sameDirectoryVectors struct {
	Cases []struct {
		Name string     `json:"name"`
		Ops  [][]string `json:"ops"`
		A    string     `json:"a"`
		B    string     `json:"b"`
		Same any        `json:"same"`
	} `json:"cases"`
}

func foldsLookup(t *testing.T, dir string, name string, other string) bool {
	t.Helper()
	if err := os.MkdirAll(filepath.Join(dir, name), 0o755); err != nil {
		t.Fatal(err)
	}
	_, err := os.Stat(filepath.Join(dir, other))
	return err == nil
}

func TestSameDirectoryAgreesWithThePythonOracleVectors(t *testing.T) {
	raw, err := os.ReadFile(sandbox.OriginPath("..", "..", "🧪️tests", "🧫️fixtures", "📡️same-directory-vectors.json"))
	if err != nil {
		t.Fatal(err)
	}
	var vectors sameDirectoryVectors
	if err := json.Unmarshal(raw, &vectors); err != nil {
		t.Fatal(err)
	}
	probe := t.TempDir()
	folds := map[string]bool{
		"folds-case":          foldsLookup(t, probe, "Probe", "pROBE"),
		"folds-normalization": foldsLookup(t, probe, "\u00e9", "e\u0301"),
	}
	for _, vector := range vectors.Cases {
		t.Run(vector.Name, func(t *testing.T) {
			base := t.TempDir()
			for _, op := range vector.Ops {
				target := filepath.Join(base, filepath.FromSlash(op[1]))
				if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
					t.Fatal(err)
				}
				switch op[0] {
				case "mkdir":
					err = os.MkdirAll(target, 0o755)
				case "file":
					err = os.WriteFile(target, nil, 0o644)
				default:
					if runtime.GOOS == "windows" {
						t.Skip("symbolic links need a privilege on Windows")
					}
					err = os.Symlink(filepath.Join(base, filepath.FromSlash(op[2])), target)
				}
				if err != nil {
					t.Fatal(err)
				}
			}
			expected, _ := vector.Same.(bool)
			if kind, named := vector.Same.(string); named {
				expected = folds[kind]
			}
			a, b := filepath.Join(base, filepath.FromSlash(vector.A)), filepath.Join(base, filepath.FromSlash(vector.B))
			if strings.HasSuffix(vector.B, "/") {
				b += string(filepath.Separator)
			}
			if got := sandbox.SameDirectory(a, b); got != expected {
				t.Errorf("SameDirectory(%q, %q) = %v, the oracle says %v", a, b, got, expected)
			}
			if got := sandbox.SameDirectory(b, a); got != expected {
				t.Errorf("SameDirectory is not symmetric for %q and %q: %v", a, b, got)
			}
		})
	}
}

func TestSameDirectoryRefusesEmptyPaths(t *testing.T) {
	if sandbox.SameDirectory("", "") || sandbox.SameDirectory(t.TempDir(), "") {
		t.Fatal("an empty path names no directory")
	}
}

func TestSameDirectoryIgnoresDriveLetterCaseOnWindows(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("drive letters exist on Windows only")
	}
	dir := t.TempDir()
	volume := filepath.VolumeName(dir)
	flipped := strings.ToLower(volume) + dir[len(volume):]
	if volume == flipped[:len(volume)] {
		flipped = strings.ToUpper(volume) + dir[len(volume):]
	}
	if !sandbox.SameDirectory(dir, flipped) {
		t.Fatalf("%q and %q are one directory", dir, flipped)
	}
}

func TestSettingTheRootInsideTheCheckoutIsObserved(t *testing.T) {
	checkout := t.TempDir()
	if err := os.MkdirAll(filepath.Join(checkout, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	guard := sandbox.NewGuard(checkout)
	previous, _ := workspace.LookupRootDir()
	stop := workspace.ObserveRootResolution(guard.Observe)
	t.Cleanup(func() {
		workspace.SetRootDir(previous)
	})
	workspace.SetRootDir(checkout)
	stop()
	if problems := guard.Report(nil); len(problems) < 2 {
		t.Fatalf("the resolution and the binding inside the checkout must both be reported, got %v", problems)
	}
}

func TestUnownedRootLookupsAreFoundInTestSources(t *testing.T) {
	tests := t.TempDir()
	write := func(name string, source string) {
		if err := os.MkdirAll(filepath.Join(tests, name), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(tests, name, "🐹️.go"), []byte(source), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("clean", "package x\n// os.Getwd( in a comment is not a lookup\nfunc f() {}\n")
	write("leaky", "package x\nfunc f() { _, _ = os.Getwd(); _ = workspace.FindRepoRoot(\".\") }\n")
	write("guard", "package x\nfunc f() { _, _ = os.Getwd() }\n")
	write("host", "package adapter\nfunc f() { _, _ = os.Getwd() }\n")
	findings := sandbox.UnownedRootLookups(tests, "guard")
	if len(findings) != 2 || !strings.HasPrefix(findings[0], "leaky:2: ") || !strings.HasPrefix(findings[1], "leaky:2: ") {
		t.Fatalf("expected the two lookups of the leaky case only, got %v", findings)
	}
}
