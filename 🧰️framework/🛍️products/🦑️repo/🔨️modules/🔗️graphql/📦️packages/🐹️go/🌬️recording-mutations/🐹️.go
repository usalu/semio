// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎞️recording_mutations carries the mutations of the recording repository context.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎞️Recording Mutations

func (recording *RecordingContext) GoalCreate(input model.GoalCreateInput) (*model.Goal, error) {
	goal := &model.Goal{
		Title: input.Title, Description: input.Description, Prompt: input.Prompt, Status: "open",
		DueDate: input.DueDate, Dates: model.GoalDates{Due: input.DueDate}, Client: input.Client,
		LLM: input.LLM, Effort: input.Effort, Parent: input.Parent,
		Management: &model.GoalManagementData{Milestone: input.Milestone}, ID: deriveRecordID(input.Title),
	}
	recording.records.Goals = append(recording.records.Goals, goal)
	recording.record("goal.create", input)
	return goal, nil
}

/** 🎯️ The recorded goal one identifier names. */
func (recording *RecordingContext) goal(id string) (*model.Goal, error) {
	for _, goal := range recording.records.Goals {
		if goal.ID == id {
			return goal, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("goal %s not found", id)}
}

func (recording *RecordingContext) GoalChange(input model.GoalChangeInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		goal.Title = *input.Title
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.DueDate = *input.DueDate
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		goal.LLM = *input.LLM
	}
	if input.Effort != nil {
		goal.Effort = *input.Effort
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	recording.record("goal.change", input)
	return goal, nil
}

func (recording *RecordingContext) GoalClose(input model.GoalCloseInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	goal.Status = "closed"
	goal.Summary = input.Summary
	recording.record("goal.close", input)
	return goal, nil
}

func (recording *RecordingContext) GoalReopen(input model.GoalReopenInput) (*model.Goal, error) {
	goal, err := recording.goal(input.ID)
	if err != nil {
		return nil, err
	}
	goal.Status = "open"
	goal.Prompt = input.Prompt
	goal.Client = input.Client
	goal.LLM = input.LLM
	recording.record("goal.reopen", input)
	return goal, nil
}

func (recording *RecordingContext) GoalDelete(input model.GoalDeleteInput) (bool, error) {
	before := len(recording.records.Goals)
	kept := make([]*model.Goal, 0, before)
	for _, goal := range recording.records.Goals {
		if goal.ID != input.ID {
			kept = append(kept, goal)
		}
	}
	recording.records.Goals = kept
	recording.record("goal.delete", input)
	return len(kept) < before, nil
}

func (recording *RecordingContext) TodoCreate(input model.TodoCreateInput) (*model.Todo, error) {
	todo := &model.Todo{ID: deriveRecordID(input.Name), Name: input.Name, Description: input.Description, ParentID: input.ParentID}
	recording.records.Todos = append(recording.records.Todos, todo)
	recording.record("todo.create", input)
	return todo, nil
}

func (recording *RecordingContext) TodoChange(input model.TodoChangeInput) (*model.Todo, error) {
	for _, todo := range recording.records.Todos {
		if todo.ID != input.ID {
			continue
		}
		if input.Name != nil {
			todo.Name = *input.Name
		}
		if input.Description != nil {
			todo.Description = *input.Description
		}
		recording.record("todo.change", input)
		return todo, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("todo %s not found", input.ID)}
}

func (recording *RecordingContext) TodoDelete(id string) (bool, error) {
	before := len(recording.records.Todos)
	kept := make([]*model.Todo, 0, before)
	for _, todo := range recording.records.Todos {
		if todo.ID != id {
			kept = append(kept, todo)
		}
	}
	recording.records.Todos = kept
	recording.record("todo.delete", id)
	return len(kept) < before, nil
}

func (recording *RecordingContext) DraftCreate(input model.DraftCreateInput) (*model.Draft, error) {
	draft := &model.Draft{ID: deriveRecordID(input.Title)}
	recording.records.Drafts = append(recording.records.Drafts, draft)
	recording.record("draft.create", input)
	return draft, nil
}

func (recording *RecordingContext) DraftDelete(id string) (bool, error) {
	before := len(recording.records.Drafts)
	kept := make([]*model.Draft, 0, before)
	for _, draft := range recording.records.Drafts {
		if draft.ID != id {
			kept = append(kept, draft)
		}
	}
	recording.records.Drafts = kept
	recording.record("draft.delete", id)
	return len(kept) < before, nil
}

/** 🎫️ The recorded ticket one date and slug name. */
func (recording *RecordingContext) ticket(year, month, day int, slug string) (*model.Ticket, error) {
	for _, ticket := range recording.records.Tickets {
		if ticket.Year == year && ticket.Month == month && ticket.Day == day && ticket.Slug == slug {
			return ticket, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("ticket %d/%d/%d/%s not found", year, month, day, slug)}
}

func (recording *RecordingContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	now := recording.records.Now
	year, month, day := 0, 0, 0
	if len(now) >= 10 {
		year, _ = strconv.Atoi(now[0:4])
		month, _ = strconv.Atoi(now[5:7])
		day, _ = strconv.Atoi(now[8:10])
	}
	slug := strings.ToUpper(deriveRecordID(input.Title))
	ticket := &model.Ticket{
		Year: year, Month: month, Day: day, Slug: slug, Title: input.Title, Emoji: input.Emoji,
		Status: model.TicketStatusOpen, Description: input.Prompt,
		Management: &model.TicketManagementData{Issue: input.Issue}, Goal: input.Goal, Parent: input.Parent,
		Interactions: []model.Interaction{{
			Kind: "ticket.open", Date: now, Author: input.Client, Client: input.Client,
			Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort,
		}},
		FolderPath: fmt.Sprintf("%04d/%02d/%02d/%s", year, month, day, slug),
	}
	recording.records.Tickets = append(recording.records.Tickets, ticket)
	recording.record("ticket.open", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	ticket.Status = model.TicketStatusClosed
	ticket.Summary = input.Summary
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	files := make([]model.InteractionFile, 0, len(input.Files))
	for _, path := range input.Files {
		files = append(files, model.InteractionFile{Path: path})
	}
	ticket.Interactions = append(ticket.Interactions, model.Interaction{Kind: "ticket.close", Date: recording.records.Now, Summary: input.Summary, Files: files})
	recording.record("ticket.close", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	ticket.Status = model.TicketStatusOpen
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.open", Date: recording.records.Now, Author: input.Client, Client: input.Client,
		Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort,
	})
	recording.record("ticket.reopen", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	ticket, err := recording.ticket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		ticket.Title = *input.Title
	}
	if input.Prompt != nil {
		ticket.Description = *input.Prompt
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
	}
	recording.record("ticket.change", input)
	return ticket, nil
}

func (recording *RecordingContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	before := len(recording.records.Tickets)
	kept := make([]*model.Ticket, 0, before)
	for _, ticket := range recording.records.Tickets {
		if ticket.Year == input.Year && ticket.Month == input.Month && ticket.Day == input.Day && ticket.Slug == input.Slug {
			continue
		}
		kept = append(kept, ticket)
	}
	recording.records.Tickets = kept
	recording.record("ticket.delete", input)
	return len(kept) < before, nil
}

func (recording *RecordingContext) ContributorAdd(input model.ContributorAddInput) (*model.Contributor, error) {
	text := func(value *string) string {
		if value == nil {
			return ""
		}
		return *value
	}
	contributor := &model.Contributor{
		Alias: input.Github, Github: input.Github, Name: text(input.Name), Names: input.Names,
		Email: text(input.Email), Emails: input.Emails, Fingerprint: text(input.Fingerprint), Fingerprints: input.Fingerprints,
	}
	recording.records.Contributors = append(recording.records.Contributors, contributor)
	recording.record("contributor.add", input)
	return contributor, nil
}

func (recording *RecordingContext) ContributorRemove(github string) error {
	kept := make([]*model.Contributor, 0, len(recording.records.Contributors))
	for _, contributor := range recording.records.Contributors {
		if contributor.Github != github {
			kept = append(kept, contributor)
		}
	}
	recording.records.Contributors = kept
	recording.record("contributor.remove", github)
	return nil
}

/** 📁️ The last path segment of one repository-relative path. */
func lastSegment(path string) string {
	if index := strings.LastIndex(path, "/"); index >= 0 {
		return path[index+1:]
	}
	return path
}

func (recording *RecordingContext) FolderCreate(path string) (*model.Folder, error) {
	folder := &model.Folder{
		ID: deriveRecordID(path), Path: path, URI: fileURI(recording.GetRootDir(), path),
		Name: lastSegment(path), Kind: model.FolderKindOrganization,
	}
	recording.records.Folders = append(recording.records.Folders, folder)
	recording.record("folder.create", path)
	return folder, nil
}

func (recording *RecordingContext) FolderMove(src, dst string) (*model.Folder, error) {
	for _, folder := range recording.records.Folders {
		if folder.Path != src {
			continue
		}
		folder.Path = dst
		folder.URI = fileURI(recording.GetRootDir(), dst)
		folder.Name = lastSegment(dst)
		recording.record("folder.move", []string{src, dst})
		return folder, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("folder %s not found", src)}
}

func (recording *RecordingContext) FolderDelete(path string) error {
	kept := make([]*model.Folder, 0, len(recording.records.Folders))
	for _, folder := range recording.records.Folders {
		if folder.Path != path {
			kept = append(kept, folder)
		}
	}
	recording.records.Folders = kept
	recording.record("folder.delete", path)
	return nil
}

func (recording *RecordingContext) FileCreate(path string) (*model.File, error) {
	name := lastSegment(path)
	extension := ""
	if index := strings.LastIndex(name, "."); index >= 0 {
		extension = name[index:]
	}
	file := &model.File{
		ID: deriveRecordID(path), Path: path, URI: fileURI(recording.GetRootDir(), path),
		Name: name, Extension: extension, Kind: "code",
	}
	recording.records.Files = append(recording.records.Files, file)
	recording.record("file.create", path)
	return file, nil
}

func (recording *RecordingContext) FileMove(src, dst string) (*model.File, error) {
	for _, file := range recording.records.Files {
		if file.Path != src {
			continue
		}
		file.Path = dst
		file.URI = fileURI(recording.GetRootDir(), dst)
		file.Name = lastSegment(dst)
		recording.record("file.move", []string{src, dst})
		return file, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("file %s not found", src)}
}

func (recording *RecordingContext) FileDelete(path string) error {
	kept := make([]*model.File, 0, len(recording.records.Files))
	for _, file := range recording.records.Files {
		if file.Path != path {
			kept = append(kept, file)
		}
	}
	recording.records.Files = kept
	recording.record("file.delete", path)
	return nil
}

func (recording *RecordingContext) SectionCreate(file, name string, parent *string) (*model.Section, error) {
	path := name
	if parent != nil {
		path = *parent + "#" + name
	}
	section := &model.Section{Name: name, Path: path, FilePath: file}
	recording.records.Sections = append(recording.records.Sections, section)
	recording.record("section.create", []string{file, name})
	return section, nil
}

func (recording *RecordingContext) SectionMove(file, oldName, newName string) (*model.Section, error) {
	for _, section := range recording.records.Sections {
		if section.FilePath != file || section.Name != oldName {
			continue
		}
		section.Name = newName
		section.Path = newName
		recording.record("section.move", []string{file, oldName, newName})
		return section, nil
	}
	return nil, &ContextError{Message: fmt.Sprintf("section %s not found in %s", oldName, file)}
}

func (recording *RecordingContext) SectionDelete(file, name string) error {
	kept := make([]*model.Section, 0, len(recording.records.Sections))
	for _, section := range recording.records.Sections {
		if section.FilePath == file && section.Name == name {
			continue
		}
		kept = append(kept, section)
	}
	recording.records.Sections = kept
	recording.record("section.delete", []string{file, name})
	return nil
}

/** 🔤️ The optional strings one recorded write reports verbatim. */
func recordedOptionals(values ...*string) []interface{} {
	rows := make([]interface{}, 0, len(values))
	for _, value := range values {
		if value == nil {
			rows = append(rows, nil)
			continue
		}
		rows = append(rows, *value)
	}
	return rows
}

func (recording *RecordingContext) Integrate(source, targetSection, targetFile, targetParent *string) (*model.File, error) {
	recording.record("integrate", recordedOptionals(source, targetSection, targetFile, targetParent))
	target := ""
	if targetFile != nil {
		target = *targetFile
	}
	for _, file := range recording.records.Files {
		if file.Path == target {
			return file, nil
		}
	}
	return nil, &ContextError{Message: fmt.Sprintf("file %s not found", target)}
}

func (recording *RecordingContext) Extract(sourceFile, sourceSection, targetFile *string) (*model.File, error) {
	recording.record("extract", recordedOptionals(sourceFile, sourceSection, targetFile))
	target := ""
	if targetFile != nil {
		target = *targetFile
	}
	for _, file := range recording.records.Files {
		if file.Path == target {
			return file, nil
		}
	}
	return recording.FileCreate(target)
}

func (recording *RecordingContext) SyncManagement() (bool, error) {
	recording.record("sync.management", nil)
	return true, nil
}

// #endregion 🎞️Recording Mutations
