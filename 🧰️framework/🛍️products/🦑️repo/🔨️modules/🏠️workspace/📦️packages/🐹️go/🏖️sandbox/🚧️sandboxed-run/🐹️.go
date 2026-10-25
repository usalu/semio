// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚧️sandboxed-run runs a test binary against a temporary repository it owns and fails the run when anything resolved or still holds the checkout the binary was started in.

// #endregion 🧲️Header

package sandbox

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚧️Sandboxed Run

var (
	origin string
	root   string
)

// 🏠️Origin returns the working directory the sandboxed test binary was started in, which is the directory of the module under test.
func Origin() string {
	return origin
}

// 🧭️OriginPath joins parts onto the directory the sandboxed test binary was started in, which is how a test reads a fixture its module owns without locating the checkout.
func OriginPath(parts ...string) string {
	return filepath.Join(append([]string{origin}, parts...)...)
}

// 🧪️Root returns the temporary git repository the sandboxed test binary is bound to.
func Root() string {
	return root
}

// 🧰️Run runs a test binary (its scratch directory, which the issued test environment places below the checkout, is owned too) against a root it owns: a temporary git repository is bound as the workspace root and working directory, every resolution is observed, and the run fails when anything resolved or still holds the checkout the binary was started in. The held callback names extra roots the caller keeps, and it may be nil.
func Run(name string, held func() map[string]string, run func() int) int {
	cwd, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	repository, err := os.MkdirTemp("", name+"-sandbox-")
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	defer os.RemoveAll(repository)
	if output, err := workspace.ToolCommand("git", "-C", repository, "init", "-q").CombinedOutput(); err != nil {
		fmt.Fprintf(os.Stderr, "failed to initialise the sandbox repository: %v\n%s", err, output)
		return 2
	}
	origin = workspace.CanonicalPath(cwd)
	checkout := workspace.ResolveRepoRoot(cwd)
	guard := NewGuard(checkout)
	guard.Own(repository)
	if scratch := os.TempDir(); workspace.CanonicalPath(scratch) != workspace.CanonicalPath(checkout) {
		guard.Own(scratch)
	}
	defer workspace.ObserveRootResolution(guard.Observe)()
	workspace.SetRootDir(repository)
	root = workspace.GetRootDir()
	if err := os.Chdir(root); err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	code := run()
	extra := map[string]string(nil)
	if held != nil {
		extra = held()
	}
	if problems := guard.Report(extra); len(problems) > 0 {
		fmt.Fprintf(os.Stderr, "FAIL: tests touched the real repository root %s:\n  %s\n", guard.real, strings.Join(problems, "\n  "))
		if code == 0 {
			code = 1
		}
	}
	return code
}

// #endregion 🚧️Sandboxed Run
