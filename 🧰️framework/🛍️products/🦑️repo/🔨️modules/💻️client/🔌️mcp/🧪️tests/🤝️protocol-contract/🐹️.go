// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// MCP protocol contract tests: the repository entrypoint, forward-extensible frames, golden vectors and the production-owned runtime pipe.

// #endregion 🧲️Header

package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"testing"

	"github.com/usalu/semio/repo/client"
)

// #region 🤝️ProtocolContract

type contractFixture struct {
	Schema          string `json:"schema"`
	ProtocolVersion string `json:"protocolVersion"`
	Vectors         []struct {
		Name     string `json:"name"`
		Ready    bool   `json:"ready"`
		Request  string `json:"request"`
		Response string `json:"response"`
	} `json:"vectors"`
}

type entrypointContractFixture struct {
	Schema             string   `json:"schema"`
	ProfileEnvironment string   `json:"profileEnvironment"`
	Arguments          []string `json:"arguments"`
	ProtocolVersions   []string `json:"protocolVersions"`
	Profiles           []struct {
		Slug       string `json:"slug"`
		Kind       string `json:"kind"`
		ServerName string `json:"serverName"`
	} `json:"profiles"`
	BuildInstructions []string `json:"buildInstructions"`
	Flow              []string `json:"flow"`
}

// 🧫️ Resolves protocol vectors owned by the sibling protocol module from this module's own directory, never from a repository root.
func repositoryContractFixture(name string) string {
	return filepath.Join("..", "..", "🔌️mcp", "🧪️tests", "🧫️fixtures", name)
}

