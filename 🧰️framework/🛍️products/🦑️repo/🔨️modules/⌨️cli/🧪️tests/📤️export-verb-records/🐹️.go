// 🐹️ Go side of the `export` verb batch case: a frozen repository in, one deterministic event
// batch out, nothing written.
package adapter

import (
	"encoding/json"
	"fmt"
	"sort"
	"strings"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Support

// 📥️ The frozen repository every scenario exports.
func records(ctx *host.Context) (string, error) {
	raw, err := ctx.FixtureBytes("local://🗄️repo-records.json")
	if err != nil {
		return "", err
	}
	return string(raw), nil
}

// 📦️ The batch the verb would append.
func batch(ctx *host.Context) (map[string]interface{}, error) {
	source, err := records(ctx)
	if err != nil {
		return nil, err
	}
	encoded, err := cli.ExportRecords(source)
	if err != nil {
		return nil, err
	}
	var value map[string]interface{}
	if err := json.Unmarshal([]byte(encoded), &value); err != nil {
		return nil, err
	}
	return value, nil
}

// 🔤️ A string member, empty when absent.
func text(value map[string]interface{}, key string) string {
	if member, ok := value[key].(string); ok {
		return member
	}
	return ""
}

// 📜️ A string-array member as a `[]string`, never nil.
func stringsOf(value map[string]interface{}, key string) []string {
	items, _ := value[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		out = append(out, fmt.Sprintf("%v", item))
	}
	return out
}

// 🔣️ Whether every rune of a value is a hexadecimal digit.
func isHex(value string) bool {
	for _, rune_ := range value {
		if !strings.ContainsRune("0123456789abcdefABCDEF", rune_) {
			return false
		}
	}
	return true
}

// endregion 🔖️Support

// region 🔖️Scenarios

func theBatchStatesItsCounts(ctx *host.Context) (host.Outcome, error) {
	value, err := batch(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	digest := text(value, "snapshot")
	if len(digest) != 64 || !isHex(digest) {
		return host.Outcome{}, fmt.Errorf("the snapshot digest is not a sha-256: %s", digest)
	}
	return host.Outcome{Projection: map[string]any{"counts": value["counts"], "digestIsSha256": true}}, nil
}

func everyInputIDIsNamespacedByTheDigest(ctx *host.Context) (host.Outcome, error) {
	value, err := batch(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	digest := text(value, "snapshot")
	ids := stringsOf(value, "inputIds")
	prefix := "snapshot:" + digest + ":"
	known := map[string]bool{"technology": true, "bundle": true, "folder": true, "file": true, "section": true, "definition": true}
	projected := []any{}
	for _, id := range ids {
		if !strings.HasPrefix(id, prefix) {
			return host.Outcome{}, fmt.Errorf("input id %s is not namespaced by the snapshot digest", id)
		}
		rest := strings.TrimPrefix(id, prefix)
		kind := rest
		if index := strings.Index(rest, ":"); index >= 0 {
			kind = rest[:index]
		}
		if !known[kind] {
			return host.Outcome{}, fmt.Errorf("input id %s names the unknown entity kind %q", id, kind)
		}
		projected = append(projected, strings.Replace(id, prefix, "snapshot:<digest>:", 1))
	}
	sorted := append([]string{}, ids...)
	sort.Strings(sorted)
	for index := range ids {
		if sorted[index] != ids[index] {
			return host.Outcome{}, fmt.Errorf("the input ids are not sorted")
		}
	}
	return host.Outcome{Projection: map[string]any{"inputIds": projected}}, nil
}

func theBatchIsDeterministic(ctx *host.Context) (host.Outcome, error) {
	source, err := records(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	first, err := cli.ExportRecords(source)
	if err != nil {
		return host.Outcome{}, err
	}
	second, err := cli.ExportRecords(source)
	if err != nil {
		return host.Outcome{}, err
	}
	if first != second {
		return host.Outcome{}, fmt.Errorf("the export batch is not deterministic")
	}
	return host.Outcome{Projection: map[string]any{"stable": true}}, nil
}

func theDigestAndTheEncodedRecordsAgree(ctx *host.Context) (host.Outcome, error) {
	value, err := batch(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	encoded := []any{}
	for _, record := range stringsOf(value, "records") {
		encoded = append(encoded, record)
	}
	return host.Outcome{Projection: map[string]any{"snapshot": text(value, "snapshot"), "records": encoded}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 📤️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-batch-states-its-counts", theBatchStatesItsCounts).
		Subject("every-input-id-is-namespaced-by-the-digest", everyInputIDIsNamespacedByTheDigest).
		Subject("the-batch-is-deterministic", theBatchIsDeterministic).
		Subject("the-digest-and-the-encoded-records-agree", theDigestAndTheEncodedRecordsAgree)
}

// endregion 🔖️Registration
