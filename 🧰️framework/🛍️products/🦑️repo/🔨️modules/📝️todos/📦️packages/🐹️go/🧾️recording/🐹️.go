package todos

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
)

// #region 🧾️Recording

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

// #endregion 🧾️Recording
