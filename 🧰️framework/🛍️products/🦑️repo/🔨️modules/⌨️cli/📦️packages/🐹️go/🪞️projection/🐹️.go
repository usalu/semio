// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪞️projection carries the projection commands, argument projection and event stream rendering of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	io "io"
	strconv "strconv"
	strings "strings"
	time "time"

	eventspkg "github.com/usalu/semio/repo/events"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	hooks "github.com/usalu/semio/repo/hooks"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	testrunner "github.com/usalu/semio/repo/testrunner"
)

// #region 🧪️Projection

// 📃️projectionListCommand builds one `list` subcommand: a thin front for a single read document,
// exactly like every other read verb. The reference groups (`ticket`, `todo`, `goal`, `contributor`,
// `folder`, `file`, `section`, `draft`) can open, close and change their aggregate but cannot list
// it — listing goes through the monorepo-tree `list` and `search` verbs instead — which leaves a
// hole in every group. The Rust twin closes it, so the projected tree closes it here too.
func projectionListCommand(short string, query string, factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "list",
		Short: short,
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			return runGraphQL(cmd, factory, config, query, map[string]interface{}{})
		},
	}
}

// 📃️projectionTicketListCommand is `ticket list` with the status and date filters the reference
// ticket verbs already accept elsewhere.
func projectionTicketListCommand(factory EngineFactory, config *Config) *Command {
	command := projectionListCommand("List tickets", "query Tickets { repo { tickets { id slug title status prompt } } }", factory, config)
	command.RunE = func(cmd *Command, args []string) error {
		variables := map[string]interface{}{}
		for _, name := range []string{"year", "month", "day"} {
			if value, _ := cmd.Flags().GetInt(name); value > 0 {
				variables[name] = value
			}
		}
		if status, _ := cmd.Flags().GetString("status"); status != "" {
			variables["status"] = strings.ToUpper(status)
		}
		return runGraphQL(cmd, factory, config, "query Tickets { repo { tickets { id slug title status prompt } } }", variables)
	}
	command.Flags().String("status", "", "Filter by status (open or closed)")
	command.Flags().Int("year", 0, "Ticket year")
	command.Flags().Int("month", 0, "Ticket month")
	command.Flags().Int("day", 0, "Ticket day")
	return command
}

// 🎫️ticketShowQuery reads the stored document of one ticket.
const ticketShowQuery = "query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id slug year month day title status goal parent prompt summary path uri } }"

// 📄️ticketFilesQuery reads every file one ticket carries.
const ticketFilesQuery = "query TicketFiles($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id files { id path name kind } } }"

// 🎫️singleTicketCommand is a ticket verb whose operand or `--slug` flag names one ticket.
func singleTicketCommand(name string, short string, query string, factory EngineFactory, config *Config) *Command {
	command := &Command{
		Use:   name + " [path]",
		Short: short,
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			slug, _ := cmd.Flags().GetString("slug")
			if len(args) > 0 {
				parts := strings.Split(args[0], "/")
				if len(parts) >= 4 {
					tail := len(parts) - 4
					year, _ = strconv.Atoi(parts[tail])
					month, _ = strconv.Atoi(parts[tail+1])
					day, _ = strconv.Atoi(parts[tail+2])
					slug = parts[tail+3]
				}
			}
			if slug == "" {
				return fmt.Errorf("missing ticket path or --slug")
			}
			return runGraphQL(cmd, factory, config, query, map[string]interface{}{"year": year, "month": month, "day": day, "slug": slug})
		},
	}
	command.Flags().Int("year", 0, "Ticket year")
	command.Flags().Int("month", 0, "Ticket month")
	command.Flags().Int("day", 0, "Ticket day")
	command.Flags().String("slug", "", "Ticket slug")
	return command
}

