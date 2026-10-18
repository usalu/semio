// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔭️session_source carries the session source seam: memory and filesystem sources, derivation, listing and search.

// #endregion 🧲️Header

package contributors

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔭️SessionSource

// 🔑️SessionKey states where one session sits.
type SessionKey struct {
	Year  int64  `json:"year"`
	Month int64  `json:"month"`
	Day   int64  `json:"day"`
	UUID  string `json:"uuid"`
}

// 📦️MemorySession holds the files and age of one in-memory session.
type MemorySession struct {
	Files      map[string]string `json:"files"`
	AgeSeconds *int64            `json:"ageSeconds"`
}

// 📥️MemorySessionEntry pairs a key with its in-memory session.
type MemorySessionEntry struct {
	Key     SessionKey
	Session MemorySession
}

// 📂️SessionSource is a directory of session event files addressed by date and identifier.
type SessionSource interface {
	// 📋️Sessions returns every key the source carries, in ascending order.
	Sessions() []SessionKey
	// 📄️Meta returns the session.json document of one session.
	Meta(key SessionKey) (string, bool)
	// 📁️Entries returns the event file names of one session, in directory order.
	Entries(key SessionKey) []string
	// 📄️Entry returns one event file of one session.
	Entry(key SessionKey, name string) (string, bool)
	// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
	AgeSeconds(key SessionKey) (int64, bool)
}

// 🧠️MemorySessionSource is the in-memory source: a session is a name, a set of files and an age.
type MemorySessionSource struct {
	keys     []SessionKey
	sessions map[SessionKey]MemorySession
}

// 🌱️NewMemorySessionSource seeds a source with sessions, ordered by key.
func NewMemorySessionSource(entries []MemorySessionEntry) *MemorySessionSource {
	source := &MemorySessionSource{sessions: map[SessionKey]MemorySession{}}
	for _, entry := range entries {
		if _, seen := source.sessions[entry.Key]; !seen {
			source.keys = append(source.keys, entry.Key)
		}
		source.sessions[entry.Key] = entry.Session
	}
	sort.SliceStable(source.keys, func(left, right int) bool { return lessSessionKey(source.keys[left], source.keys[right]) })
	return source
}

// 🔢️lessSessionKey orders keys by year, month, day and then identifier.
func lessSessionKey(left, right SessionKey) bool {
	if left.Year != right.Year {
		return left.Year < right.Year
	}
	if left.Month != right.Month {
		return left.Month < right.Month
	}
	if left.Day != right.Day {
		return left.Day < right.Day
	}
	return left.UUID < right.UUID
}

// 📋️Sessions returns every seeded key in ascending order.
func (source *MemorySessionSource) Sessions() []SessionKey {
	return append([]SessionKey{}, source.keys...)
}

// 📄️Meta returns the session.json document of one session.
func (source *MemorySessionSource) Meta(key SessionKey) (string, bool) {
	return source.Entry(key, "session.json")
}

