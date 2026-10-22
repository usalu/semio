// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The build surface of the repo MCP server: the tools derived from the neutral operations catalogue, run through the `repo` executable itself with progress, cancellation and the calling agent's identity.

// #endregion 🧲️Header

package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

// #region 🔨️BuildSurface

// 🔨️BuildSurface is the registered build tools of one server and everything a call needs.
type BuildSurface struct {
	Policy    Policy
	Catalogue Catalogue
	Tools     []BuildTool
	Runner    ProcessRunner
	Binary    string
	Prefix    []string
	Root      string
	Environ   func() []string
	Sessions  SessionOpener
	Process   int
	Parent    int
	gate      chan struct{}
}

// 🔨️PolicyPath is where the authored policy lives, relative to the repository root.
const PolicyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json"

// 🔨️LoadBuildSurface reads the policy and the catalogue it names and derives the tools; a failure leaves the rest of the server unaffected.
func LoadBuildSurface(root string) (*BuildSurface, error) {
	policyData, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(PolicyPath)))
	if err != nil {
		return nil, err
	}
	policy, err := ParsePolicy(policyData)
	if err != nil {
		return nil, err
	}
	catalogueData, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(policy.Catalogue)))
	if err != nil {
		return nil, err
	}
	catalogue, err := ParseCatalogue(catalogueData)
	if err != nil {
		return nil, err
	}
	return NewBuildSurface(root, catalogue, policy)
}

// 🔨️NewBuildSurface derives the tools and prepares the production runner.
func NewBuildSurface(root string, catalogue Catalogue, policy Policy) (*BuildSurface, error) {
	tools, err := Derive(catalogue, policy)
	if err != nil {
		return nil, err
	}
	binary, err := os.Executable()
	if err != nil {
		return nil, err
	}
	surface := &BuildSurface{Policy: policy, Catalogue: catalogue, Tools: tools, Runner: ExecRunner{}, Binary: binary, Root: root, Environ: os.Environ, Process: os.Getpid(), Parent: os.Getppid(), gate: make(chan struct{}, policy.Limits.Concurrent)}
	surface.Sessions = &surfaceSessions{surface: surface, known: map[SessionRequest]AgentIdentity{}}
	return surface, nil
}

// 🪪️Agent opens the agent session of a call: the `agent` argument names it, else the connection does (client name and process ids; the client's own session variable wins inside `repo session open`). A session that cannot be opened leaves the call without an address, so the child resolves its own, and the reason is returned as a hint.
func (surface *BuildSurface) Agent(ctx context.Context, explicit string) (AgentIdentity, string) {
	request := SessionRequest{Agent: explicit}
	if explicit == "" {
		client := ""
		if session, ok := SessionFrom(ctx); ok {
			client = session.ClientInfo().Name
		}
		request = SessionRequest{Client: client, ClientProcess: surface.Parent, Process: surface.Process}
		if client == "" {
			request.Client = "agent"
		}
	}
	identity, err := surface.Sessions.Open(ctx, request)
	if err != nil {
		return AgentIdentity{ID: "unattached"}, "agent session not opened (" + err.Error() + "); the command attaches to its own session"
	}
	return identity, ""
}

// 🌿️Environment builds the environment of a child: the inherited one without the authored strip list, the authored constants, then the call's own variables.
func (surface *BuildSurface) Environment(invocation Invocation, agent AgentIdentity) []string {
	values := map[string]string{}
	order := []string{}
	set := func(name, value string) {
		if _, exists := values[name]; !exists {
			order = append(order, name)
		}
		values[name] = value
	}
	stripped := map[string]bool{}
	for _, name := range surface.Policy.Strip {
		stripped[name] = true
	}
	for _, entry := range surface.Environ() {
		name, value, found := strings.Cut(entry, "=")
		if found && name != "" && !stripped[name] {
			set(name, value)
		}
	}
	for name, value := range surface.Policy.Set {
		set(name, value)
	}
	for name, value := range agent.Environment(surface.Policy.Identity.Environment) {
		set(name, value)
	}
	for name, value := range invocation.Environment {
		set(name, value)
	}
	environment := make([]string, 0, len(order))
	for _, name := range order {
		environment = append(environment, name+"="+values[name])
	}
	return environment
}

// 📢️throttle forwards at most one progress report per interval, always the first and always the latest: a report that arrives too early waits for the interval to pass instead of being dropped.
type throttle struct {
	mu       sync.Mutex
	last     time.Time
	interval time.Duration
	count    float64
	pending  string
	closed   bool
	timer    *time.Timer
	deliver  func(float64, string)
}

func (gate *throttle) send(message string) {
	gate.mu.Lock()
	defer gate.mu.Unlock()
	if gate.closed {
		return
	}
	wait := gate.interval - time.Since(gate.last)
	if gate.last.IsZero() || wait <= 0 {
		gate.emit(message)
		return
	}
	gate.pending = message
	if gate.timer == nil {
		gate.timer = time.AfterFunc(wait, func() {
			gate.mu.Lock()
			defer gate.mu.Unlock()
			gate.timer = nil
			if gate.pending != "" && !gate.closed {
				gate.emit(gate.pending)
			}
		})
	}
}

// emit delivers one report; the caller holds the lock.
func (gate *throttle) emit(message string) {
	gate.pending = ""
	gate.last = time.Now()
	gate.count++
	gate.deliver(gate.count, message)
}

