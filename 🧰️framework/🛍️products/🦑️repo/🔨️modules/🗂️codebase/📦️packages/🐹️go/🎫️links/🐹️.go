// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️links carries the codebase ticket, policy and breach builders.

// #endregion 🧲️Header

package codebase

import (
	fmt "fmt"
	filepath "path/filepath"
	strings "strings"
	time "time"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎫️Links

// 🗃️BuildCodebaseTickets MUST assemble the codebase tickets from the available context data.
// 🗺️BuildCodebaseTickets constructs and returns the codebase tickets structure.
func BuildCodebaseTickets(ctx *CodebaseContext) []model.CodebaseTicket {
	var result []model.CodebaseTicket

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		ticketPath := ticket.FolderPath
		if ticketPath == "" {
			ticketPath = ".🧬semio/🦑️repo/🎫️tickets/" + model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
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
		var bundleContribs []model.TicketBundleContribInfo
		for bundleName, fileCount := range bundleFiles {
			bundleContribs = append(bundleContribs, model.TicketBundleContribInfo{
				ID: bundleName,
				Metrics: &model.CountMetrics{
					Added: fileCount,
				},
			})
		}

		llm := ticket.GetLLM()

		var finishedStr string
		if f := ticket.GetDateFinished(); f != nil {
			finishedStr = f.Format(time.RFC3339)
		}

		result = append(result, model.CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &model.TicketDateInfo{
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
func BuildCodebasePolicies(ctx *CodebaseContext) []model.CodebasePolicy {
	var result []model.CodebasePolicy
	breachsByPolicy := make(map[string][]model.Breach)

	for _, v := range ctx.Breachs {
		parts := strings.Split(string(v.Kind), "/")
		if len(parts) > 0 {
			policyID := parts[0]
			breachsByPolicy[policyID] = append(breachsByPolicy[policyID], v)
		}
	}

	for _, policy := range ctx.Policies {
		var breachs []model.PolicyBreachRef
		for _, v := range breachsByPolicy[policy.ID] {
			info := v.Kind.Info()
			breachs = append(breachs, model.PolicyBreachRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}
		result = append(result, model.CodebasePolicy{
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
func BuildCodebaseBreachs(ctx *CodebaseContext) []model.CodebaseBreach {
	var result []model.CodebaseBreach

	for i, v := range ctx.Breachs {
		filePath := extractFilePath(v.Scope)
		bundleName := ctx.GetBundleForFile(filePath)
		info := v.Kind.Info()

		breachID := fmt.Sprintf("%s#|%s|%s#%d", v.Kind, bundleName, filePath, i)

		var folders []model.BreachFolder
		if filePath != "" {
			folder := workspace.NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				folderID := folder
				if bundleName != "" {
					folderID = bundleName + "/" + folder
				}
				folders = append(folders, model.BreachFolder{
					ID:   folderID,
					Path: folder,
					URI:  ctx.FolderURI(folder),
				})
			}
		}

		var files []model.BreachFile
		if filePath != "" {
			fileID := filePath
			if bundleName != "" {
				fileID = bundleName + "/" + filepath.Base(filePath)
			}
			files = append(files, model.BreachFile{
				ID:   fileID,
				Path: filePath,
				URI:  ctx.FileURI(filePath),
				Range: &model.FileRange{
					Start: model.RangePosition{Line: v.Line, Column: v.Column},
					End:   model.RangePosition{Line: v.Line, Column: v.Column},
				},
			})
		}

		result = append(result, model.CodebaseBreach{
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

// #endregion 🎫️Links
