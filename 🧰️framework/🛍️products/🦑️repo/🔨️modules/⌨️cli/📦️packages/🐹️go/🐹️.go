// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⌨️cli is the cli domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	io "io"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strconv "strconv"
	strings "strings"
	sync "sync"
	time "time"

	codebasepkg "github.com/usalu/semio/repo/codebase"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	eventspkg "github.com/usalu/semio/repo/events"
	goalspkg "github.com/usalu/semio/repo/goals"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	hooks "github.com/usalu/semio/repo/hooks"
	languagespkg "github.com/usalu/semio/repo/languages"
	metricspkg "github.com/usalu/semio/repo/metrics"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	providers "github.com/usalu/semio/repo/providers"
	statutes "github.com/usalu/semio/repo/statutes"
	testrunner "github.com/usalu/semio/repo/testrunner"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	treepkg "github.com/usalu/semio/repo/tree"
	workspace "github.com/usalu/semio/repo/workspace"
	yamlpkg "github.com/usalu/semio/repo/yaml"
)

// #region 🚚️Split

// #region 📤️Event Export

type ExportResult struct {
	Path         string `json:"path"`
	Snapshot     string `json:"snapshot"`
	Technologies int    `json:"technologies"`
	Bundles      int    `json:"bundles"`
	Folders      int    `json:"folders"`
	Files        int    `json:"files"`
	Sections     int    `json:"sections"`
	Definitions  int    `json:"definitions"`
}

// 🗂️repoExportSource adapts a RepoContext to the events.ExportSource port.
type repoExportSource struct{ repo model.RepoContext }

// 📋️ExportEntities enumerates every snapshot entity in taxonomy order.
func (source repoExportSource) ExportEntities() []eventspkg.ExportEntity {
	var entities []eventspkg.ExportEntity
	for _, value := range source.repo.GetTechnologies() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "technology", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetBundles() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "bundle", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetFolders() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "folder", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetFiles() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "file", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetSections() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "section", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetDefinitions() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "definition", ID: value.GetID(), Value: value})
	}
	return entities
}

// 📦️ExportToEventLog writes a deterministic snapshot as append-only events.
func ExportToEventLog(outputPath string, repo model.RepoContext) (*ExportResult, error) {
	return ExportToEventLogContext(context.Background(), outputPath, repo, nil)
}

// ⏯️ExportToEventLogContext writes an atomic, cancellable event batch with progress.
func ExportToEventLogContext(ctx context.Context, outputPath string, repo model.RepoContext, progress func(eventspkg.Progress)) (*ExportResult, error) {
	if outputPath == "" {
		outputPath = filepath.Join(repo.GetRootDir(), "repo.events.jsonl")
	}
	snapshot, err := eventspkg.BuildExportSnapshot(ctx, repoExportSource{repo: repo}.ExportEntities())
	if err != nil {
		return nil, err
	}
	if _, err := (eventspkg.Store{Path: outputPath}).Append(ctx, snapshot.Inputs, progress); err != nil {
		return nil, err
	}
	return &ExportResult{
		Path:         outputPath,
		Snapshot:     snapshot.Snapshot,
		Technologies: snapshot.Counts["technology"],
		Bundles:      snapshot.Counts["bundle"],
		Folders:      snapshot.Counts["folder"],
		Files:        snapshot.Counts["file"],
		Sections:     snapshot.Counts["section"],
		Definitions:  snapshot.Counts["definition"],
	}, nil
}

// 🔷️ToolExport emits the repository snapshot event stream.
func ToolExport(outputPath string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventExportStarting, "repo-cli", eventspkg.FilePayload{Path: outputPath})
	output := workspace.NewOutput()
	output.Info("\n📦️Exporting repo to event log...")
	result, err := ExportToEventLog(outputPath, graphqlpkg.NewRepoContext(workspace.RootDir))
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventExportEnded, "repo-cli", eventspkg.FilePayload{Path: outputPath})
	output.Success(fmt.Sprintf("Exported to: %s", result.Path))
	output.Plain(fmt.Sprintf("  Technologies: %d", result.Technologies))
	output.Plain(fmt.Sprintf("  Bundles: %d", result.Bundles))
	output.Plain(fmt.Sprintf("  Folders: %d", result.Folders))
	output.Plain(fmt.Sprintf("  Files: %d", result.Files))
	output.Plain(fmt.Sprintf("  Sections: %d", result.Sections))
	output.Plain(fmt.Sprintf("  Definitions: %d", result.Definitions))
	return workspace.ToolResult{Output: *output, Data: result}
}

// #endregion 📤️Event Export

// #region 🔑️Engine Events

// 🏷️Kind represents a kind value.
type Kind string

const KindStart Kind = "start"

const KindLog Kind = "log"

const KindProgress Kind = "progress"

const KindResult Kind = "result"

const KindArtifact Kind = "artifact"

const KindError Kind = "error"

const KindDone Kind = "done"

// 💿️Event holds the data fields for a event record.
type Event struct {
	Kind     Kind            `json:"kind"`
	Command  string          `json:"command,omitempty"`
	ID       string          `json:"id,omitempty"`
	Message  string          `json:"message,omitempty"`
	Level    string          `json:"level,omitempty"`
	Progress *Progress       `json:"progress,omitempty"`
	Data     json.RawMessage `json:"data,omitempty"`
	Artifact *Artifact       `json:"artifact,omitempty"`
	Error    *ErrPayload     `json:"error,omitempty"`
	Done     *DonePayload    `json:"done,omitempty"`
}

// 🔶️Progress holds the data fields for a progress record.
type Progress struct {
	Current int    `json:"current,omitempty"`
	Total   int    `json:"total,omitempty"`
	Percent int    `json:"percent,omitempty"`
	Step    string `json:"step,omitempty"`
}

// 🏺️Artifact holds the data fields for a artifact record.
type Artifact struct {
	Type string `json:"type"`
	URI  string `json:"uri"`
	Note string `json:"note,omitempty"`
}

type ErrPayload struct {
	Code    string `json:"code"`
	Message string `json:"message"`
	Detail  string `json:"detail,omitempty"`
	Fatal   bool   `json:"fatal,omitempty"`
}

// 📦️DonePayload holds the data fields for a done payload record.
type DonePayload struct {
	ExitCode int    `json:"exit_code"`
	Status   string `json:"status"`
}

// #endregion 🔑️Engine Events

// #region 🕌️Engine Errors

// ❌️ErrorCode represents a error code value.
type ErrorCode string

const ErrInternal ErrorCode = "E_INTERNAL"

const ErrParse ErrorCode = "E_PARSE"

const ErrCanceled ErrorCode = "E_CANCELED"

const ErrNetwork ErrorCode = "E_NETWORK"

const ErrAuth ErrorCode = "E_AUTH"

// #endregion 🕌️Engine Errors

// #region 🎸️Engine Requests

// 🔷️Command represents a command value.
type EngineCommand string

const CmdGraphQL EngineCommand = "graphql"

const CmdAnalyze EngineCommand = "analyze"

const CmdAutofix EngineCommand = "autofix"

const CmdPolicy EngineCommand = "policy"

const CmdTicket EngineCommand = "ticket"

const CmdBundle EngineCommand = "bundle"

const CmdFolder EngineCommand = "folder"

const CmdFile EngineCommand = "file"

const CmdSection EngineCommand = "section"

const CmdDef EngineCommand = "definition"

type Request struct {
	Command  EngineCommand
	Args     json.RawMessage
	RepoRoot string
	Verbose  bool
}

// 🕸️GraphQLArgs holds the data fields for a graph q l args record.
type GraphQLArgs struct {
	Query     string         `json:"query"`
	Variables map[string]any `json:"variables,omitempty"`
}

// #endregion 🎸️Engine Requests

// #region 🎖️Engine

// 🕸️GraphQLExecutor defines the interface contract for graph q l executor operations.
type GraphQLExecutor interface {
	Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error)
}

// 💿️Engine holds the data fields for a engine record.
type Engine struct {
	GraphQL GraphQLExecutor
}

// 🔷️NewEngine MUST initialize all required fields and return a valid Engine.
// 🆕️NewEngine creates and returns a new Engine instance.
func NewEngine(graphql GraphQLExecutor) *Engine {
	return &Engine{GraphQL: graphql}
}

// 📡️Run MUST emit start, result or error, and done events in order.
// 📤️Run dispatches the request and returns an event channel.
func (e *Engine) Run(ctx context.Context, req Request) <-chan Event {
	out := make(chan Event)
	go func() {
		defer func() {
			if recovered := recover(); recovered != nil {
				e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "internal error", Detail: fmt.Sprintf("%v", recovered), Fatal: true})
				e.emitDone(out, exitCodeError, "error")
			}
			close(out)
		}()

		e.emitStart(out, req)

		if ctx.Err() != nil {
			e.emitError(out, req, ErrPayload{Code: string(ErrCanceled), Message: ctx.Err().Error(), Fatal: true})
			e.emitDone(out, exitCodeCanceled, "canceled")
			return
		}

		switch req.Command {
		case CmdGraphQL, CmdAnalyze, CmdAutofix, CmdPolicy, CmdTicket, CmdBundle, CmdFolder, CmdFile, CmdSection, CmdDef:
			e.runGraphQL(ctx, req, out)
		default:
			e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "unsupported command", Fatal: true})
			e.emitDone(out, exitCodeError, "error")
		}
	}()
	return out
}

// 🔶️runGraphQL holds the data fields for a runGraphQL record.
func (e *Engine) runGraphQL(ctx context.Context, req Request, out chan<- Event) {
	var args GraphQLArgs
	if err := json.Unmarshal(req.Args, &args); err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrParse), Message: "invalid arguments", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeUsage, "error")
		return
	}
	if e.GraphQL == nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "graphql executor missing", Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	result, err := e.GraphQL.Execute(ctx, args.Query, args.Variables)
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	payload, err := json.Marshal(result)
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "failed to encode result", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	out <- Event{Kind: KindResult, Command: string(req.Command), Data: payload}
	e.emitDone(out, exitCodeOK, "ok")
}

func (e *Engine) emitStart(out chan<- Event, req Request) {
	out <- Event{Kind: KindStart, Command: string(req.Command)}
}

// ❌️emitError holds the data fields for a emitError record.
func (e *Engine) emitError(out chan<- Event, req Request, payload ErrPayload) {
	out <- Event{Kind: KindError, Command: string(req.Command), Error: &payload}
}

func (e *Engine) emitDone(out chan<- Event, code int, status string) {
	out <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: code, Status: status}}
}

const exitCodeOK = 0

const exitCodeError = 1

const exitCodeUsage = 2

const exitCodeCanceled = 130

// #endregion 🎖️Engine

// #region 🌧️Cli Adapter

// ⚙️Config holds the data fields for a config record.
type Config struct {
	Format  string
	Verbose bool
	Repo    string
	Timeout time.Duration
}

// DefaultCommandTimeout is the wall-clock budget for MCP tool calls, hooks, and other repo CLI work when --timeout is not set.
const DefaultCommandTimeout = 5 * time.Minute

// 📋️IsJSON MUST return true only when the condition is met.
// ❓️IsJSON reports whether the Config is j s o n.
func (c *Config) IsJSON() bool {
	return c.Format == "json"
}

// 📰️IsMarkdown MUST return true only when the condition is met.
// 🐙️IsMarkdown reports whether the Config is markdown.
func (c *Config) IsMarkdown() bool {
	return c.Format == "md"
}

// 📝️IsText MUST return true only when the condition is met.
// 🔤️IsText reports whether the Config is text.
func (c *Config) IsText() bool {
	return c.Format == "text"
}

// 🏭️EngineFactory is a function type for engine factory callbacks.
type EngineFactory func(Config) (*Engine, error)

// 🌱️NewRoot MUST initialize all required fields and return a valid Root.
// 🌱️NewRoot creates and returns a new Root instance.
func NewRoot(factory EngineFactory) *Command {
	root, _ := NewRootWithConfig(factory)
	return root
}

// 🔷️NewRootWithConfig MUST initialize all required fields and return a valid RootWithConfig.
// 🆕️NewRootWithConfig creates and returns a new RootWithConfig instance.
func NewRootWithConfig(factory EngineFactory) (*Command, *Config) {
	config := Config{}
	root := &Command{
		Use:           "repo",
		Short:         "Monorepo CLI for Compose",
		SilenceUsage:  true,
		SilenceErrors: true,
	}
	root.PersistentFlags().StringVar(&config.Format, "format", "md", "Output format: md, text, json")
	root.PersistentFlags().BoolP("md", "", false, "Shorthand for --format md")
	root.PersistentFlags().BoolP("text", "", false, "Shorthand for --format text")
	root.PersistentFlags().BoolP("json", "", false, "Shorthand for --format json")
	root.PersistentPreRunE = func(cmd *Command, args []string) error {
		if b, _ := cmd.Flags().GetBool("json"); b {
			config.Format = "json"
		} else if b, _ := cmd.Flags().GetBool("text"); b {
			config.Format = "text"
		} else if b, _ := cmd.Flags().GetBool("md"); b {
			config.Format = "md"
		}
		return nil
	}
	root.PersistentFlags().BoolVar(&config.Verbose, "verbose", false, "Verbose output")
	root.PersistentFlags().StringVar(&config.Repo, "repo", "", "Repo root path")
	root.PersistentFlags().DurationVar(&config.Timeout, "timeout", DefaultCommandTimeout, "Timeout for command execution")
	root.AddCommand(mcpCommand(factory, &config))
	root.AddCommand(graphqlCommand(factory, &config))
	root.AddCommand(testCommand(factory, &config))
	root.AddCommand(ticketCommand(factory, &config))
	root.AddCommand(todoCommand(factory, &config))
	root.AddCommand(goalCommand(factory, &config))
	root.AddCommand(contributorCommand(factory, &config))
	root.AddCommand(folderCommand(factory, &config))
	root.AddCommand(fileCommand(factory, &config))
	root.AddCommand(sectionCommand(factory, &config))
	root.AddCommand(moveCommand(factory, &config))
	root.AddCommand(integrateCommand(factory, &config))
	root.AddCommand(extractCommand(factory, &config))
	root.AddCommand(renameCommand(factory, &config))
	root.AddCommand(syncCommand(factory, &config))
	root.AddCommand(searchCommand(factory, &config))
	root.AddCommand(listCommand(factory, &config))
	root.AddCommand(queryCommand(factory, &config))
	root.AddCommand(treeCommand(factory, &config))
	root.AddCommand(exportCommand(factory, &config))
	root.AddCommand(hookCommand(factory, &config))
	root.AddCommand(mermaidCommand(factory, &config))
	root.AddCommand(locCommand(factory, &config))
	root.AddCommand(technologyCommand(factory, &config))
	root.AddCommand(bundleCommand(factory, &config))
	root.AddCommand(analyzeCmd)
	root.AddCommand(entityEmojisCommand(&config))
	root.AddCommand(configureCommand(factory, &config))
	root.AddCommand(microCommitCommand(factory, &config))
	root.AddCommand(benchmarkCmd)
	root.AddCommand(updateCmd)
	root.AddCommand(authCommand(&config))
	root.AddCommand(&Command{Use: "autofix [scope]", Short: "Apply autofixes for breachs", Args: MaximumNArgs(1), RunE: autofixCmd.RunE})
	root.AddCommand(statuteCommand(factory, &config))
	root.AddCommand(checkpointCommand(factory, &config))
	root.AddCommand(interactionCommand(factory, &config))
	draft := draftCommand(factory, &config)
	draft.AddCommand(projectionListCommand("List drafts", "query Drafts { drafts { id uri } }", factory, &config))
	root.AddCommand(draft)
	root.AddCommand(definitionCommand(factory, &config))
	if microCommit := root.child("micro-commit"); microCommit != nil {
		microCommit.DisableFlagParsing = true
	}
	if entityEmojis := root.child("entity-emojis"); entityEmojis != nil {
		entityEmojis.Args = NoArgs
	}
	root.child("ticket").AddCommand(projectionTicketListCommand(factory, &config), singleTicketCommand("show", "Show one ticket", ticketShowQuery, factory, &config), singleTicketCommand("files", "List the files of one ticket", ticketFilesQuery, factory, &config))
	root.child("todo").AddCommand(projectionListCommand("List todos", "query Todos($filter: FilterInput) { todos(filter: $filter) { id name description } }", factory, &config))
	root.child("goal").AddCommand(
		projectionListCommand("List goals", "query Goals { repo { goals { id title status dueDate description } } }", factory, &config),
		&Command{Use: "tree", Short: "Show the goal and ticket tree", Args: NoArgs, RunE: func(cmd *Command, args []string) error {
			return runGraphQL(cmd, factory, &config, "query GoalTree { repo { goals { id title status dueDate description } tickets { id slug title status goal parent year month day } } }", map[string]interface{}{})
		}},
	)
	root.child("contributor").AddCommand(projectionListCommand("List contributors", "query Contributors { repo { contributors { id emails name } } }", factory, &config))
	root.child("folder").AddCommand(projectionListCommand("List folders", "query Folders { repo { folders { id path name kind } } }", factory, &config))
	root.child("file").AddCommand(projectionListCommand("List files", "query Files { repo { files { id path name kind extension } } }", factory, &config))
	root.child("section").AddCommand(projectionSectionListCommand(factory, &config))
	return root, &config
}

// ❌️Execute MUST delegate to the root command and propagate errors.
// ⌨️Execute runs the root command and returns any error.
func Execute(factory EngineFactory) error {
	return NewRoot(factory).Execute()
}

// 🔶️defaultEngineFactory holds the data fields for a defaultEngineFactory record.
func defaultEngineFactory(config Config) (*Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = workspace.FindRepoRoot(cwd)
	}
	workspace.SetRootDir(repoRoot)
	exec, err := graphqlpkg.NewExecutorWithContext(repoRoot, graphqlpkg.NewRepoContext(repoRoot))
	if err != nil {
		return nil, err
	}
	return NewEngine(exec), nil
}

// 🔊️RunCLI runs the repo CLI entry point.
func RunCLI() error {
	if err := Execute(defaultEngineFactory); err != nil {
		var exitErr workspace.ExitError
		if errors.As(err, &exitErr) {
			return exitErr
		}
		return err
	}
	return nil
}

// 🦀️RunMCP runs the repo MCP entry point (generic kind).
func RunMCP() error {
	return RunMcpServerFor(providers.McpClientGeneric, DefaultCommandTimeout)
}

// #endregion 🌧️Cli Adapter

// #region 🎵️Auth Command

// 🆕️authCommand creates the auth command with whoami subcommand.
func authCommand(config *Config) *Command {
	auth := &Command{
		Use:   "auth",
		Short: "Server authentication",
	}
	auth.AddCommand(&Command{
		Use:   "whoami",
		Short: "Show current authenticated developer",
		RunE: func(cmd *Command, args []string) error {
			info, err := eventspkg.ServerWhoami()
			if err != nil {
				return fmt.Errorf("not authenticated: %w", err)
			}
			if config.IsJSON() {
				jsonBytes, _ := json.MarshalIndent(info, "", "  ")
				fmt.Println(string(jsonBytes))
			} else {
				fmt.Printf("Email: %s\n", info["email"])
				fmt.Printf("Name: %s\n", info["display_name"])
				fmt.Printf("Role: %s\n", info["role"])
			}
			return nil
		},
	})
	auth.AddCommand(&Command{
		Use:   "status",
		Short: "Show server connection status",
		RunE: func(cmd *Command, args []string) error {
			addr := eventspkg.GetServerAddr()
			if addr == "" {
				fmt.Println("Server: not configured (set COMPOSE_SERVER_ADDR)")
				return nil
			}
			fmt.Printf("Server: %s\n", addr)
			token := eventspkg.GetServerToken()
			if token == "" {
				fmt.Println("Token: not set (set COMPOSE_SERVER_TOKEN)")
				return nil
			}
			fmt.Println("Token: configured")
			info, err := eventspkg.ServerWhoami()
			if err != nil {
				fmt.Printf("Auth: failed (%v)\n", err)
				return nil
			}
			fmt.Printf("Auth: %s (%s)\n", info["email"], info["role"])
			return nil
		},
	})
	return auth
}

// #endregion 🎵️Auth Command

// #region 🌧️Cli Adapter

// 🔹️syncCommand holds the data fields for a syncCommand record.
func syncCommand(factory EngineFactory, config *Config) *Command {
	sync := &Command{
		Use:   "sync",
		Short: "Synchronize monorepo artifacts",
		RunE: func(cmd *Command, args []string) error {
			if len(args) > 0 {
				return fmt.Errorf("unknown sync target %q", args[0])
			}
			return cmd.Help()
		},
	}
	sync.AddCommand(syncGitHubCommand(factory, config))
	sync.AddCommand(syncManagementCommand(factory, config))
	return sync
}

// 🐙️syncGitHubCommand wires the explicit GitHub synchronization CLI command.
func syncGitHubCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "github",
		Short: "Synchronize local state with GitHub",
		RunE: func(cmd *Command, args []string) error {
			return runSyncManagementMutation(cmd, factory, config)
		},
	}
}

// 🔸️syncManagementCommand holds the data fields for a syncManagementCommand record.
func syncManagementCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "management",
		Short: "Synchronize local state with management provider",
		RunE: func(cmd *Command, args []string) error {
			return runSyncManagementMutation(cmd, factory, config)
		},
	}
}

// 🔁️runSyncManagementMutation executes the shared management synchronization mutation.
func runSyncManagementMutation(cmd *Command, factory EngineFactory, config *Config) error {
	query := `mutation SyncManagement { syncManagement }`
	return runGraphQL(cmd, factory, config, query, nil)
}

// 🔺️mcpCommand holds the data fields for a mcpCommand record.
func mcpCommand(factory EngineFactory, config *Config) *Command {
	var dryRun bool
	cmd := &Command{
		Use:   "mcp [kind]",
		Short: "Run MCP server (optional kind: client, cursor, copilot, claude, codex, kiro)",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			if dryRun {
				return nil
			}
			kind := providers.McpClientGeneric
			if len(args) > 0 {
				parsed, err := providers.ParseMcpClientKind(args[0])
				if err != nil {
					return err
				}
				kind = parsed
			}
			return serveMcp(cmd.Context(), kind, config.Timeout)
		},
	}
	cmd.Flags().BoolVar(&dryRun, "dry-run", false, "Initialize and exit without starting server")
	return cmd
}

func serveMcp(ctx context.Context, kind providers.McpClientKind, toolTimeout time.Duration) error {
	errCh := make(chan error, 1)
	go func() {
		errCh <- RunMcpServerFor(kind, toolTimeout)
	}()
	select {
	case err := <-errCh:
		return err
	case <-ctx.Done():
		return ctx.Err()
	}
}

// 🕸️graphqlCommand holds the data fields for a graphqlCommand record.
func graphqlCommand(factory EngineFactory, config *Config) *Command {
	var query string
	var variablesJSON string
	cmd := &Command{
		Use:   "graphql [query]",
		Short: "Execute a GraphQL query",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			resolvedQuery := query
			if resolvedQuery == "" && len(args) > 0 {
				resolvedQuery = args[0]
			}
			if resolvedQuery == "" {
				return fmt.Errorf("missing query")
			}
			var variables map[string]interface{}
			if variablesJSON != "" {
				if err := json.Unmarshal([]byte(variablesJSON), &variables); err != nil {
					return fmt.Errorf("invalid variables JSON: %w", err)
				}
			}
			var payload struct {
				Query     string                 `json:"query"`
				Variables map[string]interface{} `json:"variables"`
			}
			if err := json.Unmarshal([]byte(resolvedQuery), &payload); err == nil && payload.Query != "" {
				resolvedQuery = payload.Query
				if variables == nil {
					variables = map[string]interface{}{}
				}
				for key, value := range payload.Variables {
					if _, exists := variables[key]; !exists {
						variables[key] = value
					}
				}
			}
			return runGraphQL(cmd, factory, config, resolvedQuery, variables)
		},
	}
	cmd.Flags().StringVar(&query, "query", "", "GraphQL query")
	cmd.Flags().StringVarP(&variablesJSON, "vars", "v", "", "GraphQL variables JSON")
	return cmd
}

