// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package workspace owns repository-root discovery, the `.🧬semio` layout, `📋️config.toml`
// settings, and the owned glob and ignore primitives every other repo module matches paths with.

// #endregion 🧲️Header

package workspace

import (
	bufio "bufio"
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	fs "io/fs"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strconv "strconv"
	strings "strings"
	sync "sync"
	time "time"
)

// #region 🖍️Pattern

// 🪧️maxCachedPatterns bounds the compiled-pattern cache so a pathological caller cannot grow it forever.
const maxCachedPatterns = 256

var patternCache = struct {
	sync.RWMutex
	values map[string]*regexp.Regexp
}{values: map[string]*regexp.Regexp{}}

// 🪧️maxBraceExpansions bounds brace alternation so a pathological pattern cannot explode; a pattern
// that would expand past it keeps its braces literal instead.
const maxBraceExpansions = 1024

// 🃏️Match reports whether name satisfies a slash-separated glob pattern. Brace alternation
// (`{a,b}`, nested and possibly empty) stands for the set of brace-free patterns it expands to and
// matches when any of them matches. An unpaired brace, and a group without a top-level comma, are
// literal. Escaping a brace with a backslash only survives where the platform keeps backslashes out
// of the separator normalisation, so it is not a portable spelling.
func Match(pattern, name string) (bool, error) {
	target := filepath.ToSlash(name)
	for _, expanded := range expandBraces(pattern) {
		expression, err := compile(expanded)
		if err != nil {
			return false, err
		}
		if expression.MatchString(target) {
			return true, nil
		}
	}
	return false, nil
}

// 🪄️expandBraces rewrites brace alternation into the brace-free patterns it stands for.
func expandBraces(pattern string) []string {
	if !strings.ContainsRune(pattern, '{') {
		return []string{pattern}
	}
	expanded := make([]string, 0, 1)
	queue := []string{pattern}
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		runes := []rune(current)
		start, end, found := braceGroup(runes)
		if !found {
			expanded = append(expanded, current)
			continue
		}
		prefix := string(runes[:start])
		suffix := string(runes[end+1:])
		for _, alternative := range braceAlternatives(runes[start+1 : end]) {
			queue = append(queue, prefix+alternative+suffix)
		}
		if len(expanded)+len(queue) > maxBraceExpansions {
			return []string{pattern}
		}
	}
	return expanded
}

// 🔎️braceGroup returns the rune bounds of the first closed brace group carrying a top-level comma.
func braceGroup(runes []rune) (int, int, bool) {
	for index := 0; index < len(runes); index++ {
		switch runes[index] {
		case '\\':
			index++
		case '{':
			if end, comma, closed := braceEnd(runes, index); closed && comma {
				return index, end, true
			}
		}
	}
	return 0, 0, false
}

// 🧮️braceEnd scans from an opening brace to its partner, reporting whether it closed at all and
// whether the body it encloses carries a comma at its own nesting level.
func braceEnd(runes []rune, start int) (int, bool, bool) {
	depth := 0
	class := false
	comma := false
	for cursor := start; cursor < len(runes); cursor++ {
		switch runes[cursor] {
		case '\\':
			cursor++
		case '[':
			class = true
		case ']':
			class = false
		case '{':
			if !class {
				depth++
			}
		case '}':
			if !class {
				depth--
				if depth == 0 {
					return cursor, comma, true
				}
			}
		case ',':
			if !class && depth == 1 {
				comma = true
			}
		}
	}
	return 0, false, false
}

// ✂️braceAlternatives splits a brace body on the commas that belong to it rather than to a nested group.
func braceAlternatives(body []rune) []string {
	alternatives := []string{}
	depth := 0
	class := false
	start := 0
	for cursor := 0; cursor < len(body); cursor++ {
		switch body[cursor] {
		case '\\':
			cursor++
		case '[':
			class = true
		case ']':
			class = false
		case '{':
			if !class {
				depth++
			}
		case '}':
			if !class && depth > 0 {
				depth--
			}
		case ',':
			if !class && depth == 0 {
				alternatives = append(alternatives, string(body[start:cursor]))
				start = cursor + 1
			}
		}
	}
	return append(alternatives, string(body[start:]))
}