// 📑️projectionSectionListCommand is `section list`, whose operand or `--file` flag names the file
// whose sections are listed.
func projectionSectionListCommand(factory EngineFactory, config *Config) *Command {
	command := &Command{
		Use:   "list",
		Short: "List the sections of a file",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			if len(args) > 0 {
				file = args[0]
			}
			query := "query Sections($path: String!) { file(path: $path) { sections { id path name range { start end } definitions { name } children { name } } } }"
			return runGraphQL(cmd, factory, config, query, map[string]interface{}{"path": file})
		},
	}
	command.Flags().String("file", "", "File path")
	return command
}

// 🌳️ProjectionRoot builds the declared repo command tree the language-agnostic harness measures.
//
// It is [NewRootWithConfig] itself: the whole verb surface is registered there, so what the harness
// measures and what `semio-repo` serves are one tree, and a wizard leaf that spawns the binary can
// be checked against `semio-repo --help`.
func ProjectionRoot() *Command {
	root := NewRoot(defaultEngineFactory)
	root.SetOut(io.Discard)
	root.SetErr(io.Discard)
	root.resetFlagsRecursive()
	return root
}

// 📖️projectionHelpText is [Command.Help] as a string rather than a write to the command's writer.
func projectionHelpText(command *Command) string {
	var text strings.Builder
	fmt.Fprintf(&text, "%s\n\nUsage:\n  %s", command.Short, command.Use)
	if len(command.children) > 0 {
		fmt.Fprintln(&text, "\n\nCommands:")
		for _, child := range command.children {
			fmt.Fprintf(&text, "  %-20s %s\n", child.Name(), child.Short)
		}
	}
	return text.String()
}

// 🚩️projectionFlagValue reduces one flag value to the JSON shape the projection states: a duration
// is milliseconds, a list is always an array, everything else is its own scalar.
func projectionFlagValue(flag *Flag) interface{} {
	switch value := flag.value.(type) {
	case string:
		return value
	case bool:
		return value
	case int:
		return value
	case []string:
		if value == nil {
			return []string{}
		}
		return value
	case []int:
		if value == nil {
			return []int{}
		}
		return value
	case time.Duration:
		return int64(value / time.Millisecond)
	default:
		return nil
	}
}