// 📁️Entries returns the event file names of one session, in name order.
func (source *MemorySessionSource) Entries(key SessionKey) []string {
	session, found := source.sessions[key]
	if !found {
		return nil
	}
	names := make([]string, 0, len(session.Files))
	for name := range session.Files {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// 📄️Entry returns one event file of one session.
func (source *MemorySessionSource) Entry(key SessionKey, name string) (string, bool) {
	session, found := source.sessions[key]
	if !found {
		return "", false
	}
	content, found := session.Files[name]
	return content, found
}

// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
func (source *MemorySessionSource) AgeSeconds(key SessionKey) (int64, bool) {
	session, found := source.sessions[key]
	if !found || session.AgeSeconds == nil {
		return 0, false
	}
	return *session.AgeSeconds, true
}

// 💽️FsSessionSource is the source over a real agent event directory.
type FsSessionSource struct {
	root  string
	clock Clock
}

// 🆕️NewFsSessionSource builds the source of an agent event directory.
func NewFsSessionSource(root string, clock Clock) *FsSessionSource {
	if clock == nil {
		clock = SystemClock{}
	}
	return &FsSessionSource{root: root, clock: clock}
}

// 📁️directory returns the absolute directory of one session.
func (source *FsSessionSource) directory(key SessionKey) string {
	return filepath.Join(source.root, fmt.Sprintf("%d", key.Year), fmt.Sprintf("%d", key.Month), fmt.Sprintf("%d", key.Day), key.UUID)
}

// 📋️Sessions walks the year, month, day and session directories of the source.
func (source *FsSessionSource) Sessions() []SessionKey {
	var keys []SessionKey
	years, err := os.ReadDir(source.root)
	if err != nil {
		return nil
	}
	for _, year := range years {
		yearNumber, err := strconv.ParseInt(year.Name(), 10, 64)
		if !year.IsDir() || err != nil {
			continue
		}
		months, err := os.ReadDir(filepath.Join(source.root, year.Name()))
		if err != nil {
			continue
		}
		for _, month := range months {
			monthNumber, err := strconv.ParseInt(month.Name(), 10, 64)
			if !month.IsDir() || err != nil {
				continue
			}
			days, err := os.ReadDir(filepath.Join(source.root, year.Name(), month.Name()))
			if err != nil {
				continue
			}
			for _, day := range days {
				dayNumber, err := strconv.ParseInt(day.Name(), 10, 64)
				if !day.IsDir() || err != nil {
					continue
				}
				sessions, err := os.ReadDir(filepath.Join(source.root, year.Name(), month.Name(), day.Name()))
				if err != nil {
					continue
				}
				for _, session := range sessions {
					if !session.IsDir() {
						continue
					}
					keys = append(keys, SessionKey{Year: yearNumber, Month: monthNumber, Day: dayNumber, UUID: session.Name()})
				}
			}
		}
	}
	sort.SliceStable(keys, func(left, right int) bool { return lessSessionKey(keys[left], keys[right]) })
	return keys
}

// 📄️Meta reads the session.json document of one session.
func (source *FsSessionSource) Meta(key SessionKey) (string, bool) {
	return source.Entry(key, "session.json")
}

// 📁️Entries lists the event file names of one session.
func (source *FsSessionSource) Entries(key SessionKey) []string {
	entries, err := os.ReadDir(source.directory(key))
	if err != nil {
		return nil
	}
	names := make([]string, 0, len(entries))
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	sort.Strings(names)
	return names
}

// 📄️Entry reads one event file of one session.
func (source *FsSessionSource) Entry(key SessionKey, name string) (string, bool) {
	data, err := os.ReadFile(filepath.Join(source.directory(key), name))
	if err != nil {
		return "", false
	}
	return string(data), true
}

// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
func (source *FsSessionSource) AgeSeconds(key SessionKey) (int64, bool) {
	entries, err := os.ReadDir(source.directory(key))
	if err != nil || len(entries) == 0 {
		return 0, false
	}
	var latest time.Time
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		if info.ModTime().After(latest) {
			latest = info.ModTime()
		}
	}
	if latest.IsZero() {
		return 0, false
	}
	return source.clock.NowSeconds() - latest.Unix(), true
}

// ⏱️RunningWindowSeconds is how long a session may stay silent before it stops counting as running.
const RunningWindowSeconds int64 = 30 * 60

// 🕵️AgentEndedKind is the event kind whose presence ends a session.
const AgentEndedKind = "agent.ended"

// 📡️DeriveSessionKindFrom decides what became of a session from its own files.
func DeriveSessionKindFrom(source SessionSource, key SessionKey) SessionKind {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if kind, ok := event["kind"].(string); ok && kind == AgentEndedKind {
					return SessionKindCompleted
				}
			}
			if age, aged := source.AgeSeconds(key); aged && age < RunningWindowSeconds {
				return SessionKindRunning
			}
			if len(meta.Events) > 0 {
				return SessionKindInterrupted
			}
		}
	}
	entries := source.Entries(key)
	for _, name := range entries {
		if strings.Contains(name, "agent-ended") {
			return SessionKindCompleted
		}
	}
	if len(entries) == 0 {
		return SessionKindInterrupted
	}
	if age, aged := source.AgeSeconds(key); aged && age < RunningWindowSeconds {
		return SessionKindRunning
	}
	return SessionKindInterrupted
}