// 🛠️compile translates a glob pattern into an anchored regular expression, memoised per pattern.
func compile(pattern string) (*regexp.Regexp, error) {
	pattern = filepath.ToSlash(pattern)
	patternCache.RLock()
	cached := patternCache.values[pattern]
	patternCache.RUnlock()
	if cached != nil {
		return cached, nil
	}
	runes := []rune(pattern)
	var expression strings.Builder
	expression.WriteString("^")
	for index := 0; index < len(runes); index++ {
		switch runes[index] {
		case '*':
			if index+1 < len(runes) && runes[index+1] == '*' {
				index++
				if index+1 < len(runes) && runes[index+1] == '/' {
					index++
					expression.WriteString("(?:.*/)?")
				} else {
					expression.WriteString(".*")
				}
			} else {
				expression.WriteString("[^/]*")
			}
		case '?':
			expression.WriteString("[^/]")
		case '[':
			end := -1
			for cursor := index + 1; cursor < len(runes); cursor++ {
				if runes[cursor] == ']' {
					end = cursor
					break
				}
			}
			if end < 0 {
				return nil, fmt.Errorf("invalid glob %q: unclosed character class", pattern)
			}
			class := string(runes[index+1 : end])
			if strings.HasPrefix(class, "!") {
				class = "^" + regexp.QuoteMeta(class[1:])
			}
			expression.WriteString("[" + class + "]")
			index = end
		case '\\':
			if index+1 >= len(runes) {
				return nil, fmt.Errorf("invalid glob %q: trailing escape", pattern)
			}
			index++
			expression.WriteString(regexp.QuoteMeta(string(runes[index])))
		default:
			expression.WriteString(regexp.QuoteMeta(string(runes[index])))
		}
	}
	expression.WriteString("$")
	compiled, err := regexp.Compile(expression.String())
	if err != nil {
		return nil, err
	}
	patternCache.Lock()
	if len(patternCache.values) >= maxCachedPatterns {
		clear(patternCache.values)
	}
	patternCache.values[pattern] = compiled
	patternCache.Unlock()
	return compiled, nil
}

// #endregion 🖍️Pattern

// #region 🗂️Traversal

// 🗺️FilepathGlob walks the static prefix of the pattern and returns every matching path, sorted.
func FilepathGlob(pattern string) ([]string, error) {
	return FilepathGlobContext(context.Background(), pattern, nil)
}

// 🚦️FilepathGlobContext walks the static prefix of the pattern with cancellation and progress reporting.
func FilepathGlobContext(ctx context.Context, pattern string, progress func(int)) ([]string, error) {
	root := traversalRoot(pattern)
	if _, err := os.Stat(root); err != nil {
		return nil, nil
	}
	var matches []string
	visited := 0
	err := filepath.WalkDir(root, func(path string, entry fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if err := ctx.Err(); err != nil {
			return err
		}
		visited++
		if progress != nil {
			progress(visited)
		}
		matched, err := Match(pattern, path)
		if err != nil {
			return err
		}
		if matched {
			matches = append(matches, path)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	sort.Strings(matches)
	return matches, nil
}

// 🌱️traversalRoot returns the longest wildcard-free directory prefix of a pattern.
func traversalRoot(pattern string) string {
	normalized := filepath.Clean(pattern)
	volume := filepath.VolumeName(normalized)
	parts := strings.Split(strings.TrimPrefix(normalized, volume+string(filepath.Separator)), string(filepath.Separator))
	root := volume
	if filepath.IsAbs(normalized) {
		root += string(filepath.Separator)
	}
	for _, part := range parts {
		if strings.ContainsAny(part, "*?[") {
			break
		}
		root = filepath.Join(root, part)
	}
	if root == "" {
		return "."
	}
	info, err := os.Stat(root)
	if err == nil && !info.IsDir() {
		return filepath.Dir(root)
	}
	return root
}

// #endregion 🗂️Traversal

// #region 🙈️Ignore

// 📏️rule is one compiled ignore line together with its negation flag.
type rule struct {
	pattern string
	negated bool
	literal string
}

// 🙈️GitIgnore is an ordered ignore rule set in which the last matching rule decides.
type GitIgnore struct{ rules []rule }

// 📄️CompileIgnoreFile parses an ignore file into an ordered rule set.
func CompileIgnoreFile(path string) (*GitIgnore, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	result := &GitIgnore{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		result.addLine(scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return result, nil
}

// 📝️CompileIgnoreLines parses ignore lines already held in memory into an ordered rule set.
func CompileIgnoreLines(lines ...string) *GitIgnore {
	result := &GitIgnore{}
	for _, line := range lines {
		result.addLine(line)
	}
	return result
}

// ➕️addLine normalises one ignore line and appends it unless it is blank or a comment.
func (ignore *GitIgnore) addLine(raw string) {
	line := strings.TrimSpace(raw)
	if line == "" || strings.HasPrefix(line, "#") {
		return
	}
	item := rule{}
	if strings.HasPrefix(line, "!") {
		item.negated = true
		line = strings.TrimPrefix(line, "!")
	}
	line = filepath.ToSlash(strings.TrimPrefix(line, "/"))
	if strings.HasSuffix(line, "/") {
		line += "**"
	}
	if !strings.Contains(line, "/") {
		line = "**/" + line
	}
	item.pattern = line
	item.literal = requiredLiteral(line)
	ignore.rules = append(ignore.rules, item)
}

// 🔤️requiredLiteral MUST answer a run the pattern demands, or the empty string.
//
// 🔤️requiredLiteral returns the longest run of literal characters a pattern requires, so a path
// that does not carry it cannot match and never has to be matched. A pattern carrying brace
// alternation has no single required run and is answered with the empty string, which disables the
// prefilter for that rule. This is what keeps a 625-rule ignore file affordable against the half a
// million historical paths a `loc` walk classifies.
func requiredLiteral(pattern string) string {
	if strings.ContainsAny(pattern, "{}") {
		return ""
	}
	longest := ""
	current := strings.Builder{}
	runes := []rune(pattern)
	flush := func() {
		if current.Len() > len(longest) {
			longest = current.String()
		}
		current.Reset()
	}
	for index := 0; index < len(runes); index++ {
		switch runes[index] {
		case '[':
			flush()
			for index < len(runes) && runes[index] != ']' {
				index++
			}
		case '?':
			flush()
		case '*':
			flush()
			for index+1 < len(runes) && runes[index+1] == '*' {
				index++
			}
			if index+1 < len(runes) && runes[index+1] == '/' {
				index++
			}
		default:
			current.WriteRune(runes[index])
		}
	}
	flush()
	return longest
}

// 🔍️MatchesPath reports whether the last matching rule ignores the path.
func (ignore *GitIgnore) MatchesPath(path string) bool {
	path = filepath.ToSlash(strings.TrimPrefix(path, "./"))
	matched := false
	for _, item := range ignore.rules {
		if item.literal != "" && !strings.Contains(path, item.literal) {
			continue
		}
		ok, err := Match(item.pattern, path)
		if err == nil && ok {
			matched = !item.negated
		}
	}
	return matched
}

// #endregion 🙈️Ignore

// #region 🧭️Layout

// 🧬️SemioDirName is the workspace-local semio directory inside a monorepo root.
const SemioDirName = ".🧬semio"

// 🦑️RepoDirName is the repo product directory inside the semio directory.
const RepoDirName = "🦑️repo"

// 🎫️TicketsDirName is the ticket collection directory inside the repo meta directory.
const TicketsDirName = "🎫️tickets"

// 🎯️GoalsDirName is the goal collection directory inside the repo meta directory.
const GoalsDirName = "🎯️goals"

// 🧑️DevsDirName is the contributor collection directory inside the repo meta directory.
const DevsDirName = "🧑️‍💻️devs"

// 📁️FilesIndexName is the codebase file index inside the repo meta directory.
const FilesIndexName = "📁️files.json"

// ⚙️ConfigFileName is the repo settings file inside the repo meta directory.
const ConfigFileName = "📋️config.toml"

// 🧬️SemioDirForRoot returns the workspace-local semio root under an explicit monorepo root.
func SemioDirForRoot(repoRoot string) string {
	return filepath.Join(repoRoot, SemioDirName)
}

// 📦️RepoMetaDirForRoot returns the repo meta dir for an explicit monorepo root.
func RepoMetaDirForRoot(repoRoot string) string {
	return filepath.Join(repoRoot, SemioDirName, RepoDirName)
}

// 🛤️RepoMetaPathForRoot joins a relative path onto the repo meta dir of an explicit monorepo root.
func RepoMetaPathForRoot(repoRoot string, path string) string {
	return filepath.Join(RepoMetaDirForRoot(repoRoot), path)
}

// 🎫️TicketsDirForRoot returns the ticket collection directory of an explicit monorepo root.
func TicketsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, TicketsDirName)
}

// 🎯️GoalsDirForRoot returns the goal collection directory of an explicit monorepo root.
func GoalsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, GoalsDirName)
}

// 🧑️DevsDirForRoot returns the contributor collection directory of an explicit monorepo root.
func DevsDirForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, DevsDirName)
}

