// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌿️git carries command execution, git author and checkpoint queries, ignored set lookups and checkpoint file reads.

// #endregion 🧲️Header

package workspace

import (
	context "context"
	fmt "fmt"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	sync "sync"
)

// #region 🌿️Git

// ▪️ExecCommand MUST complete the operation and return consistent results.
func ExecCommand(command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	return ExecCommandContext(context.Background(), command, args, cwd)
}

// ▫️ExecCommandContext runs a child process with explicit cancellation and deadline propagation.
func ExecCommandContext(ctx context.Context, command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	if cwd == "" {
		cwd = GetRootDir()
	}
	cmd := exec.CommandContext(ctx, command, args...)
	cmd.Dir = cwd
	var stdoutBuf, stderrBuf strings.Builder
	cmd.Stdout = &stdoutBuf
	cmd.Stderr = &stderrBuf
	err := cmd.Run()
	exitCode = 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = 1
		}
	}
	return stdoutBuf.String(), stderrBuf.String(), exitCode
}

// ✍️GetGitAuthor MUST return the stored value without modification.
// 🔐️GetGitAuthor returns the git author of the value.
func GetGitAuthor() string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	name = strings.TrimSpace(name)
	email = strings.TrimSpace(email)
	if email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	return name
}

// 💾️GetGitCheckpoint MUST return the stored value without modification.
// ✔️GetGitCheckpoint returns the git checkpoint of the value.
func GetGitCheckpoint() string {
	sha, _, _ := ExecCommand("git", []string{"rev-parse", "HEAD"}, "")
	return strings.TrimSpace(sha)
}

// 🗃️GetGitIgnoredSet MUST return the stored value without modification.

// 🧭️gitTopLevelCache remembers, per root directory, whether git resolves that directory as its own
// repository top level.
var gitTopLevelCache sync.Map

// 🧭️RootDirIsGitTopLevel MUST return true only when git treats the configured root as the top level
// of its own repository. A root that is not one — a materialised fixture tree, a temporary
// workspace — would otherwise make `git check-ignore` answer for the enclosing repository and
// silently drop every file below it.
func RootDirIsGitTopLevel() bool {
	root := GetRootDir()
	if strings.TrimSpace(root) == "" {
		return false
	}
	if cached, ok := gitTopLevelCache.Load(root); ok {
		return cached.(bool)
	}
	stdout, _, exitCode := ExecCommand("git", []string{"rev-parse", "--show-toplevel"}, root)
	verdict := false
	if exitCode == 0 {
		verdict = NormalizePath(CanonicalPath(strings.TrimSpace(stdout))) == NormalizePath(CanonicalPath(root))
	}
	gitTopLevelCache.Store(root, verdict)
	return verdict
}

// ▫️GetGitIgnoredSet returns the git ignored set of the value.
func GetGitIgnoredSet(paths []string) map[string]bool {
	if len(paths) == 0 {
		return make(map[string]bool)
	}
	if !RootDirIsGitTopLevel() {
		return make(map[string]bool)
	}
	args := append([]string{"-c", "core.quotepath=off", "check-ignore"}, paths...)
	stdout, _, _ := ExecCommand("git", args, "")
	ignored := make(map[string]bool)
	for _, line := range strings.Split(stdout, "\n") {
		line = strings.TrimSpace(line)
		if line != "" {
			ignored[NormalizePath(line)] = true
		}
	}
	return ignored
}

// 💾️CountLinesAtCheckpoint MUST complete the operation and return consistent results.
func CountLinesAtCheckpoint(checkpoint, filePath string) int {
	stdout, _, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return 0
	}
	return CountLines(stdout)
}

// ❌️ReadTextFileAtCheckpoint MUST return the text file at checkpoint content or an error if unavailable.
// ✔️ReadTextFileAtCheckpoint reads and returns text file at checkpoint from the source.
func ReadTextFileAtCheckpoint(checkpoint, filePath string) (string, error) {
	if checkpoint == "" {
		return ReadTextFile(filepath.Join(GetRootDir(), filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

// 🔷️splitCommandSegments splits a command into segments.
func SplitCommandSegments(cmd string) []string {
	cmd = strings.TrimSpace(cmd)
	if cmd == "" {
		return nil
	}
	var segments []string
	var current strings.Builder
	quote := rune(0)
	flush := func() {
		segment := strings.TrimSpace(current.String())
		if segment != "" {
			segments = append(segments, segment)
		}
		current.Reset()
	}
	for i := 0; i < len(cmd); i++ {
		ch := rune(cmd[i])
		if quote != 0 {
			current.WriteRune(ch)
			if ch == quote {
				quote = 0
			}
			continue
		}
		if ch == '\'' || ch == '"' {
			quote = ch
			current.WriteRune(ch)
			continue
		}
		if ch == ';' {
			flush()
			continue
		}
		if ch == '&' && i+1 < len(cmd) && cmd[i+1] == '&' {
			flush()
			i++
			continue
		}
		if ch == '|' {
			flush()
			if i+1 < len(cmd) && cmd[i+1] == '|' {
				i++
			}
			continue
		}
		current.WriteRune(ch)
	}
	flush()
	return segments
}

// #endregion 🌿️Git
