// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Goal tree building and rendering.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"sort"
	"strings"
)

func buildGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}) []*GoalNode {
	goalMap := make(map[string]*GoalNode)

	for _, g := range goalsRaw {
		if gm, ok := g.(map[string]interface{}); ok {
			id, _ := gm["id"].(string)
			title, _ := gm["title"].(string)
			status, _ := gm["status"].(string)
			dueDate, _ := gm["dueDate"].(string)
			createdAt, _ := gm["createdAt"].(string)
			description, _ := gm["description"].(string)
			node := &GoalNode{ID: id, Title: title, Status: status, DueDate: dueDate, CreatedAt: createdAt, Description: description}
			goalMap[id] = node
		}
	}

	var rootGoals []*GoalNode
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

	var sortGoals func(goals []*GoalNode)
	sortGoals = func(goals []*GoalNode) {
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

	allTickets := make([]*TicketNode, 0)
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

			uri := fmt.Sprintf("repo://ticket/%s%s", emojiText(EmojiTicket), Flat(slug))

			node := &TicketNode{ID: id, Slug: slug, Status: status, Title: title, URI: uri, GoalID: goalID, ParentID: parentID, Created: created, Finished: finished, Description: prompt, Summary: summary}
			allTickets = append(allTickets, node)
		}
	}

	var noGoalTickets []*TicketNode
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

	nestTickets := func(nodes []*TicketNode) []*TicketNode {
		var roots []*TicketNode
		lookup := make(map[string]*TicketNode)
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
		noGoal := &GoalNode{Title: "No Goal", Children: nil, Tickets: nestTickets(noGoalTickets)}
		rootGoals = append(rootGoals, noGoal)
	}

	return rootGoals
}

func countOpenSubgoals(g *GoalNode) int {
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
func countOpenTickets(g *GoalNode) int {
	c := 0
	var countT func(ts []*TicketNode)
	countT = func(ts []*TicketNode) {
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
func renderGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	roots := buildGoalTree(goalsRaw, ticketsRaw)
	format := "text"
	if useMD {
		format = "md"
	}
	return renderGoalTreeNodes(roots, format)
}

// 💿️goalNodeToData holds the data fields for a goalNodeToData record.
func goalNodeToData(n *GoalNode) map[string]interface{} {
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
func ticketNodeToData(n *TicketNode) map[string]interface{} {
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
func renderGoalTreeNodes(roots []*GoalNode, format string) string {
	var sb strings.Builder

	var render func(node interface{}, prefix string, isLast bool, isRoot bool)
	render = func(node interface{}, prefix string, isLast bool, isRoot bool) {
		var lineContent string
		var children []interface{}

		switch n := node.(type) {
		case *GoalNode:
			data := goalNodeToData(n)
			if format == "md" {
				lineContent = renderEntityMarkdownLink("goal", data)
			} else {
				lineContent = renderEntityHuman("goal", data, false)
			}
			for _, c := range n.Children {
				children = append(children, c)
			}
			for _, t := range n.Tickets {
				children = append(children, t)
			}
		case *TicketNode:
			data := ticketNodeToData(n)
			if format == "md" {
				lineContent = renderEntityMarkdownLink("ticket", data)
			} else {
				lineContent = renderEntityHuman("ticket", data, false)
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

func renderSectionTree(root *Section, isTTY bool, useMD bool) string {
	var sb strings.Builder

	var render func(s *Section, prefix string, isLast bool, isRoot bool)
	render = func(s *Section, prefix string, isLast bool, isRoot bool) {
		data := map[string]interface{}{
			"path":      s.Path,
			"name":      s.Name,
			"startLine": float64(s.StartLine),
			"endLine":   float64(s.EndLine),
		}

		var lineContent string
		if useMD {
			lineContent = renderEntityMarkdown("section", data)
		} else {
			lineContent = renderEntityHuman("section", data, false)
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
func renderTicketList(ticketsRaw []interface{}, isTTY bool, useMD bool) string {
	var sb strings.Builder

	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			if useMD {
				sb.WriteString(renderEntityMarkdown("ticket", tm) + "\n")
			} else {
				sb.WriteString("  " + renderEntityHuman("ticket", tm, false) + "\n")
			}
		}
	}
	return sb.String()
}
