// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌿️nodes carries the tree node helpers and the ported monorepo tree builders.

// #endregion 🧲️Header

package tree

import (
	sort "sort"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
)

// #region 🌿️NodeHelpers

// 🌿️NewTreeNode MUST return an empty node of one kind.
func NewTreeNode(kind TreeNodeKind, id string, label string, uri string) *TreeNode {
	return &TreeNode{Kind: kind, ID: id, Label: label, URI: uri}
}

// ➕️PushChild MUST append one child to a node.
func PushChild(node *TreeNode, child *TreeNode) {
	node.Children = append(node.Children, child)
}

// 🗃️dataOf builds a data map from ordered pairs.
func dataOf(pairs ...interface{}) map[string]interface{} {
	out := make(map[string]interface{}, len(pairs)/2)
	for index := 0; index+1 < len(pairs); index += 2 {
		key, _ := pairs[index].(string)
		out[key] = pairs[index+1]
	}
	return out
}

// 🔢️number wraps a count the way the record projections carry it.
func number(value int) interface{} { return float64(value) }

// 📃️list wraps a string list as a JSON-compatible value.
func list(values []string) interface{} {
	out := make([]interface{}, 0, len(values))
	for _, value := range values {
		out = append(out, value)
	}
	return out
}

// 📁️ParentPath MUST return the parent of a slash-separated path, or `.` when there is none.
func ParentPath(path string) string {
	index := strings.LastIndex(path, "/")
	switch {
	case index == 0:
		return "/"
	case index > 0:
		return path[:index]
	}
	return "."
}

// 📄️BaseName MUST return the last segment of a slash-separated path.
func BaseName(path string) string {
	if index := strings.LastIndex(path, "/"); index >= 0 {
		return path[index+1:]
	}
	return path
}

// 🪜️nestOrderedNodes nests nodes under the node carrying their parent id, preserving input order.
func nestOrderedNodes(ids []string, parents []string, values []*TreeNode) []*TreeNode {
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
	var assemble func(index int) *TreeNode
	assemble = func(index int) *TreeNode {
		node := values[index]
		if node == nil {
			return nil
		}
		values[index] = nil
		for _, child := range children[index] {
			if assembled := assemble(child); assembled != nil {
				PushChild(node, assembled)
			}
		}
		return node
	}
	out := []*TreeNode{}
	for _, index := range roots {
		if assembled := assemble(index); assembled != nil {
			out = append(out, assembled)
		}
	}
	return out
}

// 🎨️PriorityIcon MUST return the icon of a priority.
func PriorityIcon(priority model.BreachPriority) string {
	switch priority {
	case model.BreachPriorityHigh:
		return PriorityIconHigh
	case model.BreachPriorityMedium:
		return PriorityIconMedium
	}
	return PriorityIconLow
}

// #endregion 🌿️NodeHelpers

// #region 🩻️PortedMonorepoTree

// 🏢️BuildMonorepoTree MUST assemble the monorepo tree from the records a TreeSource supplies.
func BuildMonorepoTree(source TreeSource, options TreeBuildOptions) *TreeNode {
	root := NewTreeNode(TreeNodeCategory, "", ".", "")
	root.Children = []*TreeNode{}
	PushChild(root, buildCodebaseNode(source, options))
	PushChild(root, buildGoalsNode(source))
	PushChild(root, buildDraftsNode(source))
	PushChild(root, buildPoliciesNode(source))
	PushChild(root, buildContributorsNode(source))
	PushChild(root, buildCheckpointsNode(source))
	PushChild(root, buildSessionsNode(source))
	return root
}

// 🏢️BuildMonorepoTreeWithIDs MUST stamp every node with its parent artifact id.
func BuildMonorepoTreeWithIDs(source TreeSource, options TreeBuildOptions, identifier ArtifactIdentifier) *TreeNode {
	root := BuildMonorepoTree(source, options)
	PropagateParentIDs(root, "", identifier)
	return root
}

// 📄️buildFileNode projects one file record, with its sections when they are requested.
func buildFileNode(source TreeSource, file FileRecord, options TreeBuildOptions) *TreeNode {
	node := NewTreeNode(TreeNodeFile, file.ID, file.Name, file.URI)
	node.SubKind = file.Kind
	node.Data = dataOf("path", file.Path, "name", file.Name, "kind", file.Kind, "parentId", file.ParentID)
	if options.IncludeSections {
		for _, section := range source.Sections(file.Path) {
			PushChild(node, BuildSectionTreeNodeRecord(section))
		}
	}
	return node
}

// 📁️buildFolderNode projects one folder record.
func buildFolderNode(folder FolderRecord) *TreeNode {
	node := NewTreeNode(TreeNodeFolder, folder.ID, folder.Name, folder.URI)
	node.SubKind = folder.Kind
	node.Data = dataOf("path", folder.Path, "name", folder.Name, "kind", folder.Kind, "parentId", folder.ParentID)
	return node
}

