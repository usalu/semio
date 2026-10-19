// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗃️sources carries the source projections of the repository entities.

// #endregion 🧲️Header

package graphql

import (
	sort "sort"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🗃️Sources

/** 📜️ The technology source object. */
func technologySource(technology *model.Technology, technologies []*model.Technology, rootDir string) interface{} {
	fields := body(technology)
	id := technologyID(technology)
	fields["uri"] = "repo://technology/" + id
	fields["id"] = id
	fields["kind"] = string(technology.Kind)
	rows := make([]interface{}, 0, len(technology.Bundles))
	for index := range technology.Bundles {
		rows = append(rows, bundleSource(&technology.Bundles[index], technologies, rootDir))
	}
	fields["bundles"] = rows
	return tagged(fields, "Technology")
}

/** 🔵️ The bundle source object. */
func bundleSource(bundle *model.Bundle, technologies []*model.Technology, rootDir string) interface{} {
	fields := body(bundle)
	fields["id"] = bundleID(bundle, technologies)
	fields["uri"] = fileURI(rootDir, bundle.Root)
	fields["kind"] = string(bundle.Kind)
	fields["tags"] = stringArray(bundle.Tags)
	packages := make([]interface{}, 0, len(bundle.Packages))
	for index := range bundle.Packages {
		packages = append(packages, tagged(body(&bundle.Packages[index]), "Package"))
	}
	fields["packages"] = packages
	return tagged(fields, "Bundle")
}

/** 🩷️ The folder source object. */
func folderSource(folder *model.Folder) interface{} {
	fields := body(folder)
	fields["kind"] = string(folder.Kind)
	return tagged(fields, "Folder")
}

/** 📄️ The file source object. */
func fileSource(file *model.File) interface{} {
	return tagged(body(file), "File")
}

/** 📐️ The line range of a start/end pair. */
func rangeSource(start int, end int) interface{} {
	return tagged(body(&model.Range{Start: start, End: end}), "Range")
}

/** 💗️ The section source object, recursing through children and definitions. */
func sectionSource(section *model.Section) interface{} {
	fields := body(section)
	fields["id"] = sectionID(section)
	fields["range"] = rangeSource(section.StartLine, section.EndLine)
	children := make([]interface{}, 0, len(section.Children))
	for index := range section.Children {
		children = append(children, sectionSource(&section.Children[index]))
	}
	fields["children"] = children
	definitions := make([]interface{}, 0, len(section.Definitions))
	for index := range section.Definitions {
		definitions = append(definitions, definitionSource(&section.Definitions[index]))
	}
	fields["definitions"] = definitions
	return tagged(fields, "Section")
}

/** 💕️ The definition source object. */
func definitionSource(definition *model.Definition) interface{} {
	fields := body(definition)
	fields["id"] = definitionID(definition)
	fields["kind"] = string(definition.Kind)
	fields["range"] = rangeSource(definition.StartLine, definition.EndLine)
	return tagged(fields, "Definition")
}

/** 📜️ The declared metadata of one statute, or the reference implementation's unknown default. */
func statuteMetaOf(kind model.Statute, statutes []*model.StatuteMeta) model.StatuteMeta {
	for _, candidate := range statutes {
		if candidate != nil && candidate.Kind == kind {
			return *candidate
		}
	}
	return model.StatuteMeta{Kind: kind, Priority: model.BreachPriorityLow, Reason: "Unknown breach", Solution: "Fix the breach"}
}

/** 📜️ The statute source object. */
func statuteSource(meta *model.StatuteMeta) interface{} {
	fields := body(meta)
	fields["id"] = model.EmojiText(entityEmoji("statute")) + statutePathToIDValue(string(meta.Kind))
	fields["priority"] = string(meta.Priority)
	return tagged(fields, "Statute")
}

/** 🔶️ The breach source object, with the statute metadata overlaid by the breach's own overrides. */
func breachSource(breach *model.Breach, statutes []*model.StatuteMeta) interface{} {
	meta := statuteMetaOf(breach.Kind, statutes)
	if breach.LintPriority != "" {
		meta.Priority = breach.LintPriority
	}
	if breach.LintAutofixable != nil {
		meta.Autofixable = *breach.LintAutofixable
	}
	if breach.Reason != "" {
		meta.Reason = breach.Reason
	}
	if breach.Solution != "" {
		meta.Solution = breach.Solution
	}
	fields := body(breach)
	fields["id"] = model.EmojiText(entityEmoji("breach")) + breach.ID
	fields["kindId"] = string(breach.Kind)
	fields["kind"] = statuteSource(&meta)
	fields["priority"] = string(meta.Priority)
	fields["autofixable"] = meta.Autofixable
	fields["line"] = breach.Line
	fields["column"] = breach.Column
	fields["excerpt"] = optional(breach.Excerpt)
	return tagged(fields, "Breach")
}

/** 🟣️ The territory source object. */
func territorySource(territory *model.Territory, statutes []*model.StatuteMeta) interface{} {
	fields := body(territory)
	groups := make([]interface{}, 0, len(territory.Groups))
	for index := range territory.Groups {
		groups = append(groups, territorySource(&territory.Groups[index], statutes))
	}
	fields["groups"] = groups
	kinds := make([]interface{}, 0, len(territory.Kinds))
	for _, kind := range territory.Kinds {
		meta := statuteMetaOf(kind, statutes)
		kinds = append(kinds, statuteSource(&meta))
	}
	fields["kinds"] = kinds
	return tagged(fields, "Territory")
}

/** 👮️ The policy source object. */
func policySource(policy *model.Policy, statutes []*model.StatuteMeta) interface{} {
	fields := body(policy)
	fields["scopes"] = stringArray(policy.Scopes)
	groups := make([]interface{}, 0, len(policy.Groups))
	for index := range policy.Groups {
		groups = append(groups, territorySource(&policy.Groups[index], statutes))
	}
	fields["groups"] = groups
	declared := make([]interface{}, 0, len(policy.Statutes))
	for _, meta := range policy.Statutes {
		if meta == nil {
			continue
		}
		declared = append(declared, statuteSource(meta))
	}
	fields["statutes"] = declared
	return tagged(fields, "Policy")
}

/** 💬️ The interaction source object. */
func interactionSource(interaction *model.Interaction) map[string]interface{} {
	fields := body(interaction)
	fields["prompt"] = interaction.Prompt
	fields["summary"] = interaction.Summary
	fields["llm"] = interaction.LLM
	fields["effort"] = interaction.Effort
	fields["__typename"] = "Interaction"
	return fields
}

/** 🎁️ The flattened interaction-resource source object. */
func interactionResourceSource(resource *model.InteractionResource) interface{} {
	fields := interactionSource(&resource.Interaction)
	fields["goalId"] = resource.GoalID
	fields["ticketId"] = resource.TicketID
	fields["sourceKind"] = resource.SourceKind
	fields["sourceId"] = resource.SourceID
	return tagged(fields, "InteractionResource")
}

/** 🎫️ The ticket source object, with every derived field the reference resolvers compute. */
func ticketSource(ticket *model.Ticket, rootDir string) interface{} {
	fields := ticketBody(ticket)
	fields["id"] = model.EmojiText(entityEmoji("ticket")) + workspace.Flat(ticket.Slug)
	fields["year"] = ticket.Year
	fields["month"] = ticket.Month
	fields["day"] = ticket.Day
	fields["slug"] = ticket.Slug
	path := ticket.FolderPath
	if path == "" {
		path = ticket.JsonPath
	}
	fields["path"] = workspace.NormalizePath(path)
	fields["uri"] = fileURI(rootDir, ticket.FolderPath)
	fields["title"] = ticket.Title
	fields["emoji"] = optional(ticket.Emoji)
	prompt := ticket.Description
	if prompt == "" && len(ticket.Interactions) > 0 {
		prompt = ticket.Interactions[0].Prompt
	}
	fields["prompt"] = prompt
	fields["summary"] = nil
	fields["status"] = string(ticket.Status)
	fields["llm"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.LLM }, func(value *model.TicketAgent) string { return value.LLM }))
	fields["effort"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.Effort }, func(value *model.TicketAgent) string { return value.Effort }))
	fields["client"] = optional(ticketLatest(ticket, func(value *model.Interaction) string { return value.Client }, func(value *model.TicketAgent) string { return value.Client }))
	fields["checkpoint"] = optional(ticketCheckpoint(ticket))
	fields["goal"] = optional(ticket.Goal)
	fields["parent"] = optional(ticket.Parent)
	interactions := make([]interface{}, 0, len(ticket.Interactions))
	for index := range ticket.Interactions {
		interactions = append(interactions, interactionSource(&ticket.Interactions[index]))
	}
	fields["interactions"] = interactions
	fields["dates"] = tagged(map[string]interface{}{"started": ticketStarted(ticket), "finished": ticketFinished(ticket)}, "TicketDate")
	return tagged(fields, "Ticket")
}

