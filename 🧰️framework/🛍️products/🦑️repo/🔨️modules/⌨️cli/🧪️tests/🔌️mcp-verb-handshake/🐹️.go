// 🐹️ Go side of the `mcp` verb handshake. It serves the recorded conversation with the very server
// the verb builds and holds the declared vocabulary against the same committed expectation.
package adapter

import (
	"encoding/json"
	"fmt"
	"sort"

	cli "github.com/usalu/semio/repo/cli"
	host "semio.tech/repo/test"
)

// region 🔖️Conversation

// 📥️ The committed conversation every scenario reads.
func conversation(ctx *host.Context) (map[string]interface{}, error) {
	raw, err := ctx.FixtureBytes("local://🤝️handshake.json")
	if err != nil {
		return nil, err
	}
	var document map[string]interface{}
	return document, json.Unmarshal(raw, &document)
}

// 🔤️ A string member, empty when absent.
func text(value map[string]interface{}, key string) string {
	if member, ok := value[key].(string); ok {
		return member
	}
	return ""
}

// 📃️ A string-array member as a `[]string`.
func list(value map[string]interface{}, key string) []string {
	items, _ := value[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		out = append(out, fmt.Sprintf("%v", item))
	}
	return out
}

// ▶️ Runs the stated conversation against the server the `mcp` verb builds.
func runConversation(document map[string]interface{}) ([]map[string]interface{}, error) {
	lines, err := cli.McpVerbConversation(text(document, "profile"), list(document, "requests"))
	if err != nil {
		return nil, err
	}
	responses := []map[string]interface{}{}
	for _, line := range lines {
		var response map[string]interface{}
		if err := json.Unmarshal([]byte(line), &response); err != nil {
			return nil, err
		}
		responses = append(responses, response)
	}
	if len(responses) == 0 {
		return nil, fmt.Errorf("the server answered nothing")
	}
	return responses, nil
}

// 🔎️ The response carrying a given identifier.
func response(responses []map[string]interface{}, id float64) map[string]interface{} {
	for _, candidate := range responses {
		if member, ok := candidate["id"].(float64); ok && member == id {
			return candidate
		}
	}
	return nil
}

// 📛️ The `name` member of every object in an array member of a result.
func names(result map[string]interface{}, key string) []string {
	items, _ := result[key].([]interface{})
	out := make([]string, 0, len(items))
	for _, item := range items {
		entry, _ := item.(map[string]interface{})
		out = append(out, text(entry, "name"))
	}
	return out
}

// ⚖️ Whether two string vectors are equal, in order.
func same(left, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

// 📦️ The `result` member of a response.
func result(response map[string]interface{}) (map[string]interface{}, bool) {
	member, ok := response["result"].(map[string]interface{})
	return member, ok
}

// 🧬️ Every value of the projection as an `any` slice, so the emitted shape matches every adapter.
func anys(values []string) []any {
	out := make([]any, 0, len(values))
	for _, value := range values {
		out = append(out, value)
	}
	return out
}

// endregion 🔖️Conversation

// region 🔖️Scenarios

func theVerbCompletesTheHandshake(ctx *host.Context) (host.Outcome, error) {
	document, err := conversation(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	expect, ok := document["expect"].(map[string]interface{})
	if !ok {
		return host.Outcome{}, fmt.Errorf("the conversation states no expectation")
	}
	responses, err := runConversation(document)
	if err != nil {
		return host.Outcome{}, err
	}

	initialize := response(responses, 1)
	if initialize == nil {
		return host.Outcome{}, fmt.Errorf("no initialize response")
	}
	initialized, ok := result(initialize)
	if !ok {
		return host.Outcome{}, fmt.Errorf("initialize failed")
	}
	if text(initialized, "protocolVersion") != text(expect, "protocolVersion") {
		return host.Outcome{}, fmt.Errorf("protocol version %q != %q", text(initialized, "protocolVersion"), text(expect, "protocolVersion"))
	}
	server, ok := initialized["serverInfo"].(map[string]interface{})
	if !ok {
		return host.Outcome{}, fmt.Errorf("no serverInfo")
	}
	if text(server, "name") != text(expect, "serverName") || text(server, "version") != text(expect, "serverVersion") {
		return host.Outcome{}, fmt.Errorf("serverInfo %v does not match the stated one", server)
	}
	capabilities := []string{}
	if declared, ok := initialized["capabilities"].(map[string]interface{}); ok {
		for name := range declared {
			capabilities = append(capabilities, name)
		}
	}
	sort.Strings(capabilities)
	if !same(capabilities, list(expect, "capabilities")) {
		return host.Outcome{}, fmt.Errorf("capabilities %v != %v", capabilities, list(expect, "capabilities"))
	}

	listed := response(responses, 2)
	if listed == nil {
		return host.Outcome{}, fmt.Errorf("no tools/list response")
	}
	toolsResult, ok := result(listed)
	if !ok {
		return host.Outcome{}, fmt.Errorf("tools/list failed")
	}
	tools := names(toolsResult, "tools")
	if !same(tools, list(expect, "tools")) {
		return host.Outcome{}, fmt.Errorf("tools %v != %v", tools, list(expect, "tools"))
	}

	listedResources := response(responses, 3)
	if listedResources == nil {
		return host.Outcome{}, fmt.Errorf("no resources/list response")
	}
	resourcesResult, ok := result(listedResources)
	if !ok {
		return host.Outcome{}, fmt.Errorf("resources/list failed")
	}
	items, _ := resourcesResult["resources"].([]interface{})
	resources := make([]string, 0, len(items))
	for _, item := range items {
		entry, _ := item.(map[string]interface{})
		resources = append(resources, text(entry, "uri"))
	}
	if !same(resources, list(expect, "resources")) {
		return host.Outcome{}, fmt.Errorf("resources %v != %v", resources, list(expect, "resources"))
	}

	return host.Outcome{Projection: map[string]any{
		"protocolVersion": text(initialized, "protocolVersion"),
		"serverName":      text(server, "name"),
		"capabilities":    anys(capabilities),
		"tools":           anys(tools),
		"resources":       anys(resources),
	}}, nil
}

func initializeIgnoresUnknownMembers(ctx *host.Context) (host.Outcome, error) {
	document, err := conversation(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	responses, err := runConversation(document)
	if err != nil {
		return host.Outcome{}, err
	}
	initialize := response(responses, 1)
	if initialize == nil {
		return host.Outcome{}, fmt.Errorf("no initialize response")
	}
	if _, refused := initialize["error"]; refused {
		return host.Outcome{}, fmt.Errorf("initialize refused an unknown member: %v", initialize)
	}
	return host.Outcome{Projection: map[string]any{"lenient": true}}, nil
}

func theDryRunStartsNoServer(_ *host.Context) (host.Outcome, error) {
	code := cli.McpVerbDryRun()
	if code != 0 {
		return host.Outcome{}, fmt.Errorf("--dry-run exited with %d", code)
	}
	return host.Outcome{Projection: map[string]any{"exitCode": 0}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// 🧭️ Registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-verb-completes-the-handshake", theVerbCompletesTheHandshake).
		Subject("initialize-ignores-unknown-members", initializeIgnoresUnknownMembers).
		Subject("the-dry-run-starts-no-server", theDryRunStartsNoServer)
}

// endregion 🔖️Registration
