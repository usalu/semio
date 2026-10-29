// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Technology and bundle loading and streaming.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	workspace "github.com/usalu/semio/repo/workspace"
	yaml "github.com/usalu/semio/repo/yaml"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"
)

var (
	technologyCache       []Technology
	technologyCacheLoaded bool
	technologyCacheMutex  sync.Mutex
)

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
func LoadTechnologies() []Technology {
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
	readmePath := filepath.Join(rootDir, name, "README.md")
	content, err := ReadTextFile(readmePath)
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
func loadTechnologiesInternal() []Technology {
	var technologies []Technology
	technologiesDir := rootDir
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
		technologyKind := DeriveTechnologyKind(rawName)
		readmePath := filepath.Join(rootDir, name, "README.md")
		if content, err := ReadTextFile(readmePath); err == nil && strings.HasPrefix(content, "---") {
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
							technologyKind = TechnologyKindUser
						case "infrastructure":
							technologyKind = TechnologyKindInfrastructure
						case "research":
							technologyKind = TechnologyKindResearch
						}
					}
				}
			}
		}

		technology := Technology{
			Name:    technologyName,
			Root:    name,
			Kind:    technologyKind,
			Bundles: []Bundle{},
		}

		// Read technology emoji from AGENTS.md frontmatter
		agentsPath := filepath.Join(rootDir, name, "AGENTS.md")
		if content, err := ReadTextFile(agentsPath); err == nil && strings.HasPrefix(content, "---") {
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
					bundlePath := filepath.Join(name, bunName, siteName)
					bundle := Bundle{
						Name:           fullBundleName,
						Root:           bundlePath,
						TechnologyName: technologyName,
						Kind:           BundleKindSite,
					}
					// Read site bundle emoji from AGENTS.md frontmatter
					siteAgentsPath := filepath.Join(rootDir, bundlePath, "AGENTS.md")
					if content, err := ReadTextFile(siteAgentsPath); err == nil && strings.HasPrefix(content, "---") {
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
					bundle.Packages = loadPackages(filepath.Join(rootDir, bundlePath))
					configPath := filepath.Join(technologyPath, bunName, siteName, "project.json")
					if !FileExists(configPath) {
						configPath = filepath.Join(technologyPath, bunName, siteName, "package.json")
					}
					if FileExists(configPath) {
						content, err := ReadTextFile(configPath)
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

			bundlePath := filepath.Join(name, bunName)
			kind := DeriveBundleKind(fullBundleName, bundlePath)

			bundle := Bundle{
				Name:           fullBundleName,
				Root:           bundlePath,
				TechnologyName: technologyName,
				Kind:           kind,
			}

			// Read bundle emoji from AGENTS.md frontmatter
			bundleAgentsPath := filepath.Join(rootDir, bundlePath, "AGENTS.md")
			if content, err := ReadTextFile(bundleAgentsPath); err == nil && strings.HasPrefix(content, "---") {
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

			bundle.Packages = loadPackages(filepath.Join(rootDir, bundlePath))

			configPath := filepath.Join(technologyPath, bunName, "project.json")
			if !FileExists(configPath) {
				configPath = filepath.Join(technologyPath, bunName, "package.json")
			}
			if FileExists(configPath) {
				content, err := ReadTextFile(configPath)
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

// ◼LoadCheckpoints MUST return all matching checkpoints from the data source.
// 🔢️LoadCheckpoints loads and returns checkpoints from the data source.
func LoadCheckpoints(limit *int) []Checkpoint {
	args := []string{"log", "--pretty=format:%H|%aN|%ad|%s", "--date=iso-strict"}
	if limit != nil {
		args = append(args, fmt.Sprintf("-n%d", *limit))
	}

	cmd := workspace.ToolCommand("git", args...)
	cmd.Dir = rootDir
	out, err := cmd.Output()
	if err != nil {
		return []Checkpoint{}
	}
	lines := strings.Split(string(out), "\n")
	var checkpoints []Checkpoint
	for _, line := range lines {
		parts := strings.Split(line, "|")
		if len(parts) >= 4 {
			sha := parts[0]

			dateStr := parts[2]
			title := strings.Join(parts[3:], "|")
			date, _ := time.Parse(time.RFC3339, dateStr)

			checkpoints = append(checkpoints, Checkpoint{
				ID:    sha,
				SHA:   sha,
				Title: title,
				Date:  date,
			})
		}
	}
	return checkpoints
}

// 📦️LoadBundles MUST return all matching bundles from the data source.
// 🗄️LoadBundles loads and returns bundles from the data source.
func LoadBundles() []Bundle {
	var bundles []Bundle
	technologies := LoadTechnologies()
	for _, p := range technologies {
		bundles = append(bundles, p.Bundles...)
	}
	return bundles
}

// 📨️GetTechnologies MUST retrieve the requested value or return an error.
// 🔺️GetTechnologies retrieves and returns the technologies.
func GetTechnologies() []Bundle {
	return LoadBundles()
}

// 🔵️StreamBundles MUST invoke the callback for each matching bundles entry.
// 🎯️StreamBundles streams bundles entries through the callback.
func StreamBundles(ctx context.Context, out chan<- Bundle, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	bundles := LoadBundles()
	for _, b := range bundles {
		if !matchesFilter(b.Name, options) {
			continue
		}
		queryMatches, queryErr := matchesQuery(ctx, b.Name+" "+string(b.Kind), options)
		if queryErr != nil {
			return queryErr
		}
		if !queryMatches {
			continue
		}

		if !shouldIncludeBundleKind(b.Kind, options) {
			continue
		}

		if !bundleMatchesKinds(b, options) {
			continue
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- b
		}
	}
	return nil
}

// 🔴️bundleMatchesKinds holds the data fields for a bundleMatchesKinds record.
func bundleMatchesKinds(b Bundle, opts StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}

	bundleRoot := filepath.Join(rootDir, b.Root)
	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() {
			return nil
		}
		if isRepoExcludedPath(path) || isGitIgnored(path) {
			return nil
		}

		kind := DeriveFileKind(filepath.Base(path))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}

		if hasIncluded && (len(opts.ExcludeKinds) == 0 || hasExcluded) {
			return filepath.SkipAll
		}
		return nil
	})

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

// 🟠️loadPackages holds the data fields for a loadPackages record.
func loadPackages(bundleRoot string) []Package {
	var packages []Package
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
			packages = append(packages, Package{
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

// 🟡️StreamTechnologies MUST invoke the callback for each matching technologies entry.
// 🔻️StreamTechnologies streams technologies entries through the callback.
func StreamTechnologies(ctx context.Context, out chan<- Technology, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	technologies := LoadTechnologies()
	for _, p := range technologies {
		if !matchesFilter(p.Name, options) {
			continue
		}
		queryMatches, queryErr := matchesQuery(ctx, p.Name+" "+string(p.Kind), options)
		if queryErr != nil {
			return queryErr
		}
		if !queryMatches {
			continue
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- p
		}
	}
	return nil
}

// 🟢️runTechnologyList holds the data fields for a runTechnologyList record.
func runTechnologyList(factory EngineFactory, config Config, cmd *command.Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology list"}

		projChan := make(chan Technology)
		go func() {
			StreamTechnologies(context.Background(), projChan, opts)
		}()

		for p := range projChan {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			stream <- Event{Kind: KindResult, Command: "technology list", Data: data}
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

// 🌳️runTechnologyTree holds the data fields for a runTechnologyTree record.
func runTechnologyTree(factory EngineFactory, config Config, cmd *command.Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology tree"}

		projChan := make(chan Technology)
		go func() {
			StreamTechnologies(context.Background(), projChan, opts)
		}()

		var technologies []Technology
		for p := range projChan {
			technologies = append(technologies, p)
		}
		var events []Event
		for _, p := range technologies {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			events = append(events, Event{Kind: KindResult, Command: "technology tree", Data: data})
		}
		sort.Slice(events, func(i, j int) bool {
			return formatMarkdownResult(events[i].Command, events[i].Data) < formatMarkdownResult(events[j].Command, events[j].Data)
		})
		for _, ev := range events {
			stream <- ev
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}
