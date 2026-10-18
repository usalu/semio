// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛡️blocking carries tool and command classification and the blocking policy: blocked tool patterns, git verbs and command segments.

// #endregion 🧲️Header

package hooks

import (
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	testrunner "github.com/usalu/semio/repo/testrunner"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏛️ToolClassification

// 🏛️ClassifyTool answers the tool kind of a named tool, across every IDE's tool vocabulary.
func ClassifyTool(toolName string) model.ToolKind { return todos.ClassifyTool(toolName) }

// 🔤️ClassifyCommandKind answers the tool kind a raw shell command implies.
func ClassifyCommandKind(command string) model.ToolKind { return model.ClassifyCommandKind(command) }

// ✂️TrimPipelineTail cuts a command at its first unquoted pipe.
func TrimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	var current strings.Builder
	quote := rune(0)
	for _, character := range trimmed {
		if quote != 0 {
			current.WriteRune(character)
			if character == quote {
				quote = 0
			}
			continue
		}
		if character == '\'' || character == '"' {
			quote = character
			current.WriteRune(character)
			continue
		}
		if character == '|' {
			break
		}
		current.WriteRune(character)
	}
	return strings.TrimSpace(current.String())
}

// 🧪️ExtractTestSegment answers the test-running segment of a composite command, with the directory a leading `cd` selected.
func ExtractTestSegment(command string) (string, string) {
	return testrunner.ExtractTestSegmentFromCommand(command)
}

// #endregion 🏛️ToolClassification

// #region 🛡️BlockingPolicy

// ⌨️BlockedGitVerbs lists the git subcommands that modify repository state and are therefore refused.
var BlockedGitVerbs = []string{"add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag", "clean"}

// 🔷️SplitCommandSegments splits a command at every unquoted `;`, `&&`, `|` and `||`.
func SplitCommandSegments(command string) []string {
	segments := workspace.SplitCommandSegments(command)
	if segments == nil {
		return []string{}
	}
	return segments
}

// 🌿️ContainsBlockedGitInCode scans arbitrary inline code (`python -c`, `node -e`, …) for a blocked git invocation.
func ContainsBlockedGitInCode(code string) (bool, string) { return containsBlockedGitInCode(code) }

// 🐙️IsCommandSegmentBlocked answers why one command segment is refused, or false when it is allowed.
func IsCommandSegmentBlocked(segment string) (bool, string) { return isCommandSegmentBlocked(segment) }

// #endregion 🛡️BlockingPolicy

// #region 💾️Missing Utility Functions

// 🧩️BlockedToolPatterns are patterns for blocked tools.
var BlockedToolPatterns = map[string]bool{
	"rm":              true,
	"rmdir":           true,
	"mv":              true,
	"cp":              true,
	"git add":         true,
	"git branch":      true,
	"git checkout":    true,
	"git cherry-pick": true,
	"git clean":       true,
	"git clone":       true,
	"git commit":      true,
	"git config":      true,
	"git fetch":       true,
	"git init":        true,
	"git merge":       true,
	"git mv":          true,
	"git pull":        true,
	"git push":        true,
	"git rebase":      true,
	"git remote":      true,
	"git reset":       true,
	"git restore":     true,
	"git revert":      true,
	"git rm":          true,
	"git stash":       true,
	"git switch":      true,
	"git tag":         true,
}

// ✔️IsToolBlocked checks if a tool is blocked.
func IsToolBlocked(tool string, args string) (bool, string) {
	for _, segment := range workspace.SplitCommandSegments(args) {
		if blocked, reason := isCommandSegmentBlocked(segment); blocked {
			return true, reason + "; other developers and agents may be editing the same files concurrently"
		}
	}
	tool = strings.ToLower(strings.TrimSpace(tool))
	if pattern, ok := BlockedToolPatterns[tool]; ok && pattern {
		return true, "blocked: " + tool + "; other developers and agents may be editing the same files concurrently"
	}
	return false, ""
}

// 🐙️isCommandSegmentBlocked checks if a command segment is blocked.
// ⌨️blockedGitVerbs are git subcommands that modify repository state.
var blockedGitVerbs = map[string]bool{
	"add": true, "branch": true, "checkout": true, "cherry-pick": true, "clone": true,
	"commit": true, "config": true, "fetch": true, "init": true, "merge": true,
	"mv": true, "pull": true, "push": true, "rebase": true, "remote": true,
	"reset": true, "restore": true, "revert": true, "rm": true, "stash": true,
	"switch": true, "tag": true, "clean": true,
}

// 🎯️blockedKillLsofPortPattern matches kill commands that use lsof to select a TCP port PID.
// This is intentionally denied because in containerized environments the matched PID can be critical (e.g. PID 1),
// ⏹️causing the entire devcontainer to terminate and stopping all running work.
var blockedKillLsofPortPattern = regexp.MustCompile(`(?i)\bkill\b[\s\S]*\$\(\s*lsof\b[\s\S]*-t[\s\S]*-i\s*:\s*\d+[\s\S]*\)`)

