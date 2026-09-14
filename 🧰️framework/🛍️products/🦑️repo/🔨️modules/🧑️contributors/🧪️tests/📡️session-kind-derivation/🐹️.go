// 🐹️ Go side of the session kind derivation case.
package adapter

import (
	"encoding/json"
	"fmt"

	contributors "github.com/usalu/semio/repo/contributors"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type sessionVector struct {
	Year       int64             `json:"year"`
	Month      int64             `json:"month"`
	Day        int64             `json:"day"`
	UUID       string            `json:"uuid"`
	AgeSeconds *int64            `json:"ageSeconds"`
	Files      map[string]string `json:"files"`
}

type sessionVectorFile struct {
	Schema   string          `json:"schema"`
	Sessions []sessionVector `json:"sessions"`
}

func loadSessionSource(ctx *host.Context) (*contributors.MemorySessionSource, error) {
	data, err := ctx.FixtureBytes("shared://📡️session-vectors.json")
	if err != nil {
		return nil, err
	}
	var file sessionVectorFile
	if err := json.Unmarshal(data, &file); err != nil {
		return nil, err
	}
	entries := make([]contributors.MemorySessionEntry, 0, len(file.Sessions))
	for _, vector := range file.Sessions {
		entries = append(entries, contributors.MemorySessionEntry{
			Key:     contributors.SessionKey{Year: vector.Year, Month: vector.Month, Day: vector.Day, UUID: vector.UUID},
			Session: contributors.MemorySession{Files: vector.Files, AgeSeconds: vector.AgeSeconds},
		})
	}
	return contributors.NewMemorySessionSource(entries), nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyBranchOfTheDerivationAgrees(ctx *host.Context) (host.Outcome, error) {
	source, err := loadSessionSource(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	sessions := contributors.ListSessions(source)
	records := make([]string, 0, len(sessions))
	identifiers := make([]string, 0, len(sessions))
	for _, session := range sessions {
		records = append(records, fmt.Sprintf("%s|%s|%s|%s|%s|%02d-%02d-%02d",
			session.UUID, session.Kind, session.Client, session.StartedAt, session.Checkpoint, session.Year, session.Month, session.Day))
		identifiers = append(identifiers, fmt.Sprintf("%s=%s|%s", session.UUID, session.ID(), session.URI()))
	}
	return host.Outcome{Projection: map[string]any{
		"records":     records,
		"identifiers": identifiers,
		"found":       len(sessions),
	}}, nil
}

func theKindEmojiComesFromTheSharedVocabulary(ctx *host.Context) (host.Outcome, error) {
	source, err := loadSessionSource(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	sessions := contributors.ListSessions(source)
	emojis := make([]string, 0, len(sessions)+1)
	for _, session := range sessions {
		kind := contributors.SessionKindInterrupted
		switch session.Kind {
		case "running":
			kind = contributors.SessionKindRunning
		case "completed":
			kind = contributors.SessionKindCompleted
		}
		emojis = append(emojis, fmt.Sprintf("%s=%s", session.Kind, contributors.SessionKindEmoji(kind)))
	}
	emojis = append(emojis, fmt.Sprintf("none=%s", contributors.SessionKindEmoji("")))
	return host.Outcome{Projection: map[string]any{"emojis": emojis}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-branch-of-the-derivation-agrees", everyBranchOfTheDerivationAgrees).
		Subject("the-kind-emoji-comes-from-the-shared-vocabulary", theKindEmojiComesFromTheSharedVocabulary)
}

// endregion 🔖️Registration
