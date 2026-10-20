// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Monorepo CLI tool for repository management, analysis and code generation.

// Per-IDE MCP server kind, native descriptions, plan/spec resolution, and hook dispatch.
// #endregion 🧲️Header

package client

import (
	"errors"
	"fmt"
	"github.com/usalu/semio/repo/client/internal/command"
	"github.com/usalu/semio/repo/graphql"
	"os"
	"time"

)

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

// 💿️ExitError holds the data fields for a exit error record.
type ExitError struct {
	Code int
}

// 🔤️Error MUST return a formatted string representation.
// ❌️Error returns the string representation of the error.
func (e ExitError) Error() string {
	return fmt.Sprintf("exit status %d", e.Code)
}

// 🌱️NewRoot MUST initialize all required fields and return a valid Root.
// 🌱️NewRoot creates and returns a new Root instance.
func NewRoot(factory EngineFactory) *command.Command {
	root, _ := NewRootWithConfig(factory)
	return root
}

// 🔷️NewRootWithConfig MUST initialize all required fields and return a valid RootWithConfig.
// 🆕️NewRootWithConfig creates and returns a new RootWithConfig instance.
func NewRootWithConfig(factory EngineFactory) (*command.Command, *Config) {
	config := Config{}
	root := &command.Command{
		Use:           "repo",
		Short:         "Monorepo CLI for Compose",
		SilenceUsage:  true,
		SilenceErrors: true,
	}
	root.PersistentFlags().StringVar(&config.Format, "format", "md", "Output format: md, text, json")
	root.PersistentFlags().BoolP("md", "", false, "Shorthand for --format md")
	root.PersistentFlags().BoolP("text", "", false, "Shorthand for --format text")
	root.PersistentFlags().BoolP("json", "", false, "Shorthand for --format json")
	root.PersistentPreRunE = func(cmd *command.Command, args []string) error {
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
	root.AddCommand(exportCommand(factory, &config))
	root.AddCommand(hookCommand(factory, &config))
	root.AddCommand(mermaidCommand(factory, &config))
	root.AddCommand(locCommand(factory, &config))
	root.AddCommand(technologyCommand(factory, &config))
	root.AddCommand(bundleCommand(factory, &config))
	root.AddCommand(analyzeCommand(factory, &config))
	root.AddCommand(entityEmojisCommand(&config))
	root.AddCommand(configureCommand(factory, &config))
	root.AddCommand(microCommitCommand(factory, &config))
	root.AddCommand(benchmarkCmd)
	root.AddCommand(updateCmd)
	root.AddCommand(authCommand(&config))
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
		repoRoot = findRepoRoot(cwd)
	}
	SetRootDir(repoRoot)
	exec, err := graphql.NewExecutor(repoRoot)
	if err != nil {
		return nil, err
	}
	return NewEngine(exec), nil
}

// 🔊️RunCLI runs the repo CLI entry point.
func RunCLI() error {
	if err := BindWorkingDirectory(); err != nil {
		return err
	}
	if err := Execute(defaultEngineFactory); err != nil {
		var exitErr ExitError
		if errors.As(err, &exitErr) {
			return exitErr
		}
		return err
	}
	return nil
}

// 🦀️RunMCP runs the repo MCP entry point (generic kind).
func RunMCP() error {
	return RunMcpServerFor(McpClientGeneric, DefaultCommandTimeout)
}
