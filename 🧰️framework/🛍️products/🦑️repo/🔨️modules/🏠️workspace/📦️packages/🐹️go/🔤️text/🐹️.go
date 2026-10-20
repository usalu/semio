// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔤️text carries the text, time, slug, scope and set helpers.

// #endregion 🧲️Header

package workspace

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	strconv "strconv"
	strings "strings"
	time "time"
)

// #region 🔤️Text

// #region 🎼️Utilities
// 📥️General-purpose utility functions for time parsing and formatting.
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

// ⚠️writeWarningf writes operational diagnostics without contaminating structured stdout.
func WriteWarningf(format string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, "Warning: "+format+"\n", args...)
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

// #endregion 🔤️Text