func TestRepositoryEntrypointContract(t *testing.T) {
	data, err := os.ReadFile(repositoryContractFixture("🚪️entrypoint-contract.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture entrypointContractFixture
	if err := decodeExact(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.mcp.entrypoint/1" || fixture.ProfileEnvironment != MCPProfileEnvironment || len(fixture.Arguments) != 0 {
		t.Fatalf("unexpected entrypoint fixture identity: %#v", fixture)
	}
	wantFlow := []string{"initialize", "notifications/initialized", "resources/list", "resources/read:repo://goals", "tools/list", "tools/call"}
	if !slices.Equal(fixture.Flow, wantFlow) {
		t.Fatalf("entrypoint flow got=%q want=%q", fixture.Flow, wantFlow)
	}
	if !slices.Equal(fixture.ProtocolVersions, SupportedProtocolVersions) {
		t.Fatalf("protocol versions got=%q want=%q", SupportedProtocolVersions, fixture.ProtocolVersions)
	}
	for _, profile := range fixture.Profiles {
		t.Run(profile.Slug, func(t *testing.T) {
			kind, err := resolveMCPProfile(fixture.Arguments, func(key string) (string, bool) {
				if key != fixture.ProfileEnvironment {
					t.Fatalf("profile lookup key got=%q want=%q", key, fixture.ProfileEnvironment)
				}
				return profile.Slug, true
			})
			if err != nil {
				t.Fatal(err)
			}
			if string(kind) != profile.Kind || client.McpServerName(kind) != profile.ServerName {
				t.Fatalf("profile %q resolved kind=%q server=%q", profile.Slug, kind, client.McpServerName(kind))
			}
			server, err := NewRepositoryServerFor(&testRepository{}, kind)
			if err != nil {
				t.Fatal(err)
			}
			for _, instruction := range fixture.BuildInstructions {
				if !strings.Contains(server.config.Instructions, instruction) {
					t.Fatalf("missing neutral build instruction %q", instruction)
				}
			}
			if server.config.ServerInfo.Name != profile.ServerName {
				t.Fatalf("server name got=%q want=%q", server.config.ServerInfo.Name, profile.ServerName)
			}
			open := server.tools["ticket_open"].schema.InputSchema.Properties
			_, hasPlan := open["plan_id"]
			_, hasSpec := open["spec_id"]
			if profile.Kind == "kiro" {
				if hasPlan || !hasSpec {
					t.Fatalf("kiro ticket schema plan=%v spec=%v", hasPlan, hasSpec)
				}
			} else if profile.Kind == "generic" {
				if hasPlan || hasSpec {
					t.Fatalf("generic ticket schema plan=%v spec=%v", hasPlan, hasSpec)
				}
			} else if !hasPlan || hasSpec {
				t.Fatalf("profile %q ticket schema plan=%v spec=%v", profile.Kind, hasPlan, hasSpec)
			}
		})
	}
	for _, version := range fixture.ProtocolVersions {
		server, err := NewRepositoryServer(&testRepository{})
		if err != nil {
			t.Fatal(err)
		}
		session, err := server.Connect("protocol-"+version, nil)
		if err != nil {
			t.Fatal(err)
		}
		payload := fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"fixture","version":"1"}}}`, version)
		response, err := session.Dispatch(context.Background(), []byte(payload))
		if err != nil {
			t.Fatal(err)
		}
		var envelope Response
		if err := json.Unmarshal(response, &envelope); err != nil {
			t.Fatal(err)
		}
		var initialized InitializeResult
		if err := json.Unmarshal(envelope.Result, &initialized); err != nil {
			t.Fatal(err)
		}
		if initialized.ProtocolVersion != version {
			t.Fatalf("negotiated protocol got=%q want=%q", initialized.ProtocolVersion, version)
		}
	}
	if _, err := resolveMCPProfile([]string{"mcp", "client"}, func(string) (string, bool) { return "", false }); err == nil {
		t.Fatal("legacy CLI delegation arguments must be rejected")
	}
}

// 🤝️The handshake fixture is owned by the `🔌️mcp` module (both twins read the same file), so this
// implementation never restates the contract it must satisfy.
type lenientHandshakeFixture struct {
	Schema                   string          `json:"schema"`
	RequestedProtocolVersion string          `json:"requestedProtocolVersion"`
	ClientInfo               json.RawMessage `json:"clientInfo"`
	Capabilities             json.RawMessage `json:"capabilities"`
	Expected                 struct {
		Accepted              bool     `json:"accepted"`
		ProtocolVersionEchoed bool     `json:"protocolVersionEchoed"`
		ServerVersion         string   `json:"serverVersion"`
		Capabilities          []string `json:"capabilities"`
	} `json:"expected"`
}

func TestInitializeAdmitsForwardExtensibleClientFrames(t *testing.T) {
	data, err := os.ReadFile(repositoryContractFixture("🤝️initialize-lenient.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture lenientHandshakeFixture
	if err := decodeExact(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.repo.mcp.handshake/1" || !fixture.Expected.Accepted {
		t.Fatalf("unexpected handshake fixture identity: %#v", fixture)
	}
	server, err := NewRepositoryServer(&testRepository{})
	if err != nil {
		t.Fatal(err)
	}
	session, err := server.Connect("lenient", nil)
	if err != nil {
		t.Fatal(err)
	}
	payload := fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":%s,"clientInfo":%s}}`, fixture.RequestedProtocolVersion, fixture.Capabilities, fixture.ClientInfo)
	response, err := session.Dispatch(context.Background(), []byte(payload))
	if err != nil {
		t.Fatal(err)
	}
	var envelope Response
	if err := json.Unmarshal(response, &envelope); err != nil {
		t.Fatal(err)
	}
	if envelope.Error != nil {
		t.Fatalf("forward-extensible initialize was rejected: %s", response)
	}
	var initialized InitializeResult
	if err := json.Unmarshal(envelope.Result, &initialized); err != nil {
		t.Fatal(err)
	}
	if fixture.Expected.ProtocolVersionEchoed && initialized.ProtocolVersion != fixture.RequestedProtocolVersion {
		t.Fatalf("negotiated protocol got=%q want=%q", initialized.ProtocolVersion, fixture.RequestedProtocolVersion)
	}
	if initialized.ServerInfo.Version != fixture.Expected.ServerVersion {
		t.Fatalf("server version got=%q want=%q", initialized.ServerInfo.Version, fixture.Expected.ServerVersion)
	}
	present := map[string]bool{"prompts": initialized.Capabilities.Prompts != nil, "resources": initialized.Capabilities.Resources != nil, "tools": initialized.Capabilities.Tools != nil}
	for _, capability := range fixture.Expected.Capabilities {
		if !present[capability] {
			t.Fatalf("server capability %q was not advertised", capability)
		}
	}
	strict, err := NewRepositoryServer(&testRepository{})
	if err != nil {
		t.Fatal(err)
	}
	strictSession, err := strict.Connect("strict", nil)
	if err != nil {
		t.Fatal(err)
	}
	strictResponse, err := strictSession.Dispatch(context.Background(), []byte(`{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"ticket_open","arguments":{},"unknownField":1}}`))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Contains(strictResponse, []byte(`"error"`)) {
		t.Fatalf("frames other than initialize must stay strictly decoded: %s", strictResponse)
	}
}

