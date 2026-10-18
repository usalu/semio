// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📜️statute carries the statute command of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	sort "sort"
	strings "strings"

	goalspkg "github.com/usalu/semio/repo/goals"
	model "github.com/usalu/semio/repo/model"
	treepkg "github.com/usalu/semio/repo/tree"
)

// #region 🕸️Test Command

// 🟧️statuteCommand holds the data fields for a statuteCommand record.
func statuteCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "statute", Short: "Statute management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List statutes",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute list"}
				vkChan := make(chan model.StatuteMeta)
				go func() {
					goalspkg.StreamStatutes(context.Background(), vkChan, opts)
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
	treeCmd := &Command{
		Use:   "tree",
		Short: "Show statute tree",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "statute tree"}
				vkChan := make(chan model.StatuteMeta)
				go func() {
					goalspkg.StreamStatutes(context.Background(), vkChan, opts)
				}()
				var vks []model.StatuteMeta
				for vk := range vkChan {
					vks = append(vks, vk)
				}
				sort.Slice(vks, func(i, j int) bool { return string(vks[i].Kind) < string(vks[j].Kind) })
				if config.IsMarkdown() {
					var sb strings.Builder
					var kinds []model.Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := treepkg.BuildStatuteTreeForRepo(kinds)
					var renderVkTree func(nodes []*treepkg.TreeNode, indent string)
					renderVkTree = func(nodes []*treepkg.TreeNode, indent string) {
						for _, n := range nodes {
							if n.Data != nil {
								vkData := map[string]interface{}{"id": n.Data["id"], "description": n.Data["reason"]}
								sb.WriteString(indent + model.RenderEntityMarkdown("statute", vkData) + "\n")
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
					var kinds []model.Statute
					for _, vk := range vks {
						kinds = append(kinds, vk.Kind)
					}
					vkTree := treepkg.BuildStatuteTreeForRepo(kinds)
					var renderVkText func(nodes []*treepkg.TreeNode, prefix string)
					renderVkText = func(nodes []*treepkg.TreeNode, prefix string) {
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

// #endregion 🕸️Test Command
