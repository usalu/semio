// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪝️hook carries the hook, configure and micro-commit commands of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	strings "strings"

	hooks "github.com/usalu/semio/repo/hooks"
	workspace "github.com/usalu/semio/repo/workspace"
)

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
