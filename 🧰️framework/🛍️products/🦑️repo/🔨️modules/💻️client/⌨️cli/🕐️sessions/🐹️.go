// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Session model and streaming.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// 🪪️SessionKind represents the status of a session.
type SessionKind string

const (
	SessionKindRunning     SessionKind = "running"
	SessionKindCompleted   SessionKind = "completed"
	SessionKindInterrupted SessionKind = "interrupted"
)

// 💿️Session holds the data fields for a session record.
type Session struct {
	UUID       string      `json:"uuid"`
	Year       int         `json:"year"`
	Month      int         `json:"month"`
	Day        int         `json:"day"`
	Checkpoint string      `json:"checkpoint,omitempty"`
	Kind       SessionKind `json:"kind"`
	Client     string      `json:"client,omitempty"`
	LLM        string      `json:"llm,omitempty"`
	StartedAt  string      `json:"startedAt,omitempty"`
	EndedAt    string      `json:"endedAt,omitempty"`
}

// 🔷️GetID returns the repo ID for a session.
// ✔️GetID MUST use the checkpoint ID as parent for the session ID. Falls back to date document if checkpoint is unknown.
func (s *Session) GetID() string {
	var parentId string
	if s.Checkpoint != "" {
		parentId = GetArtifactID("checkpoint", map[string]interface{}{"sha": s.Checkpoint})
	} else {
		parentId = GetArtifactID("day", map[string]interface{}{
			"parentId": GetArtifactID("month", map[string]interface{}{
				"parentId": GetArtifactID("year", map[string]interface{}{
					"parentId": "",
					"yy":       fmt.Sprintf("%02d", s.Year),
				}),
				"mm": fmt.Sprintf("%02d", s.Month),
			}),
			"dd": fmt.Sprintf("%02d", s.Day),
		})
	}
	return GetArtifactID("session", map[string]interface{}{
		"parentId": parentId,
		"uuid":     s.UUID,
	})
}

// 🔗️GetURI returns the repo URI for a session.
func (s *Session) GetURI() string {
	return GetArtifactURI("session", map[string]interface{}{
		"uuid": s.UUID,
	})
}

// 😀️SessionKindEmoji returns the emoji for the given session kind.
func SessionKindEmoji(kind SessionKind) string {
	switch kind {
	case SessionKindRunning:
		return EmojiSessionRunning
	case SessionKindCompleted:
		return EmojiSessionCompleted
	case SessionKindInterrupted:
		return EmojiSessionInterrupted
	}
	return EmojiSession
}

// 📡️DeriveSessionKind determines the session kind by examining its event files.
func DeriveSessionKind(sessionDir string) SessionKind {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil {
			for _, entry := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(entry.Event, &evt); err != nil {
					continue
				}
				if kind, ok := evt["kind"].(string); ok && kind == string(HookAgentEnded) {
					return SessionKindCompleted
				}
			}
			if info, statErr := os.Stat(metaPath); statErr == nil && time.Since(info.ModTime()) < 30*time.Minute {
				return SessionKindRunning
			}
			if len(meta.Events) > 0 {
				return SessionKindInterrupted
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil {
		return SessionKindInterrupted
	}
	for _, e := range entries {
		if strings.Contains(e.Name(), "agent-ended") {
			return SessionKindCompleted
		}
	}
	if len(entries) == 0 {
		return SessionKindInterrupted
	}

	var latestTime time.Time
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		info, err := e.Info()
		if err != nil {
			continue
		}
		if info.ModTime().After(latestTime) {
			latestTime = info.ModTime()
		}
	}
	if !latestTime.IsZero() && time.Since(latestTime) < 30*time.Minute {
		return SessionKindRunning
	}
	return SessionKindInterrupted
}

// 🧲️ExtractSessionClient reads the client from session event files.
func ExtractSessionClient(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil && meta.Client != "" {
			return meta.Client
		}
		if err := json.Unmarshal(data, &meta); err == nil {
			for _, item := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(item.Event, &evt); err == nil {
					if client, ok := evt["client"].(string); ok && client != "" {
						return client
					}
				}
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil || len(entries) == 0 {
		return ""
	}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") || e.Name() == "session.json" {
			continue
		}
		data, err := os.ReadFile(filepath.Join(sessionDir, e.Name()))
		if err != nil {
			continue
		}
		var entry struct {
			Event struct {
				Client string `json:"client"`
			} `json:"event"`
		}
		if err := json.Unmarshal(data, &entry); err == nil && entry.Event.Client != "" {
			return entry.Event.Client
		}
	}
	return ""
}

