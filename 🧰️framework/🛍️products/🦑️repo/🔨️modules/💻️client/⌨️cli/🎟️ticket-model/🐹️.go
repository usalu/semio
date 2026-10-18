// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket model, JSON codec, getters, and contribution types.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

// 🧷️TicketPlan records an IDE-native plan or spec file to archive into the ticket folder on close.
type TicketPlan struct {
	Client string `json:"client,omitempty" yaml:"client,omitempty"`
	ID     string `json:"id,omitempty" yaml:"id,omitempty"`
	Source string `json:"source,omitempty" yaml:"source,omitempty"`
	Local  string `json:"local,omitempty" yaml:"local,omitempty"`
}

// 🔖️Ticket holds the data fields for a ticket record.
type Ticket struct {
	Year          int                   `json:"-" yaml:"-"`
	Month         int                   `json:"-" yaml:"-"`
	Day           int                   `json:"-" yaml:"-"`
	Slug          string                `json:"-" yaml:"-"`
	Title         string                `json:"title" yaml:"title"`
	Emoji         string                `json:"emoji,omitempty" yaml:"emoji,omitempty"`
	Status        TicketStatus          `json:"status" yaml:"status"`
	Description   string                `json:"description,omitempty" yaml:"description,omitempty"`
	Summary       string                `json:"summary,omitempty" yaml:"summary,omitempty"`
	Management    *TicketManagementData `json:"github,omitempty" yaml:"github,omitempty"`
	Goal          string                `json:"goal,omitempty" yaml:"goal,omitempty"`
	Parent        string                `json:"-" yaml:"-"`
	Plan          *TicketPlan           `json:"plan,omitempty" yaml:"plan,omitempty"`
	Sessions      []string              `json:"sessions,omitempty" yaml:"sessions,omitempty"`
	Interactions  []Interaction         `json:"-" yaml:"-"`
	Agents        []TicketAgent         `json:"-" yaml:"-"`
	FolderPath    string                `json:"-" yaml:"-"`
	JsonPath      string                `json:"-" yaml:"-"`
	ImportantPath string                `json:"-" yaml:"-"`
}

func appendTicketSessionID(ticket *Ticket, sessionID string) {
	sessionID = strings.TrimSpace(sessionID)
	if ticket == nil || sessionID == "" {
		return
	}
	for _, existing := range ticket.Sessions {
		if existing == sessionID {
			return
		}
	}
	ticket.Sessions = append(ticket.Sessions, sessionID)
}

func isTicketInteractionKind(kind string, expected string) bool {
	kind = strings.TrimSpace(kind)
	expected = strings.TrimSpace(expected)
	if kind == expected {
		return true
	}
	if strings.HasSuffix(kind, ".ended") {
		return strings.TrimSuffix(kind, ".ended") == expected
	}
	return false
}

// 📋️UnmarshalJSON MUST require an explicit valid ticket status.
func (t *Ticket) UnmarshalJSON(data []byte) error {
	type TicketAlias struct {
		Title       string                `json:"title"`
		Emoji       string                `json:"emoji,omitempty"`
		Description string                `json:"description,omitempty"`
		Status      TicketStatus          `json:"status"`
		Summary     string                `json:"summary,omitempty"`
		Management  *TicketManagementData `json:"github,omitempty"`
		Goal        string                `json:"goal,omitempty"`
		Parent      string                `json:"parent,omitempty"`
	}
	aux := &struct {
		TicketAlias
		Plan               *TicketPlan     `json:"plan,omitempty"`
		PromptLegacy       string          `json:"prompt"`
		InteractionsLegacy []Interaction   `json:"interactions"`
		AgentsLegacy       []TicketAgent   `json:"agents"`
		SessionsRaw        json.RawMessage `json:"sessions"`
	}{}
	if err := json.Unmarshal(data, aux); err != nil {
		return err
	}
	if !aux.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	*t = Ticket{
		Title:       aux.Title,
		Emoji:       aux.Emoji,
		Description: aux.Description,
		Status:      aux.Status,
		Summary:     aux.Summary,
		Management:  aux.Management,
		Goal:        aux.Goal,
		Parent:      aux.Parent,
		Plan:        aux.Plan,
	}
	if t.Description == "" && aux.PromptLegacy != "" {
		t.Description = aux.PromptLegacy
	}
	if len(aux.InteractionsLegacy) > 0 {
		t.Interactions = aux.InteractionsLegacy
	}
	if len(aux.AgentsLegacy) > 0 {
		t.Agents = aux.AgentsLegacy
	}
	if len(aux.SessionsRaw) > 0 {
		var sessionIDs []string
		if err := json.Unmarshal(aux.SessionsRaw, &sessionIDs); err == nil {
			for _, sessionID := range sessionIDs {
				appendTicketSessionID(t, sessionID)
			}
		} else {
			var legacyAgents []TicketAgent
			if err := json.Unmarshal(aux.SessionsRaw, &legacyAgents); err == nil {
				for _, legacyAgent := range legacyAgents {
					appendTicketSessionID(t, legacyAgent.Session)
				}
				if len(t.Agents) == 0 {
					t.Agents = legacyAgents
				}
			}
		}
	}
	if t.Summary == "" {
		for i := len(t.Interactions) - 1; i >= 0; i-- {
			if isTicketInteractionKind(t.Interactions[i].Kind, "ticket.close") && t.Interactions[i].Summary != "" {
				t.Summary = t.Interactions[i].Summary
				break
			}
		}
	}
	for _, agent := range t.Agents {
		appendTicketSessionID(t, agent.Session)
	}

	if t.Goal != "" && strings.Contains(t.Goal, emojiText(EmojiGoal)) {
		t.Goal = composeIDToGoalPath(t.Goal)
	}

	contributorPrefix := emojiText(EmojiContributor)
	for i := range t.Interactions {
		if strings.HasPrefix(t.Interactions[i].Author, contributorPrefix) {
			t.Interactions[i].Author = composeIDToContributorGithub(t.Interactions[i].Author)
		}
	}

	for i := range t.Agents {
		if strings.HasPrefix(t.Agents[i].Contributor, contributorPrefix) {
			t.Agents[i].Contributor = composeIDToContributorGithub(t.Agents[i].Contributor)
		}
	}
	return nil
}

