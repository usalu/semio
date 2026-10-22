// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Build session tests: an MCP client harness over the production pipe drives the derived tools against shell-script children and checks the structured result, the progress, the agent attribution, the cancellation, the tasks and the survival of a faulty tool.

// #endregion 🧲️Header

package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/usalu/semio/repo/providers"
)

// #region 🔌️BuildSessionTests

type rpcPeer struct {
	t       *testing.T
	conn    net.Conn
	reader  *bufio.Reader
	next    int
	pending []json.RawMessage
	stashed map[int]rpcReply
}

type rpcReply struct {
	Result        json.RawMessage
	Error         *RPCError
	Notifications []json.RawMessage
}

func (peer *rpcPeer) send(payload string) {
	peer.t.Helper()
	_ = peer.conn.SetWriteDeadline(time.Now().Add(5 * time.Second))
	if _, err := fmt.Fprintln(peer.conn, payload); err != nil {
		peer.t.Fatal(err)
	}
}

func (peer *rpcPeer) call(method string, params any) rpcReply {
	peer.t.Helper()
	id := peer.begin(method, params)
	return peer.await(id)
}

func (peer *rpcPeer) begin(method string, params any) int {
	peer.t.Helper()
	peer.next++
	encoded, err := json.Marshal(params)
	if err != nil {
		peer.t.Fatal(err)
	}
	peer.send(fmt.Sprintf(`{"jsonrpc":"2.0","id":%d,"method":%q,"params":%s}`, peer.next, method, encoded))
	return peer.next
}

func (peer *rpcPeer) await(id int) rpcReply {
	peer.t.Helper()
	reply := rpcReply{}
	if early, found := peer.stashed[id]; found {
		delete(peer.stashed, id)
		return early
	}
	for {
		_ = peer.conn.SetReadDeadline(time.Now().Add(20 * time.Second))
		line, err := peer.reader.ReadBytes('\n')
		if err != nil {
			peer.t.Fatalf("waiting for response %d: %v (notifications %s)", id, err, reply.Notifications)
		}
		var envelope struct {
			ID     *int            `json:"id"`
			Method string          `json:"method"`
			Result json.RawMessage `json:"result"`
			Error  *RPCError       `json:"error"`
		}
		if err := json.Unmarshal(line, &envelope); err != nil {
			peer.t.Fatal(err)
		}
		if envelope.ID == nil {
			reply.Notifications = append(reply.Notifications, json.RawMessage(strings.TrimSpace(string(line))))
			continue
		}
		if *envelope.ID == id {
			reply.Result, reply.Error = envelope.Result, envelope.Error
			return reply
		}
		if peer.stashed == nil {
			peer.stashed = map[int]rpcReply{}
		}
		peer.stashed[*envelope.ID] = rpcReply{Result: envelope.Result, Error: envelope.Error}
	}
}

func (peer *rpcPeer) notify(method string, params any) {
	peer.t.Helper()
	encoded, _ := json.Marshal(params)
	peer.send(fmt.Sprintf(`{"jsonrpc":"2.0","method":%q,"params":%s}`, method, encoded))
}

func (reply rpcReply) progress() []ProgressParams {
	var all []ProgressParams
	for _, raw := range reply.Notifications {
		var notification struct {
			Method string         `json:"method"`
			Params ProgressParams `json:"params"`
		}
		if json.Unmarshal(raw, &notification) == nil && notification.Method == "notifications/progress" {
			all = append(all, notification.Params)
		}
	}
	return all
}

type sessionFixture struct {
	peer    *rpcPeer
	server  *Server
	surface *BuildSurface
	marks   string
}