// 📑️BuildSectionTreeNodeRecord MUST project one section record with definitions and children.
func BuildSectionTreeNodeRecord(section SectionRecord) *TreeNode {
	node := NewTreeNode(TreeNodeSection, section.ID, section.Name, section.URI)
	node.Data = dataOf("path", section.Path, "name", section.Name, "startLine", number(section.StartLine), "endLine", number(section.EndLine))
	for _, definition := range section.Definitions {
		definitionNode := NewTreeNode(TreeNodeDefinition, definition.ID, definition.Name, definition.URI)
		definitionNode.SubKind = definition.Kind
		definitionNode.Data = dataOf("path", definition.FilePath, "name", definition.Name, "kind", definition.Kind, "startLine", number(definition.StartLine), "endLine", number(definition.EndLine))
		PushChild(node, definitionNode)
	}
	for _, child := range section.Children {
		PushChild(node, BuildSectionTreeNodeRecord(child))
	}
	return node
}

// 📁️BuildFolderRoots MUST nest every folder under its parent folder.
func BuildFolderRoots(folders []FolderRecord) []*TreeNode {
	ids := make([]string, 0, len(folders))
	parents := make([]string, 0, len(folders))
	values := make([]*TreeNode, 0, len(folders))
	for _, folder := range folders {
		ids = append(ids, folder.Path)
		parents = append(parents, ParentPath(folder.Path))
		values = append(values, buildFolderNode(folder))
	}
	return nestOrderedNodes(ids, parents, values)
}

// 📄️AttachFilesToFolders MUST attach every file node under the folder node of its directory.
func AttachFilesToFolders(root *TreeNode, files []FileRecord, fileNodes map[string]*TreeNode) {
	for _, file := range files {
		node, ok := fileNodes[file.Path]
		if !ok {
			continue
		}
		if !attachIntoFolder(root, ParentPath(file.Path), node) {
			PushChild(root, cloneTreeNode(node))
		}
	}
}

// 🔎️attachIntoFolder attaches a node under the folder node carrying a path.
func attachIntoFolder(node *TreeNode, folderPath string, fileNode *TreeNode) bool {
	if node.Kind == TreeNodeFolder && node.Data != nil {
		if path, ok := node.Data["path"].(string); ok && path != "" && path == folderPath {
			PushChild(node, cloneTreeNode(fileNode))
			return true
		}
	}
	for _, child := range node.Children {
		if attachIntoFolder(child, folderPath, fileNode) {
			return true
		}
	}
	return false
}

// 🗂️buildCodebaseNode projects the folder/file hierarchy plus the technology/bundle view.
func buildCodebaseNode(source TreeSource, options TreeBuildOptions) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "codebase", identity.EmojiText(identity.Collection("codebase"))+"Codebase", "repo://codebase")
	folders := source.Folders()
	files := source.Files()
	fileNodes := map[string]*TreeNode{}
	for _, file := range files {
		fileNodes[file.Path] = buildFileNode(source, file, options)
	}
	folderNodes := map[string]*TreeNode{}
	for _, folder := range folders {
		folderNodes[folder.Path] = buildFolderNode(folder)
	}
	for _, folderRoot := range BuildFolderRoots(folders) {
		PushChild(node, folderRoot)
	}
	AttachFilesToFolders(node, files, fileNodes)
	technologies := append([]TechnologyRecord{}, source.Technologies()...)
	sort.SliceStable(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })
	for _, technology := range technologies {
		technologyNode := NewTreeNode(TreeNodeTechnology, technology.ID, technology.Name, technology.URI)
		technologyNode.SubKind = technology.Kind
		technologyNode.Data = dataOf("name", technology.Name, "kind", technology.Kind, "emoji", technology.Emoji)
		bundles := append([]BundleRecord{}, technology.Bundles...)
		sort.SliceStable(bundles, func(i, j int) bool { return bundles[i].Name < bundles[j].Name })
		for _, bundle := range bundles {
			PushChild(technologyNode, buildBundleNode(bundle, folderNodes, fileNodes))
		}
		PushChild(node, technologyNode)
	}
	SortTreeChildren(node)
	return node
}

