package tickets

import (
 "fmt"
 "path"
 "unicode/utf16"
	sort "sort"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
)

// #region 🛎️Service

// 📬️TicketOpenRequest is everything an open request carries.
type TicketOpenRequest struct {
	Emoji        string `json:"emoji"`
	Title        string `json:"title"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort"`
	Client       string `json:"client"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent"`
	NoIssue      bool   `json:"noIssue"`
	NoManagement bool   `json:"noManagement"`
	Issue        string `json:"issue"`
	Session      string `json:"session"`
	Draft        string `json:"draft"`
	PlanID       string `json:"planId"`
	SpecID       string `json:"specId"`
}

// 📪️TicketCloseRequest is everything a close request carries.
type TicketCloseRequest struct {
	ID           string   `json:"id"`
	Summary      string   `json:"summary"`
	Files        []string `json:"files"`
	NoManagement bool     `json:"noManagement"`
	Bulk         bool     `json:"bulk"`
}

// 🔓️TicketReopenRequest is everything a reopen request carries.
type TicketReopenRequest struct {
	ID           string `json:"id"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort"`
	Client       string `json:"client"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent"`
	NoManagement bool   `json:"noManagement"`
	Session      string `json:"session"`
	Draft        string `json:"draft"`
	PlanID       string `json:"planId"`
	SpecID       string `json:"specId"`
}

// ♻️TicketChangeRequest is everything a change request carries.
type TicketChangeRequest struct {
	ID           string  `json:"id"`
	Title        *string `json:"title"`
	Description  *string `json:"description"`
	LLM          *string `json:"llm"`
	Effort       *string `json:"effort"`
	Client       *string `json:"client"`
	Goal         *string `json:"goal"`
	Parent       *string `json:"parent"`
	NoManagement bool    `json:"noManagement"`
}

// 🎫️TicketOutcome is the outcome of one lifecycle operation.
type TicketOutcome struct {
	ID       string             `json:"id"`
	RelPath  string             `json:"relPath"`
	Status   string             `json:"status"`
	Document string             `json:"document"`
	Journal  TransactionJournal `json:"journal"`
	Warnings []string           `json:"warnings"`
}

// 🧹️PurgeReport is what an artifact purge removed.
type PurgeReport struct {
	RemovedDirectories []string `json:"removedDirectories"`
	RemovedFiles       []string `json:"removedFiles"`
}

// 🧹️OversizedFileBytes is the size above which a file is purged from a closed ticket folder.
const OversizedFileBytes int64 = 5 << 20

// 🧹️OversizedFolderBytes is the size above which a subfolder is purged.
const OversizedFolderBytes int64 = 10 << 20

// 🪪️TicketInteractionMetadata holds repository-bound interaction facts.
type TicketInteractionMetadata struct {
 Author string `json:"author"`
 System string `json:"system"`
 Checkpoint string `json:"checkpoint"`
}

// 🎫️TicketService is the ticket domain bound to one store, tracker, clock and event sink.
type TicketService struct {
	Layout    TicketLayout
	Store     TicketStore
	Tracker   IssueTracker
	Clock     TicketClock
	Events    EventSink
	Roots     PlanRoots
	McpClient providers.McpClientKind
	InteractionMetadata TicketInteractionMetadata
}

// 🆕️NewTicketService binds the domain to its ports.
func NewTicketService(layout TicketLayout, store TicketStore, tracker IssueTracker, clock TicketClock, sink EventSink) *TicketService {
	return &TicketService{Layout: layout, Store: store, Tracker: tracker, Clock: clock, Events: sink, McpClient: providers.McpClientGeneric}
}

// 🗺️WithRoots sets the roots plan resolution reads.
func (service *TicketService) WithRoots(roots PlanRoots) *TicketService {
	service.Roots = roots
	return service
}

// 🪪️WithMcpClient sets the MCP surface a plan or spec id is resolved against.
func (service *TicketService) WithMcpClient(kind providers.McpClientKind) *TicketService {
	service.McpClient = kind
	return service
}

// 📖️Read reads one ticket, filling in every derived path.
func (service *TicketService) Read(id TicketID) (*model.Ticket, error) {
 if err:=service.validateID(id);err!=nil {return nil,err}
	documentPath := service.Layout.DocumentPath(id)
	if err:=service.validateAncestors(id);err!=nil {return nil,err}
 metadata,found:=service.Store.Metadata(documentPath)
	if !found {
		return nil, errTicketNotFound("ticket not found: " + documentPath)
	}
 if metadata.Kind!=NodeFile {return nil,errInvalid("ticket document must be a regular file")}
	document, err := service.Store.Read(documentPath)
	if err != nil {
		return nil, err
	}
	ticket, err := DecodeTicketDocument(document)
	if err != nil {
		return nil, err
	}
	service.hydrate(ticket, id)
	return ticket, nil
}