// newBuildSession starts the production server with the real derived tools and a shell script as the `repo` executable; the script sees the derived argument vector as its positional parameters.
func newBuildSession(t *testing.T, script string, adjust func(*Policy)) *sessionFixture {
	t.Helper()
	if _, err := exec.LookPath("sh"); err != nil {
		t.Skip("no POSIX shell")
	}
	catalogue, policy := authoredSurface(t)
	policy.Limits.ProgressIntervalMs, policy.Limits.HeartbeatMs, policy.Limits.CancelGraceMs = 10, 100, 3000
	if adjust != nil {
		adjust(&policy)
	}
	marks := t.TempDir()
	surface, err := NewBuildSurface(t.TempDir(), catalogue, policy)
	if err != nil {
		t.Fatal(err)
	}
	surface.Binary, surface.Prefix = "sh", []string{"-c", script, "repo"}
	surface.Process, surface.Parent = 4242, 4241
	surface.Environ = func() []string {
		return []string{"PATH=" + os.Getenv("PATH"), "MARKS=" + marks, "SEMIO_SCRIPT_POLICY=inherited", "SEMIO_PROCESS_OWNER_CONTEXT=inherited"}
	}
	server, err := NewRepositoryServerWithLimitsFor(&testRepository{}, providers.McpClientGeneric, ProductionLimits())
	if err != nil {
		t.Fatal(err)
	}
	if err := surface.Register(server); err != nil {
		t.Fatal(err)
	}
	serverPipe, clientPipe := net.Pipe()
	finished := make(chan error, 1)
	go func() { finished <- server.Serve(context.Background(), "build-session", serverPipe) }()
	t.Cleanup(func() {
		_ = clientPipe.Close()
		select {
		case <-finished:
		case <-time.After(10 * time.Second):
			t.Error("server did not stop")
		}
	})
	peer := &rpcPeer{t: t, conn: clientPipe, reader: bufio.NewReader(clientPipe)}
	reply := peer.call("initialize", map[string]any{"protocolVersion": ProtocolVersion, "capabilities": map[string]any{}, "clientInfo": map[string]any{"name": "Harness Client", "version": "1"}})
	if reply.Error != nil {
		t.Fatal(reply.Error)
	}
	peer.notify("notifications/initialized", map[string]any{})
	return &sessionFixture{peer: peer, server: server, surface: surface, marks: marks}
}

func (fixture *sessionFixture) callTool(name string, arguments map[string]any, extra map[string]any) rpcReply {
	params := map[string]any{"name": name, "arguments": arguments}
	for key, value := range extra {
		params[key] = value
	}
	return fixture.peer.call("tools/call", params)
}

func decodeToolResult(t *testing.T, reply rpcReply) (CallToolResult, BuildResult) {
	t.Helper()
	if reply.Error != nil {
		t.Fatalf("protocol error %v", reply.Error)
	}
	var result struct {
		Content           []Content       `json:"content"`
		StructuredContent json.RawMessage `json:"structuredContent"`
		IsError           bool            `json:"isError"`
	}
	if err := json.Unmarshal(reply.Result, &result); err != nil {
		t.Fatal(err)
	}
	var build BuildResult
	if len(result.StructuredContent) > 0 {
		if err := json.Unmarshal(result.StructuredContent, &build); err != nil {
			t.Fatal(err)
		}
	}
	return CallToolResult{Content: result.Content, IsError: result.IsError}, build
}

// 🧪️reportScript stands in for the `repo` executable: `session open --json` answers like the real command (a deterministic session per agent name or per connection, and one line per call in $MARKS/opens), everything else reports what the child sees.
const reportScript = `if [ "$1" = session ]; then
 agent=""; client=""; clientpid=""; ownerpid=""
 for a in "$@"; do case "$a" in --agent=*) agent="${a#--agent=}";; --client=*) client="${a#--client=}";; --client-pid=*) clientpid="${a#--client-pid=}";; --owner-pid=*) ownerpid="${a#--owner-pid=}";; esac; done
 [ -n "$MARKS" ] && echo "$*" >> "$MARKS/opens"
 [ -n "$FAIL_SESSION" ] && { echo "no session" >&2; exit 3; }
 if [ -z "$agent" ]; then agent="$(printf '%s' "$client" | tr 'A-Z ' 'a-z_')-$clientpid-$ownerpid"; fi
 printf '{"version":1,"created":true,"address":"0123456789ab.s%s","session":{"id":"s%s","agent":{"id":"%s"},"resources":{"cargo":{"lane":"l%s"}}}}' "$agent" "$agent" "$agent" "$agent"
 exit 0
fi
printf '{"address":"%s","tool":"%s","language":"%s","level":"%s","policy":"%s","owner":"%s","args":"%s","legacy":"%s"}' "$SEMIO_AGENT_SESSION" "${SEMIO_BUILD_TOOL:-}" "${SEMIO_LANGUAGE:-}" "${SEMIO_TEST_LEVEL:-}" "${SEMIO_SCRIPT_POLICY:-none}" "${SEMIO_PROCESS_OWNER_CONTEXT:-none}" "$*" "${SEMIO_AGENT_ID:-}${SEMIO_SESSION:-}${SEMIO_CARGO_LANE:-}"`

