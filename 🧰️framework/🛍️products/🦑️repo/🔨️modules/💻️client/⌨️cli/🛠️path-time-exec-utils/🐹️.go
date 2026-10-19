// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Path, date, slug, exec, and output utilities.

// #endregion 🧲️Header

package client

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// 🟪️FormatSecond MUST complete the operation and return consistent results.
func FormatSecond(t time.Time) string {
	return fmt.Sprintf("🎆️%02d🌙️%02d☀️%02d⏰️%02d⌚️%02d⏱️%02d", t.Year()%100, t.Month(), t.Day(), t.Hour(), t.Minute(), t.Second())
}

// 🔤️FormatDate MUST produce a well-formed date string.
// 📩️FormatDate formats the date into its string representation.
func FormatDate(t time.Time) (year, month, day int) {
	return t.Year() % 100, int(t.Month()), t.Day()
}

// 🔢️PadNumber MUST complete the operation and return consistent results.
func PadNumber(n, width int) string {
	return fmt.Sprintf("%0*d", width, n)
}

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string {
	return EmojiYear + PadNumber(year, 2)
}

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string {
	return EmojiMonth + PadNumber(month, 2)
}

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string {
	return EmojiDay + PadNumber(day, 2)
}

// 🧭️parseDatedDir parses one canonical emoji-prefixed date directory segment.
func parseDatedDir(segment, prefix string) (int, error) {
	value := strings.TrimPrefix(segment, prefix)
	if value == segment {
		return 0, fmt.Errorf("date directory %q is missing prefix %q", segment, prefix)
	}
	parsed, err := strconv.Atoi(value)
	if err != nil {
		return 0, fmt.Errorf("invalid date directory %q: %w", segment, err)
	}
	return parsed, nil
}

// 🎫️FormatTicketRelPath returns YY/MM/DD/SLUG with emoji date segments.
func FormatTicketRelPath(year, month, day int, slug string) string {
	return FormatYearDir(year) + "/" + FormatMonthDir(month) + "/" + FormatDayDir(day) + "/" + slug
}

// 🔗️PathToUriPath MUST complete the operation and return consistent results.
// 🌐️PathToUriPath performs the path to uri path operation (no whitespace, reversible).
func PathToUriPath(path string) string {
	segments := strings.Split(path, "/")
	for i, s := range segments {
		segments[i] = strings.ReplaceAll(s, " ", "%20")
	}
	return strings.Join(segments, "/")
}

// 🟫️PathFromUriPath MUST complete the operation and return consistent results.
// 🟨️PathFromUriPath performs the uri path to path operation (reverse of PathToUriPath).
func PathFromUriPath(uriPath string) string {
	segments := strings.Split(uriPath, "/")
	for i, s := range segments {
		segments[i] = strings.ReplaceAll(s, "%20", " ")
	}
	return strings.Join(segments, "/")
}

// 😀️Flat MUST preserve only alphanumeric characters and emojis, then lower case.
func Flat(text string) string {
	var buf strings.Builder
	for _, r := range text {
		if (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') || r > 0x7F {
			buf.WriteRune(r)
		}
	}
	return strings.ToLower(buf.String())
}

// 💠️Slugify MUST complete the operation and return consistent results.
func Slugify(text string) string {
	runes := []rune(text)
	var buf strings.Builder
	for i, r := range runes {
		if i > 0 && r >= 'A' && r <= 'Z' {
			prev := runes[i-1]
			if prev >= 'a' && prev <= 'z' {
				buf.WriteRune('-')
			} else if prev >= 'A' && prev <= 'Z' && i+1 < len(runes) && runes[i+1] >= 'a' && runes[i+1] <= 'z' {
				buf.WriteRune('-')
			}
		}
		buf.WriteRune(r)
	}
	re := regexp.MustCompile(`[^A-Z0-9]+`)
	slug := re.ReplaceAllString(strings.ToUpper(buf.String()), "-")
	return strings.Trim(slug, "-")
}

// 📌️TitleizeSlug MUST complete the operation and return consistent results.
func TitleizeSlug(slug string) string {
	words := strings.Split(slug, "-")
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + strings.ToLower(w[1:])
		}
	}
	return strings.Join(words, " ")
}

// 🔳️StatutePathToIdValue MUST complete the operation and return consistent results.
func StatutePathToIdValue(path string) string {
	parts := strings.Split(path, "/")
	for i, p := range parts {
		parts[i] = TitleizeSlug(p)
	}
	return strings.Join(parts, "#")
}

// 🔲️StatuteIdValueToPath MUST complete the operation and return consistent results.
func StatuteIdValueToPath(idValue string) string {
	parts := strings.Split(idValue, "#")
	for i, p := range parts {
		parts[i] = strings.ToLower(strings.ReplaceAll(strings.TrimSpace(p), " ", "-"))
	}
	return strings.Join(parts, "/")
}

