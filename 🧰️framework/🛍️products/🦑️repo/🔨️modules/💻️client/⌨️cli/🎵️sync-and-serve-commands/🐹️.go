// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Auth, sync, MCP, GraphQL, analyze, and entity-emoji commands.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"time"
)

// 🆕️authCommand creates the auth command with whoami subcommand.
func authCommand(config *Config) *command.Command {
	auth := &command.Command{
		Use:   "auth",
		Short: "Server authentication",
	}
	auth.AddCommand(&command.Command{
		Use:   "whoami",
		Short: "Show current authenticated developer",
		RunE: func(cmd *command.Command, args []string) error {
			info, err := serverWhoami()
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
	auth.AddCommand(&command.Command{
		Use:   "status",
		Short: "Show server connection status",
		RunE: func(cmd *command.Command, args []string) error {
			addr := getServerAddr()
			if addr == "" {
				fmt.Println("Server: not configured (set COMPOSE_SERVER_ADDR)")
				return nil
			}
			fmt.Printf("Server: %s\n", addr)
			token := getServerToken()
			if token == "" {
				fmt.Println("Token: not set (set COMPOSE_SERVER_TOKEN)")
				return nil
			}
			fmt.Println("Token: configured")
			info, err := serverWhoami()
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

// 🔹️syncCommand holds the data fields for a syncCommand record.
func syncCommand(factory EngineFactory, config *Config) *command.Command {
	sync := &command.Command{
		Use:   "sync",
		Short: "Synchronize monorepo artifacts",
		RunE: func(cmd *command.Command, args []string) error {
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
func syncGitHubCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "github",
		Short: "Synchronize local state with GitHub",
		RunE: func(cmd *command.Command, args []string) error {
			return runSyncManagementMutation(cmd, factory, config)
		},
	}
}

// 🔸️syncManagementCommand holds the data fields for a syncManagementCommand record.
func syncManagementCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "management",
		Short: "Synchronize local state with management provider",
		RunE: func(cmd *command.Command, args []string) error {
			return runSyncManagementMutation(cmd, factory, config)
		},
	}
}

// 🔁️runSyncManagementMutation executes the shared management synchronization mutation.
func runSyncManagementMutation(cmd *command.Command, factory EngineFactory, config *Config) error {
	query := `mutation SyncManagement { syncManagement }`
	return runGraphQL(cmd, factory, config, query, nil)
}

// 🔺️mcpCommand holds the data fields for a mcpCommand record.
func mcpCommand(factory EngineFactory, config *Config) *command.Command {
	var dryRun bool
	cmd := &command.Command{
		Use:   "mcp [kind]",
		Short: "Run MCP server (optional kind: client, cursor, copilot, claude, codex, kiro)",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			if dryRun {
				return nil
			}
			kind := McpClientGeneric
			if len(args) > 0 {
				parsed, err := ParseMcpClientKind(args[0])
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

func serveMcp(ctx context.Context, kind McpClientKind, toolTimeout time.Duration) error {
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
func graphqlCommand(factory EngineFactory, config *Config) *command.Command {
	var query string
	var variablesJSON string
	cmd := &command.Command{
		Use:   "graphql [query]",
		Short: "Execute a GraphQL query",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
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

func analyzeCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "analyze <id>",
		Short: "Analyze an entity by its ID",
		Args:  command.ExactArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			ctx := cmd.Context()
			id := args[0]
			buildOpts := TreeBuildOptions{}
			tree := BuildMonorepoTreeCached(ctx, buildOpts)
			var found *TreeNode
			var findByID func(node *TreeNode)
			findByID = func(node *TreeNode) {
				if found != nil {
					return
				}
				if node.ID == id {
					found = node
					return
				}
				for _, c := range node.Children {
					findByID(c)
				}
			}
			findByID(tree)
			if found == nil {
				return fmt.Errorf("entity not found: %s", id)
			}
			type summaryOutput struct {
				ID      string `json:"id"`
				Kind    string `json:"kind"`
				Label   string `json:"label"`
				Summary string `json:"summary"`
				URI     string `json:"uri"`
			}
			out := summaryOutput{
				ID:      found.ID,
				Kind:    string(found.Kind),
				Label:   found.Label,
				Summary: found.Summary,
				URI:     found.URI,
			}
			switch {
			case config.IsJSON():
				encoded, err := json.Marshal(out)
				if err != nil {
					return err
				}
				fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
			default:
				fmt.Fprintf(cmd.OutOrStdout(), "[%s] %s\n", out.Kind, out.Label)
				if out.Summary != "" {
					fmt.Fprintf(cmd.OutOrStdout(), "  %s\n", out.Summary)
				}
				if out.URI != "" {
					fmt.Fprintf(cmd.OutOrStdout(), "  %s\n", out.URI)
				}
			}
			return nil
		},
	}
	return cmd
}

// through emojiText and deduplicated first-wins, so the CLI, the VS Code extension and the Rust
// catalog answer with the same set. The catalog is the generated projection EntityKindCatalog
// (🏷️entity-kinds/🐹️.go) of the framework-owned `framework.schema` export EntityKindCatalog; this
// function MUST NOT restate it.
func AllEntityEmojis() []string {
	seen := map[string]bool{}
	result := make([]string, 0, len(EntityKindCatalog))
	for _, kind := range EntityKindCatalog {
		normalized := emojiText(kind.Emoji)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		result = append(result, normalized)
	}
	return result
}

func entityEmojisCommand(config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "entity-emojis",
		Short: "List all entity-identifying emojis",
		RunE: func(cmd *command.Command, args []string) error {
			emojis := AllEntityEmojis()
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