// 💾️Save writes one ticket back.
func (service *TicketService) Save(ticket *model.Ticket) error {
 if ticket==nil {return errInvalid("ticket is nil")}
 id:=NewTicketID(ticket.Year,ticket.Month,ticket.Day,ticket.Slug);if err:=service.validateID(id);err!=nil {return err}
 if NormalizeSeparators(ticket.JsonPath)!=service.Layout.DocumentPath(id) {return errInvalid("ticket document is outside its selected layout")}
	if ticket.JsonPath == "" {
		return errInvalid("ticket json path is empty")
	}
 if err:=service.validateAncestors(id);err!=nil {return err}
 if metadata,found:=service.Store.Metadata(ticket.JsonPath);found&&metadata.Kind!=NodeFile {return errInvalid("ticket document must be a regular file")}
	return service.Store.Write(ticket.JsonPath, EncodeTicketDocument(ticket))
}

// ▪️List returns every ticket, optionally narrowed to a year, a month and a day.
func (service *TicketService) List(year, month, day *int) ([]*model.Ticket, error) {
	ticketsDir := service.Layout.TicketsDir()
	if !StoreExists(service.Store, ticketsDir) {
		return nil, nil
	}
	found := make([]*model.Ticket, 0)
	for _, yearValue := range service.datedChildren(ticketsDir, EmojiYear, year) {
		yearPath := JoinPath(ticketsDir, FormatYearDir(yearValue))
		for _, monthValue := range service.datedChildren(yearPath, EmojiMonth, month) {
			monthPath := JoinPath(yearPath, FormatMonthDir(monthValue))
			for _, dayValue := range service.datedChildren(monthPath, EmojiDay, day) {
				dayPath := JoinPath(monthPath, FormatDayDir(dayValue))
				for _, slug := range service.slugsUnder(dayPath, "") {
					if ticket, err := service.Read(NewTicketID(yearValue, monthValue, dayValue, slug)); err == nil {
						found = append(found, ticket)
					}
				}
			}
		}
	}
	return found, nil
}

// 🔎️FindBySlug returns the most recently created ticket whose slug matches.
func (service *TicketService) FindBySlug(slug string) (*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	for index := len(tickets) - 1; index >= 0; index-- {
		if tickets[index].Slug == slug || BaseName(tickets[index].Slug) == slug {
			return tickets[index], nil
		}
	}
	return nil, errTicketNotFound("ticket not found: " + slug)
}

// 🕰️Latest returns the newest ticket by date, then by slug.
func (service *TicketService) Latest() (*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, errTicketNotFound("no tickets found")
	}
	sort.SliceStable(tickets, func(left, right int) bool {
		first, second := tickets[left], tickets[right]
		if first.Year != second.Year {
			return first.Year > second.Year
		}
		if first.Month != second.Month {
			return first.Month > second.Month
		}
		if first.Day != second.Day {
			return first.Day > second.Day
		}
		return first.Slug > second.Slug
	})
	return tickets[0], nil
}

