// 🐹️ Go side of the canonical-envelope case. Decodes the golden record, re-encodes it and projects
// the header, the canonical payload and the checksum the store would persist.
package adapter

import (
	"encoding/json"
	"strings"

	coordinator "github.com/usalu/semio/repo/coordinator"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

func goldenEvent(ctx *host.Context) (coordinator.EventEnvelope, string, error) {
	var event coordinator.EventEnvelope
	data, err := ctx.FixtureBytes("shared://📜️g3-event-log.jsonl")
	if err != nil {
		return event, "", err
	}
	line := strings.TrimRight(string(data), "\n")
	if err := json.Unmarshal([]byte(line), &event); err != nil {
		return event, "", err
	}
	return event, line, nil
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func goldenLineIsCanonical(ctx *host.Context) (host.Outcome, error) {
	event, line, err := goldenEvent(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	encoded, err := coordinator.EncodeEvent(event)
	if err != nil {
		return host.Outcome{}, err
	}
	schema, err := ctx.FixtureBytes("shared://🧬️g3-event-schema.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var declared map[string]any
	if err := json.Unmarshal(schema, &declared); err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{
		"line":       strings.TrimRight(string(encoded), "\n"),
		"reproduced": strings.TrimRight(string(encoded), "\n") == line,
		"fields":     declared["fields"],
		"formula":    declared["checksum"],
		"encoding":   declared["encoding"],
		"schema":     declared["schema"],
	}}, nil
}

func checksumIsSha256OfThePreimage(ctx *host.Context) (host.Outcome, error) {
	event, _, err := goldenEvent(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	payload, err := coordinator.CanonicalPayload(event.Payload)
	if err != nil {
		return host.Outcome{}, err
	}
	event.Payload = payload
	return host.Outcome{Projection: map[string]any{
		"stream":      event.Stream,
		"sequence":    event.Sequence,
		"id":          event.ID,
		"generation":  event.Generation,
		"type":        event.Type,
		"payloadJson": string(payload),
		"checksum":    coordinator.EventChecksum(event),
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("golden-line-is-canonical", goldenLineIsCanonical).
		Subject("checksum-is-sha256-of-the-preimage", checksumIsSha256OfThePreimage)
}

// endregion 🔖️Registration
