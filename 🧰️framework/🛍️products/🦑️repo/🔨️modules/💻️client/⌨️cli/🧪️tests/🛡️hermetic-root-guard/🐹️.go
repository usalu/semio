// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hermeticity guard: no test may resolve, bind or hold an executor on the checkout it runs in.

// #endregion 🧲️Header

package client

import (
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🛡️TestTestsRunAgainstAnOwnedRoot fails when the bound root, the working directory or a test source points at the checkout the binary was started in; the end-of-run check in TestMain additionally fails on any resolution that landed inside it.
func TestTestsRunAgainstAnOwnedRoot(t *testing.T) {
	for _, problem := range sandbox.Problems("🛡️hermetic-root-guard") {
		t.Error(problem)
	}
}

// 📌️TestHermeticGuardFlagsHeldBindings proves the client's own roots, a test override and a bound executor are reported when they point into the real root.
func TestHermeticGuardFlagsHeldBindings(t *testing.T) {
	base := t.TempDir()
	real := filepath.Join(base, "real")
	if err := os.MkdirAll(real, 0o755); err != nil {
		t.Fatal(err)
	}
	guard := sandbox.NewGuard(real)
	if got := guard.Report(heldBindings()); len(got) != 0 {
		t.Fatalf("the sandbox bindings were flagged: %v", got)
	}
	previousRoot, previousOverride := rootDir, testRepoRootOverride
	rootDir, testRepoRootOverride = real, filepath.Join(real, "nested")
	executorMu.Lock()
	previousExecutorRoot := executorRoot
	executorRoot = real
	executorMu.Unlock()
	t.Cleanup(func() {
		rootDir, testRepoRootOverride = previousRoot, previousOverride
		executorMu.Lock()
		executorRoot = previousExecutorRoot
		executorMu.Unlock()
	})
	if got := guard.Report(heldBindings()); len(got) != 3 {
		t.Fatalf("expected rootDir, executor and override to be flagged, got %v", got)
	}
}

// 🔓️TestUnboundRootRefusesGraphQLQueries proves the executor never falls back to a default root.
func TestUnboundRootRefusesGraphQLQueries(t *testing.T) {
	previous := rootDir
	rootDir = ""
	t.Cleanup(func() { rootDir = previous })
	if _, err := gql("{ __typename }", nil); err == nil || !strings.Contains(err.Error(), "not bound") {
		t.Fatalf("expected an unbound-root error, got %v", err)
	}
}

// 🔗️TestBindWorkingDirectoryBindsTheExecutorToTheInvocation proves an explicit per-invocation bind moves the root, the workspace and the executor together.
func TestBindWorkingDirectoryBindsTheExecutorToTheInvocation(t *testing.T) {
	previous := rootDir
	t.Cleanup(func() { SetRootDir(previous) })
	first, second := t.TempDir(), t.TempDir()
	for _, root := range []string{first, second} {
		t.Chdir(root)
		if err := BindWorkingDirectory(); err != nil {
			t.Fatal(err)
		}
		bound, err := boundExecutor()
		if err != nil || bound == nil {
			t.Fatalf("no executor for %s: %v", root, err)
		}
		executorMu.Lock()
		got := executorRoot
		executorMu.Unlock()
		if workspace.CanonicalPath(got) != workspace.CanonicalPath(root) || workspace.CanonicalPath(GetRootDir()) != workspace.CanonicalPath(root) {
			t.Fatalf("expected root and executor bound to %s, got root %s executor %s", root, GetRootDir(), got)
		}
		if workspace.CanonicalPath(workspace.GetRootDir()) != workspace.CanonicalPath(root) {
			t.Fatalf("workspace root %s did not follow the bind to %s", workspace.GetRootDir(), root)
		}
	}
}