// 📬️Open opens a ticket: validates, materialises the folder, syncs the issue and emits.
func (service *TicketService) Open(request TicketOpenRequest) (TicketOutcome, error) {
 if request.Prompt=="" {request.Prompt=request.Title}
	if strings.TrimSpace(request.Goal) == "" {
		return TicketOutcome{}, errInvalid("ticket goal is required")
	}
	slug, err := ValidateTicketEmojiTitle(request.Emoji, request.Title)
	if err != nil {
		return TicketOutcome{}, err
	}
	llm,effort,client,err:=ResolveTicketInteraction(request.LLM,request.Effort,request.Client)
	if err!=nil {return TicketOutcome{},err}
	year, month, day := service.Clock.Today()
	if strings.TrimSpace(request.Parent) != "" {
		parent, err := service.FindBySlug(strings.TrimSpace(request.Parent))
		if err != nil {
			return TicketOutcome{}, err
		}
		year, month, day = parent.Year, parent.Month, parent.Day
		slug = parent.Slug + "/" + slug
	}
	id := NewTicketID(year, month, day, slug)
	if err:=service.validateID(id);err!=nil {return TicketOutcome{},err}
	ticketDir := service.Layout.TicketDir(id)
	if StoreExists(service.Store, ticketDir) {
		return TicketOutcome{}, errConflict("ticket folder already exists: " + ticketDir)
	}
	journal := TransactionJournal{}

	ticket := &model.Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        id.Slug,
		Title:       strings.TrimSpace(request.Title),
		Emoji:       strings.TrimSpace(request.Emoji),
		Status:      model.TicketStatusOpen,
		Description: request.Prompt,
		Goal:        request.Goal,
		Parent:      request.Parent,
		Interactions: []model.Interaction{{
			Kind:   "ticket.open",
			Date:   service.Clock.Stamp(),
			Client: client,
			Prompt: request.Prompt,
			LLM:    llm,
			Effort: effort,
		}},
		FolderPath:    ticketDir,
		JsonPath:      service.Layout.DocumentPath(id),
		ImportantPath: service.Layout.ImportantPath(id),
	}
	service.stampInteraction(&ticket.Interactions[0])
	AppendSessionID(ticket, request.Session)
	if err := ApplyTicketPlanFromIDs(service.Store, service.Roots, ticket, service.McpClient, request.PlanID, request.SpecID); err != nil {
		return TicketOutcome{}, err
	}
 transfer,err:=service.prepareDraft(ticket,request.Draft);if err!=nil {return TicketOutcome{},err}
	if err:=ticketPathBound(service.Layout.ImportantPath(id));err!=nil {return TicketOutcome{},err}
 if err:=service.validateAncestors(id);err!=nil {return TicketOutcome{},err}
	if err := service.Store.CreateDirectory(ticketDir); err != nil {
		return TicketOutcome{}, err
	}
	if err := service.Store.Write(service.Layout.ImportantPath(id), ""); err != nil {
		return TicketOutcome{}, err
	}
	journal.Record("create-important", service.Layout.ImportantPath(id), "ok")

 if err=service.transferDraft(transfer);err!=nil {return TicketOutcome{},err}
	warnings := make([]string, 0)
	if !request.NoIssue && !request.NoManagement {
		milestone := MilestoneNumberForTitle(service.Tracker, request.Goal)
		warnings = append(warnings, EnsureTicketIssue(service.Tracker, ticket, ticket.Title, request.Prompt, request.Issue, milestone, false)...)
	}
	if err := service.Save(ticket); err != nil {
		return TicketOutcome{}, err
	}
	service.emitTicketEvent(string(events.EventTicketOpenEnded), id, map[string]interface{}{
		"title": ticket.Title, "prompt": request.Prompt, "llm": llm, "effort": effort,
		"client": client, "goal": request.Goal, "parent": request.Parent,
	})
	return service.outcome(id, ticket, journal, warnings), nil
}

// #endregion 🛎️Service

// 🪪️stampInteraction records facts supplied by the selected repository owner.
func (service *TicketService) stampInteraction(interaction *model.Interaction) {
 interaction.Author=service.InteractionMetadata.Author
 interaction.System=service.InteractionMetadata.System
 interaction.Checkpoint=service.InteractionMetadata.Checkpoint
}

// 📏️ticketPathBound refuses a path before its filesystem effect.
func ticketPathBound(value string) error {
 if len(utf16.Encode([]rune(value)))>256 {return errInvalid("ticket path exceeds 256 characters")}
 return nil
}

// 🔤️portableDraftSegment is shared by identifiers and attached portable file names.
func portableDraftSegment(value string) bool {
 if value==""||value=="."||value==".."||strings.TrimRight(value," .")!=value||strings.ContainsAny(value,"/\\<>:\"|?*") {return false}
 for _,character:=range value {if character<32||character==127 {return false}}
 base:=strings.ToUpper(strings.SplitN(value,".",2)[0])
 switch base {case "CON","PRN","AUX","NUL":return false}
 if strings.HasPrefix(base,"COM")||strings.HasPrefix(base,"LPT") {suffix:=[]rune(base[3:]);if len(suffix)==1&&strings.ContainsRune("123456789¹²³",suffix[0]) {return false}}
 return true
}

// 🧭️validateID keeps every service path in its dated collection.
func (service *TicketService) validateID(id TicketID) error {
 if id.Year<0||id.Year>99||id.Month<1||id.Month>12||id.Day<1||id.Day>31 {return errInvalid("ticket date is invalid")}
 for _,part:=range strings.Split(id.Slug,"/") {if !portableDraftSegment(part) {return errInvalid("ticket slug contains an invalid path segment")}}
 return ticketPathBound(service.Layout.ImportantPath(id))
}

