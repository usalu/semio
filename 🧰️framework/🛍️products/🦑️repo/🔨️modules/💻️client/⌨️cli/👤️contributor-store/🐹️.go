// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Contributor storage and contributions.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// 🔬️AnalyzeFile MUST return a non-nil error when the operation fails.
func AnalyzeFile(filePath string, bundles []Bundle) ([]Breach, error) {
	scope := Scope{
		Kind:     ScopeFile,
		FilePath: filePath,
	}
	files := filterConsideredFiles([]string{filePath})
	files = filterGitIgnored(files)
	ctx := NewPolicyContextWithFiles(scope, bundles, files)
	return CheckPoliciesWithContext(ctx, nil)
}

// 🤝️ParseContributorIdentity MUST return the parsed result or an error for invalid input.
// 🔖️ParseContributorIdentity parses and returns the contributor identity from the input.
func ParseContributorIdentity(line string) (name, email string, ok bool) {
	re := regexp.MustCompile(`\d{4}\s+(.+?)\s*<([^>]+)>`)
	m := re.FindStringSubmatch(line)
	if m == nil {
		return "", "", false
	}
	return strings.TrimSpace(m[1]), strings.TrimSpace(m[2]), true
}

func findSectionForDefinition(sections []Section, startLine, endLine int, prefix string) string {
	for _, s := range sections {
		if startLine >= s.StartLine && endLine <= s.EndLine {
			p := s.Name
			if prefix != "" {
				p = prefix + "#" + s.Name
			}
			if len(s.Children) > 0 {
				if cp := findSectionForDefinition(s.Children, startLine, endLine, p); cp != "" {
					return cp
				}
			}
			return p
		}
	}
	return prefix
}

// 📋️ListContributors MUST return all available contributors entries.
// 🔖️ListContributors returns a list of contributors entries.
func ListContributors() ([]Contributor, error) {
	var result []Contributor
	dir := filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs")
	if !FileExists(dir) {
		return []Contributor{{Alias: "unknown", Github: "unknown", Name: "Unknown"}}, nil
	}
	entries, _ := os.ReadDir(dir)
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		p := filepath.Join(dir, e.Name(), "🧑️‍💻️contributor.json")
		if !FileExists(p) {
			continue
		}
		content, _ := ReadTextFile(p)
		var c Contributor
		json.Unmarshal([]byte(content), &c)
		if c.Alias == "" {
			c.Alias = e.Name()
		}
		if c.Github == "" {
			c.Github = e.Name()
		}
		result = append(result, c)
	}
	return result, nil
}

// 🎯️StreamContributors MUST invoke the callback for each matching contributors entry.
// 🔖️StreamContributors streams contributors entries through the callback.
func StreamContributors(ctx context.Context, out chan<- Contributor, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	contributors, err := ListContributors()
	if err != nil {
		return err
	}

	for _, c := range contributors {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(c.Alias, options) && !matchesFilter(c.Name, options) {
				continue
			}
			queryTextParts := []string{c.Alias, c.Name, c.Github, c.Email}
			queryTextParts = append(queryTextParts, c.Aliases...)
			queryTextParts = append(queryTextParts, c.Githubs...)
			queryTextParts = append(queryTextParts, c.Names...)
			queryTextParts = append(queryTextParts, c.Emails...)
			queryMatches, queryErr := matchesQuery(ctx, strings.Join(queryTextParts, " "), options)
			if queryErr != nil {
				return queryErr
			}
			if !queryMatches {
				continue
			}
			if len(options.IncludeContributors) > 0 {
				found := false
				for _, id := range options.IncludeContributors {
					if c.Alias == id || c.Name == id {
						found = true
						break
					}
				}
				if !found {
					continue
				}
			}
			if len(options.ExcludeContributors) > 0 {
				excluded := false
				for _, id := range options.ExcludeContributors {
					if c.Alias == id || c.Name == id {
						excluded = true
						break
					}
				}
				if excluded {
					continue
				}
			}
			out <- c
		}
	}
	return nil
}

// 📨️GetContributorAvatarPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarPath retrieves and returns the contributor avatar path.
func GetContributorAvatarPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar.png")
}

// ❌️GetContributorAvatarRoundPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorAvatarRoundPath retrieves and returns the contributor avatar round path.
func GetContributorAvatarRoundPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias, "avatar-round.png")
}

// 🛤️GetContributorPath MUST retrieve the requested value or return an error.
// 🔖️GetContributorPath retrieves and returns the contributor path.
func GetContributorPath(alias string) string {
	return filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias)
}

// 🆕️CreateContributor MUST create a new entry and return an error on conflict.
// 🔖️CreateContributor creates a new contributor entry.
func CreateContributor(alias string) (*Contributor, error) {
	dir := GetContributorPath(alias)
	if FileExists(dir) {
		return nil, fmt.Errorf("contributor already exists: %s", alias)
	}
	EnsureDir(dir)
	c := Contributor{Alias: alias}
	data, _ := json.MarshalIndent(c, "", "  ")
	WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
	return &c, nil
}

// 🔷️LoadContributor MUST return all matching contributor from the data source.
// 🔖️LoadContributor loads and returns contributor from the data source.
func LoadContributor(alias string) (*Contributor, error) {
	path := filepath.Join(GetContributorPath(alias), "🧑️‍💻️contributor.json")
	if !FileExists(path) {
		return nil, fmt.Errorf("contributor not found: %s", alias)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var c Contributor
	if err := json.Unmarshal(data, &c); err != nil {
		return nil, err
	}
	if c.Alias == "" {
		c.Alias = alias
	}
	return &c, nil
}

// 🏪️SaveContributor MUST persist the contributor atomically to the data store.
// 🔖️SaveContributor persists contributor to the data store.
func SaveContributor(c Contributor) error {
	if c.Alias == "" {
		c.Alias = c.Github
	}
	dir := GetContributorPath(c.Alias)
	if !FileExists(dir) {
		EnsureDir(dir)
	}
	data, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
}

// 🚚️RemoveContributor MUST remove the target and return an error on failure.
// 🔖️RemoveContributor removes the specified contributor.
func RemoveContributor(alias string) error {
	dir := filepath.Join(GetRepoMetaDir(), "🧑️‍💻️devs", alias)
	if !FileExists(dir) {
		return fmt.Errorf("contributor not found: %s", alias)
	}
	return os.RemoveAll(dir)
}

// 🔶️GetRegisteredPolicies MUST retrieve the requested value or return an error.
// 🔖️GetRegisteredPolicies retrieves and returns the registered policies.
func GetRegisteredPolicies() []PolicyDef {
	return GetPolicies()
}

// 🧹️filterGitIgnored holds the data fields for a filterGitIgnored record.
func filterGitIgnored(files []string) []string {
	if len(files) == 0 {
		return files
	}
	relPaths := make([]string, len(files))
	for i, filePath := range files {
		relPaths[i] = normalizeRepoPath(filePath)
	}
	ignored := GetGitIgnoredSet(relPaths)
	filtered := make([]string, 0, len(files))
	for i, filePath := range files {
		normalized := normalizeRepoPath(filePath)
		if normalized != "" && ignored[relPaths[i]] {
			continue
		}
		if normalized != "" && isGitIgnored(normalized) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}
