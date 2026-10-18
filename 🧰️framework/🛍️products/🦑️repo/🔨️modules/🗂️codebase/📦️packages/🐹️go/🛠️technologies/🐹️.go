// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛠️technologies carries the technology cache, technology and package loading and the file header identifiers.

// #endregion 🧲️Header

package codebase

import (
	json "encoding/json"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	strings "strings"
	sync "sync"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	yaml "github.com/usalu/semio/repo/yaml"
)

// #region 🛠️Technologies

var technologyCache []model.Technology

var technologyCacheLoaded bool

var technologyCacheMutex sync.Mutex

// 📜️InvalidateTechnologyCache MUST clear the cached state to force a reload.
// ✔️InvalidateTechnologyCache invalidates the cached technology cache.
func InvalidateTechnologyCache() {
	technologyCacheMutex.Lock()
	defer technologyCacheMutex.Unlock()
	technologyCacheLoaded = false
	technologyCache = nil
}

// ◾LoadTechnologies MUST return all matching technologies from the data source.
// 📺️LoadTechnologies loads and returns technologies from the data source.
func LoadTechnologies() []model.Technology {
	technologyCacheMutex.Lock()
	defer technologyCacheMutex.Unlock()
	if technologyCacheLoaded {
		return technologyCache
	}
	technologyCache = loadTechnologiesInternal()
	technologyCacheLoaded = true
	return technologyCache
}

// ◽isTechnologyDir holds the data fields for a isTechnologyDir record.
func isTechnologyDir(name string) bool {
	readmePath := filepath.Join(workspace.RootDir, name, "README.md")
	content, err := workspace.ReadTextFile(readmePath)
	if err != nil || !strings.HasPrefix(content, "---") {
		return false
	}
	if strings.HasPrefix(name, ".") {
		return false
	}
	if name == "node_modules" || name == "target" || name == "temp" {
		return false
	}
	return true
}

