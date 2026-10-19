// 🏖️ Every test binary of this module runs against a repository root it owns, and the sandbox that enforces it is itself proven here.

package workspace_test

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestMain(m *testing.M) {
	os.Exit(sandbox.Run("workspace", nil, m.Run))
}

func TestTestsRunAgainstAnOwnedRoot(t *testing.T) {
	for _, problem := range sandbox.Problems("🏖️sandbox") {
		t.Error(problem)
	}
}

func TestRootGuardFlagsResolutionsInsideTheCheckoutOnly(t *testing.T) {
	checkout := t.TempDir()
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
	guard.Observe(filepath.Join(checkout, "nested"))
	problems := guard.Report(nil)
	if len(problems) != 1 || !strings.Contains(problems[0], "TestRootGuardFlagsResolutionsInsideTheCheckoutOnly") {
		t.Fatalf("an inside resolution must be reported with the test that caused it, got %v", problems)
	}
}

func TestRootGuardFlagsHeldBindingsInsideTheCheckout(t *testing.T) {
	checkout := t.TempDir()
	guard := sandbox.NewGuard(checkout)
	problems := guard.Report(map[string]string{"executor": checkout, "cache": filepath.Join(os.TempDir(), "owned")})
	if len(problems) != 1 || !strings.Contains(problems[0], "executor is bound to the real repository root") {
		t.Fatalf("only the held binding inside the checkout must be reported, got %v", problems)
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