// 😀️MarshalJSON converts internal filesystem paths to repo emoji IDs for serialization.
func (t Ticket) MarshalJSON() ([]byte, error) {
	if !t.Status.IsValid() {
		return nil, fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	type TicketAlias Ticket
	alias := TicketAlias(t)

	if alias.Goal != "" && !strings.Contains(alias.Goal, emojiText(EmojiGoal)) {
		alias.Goal = goalPathToComposeID(alias.Goal)
	}
	for i := range alias.Sessions {
		alias.Sessions[i] = normalizeTicketSessionID(alias.Sessions[i])
	}
	return json.Marshal(alias)
}

// 🔖️IsNode MUST return true only when the condition is met.
// 🌻️IsNode reports whether the Ticket is node.
func (t *Ticket) IsNode() {}

// 🔖️GetID MUST return the stored value without modification.
// 🌼️GetID returns the i d of the Ticket.
func (t *Ticket) GetID() string {
	return emojiText(EmojiTicket) + Flat(t.Slug)
}

// 🧲️ticketIdToParts extracts year, month, day, slug from a ticket emoji ID.
// 🧩️It walks all tickets to find the one whose ID matches.
func ticketIdToParts(emojiId string) (year, month, day int, slug string) {
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	normalizedId := norm(emojiId)
	// Strip goal prefix to get ticket-only part
	ticketEmoji := norm(emojiText(EmojiTicket))
	lastTicketIdx := strings.LastIndex(normalizedId, ticketEmoji)
	if lastTicketIdx < 0 {
		return 0, 0, 0, ""
	}
	ticketSlugFlat := normalizedId[lastTicketIdx+len(ticketEmoji):]
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return 0, 0, 0, ""
	}
	for _, t := range tickets {
		if Flat(t.Slug) == ticketSlugFlat {
			return t.Year, t.Month, t.Day, t.Slug
		}
	}
	return 0, 0, 0, ""
}

// 🔖️GetURI MUST return the stored value without modification.
// 🌷️GetURI returns the u r i of the Ticket.
func (t *Ticket) GetURI() string {
	data := map[string]interface{}{"slug": t.Slug}
	if t.Goal != "" {
		data["goalId"] = t.Goal
	}
	return GetArtifactURI("ticket", data)
}

// 📌️GetTitle MUST return the stored value without modification.
// 🌹️GetTitle returns the title of the Ticket.
func (t *Ticket) GetTitle() string {
	return t.Title
}

// 🔖️GetPrompt MUST return the description or the first interaction prompt.
// 🥀️GetPrompt returns the prompt of the Ticket.
func (t *Ticket) GetPrompt() string {
	if t.Description != "" {
		return t.Description
	}
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Prompt
	}
	return ""
}

// 🪪️GetLatestPrompt MUST return the latest prompt from sessions or interactions.
// 🧪️GetLatestPrompt returns the latest prompt of the Ticket.
func (t *Ticket) GetLatestPrompt() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Prompt
	}
	return t.Description
}