func TestOpenRequestMetaAdmitsClientKeys(t *testing.T) {
	var mu sync.Mutex
	var messages [][]byte
	sink := func(_ context.Context, payload []byte) error {
		mu.Lock()
		messages = append(messages, append([]byte(nil), payload...))
		mu.Unlock()
		return nil
	}
	server := newContractServer(t, DefaultLimits(), nil, sink)
	session, err := server.Connect("meta", sink)
	if err != nil {
		t.Fatal(err)
	}
	initializeSession(t, session)
	meta := `{"progressToken":1,"claudecode/toolUseId":"x"}`
	calls := []struct {
		id     string
		method string
		params string
	}{
		{`"call"`, "tools/call", `{"name":"echo","arguments":{"text":"ok"},"_meta":` + meta + `}`},
		{`"read"`, "resources/read", `{"uri":"repo://goals","_meta":` + meta + `}`},
		{`"prompt"`, "prompts/get", `{"name":"review","arguments":{"scope":"all"},"_meta":` + meta + `}`},
		{`"tools"`, "tools/list", `{"_meta":` + meta + `}`},
		{`"resources"`, "resources/list", `{"cursor":"0","_meta":` + meta + `}`},
		{`"templates"`, "resources/templates/list", `{"_meta":` + meta + `}`},
		{`"prompts"`, "prompts/list", `{"_meta":` + meta + `}`},
	}
	for _, call := range calls {
		response, err := session.Dispatch(context.Background(), request(call.method, call.id, call.params))
		if err != nil {
			t.Fatal(err)
		}
		assertResponseCode(t, response, 0)
		if call.method == "tools/call" && !bytes.Contains(response, []byte(`"text":"ok"`)) {
			t.Fatalf("tools/call result: %s", response)
		}
	}
	mu.Lock()
	progress := append([][]byte(nil), messages...)
	mu.Unlock()
	if len(progress) != 1 || !bytes.Contains(progress[0], []byte(`"progressToken":1`)) {
		t.Fatalf("progress token was not preserved: %q", progress)
	}
	rejected, err := session.Dispatch(context.Background(), request("tools/call", `"strict"`, `{"name":"echo","arguments":{"text":"ok"},"unknownField":1,"_meta":`+meta+`}`))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, rejected, CodeInvalidParams)
	malformed, err := session.Dispatch(context.Background(), request("tools/call", `"array"`, `{"name":"echo","arguments":{"text":"ok"},"_meta":[]}`))
	if err != nil {
		t.Fatal(err)
	}
	assertResponseCode(t, malformed, CodeInvalidParams)
}

