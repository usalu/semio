// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖍️pattern carries the owned glob matcher, brace expansion, filepath traversal and the simple extension globs.

// #endregion 🧲️Header

package workspace

import (
	context "context"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strings "strings"
	sync "sync"
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

// 🟩️SimpleGlob MUST complete the operation and return consistent results.
func SimpleGlob(pattern string, cwd string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	if cwd == "" {
		cwd = GetRootDir()
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

// #endregion 🖍️Pattern
