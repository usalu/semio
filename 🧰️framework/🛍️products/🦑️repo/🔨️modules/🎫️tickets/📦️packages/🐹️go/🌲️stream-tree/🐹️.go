package tickets

import (
	context "context"
	"fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌲️Stream Tree

// 🩷️StreamFolders MUST invoke the callback for each matching folders entry.
// 📄️StreamFolders streams folders entries through the callback.
func StreamFolders(ctx context.Context, scope string, out chan<- model.Folder, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	layout := options.Layout
	if layout == nil {
		boundaries := []model.ArtifactBoundary{}
		for _, bundle := range codebase.GetTechnologies() {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
		layout = model.NewArtifactLayout(workspace.GetRootDir(), boundaries)
	}
	if layout.Root() != workspace.GetRootDir() {
		return fmt.Errorf("Exact invocation layout repository required")
	}
	root := workspace.GetRootDir()
	if bundleName, found := strings.CutPrefix(scope, "compose/"); found {
		bundles := codebase.GetTechnologies()
		for _, b := range bundles {
			if b.Name == bundleName || model.NormalizeBundleLabel(b.Name) == bundleName {
				root = filepath.Join(workspace.GetRootDir(), b.Root)
				break
			}
		}
	} else if scope != "" && scope != "compose" {
		if filepath.IsAbs(scope) {
			root = scope
		} else {
			root = filepath.Join(workspace.GetRootDir(), scope)
		}
	}

	if err := ctx.Err(); err != nil {
		return err
	}
	if workspace.IsRepoExcludedPath(root) {
		return nil
	}
	visited := 0
	phase := "discovering-folders"
	var folders []string
	err := filepath.WalkDir(root, func(path string, info os.DirEntry, err error) error {
		if err := ctx.Err(); err != nil {
			return err
		}
		visited++
		if options.Progress != nil && visited%256 == 0 {
			options.Progress(model.StreamProgress{Current: visited, Phase: phase})
		}
		if err := ctx.Err(); err != nil {
			return err
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
		if workspace.IsRepoExcludedPath(path) {
			return filepath.SkipDir
		}
		if workspace.IsGitIgnored(path) && !options.ShowIgnored {
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
			options.Progress(model.StreamProgress{Current: position, Phase: "resolving-folder-identities"})
		}
		if err := ctx.Err(); err != nil {
			return err
		}
		ignored := workspace.IsGitIgnored(folderPath)
		relPath, _ := filepath.Rel(workspace.GetRootDir(), folderPath)
		generated := model.IsGenerated(folderPath) || model.IsGeneratedFolder(relPath)
		folderKind := layout.FolderKind(relPath)

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeFolderKind(folderKind, options) {
			continue
		}

		if !model.MatchesFilter(filepath.Base(folderPath), options) {
			continue
		}
		if !model.MatchesQuery(folderPath, options) {
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
		case out <- model.Folder{
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
func StreamFiles(ctx context.Context, scope string, out chan<- model.File, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	layout := options.Layout
	if layout == nil {
		boundaries := []model.ArtifactBoundary{}
		for _, bundle := range codebase.GetTechnologies() {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
		layout = model.NewArtifactLayout(workspace.GetRootDir(), boundaries)
	}
	if layout.Root() != workspace.GetRootDir() {
		return fmt.Errorf("Exact invocation layout repository required")
	}
	root := workspace.GetRootDir()
	if scope != "" && scope != "compose" {
		bundles := codebase.GetTechnologies()
		matched := false
		for _, b := range bundles {
			if b.Name == scope || b.Root == scope || model.NormalizeBundleLabel(b.Name) == scope {
				root = filepath.Join(workspace.GetRootDir(), b.Root)
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
					if b.Name == bundleScope || b.Root == bundleScope || model.NormalizeBundleLabel(b.Name) == bundleScope {
						root = filepath.Join(workspace.GetRootDir(), b.Root, subPath)
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
				root = filepath.Join(workspace.GetRootDir(), scope)
			}
		}
	}

	if err := ctx.Err(); err != nil {
		return err
	}
	if workspace.IsRepoExcludedPath(root) {
		return nil
	}
	if info, err := os.Stat(root); err == nil && !info.IsDir() {
		relPath, _ := filepath.Rel(workspace.GetRootDir(), root)
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
		file := model.File{
			ID:        resolvedID,
			Path:      relPath,
			URI:       model.ArtifactURIForID("file", resolvedID),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
			Kind:      model.DeriveFileKind(filepath.Base(relPath)),
			Ignored:   workspace.IsGitIgnored(root),
			Generated: model.IsGenerated(root),
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		case out <- file:
			return nil
		}
	}

	visited := 0
	phase := "discovering-files"
	var files []string
	err := filepath.WalkDir(root, func(path string, info os.DirEntry, err error) error {
		if err := ctx.Err(); err != nil {
			return err
		}
		visited++
		if options.Progress != nil && visited%256 == 0 {
			options.Progress(model.StreamProgress{Current: visited, Phase: phase})
		}
		if err := ctx.Err(); err != nil {
			return err
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
				if workspace.IsRepoExcludedPath(path) {
					return filepath.SkipDir
				}
				if workspace.IsGitIgnored(path) && !options.ShowIgnored {
					return filepath.SkipDir
				}
			}
			return nil
		}
		if workspace.IsRepoExcludedPath(path) {
			return nil
		}
		if workspace.IsGitIgnored(path) && !options.ShowIgnored {
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
			options.Progress(model.StreamProgress{Current: position, Phase: "resolving-file-identities"})
		}
		if err := ctx.Err(); err != nil {
			return err
		}
		ignored := workspace.IsGitIgnored(filePath)
		generated := model.IsGenerated(filePath)
		kind := model.DeriveFileKind(filepath.Base(filePath))

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
		if !model.MatchesFilter(name, options) {
			continue
		}
		if !model.MatchesQuery(filePath, options) {
			continue
		}

		relPath, _ := filepath.Rel(workspace.GetRootDir(), filePath)
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
		case out <- model.File{
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
func flattenSections(sections []model.Section) []model.Section {
	return flattenSectionsWithPrefix(sections, "")
}

// 🔧️flattenSectionsWithPrefix holds the data fields for a flattenSectionsWithPrefix record.
func flattenSectionsWithPrefix(sections []model.Section, prefix string) []model.Section {
	var result []model.Section
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
func hydrateSectionMetadata(s *model.Section, filePath string, prefix string) {
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
func StreamSections(ctx context.Context, scope string, out chan<- model.Section, opts ...model.StreamOptions) error {
	defer close(out)
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan model.File)
	streamErrors := make(chan error, 1)
	go func() { streamErrors <- StreamFiles(ctx, scope, fileChan, filesOpts) }()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(workspace.GetRootDir(), fullPath)
		}
		content, err := workspace.ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		sections := languages.ParseSections(content, f.Path)
		if options.Filter != "" || options.Regex {
			flatSections := flattenSections(sections)
			for _, s := range flatSections {
				s.FilePath = f.Path
				if !model.MatchesFilter(s.Name, options) {
					continue
				}
				if !model.MatchesQuery(s.Name+" "+s.FilePath, options) {
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
func StreamDefinitions(ctx context.Context, scope string, out chan<- model.Definition, opts ...model.StreamOptions) error {
	defer close(out)
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	fileChan := make(chan model.File)
	streamErrors := make(chan error, 1)
	go func() { streamErrors <- StreamFiles(ctx, scope, fileChan, filesOpts) }()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(workspace.GetRootDir(), fullPath)
		}
		content, err := workspace.ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(f.Path)
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
			kind := model.DeriveDefinitionKind(rawKind)

			if !shouldIncludeDefinitionKind(kind, options) {
				continue
			}

			def := model.Definition{
				Name:      d.Name,
				Kind:      kind,
				FilePath:  f.Path,
				StartLine: d.Start,
				EndLine:   d.End,
			}

			if !model.MatchesFilter(def.Name, options) {
				continue
			}
			if !model.MatchesQuery(def.Name+" "+string(def.Kind)+" "+def.FilePath, options) {
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

// ❤️ResolveBundleForPath MUST return the resolved value or an error if unresolvable.
// 📩️ResolveBundleForPath resolves and returns the bundle for path.
func ResolveBundleForPath(filePath string, bundles []model.Bundle) string {
	var bestMatch string
	var maxLen int
	for _, b := range bundles {
		if b.Name == "" {
			continue
		}
		if strings.HasPrefix(filePath, b.Root+"/") || filePath == b.Root {
			if len(b.Root) > maxLen {
				maxLen = len(b.Root)
				bestMatch = model.NormalizeBundleLabel(b.Name)
			}
		}
	}
	return bestMatch
}

// #endregion 🌲️Stream Tree