func TestG2CanonicalGoldenVectors(t *testing.T) {
	data, err := os.ReadFile(repositoryContractFixture("2️⃣g2-contract.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture contractFixture
	if err := decodeExact(data, &fixture); err != nil {
		t.Fatal(err)
	}
	if fixture.Schema != "semio.mcp.contract/1" || fixture.ProtocolVersion != ProtocolVersion {
		t.Fatalf("unexpected fixture identity: %#v", fixture)
	}
	for _, vector := range fixture.Vectors {
		t.Run(vector.Name, func(t *testing.T) {
			server := newContractServer(t, DefaultLimits(), nil, nil)
			session, err := server.Connect("fixture", nil)
			if err != nil {
				t.Fatal(err)
			}
			if vector.Ready {
				initializeSession(t, session)
			}
			response, err := session.Dispatch(context.Background(), []byte(vector.Request))
			if err != nil {
				t.Fatal(err)
			}
			if string(response) != vector.Response {
				t.Fatalf("response\n got: %s\nwant: %s", response, vector.Response)
			}
		})
	}
}

func TestG2ProductionOwnedRuntimePipe(t *testing.T) {
	repository := &testRepository{called: make(chan string, 1)}
	serverPipe, clientPipe := net.Pipe()
	type outcome struct {
		server *Server
		err    error
	}
	finished := make(chan outcome, 1)
	go func() {
		server, err := runMCP(context.Background(), serverPipe, repository)
		finished <- outcome{server: server, err: err}
	}()
	reader := bufio.NewReader(clientPipe)
	writeLine := func(payload string) {
		t.Helper()
		if _, err := io.WriteString(clientPipe, payload+"\n"); err != nil {
			t.Fatal(err)
		}
	}
	readLine := func() []byte {
		t.Helper()
		line, err := reader.ReadBytes('\n')
		if err != nil {
			t.Fatal(err)
		}
		return bytes.TrimSpace(line)
	}
	writeLine(fmt.Sprintf(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":%q,"capabilities":{},"clientInfo":{"name":"production-test","version":"1"}}}`, ProtocolVersion))
	line := readLine()
	assertResponseCode(t, line, 0)
	if !bytes.Contains(line, []byte(`"protocolVersion":"`+ProtocolVersion+`"`)) || !bytes.Contains(line, []byte(`"tools":{}`)) {
		t.Fatalf("production lifecycle/capability response: %s", line)
	}
	writeLine(`{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}`)
	writeLine(`{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"ticket_open","arguments":{"emoji":"🎫️","title":"Fixture","prompt":"Fixture","goal":"fixture"}}}`)
	assertResponseCode(t, readLine(), 0)
	if called := <-repository.called; called != "ticket_open" {
		t.Fatalf("production handler got=%q", called)
	}
	_ = clientPipe.Close()
	result := <-finished
	if result.err != nil {
		t.Fatal(result.err)
	}
	if result.server == nil {
		t.Fatal("production run did not return its owned server")
	}
	events := result.server.Events().Events()
	if len(events) < 6 || events[0].Kind != "session.opened" {
		t.Fatalf("production event evidence: %#v", events)
	}
}

func TestG2ProductionComponentRejectsDirectDelegationMutation(t *testing.T) {
	source, err := os.ReadFile("🧩️component/🐹️.go")
	if err != nil {
		t.Fatal(err)
	}
	for _, forbidden := range [][]byte{[]byte("client.RunMCP"), []byte("client.RunCLI"), []byte(`[]string{"mcp"`)} {
		if bytes.Contains(source, forbidden) {
			t.Fatalf("production restored legacy delegation marker %q", forbidden)
		}
	}
	for _, required := range [][]byte{[]byte("runMCPForProfile("), []byte("NewRepositoryServerFor("), []byte("server.Serve(")} {
		if !bytes.Contains(source, required) {
			t.Fatalf("production component bypasses owned runtime marker %q", required)
		}
	}
}

func TestG2TransportRejectsBusyBranchMutation(t *testing.T) {
	source, err := os.ReadFile("🚚️transport/🐹️.go")
	if err != nil {
		t.Fatal(err)
	}
	if err := verifyBusyBranch(source); err != nil {
		t.Fatal(err)
	}
	mutations := []struct {
		name   string
		source []byte
	}{
		{name: "blocking", source: bytes.Replace(source, []byte("case jobs <- payload:\n\t\tdefault:"), []byte("case jobs <- payload:\n\t\tcase <-ctx.Done():"), 1)},
		{name: "code", source: bytes.Replace(source, []byte("CodeServerBusy"), []byte("CodeInternalError"), 1)},
		{name: "credit", source: bytes.Replace(source, []byte("default:\n\t\t\tserver.handlerQueued.Add(-1)"), []byte("default:\n\t\t\tserver.handlerQueued.Add(0)"), 1)},
	}
	for _, mutation := range mutations {
		t.Run(mutation.name, func(t *testing.T) {
			if err := verifyBusyBranch(mutation.source); err == nil {
				t.Fatal("hostile busy-branch mutation survived")
			}
		})
	}
}

func verifyBusyBranch(source []byte) error {
	start := bytes.Index(source, []byte("server.handlerQueued.Add(1)"))
	if start < 0 {
		return errors.New("transport removed bounded queue admission")
	}
	endOffset := bytes.Index(source[start:], []byte("if err := write(ctx, response)"))
	if endOffset < 0 {
		return errors.New("transport removed busy response write")
	}
	branch := source[start : start+endOffset]
	for _, required := range [][]byte{[]byte("select {"), []byte("case jobs <- payload:"), []byte("default:"), []byte("server.handlerQueued.Add(-1)"), []byte("CodeServerBusy"), []byte(`"handler queue full"`)} {
		if !bytes.Contains(branch, required) {
			return fmt.Errorf("transport weakened bounded busy branch marker %q", required)
		}
	}
	return nil
}

// #endregion 🤝️ProtocolContract
