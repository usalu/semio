// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧑️contributor carries the contributor and technology commands of the cli.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"

	testrunner "github.com/usalu/semio/repo/testrunner"
)

// #region 🕸️Test Command

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

// #endregion 🕸️Test Command
