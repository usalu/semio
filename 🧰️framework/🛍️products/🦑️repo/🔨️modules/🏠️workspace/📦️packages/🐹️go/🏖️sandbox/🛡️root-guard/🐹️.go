// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️root-guard records every root resolution and binding that lands on the root of the checkout a test binary was started in.

// #endregion 🧲️Header

package sandbox

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	runtime "runtime"
	sort "sort"
	strings "strings"
	sync "sync"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🛡️Root Guard

// 🛡️Guard records every resolution and binding that lands on the root of the real checkout a test binary was started in; a repository nested below the checkout is a root of its own and never counts.
type Guard struct {
	real       string
	abort      bool
	mu         sync.Mutex
	violations []string
}

// 🆕️NewGuard guards the checkout rooted at real.
func NewGuard(real string) *Guard {
	return &Guard{real: real}
}

// ⛔️AbortOnTouch makes the guard panic at the first resolution of the checkout root, before the offending test can write anything into the real repository; the violation stays recorded for the report.
func (g *Guard) AbortOnTouch() *Guard {
	g.abort = true
	return g
}

// 🎯️touchesCheckout reports whether path, once resolved to the repository root it belongs to, is the root of the checkout itself, which is how a directory below the checkout without a `.git` marker of its own is caught.
func (g *Guard) touchesCheckout(path string) bool {
	return path != "" && SameDirectory(g.real, workspace.ResolveRepoRoot(path))
}

// 📏️PathWithin reports whether path is root or lies below it, never matching a sibling that merely shares a name prefix and comparing by directory identity, so symlinks, case and Unicode normalisation of the file system never hide a match.
func PathWithin(root string, path string) bool {
	if root == "" || path == "" {
		return false
	}
	for current := workspace.CanonicalPath(path); ; current = filepath.Dir(current) {
		if SameDirectory(root, current) {
			return true
		}
		if filepath.Dir(current) == current {
			return false
		}
	}
}

// 🪞️SameDirectory reports whether two spellings name one directory: the deepest existing ancestors must be one file system object (symlinks, separators, dot segments, case and Unicode normalisation of the file system, Windows drive letters and short names all compare by identity, see https://pkg.go.dev/os#SameFile) and the missing remainders must be spelled alike.
func SameDirectory(first string, second string) bool {
	if first == "" || second == "" {
		return false
	}
	firstAnchor, firstTail := anchor(first)
	secondAnchor, secondTail := anchor(second)
	return firstTail == secondTail && os.SameFile(firstAnchor, secondAnchor)
}

// ⚓️anchor returns the file system object of the deepest existing ancestor of path and the cleaned remainder below it.
func anchor(path string) (os.FileInfo, string) {
	absolute, err := filepath.Abs(path)
	if err != nil {
		absolute = filepath.Clean(path)
	}
	tail := ""
	for current := absolute; ; current = filepath.Dir(current) {
		if info, err := os.Stat(current); err == nil {
			return info, tail
		}
		if filepath.Dir(current) == current {
			return nil, absolute
		}
		tail = filepath.Join(filepath.Base(current), tail)
	}
}

// 🔎️callingTests names the test functions on the current stack.
func callingTests() string {
	pcs := make([]uintptr, 64)
	frames := runtime.CallersFrames(pcs[:runtime.Callers(3, pcs)])
	names := []string{}
	for {
		frame, more := frames.Next()
		if index := strings.LastIndex(frame.Function, ".Test"); index >= 0 {
			names = append(names, frame.Function[index+1:])
		}
		if !more {
			break
		}
	}
	if len(names) == 0 {
		return "a non-test goroutine"
	}
	return strings.Join(names, " < ")
}

// 👁️Observe records a resolution that landed on the root of the guarded checkout together with the tests on the stack, and aborts at once when the guard was told to.
func (g *Guard) Observe(resolved string) {
	if !g.touchesCheckout(resolved) {
		return
	}
	violation := fmt.Sprintf("resolved %s via %s", resolved, callingTests())
	g.mu.Lock()
	g.violations = append(g.violations, violation)
	g.mu.Unlock()
	if g.abort {
		panic("FAIL: a test resolved the real repository root before writing anything: " + violation)
	}
}

// 📋️Report returns the recorded resolutions plus every named binding (the workspace root and the extra roots the caller holds) that still points into the guarded checkout.
func (g *Guard) Report(held map[string]string) []string {
	g.mu.Lock()
	defer g.mu.Unlock()
	problems := append([]string{}, g.violations...)
	bindings := map[string]string{}
	if root, bound := workspace.LookupRootDir(); bound {
		bindings["workspace root"] = root
	}
	for name, bound := range held {
		bindings[name] = bound
	}
	late := []string{}
	for name, bound := range bindings {
		if g.touchesCheckout(bound) {
			late = append(late, fmt.Sprintf("%s is bound to the real repository root %s", name, bound))
		}
	}
	sort.Strings(late)
	return append(problems, late...)
}

// #endregion 🛡️Root Guard