func TestServerListsDerivedBuildToolsAndTasks(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	reply := fixture.peer.call("tools/list", map[string]any{})
	var listing struct {
		Tools []Tool `json:"tools"`
	}
	if err := json.Unmarshal(reply.Result, &listing); err != nil {
		t.Fatal(err)
	}
	index := map[string]Tool{}
	for _, tool := range listing.Tools {
		index[tool.Name] = tool
	}
	for _, name := range append(golden(t).Tools, "ticket_open", "goal_close") {
		if _, found := index[name]; !found {
			t.Errorf("tools/list lacks %s", name)
		}
	}
	run := index["run"]
	if run.Execution == nil || run.Execution.TaskSupport != "optional" || run.OutputSchema == nil || run.Annotations == nil || run.Annotations.ReadOnlyHint == nil || *run.Annotations.ReadOnlyHint {
		t.Errorf("run descriptor %+v", run)
	}
	if hint := index["leases_status"].Annotations; hint == nil || hint.ReadOnlyHint == nil || !*hint.ReadOnlyHint {
		t.Errorf("a query is read-only: %+v", hint)
	}
	if !strings.Contains(run.Description, "EN: ") || !strings.Contains(run.Description, "DE: ") {
		t.Errorf("descriptions are bilingual: %q", run.Description)
	}
	resource := fixture.peer.call("resources/read", map[string]any{"uri": "repo://build-surface"})
	if resource.Error != nil || !strings.Contains(string(resource.Result), "watch") {
		t.Errorf("the surface lists what is left out: %v %s", resource.Error, resource.Result)
	}
}

func TestToolCallReturnsStructuredResultAndAttributesTheAgent(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	reply := fixture.callTool("run", map[string]any{"target": "a:b", "tool": "bazel", "language": "en", "agent": "alpha"}, nil)
	text, build := decodeToolResult(t, reply)
	if text.IsError || !build.OK || build.ExitCode != 0 || build.Operation != "run" || build.BuildTool != "bazel" {
		t.Fatalf("result %+v", build)
	}
	var data struct{ Address, Tool, Language, Policy, Owner, Args, Legacy string }
	if err := json.Unmarshal(build.Data, &data); err != nil {
		t.Fatalf("data %s: %v", build.Data, err)
	}
	if data.Address != "0123456789ab.salpha" || data.Tool != "bazel" || data.Language != "en" || data.Args != "run a:b" {
		t.Fatalf("the child must see the session address and the selection: %+v", data)
	}
	if data.Legacy != "" {
		t.Fatalf("no per-tool identity variable reaches the child: %q", data.Legacy)
	}
	if data.Policy != "none" || data.Owner != "none" {
		t.Fatalf("a call is its own original invocation: %+v", data)
	}
	if build.Agent.ID != "alpha" || build.Agent.Session != "salpha" || build.Agent.Lane != "lalpha" || strings.Join(build.Command, " ") != "repo run a:b" {
		t.Fatalf("the result names the agent and the exact command: %+v", build)
	}
	if len(text.Content) != 1 || !strings.HasPrefix(text.Content[0].Text, "OK repo run a:b") {
		t.Fatalf("text %+v", text.Content)
	}
	other := fixture.callTool("test", map[string]any{"project": "@semio-tech/repo-client", "test_level": "quick"}, nil)
	_, tested := decodeToolResult(t, other)
	var testData struct{ Address, Level, Args string }
	_ = json.Unmarshal(tested.Data, &testData)
	if testData.Address != "0123456789ab.sharness_client-4241-4242" || tested.Agent.ID != "harness_client-4241-4242" || testData.Level != "quick" || testData.Args != "run @semio-tech/repo-client:test" {
		t.Fatalf("a call without agent runs as the agent of the connection: %+v %+v", testData, tested.Agent)
	}
}