// 💻️ExtractSessionClientFrom returns the client of a session by the three-step fallback.
func ExtractSessionClientFrom(source SessionSource, key SessionKey) string {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			if meta.Client != "" {
				return meta.Client
			}
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if client, ok := event["client"].(string); ok && client != "" {
					return client
				}
			}
		}
	}
	for _, name := range source.Entries(key) {
		if name == "session.json" || !strings.HasSuffix(name, ".json") {
			continue
		}
		document, found := source.Entry(key, name)
		if !found {
			continue
		}
		var entry struct {
			Event struct {
				Client string `json:"client"`
			} `json:"event"`
		}
		if err := json.Unmarshal([]byte(document), &entry); err == nil && entry.Event.Client != "" {
			return entry.Event.Client
		}
	}
	return ""
}

// 📄️ExtractSessionSecondFrom returns the earliest second a session recorded.
func ExtractSessionSecondFrom(source SessionSource, key SessionKey) string {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			if meta.Second != "" {
				return meta.Second
			}
			earliest := ""
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if second, ok := event["second"].(string); ok && second != "" && (earliest == "" || second < earliest) {
					earliest = second
				}
			}
			if earliest != "" {
				return earliest
			}
		}
	}
	earliest := ""
	for _, name := range source.Entries(key) {
		if name == "session.json" || !strings.HasSuffix(name, ".json") {
			continue
		}
		document, found := source.Entry(key, name)
		if !found {
			continue
		}
		var entry struct {
			Event struct {
				Second string `json:"second"`
			} `json:"event"`
		}
		if err := json.Unmarshal([]byte(document), &entry); err == nil && entry.Event.Second != "" && (earliest == "" || entry.Event.Second < earliest) {
			earliest = entry.Event.Second
		}
	}
	return earliest
}

// 💾️ExtractSessionCheckpointFrom returns the checkpoint a session recorded in its session.json.
func ExtractSessionCheckpointFrom(source SessionSource, key SessionKey) string {
	document, found := source.Meta(key)
	if !found {
		return ""
	}
	var meta model.SessionMeta
	if err := json.Unmarshal([]byte(document), &meta); err != nil {
		return ""
	}
	return meta.Checkpoint
}

// 📂️ListSessions assembles every session the source carries from its own files.
func ListSessions(source SessionSource) []Session {
	keys := source.Sessions()
	sessions := make([]Session, 0, len(keys))
	for _, key := range keys {
		sessions = append(sessions, Session{
			UUID:       key.UUID,
			Year:       int(key.Year),
			Month:      int(key.Month),
			Day:        int(key.Day),
			Checkpoint: ExtractSessionCheckpointFrom(source, key),
			Kind:       DeriveSessionKindFrom(source, key),
			Client:     ExtractSessionClientFrom(source, key),
			StartedAt:  ExtractSessionSecondFrom(source, key),
		})
	}
	return sessions
}

// 🔎️SearchSessions returns every session whose identifier, kind or client contains the term.
func SearchSessions(source SessionSource, term string) []Session {
	needle := strings.ToLower(term)
	found := make([]Session, 0)
	for _, session := range ListSessions(source) {
		haystack := strings.ToLower(session.UUID + " " + string(session.Kind) + " " + session.Client)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, session)
		}
	}
	return found
}

// 🪪️ID returns the artifact identifier of a session: the checkpoint when it recorded one,
// otherwise the year, month and day it sits under, followed by the session own segment.
func (s Session) ID() string {
	parent := ""
	if s.Checkpoint == "" {
		parent = segment("year", fmt.Sprintf("%02d", s.Year), "")
		parent = segment("month", fmt.Sprintf("%02d", s.Month), parent)
		parent = segment("day", fmt.Sprintf("%02d", s.Day), parent)
	} else {
		parent = segment("checkpoint", s.Checkpoint, "")
	}
	return segment("session", workspace.Flat(s.UUID), parent)
}

// 🔗️URI returns the artifact URI of a session, which carries the session segment alone.
func (s Session) URI() string {
	return "repo://session/" + segment("session", workspace.Flat(s.UUID), "")
}

// 🧱️segment appends one emoji-tagged identifier segment to a parent identifier.
func segment(entity, value, parent string) string {
	return parent + identity.SemanticId{Emoji: identity.Entity(entity), Value: value}.String()
}

// #endregion 🔭️SessionSource
