// 🐹️ Go side of the git version control case. Every fact is read back through the provider, never
// through a second git call of the adapter's own, so what is projected is the provider's answer.
package adapter

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"

	repo "github.com/usalu/semio/repo/providers"
	host "semio.tech/repo/test"
)

// region 🔖️Scratch

// 🧪️scratchRepo builds a throwaway repository for one scenario.
//
// Deliberately NOT under ctx.WorkDir: the work directory lives inside this repository's own cache,
// so git rev-parse there would answer from the surrounding checkout and the not-a-repository
// scenario could never fail. The name is fixed per implementation and scenario, so a re-run reuses
// one directory instead of leaving a new one behind.
func scratchRepo(ctx *host.Context, initialised bool) (string, error) {
	root := filepath.Join(os.TempDir(), "semio-providers-go-"+ctx.Scenario.ID)
	if err := os.RemoveAll(root); err != nil {
		return "", err
	}
	if err := os.MkdirAll(root, 0o755); err != nil {
		return "", err
	}
	if !initialised {
		return root, nil
	}
	run := func(args ...string) error {
		cmd := exec.Command("git", args...)
		cmd.Dir = root
		if out, err := cmd.CombinedOutput(); err != nil {
			return fmt.Errorf("git %v failed: %s\n%s", args, err, string(out))
		}
		return nil
	}
	for _, args := range [][]string{
		{"init", "-b", "main"},
		{"config", "user.email", "test@test.com"},
		{"config", "user.name", "Test"},
		{"config", "commit.gpgsign", "false"},
	} {
		if err := run(args...); err != nil {
			return "", err
		}
	}
	if err := os.WriteFile(filepath.Join(root, "file.txt"), []byte("hello\n"), 0o644); err != nil {
		return "", err
	}
	if err := run("add", "-A"); err != nil {
		return "", err
	}
	if err := run("commit", "-m", "initial"); err != nil {
		return "", err
	}
	return root, nil
}

func isObjectName(value string) bool {
	if len(value) != 40 {
		return false
	}
	for _, character := range value {
		if !((character >= '0' && character <= '9') || (character >= 'a' && character <= 'f') || (character >= 'A' && character <= 'F')) {
			return false
		}
	}
	return true
}

// endregion 🔖️Scratch

// region 🔖️Scenarios

func aFreshRepositoryReportsItsBranchAndHead(ctx *host.Context) (host.Outcome, error) {
	root, err := scratchRepo(ctx, true)
	if err != nil {
		return host.Outcome{}, err
	}
	provider := &repo.GitVersionControlProvider{}
	branch, err := provider.CurrentBranch(root)
	if err != nil {
		return host.Outcome{}, err
	}
	first, err := provider.CurrentCheckpoint(root)
	if err != nil {
		return host.Outcome{}, err
	}
	second, err := provider.CurrentCheckpoint(root)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"branch":             branch,
		"headIsAnObjectName": isObjectName(first),
		"bothReadsAgree":     first == second,
		"kind":               provider.Kind(),
	}}, nil
}

func stagingListsTheAddedPaths(ctx *host.Context) (host.Outcome, error) {
	root, err := scratchRepo(ctx, true)
	if err != nil {
		return host.Outcome{}, err
	}
	provider := &repo.GitVersionControlProvider{}
	before, err := provider.StagedFiles(root)
	if err != nil {
		return host.Outcome{}, err
	}
	for name, content := range map[string]string{"a.txt": "a\n", "b.txt": "b\n"} {
		if err := os.WriteFile(filepath.Join(root, name), []byte(content), 0o644); err != nil {
			return host.Outcome{}, err
		}
	}
	if err := provider.StageAll(root); err != nil {
		return host.Outcome{}, err
	}
	after, err := provider.StagedFiles(root)
	if err != nil {
		return host.Outcome{}, err
	}
	sort.Strings(after)
	if before == nil {
		before = []string{}
	}
	if after == nil {
		after = []string{}
	}
	return host.Outcome{Projection: map[string]any{"before": before, "after": after}}, nil
}

func aCheckpointCommitsAndAdvancesHead(ctx *host.Context) (host.Outcome, error) {
	root, err := scratchRepo(ctx, true)
	if err != nil {
		return host.Outcome{}, err
	}
	provider := &repo.GitVersionControlProvider{}
	before, err := provider.CurrentCheckpoint(root)
	if err != nil {
		return host.Outcome{}, err
	}
	if err := os.WriteFile(filepath.Join(root, "file2.txt"), []byte("world\n"), 0o644); err != nil {
		return host.Outcome{}, err
	}
	checkpoint, err := provider.Checkpoint(root, "add file2")
	if err != nil {
		return host.Outcome{}, err
	}
	after, err := provider.CurrentCheckpoint(root)
	if err != nil {
		return host.Outcome{}, err
	}
	staged, err := provider.StagedFiles(root)
	if err != nil {
		return host.Outcome{}, err
	}
	if staged == nil {
		staged = []string{}
	}
	return host.Outcome{Projection: map[string]any{
		"headAdvanced":             checkpoint != before,
		"checkpointIsAnObjectName": isObjectName(checkpoint),
		"checkpointIsHead":         checkpoint == after,
		"stagedAfterCheckpoint":    staged,
	}}, nil
}

func readingOutsideARepositoryIsAnError(ctx *host.Context) (host.Outcome, error) {
	root, err := scratchRepo(ctx, false)
	if err != nil {
		return host.Outcome{}, err
	}
	provider := &repo.GitVersionControlProvider{}
	_, branchErr := provider.CurrentBranch(root)
	_, checkpointErr := provider.CurrentCheckpoint(root)
	return host.Outcome{Projection: map[string]any{"branchFailed": branchErr != nil, "checkpointFailed": checkpointErr != nil}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("a-fresh-repository-reports-its-branch-and-head", aFreshRepositoryReportsItsBranchAndHead).
		Subject("staging-lists-the-added-paths", stagingListsTheAddedPaths).
		Subject("a-checkpoint-commits-and-advances-head", aCheckpointCommitsAndAdvancesHead).
		Subject("reading-outside-a-repository-is-an-error", readingOutsideARepositoryIsAnError)
}

// endregion 🔖️Registration