// #endregion 🌧️Cli Adapter

// #region 🖥️Entity Emojis Command

func entityEmojisCommand(config *Config) *Command {
	cmd := &Command{
		Use:   "entity-emojis",
		Short: "List all entity-identifying emojis",
		RunE: func(cmd *Command, args []string) error {
			emojis := model.AllEntityEmojis()
			switch {
			case config.IsJSON():
				encoded, err := json.Marshal(emojis)
				if err != nil {
					return err
				}
				fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
			default:
				for _, e := range emojis {
					fmt.Fprintln(cmd.OutOrStdout(), e)
				}
			}
			return nil
		},
	}
	return cmd
}

// #endregion 🖥️Entity Emojis Command

// #region 🌧️Cli Adapter

// #endregion 🖥️Entity Emojis Command
// ⌨️searchCommand holds the data fields for a searchCommand record.
func searchCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "search [query]",
		Short: "Search monorepo tree",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			ctx := cmd.Context()

			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}

			buildOpts := treepkg.TreeBuildOptions{
				IncludeSections: filter.OnlyKinds[treepkg.TreeNodeSection] ||
					filter.OnlyKinds[treepkg.TreeNodeDefinition],
			}
			tree := treepkg.BuildMonorepoTreeCached(ctx, buildOpts)
			tree, err := treepkg.SearchMonorepoTreeWithCache(ctx, tree, filter.Query)
			if err != nil {
				return err
			}
			tree = treepkg.FilterMonorepoTree(tree, &filter)

			var output string
			switch {
			case config.IsJSON():
				encoded, err := json.Marshal(tree)
				if err != nil {
					return err
				}
				output = string(encoded) + "\n"
			case config.IsMarkdown():
				output = treepkg.RenderMonorepoTreeMarkdown(tree, treepkg.DefaultEntityRenderer{})
			default:
				output = treepkg.RenderMonorepoTree(tree, treepkg.DefaultEntityRenderer{})
			}
			fmt.Fprint(cmd.OutOrStdout(), output)
			return nil
		},
	}
	bindTreeFlags(cmd)
	return cmd
}

// 🌳️flattenTreeNodes holds the data fields for a flattenTreeNodes record.
func flattenTreeNodes(node *treepkg.TreeNode, out *[]*treepkg.TreeNode) {
	if node.Kind != treepkg.TreeNodeCategory {
		*out = append(*out, node)
	}
	for _, c := range node.Children {
		flattenTreeNodes(c, out)
	}
}

// ⬛️listCommand holds the data fields for a listCommand record.
func listCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "list [query]",
		Short: "Stream a flat list of monorepo items",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			ctx := cmd.Context()
			sorted, _ := cmd.Flags().GetBool("sorted")
			limit, _ := cmd.Flags().GetInt("limit")
			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}
			buildOpts := treepkg.TreeBuildOptions{
				IncludeSections: filter.OnlyKinds[treepkg.TreeNodeSection] ||
					filter.OnlyKinds[treepkg.TreeNodeDefinition],
			}
			tree := treepkg.BuildMonorepoTreeCached(ctx, buildOpts)
			tree, err := treepkg.SearchMonorepoTreeWithCache(ctx, tree, filter.Query)
			if err != nil {
				return err
			}
			tree = treepkg.FilterMonorepoTree(tree, &filter)
			var nodes []*treepkg.TreeNode
			flattenTreeNodes(tree, &nodes)
			if sorted {
				sort.Slice(nodes, func(i, j int) bool { return nodes[i].ID < nodes[j].ID })
			}
			if limit > 0 && len(nodes) > limit {
				nodes = nodes[:limit]
			}
			switch {
			case config.IsJSON():
				for _, n := range nodes {
					entityKind := treepkg.TreeNodeKindToEntityKind(n.Kind)
					if entityKind == "" {
						entityKind = string(n.Kind)
					}
					if n.Data != nil {
						wrapped := map[string]interface{}{entityKind: n.Data}
						encoded, err := json.Marshal(wrapped)
						if err != nil {
							continue
						}
						fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
					} else {
						encoded, err := json.Marshal(n)
						if err != nil {
							continue
						}
						fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
					}
				}
			case config.IsMarkdown():
				for _, n := range nodes {
					entityKind := treepkg.TreeNodeKindToEntityKind(n.Kind)
					if entityKind == "" {
						entityKind = string(n.Kind)
					}
					if n.Data != nil {
						fmt.Fprintln(cmd.OutOrStdout(), model.RenderEntityMarkdown(entityKind, n.Data))
					} else {
						label := n.Label
						if n.URI != "" {
							label = "[" + n.Label + "](" + n.URI + ")"
						}
						fmt.Fprintln(cmd.OutOrStdout(), "- "+label)
					}
				}
			default:
				for _, n := range nodes {
					entityKind := treepkg.TreeNodeKindToEntityKind(n.Kind)
					if entityKind != "" && n.Data != nil {
						fmt.Fprintln(cmd.OutOrStdout(), model.RenderEntityHuman(entityKind, n.Data, false))
					} else {
						label := n.Label
						if n.URI != "" {
							label = "[" + n.Label + "](" + n.URI + ")"
						}
						fmt.Fprintln(cmd.OutOrStdout(), label)
					}
				}
			}
			return nil
		},
	}
	cmd.Flags().Int("limit", 0, "Maximum number of items to output (0 = unlimited)")
	bindTreeFlags(cmd)
	cmd.Flags().Bool("sorted", false, "Collect all results and sort by ID before output")
	return cmd
}

type boolFlagSpec struct {
	Name  string
	Usage string
}

type boolFlagPairSpec struct {
	OnlyName  string
	NoName    string
	OnlyUsage string
	NoUsage   string
}

// 🔘️bindBoolFlags MUST register all boolean flags from the input specs.
func bindBoolFlags(cmd *Command, specs []boolFlagSpec) {
	for _, spec := range specs {
		cmd.Flags().Bool(spec.Name, false, spec.Usage)
	}
}

// 🚩️bindOnlyNoFlagPairs MUST register only/no boolean flag pairs from the input specs.
func bindOnlyNoFlagPairs(cmd *Command, specs []boolFlagPairSpec) {
	for _, spec := range specs {
		cmd.Flags().Bool(spec.OnlyName, false, spec.OnlyUsage)
		cmd.Flags().Bool(spec.NoName, false, spec.NoUsage)
	}
}

func bindTreeFlags(cmd *Command) {
	bindOnlyNoFlagPairs(cmd, []boolFlagPairSpec{
		{OnlyName: "only-technology", NoName: "no-technology", OnlyUsage: "Only show technologies", NoUsage: "Exclude technologies"},
		{OnlyName: "only-bundle", NoName: "no-bundle", OnlyUsage: "Only show bundles", NoUsage: "Exclude bundles"},
		{OnlyName: "only-folder", NoName: "no-folder", OnlyUsage: "Only show folders", NoUsage: "Exclude folders"},
		{OnlyName: "only-file", NoName: "no-file", OnlyUsage: "Only show files", NoUsage: "Exclude files"},
		{OnlyName: "only-section", NoName: "no-section", OnlyUsage: "Only show sections", NoUsage: "Exclude sections"},
		{OnlyName: "only-definition", NoName: "no-definition", OnlyUsage: "Only show definitions", NoUsage: "Exclude definitions"},
		{OnlyName: "only-goal", NoName: "no-goal", OnlyUsage: "Only show goals", NoUsage: "Exclude goals"},
		{OnlyName: "only-ticket", NoName: "no-ticket", OnlyUsage: "Only show tickets", NoUsage: "Exclude tickets"},
		{OnlyName: "only-draft", NoName: "no-draft", OnlyUsage: "Only show drafts", NoUsage: "Exclude drafts"},
		{OnlyName: "only-policy", NoName: "no-policy", OnlyUsage: "Only show policies", NoUsage: "Exclude policies"},
		{OnlyName: "only-contributor", NoName: "no-contributor", OnlyUsage: "Only show contributors", NoUsage: "Exclude contributors"},
		{OnlyName: "only-checkpoint", NoName: "no-checkpoint", OnlyUsage: "Only show checkpoints", NoUsage: "Exclude checkpoints"},
		{OnlyName: "only-statute", NoName: "no-statute", OnlyUsage: "Only show statutes", NoUsage: "Exclude statutes"},
		{OnlyName: "only-todo", NoName: "no-todo", OnlyUsage: "Only show todos", NoUsage: "Exclude todos"},
		{OnlyName: "only-breach", NoName: "no-breach", OnlyUsage: "Only show breaches", NoUsage: "Exclude breaches"},
		{OnlyName: "only-library", NoName: "no-library", OnlyUsage: "Only show library bundles", NoUsage: "Exclude library bundles"},
		{OnlyName: "only-schema", NoName: "no-schema", OnlyUsage: "Only show schema bundles", NoUsage: "Exclude schema bundles"},
		{OnlyName: "only-binary", NoName: "no-binary", OnlyUsage: "Only show binary bundles", NoUsage: "Exclude binary bundles"},
		{OnlyName: "only-client", NoName: "no-client", OnlyUsage: "Only show client bundles", NoUsage: "Exclude client bundles"},
		{OnlyName: "only-site", NoName: "no-site", OnlyUsage: "Only show site bundles", NoUsage: "Exclude site bundles"},
		{OnlyName: "only-assets", NoName: "no-assets", OnlyUsage: "Only show asset bundles", NoUsage: "Exclude asset bundles"},
		{OnlyName: "only-organization", NoName: "no-organization", OnlyUsage: "Only show organization folders", NoUsage: "Exclude organization folders"},
		{OnlyName: "only-required", NoName: "no-required", OnlyUsage: "Only show required folders", NoUsage: "Exclude required folders"},
		{OnlyName: "only-code", NoName: "no-code", OnlyUsage: "Only show code files", NoUsage: "Exclude code files"},
		{OnlyName: "only-script", NoName: "no-script", OnlyUsage: "Only show script files", NoUsage: "Exclude script files"},
		{OnlyName: "only-config", NoName: "no-config", OnlyUsage: "Only show config files", NoUsage: "Exclude config files"},
		{OnlyName: "only-lab", NoName: "no-lab", OnlyUsage: "Only show lab files", NoUsage: "Exclude lab files"},
		{OnlyName: "only-docs", NoName: "no-docs", OnlyUsage: "Only show docs files", NoUsage: "Exclude docs files"},
		{OnlyName: "only-resource", NoName: "no-resource", OnlyUsage: "Only show resource files", NoUsage: "Exclude resource files"},
		{OnlyName: "only-template", NoName: "no-template", OnlyUsage: "Only show template files", NoUsage: "Exclude template files"},
		{OnlyName: "only-license", NoName: "no-license", OnlyUsage: "Only show license files", NoUsage: "Exclude license files"},
		{OnlyName: "only-implementation", NoName: "no-implementation", OnlyUsage: "Only show implementation definitions", NoUsage: "Exclude implementation definitions"},
		{OnlyName: "only-interface", NoName: "no-interface", OnlyUsage: "Only show interface definitions", NoUsage: "Exclude interface definitions"},
		{OnlyName: "only-constant", NoName: "no-constant", OnlyUsage: "Only show constant definitions", NoUsage: "Exclude constant definitions"},
	})

	bindBoolFlags(cmd, []boolFlagSpec{
		{Name: "only-open", Usage: "Only show open items"},
		{Name: "only-closed", Usage: "Only show closed items"},
		{Name: "open", Usage: "Only show open items"},
		{Name: "closed", Usage: "Only show closed items"},
	})

	cmd.Flags().IntSlice("only-year", nil, "Only show years")
	cmd.Flags().IntSlice("no-year", nil, "Exclude years")
	cmd.Flags().IntSlice("only-month", nil, "Only show months")
	cmd.Flags().IntSlice("no-month", nil, "Exclude months")
	cmd.Flags().IntSlice("only-day", nil, "Only show days")
	cmd.Flags().IntSlice("no-day", nil, "Exclude days")

	cmd.Flags().StringSlice("only-contributor-name", nil, "Only show specific contributors")
	cmd.Flags().StringSlice("no-contributor-name", nil, "Exclude specific contributors")
	cmd.Flags().StringSlice("only-policy-name", nil, "Only show specific policies")
	cmd.Flags().StringSlice("no-policy-name", nil, "Exclude specific policies")
	cmd.Flags().String("query", "", "Full-text search query")
}

// 🔍️queryCommand holds the data fields for a queryCommand record.
func queryCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "query [keywords]",
		Short: "Keyword search across monorepo resources",
		Args:  ArbitraryArgs,
		RunE: func(cmd *Command, args []string) error {
			ctx := cmd.Context()
			query := strings.Join(args, " ")
			tree := treepkg.BuildMonorepoTreeCached(ctx, treepkg.TreeBuildOptions{IncludeSections: true})
			matched := treepkg.SearchTreeInMemory(tree, query)
			var nodes []*treepkg.TreeNode
			flattenTreeNodes(matched, &nodes)
			for _, n := range nodes {
				id := n.ID
				if id == "" {
					id = n.Label
				}
				fmt.Fprintln(cmd.OutOrStdout(), id)
			}
			return nil
		},
	}
	return cmd
}

// 🧹️buildTreeFilterFromFlags holds the data fields for a buildTreeFilterFromFlags record.
func buildTreeFilterFromFlags(cmd *Command) treepkg.TreeFilter {
	filter := treepkg.TreeFilter{
		OnlyKinds:       make(map[treepkg.TreeNodeKind]bool),
		ExcludeKinds:    make(map[treepkg.TreeNodeKind]bool),
		OnlySubKinds:    make(map[treepkg.TreeNodeKind][]string),
		ExcludeSubKinds: make(map[treepkg.TreeNodeKind][]string),
	}

	kindFlags := []struct {
		onlyFlag string
		noFlag   string
		kind     treepkg.TreeNodeKind
	}{
		{"only-technology", "no-technology", treepkg.TreeNodeTechnology},
		{"only-bundle", "no-bundle", treepkg.TreeNodeBundle},
		{"only-folder", "no-folder", treepkg.TreeNodeFolder},
		{"only-file", "no-file", treepkg.TreeNodeFile},
		{"only-section", "no-section", treepkg.TreeNodeSection},
		{"only-definition", "no-definition", treepkg.TreeNodeDefinition},
		{"only-goal", "no-goal", treepkg.TreeNodeGoal},
		{"only-ticket", "no-ticket", treepkg.TreeNodeTicket},
		{"only-draft", "no-draft", treepkg.TreeNodeDraft},
		{"only-policy", "no-policy", treepkg.TreeNodePolicy},
		{"only-contributor", "no-contributor", treepkg.TreeNodeContributor},
		{"only-checkpoint", "no-checkpoint", treepkg.TreeNodeCheckpoint},
		{"only-statute", "no-statute", treepkg.TreeNodeStatute},
		{"only-todo", "no-todo", treepkg.TreeNodeTodo},
		{"only-breach", "no-breach", treepkg.TreeNodeBreach},
	}

	for _, kf := range kindFlags {
		if v, _ := cmd.Flags().GetBool(kf.onlyFlag); v {
			filter.OnlyKinds[kf.kind] = true
		}
		if v, _ := cmd.Flags().GetBool(kf.noFlag); v {
			filter.ExcludeKinds[kf.kind] = true
		}
	}

	type subKindFlag struct {
		onlyFlag string
		noFlag   string
		kind     treepkg.TreeNodeKind
		value    string
	}
	subKindFlags := []subKindFlag{
		{"only-library", "no-library", treepkg.TreeNodeBundle, string(model.BundleKindLibrary)},
		{"only-schema", "no-schema", treepkg.TreeNodeBundle, string(model.BundleKindSchema)},
		{"only-binary", "no-binary", treepkg.TreeNodeBundle, string(model.BundleKindBinary)},
		{"only-client", "no-client", treepkg.TreeNodeBundle, string(model.BundleKindUI)},
		{"only-site", "no-site", treepkg.TreeNodeBundle, string(model.BundleKindSite)},
		{"only-assets", "no-assets", treepkg.TreeNodeBundle, string(model.BundleKindAssets)},
		{"only-organization", "no-organization", treepkg.TreeNodeFolder, string(model.FolderKindOrganization)},
		{"only-code", "no-code", treepkg.TreeNodeFile, model.FileKindCode},
		{"only-script", "no-script", treepkg.TreeNodeFile, model.FileKindScript},
		{"only-config", "no-config", treepkg.TreeNodeFile, model.FileKindConfig},
		{"only-lab", "no-lab", treepkg.TreeNodeFile, model.FileKindLab},
		{"only-docs", "no-docs", treepkg.TreeNodeFile, model.FileKindDocs},
		{"only-resource", "no-resource", treepkg.TreeNodeFile, model.FileKindResource},
		{"only-template", "no-template", treepkg.TreeNodeFile, model.FileKindTemplate},
		{"only-license", "no-license", treepkg.TreeNodeFile, model.FileKindLicense},
		{"only-implementation", "no-implementation", treepkg.TreeNodeDefinition, string(model.DefinitionKindImplementation)},
		{"only-interface", "no-interface", treepkg.TreeNodeDefinition, string(model.DefinitionKindInterface)},
		{"only-constant", "no-constant", treepkg.TreeNodeDefinition, string(model.DefinitionKindConstant)},
	}

	for _, sf := range subKindFlags {
		if v, _ := cmd.Flags().GetBool(sf.onlyFlag); v {
			filter.OnlySubKinds[sf.kind] = append(filter.OnlySubKinds[sf.kind], sf.value)
		}
		if v, _ := cmd.Flags().GetBool(sf.noFlag); v {
			filter.ExcludeSubKinds[sf.kind] = append(filter.ExcludeSubKinds[sf.kind], sf.value)
		}
	}

	if v, _ := cmd.Flags().GetBool("only-open"); v {
		filter.OnlyStatus = "open"
	} else if v, _ := cmd.Flags().GetBool("open"); v {
		filter.OnlyStatus = "open"
	}
	if v, _ := cmd.Flags().GetBool("only-closed"); v {
		filter.OnlyStatus = "closed"
	} else if v, _ := cmd.Flags().GetBool("closed"); v {
		filter.OnlyStatus = "closed"
	}

	filter.OnlyYears, _ = cmd.Flags().GetIntSlice("only-year")
	filter.ExcludeYears, _ = cmd.Flags().GetIntSlice("no-year")
	filter.OnlyMonths, _ = cmd.Flags().GetIntSlice("only-month")
	filter.ExcludeMonths, _ = cmd.Flags().GetIntSlice("no-month")
	filter.OnlyDays, _ = cmd.Flags().GetIntSlice("only-day")
	filter.ExcludeDays, _ = cmd.Flags().GetIntSlice("no-day")

	filter.OnlyContributors, _ = cmd.Flags().GetStringSlice("only-contributor-name")
	filter.ExcludeContributors, _ = cmd.Flags().GetStringSlice("no-contributor-name")
	filter.OnlyPolicies, _ = cmd.Flags().GetStringSlice("only-policy-name")
	filter.ExcludePolicies, _ = cmd.Flags().GetStringSlice("no-policy-name")
	filter.Query, _ = cmd.Flags().GetString("query")

	return filter
}

// 📤️exportCommand holds the data fields for a exportCommand record.
func exportCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "export [output]",
		Short: "Export repo data to an event log",
		Long:  `Export repo data as deterministic append-only events for replay.`,
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			outputPath := ""
			if len(args) > 0 {
				outputPath = args[0]
			}
			repoRoot := config.Repo
			if repoRoot == "" {
				repoRoot = workspace.FindRepoRoot(".")
			}
			ctx := graphqlpkg.NewRepoContext(repoRoot)
			result, err := ExportToEventLogContext(cmd.Context(), outputPath, ctx, nil)
			if err != nil {
				return err
			}
			jsonBytes, err := json.MarshalIndent(result, "", "  ")
			if err != nil {
				return err
			}
			cmd.Println(string(jsonBytes))
			return nil
		},
	}
}

// #endregion 🌧️Cli Adapter

// #region 🕸️Test Command

// ⬛️testCommand holds the data fields for a testCommand record.
func testCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "test [testable-id-or-uri]...",
		Short: "Run tests for given entities",
		Long: `Run tests for one or more testable entities identified by their entity IDs or URIs.

With no arguments, runs all tests in the repository.

Supported entity scopes (narrowing order):
  🧰️technology                       - all tests in the technology
  🧰️technology⌨️bundle               - all tests in the bundle
  🧰️technology⌨️bundle🥼️file         - all tests in the test file
  🧰️technology⌨️bundle🥼️file🔖️sec    - all tests in the section
  🧰️technology⌨️bundle🥼️file🔖️sec🧪️fn - a single test function`,
		Args: ArbitraryArgs,
		RunE: func(cmd *Command, args []string) error {
			scopes := testrunner.ResolveTestScopes(args)
			var firstErr error
			for _, scope := range scopes {
				if err := testrunner.RunTestScope(scope, cmd); err != nil && firstErr == nil {
					firstErr = err
				}
			}
			return firstErr
		},
	}
}

