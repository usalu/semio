// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Draft management for creating, listing, and deleting draft file sets.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"os"
	"path/filepath"
)

// 💿️Draft holds the data fields for a draft record.
type Draft struct {
	ID string `json:"id"`
}

// 🏪️GetID MUST return the stored value without modification.
// 📖️GetID returns the i d of the Draft.
func (d *Draft) GetID() string {
	return emojiText(EmojiDraft) + Flat(d.ID)
}

// 🔗️GetURI MUST return the stored value without modification.
// 🌐️GetURI returns the u r i of the Draft.
func (d *Draft) GetURI() string {
	return "repo://draft/" + d.GetID()
}

// 📝️GetDraftsPath MUST return the stored value without modification.
// 📦️GetDraftsPath returns the drafts path of the value.
func GetDraftsPath() string {
	return GetRepoMetaPath("✍️notes")
}

// 📸️ListDrafts MUST return a consistent snapshot of available entries.
// 📋️ListDrafts returns all available drafts entries.
func ListDrafts() ([]*Draft, error) {
	draftsDir := GetDraftsPath()
	if !IsDir(draftsDir) {
		return []*Draft{}, nil
	}
	entries, err := os.ReadDir(draftsDir)
	if err != nil {
		return nil, err
	}
	var drafts []*Draft
	for _, entry := range entries {
		if entry.IsDir() {
			drafts = append(drafts, &Draft{ID: entry.Name()})
		}
	}
	return drafts, nil
}

// 🆕️CreateDraft MUST persist the new entity and return a reference to it.
// 📝️CreateDraft creates a new draft and persists it.
func CreateDraft(title string, files []string) (*Draft, error) {
	id := Slugify(title)
	if id == "" {
		return nil, fmt.Errorf("invalid draft title")
	}
	draftPath := filepath.Join(GetDraftsPath(), id)
	if IsDir(draftPath) {
		return nil, fmt.Errorf("draft already exists: %s", id)
	}
	if err := EnsureDir(draftPath); err != nil {
		return nil, err
	}
	for _, f := range files {
		src := f
		dst := filepath.Join(draftPath, filepath.Base(f))

		if err := CopyFile(src, dst); err != nil {

			os.RemoveAll(draftPath)
			return nil, fmt.Errorf("failed to copy file %s: %w", src, err)
		}
	}
	return &Draft{ID: id}, nil
}

// 🗑️DeleteDraft MUST remove all associated data for the entity.
// 🗑️DeleteDraft removes the specified draft.
func DeleteDraft(id string) error {
	draftPath := filepath.Join(GetDraftsPath(), id)
	if !IsDir(draftPath) {
		return nil
	}
	return os.RemoveAll(draftPath)
}
