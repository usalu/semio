package tickets

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	sort "sort"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
)

// #region 🔒️Service Close

// 📪️Close closes a ticket: consumes the important document, syncs and emits.
func (service *TicketService) Close(request TicketCloseRequest) (TicketOutcome, error) {
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	if ticket.Status != model.TicketStatusOpen {
		return TicketOutcome{}, errConflict("ticket is not open")
	}
	files := NormalizeTicketFileInputs(request.Files)
	summary := request.Summary
	if request.Bulk {
		if summary == "" {
			summary = "Bulk close"
		}
	}
	if err:=ValidateTicketCloseRequest(request);err!=nil {return TicketOutcome{},err}
	journal := TransactionJournal{}
	preimage, err := InspectImportantDocument(service.Store, ticket.ImportantPath, &journal)
	if err != nil {
		return TicketOutcome{}, err
	}
	if err := MovePlanIntoFolderIn(service.Store, ticket); err != nil {
		return TicketOutcome{}, err
	}
	warnings := make([]string, 0)
	if !request.NoManagement {
		warnings = append(warnings, CloseTicketIssue(service.Tracker, ticket, summary, nil, request.Bulk)...)
	}
	if err := RemoveImportantDocument(service.Store, preimage, &journal); err != nil {
		return TicketOutcome{}, err
	}
	lastClient := ""
	if len(ticket.Interactions) > 0 {
		lastClient = ticket.Interactions[len(ticket.Interactions)-1].Client
	}
	interactionFiles := make([]model.InteractionFile, 0, len(files))
	for _, filePath := range files {
		interactionFiles = append(interactionFiles, model.InteractionFile{Path: filePath})
	}
	ticket.Summary = summary
	ticket.Status = model.TicketStatusClosed
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.close", Date: service.Clock.Stamp(), Client: lastClient, Summary: summary, Files: interactionFiles,
	})
	service.stampInteraction(&ticket.Interactions[len(ticket.Interactions)-1])
	if err := service.Save(ticket); err != nil {
		journal.Record("save", ticket.JsonPath, "failed")
		if restore := RestoreImportantDocument(service.Store, preimage, &journal); restore != nil {
			return TicketOutcome{}, errStore(fmt.Sprintf("save ticket: %v; restore important document: %v", err, restore))
		}
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
	service.emitTicketEvent(string(events.EventTicketCloseEnded), id, map[string]interface{}{"summary": summary, "files": files})
	return service.outcome(id, ticket, journal, warnings), nil
}

