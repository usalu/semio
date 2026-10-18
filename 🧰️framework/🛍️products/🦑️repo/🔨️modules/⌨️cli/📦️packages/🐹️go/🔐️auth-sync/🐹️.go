// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔐️auth_sync carries the auth and sync commands of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	fmt "fmt"

	eventspkg "github.com/usalu/semio/repo/events"
)

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

// #endregion 🌧️Cli Adapter
