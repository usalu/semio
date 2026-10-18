// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Metric types, technology and bundle nodes, and kind derivation.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

// 💿️Range holds the data fields for a range record.
type Range struct {
	Start int `json:"start"`
	End   int `json:"end"`
}

// 🟨️LineMetrics holds the data fields for a line metrics record.
type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

// 🟩️DiffLines holds the data fields for a diff lines record.
type DiffLines struct {
	Added   []int
	Removed []int
}

// 🟦️CountMetrics holds the data fields for a count metrics record.
type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}

// 🤝️ContributorIcons holds the data fields for a contributor icons record.
type ContributorIcons struct {
	Avatar      *string `json:"avatar,omitempty"`
	AvatarRound *string `json:"avatarRound,omitempty"`
	Github      *string `json:"github,omitempty"`
}

// 🟪️TicketDate holds the data fields for a ticket date record.
type TicketDate struct {
	Created  time.Time  `json:"created"`
	Finished *time.Time `json:"finished,omitempty"`
}

// 📑️TicketSectionMetrics holds the data fields for a ticket section metrics record.
type TicketSectionMetrics struct {
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// 📍️TicketFileMetricsEntry holds the data fields for a ticket file metrics entry record.
type TicketFileMetricsEntry struct {
	Path     string                          `json:"path"`
	Lines    *LineMetrics                    `json:"lines,omitempty"`
	Sections map[string]TicketSectionMetrics `json:"sections,omitempty"`
}

// 🔬️AnalyzeMetrics holds the data fields for a analyze metrics record.
type AnalyzeMetrics struct {
	Total       int            `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int            `json:"autofixable"`
}

// 🟫️PriorityCount holds the data fields for a priority count record.
type PriorityCount struct {
	High   int `json:"high"`
	Medium int `json:"medium"`
	Low    int `json:"low"`
}

// 💠️Repo holds the data fields for a repo record.
type Repo struct {
	ID           string       `json:"id"`
	Name         string       `json:"name"`
	Path         string       `json:"path"`
	Technologies []Technology `json:"technologies"`
	Bundles      []Bundle     `json:"bundles"`
}

// 🌿️IsNode MUST return true only when the condition is met.
// 🐙️IsNode reports whether the Repo is node.
func (r *Repo) IsNode() {}

// 🏪️GetID MUST return the stored value without modification.
// 🔺️GetID returns the i d of the Repo.
func (r *Repo) GetID() string { return emojiText(EmojiRepo) }

// 🔳️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Repo.
func (r *Repo) GetURI() string { return "repo://root" }

// 🛠️TechnologyKind represents a technology kind value.
type TechnologyKind string

const (
	TechnologyKindUser           TechnologyKind = "👤️"
	TechnologyKindInfrastructure TechnologyKind = "🧰️"
	TechnologyKindResearch       TechnologyKind = "🔬️"
	TechnologyKindMono           TechnologyKind = "🌱️"
)

// 🔲️String MUST return the canonical string value.
// 📺️String returns the string representation of the TechnologyKind.
func (e TechnologyKind) String() string {
	return string(e)
}

// 📦️BundleKind represents a bundle kind value.
type BundleKind string

const (
	BundleKindLibrary BundleKind = "library"
	BundleKindSchema  BundleKind = "schema"
	BundleKindBinary  BundleKind = "binary"
	BundleKindUI      BundleKind = "ui"
	BundleKindSite    BundleKind = "site"
	BundleKindAssets  BundleKind = "assets"
	BundleKindRepo    BundleKind = "repo"
)

// ▪️IsValid MUST return true only when the condition is met.
// 🔻️IsValid reports whether the BundleKind is valid.
func (e BundleKind) IsValid() bool {
	switch e {
	case BundleKindLibrary, BundleKindSchema, BundleKindBinary, BundleKindUI, BundleKindSite, BundleKindAssets, BundleKindRepo:
		return true
	}
	return false
}

// ▫️String MUST return the canonical string value.
// ⬜️String returns the string representation of the BundleKind.
func (e BundleKind) String() string {
	return string(e)
}

// 🏷️DeriveTechnologyKind MUST return a valid value for any recognized input.
// 🟥️DeriveTechnologyKind infers and returns the technology kind from the given input.
func DeriveTechnologyKind(name string) TechnologyKind {
	switch name {
	case "compose":
		return TechnologyKindUser
	case "repo":
		return TechnologyKindInfrastructure
	case "coda":
		return TechnologyKindResearch
	}
	if strings.HasPrefix(name, "@") {
		return DeriveTechnologyKind(strings.TrimPrefix(name, "@"))
	}
	return TechnologyKindUser
}

// ◾DeriveBundleKind MUST return a valid value for any recognized input.
// 🟧️DeriveBundleKind infers and returns the bundle kind from the given input.
func DeriveBundleKind(name string, root string) BundleKind {
	absRoot := root
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(GetRootDir(), root)
	}
	for _, configName := range []string{"package.json", "project.json"} {
		configPath := filepath.Join(absRoot, configName)
		if FileExists(configPath) {
			content, err := ReadTextFile(configPath)
			if err == nil {
				var meta struct {
					BundleKind string `json:"bundleKind"`
				}
				if json.Unmarshal([]byte(content), &meta) == nil && meta.BundleKind != "" {
					kind := BundleKind(strings.ToLower(meta.BundleKind))
					if kind.IsValid() {
						return kind
					}
				}
			}
		}
	}
	return BundleKindLibrary
}

// 📜️Technology holds the data fields for a technology record.
type Technology struct {
	Name    string         `json:"name"`
	Root    string         `json:"root"`
	Kind    TechnologyKind `json:"kind"`
	Emoji   string         `json:"emoji"`
	Bundles []Bundle       `json:"bundles"`
}

// ◽IsNode MUST return true only when the condition is met.
// 🔳️IsNode reports whether the Technology is node.
func (p *Technology) IsNode() {}

// ◻GetID MUST return the stored value without modification.
// 🔲️GetID returns the i d of the Technology.
func (p *Technology) GetID() string {
	emoji := p.Emoji
	if emoji == "" {
		emoji = string(p.Kind)
	}
	if emoji == "" {
		emoji = string(DeriveTechnologyKind(p.Name))
	}
	return emojiText(emoji) + Flat(p.Name)
}

// ◼GetURI MUST return the stored value without modification.
// ▪️GetURI returns the u r i of the Technology.
func (p *Technology) GetURI() string {
	return "repo://technology/" + p.GetID()
}

// 🔵️Bundle holds the data fields for a bundle record.
type Bundle struct {
	Name           string     `json:"name"`
	Root           string     `json:"root"`
	SourceRoot     string     `json:"sourceRoot,omitempty"`
	TechnologyName string     `json:"technologyName"`
	Tags           []string   `json:"tags,omitempty"`
	Kind           BundleKind `json:"kind"`
	Emoji          string     `json:"emoji"`
	Packages       []Package  `json:"packages,omitempty"`
}

// 🔴️Package holds the data fields for a package record.
type Package struct {
	Name    string `json:"name"`
	Version string `json:"version"`
	Path    string `json:"path"`
	Kind    string `json:"kind"`
}

// 🟠️IsNode MUST return true only when the condition is met.
// ▫️IsNode reports whether the Bundle is node.
func (b *Bundle) IsNode() {}

// 🟡️GetID MUST return the stored value without modification.
// ◾GetID returns the i d of the Bundle.
func (b *Bundle) GetID() string {
	emoji := b.Emoji
	if emoji == "" {
		switch string(b.Kind) {
		case "schema":
			emoji = EmojiBundleSchema
		case "binary":
			emoji = EmojiBundleBinary
		case "ui":
			emoji = EmojiBundleUI
		case "site":
			emoji = EmojiBundleSite
		case "assets":
			emoji = EmojiBundleAssets
		case "library":
			emoji = EmojiBundleLibrary
		case "example":
			emoji = EmojiBundleExample
		case "repo":
			emoji = EmojiBundleRepo
		}
	}
	parts := strings.SplitN(b.Name, "/", 2)
	technologyCode := parts[0]
	bundleCode := technologyCode
	if len(parts) > 1 {
		bundleCode = parts[1]
	}
	techEmoji := resolveTechnologyEmoji(technologyCode)
	return emojiText(techEmoji) + Flat(technologyCode) + emojiText(emoji) + Flat(bundleCode)
}

// 🟢️GetURI MUST return the stored value without modification.
// ◽GetURI returns the u r i of the Bundle.
func (b *Bundle) GetURI() string {
	return "repo://bundle/" + b.GetID()
}

// 🟣️normalizeBundleLabel holds the data fields for a normalizeBundleLabel record.
func normalizeBundleLabel(name string) string {
	if name == "" {
		return ""
	}
	if strings.HasPrefix(name, "@") {
		return name
	}
	if name == "vscode" {
		return "repo/vscode"
	}
	if name == "repo" {
		return "repo/go"
	}
	return "compose/" + name
}

// 🔧️bundlePathPrefix holds the data fields for a bundlePathPrefix record.
func bundlePathPrefix(name string) string {
	if name == "" {
		return ""
	}
	if name == "repo" {
		return ""
	}
	if strings.HasPrefix(name, "compose/") {
		return strings.TrimPrefix(name, "compose/") + "/"
	}
	return name + "/"
}

// 📁️FolderKind represents a folder kind value.
type FolderKind string

const (
	FolderKindOrganization FolderKind = "organization"
	FolderKindRequired     FolderKind = "required"
	FolderKindRoot         FolderKind = "root"
)

// ⚪️IsValid MUST return true only when the condition is met.
// ◻IsValid reports whether the FolderKind is valid.
func (e FolderKind) IsValid() bool {
	switch e {
	case FolderKindOrganization, FolderKindRequired, FolderKindRoot:
		return true
	}
	return false
}

// ⚫️String MUST return the canonical string value.
// ◼String returns the string representation of the FolderKind.
func (e FolderKind) String() string {
	return string(e)
}

// 💾️DeriveFolderKind MUST return a valid value for any recognized input.
// 🟠️DeriveFolderKind infers and returns the folder kind from the given input.
var folderKindCache sync.Map

// 🩵️DeriveFolderKind holds the data fields for a DeriveFolderKind record.
func DeriveFolderKind(path string) FolderKind {
	base := filepath.Base(path)
	if strings.HasPrefix(base, ".") {
		return FolderKindRequired
	}

	if val, ok := folderKindCache.Load(path); ok {
		return val.(FolderKind)
	}

	kind := FolderKindOrganization
	requiredIndicators := []string{"package.json", "pyproject.toml", "go.mod", "Cargo.toml"}
	for _, indicator := range requiredIndicators {
		if FileExists(filepath.Join(GetRootDir(), path, indicator)) {
			kind = FolderKindRequired
			break
		}
	}

	if kind == FolderKindOrganization {
		dir := filepath.Join(GetRootDir(), path)
		if entries, err := os.ReadDir(dir); err == nil {
			for _, entry := range entries {
				if !entry.IsDir() {
					ext := filepath.Ext(entry.Name())
					if ext == ".csproj" || ext == ".sln" {
						kind = FolderKindRequired
						break
					}
				}
			}
		}
	}

	folderKindCache.Store(path, kind)
	return kind
}

// 🩶️IsGeneratedFolder MUST return true only when the condition is met.
// 🟡️IsGeneratedFolder reports whether the value is generated folder.
func IsGeneratedFolder(path string) bool {
	parts := strings.Split(filepath.ToSlash(path), "/")
	for _, part := range parts {
		if part == "generated" || part == "dist" || part == "build" || part == "node_modules" || part == "__pycache__" || part == ".next" || part == "coverage" {
			return true
		}
	}
	generatedFolders := []string{
		"js/vscode/generated",
		"js/compose/generated",
	}
	normalized := filepath.ToSlash(path)
	for _, gen := range generatedFolders {
		if normalized == gen || strings.HasPrefix(normalized, gen+"/") {
			return true
		}
	}
	return false
}
