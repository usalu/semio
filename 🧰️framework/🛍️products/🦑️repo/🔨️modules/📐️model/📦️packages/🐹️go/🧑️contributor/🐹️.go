// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧑️contributor carries the contributor shapes, contributions and checkpoints.

// #endregion 🧲️Header

package model

import (
	regexp "regexp"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧑️Contributor

// 🤝️ContributorIcons holds the data fields for a contributor icons record.
type ContributorIcons struct {
	Avatar      *string `json:"avatar,omitempty"`
	AvatarRound *string `json:"avatarRound,omitempty"`
	Github      *string `json:"github,omitempty"`
}

// 🔗️ContributorLink holds the data fields for a contributor link record.
type ContributorLink struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

// 🔖️Contributor holds the data fields for a contributor record.
type Contributor struct {
	Alias         string                          `yaml:"alias" json:"alias"`
	Aliases       []string                        `yaml:"aliases,omitempty" json:"aliases,omitempty"`
	Emoji         string                          `yaml:"emoji,omitempty" json:"emoji,omitempty"`
	Github        string                          `yaml:"github" json:"github"`
	Githubs       []string                        `yaml:"githubs,omitempty" json:"githubs,omitempty"`
	Name          string                          `yaml:"name" json:"name"`
	Names         []string                        `yaml:"names,omitempty" json:"names,omitempty"`
	Email         string                          `yaml:"email" json:"email"`
	Emails        []string                        `yaml:"emails,omitempty" json:"emails,omitempty"`
	Links         map[string]string               `yaml:"links,omitempty" json:"links,omitempty"`
	Fingerprint   string                          `yaml:"fingerprint,omitempty" json:"fingerprint,omitempty"`
	Fingerprints  []string                        `yaml:"fingerprints,omitempty" json:"fingerprints,omitempty"`
	Contributions ContributorContributionsStorage `yaml:"contributions,omitempty" json:"contributions,omitempty"`
}

// 🌳️ContributorContributionsTree holds the data fields for a contributor contributions tree record.
type ContributorContributionsTree struct {
	Checkpoints []*Checkpoint
	Tickets     []*Ticket
	Bundles     []*ContributorBundle
}

// 🔖️ContributorBundle holds the data fields for a contributor bundle record.
type ContributorBundle struct {
	Name    string
	Lines   LineMetrics
	Folders []*ContributorFolder
}

// 🔖️ContributorFolder holds the data fields for a contributor folder record.
type ContributorFolder struct {
	Name  string
	Lines LineMetrics
	Files []*ContributorFile
}

// 🔖️ContributorFile holds the data fields for a contributor file record.
type ContributorFile struct {
	Name     string
	Lines    LineMetrics
	Sections []*ContributorSection
}

// 🔖️ContributorSection holds the data fields for a contributor section record.
type ContributorSection struct {
	Name        string
	Lines       LineMetrics
	Definitions []*ContributorDefinition
}

// 🔖️ContributorDefinition holds the data fields for a contributor definition record.
type ContributorDefinition struct {
	Name  string
	Lines LineMetrics
}

// 🔖️IsNode MUST return true only when the condition is met.
// 💝️IsNode reports whether the Contributor is node.
func (c *Contributor) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 💘️GetID returns the i d of the Contributor.
func (c *Contributor) GetID() string {
	return EmojiText(EmojiContributor) + workspace.Flat(c.Alias)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🏵️GetURI returns the u r i of the Contributor.
func (c *Contributor) GetURI() string {
	return "repo://contributor/" + c.GetID()
}

// ✔️Checkpoint holds the data fields for a checkpoint record.
type Checkpoint struct {
	ID       string  `json:"id"`
	SHA      string  `json:"sha"`
	Title    string  `json:"title"`
	AuthorID *string `json:"authorId,omitempty"`
	Date     string  `json:"date"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🔢️IsNode reports whether the Checkpoint is node.
func (c *Checkpoint) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌸️GetID returns the i d of the Checkpoint.
func (c *Checkpoint) GetID() string { return EmojiText(EmojiCheckpoint) + c.SHA }

// 🔖️GetURI MUST return the stored value without modification.
// 🌺️GetURI returns the u r i of the Checkpoint.
func (c *Checkpoint) GetURI() string {
	return "repo://checkpoint/" + c.GetID()
}

// 🔖️ContributorContributions holds the data fields for a contributor contributions record.
type ContributorContributions struct {
	Bundles     []ContributionBundle     `json:"bundles"`
	Folders     []ContributionFolder     `json:"folders"`
	Files       []ContributionFile       `json:"files"`
	Sections    []ContributionSection    `json:"sections"`
	Definitions []ContributionDefinition `json:"definitions"`
}

// 🔖️ContributionBundle holds the data fields for a contribution bundle record.
type ContributionBundle struct {
	BundleID string        `json:"bundleId"`
	Metrics  *CountMetrics `json:"metrics"`
}

// 🔖️ContributionFolder holds the data fields for a contribution folder record.
type ContributionFolder struct {
	FolderID string        `json:"folderId"`
	Metrics  *CountMetrics `json:"metrics"`
}

// 🔖️ContributionFile holds the data fields for a contribution file record.
type ContributionFile struct {
	FileID  string       `json:"fileId"`
	Metrics *LineMetrics `json:"metrics"`
}

// 🔖️ContributionSection holds the data fields for a contribution section record.
type ContributionSection struct {
	SectionID string       `json:"sectionId"`
	Metrics   *LineMetrics `json:"metrics"`
}

// 🔖️ContributionDefinition holds the data fields for a contribution definition record.
type ContributionDefinition struct {
	DefinitionID string       `json:"definitionId"`
	Metrics      *LineMetrics `json:"metrics"`
}

// 🤍️ContributorTicket holds the data fields for a contributor ticket record.
type ContributorTicket struct {
	Year     int          `json:"year"`
	Month    int          `json:"month"`
	Day      int          `json:"day"`
	Slug     string       `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string       `json:"filePath,omitempty"`
}

type ContributorCheckpoint struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

// 🖤️ContributorContributionsStorage holds the data fields for a contributor contributions storage record.
type ContributorContributionsStorage struct {
	Bundles     []string                `json:"bundles,omitempty"`
	Folders     []string                `json:"folders,omitempty"`
	Files       []string                `json:"files,omitempty"`
	Regions     []string                `json:"regions,omitempty"`
	Definitions []string                `json:"definitions,omitempty"`
	Tickets     []ContributorTicket     `json:"tickets,omitempty"`
	Checkpoints []ContributorCheckpoint `json:"checkpoints,omitempty"`
	Lines       *LineMetrics            `json:"lines,omitempty"`
}

// 🤝️ContributorBundleContrib holds the data fields for a contributor bundle contrib record.
type ContributorBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟧️ContributorFolderContrib holds the data fields for a contributor folder contrib record.
type ContributorFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟨️ContributorFileContrib holds the data fields for a contributor file contrib record.
type ContributorFileContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟩️ContributorSectionContrib holds the data fields for a contributor section contrib record.
type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟦️ContributorDefinitionContrib holds the data fields for a contributor definition contrib record.
type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟪️ContributorContributionsInternal holds the data fields for a contributor contributions internal record.
type ContributorContributionsInternal struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

// 🟫️ContributorMetricsInternal holds the data fields for a contributor metrics internal record.
type ContributorMetricsInternal struct {
	Checkpoints int `json:"checkpoints"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Lines       int `json:"lines"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
}

// 💠️CodebaseContributor holds the data fields for a codebase contributor record.
type CodebaseContributor struct {
	ID            string                            `json:"id"`
	URI           string                            `json:"uri"`
	Path          string                            `json:"path"`
	Name          string                            `json:"name,omitempty"`
	Icons         *ContributorIcons                 `json:"icons,omitempty"`
	Emails        []string                          `json:"emails,omitempty"`
	Links         map[string]string                 `json:"links,omitempty"`
	Contributions *ContributorContributionsInternal `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInternal       `json:"metrics,omitempty"`
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

// #endregion 🧑️Contributor