// 📄️ExtractSessionSecond reads the earliest second from session event files.
func ExtractSessionSecond(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil && meta.Second != "" {
			return meta.Second
		}
		if err := json.Unmarshal(data, &meta); err == nil {
			var earliest string
			for _, item := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(item.Event, &evt); err == nil {
					if second, ok := evt["second"].(string); ok && second != "" {
						if earliest == "" || second < earliest {
							earliest = second
						}
					}
				}
			}
			if earliest != "" {
				return earliest
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil || len(entries) == 0 {
		return ""
	}
	var earliest string
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") || e.Name() == "session.json" {
			continue
		}
		data, err := os.ReadFile(filepath.Join(sessionDir, e.Name()))
		if err != nil {
			continue
		}
		var entry struct {
			Event struct {
				Second string `json:"second"`
			} `json:"event"`
		}
		if err := json.Unmarshal(data, &entry); err == nil && entry.Event.Second != "" {
			if earliest == "" || entry.Event.Second < earliest {
				earliest = entry.Event.Second
			}
		}
	}
	return earliest
}

// 💾️ExtractSessionCheckpoint reads the checkpoint SHA from the session.json file.
// 💾️ExtractSessionCheckpoint MUST return the checkpoint SHA stored in session.json, or empty string if not present.
func ExtractSessionCheckpoint(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil {
			return meta.Checkpoint
		}
	}
	return ""
}

// 📂️StreamSessions streams all sessions found in the events directory.
func StreamSessions(ctx context.Context, out chan<- Session, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	agentEventsDir := filepath.Join(GetRepoMetaDir(), "⚡️cache", "🤖️generated")
	yearDirs, err := os.ReadDir(agentEventsDir)
	if err != nil {
		return nil
	}
	for _, yd := range yearDirs {
		if !yd.IsDir() {
			continue
		}
		yy, err := parseDatedDir(yd.Name(), EmojiYear)
		if err != nil {
			continue
		}
		monthDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name()))
		if err != nil {
			continue
		}
		for _, md := range monthDirs {
			if !md.IsDir() {
				continue
			}
			mm, err := parseDatedDir(md.Name(), EmojiMonth)
			if err != nil {
				continue
			}
			dayDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name(), md.Name()))
			if err != nil {
				continue
			}
			for _, dd := range dayDirs {
				if !dd.IsDir() {
					continue
				}
				dayNum, err := parseDatedDir(dd.Name(), EmojiDay)
				if err != nil {
					continue
				}
				sessionDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name(), md.Name(), dd.Name()))
				if err != nil {
					continue
				}
				for _, sd := range sessionDirs {
					if !sd.IsDir() {
						continue
					}
					sessionDir := filepath.Join(agentEventsDir, yd.Name(), md.Name(), dd.Name(), sd.Name())
					kind := DeriveSessionKind(sessionDir)
					client := ExtractSessionClient(sessionDir)
					startedAt := ExtractSessionSecond(sessionDir)
					checkpoint := ExtractSessionCheckpoint(sessionDir)
					s := Session{
						UUID:       sd.Name(),
						Year:       yy,
						Month:      mm,
						Day:        dayNum,
						Checkpoint: checkpoint,
						Kind:       kind,
						Client:     client,
						StartedAt:  startedAt,
					}
					if !matchesFilter(s.UUID, options) && !matchesFilter(string(s.Kind), options) && !matchesFilter(s.Client, options) {
						continue
					}
					queryMatches, queryErr := matchesQuery(ctx, s.UUID+" "+string(s.Kind)+" "+s.Client, options)
					if queryErr != nil {
						return queryErr
					}
					if !queryMatches {
						continue
					}
					select {
					case <-ctx.Done():
						return ctx.Err()
					case out <- s:
					}
				}
			}
		}
	}
	return nil
}
