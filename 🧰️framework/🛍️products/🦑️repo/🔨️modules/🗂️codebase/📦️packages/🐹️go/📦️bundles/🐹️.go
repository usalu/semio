// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📦️bundles carries the codebase bundle builders.

// #endregion 🧲️Header

package codebase

import (
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📦️Bundles

// 🏗️BuildCodebaseBundles MUST assemble the codebase bundles from the available context data.
// 🧱️BuildCodebaseBundles constructs and returns the codebase bundles structure.
func BuildCodebaseBundles(ctx *CodebaseContext) []model.CodebaseBundle {
	var result []model.CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})
	contributorSets := make(map[string]map[string]struct{})
	ticketSets := make(map[string]map[string]struct{})
	breachCounts := make(map[string]int)

	for _, bundle := range ctx.Bundles {
		name := model.NormalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
		contributorSets[name] = make(map[string]struct{})
		ticketSets[name] = make(map[string]struct{})
	}

	if _, ok := folderSets["repo/repo"]; !ok {
		folderSets["repo/repo"] = make(map[string]struct{})
		contributorSets["repo/repo"] = make(map[string]struct{})
		ticketSets["repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		absPath := filepath.Join(workspace.RootDir, file)
		if content, err := workspace.ReadTextFile(absPath); err == nil {
			lineCounts[bundleName] += strings.Count(content, "\n") + 1
			sections := languages.ParseSections(content, file)
			sectionCounts[bundleName] += countSections(sections)
			lang := languages.GetLanguage(file)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				definitionCounts[bundleName] += len(defs)
			}
			headerSection := languages.FindSection(sections, "Header")
			if headerSection != nil {
				headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
				for _, line := range strings.Split(headerContent, "\n") {
					if name, email, ok := model.ParseContributorIdentity(line); ok {
						_ = name
						contributorSets[bundleName][email] = struct{}{}
					}
				}
			}
		}
	}

	for _, v := range ctx.Breachs {
		bundleName := ctx.GetBundleForFile(v.Scope)
		if bundleName != "" {
			breachCounts[bundleName]++
		}
	}

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		interactionFiles := ticket.GetInteractionFiles()
		if len(interactionFiles) > 0 {
			for _, entry := range interactionFiles {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		var contributors []string
		for c := range contributorSets[name] {
			contributors = append(contributors, c)
		}
		sort.Strings(contributors)

		var tickets []string
		for t := range ticketSets[name] {
			tickets = append(tickets, t)
		}
		sort.Strings(tickets)

		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if model.NormalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, model.CodebaseBundle{
			ID:           name,
			Folder:       bundleRoot,
			URI:          ctx.FileURI(bundleRoot),
			Contributors: contributors,
			Tickets:      tickets,
			Metrics: &model.BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
				Breachs:     breachCounts[name],
			},
		})
	}
	return result
}

// 📑️countSections holds the data fields for a countSections record.
func countSections(sections []model.Section) int {
	count := len(sections)
	for _, s := range sections {
		count += countSections(s.Children)
	}
	return count
}

// 🟥️BuildCodebaseBundlesForFiles MUST assemble the codebase bundles for files from the available context data.
// 🟨️BuildCodebaseBundlesForFiles constructs and returns the codebase bundles for files structure.
func BuildCodebaseBundlesForFiles(ctx *CodebaseContext, checkpoint string) []model.CodebaseBundle {
	var result []model.CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})

	for _, bundle := range ctx.Bundles {
		name := model.NormalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
	}

	if _, ok := folderSets["repo/repo"]; !ok {
		folderSets["repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := workspace.NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		content, err := workspace.ReadTextFileAtCheckpoint(checkpoint, file)
		if err != nil {
			continue
		}
		lineCounts[bundleName] += workspace.CountLines(content)
		sections := languages.ParseSections(content, file)
		sectionCounts[bundleName] += countSections(sections)
		lang := languages.GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			definitionCounts[bundleName] += len(defs)
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if model.NormalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, model.CodebaseBundle{
			ID:     name,
			Folder: bundleRoot,
			URI:    ctx.FileURI(bundleRoot),
			Metrics: &model.BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
			},
		})
	}
	return result
}

// #endregion 📦️Bundles