/** 🎯️ The goal source object. */
func goalSource(goal *model.Goal) interface{} {
	fields := goalBody(goal)
	fields["id"] = goal.ID
	fields["dueDate"] = optional(goal.Dates.Due)
	fields["createdAt"] = nil
	fields["status"] = goal.Status
	management := model.GoalManagementData{}
	if goal.Management != nil {
		management = *goal.Management
	}
	fields["milestone"] = parseMilestoneNumber(management.Milestone)
	fields["issue"] = optional(management.Issue)
	fields["parent"] = optional(goal.Parent)
	fields["interactions"] = []interface{}{}
	return tagged(fields, "Goal")
}

/** 📝️ The draft source object. */
func draftSource(draft *model.Draft) interface{} {
	return tagged(body(draft), "Draft")
}

/** ✅️ The todo source object. */
func todoSource(todo *model.Todo) interface{} {
	fields := body(todo)
	fields["id"] = model.EmojiText(entityEmoji("todo")) + workspace.Flat(todo.ID)
	fields["description"] = optional(todo.Description)
	if todo.Location == nil {
		fields["location"] = nil
	} else {
		fields["location"] = tagged(body(todo.Location), "Location")
	}
	return tagged(fields, "Todo")
}

/** ✔️ The checkpoint source object. */
func checkpointSource(checkpoint *model.Checkpoint) interface{} {
	return tagged(body(checkpoint), "Checkpoint")
}

