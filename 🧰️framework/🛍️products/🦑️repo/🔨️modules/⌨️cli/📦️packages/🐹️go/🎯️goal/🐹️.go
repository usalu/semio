// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎯️goal carries the goal command of the cli.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🕸️Test Command

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

// #endregion 🕸️Test Command