// 🧠️extractLLMFromArgs extracts LLM from command flags and positional arguments.
func extractLLMFromArgs(cmd *Command, args []string) (string, []string) {
	if llm, _ := cmd.Flags().GetString("llm"); llm != "" {
		return llm, args
	}

	for _, allowed := range model.AllowedLLMList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundLLM := ""
	for _, arg := range args {
		if foundLLM != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeLLMSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedLLMList() {
			if strings.Contains(normalized, model.NormalizeLLMSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundLLM = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundLLM, remaining
}

// 🏋️extractEffortFromArgs extracts reasoning effort from command flags and positional arguments.
func extractEffortFromArgs(cmd *Command, args []string) (string, []string) {
	if effort, _ := cmd.Flags().GetString("effort"); effort != "" {
		return effort, args
	}

	for _, allowed := range model.AllowedEffortList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundEffort := ""
	for _, arg := range args {
		if foundEffort != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeEffortSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedEffortList() {
			if normalized == model.NormalizeEffortSlug(allowed) || strings.Contains(normalized, model.NormalizeEffortSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundEffort = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundEffort, remaining
}

// 🧲️extractClientFromArgs holds the data fields for a extractClientFromArgs record.
func extractClientFromArgs(cmd *Command, args []string) (string, []string) {

	if client, _ := cmd.Flags().GetString("client"); client != "" {
		return client, args
	}

	for _, allowed := range model.AllowedClientList() {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundClient := ""
	for _, arg := range args {
		if foundClient != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := model.NormalizeClientSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range model.AllowedClientList() {
			if strings.Contains(normalized, model.NormalizeClientSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundClient = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundClient, remaining
}

// ➕️addLLMFlags holds the data fields for a addLLMFlags record.
func addLLMFlags(cmd *Command) {
	for _, llm := range model.AllowedLLMList() {
		if cmd.Flags().Lookup(llm) == nil {
			cmd.Flags().Bool(llm, false, fmt.Sprintf("Use %s as LLM", llm))
		}
	}
}

// 🏋️addEffortFlags adds boolean flags for all allowed effort values.
func addEffortFlags(cmd *Command) {
	for _, effort := range model.AllowedEffortList() {
		if cmd.Flags().Lookup(effort) == nil {
			cmd.Flags().Bool(effort, false, fmt.Sprintf("Use %s as LLM reasoning effort", effort))
		}
	}
}

// 💻️addClientFlags holds the data fields for a addClientFlags record.
func addClientFlags(cmd *Command) {
	for _, client := range model.AllowedClientList() {
		if cmd.Flags().Lookup(client) == nil {
			cmd.Flags().Bool(client, false, fmt.Sprintf("Use %s as Client", client))
		}
	}
}

// 🎨️draftStore returns the store the notes directory of the repository keeps drafts in.
func draftStore() todos.DraftStore { return todos.NewFsDraftStore(workspace.GetDraftsPath()) }

// ⬜️draftCommand holds the data fields for a draftCommand record.
func draftCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "draft", Short: "Draft management commands"}
	createCmd := &Command{
		Use:   "create [title]",
		Short: "Create a new draft",
		RunE: func(cmd *Command, args []string) error {
			if len(args) < 1 {
				return fmt.Errorf("missing title")
			}
			title := args[0]
			paths, _ := cmd.Flags().GetStringSlice("files")

			files, err := todos.LoadTreeFiles(paths)
			if err != nil {
				return err
			}
			draft, err := todos.CreateDraft(draftStore(), title, files)
			if err != nil {
				return err
			}
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "draft create"}
				data, _ := json.Marshal(map[string]interface{}{"draft": draft})
				stream <- Event{Kind: KindResult, Command: "draft create", Data: data}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	createCmd.Flags().StringSlice("files", nil, "Files to include in the draft")

	deleteCmd := &Command{
		Use:   "delete [slug]",
		Short: "Delete a draft",
		RunE: func(cmd *Command, args []string) error {
			if len(args) < 1 {
				return fmt.Errorf("missing slug")
			}
			slug := args[0]
			if err := todos.DeleteDraft(draftStore(), slug); err != nil {
				return err
			}
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "draft delete"}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}

	root.AddCommand(createCmd)
	root.AddCommand(deleteCmd)
	return root
}

// ✅️todoCommand holds the data fields for a todoCommand record.
func todoCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "todo", Short: "Todo management commands"}
	createCmd := &Command{
		Use:   "create [parent-id] [name] [description]",
		Short: "Create a todo",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			parentID, _ := cmd.Flags().GetString("parent")
			name, _ := cmd.Flags().GetString("name")
			description, _ := cmd.Flags().GetString("description")
			if len(args) > 0 {
				parentID = args[0]
			}
			if len(args) > 1 {
				name = args[1]
			}
			if len(args) > 2 {
				description = args[2]
			}
			if parentID == "" || name == "" {
				return fmt.Errorf("missing parent-id or name")
			}
			variables := map[string]interface{}{
				"input": map[string]interface{}{
					"parentId":    parentID,
					"name":        name,
					"description": description,
				},
			}
			query := `mutation TodoCreate($input: TodoCreateInput!) { todoCreate(input: $input) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("parent", "", "Parent ID")
	createCmd.Flags().String("name", "", "Todo name")
	createCmd.Flags().String("description", "", "Todo description")

	changeCmd := &Command{
		Use:   "change [id] --name <new-name> --description <new-description>",
		Short: "Change a todo",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			id, _ := cmd.Flags().GetString("id")
			if id == "" && len(args) > 0 {
				id = args[0]
			}
			name, _ := cmd.Flags().GetString("name")
			description, _ := cmd.Flags().GetString("description")
			if id == "" {
				return fmt.Errorf("missing id")
			}
			input := map[string]interface{}{
				"id": id,
			}
			if cmd.Flags().Changed("name") {
				input["name"] = name
			}
			if cmd.Flags().Changed("description") {
				input["description"] = description
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TodoChange($input: TodoUpdateInput!) { todoChange(input: $input) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("id", "", "Todo ID")
	changeCmd.Flags().String("name", "", "New name")
	changeCmd.Flags().String("description", "", "New description")

	deleteCmd := &Command{
		Use:   "delete [id]",
		Short: "Delete a todo",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			id, _ := cmd.Flags().GetString("id")
			if id == "" && len(args) > 0 {
				id = args[0]
			}
			if id == "" {
				return fmt.Errorf("missing id")
			}
			variables := map[string]interface{}{"id": id}
			query := `mutation TodoDelete($id: ID!) { todoDelete(id: $id) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("id", "", "Todo ID")

	searchCmd := &Command{
		Use:   "search [search-string]",
		Short: "Search todos",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			search := ""
			if len(args) > 0 {
				search = args[0]
			}
			variables := map[string]interface{}{"filter": map[string]interface{}{"filter": search}}
			query := `query Todos($filter: FilterInput) { todos(filter: $filter) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}

	root.AddCommand(createCmd)
	root.AddCommand(changeCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(searchCmd)
	return root
}

// 🎫️ticketCommand holds the data fields for a ticketCommand record.
func ticketCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "ticket", Short: "Ticket management commands"}
	openCmd := &Command{
		Use:   "open [emoji] [goal] [title] [prompt] [client] [llm]",
		Short: "Open a new ticket",
		RunE: func(cmd *Command, args []string) error {
			emoji, _ := cmd.Flags().GetString("emoji")
			title, _ := cmd.Flags().GetString("title")
			prompt, _ := cmd.Flags().GetString("prompt")
			noIssue, _ := cmd.Flags().GetBool("no-issue")
			draft, _ := cmd.Flags().GetString("draft")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")
			noManagement, _ := cmd.Flags().GetBool("no-management")
			issue, _ := cmd.Flags().GetString("issue")

			remainingArgs := args
			if emoji == "" && len(remainingArgs) > 0 {
				emoji = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if goal == "" && len(remainingArgs) > 0 {
				goal = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if title == "" && len(remainingArgs) > 0 {
				title = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			client, remainingArgs := extractClientFromArgs(cmd, remainingArgs)
			llm, remainingArgs := extractLLMFromArgs(cmd, remainingArgs)
			effort, _ := extractEffortFromArgs(cmd, remainingArgs)

			if emoji == "" {
				return fmt.Errorf("missing emoji")
			}
			if title == "" {
				return fmt.Errorf("missing title")
			}
			if prompt == "" {
				prompt = title
			}
			if client == "" {
				return fmt.Errorf("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedClientList(), ", "))
			}
			if goal == "" {
				return fmt.Errorf("missing goal. Use --goal <goal-id>")
			}
			input := map[string]interface{}{
				"emoji":        emoji,
				"title":        title,
				"prompt":       prompt,
				"client":       strings.ToUpper(strings.ReplaceAll(client, "-", "_")),
				"noIssue":      noIssue,
				"noManagement": noManagement,
			}
			if llm != "" {
				input["llm"] = llm
			}
			if effort != "" {
				input["effort"] = effort
			}
			if draft != "" {
				input["draft"] = draft
			}
			if goal != "" {
				input["goal"] = goal
			}
			if parent != "" {
				input["parent"] = parent
			}
			if issue != "" {
				input["issue"] = issue
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketOpen($input: TicketOpenInput!) {
				ticketOpen(input: $input) {
					id
					slug
					year
					month
					day
					status
					path
					uri
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	openCmd.Flags().String("emoji", "", "Ticket emoji")
	openCmd.Flags().String("title", "", "Ticket title")
	openCmd.Flags().String("prompt", "", "Ticket prompt")
	openCmd.Flags().String("llm", "", "LLM")
	openCmd.Flags().String("effort", "", "LLM reasoning effort (low, medium, high, max)")
	openCmd.Flags().String("client", "", "Client")
	openCmd.Flags().Bool("no-issue", false, "Skip management provider issue")
	openCmd.Flags().String("draft", "", "Draft ID")
	openCmd.Flags().String("goal", "", "Goal ID")
	openCmd.Flags().Bool("no-management", false, "Skip management provider operations")
	openCmd.Flags().MarkHidden("no-management")
	openCmd.Flags().String("parent", "", "Parent ticket slug")
	openCmd.Flags().String("issue", "", "Link to existing GitHub issue URL instead of creating new one")
	addLLMFlags(openCmd)
	addEffortFlags(openCmd)
	addClientFlags(openCmd)
	closeCmd := &Command{
		Use:   "close [path] [summary] [files...]",
		Short: "Close a ticket",
		RunE: func(cmd *Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			slug, _ := cmd.Flags().GetString("slug")
			summary, _ := cmd.Flags().GetString("summary")
			noManagement, _ := cmd.Flags().GetBool("no-management")
			files, _ := cmd.Flags().GetStringSlice("files")
			title, _ := cmd.Flags().GetString("title")
			closeAll, _ := cmd.Flags().GetBool("all")

			if !closeAll {
				if len(args) > 0 && (year == 0 || month == 0 || day == 0 || slug == "") {
					parts := strings.Split(args[0], "/")
					if len(parts) >= 4 {
						if y, err := strconv.Atoi(parts[0]); err == nil {
							year = y
						}
						if m, err := strconv.Atoi(parts[1]); err == nil {
							month = m
						}
						if d, err := strconv.Atoi(parts[2]); err == nil {
							day = d
						}
						slug = strings.Join(parts[3:], "/")
					}
				}

				if summary == "" && len(args) > 1 {
					summary = args[1]
				}
				if len(files) == 0 && len(args) > 2 {
					files = args[2:]
				}

				if year == 0 || month == 0 || day == 0 || slug == "" {
					return fmt.Errorf("missing ticket path (use YYYY/MM/DD/SLUG or flags)")
				}
				if summary == "" {
					return fmt.Errorf("missing summary")
				}
				if len(files) == 0 {
					return fmt.Errorf("missing files")
				}
			}

			input := map[string]interface{}{
				"noManagement": noManagement,
				"all":          closeAll,
			}
			if !closeAll {
				input["year"] = year
				input["month"] = month
				input["day"] = day
				input["slug"] = slug
				input["summary"] = summary
				input["files"] = files
			}
			if title != "" {
				input["title"] = title
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketClose($input: TicketCloseInput!) {
				ticketClose(input: $input) {
					id
					slug
					status
					dates { started finished }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	closeCmd.Flags().Bool("all", false, "Close all open tickets")
	closeCmd.Flags().Int("year", 0, "Ticket year")
	closeCmd.Flags().Int("month", 0, "Ticket month")
	closeCmd.Flags().Int("day", 0, "Ticket day")
	closeCmd.Flags().String("slug", "", "Ticket slug")
	closeCmd.Flags().Bool("no-management", false, "Skip management provider operations")
	closeCmd.Flags().MarkHidden("no-management")
	closeCmd.Flags().String("summary", "", "Summary")
	closeCmd.Flags().StringSlice("files", nil, "Files")
	closeCmd.Flags().String("title", "", "Title")
	reopenCmd := &Command{
		Use:   "reopen [path] [prompt] [client] [llm]",
		Short: "Reopen a ticket",
		RunE: func(cmd *Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			noManagement, _ := cmd.Flags().GetBool("no-management")
			slug, _ := cmd.Flags().GetString("slug")
			prompt, _ := cmd.Flags().GetString("prompt")
			title, _ := cmd.Flags().GetString("title")
			draft, _ := cmd.Flags().GetString("draft")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")

			remainingArgs := args
			if len(remainingArgs) > 0 && (year == 0 || month == 0 || day == 0 || slug == "") {
				parts := strings.Split(remainingArgs[0], "/")
				if len(parts) >= 4 {
					if y, err := strconv.Atoi(parts[0]); err == nil {
						year = y
					}
					if m, err := strconv.Atoi(parts[1]); err == nil {
						month = m
					}
					if d, err := strconv.Atoi(parts[2]); err == nil {
						day = d
					}
					slug = strings.Join(parts[3:], "/")
					remainingArgs = remainingArgs[1:]
				}
			}

			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			client, remainingArgs := extractClientFromArgs(cmd, remainingArgs)
			llm, remainingArgs := extractLLMFromArgs(cmd, remainingArgs)
			effort, _ := extractEffortFromArgs(cmd, remainingArgs)

			if year == 0 || month == 0 || day == 0 || slug == "" {
				return fmt.Errorf("missing ticket path")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if client == "" {
				return fmt.Errorf("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedClientList(), ", "))
			}
			input := map[string]interface{}{
				"year":         year,
				"month":        month,
				"noManagement": noManagement,
				"day":          day,
				"slug":         slug,
				"prompt":       prompt,
				"client":       strings.ToUpper(strings.ReplaceAll(client, "-", "_")),
			}
			if llm != "" {
				input["llm"] = llm
			}
			if effort != "" {
				input["effort"] = effort
			}
			if title != "" {
				input["title"] = title
			}
			if draft != "" {
				input["draft"] = draft
			}
			if goal != "" {
				input["goal"] = goal
			}
			if parent != "" {
				input["parent"] = parent
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketReopen($input: TicketReopenInput!) {
				ticketReopen(input: $input) {
					id
					slug
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	reopenCmd.Flags().Bool("no-management", false, "Skip management provider operations")
	reopenCmd.Flags().MarkHidden("no-management")
	reopenCmd.Flags().Int("year", 0, "Ticket year")
	reopenCmd.Flags().Int("month", 0, "Ticket month")
	reopenCmd.Flags().Int("day", 0, "Ticket day")
	reopenCmd.Flags().String("slug", "", "Ticket slug")
	reopenCmd.Flags().String("prompt", "", "Prompt")
	reopenCmd.Flags().String("llm", "", "LLM")
	reopenCmd.Flags().String("effort", "", "LLM reasoning effort (low, medium, high, max)")
	reopenCmd.Flags().String("client", "", "Client")
	reopenCmd.Flags().String("title", "", "Title")
	reopenCmd.Flags().String("draft", "", "Draft ID")
	reopenCmd.Flags().String("goal", "", "Goal ID")
	reopenCmd.Flags().String("parent", "", "Parent ticket slug")
	addLLMFlags(reopenCmd)
	addEffortFlags(reopenCmd)
	addClientFlags(reopenCmd)

	changeCmd := &Command{
		Use:   "change <path>",
		Short: "Change a ticket",
		Args:  ExactArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path := args[0]
			parts := strings.Split(path, "/")
			if len(parts) < 4 {
				return fmt.Errorf("invalid ticket path format: expected YYYY/MM/DD/SLUG")
			}
			y, err := strconv.Atoi(parts[0])
			if err != nil {
				return fmt.Errorf("invalid year")
			}
			m, err := strconv.Atoi(parts[1])
			if err != nil {
				return fmt.Errorf("invalid month")
			}
			d, err := strconv.Atoi(parts[2])
			if err != nil {
				return fmt.Errorf("invalid day")
			}
			slug := strings.Join(parts[3:], "/")

			title, _ := cmd.Flags().GetString("title")
			prompt, _ := cmd.Flags().GetString("prompt")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")
			noManagement, _ := cmd.Flags().GetBool("no-management")

			client, _ := extractClientFromArgs(cmd, []string{})
			llm, _ := extractLLMFromArgs(cmd, []string{})
			effort, _ := extractEffortFromArgs(cmd, []string{})

			input := map[string]interface{}{
				"year":         y,
				"month":        m,
				"day":          d,
				"slug":         slug,
				"noManagement": noManagement,
			}
			if cmd.Flags().Changed("title") {
				input["title"] = title
			}
			if cmd.Flags().Changed("prompt") {
				input["prompt"] = prompt
			}
			if cmd.Flags().Changed("goal") {
				input["goal"] = goal
			}
			if cmd.Flags().Changed("parent") {
				input["parent"] = parent
			}
			if client != "" {
				input["client"] = strings.ToUpper(strings.ReplaceAll(client, "-", "_"))
			}
			if llm != "" {
				input["llm"] = llm
			}
			if effort != "" {
				input["effort"] = effort
			}

			variables := map[string]interface{}{"input": input}
			query := `mutation TicketChange($input: TicketChangeInput!) {
				ticketChange(input: $input) {
					id
					slug
					status
					parent
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("parent", "", "New parent ticket slug")
	changeCmd.Flags().String("title", "", "New title")
	changeCmd.Flags().String("prompt", "", "New prompt")
	changeCmd.Flags().String("goal", "", "New goal ID")
	changeCmd.Flags().String("effort", "", "LLM reasoning effort (low, medium, high, max)")
	changeCmd.Flags().Bool("no-management", false, "Skip management provider sync")
	changeCmd.Flags().MarkHidden("no-management")
	addLLMFlags(changeCmd)
	addEffortFlags(changeCmd)
	addClientFlags(changeCmd)

	purgeArtifactsCmd := &Command{
		Use:   "purge-artifacts [path]",
		Short: "Delete ticket-folder files above 5 MiB and subfolders above 10 MiB",
		RunE: func(cmd *Command, args []string) error {
			all, _ := cmd.Flags().GetBool("all")
			if all || len(args) == 0 {
				count, err := ticketspkg.PurgeAllOversizedTicketArtifacts()
				if err != nil {
					return err
				}
				fmt.Fprintf(cmd.OutOrStdout(), "Purged oversized artifacts in %d ticket folders\n", count)
				return nil
			}
			year, month, day, slug, err := parseTicketPath(args[0])
			if err != nil {
				return err
			}
			ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
			if err != nil {
				return err
			}
			if err := ticketspkg.PurgeOversizedTicketArtifacts(ticket.FolderPath); err != nil {
				return err
			}
			fmt.Fprintf(cmd.OutOrStdout(), "Purged oversized artifacts in %s\n", ticket.FolderPath)
			return nil
		},
	}
	purgeArtifactsCmd.Flags().Bool("all", false, "Purge oversized artifacts in every ticket folder")

	root.AddCommand(changeCmd)
	root.AddCommand(openCmd)
	root.AddCommand(closeCmd)
	root.AddCommand(reopenCmd)
	root.AddCommand(purgeArtifactsCmd)

	return root
}

// ⛳️goalCommand holds the data fields for a goalCommand record.
func goalCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "goal", Short: "Goal management commands"}
	changeCmd := &Command{
		Use:   "change <SLUG>",
		Short: "Change a goal",
		Args:  ExactArgs(1),
		RunE: func(cmd *Command, args []string) error {
			id := args[0]
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			dueDate, _ := cmd.Flags().GetString("due-date")
			parent, _ := cmd.Flags().GetString("parent")
			noManagement, _ := cmd.Flags().GetBool("no-management")

			llm, _ := extractLLMFromArgs(cmd, []string{})
			effort, _ := extractEffortFromArgs(cmd, []string{})

			input := map[string]interface{}{
				"id":           id,
				"noManagement": noManagement,
			}
			if cmd.Flags().Changed("title") {
				input["title"] = title
			}
			if cmd.Flags().Changed("description") {
				input["description"] = description
			}
			if cmd.Flags().Changed("due-date") {
				input["dueDate"] = dueDate
			}
			if cmd.Flags().Changed("parent") {
				input["parent"] = parent
			}
			if llm != "" {
				input["llm"] = llm
			}
			if effort != "" {
				input["effort"] = effort
			}

			variables := map[string]interface{}{"id": id, "input": input}
			query := `mutation GoalChange($id: ID!, $input: GoalChangeInput!) {
				goalChange(id: $id, input: $input) {
					id
					title
					description
					dueDate
					parent
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("title", "", "New title")
	changeCmd.Flags().String("description", "", "New description")
	changeCmd.Flags().String("due-date", "", "New due date (YYYY-MM-DD)")
	changeCmd.Flags().String("parent", "", "New parent goal ID")
	changeCmd.Flags().String("llm", "", "New LLM")
	changeCmd.Flags().String("effort", "", "New LLM reasoning effort (low, medium, high, max)")
	changeCmd.Flags().Bool("no-management", false, "Skip management provider sync")
	changeCmd.Flags().MarkHidden("no-management")
	addLLMFlags(changeCmd)
	addEffortFlags(changeCmd)

	openCmd := &Command{
		Use:   "open [title] [description] [prompt] [client] [llm]",
		Short: "Open a new goal",
		RunE: func(cmd *Command, args []string) error {
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			prompt, _ := cmd.Flags().GetString("prompt")
			dueDate, _ := cmd.Flags().GetString("due-date")
			noManagement, _ := cmd.Flags().GetBool("no-management")

			remainingArgs := args
			if title == "" && len(remainingArgs) > 0 {
				title = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if description == "" && len(remainingArgs) > 0 {
				description = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			client, remainingArgs := extractClientFromArgs(cmd, remainingArgs)
			llm, remainingArgs := extractLLMFromArgs(cmd, remainingArgs)
			effort, _ := extractEffortFromArgs(cmd, remainingArgs)

			if title == "" {
				return fmt.Errorf("missing title")
			}
			if description == "" {
				return fmt.Errorf("missing description")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if dueDate == "" {
				return fmt.Errorf("missing due-date")
			}
			if client == "" {
				return fmt.Errorf("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedClientList(), ", "))
			}
			if llm == "" {
				return fmt.Errorf("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedLLMList(), ", "))
			}

			input := map[string]interface{}{
				"title":        title,
				"description":  description,
				"prompt":       prompt,
				"dueDate":      dueDate,
				"llm":          llm,
				"client":       client,
				"noManagement": noManagement,
			}
			if effort != "" {
				input["effort"] = effort
			}
			parent, _ := cmd.Flags().GetString("parent")
			if parent != "" {
				input["parent"] = parent
			}
			bundle, _ := cmd.Flags().GetString("bundle")
			if bundle != "" {
				input["bundle"] = bundle
			}
			milestone, _ := cmd.Flags().GetString("milestone")
			if milestone != "" {
				input["milestone"] = milestone
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation GoalCreate($input: GoalCreateInput!) {
				goalCreate(input: $input) {
					id
					title
					status
					prompt
					dueDate
					client
					llm
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	openCmd.Flags().String("title", "", "Goal title")
	openCmd.Flags().String("description", "", "Goal description")
	openCmd.Flags().String("prompt", "", "Goal prompt")
	openCmd.Flags().String("due-date", "", "Goal due date (e.g., 2026-02-15)")
	openCmd.Flags().String("llm", "", "LLM")
	openCmd.Flags().String("effort", "", "LLM reasoning effort (low, medium, high, max)")
	openCmd.Flags().String("client", "", "Client")
	openCmd.Flags().Bool("no-management", false, "Skip management provider synchronization")
	openCmd.Flags().MarkHidden("no-management")
	openCmd.Flags().String("parent", "", "Parent goal ID")
	openCmd.Flags().String("bundle", "", "Bundle name associated with this goal")
	openCmd.Flags().String("milestone", "", "Link to existing GitHub milestone URL instead of creating new one")
	addLLMFlags(openCmd)
	addEffortFlags(openCmd)
	addClientFlags(openCmd)

	closeCmd := &Command{
		Use:   "close [id] [summary]",
		Short: "Close a goal",
		RunE: func(cmd *Command, args []string) error {
			id := ""
			summary := ""
			if len(args) > 0 {
				id = args[0]
			}
			if len(args) > 1 {
				summary = args[1]
			}
			if id == "" {
				return fmt.Errorf("missing goal id")
			}
			if summary == "" {
				return fmt.Errorf("missing summary")
			}
			noManagement, _ := cmd.Flags().GetBool("no-management")
			input := map[string]interface{}{
				"id":           id,
				"summary":      summary,
				"noManagement": noManagement,
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation GoalClose($input: GoalCloseInput!) {
				goalClose(input: $input) {
					id
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	closeCmd.Flags().Bool("no-management", false, "Skip management provider synchronization")
	closeCmd.Flags().MarkHidden("no-management")

	reopenCmd := &Command{
		Use:   "reopen [id] [prompt] [client] [llm]",
		Short: "Reopen a goal",
		RunE: func(cmd *Command, args []string) error {
			id := ""
			prompt, _ := cmd.Flags().GetString("prompt")
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			dueDate, _ := cmd.Flags().GetString("due-date")
			parent, _ := cmd.Flags().GetString("parent")
			noManagement, _ := cmd.Flags().GetBool("no-management")

			if len(args) > 0 {
				id = args[0]
			}
			remainingArgs := args
			if len(remainingArgs) > 0 {
				remainingArgs = remainingArgs[1:]
			}

			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			client, remainingArgs := extractClientFromArgs(cmd, remainingArgs)
			llm, remainingArgs := extractLLMFromArgs(cmd, remainingArgs)
			effort, _ := extractEffortFromArgs(cmd, remainingArgs)

			if id == "" {
				return fmt.Errorf("missing goal id")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if client == "" {
				return fmt.Errorf("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedClientList(), ", "))
			}
			if llm == "" {
				return fmt.Errorf("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: %s", strings.Join(model.AllowedLLMList(), ", "))
			}

			input := map[string]interface{}{
				"id":           id,
				"prompt":       prompt,
				"client":       client,
				"llm":          llm,
				"noManagement": noManagement,
			}
			if effort != "" {
				input["effort"] = effort
			}
			if title != "" {
				input["title"] = title
			}
			if description != "" {
				input["description"] = description
			}
			if dueDate != "" {
				input["dueDate"] = dueDate
			}
			if parent != "" {
				input["parent"] = parent
			}

			variables := map[string]interface{}{"input": input}
			query := `mutation GoalReopen($input: GoalReopenInput!) {
				goalReopen(input: $input) {
					id
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	reopenCmd.Flags().Bool("no-management", false, "Skip management provider synchronization")
	reopenCmd.Flags().MarkHidden("no-management")
	reopenCmd.Flags().String("prompt", "", "Prompt")
	reopenCmd.Flags().String("title", "", "New title")
	reopenCmd.Flags().String("description", "", "New description")
	reopenCmd.Flags().String("due-date", "", "New due date")
	reopenCmd.Flags().String("parent", "", "New parent goal")
	reopenCmd.Flags().String("llm", "", "LLM")
	reopenCmd.Flags().String("effort", "", "LLM reasoning effort (low, medium, high, max)")
	reopenCmd.Flags().String("client", "", "Client")
	addLLMFlags(reopenCmd)
	addEffortFlags(reopenCmd)
	addClientFlags(reopenCmd)

	root.AddCommand(changeCmd)
	root.AddCommand(openCmd)
	root.AddCommand(closeCmd)
	root.AddCommand(reopenCmd)
	return root
}

// 🟥️interactionCommand holds the data fields for a interactionCommand record.
func interactionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "interaction", Short: "Interaction management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List all interactions",
		RunE: func(cmd *Command, args []string) error {
			sorted, _ := cmd.Flags().GetBool("sorted")
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction list"}
				if sorted {
					interactions, _ := ticketspkg.ListInteractions()
					sort.Slice(interactions, func(i, j int) bool {
						return interactions[i].Date < interactions[j].Date
					})
					for _, ix := range interactions {
						data, err := json.Marshal(map[string]interface{}{"interaction": ix})
						if err != nil {
							continue
						}
						stream <- Event{Kind: KindResult, Command: "interaction list", Data: data}
					}
				} else {
					ch := make(chan model.InteractionResource)
					go func() {
						ticketspkg.StreamInteractions(context.Background(), ch)
					}()
					for ix := range ch {
						data, err := json.Marshal(map[string]interface{}{"interaction": ix})
						if err != nil {
							continue
						}
						stream <- Event{Kind: KindResult, Command: "interaction list", Data: data}
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().Bool("sorted", false, "Sort interactions by date instead of streaming")
	bindStreamFlags(listCmd)
	treeCmd := &Command{
		Use:   "tree",
		Short: "Show interactions within goal/ticket tree",
		RunE: func(cmd *Command, args []string) error {
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction tree"}
				goals, _ := goalspkg.ListGoals()
				tickets, _ := ticketspkg.ListTickets(nil, nil, nil)
				sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
				sort.Slice(tickets, func(i, j int) bool { return tickets[i].GetID() < tickets[j].GetID() })
				type treeItem struct {
					id       string
					label    string
					parent   string
					children []*treeItem
				}
				itemMap := make(map[string]*treeItem)
				var roots []*treeItem
				for _, g := range goals {
					item := &treeItem{id: g.ID, label: g.GetID() + " - " + g.Title + " - " + g.Status, parent: g.Parent}
					itemMap[g.ID] = item
				}
				for _, t := range tickets {
					tid := t.GetID()
					item := &treeItem{id: tid, label: tid + " - " + t.Title + " - " + string(t.Status)}
					for _, ix := range t.Interactions {
						ixLabel := fmt.Sprintf("🔄️ %s [%s] %s @%s", ix.Kind, ix.Date, ix.Client, ix.Author)
						item.children = append(item.children, &treeItem{id: tid + "/" + ix.Kind + "/" + ix.Date, label: ixLabel})
					}
					goalID := t.Goal
					if goalID != "" {
						item.parent = goalID
					}
					itemMap[tid] = item
				}
				for _, item := range itemMap {
					if item.parent != "" {
						if p, ok := itemMap[item.parent]; ok {
							p.children = append(p.children, item)
							continue
						}
					}
					roots = append(roots, item)
				}
				sort.Slice(roots, func(i, j int) bool { return roots[i].id < roots[j].id })
				var renderTree func(items []*treeItem, prefix string)
				renderTree = func(items []*treeItem, prefix string) {
					for i, item := range items {
						connector := prefix + "├️─️─️ "
						childPrefix := prefix + "│️   "
						if i == len(items)-1 {
							connector = prefix + "└️─️─️ "
							childPrefix = prefix + "    "
						}
						stream <- Event{Kind: KindLog, Level: "info", Command: "interaction tree", Message: connector + item.label}
						sort.Slice(item.children, func(a, b int) bool { return item.children[a].id < item.children[b].id })
						renderTree(item.children, childPrefix)
					}
				}
				renderTree(roots, "")
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

// 🟧️statuteCommand holds the data fields for a statuteCommand record.
func statuteCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "statute", Short: "Statute management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List statutes",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute list"}
				vkChan := make(chan model.StatuteMeta)
				go func() {
					goalspkg.StreamStatutes(context.Background(), vkChan, opts)
				}()
				for vk := range vkChan {
					data, err := json.Marshal(map[string]interface{}{"statute": vk})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "statute list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	treeCmd := &Command{
		Use:   "tree",
		Short: "Show statute tree",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute tree"}
				vkChan := make(chan model.StatuteMeta)
				go func() {
					goalspkg.StreamStatutes(context.Background(), vkChan, opts)
				}()
				var vks []model.StatuteMeta
				for vk := range vkChan {
					vks = append(vks, vk)
				}
				sort.Slice(vks, func(i, j int) bool { return string(vks[i].Kind) < string(vks[j].Kind) })
				if config.IsMarkdown() {
					var sb strings.Builder
					var kinds []model.Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := treepkg.BuildStatuteTreeForRepo(kinds)
					var renderVkTree func(nodes []*treepkg.TreeNode, indent string)
					renderVkTree = func(nodes []*treepkg.TreeNode, indent string) {
						for _, n := range nodes {
							if n.Data != nil {
								vkData := map[string]interface{}{"id": n.Data["id"], "description": n.Data["reason"]}
								sb.WriteString(indent + model.RenderEntityMarkdown("statute", vkData) + "\n")
							} else {
								sb.WriteString(indent + "- " + n.Label + "\n")
							}
							renderVkTree(n.Children, indent+"  ")
						}
					}
					renderVkTree(vkTree, "")
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "statute tree", Data: data}
				} else {
					var kinds []model.Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := treepkg.BuildStatuteTreeForRepo(kinds)
					var renderVkText func(nodes []*treepkg.TreeNode, prefix string)
					renderVkText = func(nodes []*treepkg.TreeNode, prefix string) {
						for j, n := range nodes {
							conn := prefix + "├️─️─️ "
							childPrefix := prefix + "│️   "
							if j == len(nodes)-1 {
								conn = prefix + "└️─️─️ "
								childPrefix = prefix + "    "
							}
							label := n.Label
							if n.Description != "" {
								label += " - " + n.Description
							}
							stream <- Event{Kind: KindLog, Level: "info", Command: "statute tree", Message: conn + label}
							renderVkText(n.Children, childPrefix)
						}
					}
					renderVkText(vkTree, "")
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

func checkpointCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "checkpoint", Short: "Checkpoint management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List checkpoints",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			limit := 100
			if l, _ := cmd.Flags().GetInt("limit"); l > 0 {
				limit = l
			}
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "checkpoint list"}
				checkpointChan := make(chan model.Checkpoint)
				go func() {
					contributorspkg.StreamCheckpoints(context.Background(), &limit, checkpointChan, opts)
				}()
				for c := range checkpointChan {
					data, err := json.Marshal(map[string]interface{}{"checkpoint": c})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "checkpoint list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().Int("limit", 100, "Maximum number of checkpoints to show")
	bindStreamFlags(listCmd)
	root.AddCommand(listCmd)
	return root
}

// 🤝️contributorCommand holds the data fields for a contributorCommand record.
func contributorCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "contributor", Short: "Contributor management commands"}
	addCmd := &Command{
		Use:   "add",
		Short: "Add a contributor",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			name, _ := cmd.Flags().GetString("name")
			emails, _ := cmd.Flags().GetStringSlice("email")
			if github == "" && len(args) > 0 {
				github = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if len(emails) == 0 && len(args) > 2 {
				emails = args[2:]
			}
			if github == "" {
				return fmt.Errorf("missing github")
			}
			input := map[string]interface{}{"github": github}
			if name != "" {
				input["name"] = name
			}
			if len(emails) > 0 {
				input["emails"] = emails
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation ContributorAdd($input: ContributorAddInput!) {
				contributorAdd(input: $input) { id github name emails }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	addCmd.Flags().String("github", "", "GitHub username")
	addCmd.Flags().String("name", "", "Contributor name")
	addCmd.Flags().StringSlice("email", nil, "Contributor emails")
	removeCmd := &Command{
		Use:   "remove",
		Short: "Remove a contributor",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			if github == "" && len(args) > 0 {
				github = args[0]
			}
			if github == "" {
				return fmt.Errorf("missing github")
			}
			variables := map[string]interface{}{"github": github}
			query := `mutation ContributorRemove($github: String!) { contributorRemove(github: $github) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	removeCmd.Flags().String("github", "", "GitHub username")
	root.AddCommand(addCmd)
	root.AddCommand(removeCmd)
	return root
}

// 🛠️technologyCommand holds the data fields for a technologyCommand record.
func technologyCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:                "technology",
		Short:              "Manage technologies",
		DisableFlagParsing: false,
		Args:               ArbitraryArgs,
		RunE: func(cmd *Command, args []string) error {
			if len(args) == 3 && args[1] == "generate" {
				technologyName := args[0]
				kind := args[2]
				switch kind {
				case "requirements":
					return testrunner.GenerateTechnologyRequirements(technologyName)
				case "docs":
					return testrunner.GenerateTechnologyDocs(technologyName)
				case "todos":
					return testrunner.GenerateTechnologyTodos(technologyName)
				default:
					return fmt.Errorf("unknown generate kind %q (use requirements, docs, or todos)", kind)
				}
			}
			return cmd.Help()
		},
	}

	listCmd := &Command{
		Use:   "list",
		Short: "List technologies",
		RunE: func(cmd *Command, args []string) error {
			return runTechnologyList(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(listCmd)
	cmd.AddCommand(listCmd)

	treeCmd := &Command{
		Use:   "tree",
		Short: "Show technology tree Structure",
		RunE: func(cmd *Command, args []string) error {
			return runTechnologyTree(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(treeCmd)
	cmd.AddCommand(treeCmd)

	return cmd
}

// 📦️bundleCommand holds the data fields for a bundleCommand record.
func bundleCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "bundle", Short: "Bundle management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List bundles",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = testrunner.GetBundlesWithOpenTickets()
			}

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle list"}

				bundleChan := make(chan model.Bundle)
				go func() {
					ticketspkg.StreamBundles(context.Background(), bundleChan, opts)
				}()

				for b := range bundleChan {

					if statusFilter != nil {
						hasOpenTicket := bundlesWithOpenTickets[b.Name]
						if *statusFilter == "open" && !hasOpenTicket {
							continue
						}
						if *statusFilter == "closed" && hasOpenTicket {
							continue
						}
					}

					data, err := json.Marshal(map[string]interface{}{"bundle": b})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "bundle list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	bindStatusFlags(listCmd)

	treeCmd := &Command{
		Use:   "tree",
		Short: "Show bundle tree",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = testrunner.GetBundlesWithOpenTickets()
			}
			_ = bundlesWithOpenTickets

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle tree"}

				bundleChan := make(chan model.Bundle)
				go func() {
					ticketspkg.StreamBundles(context.Background(), bundleChan, opts)
				}()

				var bundles []model.Bundle
				for b := range bundleChan {
					bundles = append(bundles, b)
				}

				sort.Slice(bundles, func(i, j int) bool {
					return bundles[i].Name < bundles[j].Name
				})

				if config.IsMarkdown() {
					var sb strings.Builder
					for _, b := range bundles {
						bMap := map[string]interface{}{}
						bBytes, _ := json.Marshal(b)
						json.Unmarshal(bBytes, &bMap)
						line := model.RenderEntityMarkdown("bundle", bMap)
						sb.WriteString(line + "\n")
					}
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "bundle tree", Data: data}
				} else {
					stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: "."}
					for i, b := range bundles {
						connector := "├️─️─️ "
						if i == len(bundles)-1 {
							connector = "└️─️─️ "
						}
						stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: connector + b.Name}
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)
	bindStatusFlags(treeCmd)

	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

// 💠️folderCommand holds the data fields for a folderCommand record.
func folderCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "folder", Short: "Folder management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a folder",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "Folder path")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a folder",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a folder",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderDelete($path: String!) { folderDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "Folder path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

// 🔳️bindStreamFlags holds the data fields for a bindStreamFlags record.
func bindStreamFlags(cmd *Command) {
	bindBoolFlags(cmd, []boolFlagSpec{
		{Name: "show-ignored", Usage: "Show ignored folders and files"},
		{Name: "show-generated", Usage: "Show generated folders and files"},
	})

	bindOnlyNoFlagPairs(cmd, []boolFlagPairSpec{
		{OnlyName: "only-code", NoName: "no-code", OnlyUsage: "Only show code files", NoUsage: "Exclude code files"},
		{OnlyName: "only-script", NoName: "no-script", OnlyUsage: "Only show script files", NoUsage: "Exclude script files"},
		{OnlyName: "only-config", NoName: "no-config", OnlyUsage: "Only show config files", NoUsage: "Exclude config files"},
		{OnlyName: "only-lab", NoName: "no-lab", OnlyUsage: "Only show lab files", NoUsage: "Exclude lab files"},
		{OnlyName: "only-docs", NoName: "no-docs", OnlyUsage: "Only show docs files", NoUsage: "Exclude docs files"},
		{OnlyName: "only-resource", NoName: "no-resource", OnlyUsage: "Only show resource files", NoUsage: "Exclude resource files"},
		{OnlyName: "only-template", NoName: "no-template", OnlyUsage: "Only show template files", NoUsage: "Exclude template files"},
		{OnlyName: "only-license", NoName: "no-license", OnlyUsage: "Only show license files", NoUsage: "Exclude license files"},
		{OnlyName: "only-library", NoName: "no-library", OnlyUsage: "Only show library bundles", NoUsage: "Exclude library bundles"},
		{OnlyName: "only-schema", NoName: "no-schema", OnlyUsage: "Only show schema bundles", NoUsage: "Exclude schema bundles"},
		{OnlyName: "only-binary", NoName: "no-binary", OnlyUsage: "Only show binary bundles", NoUsage: "Exclude binary bundles"},
		{OnlyName: "only-client", NoName: "no-client", OnlyUsage: "Only show client bundles", NoUsage: "Exclude client bundles"},
		{OnlyName: "only-site", NoName: "no-site", OnlyUsage: "Only show site bundles", NoUsage: "Exclude site bundles"},
		{OnlyName: "only-assets", NoName: "no-assets", OnlyUsage: "Only show asset bundles", NoUsage: "Exclude asset bundles"},
		{OnlyName: "only-organization", NoName: "no-organization", OnlyUsage: "Only show organization folders", NoUsage: "Exclude organization folders"},
		{OnlyName: "only-required", NoName: "no-required", OnlyUsage: "Only show required folders", NoUsage: "Exclude required folders"},
		{OnlyName: "only-implementation", NoName: "no-implementation", OnlyUsage: "Only show implementation definitions", NoUsage: "Exclude implementation definitions"},
		{OnlyName: "only-interface", NoName: "no-interface", OnlyUsage: "Only show interface definitions", NoUsage: "Exclude interface definitions"},
		{OnlyName: "only-constant", NoName: "no-constant", OnlyUsage: "Only show constant definitions", NoUsage: "Exclude constant definitions"},
	})

	cmd.Flags().IntSlice("no-year", nil, "Exclude years")
	cmd.Flags().IntSlice("only-year", nil, "Only show years")
	cmd.Flags().IntSlice("no-month", nil, "Exclude months")
	cmd.Flags().IntSlice("only-month", nil, "Only show months")
	cmd.Flags().IntSlice("no-day", nil, "Exclude days")
	cmd.Flags().IntSlice("only-day", nil, "Only show days")

	cmd.Flags().StringSlice("no-contributor", nil, "Exclude contributors")
	cmd.Flags().StringSlice("only-contributor", nil, "Only show contributors")
	cmd.Flags().StringSlice("no-policy", nil, "Exclude policies")
	cmd.Flags().StringSlice("only-policy", nil, "Only show policies")
	cmd.Flags().StringSlice("no-breach", nil, "Exclude statutes")
	cmd.Flags().StringSlice("only-breach", nil, "Only show statutes")

	cmd.Flags().String("filter", "", "Filter string")
	cmd.Flags().String("query", "", "Full-text search query")
	cmd.Flags().Bool("regex", false, "Use regex for filter")
	cmd.Flags().Bool("match-case", false, "Match case for filter")
	cmd.Flags().Bool("match-whole-word", false, "Match whole word for filter")
}

// 🔲️bindStatusFlags holds the data fields for a bindStatusFlags record.
func bindStatusFlags(cmd *Command) {
	cmd.Flags().Bool("open", false, "Show only open items")
	cmd.Flags().Bool("closed", false, "Show only closed items")
	cmd.Flags().String("status", "", "Filter by status (open or closed)")
}

// ▪️getStatusFilter holds the data fields for a getStatusFilter record.
func getStatusFilter(cmd *Command) *string {
	open, _ := cmd.Flags().GetBool("open")
	closed, _ := cmd.Flags().GetBool("closed")
	status, _ := cmd.Flags().GetString("status")

	if open {
		s := "open"
		return &s
	}
	if closed {
		s := "closed"
		return &s
	}
	if status != "" {
		return &status
	}
	return nil
}

// ▫️getStreamOptions holds the data fields for a getStreamOptions record.
func getStreamOptions(cmd *Command) model.StreamOptions {
	showIgnored, _ := cmd.Flags().GetBool("show-ignored")
	showGenerated, _ := cmd.Flags().GetBool("show-generated")

	var excludeKinds []string
	if v, _ := cmd.Flags().GetBool("no-code"); v {
		excludeKinds = append(excludeKinds, model.FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("no-script"); v {
		excludeKinds = append(excludeKinds, model.FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("no-config"); v {
		excludeKinds = append(excludeKinds, model.FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("no-lab"); v {
		excludeKinds = append(excludeKinds, model.FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("no-docs"); v {
		excludeKinds = append(excludeKinds, model.FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("no-resource"); v {
		excludeKinds = append(excludeKinds, model.FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("no-template"); v {
		excludeKinds = append(excludeKinds, model.FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("no-license"); v {
		excludeKinds = append(excludeKinds, model.FileKindLicense)
	}

	var includeKinds []string
	if v, _ := cmd.Flags().GetBool("only-code"); v {
		includeKinds = append(includeKinds, model.FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("only-script"); v {
		includeKinds = append(includeKinds, model.FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("only-config"); v {
		includeKinds = append(includeKinds, model.FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("only-lab"); v {
		includeKinds = append(includeKinds, model.FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("only-docs"); v {
		includeKinds = append(includeKinds, model.FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("only-resource"); v {
		includeKinds = append(includeKinds, model.FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("only-template"); v {
		includeKinds = append(includeKinds, model.FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("only-license"); v {
		includeKinds = append(includeKinds, model.FileKindLicense)
	}

	var excludeBundleKinds []model.BundleKind
	if v, _ := cmd.Flags().GetBool("no-library"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("no-schema"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("no-binary"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("no-client"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("no-site"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("no-assets"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindAssets)
	}

	var includeBundleKinds []model.BundleKind
	if v, _ := cmd.Flags().GetBool("only-library"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("only-schema"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("only-binary"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("only-client"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("only-site"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("only-assets"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindAssets)
	}

	var excludeFolderKinds []model.FolderKind
	if v, _ := cmd.Flags().GetBool("no-organization"); v {
		excludeFolderKinds = append(excludeFolderKinds, model.FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("no-required"); v {
		excludeFolderKinds = append(excludeFolderKinds, model.FolderKindRequired)
	}

	var includeFolderKinds []model.FolderKind
	if v, _ := cmd.Flags().GetBool("only-organization"); v {
		includeFolderKinds = append(includeFolderKinds, model.FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("only-required"); v {
		includeFolderKinds = append(includeFolderKinds, model.FolderKindRequired)
	}

	var excludeDefinitionKinds []model.DefinitionKind
	if v, _ := cmd.Flags().GetBool("no-implementation"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("no-interface"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("no-constant"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindConstant)
	}

	var includeDefinitionKinds []model.DefinitionKind
	if v, _ := cmd.Flags().GetBool("only-implementation"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("only-interface"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("only-constant"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindConstant)
	}

	excludeYears, _ := cmd.Flags().GetIntSlice("no-year")
	includeYears, _ := cmd.Flags().GetIntSlice("only-year")
	excludeMonths, _ := cmd.Flags().GetIntSlice("no-month")
	includeMonths, _ := cmd.Flags().GetIntSlice("only-month")
	excludeDays, _ := cmd.Flags().GetIntSlice("no-day")
	includeDays, _ := cmd.Flags().GetIntSlice("only-day")

	excludeContributors, _ := cmd.Flags().GetStringSlice("no-contributor")
	includeContributors, _ := cmd.Flags().GetStringSlice("only-contributor")
	excludePolicies, _ := cmd.Flags().GetStringSlice("no-policy")
	includePolicies, _ := cmd.Flags().GetStringSlice("only-policy")
	excludeBreachs, _ := cmd.Flags().GetStringSlice("no-breach")
	includeBreachs, _ := cmd.Flags().GetStringSlice("only-breach")

	filter, _ := cmd.Flags().GetString("filter")
	query, _ := cmd.Flags().GetString("query")
	regex, _ := cmd.Flags().GetBool("regex")
	matchCase, _ := cmd.Flags().GetBool("match-case")
	matchWholeWord, _ := cmd.Flags().GetBool("match-whole-word")

	return model.StreamOptions{
		ShowIgnored:            showIgnored,
		ShowGenerated:          showGenerated,
		ExcludeKinds:           excludeKinds,
		IncludeKinds:           includeKinds,
		ExcludeBundleKinds:     excludeBundleKinds,
		IncludeBundleKinds:     includeBundleKinds,
		ExcludeFolderKinds:     excludeFolderKinds,
		IncludeFolderKinds:     includeFolderKinds,
		ExcludeDefinitionKinds: excludeDefinitionKinds,
		IncludeDefinitionKinds: includeDefinitionKinds,
		ExcludeYears:           excludeYears,
		IncludeYears:           includeYears,
		ExcludeMonths:          excludeMonths,
		IncludeMonths:          includeMonths,
		ExcludeDays:            excludeDays,
		IncludeDays:            includeDays,
		ExcludeContributors:    excludeContributors,
		IncludeContributors:    includeContributors,
		ExcludePolicies:        excludePolicies,
		IncludePolicies:        includePolicies,
		ExcludeBreachs:         excludeBreachs,
		IncludeBreachs:         includeBreachs,
		Filter:                 filter,
		Query:                  query,
		Regex:                  regex,
		MatchCase:              matchCase,
		MatchWholeWord:         matchWholeWord,
	}
}

// ◾fileCommand holds the data fields for a fileCommand record.
func fileCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "file", Short: "File management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a file",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileCreate($path: String!) { fileCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "File path")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a file",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FileMove($src: String!, $dst: String!) { fileMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a file",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileDelete($path: String!) { fileDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "File path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

// ◽sectionCommand holds the data fields for a sectionCommand record.
func sectionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "section", Short: "Section management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a section",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			parent, _ := cmd.Flags().GetString("parent")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if parent == "" && len(args) > 2 {
				parent = args[2]
			}
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			if parent != "" {
				variables["parent"] = parent
			}
			query := `mutation SectionCreate($file: String!, $name: String!, $parent: String) { sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start end } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("file", "", "File path")
	createCmd.Flags().String("name", "", "Section name")
	createCmd.Flags().String("parent", "", "Parent section")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a section",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			oldName, _ := cmd.Flags().GetString("old")
			newName, _ := cmd.Flags().GetString("new")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if oldName == "" && len(args) > 1 {
				oldName = args[1]
			}
			if newName == "" && len(args) > 2 {
				newName = args[2]
			}
			if file == "" || oldName == "" || newName == "" {
				return fmt.Errorf("missing file or names")
			}
			variables := map[string]interface{}{"file": file, "oldName": oldName, "newName": newName}
			query := `mutation SectionMove($file: String!, $oldName: String!, $newName: String!) { sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start end } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("file", "", "File path")
	moveCmd.Flags().String("old", "", "Old section name")
	moveCmd.Flags().String("new", "", "New section name")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a section",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			query := `mutation SectionDelete($file: String!, $name: String!) { sectionDelete(file: $file, name: $name) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("file", "", "File path")
	deleteCmd.Flags().String("name", "", "Section name")
	integrateCmd := &Command{
		Use:   "integrate",
		Short: "Integrate source code into a target file section",
		Args:  MaximumNArgs(4),
		RunE: func(cmd *Command, args []string) error {
			source, _ := cmd.Flags().GetString("source")
			targetSection, _ := cmd.Flags().GetString("target-section")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetParent, _ := cmd.Flags().GetString("target-parent")
			if source == "" && len(args) > 0 {
				source = args[0]
			}
			if targetSection == "" && len(args) > 1 {
				targetSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}
			if targetParent == "" && len(args) > 3 {
				targetParent = args[3]
			}
			if source == "" || targetSection == "" || targetFile == "" {
				return fmt.Errorf("missing source, target section, or target file")
			}
			variables := map[string]interface{}{
				"source":        source,
				"targetSection": targetSection,
				"targetFile":    targetFile,
			}
			if targetParent != "" {
				variables["targetParent"] = targetParent
			}
			query := `mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) { integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	integrateCmd.Flags().String("source", "", "Source file path")
	integrateCmd.Flags().String("target-section", "", "Target section name")
	integrateCmd.Flags().String("target-file", "", "Target file path")
	integrateCmd.Flags().String("target-parent", "", "Target parent section name")

	extractCmd := &Command{
		Use:   "extract",
		Short: "Extract a section from a source file into a target file",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			sourceFile, _ := cmd.Flags().GetString("source-file")
			sourceSection, _ := cmd.Flags().GetString("source-section")
			targetFile, _ := cmd.Flags().GetString("target-file")

			if sourceFile == "" && len(args) > 0 {
				sourceFile = args[0]
			}
			if sourceSection == "" && len(args) > 1 {
				sourceSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}

			if sourceFile == "" || sourceSection == "" || targetFile == "" {
				return fmt.Errorf("missing source file, source section, or target file")
			}

			variables := map[string]interface{}{
				"sourceFile":    sourceFile,
				"sourceSection": sourceSection,
				"targetFile":    targetFile,
			}
			query := `mutation Extract($sourceFile: String!, $sourceSection: String!, $targetFile: String!) { extract(sourceFile: $sourceFile, sourceSection: $sourceSection, targetFile: $targetFile) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	extractCmd.Flags().String("source-file", "", "Source file path")
	extractCmd.Flags().String("source-section", "", "Source section name")
	extractCmd.Flags().String("target-file", "", "Target file path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(integrateCmd)
	root.AddCommand(extractCmd)
	return root
}

// ◻definitionCommand holds the data fields for a definitionCommand record.
func definitionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "definition", Short: "Definition management commands"}
	listCmd := &Command{
		Use:     "list",
		Aliases: []string{"tree"},
		Short:   "List definitions",
		Args:    MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("file")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing file")
			}

			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "definition list"}

				defChan := make(chan model.Definition)
				go func() {
					ticketspkg.StreamDefinitions(context.Background(), path, defChan, opts)
				}()

				for d := range defChan {
					data, err := json.Marshal(map[string]interface{}{"definition": d})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "definition list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	bindStreamFlags(listCmd)
	root.AddCommand(listCmd)
	return root
}

// 🚚️moveCommand holds the data fields for a moveCommand record.
func moveCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "move <source> <target>",
		Short: "Move an artifact from source to target",
		Long:  "Move an artifact (file, folder, section) between locations. Supports cross-kind moves: file→section calls integrate, section→file calls extract.",
		Args:  ExactArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			source := model.ParseArtifactRef(args[0])
			target := model.ParseArtifactRef(args[1])

			var result workspace.ToolResult

			switch {
			case source.Kind == "file" && target.Kind == "file":
				result = move.ToolFileMove(source.Path, target.Path)
			case source.Kind == "folder" && target.Kind == "folder":
				result = move.ToolFolderMove(source.Path, target.Path)
			case source.Kind == "section" && target.Kind == "section" && source.Path == target.Path:
				if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				oldName := codebasepkg.ResolveSectionName(source.Path, source.SectionParts[len(source.SectionParts)-1])
				newName := codebasepkg.ResolveSectionName(target.Path, target.SectionParts[len(target.SectionParts)-1])
				result = move.ToolSectionMove(source.Path, oldName, newName)
			case source.Kind == "file" && target.Kind == "section":
				targetFile := target.Path
				var sectionName, parentSection string
				if len(target.SectionParts) > 0 {
					sectionName = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
				}
				if len(target.SectionParts) > 1 {
					parentSection = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
				}
				result = move.ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
				if result.Error == "" {
					absSource := filepath.Join(workspace.RootDir, source.Path)
					os.Remove(absSource)
					ticketspkg.RemoveAgentsDocsEntry(source.Path)
				}
			case source.Kind == "section" && target.Kind == "file":
				if len(source.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				sourceFile := source.Path
				sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
				result = move.ToolExtract(sourceFile, sectionName, target.Path)
			default:
				return fmt.Errorf("unsupported move: %s → %s", source.Kind, target.Kind)
			}

			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
}

func integrateCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "integrate [source] [target]",
		Short: "Integrate source code into a target file section",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := model.ParseArtifactRef(args[0])
				target := model.ParseArtifactRef(args[1])

				if source.Kind == "file" && target.Kind == "section" {
					targetFile := target.Path
					var sectionName, parentSection string
					if len(target.SectionParts) > 0 {
						sectionName = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
					}
					if len(target.SectionParts) > 1 {
						parentSection = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
					}
					result := move.ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
					if result.Error != "" {
						return fmt.Errorf("%s", result.Error)
					}
					for _, line := range result.Output.Lines {
						fmt.Println(line.Text)
					}
					return nil
				}
			}

			file, _ := cmd.Flags().GetString("file")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetSection, _ := cmd.Flags().GetString("target-section")
			parentSection, _ := cmd.Flags().GetString("parent-section")

			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" || targetFile == "" || targetSection == "" {
				return fmt.Errorf("missing file, target-file, or target-section")
			}

			result := move.ToolIntegrate(file, targetSection, targetFile, parentSection)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
	cmd.Flags().String("file", "", "Source file path")
	cmd.Flags().String("target-file", "", "Target file path")
	cmd.Flags().String("target-section", "", "Target section name")
	cmd.Flags().String("parent-section", "", "Parent section name")
	return cmd
}

// ◼extractCommand holds the data fields for a extractCommand record.
func extractCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "extract [source] [target]",
		Short: "Extract a section from a source file into a target file",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := model.ParseArtifactRef(args[0])
				target := model.ParseArtifactRef(args[1])

				if source.Kind == "section" && target.Kind == "file" {
					if len(source.SectionParts) == 0 {
						return fmt.Errorf("missing section path")
					}
					sourceFile := source.Path
					sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
					result := move.ToolExtract(sourceFile, sectionName, target.Path)
					if result.Error != "" {
						return fmt.Errorf("%s", result.Error)
					}
					for _, line := range result.Output.Lines {
						fmt.Println(line.Text)
					}
					return nil
				}
			}

			file, _ := cmd.Flags().GetString("file")
			section, _ := cmd.Flags().GetString("section")
			targetFile, _ := cmd.Flags().GetString("target-file")

			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" || section == "" || targetFile == "" {
				return fmt.Errorf("missing file, section, or target-file")
			}

			result := move.ToolExtract(file, section, targetFile)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
	cmd.Flags().String("file", "", "Source file path")
	cmd.Flags().String("section", "", "Section name")
	cmd.Flags().String("parent-section", "", "Parent section name")
	cmd.Flags().String("target-file", "", "Target file path")
	return cmd
}

// #endregion 🕸️Test Command

// #region 🔤️Rename

// 🔤️renameCommand returns a command command that renames a token across the repo in all case variants.
func renameCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "rename <old> <new> [scope]",
		Short: "Rename a token across non-gitignored files (all case variants)",
		Long:  "Rewrite UPPER, Title and lower case variants of <old> to <new> in every non-gitignored file's contents and filenames (including folder names) under the optional scope directory (default: repo root). Example: repo rename model representation compose.",
		Args:  RangeArgs(2, 3),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}
			scope := ""
			if len(args) == 3 {
				scope = args[2]
			}
			result := move.ToolRename(args[0], args[1], scope)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
}

// #endregion 🔤️Rename

// #region 🔢️LOC Command

// 🙈️locIgnorer is the repository gitignore seen through the metrics domain's ignore port.
type locIgnorer struct{}

// 🙈️MatchesPath answers the metrics domain's only ignore question.
func (locIgnorer) MatchesPath(path string) bool { return workspace.IsIgnoredByGitignore(path) }

// 👤️locContributorAlias is the metrics domain's alias port, backed by the contributor registry.
func locContributorAlias(name, email string) string {
	author := strings.TrimSpace(name)
	mail := strings.TrimSpace(email)
	if mail != "" {
		author = fmt.Sprintf("%s <%s>", author, mail)
	}
	if author == "" {
		return "unknown"
	}
	return contributorspkg.FindAndUpdateContributor(author)
}

// 🔢️runLocCommand builds the report through 📊️metrics' ports and renders it in the requested format.
func runLocCommand(cmd *Command, config *Config, languages []string, history, byContrib bool, historyBranch, contribFilter string) error {
	repoRoot := strings.TrimSpace(config.Repo)
	if repoRoot == "" {
		if wd, err := os.Getwd(); err == nil {
			repoRoot = workspace.FindRepoRoot(wd)
		} else {
			repoRoot = "."
		}
	} else {
		repoRoot = workspace.FindRepoRoot(repoRoot)
	}
	workspace.SetRootDir(repoRoot)
	options := metricspkg.LocOptions{Languages: languages, History: history, ByContributors: byContrib, Branch: historyBranch, Contributor: contribFilter}
	report, err := metricspkg.BuildLocReport(metricspkg.SystemGit{Repo: repoRoot}, options, locIgnorer{}, locContributorAlias)
	if err != nil {
		return err
	}
	out := cmd.OutOrStdout()
	if config.IsJSON() {
		encoder := json.NewEncoder(out)
		encoder.SetIndent("", "  ")
		return encoder.Encode(map[string]*metricspkg.LocReport{"loc": report})
	}
	if config.IsMarkdown() {
		_, err := io.WriteString(out, metricspkg.RenderMarkdown(report, history, byContrib))
		return err
	}
	renderLocText(out, report, history, byContrib, locIsTTY(out))
	return nil
}

// 🖥️locIsTTY reports whether the sink is an interactive terminal that may carry colour.
func locIsTTY(out io.Writer) bool {
	if os.Getenv("NO_COLOR") != "" {
		return false
	}
	file, ok := out.(*os.File)
	if !ok {
		return false
	}
	info, err := file.Stat()
	if err != nil {
		return false
	}
	return (info.Mode() & os.ModeCharDevice) != 0
}

// 📤️renderLocText is the coloured terminal presentation of a report; every table comes from 📊️metrics.
func renderLocText(w io.Writer, report *metricspkg.LocReport, withHistory, byContrib, isTTY bool) {
	fmt.Fprintln(w, model.Colorize("Snapshot", model.ColorBold, isTTY))
	fmt.Fprint(w, metricspkg.TextTable("", report.Snapshot, metricspkg.UseFullTreeTable(report.Snapshot), false))
	if byContrib && len(report.ByContributors) > 0 {
		for _, alias := range locSortedAliases(report.ByContributors) {
			rows := report.ByContributors[alias]
			fmt.Fprintln(w, "")
			fmt.Fprintln(w, model.Colorize("Contributor: "+metricspkg.ContributorEmojiID(alias), model.ColorBlue, isTTY))
			fmt.Fprint(w, metricspkg.TextTable("", rows, metricspkg.UseFullTreeTable(rows), false))
		}
	}
	if !withHistory || len(report.History) == 0 {
		return
	}
	fmt.Fprintln(w, "")
	fmt.Fprintln(w, model.Colorize("History: "+metricspkg.DisplayHistoryBranch(report.Branch), model.ColorBold, isTTY))
	for _, entry := range report.History {
		label := model.Colorize(metricspkg.HistoryCheckpointLabel(entry.SHA)+"  "+entry.Date, model.ColorDim, isTTY)
		if !byContrib {
			fmt.Fprintln(w, label)
			fmt.Fprint(w, metricspkg.TextTable("", entry.Languages, metricspkg.UseFullTreeTable(entry.Languages), true))
			continue
		}
		fmt.Fprintln(w, label, metricspkg.ContributorEmojiID(entry.Author))
		aliases := locSortedAliases(entry.ByContributors)
		for _, alias := range aliases {
			rows := entry.ByContributors[alias]
			if len(aliases) > 1 || !strings.EqualFold(strings.TrimSpace(alias), strings.TrimSpace(entry.Author)) {
				fmt.Fprint(w, "  ")
				fmt.Fprintln(w, model.Colorize(metricspkg.ContributorEmojiID(alias), model.ColorBlue, isTTY))
			}
			fmt.Fprint(w, metricspkg.TextTable("", rows, metricspkg.UseFullTreeTable(rows), true))
		}
	}
}

// 🔤️locSortedAliases is the sorted contributor key list of a contributor-keyed table.
func locSortedAliases(rows map[string]map[string]metricspkg.LocLangStats) []string {
	aliases := make([]string, 0, len(rows))
	for alias := range rows {
		aliases = append(aliases, alias)
	}
	sort.Strings(aliases)
	return aliases
}

// 🔢️ locCommand wires the `loc` command.
func locCommand(factory EngineFactory, config *Config) *Command {
	_ = factory
	cmd := &Command{
		Use:   "loc",
		Short: "Tracked-file LOC (code, markup, data) plus git deltas; internal scan (no cloc)",
		Long:  "Counts tracked files via git (not cloc). The runnable CLI is built from the repo client's MCP Go implementation into the platform-specific client binary. Wip% is each row's edited churn vs the whole first-parent walk on the logged ref (default ⛳️wip), including all contributors. With --history, each row scans the tree at that commit for physical LOC; Δ% is change vs the previous printed history step. Branch and checkpoint lines use the same emoji ids as the repo tree (⛳️wip, 🔀️abc1234, 🧑️‍💻️alias).",
		Args:  NoArgs,
		RunE: func(c *Command, args []string) error {
			langs, _ := c.Flags().GetStringSlice("languages")
			if len(langs) == 0 {
				langs = []string{"TypeScript", "Go", "C#", "Python", "Rust"}
			}
			h, _ := c.Flags().GetBool("history")
			by, _ := c.Flags().GetBool("by-contributors")
			br, _ := c.Flags().GetString("branch")
			bc, _ := c.Flags().GetString("by-contributor")
			if h {
				if strings.TrimSpace(br) == "" {
					br = metricspkg.DefaultBranch
				}
			} else {
				br = ""
			}
			return runLocCommand(c, config, langs, h, by, br, bc)
		},
	}
	cmd.Flags().Bool("history", false, "Per-commit time series; walks --branch (default: "+metricspkg.DefaultBranch+") without checkout")
	cmd.Flags().Bool("by-contributors", false, "Break down cumulative line deltas by first author (FindAndUpdateContributor alias)")
	cmd.Flags().String("by-contributor", "", "Restrict git deltas and history rows to this contributor alias (case-insensitive)")
	cmd.Flags().String("branch", metricspkg.DefaultBranch, "With --history: git ref to log (default dev branch). Ignored when --history is false")
	cmd.Flags().StringSlice("languages", []string{"TypeScript", "Go", "C#", "Python", "Rust"}, "Limits which programming-language buckets are counted; markup/data always included when matched")
	return cmd
}

// #endregion 🔢️LOC Command

// #region ⏲️Mermaid

func mermaidCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{
		Use:   "mermaid <visualization>",
		Short: "Generate mermaid diagram strings",
	}
	root.AddCommand(&Command{
		Use:   "loc-by-technologies-bundles-folders-files",
		Short: "LOC treemap grouped by technology, bundle, folder, file",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByTechnologiesBundlesFoldersFiles())
			return nil
		},
	})
	root.AddCommand(&Command{
		Use:   "loc-by-contributors",
		Short: "LOC treemap grouped by contributor (via git blame)",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByContributors())
			return nil
		},
	})
	root.AddCommand(&Command{
		Use:   "loc-by-language",
		Short: "LOC treemap grouped by programming language",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByLanguage())
			return nil
		},
	})
	return root
}

// #endregion ⏲️Mermaid

// #region 🏩️Codebase

// 🟫️ToolCodebase MUST complete the operation and return consistent results.
func ToolCodebase() workspace.ToolResult {
	output := workspace.NewOutput()
	ctx := codebasepkg.NewCodebaseContext()

	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := ctx.LoadBreachs(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := ctx.LoadTickets(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	ctx.LoadPolicies()

	codebase := codebasepkg.BuildCodebase(ctx)

	output.Success(fmt.Sprintf("Codebase loaded: %d bundles, %d files, %d breachs",
		len(codebase.Bundles), len(codebase.Files), len(codebase.Breachs)))

	return workspace.ToolResult{Output: *output, Data: codebase}
}

// #endregion 🏩️Codebase

// #region 📋️Tickets

// 🟢️runTechnologyList holds the data fields for a runTechnologyList record.
func runTechnologyList(factory EngineFactory, config Config, cmd *Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology list"}

		projChan := make(chan model.Technology)
		go func() {
			ticketspkg.StreamTechnologies(context.Background(), projChan, opts)
		}()

		for p := range projChan {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			stream <- Event{Kind: KindResult, Command: "technology list", Data: data}
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

// 🌳️runTechnologyTree holds the data fields for a runTechnologyTree record.
func runTechnologyTree(factory EngineFactory, config Config, cmd *Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology tree"}

		projChan := make(chan model.Technology)
		go func() {
			ticketspkg.StreamTechnologies(context.Background(), projChan, opts)
		}()

		var technologies []model.Technology
		for p := range projChan {
			technologies = append(technologies, p)
		}
		var events []Event
		for _, p := range technologies {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			events = append(events, Event{Kind: KindResult, Command: "technology tree", Data: data})
		}
		sort.Slice(events, func(i, j int) bool {
			return formatMarkdownResult(events[i].Command, events[i].Data) < formatMarkdownResult(events[j].Command, events[j].Data)
		})
		for _, ev := range events {
			stream <- ev
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

// 🔖️ToolTicketOpen MUST complete the operation successfully.
func ToolTicketOpen(emoji, title, prompt, llm, effort, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string, mcpKind providers.McpClientKind, planID, specID string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketOpenStarting, "repo-cli", eventspkg.TicketOpenPayload{
		Title: title, Prompt: prompt, LLM: llm, Effort: effort, Client: client, Goal: goal, Parent: parent,
	})
	ticket, err := ticketspkg.OpenTicket(emoji, title, prompt, llm, effort, client, draft, noIssue, goal, parent, noManagement, issue, mcpKind, planID, specID)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if ticket == nil {
		output := workspace.NewOutput()
		output.Info("Ticket creation skipped (NOTICKET)")
		return workspace.ToolResult{Output: *output}
	}
	ticketOpenPayload := map[string]interface{}{
		"id":     ticket.GetID(),
		"slug":   ticket.Slug,
		"year":   ticket.Year,
		"month":  ticket.Month,
		"day":    ticket.Day,
		"status": ticket.Status,
		"path":   ticket.FolderPath,
		"uri":    ticket.GetURI(),
	}
	if ticket.Management != nil && ticket.Management.Issue != "" {
		ticketOpenPayload["github"] = map[string]interface{}{"issue": ticket.Management.Issue}
	}
	data, _ := json.Marshal(map[string]interface{}{"ticketOpen": ticketOpenPayload})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	result := toolResultFromEvents(events, ticket)
	if ticket.Management != nil && ticket.Management.Issue != "" {
		result.Output.Success(fmt.Sprintf("GitHub issue: %s", ticket.Management.Issue))
	}
	return result
}

// 🔖️ToolTicketList MUST complete the operation successfully.
// Always lists from ticket storage without building the full monorepo tree.
func ToolTicketList(year, month, day *int) workspace.ToolResult {
	tickets, err := ticketspkg.ListTickets(year, month, day)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	var sb strings.Builder
	for _, t := range tickets {
		created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+t.Year, t.Month, t.Day)
		if started := t.GetDateStarted(); !started.IsZero() {
			created = started.Format(time.RFC3339)
		}
		data := map[string]interface{}{
			"slug":      t.Slug,
			"year":      t.Year,
			"month":     t.Month,
			"day":       t.Day,
			"title":     t.Title,
			"status":    t.Status,
			"createdAt": created,
		}
		sb.WriteString(model.RenderEntityMarkdown("ticket", data) + "\n")
	}
	output := workspace.NewOutput()
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: tickets}
}

// 🔖️ToolTicketRead MUST complete the operation successfully.
func ToolTicketRead(year, month, day int, slug string) workspace.ToolResult {
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+ticket.Year, ticket.Month, ticket.Day)
	if started := ticket.GetDateStarted(); !started.IsZero() {
		created = started.Format(time.RFC3339)
	}
	dates := map[string]interface{}{"created": created}
	if finished := ticket.GetDateFinished(); finished != nil {
		dates["finished"] = finished.Format(time.RFC3339)
	}
	flat := map[string]interface{}{
		"slug":   ticket.Slug,
		"year":   ticket.Year,
		"month":  ticket.Month,
		"day":    ticket.Day,
		"title":  ticket.Title,
		"status": ticket.Status,
		"date":   dates,
	}
	data, _ := json.Marshal(map[string]interface{}{"ticket": flat})
	events := []Event{{Kind: KindResult, Command: "ticket read", Data: data}}
	return toolResultFromEvents(events, ticket)
}

// 🔖️ToolTicketClose MUST complete the operation successfully.
func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noManagement bool) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketCloseStarting, "repo-cli", eventspkg.TicketClosePayload{
		TicketPayload: eventspkg.TicketPayload{ID: model.FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Summary:       summary, Files: files,
	})
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, title); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := providers.GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.FinishTicket(ticket, summary, files, noManagement, false); err != nil {
		return workspace.ToolErrorResult(err)
	}
	created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", 2000+ticket.Year, ticket.Month, ticket.Day)
	if started := ticket.GetDateStarted(); !started.IsZero() {
		created = started.Format(time.RFC3339)
	}
	closeDates := map[string]interface{}{"created": created}
	if finished := ticket.GetDateFinished(); finished != nil {
		closeDates["finished"] = finished.Format(time.RFC3339)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"ticketClose": map[string]interface{}{
			"id":     ticket.GetID(),
			"slug":   ticket.Slug,
			"status": ticket.Status,
			"dates":  closeDates,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, ticket)
}

// 🔖️ToolTicketReopen MUST complete the operation successfully.
func ToolTicketReopen(year, month, day int, slug, prompt, llm, effort, client, draft string, title string, goal string, parent string, noManagement bool, mcpKind providers.McpClientKind, planID, specID string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventTicketReopenStarting, "repo-cli", eventspkg.TicketReopenPayload{
		TicketPayload: eventspkg.TicketPayload{ID: model.FormatTicketRelPath(year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
		Prompt:        prompt, LLM: llm, Effort: effort, Client: client,
	})
	output := workspace.NewOutput()
	ticket, err := ticketspkg.ReadTicket(year, month, day, slug)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	if title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, title); err != nil {
			return workspace.ToolErrorResult(err)
		}
		if ticket.Management != nil && ticket.Management.Issue != "" && !noManagement {
			if err := providers.GetManagementProvider().UpdateIssueTitle(ticket.Management.Issue, title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.ReopenTicket(ticket, prompt, llm, effort, client, draft, goal, parent, noManagement, mcpKind, planID, specID); err != nil {
		return workspace.ToolErrorResult(err)
	}
	output.Success(fmt.Sprintf("\n🔓️ Ticket reopened: %s", ticket.Slug))
	return workspace.ToolResult{Output: *output, Data: ticket}
}

// 📝️ToolDraftCreate MUST complete the operation successfully.
func ToolDraftCreate(title string, paths []string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventDraftCreateStarting, "repo-cli", eventspkg.DraftPayload{
		Title: title,
	})
	output := workspace.NewOutput()
	files, err := todos.LoadTreeFiles(paths)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	draft, err := todos.CreateDraft(draftStore(), title, files)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventDraftCreateEnded, "repo-cli", eventspkg.DraftPayload{Slug: draft.ID, Title: title})
	output.Success(fmt.Sprintf("\n📝️Created draft: %s", draft.ID))
	return workspace.ToolResult{Output: *output, Data: draft}
}

// 🔖️ToolDraftList MUST complete the operation successfully.
func ToolDraftList() workspace.ToolResult {
	drafts := todos.ListDrafts(draftStore())
	var events []Event
	for _, d := range drafts {
		data, _ := json.Marshal(map[string]interface{}{"draft": d})
		events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
	}
	return toolResultFromEvents(events, drafts)
}

// 🔖️ToolDraftDelete MUST complete the operation successfully.
func ToolDraftDelete(slug string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventDraftDeleteStarting, "repo-cli", eventspkg.DraftPayload{Slug: slug})
	if err := todos.DeleteDraft(draftStore(), slug); err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventDraftDeleteEnded, "repo-cli", eventspkg.DraftPayload{Slug: slug})
	return toolResultFromEvents(nil, nil)
}

// 🔖️ToolGoalCreate MUST complete the operation successfully.
func ToolGoalCreate(title, description, prompt, dueDate, llm, client string, noManagement bool, parent, milestone string) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.RootDir)
	goal, err := ctx.GoalCreate(model.GoalCreateInput{
		Title:        title,
		Description:  description,
		Prompt:       prompt,
		DueDate:      dueDate,
		LLM:          llm,
		Client:       client,
		NoManagement: noManagement,
		Parent:       parent,
		Milestone:    milestone,
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalCreate": map[string]interface{}{
			"id":      goal.ID,
			"title":   goal.Title,
			"status":  goal.Status,
			"prompt":  goal.Prompt,
			"dueDate": goal.Dates.Due,
			"client":  goal.Client,
			"llm":     goal.LLM,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, goal)
}

// 🔖️ToolGoalList MUST complete the operation successfully.
// Lists goals from `.🦑️repo/🎯️goals` only — avoids BuildMonorepoTree (full repo walk) while
// keeping the same markdown lines as goal nodes in the monorepo tree.
func ToolGoalList() workspace.ToolResult {
	goals, err := goalspkg.ListGoals()
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	output := workspace.NewOutput()
	var sb strings.Builder
	for _, g := range goals {
		goalCreatedAt := ""
		sb.WriteString(model.RenderEntityMarkdown("goal", map[string]interface{}{
			"id":          g.ID,
			"title":       g.Title,
			"status":      g.Status,
			"dueDate":     g.DueDate,
			"createdAt":   goalCreatedAt,
			"description": g.Description,
		}) + "\n")
	}
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: goals}
}

// 🔖️ToolGoalClose MUST complete the operation successfully.
func ToolGoalClose(id, summary string, noManagement bool) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.RootDir)
	res, err := ctx.GoalClose(model.GoalCloseInput{
		ID:           id,
		Summary:      summary,
		NoManagement: noManagement,
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalClose": map[string]interface{}{
			"id":     res.ID,
			"status": res.Status,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🔖️ToolGoalReopen MUST complete the operation successfully.
func ToolGoalReopen(id, prompt, llm, client, title, description, dueDate string, noManagement bool) workspace.ToolResult {
	ctx := graphqlpkg.NewRepoContext(workspace.RootDir)
	var titlePtr, descriptionPtr, dueDatePtr *string
	if title != "" {
		titlePtr = &title
	}
	if description != "" {
		descriptionPtr = &description
	}
	if dueDate != "" {
		dueDatePtr = &dueDate
	}
	res, err := ctx.GoalReopen(model.GoalReopenInput{
		ID:           id,
		Prompt:       prompt,
		LLM:          llm,
		Client:       client,
		Title:        titlePtr,
		Description:  descriptionPtr,
		DueDate:      dueDatePtr,
		NoManagement: noManagement,
	})
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"goalReopen": map[string]interface{}{
			"id":     res.ID,
			"status": res.Status,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🤝️ToolContributorAdd MUST complete the operation successfully.
func ToolContributorAdd(github string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventContributorAddStarting, "repo-cli", eventspkg.ContributorPayload{Github: github})
	contributor, err := contributorspkg.CreateContributor(github)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"contributorAdd": map[string]interface{}{
			"id":     contributor.Github,
			"github": contributor.Github,
			"name":   contributor.Name,
			"emails": contributor.Emails,
		},
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, contributor)
}

// 🔖️ToolContributorList MUST complete the operation successfully.
func ToolContributorList() workspace.ToolResult {
	contributors := contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(workspace.GetRootDir()))
	var events []Event
	for _, c := range contributors {
		data, _ := json.Marshal(map[string]interface{}{"contributor": c})
		events = append(events, Event{Kind: KindResult, Command: "contributor list", Data: data})
	}
	return toolResultFromEvents(events, contributors)
}

// ➖️ToolContributorRemove MUST complete the operation successfully.
func ToolContributorRemove(github string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventContributorRemoveStarting, "repo-cli", eventspkg.ContributorPayload{Github: github})
	if err := contributorspkg.RemoveContributor(github); err != nil {
		return workspace.ToolErrorResult(err)
	}
	data, _ := json.Marshal(map[string]interface{}{
		"contributorRemove": github,
	})
	events := []Event{{Kind: KindResult, Command: "graphql", Data: data}}
	return toolResultFromEvents(events, nil)
}

// 🔖️ToolTechnologyList MUST complete the operation successfully.
func ToolTechnologyList() workspace.ToolResult {
	technologies := codebasepkg.LoadTechnologies()
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })
	var events []Event
	for _, p := range technologies {
		data, _ := json.Marshal(map[string]interface{}{"technology": p})
		events = append(events, Event{Kind: KindResult, Command: "technology list", Data: data})
	}
	return toolResultFromEvents(events, technologies)
}

// 🔖️ToolBundleList MUST complete the operation successfully.
func ToolBundleList() workspace.ToolResult {
	bundles := codebasepkg.LoadBundles()
	sort.Slice(bundles, func(i, j int) bool { return bundles[i].Name < bundles[j].Name })
	var events []Event
	for _, b := range bundles {
		data, _ := json.Marshal(map[string]interface{}{"bundle": b})
		events = append(events, Event{Kind: KindResult, Command: "bundle list", Data: data})
	}
	return toolResultFromEvents(events, bundles)
}

// 🔖️ToolTechnologyTree MUST complete the operation successfully.
func ToolTechnologyTree() workspace.ToolResult {
	return toolResultFromTreeRender(treepkg.TreeNodeTechnology)
}

// 🔖️ToolFolderList MUST complete the operation successfully.
func ToolFolderList(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	folders, err := workspace.ListDirEntries(absPath, true)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	var relPaths []string
	for _, f := range folders {
		relPaths = append(relPaths, workspace.NormalizePath(filepath.Join(path, f)))
	}
	ignored := workspace.GetGitIgnoredSet(relPaths)
	var filtered []string
	for _, f := range folders {
		relPath := workspace.NormalizePath(filepath.Join(path, f))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, f)
		}
	}
	output.Info(fmt.Sprintf("\n📁️Found %d folders in %s:\n", len(filtered), path))
	for _, f := range filtered {
		output.Plain(fmt.Sprintf("   %s/", f))
	}
	return workspace.ToolResult{Output: *output, Data: filtered}
}

// 🔖️ToolFolderTree MUST complete the operation successfully.
func ToolFolderTree(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📁️Folder tree: %s\n", path))
	ticketspkg.PrintTree(output, absPath, "")
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileList MUST complete the operation successfully.
func ToolFileList(scopeRaw string) workspace.ToolResult {
	output := workspace.NewOutput()
	scope := workspace.ParseScope(scopeRaw)
	bundles := codebasepkg.GetTechnologies()
	files, err := model.ScopeToFiles(scope, bundles)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	output.Info(fmt.Sprintf("\n📄️Found %d files in scope \"%s\":\n", len(files), scopeRaw))
	for i, f := range files {
		if i >= 50 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(files)-50))
			break
		}
		output.Plain(fmt.Sprintf("   %s", f))
	}
	return workspace.ToolResult{Output: *output, Data: files}
}

