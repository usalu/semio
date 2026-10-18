// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📰️generate carries the generation of technology requirements, docs and todos and the test info parsing of a command.

// #endregion 🧲️Header

package testrunner

import (
	fmt "fmt"
	fs "io/fs"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🕸️Test Command

// 🟩️findTechnologyByName holds the data fields for a findTechnologyByName record.
func findTechnologyByName(name string) *model.Technology {
	technologies := codebase.LoadTechnologies()
	for i := range technologies {
		if technologies[i].Name == name {
			return &technologies[i]
		}
	}
	return nil
}

// 🟦️walkTechnologyFiles holds the data fields for a walkTechnologyFiles record.
func walkTechnologyFiles(technology *model.Technology) []model.File {
	var files []model.File
	for _, bundle := range technology.Bundles {
		bundleRoot := filepath.Join(workspace.RootDir, bundle.Root)
		err := filepath.WalkDir(bundleRoot, func(path string, d fs.DirEntry, err error) error {
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
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			if workspace.IsGitIgnored(path) || model.IsGenerated(path) {
				return nil
			}
			kind := model.DeriveFileKind(d.Name())
			bundleID := bundle.GetID()
			files = append(files, model.File{
				ID:        model.BuildFileID(relPath, &bundleID),
				Path:      relPath,
				URI:       model.BuildFileUriFromPath(relPath),
				Name:      d.Name(),
				Extension: filepath.Ext(d.Name()),
				BundleID:  &bundleID,
				Kind:      kind,
			})
			return nil
		})
		_ = err
	}
	return files
}

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
	technologyReadmePath := filepath.Join(workspace.RootDir, technology.Root, "README.md")
	if content, err := workspace.ReadTextFile(technologyReadmePath); err == nil {
		requirements := ExtractMarkdownSection(content, "Requirements")
		if requirements != "" {
			p := &model.Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: requirements})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(workspace.RootDir, bundle.Root, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil {
			requirements := ExtractMarkdownSection(content, "Requirements")
			if requirements != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: requirements})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := workspace.ReadTextFile(filepath.Join(workspace.RootDir, frp)); err == nil {
				requirements := ExtractMarkdownSection(content, "Requirements")
				if requirements != "" {
					folderPath := filepath.Dir(frp)
					f := &model.Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: requirements})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != model.FileKindCode && file.Kind != model.FileKindScript {
			continue
		}
		headerRequirements := ExtractFileHeaderRequirements(file.Path)
		if headerRequirements != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: headerRequirements})
		}
		absPath := filepath.Join(workspace.RootDir, file.Path)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []model.Section)
		walkSections = func(secs []model.Section) {
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
				def := model.Definition{
					Name:      dr.Name,
					Kind:      model.DeriveDefinitionKind(dr.Kind),
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
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "SPECS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 🟪️GenerateTechnologyDocs holds the data fields for a GenerateTechnologyDocs record.
func GenerateTechnologyDocs(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	technologyReadmePath := filepath.Join(workspace.RootDir, technology.Root, "README.md")
	if content, err := workspace.ReadTextFile(technologyReadmePath); err == nil {
		docs := ExtractMarkdownSection(content, "Docs")
		if docs != "" {
			p := &model.Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: docs})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(workspace.RootDir, bundle.Root, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil {
			docs := ExtractMarkdownSection(content, "Docs")
			if docs != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: docs})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := workspace.ReadTextFile(filepath.Join(workspace.RootDir, frp)); err == nil {
				docs := ExtractMarkdownSection(content, "Docs")
				if docs != "" {
					folderPath := filepath.Dir(frp)
					f := &model.Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: docs})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != model.FileKindCode && file.Kind != model.FileKindScript {
			continue
		}
		summary := ExtractFileHeaderSummary(file.Path)
		if summary != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: summary})
		}
		absPath := filepath.Join(workspace.RootDir, file.Path)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []model.Section)
		walkSections = func(secs []model.Section) {
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
				def := model.Definition{
					Name:      dr.Name,
					Kind:      model.DeriveDefinitionKind(dr.Kind),
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
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "DOCS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 🟫️GenerateTechnologyTodos holds the data fields for a GenerateTechnologyTodos record.
func GenerateTechnologyTodos(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	for _, bundle := range technology.Bundles {
		bundleRoot := filepath.Join(workspace.RootDir, bundle.Root)
		todos := todospkg.LocateTodosOnDisk(bundleRoot, todospkg.ScanTodos(todospkg.NewFsTodoTree(bundleRoot)))
		for _, todo := range todos {
			if todo.Location != nil && workspace.IsGitIgnored(todo.Location.FilePath) {
				continue
			}
			if todo.Location != nil && model.IsGenerated(todo.Location.FilePath) {
				continue
			}
			title := todo.Name
			desc := todo.Description
			parentPath := todo.ParentID
			relParent, _ := filepath.Rel(workspace.RootDir, parentPath)
			relParent = workspace.NormalizePath(relParent)
			entityID := relParent
			entityURI := model.BuildFileUriFromPath(relParent)
			if todo.Location != nil {
				relLoc, _ := filepath.Rel(workspace.RootDir, todo.Location.FilePath)
				relLoc = workspace.NormalizePath(relLoc)
				entityID = relLoc
				entityURI = model.BuildFileUriFromPath(relLoc)
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
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "TODOS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 📁️findFolderReadmes holds the data fields for a findFolderReadmes record.
func findFolderReadmes(bundleRoot string) []string {
	var readmes []string
	absRoot := filepath.Join(workspace.RootDir, bundleRoot)
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
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			if relPath != workspace.NormalizePath(filepath.Join(bundleRoot, "README.md")) {
				readmes = append(readmes, relPath)
			}
		}
		return nil
	})
	return readmes
}

// 📬️getBundlesWithOpenTickets holds the data fields for a getBundlesWithOpenTickets record.
func GetBundlesWithOpenTickets() map[string]bool {
	bundlesWithOpenTickets := make(map[string]bool)
	tickets, err := ticketspkg.ListTickets(nil, nil, nil)
	if err != nil {
		return bundlesWithOpenTickets
	}

	for _, t := range tickets {
		if t.Status != model.TicketStatusOpen {
			continue
		}
		for _, iter := range t.Interactions {
			for _, f := range iter.Files {
				parts := strings.Split(f.Path, "/")
				if len(parts) >= 2 && strings.HasPrefix(parts[0], "@") {
					bundleName := parts[0] + "/" + parts[1]
					bundlesWithOpenTickets[bundleName] = true
				}
			}
		}
	}
	return bundlesWithOpenTickets
}

// #endregion 🕸️Test Command

// #region 📰️Todos

// 🔬️parseTestInfoFromCommand parses test information from a command string.
// ⏰️Returns a slice of test names/patterns and a timeout duration string.
func ParseTestInfoFromCommand(command string) ([]string, string) {
	segment, _ := ExtractTestSegmentFromCommand(command)
	if segment != "" {
		command = segment
	}
	command = strings.TrimSpace(command)
	fields := strings.Fields(command)
	var tests []string
	timeout := ""

	if strings.Contains(command, "-timeout ") {
		parts := strings.Split(command, "-timeout ")
		if len(parts) > 1 {
			timeoutPart := strings.Fields(parts[1])[0]
			if strings.HasSuffix(timeoutPart, "s") {
				timeout = strings.TrimSuffix(timeoutPart, "s")
			} else if strings.HasSuffix(timeoutPart, "m") {
				mins := strings.TrimSuffix(timeoutPart, "m")
				if minsInt, err := strconv.Atoi(mins); err == nil {
					timeout = strconv.Itoa(minsInt * 60)
				}
			} else {
				timeout = timeoutPart
			}
		}
	} else if strings.Contains(command, "--timeout ") {
		parts := strings.Split(command, "--timeout ")
		if len(parts) > 1 {
			timeout = strings.Fields(parts[1])[0]
		}
	}
	flagValue := func(flags ...string) string {
		for i := 0; i < len(fields); i++ {
			for _, flag := range flags {
				if fields[i] == flag && i+1 < len(fields) {
					return fields[i+1]
				}
				prefix := flag + "="
				if strings.HasPrefix(fields[i], prefix) {
					return strings.TrimPrefix(fields[i], prefix)
				}
			}
		}
		return ""
	}
	joinAfterMarker := func(marker string) string {
		for i := 0; i < len(fields); i++ {
			if fields[i] == marker && i+1 < len(fields) {
				return fields[i+1]
			}
		}
		return ""
	}

	switch {
	case strings.HasPrefix(command, "go test"):
		if value := flagValue("-run"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "pytest"), strings.HasPrefix(command, "python -m pytest"), strings.HasPrefix(command, "python3 -m pytest"), strings.HasPrefix(command, "uv run pytest"), strings.HasPrefix(command, "uvx pytest"):
		if value := flagValue("-k"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "jest"), strings.HasPrefix(command, "npx jest"), strings.HasPrefix(command, "bunx jest"), strings.HasPrefix(command, "pnpx jest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "mocha"), strings.HasPrefix(command, "npx mocha"), strings.HasPrefix(command, "bunx mocha"), strings.HasPrefix(command, "pnpx mocha"):
		if value := flagValue("--grep"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo test"):
		if value := joinAfterMarker("--"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo nextest"):
		tests = []string{""}
	case strings.HasPrefix(command, "dotnet test"):
		if value := flagValue("--filter"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "rspec"), strings.HasPrefix(command, "bundle exec rspec"):
		if value := flagValue("--example"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "vitest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "phpunit"), strings.Contains(command, "npm test"), strings.Contains(command, "pnpm test"), strings.Contains(command, "yarn test"), strings.Contains(command, "bun test"):
		tests = []string{""}
	default:
		tests = []string{""}
	}

	return tests, timeout
}

// #endregion 📰️Todos