// 🧾️ProjectArgv parses one argv against the repo command tree and returns the projection as JSON
// text: the selected command path, the operands, ONLY the flags the caller passed, and whether
// `--help` short-circuited. Text crosses the boundary on purpose — no serialization type from
// outside this codebase may reach the harness.
func ProjectArgv(argv []string) (string, error) {
	root := ProjectionRoot()
	selected, positional, err := root.selectAndParse(argv)
	help := false
	if err != nil {
		if !errors.Is(err, errHelp) {
			return "", err
		}
		help = true
		positional = nil
	}
	if !help && selected.Args != nil {
		if err := selected.Args(selected, positional); err != nil {
			return "", err
		}
	}
	path := []string{}
	for _, ancestor := range selected.ancestry() {
		path = append(path, ancestor.Name())
	}
	scope := map[string]*Flag{}
	for _, ancestor := range selected.ancestry() {
		if ancestor.persistentFlags == nil {
			continue
		}
		for name, flag := range ancestor.persistentFlags.flags {
			scope[name] = flag
		}
	}
	if selected.flags != nil {
		for name, flag := range selected.flags.flags {
			scope[name] = flag
		}
	}
	flags := map[string]interface{}{}
	for name, flag := range scope {
		if !flag.Changed {
			continue
		}
		flags[name] = projectionFlagValue(flag)
	}
	if positional == nil {
		positional = []string{}
	}
	encoded, err := json.Marshal(map[string]interface{}{"path": path, "positional": positional, "flags": flags, "help": help})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 📖️UsageText is the usage text of one command path below the root, or false when no such command
// is registered.
func UsageText(path []string) (string, bool) {
	command := ProjectionRoot()
	for _, name := range path {
		child := command.child(name)
		if child == nil {
			return "", false
		}
		command = child
	}
	return projectionHelpText(command), true
}

// 🗺️CommandPaths lists every command path of the tree, root first, each as `/`-joined names.
func CommandPaths() []string {
	root := ProjectionRoot()
	paths := []string{""}
	for _, child := range root.children {
		paths = append(paths, child.Name())
		for _, grandchild := range child.children {
			paths = append(paths, child.Name()+"/"+grandchild.Name())
		}
	}
	return paths
}

// 🧹️projectionCompactJSON re-encodes one payload so a fixture's indentation never reaches the
// renderers; the renderers own the bytes, the fixture only owns the values.
func projectionCompactJSON(raw json.RawMessage) json.RawMessage {
	var value interface{}
	if err := json.Unmarshal(raw, &value); err != nil {
		return raw
	}
	var encoded strings.Builder
	encoder := json.NewEncoder(&encoded)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(value); err != nil {
		return raw
	}
	return json.RawMessage(strings.TrimSuffix(encoded.String(), "\n"))
}

// 📡️projectionStream replays a recorded event slice as the channel the renderers consume.
func projectionStream(events []Event) <-chan Event {
	stream := make(chan Event, len(events))
	for _, event := range events {
		stream <- event
	}
	close(stream)
	return stream
}

// 👤️projectionHuman is [HumanRenderer.Render] with the terminal decision and the elapsed time
// supplied rather than measured, so one recorded stream renders the same bytes on every host.
func projectionHuman(out, errOut *strings.Builder, events []Event, isTTY bool, verbose bool, elapsedMs int64) int {
	exitCode := 0
	duration := time.Duration(elapsedMs) * time.Millisecond
	for _, event := range events {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			var summary string
			if exitCode != 0 {
				summary = model.RenderTemplate(model.TextTpl, "text/done_failure", map[string]interface{}{"IsTTY": isTTY, "Command": event.Command, "Duration": duration, "ExitCode": exitCode})
			} else {
				summary = model.RenderTemplate(model.TextTpl, "text/done_success", map[string]interface{}{"IsTTY": isTTY, "Command": event.Command, "Duration": duration})
			}
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(out, summary)
			continue
		}
		if event.Kind == KindError && event.Error != nil {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/error", map[string]interface{}{"IsTTY": isTTY, "Message": event.Error.Message}))
			if verbose && event.Error.Detail != "" {
				fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/error_detail", map[string]interface{}{"Detail": event.Error.Detail}))
			}
			continue
		}
		if event.Kind == KindLog && event.Message != "" {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintln(errOut, model.RenderTemplate(model.TextTpl, "text/log", map[string]interface{}{"IsTTY": isTTY, "Message": event.Message}))
			continue
		}
		if event.Kind == KindResult && len(event.Data) > 0 {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprint(out, formatResult(event.Command, event.Data, isTTY))
		}
		if event.Kind == KindProgress && event.Progress != nil {
			if isTTY {
				fmt.Fprint(out, model.RenderTemplate(model.TextTpl, "text/progress_tty", map[string]interface{}{"Percent": event.Progress.Percent, "Current": event.Progress.Current, "Total": event.Progress.Total, "Step": event.Progress.Step}))
			} else if event.Progress.Percent%10 == 0 && event.Progress.Percent > 0 {
				fmt.Fprintln(out, model.RenderTemplate(model.TextTpl, "text/progress", map[string]interface{}{"Percent": event.Progress.Percent, "Step": event.Progress.Step}))
			}
		}
	}
	return exitCode
}

// 🖨️projectionRender renders one recorded stream in one format and returns stdout, stderr and the
// exit code the terminal event carried.
func projectionRender(events []Event, format string, isTTY bool, verbose bool, elapsedMs int64) (string, string, int) {
	var out, errOut strings.Builder
	exitCode := 0
	switch format {
	case "json":
		exitCode, _ = NDJSONRenderer{}.Render(context.Background(), &out, &errOut, projectionStream(events))
	case "md":
		exitCode, _ = MarkdownRenderer{}.Render(context.Background(), &out, &errOut, projectionStream(events))
	default:
		exitCode = projectionHuman(&out, &errOut, events, isTTY, verbose, elapsedMs)
	}
	return out.String(), errOut.String(), exitCode
}

