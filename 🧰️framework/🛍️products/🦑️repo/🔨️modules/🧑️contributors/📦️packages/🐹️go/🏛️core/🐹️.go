package contributors

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧑️Contributors

// 🤝️FindAndUpdateContributor MUST return nil when no match is found.
// 🔎️FindAndUpdateContributor searches for and returns the matching and update contributor.
func FindAndUpdateContributor(authorStr string) string {
	parsed := languages.ParseGitAuthor(authorStr)
	if parsed.Name == "" && parsed.Email == "" {
		return "unknown"
	}

	contributors := hostContributors()

	var found *model.Contributor

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

// 🔐️resolveAuthorToAlias holds the data fields for a resolveAuthorToAlias record.
func resolveAuthorToAlias(name, email string) string {
	for _, c := range hostContributors() {
		if email != "" {
			for _, e := range c.Emails {
				if strings.EqualFold(e, email) {
					return c.Alias
				}
			}
		}
		if name != "" {
			if strings.EqualFold(c.Name, name) {
				return c.Alias
			}
			for _, n := range c.Names {
				if strings.EqualFold(n, name) {
					return c.Alias
				}
			}
		}
	}
	if name != "" && email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	if name != "" {
		return name
	}
	if email != "" {
		return email
	}
	return ""
}

// 🔐️GetGitAuthorAlias MUST return the stored value without modification.
// 🟫️GetGitAuthorAlias returns the git author alias of the value.
func GetGitAuthorAlias() string {
	name, _, _ := workspace.ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	name = strings.TrimSpace(name)
	email, _, _ := workspace.ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	email = strings.TrimSpace(email)

	fallback := name
	if email != "" {
		fallback = fmt.Sprintf("%s <%s>", name, email)
	}

	return FindAndUpdateContributor(fallback)
}

// 🤝️BuildCodebaseContributors MUST assemble the codebase contributors from the available context data.
// 🔻️BuildCodebaseContributors constructs and returns the codebase contributors structure.
func BuildCodebaseContributors(ctx *codebase.CodebaseContext) []model.CodebaseContributor {
	contributors := hostContributors()

	var result []model.CodebaseContributor
	for _, c := range contributors {
		links := c.Links
		if links == nil {
			links = map[string]string{}
		}
		if c.Github != "" {
			links["github"] = c.Github
		}
		emails := append([]string{}, c.Emails...)
		if c.Email != "" {
			emails = append(emails, c.Email)
		}
		avatarPath := workspace.GetContributorAvatarPath(c.Alias)
		avatarRoundPath := workspace.GetContributorAvatarRoundPath(c.Alias)

		var icons *model.ContributorIcons
		if workspace.FileExists(avatarPath) || workspace.FileExists(avatarRoundPath) {
			icons = &model.ContributorIcons{}
			if workspace.FileExists(avatarPath) {
				avatar := ctx.FileURI(workspace.GetRelativePath(avatarPath))
				icons.Avatar = &avatar
			}
			if workspace.FileExists(avatarRoundPath) {
				avatarRound := ctx.FileURI(workspace.GetRelativePath(avatarRoundPath))
				icons.AvatarRound = &avatarRound
			}
			if githubLink, ok := links["github"]; ok {
				github := githubLink + ".png"
				icons.Github = &github
			}
		}

		var contributions *model.ContributorContributionsInternal
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &model.ContributorContributionsInternal{}
			for _, b := range c.Contributions.Bundles {
				contributions.Bundles = append(contributions.Bundles, model.ContributorBundleContrib{ID: b})
			}
			for _, f := range c.Contributions.Folders {
				contributions.Folders = append(contributions.Folders, model.ContributorFolderContrib{ID: f})
			}
			for _, f := range c.Contributions.Files {
				contributions.Files = append(contributions.Files, model.ContributorFileContrib{ID: f})
			}
			for _, r := range c.Contributions.Regions {
				contributions.Sections = append(contributions.Sections, model.ContributorSectionContrib{ID: r})
			}
			for _, d := range c.Contributions.Definitions {
				contributions.Definitions = append(contributions.Definitions, model.ContributorDefinitionContrib{ID: d})
			}
		}

		linesTotal := 0
		if c.Contributions.Lines != nil {
			linesTotal = c.Contributions.Lines.Added + c.Contributions.Lines.Removed
		}

		result = append(result, model.CodebaseContributor{
			ID:            c.Alias,
			URI:           ctx.FileURI(".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias),
			Path:          ".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias + "/contributor.json",
			Name:          c.Name,
			Icons:         icons,
			Emails:        emails,
			Links:         links,
			Contributions: contributions,
			Metrics: &model.ContributorMetricsInternal{
				Checkpoints: len(c.Contributions.Checkpoints),
				Tickets:     len(c.Contributions.Tickets),
				Bundles:     len(c.Contributions.Bundles),
				Folders:     len(c.Contributions.Folders),
				Files:       len(c.Contributions.Files),
				Lines:       linesTotal,
				Sections:    len(c.Contributions.Regions),
				Definitions: len(c.Contributions.Definitions),
			},
		})
	}
	return result
}

// 🎯️StreamContributors MUST invoke the callback for each matching contributors entry.
// 🔖️StreamContributors streams contributors entries through the callback.
func StreamContributors(ctx context.Context, out chan<- model.Contributor, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	contributors := hostContributors()

	for _, c := range contributors {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(c.Alias, options) && !model.MatchesFilter(c.Name, options) {
				continue
			}
			queryTextParts := []string{c.Alias, c.Name, c.Github, c.Email}
			queryTextParts = append(queryTextParts, c.Aliases...)
			queryTextParts = append(queryTextParts, c.Githubs...)
			queryTextParts = append(queryTextParts, c.Names...)
			queryTextParts = append(queryTextParts, c.Emails...)
			if !model.MatchesQuery(strings.Join(queryTextParts, " "), options) {
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

// 🆕️CreateContributor MUST create a new entry and return an error on conflict.
// 🔖️CreateContributor creates a new contributor entry.
func CreateContributor(alias string) (*model.Contributor, error) {
	dir := workspace.GetContributorPath(alias)
	if workspace.FileExists(dir) {
		return nil, fmt.Errorf("contributor already exists: %s", alias)
	}
	workspace.EnsureDir(dir)
	c := model.Contributor{Alias: alias}
	data, _ := json.MarshalIndent(c, "", "  ")
	workspace.WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
	return &c, nil
}

// 🔷️LoadContributor MUST return all matching contributor from the data source.
// 🔖️LoadContributor loads and returns contributor from the data source.
func LoadContributor(alias string) (*model.Contributor, error) {
	path := filepath.Join(workspace.GetContributorPath(alias), "🧑️‍💻️contributor.json")
	if !workspace.FileExists(path) {
		return nil, fmt.Errorf("contributor not found: %s", alias)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var c model.Contributor
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
func SaveContributor(c model.Contributor) error {
	if c.Alias == "" {
		c.Alias = c.Github
	}
	dir := workspace.GetContributorPath(c.Alias)
	if !workspace.FileExists(dir) {
		workspace.EnsureDir(dir)
	}
	data, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		return err
	}
	return workspace.WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
}

// 🚚️RemoveContributor MUST remove the target and return an error on failure.
// 🔖️RemoveContributor removes the specified contributor.
func RemoveContributor(alias string) error {
	dir := filepath.Join(workspace.GetRepoMetaDir(), "🧑️‍💻️devs", alias)
	if !workspace.FileExists(dir) {
		return fmt.Errorf("contributor not found: %s", alias)
	}
	return os.RemoveAll(dir)
}

// 🔌️init wires the contributor reads 📐️model and 🗂️codebase declare but cannot implement.
func init() {
	model.LookupContributors = func() ([]model.Contributor, error) { return hostContributors(), nil }
	codebase.LookupCodebaseContributors = BuildCodebaseContributors
}

// #endregion 🧑️Contributors
