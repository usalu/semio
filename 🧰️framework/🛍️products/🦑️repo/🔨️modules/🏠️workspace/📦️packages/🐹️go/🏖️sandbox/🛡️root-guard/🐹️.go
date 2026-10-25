// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️root-guard records every root resolution and binding that lands inside the checkout a test binary was started in.

// #endregion 🧲️Header

package sandbox

import (
	fmt "fmt"
	filepath "path/filepath"
	runtime "runtime"
	sort "sort"
	strings "strings"
	sync "sync"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🛡️Root Guard

// 🛡️Guard records every root resolution and binding that lands inside the real checkout a test binary was started in.
type Guard struct {
	real       string
	owned      []string
	mu         sync.Mutex
	violations []string
}

// 🆕️NewGuard guards the checkout rooted at real.
func NewGuard(real string) *Guard {
	return &Guard{real: real}
}

// 🏠️Own declares a directory the test binary created for itself, such as its sandbox repository inside an issued artifact folder below the checkout: resolutions and bindings within it never count as touching the checkout.
func (g *Guard) Own(path string) {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.owned = append(g.owned, path)
}

func (g *Guard) touchesCheckout(path string) bool {
	if !PathWithin(g.real, path) {
		return false
	}
	for _, owned := range g.owned {
		if PathWithin(owned, path) {
			return false
		}
	}
	return true
}

// 📏️PathWithin reports whether path is root or lies below it, never matching a sibling that merely shares a name prefix.
func PathWithin(root string, path string) bool {
	if root == "" || path == "" {
		return false
	}
	root, path = workspace.CanonicalPath(root), workspace.CanonicalPath(path)
	return path == root || strings.HasPrefix(path, root+string(filepath.Separator))
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

// 👁️Observe records a resolution that landed inside the guarded checkout together with the tests on the stack.
func (g *Guard) Observe(resolved string) {
	g.mu.Lock()
	defer g.mu.Unlock()
	if !g.touchesCheckout(resolved) {
		return
	}
	g.violations = append(g.violations, fmt.Sprintf("resolved %s via %s", resolved, callingTests()))
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
