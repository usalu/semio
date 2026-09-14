// 🐹️ Go side of the ticket id scheme case.
package adapter

import (
	"encoding/json"
	"fmt"

	tickets "github.com/usalu/semio/repo/tickets"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type idVectors struct {
	RepoMetaDir string     `json:"repoMetaDir"`
	IDs         []string   `json:"ids"`
	InvalidIDs  []string   `json:"invalidIds"`
	Titles      []string   `json:"titles"`
	EmojiTitles [][]string `json:"emojiTitles"`
}

func loadIDVectors(ctx *host.Context) (idVectors, error) {
	data, err := ctx.FixtureBytes("local://🪪️id-vectors.json")
	if err != nil {
		return idVectors{}, err
	}
	var vectors idVectors
	err = json.Unmarshal(data, &vectors)
	return vectors, err
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func bothSpellingsNameTheSameIdentity(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, raw := range vectors.IDs {
		id, err := tickets.ParseTicketID(raw)
		if err != nil {
			return host.Outcome{}, err
		}
		again, err := tickets.ParseTicketID(id.RelPath())
		if err != nil {
			return host.Outcome{}, err
		}
		stable := "no"
		if again == id {
			stable = "yes"
		}
		parent, _ := id.ParentSlug()
		projection[raw] = map[string]any{
			"id":      id.ID(),
			"relPath": id.RelPath(),
			"uri":     id.URI(),
			"stable":  stable,
			"parent":  parent,
		}
	}
	return host.Outcome{Projection: projection}, nil
}

func aMalformedIDIsRefused(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, raw := range vectors.InvalidIDs {
		id, err := tickets.ParseTicketID(raw)
		if err != nil {
			projection[raw] = fmt.Sprintf("refused:%s", tickets.TicketErrorClass(err))
			continue
		}
		projection[raw] = fmt.Sprintf("accepted:%s", id.ID())
	}
	return host.Outcome{Projection: projection}, nil
}

func aTitleBecomesOneSlug(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, title := range vectors.Titles {
		first, err := tickets.TicketSlugFromTitle(title)
		if err != nil {
			first = fmt.Sprintf("refused:%s", tickets.TicketErrorClass(err))
		}
		second, err := tickets.TicketSlugFromTitle(first)
		if err != nil {
			second = fmt.Sprintf("refused:%s", tickets.TicketErrorClass(err))
		}
		idempotent := second
		if first == second {
			idempotent = "yes"
		}
		projection[title] = map[string]any{"slug": first, "idempotent": idempotent}
	}
	return host.Outcome{Projection: projection}, nil
}

func anEmojiAndTitlePairIsValidatedTogether(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, pair := range vectors.EmojiTitles {
		emoji, title := "", ""
		if len(pair) > 0 {
			emoji = pair[0]
		}
		if len(pair) > 1 {
			title = pair[1]
		}
		slug, err := tickets.ValidateTicketEmojiTitle(emoji, title)
		outcome := fmt.Sprintf("ok:%s", slug)
		if err != nil {
			outcome = fmt.Sprintf("refused:%s", err.Error())
		}
		projection[fmt.Sprintf("%s|%s", emoji, title)] = outcome
	}
	return host.Outcome{Projection: projection}, nil
}

func everyOwnedPathHangsOffTheFolder(ctx *host.Context) (host.Outcome, error) {
	vectors, err := loadIDVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	layout := tickets.NewTicketLayout(vectors.RepoMetaDir)
	projection := map[string]any{"ticketsDir": layout.TicketsDir()}
	for _, raw := range vectors.IDs {
		id, err := tickets.ParseTicketID(raw)
		if err != nil {
			return host.Outcome{}, err
		}
		projection[id.ID()] = map[string]any{
			"folder":            layout.TicketDir(id),
			"document":          layout.DocumentPath(id),
			"importantDir":      layout.ImportantDir(id),
			"importantDocument": layout.ImportantPath(id),
		}
	}
	return host.Outcome{Projection: projection}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("both-spellings-name-the-same-identity", bothSpellingsNameTheSameIdentity).
		Subject("a-malformed-id-is-refused", aMalformedIDIsRefused).
		Subject("a-title-becomes-one-slug", aTitleBecomesOneSlug).
		Subject("an-emoji-and-title-pair-is-validated-together", anEmojiAndTitlePairIsValidatedTogether).
		Subject("every-owned-path-hangs-off-the-folder", everyOwnedPathHangsOffTheFolder)
}

// endregion 🔖️Registration
