// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Folder, file, section, and definition streaming.

// #endregion 🧲️Header

package client

import (
	"context"
	"fmt"
	model "github.com/usalu/semio/repo/model"
	"os"
	"path/filepath"
	"strings"
)

// 🩷️StreamFolders MUST invoke the callback for each matching folders entry.
// 📄️StreamFolders streams folders entries through the callback.
func StreamFolders(ctx context.Context, scope string, out chan<- Folder, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	layout := options.Layout
	if layout == nil {
		boundaries := []model.ArtifactBoundary{}
		for _, bundle := range GetTechnologies() {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
		layout = model.NewArtifactLayout(rootDir, boundaries)
	}
	if layout.Root() != rootDir {
		return fmt.Errorf("Exact invocation layout repository required")
	}
	root := rootDir
	if bundleName, found := strings.CutPrefix(scope, "compose/"); found {
		bundles := GetTechnologies()
		for _, b := range bundles {
			if b.Name == bundleName || normalizeBundleLabel(b.Name) == bundleName {
				root = filepath.Join(rootDir, b.Root)
				break
			}
		}
	} else if scope != "" && scope != "compose" {
		if filepath.IsAbs(scope) {
			root = scope
		} else {
			root = filepath.Join(rootDir, scope)
		}
	}

	if err := ctx.Err(); err != nil {
		return err
	}
	if isRepoExcludedPath(root) {
		return nil
	}
	visited, phase := 0, "discovering-folders"
	var folders []string
	err := filepath.WalkDir(root, func(path string, info os.DirEntry, err error) error {
		if cancelled := ctx.Err(); cancelled != nil {
			return cancelled
		}
		visited++
		if options.Progress != nil && visited%256 == 0 {
			options.Progress(StreamProgress{Current: visited, Phase: phase})
		}
		if cancelled := ctx.Err(); cancelled != nil {
			return cancelled
		}
		if err != nil {
			return err
		}
		if info.Type()&os.ModeSymlink != 0 {
			return nil
		}
		if !info.IsDir() {
			return nil
		}
		rel, _ := filepath.Rel(root, path)
		if rel == "." {
			return nil
		}
		if isRepoExcludedPath(path) {
			return filepath.SkipDir
		}
		if isGitIgnored(path) && !options.ShowIgnored {
			return filepath.SkipDir
		}
		folders = append(folders, path)
		return nil
	})
	if err != nil {
		return err
	}

	for position, folderPath := range folders {
		if err := ctx.Err(); err != nil {
			return err
		}
		if options.Progress != nil && position%256 == 0 {
			options.Progress(StreamProgress{Current: position, Phase: "resolving-folder-identities"})
		}
		if err := ctx.Err(); err != nil {
			return err
		}
		ignored := isGitIgnored(folderPath)
		relPath, _ := filepath.Rel(rootDir, folderPath)
		generated := IsGenerated(folderPath) || IsGeneratedFolder(relPath)
		folderKind := FolderKind(layout.FolderKind(relPath))

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeFolderKind(folderKind, options) {
			continue
		}

		if !matchesFilter(filepath.Base(folderPath), options) {
			continue
		}
		queryMatches, queryErr := matchesQuery(ctx, folderPath, options)
		if queryErr != nil {
			return queryErr
		}
		if !queryMatches {
			continue
		}

		var bundleID *string
		if id, found := layout.BoundaryIDByPath(relPath); found {
			bundleID = &id
		}

		parentPath := filepath.Dir(relPath)
		var parentID *string
		if parentPath != "." {
			id := layout.BuildFolderID(parentPath, bundleID)
			parentID = &id
		}

		resolvedID := layout.BuildFolderID(relPath, bundleID)
		select {
		case <-ctx.Done():
			return ctx.Err()
		case out <- Folder{
			ID:        resolvedID,
			Path:      relPath,
			URI:       model.ArtifactURIForID("folder", resolvedID),
			Name:      filepath.Base(relPath),
			ParentID:  parentID,
			BundleID:  bundleID,
			Kind:      folderKind,
			Ignored:   ignored,
			Generated: generated,
		}:
		}
	}
	return nil
}

// 💜️StreamFiles MUST invoke the callback for each matching files entry.
// 🔲️StreamFiles streams files entries through the callback.
func StreamFiles(ctx context.Context, scope string, out chan<- File, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	layout := options.Layout
	if layout == nil {
		boundaries := []model.ArtifactBoundary{}
		for _, bundle := range GetTechnologies() {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
		layout = model.NewArtifactLayout(rootDir, boundaries)
	}
	if layout.Root() != rootDir {
		return fmt.Errorf("Exact invocation layout repository required")
	}
	root := rootDir
	if scope != "" && scope != "compose" {
		bundles := GetTechnologies()
		matched := false
		for _, b := range bundles {
			if b.Name == scope || b.Root == scope || normalizeBundleLabel(b.Name) == scope {
				root = filepath.Join(rootDir, b.Root)
				matched = true
				break
			}
		}
		if !matched {
			parts := strings.SplitN(scope, "/", 3)
			if len(parts) == 3 {
				bundleScope := parts[0] + "/" + parts[1]
				subPath := parts[2]
				for _, b := range bundles {
					if b.Name == bundleScope || b.Root == bundleScope || normalizeBundleLabel(b.Name) == bundleScope {
						root = filepath.Join(rootDir, b.Root, subPath)
						matched = true
						break
					}
				}
			}
		}
		if !matched {
			if filepath.IsAbs(scope) {
				root = scope
			} else {
				root = filepath.Join(rootDir, scope)
			}
		}
	}

	if err := ctx.Err(); err != nil {
		return err
	}
	if isRepoExcludedPath(root) {
		return nil
	}
	if info, err := os.Stat(root); err == nil && !info.IsDir() {
		relPath, _ := filepath.Rel(rootDir, root)
		var bundleID *string
		if id, found := layout.BoundaryIDByPath(relPath); found {
			bundleID = &id
		}
		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := layout.BuildFolderID(folderPath, bundleID)
			folderID = &id
		}
		resolvedID := layout.BuildFileID(relPath, bundleID)
		file := File{
			ID:        resolvedID,
			Path:      relPath,
			URI:       model.ArtifactURIForID("file", resolvedID),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
			Kind:      DeriveFileKind(filepath.Base(relPath)),
			Ignored:   isGitIgnored(root),
			Generated: IsGenerated(root),
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		case out <- file:
		}
		return nil
	}

	visited, phase := 0, "discovering-files"
	var files []string
	err := filepath.WalkDir(root, func(path string, info os.DirEntry, err error) error {
		if cancelled := ctx.Err(); cancelled != nil {
			return cancelled
		}
		visited++
		if options.Progress != nil && visited%256 == 0 {
			options.Progress(StreamProgress{Current: visited, Phase: phase})
		}
		if cancelled := ctx.Err(); cancelled != nil {
			return cancelled
		}
		if err != nil {
			return err
		}
		if info.Type()&os.ModeSymlink != 0 {
			return nil
		}
		if info.IsDir() {
			rel, _ := filepath.Rel(root, path)
			if rel != "." {
				if isRepoExcludedPath(path) {
					return filepath.SkipDir
				}
				if isGitIgnored(path) && !options.ShowIgnored {
					return filepath.SkipDir
				}
			}
			return nil
		}
		if isRepoExcludedPath(path) {
			return nil
		}
		if isGitIgnored(path) && !options.ShowIgnored {
			return nil
		}
		files = append(files, path)
		return nil
	})
	if err != nil {
		return err
	}

	for position, filePath := range files {
		if err := ctx.Err(); err != nil {
			return err
		}
		if options.Progress != nil && position%256 == 0 {
			options.Progress(StreamProgress{Current: position, Phase: "resolving-file-identities"})
		}
		if err := ctx.Err(); err != nil {
			return err
		}
		ignored := isGitIgnored(filePath)
		generated := IsGenerated(filePath)
		kind := DeriveFileKind(filepath.Base(filePath))

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeKind(kind, options) {
			continue
		}

		name := filepath.Base(filePath)
		if !matchesFilter(name, options) {
			continue
		}
		queryMatches, queryErr := matchesQuery(ctx, filePath, options)
		if queryErr != nil {
			return queryErr
		}
		if !queryMatches {
			continue
		}

		relPath, _ := filepath.Rel(rootDir, filePath)
		var bundleID *string
		if id, found := layout.BoundaryIDByPath(relPath); found {
			bundleID = &id
		}

		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := layout.BuildFolderID(folderPath, bundleID)
			folderID = &id
		}

		resolvedID := layout.BuildFileID(relPath, bundleID)
		select {
		case <-ctx.Done():
			return ctx.Err()
		case out <- File{
			ID:        resolvedID,
			Path:      relPath,
			URI:       model.ArtifactURIForID("file", resolvedID),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
			Kind:      kind,
			Ignored:   ignored,
			Generated: generated,
		}:
		}
	}
	return nil
}

