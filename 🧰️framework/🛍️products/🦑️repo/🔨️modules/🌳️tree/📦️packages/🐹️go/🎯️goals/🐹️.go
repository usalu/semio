// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎯️goals carries the ported goal tree: goal and ticket nodes, nesting, sorting and rendering.

// #endregion 🧲️Header

package tree

import (
	sort "sort"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
)

// #region 🎯️PortedGoalTree

// 🎯️GoalNode is one goal of the goal tree, with its subgoals and tickets.
type GoalNode struct {
	ID          string       `json:"id"`
	Title       string       `json:"title"`
	Status      string       `json:"status"`
	DueDate     string       `json:"dueDate"`
	CreatedAt   string       `json:"createdAt"`
	Description string       `json:"description"`
	Children    []GoalNode   `json:"children"`
	Tickets     []TicketNode `json:"tickets"`
}

// 🎫️TicketNode is one ticket of the goal tree, with its child tickets.
type TicketNode struct {
	ID          string       `json:"id"`
	Slug        string       `json:"slug"`
	Status      string       `json:"status"`
	Title       string       `json:"title"`
	URI         string       `json:"uri"`
	GoalID      string       `json:"goalId"`
	ParentID    string       `json:"parentId"`
	Children    []TicketNode `json:"children"`
	Created     string       `json:"created"`
	Finished    string       `json:"finished"`
	Description string       `json:"description"`
	Summary     string       `json:"summary"`
}

// 🎯️BuildGoalTree MUST nest subgoals by id path and tickets by parent.
func BuildGoalTree(goals []GoalRecord, tickets []TicketRecord) []GoalNode {
	nodes := map[string]*GoalNode{}
	for _, goal := range goals {
		nodes[goal.ID] = &GoalNode{ID: goal.ID, Title: goal.Title, Status: goal.Status, DueDate: goal.DueDate, CreatedAt: goal.CreatedAt, Description: goal.Description}
	}
	orphans := []TicketNode{}
	for _, ticket := range tickets {
		node := TicketNode{
			ID: ticket.ID, Slug: ticket.Slug, Status: ticket.Status, Title: ticket.Title,
			URI: TicketURI(ticket), GoalID: ticket.Goal, ParentID: ticket.Parent,
			Created: ticket.Started, Finished: ticket.Finished, Description: ticket.Description, Summary: ticket.Summary,
		}
		if goal, ok := nodes[ticket.Goal]; ok && ticket.Goal != "" {
			goal.Tickets = append(goal.Tickets, node)
			continue
		}
		orphans = append(orphans, node)
	}
	for _, node := range nodes {
		node.Tickets = nestTickets(node.Tickets)
	}
	ids := []string{}
	parents := []string{}
	values := []*GoalNode{}
	for _, goal := range goals {
		node, ok := nodes[goal.ID]
		if !ok {
			continue
		}
		delete(nodes, goal.ID)
		ids = append(ids, goal.ID)
		parents = append(parents, GoalParentID(goal.ID))
		values = append(values, node)
	}
	result := nestOrderedGoals(ids, parents, values)
	SortGoalNodes(result)
	if len(orphans) > 0 {
		result = append(result, GoalNode{Title: NoGoalLabel, Tickets: nestTickets(orphans)})
	}
	return result
}

// 🔗️TicketURI MUST return the declared uri, or build the repository's own.
func TicketURI(ticket TicketRecord) string {
	if ticket.URI != "" {
		return ticket.URI
	}
	return "repo://ticket/" + identity.EmojiText(identity.Entity("ticket")) + identity.Flat(ticket.Slug)
}

// 🪜️nestTickets nests tickets under their parent ticket.
func nestTickets(tickets []TicketNode) []TicketNode {
	if len(tickets) == 0 {
		return nil
	}
	positions := map[string]int{}
	for index, ticket := range tickets {
		if _, seen := positions[ticket.ID]; !seen {
			positions[ticket.ID] = index
		}
	}
	roots := []int{}
	children := map[int][]int{}
	taken := make([]bool, len(tickets))
	for index, ticket := range tickets {
		parentIndex, ok := positions[ticket.ParentID]
		if ok && ticket.ParentID != ticket.ID && parentIndex != index {
			children[parentIndex] = append(children[parentIndex], index)
			continue
		}
		roots = append(roots, index)
	}
	var assemble func(index int) (TicketNode, bool)
	assemble = func(index int) (TicketNode, bool) {
		if taken[index] {
			return TicketNode{}, false
		}
		taken[index] = true
		node := tickets[index]
		node.Children = nil
		for _, child := range children[index] {
			if assembled, ok := assemble(child); ok {
				node.Children = append(node.Children, assembled)
			}
		}
		return node, true
	}
	out := []TicketNode{}
	for _, index := range roots {
		if assembled, ok := assemble(index); ok {
			out = append(out, assembled)
		}
	}
	if len(out) == 0 {
		return nil
	}
	return out
}

// 🪜️nestOrderedGoals nests goals under the goal carrying their parent id.
func nestOrderedGoals(ids []string, parents []string, values []*GoalNode) []GoalNode {
	positions := map[string]int{}
	for index, id := range ids {
		if _, seen := positions[id]; !seen {
			positions[id] = index
		}
	}
	roots := []int{}
	children := map[int][]int{}
	for index := range ids {
		parentIndex, ok := positions[parents[index]]
		if ok && parents[index] != ids[index] && parentIndex != index {
			children[parentIndex] = append(children[parentIndex], index)
			continue
		}
		roots = append(roots, index)
	}
	var assemble func(index int) (GoalNode, bool)
	assemble = func(index int) (GoalNode, bool) {
		if values[index] == nil {
			return GoalNode{}, false
		}
		node := *values[index]
		values[index] = nil
		for _, child := range children[index] {
			if assembled, ok := assemble(child); ok {
				node.Children = append(node.Children, assembled)
			}
		}
		return node, true
	}
	out := []GoalNode{}
	for _, index := range roots {
		if assembled, ok := assemble(index); ok {
			out = append(out, assembled)
		}
	}
	return out
}

