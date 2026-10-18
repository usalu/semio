// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ✍️authors carries the git author shapes and contributor identity parsing.

// #endregion 🧲️Header

package contributors

import (
	strings "strings"
	unicode "unicode"
)

// #region ✍️AuthorShapes

// ✍️GitAuthor is a git author line split into its parts.
type GitAuthor struct {
	Name   string `json:"name,omitempty"`
	Email  string `json:"email,omitempty"`
	Github string `json:"github,omitempty"`
}

// 🔤️String renders the author back as `Name <email>`, or as the bare name without an email.
func (author GitAuthor) String() string {
	if author.Email == "" {
		return author.Name
	}
	return author.Name + " <" + author.Email + ">"
}

// ✂️ParseGitAuthor splits a `Name <email>` author line. A line without ` <` is all name.
func ParseGitAuthor(line string) GitAuthor {
	index := strings.Index(line, " <")
	if index < 0 {
		return GitAuthor{Name: line}
	}
	rest := line[index+2:]
	return GitAuthor{Name: strings.TrimSpace(line[:index]), Email: strings.TrimSuffix(rest, ">")}
}

// 🤝️ParseContributorIdentity splits a git shortlog or git log identity line into its name and
// email. The line must carry a four digit run before the name and an angle bracketed email.
func ParseContributorIdentity(line string) (string, string, bool) {
	characters := []rune(line)
	for start := range characters {
		if !runOfFourDigits(characters, start) {
			continue
		}
		afterDigits := start + 4
		cursor := afterDigits
		for cursor < len(characters) && unicode.IsSpace(characters[cursor]) {
			cursor++
		}
		if cursor == afterDigits {
			continue
		}
		open := -1
		for index := cursor; index < len(characters); index++ {
			if characters[index] == '<' {
				open = index
				break
			}
		}
		if open < 0 {
			continue
		}
		closing := -1
		for index := open + 1; index < len(characters); index++ {
			if characters[index] == '>' {
				closing = index
				break
			}
		}
		if closing < 0 {
			continue
		}
		name := string(characters[cursor:open])
		email := string(characters[open+1 : closing])
		if strings.TrimSpace(name) == "" || email == "" {
			continue
		}
		return strings.TrimSpace(name), strings.TrimSpace(email), true
	}
	return "", "", false
}

// 🔢️runOfFourDigits reports whether exactly four digits start at an index.
func runOfFourDigits(characters []rune, start int) bool {
	if start+4 > len(characters) {
		return false
	}
	if start > 0 && isASCIIDigit(characters[start-1]) {
		return false
	}
	for index := start; index < start+4; index++ {
		if !isASCIIDigit(characters[index]) {
			return false
		}
	}
	return start+4 >= len(characters) || !isASCIIDigit(characters[start+4])
}

// 🔢️isASCIIDigit reports whether a rune is a decimal digit.
func isASCIIDigit(value rune) bool { return value >= '0' && value <= '9' }

// #endregion ✍️AuthorShapes