// 💙️flattenSections holds the data fields for a flattenSections record.
func flattenSections(sections []Section) []Section {
	return flattenSectionsWithPrefix(sections, "")
}

// 🔧️flattenSectionsWithPrefix holds the data fields for a flattenSectionsWithPrefix record.
func flattenSectionsWithPrefix(sections []Section, prefix string) []Section {
	var result []Section
	for _, s := range sections {
		children := s.Children
		s.Children = nil
		sPrefix := s.Name
		if s.Emoji != "" {
			sPrefix = s.Emoji + s.Name
		}
		if prefix != "" {
			s.Path = prefix + "#" + sPrefix
		} else {
			s.Path = sPrefix
		}
		result = append(result, s)
		result = append(result, flattenSectionsWithPrefix(children, s.Path)...)
	}
	return result
}

// 💚️hydrateSectionMetadata holds the data fields for a hydrateSectionMetadata record.
func hydrateSectionMetadata(s *Section, filePath string, prefix string) {
	s.FilePath = filePath
	s.Path = prefix
	for i := range s.Children {
		childEmoji := s.Children[i].Emoji
		childPrefix := s.Children[i].Name
		if childEmoji != "" {
			childPrefix = childEmoji + childPrefix
		}
		childPath := prefix + "#" + childPrefix
		hydrateSectionMetadata(&s.Children[i], filePath, childPath)
	}
}

