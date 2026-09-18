// 🐹️ Go subject for the MCP initialize handshake. The Go implementation is a `package main` process,
// so the adapter drives the real `semio-repo-mcp` binary over its own stdio transport — the same
// surface a client sees — and projects only what the reply carries.
package adapter

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"time"

	host "semio.tech/repo/test"
)

// region 🔖️Support

// 🕰️sourceIsNewer reports whether any file of the package was modified after the cached build.
func sourceIsNewer(source string, built time.Time) bool {
	entries, err := os.ReadDir(source)
	if err != nil {
		return true
	}
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil || info.ModTime().After(built) {
			return true
		}
	}
	return false
}

// 🗃️binaryPath returns the built MCP binary, rebuilding it into the marked cache when it is absent or stale.
func binaryPath(repoRoot string) (string, error) {
	name := "semio-repo-mcp"
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	binary := filepath.Join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", name)
	source := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client", "🔌️mcp")
	// 🕰️A cached binary is reused only while it is NEWER than every source file it was built from.
	// Reusing it unconditionally made the Go subject answer from a build that predates the change under
	// test, so a real fix read as a parity difference against the other implementations.
	if info, err := os.Stat(binary); err == nil && !sourceIsNewer(source, info.ModTime()) {
		return binary, nil
	}
	if err := os.MkdirAll(filepath.Dir(binary), 0o755); err != nil {
		return "", err
	}
	build := exec.Command("bun", "./📜️script.ts", "build")
	build.Dir = source
	build.Env = append(os.Environ(), "GOWORK="+filepath.Join(repoRoot, "go.work"), "GOFLAGS=")
	if output, err := build.CombinedOutput(); err != nil {
		return "", fmt.Errorf("build semio-repo-mcp: %v: %s", err, output)
	}
	return binary, nil
}

// 📡️exchange runs the binary for one profile, writes every request and returns one reply per request id.
func exchange(repoRoot string, profile string, requests []string) ([]map[string]json.RawMessage, error) {
	binary, err := binaryPath(repoRoot)
	if err != nil {
		return nil, err
	}
	command := exec.Command(binary)
	command.Dir = repoRoot
	command.Env = append(os.Environ(), "SEMIO_REPO_MCP_CLIENT="+profile)
	stdin, err := command.StdinPipe()
	if err != nil {
		return nil, err
	}
	stdout, err := command.StdoutPipe()
	if err != nil {
		return nil, err
	}
	if err := command.Start(); err != nil {
		return nil, err
	}
	reader := bufio.NewReader(stdout)
	replies := make([]map[string]json.RawMessage, 0, len(requests))
	for _, request := range requests {
		if _, err := fmt.Fprintln(stdin, request); err != nil {
			return nil, err
		}
		var envelope map[string]json.RawMessage
		if err := json.Unmarshal([]byte(request), &envelope); err != nil {
			return nil, err
		}
		if _, expectsReply := envelope["id"]; !expectsReply {
			continue
		}
		line, err := reader.ReadBytes('\n')
		if err != nil {
			return nil, err
		}
		var reply map[string]json.RawMessage
		if err := json.Unmarshal(line, &reply); err != nil {
			return nil, err
		}
		replies = append(replies, reply)
	}
	_ = stdin.Close()
	_ = command.Wait()
	return replies, nil
}

// 🗜️compacted strips the fixture file's own indentation so one request stays one transport line.
func compacted(raw json.RawMessage) string {
	var buffer bytes.Buffer
	if err := json.Compact(&buffer, raw); err != nil {
		return "{}"
	}
	return buffer.String()
}

// 🤝️handshake performs the fixture's initialize against the binary and returns the reply.
func handshake(ctx *host.Context) (map[string]json.RawMessage, string, error) {
	raw, err := ctx.FixtureBytes("shared://🤝️initialize-lenient.json")
	if err != nil {
		return nil, "", err
	}
	var fixture struct {
		RequestedProtocolVersion string          `json:"requestedProtocolVersion"`
		ClientInfo               json.RawMessage `json:"clientInfo"`
		Capabilities             json.RawMessage `json:"capabilities"`
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return nil, "", err
	}
	request := fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":%s,"clientInfo":%s}}`, fixture.RequestedProtocolVersion, compacted(fixture.Capabilities), compacted(fixture.ClientInfo))
	replies, err := exchange(ctx.RepoRoot, "client", []string{request})
	if err != nil {
		return nil, "", err
	}
	if len(replies) != 1 {
		return nil, "", errors.New("no initialize reply")
	}
	return replies[0], fixture.RequestedProtocolVersion, nil
}