// 🔖️ToolFileTree MUST complete the operation successfully.
func ToolFileTree(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Path not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📄️File tree: %s\n", path))
	ticketspkg.PrintTree(output, absPath, "")
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolSectionTree MUST complete the operation successfully.
func ToolSectionTree(filePath string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.Split(filePath, "#")[0])
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sections := languagespkg.ParseSections(content, filePath)
	output.Info(fmt.Sprintf("\n🏷️Sections in %s:\n", filePath))
	var printSection func(s model.Section, prefix string)
	printSection = func(s model.Section, prefix string) {
		output.Plain(fmt.Sprintf("%s└️─️─️ %s (lines %d-%d)", prefix, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, prefix+"    ")
		}
	}
	for _, s := range sections {
		printSection(s, "")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return workspace.ToolResult{Output: *output, Data: sections}
}

// 🔖️ToolDefinitionTree MUST complete the operation successfully.
func ToolDefinitionTree(filePath string) workspace.ToolResult {
	return move.ToolDefinitionList(filePath)
}

// 🔖️ToolUpdateMetabolism MUST complete the operation successfully.
func ToolUpdateMetabolism() workspace.ToolResult {
	output := workspace.NewOutput()
	output.Info("\n🔄️ Running update-metabolism via npx tsx...")
	stdout, stderr, exitCode := workspace.ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
	if exitCode != 0 {
		return workspace.ToolErrorMsg(fmt.Sprintf("%s%s", stdout, stderr))
	}
	output.Success(stdout)
	return workspace.ToolResult{Output: *output}
}

// #endregion 📋️Tickets

// #region 🪅️Analyze Command

// #region 🪅️Analyze Command
// Analyze command implementation for policy breach detection.
// 🔬️analyzeCmd holds the data fields for a analyzeCmd record.
var analyzeCmd = &Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for breachs",
	Args:  MaximumNArgs(1),
	RunE: func(cmd *Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		if scope == nil {
			ctx := codebasepkg.NewCodebaseContext()
			ctx.LoadBundles()
			if err := ctx.LoadFiles(); err != nil {
				return err
			}
			if err := ctx.LoadBreachs(); err != nil {
				return err
			}
			if err := ctx.LoadTickets(); err != nil {
				return err
			}
			ctx.LoadPolicies()
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return graphqlpkg.PrintGQL(`
			query Analyze($scope: String) {
				analyze(scope: $scope) {
					breachs {
						id
						summary
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
					}
					metrics { total autofixable byPriority { high medium low } }
				}
			}
		`, variables)
	},
}

// #endregion 🪅️Analyze Command

// #region 📃️Fix Command

// 🔧️Fix command implementation for automatic policy breach repair.
// 🔧️autofixCmd holds the data fields for a autofixCmd record.
var autofixCmd = &Command{
	Use:   "autofix [scope]",
	Short: "Apply autofixes for breachs",
	RunE: func(cmd *Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return graphqlpkg.PrintGQL(`
			mutation Fix($scope: String) {
				fix(scope: $scope) {
					fixed
					remaining
					breachs {
						id
						summary
						scope
						excerpt
						line
					}
				}
			}
		`, variables)
	},
}

// #endregion 📃️Fix Command

// #region 🧬️Missing Utilities

// 📪️CanCloseTicket MUST return a deterministic boolean result.
func CanCloseTicket(ticket *model.Ticket) (bool, []string) {
	var reasons []string
	if ticket == nil {
		reasons = append(reasons, "Ticket data is nil")
		return false, reasons
	}
	return len(reasons) == 0, reasons
}

// 🔶️GuessSectionName MUST complete the operation successfully.
func GuessSectionName(filePath string) string {
	base := filepath.Base(filePath)
	ext := filepath.Ext(base)
	name := strings.TrimSuffix(base, ext)
	name = strings.ReplaceAll(name, "-", " ")
	name = strings.ReplaceAll(name, "_", " ")
	name = strings.ReplaceAll(name, ".", " ")
	words := strings.Fields(name)
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}

// 🐙️GetGitDiffSectionLineMetrics MUST retrieve the requested value or return an error.
// ⚖️GetGitDiffSectionLineMetrics retrieves and returns the git diff section line metrics.
func GetGitDiffSectionLineMetrics(baseCheckpoint, endCheckpoint, filePath string) map[string]model.LineMetrics {
	return nil
}

// 🔖️FlattenSections MUST return a single-level collection with all nested items.
// 🔢️FlattenSections flattens the nested sections into a single level.
func FlattenSections(sections []model.Section) []model.Section {
	var result []model.Section
	var flatten func(secs []model.Section)
	flatten = func(secs []model.Section) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

func computeAffectedSections(filePath string, sections []model.Section, defs []languagespkg.DefinitionRange, addedLineMap map[string][]int, removedLineMap map[string][]int, parentPath string) []model.TicketSection {
	var result []model.TicketSection
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		exclusiveAddedLines := addedLineMap[sectionPath]
		exclusiveRemovedLines := removedLineMap[sectionPath]

		if len(exclusiveAddedLines) > 0 || len(exclusiveRemovedLines) > 0 {
			var affectedDefs []string
			for _, def := range defs {
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						defAddedLines := codebasepkg.ComputeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, model.TicketSection{
				Name:        sectionPath,
				Range:       &model.Range{Start: section.StartLine, End: section.EndLine},
				Definitions: uniqueStrings(affectedDefs),
				Lines:       &model.LineMetrics{Added: len(exclusiveAddedLines), Removed: len(exclusiveRemovedLines)},
			})
		}

		if len(section.Children) > 0 {
			childResults := computeAffectedSections(filePath, section.Children, defs, addedLineMap, removedLineMap, sectionPath)
			result = append(result, childResults...)
		}
	}
	return result
}

// 🔹️setIntersection holds the data fields for a setIntersection record.
func setIntersection(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var intersection []int
	for _, x := range a {
		if m[x] {
			intersection = append(intersection, x)
		}
	}
	return intersection
}

// 🔤️uniqueStrings holds the data fields for a uniqueStrings record.
func uniqueStrings(strs []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	for _, s := range strs {
		if !seen[s] {
			seen[s] = true
			result = append(result, s)
		}
	}
	return result
}

// 🔸️findSectionForLine holds the data fields for a findSectionForLine record.
func findSectionForLine(sections []model.Section, line int) string {
	for _, section := range sections {
		if line >= section.StartLine && line <= section.EndLine {
			if len(section.Children) > 0 {
				childSection := findSectionForLine(section.Children, line)
				if childSection != "" {
					return section.Name + "/" + childSection
				}
			}
			return section.Name
		}
	}
	return ""
}

// #endregion 🧬️Missing Utilities

// #region 🔊️Cli

// 🔹️ensureRepoResolver lazily initializes the repoResolverInstance on first use.
func ensureRepoResolver() {
	if graphqlpkg.RepoResolverInstance == nil {
		graphqlpkg.RepoResolverInstance = graphqlpkg.NewResolver(workspace.RootDir)
	}
}

// #endregion 🔊️Cli

// #region 🖋️Missing Tool Functions

// 🔬️ToolAnalyze MUST complete the operation successfully.
func ToolAnalyze(scopeRaw string, policyIDs []string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventAnalyzeStarting, "repo-cli", eventspkg.FolderPayload{Path: scopeRaw})
	scope := workspace.ParseScope(scopeRaw)
	bundles := codebasepkg.GetTechnologies()
	breachs, err := statutes.CheckPolicies(scope, bundles, policyIDs)
	if err != nil {
		return workspace.ToolResult{Error: err.Error()}
	}

	byPriority := make(map[string]int)
	for range breachs {

	}

	report := languagespkg.AnalyzeReport{
		Second:  time.Now().Format(time.RFC3339),
		Status:  "success",
		Scope:   scopeRaw,
		Breachs: breachs,
		Summary: languagespkg.Summary{
			Total:      len(breachs),
			ByPriority: byPriority,
		},
	}
	output := workspace.NewOutput()
	eventspkg.Emit(eventspkg.EventAnalyzeEnded, "repo-cli", eventspkg.FolderPayload{Path: scopeRaw})
	bytes, _ := json.MarshalIndent(report, "", "  ")
	output.Plain(string(bytes))
	return workspace.ToolResult{Output: *output, Data: report}
}

