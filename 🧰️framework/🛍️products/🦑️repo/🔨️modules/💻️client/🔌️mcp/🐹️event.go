// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// #endregion 🧲️Header

package main

import (
	"bufio"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"sync"
)

// #region 📚️EventSchema

const EventSchema = "semio.mcp.event/1"

type Event struct {
	Schema     string          `json:"schema"`
	Sequence   uint64          `json:"sequence"`
	Kind       string          `json:"kind"`
	Peer       string          `json:"peer"`
	Generation uint64          `json:"generation"`
	RequestID  string          `json:"requestId,omitempty"`
	Payload    json.RawMessage `json:"payload"`
	Previous   string          `json:"previous,omitempty"`
	Hash       string          `json:"hash"`
}

type EventInput struct {
	Kind       string
	Peer       string
	Generation uint64
	RequestID  string
	Payload    json.RawMessage
}

type EventLog struct {
	mu       sync.RWMutex
	events   []Event
	data     []byte
	maxBytes int
	maxCount int
}

func NewEventLog(maxBytes, maxCount int) *EventLog {
	return &EventLog{maxBytes: maxBytes, maxCount: maxCount}
}

func (log *EventLog) Commit(ctx context.Context, inputs ...EventInput) error {
	if len(inputs) == 0 {
		return nil
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	log.mu.Lock()
	defer log.mu.Unlock()
	if log.maxCount > 0 && len(log.events)+len(inputs) > log.maxCount {
		return ErrLimit
	}
	previous := ""
	if len(log.events) > 0 {
		previous = log.events[len(log.events)-1].Hash
	}
	stagedEvents := make([]Event, 0, len(inputs))
	var staged bytes.Buffer
	for index, input := range inputs {
		if err := ctx.Err(); err != nil {
			return err
		}
		if input.Kind == "" || !json.Valid(input.Payload) {
			return errors.New("mcp: invalid event")
		}
		event := Event{Schema: EventSchema, Sequence: uint64(len(log.events) + index + 1), Kind: input.Kind, Peer: input.Peer, Generation: input.Generation, RequestID: input.RequestID, Payload: append(json.RawMessage(nil), input.Payload...), Previous: previous}
		digest, err := eventDigest(event)
		if err != nil {
			return err
		}
		event.Hash = digest
		encoded, err := json.Marshal(event)
		if err != nil {
			return err
		}
		staged.Write(encoded)
		staged.WriteByte('\n')
		stagedEvents = append(stagedEvents, event)
		previous = digest
	}
	if log.maxBytes > 0 && len(log.data)+staged.Len() > log.maxBytes {
		return ErrLimit
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	log.events = append(log.events, stagedEvents...)
	log.data = append(log.data, staged.Bytes()...)
	return nil
}

func (log *EventLog) Snapshot() []byte {
	log.mu.RLock()
	defer log.mu.RUnlock()
	return append([]byte(nil), log.data...)
}

func (log *EventLog) Events() []Event {
	log.mu.RLock()
	defer log.mu.RUnlock()
	result := make([]Event, len(log.events))
	copy(result, log.events)
	return result
}

func ReplayEvents(ctx context.Context, data []byte, maxBytes, maxCount int) ([]Event, error) {
	if maxBytes > 0 && len(data) > maxBytes {
		return nil, ErrLimit
	}
	scanner := bufio.NewScanner(bytes.NewReader(data))
	limit := maxBytes
	if limit <= 0 {
		limit = 64 << 20
	}
	scanner.Buffer(make([]byte, 64*1024), limit)
	result := make([]Event, 0)
	previous := ""
	for scanner.Scan() {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if maxCount > 0 && len(result) == maxCount {
			return nil, ErrLimit
		}
		var event Event
		if err := decodeExact(scanner.Bytes(), &event); err != nil {
			return nil, fmt.Errorf("mcp: corrupt event %d: %w", len(result)+1, err)
		}
		if event.Schema != EventSchema || event.Sequence != uint64(len(result)+1) || event.Previous != previous || event.Kind == "" || !json.Valid(event.Payload) {
			return nil, fmt.Errorf("mcp: corrupt event %d", len(result)+1)
		}
		digest, err := eventDigest(event)
		if err != nil || digest != event.Hash {
			return nil, fmt.Errorf("mcp: corrupt event %d", len(result)+1)
		}
		result = append(result, event)
		previous = event.Hash
	}
	if err := scanner.Err(); err != nil {
		if errors.Is(err, bufio.ErrTooLong) {
			return nil, ErrLimit
		}
		return nil, err
	}
	return result, nil
}

func eventDigest(event Event) (string, error) {
	event.Hash = ""
	encoded, err := json.Marshal(event)
	if err != nil {
		return "", err
	}
	digest := sha256.Sum256(encoded)
	return hex.EncodeToString(digest[:]), nil
}

func cloneRaw(raw json.RawMessage) json.RawMessage {
	return append(json.RawMessage(nil), raw...)
}

func readOneJSON(decoder *json.Decoder, target any) error {
	if err := decoder.Decode(target); err != nil {
		return err
	}
	var extra any
	if err := decoder.Decode(&extra); !errors.Is(err, io.EOF) {
		if err == nil {
			return errors.New("mcp: trailing JSON value")
		}
		return err
	}
	return nil
}

// #endregion 📚️EventSchema