// 📦️buildBundleNode projects one bundle: every folder and file below its source root.
func buildBundleNode(bundle BundleRecord, folderNodes map[string]*TreeNode, fileNodes map[string]*TreeNode) *TreeNode {
	node := NewTreeNode(TreeNodeBundle, bundle.ID, bundle.Name, bundle.URI)
	node.SubKind = bundle.Kind
	node.Data = dataOf("name", bundle.Name, "root", bundle.Root, "kind", bundle.Kind, "emoji", bundle.Emoji)
	bundleRoot := bundle.SourceRoot
	if bundleRoot == "" {
		bundleRoot = bundle.Root
	}
	owned := map[string]*TreeNode{}
	ownedPaths := []string{}
	for path, folderNode := range folderNodes {
		if isUnder(path, bundleRoot) {
			owned[path] = cloneTreeNode(folderNode)
			ownedPaths = append(ownedPaths, path)
		}
	}
	sort.Strings(ownedPaths)
	filePaths := make([]string, 0, len(fileNodes))
	for path := range fileNodes {
		filePaths = append(filePaths, path)
	}
	sort.Strings(filePaths)
	for _, path := range filePaths {
		if !isUnder(path, bundleRoot) {
			continue
		}
		if folderNode, ok := owned[ParentPath(path)]; ok {
			PushChild(folderNode, cloneTreeNode(fileNodes[path]))
			continue
		}
		PushChild(node, cloneTreeNode(fileNodes[path]))
	}
	ids := make([]string, 0, len(ownedPaths))
	parents := make([]string, 0, len(ownedPaths))
	values := make([]*TreeNode, 0, len(ownedPaths))
	for _, path := range ownedPaths {
		ids = append(ids, path)
		parents = append(parents, ParentPath(path))
		values = append(values, owned[path])
	}
	for _, folderRoot := range nestOrderedNodes(ids, parents, values) {
		PushChild(node, folderRoot)
	}
	SortTreeChildren(node)
	return node
}

// 📏️isUnder reports whether a path is the given root or lies below it.
func isUnder(path string, root string) bool {
	return path == root || strings.HasPrefix(path, root+"/")
}

// 🎯️buildGoalsNode projects the goals category, nesting subgoals and attaching tickets.
func buildGoalsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "goals", "🎯️Goals", "repo://goals")
	goals := append([]GoalRecord{}, source.GoalRecords()...)
	sort.SliceStable(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	goalNodes := map[string]*TreeNode{}
	for _, goal := range goals {
		artifactID := goal.ArtifactID
		if artifactID == "" {
			artifactID = goal.ID
		}
		goalNode := NewTreeNode(TreeNodeGoal, artifactID, goal.Title, goal.URI)
		goalNode.SubKind = goal.Status
		goalNode.Description = goal.Description
		goalNode.Status = goal.Status
		goalNode.Data = dataOf("id", goal.ID, "title", goal.Title, "status", goal.Status, "dueDate", goal.DueDate, "createdAt", "", "description", goal.Description)
		goalNodes[goal.ID] = goalNode
	}
	goalTickets := map[string][]*TreeNode{}
	looseTickets := []*TreeNode{}
	for _, ticket := range source.TicketRecords() {
		ticketNode := buildTicketTreeNode(ticket)
		if _, ok := goalNodes[ticket.Goal]; ok && ticket.Goal != "" {
			goalTickets[ticket.Goal] = append(goalTickets[ticket.Goal], ticketNode)
			continue
		}
		looseTickets = append(looseTickets, ticketNode)
	}
	roots := []string{}
	subgoals := map[string][]string{}
	for _, goal := range goals {
		parent := GoalParentID(goal.ID)
		if _, ok := goalNodes[parent]; parent != "" && parent != goal.ID && ok {
			subgoals[parent] = append(subgoals[parent], goal.ID)
			continue
		}
		roots = append(roots, goal.ID)
	}
	var assemble func(id string) *TreeNode
	assemble = func(id string) *TreeNode {
		current, ok := goalNodes[id]
		if !ok {
			return nil
		}
		delete(goalNodes, id)
		for _, childID := range subgoals[id] {
			if child := assemble(childID); child != nil {
				PushChild(current, child)
			}
		}
		for _, ticket := range goalTickets[id] {
			PushChild(current, ticket)
		}
		delete(goalTickets, id)
		return current
	}
	children := []*TreeNode{}
	for _, id := range roots {
		if assembled := assemble(id); assembled != nil {
			children = append(children, assembled)
		}
	}
	children = append(children, looseTickets...)
	node.Children = children
	return node
}

// 🎫️buildTicketTreeNode projects one ticket record as a monorepo tree node.
func buildTicketTreeNode(ticket TicketRecord) *TreeNode {
	node := NewTreeNode(TreeNodeTicket, ticket.ID, ticket.Title, ticket.URI)
	node.Description = ticket.Description
	node.Year = ticket.Year
	node.Month = ticket.Month
	node.Day = ticket.Day
	node.Status = ticket.Status
	node.Data = dataOf(
		"year", number(ticket.Year), "month", number(ticket.Month), "day", number(ticket.Day),
		"slug", ticket.Slug, "title", ticket.Title, "status", ticket.Status,
		"started", ticket.Started, "finished", ticket.Finished,
		"prompt", ticket.Description, "summary", ticket.Summary, "goalId", ticket.Goal)
	return node
}