// 📜️ToolPolicyList MUST complete the operation successfully.
// Uses registered policy metadata only — avoids building the monorepo tree.
func ToolPolicyList() workspace.ToolResult {
	allPolicies := statutes.GetRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(model.RenderEntityMarkdown("policy", pData) + "\n")
	}
	output := workspace.NewOutput()
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: allPolicies}
}

// 🌳️ToolPolicyTree MUST complete the operation successfully.
func ToolPolicyTree() workspace.ToolResult {
	allPolicies := statutes.GetRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(model.RenderEntityMarkdown("policy", pData) + "\n")
		for _, k := range p.AllKinds() {
			meta := k.Info()
			vkData := map[string]interface{}{"id": string(k), "description": meta.Reason}
			sb.WriteString("  " + model.RenderEntityMarkdown("statute", vkData) + "\n")
		}
	}
	treeOutput := workspace.NewOutput()
	treeOutput.Plain(sb.String())
	return workspace.ToolResult{Output: *treeOutput, Data: allPolicies}
}

// ✔️ToolPolicyCheck MUST complete the operation successfully.
func ToolPolicyCheck(policyID, scopeRaw string) workspace.ToolResult {
	return ToolAnalyze(scopeRaw, []string{policyID})
}

// 📋️ToolPolicyBreachList MUST complete the operation successfully.
func ToolPolicyBreachList(policyID string) workspace.ToolResult {

	return ToolAnalyze("compose", []string{policyID})
}

