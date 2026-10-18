// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧩️numstat carries the git numstat model and its log parser.

// #endregion 🧲️Header

package metrics

import (
	"strconv"
	"strings"
)

// #region 🧩️Numstat

// 🟨️LinePair holds added and removed line counts.
type LinePair struct {
	Added   int64 `json:"added"`
	Removed int64 `json:"removed"`
}

// 🧩️FileDelta is one file entry of a numstat commit block.
type FileDelta struct {
	Path       string `json:"path"`
	RenameFrom string `json:"rename_from,omitempty"`
	Added      int64  `json:"added"`
	Removed    int64  `json:"removed"`
	Binary     bool   `json:"binary"`
	Bucket     string `json:"bucket,omitempty"`
}

// 💿️CommitDelta is one parsed commit and the per-bucket line deltas it contributes.
type CommitDelta struct {
	SHA        string              `json:"sha"`
	WhenUnix   int64               `json:"when_unix"`
	Author     string              `json:"author"`
	AuthorMail string              `json:"author_mail"`
	Files      []FileDelta         `json:"files"`
	Delta      map[string]LinePair `json:"delta"`
}

// 🔤️UnquoteGitPath decodes the octal-escaped quoted form git writes for non-ASCII paths.
func UnquoteGitPath(raw string) string {
	trimmed := strings.TrimSpace(raw)
	if len(trimmed) < 2 || !strings.HasPrefix(trimmed, "\"") || !strings.HasSuffix(trimmed, "\"") {
		return raw
	}
	inner := []rune(trimmed[1 : len(trimmed)-1])
	out := make([]byte, 0, len(inner))
	for index := 0; index < len(inner); {
		if inner[index] != '\\' {
			out = append(out, string(inner[index])...)
			index++
			continue
		}
		index++
		if index >= len(inner) {
			break
		}
		escape := inner[index]
		index++
		switch escape {
		case 'n':
			out = append(out, '\n')
		case 't':
			out = append(out, '\t')
		case 'r':
			out = append(out, '\r')
		case 'a':
			out = append(out, 7)
		case 'b':
			out = append(out, 8)
		case 'f':
			out = append(out, 12)
		case 'v':
			out = append(out, 11)
		case '"':
			out = append(out, '"')
		case '\\':
			out = append(out, '\\')
		default:
			if escape >= '0' && escape <= '7' {
				value := int(escape - '0')
				for taken := 1; taken < 3 && index < len(inner) && inner[index] >= '0' && inner[index] <= '7'; taken++ {
					value = value*8 + int(inner[index]-'0')
					index++
				}
				out = append(out, byte(value&0xff))
				continue
			}
			out = append(out, string(escape)...)
		}
	}
	return string(out)
}

// ↩️ResolveNumstatPath splits a numstat path field into the new path and its rename source.
func ResolveNumstatPath(raw string) (string, string) {
	field := strings.TrimRight(raw, "\r")
	open := strings.Index(field, "{")
	closing := strings.Index(field, "}")
	if open >= 0 && closing > open {
		middle := field[open+1 : closing]
		if arrow := strings.Index(middle, " => "); arrow >= 0 {
			prefix := field[:open]
			suffix := field[closing+1:]
			old := collapseSlashes(prefix + middle[:arrow] + suffix)
			updated := collapseSlashes(prefix + middle[arrow+4:] + suffix)
			return UnquoteGitPath(updated), UnquoteGitPath(old)
		}
	}
	if arrow := strings.Index(field, " => "); arrow >= 0 {
		return UnquoteGitPath(strings.TrimSpace(field[arrow+4:])), UnquoteGitPath(strings.TrimSpace(field[:arrow]))
	}
	return UnquoteGitPath(field), ""
}

// 🧹️collapseSlashes collapses the empty segment a `{ => new}` rename leaves behind.
func collapseSlashes(path string) string {
	for strings.Contains(path, "//") {
		path = strings.ReplaceAll(path, "//", "/")
	}
	return path
}

// 🧩️ParseNumstatLog breaks a `git log --numstat` stream into per-commit records.
func ParseNumstatLog(stdout string, weights map[string]bool, ignore Ignorer) []CommitDelta {
	out := make([]CommitDelta, 0)
	var current *CommitDelta
	buckets := map[string]string{}
	for _, raw := range strings.Split(stdout, "\n") {
		line := strings.TrimRight(raw, "\r")
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "COMMIT") {
			fields := strings.SplitN(line, "\t", 5)
			if len(fields) < 5 {
				continue
			}
			if current != nil {
				out = append(out, *current)
			}
			when, _ := strconv.ParseInt(strings.TrimSpace(fields[4]), 10, 64)
			current = &CommitDelta{SHA: fields[1], Author: fields[2], AuthorMail: fields[3], WhenUnix: when, Files: []FileDelta{}, Delta: map[string]LinePair{}}
			continue
		}
		if current == nil {
			continue
		}
		parts := strings.SplitN(line, "\t", 3)
		if len(parts) < 3 {
			continue
		}
		binary := parts[0] == "-" || parts[1] == "-"
		added, errAdded := strconv.ParseInt(parts[0], 10, 64)
		removed, errRemoved := strconv.ParseInt(parts[1], 10, 64)
		if !binary && (errAdded != nil || errRemoved != nil) {
			continue
		}
		path, renameFrom := ResolveNumstatPath(parts[2])
		bucket := ""
		if !binary {
			remembered, seen := buckets[path]
			if !seen {
				if !PathSkippedForLoc(path, ignore) {
					remembered = ClassifyForNumstat(path, weights)
				}
				buckets[path] = remembered
			}
			bucket = remembered
		}
		if bucket != "" {
			pair := current.Delta[bucket]
			pair.Added += added
			pair.Removed += removed
			current.Delta[bucket] = pair
		}
		entry := FileDelta{Path: path, RenameFrom: renameFrom, Binary: binary, Bucket: bucket}
		if !binary {
			entry.Added = added
			entry.Removed = removed
		}
		current.Files = append(current.Files, entry)
	}
	if current != nil {
		out = append(out, *current)
	}
	return out
}

// #endregion 🧩️Numstat
