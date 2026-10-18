// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 💬️interaction carries the interaction and checkpoint commands of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	sort "sort"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	goalspkg "github.com/usalu/semio/repo/goals"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
)

// #region 🕸️Test Command

// 🟥️interactionCommand holds the data fields for a interactionCommand record.
func interactionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "interaction", Short: "Interaction management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List all interactions",
		RunE: func(cmd *Command, args []string) error {
			sorted, _ := cmd.Flags().GetBool("sorted")
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction list"}
				if sorted {
					interactions, _ := ticketspkg.ListInteractions()
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
					ch := make(chan model.InteractionResource)
					go func() {
						ticketspkg.StreamInteractions(context.Background(), ch)
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
	treeCmd := &Command{
		Use:   "tree",
		Short: "Show interactions within goal/ticket tree",
		RunE: func(cmd *Command, args []string) error {
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "interaction tree"}
				goals, _ := goalspkg.ListGoals()
				tickets, _ := ticketspkg.ListTickets(nil, nil, nil)
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

func checkpointCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "checkpoint", Short: "Checkpoint management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List checkpoints",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			limit := 100
			if l, _ := cmd.Flags().GetInt("limit"); l > 0 {
				limit = l
			}
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "checkpoint list"}
				checkpointChan := make(chan model.Checkpoint)
				go func() {
					contributorspkg.StreamCheckpoints(context.Background(), &limit, checkpointChan, opts)
				}()
				for c := range checkpointChan {
					data, err := json.Marshal(map[string]interface{}{"checkpoint": c})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "checkpoint list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().Int("limit", 100, "Maximum number of checkpoints to show")
	bindStreamFlags(listCmd)
	root.AddCommand(listCmd)
	return root
}

// #endregion 🕸️Test Command