// 🔤️blockedGitVerbPattern matches any blocked git verb in inline code strings (shell-style invocation).
var blockedGitVerbPattern = regexp.MustCompile(`(?i)\bgit\s+(add|branch|checkout|cherry-pick|clone|commit|config|fetch|init|merge|mv|pull|push|rebase|remote|reset|restore|revert|rm|stash|switch|tag|clean)\b`)

// 📚️blockedGitListPattern matches array/list-style git invocations like ['git', 'stash'] or ["git", "checkout"].
var blockedGitListPattern = regexp.MustCompile(`(?i)['"]\s*git\s*['"]\s*,\s*['"]\s*(add|branch|checkout|cherry-pick|clone|commit|config|fetch|init|merge|mv|pull|push|rebase|remote|reset|restore|revert|rm|stash|switch|tag|clean)\s*['"]`)

// 🌿️containsBlockedGitInCode scans arbitrary inline code (e.g. python -c, node -e) for blocked git invocations.
func containsBlockedGitInCode(code string) (bool, string) {
	if m := blockedGitVerbPattern.FindString(code); m != "" {
		return true, "blocked: " + strings.ToLower(strings.TrimSpace(m))
	}
	if m := blockedGitListPattern.FindString(code); m != "" {
		return true, "blocked: git (list form) in inline code"
	}
	return false, ""
}

func isCommandSegmentBlocked(segment string) (bool, string) {
	segment = strings.TrimSpace(segment)
	if segment == "" {
		return false, ""
	}
	lower := strings.ToLower(segment)
	if blockedKillLsofPortPattern.MatchString(lower) {
		return true, "blocked: kill $(lsof -t -i:PORT); this can match PID 1 in containers and terminate the devcontainer, stopping all running work"
	}
	allowedPrefixes := []string{"grep ", "rg ", "ripgrep ", "echo ", "printf ", "ls ", "pwd", "cat ", "sed ", "awk "}
	for _, prefix := range allowedPrefixes {
		if strings.HasPrefix(lower, prefix) {
			return false, ""
		}
	}
	tokens := strings.Fields(lower)
	for len(tokens) > 0 {
		first := tokens[0]
		if strings.Contains(first, "=") {
			tokens = tokens[1:]
			continue
		}
		if first == "env" || first == "command" || first == "sudo" {
			tokens = tokens[1:]
			continue
		}
		// Shell interpreters: scan full joined string for git invocations.
		if first == "bash" || first == "sh" || first == "zsh" || first == "fish" ||
			first == "ksh" || first == "csh" || first == "tcsh" || first == "dash" {
			joined := strings.Join(tokens, " ")
			if idx := strings.Index(joined, "git "); idx >= 0 {
				return isCommandSegmentBlocked(joined[idx:])
			}
			return false, ""
		}
		// Script interpreters with inline code flags (-c, -e): scan inline code for git.
		if first == "python" || first == "python3" || first == "python2" ||
			first == "node" || first == "nodejs" ||
			first == "perl" || first == "ruby" || first == "php" ||
			first == "lua" || first == "tclsh" || first == "groovy" || first == "scala" {
			// Extract the inline code argument (after -c or -e flag).
			joined := strings.Join(tokens[1:], " ")
			return containsBlockedGitInCode(joined)
		}
		// xargs can forward arguments to git directly.
		if first == "xargs" {
			joined := strings.Join(tokens, " ")
			if idx := strings.Index(joined, "git "); idx >= 0 {
				return isCommandSegmentBlocked(joined[idx:])
			}
			return false, ""
		}
		break
	}
	if len(tokens) == 0 {
		return false, ""
	}
	if strings.HasSuffix(tokens[0], "/git") {
		tokens[0] = "git"
	}
	if tokens[0] != "git" {
		return false, ""
	}
	verbIndex := 1
	for verbIndex < len(tokens) && strings.HasPrefix(tokens[verbIndex], "-") {
		verbIndex++
		if verbIndex < len(tokens) && !strings.HasPrefix(tokens[verbIndex], "-") && (tokens[verbIndex-1] == "-c" || tokens[verbIndex-1] == "-C") {
			verbIndex++
		}
	}
	if verbIndex >= len(tokens) {
		return false, ""
	}
	verb := strings.Trim(tokens[verbIndex], `"'`)
	if !blockedGitVerbs[verb] {
		return false, ""
	}
	if verb == "clean" && !strings.Contains(lower, "-fd") && !strings.Contains(lower, "-df") {
		return false, ""
	}
	return true, "blocked: git " + verb
}

// #endregion 💾️Missing Utility Functions
