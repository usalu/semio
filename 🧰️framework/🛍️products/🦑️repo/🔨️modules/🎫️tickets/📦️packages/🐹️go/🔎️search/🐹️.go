package tickets

import (
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🔎️Search

// 🔍️TicketQuery is what narrows a ticket listing.
type TicketQuery struct {
	Filter         string   `json:"filter"`
	Query          string   `json:"query"`
	MatchCase      bool     `json:"matchCase"`
	MatchWholeWord bool     `json:"matchWholeWord"`
	Status         *string  `json:"status"`
	IncludeYears   []int    `json:"includeYears"`
	ExcludeYears   []int    `json:"excludeYears"`
	Slugs          []string `json:"slugs"`
}

// ✔️Matches reports whether a ticket survives the query.
func (query TicketQuery) Matches(ticket *model.Ticket) bool {
	if query.Status != nil && string(ticket.Status) != *query.Status {
		return false
	}
	if len(query.IncludeYears) > 0 && !containsTicketInt(query.IncludeYears, ticket.Year) {
		return false
	}
	if containsTicketInt(query.ExcludeYears, ticket.Year) {
		return false
	}
	if len(query.Slugs) > 0 && !containsTicketString(query.Slugs, ticket.Slug) {
		return false
	}
	identifier := NewTicketID(ticket.Year, ticket.Month, ticket.Day, ticket.Slug).ID()
	if query.Filter != "" {
		matched := false
		for _, candidate := range []string{identifier, ticket.Slug, ticket.Title} {
			if query.matchesFilter(candidate) {
				matched = true
				break
			}
		}
		if !matched {
			return false
		}
	}
	if query.Query != "" {
		haystack := strings.ToLower(fmt.Sprintf("%s %s %s %s %s", identifier, ticket.Slug, ticket.Title, ticket.Description, ticket.Status))
		if !strings.Contains(haystack, strings.ToLower(query.Query)) {
			return false
		}
	}
	return true
}

// 🔤️matchesFilter reports whether one candidate satisfies the filter.
func (query TicketQuery) matchesFilter(name string) bool {
	target, pattern := name, query.Filter
	if !query.MatchCase {
		target, pattern = strings.ToLower(name), strings.ToLower(query.Filter)
	}
	if query.MatchWholeWord {
		for _, word := range strings.FieldsFunc(target, func(character rune) bool {
			return !((character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z') || (character >= '0' && character <= '9') || character == '_')
		}) {
			if word == pattern {
				return true
			}
		}
		return false
	}
	return strings.Contains(target, pattern)
}

// 🔎️containsTicketInt reports whether a slice carries a value.
func containsTicketInt(values []int, value int) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🔍️Search returns every ticket a query keeps, in listing order.
func (service *TicketService) Search(query TicketQuery) ([]*model.Ticket, error) {
	tickets, err := service.List(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	kept := make([]*model.Ticket, 0, len(tickets))
	for _, ticket := range tickets {
		if query.Matches(ticket) {
			kept = append(kept, ticket)
		}
	}
	return kept, nil
}

// #endregion 🔎️Search
