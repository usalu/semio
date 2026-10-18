// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Technology requirements, docs, and todos generation.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"io/fs"
	"path/filepath"
	"strings"
)

// 📍️EntityEntry holds the data fields for a EntityEntry record.
type EntityEntry struct {
	ID   string
	URI  string
	Text string
}

func GenerateTechnologyRequirements(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	technologyReadmePath := filepath.Join(rootDir, technology.Root, "README.md")
	if content, err := ReadTextFile(technologyReadmePath); err == nil {
		requirements := ExtractMarkdownSection(content, "Requirements")
		if requirements != "" {
			p := &Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: requirements})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(rootDir, bundle.Root, "README.md")
		if content, err := ReadTextFile(readmePath); err == nil {
			requirements := ExtractMarkdownSection(content, "Requirements")
			if requirements != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: requirements})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := ReadTextFile(filepath.Join(rootDir, frp)); err == nil {
				requirements := ExtractMarkdownSection(content, "Requirements")
				if requirements != "" {
					folderPath := filepath.Dir(frp)
					f := &Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: requirements})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != FileKindCode && file.Kind != FileKindScript {
			continue
		}
		headerRequirements := ExtractFileHeaderRequirements(file.Path)
		if headerRequirements != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: headerRequirements})
		}
		absPath := filepath.Join(rootDir, file.Path)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []Section)
		walkSections = func(secs []Section) {
			for _, s := range secs {
				if strings.ToLower(s.Name) == "header" {
					walkSections(s.Children)
					continue
				}
				requirements, _ := ExtractSectionLeadComments(content, s, prefix)
				if requirements != "" {
					s.FilePath = file.Path
					entries = append(entries, EntityEntry{ID: s.GetID(), URI: s.GetURI(), Text: requirements})
				}
				walkSections(s.Children)
			}
		}
		walkSections(sections)
		if lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			for _, dr := range defs {
				def := Definition{
					Name:      dr.Name,
					Kind:      DeriveDefinitionKind(dr.Kind),
					FilePath:  file.Path,
					StartLine: dr.Start,
					EndLine:   dr.End,
				}
				requirements, _ := ExtractDefinitionDocstring(content, def, prefix)
				if requirements != "" {
					entries = append(entries, EntityEntry{ID: def.GetID(), URI: def.GetURI(), Text: requirements})
				}
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F4AF Requirements\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		sb.WriteString(e.Text + "\n")
	}
	outputPath := filepath.Join(rootDir, technology.Root, "SPECS.md")
	return WriteTextFile(outputPath, sb.String())
}

// 🟪️GenerateTechnologyDocs holds the data fields for a GenerateTechnologyDocs record.
func GenerateTechnologyDocs(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	technologyReadmePath := filepath.Join(rootDir, technology.Root, "README.md")
	if content, err := ReadTextFile(technologyReadmePath); err == nil {
		docs := ExtractMarkdownSection(content, "Docs")
		if docs != "" {
			p := &Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: docs})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(rootDir, bundle.Root, "README.md")
		if content, err := ReadTextFile(readmePath); err == nil {
			docs := ExtractMarkdownSection(content, "Docs")
			if docs != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: docs})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := ReadTextFile(filepath.Join(rootDir, frp)); err == nil {
				docs := ExtractMarkdownSection(content, "Docs")
				if docs != "" {
					folderPath := filepath.Dir(frp)
					f := &Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: docs})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != FileKindCode && file.Kind != FileKindScript {
			continue
		}
		summary := ExtractFileHeaderSummary(file.Path)
		if summary != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: summary})
		}
		absPath := filepath.Join(rootDir, file.Path)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []Section)
		walkSections = func(secs []Section) {
			for _, s := range secs {
				if strings.ToLower(s.Name) == "header" {
					walkSections(s.Children)
					continue
				}
				_, summaryText := ExtractSectionLeadComments(content, s, prefix)
				if summaryText != "" {
					s.FilePath = file.Path
					entries = append(entries, EntityEntry{ID: s.GetID(), URI: s.GetURI(), Text: summaryText})
				}
				walkSections(s.Children)
			}
		}
		walkSections(sections)
		if lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			for _, dr := range defs {
				def := Definition{
					Name:      dr.Name,
					Kind:      DeriveDefinitionKind(dr.Kind),
					FilePath:  file.Path,
					StartLine: dr.Start,
					EndLine:   dr.End,
				}
				_, summaryText := ExtractDefinitionDocstring(content, def, prefix)
				if summaryText != "" {
					entries = append(entries, EntityEntry{ID: def.GetID(), URI: def.GetURI(), Text: summaryText})
				}
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F4DA Docs\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		text := e.Text
		if text != "" {
			var filtered []string
			for _, line := range strings.Split(text, "\n") {
				trimmed := strings.TrimSpace(line)
				if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") || strings.HasPrefix(trimmed, "#region") || strings.HasPrefix(trimmed, "#endregion") {
					continue
				}
				filtered = append(filtered, line)
			}
			text = strings.TrimSpace(strings.Join(filtered, "\n"))
		}
		if text == "" {
			continue
		}
		sb.WriteString(text + "\n")
	}
	outputPath := filepath.Join(rootDir, technology.Root, "DOCS.md")
	return WriteTextFile(outputPath, sb.String())
}

// 🟫️GenerateTechnologyTodos holds the data fields for a GenerateTechnologyTodos record.
func GenerateTechnologyTodos(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	for _, bundle := range technology.Bundles {
		bundleRoot := filepath.Join(rootDir, bundle.Root)
		todos, _ := ScanTodos(bundleRoot)
		for _, todo := range todos {
			if todo.Location != nil && isGitIgnored(todo.Location.FilePath) {
				continue
			}
			if todo.Location != nil && IsGenerated(todo.Location.FilePath) {
				continue
			}
			title := todo.Name
			desc := todo.Description
			parentPath := todo.ParentID
			relParent, _ := filepath.Rel(rootDir, parentPath)
			relParent = NormalizePath(relParent)
			entityID := relParent
			entityURI := buildFileUriFromPath(relParent)
			if todo.Location != nil {
				relLoc, _ := filepath.Rel(rootDir, todo.Location.FilePath)
				relLoc = NormalizePath(relLoc)
				entityID = relLoc
				entityURI = buildFileUriFromPath(relLoc)
			}
			text := "### TODO: " + title
			if desc != "" {
				text += "\n" + desc
			}
			entries = append(entries, EntityEntry{ID: entityID, URI: entityURI, Text: text})
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F533 TODOs\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		sb.WriteString(e.Text + "\n")
	}
	outputPath := filepath.Join(rootDir, technology.Root, "TODOS.md")
	return WriteTextFile(outputPath, sb.String())
}

// 📁️findFolderReadmes holds the data fields for a findFolderReadmes record.
func findFolderReadmes(bundleRoot string) []string {
	var readmes []string
	absRoot := filepath.Join(rootDir, bundleRoot)
	filepath.WalkDir(absRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			name := d.Name()
			if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
				return fs.SkipDir
			}
			return nil
		}
		if d.Name() == "README.md" {
			relPath, _ := filepath.Rel(rootDir, path)
			relPath = NormalizePath(relPath)
			if relPath != NormalizePath(filepath.Join(bundleRoot, "README.md")) {
				readmes = append(readmes, relPath)
			}
		}
		return nil
	})
	return readmes
}