// 📶️SortGoalNodes MUST sort goals by due date, blank dates last, ties broken by id.
func SortGoalNodes(goals []GoalNode) {
	sort.SliceStable(goals, func(i, j int) bool {
		left, right := goals[i], goals[j]
		if left.DueDate != right.DueDate {
			if left.DueDate == "" {
				return false
			}
			if right.DueDate == "" {
				return true
			}
			return left.DueDate < right.DueDate
		}
		return left.ID < right.ID
	})
	for index := range goals {
		SortGoalNodes(goals[index].Children)
	}
}

// 🧮️CountOpenSubgoals MUST count the open subgoals below a goal.
func CountOpenSubgoals(goal *GoalNode) int {
	count := 0
	for index := range goal.Children {
		child := &goal.Children[index]
		if child.Status == "open" {
			count++
		}
		count += CountOpenSubgoals(child)
	}
	return count
}

// 🧮️CountOpenTickets MUST count the open tickets below a goal, including every subgoal's.
func CountOpenTickets(goal *GoalNode) int {
	var walk func(tickets []TicketNode) int
	walk = func(tickets []TicketNode) int {
		count := 0
		for index := range tickets {
			if tickets[index].Status == "open" {
				count++
			}
			count += walk(tickets[index].Children)
		}
		return count
	}
	count := walk(goal.Tickets)
	for index := range goal.Children {
		count += CountOpenTickets(&goal.Children[index])
	}
	return count
}

// 💿️GoalNodeData MUST return the data map a goal line renders from.
func GoalNodeData(goal *GoalNode) map[string]interface{} {
	return dataOf("id", goal.ID, "title", goal.Title, "status", goal.Status, "dueDate", goal.DueDate, "createdAt", goal.CreatedAt, "description", goal.Description)
}

// 🌿️TicketNodeData MUST return the data map a ticket line renders from.
func TicketNodeData(ticket *TicketNode) map[string]interface{} {
	return dataOf("slug", ticket.Slug, "title", ticket.Title, "status", ticket.Status, "started", ticket.Created, "finished", ticket.Finished, "prompt", ticket.Description, "summary", ticket.Summary)
}

// 🖨️TreeRenderFormat is the output format of a goal tree rendering.
type TreeRenderFormat string

// 🖥️TreeRenderFormatText is the indented text form with box-drawing connectors.
const TreeRenderFormatText TreeRenderFormat = "text"

// 📰️TreeRenderFormatMarkdown is the markdown list form.
const TreeRenderFormatMarkdown TreeRenderFormat = "md"

// ⛳️RenderGoalTreeNodes MUST render one line per goal and ticket.
func RenderGoalTreeNodes(roots []GoalNode, format TreeRenderFormat, renderer EntityRenderer) string {
	var out strings.Builder
	for index := range roots {
		renderGoalNode(&out, &roots[index], "", index == len(roots)-1, true, format, renderer)
	}
	return out.String()
}

// 🎯️renderGoalNode renders one goal and everything below it.
func renderGoalNode(out *strings.Builder, goal *GoalNode, prefix string, isLast bool, isRoot bool, format TreeRenderFormat, renderer EntityRenderer) {
	data := GoalNodeData(goal)
	line := renderer.Human("goal", data)
	if format == TreeRenderFormatMarkdown {
		line = renderer.MarkdownLink("goal", data)
	}
	total := len(goal.Children) + len(goal.Tickets)
	newPrefix := writeTreeLine(out, prefix, line, isLast, isRoot, format)
	for index := range goal.Children {
		renderGoalNode(out, &goal.Children[index], newPrefix, index == total-1, false, format, renderer)
	}
	for index := range goal.Tickets {
		renderTicketNode(out, &goal.Tickets[index], newPrefix, len(goal.Children)+index == total-1, false, format, renderer)
	}
}

// 🎫️renderTicketNode renders one ticket and everything below it.
func renderTicketNode(out *strings.Builder, ticket *TicketNode, prefix string, isLast bool, isRoot bool, format TreeRenderFormat, renderer EntityRenderer) {
	data := TicketNodeData(ticket)
	line := renderer.Human("ticket", data)
	if format == TreeRenderFormatMarkdown {
		line = renderer.MarkdownLink("ticket", data)
	}
	newPrefix := writeTreeLine(out, prefix, line, isLast, isRoot, format)
	for index := range ticket.Children {
		renderTicketNode(out, &ticket.Children[index], newPrefix, index == len(ticket.Children)-1, false, format, renderer)
	}
}

// ✏️writeTreeLine writes one rendered line and returns the prefix its children carry.
func writeTreeLine(out *strings.Builder, prefix string, line string, isLast bool, isRoot bool, format TreeRenderFormat) string {
	if format == TreeRenderFormatMarkdown {
		out.WriteString(prefix + "- " + line + "\n")
		return prefix + "  "
	}
	connector := TextBranchConnector
	if isRoot {
		connector = ""
	} else if isLast {
		connector = TextLastConnector
	}
	out.WriteString(prefix + connector + line + "\n")
	if isRoot {
		return prefix
	}
	if isLast {
		return prefix + TextBlankIndent
	}
	return prefix + TextPipeIndent
}

// #endregion 🎯️PortedGoalTree
