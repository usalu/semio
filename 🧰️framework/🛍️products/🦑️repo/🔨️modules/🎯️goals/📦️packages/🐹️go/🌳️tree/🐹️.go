package goals

import (
	fmt "fmt"
	sort "sort"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌳️Tree

// 🌱️GoalSeed is the flat goal record the tree is assembled from.
type GoalSeed struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Status      string `json:"status"`
	DueDate     string `json:"dueDate"`
	CreatedAt   string `json:"createdAt"`
	Description string `json:"description"`
}

// 🎫️TicketSeed is the flat ticket record the tree is assembled from.
type TicketSeed struct {
	ID       string `json:"id"`
	Slug     string `json:"slug"`
	Status   string `json:"status"`
	Title    string `json:"title"`
	Goal     string `json:"goal"`
	Parent   string `json:"parent"`
	Prompt   string `json:"prompt"`
	Summary  string `json:"summary"`
	Created  string `json:"created"`
	Finished string `json:"finished"`
}

// 🖨️TreeFormat states how a rendered tree lays its lines out.
type TreeFormat int

// 🌲️TreeFormatText draws box connectors, one indentation step per level.
const TreeFormatText TreeFormat = 0

// 📝️TreeFormatMarkdown draws markdown list items, two spaces per level.
const TreeFormatMarkdown TreeFormat = 1

// 🖋️TreeLines renders the content of one tree line.
type TreeLines interface {
	// 🎯️Goal renders the line of a goal node.
	Goal(node *model.GoalNode) string
	// 🎫️Ticket renders the line of a ticket node.
	Ticket(node *model.TicketNode) string
}

// 🔤️PlainTreeLines states the identifying members and nothing else.
type PlainTreeLines struct{}

// 🎯️Goal renders a goal node as its identifying members.
func (PlainTreeLines) Goal(node *model.GoalNode) string {
	return fmt.Sprintf("goal %s %s [%s] %s", node.ID, node.Title, node.Status, node.DueDate)
}

// 🎫️Ticket renders a ticket node as its identifying members.
func (PlainTreeLines) Ticket(node *model.TicketNode) string {
	return fmt.Sprintf("ticket %s %s [%s] %s", node.Slug, node.Title, node.Status, node.URI)
}

// 🌳️BuildGoalTree assembles the goal forest out of goal and ticket seeds.
func BuildGoalTree(goalSeeds []GoalSeed, ticketSeeds []TicketSeed) []*model.GoalNode {
	nodes := map[string]*model.GoalNode{}
	for _, seed := range goalSeeds {
		nodes[seed.ID] = &model.GoalNode{
			ID:          seed.ID,
			Title:       seed.Title,
			Status:      seed.Status,
			DueDate:     seed.DueDate,
			CreatedAt:   seed.CreatedAt,
			Description: seed.Description,
		}
	}
	known := make([]string, 0, len(nodes))
	for id := range nodes {
		known = append(known, id)
	}
	sort.Strings(known)

	ticketNodes := make([]*model.TicketNode, 0, len(ticketSeeds))
	for _, seed := range ticketSeeds {
		ticketNodes = append(ticketNodes, &model.TicketNode{
			ID:          seed.ID,
			Slug:        seed.Slug,
			Status:      seed.Status,
			Title:       seed.Title,
			URI:         "repo://ticket/" + identity.EmojiText(identity.Entity("ticket")) + workspace.Flat(seed.Slug),
			GoalID:      seed.Goal,
			ParentID:    seed.Parent,
			Created:     seed.Created,
			Finished:    seed.Finished,
			Description: seed.Prompt,
			Summary:     seed.Summary,
		})
	}

	perGoal := map[string][]*model.TicketNode{}
	orphans := make([]*model.TicketNode, 0)
	for _, node := range ticketNodes {
		if node.GoalID != "" && containsString(known, node.GoalID) {
			perGoal[node.GoalID] = append(perGoal[node.GoalID], node)
		} else {
			orphans = append(orphans, node)
		}
	}
	for id, owned := range perGoal {
		if node, found := nodes[id]; found {
			node.Tickets = nestTickets(owned)
		}
	}

	childIDs := map[string][]string{}
	rootIDs := make([]string, 0)
	for _, id := range known {
		parent := ""
		if index := strings.LastIndex(id, "/"); index != -1 {
			parent = id[:index]
		}
		if parent != "" && parent != id && containsString(known, parent) {
			childIDs[parent] = append(childIDs[parent], id)
		} else {
			rootIDs = append(rootIDs, id)
		}
	}
	roots := make([]*model.GoalNode, 0, len(rootIDs))
	for _, id := range rootIDs {
		if node := assembleGoalNode(id, nodes, childIDs); node != nil {
			roots = append(roots, node)
		}
	}
	sortGoalNodes(roots)

	if len(orphans) > 0 {
		roots = append(roots, &model.GoalNode{Title: "No Goal", Tickets: nestTickets(orphans)})
	}
	return roots
}

// 🔎️containsString reports whether a slice carries a value.
func containsString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🧩️assembleGoalNode copies a goal node together with everything below it.
func assembleGoalNode(id string, nodes map[string]*model.GoalNode, childIDs map[string][]string) *model.GoalNode {
	source, found := nodes[id]
	if !found {
		return nil
	}
	node := *source
	children := make([]*model.GoalNode, 0)
	for _, child := range childIDs[id] {
		if assembled := assembleGoalNode(child, nodes, childIDs); assembled != nil {
			children = append(children, assembled)
		}
	}
	if len(children) == 0 {
		node.Children = nil
	} else {
		node.Children = children
	}
	return &node
}

