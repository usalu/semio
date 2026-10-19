package statutes

import (
	json "encoding/json"
	fmt "fmt"
	io "io"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📁️Folder And File Policy

// 📁folderPolicy holds the data fields for a folderPolicy record.
func folderPolicy(ctx *PolicyContext) []model.Breach {
	if ctx.Scope.Kind != workspace.ScopeRepo {
		return nil
	}
	var breachs []model.Breach
	breachs = append(breachs, pathEmojiBreaches(ctx, "directory")...)
	excludePrefixes := []string{
		".git/",
		".🧬semio/🦑️repo/",
		"node_modules/",
		".venv/",
		".nx/",
	}

	err := filepath.WalkDir(ctx.RootDir, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() {
			return nil
		}
		relPath, _ := filepath.Rel(ctx.RootDir, path)
		relPath = workspace.NormalizePath(relPath)
		if relPath == "." {
			return nil
		}
		for _, prefix := range excludePrefixes {
			if strings.HasPrefix(relPath+"/", prefix) {
				return filepath.SkipDir
			}
		}

		if workspace.IsIgnoredByGitignore(filepath.Join(ctx.RootDir, relPath)) {
			return filepath.SkipDir
		}

		f, readErr := os.Open(path)
		if readErr != nil {
			return nil
		}
		defer f.Close()
		_, readErr = f.Readdirnames(1)
		if readErr == io.EOF {
			breachs = append(breachs, ctx.CreateBreach(
				"Empty folder \""+relPath+"\" must be removed",
				model.BreachFolderIllegalEmpty,
				relPath+"/", 0, 0, relPath))
		}
		return nil
	})
	_ = err
	return breachs
}

// ◼️️Godfile holds the data fields for a Godfile record.
type Godfile struct {
	Exact map[string]bool
	Globs []string
}

// 🔵isGlobPattern holds the data fields for a isGlobPattern record.
func isGlobPattern(value string) bool {
	return strings.ContainsAny(value, "*?[{")
}

// 🔴️loadGodfile holds the data fields for a loadGodfile record.
func loadGodfile() (*Godfile, error) {
	godfilePath := filepath.Join(workspace.GetRepoMetaDir(), "📁️files.json")
	data, err := os.ReadFile(godfilePath)
	if err != nil {
		return nil, err
	}
	var files []string
	if err := json.Unmarshal(data, &files); err != nil {
		return nil, err
	}
	result := &Godfile{
		Exact: make(map[string]bool, len(files)),
	}
	for _, f := range files {
		normalized := workspace.NormalizePath(f)
		if isGlobPattern(normalized) {
			result.Globs = append(result.Globs, normalized)
			continue
		}
		result.Exact[normalized] = true
	}
	return result, nil
}

// 🟠godfileMatchesPath holds the data fields for a godfileMatchesPath record.
func godfileMatchesPath(godfile *Godfile, relPath string) bool {
	relPath = workspace.NormalizeRepoPath(relPath)
	if godfile.Exact[relPath] {
		return true
	}
	for _, pattern := range godfile.Globs {
		matched, err := workspace.Match(pattern, relPath)
		if err != nil {
			continue
		}
		if matched {
			return true
		}
	}
	return false
}

// 🟡filePolicy holds the data fields for a filePolicy record.
func filePolicy(ctx *PolicyContext) []model.Breach {
	if ctx.Scope.Kind != workspace.ScopeRepo {
		return nil
	}
	var breachs []model.Breach
	breachs = append(breachs, pathEmojiBreaches(ctx, "file")...)
	godfile, err := loadGodfile()
	if err != nil {
		return breachs
	}
	_ = filepath.WalkDir(workspace.GetRootDir(), func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if workspace.IsRepoExcludedPath(path) || strings.HasPrefix(d.Name(), ".git") || d.Name() == "node_modules" || d.Name() == ".venv" {
				return filepath.SkipDir
			}
			return nil
		}
		if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
			return nil
		}
		relPath := workspace.NormalizeRepoPath(path)
		if !godfileMatchesPath(godfile, relPath) {
			breachs = append(breachs, ctx.CreateBreach(
				"File \""+relPath+"\" is not listed in .🧬semio/🦑️repo/📁️files.json",
				model.BreachFileIllegalUseGodfile,
				relPath, 0, 0, relPath))
		}
		return nil
	})
	return breachs
}

// 🟢composePolicy validates that compose.* files do not import UI dependencies.
func composePolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	uiPackages := []string{
		"clsx", "tailwind-merge", "tailwind", "tailwindcss",
		"three", "@react-three",
		"react", "react-dom",
		"framer-motion",
		"@radix-ui", "@radix-client",
		"@dnd-kit",
		"zustand",
		"elements/ui",
	}
	for _, file := range files {
		baseName := filepath.Base(file)
		if !strings.HasPrefix(baseName, "compose.") {
			continue
		}
		if !strings.HasSuffix(file, ".ts") && !strings.HasSuffix(file, ".tsx") {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lines := strings.Split(content, "\n")
		for lineNum, line := range lines {
			lineNumber := lineNum + 1
			if !strings.Contains(line, "import ") {
				continue
			}
			for _, pkg := range uiPackages {
				importPattern := fmt.Sprintf(`from\s+['"].*%s`, regexp.QuoteMeta(pkg))
				if matched, _ := regexp.MatchString(importPattern, line); matched {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("UI dependency '%s' is not allowed in compose.* files", pkg),
						model.BreachComposeNoUiDependency,
						file, lineNumber, 0, strings.TrimSpace(line)))
					break
				}
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

// #endregion 📁️Folder And File Policy
