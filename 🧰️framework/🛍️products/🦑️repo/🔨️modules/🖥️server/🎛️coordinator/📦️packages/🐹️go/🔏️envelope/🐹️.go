// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔏️envelope carries the canonical event encoding: checksums, payload validation and canonical JSON.

// #endregion 🧲️Header

package coordinator

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
)

// #region 🔏️Envelope

// 🔏️EventChecksum is the SHA-256 identity of a record over its NUL-separated header and canonical payload.
func EventChecksum(event EventEnvelope) string {
	return eventChecksum(event)
}

// 🧾️EncodeEvent renders one envelope as its canonical line, terminated by the record separator.
func EncodeEvent(event EventEnvelope) ([]byte, error) {
	return encodeEvent(event)
}

// ♻️CanonicalPayload normalises a payload the way an append persists it.
func CanonicalPayload(payload []byte) (json.RawMessage, error) {
	return canonicalJSONPayload(payload, DefaultStoreLimits().MaxJSONDepth)
}

func encodeEventBatch(events []EventEnvelope) ([]byte, error) {
	var output bytes.Buffer
	for _, event := range events {
		line, err := encodeEvent(event)
		if err != nil {
			return nil, err
		}
		output.Write(line)
	}
	return output.Bytes(), nil
}

func encodeEvent(event EventEnvelope) ([]byte, error) {
	encoded, err := json.Marshal(event)
	if err != nil {
		return nil, err
	}
	return append(encoded, '\n'), nil
}

func eventChecksum(event EventEnvelope) string {
	hash := sha256.New()
	fmt.Fprintf(hash, "%s\x00%d\x00%s\x00%d\x00%s\x00", event.Stream, event.Sequence, event.ID, event.Generation, event.Type)
	hash.Write(event.Payload)
	return hex.EncodeToString(hash.Sum(nil))
}

func bytesChecksum(data []byte) string {
	value := sha256.Sum256(data)
	return hex.EncodeToString(value[:])
}

func validEventScalar(value string) bool {
	return value != "" && !bytes.ContainsRune([]byte(value), '\x00')
}

func validateJSONPayload(payload []byte, maximumDepth int) error {
	if len(payload) == 0 || !json.Valid(payload) {
		return errors.New("event payload must be valid JSON")
	}
	depth := 0
	inString := false
	escaped := false
	for _, value := range payload {
		if inString {
			if escaped {
				escaped = false
			} else if value == '\\' {
				escaped = true
			} else if value == '"' {
				inString = false
			}
			continue
		}
		if value == '"' {
			inString = true
			continue
		}
		if value == '{' || value == '[' {
			depth++
			if depth > maximumDepth {
				return fmt.Errorf("%w: JSON depth %d > %d", ErrStoreLimit, depth, maximumDepth)
			}
		} else if value == '}' || value == ']' {
			depth--
		}
	}
	return nil
}

func canonicalJSONPayload(payload []byte, maximumDepth int) (json.RawMessage, error) {
	if err := validateJSONPayload(payload, maximumDepth); err != nil {
		return nil, err
	}
	decoder := json.NewDecoder(bytes.NewReader(payload))
	decoder.UseNumber()
	var value any
	if err := decoder.Decode(&value); err != nil {
		return nil, err
	}
	canonical, err := json.Marshal(value)
	if err != nil {
		return nil, err
	}
	if err := validateJSONPayload(canonical, maximumDepth); err != nil {
		return nil, err
	}
	return canonical, nil
}

func reportStoreProgress(progress func(StoreProgress), value StoreProgress) {
	if progress != nil {
		progress(value)
	}
}

// #endregion 🔏️Envelope