// ◻loadTechnologiesInternal holds the data fields for a loadTechnologiesInternal record.
func loadTechnologiesInternal() []model.Technology {
	var technologies []model.Technology
	technologiesDir := workspace.RootDir
	entries, err := os.ReadDir(technologiesDir)
	if err != nil {
		return nil
	}

	for _, d := range entries {
		if !d.IsDir() {
			continue
		}
		name := d.Name()
		if !isTechnologyDir(name) {
			continue
		}

		rawName := strings.TrimPrefix(name, "@")

		technologyName := rawName
		technologyKind := model.DeriveTechnologyKind(rawName)
		readmePath := filepath.Join(workspace.RootDir, name, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil && strings.HasPrefix(content, "---") {
			endIdx := strings.Index(content[3:], "---")
			if endIdx > 0 {
				fmContent := content[3 : 3+endIdx]
				var fm struct {
					Name string `yaml:"name"`
					Kind string `yaml:"kind"`
				}
				if yaml.Unmarshal([]byte(fmContent), &fm) == nil {
					if fm.Name != "" {
						technologyName = fm.Name
					}
					if fm.Kind != "" {
						switch strings.ToLower(fm.Kind) {
						case "user":
							technologyKind = model.TechnologyKindUser
						case "infrastructure":
							technologyKind = model.TechnologyKindInfrastructure
						case "research":
							technologyKind = model.TechnologyKindResearch
						}
					}
				}
			}
		}

		technology := model.Technology{
			Name:    technologyName,
			Root:    name,
			Kind:    technologyKind,
			Bundles: []model.Bundle{},
		}

		// Read technology emoji from AGENTS.md frontmatter
		agentsPath := filepath.Join(workspace.RootDir, name, "AGENTS.md")
		if content, err := workspace.ReadTextFile(agentsPath); err == nil && strings.HasPrefix(content, "---") {
			endIdx := strings.Index(content[3:], "---")
			if endIdx > 0 {
				fmContent := content[3 : 3+endIdx]
				var agentsFm struct {
					Emoji string `yaml:"emoji"`
				}
				if yaml.Unmarshal([]byte(fmContent), &agentsFm) == nil && agentsFm.Emoji != "" {
					technology.Emoji = agentsFm.Emoji
				}
			}
		}

		technologyPath := filepath.Join(technologiesDir, name)
		subEntries, _ := os.ReadDir(technologyPath)
		for _, sub := range subEntries {
			if !sub.IsDir() {
				continue
			}
			if strings.HasPrefix(sub.Name(), ".") {
				continue
			}
			if sub.Name() == "node_modules" {
				continue
			}
			bunName := sub.Name()
			if bunName == "sites" {
				siteEntries, _ := os.ReadDir(filepath.Join(technologyPath, bunName))
				for _, site := range siteEntries {
					if !site.IsDir() {
						continue
					}
					if strings.HasPrefix(site.Name(), ".") || site.Name() == "node_modules" {
						continue
					}
					siteName := site.Name()
					fullBundleName := technologyName + "/" + siteName
					bundlePath := filepath.ToSlash(filepath.Join(name, bunName, siteName))
					bundle := model.Bundle{
						Name:           fullBundleName,
						Root:           bundlePath,
						TechnologyName: technologyName,
						Kind:           model.BundleKindSite,
					}
					// Read site bundle emoji from AGENTS.md frontmatter
					siteAgentsPath := filepath.Join(workspace.RootDir, bundlePath, "AGENTS.md")
					if content, err := workspace.ReadTextFile(siteAgentsPath); err == nil && strings.HasPrefix(content, "---") {
						endIdx := strings.Index(content[3:], "---")
						if endIdx > 0 {
							fmContent := content[3 : 3+endIdx]
							var siteFm struct {
								Bundle struct {
									Emoji string `yaml:"emoji"`
								} `yaml:"bundle"`
							}
							if yaml.Unmarshal([]byte(fmContent), &siteFm) == nil && siteFm.Bundle.Emoji != "" {
								bundle.Emoji = siteFm.Bundle.Emoji
							}
						}
					}
					bundle.Packages = loadPackages(filepath.Join(workspace.RootDir, bundlePath))
					configPath := filepath.Join(technologyPath, bunName, siteName, "project.json")
					if !workspace.FileExists(configPath) {
						configPath = filepath.Join(technologyPath, bunName, siteName, "package.json")
					}
					if workspace.FileExists(configPath) {
						content, err := workspace.ReadTextFile(configPath)
						if err == nil {
							var meta struct {
								SourceRoot string   `json:"sourceRoot"`
								Tags       []string `json:"tags"`
							}
							if json.Unmarshal([]byte(content), &meta) == nil {
								bundle.SourceRoot = meta.SourceRoot
								bundle.Tags = meta.Tags
							}
						}
					}
					technology.Bundles = append(technology.Bundles, bundle)
				}
				continue
			}
			fullBundleName := technologyName + "/" + bunName

			bundlePath := filepath.ToSlash(filepath.Join(name, bunName))
			kind := model.DeriveBundleKind(fullBundleName, bundlePath)

			bundle := model.Bundle{
				Name:           fullBundleName,
				Root:           bundlePath,
				TechnologyName: technologyName,
				Kind:           kind,
			}

			// Read bundle emoji from AGENTS.md frontmatter
			bundleAgentsPath := filepath.Join(workspace.RootDir, bundlePath, "AGENTS.md")
			if content, err := workspace.ReadTextFile(bundleAgentsPath); err == nil && strings.HasPrefix(content, "---") {
				endIdx := strings.Index(content[3:], "---")
				if endIdx > 0 {
					fmContent := content[3 : 3+endIdx]
					var bundleFm struct {
						Bundle struct {
							Emoji string `yaml:"emoji"`
						} `yaml:"bundle"`
					}
					if yaml.Unmarshal([]byte(fmContent), &bundleFm) == nil && bundleFm.Bundle.Emoji != "" {
						bundle.Emoji = bundleFm.Bundle.Emoji
					}
				}
			}

			bundle.Packages = loadPackages(filepath.Join(workspace.RootDir, bundlePath))

			configPath := filepath.Join(technologyPath, bunName, "project.json")
			if !workspace.FileExists(configPath) {
				configPath = filepath.Join(technologyPath, bunName, "package.json")
			}
			if workspace.FileExists(configPath) {
				content, err := workspace.ReadTextFile(configPath)
				if err == nil {
					var meta struct {
						SourceRoot string   `json:"sourceRoot"`
						Tags       []string `json:"tags"`
					}
					if json.Unmarshal([]byte(content), &meta) == nil {
						bundle.SourceRoot = meta.SourceRoot
						bundle.Tags = meta.Tags
					}
				}
			}
			technology.Bundles = append(technology.Bundles, bundle)
		}
		technologies = append(technologies, technology)
	}

	return technologies
}

// 📦️LoadBundles MUST return all matching bundles from the data source.
// 🗄️LoadBundles loads and returns bundles from the data source.
func LoadBundles() []model.Bundle {
	var bundles []model.Bundle
	technologies := LoadTechnologies()
	for _, p := range technologies {
		bundles = append(bundles, p.Bundles...)
	}
	return bundles
}

// 📨️GetTechnologies MUST retrieve the requested value or return an error.
// 🔺️GetTechnologies retrieves and returns the technologies.
func GetTechnologies() []model.Bundle {
	return LoadBundles()
}

// 🟠️loadPackages holds the data fields for a loadPackages record.
func loadPackages(bundleRoot string) []model.Package {
	var packages []model.Package
	filepath.WalkDir(bundleRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}

		if d.IsDir() {
			name := d.Name()
			if name == "node_modules" || name == ".git" || name == "bin" || name == "obj" || name == "target" || name == "dist" || name == "build" || name == "__pycache__" || strings.HasPrefix(name, ".") {
				return fs.SkipDir
			}
			return nil
		}

		rel, _ := filepath.Rel(bundleRoot, path)

		name := ""
		version := ""
		kind := ""

		filename := d.Name()
		switch filename {
		case "package.json":
			kind = "npm"
			if content, err := os.ReadFile(path); err == nil {
				var meta struct {
					Name    string `json:"name"`
					Version string `json:"version"`
				}
				json.Unmarshal(content, &meta)
				name = meta.Name
				version = meta.Version
			}
		case "go.mod":
			kind = "go"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				if len(lines) > 0 && strings.HasPrefix(lines[0], "module ") {
					name = strings.TrimSpace(strings.TrimPrefix(lines[0], "module "))
				}
			}
		case "Cargo.toml":
			kind = "cargo"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		case "pyproject.toml":
			kind = "pip"
			if content, err := os.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		}

		if strings.HasSuffix(filename, ".csproj") {
			kind = "nuget"
			name = strings.TrimSuffix(filename, ".csproj")
		}

		if kind != "" && name != "" {
			packages = append(packages, model.Package{
				Name:    name,
				Version: version,
				Path:    rel,
				Kind:    kind,
			})
		}
		return nil
	})
	return packages
}

// 🔖️FileHeaderId MUST complete the operation successfully.
func FileHeaderId(path string) string {
	kind := model.DeriveFileKind(filepath.Base(path))
	if kind == model.FileKindCode {
		absPath := filepath.Join(workspace.RootDir, path)
		if content, err := os.ReadFile(absPath); err == nil {
			text := string(content)
			if strings.HasPrefix(text, "#!") || strings.HasPrefix(text, "\xEF\xBB\xBF#!") {
				kind = model.FileKindScript
			}
		}
	}
	normalized := workspace.NormalizePath(path)
	dir := filepath.Dir(normalized)
	parentID := ""
	if dir != "." && dir != "" {
		parentID = model.ResolveParentIDFromPath(dir)
	}
	data := map[string]interface{}{"path": normalized, "kind": kind, "parentId": parentID}
	return model.GetArtifactID("file", data)
}

// 🔗️FileHeaderUri MUST return the repo URI for a file path.
// ◾FileHeaderUri returns the artifact URI for the given file path.
func FileHeaderUri(path string) string {
	data := map[string]interface{}{"path": path}
	return model.GetArtifactURI("file", data)
}

// #endregion 🛠️Technologies
