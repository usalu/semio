// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧾️analysis carries the git author, system and analysis report types.

// #endregion 🧲️Header

package languages

import (
	fmt "fmt"
	runtime "runtime"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎽️Languages

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

func ParseGitAuthor(s string) GitAuthor {
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

// 🩷️AnalyzeReport holds the data fields for a analyze report record.
type AnalyzeReport struct {
	Second  string         `json:"second"`
	Status  string         `json:"status"`
	Scope   string         `json:"scope"`
	Summary Summary        `json:"summary"`
	Breachs []model.Breach `json:"breachs"`
}

// 💜️Summary holds the data fields for a summary record.
type Summary struct {
	Total      int            `json:"total"`
	ByPriority map[string]int `json:"byPriority"`
	ByKind     map[string]int `json:"byKind"`
}

// 💙️FileCache holds the data fields for a file cache record.
type FileCache struct {
	FilePath string         `json:"filePath"`
	Hash     string         `json:"hash"`
	Second   string         `json:"second"`
	Breachs  []model.Breach `json:"breachs"`
}

// #endregion 🎽️Languages
