// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package eventstore

import (
	"bytes"
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// #region 🛟️Recovery

func TestStagedAppendRecovery(t *testing.T) {
	for _, test := range []struct {
		name        string
		written     func([]byte) []byte
		wantEvents  int
		wantChanged bool
	}{
		{name: "stage only", written: func([]byte) []byte { return nil }, wantEvents: 1},
		{name: "partial batch", written: func(batch []byte) []byte { return batch[:len(batch)/2] }, wantEvents: 1},
		{name: "complete batch", written: func(batch []byte) []byte { return batch }, wantEvents: 2, wantChanged: true},
	} {
		t.Run(test.name, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "events.jsonl")
			store := Store{Path: path}
			if _, err := store.Append(context.Background(), []Input{{ID: "first", Kind: "recorded", Data: "first"}}, nil); err != nil {
				t.Fatal(err)
			}
			before, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			batch := encodedEvent(t, Event{Schema: Schema, Sequence: 2, ID: "second", Kind: "recorded", Data: json.RawMessage(`"second"`)})
			staged := stage{
				Schema:        stageSchema,
				PriorExists:   true,
				PriorSize:     int64(len(before)),
				PriorChecksum: digest(before),
				BatchSize:     len(batch),
				BatchChecksum: digest(batch),
			}
			stageData, err := json.Marshal(staged)
			if err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(store.stagePath(), stageData, 0o600); err != nil {
				t.Fatal(err)
			}
			written := test.written(batch)
			if len(written) > 0 {
				file, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0o644)
				if err != nil {
					t.Fatal(err)
				}
				if _, err := file.Write(written); err != nil {
					file.Close()
					t.Fatal(err)
				}
				if err := file.Close(); err != nil {
					t.Fatal(err)
				}
			}
			events, err := store.Replay(context.Background(), nil)
			if err != nil {
				t.Fatal(err)
			}
			if len(events) != test.wantEvents {
				t.Fatalf("events = %d, want %d", len(events), test.wantEvents)
			}
			after, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			if bytes.Equal(before, after) != !test.wantChanged {
				t.Fatalf("changed = %t, want %t", !bytes.Equal(before, after), test.wantChanged)
			}
			if _, err := os.Stat(store.stagePath()); !os.IsNotExist(err) {
				t.Fatalf("stage remained: %v", err)
			}
		})
	}
}

func encodedEvent(t *testing.T, event Event) []byte {
	t.Helper()
	event.Checksum = checksum(event)
	var output bytes.Buffer
	if err := json.NewEncoder(&output).Encode(event); err != nil {
		t.Fatal(err)
	}
	return output.Bytes()
}

// #endregion 🛟️Recovery