// 📁️FilesIndexForRoot returns the codebase file index path of an explicit monorepo root.
func FilesIndexForRoot(repoRoot string) string {
	return RepoMetaPathForRoot(repoRoot, FilesIndexName)
}

// #endregion 🧭️Layout

// #region 🧭️Discovery

// 🧭️FindRepoRoot walks upwards from startDir and returns the first monorepo root it recognises.
func FindRepoRoot(startDir string) string {
	if strings.TrimSpace(startDir) == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return "."
		}
		startDir = cwd
	}
	dir, err := filepath.Abs(startDir)
	if err != nil {
		return startDir
	}
	if found, ok := ascend(dir, func(candidate string) bool {
		info, statErr := os.Stat(filepath.Join(candidate, "repo", "cli", "main.go"))
		return statErr == nil && !info.IsDir()
	}); ok {
		return found
	}
	if found, ok := ascend(dir, func(candidate string) bool {
		_, statErr := os.Stat(filepath.Join(candidate, ".git"))
		return statErr == nil
	}); ok {
		return found
	}
	if found, ok := ascend(dir, func(candidate string) bool {
		_, statErr := os.Stat(filepath.Join(candidate, "go.mod"))
		return statErr == nil
	}); ok {
		return found
	}
	return dir
}

// ⬆️ascend walks from dir to the filesystem root and returns the first directory accepted by match.
func ascend(dir string, match func(string) bool) (string, bool) {
	searchDir := dir
	for {
		if match(searchDir) {
			return searchDir, true
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			return "", false
		}
		searchDir = parent
	}
}

// #endregion 🧭️Discovery

// #region ⚙️RepoConfig

// 📋️LoggingConfig holds hook logging switches and detail for `📋️config.toml` `[logging]`.
type LoggingConfig struct {
	Session    bool
	Operations bool
	Plan       bool
	Detail     string
}

// ⚙️RepoConfig holds repo-wide settings loaded from `.🧬semio/🦑️repo/📋️config.toml`.
type RepoConfig struct {
	Logging LoggingConfig
}

