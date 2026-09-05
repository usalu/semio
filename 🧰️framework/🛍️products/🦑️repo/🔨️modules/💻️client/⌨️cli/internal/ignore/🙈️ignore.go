// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package ignore implements the repository ignore language on owned glob primitives.

// #endregion 🧲️Header

package ignore

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"

	"github.com/usalu/semio/repo/client/internal/glob"
)

// #region 📜️Schema

type rule struct {
	pattern string
	negated bool
}

type GitIgnore struct{ rules []rule }

// #endregion 📜️Schema

// #region 📄️Parsing

func CompileIgnoreFile(path string) (*GitIgnore, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	result := &GitIgnore{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
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
		result.rules = append(result.rules, item)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return result, nil
}

// #endregion 📄️Parsing

// #region 🔍️Matching

func (ignore *GitIgnore) MatchesPath(path string) bool {
	path = filepath.ToSlash(strings.TrimPrefix(path, "./"))
	matched := false
	for _, item := range ignore.rules {
		ok, err := glob.Match(item.pattern, path)
		if err == nil && ok {
			matched = !item.negated
		}
	}
	return matched
}

// #endregion 🔍️Matching
