// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌱️fixture-repository builds the throwaway repositories and trees a test materialises inside a root it owns.

// #endregion 🧲️Header

package sandbox

import (
	os "os"
	filepath "path/filepath"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌱️Fixture Repository

// 🧫️T is the part of testing.TB the fixture repository builders need.
type T interface {
	Helper()
	TempDir() string
	Cleanup(func())
	Fatalf(format string, args ...any)
}

// 🧷️BindRoot binds dir as the workspace root until the test ends and then restores the previous binding.
func BindRoot(t T, dir string) {
	t.Helper()
	previous, bound := workspace.LookupRootDir()
	fenceRepositoryRoot(t, dir)
	workspace.SetRootDir(dir)
	t.Cleanup(func() {
		if bound {
			workspace.SetRootDir(previous)
		}
	})
}

// 🧱️fenceRepositoryRoot makes dir a repository of its own when it has no `.git`, so root discovery and git itself end in dir even where the test directory lies inside a checkout (a bare marker directory would let git answer for the enclosing checkout).
func fenceRepositoryRoot(t T, dir string) {
	t.Helper()
	if _, err := os.Lstat(filepath.Join(dir, ".git")); err == nil {
		return
	}
	if output, err := workspace.ToolCommand("git", "-C", dir, "init", "-q").CombinedOutput(); err != nil {
		t.Fatalf("git init %s: %v\n%s", dir, err, output)
	}
}

// 🌱️NewRepository creates a throwaway git repository with one empty commit, binds it as the workspace root until the test ends and returns its path.
func NewRepository(t T) string {
	t.Helper()
	repository := t.TempDir()
	for _, arguments := range [][]string{{"init", "-q"}, {"config", "user.email", "test@test.com"}, {"config", "user.name", "Test"}, {"config", "commit.gpgsign", "false"}, {"commit", "-q", "--allow-empty", "-m", "initial"}} {
		if output, err := workspace.ToolCommand("git", append([]string{"-C", repository}, arguments...)...).CombinedOutput(); err != nil {
			t.Fatalf("git %v: %v\n%s", arguments, err, output)
		}
	}
	BindRoot(t, repository)
	return repository
}

// ✍️WriteFile writes content to the slash-separated relative path below root and creates the missing directories.
func WriteFile(t T, root string, relative string, content string) {
	t.Helper()
	absolute := filepath.Join(root, filepath.FromSlash(relative))
	if err := os.MkdirAll(filepath.Dir(absolute), 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", relative, err)
	}
	if err := os.WriteFile(absolute, []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", relative, err)
	}
}

// 📋️CopyTree copies the directory tree source into target, which a test uses to materialise a module fixture inside a repository it owns.
func CopyTree(t T, source string, target string) {
	t.Helper()
	err := filepath.WalkDir(source, func(path string, entry os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		relative, err := filepath.Rel(source, path)
		if err != nil {
			return err
		}
		destination := filepath.Join(target, relative)
		if entry.IsDir() {
			return os.MkdirAll(destination, 0o755)
		}
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		return os.WriteFile(destination, data, 0o644)
	})
	if err != nil {
		t.Fatalf("copy %s to %s: %v", source, target, err)
	}
}

// #endregion 🌱️Fixture Repository