// 🔷️DefaultRepoConfig returns the settings used when the config file is missing.
func DefaultRepoConfig() RepoConfig {
	return RepoConfig{
		Logging: LoggingConfig{
			Session:    false,
			Operations: true,
			Plan:       true,
			Detail:     "standard",
		},
	}
}

// ✅️parseRepoConfigBool reads the affirmative spellings a config value may use.
func parseRepoConfigBool(value string) bool {
	switch strings.ToLower(strings.TrimSpace(value)) {
	case "true", "yes", "1", "on":
		return true
	default:
		return false
	}
}

// ✂️unquoteRepoConfigValue strips one matching pair of surrounding quotes.
func unquoteRepoConfigValue(value string) string {
	value = strings.TrimSpace(value)
	if len(value) >= 2 {
		if (value[0] == '"' && value[len(value)-1] == '"') || (value[0] == '\'' && value[len(value)-1] == '\'') {
			return value[1 : len(value)-1]
		}
	}
	return value
}

// 📥️LoadRepoConfig reads `.🧬semio/🦑️repo/📋️config.toml` under repoRoot; a missing file yields defaults.
func LoadRepoConfig(repoRoot string) RepoConfig {
	if strings.TrimSpace(repoRoot) == "" {
		return DefaultRepoConfig()
	}
	data, err := os.ReadFile(RepoMetaPathForRoot(repoRoot, ConfigFileName))
	if err != nil {
		return DefaultRepoConfig()
	}
	return ParseRepoConfig(string(data))
}

// 📜️ParseRepoConfig applies the recognised `[logging]` keys of a config document onto the defaults.
func ParseRepoConfig(document string) RepoConfig {
	config := DefaultRepoConfig()
	section := ""
	for _, line := range strings.Split(document, "\n") {
		trim := strings.TrimSpace(line)
		if trim == "" || strings.HasPrefix(trim, "#") {
			continue
		}
		if strings.HasPrefix(trim, "[") && strings.HasSuffix(trim, "]") {
			section = strings.ToLower(strings.Trim(trim, "[]"))
			continue
		}
		if !strings.Contains(trim, "=") {
			continue
		}
		parts := strings.SplitN(trim, "=", 2)
		key := strings.ToLower(strings.TrimSpace(parts[0]))
		value := unquoteRepoConfigValue(parts[1])
		if section != "logging" {
			continue
		}
		switch key {
		case "session":
			config.Logging.Session = parseRepoConfigBool(value)
		case "operations":
			config.Logging.Operations = parseRepoConfigBool(value)
		case "plan":
			config.Logging.Plan = parseRepoConfigBool(value)
		case "detail":
			if value != "" {
				config.Logging.Detail = value
			}
		}
	}
	return config
}

// 📤️IncludeResponse reports whether hook logs carry the response payload at this detail level.
func (config LoggingConfig) IncludeResponse() bool {
	return strings.ToLower(strings.TrimSpace(config.Detail)) != "minimal"
}

// 🔬️IncludeNative reports whether hook logs carry the native payload at this detail level.
func (config LoggingConfig) IncludeNative() bool {
	return strings.ToLower(strings.TrimSpace(config.Detail)) == "full"
}

// #endregion ⚙️RepoConfig

// #region 🚚️Split

// #region 🌧️Cli Adapter

// 💿️ExitError holds the data fields for a exit error record.
type ExitError struct {
	Code int
}

// 🔤️Error MUST return a formatted string representation.
// ❌️Error returns the string representation of the error.
func (e ExitError) Error() string {
	return fmt.Sprintf("exit status %d", e.Code)
}

// #endregion 🌧️Cli Adapter

// #region 🎼️Utilities

// #region 🎼️Utilities
// General-purpose utility functions for time parsing and formatting.
// ⏰️parseFlexibleTime holds the data fields for a parseFlexibleTime record.
func ParseFlexibleTime(t string) (time.Time, error) {
	if t == "" {
		return time.Time{}, fmt.Errorf("empty time string")
	}
	layouts := []string{
		time.RFC3339,
		"2006-01-02",
		"2006-01-02 15:04:05",
	}
	for _, layout := range layouts {
		if parsed, err := time.Parse(layout, t); err == nil {
			return parsed, nil
		}
	}
	return time.Time{}, fmt.Errorf("could not parse time: %s", t)
}

// #endregion 🎼️Utilities

// #region 🌩️CLI Renderers

// ❌️toolErrorResult holds the data fields for a toolErrorResult record.
func ToolErrorResult(err error) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %v", err))
	return ToolResult{Output: *output, Error: err.Error()}
}

// 🔺️toolErrorMsg holds the data fields for a toolErrorMsg record.
func ToolErrorMsg(msg string) ToolResult {
	output := NewOutput()
	output.Error(fmt.Sprintf("Error: %s", msg))
	return ToolResult{Output: *output, Error: msg}
}

// #endregion 🌩️CLI Renderers

// #region 🎨️Drafts

// 📝️GetDraftsPath MUST return the stored value without modification.
// 📦️GetDraftsPath returns the drafts path of the value.
func GetDraftsPath() string {
	return GetRepoMetaPath("✍️notes")
}

