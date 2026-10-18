// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎁️interactions carries the attributed interactions, the clock seam and the memory emitter.

// #endregion 🧲️Header

package contributors

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	strings "strings"
	time "time"

	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
)

// #region 🎁️Interactions

// 🎁️InteractionOrigin states where an interaction was recorded.
type InteractionOrigin struct {
	SourceKind string `json:"sourceKind"`
	SourceID   string `json:"sourceId"`
	GoalID     string `json:"goalId,omitempty"`
	TicketID   string `json:"ticketId,omitempty"`
}

// ✍️InteractionsAuthor returns the first interaction author that is not blank.
func InteractionsAuthor(interactions []model.Interaction) string {
	for _, interaction := range interactions {
		if interaction.Author != "" {
			return interaction.Author
		}
	}
	return ""
}

// 💾️InteractionsCheckpoint returns the first interaction checkpoint that is not blank.
func InteractionsCheckpoint(interactions []model.Interaction) string {
	for _, interaction := range interactions {
		if interaction.Checkpoint != "" {
			return interaction.Checkpoint
		}
	}
	return ""
}

// 🧑️AttributedInteraction pairs an origin with the alias its interaction resolved to.
type AttributedInteraction struct {
	Origin InteractionOrigin `json:"origin"`
	Alias  string            `json:"alias"`
}

// 🧑️AttributeInteractions resolves the interactions of one origin to contributor aliases.
func AttributeInteractions(store ContributorStore, origin InteractionOrigin, interactions []model.Interaction) []AttributedInteraction {
	attributed := make([]AttributedInteraction, 0, len(interactions))
	for _, interaction := range interactions {
		parsed := ParseGitAuthor(interaction.Author)
		attributed = append(attributed, AttributedInteraction{Origin: origin, Alias: ResolveAuthorToAlias(store, parsed.Name, parsed.Email)})
	}
	return attributed
}

// 🕰️Clock is the instant a decision about elapsed time is made against.
type Clock interface {
	// ⏱️NowSeconds returns seconds since the Unix epoch.
	NowSeconds() int64
}

// 📌️FixedClock is a clock frozen at one instant.
type FixedClock struct {
	Instant int64
}

// ⏱️NowSeconds returns the frozen instant.
func (clock FixedClock) NowSeconds() int64 { return clock.Instant }

// 🕰️SystemClock is the clock of the machine this runs on.
type SystemClock struct{}

// ⏱️NowSeconds returns the current second since the Unix epoch.
func (SystemClock) NowSeconds() int64 { return time.Now().Unix() }

// 🧠️MemoryEmitter keeps kind, source and payload lines instead of reaching a coordinator.
type MemoryEmitter struct {
	envelopes []string
}

// 🆕️NewMemoryEmitter builds an emitter with an empty log.
func NewMemoryEmitter() *MemoryEmitter { return &MemoryEmitter{} }

// 📤️Emit records one envelope with its payload rendered as canonical JSON.
func (emitter *MemoryEmitter) Emit(kind events.EventKind, source string, payload interface{}) {
	emitter.envelopes = append(emitter.envelopes, fmt.Sprintf("%s\t%s\t%s", kind, source, canonicalPayload(payload)))
}

// 📜️Envelopes returns every envelope emitted so far, in order.
func (emitter *MemoryEmitter) Envelopes() []string {
	return append([]string{}, emitter.envelopes...)
}

// 🧮️canonicalPayload renders a payload as compact JSON with object members in ascending key order
// and no HTML escaping, which is the one rendering both implementations agree on.
func canonicalPayload(payload interface{}) string {
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

// #endregion 🎁️Interactions
