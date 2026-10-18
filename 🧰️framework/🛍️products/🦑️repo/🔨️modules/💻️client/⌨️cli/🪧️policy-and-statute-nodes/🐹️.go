// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Policy, statute meta, analyze, fix, and contribution nodes.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
)

// 🔖️Policy holds the data fields for a policy record.
type Policy struct {
	ID          string         `json:"id"`
	Name        string         `json:"name"`
	Description *string        `json:"description,omitempty"`
	Scopes      []string       `json:"scopes"`
	Groups      []Territory    `json:"groups"`
	Statutes    []*StatuteMeta `json:"statutes"`
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🍀️IsNode reports whether the Policy is node.
func (p *Policy) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🪴️GetID returns the i d of the Policy.
func (p *Policy) GetID() string {
	slug := strings.TrimPrefix(p.ID, "/")
	return emojiText(EmojiPolicy) + Flat(slug)
}

// 🔖️GetURI MUST return the stored value without modification.
// 🌱️GetURI returns the u r i of the Policy.
func (p *Policy) GetURI() string {
	return "repo://policy/" + p.GetID()
}

// 🔖️StatuteMeta holds the data fields for a statute meta record.
type StatuteMeta struct {
	Kind        Statute        `json:"kind"`
	PolicyID    string         `json:"policyId"`
	Priority    BreachPriority `json:"priority"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
	Autofixable bool           `json:"autofixable"`
}

func normalizeStatuteMeta(meta *StatuteMeta) {
	if meta == nil {
		return
	}
	if meta.Priority == "" {
		meta.Priority = BreachPriorityLow
	}
}

// 🔖️IsNode MUST return true only when the condition is met.
// 📰️IsNode reports whether the StatuteMeta is node.
func (v *StatuteMeta) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌲️GetID returns the i d of the StatuteMeta.
func (v *StatuteMeta) GetID() string {
	return fmt.Sprintf("%s%s", emojiText(EmojiStatute), StatutePathToIdValue(string(v.Kind)))
}

// 🔖️GetURI MUST return the stored value without modification.
// 🔖️GetURI returns the u r i of the StatuteMeta.
func (v *StatuteMeta) GetURI() string {
	return "repo://statute/" + v.GetID()
}

// 🔖️AnalyzeResult holds the data fields for a analyze result record.
type AnalyzeResult struct {
	Breachs []*Breach       `json:"breachs"`
	Metrics *AnalyzeMetrics `json:"metrics"`
}

// 🔖️FixResult holds the data fields for a fix result record.
type FixResult struct {
	Fixed     int       `json:"fixed"`
	Remaining int       `json:"remaining"`
	Breachs   []*Breach `json:"breachs"`
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