// #endregion 🎨️Drafts

// #region ⚙️Types

// 🔭️ScopeKind represents a scope kind value.
type ScopeKind string

const ScopeRepo ScopeKind = "repo"

const ScopeTechnology ScopeKind = "bundle"

const ScopeFolder ScopeKind = "folder"

const ScopeFile ScopeKind = "file"

const ScopeSection ScopeKind = "section"

const ScopeDefinition ScopeKind = "definition"

// 💿️Scope holds the data fields for a scope record.
type Scope struct {
	Raw            string    `json:"raw"`
	Kind           ScopeKind `json:"kind"`
	TechnologyName string    `json:"technologyName,omitempty"`
	FilePath       string    `json:"filePath,omitempty"`
	SectionPath    []string  `json:"sectionPath,omitempty"`
	DefinitionName string    `json:"definitionName,omitempty"`
}

// #endregion ⚙️Types

// #region 🎽️Languages

// 💚️OutputType represents a output type value.
type OutputType string

const OutputInfo OutputType = "info"

const OutputSuccess OutputType = "success"

const OutputError OutputType = "error"

const OutputWarn OutputType = "warn"

const OutputPlain OutputType = "plain"

// 💛️OutputLine holds the data fields for a output line record.
type OutputLine struct {
	Type OutputType `json:"type"`
	Text string     `json:"text"`
}

// 🧡️CommandOutput holds the data fields for a command output record.
type CommandOutput struct {
	Lines    []OutputLine `json:"lines"`
	ExitCode int          `json:"exitCode"`
}

// ❤️ToolResult holds the data fields for a tool result record.
type ToolResult struct {
	Output CommandOutput `json:"output"`
	Data   interface{}   `json:"data,omitempty"`
	Error  string        `json:"error,omitempty"`
}

// #endregion 🎽️Languages

// #region 📦️Utils

var RootDir string

var ExecutorOnce sync.Once

var FormatterBinaryLookup = exec.LookPath

var FormatterCommandRun = runFormatterCommand

// ⚠️writeWarningf writes operational diagnostics without contaminating structured stdout.
func WriteWarningf(format string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, "Warning: "+format+"\n", args...)
}

// 💿️init holds the data fields for a init record.
func init() {
	wd, err := os.Getwd()
	if err != nil {
		RootDir = "."
	} else {
		RootDir = FindRepoRoot(wd)
	}
	SetRootDir(RootDir)
}

// 🏪️GetRootDir MUST return the stored value without modification.
// 📖️GetRootDir returns the root dir of the value.
func GetRootDir() string {
	return RootDir
}

// 📥️SetRootDir MUST update the value on the receiver.
// 🗺️SetRootDir sets the root dir on the value.
func SetRootDir(dir string) {
	RootDir = FindRepoRoot(dir)
	GitignoreMutex.Lock()
	CachedGitignore = nil
	GitignoreLoaded = false
	GitignoreMutex.Unlock()
	notifyRootDirWatchers()
}

// 🔷️GetSemioRootDir MUST return the stored value without modification.
// 🧬️GetSemioRootDir returns the workspace-local semio root under the monorepo.
func GetSemioRootDir() string {
	return filepath.Join(GetRootDir(), ".🧬semio")
}

// 🔷️GetRepoMetaDir MUST return the stored value without modification.
// 📦️GetRepoMetaDir returns the repo meta dir of the value.
func GetRepoMetaDir() string {
	return RepoMetaDirForRoot(GetRootDir())
}

// 🛤️GetRepoMetaPath MUST return the stored value without modification.
// 📰️GetRepoMetaPath returns the repo meta path of the value.
func GetRepoMetaPath(path string) string {
	return filepath.Join(GetRepoMetaDir(), path)
}

var CachedGitignore *GitIgnore

var GitignoreLoaded bool

var GitignoreMutex sync.Mutex

// 🐙️getGitignore holds the data fields for a getGitignore record.
func getGitignore() *GitIgnore {
	GitignoreMutex.Lock()
	defer GitignoreMutex.Unlock()
	if GitignoreLoaded {
		return CachedGitignore
	}
	gitignorePath := filepath.Join(RootDir, ".gitignore")
	ign, err := CompileIgnoreFile(gitignorePath)
	if err != nil {
		GitignoreLoaded = true
		return nil
	}
	CachedGitignore = ign
	GitignoreLoaded = true
	return CachedGitignore
}

// 🔶️isGitIgnored holds the data fields for a isGitIgnored record.
func IsGitIgnored(filePath string) bool {
	if filepath.Base(filePath) == "LICENSE.md" {
		return true
	}
	return IsIgnoredByGitignore(filePath)
}

func IsIgnoredByGitignore(filePath string) bool {
	relPath := NormalizeRepoPath(filePath)
	if relPath == "" {
		return false
	}
	ign := getGitignore()
	if ign == nil {
		return false
	}
	return ign.MatchesPath(relPath)
}

// 📄️isSourceFile holds the data fields for a isSourceFile record.
func IsSourceFile(filePath string) bool {
	ext := filepath.Ext(filePath)
	return ext == ".ts" || ext == ".tsx" || ext == ".js" || ext == ".jsx" ||
		ext == ".py" || ext == ".go" || ext == ".cs"
}

