// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⌨️cli is the cli domain of the semio repository tooling: the root command, its configuration and the process entry points.

// #endregion 🧲️Header

package cli

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
 "path/filepath"
	"time"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	"github.com/usalu/semio/repo/providers"
	"github.com/usalu/semio/repo/workspace"

)

// #region 🌧️Cli Adapter

// ⚙️Config holds the data fields for a config record.
type Config struct {
	Format  string
	Verbose bool
	Repo    string
	Timeout time.Duration
}

// 🎮️DefaultCommandTimeout is the wall-clock budget for MCP tool calls, hooks, and other repo CLI work when --timeout is not set.
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
	analyzeCmd.Flags().StringSlice("policy", nil, "Analyze only the selected registered policies")
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

// 🚪️NativeEntrypointPlan selects the owned process port without parsing its arguments.
type NativeEntrypointPlan struct { Kind string; Arguments []string }

// 🧭️PlanNativeEntrypoint preserves opaque arguments at the native process boundary.
func PlanNativeEntrypoint(argv []string) NativeEntrypointPlan {
 arguments := append([]string{}, argv...)
 if len(argv) >= 4 && argv[0] == "dev" && argv[1] == "mcp" && argv[2] == "stdio" && argv[3] == "os" { return NativeEntrypointPlan{"workspace", arguments} }
 if len(argv) > 0 && argv[0] == "mcp" { return NativeEntrypointPlan{"mcp", arguments[1:]} }
 if len(argv) > 0 && argv[0] == "dashboard" { return NativeEntrypointPlan{"build", append([]string{"run", "@semio-tech/repo-dashboard-rs:run", "--output=stream", "--"}, argv[1:]...)} }
 index := 0
 for index < len(argv) {
  if argv[index] == "--tool" { index += 2; continue }
  if len(argv[index]) >= 7 && argv[index][:7] == "--tool=" { index++; continue }; break
 }
 if index > 0 { return NativeEntrypointPlan{"build", arguments} }
 if index < len(argv) { switch argv[index] { case "run", "run-many", "affected", "exec", "watch", "dependencies", "cargo", "tools", "reset", "cache", "preflight", "show", "graph": return NativeEntrypointPlan{"build", arguments} } }
 return NativeEntrypointPlan{"domain", arguments}
}

// 🛠️NativeScriptTool is the tool-independent owned build process port.
type NativeScriptTool interface { Run(root string, arguments []string) error }

type nativeBunBuildTool struct{}

func (nativeBunBuildTool) Run(root string, arguments []string) error {
 entry := filepath.Join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")
 return runNativeScriptProcess(append([]string{entry, "repo"}, arguments...))
}

type nativeRootScriptTool struct{}
func (nativeRootScriptTool) Run(root string, arguments []string) error {
 return runNativeScriptProcess(append([]string{filepath.Join(root, "📜️script.ts")}, arguments...))
}

// 🏗️RunNativeScript selects the issued build invocation without rewriting its environment.
func RunNativeScript(arguments []string, tool NativeScriptTool) error {
 cwd, err := os.Getwd(); if err != nil { return err }
 return tool.Run(workspace.FindRepoRoot(cwd), arguments)
}

// 🔊️RunCLI runs the repo CLI entry point.
func RunCLI() error {
 plan := PlanNativeEntrypoint(os.Args[1:])
 if plan.Kind == "build" { return RunNativeScript(plan.Arguments, nativeBunBuildTool{}) }
 if plan.Kind == "workspace" { return RunNativeScript(plan.Arguments, nativeRootScriptTool{}) }
 if len(os.Args) == 1 || os.Args[1] == "--help" || os.Args[1] == "-h" { fmt.Fprintln(os.Stdout, "Repository build commands: run, run-many, affected, exec, watch, dependencies, cargo, tools, reset, cache, preflight, show, graph\nHuman control plane: repo dashboard [arguments]\nMCP stdio: repo mcp (SEMIO_REPO_MCP_CLIENT selects the profile)\nOS MCP stdio: repo dev mcp stdio os [arguments]") }
	if err := workspace.BindWorkingDirectory(); err != nil {
		return err
	}
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
