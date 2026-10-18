package tickets

import (
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
	PlanID       string `json:"planId"`
	SpecID       string `json:"specId"`
}

// ♻️TicketChangeRequest is everything a change request carries.
type TicketChangeRequest struct {
	ID           string  `json:"id"`
	Title        *string `json:"title"`
	Description  *string `json:"description"`
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

// 🎫️TicketService is the ticket domain bound to one store, tracker, clock and event sink.
type TicketService struct {
	Layout    TicketLayout
	Store     TicketStore
	Tracker   IssueTracker
	Clock     TicketClock
	Events    EventSink
	Roots     PlanRoots
	McpClient providers.McpClientKind
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
	documentPath := service.Layout.DocumentPath(id)
	if !StoreExists(service.Store, documentPath) {
		return nil, errTicketNotFound("ticket not found: " + documentPath)
	}
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
	if ticket.JsonPath == "" {
		return errInvalid("ticket json path is empty")
	}
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
	if strings.TrimSpace(request.Goal) == "" {
		return TicketOutcome{}, errInvalid("ticket goal is required")
	}
	slug, err := ValidateTicketEmojiTitle(request.Emoji, request.Title)
	if err != nil {
		return TicketOutcome{}, err
	}
	llm, err := resolveOptionalSlug(request.LLM, model.ResolveAllowedLLM)
	if err != nil {
		return TicketOutcome{}, err
	}
	effort, err := resolveOptionalSlug(request.Effort, model.ResolveAllowedEffort)
	if err != nil {
		return TicketOutcome{}, err
	}
	client, err := model.ResolveAllowedClient(request.Client)
	if err != nil {
		return TicketOutcome{}, errInvalid(err.Error())
	}
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
	ticketDir := service.Layout.TicketDir(id)
	if StoreExists(service.Store, ticketDir) {
		return TicketOutcome{}, errConflict("ticket folder already exists: " + ticketDir)
	}
	if err := service.Store.CreateDirectory(ticketDir); err != nil {
		return TicketOutcome{}, err
	}
	journal := TransactionJournal{}
	if err := service.Store.Write(service.Layout.ImportantPath(id), ""); err != nil {
		return TicketOutcome{}, err
	}
	journal.Record("create-important", service.Layout.ImportantPath(id), "ok")

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
	AppendSessionID(ticket, request.Session)
	if err := ApplyTicketPlanFromIDs(service.Store, service.Roots, ticket, service.McpClient, request.PlanID, request.SpecID); err != nil {
		return TicketOutcome{}, err
	}
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