// stop silences the throttle: nothing is sent after the call has ended.
func (gate *throttle) stop() {
	gate.mu.Lock()
	defer gate.mu.Unlock()
	gate.closed = true
	if gate.timer != nil {
		gate.timer.Stop()
		gate.timer = nil
	}
}

// finish delivers the waiting report, then the closing one, in order.
func (gate *throttle) finish(message string) {
	gate.mu.Lock()
	defer gate.mu.Unlock()
	if gate.timer != nil {
		gate.timer.Stop()
		gate.timer = nil
	}
	if gate.pending != "" {
		gate.emit(gate.pending)
	}
	gate.emit(message)
	gate.closed = true
}

func (surface *BuildSurface) handler(tool BuildTool) ToolHandler {
	return func(ctx context.Context, params CallToolParams, progress ProgressReporter) (CallToolResult, error) {
		invocation, err := tool.Plan(params.Arguments)
		if err != nil {
			var refusal *ArgumentError
			if errors.As(err, &refusal) {
				return refused(refusal.Message), nil
			}
			return CallToolResult{}, err
		}
		agent, sessionHint := surface.Agent(ctx, invocation.Agent)
		limits := surface.Policy.Limits
		report := &throttle{interval: time.Duration(limits.ProgressIntervalMs) * time.Millisecond, deliver: func(value float64, message string) { _ = progress.Report(ctx, value, nil, message) }}
		send := report.send
		defer report.stop()
		select {
		case surface.gate <- struct{}{}:
		default:
			send(fmt.Sprintf("waiting for a free slot (%d running)", cap(surface.gate)))
			select {
			case surface.gate <- struct{}{}:
			case <-ctx.Done():
				return CallToolResult{}, ctx.Err()
			}
		}
		defer func() { <-surface.gate }()
		send("started: repo " + strings.Join(invocation.Argv, " "))
		spec := ProcessSpec{Binary: surface.Binary, Prefix: surface.Prefix, Argv: invocation.Argv, Dir: surface.Root, Env: surface.Environment(invocation, agent), Grace: time.Duration(limits.CancelGraceMs) * time.Millisecond, Heartbeat: time.Duration(limits.HeartbeatMs) * time.Millisecond, HeadMax: limits.CaptureBytes, StdoutMax: limits.StdoutBytes, StderrMax: limits.StderrBytes}
		outcome := surface.Runner.Run(ctx, spec, func(event ProcessEvent) {
			message := event.Message
			if message == "" {
				return
			}
			if event.Kind == "heartbeat" {
				message = "running: " + message
			}
			send(message)
		})
		if outcome.Cancelled && ctx.Err() != nil {
			return CallToolResult{}, ctx.Err()
		}
		result := NewBuildResult(tool, invocation, agent, outcome)
		if sessionHint != "" {
			result.Hint = strings.TrimSpace(result.Hint + " " + sessionHint)
		}
		structured, err := json.Marshal(result)
		if err != nil {
			return CallToolResult{}, err
		}
		report.finish("finished: exit " + fmt.Sprint(result.ExitCode))
		return CallToolResult{Content: []Content{{Type: "text", Text: result.Text(limits.TextBytes)}}, StructuredContent: structured, IsError: !result.OK, OutputSchemaDeclared: true}, nil
	}
}

// ❌️refused answers a call whose arguments are wrong with a tool error the agent can read and correct.
func refused(message string) CallToolResult {
	return CallToolResult{Content: []Content{{Type: "text", Text: "Invalid arguments / Ungültige Argumente: " + message}}, IsError: true}
}

// 📚️Register adds every derived tool, and the resource that lists what is exposed and what is not, to a server.
func (surface *BuildSurface) Register(server *Server) error {
	for _, tool := range surface.Tools {
		if err := server.RegisterTool(tool.Schema, surface.handler(tool)); err != nil {
			return err
		}
	}
	listing, err := surface.Listing()
	if err != nil {
		return err
	}
	return server.RegisterResource(Resource{URI: "repo://build-surface", Name: "build-surface", Title: "Build surface", Description: "The build tools derived from the neutral repo catalogue and the operations left out, with reasons in English and German.", MIMEType: "application/json"}, func(ctx context.Context, _ ReadResourceParams, _ ProgressReporter) (ReadResourceResult, error) {
		return ReadResourceResult{Contents: []ResourceContent{{URI: "repo://build-surface", MIMEType: "application/json", Text: string(listing)}}}, nil
	})
}

// 📚️Listing is the content of `repo://build-surface`.
func (surface *BuildSurface) Listing() ([]byte, error) {
	type exposed struct {
		Tool       string `json:"tool"`
		Operation  string `json:"operation"`
		Subcommand string `json:"subcommand,omitempty"`
		Kind       string `json:"kind"`
	}
	type excluded struct {
		Operation  string `json:"operation"`
		Subcommand string `json:"subcommand,omitempty"`
		Reason     Text   `json:"reason"`
	}
	listing := struct {
		Version  int        `json:"version"`
		Tools    []exposed  `json:"tools"`
		Excluded []excluded `json:"excluded"`
	}{Version: 1}
	for _, tool := range surface.Tools {
		listing.Tools = append(listing.Tools, exposed{tool.Name, tool.Operation, tool.Subcommand, tool.Kind})
	}
	for _, row := range surface.Policy.Excluded {
		listing.Excluded = append(listing.Excluded, excluded{row.Operation, row.Subcommand, row.Reason})
	}
	return json.MarshalIndent(listing, "", "  ")
}

// #endregion 🔨️BuildSurface