// 📖️NormalizePath MUST be idempotent for already-normalized values.
// 📝️NormalizePath normalizes the path to its canonical form.
func NormalizePath(p string) string {
	return strings.ReplaceAll(p, "\\", "/")
}

// 🎛️EnsureDir MUST be idempotent and MUST NOT fail if the target already exists.
// ❓️EnsureDir ensures the dir exists, creating it if necessary.
func EnsureDir(dirPath string) error {
	return os.MkdirAll(dirPath, 0755)
}

// 🔹️GetRelativePath MUST return the stored value without modification.
// 🔷️GetRelativePath returns the relative path of the value.
func GetRelativePath(filePath string) string {
	rel, err := filepath.Rel(RootDir, filePath)
	if err != nil {
		return filePath
	}
	return NormalizePath(rel)
}

// 📝️ReadTextFile MUST return the full content from the given path.
// 🔤️ReadTextFile reads and returns the text file content.
func ReadTextFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// ✏️WriteTextFile MUST persist the content atomically.
// 💾️WriteTextFile writes the text file content to storage.
func WriteTextFile(filePath string, content string) error {
	if err := EnsureDir(filepath.Dir(filePath)); err != nil {
		return err
	}
	return os.WriteFile(filePath, []byte(content), 0644)
}

// 📋️formatterPlan holds the data fields for a formatterPlan record.
type formatterPlan struct {
	Binary      string
	Args        []string
	requirement []string
}

// 🔸️runFormatterCommand holds the data fields for a runFormatterCommand record.
func runFormatterCommand(binary string, args []string, workDir string) error {
	commandPath := binary
	if strings.Contains(binary, "/") || strings.Contains(binary, "\\") {
		commandPath = filepath.Join(workDir, binary)
	}
	cmd := exec.Command(commandPath, args...)
	cmd.Dir = workDir
	return cmd.Run()
}

