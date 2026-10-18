// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📝️todo carries the draft and todo commands of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	fmt "fmt"

	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🕸️Test Command

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

// #endregion 🕸️Test Command
