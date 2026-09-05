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
	for index := 0; index < len(pattern); index++ {
		switch pattern[index] {
		case '*':
			if index+1 < len(pattern) && pattern[index+1] == '*' {
				index++
				if index+1 < len(pattern) && pattern[index+1] == '/' {
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
			end := strings.IndexByte(pattern[index+1:], ']')
			if end < 0 {
				return nil, fmt.Errorf("invalid glob %q: unclosed character class", pattern)
			}
			end += index + 1
			class := pattern[index+1 : end]
			if strings.HasPrefix(class, "!") {
				class = "^" + regexp.QuoteMeta(class[1:])
			}
			expression.WriteString("[" + class + "]")
			index = end
		case '\\':
			if index+1 >= len(pattern) {
				return nil, fmt.Errorf("invalid glob %q: trailing escape", pattern)
			}
			index++
			expression.WriteString(regexp.QuoteMeta(string(pattern[index])))
		default:
			expression.WriteString(regexp.QuoteMeta(string(pattern[index])))
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
