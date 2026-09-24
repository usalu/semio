// 🐹️ Go side of the ticket document codec case, run against real committed documents.
package adapter

import (
	"encoding/json"
	"fmt"
	"strings"

	tickets "github.com/usalu/semio/repo/tickets"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type committedDocument struct {
	Source string `json:"source"`
	Text   string `json:"text"`
}

type documentVectors struct {
	Documents []committedDocument `json:"documents"`
	Refused   []string            `json:"refused"`
}

func loadDocumentVectors(ctx *host.Context) (documentVectors, error) {
	data, err := ctx.FixtureBytes("shared://📄️ticket-document-codec/📄️documents.json")
	if err != nil {
		return documentVectors{}, err
	}
	var vectors documentVectors
	err = json.Unmarshal(data, &vectors)
	return vectors, err
}

// memberNames returns the top-level member names of a JSON object, in document order.
func memberNames(text string) []string {
	decoder := json.NewDecoder(strings.NewReader(text))
	token, err := decoder.Token()
	if err != nil {
		return nil
	}
	if delimiter, ok := token.(json.Delim); !ok || delimiter != '{' {
		return nil
	}
	names := make([]string, 0)
	for decoder.More() {
		key, err := decoder.Token()
		if err != nil {
			return names
		}
		name, ok := key.(string)
		if !ok {
			return names
		}
		names = append(names, name)
		var value json.RawMessage
		if err := decoder.Decode(&value); err != nil {
			return names
		}
	}
	return names
}

func contains(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func realDocumentsDecodeToTheSameTicket(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadDocumentVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, entry := range vectors.Documents {
		ticket, err := tickets.DecodeTicketDocument(entry.Text)
		if err != nil {
			return host.Outcome{}, err
		}
		issue := ""
		if ticket.Management != nil {
			issue = ticket.Management.Issue
		}
		plan := ""
		if ticket.Plan != nil {
			plan = fmt.Sprintf("%s|%s|%s|%s", ticket.Plan.Client, ticket.Plan.ID, ticket.Plan.Source, ticket.Plan.Local)
		}
		sessions := ticket.Sessions
		if sessions == nil {
			sessions = []string{}
		}
		projection[entry.Source] = map[string]any{
			"title":       ticket.Title,
			"emoji":       ticket.Emoji,
			"status":      string(ticket.Status),
			"description": ticket.Description,
			"summary":     ticket.Summary,
			"issue":       issue,
			"goal":        ticket.Goal,
			"parent":      ticket.Parent,
			"plan":        plan,
			"sessions":    sessions,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func encodingIsGoMarshalIndent(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadDocumentVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, entry := range vectors.Documents {
		ticket, err := tickets.DecodeTicketDocument(entry.Text)
		if err != nil {
			return host.Outcome{}, err
		}
		projection[entry.Source] = tickets.EncodeTicketDocument(ticket)
	}
	return host.Outcome{Projection: projection}, nil
}

func unknownMembersAreDropped(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadDocumentVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, entry := range vectors.Documents {
		decoded, err := tickets.DecodeTicketDocument(entry.Text)
		if err != nil {
			return host.Outcome{}, err
		}
		first := tickets.EncodeTicketDocument(decoded)
		again, err := tickets.DecodeTicketDocument(first)
		if err != nil {
			return host.Outcome{}, err
		}
		second := tickets.EncodeTicketDocument(again)
		before := memberNames(entry.Text)
		after := memberNames(first)
		lost := make([]string, 0)
		for _, name := range before {
			if !contains(after, name) {
				lost = append(lost, name)
			}
		}
		stable := "no"
		if first == second {
			stable = "yes"
		}
		projection[entry.Source] = map[string]any{"stable": stable, "lost": lost}
	}
	return host.Outcome{Projection: projection}, nil
}

func aDocumentWithoutAStatusIsRefused(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadDocumentVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, text := range vectors.Refused {
		if _, err := tickets.DecodeTicketDocument(text); err != nil {
			projection[text] = fmt.Sprintf("refused:%s", tickets.TicketErrorClass(err))
			continue
		}
		projection[text] = "accepted"
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("real-documents-decode-to-the-same-ticket", realDocumentsDecodeToTheSameTicket).
		Subject("encoding-is-go-marshal-indent", encodingIsGoMarshalIndent).
		Subject("unknown-members-are-dropped", unknownMembersAreDropped).
		Subject("a-document-without-a-status-is-refused", aDocumentWithoutAStatusIsRefused)
}

// endregion 🔖️Registration