// 🖨️RenderEventStream renders one recorded event stream given as JSON text and returns `out`, `err`
// and `exitCode` as JSON text.
func RenderEventStream(eventsJSON string, format string, isTTY bool, verbose bool, elapsedMs int64) (string, error) {
	var events []Event
	if err := json.Unmarshal([]byte(eventsJSON), &events); err != nil {
		return "", err
	}
	for index := range events {
		if len(events[index].Data) > 0 {
			events[index].Data = projectionCompactJSON(events[index].Data)
		}
	}
	out, errOut, exitCode := projectionRender(events, format, isTTY, verbose, elapsedMs)
	encoded, err := json.Marshal(map[string]interface{}{"out": out, "err": errOut, "exitCode": exitCode})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 🕸️GraphQLRoundtrip executes one document against a recorded repository given as JSON text and
// returns the three renderings of the resulting event stream, plus its exit code, as JSON text.
func GraphQLRoundtrip(recordsJSON string, query string, variablesJSON string) (string, error) {
	recording, err := graphqlpkg.NewRecordingContext([]byte(recordsJSON))
	if err != nil {
		return "", err
	}
	executor, err := graphqlpkg.NewExecutorWithContext(recording.GetRootDir(), recording)
	if err != nil {
		return "", err
	}
	variables := map[string]interface{}{}
	if strings.TrimSpace(variablesJSON) != "" {
		if err := json.Unmarshal([]byte(variablesJSON), &variables); err != nil {
			return "", err
		}
	}
	payload, err := json.Marshal(GraphQLArgs{Query: query, Variables: variables})
	if err != nil {
		return "", err
	}
	var events []Event
	for event := range NewEngine(executor).Run(context.Background(), Request{Command: CmdGraphQL, Args: payload}) {
		events = append(events, event)
	}
	ndjson, _, exitCode := projectionRender(events, "json", false, false, 0)
	human, _, _ := projectionRender(events, "text", false, false, 0)
	markdown, _, _ := projectionRender(events, "md", false, false, 0)
	encoded, err := json.Marshal(map[string]interface{}{"ndjson": ndjson, "human": human, "markdown": markdown, "exitCode": exitCode})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 🤝️McpVerbConversation serves the recorded stdio conversation with the server the `mcp` verb
// builds, and returns the response lines the client would have read.
func McpVerbConversation(rootDir, profile string, requests []string) ([]string, error) {
	kind := providers.McpClientGeneric
	if profile != "" {
		parsed, err := providers.ParseMcpClientKind(profile)
		if err != nil {
			return nil, err
		}
		kind = parsed
	}
	var out strings.Builder
	server,err:=CreateMcpServer(rootDir,kind,DefaultCommandTimeout);if err!=nil { return nil,err }
	if err := Serve(context.Background(),server,io.NopCloser(strings.NewReader(strings.Join(requests, "\n")+"\n")),&out); err != nil {
		return nil, err
	}
	lines := []string{}
	for _, line := range strings.Split(out.String(), "\n") {
		if strings.TrimSpace(line) != "" {
			lines = append(lines, line)
		}
	}
	return lines, nil
}

// 🕸️TestVerbLines answers the invocations the `test` verb would announce for one snapshot and one
// operand list, encoded as JSON text: the `Running: …` lines in order plus the planner's refusals.
// Nothing is executed.
func TestVerbLines(snapshotJSON string, operands []string) (string, error) {
	var snapshot testrunner.FilesystemSnapshot
	if err := json.Unmarshal([]byte(snapshotJSON), &snapshot); err != nil {
		return "", err
	}
	identity := testrunner.PendingIdentity{}
	scopes := snapshot.ResolveTestScopes(operands, identity, identity)
	plan := snapshot.PlanScopes(scopes)
	lines := []string{}
	for _, invocation := range plan.Invocations {
		lines = append(lines, fmt.Sprintf("Running: %s (in %s)", strings.Join(invocation.Argv, " "), invocation.Cwd))
	}
	problems := plan.Problems
	if problems == nil {
		problems = []string{}
	}
	encoded, err := json.Marshal(map[string]interface{}{"lines": lines, "problems": problems})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 📤️ExportRecords answers the event batch the `export` verb would append for one recorded
// repository, encoded as JSON text: the snapshot digest, the per-kind counts in taxonomy order and
// every input id.
func ExportRecords(recordsJSON string) (string, error) {
	recording, err := graphqlpkg.NewRecordingContext([]byte(recordsJSON))
	if err != nil {
		return "", err
	}
	entities := repoExportSource{repo: recording}.ExportEntities()
	snapshot, err := eventspkg.BuildExportSnapshot(context.Background(), entities)
	if err != nil {
		return "", err
	}
	counts := []map[string]interface{}{}
	for _, kind := range []string{"technology", "bundle", "folder", "file", "section", "definition"} {
		counts = append(counts, map[string]interface{}{"kind": kind, "count": snapshot.Counts[kind]})
	}
	ids := []string{}
	records := []string{}
	for _, input := range snapshot.Inputs {
		ids = append(ids, input.ID)
		data, err := json.Marshal(input.Data)
		if err != nil {
			return "", err
		}
		records = append(records, input.Kind+"\x01"+string(data))
	}
	encoded, err := json.Marshal(map[string]interface{}{"snapshot": snapshot.Snapshot, "counts": counts, "inputIds": ids, "records": records})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 🪝️HookVerbDispatch answers what the `hook` verb would write for one native invocation, encoded as
// JSON text. The environment and the test-file resolver are the inert ones, so the projection is a
// pure function of the request.
func HookVerbDispatch(requestJSON string) (string, error) {
	var request struct {
		Event    string          `json:"event"`
		Client   string          `json:"client"`
		ToolName string          `json:"toolName"`
		ToolArgs string          `json:"toolArgs"`
		File     string          `json:"file"`
		Parent   string          `json:"parent"`
		Second   string          `json:"second"`
		RepoRoot string          `json:"repoRoot"`
		JSON     bool            `json:"json"`
		Input    json.RawMessage `json:"input"`
	}
	if err := json.Unmarshal([]byte(requestJSON), &request); err != nil {
		return "", err
	}
	toolName := request.ToolName
	if toolName == "" {
		toolName = hooks.ExtractToolName(request.Input)
	}
	event, resolvedParent, err := hooks.ResolveHookEvent(request.Event, request.Client, toolName, request.Input)
	if err != nil {
		return "", err
	}
	parentInfo := request.Parent
	if parentInfo == "" {
		parentInfo = resolvedParent
	}
	hookContext := hooks.HookContext{
		Event:      string(event),
		Client:     request.Client,
		Second:     request.Second,
		RepoRoot:   request.RepoRoot,
		ToolName:   toolName,
		ToolArgs:   request.ToolArgs,
		FilePath:   request.File,
		ParentInfo: parentInfo,
		Input:      request.Input,
	}
	result := hooks.DispatchHook(hookContext, hooks.InertEnvironment{}, hooks.InertTestFileResolver{})
	native := hooks.ResolveNativeEventName(request.Client, event, parentInfo, request.Input)
	output := hooks.RenderHookOutput(request.Client, event, parentInfo, native, result, request.JSON)
	encoded, err := json.Marshal(map[string]interface{}{"stdout": output.Stdout, "stderr": output.Stderr, "exitCode": output.ExitCode})
	if err != nil {
		return "", err
	}
	return string(encoded), nil
}

// 🏜️McpVerbDryRun runs `mcp --dry-run` through the command tree and returns its exit code.
func McpVerbDryRun() int {
	root := ProjectionRoot()
	root.SetArgs([]string{"mcp", "--dry-run"})
	if err := root.Execute(); err != nil {
		return 1
	}
	return 0
}

// #endregion 🧪️Projection
