// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏩️goal-render carries the model goal tree builders and renderers: goal and ticket nodes, open counts and section and ticket list rendering.

// #endregion 🧲️Header

package tree

import (
	fmt "fmt"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏩️Tree Logic

// 🏢️Tree construction, filtering, searching, and rendering for goals, sections, and monorepo nodes.
func buildGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}) []*model.GoalNode {
	goalMap := make(map[string]*model.GoalNode)

	for _, g := range goalsRaw {
		if gm, ok := g.(map[string]interface{}); ok {
			id, _ := gm["id"].(string)
			title, _ := gm["title"].(string)
			status, _ := gm["status"].(string)
			dueDate, _ := gm["dueDate"].(string)
			createdAt, _ := gm["createdAt"].(string)
			description, _ := gm["description"].(string)
			node := &model.GoalNode{ID: id, Title: title, Status: status, DueDate: dueDate, CreatedAt: createdAt, Description: description}
			goalMap[id] = node
		}
	}

	var rootGoals []*model.GoalNode
	for _, g := range goalMap {
		parentID := ""
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			parentID = g.ID[:idx]
		}
		if parentID != "" && parentID != g.ID {
			if parent, ok := goalMap[parentID]; ok {
				parent.Children = append(parent.Children, g)
			} else {
				rootGoals = append(rootGoals, g)
			}
		} else {
			rootGoals = append(rootGoals, g)
		}
	}

	var sortGoals func(goals []*model.GoalNode)
	sortGoals = func(goals []*model.GoalNode) {
		sort.SliceStable(goals, func(i, j int) bool {
			a, b := goals[i], goals[j]
			if a.DueDate != b.DueDate {
				if a.DueDate == "" {
					return false
				}
				if b.DueDate == "" {
					return true
				}
				return a.DueDate < b.DueDate
			}
			return a.ID < b.ID
		})
		for _, g := range goals {
			if len(g.Children) > 0 {
				sortGoals(g.Children)
			}
		}
	}
	sortGoals(rootGoals)

	allTickets := make([]*model.TicketNode, 0)
	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			id, _ := tm["id"].(string)
			slug, _ := tm["slug"].(string)
			status, _ := tm["status"].(string)
			title, _ := tm["title"].(string)
			goalID, _ := tm["goal"].(string)
			parentID, _ := tm["parent"].(string)
			prompt, _ := tm["prompt"].(string)
			summary, _ := tm["summary"].(string)

			created := ""
			finished := ""
			if dates, ok := tm["date"].(map[string]interface{}); ok {
				created, _ = dates["created"].(string)
				finished, _ = dates["finished"].(string)
			}

			if created == "" {
				created, _ = tm["createdAt"].(string)
			}

			uri := fmt.Sprintf("repo://ticket/%s%s", model.EmojiText(model.EmojiTicket), workspace.Flat(slug))

			node := &model.TicketNode{ID: id, Slug: slug, Status: status, Title: title, URI: uri, GoalID: goalID, ParentID: parentID, Created: created, Finished: finished, Description: prompt, Summary: summary}
			allTickets = append(allTickets, node)
		}
	}

	var noGoalTickets []*model.TicketNode
	for _, t := range allTickets {
		if t.GoalID != "" {
			if g, ok := goalMap[t.GoalID]; ok {
				g.Tickets = append(g.Tickets, t)
			} else {
				noGoalTickets = append(noGoalTickets, t)
			}
		} else {
			noGoalTickets = append(noGoalTickets, t)
		}
	}

	nestTickets := func(nodes []*model.TicketNode) []*model.TicketNode {
		var roots []*model.TicketNode
		lookup := make(map[string]*model.TicketNode)
		for _, n := range nodes {
			lookup[n.ID] = n
		}
		for _, n := range nodes {
			if n.ParentID != "" && lookup[n.ParentID] != nil {
				lookup[n.ParentID].Children = append(lookup[n.ParentID].Children, n)
			} else {
				roots = append(roots, n)
			}
		}
		return roots
	}

	for _, g := range goalMap {
		g.Tickets = nestTickets(g.Tickets)
	}

	if len(noGoalTickets) > 0 {
		noGoal := &model.GoalNode{Title: "No Goal", Children: nil, Tickets: nestTickets(noGoalTickets)}
		rootGoals = append(rootGoals, noGoal)
	}

	return rootGoals
}

func countOpenSubgoals(g *model.GoalNode) int {
	c := 0
	for _, child := range g.Children {
		if child.Status == "open" {
			c++
		}
		c += countOpenSubgoals(child)
	}
	return c
}

// 🎫️countOpenTickets holds the data fields for a countOpenTickets record.
func countOpenTickets(g *model.GoalNode) int {
	c := 0
	var countT func(ts []*model.TicketNode)
	countT = func(ts []*model.TicketNode) {
		for _, t := range ts {
			if t.Status == "open" {
				c++
			}
			countT(t.Children)
		}
	}
	countT(g.Tickets)
	for _, child := range g.Children {
		c += countOpenTickets(child)
	}
	return c
}

