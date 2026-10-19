// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🙈️ignore carries the owned gitignore rules and the cached repository gitignore.

// #endregion 🧲️Header

package workspace

import (
	bufio "bufio"
	os "os"
	filepath "path/filepath"
	strings "strings"
	sync "sync"
)

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
	gitignorePath := filepath.Join(GetRootDir(), ".gitignore")
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

// #endregion 🙈️Ignore
