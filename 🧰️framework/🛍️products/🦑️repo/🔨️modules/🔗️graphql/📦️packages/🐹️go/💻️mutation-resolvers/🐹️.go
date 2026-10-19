// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 💻️mutation_resolvers carries the mutation resolvers.

// #endregion 🧲️Header

package graphql

import (
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 💻️MutationResolvers

/** ⚡️ Runs one named resolver. */
func (run *execution) dispatch(key string, source interface{}, args map[string]interface{}) (interface{}, *ExecutionError) {
	text := func(name string) string { return argText(args, name) }
	optionalText := func(name string) *string { return argOptionalText(args, name) }
	sourceText := func(name string) string {
		fields, ok := source.(map[string]interface{})
		if !ok {
			return ""
		}
		value, _ := fields[name].(string)
		return value
	}
	input := func() interface{} { return args["input"] }
	switch key {
	case "empty-list":
		return []interface{}{}, nil
	case "technologies":
		return run.technologyRows(args), nil
	case "bundles":
		return run.bundles(args), nil
	case "folders":
		rows := make([]interface{}, 0)
		for _, folder := range run.repo.GetFolders() {
			rows = append(rows, folderSource(folder))
		}
		return rows, nil
	case "files":
		rows := make([]interface{}, 0)
		for _, file := range run.repo.GetFiles() {
			rows = append(rows, fileSource(file))
		}
		return rows, nil
	case "sections":
		rows := make([]interface{}, 0)
		for _, section := range run.repo.GetSections() {
			rows = append(rows, sectionSource(section))
		}
		return rows, nil
	case "definitions":
		rows := make([]interface{}, 0)
		for _, definition := range run.repo.GetDefinitions() {
			rows = append(rows, definitionSource(definition))
		}
		return rows, nil
	case "contributors":
		filter := run.filterOf(args)
		contributors, err := run.repo.GetContributors()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(contributors))
		for _, contributor := range contributors {
			if !matchesFilterInput(contributor.Github, filter) {
				continue
			}
			rows = append(rows, contributorSource(contributor))
		}
		return rows, nil
	case "goals":
		goals, err := run.repo.GetGoals()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(goals))
		for _, goal := range goals {
			rows = append(rows, goalSource(goal))
		}
		return rows, nil
	case "tickets":
		return run.tickets(args)
	case "policies":
		filter := run.filterOf(args)
		rows := make([]interface{}, 0)
		for _, policy := range run.repo.GetPolicies() {
			if !matchesFilterInput(policy.Name, filter) {
				continue
			}
			rows = append(rows, policySource(policy, run.statutes))
		}
		return rows, nil
	case "statutes":
		rows := make([]interface{}, 0, len(run.statutes))
		for _, meta := range run.statutes {
			rows = append(rows, statuteSource(meta))
		}
		return rows, nil
	case "breachs":
		result, err := run.repo.Analyze(optionalText("scope"))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0)
		if result != nil {
			for _, breach := range result.Breachs {
				rows = append(rows, breachSource(breach, run.statutes))
			}
		}
		return rows, nil
	case "checkpoints":
		checkpoints, err := run.repo.GetCheckpoints(argInt(args, "limit"))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(checkpoints))
		for _, checkpoint := range checkpoints {
			rows = append(rows, checkpointSource(checkpoint))
		}
		return rows, nil
	case "folder.children":
		id := sourceText("id")
		rows := make([]interface{}, 0)
		for _, folder := range run.repo.GetFolders() {
			if folder.ParentID != nil && *folder.ParentID == id {
				rows = append(rows, folderSource(folder))
			}
		}
		return rows, nil
	case "folder.files":
		id := sourceText("id")
		rows := make([]interface{}, 0)
		for _, file := range run.repo.GetFiles() {
			if file.FolderID != nil && *file.FolderID == id {
				rows = append(rows, fileSource(file))
			}
		}
		return rows, nil
	case "file.sections":
		path := sourceText("path")
		rows := make([]interface{}, 0)
		for _, section := range run.repo.GetSections() {
			if section.FilePath == path {
				rows = append(rows, sectionSource(section))
			}
		}
		return rows, nil
	case "file.definitions":
		path := sourceText("path")
		rows := make([]interface{}, 0)
		for _, definition := range run.repo.GetDefinitions() {
			if definition.FilePath == path {
				rows = append(rows, definitionSource(definition))
			}
		}
		return rows, nil
	case "item.file":
		path := sourceText("filePath")
		if path == "" {
			return nil, nil
		}
		return run.fileByPath(path), nil
	case "definition.section":
		path := sourceText("sectionPath")
		if path == "" {
			return nil, nil
		}
		return sectionSource(&model.Section{Name: path, Path: path, FilePath: sourceText("filePath")}), nil
	case "section.children":
		return sectionChildren(source), nil
	case "statute.policy":
		return policySource(&model.Policy{
			ID: "/policies/lint-scripts", Name: "Lint scripts", Scopes: []string{"**/*"}, Groups: []model.Territory{}, Statutes: []*model.StatuteMeta{},
		}, run.statutes), nil
	case "ticket.author":
		return run.ticketAuthorOf(source)
	case "contributor.contributions":
		return run.contributions(source)
	case "query.node":
		return run.node(args)
	case "query.repo":
		return run.repoSource(), nil
	case "query.todos":
		todos, err := run.repo.GetTodos(run.filterOf(args))
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(todos))
		for _, todo := range todos {
			rows = append(rows, todoSource(todo))
		}
		return rows, nil
	case "query.interactions":
		interactions, err := run.repo.GetInteractions()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(interactions))
		for index := range interactions {
			rows = append(rows, interactionResourceSource(&interactions[index]))
		}
		return rows, nil
	case "query.drafts":
		drafts, err := run.repo.GetDrafts()
		if err != nil {
			return nil, lift(err)
		}
		rows := make([]interface{}, 0, len(drafts))
		for _, draft := range drafts {
			rows = append(rows, draftSource(draft))
		}
		return rows, nil
	case "query.bundle":
		return run.bundleByName(text("name")), nil
	case "query.folder":
		return run.folderByPath(text("path")), nil
	case "query.file":
		return run.fileByPath(text("path")), nil
	case "query.section":
		path := text("path")
		segments, ok := args["sectionPath"].([]interface{})
		if !ok {
			return nil, nil
		}
		parts := make([]string, 0, len(segments))
		for _, segment := range segments {
			if value, ok := segment.(string); ok {
				parts = append(parts, value)
			}
		}
		joined := strings.Join(parts, "#")
		for _, section := range run.repo.GetSections() {
			if section.FilePath == path && section.Path == joined {
				return sectionSource(section), nil
			}
		}
		return sectionSource(&model.Section{Name: joined}), nil
	case "query.definition":
		path, name := text("path"), text("name")
		for _, definition := range run.repo.GetDefinitions() {
			if definition.FilePath == path && definition.Name == name {
				return definitionSource(definition), nil
			}
		}
		return definitionSource(&model.Definition{Name: name, Kind: model.DefinitionKindImplementation}), nil
	case "query.contributor":
		return run.contributorByID(text("id"))
	case "query.ticket":
		slug := text("slug")
		tickets, err := run.repo.GetTickets(argInt(args, "year"), argInt(args, "month"), argInt(args, "day"), nil)
		if err != nil {
			return nil, lift(err)
		}
		for _, ticket := range tickets {
			if ticket.Slug == slug {
				return ticketSource(ticket, run.rootDir), nil
			}
		}
		return nil, nil
	case "query.policy":
		return run.policyByID(text("id")), nil
	case "query.statute":
		meta := statuteMetaOf(model.Statute(text("id")), run.statutes)
		return statuteSource(&meta), nil
	case "query.analyze":
		result, err := run.repo.Analyze(optionalText("scope"))
		if err != nil {
			return nil, lift(err)
		}
		return analyzeResultSource(result, run.statutes), nil
	case "mutation.fix":
		result, err := run.repo.Fix(optionalText("scope"))
		if err != nil {
			return nil, lift(err)
		}
		return fixResultSource(result, run.statutes), nil
	case "mutation.syncManagement":
		done, err := run.repo.SyncManagement()
		if err != nil {
			return nil, lift(err)
		}
		return done, nil
	case "mutation.goalCreate":
		decoded := model.GoalCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalChange":
		decoded := model.GoalChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		decoded.ID = text("id")
		goal, err := run.repo.GoalChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalClose":
		decoded := model.GoalCloseInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalClose(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.goalReopen":
		decoded := model.GoalReopenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		goal, err := run.repo.GoalReopen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return goalSource(goal), nil
	case "mutation.draftCreate":
		decoded := model.DraftCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		draft, err := run.repo.DraftCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return draftSource(draft), nil
	case "mutation.draftDelete":
		deleted, err := run.repo.DraftDelete(text("id"))
		if err != nil {
			return nil, lift(err)
		}
		return deleted, nil
	case "mutation.todoCreate":
		decoded := model.TodoCreateInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		todo, err := run.repo.TodoCreate(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return todoSource(todo), nil
	case "mutation.todoChange":
		decoded := model.TodoChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		todo, err := run.repo.TodoChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return todoSource(todo), nil
	case "mutation.todoDelete":
		deleted, err := run.repo.TodoDelete(text("id"))
		if err != nil {
			return nil, lift(err)
		}
		return deleted, nil
	case "mutation.ticketOpen":
		decoded := model.TicketOpenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketOpen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketClose":
		decoded := model.TicketCloseInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketClose(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketReopen":
		decoded := model.TicketReopenInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketReopen(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.ticketChange":
		decoded := model.TicketChangeInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		ticket, err := run.repo.TicketChange(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return ticketSource(ticket, run.rootDir), nil
	case "mutation.contributorAdd":
		decoded := model.ContributorAddInput{}
		if failure := decodeInput(input(), &decoded); failure != nil {
			return nil, failure
		}
		contributor, err := run.repo.ContributorAdd(decoded)
		if err != nil {
			return nil, lift(err)
		}
		return contributorSource(contributor), nil
	case "mutation.contributorRemove":
		if err := run.repo.ContributorRemove(text("github")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.folderCreate":
		folder, err := run.repo.FolderCreate(text("path"))
		if err != nil {
			return nil, lift(err)
		}
		return folderSource(folder), nil
	case "mutation.folderMove":
		folder, err := run.repo.FolderMove(text("src"), text("dst"))
		if err != nil {
			return nil, lift(err)
		}
		return folderSource(folder), nil
	case "mutation.folderDelete":
		if err := run.repo.FolderDelete(text("path")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.fileCreate":
		file, err := run.repo.FileCreate(text("path"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.fileMove":
		file, err := run.repo.FileMove(text("src"), text("dst"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.fileDelete":
		if err := run.repo.FileDelete(text("path")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.sectionCreate":
		section, err := run.repo.SectionCreate(text("file"), text("name"), optionalText("parent"))
		if err != nil {
			return nil, lift(err)
		}
		return sectionSource(section), nil
	case "mutation.sectionMove":
		section, err := run.repo.SectionMove(text("file"), text("oldName"), text("newName"))
		if err != nil {
			return nil, lift(err)
		}
		return sectionSource(section), nil
	case "mutation.sectionDelete":
		if err := run.repo.SectionDelete(text("file"), text("name")); err != nil {
			return nil, lift(err)
		}
		return true, nil
	case "mutation.integrate":
		file, err := run.repo.Integrate(optionalText("source"), optionalText("targetSection"), optionalText("targetFile"), optionalText("targetParent"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	case "mutation.extract":
		file, err := run.repo.Extract(optionalText("sourceFile"), optionalText("sourceSection"), optionalText("targetFile"))
		if err != nil {
			return nil, lift(err)
		}
		return fileSource(file), nil
	default:
		return nil, NewExecutionError("no resolver registered for " + key)
	}
}

// #endregion 💻️MutationResolvers
