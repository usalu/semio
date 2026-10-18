// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📁️filesystem carries the folder, file and section mutations of the repository context.

// #endregion 🧲️Header

package graphql

import (
	errors "errors"
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
)

// #region 📁️Filesystem

// 🟫️FolderCreate MUST return a non-nil error when the operation fails.
// 🩷️FolderCreate performs the folder create operation on the repo context.
func (c *RepoContext) FolderCreate(path string) (*model.Folder, error) { return nil, nil }

// 🚚️FolderMove MUST return a non-nil error when the operation fails.
// 💜️FolderMove performs the folder move operation on the repo context.
func (c *RepoContext) FolderMove(src, dst string) (*model.Folder, error) {
	result := move.ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.Folder{ID: ctx.GetFolderID(dst), Path: dst, Name: filepath.Base(dst)}, nil
}

// 💠️FolderDelete MUST return a non-nil error when the operation fails.
// 💙️FolderDelete performs the folder delete operation on the repo context.
func (c *RepoContext) FolderDelete(path string) error {
	result := move.ToolFolderDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🔳️FileCreate MUST return a non-nil error when the operation fails.
// 💚️FileCreate performs the file create operation on the repo context.
func (c *RepoContext) FileCreate(path string) (*model.File, error) {
	result := move.ToolFileCreate(path)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.File{ID: ctx.GetFileID(path), Path: path, Name: filepath.Base(path), Extension: strings.TrimPrefix(filepath.Ext(path), ".")}, nil
}

// 🔲️FileMove MUST return a non-nil error when the operation fails.
// 💛️FileMove performs the file move operation on the repo context.
func (c *RepoContext) FileMove(src, dst string) (*model.File, error) {
	result := move.ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &codebase.CodebaseContext{Bundles: c.bundles}
	return &model.File{ID: ctx.GetFileID(dst), Path: dst, Name: filepath.Base(dst), Extension: strings.TrimPrefix(filepath.Ext(dst), ".")}, nil
}

// ▪️FileDelete MUST return a non-nil error when the operation fails.
// 🧡️FileDelete performs the file delete operation on the repo context.
func (c *RepoContext) FileDelete(path string) error {
	result := move.ToolFileDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// ▫️SectionCreate MUST return a non-nil error when the operation fails.
// ❤️SectionCreate performs the section create operation on the repo context.
func (c *RepoContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := move.ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := model.BuildSectionID(fileID, strings.Split(sectionPath, "/"))
	return &model.Section{ID: id, Name: name, Path: sectionPath, FilePath: file}, nil
}

// ◾SectionMove MUST return a non-nil error when the operation fails.
// 🤍️SectionMove performs the section move operation on the repo context.
func (c *RepoContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	result := move.ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := model.BuildSectionID(fileID, strings.Split(newName, "/"))
	return &model.Section{ID: id, Name: newName, Path: newName, FilePath: file}, nil
}

// ◽SectionDelete MUST return a non-nil error when the operation fails.
// 🖤️SectionDelete performs the section delete operation on the repo context.
func (c *RepoContext) SectionDelete(file, name string) error {
	result := move.ToolSectionDelete(file, name)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

// 🧬️Integrate MUST return a non-nil error when the operation fails.
// 🧬️Integrate performs the integrate operation on the repo context.
func (c *RepoContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	s := ""
	if source != nil {
		s = *source
	}
	ts := ""
	if targetSection != nil {
		ts = *targetSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}
	tp := ""
	if targetParent != nil {
		tp = *targetParent
	}
	result := move.ToolIntegrate(s, ts, tf, tp)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &model.File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// 🧲️Extract MUST return the extracted component from the input.
// 🤎️Extract extracts the extract from the source.
func (c *RepoContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	s := ""
	if sourceFile != nil {
		s = *sourceFile
	}
	ss := ""
	if sourceSection != nil {
		ss = *sourceSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}

	result := move.ToolExtract(s, ss, tf)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &model.File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

// #endregion 📁️Filesystem