// #endregion 🖋️Missing Tool Functions

// #region 🔮️Benchmark Command

// 💿️benchmarkCmd holds the data fields for a benchmarkCmd record.
var benchmarkCmd = &Command{
	Use:   "benchmark",
	Short: "Run benchmarks for all ecosystems",
	RunE:  runBenchmark,
}

// 🔷️benchmarkDryRun holds the data fields for a benchmarkDryRun record.
var benchmarkDryRun bool

// 🔹️runBenchmark holds the data fields for a runBenchmark record.
func runBenchmark(cmd *Command, args []string) error {
	if benchmarkDryRun {
		return nil
	}
	rootDir := workspace.FindRepoRoot(".")
	results := make([]metricspkg.BenchmarkResult, 0)
	var mu sync.Mutex
	var wg sync.WaitGroup

	tasks := []struct {
		Name    string
		Cmd     string
		Args    []string
		Dir     string
		Enabled bool
	}{
		{
			Name:    "Typescript",
			Cmd:     "npx",
			Args:    []string{"tsx", "compose.benchmark.ts"},
			Dir:     filepath.Join(rootDir, "js", "compose"),
			Enabled: true,
		},
		{
			Name:    "Python",
			Cmd:     "uv",
			Args:    []string{"run", "compose.benchmark.py"},
			Dir:     filepath.Join(rootDir, "py", "compose"),
			Enabled: true,
		},
		{
			Name:    "Go",
			Cmd:     "go",
			Args:    []string{"run", "compose_benchmark.go"},
			Dir:     filepath.Join(rootDir, "go", "compose"),
			Enabled: true,
		},
		{
			Name:    "C#",
			Cmd:     "dotnet",
			Args:    []string{"run", "--technology", "Compose.Benchmark/Compose.Benchmark.csproj", "--configuration", "Release"},
			Dir:     filepath.Join(rootDir, "net"),
			Enabled: true,
		},
		{
			Name:    "Rust",
			Cmd:     "cargo",
			Args:    []string{"run", "--release", "--bin", "compose-benchmark"},
			Dir:     filepath.Join(rootDir, "rs", "compose"),
			Enabled: true,
		},
	}

	fmt.Println("Running benchmarks...")

	for _, task := range tasks {
		if !task.Enabled {
			continue
		}
		wg.Add(1)
		go func(t struct {
			Name    string
			Cmd     string
			Args    []string
			Dir     string
			Enabled bool
		}) {
			defer wg.Done()
			fmt.Printf("Running %s...\n", t.Name)
			if _, err := os.Stat(t.Dir); os.IsNotExist(err) {
				fmt.Printf("Skipping %s: directory %s not found\n", t.Name, t.Dir)
				return
			}

			c := exec.Command(t.Cmd, t.Args...)
			c.Dir = t.Dir
			output, err := c.Output()
			if err != nil {
				if exitErr, ok := err.(*exec.ExitError); ok {
					fmt.Printf("%s failed: %s\n%s\n", t.Name, err, string(exitErr.Stderr))
				} else {
					fmt.Printf("%s failed: %s\n", t.Name, err)
				}
				return
			}

			mu.Lock()
			results = append(results, metricspkg.ParseBenchmarkOutput(t.Name, string(output))...)
			mu.Unlock()
		}(task)
	}

	wg.Wait()
	if len(results) > 0 {
		return writeBenchmarkReport(rootDir, results)
	}

	return nil
}

