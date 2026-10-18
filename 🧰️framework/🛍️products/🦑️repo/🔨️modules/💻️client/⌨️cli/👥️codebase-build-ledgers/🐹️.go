// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Codebase contributor, ticket, policy, breach, and tree builders.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"strings"
	"time"
)

// 🤝️BuildCodebaseContributors MUST assemble the codebase contributors from the available context data.
// 🔻️BuildCodebaseContributors constructs and returns the codebase contributors structure.
func BuildCodebaseContributors(ctx *CodebaseContext) []CodebaseContributor {
	contributors, err := ListContributors()
	if err != nil {
		return nil
	}

	var result []CodebaseContributor
	for _, c := range contributors {
		links := c.Links
		if links == nil {
			links = map[string]string{}
		}
		if c.Github != "" {
			links["github"] = c.Github
		}
		emails := append([]string{}, c.Emails...)
		if c.Email != "" {
			emails = append(emails, c.Email)
		}
		avatarPath := GetContributorAvatarPath(c.Alias)
		avatarRoundPath := GetContributorAvatarRoundPath(c.Alias)

		var icons *ContributorIcons
		if FileExists(avatarPath) || FileExists(avatarRoundPath) {
			icons = &ContributorIcons{}
			if FileExists(avatarPath) {
				avatar := ctx.FileURI(GetRelativePath(avatarPath))
				icons.Avatar = &avatar
			}
			if FileExists(avatarRoundPath) {
				avatarRound := ctx.FileURI(GetRelativePath(avatarRoundPath))
				icons.AvatarRound = &avatarRound
			}
			if githubLink, ok := links["github"]; ok {
				github := githubLink + ".png"
				icons.Github = &github
			}
		}

		var contributions *ContributorContributionsInternal
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &ContributorContributionsInternal{}
			for _, b := range c.Contributions.Bundles {
				contributions.Bundles = append(contributions.Bundles, ContributorBundleContrib{ID: b})
			}
			for _, f := range c.Contributions.Folders {
				contributions.Folders = append(contributions.Folders, ContributorFolderContrib{ID: f})
			}
			for _, f := range c.Contributions.Files {
				contributions.Files = append(contributions.Files, ContributorFileContrib{ID: f})
			}
			for _, r := range c.Contributions.Regions {
				contributions.Sections = append(contributions.Sections, ContributorSectionContrib{ID: r})
			}
			for _, d := range c.Contributions.Definitions {
				contributions.Definitions = append(contributions.Definitions, ContributorDefinitionContrib{ID: d})
			}
		}

		linesTotal := 0
		if c.Contributions.Lines != nil {
			linesTotal = c.Contributions.Lines.Added + c.Contributions.Lines.Removed
		}

		result = append(result, CodebaseContributor{
			ID:            c.Alias,
			URI:           ctx.FileURI(".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias),
			Path:          ".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias + "/contributor.json",
			Name:          c.Name,
			Icons:         icons,
			Emails:        emails,
			Links:         links,
			Contributions: contributions,
			Metrics: &ContributorMetricsInternal{
				Checkpoints: len(c.Contributions.Checkpoints),
				Tickets:     len(c.Contributions.Tickets),
				Bundles:     len(c.Contributions.Bundles),
				Folders:     len(c.Contributions.Folders),
				Files:       len(c.Contributions.Files),
				Lines:       linesTotal,
				Sections:    len(c.Contributions.Regions),
				Definitions: len(c.Contributions.Definitions),
			},
		})
	}
	return result
}