// 🎨️renderGoalTree holds the data fields for a renderGoalTree record.
func RenderGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	roots := buildGoalTree(goalsRaw, ticketsRaw)
	format := "text"
	if useMD {
		format = "md"
	}
	return RenderModelGoalTreeNodes(roots, format)
}

// 💿️goalNodeToData holds the data fields for a goalNodeToData record.
func goalNodeToData(n *model.GoalNode) map[string]interface{} {
	return map[string]interface{}{
		"id":          n.ID,
		"title":       n.Title,
		"status":      n.Status,
		"dueDate":     n.DueDate,
		"createdAt":   n.CreatedAt,
		"description": n.Description,
	}
}

// 🌿️ticketNodeToData holds the data fields for a ticketNodeToData record.
func ticketNodeToData(n *model.TicketNode) map[string]interface{} {
	data := map[string]interface{}{
		"slug":     n.Slug,
		"title":    n.Title,
		"status":   n.Status,
		"started":  n.Created,
		"finished": n.Finished,
		"prompt":   n.Description,
		"summary":  n.Summary,
	}
	return data
}

// ⛳️renderGoalTreeNodes holds the data fields for a renderGoalTreeNodes record.
func RenderModelGoalTreeNodes(roots []*model.GoalNode, format string) string {
	var sb strings.Builder

	var render func(node interface{}, prefix string, isLast bool, isRoot bool)
	render = func(node interface{}, prefix string, isLast bool, isRoot bool) {
		var lineContent string
		var children []interface{}

		switch n := node.(type) {
		case *model.GoalNode:
			data := goalNodeToData(n)
			if format == "md" {
				lineContent = model.RenderEntityMarkdownLink("goal", data)
			} else {
				lineContent = model.RenderEntityHuman("goal", data, false)
			}
			for _, c := range n.Children {
				children = append(children, c)
			}
			for _, t := range n.Tickets {
				children = append(children, t)
			}
		case *model.TicketNode:
			data := ticketNodeToData(n)
			if format == "md" {
				lineContent = model.RenderEntityMarkdownLink("ticket", data)
			} else {
				lineContent = model.RenderEntityHuman("ticket", data, false)
			}
			for _, c := range n.Children {
				children = append(children, c)
			}
		}

		if format == "text" {
			connector := ""
			if !isRoot {
				if isLast {
					connector = "└️─️─️ "
				} else {
					connector = "├️─️─️ "
				}
			}
			sb.WriteString(prefix + connector + lineContent + "\n")

			newPrefix := prefix
			if !isRoot {
				if isLast {
					newPrefix += "    "
				} else {
					newPrefix += "│️   "
				}
			}

			for i, child := range children {
				render(child, newPrefix, i == len(children)-1, false)
			}
		} else {

			sb.WriteString(prefix + "- " + lineContent + "\n")
			newPrefix := prefix + "  "
			for i, child := range children {
				render(child, newPrefix, i == len(children)-1, false)
			}
		}
	}

	for i, root := range roots {
		render(root, "", i == len(roots)-1, true)
	}
	return sb.String()
}

func RenderSectionTree(root *model.Section, isTTY bool, useMD bool) string {
	var sb strings.Builder

	var render func(s *model.Section, prefix string, isLast bool, isRoot bool)
	render = func(s *model.Section, prefix string, isLast bool, isRoot bool) {
		data := map[string]interface{}{
			"path":      s.Path,
			"name":      s.Name,
			"startLine": float64(s.StartLine),
			"endLine":   float64(s.EndLine),
		}

		var lineContent string
		if useMD {
			lineContent = model.RenderEntityMarkdown("section", data)
		} else {
			lineContent = model.RenderEntityHuman("section", data, false)
		}

		if !useMD {
			connector := ""
			if !isRoot {
				if isLast {
					connector = "└️─️─️ "
				} else {
					connector = "├️─️─️ "
				}
			}
			sb.WriteString(prefix + connector + lineContent + "\n")
			newPrefix := prefix
			if !isRoot {
				if isLast {
					newPrefix += "    "
				} else {
					newPrefix += "│️   "
				}
			}
			for i := range s.Children {
				childPtr := &s.Children[i]
				render(childPtr, newPrefix, i == len(s.Children)-1, false)
			}
		} else {
			sb.WriteString(prefix + lineContent + "\n")
			newPrefix := prefix + "  "
			for i := range s.Children {
				childPtr := &s.Children[i]
				render(childPtr, newPrefix, i == len(s.Children)-1, false)
			}
		}
	}

	render(root, "", true, true)
	return sb.String()
}

// 📋️renderTicketList holds the data fields for a renderTicketList record.
func RenderTicketList(ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	var sb strings.Builder

	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			if useMD {
				sb.WriteString(model.RenderEntityMarkdown("ticket", tm) + "\n")
			} else {
				sb.WriteString("  " + model.RenderEntityHuman("ticket", tm, false) + "\n")
			}
		}
	}
	return sb.String()
}

// #endregion 🏩️Tree Logic