func TestEachAgentOpensItsSessionOnceAndAFailureNeverStopsTheCall(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	for index := 0; index < 3; index++ {
		fixture.callTool("run", map[string]any{"target": "a:b", "agent": "alpha"}, nil)
	}
	fixture.callTool("run", map[string]any{"target": "a:b", "agent": "beta"}, nil)
	fixture.callTool("run", map[string]any{"target": "a:b"}, nil)
	fixture.callTool("run", map[string]any{"target": "a:b"}, nil)
	opens, err := os.ReadFile(filepath.Join(fixture.marks, "opens"))
	if err != nil {
		t.Fatal(err)
	}
	lines := strings.Split(strings.TrimSpace(string(opens)), "\n")
	want := []string{"session open --json --agent=alpha", "session open --json --agent=beta", "session open --json --client=Harness Client --client-pid=4241 --owner-pid=4242"}
	if len(lines) != len(want) {
		t.Fatalf("one open per distinct agent: %q", lines)
	}
	for index := range want {
		if lines[index] != want[index] {
			t.Fatalf("open %d got=%q want=%q", index, lines[index], want[index])
		}
	}
	broken := newBuildSession(t, reportScript, nil)
	broken.surface.Environ = func() []string {
		return []string{"PATH=" + os.Getenv("PATH"), "MARKS=" + broken.marks, "FAIL_SESSION=1"}
	}
	text, build := decodeToolResult(t, broken.callTool("run", map[string]any{"target": "a:b"}, nil))
	var data struct{ Address string }
	_ = json.Unmarshal(build.Data, &data)
	if text.IsError || !build.OK || data.Address != "" || build.Agent.ID != "unattached" || !strings.Contains(build.Hint, "agent session not opened") {
		t.Fatalf("a failed session open leaves the call running without an address: %+v %+v", build, data)
	}
}

func TestSessionRequestsBuildTheRepoArguments(t *testing.T) {
	cases := map[string]SessionRequest{
		"session open --json --agent=x":                                          {Agent: "x"},
		"session open --json --client=c --client-pid=3 --owner-pid=4":            {Client: "c", ClientProcess: 3, Process: 4},
		"session open --json --client=c":                                         {Client: "c"},
		"session open --json":                                                    {},
	}
	for want, request := range cases {
		if got := strings.Join(request.Argv(), " "); got != want {
			t.Errorf("argv got=%q want=%q", got, want)
		}
	}
	if _, err := ParseSession([]byte(`{"address":"","session":{}}`)); err == nil {
		t.Error("a reply without an address is refused")
	}
	if _, err := ParseSession([]byte("not json")); err == nil {
		t.Error("garbage is refused")
	}
	identity, err := ParseSession([]byte(`{"address":"0123456789ab.sa","session":{"id":"sa","agent":{"id":"agent-1"},"resources":{"cargo":{"lane":"sa"}}}}`))
	if err != nil || identity != (AgentIdentity{ID: "agent-1", Session: "sa", Lane: "sa", Address: "0123456789ab.sa"}) {
		t.Errorf("identity %+v %v", identity, err)
	}
	if env := (AgentIdentity{}).Environment(AgentEnvironment{Session: "SEMIO_AGENT_SESSION"}); len(env) != 0 {
		t.Errorf("a call without an address carries no variable: %v", env)
	}
	if env := identity.Environment(AgentEnvironment{Session: "SEMIO_AGENT_SESSION"}); env["SEMIO_AGENT_SESSION"] != "0123456789ab.sa" || len(env) != 1 {
		t.Errorf("the address is the one variable: %v", env)
	}
}

