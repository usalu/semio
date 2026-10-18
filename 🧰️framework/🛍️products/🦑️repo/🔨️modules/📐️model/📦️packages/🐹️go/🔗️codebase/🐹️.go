// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔗️codebase carries the codebase tree shapes and the semantic change vocabulary.

// #endregion 🧲️Header

package model

import (
	strings "strings"
)

// #region 🔗️Codebase

// ♻️SemanticChangeType represents a semantic change type value.
type SemanticChangeType string

const SemanticChangeAdded SemanticChangeType = "added"

const SemanticChangeDeleted SemanticChangeType = "deleted"

const SemanticChangeModified SemanticChangeType = "modified"

const SemanticChangeRenamed SemanticChangeType = "renamed"

// 🔖️SemanticChange holds the data fields for a semantic change record.
type SemanticChange struct {
	Kind     string
	Status   SemanticChangeType
	Path     string
	FromPath string
	ToPath   string
	Lines    LineMetrics
}

func buildCodebasePathSet(codebase *Codebase) map[string]struct{} {
	result := make(map[string]struct{})
	if codebase == nil {
		return result
	}
	for _, bundle := range codebase.Bundles {
		result[bundle.ID] = struct{}{}
	}
	for _, folder := range codebase.Folders {
		result[folder.Path] = struct{}{}
	}
	for _, file := range codebase.Files {
		result[file.Path] = struct{}{}
	}
	for _, section := range codebase.Sections {
		result[section.Path] = struct{}{}
	}
	for _, def := range codebase.Definitions {
		result[def.Path] = struct{}{}
	}
	return result
}

// 🔖️extractFilePrefix holds the data fields for a extractFilePrefix record.
func ExtractFilePrefix(path string) string {
	if path == "" {
		return ""
	}
	if idx := strings.Index(path, "#"); idx != -1 {
		return path[:idx]
	}
	if idx := strings.Index(path, "§"); idx != -1 {
		return path[:idx]
	}
	return path
}

// 📦️CodebaseBundle holds the data fields for a codebase bundle record.
type CodebaseBundle struct {
	ID           string                 `json:"id"`
	Folder       string                 `json:"folder"`
	URI          string                 `json:"uri"`
	Contributors []string               `json:"contributors,omitempty"`
	Tickets      []string               `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

// 🔻️CodebaseFolder holds the data fields for a codebase folder record.
type CodebaseFolder struct {
	ID       string                 `json:"id"`
	Path     string                 `json:"path"`
	URI      string                 `json:"uri"`
	Name     string                 `json:"name"`
	ParentID *string                `json:"parentId,omitempty"`
	BundleID *string                `json:"bundleId,omitempty"`
	Metrics  *FolderMetricsInternal `json:"metrics,omitempty"`
}

// 🔗️FileBreachRef holds the data fields for a file breach ref record.
type FileBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ⬛️CodebaseFile holds the data fields for a codebase file record.
type CodebaseFile struct {
	ID       string               `json:"id"`
	Path     string               `json:"path"`
	URI      string               `json:"uri"`
	FolderID *string              `json:"folderId,omitempty"`
	BundleID *string              `json:"bundleId,omitempty"`
	Metrics  *FileMetricsInternal `json:"metrics,omitempty"`
	Breachs  []FileBreachRef      `json:"breachs,omitempty"`
}

// ⬜️CodebaseSection holds the data fields for a codebase section record.
type CodebaseSection struct {
	ID      string                  `json:"id"`
	Path    string                  `json:"path"`
	URI     string                  `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

// 🟥️CodebaseDefinition holds the data fields for a codebase definition record.
type CodebaseDefinition struct {
	ID      string                     `json:"id"`
	Path    string                     `json:"path"`
	URI     string                     `json:"uri"`
	Metrics *DefinitionMetricsInternal `json:"metrics,omitempty"`
}

// 🌳️CbTreeNodeKind represents a cb tree node kind value.
type CbTreeNodeKind string

const CbTreeNodeRepo CbTreeNodeKind = "repo"

const CbTreeNodeBundle CbTreeNodeKind = "bundle"

const CbTreeNodeFolder CbTreeNodeKind = "folder"

const CbTreeNodeFile CbTreeNodeKind = "file"

const CbTreeNodeSection CbTreeNodeKind = "section"

const CbTreeNodeDefinition CbTreeNodeKind = "definition"

// 🌿️CbTreeNode holds the data fields for a cb tree node record.
type CbTreeNode struct {
	Kind     CbTreeNodeKind         `json:"kind"`
	Children map[string]*CbTreeNode `json:"children,omitempty"`
}

// ◻Codebase holds the data fields for a codebase record.
type Codebase struct {
	Bundles      []CodebaseBundle       `json:"bundles"`
	Folders      []CodebaseFolder       `json:"folders"`
	Files        []CodebaseFile         `json:"files"`
	Sections     []CodebaseSection      `json:"sections"`
	Definitions  []CodebaseDefinition   `json:"definitions"`
	Contributors []CodebaseContributor  `json:"contributors"`
	Tickets      []CodebaseTicket       `json:"tickets"`
	Policies     []CodebasePolicy       `json:"policies"`
	Breachs      []CodebaseBreach       `json:"breachs"`
	Tree         map[string]*CbTreeNode `json:"tree"`
}

// #endregion 🔗️Codebase
