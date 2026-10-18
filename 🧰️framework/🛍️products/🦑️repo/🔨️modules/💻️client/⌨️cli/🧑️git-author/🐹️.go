// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Git author parsing and contributor update.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"runtime"
	"strings"
)

// ✍️GitAuthor holds the data fields for a git author record.
type GitAuthor struct {
	Name   string `json:"name,omitempty" yaml:"name,omitempty"`
	Email  string `json:"email,omitempty" yaml:"email,omitempty"`
	GitHub string `json:"github,omitempty" yaml:"github,omitempty"`
}

// 🟪️String MUST return the canonical string value.
// 🟩️String returns the string representation of the GitAuthor.
func (a GitAuthor) String() string {
	if a.Email != "" {
		return fmt.Sprintf("%s <%s>", a.Name, a.Email)
	}
	return a.Name
}

func parseGitAuthor(s string) GitAuthor {
	res := GitAuthor{}
	if strings.Contains(s, " <") {
		parts := strings.Split(s, " <")
		res.Name = strings.TrimSpace(parts[0])
		res.Email = strings.TrimSuffix(parts[1], ">")
	} else {
		res.Name = s
	}
	return res
}

// 🤝️FindAndUpdateContributor MUST return nil when no match is found.
// 🔎️FindAndUpdateContributor searches for and returns the matching and update contributor.
func FindAndUpdateContributor(authorStr string) string {
	parsed := parseGitAuthor(authorStr)
	if parsed.Name == "" && parsed.Email == "" {
		return "unknown"
	}

	contributors, _ := ListContributors()

	var found *Contributor

	if parsed.Email != "" {
		for i := range contributors {
			matches := false
			if strings.EqualFold(contributors[i].Email, parsed.Email) {
				matches = true
			} else {
				for _, e := range contributors[i].Emails {
					if strings.EqualFold(e, parsed.Email) {
						matches = true
						break
					}
				}
			}
			if matches {
				found = &contributors[i]

				nameExists := strings.EqualFold(found.Name, parsed.Name)
				if !nameExists && parsed.Name != "" {
					for _, n := range found.Names {
						if strings.EqualFold(n, parsed.Name) {
							nameExists = true
							break
						}
					}
				}
				if !nameExists && parsed.Name != "" {
					found.Names = append(found.Names, parsed.Name)
					SaveContributor(*found)
				}
				break
			}
		}
	}

	if found == nil && parsed.Name != "" {
		for i := range contributors {
			matches := false
			if strings.EqualFold(contributors[i].Name, parsed.Name) {
				matches = true
			} else {
				for _, n := range contributors[i].Names {
					if strings.EqualFold(n, parsed.Name) {
						matches = true
						break
					}
				}
			}
			if matches {
				found = &contributors[i]

				emailExists := strings.EqualFold(found.Email, parsed.Email)
				if !emailExists && parsed.Email != "" {
					for _, e := range found.Emails {
						if strings.EqualFold(e, parsed.Email) {
							emailExists = true
							break
						}
					}
				}
				if !emailExists && parsed.Email != "" {
					found.Emails = append(found.Emails, parsed.Email)
					SaveContributor(*found)
				}
				break
			}
		}
	}

	if found != nil {
		return found.Alias
	}

	return authorStr
}

// 🏪️GetSystem MUST return the stored value without modification.
// 📦️GetSystem returns the system of the value.
func GetSystem() string {
	switch runtime.GOOS {
	case "darwin":
		return "mac"
	case "windows":
		return "windows"
	default:
		return "linux"
	}
}
