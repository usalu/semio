// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Hermeticity guard: no MCP test binds or resolves a repository root.

// #endregion 🧲️Header

package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/usalu/semio/repo/client"
)

// 🛡️TestNoTestBindsTheClientRepositoryRoot fails when any test in this process left the client bound to a repository; the client starts unbound and only an entry point may bind it.
func TestNoTestBindsTheClientRepositoryRoot(t *testing.T) {
	if root := client.GetRootDir(); root != "" {
		t.Fatalf("the client is bound to %q; MCP tests must not bind or resolve a repository root", root)
	}
}

// 🚷️TestTestSourcesDoNotResolveARepositoryRoot fails when a test source binds, resolves or walks to a repository root.
func TestTestSourcesDoNotResolveARepositoryRoot(t *testing.T) {
	forbidden := []string{"GetRootDir(", "SetRootDir(", "BindWorkingDirectory(", "FindRepoRoot(", "os.Getwd(", "runtime.Caller("}
	pattern := filepath.Join("🧪️tests", "*", "🐹️.go")
	paths, err := filepath.Glob(pattern)
	if err != nil {
		t.Fatal(err)
	}
	scanned := 0
	for _, path := range paths {
		if filepath.Base(filepath.Dir(path)) == "🛡️hermetic-root-guard" {
			continue
		}
		data, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}
		scanned++
		for number, line := range strings.Split(string(data), "\n") {
			for _, needle := range forbidden {
				if strings.Contains(line, needle) {
					t.Errorf("%s:%d uses %q; tests must own their fixtures instead of resolving a repository root", path, number+1, needle)
				}
			}
		}
	}
	if scanned < 5 {
		t.Fatalf("scanned only %d test sources through %s", scanned, pattern)
	}
}
