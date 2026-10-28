// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚧️sandboxed-run runs a test binary against a temporary repository it owns and fails the run when anything resolved or still holds the root of the checkout the binary was started in.

// #endregion 🧲️Header

package sandbox

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚧️Sandboxed Run

// 🎚️GuardEnv names the environment variable that lowers the root guard from aborting at the first touch of the real checkout root to reporting every touch at the end of the run (`report`), which is how a maintainer lists all offenders at once.
const GuardEnv = "SEMIO_SANDBOX_GUARD"

// 🧺️ScratchEnv names the environment variable that keeps the issued scratch directory below the checkout (`keep`) instead of relocating it, for a harness whose tests need the scratch to be the issued one.
const ScratchEnv = "SEMIO_SANDBOX_SCRATCH"

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

// 🧰️Run runs a test binary against a root it owns: a temporary git repository is bound as the workspace root and working directory, every resolution is observed, and the run aborts when anything resolves the root of the checkout the binary was started in (`SEMIO_SANDBOX_GUARD=report` collects every offender instead) and fails when anything still holds it; an issued scratch directory below the checkout is replaced by one outside it (see RelocateScratch), and repositories below the checkout are roots of their own and pass. The held callback names extra roots the caller keeps, and it may be nil.
func Run(name string, held func() map[string]string, run func() int) int {
	cwd, err := os.Getwd()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	checkout := workspace.ResolveRepoRoot(cwd)
	if os.Getenv(ScratchEnv) != "keep" {
		if restore := RelocateScratch(checkout, ScratchBases()...); restore != nil {
			defer restore()
		}
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
	guard := NewGuard(checkout)
	if os.Getenv(GuardEnv) != "report" {
		guard.AbortOnTouch()
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

// 🗑️scratchEnvironment lists the variables through which `os.TempDir` finds the scratch directory on Unix (`TMPDIR`) and Windows (`TMP`, `TEMP`).
var scratchEnvironment = []string{"TMPDIR", "TMP", "TEMP"}

// 📍️ScratchBases lists the directories, in order of preference, below which a scratch directory outside the checkout is created: the per-user cache directory, then the shared temporary directories of Unix.
func ScratchBases() []string {
	bases := []string{}
	if cache, err := os.UserCacheDir(); err == nil {
		bases = append(bases, cache)
	}
	if runtime.GOOS != "windows" {
		bases = append(bases, "/tmp", "/var/tmp")
	}
	return bases
}

// 🧹️RelocateScratch moves the scratch directory of this process out of the checkout when the issued one lies inside it, because a test directory below a checkout resolves to the checkout as soon as it carries no `.git` of its own. It creates a fresh directory below the first base that lies outside the checkout, has no repository above it and accepts a write, points the scratch variables at it and returns the function that removes it and restores the variables, or nil when the scratch already lies outside or no base qualifies (the guard then remains the defence).
func RelocateScratch(checkout string, bases ...string) func() {
	if !PathWithin(checkout, os.TempDir()) {
		return nil
	}
	for _, base := range bases {
		if base == "" || PathWithin(checkout, base) {
			continue
		}
		scratch, err := os.MkdirTemp(base, "sg-")
		if err != nil {
			continue
		}
		probe := filepath.Join(scratch, "probe")
		if os.WriteFile(probe, nil, 0o600) != nil || os.Remove(probe) != nil || !SameDirectory(workspace.ResolveRepoRoot(scratch), scratch) {
			os.RemoveAll(scratch)
			continue
		}
		previous := map[string]*string{}
		for _, variable := range scratchEnvironment {
			if value, set := os.LookupEnv(variable); set {
				previous[variable] = &value
			} else {
				previous[variable] = nil
			}
			os.Setenv(variable, scratch)
		}
		return func() {
			for variable, value := range previous {
				if value == nil {
					os.Unsetenv(variable)
				} else {
					os.Setenv(variable, *value)
				}
			}
			os.RemoveAll(scratch)
		}
	}
	return nil
}

// #endregion 🚧️Sandboxed Run
