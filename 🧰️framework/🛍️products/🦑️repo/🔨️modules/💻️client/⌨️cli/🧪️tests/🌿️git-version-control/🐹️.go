// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Git version control provider and composite fingerprints.

// #endregion 🧲️Header

package client

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

func TestComputeCompositeFingerprintIncludesAuthoredUntrackedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	firstFingerprint, firstMeta, fingerprintErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if fingerprintErr != nil {
		t.Fatal(fingerprintErr)
	}
	if firstFingerprint == "" || firstMeta == nil {
		t.Fatalf("expected first fingerprint and meta to be set, got %q / %+v", firstFingerprint, firstMeta)
	}

	if err := os.WriteFile(filepath.Join(tmpDir, "untracked.txt"), []byte("untracked"), 0644); err != nil {
		t.Fatalf("failed to create untracked file: %v", err)
	}

	secondFingerprint, secondMeta, secondErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if secondErr != nil {
		t.Fatal(secondErr)
	}
	if secondFingerprint == "" || secondMeta == nil {
		t.Fatalf("expected second fingerprint and meta to be set, got %q / %+v", secondFingerprint, secondMeta)
	}
	if firstFingerprint == secondFingerprint {
		t.Fatalf("expected authored untracked files to change fingerprint, got %q and %q", firstFingerprint, secondFingerprint)
	}
}

func TestComputeCompositeFingerprintFallsBackWhenRecursiveSubmoduleStatusFails(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess submodule fingerprint test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	childDir := initTestGitRepo(t, "main")
	nestedDir := initTestGitRepo(t, "main")
	run := func(dir string, args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = dir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v in %s failed: %s\n%s", args, dir, err, string(out))
		}
	}

	run(tmpDir, "-c", "protocol.file.allow=always", "submodule", "add", childDir, "child")
	run(tmpDir, "commit", "-am", "add child submodule")
	childWorkTree := filepath.Join(tmpDir, "child")
	run(childWorkTree, "config", "user.email", "test@test.com")
	run(childWorkTree, "config", "user.name", "Test")
	run(childWorkTree, "config", "commit.gpgsign", "false")
	run(childWorkTree, "-c", "protocol.file.allow=always", "submodule", "add", nestedDir, "nested")
	run(childWorkTree, "commit", "-am", "add nested submodule")

	nestedGitDir := filepath.Join(tmpDir, ".git", "modules", "child", "modules", "nested")
	if err := os.RemoveAll(nestedGitDir); err != nil {
		t.Fatalf("failed to remove nested gitdir: %v", err)
	}

	_, meta, fingerprintErr := computeCompositeFingerprintContext(context.Background(), tmpDir, nil)

	if fingerprintErr != nil {
		t.Fatal(fingerprintErr)
	}
	if _, ok := meta.SubmodulePointers["child"]; !ok {
		t.Fatalf("expected fallback submodule status to include child pointer, got %+v", meta.SubmodulePointers)
	}
	if _, ok := meta.SubmodulePointers["child/nested"]; ok {
		t.Fatalf("expected broken nested submodule to be excluded after fallback, got %+v", meta.SubmodulePointers)
	}
}

// 🆕️initTestGitRepoWithRemote creates a git repo with a bare remote and returns (workDir, remoteDir).
func initTestGitRepoWithRemote(t *testing.T) (string, string) {
	t.Helper()
	remoteDir := t.TempDir()
	cmd := exec.Command("git", "init", "--bare", "-b", "main", remoteDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init --bare failed: %s\n%s", err, string(out))
	}

	mainDir := initTestGitRepo(t, "main")
	run := func(dir string, args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = dir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v in %s failed: %s\n%s", args, dir, err, string(out))
		}
	}
	run(mainDir, "remote", "add", "origin", remoteDir)
	run(mainDir, "push", "-u", "origin", "main")

	workDir := t.TempDir()
	cmd = exec.Command("git", "clone", remoteDir, workDir)
	out, err = cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git clone failed: %s\n%s", err, string(out))
	}
	run(workDir, "config", "user.email", "test@test.com")
	run(workDir, "config", "user.name", "Test")
	run(workDir, "config", "commit.gpgsign", "false")
	run(workDir, "config", "tag.gpgsign", "false")
	return workDir, remoteDir
}