/** 🧑️‍💻️ The contributor source object. */
func contributorSource(contributor *model.Contributor) interface{} {
	fields := body(contributor)
	fields["id"] = model.EmojiText(entityEmoji("contributor")) + workspace.Flat(contributor.Alias)
	names := make([]string, 0, len(contributor.Links))
	for name := range contributor.Links {
		names = append(names, name)
	}
	sort.Strings(names)
	links := make([]interface{}, 0, len(names))
	for _, name := range names {
		links = append(links, tagged(body(&model.ContributorLink{Name: name, URL: contributor.Links[name]}), "ContributorLink"))
	}
	fields["links"] = links
	fields["emoji"] = optional(contributor.Emoji)
	fields["fingerprint"] = optional(contributor.Fingerprint)
	fields["names"] = stringArray(contributor.Names)
	fields["emails"] = stringArray(contributor.Emails)
	fields["fingerprints"] = stringArray(contributor.Fingerprints)
	fields["githubs"] = stringArray(contributor.Githubs)
	fields["aliases"] = stringArray(contributor.Aliases)
	fields["icons"] = nil
	return tagged(fields, "Contributor")
}

/** 🔬️ The analyze-result source object. */
func analyzeResultSource(result *model.AnalyzeResult, statutes []*model.StatuteMeta) interface{} {
	breachs := make([]interface{}, 0)
	if result != nil {
		for _, breach := range result.Breachs {
			if breach == nil {
				continue
			}
			breachs = append(breachs, breachSource(breach, statutes))
		}
	}
	metrics := model.AnalyzeMetrics{}
	if result != nil && result.Metrics != nil {
		metrics = *result.Metrics
	}
	priority := model.PriorityCount{}
	if metrics.ByPriority != nil {
		priority = *metrics.ByPriority
	}
	metricFields := map[string]interface{}{
		"total":       metrics.Total,
		"autofixable": metrics.Autofixable,
		"byPriority":  tagged(body(&priority), "PriorityCount"),
	}
	return tagged(map[string]interface{}{"breachs": breachs, "metrics": tagged(metricFields, "AnalyzeMetrics")}, "AnalyzeResult")
}

/** 🩹️ The fix-result source object. */
func fixResultSource(result *model.FixResult, statutes []*model.StatuteMeta) interface{} {
	breachs := make([]interface{}, 0)
	fixed, remaining := 0, 0
	if result != nil {
		fixed, remaining = result.Fixed, result.Remaining
		for _, breach := range result.Breachs {
			if breach != nil {
				breachs = append(breachs, breachSource(breach, statutes))
			}
		}
	}
	return tagged(map[string]interface{}{"fixed": fixed, "remaining": remaining, "breachs": breachs}, "FixResult")
}

// #endregion 🗃️Sources