// ▪️ExecCommand MUST complete the operation and return consistent results.
func ExecCommand(command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	return ExecCommandContext(context.Background(), command, args, cwd)
}

// ▫️ExecCommandContext runs a child process with explicit cancellation and deadline propagation.
func ExecCommandContext(ctx context.Context, command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	if cwd == "" {
		cwd = rootDir
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

// 🔐️GetGitAuthorAlias MUST return the stored value without modification.
// 🟫️GetGitAuthorAlias returns the git author alias of the value.
func GetGitAuthorAlias() string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	name = strings.TrimSpace(name)
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	email = strings.TrimSpace(email)

	fallback := name
	if email != "" {
		fallback = fmt.Sprintf("%s <%s>", name, email)
	}

	return FindAndUpdateContributor(fallback)
}

// 💾️GetGitCheckpoint MUST return the stored value without modification.
// ✔️GetGitCheckpoint returns the git checkpoint of the value.
func GetGitCheckpoint() string {
	sha, _, _ := ExecCommand("git", []string{"rev-parse", "HEAD"}, "")
	return strings.TrimSpace(sha)
}

// 🗃️GetGitIgnoredSet MUST return the stored value without modification.
// ▫️GetGitIgnoredSet returns the git ignored set of the value.
func GetGitIgnoredSet(paths []string) map[string]bool {
	if len(paths) == 0 {
		return make(map[string]bool)
	}
	args := append([]string{"-c", "core.quotepath=false", "check-ignore"}, paths...)
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

// ▫️NewOutput MUST initialize all required fields and return a valid Output.
// 🆕️NewOutput creates and returns a new Output instance.
func NewOutput() *CommandOutput {
	return &CommandOutput{Lines: []OutputLine{}, ExitCode: 0}
}

// 📍️Info MUST return the metadata entry for the statute.
// ℹInfo returns the metadata for the statute.
func (o *CommandOutput) Info(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputInfo, Text: text})
}

// ◾Success MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Success(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputSuccess, Text: text})
}

// ❌️Error MUST return a formatted string representation.
// ❌️Error returns the string representation of the error.
func (o *CommandOutput) Error(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputError, Text: text})
	o.ExitCode = 1
}

// ⚠️Warn MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Warn(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputWarn, Text: text})
}

// ◽Plain MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Plain(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: text})
}

// ◻Print MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Print() {
	for _, line := range o.Lines {
		fmt.Println(line.Text)
	}
}

// ◼Json MUST operate on the CommandOutput receiver and return consistent results.
func (o *CommandOutput) Json(data interface{}) {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err == nil {
		o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: string(bytes)})
	}
}

// 📸️ListDirEntries MUST return a consistent snapshot of available entries.
// 🔵️ListDirEntries returns all available dir entries entries.
func ListDirEntries(dir string, dirsOnly bool) ([]string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasPrefix(e.Name(), ".") {
			continue
		}
		if dirsOnly && !e.IsDir() {
			continue
		}
		if !dirsOnly && e.IsDir() {
			continue
		}
		names = append(names, e.Name())
	}
	return names, nil
}

// ⏹️WalkDir MUST visit every entry and MUST stop when the callback returns an error.
// 🔬️WalkDir recursively walks the dir and invokes the callback.
func WalkDir(dir string, fn func(path string, isDir bool) error) error {
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if strings.HasPrefix(info.Name(), ".") {
			if info.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		return fn(path, info.IsDir())
	})
}

// 🔭️ParseScope MUST return an error when the input is malformed.
// 🔴️ParseScope parses the input and returns the scope result.
func ParseScope(raw string) Scope {
	if raw == "" || raw == "compose" {
		return Scope{Raw: "compose", Kind: ScopeRepo}
	}
	if strings.Contains(raw, "§") {
		parts := strings.SplitN(raw, "§", 2)
		return Scope{Raw: raw, Kind: ScopeDefinition, FilePath: parts[0], DefinitionName: parts[1]}
	}
	if strings.Contains(raw, "#") {
		parts := strings.Split(raw, "#")
		return Scope{Raw: raw, Kind: ScopeSection, FilePath: parts[0], SectionPath: parts[1:]}
	}
	ext := strings.ToLower(filepath.Ext(raw))
	codeExtensions := map[string]bool{".ts": true, ".tsx": true, ".js": true, ".jsx": true, ".py": true, ".cs": true, ".go": true, ".json": true, ".md": true, ".yaml": true, ".yml": true, ".sql": true, ".graphql": true}
	if codeExtensions[ext] {
		return Scope{Raw: raw, Kind: ScopeFile, FilePath: raw}
	}
	if strings.HasPrefix(raw, "compose/") {
		return Scope{Raw: raw, Kind: ScopeTechnology, TechnologyName: raw}
	}
	if strings.HasSuffix(raw, "/") {
		return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
	}
	return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
}

// 🔵️ReadLines MUST return the full content from the given path.
// 🟠️ReadLines reads and returns the lines content.
func ReadLines(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}