// 🗃️BuildCodebaseTickets MUST assemble the codebase tickets from the available context data.
// 🗺️BuildCodebaseTickets constructs and returns the codebase tickets structure.
func BuildCodebaseTickets(ctx *CodebaseContext) []CodebaseTicket {
	var result []CodebaseTicket

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		ticketPath := ticket.FolderPath
		if ticketPath == "" {
			ticketPath = ".🧬semio/🦑️repo/🎫️tickets/" + FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		}

		bundleFiles := make(map[string]int)
		var paths []string
		for _, f := range ticket.GetInteractionFiles() {
			paths = append(paths, f.Path)
		}
		for _, path := range paths {
			bundleName := ctx.GetBundleForFile(path)
			if bundleName != "" {
				bundleFiles[bundleName]++
			}
		}
		var bundleContribs []TicketBundleContribInfo
		for bundleName, fileCount := range bundleFiles {
			bundleContribs = append(bundleContribs, TicketBundleContribInfo{
				ID: bundleName,
				Metrics: &CountMetrics{
					Added: fileCount,
				},
			})
		}

		llm := ticket.GetLLM()

		var finishedStr string
		if f := ticket.GetDateFinished(); f != nil {
			finishedStr = f.Format(time.RFC3339)
		}

		result = append(result, CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &TicketDateInfo{
				Created:  ticket.GetDateStarted().Format(time.RFC3339),
				Finished: finishedStr,
			},
			Checkpoint: ticket.GetCheckpoint(),
			Year:       fmt.Sprintf("%04d", ticket.Year),
			Month:      fmt.Sprintf("%02d", ticket.Month),
			Day:        fmt.Sprintf("%02d", ticket.Day),
			Slug:       ticket.Slug,
			Prompt:     ticket.GetPrompt(),
			LLM:        llm,
			Author:     ticket.GetAuthor(),
			Status:     ticket.GetStatus(),
			Bundles:    bundleContribs,
		})
	}
	return result
}

// 🔻️BuildCodebasePolicies MUST assemble the codebase policies from the available context data.
// ⬛️BuildCodebasePolicies constructs and returns the codebase policies structure.
func BuildCodebasePolicies(ctx *CodebaseContext) []CodebasePolicy {
	var result []CodebasePolicy
	breachsByPolicy := make(map[string][]Breach)

	for _, v := range ctx.Breachs {
		parts := strings.Split(string(v.Kind), "/")
		if len(parts) > 0 {
			policyID := parts[0]
			breachsByPolicy[policyID] = append(breachsByPolicy[policyID], v)
		}
	}

	for _, policy := range ctx.Policies {
		var breachs []PolicyBreachRef
		for _, v := range breachsByPolicy[policy.ID] {
			info := v.Kind.Info()
			breachs = append(breachs, PolicyBreachRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}
		result = append(result, CodebasePolicy{
			ID:      policy.ID,
			Name:    policy.Name,
			Scopes:  policy.Scopes,
			Breachs: breachs,
		})
	}
	return result
}

// ⬛️BuildCodebaseBreachs MUST assemble the codebase breachs from the available context data.
// ⬜️BuildCodebaseBreachs constructs and returns the codebase breachs structure.
func BuildCodebaseBreachs(ctx *CodebaseContext) []CodebaseBreach {
	var result []CodebaseBreach

	for i, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		bundleName := ctx.GetBundleForFile(filePath)
		info := v.Kind.Info()

		breachID := fmt.Sprintf("%s#|%s|%s#%d", v.Kind, bundleName, filePath, i)

		var folders []BreachFolder
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				folderID := folder
				if bundleName != "" {
					folderID = bundleName + "/" + folder
				}
				folders = append(folders, BreachFolder{
					ID:   folderID,
					Path: folder,
					URI:  ctx.FolderURI(folder),
				})
			}
		}

		var files []BreachFile
		if filePath != "" {
			fileID := filePath
			if bundleName != "" {
				fileID = bundleName + "/" + filepath.Base(filePath)
			}
			files = append(files, BreachFile{
				ID:   fileID,
				Path: filePath,
				URI:  ctx.FileURI(filePath),
				Range: &FileRange{
					Start: RangePosition{Line: v.Line, Column: v.Column},
					End:   RangePosition{Line: v.Line, Column: v.Column},
				},
			})
		}

		result = append(result, CodebaseBreach{
			ID:          breachID,
			Folders:     folders,
			Files:       files,
			Kind:        v.Kind,
			Priority:    info.Priority,
			Autofixable: info.Autofixable,
			Reason:      info.Reason,
			Solution:    info.Solution,
		})
	}
	return result
}

