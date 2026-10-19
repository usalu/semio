// 🛡️ Every test binary of this module runs against a repository root it owns: the sandbox from sandbox.Run is bound as the root and the working directory, and the run fails when anything resolves or still holds the checkout the binary was started in.

package testrunner

import (
	os "os"
	testing "testing"

	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestMain(m *testing.M) {
	os.Exit(sandbox.Run("testrunner", nil, m.Run))
}

func TestTestsRunAgainstAnOwnedRoot(t *testing.T) {
	for _, problem := range sandbox.Problems("🛡️hermetic-root-guard") {
		t.Error(problem)
	}
}
