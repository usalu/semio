// 🛡️ No root is ever bound implicitly: an unbound workspace refuses every root lookup until an entry point binds the invocation.

package workspace

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
)

func unbindRoot(t *testing.T) {
	t.Helper()
	previous, bound := LookupRootDir()
	rootStateMutex.Lock()
	rootDir = ""
	rootStateMutex.Unlock()
	t.Cleanup(func() {
		if bound {
			SetRootDir(previous)
		}
	})
}

func TestUnboundWorkspaceRefusesRootLookups(t *testing.T) {
	unbindRoot(t)
	if root, bound := LookupRootDir(); bound || root != "" {
		t.Fatalf("an unbound workspace reported the root %q", root)
	}
	for name, lookup := range map[string]func(){
		"GetRootDir":      func() { GetRootDir() },
		"GetRepoMetaDir":  func() { GetRepoMetaDir() },
		"GetRepoGoalsDir": func() { GetRepoGoalsDir() },
	} {
		func() {
			defer func() {
				message, _ := recover().(string)
				if !strings.Contains(message, "not bound") {
					t.Errorf("%s did not refuse while unbound: %q", name, message)
				}
			}()
			lookup()
		}()
	}
}

func TestWorkspaceStartsUnboundUntilTheInvocationBindsIt(t *testing.T) {
	unbindRoot(t)
	repository := t.TempDir()
	if err := os.MkdirAll(filepath.Join(repository, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	nested := filepath.Join(repository, "a", "b")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Chdir(nested)
	if _, bound := LookupRootDir(); bound {
		t.Fatal("changing the working directory must not bind a root")
	}
	if err := BindWorkingDirectory(); err != nil {
		t.Fatal(err)
	}
	if got, want := CanonicalPath(GetRootDir()), CanonicalPath(repository); got != want {
		t.Fatalf("BindWorkingDirectory bound %q, want %q", got, want)
	}
}
