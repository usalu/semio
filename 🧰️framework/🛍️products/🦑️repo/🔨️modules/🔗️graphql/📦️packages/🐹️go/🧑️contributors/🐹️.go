// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧑️contributors carries the contributor mutations of the repository context.

// #endregion 🧲️Header

package graphql

import (
	slices "slices"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
)

// #region 🧑️Contributors

// ➕️ContributorAdd MUST return a non-nil error when the operation fails.
// ➕️ContributorAdd performs the contributor add operation on the repo context.
func (c *RepoContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	contributor, err := contributorspkg.LoadContributor(input.Github)
	if err != nil {
		contributor, err = contributorspkg.CreateContributor(input.Github)
		if err != nil {
			return nil, err
		}
	}

	changed := false
	if input.Name != nil && *input.Name != "" && contributor.Name == "" {
		contributor.Name = *input.Name
		changed = true
	}
	for _, n := range input.Names {
		if !slices.Contains(contributor.Names, n) {
			contributor.Names = append(contributor.Names, n)
			changed = true
		}
	}
	if input.Email != nil && *input.Email != "" && contributor.Email == "" {
		contributor.Email = *input.Email
		changed = true
	}
	for _, e := range input.Emails {
		if !slices.Contains(contributor.Emails, e) {
			contributor.Emails = append(contributor.Emails, e)
			changed = true
		}
	}
	if input.Fingerprint != nil && *input.Fingerprint != "" && contributor.Fingerprint == "" {
		contributor.Fingerprint = *input.Fingerprint
		changed = true
	}
	for _, f := range input.Fingerprints {
		if !slices.Contains(contributor.Fingerprints, f) {
			contributor.Fingerprints = append(contributor.Fingerprints, f)
			changed = true
		}
	}

	if changed {
		if err := contributorspkg.SaveContributor(*contributor); err != nil {
			return nil, err
		}
		events.Emit(events.EventContributorAddEnded, "repo-cli", events.ContributorPayload{
			Github: contributor.Github, Author: contributorspkg.GetGitAuthorAlias(),
		})
	}
	return contributor, nil
}

// ➖️ContributorRemove MUST return a non-nil error when the operation fails.
// 💗️ContributorRemove performs the contributor remove operation on the repo context.
func (c *RepoContext) ContributorRemove(github string) error {
	events.Emit(events.EventContributorRemoveEnded, "repo-cli", events.ContributorPayload{
		Github: github, Author: contributorspkg.GetGitAuthorAlias(),
	})
	return nil
}

// #endregion 🧑️Contributors
