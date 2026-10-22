// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏗️technology_bundle carries the repo, technology and bundle shapes with their kind derivations.

// #endregion 🧲️Header

package model

import (
	json "encoding/json"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏗️Technology Bundle

// 🔌️Node defines the interface contract for node operations.
type Node interface {
	IsNode()
	GetID() string
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
func (r *Repo) GetID() string { return EmojiText(EmojiRepo) }

// 🔳️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Repo.
func (r *Repo) GetURI() string { return "repo://root" }

// 🛠️TechnologyKind represents a technology kind value.
type TechnologyKind string

const TechnologyKindUser TechnologyKind = "👤️"

const TechnologyKindInfrastructure TechnologyKind = "🧰️"

const TechnologyKindResearch TechnologyKind = "🔬️"

const TechnologyKindMono TechnologyKind = "🌱️"

// 🔲️String MUST return the canonical string value.
// 📺️String returns the string representation of the TechnologyKind.
func (e TechnologyKind) String() string {
	return string(e)
}

// 📦️BundleKind represents a bundle kind value.
type BundleKind string

const BundleKindLibrary BundleKind = "library"

const BundleKindSchema BundleKind = "schema"

const BundleKindBinary BundleKind = "binary"

const BundleKindUI BundleKind = "ui"

const BundleKindSite BundleKind = "site"

const BundleKindAssets BundleKind = "assets"

const BundleKindRepo BundleKind = "repo"

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
		absRoot = filepath.Join(workspace.GetRootDir(), root)
	}
	for _, configName := range []string{"package.json", "project.json"} {
		configPath := filepath.Join(absRoot, configName)
		if workspace.FileExists(configPath) {
			content, err := workspace.ReadTextFile(configPath)
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
	return EmojiText(emoji) + workspace.Flat(p.Name)
}

// ◼GetURI MUST return the stored value without modification.
// ▪️GetURI returns the u r i of the Technology.
func (p *Technology) GetURI() string {
	return "repo://technology/" + p.GetID()
}

// 🔵️Bundle holds the data fields for a bundle record.
type Bundle struct {
 technologyID string
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
	technologyID:=b.technologyID
 if technologyID=="" { technologyID=EmojiText(resolveTechnologyEmoji(technologyCode))+workspace.Flat(technologyCode) }
	return technologyID + EmojiText(emoji) + workspace.Flat(bundleCode)
}

// 🟢️GetURI MUST return the stored value without modification.
// ◽GetURI returns the u r i of the Bundle.
func (b *Bundle) GetURI() string {
	return "repo://bundle/" + b.GetID()
}

// 🟣️normalizeBundleLabel holds the data fields for a normalizeBundleLabel record.
func NormalizeBundleLabel(name string) string {
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

// 🟤️normalizeBundleID holds the data fields for a normalizeBundleID record.
func NormalizeBundleID(name string) string {
	return NormalizeBundleLabel(name)
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

// #endregion 🏗️Technology Bundle

// 🧬️WithTechnology binds the bundle to an immutable parent identity.
func (b Bundle) WithTechnology(technology Technology) Bundle { b.technologyID=technology.GetID();return b }