func TestQueriesDefaultToTheSealedGraphAndNeverWait(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	_, build := decodeToolResult(t, fixture.callTool("show_projects", map[string]any{}, nil))
	var data struct{ Args string }
	_ = json.Unmarshal(build.Data, &data)
	if data.Args != "show projects --json --graph-policy=current" {
		t.Fatalf("args %q", data.Args)
	}
}

func TestInvalidArgumentsAreAToolErrorTheAgentCanRead(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	reply := fixture.callTool("run", map[string]any{"target": "nocolon"}, nil)
	text, _ := decodeToolResult(t, reply)
	if !text.IsError || !strings.Contains(text.Content[0].Text, "target") {
		t.Fatalf("a refusal explains itself: %+v", text)
	}
	unknown := fixture.callTool("run", map[string]any{"target": "a:b", "bogus": true}, nil)
	text, _ = decodeToolResult(t, unknown)
	if !text.IsError || !strings.Contains(text.Content[0].Text, "unknown arguments: bogus") {
		t.Fatalf("an unknown argument names itself and the accepted ones: %+v", text)
	}
}

func TestFailureIsAnErrorResultWithTheTailOfTheOutput(t *testing.T) {
	fixture := newBuildSession(t, `echo "boom: SEMIO_LANGUAGE missing" >&2; exit 7`, nil)
	text, build := decodeToolResult(t, fixture.callTool("run", map[string]any{"target": "a:b"}, nil))
	if !text.IsError || build.OK || build.ExitCode != 7 || !strings.Contains(build.Stderr, "boom") || build.Hint == "" {
		t.Fatalf("result %+v", build)
	}
	if !strings.Contains(text.Content[0].Text, "FAILED repo run a:b (exit 7") || !strings.Contains(text.Content[0].Text, "boom") {
		t.Fatalf("text %q", text.Content[0].Text)
	}
}

func TestProgressNotificationsFollowTheChildAndGrow(t *testing.T) {
	script := `printf '%s\n' '[script] {"owner":"o","command":"build:graph","stage":"running"}' >&2; sleep 0.1
printf '%s\n' '[script] {"owner":"o","command":"repo","stage":"running","activity":{"resource":"lease-queue:r","mode":"exclusive","elapsedMs":0}}' >&2
printf '%s\n' '[script] repo · Waiting for access to r (exclusive): position 2 of 2' >&2; sleep 0.1; echo done`
	fixture := newBuildSession(t, script, nil)
	reply := fixture.callTool("run", map[string]any{"target": "a:b"}, map[string]any{"_meta": map[string]any{"progressToken": "p1"}})
	_, build := decodeToolResult(t, reply)
	if !build.OK || build.ProgressEvents != 2 {
		t.Fatalf("result %+v", build)
	}
	progress := reply.progress()
	if len(progress) < 3 {
		t.Fatalf("progress %+v", progress)
	}
	last := 0.0
	joined := ""
	for _, step := range progress {
		if step.Progress <= last {
			t.Fatalf("progress must grow: %+v", progress)
		}
		last, joined = step.Progress, joined+step.Message+"|"
	}
	if !strings.Contains(joined, "started: repo run a:b") || !strings.Contains(joined, "position 2 of 2") || !strings.Contains(joined, "finished: exit 0") {
		t.Fatalf("messages %q", joined)
	}
}

func TestCancellingTheRequestInterruptsTheChild(t *testing.T) {
	script := `trap 'echo interrupted > "$MARKS/interrupted"; exit 130' INT; echo go > "$MARKS/running"; while :; do sleep 0.05; done`
	fixture := newBuildSession(t, script, nil)
	id := fixture.peer.begin("tools/call", map[string]any{"name": "run", "arguments": map[string]any{"target": "a:b"}})
	waitForFile(t, filepath.Join(fixture.marks, "running"))
	fixture.peer.notify("notifications/cancelled", map[string]any{"requestId": id, "reason": "user stopped"})
	reply := fixture.peer.await(id)
	if reply.Error == nil || reply.Error.Code != CodeRequestCancelled {
		t.Fatalf("a cancelled call answers as cancelled: %+v", reply)
	}
	waitForFile(t, filepath.Join(fixture.marks, "interrupted"))
}