func TestGitVersionControlProviderCurrentCheckpoint(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	sha, err := p.CurrentCheckpoint(tmpDir)
	if err != nil {
		t.Fatalf("CurrentCheckpoint failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}
}

func TestGitVersionControlProviderCurrentBranch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	branch, err := p.CurrentBranch(tmpDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "main" {
		t.Errorf("expected main, got %q", branch)
	}
}

func TestGitVersionControlProviderCheckpoint(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	os.WriteFile(filepath.Join(tmpDir, "file2.txt"), []byte("world"), 0644)

	p := &GitVersionControlProvider{}
	sha, err := p.Checkpoint(tmpDir, "add file2")
	if err != nil {
		t.Fatalf("Checkpoint failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}

	currentSha, _ := p.CurrentCheckpoint(tmpDir)
	if currentSha != sha {
		t.Errorf("expected current checkpoint %q to match checkpoint result %q", currentSha, sha)
	}
}

func TestGitVersionControlProviderStageAll(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")
	os.WriteFile(filepath.Join(tmpDir, "newfile.txt"), []byte("new"), 0644)

	p := &GitVersionControlProvider{}
	if err := p.StageAll(tmpDir); err != nil {
		t.Fatalf("StageAll failed: %v", err)
	}

	files, err := p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) == 0 {
		t.Error("expected staged files after StageAll")
	}
}

func TestGitVersionControlProviderStagedFiles(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	tmpDir := initTestGitRepo(t, "main")

	p := &GitVersionControlProvider{}
	files, err := p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) != 0 {
		t.Errorf("expected no staged files, got %d", len(files))
	}

	os.WriteFile(filepath.Join(tmpDir, "file2.txt"), []byte("world"), 0644)
	cmd := exec.Command("git", "add", "file2.txt")
	cmd.Dir = tmpDir
	cmd.Run()
	files, err = p.StagedFiles(tmpDir)
	if err != nil {
		t.Fatalf("StagedFiles failed: %v", err)
	}
	if len(files) != 1 || files[0] != "file2.txt" {
		t.Errorf("expected [file2.txt], got %v", files)
	}
}

func TestGitVersionControlProviderCheckin(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	workDir, _ := initTestGitRepoWithRemote(t)

	p := &GitVersionControlProvider{}
	err := p.Checkin(workDir, "testuser")
	if err != nil {
		t.Fatalf("Checkin failed: %v", err)
	}

	branch, err := p.CurrentBranch(workDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "testuser/latest" {
		t.Errorf("expected testuser/latest, got %q", branch)
	}
}

func TestGitVersionControlProviderCheckout(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow real-git-subprocess provider test in short mode")
	}
	workDir, _ := initTestGitRepoWithRemote(t)

	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = workDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	run("switch", "-c", "testuser/latest")
	os.WriteFile(filepath.Join(workDir, "feature.txt"), []byte("feature"), 0644)
	run("add", "-A")
	run("commit", "-m", "add feature")

	p := &GitVersionControlProvider{}
	sha, err := p.Checkout(workDir, "testuser", "merge feature")
	if err != nil {
		t.Fatalf("Checkout failed: %v", err)
	}
	if len(sha) < 7 {
		t.Errorf("expected a SHA hash, got %q", sha)
	}

	branch, err := p.CurrentBranch(workDir)
	if err != nil {
		t.Fatalf("CurrentBranch failed: %v", err)
	}
	if branch != "main" {
		t.Errorf("expected main after checkout, got %q", branch)
	}

	if _, err := os.Stat(filepath.Join(workDir, "feature.txt")); os.IsNotExist(err) {
		t.Error("expected feature.txt to exist on main after checkout")
	}
}

func TestVersionHookEventsDispatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow version hook events dispatch test in short mode")
	}
	cases := []struct {
		name  string
		event HookEvent
	}{
		{"checkin starting", HookVersionCheckinStarting},
		{"checkin ended", HookVersionCheckinEnded},
		{"checkout starting", HookVersionCheckoutStarting},
		{"checkout ended", HookVersionCheckoutEnded},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:    tc.event,
				Client:   "",
				Second:   "2026-02-24T12:00:00Z",
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if !result.IsAllowed() {
				t.Errorf("expected allowed for %s, got denied: %s", tc.event, result.GetMessage())
			}
		})
	}
}
