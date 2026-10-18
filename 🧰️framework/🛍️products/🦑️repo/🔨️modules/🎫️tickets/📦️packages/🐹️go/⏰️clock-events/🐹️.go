package tickets

import (
	json "encoding/json"
	time "time"

	events "github.com/usalu/semio/repo/events"
)

// #region ⏰️Clock And Events

// ⏰️TicketClock is the clock the lifecycle reads, so a case pins every timestamp.
type TicketClock interface {
	// 📅️Today returns the two-digit year, month and day the folder scheme uses.
	Today() (int, int, int)
	// 🕰️Stamp returns the interaction timestamp.
	Stamp() string
}

// 📌️FixedTicketClock is a clock frozen at one instant.
type FixedTicketClock struct {
	Year      int
	Month     int
	Day       int
	Timestamp string
}

// 🆕️NewFixedTicketClock freezes a clock at a date and a timestamp.
func NewFixedTicketClock(year, month, day int, timestamp string) FixedTicketClock {
	return FixedTicketClock{Year: year, Month: month, Day: day, Timestamp: timestamp}
}

// 📅️Today returns the frozen date.
func (clock FixedTicketClock) Today() (int, int, int) { return clock.Year, clock.Month, clock.Day }

// 🕰️Stamp returns the frozen timestamp.
func (clock FixedTicketClock) Stamp() string { return clock.Timestamp }

// 🕰️SystemTicketClock is the clock of the machine this runs on.
type SystemTicketClock struct{}

// 📅️Today returns today as a two-digit year, month and day.
func (SystemTicketClock) Today() (int, int, int) {
	now := time.Now()
	return now.Year() % 100, int(now.Month()), now.Day()
}

// 🕰️Stamp returns the current timestamp.
func (SystemTicketClock) Stamp() string { return time.Now().Format("2006-01-02 15:04:05") }

// 📡️RecordedTicketEvent is one emitted event: the kind, the source and the payload as JSON text.
type RecordedTicketEvent struct {
	Kind    string `json:"kind"`
	Source  string `json:"source"`
	Payload string `json:"payload"`
}

// 📡️EventSink is where the lifecycle sends its events.
type EventSink interface {
	// 📤️Emit emits one event, its payload already encoded.
	Emit(kind, source, payload string)
}

// 📼️RecordingEventSink keeps every event in emission order.
type RecordingEventSink struct {
	events []RecordedTicketEvent
}

// 🆕️NewRecordingEventSink builds an empty sink.
func NewRecordingEventSink() *RecordingEventSink { return &RecordingEventSink{} }

// 📼️Recorded returns everything emitted so far, in order.
func (sink *RecordingEventSink) Recorded() []RecordedTicketEvent {
	return append([]RecordedTicketEvent{}, sink.events...)
}

// 📤️Emit records one event.
func (sink *RecordingEventSink) Emit(kind, source, payload string) {
	sink.events = append(sink.events, RecordedTicketEvent{Kind: kind, Source: source, Payload: payload})
}

// 🌐️CoordinatorEventSink posts to the repository coordinator through 📡️events.
type CoordinatorEventSink struct{}

// 📤️Emit posts one event to the coordinator.
func (CoordinatorEventSink) Emit(kind, source, payload string) {
	var value interface{}
	if err := json.Unmarshal([]byte(payload), &value); err != nil {
		return
	}
	events.Emit(events.EventKind(kind), source, value)
}

// 🏷️TicketEventSource is the event source every ticket operation carries.
const TicketEventSource = "repo-cli"

// #endregion ⏰️Clock And Events