// 📝️draftSource resolves a relative draft through the selected store only.
func (service *TicketService) draftSource(draft string) (string,[]DirectoryEntry,error) {
 if draft=="" {return "",nil,nil}
 if strings.Contains(draft,"\\") {return "",nil,errInvalid("draft identifiers use forward slashes")}
 normalized:=NormalizeSeparators(draft)
 if strings.HasPrefix(normalized,"/")||strings.Contains(normalized,":") {return "",nil,errInvalid("draft must be repository relative")}
 if err:=service.validateParentDirectories(service.Layout.RepoMetaDir());err!=nil {return "",nil,err}
 source:=JoinPath(service.Layout.RepoMetaDir(),"📝️drafts")
 for _,ancestor:=range []string{service.Layout.RepoMetaDir(),source} {metadata,found:=service.Store.Metadata(ancestor);if !found {return "",nil,errTicketNotFound("draft root not found: "+ancestor)};if metadata.Kind!=NodeDirectory {return "",nil,errInvalid("draft root must be a physical directory")}}
 for _,part:=range strings.Split(normalized,"/") {
  if !portableDraftSegment(part) {return "",nil,errInvalid("draft contains an invalid path segment")}
  source=JoinPath(source,part)
  if err:=ticketPathBound(source);err!=nil {return "",nil,err}
  metadata,found:=service.Store.Metadata(source)
  if !found {return "",nil,errTicketNotFound("draft not found: "+source)}
  if metadata.Kind!=NodeDirectory {return "",nil,errInvalid("draft must be a physical directory")}
 }
 entries,err:=service.Store.Entries(source);if err!=nil {return "",nil,err}
 if len(entries)>1024 {return "",nil,errInvalid("draft contains more than 1024 entries")}
 for _,entry:=range entries {
  if !portableDraftSegment(entry.Name)||entry.Kind==NodeSymlink {return "",nil,errInvalid("draft contains an unsupported entry")}
 }
 nodes:=0
 if err:=service.inspectProjection(source,source,0,&nodes);err!=nil {return "",nil,err}
 return source,entries,nil
}

// 🌳️inspectProjection validates an entire bounded tree before any directory move.
func (service *TicketService) inspectProjection(source,destination string,depth int,nodes *int) error {
 if depth>32 {return errInvalid("draft depth exceeds 32")}
 if err:=ticketPathBound(source);err!=nil {return err};if err:=ticketPathBound(destination);err!=nil {return err}
 metadata,found:=service.Store.Metadata(source);if !found {return errConflict("source entry changed")}
 if metadata.Kind==NodeFile {return nil};if metadata.Kind!=NodeDirectory {return errInvalid("source contains an unsupported entry")}
 entries,err:=service.Store.Entries(source);if err!=nil {return err}
 for _,entry:=range entries {
  *nodes++;if *nodes>4096 {return errInvalid("source contains more than 4096 nodes")}
  if !portableDraftSegment(entry.Name) {return errInvalid("source contains an invalid entry")}
  if err:=service.inspectProjection(JoinPath(source,entry.Name),JoinPath(destination,entry.Name),depth+1,nodes);err!=nil {return err}
 }
 return nil
}

type draftTransferPlan struct {source string;entries []DirectoryEntry;destinations []string}

// 📋️prepareDraft validates every source and projected destination before any effect.
func (service *TicketService) prepareDraft(ticket *model.Ticket,draft string) (draftTransferPlan,error) {
 if ticket==nil {return draftTransferPlan{},errInvalid("ticket is nil")}
 id:=NewTicketID(ticket.Year,ticket.Month,ticket.Day,ticket.Slug)
 if err:=service.validateID(id);err!=nil {return draftTransferPlan{},err}
 if NormalizeSeparators(ticket.FolderPath)!=service.Layout.TicketDir(id) {return draftTransferPlan{},errInvalid("ticket folder is outside its selected layout")}
 source,entries,err:=service.draftSource(draft);if err!=nil||source=="" {return draftTransferPlan{},err}
 destinations:=make([]string,len(entries));reserved:=map[string]bool{JoinPath(ticket.FolderPath,TicketDocumentName):true,JoinPath(ticket.FolderPath,ImportantDirName):true}
 for i,entry:=range entries {
  destination:=JoinPath(ticket.FolderPath,entry.Name)
  extension:=path.Ext(entry.Name);stem:=strings.TrimSuffix(entry.Name,extension)
  for suffix:=2;StoreExists(service.Store,destination)||reserved[destination];suffix++ {
   if suffix>1024 {return draftTransferPlan{},errConflict("draft attachment name has too many collisions")}
   destination=JoinPath(ticket.FolderPath,fmt.Sprintf("%s_%d%s",stem,suffix,extension))
  }
  if err:=ticketPathBound(destination);err!=nil {return draftTransferPlan{},err}
  nodes:=0
  if err:=service.inspectProjection(JoinPath(source,entry.Name),destination,0,&nodes);err!=nil {return draftTransferPlan{},err}
  destinations[i]=destination;reserved[destination]=true
 }
 return draftTransferPlan{source:source,entries:entries,destinations:destinations},nil
}