// 🧬️nestTickets hangs every ticket under the ticket it names.
func nestTickets(nodes []*model.TicketNode) []*model.TicketNode {
	ids := make([]string, 0, len(nodes))
	byID := map[string]*model.TicketNode{}
	for _, node := range nodes {
		ids = append(ids, node.ID)
		byID[node.ID] = node
	}
	childIDs := map[string][]string{}
	rootIDs := make([]string, 0)
	for _, node := range nodes {
		if node.ParentID != "" && containsString(ids, node.ParentID) && node.ParentID != node.ID {
			childIDs[node.ParentID] = append(childIDs[node.ParentID], node.ID)
		} else {
			rootIDs = append(rootIDs, node.ID)
		}
	}
	roots := make([]*model.TicketNode, 0, len(rootIDs))
	for _, id := range rootIDs {
		if assembled := assembleTicketNode(id, byID, childIDs); assembled != nil {
			roots = append(roots, assembled)
		}
	}
	return roots
}

// 🧩️assembleTicketNode copies a ticket node together with everything below it.
func assembleTicketNode(id string, nodes map[string]*model.TicketNode, childIDs map[string][]string) *model.TicketNode {
	source, found := nodes[id]
	if !found {
		return nil
	}
	node := *source
	children := make([]*model.TicketNode, 0)
	for _, child := range childIDs[id] {
		if assembled := assembleTicketNode(child, nodes, childIDs); assembled != nil {
			children = append(children, assembled)
		}
	}
	if len(children) == 0 {
		node.Children = nil
	} else {
		node.Children = children
	}
	return &node
}

// 🔢️sortGoalNodes orders siblings by due date and then by identifier, a blank due date last.
func sortGoalNodes(nodes []*model.GoalNode) {
	sort.SliceStable(nodes, func(left, right int) bool {
		first, second := nodes[left], nodes[right]
		if first.DueDate != second.DueDate {
			if first.DueDate == "" {
				return false
			}
			if second.DueDate == "" {
				return true
			}
			return first.DueDate < second.DueDate
		}
		return first.ID < second.ID
	})
	for _, node := range nodes {
		sortGoalNodes(node.Children)
	}
}

// 🔢️CountOpenSubgoals returns how many open goals hang below a node, at any depth.
func CountOpenSubgoals(node *model.GoalNode) int {
	total := 0
	for _, child := range node.Children {
		if child.Status == "open" {
			total++
		}
		total += CountOpenSubgoals(child)
	}
	return total
}

// 🔢️CountOpenTickets returns how many open tickets hang below a node, at any depth.
func CountOpenTickets(node *model.GoalNode) int {
	total := countOpenTicketNodes(node.Tickets)
	for _, child := range node.Children {
		total += CountOpenTickets(child)
	}
	return total
}

// 🔢️countOpenTicketNodes counts the open tickets of a ticket forest.
func countOpenTicketNodes(nodes []*model.TicketNode) int {
	total := 0
	for _, node := range nodes {
		if node.Status == "open" {
			total++
		}
		total += countOpenTicketNodes(node.Children)
	}
	return total
}

// 🎨️RenderGoalTree renders a goal forest, goals first and their tickets after them at every level.
func RenderGoalTree(roots []*model.GoalNode, format TreeFormat, lines TreeLines) string {
	var out strings.Builder
	last := len(roots) - 1
	for index, root := range roots {
		renderGoalNode(root, "", index == last, true, format, lines, &out)
	}
	return out.String()
}

// 🎯️renderGoalNode renders one goal node and everything below it.
func renderGoalNode(node *model.GoalNode, prefix string, isLast, isRoot bool, format TreeFormat, lines TreeLines, out *strings.Builder) {
	content := lines.Goal(node)
	total := len(node.Children) + len(node.Tickets)
	next := writeTreeLine(out, prefix, content, isLast, isRoot, format)
	for index, child := range node.Children {
		renderGoalNode(child, next, index == total-1, false, format, lines, out)
	}
	for index, ticket := range node.Tickets {
		renderTicketNode(ticket, next, len(node.Children)+index == total-1, format, lines, out)
	}
}

// 🎫️renderTicketNode renders one ticket node and everything below it.
func renderTicketNode(node *model.TicketNode, prefix string, isLast bool, format TreeFormat, lines TreeLines, out *strings.Builder) {
	content := lines.Ticket(node)
	next := writeTreeLine(out, prefix, content, isLast, false, format)
	last := len(node.Children) - 1
	for index, child := range node.Children {
		renderTicketNode(child, next, index == last, format, lines, out)
	}
}

// 🖊️writeTreeLine writes one rendered line and returns the prefix of everything below it.
func writeTreeLine(out *strings.Builder, prefix, content string, isLast, isRoot bool, format TreeFormat) string {
	if format == TreeFormatMarkdown {
		out.WriteString(prefix)
		out.WriteString("- ")
		out.WriteString(content)
		out.WriteString("\n")
		return prefix + "  "
	}
	connector := "├️─️─️ "
	switch {
	case isRoot:
		connector = ""
	case isLast:
		connector = "└️─️─️ "
	}
	out.WriteString(prefix)
	out.WriteString(connector)
	out.WriteString(content)
	out.WriteString("\n")
	switch {
	case isRoot:
		return prefix
	case isLast:
		return prefix + "    "
	}
	return prefix + "│️   "
}

// #endregion 🌳️Tree
