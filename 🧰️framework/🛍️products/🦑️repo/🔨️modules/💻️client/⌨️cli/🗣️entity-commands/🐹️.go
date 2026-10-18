// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Interaction, statute, contributor, and technology commands.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"sort"
	"strings"
)

// 🟥️interactionCommand holds the data fields for a interactionCommand record.
func interactionCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "interaction", Short: "Interaction management commands"}
	listCmd := &command.Command{
		Use:   "list",
		Short: "List all interactions",
		RunE: func(cmd *command.Command, args []string) error {
			sorted, _ := cmd.Flags().GetBool("sorted")
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction list"}
				if sorted {
					interactions, _ := ListInteractions()
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
					ch := make(chan InteractionResource)
					go func() {
						StreamInteractions(context.Background(), ch)
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
	treeCmd := &command.Command{
		Use:   "tree",
		Short: "Show interactions within goal/ticket tree",
		RunE: func(cmd *command.Command, args []string) error {
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction tree"}
				goals, _ := ListGoals()
				tickets, _ := ListTickets(nil, nil, nil)
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
func statuteCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "statute", Short: "Statute management commands"}
	listCmd := &command.Command{
		Use:   "list",
		Short: "List statutes",
		RunE: func(cmd *command.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute list"}
				vkChan := make(chan StatuteMeta)
				go func() {
					StreamStatutes(context.Background(), vkChan, opts)
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
	treeCmd := &command.Command{
		Use:   "tree",
		Short: "Show statute tree",
		RunE: func(cmd *command.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute tree"}
				vkChan := make(chan StatuteMeta)
				go func() {
					StreamStatutes(context.Background(), vkChan, opts)
				}()
				var vks []StatuteMeta
				for vk := range vkChan {
					vks = append(vks, vk)
				}
				sort.Slice(vks, func(i, j int) bool { return string(vks[i].Kind) < string(vks[j].Kind) })
				if config.IsMarkdown() {
					var sb strings.Builder
					var kinds []Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := buildStatuteTree(kinds)
					var renderVkTree func(nodes []*TreeNode, indent string)
					renderVkTree = func(nodes []*TreeNode, indent string) {
						for _, n := range nodes {
							if n.Data != nil {
								vkData := map[string]interface{}{"id": n.Data["id"], "description": n.Data["reason"]}
								sb.WriteString(indent + renderEntityMarkdown("statute", vkData) + "\n")
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
					var kinds []Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := buildStatuteTree(kinds)
					var renderVkText func(nodes []*TreeNode, prefix string)
					renderVkText = func(nodes []*TreeNode, prefix string) {
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

// 🤝️contributorCommand holds the data fields for a contributorCommand record.
func contributorCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "contributor", Short: "Contributor management commands"}
	addCmd := &command.Command{
		Use:   "add",
		Short: "Add a contributor",
		Args:  command.MaximumNArgs(3),
		RunE: func(cmd *command.Command, args []string) error {
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
	removeCmd := &command.Command{
		Use:   "remove",
		Short: "Remove a contributor",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
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
func technologyCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:                "technology",
		Short:              "Manage technologies",
		DisableFlagParsing: false,
		Args:               command.ArbitraryArgs,
		RunE: func(cmd *command.Command, args []string) error {
			if len(args) == 3 && args[1] == "generate" {
				technologyName := args[0]
				kind := args[2]
				switch kind {
				case "requirements":
					return GenerateTechnologyRequirements(technologyName)
				case "docs":
					return GenerateTechnologyDocs(technologyName)
				case "todos":
					return GenerateTechnologyTodos(technologyName)
				default:
					return fmt.Errorf("unknown generate kind %q (use requirements, docs, or todos)", kind)
				}
			}
			return cmd.Help()
		},
	}

	listCmd := &command.Command{
		Use:   "list",
		Short: "List technologies",
		RunE: func(cmd *command.Command, args []string) error {
			return runTechnologyList(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(listCmd)
	cmd.AddCommand(listCmd)

	treeCmd := &command.Command{
		Use:   "tree",
		Short: "Show technology tree Structure",
		RunE: func(cmd *command.Command, args []string) error {
			return runTechnologyTree(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(treeCmd)
	cmd.AddCommand(treeCmd)

	return cmd
}