// 🧪️GetLLM MUST return the LLM from the latest session or interaction.
// 🪻️GetLLM returns the l l m of the Ticket.
func (t *Ticket) GetLLM() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].LLM
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].LLM
	}
	return ""
}

// 🏋️GetEffort MUST return the reasoning effort from the latest session or interaction.
// 🏋️GetEffort returns the reasoning effort of the Ticket.
func (t *Ticket) GetEffort() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Effort
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].Effort
	}
	return ""
}

// 🔖️GetClient MUST return the client from the latest session or interaction.
// 🪷️GetClient returns the client of the Ticket.
func (t *Ticket) GetClient() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[len(t.Interactions)-1].Client
	}
	if len(t.Agents) > 0 {
		return t.Agents[len(t.Agents)-1].Client
	}
	return ""
}

// 🔖️GetStatus MUST return the stored value without modification.
// 🍁️GetStatus returns the status of the Ticket.
func (t *Ticket) GetStatus() TicketStatus {
	return t.Status
}

// ✍️GetAuthor MUST return the author from the first session or interaction.
// 🔐️GetAuthor returns the author of the Ticket.
func (t *Ticket) GetAuthor() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Author
	}
	if len(t.Agents) > 0 {
		return t.Agents[0].Contributor
	}
	return ""
}

// 🔖️GetCheckpoint MUST return the checkpoint from the first interaction.
// 🍂️GetCheckpoint returns the checkpoint of the Ticket.
func (t *Ticket) GetCheckpoint() string {
	if len(t.Interactions) > 0 {
		return t.Interactions[0].Checkpoint
	}
	return ""
}

// 🔖️GetSummary MUST return the stored value without modification.
// 🍃️GetSummary returns the summary of the Ticket.
func (t *Ticket) GetSummary() string {
	if t.Summary != "" {
		return t.Summary
	}
	for i := len(t.Interactions) - 1; i >= 0; i-- {
		if isTicketInteractionKind(t.Interactions[i].Kind, "ticket.close") && t.Interactions[i].Summary != "" {
			return t.Interactions[i].Summary
		}
	}
	return ""
}

// ▶️GetDateStarted MUST return the earliest date from interactions or sessions.
// 🌿️GetDateStarted returns the date started of the Ticket.
func (t *Ticket) GetDateStarted() time.Time {
	for _, interaction := range t.Interactions {
		if isTicketInteractionKind(interaction.Kind, "ticket.open") && interaction.Date != "" {
			if parsed, err := time.Parse("2006-01-02 15:04:05", interaction.Date); err == nil {
				return parsed
			}
			if parsed, err := time.Parse(time.RFC3339, interaction.Date); err == nil {
				return parsed
			}
		}
	}
	if len(t.Interactions) > 0 && t.Interactions[0].Date != "" {
		if parsed, err := time.Parse("2006-01-02 15:04:05", t.Interactions[0].Date); err == nil {
			return parsed
		}
		if parsed, err := time.Parse(time.RFC3339, t.Interactions[0].Date); err == nil {
			return parsed
		}
	}
	return time.Date(t.Year, time.Month(t.Month), t.Day, 0, 0, 0, 0, time.UTC)
}

// 🔖️GetDateFinished MUST return the stored value without modification.
// ⏹️GetDateFinished returns the date finished of the Ticket.
func (t *Ticket) GetDateFinished() *time.Time {
	for i := len(t.Interactions) - 1; i >= 0; i-- {
		if isTicketInteractionKind(t.Interactions[i].Kind, "ticket.close") && t.Interactions[i].Date != "" {
			if parsed, err := time.Parse("2006-01-02 15:04:05", t.Interactions[i].Date); err == nil {
				return &parsed
			}
			if parsed, err := time.Parse(time.RFC3339, t.Interactions[i].Date); err == nil {
				return &parsed
			}
		}
	}
	return nil
}

// 🔖️GetInteractionFiles MUST return the stored value without modification.
// ☘️GetInteractionFiles returns all unique InteractionFile entries across all interactions.
func (t *Ticket) GetInteractionFiles() []InteractionFile {
	seen := make(map[string]struct{})
	var result []InteractionFile
	for _, interaction := range t.Interactions {
		for _, f := range interaction.Files {
			if _, ok := seen[f.Path]; ok {
				continue
			}
			seen[f.Path] = struct{}{}
			result = append(result, f)
		}
	}
	return result
}

// 🔖️TicketBundleContrib holds the data fields for a ticket bundle contrib record.
type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

// 🔖️TicketFileContrib holds the data fields for a ticket file contrib record.
type TicketFileContrib struct {
	FileID   string                 `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

// 🔖️TicketSectionContrib holds the data fields for a ticket section contrib record.
type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}