// 🚚️transferDraft preserves any failed remainder instead of recursively deleting it.
func (service *TicketService) transferDraft(plan draftTransferPlan) error {
 if plan.source=="" {return nil}
 for i,entry:=range plan.entries {if err:=service.Store.Rename(JoinPath(plan.source,entry.Name),plan.destinations[i]);err!=nil {return err}}
 return service.Store.RemoveDirectory(plan.source)
}

// 📎️AttachDraft transfers a selected repository draft after complete bounded preflight.
func (service *TicketService) AttachDraft(ticket *model.Ticket,draft string) error {
 plan,err:=service.prepareDraft(ticket,draft);if err!=nil {return err};return service.transferDraft(plan)
}

// 🗑️Delete removes the selected ticket after its requested management effect succeeds.
func (service *TicketService) Delete(id TicketID,noManagement bool) error {
 ticket,err:=service.Read(id);if err!=nil {return err}
 if !noManagement&&ticket.Management!=nil&&ticket.Management.Issue!="" {if err=service.Tracker.DeleteIssue(ticket.Management.Issue);err!=nil {return err}}
 return service.Store.RemoveTree(service.Layout.TicketDir(id))
}

// 🪪️ResolveTicketInteraction validates the complete interaction before any lifecycle effect.
func ResolveTicketInteraction(llm,effort,client string) (string,string,string,error) {
 resolvedLLM,err:=resolveOptionalSlug(llm,model.ResolveAllowedLLM);if err!=nil {return "","","",err}
 resolvedEffort,err:=resolveOptionalSlug(effort,model.ResolveAllowedEffort);if err!=nil {return "","","",err}
 if strings.TrimSpace(client)=="" {return "","","",errInvalid("client is required")}
 resolvedClient,err:=model.ResolveAllowedClient(client);if err!=nil {return "","","",errInvalid(err.Error())}
 return resolvedLLM,resolvedEffort,resolvedClient,nil
}

// 📪️ValidateTicketCloseRequest checks effect-free close input requirements.
func ValidateTicketCloseRequest(request TicketCloseRequest) error {
 if request.Bulk {return nil}
 if request.Summary=="" {return errInvalid("summary is required to finish a ticket")}
 if len(NormalizeTicketFileInputs(request.Files))==0 {return errInvalid("at least one file is required to finish a ticket")}
 return nil
}

// 🔗️validateAncestors refuses symlinked or non-directory namespace components.
func (service *TicketService) validateAncestors(id TicketID) error {
 if err:=service.validateParentDirectories(service.Layout.RepoMetaDir());err!=nil {return err}
 current:=service.Layout.RepoMetaDir()
 parts:=append([]string{TicketsDirName,FormatYearDir(id.Year),FormatMonthDir(id.Month),FormatDayDir(id.Day)},strings.Split(id.Slug,"/")...)
 if metadata,found:=service.Store.Metadata(current);found&&metadata.Kind!=NodeDirectory {return errInvalid("ticket namespace must be a physical directory")}
 for _,part:=range parts {current=JoinPath(current,part);if metadata,found:=service.Store.Metadata(current);found&&metadata.Kind!=NodeDirectory {return errInvalid("ticket namespace must be a physical directory")}}
 return nil
}

// 🏠️validateParentDirectories keeps missing or existing ancestors in one physical namespace.
func (service *TicketService) validateParentDirectories(current string) error {
 for remaining:=256;remaining>0;remaining-- {
  if metadata,found:=service.Store.Metadata(current);found&&metadata.Kind!=NodeDirectory {return errInvalid("ticket namespace must be a physical directory")}
  parent:=DirName(current);if parent==current||current=="." {return nil};current=parent
 }
 return errInvalid("ticket namespace ancestry exceeds its bound")
}