func TestClosingTheConnectionStopsRunningChildren(t *testing.T) {
	script := `trap 'echo interrupted > "$MARKS/interrupted"; exit 130' INT; echo go > "$MARKS/running"; while :; do sleep 0.05; done`
	fixture := newBuildSession(t, script, nil)
	fixture.peer.begin("tools/call", map[string]any{"name": "run", "arguments": map[string]any{"target": "a:b"}})
	waitForFile(t, filepath.Join(fixture.marks, "running"))
	_ = fixture.peer.conn.Close()
	waitForFile(t, filepath.Join(fixture.marks, "interrupted"))
}

func waitForFile(t *testing.T, path string) {
	t.Helper()
	deadline := time.Now().Add(10 * time.Second)
	for time.Now().Before(deadline) {
		if _, err := os.Stat(path); err == nil {
			return
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Fatalf("%s did not appear", path)
}

func TestTaskAugmentedCallRunsOnAndDeliversTheResult(t *testing.T) {
	script := `sleep 0.3; printf '{"done":true}'`
	fixture := newBuildSession(t, script, nil)
	var capabilities struct{ Capabilities ServerCapabilities }
	initial := fixture.server.capabilities()
	if initial.Tasks == nil || initial.Tasks.Requests == nil || initial.Tasks.List == nil || initial.Tasks.Cancel == nil {
		t.Fatalf("capabilities %+v %+v", initial, capabilities)
	}
	reply := fixture.callTool("run", map[string]any{"target": "a:b"}, map[string]any{"task": map[string]any{"ttl": 60000}})
	var created CreateTaskResult
	if err := json.Unmarshal(reply.Result, &created); err != nil || created.Task.TaskID == "" || created.Task.Status != TaskWorking || created.Task.TTL == nil || *created.Task.TTL != 60000 || len(created.Task.TaskID) < 32 {
		t.Fatalf("created %s %v", reply.Result, err)
	}
	status := fixture.peer.call("tasks/get", map[string]any{"taskId": created.Task.TaskID})
	var task Task
	_ = json.Unmarshal(status.Result, &task)
	if task.Status != TaskWorking {
		t.Fatalf("status %+v", task)
	}
	result := fixture.peer.call("tasks/result", map[string]any{"taskId": created.Task.TaskID})
	if result.Error != nil {
		t.Fatal(result.Error)
	}
	var delivered struct {
		Meta              map[string]map[string]string `json:"_meta"`
		StructuredContent BuildResult                  `json:"structuredContent"`
	}
	if err := json.Unmarshal(result.Result, &delivered); err != nil || !delivered.StructuredContent.OK || delivered.Meta[RelatedTaskMeta]["taskId"] != created.Task.TaskID {
		t.Fatalf("result %s %v", result.Result, err)
	}
	done := fixture.peer.call("tasks/get", map[string]any{"taskId": created.Task.TaskID})
	_ = json.Unmarshal(done.Result, &task)
	if task.Status != TaskCompleted {
		t.Fatalf("status %+v", task)
	}
	listed := fixture.peer.call("tasks/list", map[string]any{})
	if !strings.Contains(string(listed.Result), created.Task.TaskID) {
		t.Fatalf("list %s", listed.Result)
	}
	if missing := fixture.peer.call("tasks/get", map[string]any{"taskId": "nope"}); missing.Error == nil || missing.Error.Code != CodeInvalidParams {
		t.Fatalf("an unknown task is invalid params: %+v", missing)
	}
	notifications := append(reply.Notifications, status.Notifications...)
	notifications = append(notifications, result.Notifications...)
	seen := false
	for _, raw := range notifications {
		seen = seen || strings.Contains(string(raw), "notifications/tasks/status")
	}
	if !seen {
		t.Fatalf("a finished task is announced: %s", notifications)
	}
}

func TestCancellingATaskStopsItsChild(t *testing.T) {
	script := `trap 'echo interrupted > "$MARKS/interrupted"; exit 130' INT; echo go > "$MARKS/running"; while :; do sleep 0.05; done`
	fixture := newBuildSession(t, script, nil)
	reply := fixture.callTool("run", map[string]any{"target": "a:b"}, map[string]any{"task": map[string]any{}})
	var created CreateTaskResult
	_ = json.Unmarshal(reply.Result, &created)
	waitForFile(t, filepath.Join(fixture.marks, "running"))
	cancelled := fixture.peer.call("tasks/cancel", map[string]any{"taskId": created.Task.TaskID})
	var task Task
	_ = json.Unmarshal(cancelled.Result, &task)
	if cancelled.Error != nil || task.Status != TaskCancelled {
		t.Fatalf("cancel %+v %s", cancelled.Error, cancelled.Result)
	}
	waitForFile(t, filepath.Join(fixture.marks, "interrupted"))
	again := fixture.peer.call("tasks/cancel", map[string]any{"taskId": created.Task.TaskID})
	if again.Error == nil || again.Error.Code != CodeInvalidParams {
		t.Fatalf("a finished task cannot be cancelled again: %+v", again)
	}
}

func TestTaskSupportIsEnforcedPerTool(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	reply := fixture.callTool("ticket_open", map[string]any{"emoji": "x", "title": "t", "prompt": "p", "goal": "g"}, map[string]any{"task": map[string]any{}})
	if reply.Error == nil || reply.Error.Code != CodeInvalidParams {
		t.Fatalf("a tool without task support refuses a task: %+v", reply)
	}
}

func TestAFaultyToolDoesNotEndTheConnection(t *testing.T) {
	fixture := newBuildSession(t, reportScript, nil)
	if err := fixture.server.RegisterTool(Tool{Name: "boom", InputSchema: Schema{Type: "object"}}, func(context.Context, CallToolParams, ProgressReporter) (CallToolResult, error) {
		var nothing map[string]int
		nothing["x"] = 1
		return CallToolResult{}, nil
	}); err != nil {
		t.Fatal(err)
	}
	reply := fixture.callTool("boom", map[string]any{}, nil)
	if reply.Error == nil || reply.Error.Code != CodeInternalError {
		t.Fatalf("a panic is an internal error: %+v", reply)
	}
	_, build := decodeToolResult(t, fixture.callTool("run", map[string]any{"target": "a:b"}, nil))
	if !build.OK {
		t.Fatalf("the connection must survive: %+v", build)
	}
}

func TestSlotsLimitConcurrentChildrenAndCallsWaitForOne(t *testing.T) {
	fixture := newBuildSession(t, `sleep 0.3; echo ok`, func(policy *Policy) { policy.Limits.Concurrent = 1 })
	first := fixture.peer.begin("tools/call", map[string]any{"name": "run", "arguments": map[string]any{"target": "a:b"}, "_meta": map[string]any{"progressToken": "one"}})
	second := fixture.peer.begin("tools/call", map[string]any{"name": "run", "arguments": map[string]any{"target": "c:d"}, "_meta": map[string]any{"progressToken": "two"}})
	replies := []rpcReply{fixture.peer.await(first), fixture.peer.await(second)}
	for _, reply := range replies {
		if _, build := decodeToolResult(t, reply); !build.OK {
			t.Fatalf("both calls complete: %+v", build)
		}
	}
	waited := false
	for _, reply := range replies {
		for _, step := range reply.progress() {
			waited = waited || strings.Contains(step.Message, "waiting for a free slot")
		}
	}
	if !waited {
		t.Fatal("the second call reports that it waits for a slot")
	}
}

// #endregion 🔌️BuildSessionTests