// 🧬️GoalParentID MUST return the parent goal id of a slash-separated goal id.
func GoalParentID(id string) string {
	if index := strings.LastIndex(id, "/"); index >= 0 {
		return id[:index]
	}
	return ""
}

// ✍️buildDraftsNode projects the drafts category.
func buildDraftsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "drafts", "✍️Drafts", "repo://drafts")
	for _, draft := range source.Drafts() {
		artifactID := draft.ArtifactID
		if artifactID == "" {
			artifactID = draft.ID
		}
		draftNode := NewTreeNode(TreeNodeDraft, artifactID, draft.ID, draft.URI)
		draftNode.Data = dataOf("id", draft.ID, "slug", draft.ID)
		PushChild(node, draftNode)
	}
	return node
}

// 🛡️buildPoliciesNode projects the policies category.
func buildPoliciesNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "policies", "🛡️Policies", "repo://policies")
	policies := append([]PolicyRecord{}, source.Policies()...)
	sort.SliceStable(policies, func(i, j int) bool { return policies[i].ID < policies[j].ID })
	policyEmoji := identity.EmojiText(identity.Entity("policy"))
	for _, policy := range policies {
		flat := identity.Flat(strings.TrimLeft(policy.ID, "/"))
		policyNode := NewTreeNode(TreeNodePolicy, policyEmoji+"/"+policy.ID, policy.Name, "repo://policy/"+policyEmoji+flat)
		policyNode.Description = policy.Description
		policyNode.Data = dataOf("id", policy.ID, "name", policy.Name, "description", policy.Description)
		PushChild(node, policyNode)
	}
	return node
}

// 🧑️buildContributorsNode projects the contributors category.
func buildContributorsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "contributors", "🧑️‍💻️Contributors", "repo://contributors")
	contributors := append([]ContributorRecord{}, source.Contributors()...)
	sort.SliceStable(contributors, func(i, j int) bool { return contributors[i].Alias < contributors[j].Alias })
	for _, contributor := range contributors {
		label := contributor.Alias
		if contributor.Name != "" {
			label = contributor.Name + " (" + contributor.Alias + ")"
		}
		contributorNode := NewTreeNode(TreeNodeContributor, contributor.ID, label, contributor.URI)
		contributorNode.Contributor = contributor.Alias
		contributorNode.Data = dataOf(
			"alias", contributor.Alias, "aliases", list(contributor.Aliases),
			"github", contributor.Github, "githubs", list(contributor.Githubs),
			"name", contributor.Name, "names", list(contributor.Names),
			"email", contributor.Email, "emails", list(contributor.Emails))
		PushChild(node, contributorNode)
	}
	return node
}

// 🔀️buildCheckpointsNode projects the checkpoints category.
func buildCheckpointsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "checkpoints", "🔀️Checkpoints", "repo://checkpoints")
	for _, checkpoint := range source.Checkpoints() {
		short := []rune(checkpoint.Sha)
		if len(short) > 8 {
			short = short[:8]
		}
		checkpointNode := NewTreeNode(TreeNodeCheckpoint, checkpoint.ID, string(short)+" "+checkpoint.Title, checkpoint.URI)
		checkpointNode.Year = checkpoint.Year
		checkpointNode.Month = checkpoint.Month
		checkpointNode.Day = checkpoint.Day
		checkpointNode.Data = dataOf("sha", checkpoint.Sha, "message", checkpoint.Title, "authorId", checkpoint.AuthorID)
		PushChild(node, checkpointNode)
	}
	return node
}

// ⚪️buildSessionsNode projects the sessions category, newest first.
func buildSessionsNode(source TreeSource) *TreeNode {
	node := NewTreeNode(TreeNodeCategory, "sessions", "⚪️Sessions", "repo://sessions")
	sessions := append([]SessionRecord{}, source.Sessions()...)
	sort.SliceStable(sessions, func(i, j int) bool { return sessions[i].StartedAt > sessions[j].StartedAt })
	for _, session := range sessions {
		label := session.KindEmoji + " " + session.UUID
		if session.Client != "" {
			label += " (" + session.Client + ")"
		}
		sessionNode := NewTreeNode(TreeNodeSession, session.ID, label, session.URI)
		sessionNode.SubKind = session.Kind
		sessionNode.Year = session.Year
		sessionNode.Month = session.Month
		sessionNode.Day = session.Day
		sessionNode.Status = session.Kind
		sessionNode.Data = dataOf(
			"uuid", session.UUID, "kind", session.Kind, "client", session.Client,
			"startedAt", session.StartedAt, "year", number(session.Year), "month", number(session.Month),
			"day", number(session.Day), "checkpoint", session.Checkpoint)
		PushChild(node, sessionNode)
	}
	return node
}

// #endregion 🩻️PortedMonorepoTree
