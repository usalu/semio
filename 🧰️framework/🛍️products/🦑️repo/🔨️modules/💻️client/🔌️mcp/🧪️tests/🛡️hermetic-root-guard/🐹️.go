// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Hermeticity guard: every MCP test binary runs against a repository root it owns.

// #endregion 🧲️Header

package main

import (
	"os"
	"testing"

	launcher "github.com/usalu/semio/repo/launcher"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// 🛡️TestMain runs the test binary against a sandbox repository bound as the workspace root and working directory and fails the run when anything resolved or still holds the checkout it was started in.
func TestMain(m *testing.M) {
	os.Exit(sandbox.Run("mcp", nil, func() int { launcher.BindRoot(sandbox.Origin()); return m.Run() }))
}

// 🚷️TestTestsRunAgainstAnOwnedRoot fails when the bound root, the working directory or a test source points at the checkout the binary was started in.
func TestTestsRunAgainstAnOwnedRoot(t *testing.T) {
	for _, problem := range sandbox.Problems("🛡️hermetic-root-guard") {
		t.Error(problem)
	}
}