// 🔁️pipelinedHandshake delivers the initialized notification while the session is still connected —
// the exact window a pipelining client opens — then initializes and pings over the same transport.
func pipelinedHandshake(ctx *host.Context) (map[string]any, error) {
	raw, err := ctx.FixtureBytes("shared://🤝️initialize-lenient.json")
	if err != nil {
		return nil, err
	}
	var fixture struct {
		RequestedProtocolVersion string          `json:"requestedProtocolVersion"`
		ClientInfo               json.RawMessage `json:"clientInfo"`
		Capabilities             json.RawMessage `json:"capabilities"`
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return nil, err
	}
	initialize := fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":%s,"clientInfo":%s}}`, fixture.RequestedProtocolVersion, compacted(fixture.Capabilities), compacted(fixture.ClientInfo))
	replies, err := exchange(ctx.RepoRoot, "client", []string{`{"jsonrpc":"2.0","method":"notifications/initialized"}`, initialize, `{"jsonrpc":"2.0","id":2,"method":"ping"}`})
	if err != nil {
		return nil, err
	}
	if len(replies) != 2 {
		return nil, errors.New("expected one initialize reply and one ping reply")
	}
	_, initializeFailed := replies[0]["error"]
	_, pingFailed := replies[1]["error"]
	return map[string]any{"initializeAccepted": !initializeFailed, "pingAnswered": !pingFailed}, nil
}

// 🚰️burstThenEOF writes every request, closes standard input at once and only then reads: the
// `printf … | semio-repo-mcp` shape, where end of input arrives while requests are still queued.
func burstThenEOF(repoRoot string, profile string, requests []string) ([]map[string]json.RawMessage, error) {
	binary, err := binaryPath(repoRoot)
	if err != nil {
		return nil, err
	}
	command := exec.Command(binary)
	command.Dir = repoRoot
	command.Env = append(os.Environ(), "SEMIO_REPO_MCP_CLIENT="+profile)
	stdin, err := command.StdinPipe()
	if err != nil {
		return nil, err
	}
	stdout, err := command.StdoutPipe()
	if err != nil {
		return nil, err
	}
	if err := command.Start(); err != nil {
		return nil, err
	}
	for _, request := range requests {
		if _, err := fmt.Fprintln(stdin, request); err != nil {
			return nil, err
		}
	}
	if err := stdin.Close(); err != nil {
		return nil, err
	}
	replies := make([]map[string]json.RawMessage, 0, len(requests))
	reader := bufio.NewReader(stdout)
	for {
		line, err := reader.ReadBytes('\n')
		if len(bytes.TrimSpace(line)) > 0 {
			var reply map[string]json.RawMessage
			if err := json.Unmarshal(line, &reply); err != nil {
				return nil, err
			}
			replies = append(replies, reply)
		}
		if err != nil {
			break
		}
	}
	_ = command.Wait()
	return replies, nil
}

// 🧾️pipelinedBurstProjection reduces the replies to the ids answered, whether the request that
// followed the initialized notification carries a result, and the uninitialized-session refusals.
func pipelinedBurstProjection(replies []map[string]json.RawMessage) map[string]any {
	answered := make([]string, 0, len(replies))
	refused := 0
	served := false
	for _, reply := range replies {
		var failure struct {
			Code int `json:"code"`
		}
		if raw, failed := reply["error"]; failed {
			if err := json.Unmarshal(raw, &failure); err == nil && failure.Code == -32002 {
				refused++
			}
		}
		id := string(reply["id"])
		if _, ok := reply["result"]; ok && id == "2" {
			served = true
		}
		answered = append(answered, id)
	}
	sort.Strings(answered)
	return map[string]any{"answered": answered, "followingRequestServed": served, "notInitializedRefusals": refused}
}