// 💛️StreamSections MUST invoke the callback for each matching sections entry.
// ▪️StreamSections streams sections entries through the callback.
func StreamSections(ctx context.Context, scope string, out chan<- Section, opts ...StreamOptions) error {
	defer close(out)
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan File)
	streamErrors := make(chan error, 1)
	go func() { streamErrors <- StreamFiles(ctx, scope, fileChan, filesOpts) }()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(rootDir, fullPath)
		}
		content, err := ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, f.Path)
		if options.Filter != "" || options.Regex {
			flatSections := flattenSections(sections)
			for _, s := range flatSections {
				s.FilePath = f.Path
				if !matchesFilter(s.Name, options) {
					continue
				}
				queryMatches, queryErr := matchesQuery(ctx, s.Name+" "+s.FilePath, options)
				if queryErr != nil {
					return queryErr
				}
				if !queryMatches {
					continue
				}

				select {
				case <-ctx.Done():
					return ctx.Err()
				case out <- s:
				}
			}
		} else {
			for i := range sections {
				rootPrefix := sections[i].Name
				if sections[i].Emoji != "" {
					rootPrefix = sections[i].Emoji + rootPrefix
				}
				hydrateSectionMetadata(&sections[i], f.Path, rootPrefix)
				select {
				case <-ctx.Done():
					return ctx.Err()
				case out <- sections[i]:
				}
			}
		}
	}
	return <-streamErrors
}

// 🧡️StreamDefinitions MUST invoke the callback for each matching definitions entry.
// ▶️StreamDefinitions streams definitions entries through the callback.
func StreamDefinitions(ctx context.Context, scope string, out chan<- Definition, opts ...StreamOptions) error {
	defer close(out)
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan File)
	streamErrors := make(chan error, 1)
	go func() { streamErrors <- StreamFiles(ctx, scope, fileChan, filesOpts) }()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(rootDir, fullPath)
		}
		content, err := ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		for _, d := range defs {
			rawKind := d.Kind
			if rawKind == "" {
				rawKind = "definition"
			}
			kind := DeriveDefinitionKind(rawKind)

			if !shouldIncludeDefinitionKind(kind, options) {
				continue
			}

			def := Definition{
				Name:      d.Name,
				Kind:      kind,
				FilePath:  f.Path,
				StartLine: d.Start,
				EndLine:   d.End,
			}

			if !matchesFilter(def.Name, options) {
				continue
			}
			queryMatches, queryErr := matchesQuery(ctx, def.Name+" "+string(def.Kind)+" "+def.FilePath, options)
			if queryErr != nil {
				return queryErr
			}
			if !queryMatches {
				continue
			}

			select {
			case <-ctx.Done():
				return ctx.Err()
			case out <- def:
			}
		}
	}
	return <-streamErrors
}