// ✏️writeBenchmarkReport writes the 📊️metrics benchmark table to the repository metrics directory.
func writeBenchmarkReport(rootDir string, results []metricspkg.BenchmarkResult) error {
	_ = rootDir
	reportFile := filepath.Join(workspace.GetRepoMetaDir(), "📊️metrics", "benchmark.csv")
	if err := os.MkdirAll(filepath.Dir(reportFile), 0755); err != nil {
		return err
	}
	if err := os.WriteFile(reportFile, []byte(metricspkg.BenchmarkCSV(results)), 0644); err != nil {
		return err
	}
	fmt.Printf("Benchmark report written to %s\n", reportFile)
	return nil
}

// #endregion 🔮️Benchmark Command

// #region 🦀️Hooks

// 🆕️hookCommand creates the `hook <event> <client>` command command.
func hookCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "hook <event> <client>",
		Short: "Run a lifecycle hook (git or agent)",
		Long: `Run a lifecycle hook for a given event and client.
Accepts neutral repo events or native client events (inlet adapter resolves to neutral).

📦️ Version hooks:
  version.checkpoint.starting  Run before a version checkpoint
  version.checkpoint.ended     Run after a version checkpoint (e.g. post-checkpoint)
  version.checkin.starting     Run before a version checkin (e.g. fast-forward to main)
  version.checkin.ended        Run after a version checkin
  version.checkout.starting    Run before a version checkout (e.g. squash merge)
  version.checkout.ended       Run after a version checkout

🤖️ Neutral agent hooks:
  agent.started                Agent session started
  agent.ended                  Agent session stopped
  agent.prompt.submitting      User submitting a prompt
  agent.compacting             Context compaction event
  agent.tool.starting          Tool starting (generic, excludes plan, search, code, test, build, terminal)
  agent.tool.ended             Tool completed (generic, excludes plan, search, code, test, build, terminal)
  agent.tool.plan.updating.starting  Plan/task list updating start
  agent.tool.plan.updating.ended     Plan/task list updating end
  agent.file.read.starting     Code search/read starting
  agent.file.read.ended        Code search/read ended
  agent.tool.code.edit.starting Code edit starting
  agent.tool.code.edit.ended   Code edit completed
  agent.tool.test.starting     Test starting
  agent.tool.test.ended        Test completed
  agent.tool.build.starting    Build starting
  agent.tool.build.ended       Build completed
  agent.tool.terminal.starting Terminal command starting (supports blocking)
  agent.tool.terminal.ended    Terminal command completed
  agent.thinking.starting      Agent thinking started
  agent.thinking.ended         Agent thinking completed (cursor: afterAgentThought)

🏢️ Native client events (resolved by inlet adapter):
  copilot-chat:    SessionStart, Stop, SubagentStart, SubagentStop, UserPromptSubmit, PreCompact, PreToolUse, PostToolUse
  cursor-chat:     sessionStart, sessionEnd, stop, subagentStart, subagentStop, beforeSubmitPrompt, preCompact, preToolUse, postToolUse, beforeReadFile, afterFileEdit, beforeShellExecution, afterShellExecution, beforeMCPExecution, afterMCPExecution
  windsurf-chat:   pre_user_prompt, post_cascade_response, post_setup_worktree, pre_mcp_tool_use, post_mcp_tool_use, pre_read_code, post_read_code, pre_write_code, post_write_code, pre_run_command, post_run_command
  claude-code/droid/codex/antigravity: SessionStart, SessionEnd, SubagentStart, SubagentStop, Stop, UserPromptSubmit, PreCompact, PreToolUse, PostToolUse, PostToolUseFailure, TaskCompleted, Notification
  kiro-cli:        agentSpawn, userPromptSubmit, preToolUse, postToolUse, stop`,
		Args: RangeArgs(1, 2),
		RunE: func(cmd *Command, args []string) error {
			eventStr := args[0]
			client := ""
			if len(args) > 1 {
				client = args[1]
			}
			toolName, _ := cmd.Flags().GetString("tool-name")
			toolArgs, _ := cmd.Flags().GetString("tool-args")
			filePath, _ := cmd.Flags().GetString("file")
			parentInfo, _ := cmd.Flags().GetString("parent")
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, _ := os.Getwd()
				repoRoot = workspace.FindRepoRoot(cwd)
			}
			var input json.RawMessage
			stdinReader := cmd.InOrStdin()
			if stdinReader == os.Stdin {
				if stat, err := os.Stdin.Stat(); err == nil && (stat.Mode()&os.ModeCharDevice) == 0 {
					if data, err := io.ReadAll(os.Stdin); err == nil && len(data) > 0 {
						input = json.RawMessage(data)
					}
				}
			} else {
				if data, err := io.ReadAll(stdinReader); err == nil && len(data) > 0 {
					input = json.RawMessage(data)
				}
			}
			ctx := cmd.Context()
			if config.Timeout > 0 {
				ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
				defer cancel()
				ctx = ctxWithTimeout
			}
			return hooks.RunHookExecutionCtx(ctx, client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, config.IsJSON(), cmd.OutOrStdout(), cmd.ErrOrStderr())
		},
	}
	cmd.Flags().String("tool-name", "", "Tool name for tool-related events")
	cmd.Flags().String("tool-args", "", "Tool arguments for tool-related events")
	cmd.Flags().String("file", "", "File path for code events")
	cmd.Flags().String("parent", "", "Parent agent info for agent events")
	return cmd
}

// #endregion 🦀️Hooks

// #region 🔷️Configure

// 🆕️configureCommand creates the `configure` command command.
func configureCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "configure",
		Short: "Remove blocking git hooks; repo config files are edited manually",
		RunE: func(cmd *Command, args []string) error {
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, err := os.Getwd()
				if err != nil {
					return err
				}
				repoRoot = workspace.FindRepoRoot(cwd)
			}
			removed, err := hooks.RemoveGitHooks(repoRoot)
			if err != nil {
				return err
			}
			if err := hooks.InstallMicroCommitHooks(repoRoot); err != nil {
				return err
			}
			fmt.Fprintln(cmd.OutOrStdout(), "repo config generation is disabled; edit checked-in config files manually")
			if len(removed) == 0 {
				fmt.Fprintln(cmd.OutOrStdout(), "git hooks: none to remove; micro-commit hooks installed")
				return nil
			}
			fmt.Fprintf(cmd.OutOrStdout(), "git hooks removed: %s; micro-commit hooks installed\n", strings.Join(removed, ", "))
			return nil
		},
	}
	return cmd
}

// #endregion 🔷️Configure

// #region 🔖️MicroCommit

func microCommitCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "micro-commit [subcommand] [args...]",
		Short: "Micro-commit workflow (reset, prepare-commit-msg, stage, diff, prepare)",
		RunE: func(cmd *Command, args []string) error {
			repoRoot := config.Repo
			if repoRoot == "" {
				cwd, err := os.Getwd()
				if err != nil {
					return err
				}
				repoRoot = workspace.FindRepoRoot(cwd)
			}
			if repoRoot == "" {
				return fmt.Errorf("repository root not found")
			}
			ctx := cmd.Context()
			if config.Timeout > 0 {
				ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
				defer cancel()
				ctx = ctxWithTimeout
			}
			return hooks.RunMicroCommitScript(ctx, repoRoot, args)
		},
	}
}

// #endregion 🔖️MicroCommit

// #region 🔊️Cli

// #endregion 🔖️MicroCommit
// Update command implementation for dependency updates.
// ✏️updateCmd holds the data fields for a updateCmd record.
var updateCmd = &Command{
	Use:   "update [target]",
	Short: "Update dependencies (npm, python, rust, go, dotnet)",
	RunE:  runUpdate,
}

// 🔁️updateDryRun holds the data fields for a updateDryRun record.
var updateDryRun bool

// 🔸️updateApply holds the data fields for a updateApply record.
var updateApply bool

// 🔺️init holds the data fields for a init record.
func init() {
	updateCmd.Flags().BoolVar(&updateDryRun, "dry-run", false, "Show what would be updated without making changes")
	updateCmd.Flags().BoolVar(&updateApply, "apply", false, "Apply updates (default is dry-run)")
	benchmarkCmd.Flags().BoolVar(&benchmarkDryRun, "dry-run", false, "Initialize and exit without running benchmarks")
}

// 🔻️DependabotConfig holds the data fields for a dependabot config record.
type DependabotConfig struct {
	Version int `yaml:"version"`
	Updates []struct {
		PackageEcosystem string `yaml:"package-ecosystem"`
		Directory        string `yaml:"directory"`
		Ignore           []struct {
			DependencyName string   `yaml:"dependency-name"`
			Versions       []string `yaml:"versions"`
		} `yaml:"ignore"`
	} `yaml:"updates"`
	XComposeConfig struct {
		PreserveLocalVersions struct {
			Npm struct {
				Pattern string `yaml:"pattern"`
			} `yaml:"npm"`
		} `yaml:"preserveLocalVersions"`
	} `yaml:"x-compose-config"`
}

// ⬛️UpdateConfig holds the data fields for a update config record.
type UpdateConfig struct {
	Exclude               map[string][]string
	Constraints           map[string][]Constraint
	PreserveLocalVersions struct {
		Npm struct {
			Pattern string
		}
	}
	Paths struct {
		Npm    []string
		Python []string
		Rust   []string
		Go     []string
		Dotnet []string
	}
}

// 🔒️Constraint holds the data fields for a constraint record.
type Constraint struct {
	Dependency string
	MaxMajor   int
}

// ⬜️runUpdate holds the data fields for a runUpdate record.
func runUpdate(cmd *Command, args []string) error {
	target := "all"
	if len(args) > 0 {
		target = args[0]
	}
	if !updateApply {
		updateDryRun = true
	}
	if updateDryRun {
		fmt.Println("=== Dependency Update Script ===")
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
		fmt.Printf("Target: %s\n", target)
		fmt.Println("\n=== Update Complete ===")
		return nil
	}

	rootDir := workspace.FindRepoRoot(".")
	config, err := loadUpdateConfig(rootDir)
	if err != nil {
		return err
	}

	fmt.Println("=== Dependency Update Script ===")
	if updateDryRun {
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
	}
	fmt.Printf("Target: %s\n", target)

	var wg sync.WaitGroup

	if target == "all" || target == "npm" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateNpm(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "python" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updatePython(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "rust" {
		wg.Add(1)
		go func() {
			updateRust(rootDir, config, updateDryRun)
		}()
	}
	if target == "all" || target == "go" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateGo(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "dotnet" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateDotNet(rootDir, config, updateDryRun)
		}()
	}

	wg.Wait()
	fmt.Println("\n=== Update Complete ===")
	return nil
}

// 🟥️loadUpdateConfig holds the data fields for a loadUpdateConfig record.
func loadUpdateConfig(rootDir string) (*UpdateConfig, error) {
	dependabotPath := filepath.Join(rootDir, ".github", "dependabot.yml")
	data, err := os.ReadFile(dependabotPath)
	if err != nil {
		return nil, fmt.Errorf("dependabot.yml not found: %w", err)
	}

	var dependabot DependabotConfig
	if err := yamlpkg.Unmarshal(data, &dependabot); err != nil {
		return nil, err
	}

	config := &UpdateConfig{
		Exclude:     make(map[string][]string),
		Constraints: make(map[string][]Constraint),
	}
	config.Paths.Npm = []string{}
	config.Paths.Python = []string{}
	config.Paths.Rust = []string{}
	config.Paths.Go = []string{}
	config.Paths.Dotnet = []string{}

	config.PreserveLocalVersions.Npm.Pattern = "*"
	if dependabot.XComposeConfig.PreserveLocalVersions.Npm.Pattern != "" {
		config.PreserveLocalVersions.Npm.Pattern = dependabot.XComposeConfig.PreserveLocalVersions.Npm.Pattern
	}

	for _, update := range dependabot.Updates {
		dir := strings.TrimPrefix(update.Directory, "/")
		ecosystem := update.PackageEcosystem

		switch ecosystem {
		case "npm":
			config.Paths.Npm = append(config.Paths.Npm, dir)
		case "uv":
			config.Paths.Python = append(config.Paths.Python, dir)
		case "cargo":
			config.Paths.Rust = append(config.Paths.Rust, dir)
		case "gomod":
			config.Paths.Go = append(config.Paths.Go, dir)
		case "nuget":
			files := findCsprojFiles(rootDir, dir)
			for _, file := range files {
				config.Paths.Dotnet = append(config.Paths.Dotnet, file)
				if len(update.Ignore) > 0 {
					for _, ignore := range update.Ignore {
						if len(ignore.Versions) > 0 {
							for _, v := range ignore.Versions {
								re := regexp.MustCompile(`>=\s*(\d+)\.`)
								match := re.FindStringSubmatch(v)
								if len(match) > 1 {
									maxMajor, _ := strconv.Atoi(match[1])
									maxMajor = maxMajor - 1
									config.Constraints[file] = append(config.Constraints[file], Constraint{
										Dependency: ignore.DependencyName,
										MaxMajor:   maxMajor,
									})
								}
							}
						} else {
							config.Exclude[file] = append(config.Exclude[file], ignore.DependencyName)
						}
					}
				}
			}
		}
	}
	return config, nil
}

// 📄️findCsprojFiles holds the data fields for a findCsprojFiles record.
func findCsprojFiles(rootDir, dir string) []string {
	fullDir := filepath.Join(rootDir, dir)
	var files []string
	entries, err := os.ReadDir(fullDir)
	if err != nil {
		return files
	}
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".csproj") {
			files = append(files, filepath.Join(dir, entry.Name()))
		}
	}
	return files
}

// 🟧️runCommand holds the data fields for a runCommand record.
func runCommand(dir, name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	fmt.Printf("  Running: %s %s in %s\n", name, strings.Join(args, " "), dir)
	return cmd.Run()
}

func runCommandQuiet(dir, name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	output, err := cmd.Output()
	return string(output), err
}

// 🟨️updateNpm holds the data fields for a updateNpm record.
func updateNpm(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[NPM] Updating npm packages...")
	if dryRun {
		fmt.Println("  [DRY RUN] Would run: npm update -S")
		return
	}

	if err := runCommand(rootDir, "npm", "update", "-S"); err != nil {
		fmt.Printf("Error updating npm: %v\n", err)
	}
	fmt.Println("[NPM] Done.")
}

func updatePython(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Python] Updating Python packages...")
	for _, pyPath := range config.Paths.Python {
		fullPath := filepath.Join(rootDir, pyPath)
		tomlPath := filepath.Join(fullPath, "pyproject.toml")
		if _, err := os.Stat(tomlPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", pyPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would update pyproject.toml and run uv lock")
			continue
		}

		if err := runCommand(fullPath, "uv", "lock", "--upgrade"); err != nil {
			fmt.Printf("Error updating python in %s: %v\n", pyPath, err)
		}
	}
	fmt.Println("[Python] Done.")
}

// 🟩️updateRust holds the data fields for a updateRust record.
func updateRust(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Rust] Updating Rust packages...")
	for _, rsPath := range config.Paths.Rust {
		fullPath := filepath.Join(rootDir, rsPath)
		if _, err := os.Stat(filepath.Join(fullPath, "Cargo.toml")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", rsPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run cargo update")
			continue
		}

		if err := runCommand(fullPath, "cargo", "update"); err != nil {
			fmt.Printf("Error updating rust in %s: %v\n", rsPath, err)
		}
	}
	fmt.Println("[Rust] Done.")
}

// 🟦️updateGo holds the data fields for a updateGo record.
func updateGo(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Go] Updating Go modules...")
	for _, goPath := range config.Paths.Go {
		fullPath := filepath.Join(rootDir, goPath)
		if _, err := os.Stat(filepath.Join(fullPath, "go.mod")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", goPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run: go get -u ./... && go mod tidy")
			continue
		}

		runCommand(fullPath, "go", "get", "-u", "./...")
		runCommand(fullPath, "go", "mod", "tidy")
	}
	fmt.Println("[Go] Done.")
}

func updateDotNet(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[.NET] Updating .NET packages...")
	for _, csprojPath := range config.Paths.Dotnet {
		fullPath := filepath.Join(rootDir, csprojPath)
		if _, err := os.Stat(fullPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", csprojPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would check for package updates")
			continue
		}

		output, err := runCommandQuiet(filepath.Dir(fullPath), "dotnet", "list", fullPath, "package", "--outdated")
		if err != nil {
			continue
		}

		lines := strings.Split(output, "\n")
		for _, line := range lines {
			if strings.Contains(line, ">") {
				parts := strings.Fields(line)
				if len(parts) >= 5 {
					name := parts[1]
					latest := parts[4]

					excluded := false
					if ex, ok := config.Exclude[csprojPath]; ok {
						for _, e := range ex {
							if e == name {
								excluded = true
								break
							}
						}
					}
					if excluded {
						continue
					}

					fmt.Printf("    Updating %s to %s\n", name, latest)
					runCommand(filepath.Dir(fullPath), "dotnet", "add", fullPath, "package", name, "--version", latest)
				}
			}
		}
	}
	fmt.Println("[.NET] Done.")
}

// #endregion 🔊️Cli

// #region 💾️Missing Utility Functions

// 🧲️extractSecondFromInput extracts second from input.
func extractSecondFromInput(input string) int {
	// This is a placeholder implementation
	return 0
}

// #endregion 💾️Missing Utility Functions

// #region 🎮️Command Framework

type PositionalValidator func(*Command, []string) error

type Command struct {
	Use                string
	Short              string
	Long               string
	Example            string
	Version            string
	Aliases            []string
	Args               PositionalValidator
	Run                func(*Command, []string)
	RunE               func(*Command, []string) error
	SilenceUsage       bool
	SilenceErrors      bool
	DisableFlagParsing bool
	PersistentPreRunE  func(*Command, []string) error
	PreRunE            func(*Command, []string) error
	parent             *Command
	children           []*Command
	flags              *FlagSet
	persistentFlags    *FlagSet
	args               []string
	argsSet            bool
	out                io.Writer
	err                io.Writer
	in                 io.Reader
	ctx                context.Context
}

type Flag struct {
	Name         string
	Shorthand    string
	Usage        string
	Changed      bool
	Value        Value
	value        any
	defaultValue any
	bound        any
}

type Value interface{ String() string }

type stringValue string

func (value stringValue) String() string { return string(value) }

type FlagSet struct {
	flags map[string]*Flag
	short map[string]*Flag
	owner *Command
}

func newFlagSet() *FlagSet { return &FlagSet{flags: map[string]*Flag{}, short: map[string]*Flag{}} }

func (set *FlagSet) add(name, shorthand, usage string, value, bound any) *Flag {
	flag := &Flag{Name: name, Shorthand: shorthand, Usage: usage, value: value, defaultValue: value, bound: bound}
	flag.sync()
	set.flags[name] = flag
	if shorthand != "" {
		set.short[shorthand] = flag
	}
	return flag
}

func (flag *Flag) sync() {
	switch value := flag.value.(type) {
	case string:
		flag.Value = stringValue(value)
		if target, ok := flag.bound.(*string); ok {
			*target = value
		}
	case bool:
		flag.Value = stringValue(strconv.FormatBool(value))
		if target, ok := flag.bound.(*bool); ok {
			*target = value
		}
	case int:
		flag.Value = stringValue(strconv.Itoa(value))
		if target, ok := flag.bound.(*int); ok {
			*target = value
		}
	case []string:
		flag.Value = stringValue(strings.Join(value, ","))
		if target, ok := flag.bound.(*[]string); ok {
			*target = append((*target)[:0], value...)
		}
	case []int:
		parts := make([]string, len(value))
		for index := range value {
			parts[index] = strconv.Itoa(value[index])
		}
		flag.Value = stringValue(strings.Join(parts, ","))
		if target, ok := flag.bound.(*[]int); ok {
			*target = append((*target)[:0], value...)
		}
	case time.Duration:
		flag.Value = stringValue(value.String())
		if target, ok := flag.bound.(*time.Duration); ok {
			*target = value
		}
	}
}

func (set *FlagSet) reset() {
	for _, flag := range set.flags {
		flag.Changed = false
		flag.value = flag.defaultValue
		flag.sync()
	}
}

func (set *FlagSet) String(name, value, usage string) *string {
	target := new(string)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) StringVar(target *string, name, value, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) StringVarP(target *string, name, shorthand, value, usage string) {
	set.add(name, shorthand, usage, value, target)
}

func (set *FlagSet) Bool(name string, value bool, usage string) *bool {
	target := new(bool)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) BoolP(name, shorthand string, value bool, usage string) *bool {
	target := new(bool)
	set.add(name, shorthand, usage, value, target)
	return target
}

func (set *FlagSet) BoolVar(target *bool, name string, value bool, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) Int(name string, value int, usage string) *int {
	target := new(int)
	set.add(name, "", usage, value, target)
	return target
}

func (set *FlagSet) IntSlice(name string, value []int, usage string) *[]int {
	target := new([]int)
	set.add(name, "", usage, append([]int(nil), value...), target)
	return target
}

func (set *FlagSet) DurationVar(target *time.Duration, name string, value time.Duration, usage string) {
	set.add(name, "", usage, value, target)
}

func (set *FlagSet) StringSlice(name string, value []string, usage string) *[]string {
	target := new([]string)
	set.add(name, "", usage, append([]string(nil), value...), target)
	return target
}

// 🔗️resolve finds a flag by name: the set's own flags first, then the persistent
// flags inherited from the owning command's ancestors (mirroring [Command.Flags]).
func (set *FlagSet) resolve(name string) *Flag {
	if flag := set.flags[name]; flag != nil {
		return flag
	}
	if set.owner == nil {
		return nil
	}
	return set.owner.lookupFlag(name, false)
}

func (set *FlagSet) Lookup(name string) *Flag { return set.resolve(name) }

func (set *FlagSet) Changed(name string) bool {
	flag := set.resolve(name)
	return flag != nil && flag.Changed
}

func (set *FlagSet) Set(name, value string) error {
	flag, err := set.get(name)
	if err != nil {
		return err
	}
	return set.set(flag, value)
}

func (set *FlagSet) MarkHidden(name string) error {
	_, err := set.get(name)
	return err
}

func (set *FlagSet) GetString(name string) (string, error) {
	flag, err := set.get(name)
	if err != nil {
		return "", err
	}
	value, ok := flag.value.(string)
	if !ok {
		return "", fmt.Errorf("flag --%s is not a string", name)
	}
	return value, nil
}

func (set *FlagSet) GetBool(name string) (bool, error) {
	flag, err := set.get(name)
	if err != nil {
		return false, err
	}
	value, ok := flag.value.(bool)
	if !ok {
		return false, fmt.Errorf("flag --%s is not a boolean", name)
	}
	return value, nil
}

func (set *FlagSet) GetInt(name string) (int, error) {
	flag, err := set.get(name)
	if err != nil {
		return 0, err
	}
	value, ok := flag.value.(int)
	if !ok {
		return 0, fmt.Errorf("flag --%s is not an integer", name)
	}
	return value, nil
}

func (set *FlagSet) GetStringSlice(name string) ([]string, error) {
	flag, err := set.get(name)
	if err != nil {
		return nil, err
	}
	value, ok := flag.value.([]string)
	if !ok {
		return nil, fmt.Errorf("flag --%s is not a string list", name)
	}
	return append([]string(nil), value...), nil
}

func (set *FlagSet) GetIntSlice(name string) ([]int, error) {
	flag, err := set.get(name)
	if err != nil {
		return nil, err
	}
	value, ok := flag.value.([]int)
	if !ok {
		return nil, fmt.Errorf("flag --%s is not an integer list", name)
	}
	return append([]int(nil), value...), nil
}

func (set *FlagSet) get(name string) (*Flag, error) {
	flag := set.resolve(name)
	if flag == nil {
		return nil, fmt.Errorf("unknown flag: --%s", name)
	}
	return flag, nil
}