// 🌳️BuildCodebaseTree MUST assemble the codebase tree from the available context data.
// 🌳️BuildCodebaseTree constructs and returns the codebase tree structure.
func BuildCodebaseTree(ctx *CodebaseContext, bundles []CodebaseBundle, files []CodebaseFile, sections []CodebaseSection, definitions []CodebaseDefinition) map[string]*CbTreeNode {
	tree := make(map[string]*CbTreeNode)
	tree["compose"] = &CbTreeNode{Kind: CbTreeNodeRepo, Children: make(map[string]*CbTreeNode)}
	root := tree["compose"]

	for _, bundle := range bundles {
		root.Children[bundle.ID] = &CbTreeNode{Kind: CbTreeNodeBundle, Children: make(map[string]*CbTreeNode)}
	}

	folderNodes := make(map[string]*CbTreeNode)
	for _, file := range files {
		bundleName := ctx.GetBundleForFile(file.Path)
		var parent *CbTreeNode
		if bundleName != "" {
			parent = root.Children[bundleName]
		} else {
			parent = root
		}
		folder := NormalizePath(filepath.Dir(file.Path))
		if folder != "." {
			parts := strings.Split(folder, "/")
			for i, part := range parts {
				folderPath := strings.Join(parts[:i+1], "/")
				if _, ok := folderNodes[folderPath]; !ok {
					folderNode := &CbTreeNode{Kind: CbTreeNodeFolder, Children: make(map[string]*CbTreeNode)}
					if i == 0 {
						parent.Children[part] = folderNode
					} else {
						parentPath := strings.Join(parts[:i], "/")
						folderNodes[parentPath].Children[part] = folderNode
					}
					folderNodes[folderPath] = folderNode
				}
			}
			fileNode := &CbTreeNode{Kind: CbTreeNodeFile, Children: make(map[string]*CbTreeNode)}
			folderNodes[folder].Children[file.ID] = fileNode
		} else {
			fileNode := &CbTreeNode{Kind: CbTreeNodeFile, Children: make(map[string]*CbTreeNode)}
			parent.Children[file.ID] = fileNode
		}
	}

	return tree
}

// ⬜️BuildCodebase MUST assemble the codebase from the available context data.
// 🟥️BuildCodebase constructs and returns the codebase structure.
func BuildCodebase(ctx *CodebaseContext) *Codebase {
	bundles := BuildCodebaseBundles(ctx)
	folders := BuildCodebaseFolders(ctx)
	files := BuildCodebaseFiles(ctx)
	sections := BuildCodebaseSections(ctx)
	definitions := BuildCodebaseDefinitions(ctx)
	contributors := BuildCodebaseContributors(ctx)
	tickets := BuildCodebaseTickets(ctx)
	policies := BuildCodebasePolicies(ctx)
	breachs := BuildCodebaseBreachs(ctx)
	tree := BuildCodebaseTree(ctx, bundles, files, sections, definitions)

	return &Codebase{
		Bundles:      bundles,
		Folders:      folders,
		Files:        files,
		Sections:     sections,
		Definitions:  definitions,
		Contributors: contributors,
		Tickets:      tickets,
		Policies:     policies,
		Breachs:      breachs,
		Tree:         tree,
	}
}

// 📸️BuildCodebaseSnapshot MUST assemble the codebase snapshot from the available context data.
// 🟧️BuildCodebaseSnapshot constructs and returns the codebase snapshot structure.
func BuildCodebaseSnapshot(files []string, bundles []Bundle, checkpoint string) (*Codebase, error) {
	ctx := &CodebaseContext{RootDir: rootDir, RootURI: "file://" + NormalizePath(rootDir)}
	ctx.Bundles = bundles
	ctx.Files = files
	ctx.Policies = GetPolicies()
	codebase := &Codebase{}
	codebase.Bundles = BuildCodebaseBundlesForFiles(ctx, checkpoint)
	codebase.Folders = BuildCodebaseFoldersForFiles(ctx, checkpoint)
	codebase.Files = BuildCodebaseFilesForFiles(ctx, checkpoint)
	codebase.Sections = BuildCodebaseSectionsForFiles(ctx, checkpoint)
	codebase.Definitions = BuildCodebaseDefinitionsForFiles(ctx, checkpoint)
	codebase.Tree = BuildCodebaseTree(ctx, codebase.Bundles, codebase.Files, codebase.Sections, codebase.Definitions)
	return codebase, nil
}