// 🔓️Reopen reopens a closed ticket, creating the important document transactionally.
func (service *TicketService) Reopen(request TicketReopenRequest) (TicketOutcome, error) {
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	if ticket.Status != model.TicketStatusClosed {
		return TicketOutcome{}, errConflict("ticket is already open")
	}
	llm,effort,client,err:=ResolveTicketInteraction(request.LLM,request.Effort,request.Client)
	if err!=nil {return TicketOutcome{},err}
	if strings.TrimSpace(request.Goal) != "" {
		ticket.Goal = strings.TrimSpace(request.Goal)
	}
	if strings.TrimSpace(request.Parent) != "" {
		ticket.Parent = strings.TrimSpace(request.Parent)
	}
	if err := ApplyTicketPlanFromIDs(service.Store, service.Roots, ticket, service.McpClient, request.PlanID, request.SpecID); err != nil {
		return TicketOutcome{}, err
	}
 transfer,err:=service.prepareDraft(ticket,request.Draft);if err!=nil {return TicketOutcome{},err}
	journal := TransactionJournal{}
	creation, err := EnsureImportantDocument(service.Store, ticket.ImportantPath, &journal)
	if err != nil {
		return TicketOutcome{}, err
	}
 if err=service.transferDraft(transfer);err!=nil {
  if rollback:=RollbackImportantCreation(service.Store,creation,&journal);rollback!=nil {return TicketOutcome{},errStore(fmt.Sprintf("attach draft: %v; rollback important document: %v",err,rollback))}
  return TicketOutcome{},err
 }
	previousTitle := ticket.Title
	ticket.Status = model.TicketStatusOpen
	ticket.Interactions = append(ticket.Interactions, model.Interaction{
		Kind: "ticket.reopen", Date: service.Clock.Stamp(), Client: client, Prompt: request.Prompt, LLM: llm, Effort: effort,
	})
	AppendSessionID(ticket, request.Session)

	warnings := make([]string, 0)
	if !request.NoManagement {
		warnings = append(warnings, EnsureTicketIssue(service.Tracker, ticket, previousTitle, request.Prompt, "", nil, true)...)
		if ticket.Management != nil && ticket.Management.Issue != "" {
			if err := service.Tracker.AddComment(ticket.Management.Issue, FormatPromptHeading(request.Prompt)); err != nil {
				warnings = append(warnings, "Failed to add prompt comment to GitHub issue: "+err.Error())
			}
		}
	}
	service.stampInteraction(&ticket.Interactions[len(ticket.Interactions)-1])
	if err := service.Save(ticket); err != nil {
		journal.Record("save", ticket.JsonPath, "failed")
		if rollback := RollbackImportantCreation(service.Store, creation, &journal); rollback != nil {
			return TicketOutcome{}, errStore(fmt.Sprintf("save ticket: %v; rollback important document: %v", err, rollback))
		}
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
	service.emitTicketEvent(string(events.EventTicketReopenEnded), id, map[string]interface{}{
		"prompt": request.Prompt, "llm": llm, "effort": effort, "client": client,
	})
	return service.outcome(id, ticket, journal, warnings), nil
}

// ♻️Change changes a ticket in place, renaming its folder when the title changes its slug.
func (service *TicketService) Change(request TicketChangeRequest) (TicketOutcome, error) {
 resolved:=map[string]string{}
 for _,entry:=range []struct{key string;value *string;resolve func(string)(string,error)}{
  {"llm",request.LLM,model.ResolveAllowedLLM},{"effort",request.Effort,model.ResolveAllowedEffort},{"client",request.Client,model.ResolveAllowedClient},
 } {
  if entry.value==nil {continue}
  value,err:=resolveOptionalSlug(*entry.value,entry.resolve);if err!=nil {return TicketOutcome{},err}
  if entry.key=="client"&&value=="" {return TicketOutcome{},errInvalid("client is required")}
  resolved[entry.key]=value
 }
	id, err := ParseTicketID(request.ID)
	if err != nil {
		return TicketOutcome{}, err
	}
	ticket, err := service.Read(id)
	if err != nil {
		return TicketOutcome{}, err
	}
	journal := TransactionJournal{}
	nextID := id
	if request.Title != nil {
		title := strings.TrimSpace(*request.Title)
		if title == "" {
			return TicketOutcome{}, errInvalid("ticket title is required")
		}
		slug, err := TicketSlugFromTitle(title)
		if err != nil {
			return TicketOutcome{}, err
		}
		if parent, nested := id.ParentSlug(); nested {
			slug = parent + "/" + slug
		}
		nextID = NewTicketID(id.Year, id.Month, id.Day, slug)
		if nextID.Slug != id.Slug {
			target := service.Layout.TicketDir(nextID)
			if err:=service.validateID(nextID);err!=nil {return TicketOutcome{},err}
			if StoreExists(service.Store, target) {
				return TicketOutcome{}, errConflict("ticket folder already exists: " + target)
			}
			nodes:=0
			if err:=service.inspectProjection(service.Layout.TicketDir(id),target,0,&nodes);err!=nil {return TicketOutcome{},err}
			if err := service.Store.Rename(service.Layout.TicketDir(id), target); err != nil {
				return TicketOutcome{}, err
			}
			journal.Record("rename", target, "ok")
		}
		ticket.Title = title
	}
 if request.Description != nil {
  ticket.Description=*request.Description
  if len(ticket.Interactions)>0 {ticket.Interactions[len(ticket.Interactions)-1].Prompt=*request.Description}
 }
 if len(ticket.Interactions)>0 {
  interaction:=&ticket.Interactions[len(ticket.Interactions)-1]
  if value,ok:=resolved["llm"];ok {interaction.LLM=value}
  if value,ok:=resolved["effort"];ok {interaction.Effort=value}
  if value,ok:=resolved["client"];ok {interaction.Client=value}
 }
	if request.Goal != nil {
		ticket.Goal = *request.Goal
	}
	if request.Parent != nil {
		ticket.Parent = *request.Parent
	}
	service.hydrate(ticket, nextID)
	if err := service.Save(ticket); err != nil {
		return TicketOutcome{}, err
	}
	journal.Record("save", ticket.JsonPath, "ok")
 warnings:=make([]string,0)
 if request.Title!=nil&&!request.NoManagement&&ticket.Management!=nil&&ticket.Management.Issue!="" {
  if err:=service.Tracker.UpdateIssueTitle(ticket.Management.Issue,ticket.Title);err!=nil {warnings=append(warnings,"Failed to update GitHub issue title: "+err.Error())}
 }
 service.emitTicketEvent(string(events.EventTicketChangeEnded),nextID,map[string]interface{}{"title":ticket.Title,"goal":ticket.Goal})
 return service.outcome(nextID,ticket,journal,warnings),nil
}

// 💧️hydrate fills in every derived path of a ticket.
func (service *TicketService) hydrate(ticket *model.Ticket, id TicketID) {
	ticket.Year = id.Year
	ticket.Month = id.Month
	ticket.Day = id.Day
	ticket.Slug = id.Slug
	ticket.FolderPath = service.Layout.TicketDir(id)
	ticket.JsonPath = service.Layout.DocumentPath(id)
	ticket.ImportantPath = service.Layout.ImportantPath(id)
}

// 🎫️outcome assembles the outcome of one lifecycle operation.
func (service *TicketService) outcome(id TicketID, ticket *model.Ticket, journal TransactionJournal, warnings []string) TicketOutcome {
	return TicketOutcome{
		ID:       id.ID(),
		RelPath:  id.RelPath(),
		Status:   string(ticket.Status),
		Document: EncodeTicketDocument(ticket),
		Journal:  journal,
		Warnings: warnings,
	}
}

// 📤️emitTicketEvent emits one lifecycle event with its identity members merged in.
func (service *TicketService) emitTicketEvent(kind string, id TicketID, extra map[string]interface{}) {
	if service.Events == nil {
		return
	}
	payload := map[string]interface{}{"id": id.RelPath(), "year": id.Year, "month": id.Month, "day": id.Day, "slug": id.Slug}
	for name, value := range extra {
		payload[name] = value
	}
	if service.InteractionMetadata.Author!="" { payload["author"]=service.InteractionMetadata.Author }
	service.Events.Emit(kind, TicketEventSource, canonicalTicketPayload(payload))
}

// 🎆️datedChildren returns the date directories under a path, ascending.
func (service *TicketService) datedChildren(path, prefix string, pinned *int) []int {
	if pinned != nil {
		candidate := JoinPath(path, prefix+PadNumber(*pinned, 2))
		if StoreExists(service.Store, candidate) {
			return []int{*pinned}
		}
		return nil
	}
	entries, err := service.Store.Entries(path)
	if err != nil {
		return nil
	}
	values := make([]int, 0, len(entries))
	for _, entry := range entries {
		if entry.Kind != NodeDirectory {
			continue
		}
		if value, err := ParseDatedDirSegment(entry.Name, prefix); err == nil {
			values = append(values, value)
		}
	}
	sort.Ints(values)
	return values
}

// 🔤️slugsUnder returns every ticket slug under a day directory.
func (service *TicketService) slugsUnder(dayPath, prefix string) []string {
	root := dayPath
	if prefix != "" {
		root = JoinPath(dayPath, prefix)
	}
	entries, err := service.Store.Entries(root)
	if err != nil {
		return nil
	}
	slugs := make([]string, 0)
	for _, entry := range entries {
		if entry.Kind != NodeDirectory || strings.HasPrefix(entry.Name, ".") || containsTicketString(SkippedWalkDirs, entry.Name) {
			continue
		}
		slug := entry.Name
		if prefix != "" {
			slug = prefix + "/" + entry.Name
		}
		if StoreExists(service.Store, JoinPath(JoinPath(dayPath, slug), TicketDocumentName)) {
			slugs = append(slugs, slug)
			continue
		}
		slugs = append(slugs, service.slugsUnder(dayPath, slug)...)
	}
	return slugs
}

// 🔎️containsTicketString reports whether a slice carries a value.
func containsTicketString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🔤️resolveOptionalSlug resolves a slug when it is given, and accepts a blank one.
func resolveOptionalSlug(value string, resolve func(string) (string, error)) (string, error) {
	if strings.TrimSpace(value) == "" {
		return "", nil
	}
	resolved, err := resolve(value)
	if err != nil {
		return "", errInvalid(err.Error())
	}
	return resolved, nil
}

// 🧮️canonicalTicketPayload renders a payload as compact JSON with members in ascending key order.
func canonicalTicketPayload(payload interface{}) string {
	data, err := json.Marshal(payload)
	if err != nil {
		return ""
	}
	var generic interface{}
	if err := json.Unmarshal(data, &generic); err != nil {
		return ""
	}
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(generic); err != nil {
		return ""
	}
	return strings.TrimRight(buffer.String(), "\n")
}

// 📥️ParseOpenRequest reads an open request from JSON text.
func ParseOpenRequest(text string) (TicketOpenRequest, error) {
	var request TicketOpenRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketOpenRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseCloseRequest reads a close request from JSON text.
func ParseCloseRequest(text string) (TicketCloseRequest, error) {
	var request TicketCloseRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketCloseRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseReopenRequest reads a reopen request from JSON text.
func ParseReopenRequest(text string) (TicketReopenRequest, error) {
	var request TicketReopenRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketReopenRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// 📥️ParseChangeRequest reads a change request from JSON text.
func ParseChangeRequest(text string) (TicketChangeRequest, error) {
	var request TicketChangeRequest
	if err := json.Unmarshal([]byte(text), &request); err != nil {
		return TicketChangeRequest{}, errInvalid(err.Error())
	}
	return request, nil
}

// #endregion 🔒️Service Close