// 🗣️formatterPlansForLanguage holds the data fields for a formatterPlansForLanguage record.
func FormatterPlansForLanguage(languageName string, relPath string) []formatterPlan {
	prettierBinary := filepath.Join("node_modules", ".bin", "prettier")
	prettierPlan := formatterPlan{
		Binary:      prettierBinary,
		Args:        []string{"--write", "--ignore-unknown", relPath},
		requirement: []string{prettierBinary, ".prettierrc.json"},
	}
	switch languageName {
	case "typescript", "markdown", "json", "yaml", "graphql", "sql", "toml":
		return []formatterPlan{prettierPlan}
	case "go":
		return []formatterPlan{
			{Binary: "gofmt", Args: []string{"-w", relPath}, requirement: []string{"go.work"}},
		}
	case "python":
		return []formatterPlan{
			{Binary: "uv", Args: []string{"run", "--group", "dev", "ruff", "format", relPath}, requirement: []string{"pyproject.toml"}},
			{Binary: "uv", Args: []string{"run", "--group", "dev", "black", relPath}, requirement: []string{"pyproject.toml"}},
			prettierPlan,
		}
	case "csharp":
		return []formatterPlan{
			{Binary: "dotnet", Args: []string{"format", "Monorepo.sln", "--include", relPath, "--verbosity", "minimal"}, requirement: []string{"Monorepo.sln"}},
			prettierPlan,
		}
	case "rust":
		return []formatterPlan{
			{Binary: "rustfmt", Args: []string{relPath}, requirement: []string{"Cargo.toml"}},
			prettierPlan,
		}
	case "ruby":
		return []formatterPlan{
			{Binary: "rubocop", Args: []string{"-A", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	case "shell":
		return []formatterPlan{
			{Binary: "shfmt", Args: []string{"-w", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	default:
		return []formatterPlan{prettierPlan}
	}
}

// 🔺️isFormatterPlanAvailable holds the data fields for a isFormatterPlanAvailable record.
func IsFormatterPlanAvailable(plan formatterPlan, workDir string) bool {
	if strings.Contains(plan.Binary, "/") || strings.Contains(plan.Binary, "\\") {
		if !FileExists(filepath.Join(workDir, plan.Binary)) {
			return false
		}
	} else {
		if _, err := FormatterBinaryLookup(plan.Binary); err != nil {
			return false
		}
	}
	for _, rel := range plan.requirement {
		if !FileExists(filepath.Join(workDir, rel)) {
			return false
		}
	}
	return true
}

// 🔻️fallbackFormatText holds the data fields for a fallbackFormatText record.
func FallbackFormatText(content string) string {
	lines := strings.Split(content, "\n")
	for i, line := range lines {
		lines[i] = strings.TrimRight(line, " \t")
	}
	formatted := strings.Join(lines, "\n")
	if formatted == "" {
		return formatted
	}
	if !strings.HasSuffix(formatted, "\n") {
		formatted += "\n"
	}
	return formatted
}

// ⬜️WriteJSONFile MUST persist the content atomically.
// 🔹️WriteJSONFile writes the j s o n file content to storage.
func WriteJSONFile(filePath string, data interface{}) error {
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(filePath, string(jsonBytes)+"\n")
}

// 🟥️ReadJSONFile MUST return the full content from the given path.
// ⬛️ReadJSONFile reads and returns the j s o n file content.
func ReadJSONFile(filePath string, v interface{}) error {
	data, err := ReadTextFile(filePath)
	if err != nil {
		return err
	}
	return json.Unmarshal([]byte(data), v)
}

// 🟧️FileExists MUST complete the operation and return consistent results.
func FileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// 🟨️IsDir MUST return true only when the condition is met.
// ⬜️IsDir reports whether the value is dir.
func IsDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}

// ⚙️LoadGitignore MUST read from the configured storage path.
// 🟥️LoadGitignore loads the gitignore from storage.
func LoadGitignore(cwd string) ([]string, error) {
	gitignorePath := filepath.Join(cwd, ".gitignore")
	if !FileExists(gitignorePath) {
		return nil, nil
	}
	content, err := ReadTextFile(gitignorePath)
	if err != nil {
		return nil, err
	}
	var patterns []string
	for _, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			patterns = append(patterns, line)
		}
	}
	return patterns, nil
}

func matchesIgnorePattern(path string, isDir bool, pattern string) bool {
	pattern = NormalizePath(strings.TrimSpace(pattern))
	if pattern == "" {
		return false
	}
	path = NormalizePath(strings.TrimSpace(path))
	if path == "" {
		return false
	}
	candidates := []string{path}
	if isDir && !strings.HasSuffix(path, "/") {
		candidates = append(candidates, path+"/")
	}
	patterns := []string{pattern}
	if strings.HasSuffix(pattern, "/**") {
		base := strings.TrimSuffix(pattern, "/**")
		if base != "" {
			patterns = append(patterns, base, base+"/")
		}
	}
	for _, candidatePattern := range patterns {
		if candidatePattern == "" {
			continue
		}
		for _, candidatePath := range candidates {
			if matched, _ := Match(candidatePattern, candidatePath); matched {
				return true
			}
		}
	}
	return false
}

// 🟩️SimpleGlob MUST complete the operation and return consistent results.
func SimpleGlob(pattern string, cwd string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	if cwd == "" {
		cwd = RootDir
	}
	var gitignorePatterns []string
	if respectGitignore {
		var err error
		gitignorePatterns, err = LoadGitignore(cwd)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	var files []string
	absPattern := filepath.Join(cwd, pattern)
	matches, err := FilepathGlob(absPattern)
	if err != nil {
		return nil, err
	}
	for _, match := range matches {
		rel, err := filepath.Rel(cwd, match)
		if err != nil {
			continue
		}
		relNorm := NormalizePath(rel)
		ignored := false
		for _, ig := range allIgnore {
			if matchesIgnorePattern(relNorm, false, ig) {
				ignored = true
				break
			}
		}
		if !ignored {
			files = append(files, relNorm)
		}
	}
	return files, nil
}

// 🟦️globByExtension holds the data fields for a globByExtension record.
func GlobByExtension(root string, patternBase string, exts []string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	base := strings.TrimSuffix(patternBase, "/**/*")
	if patternBase == "**/*" {
		base = ""
	}
	absBase := filepath.Join(root, base)
	info, err := os.Stat(absBase)
	if err != nil || !info.IsDir() {
		return nil, nil
	}
	gitignorePatterns := []string{}
	if respectGitignore {
		gitignorePatterns, err = LoadGitignore(root)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	allowed := make(map[string]struct{}, len(exts))
	for _, ext := range exts {
		allowed[strings.ToLower(ext)] = struct{}{}
	}
	results := make([]string, 0)
	err = filepath.WalkDir(absBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		name := d.Name()
		if d.IsDir() && strings.HasPrefix(name, ".") {
			return filepath.SkipDir
		}
		rel, relErr := filepath.Rel(root, path)
		if relErr != nil {
			return nil
		}
		rel = strings.TrimPrefix(NormalizePath(rel), "./")
		if rel == "" || rel == "." {
			return nil
		}
		for _, ig := range allIgnore {
			if matchesIgnorePattern(rel, d.IsDir(), ig) {
				if d.IsDir() {
					return filepath.SkipDir
				}
				return nil
			}
		}
		if d.IsDir() {
			return nil
		}
		ext := strings.TrimPrefix(strings.ToLower(filepath.Ext(rel)), ".")
		if _, ok := allowed[ext]; !ok {
			return nil
		}
		results = append(results, rel)
		return nil
	})
	if err != nil {
		return nil, err
	}
	return results, nil
}

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

// 🧭️parseDatedDir parses one canonical emoji-prefixed date directory segment.
func ParseDatedDir(segment, prefix string) (int, error) {
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
		cwd = RootDir
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
	root := RootDir
	if strings.TrimSpace(root) == "" {
		return false
	}
	if cached, ok := gitTopLevelCache.Load(root); ok {
		return cached.(bool)
	}
	stdout, _, exitCode := ExecCommand("git", []string{"rev-parse", "--show-toplevel"}, root)
	verdict := false
	if exitCode == 0 {
		top, err := filepath.Abs(strings.TrimSpace(stdout))
		if err == nil {
			current, absErr := filepath.Abs(root)
			verdict = absErr == nil && NormalizePath(top) == NormalizePath(current)
		}
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

// #endregion 📦️Utils

// #region 📋️Tickets

// 🔻️CountLines MUST complete the operation and return consistent results.
func CountLines(content string) int {
	if content == "" {
		return 0
	}
	return strings.Count(content, "\n") + 1
}

// 📄️CountLinesInFile MUST complete the operation and return consistent results.
func CountLinesInFile(path string) int {
	content, err := ReadTextFile(path)
	if err != nil {
		return 0
	}
	return CountLines(content)
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
		return ReadTextFile(filepath.Join(RootDir, filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", checkpoint, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

// 🗃️AGPLLicenseText MUST complete the operation successfully.
func AGPLLicenseText() string {
	return `This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.`
}

// 🔤️UniqueStrings MUST complete the operation successfully.
func UniqueStrings(input []string) []string {
	keys := make(map[string]bool)
	list := []string{}
	for _, entry := range input {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
}

// #endregion 📋️Tickets

// #region 🧬️Missing Utilities

func NormalizeRepoPath(path string) string {
	normalized := NormalizePath(path)
	if filepath.IsAbs(path) {
		normalized = GetRelativePath(path)
	}
	normalized = strings.TrimPrefix(normalized, "./")
	return normalized
}

// 💿️isRepoExcludedPath holds the data fields for a isRepoExcludedPath record.
func IsRepoExcludedPath(path string) bool {
	normalized := NormalizeRepoPath(path)
	if normalized == "" {
		return false
	}
	if normalized == ".🧬semio" || strings.HasPrefix(normalized, ".🧬semio/") {
		return true
	}
	if normalized == "assets/repo" || strings.HasPrefix(normalized, "assets/repo/") || strings.Contains(normalized, "/asset/repo/") {
		return true
	}
	if normalized == "node_modules" || strings.HasPrefix(normalized, "node_modules/") ||
		strings.Contains(normalized, "/node_modules/") {
		return true
	}
	if normalized == ".git" || strings.HasPrefix(normalized, ".git/") || strings.Contains(normalized, "/.git/") {
		return true
	}
	for _, segment := range []string{"/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"} {
		if strings.Contains(normalized, segment) {
			return true
		}
	}
	base := filepath.Base(normalized)
	if strings.HasSuffix(base, ".Designer.cs") {
		return true
	}
	if strings.Contains(normalized, "/codegen/") {
		return true
	}
	return false
}

// 🗃️setDifference holds the data fields for a setDifference record.
func SetDifference(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var diff []int
	for _, x := range a {
		if !m[x] {
			diff = append(diff, x)
		}
	}
	return diff
}

// #endregion 🧬️Missing Utilities

// #region 🔊️Cli

// 📨️GetContributorAvatarPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarPath retrieves and returns the contributor avatar path.
func GetContributorAvatarPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar.png")
}

// ❌️GetContributorAvatarRoundPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarRoundPath retrieves and returns the contributor avatar round path.
func GetContributorAvatarRoundPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar-round.png")
}

// 🛤️GetContributorPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorPath retrieves and returns the contributor path.
func GetContributorPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias)
}

// #endregion 🔊️Cli

// #region 🦀️Hooks

// 🛤️normalizeHookPath holds the data fields for a normalizeHookPath record.
func NormalizeHookPath(path string) string {
	path = strings.TrimSpace(path)
	if path == "" {
		return ""
	}
	if filepath.IsAbs(path) {
		path = GetRelativePath(path)
	}
	path = NormalizePath(path)
	return strings.TrimPrefix(path, "./")
}

// #endregion 🦀️Hooks

// #region ❄️Goals

// 📨️GetRepoGoalsDir MUST retrieve the requested value or return an error.
// 📖️GetRepoGoalsDir retrieves and returns the repo goals dir.
func GetRepoGoalsDir() string {
	return filepath.Join(GetRepoMetaDir(), "🎯️goals")
}

// #endregion ❄️Goals

// #region 💾️Missing Utility Functions

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

// #endregion 💾️Missing Utility Functions

// #region 🔌️Workspace Ports

// 👀️rootDirWatchers are the caches that must be dropped when the repository root moves.
var rootDirWatchers []func()

// 🔔️WatchRootDir registers a cache invalidation to run whenever the repository root changes.
func WatchRootDir(reset func()) { rootDirWatchers = append(rootDirWatchers, reset) }

// 📣️notifyRootDirWatchers runs every registered cache invalidation.
func notifyRootDirWatchers() {
	for _, reset := range rootDirWatchers {
		reset()
	}
}

// #endregion 🔌️Workspace Ports

// #region 🖨️Command Streams

// 🖨️CommandStreams is the narrow view of a CLI command a domain routine needs to write output. It
// lives here so the domain modules never have to import ⌨️cli; `*command.Command` satisfies it.
type CommandStreams interface {
	// 📤️OutOrStdout answers the writer for regular output.
	OutOrStdout() io.Writer
	// 📥️ErrOrStderr answers the writer for error output.
	ErrOrStderr() io.Writer
}

// #endregion 🖨️Command Streams

// #endregion 🚚️Split
