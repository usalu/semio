// 🐹️ Go side of the breach cache envelope case. Both halves are standards — gzip and SHA-256 — so
// the subject writes what its own package produces and the reference judges it.
package adapter

import (
	"encoding/binary"
	"encoding/json"
	"fmt"

	statutes "github.com/usalu/semio/repo/statutes"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

// 🧫️vectors is the committed vector file of this case.
type vectors struct {
	Digests  []string `json:"digests"`
	Payloads []string `json:"payloads"`
	Envelope string   `json:"envelope"`
}

// 🧫️readVectors reads the committed vector file.
func readVectors(ctx *host.Context) (vectors, error) {
	raw, err := ctx.FixtureBytes("shared://🗜️breach-cache-envelope/🔣️vectors.json")
	if err != nil {
		return vectors{}, err
	}
	var file vectors
	if err := json.Unmarshal(raw, &file); err != nil {
		return vectors{}, err
	}
	return file, nil
}

// 📦️frame concatenates every member length-prefixed, so one raw stream carries the whole vector set
// for the reference to inflate back. Big-endian counts, exactly as the Rust subject writes them.
func frame(members [][]byte) []byte {
	out := make([]byte, 0, 4+len(members)*8)
	out = binary.BigEndian.AppendUint32(out, uint32(len(members)))
	for _, member := range members {
		out = binary.BigEndian.AppendUint32(out, uint32(len(member)))
		out = append(out, member...)
	}
	return out
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func theDigestIsSHA256(ctx *host.Context) (host.Outcome, error) {
	file, err := readVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	digests := make([]string, 0, len(file.Digests))
	for _, vector := range file.Digests {
		digests = append(digests, statutes.SHA256Hex([]byte(vector)))
	}
	envelope, err := statutes.ParseBreachCache(file.Envelope)
	if err != nil {
		return host.Outcome{}, err
	}
	encoded, err := statutes.EncodeBreachCacheJSON(envelope)
	if err != nil {
		return host.Outcome{}, err
	}
	if encoded != file.Envelope {
		return host.Outcome{}, fmt.Errorf("the envelope did not round trip through its canonical encoding: %s", encoded)
	}
	digest, err := statutes.BreachCacheDigest(envelope)
	if err != nil {
		return host.Outcome{}, err
	}
	if digest != statutes.SHA256Hex([]byte(file.Envelope)) {
		return host.Outcome{}, fmt.Errorf("the envelope digest is not the digest of its canonical encoding")
	}
	return host.Outcome{Projection: map[string]any{"digests": digests, "envelopeDigest": statutes.SHA256Hex([]byte(file.Envelope))}}, nil
}

func aMemberInflatesAnywhere(ctx *host.Context) (host.Outcome, error) {
	file, err := readVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	members := make([][]byte, 0, len(file.Payloads))
	recovered := make([]string, 0, len(file.Payloads))
	for _, payload := range file.Payloads {
		member := statutes.GzipEncode([]byte(payload))
		back, err := statutes.GzipDecode(member)
		if err != nil {
			return host.Outcome{}, err
		}
		if string(back) != payload {
			return host.Outcome{}, fmt.Errorf("a member did not inflate back to its payload: %s", payload)
		}
		recovered = append(recovered, statutes.SHA256Hex(back))
		members = append(members, member)
	}
	return host.Outcome{
		Raw:        frame(members),
		Projection: map[string]any{"recovered": recovered, "count": fmt.Sprintf("%d", len(file.Payloads))},
	}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-digest-is-sha-256", theDigestIsSHA256).
		Subject("a-member-inflates-anywhere", aMemberInflatesAnywhere)
}

// endregion 🔖️Registration
