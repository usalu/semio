// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️ticket carries the ticket command of the cli.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
)

// #region 🕸️Test Command

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

// #endregion 🕸️Test Command
