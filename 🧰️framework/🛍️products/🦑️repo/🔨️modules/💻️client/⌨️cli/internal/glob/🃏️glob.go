// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package glob provides owned, cross-platform recursive path matching.

// #endregion 🧲️Header

package glob

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync"
)

// #region 🖍️Pattern

const maxCachedPatterns = 256

var patternCache = struct {
	sync.RWMutex
	values map[string]*regexp.Regexp
}{values: map[string]*regexp.Regexp{}}

func Match(pattern, name string) (bool, error) {
	expression, err := compile(pattern)
	if err != nil {
		return false, err
	}
	return expression.MatchString(filepath.ToSlash(name)), nil
}

// 🪗️ braceAlternations identifies balanced comma groups without treating literal braces as syntax.
func braceAlternations(pattern []rune) map[int]string {
	type group struct {
		start  int
		commas []int
	}
	stack := []group{}
	result := map[int]string{}
	for index := 0; index < len(pattern); index++ {
		switch pattern[index] {
		case '\\':
			index++
		case '[':
			for index+1 < len(pattern) && pattern[index+1] != ']' {
				index++
			}
			index++
		case '{':
			stack = append(stack, group{start: index})
		case ',':
			if len(stack) > 0 {
				top := &stack[len(stack)-1]
				top.commas = append(top.commas, index)
			}
		case '}':
			if len(stack) == 0 {
				continue
			}
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if len(top.commas) == 0 {
				continue
			}
			result[top.start], result[index] = "(?:", ")"
			for _, comma := range top.commas {
				result[comma] = "|"
			}
		}
	}
	return result
}

func compile(pattern string) (*regexp.Regexp, error) {
	pattern = filepath.ToSlash(pattern)
	patternCache.RLock()
	cached := patternCache.values[pattern]
	patternCache.RUnlock()
	if cached != nil {
		return cached, nil
	}
	var expression strings.Builder
	expression.WriteString("^")
	symbols := []rune(pattern)
	alternations := braceAlternations(symbols)
	for index := 0; index < len(symbols); index++ {
		if syntax, ok := alternations[index]; ok {
			expression.WriteString(syntax)
			continue
		}
		switch symbols[index] {
		case '*':
			if index+1 < len(symbols) && symbols[index+1] == '*' {
				index++
				if index+1 < len(symbols) && symbols[index+1] == '/' {
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
			end := index + 1
			for end < len(symbols) && symbols[end] != ']' {
				end++
			}
			if end == len(symbols) {
				return nil, fmt.Errorf("invalid glob %q: unclosed character class", pattern)
			}
			class := string(symbols[index+1 : end])
			if strings.HasPrefix(class, "!") {
				class = "^" + regexp.QuoteMeta(class[1:])
			}
			expression.WriteString("[" + class + "]")
			index = end
		case '\\':
			if index+1 >= len(symbols) {
				return nil, fmt.Errorf("invalid glob %q: trailing escape", pattern)
			}
			index++
			expression.WriteString(regexp.QuoteMeta(string(symbols[index])))
		default:
			expression.WriteString(regexp.QuoteMeta(string(symbols[index])))
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

func FilepathGlob(pattern string) ([]string, error) {
	return FilepathGlobContext(context.Background(), pattern, nil)
}

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

func traversalRoot(pattern string) string {
	normalized := filepath.Clean(pattern)
	volume := filepath.VolumeName(normalized)
	parts := strings.Split(strings.TrimPrefix(normalized, volume+string(filepath.Separator)), string(filepath.Separator))
	root := volume
	if filepath.IsAbs(normalized) {
		root += string(filepath.Separator)
	}
	for _, part := range parts {
		if strings.ContainsAny(part, "*?[{") {
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