func (set *FlagSet) set(flag *Flag, raw string) error {
	switch flag.value.(type) {
	case string:
		flag.value = raw
	case bool:
		value, err := strconv.ParseBool(raw)
		if err != nil {
			return fmt.Errorf("invalid boolean for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	case int:
		value, err := strconv.Atoi(raw)
		if err != nil {
			return fmt.Errorf("invalid integer for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	case []string:
		if raw == "" {
			flag.value = []string{}
		} else {
			flag.value = strings.Split(raw, ",")
		}
	case []int:
		var values []int
		if raw != "" {
			for _, part := range strings.Split(raw, ",") {
				value, err := strconv.Atoi(part)
				if err != nil {
					return fmt.Errorf("invalid integer list for --%s: %s", flag.Name, raw)
				}
				values = append(values, value)
			}
		}
		flag.value = values
	case time.Duration:
		value, err := time.ParseDuration(raw)
		if err != nil {
			return fmt.Errorf("invalid duration for --%s: %s", flag.Name, raw)
		}
		flag.value = value
	default:
		return fmt.Errorf("unsupported flag --%s", flag.Name)
	}
	flag.Changed = true
	flag.sync()
	return nil
}

func (command *Command) Flags() *FlagSet {
	if command.flags == nil {
		command.flags = newFlagSet()
	}
	command.flags.owner = command
	return command.flags
}

func (command *Command) PersistentFlags() *FlagSet {
	if command.persistentFlags == nil {
		command.persistentFlags = newFlagSet()
	}
	return command.persistentFlags
}

func (command *Command) AddCommand(children ...*Command) {
	for _, child := range children {
		child.parent = command
		command.children = append(command.children, child)
	}
}

func (command *Command) Commands() []*Command { return append([]*Command(nil), command.children...) }

func (command *Command) Parent() *Command { return command.parent }

func (command *Command) Root() *Command {
	root := command
	for root.parent != nil {
		root = root.parent
	}
	return root
}

func (command *Command) Name() string {
	name, _, _ := strings.Cut(command.Use, " ")
	return name
}

func (command *Command) Context() context.Context {
	if command.ctx != nil {
		return command.ctx
	}
	if command.parent != nil {
		return command.parent.Context()
	}
	return context.Background()
}

func (command *Command) SetContext(ctx context.Context) { command.ctx = ctx }

func (command *Command) SetArgs(args []string) {
	command.args = append([]string(nil), args...)
	command.argsSet = true
}

func (command *Command) SetOut(writer io.Writer) { command.Root().out = writer }

func (command *Command) SetErr(writer io.Writer) { command.Root().err = writer }

func (command *Command) SetIn(reader io.Reader) { command.Root().in = reader }

func (command *Command) OutOrStdout() io.Writer {
	if command.Root().out != nil {
		return command.Root().out
	}
	return os.Stdout
}

func (command *Command) ErrOrStderr() io.Writer {
	if command.Root().err != nil {
		return command.Root().err
	}
	return os.Stderr
}

func (command *Command) InOrStdin() io.Reader {
	if command.Root().in != nil {
		return command.Root().in
	}
	return os.Stdin
}

func (command *Command) Println(values ...interface{}) {
	fmt.Fprintln(command.OutOrStdout(), values...)
}

func (command *Command) Help() error {
	writer := command.OutOrStdout()
	fmt.Fprintf(writer, "%s\n\nUsage:\n  %s", command.Short, command.Use)
	if len(command.children) > 0 {
		fmt.Fprintln(writer, "\n\nCommands:")
		for _, child := range command.children {
			fmt.Fprintf(writer, "  %-20s %s\n", child.Name(), child.Short)
		}
	}
	return nil
}

var errHelp = errors.New("help requested")

func (command *Command) Execute() error {
	_, err := command.execute(command.dispatchArgs())
	return err
}

func (command *Command) ExecuteC() (*Command, error) { return command.execute(command.dispatchArgs()) }

// 🎛️dispatchArgs returns the argument vector to dispatch: the slice installed by
// [Command.SetArgs] when one was installed, otherwise the process arguments.
func (command *Command) dispatchArgs() []string {
	if command.argsSet {
		return command.args
	}
	return os.Args[1:]
}

func (command *Command) execute(args []string) (*Command, error) {
	selected, positional, err := command.selectAndParse(args)
	if err != nil {
		if errors.Is(err, errHelp) {
			return selected, nil
		}
		return selected, err
	}
	if selected.Args != nil {
		if err := selected.Args(selected, positional); err != nil {
			return selected, err
		}
	}
	for _, ancestor := range selected.ancestry() {
		if ancestor.PersistentPreRunE != nil {
			if err := ancestor.PersistentPreRunE(selected, positional); err != nil {
				return selected, err
			}
		}
	}
	if selected.PreRunE != nil {
		if err := selected.PreRunE(selected, positional); err != nil {
			return selected, err
		}
	}
	if selected.RunE != nil {
		return selected, selected.RunE(selected, positional)
	}
	if selected.Run != nil {
		selected.Run(selected, positional)
		return selected, nil
	}
	if len(selected.children) > 0 {
		if len(positional) > 0 {
			return selected, fmt.Errorf("unknown command %q for %q", positional[0], selected.Name())
		}
		return selected, selected.Help()
	}
	return selected, nil
}

func (command *Command) selectAndParse(args []string) (*Command, []string, error) {
	selected := command
	positional := make([]string, 0, len(args))
	for index := 0; index < len(args); index++ {
		arg := args[index]
		if selected.DisableFlagParsing {
			return selected, append(positional, args[index:]...), nil
		}
		if arg == "--help" || arg == "-h" {
			if err := selected.Help(); err != nil {
				return selected, nil, err
			}
			return selected, nil, errHelp
		}
		if arg == "--" {
			positional = append(positional, args[index+1:]...)
			break
		}
		if strings.HasPrefix(arg, "-") {
			consumed, err := selected.parseFlag(args[index:])
			if err != nil {
				return selected, nil, err
			}
			index += consumed - 1
			continue
		}
		if len(positional) == 0 {
			if child := selected.child(arg); child != nil {
				selected = child
				continue
			}
		}
		positional = append(positional, arg)
	}
	return selected, positional, nil
}

func (command *Command) parseFlag(args []string) (int, error) {
	token := args[0]
	name := strings.TrimPrefix(token, "--")
	short := false
	if name == token {
		name = strings.TrimPrefix(token, "-")
		short = true
	}
	inline := ""
	if key, value, found := strings.Cut(name, "="); found {
		name, inline = key, value
	}
	flag := command.lookupFlag(name, short)
	if flag == nil {
		return 0, fmt.Errorf("unknown flag: %s", token)
	}
	if _, ok := flag.value.(bool); ok && inline == "" {
		return 1, command.ownerSet(flag, "true")
	}
	if inline != "" {
		return 1, command.ownerSet(flag, inline)
	}
	if len(args) < 2 {
		return 0, fmt.Errorf("flag needs an argument: %s", token)
	}
	return 2, command.ownerSet(flag, args[1])
}

func (command *Command) ownerSet(flag *Flag, value string) error {
	for current := command; current != nil; current = current.parent {
		if current.flags != nil && current.flags.flags[flag.Name] == flag {
			return current.flags.set(flag, value)
		}
		if current.persistentFlags != nil && current.persistentFlags.flags[flag.Name] == flag {
			return current.persistentFlags.set(flag, value)
		}
	}
	return errors.New("flag owner missing")
}

func (command *Command) lookupFlag(name string, short bool) *Flag {
	if command.flags != nil {
		if short {
			if flag := command.flags.short[name]; flag != nil {
				return flag
			}
		}
		if flag := command.flags.flags[name]; flag != nil {
			return flag
		}
	}
	for current := command; current != nil; current = current.parent {
		if current.persistentFlags == nil {
			continue
		}
		if short {
			if flag := current.persistentFlags.short[name]; flag != nil {
				return flag
			}
		}
		if flag := current.persistentFlags.flags[name]; flag != nil {
			return flag
		}
	}
	return nil
}

func (command *Command) child(name string) *Command {
	for _, child := range command.children {
		if child.Name() == name {
			return child
		}
		for _, alias := range child.Aliases {
			if alias == name {
				return child
			}
		}
	}
	return nil
}

func (command *Command) ancestry() []*Command {
	var reversed []*Command
	for current := command; current != nil; current = current.parent {
		reversed = append(reversed, current)
	}
	ancestors := make([]*Command, len(reversed))
	for index := range reversed {
		ancestors[len(reversed)-index-1] = reversed[index]
	}
	return ancestors
}

func (command *Command) resetFlagsRecursive() {
	if command.flags != nil {
		command.flags.reset()
	}
	if command.persistentFlags != nil {
		command.persistentFlags.reset()
	}
	for _, child := range command.children {
		child.resetFlagsRecursive()
	}
}

func NoArgs(_ *Command, args []string) error {
	if len(args) != 0 {
		return fmt.Errorf("accepts 0 arg(s), received %d", len(args))
	}
	return nil
}

func ExactArgs(count int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) != count {
			return fmt.Errorf("accepts %d arg(s), received %d", count, len(args))
		}
		return nil
	}
}

func MaximumNArgs(count int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) > count {
			return fmt.Errorf("accepts at most %d arg(s), received %d", count, len(args))
		}
		return nil
	}
}

func RangeArgs(minimum, maximum int) PositionalValidator {
	return func(_ *Command, args []string) error {
		if len(args) < minimum || len(args) > maximum {
			return fmt.Errorf("accepts between %d and %d arg(s), received %d", minimum, maximum, len(args))
		}
		return nil
	}
}

func ArbitraryArgs(_ *Command, _ []string) error { return nil }

// #endregion 🎮️Command Framework

// #endregion 🚚️Split

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
func McpVerbConversation(profile string, requests []string) ([]string, error) {
	kind := providers.McpClientGeneric
	if profile != "" {
		parsed, err := providers.ParseMcpClientKind(profile)
		if err != nil {
			return nil, err
		}
		kind = parsed
	}
	var out strings.Builder
	if err := Serve(CreateMcpServer(kind, DefaultCommandTimeout), strings.NewReader(strings.Join(requests, "\n")+"\n"), &out); err != nil {
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

// #region 🌳️TreeSource

// 🗂️FsTreeSource is the treepkg.TreeSource the `tree` verb projects from: every record already read
// out of the repository through the GraphQL repo context, the twin of the Rust `FsTreeSource`.
type FsTreeSource struct {
	TechnologyRecords  []treepkg.TechnologyRecord
	FolderRecords      []treepkg.FolderRecord
	FileRecords        []treepkg.FileRecord
	GoalSeeds          []treepkg.GoalRecord
	TicketSeeds        []treepkg.TicketRecord
	DraftRecords       []treepkg.DraftRecord
	PolicyRecords      []treepkg.PolicyRecord
	ContributorRecords []treepkg.ContributorRecord
	CheckpointRecords  []treepkg.CheckpointRecord
	Root               string
	IncludeSections    bool
}

// 🧹️NormalizeTreeScope MUST return the scope as the record paths spell it.
// 🧹️NormalizeTreeScope returns one scope with forward slashes, no leading or trailing separator, and
// the empty string for the repository root.
func NormalizeTreeScope(scope string) string {
	scope = strings.Trim(strings.TrimSpace(strings.ReplaceAll(scope, "\\", "/")), "/")
	if scope == "." {
		return ""
	}
	return scope
}

// 🎯️PathWithinScope MUST return true only when the path is the scope itself or lies below it.
// 🎯️PathWithinScope reports whether a record path belongs to a scope.
func PathWithinScope(path string, scope string) bool {
	return scope == "" || path == scope || strings.HasPrefix(path, scope+"/")
}

// 🔤️derefString returns the pointed-to string, or the empty string when there is none.
func derefString(value *string) string {
	if value == nil {
		return ""
	}
	return *value
}

// 🎯️LoadFsTreeSource MUST read every record the monorepo tree projects.
// 🎯️LoadFsTreeSource reads the records of one repository-relative subtree: every folder and file at
// or below the scope, every bundle rooted there and the technologies that still own one. A scope
// names a place in the codebase, so the aggregates that have no place — goals, tickets, drafts,
// policies, contributors, checkpoints — are not part of a scoped projection. An empty scope is the
// whole repository and loads all of them.
func LoadFsTreeSource(ctx model.RepoContext, includeSections bool, scope string) *FsTreeSource {
	scope = NormalizeTreeScope(scope)
	source := &FsTreeSource{Root: ctx.GetRootDir(), IncludeSections: includeSections}
	bundles := ctx.GetBundles()
	for _, technology := range ctx.GetTechnologies() {
		record := treepkg.TechnologyRecord{
			ID:      technology.GetID(),
			Name:    technology.Name,
			URI:     "repo://technology/" + technology.GetID(),
			Kind:    string(technology.Kind),
			Emoji:   technology.Emoji,
			Bundles: []treepkg.BundleRecord{},
		}
		for _, bundle := range bundles {
			if bundle.TechnologyName != technology.Name || !PathWithinScope(bundle.Root, scope) {
				continue
			}
			record.Bundles = append(record.Bundles, treepkg.BundleRecord{
				ID:         bundle.GetID(),
				Name:       bundle.Name,
				URI:        "repo://bundle/" + bundle.GetID(),
				Kind:       string(bundle.Kind),
				Emoji:      bundle.Emoji,
				Root:       bundle.Root,
				SourceRoot: bundle.SourceRoot,
			})
		}
		if scope != "" && len(record.Bundles) == 0 {
			continue
		}
		source.TechnologyRecords = append(source.TechnologyRecords, record)
	}
	for _, folder := range ctx.GetFolders() {
		if !PathWithinScope(folder.Path, scope) {
			continue
		}
		source.FolderRecords = append(source.FolderRecords, treepkg.FolderRecord{ID: folder.ID, Path: folder.Path, Name: folder.Name, URI: folder.URI, Kind: string(folder.Kind), ParentID: derefString(folder.ParentID)})
	}
	for _, file := range ctx.GetFiles() {
		if !PathWithinScope(file.Path, scope) {
			continue
		}
		source.FileRecords = append(source.FileRecords, treepkg.FileRecord{ID: file.ID, Path: file.Path, Name: file.Name, URI: file.URI, Kind: file.Kind, ParentID: derefString(file.FolderID)})
	}
	if scope != "" {
		return source
	}
	goals, _ := ctx.GetGoals()
	for _, goal := range goals {
		source.GoalSeeds = append(source.GoalSeeds, treepkg.GoalRecord{ID: goal.ID, Title: goal.Title, URI: "repo://goal/" + goal.ID, Status: goal.Status, DueDate: goal.Dates.Due, Description: goal.Description})
	}
	tickets, _ := ctx.GetTickets(nil, nil, nil, nil)
	for _, ticket := range tickets {
		source.TicketSeeds = append(source.TicketSeeds, treepkg.TicketRecord{
			ID:          fmt.Sprintf("%d/%d/%d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug),
			Slug:        ticket.Slug,
			Title:       ticket.Title,
			URI:         "repo://ticket/" + ticket.Slug,
			Description: ticket.Description,
			Summary:     ticket.Summary,
			Goal:        ticket.Goal,
			Parent:      ticket.Parent,
			Year:        ticket.Year,
			Month:       ticket.Month,
			Day:         ticket.Day,
			Status:      string(ticket.Status),
		})
	}
	drafts, _ := ctx.GetDrafts()
	for _, draft := range drafts {
		source.DraftRecords = append(source.DraftRecords, treepkg.DraftRecord{ID: draft.ID, URI: "repo://draft/" + draft.ID})
	}
	for _, policy := range ctx.GetPolicies() {
		source.PolicyRecords = append(source.PolicyRecords, treepkg.PolicyRecord{ID: policy.ID, Name: policy.Name, Description: derefString(policy.Description), Groups: policy.Groups})
	}
	contributors, _ := ctx.GetContributors()
	for _, contributor := range contributors {
		email := ""
		if len(contributor.Emails) > 0 {
			email = contributor.Emails[0]
		}
		source.ContributorRecords = append(source.ContributorRecords, treepkg.ContributorRecord{
			ID:      contributor.Alias,
			Alias:   contributor.Alias,
			URI:     "repo://contributor/" + contributor.Alias,
			Name:    contributor.Name,
			Aliases: []string{},
			Github:  contributor.Github,
			Githubs: []string{},
			Names:   []string{},
			Email:   email,
			Emails:  contributor.Emails,
		})
	}
	limit := 100
	checkpoints, _ := ctx.GetCheckpoints(&limit)
	for _, checkpoint := range checkpoints {
		source.CheckpointRecords = append(source.CheckpointRecords, treepkg.CheckpointRecord{ID: checkpoint.ID, Sha: checkpoint.SHA, Title: checkpoint.Title, URI: "repo://checkpoint/" + checkpoint.SHA, AuthorID: derefString(checkpoint.AuthorID)})
	}
	return source
}

// 🧪️Technologies MUST return the technologies with their bundles, in load order.
func (s *FsTreeSource) Technologies() []treepkg.TechnologyRecord { return s.TechnologyRecords }

// 📁️Folders MUST return every folder of the projection.
func (s *FsTreeSource) Folders() []treepkg.FolderRecord { return s.FolderRecords }

// 📄️Files MUST return every file of the projection.
func (s *FsTreeSource) Files() []treepkg.FileRecord { return s.FileRecords }

// 🎯️GoalRecords MUST return every goal of the projection.
func (s *FsTreeSource) GoalRecords() []treepkg.GoalRecord { return s.GoalSeeds }

// 🎫️TicketRecords MUST return every ticket of the projection.
func (s *FsTreeSource) TicketRecords() []treepkg.TicketRecord { return s.TicketSeeds }

// ✍️Drafts MUST return every draft of the projection.
func (s *FsTreeSource) Drafts() []treepkg.DraftRecord { return s.DraftRecords }

// 🛡️Policies MUST return every policy of the projection.
func (s *FsTreeSource) Policies() []treepkg.PolicyRecord { return s.PolicyRecords }

// 🧑️Contributors MUST return every contributor of the projection.
func (s *FsTreeSource) Contributors() []treepkg.ContributorRecord { return s.ContributorRecords }

// 🔀️Checkpoints MUST return every checkpoint of the projection.
func (s *FsTreeSource) Checkpoints() []treepkg.CheckpointRecord { return s.CheckpointRecords }

// ⚪️Sessions MUST return every agent session of the projection.
func (s *FsTreeSource) Sessions() []treepkg.SessionRecord { return nil }

// 📑️Sections MUST return the parsed sections of one file, read only when sections are requested.
func (s *FsTreeSource) Sections(filePath string) []treepkg.SectionRecord {
	if !s.IncludeSections {
		return nil
	}
	content, err := os.ReadFile(filepath.Join(s.Root, filePath))
	if err != nil {
		return nil
	}
	sections := languagespkg.ParseSections(string(content), filePath)
	records := make([]treepkg.SectionRecord, 0, len(sections))
	for i := range sections {
		records = append(records, convertSectionRecord(&sections[i], filePath))
	}
	return records
}

// 🔁️convertSectionRecord projects one parsed section into the record the tree module reads.
func convertSectionRecord(section *model.Section, filePath string) treepkg.SectionRecord {
	path := section.Path
	if path == "" {
		path = filePath + "#" + section.Name
	}
	record := treepkg.SectionRecord{ID: section.ID, Path: path, Name: section.Name, StartLine: section.StartLine, EndLine: section.EndLine}
	record.Definitions = make([]treepkg.DefinitionRecord, 0, len(section.Definitions))
	for _, definition := range section.Definitions {
		record.Definitions = append(record.Definitions, treepkg.DefinitionRecord{ID: definition.ID, Name: definition.Name, Kind: string(definition.Kind), FilePath: filePath, StartLine: definition.StartLine, EndLine: definition.EndLine})
	}
	record.Children = make([]treepkg.SectionRecord, 0, len(section.Children))
	for i := range section.Children {
		record.Children = append(record.Children, convertSectionRecord(&section.Children[i], filePath))
	}
	return record
}

// #endregion 🌳️TreeSource

// #region 🌳️Tree Command

// 🌳️treeCommand builds the `tree` verb: the four trees 🌳️tree projects, each rendered through the
// persistent format flag. Every subcommand builds through the tree module's own ports, so the two
// implementations project the same document from the same repository.
func treeCommand(factory EngineFactory, config *Config) *Command {
	_ = factory
	root := &Command{Use: "tree", Short: "Show a repository tree"}
	monorepo := &Command{
		Use:   "monorepo [query]",
		Short: "Show the monorepo tree",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}
			scope, _ := cmd.Flags().GetString("scope")
			includeSections := filter.OnlyKinds[treepkg.TreeNodeSection] || filter.OnlyKinds[treepkg.TreeNodeDefinition]
			source := LoadFsTreeSource(graphqlpkg.NewRepoContext(repoRootOf(config)), includeSections, scope)
			node := treepkg.BuildMonorepoTree(source, treepkg.TreeBuildOptions{IncludeSections: includeSections})
			if filter.Query != "" {
				node = treepkg.SearchTreeInMemory(node, filter.Query)
			}
			return writeTreeNode(cmd, config, treepkg.FilterMonorepoTree(node, &filter))
		},
	}
	monorepo.Flags().String("scope", "", "Restrict the projection to this repository-relative path")
	bindTreeFlags(monorepo)
	goal := &Command{
		Use:   "goal",
		Short: "Show the goal and ticket tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			source := LoadFsTreeSource(graphqlpkg.NewRepoContext(repoRootOf(config)), false, "")
			roots := treepkg.BuildGoalTree(source.GoalRecords(), source.TicketRecords())
			if config.IsJSON() {
				encoded, err := json.Marshal(roots)
				if err != nil {
					return err
				}
				fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
				return nil
			}
			format := treepkg.TreeRenderFormatText
			if config.IsMarkdown() {
				format = treepkg.TreeRenderFormatMarkdown
			}
			fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderGoalTreeNodes(roots, format, treepkg.DefaultEntityRenderer{}))
			return nil
		},
	}
	statute := &Command{
		Use:   "statute",
		Short: "Show the statute tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			return writeTreeForest(cmd, config, treepkg.BuildStatuteTree(treepkg.DeclaredStatutes(), treepkg.DeclaredStatuteCatalog{}))
		},
	}
	territory := &Command{
		Use:   "territory",
		Short: "Show the territory tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			return writeTreeForest(cmd, config, treepkg.BuildTerritoryTree(treepkg.DeclaredTerritories(), treepkg.DeclaredStatuteCatalog{}))
		},
	}
	root.AddCommand(monorepo)
	root.AddCommand(goal)
	root.AddCommand(statute)
	root.AddCommand(territory)
	return root
}

// 🏠️repoRootOf resolves the repository root one invocation runs against.
func repoRootOf(config *Config) string {
	if config != nil && config.Repo != "" {
		return config.Repo
	}
	return workspace.GetRootDir()
}

// 🌲️writeTreeForest renders a forest under the same anonymous category root the monorepo tree carries.
func writeTreeForest(cmd *Command, config *Config, roots []*treepkg.TreeNode) error {
	node := treepkg.NewTreeNode(treepkg.TreeNodeCategory, "", ".", "")
	node.Children = roots
	return writeTreeNode(cmd, config, node)
}

// 🖨️writeTreeNode writes one tree node in the requested format, exactly as `search` renders one.
func writeTreeNode(cmd *Command, config *Config, node *treepkg.TreeNode) error {
	switch {
	case config.IsJSON():
		encoded, err := json.Marshal(node)
		if err != nil {
			return err
		}
		fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
	case config.IsMarkdown():
		fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderMonorepoTreeMarkdown(node, treepkg.DefaultEntityRenderer{}))
	default:
		fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderMonorepoTree(node, treepkg.DefaultEntityRenderer{}))
	}
	return nil
}

// #endregion 🌳️Tree Command