// 🧾️burstProjection reduces the replies to the request ids answered and the closed-session refusals.
func burstProjection(replies []map[string]json.RawMessage) map[string]any {
	answered := make([]string, 0, len(replies))
	refused := 0
	for _, reply := range replies {
		var failure struct {
			Code int `json:"code"`
		}
		if raw, failed := reply["error"]; failed {
			if err := json.Unmarshal(raw, &failure); err == nil && failure.Code == -32004 {
				refused++
			}
		}
		answered = append(answered, string(reply["id"]))
	}
	sort.Strings(answered)
	return map[string]any{"answered": answered, "sessionClosedRefusals": refused}
}

// endregion 🔖️Support

// region 🔖️Scenarios

func initializeAcceptsUnknownMembers(ctx *host.Context) (host.Outcome, error) {
	reply, _, err := handshake(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	var result struct {
		Capabilities map[string]json.RawMessage `json:"capabilities"`
		ServerInfo   struct {
			Version string `json:"version"`
		} `json:"serverInfo"`
	}
	if raw, ok := reply["result"]; ok {
		if err := json.Unmarshal(raw, &result); err != nil {
			return host.Outcome{}, err
		}
	}
	capabilities := make([]string, 0, len(result.Capabilities))
	for name := range result.Capabilities {
		capabilities = append(capabilities, name)
	}
	sort.Strings(capabilities)
	_, failed := reply["error"]
	return host.Outcome{Projection: map[string]any{"accepted": !failed, "serverVersion": result.ServerInfo.Version, "capabilities": capabilities}}, nil
}

func initializeAndInitializedPipelined(ctx *host.Context) (host.Outcome, error) {
	projection, err := pipelinedHandshake(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: projection}, nil
}

func initializeEchoesASupportedVersion(ctx *host.Context) (host.Outcome, error) {
	reply, requested, err := handshake(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	var result struct {
		ProtocolVersion string `json:"protocolVersion"`
	}
	if raw, ok := reply["result"]; ok {
		if err := json.Unmarshal(raw, &result); err != nil {
			return host.Outcome{}, err
		}
	}
	return host.Outcome{Projection: map[string]any{"protocolVersionEchoed": result.ProtocolVersion == requested}}, nil
}

// 🌊️burstRequests is the one burst both burst scenarios write: initialize, the initialized
// notification and one ordinary request, in the order a pipelining client puts them on the wire.
func burstRequests(ctx *host.Context) ([]string, error) {
	raw, err := ctx.FixtureBytes("shared://🤝️initialize-lenient.json")
	if err != nil {
		return nil, err
	}
	var fixture struct {
		RequestedProtocolVersion string          `json:"requestedProtocolVersion"`
		ClientInfo               json.RawMessage `json:"clientInfo"`
		Capabilities             json.RawMessage `json:"capabilities"`
	}
	if err := json.Unmarshal(raw, &fixture); err != nil {
		return nil, err
	}
	initialize := fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":%s,"clientInfo":%s}}`, fixture.RequestedProtocolVersion, compacted(fixture.Capabilities), compacted(fixture.ClientInfo))
	return []string{initialize, `{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`, `{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}`}, nil
}

func pipelinedBurstServesRequestsAfterInitialized(ctx *host.Context) (host.Outcome, error) {
	requests, err := burstRequests(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	replies, err := burstThenEOF(ctx.RepoRoot, "client", requests)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: pipelinedBurstProjection(replies)}, nil
}

func eofAfterBurstCompletesQueuedRequests(ctx *host.Context) (host.Outcome, error) {
	requests, err := burstRequests(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	replies, err := burstThenEOF(ctx.RepoRoot, "client", requests)
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: burstProjection(replies)}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("initialize-accepts-unknown-members", initializeAcceptsUnknownMembers).
		Subject("initialize-and-initialized-pipelined", initializeAndInitializedPipelined).
		Subject("initialize-echoes-a-supported-version", initializeEchoesASupportedVersion).
		Subject("pipelined-burst-serves-requests-after-initialized", pipelinedBurstServesRequestsAfterInitialized).
		Subject("eof-after-burst-completes-queued-requests", eofAfterBurstCompletesQueuedRequests)
}

// endregion 🔖️Registration
