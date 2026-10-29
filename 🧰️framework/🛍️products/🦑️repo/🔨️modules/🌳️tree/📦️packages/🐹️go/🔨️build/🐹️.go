// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔨️build carries the monorepo tree builder over the repo and the statute and territory tree builders it uses.

// #endregion 🧲️Header

package tree

import (
	context "context"
	"errors"
	fmt "fmt"
	search "github.com/usalu/semio/repo/search"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
	sync "sync"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	contributorspkg "github.com/usalu/semio/repo/contributors"
	goalspkg "github.com/usalu/semio/repo/goals"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutespkg "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🩻️Monorepo Tree

// 🌳️TreeBuildOptions holds the data fields for a tree build options record.
type TreeBuildOptions struct {
	IncludeSections bool
	Progress        func(search.Progress) `json:"-"`
}

// 🏢️BuildMonorepoTree MUST assemble the monorepo tree from the available context data.
// 🏢️BuildMonorepoTree constructs and returns the monorepo tree structure.
func BuildMonorepoTreeFromRepo(ctx context.Context, opts ...TreeBuildOptions) (*TreeNode, error) {
	var options TreeBuildOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	codebase, err := buildRepositoryCodebase(ctx, options)
	if err != nil {
		return nil, err
	}
	return buildRepositoryDomains(ctx, codebase, options)
}

// 🌲️buildRepositoryCodebase projects authored code independently of domain records.
func buildRepositoryCodebase(ctx context.Context, options TreeBuildOptions) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	reportSearchProgress(options.Progress, search.Progress{Step: "codebase-discovery"})
	var technologies []model.Technology
	var allFolders []model.Folder
	var allFiles []model.File
	codebase.InvalidateTechnologyCache()
	technologies = codebase.LoadTechnologies()
	boundaries := []model.ArtifactBoundary{}
	for _, technology := range technologies {
		for _, bundle := range technology.Bundles {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
	}
	layout := model.NewArtifactLayout(workspace.GetRootDir(), boundaries)
	streamOptions := model.StreamOptions{Layout: layout, Progress: func(value model.StreamProgress) {
		reportSearchProgress(options.Progress, search.Progress{Current: value.Current, Step: value.Phase})
	}}
	var failuresMu sync.Mutex
	var failures []error
	fail := func(err error) {
		if err != nil {
			failuresMu.Lock()
			failures = append(failures, err)
			failuresMu.Unlock()
		}
	}
	var wg sync.WaitGroup
	wg.Add(2)
	go func() {
		defer wg.Done()
		entries := make(chan model.Folder)
		completed := make(chan error, 1)
		go func() { completed <- ticketspkg.StreamFolders(ctx, "", entries, streamOptions) }()
		for entry := range entries {
			allFolders = append(allFolders, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan model.File)
		completed := make(chan error, 1)
		go func() { completed <- ticketspkg.StreamFiles(ctx, "", entries, streamOptions) }()
		for entry := range entries {
			allFiles = append(allFiles, entry)
		}
		fail(<-completed)
	}()
	wg.Wait()
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := errors.Join(failures...); err != nil {
		return nil, err
	}
	reportSearchProgress(options.Progress, search.Progress{Step: "codebase-assembling"})
	codebaseNode := &TreeNode{Kind: TreeNodeCategory, ID: "codebase", Label: model.EmojiText(model.EmojiCodebase) + "Codebase", URI: "repo://codebase"}
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })

	type folderEntry struct {
		folder *model.Folder
		node   *TreeNode
	}
	globalFolderMap := make(map[string]*folderEntry)
	for fi := range allFolders {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		f := &allFolders[fi]
		fNode := &TreeNode{
			Kind:    TreeNodeFolder,
			ID:      f.GetID(),
			Label:   f.Name,
			URI:     f.GetURI(),
			SubKind: string(f.Kind),
			Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": string(f.Kind)},
		}
		globalFolderMap[f.Path] = &folderEntry{folder: f, node: fNode}
	}

	globalFileNodes := make(map[string]*TreeNode)
	for fi := range allFiles {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		f := &allFiles[fi]
		fileNode := &TreeNode{
			Kind:    TreeNodeFile,
			ID:      f.GetID(),
			Label:   f.Name,
			URI:     f.GetURI(),
			SubKind: f.Kind,
			Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": f.Kind},
		}
		if options.IncludeSections {
			absPath := filepath.Join(workspace.GetRootDir(), f.Path)
			content, err := workspace.ReadTextFile(absPath)
			if err == nil {
				sections := languages.ParseSections(content, f.Path)
				definitions := languages.ParseDefinitions(content, f.Path)
				sections = languages.HydrateSectionsWithDefinitions(sections, definitions)
				for si := range sections {
					s := &sections[si]
					sNode := buildSectionTreeNode(s)
					fileNode.Children = append(fileNode.Children, sNode)
				}
			}
		}
		globalFileNodes[f.Path] = fileNode
	}

	folderRoots := buildFolderRoots(allFolders)
	for _, folderRoot := range folderRoots {
		codebaseNode.Children = append(codebaseNode.Children, folderRoot)
	}
	attachFilesToFolders(codebaseNode, allFiles, globalFileNodes)

	for pi := range technologies {

		if err := ctx.Err(); err != nil {
			return nil, err
		}
		p := &technologies[pi]
		pNode := &TreeNode{
			Kind:    TreeNodeTechnology,
			ID:      p.GetID(),
			Label:   p.Name,
			URI:     p.GetURI(),
			SubKind: string(p.Kind),
			Data:    map[string]interface{}{"name": p.Name, "kind": string(p.Kind), "emoji": p.Emoji},
		}
		sort.Slice(p.Bundles, func(i, j int) bool { return p.Bundles[i].Name < p.Bundles[j].Name })
		for bi := range p.Bundles {
			b := &p.Bundles[bi]
			bNode := &TreeNode{
				Kind:    TreeNodeBundle,
				ID:      b.GetID(),
				Label:   b.Name,
				URI:     b.GetURI(),
				SubKind: string(b.Kind),
				Data:    map[string]interface{}{"name": b.Name, "root": b.Root, "kind": string(b.Kind), "emoji": b.Emoji},
			}

			bundleRoot := b.Root
			if b.SourceRoot != "" {
				bundleRoot = b.SourceRoot
			}

			bundleFolderMap := make(map[string]*folderEntry)
			for path, fe := range globalFolderMap {
				if path == bundleRoot || strings.HasPrefix(path, bundleRoot+"/") {
					bundleFolderMap[path] = fe
				}
			}

			for path, fe := range bundleFolderMap {
				parentPath := filepath.Dir(path)
				if _, ok := bundleFolderMap[parentPath]; ok {
					continue
				}
				bNode.Children = append(bNode.Children, fe.node)
			}
			for path, fe := range bundleFolderMap {
				parentPath := filepath.Dir(path)
				if pfe, ok := bundleFolderMap[parentPath]; ok {
					pfe.node.Children = append(pfe.node.Children, fe.node)
				}
			}

			for path, fileNode := range globalFileNodes {
				if path != bundleRoot && !strings.HasPrefix(path, bundleRoot+"/") {
					continue
				}
				folderPath := filepath.Dir(path)
				if fe, ok := bundleFolderMap[folderPath]; ok {
					fe.node.Children = append(fe.node.Children, fileNode)
				} else {
					bNode.Children = append(bNode.Children, fileNode)
				}
			}

			SortTreeChildren(bNode)
			pNode.Children = append(pNode.Children, bNode)
		}
		codebaseNode.Children = append(codebaseNode.Children, pNode)
	}
	SortTreeChildren(codebaseNode)
	return codebaseNode, ctx.Err()

}

// 🧬️buildRepositoryDomains refreshes live and persistent domain projections for each request.
func buildRepositoryDomains(ctx context.Context, codebaseNode *TreeNode, options TreeBuildOptions) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	root := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{codebaseNode}}
	var goals []*model.Goal
	var drafts []*model.Draft
	var policies []statutespkg.PolicyDef
	var contributors []model.Contributor
	var checkpoints []model.Checkpoint
	var sessions []contributorspkg.Session
	reportSearchProgress(options.Progress, search.Progress{Step: "domain-discovery"})
	var failuresMu sync.Mutex
	var failures []error
	fail := func(err error) {
		if err != nil {
			failuresMu.Lock()
			failures = append(failures, err)
			failuresMu.Unlock()
		}
	}
	var wg sync.WaitGroup
	wg.Add(6)
	go func() {
		defer wg.Done()
		entries := make(chan *model.Goal)
		completed := make(chan error, 1)
		go func() { completed <- goalspkg.StreamGoals(ctx, entries) }()
		for entry := range entries {
			goals = append(goals, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan statutespkg.PolicyDef)
		completed := make(chan error, 1)
		go func() { completed <- statutespkg.StreamPolicies(ctx, entries) }()
		for entry := range entries {
			policies = append(policies, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan model.Contributor)
		completed := make(chan error, 1)
		go func() { completed <- contributorspkg.StreamContributors(ctx, entries) }()
		for entry := range entries {
			contributors = append(contributors, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan contributorspkg.Session)
		completed := make(chan error, 1)
		go func() { completed <- contributorspkg.StreamSessions(ctx, entries) }()
		for entry := range entries {
			sessions = append(sessions, entry)
		}
		fail(<-completed)
	}()
	go func() { defer wg.Done(); drafts = todos.ListDrafts(todos.NewFsDraftStore(workspace.GetDraftsPath())) }()
	go func() { defer wg.Done(); limit := 100; checkpoints = contributorspkg.LoadCheckpoints(&limit) }()
	wg.Wait()
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := errors.Join(failures...); err != nil {
		return nil, err
	}
	reportSearchProgress(options.Progress, search.Progress{Step: "domain-assembling"})
	goalsNode := &TreeNode{Kind: TreeNodeCategory, ID: "goals", Label: "🎯️Goals", URI: "repo://goals"}
	goalMap := make(map[string]*TreeNode)
	sort.Slice(goals, func(i, j int) bool { return goals[i].ID < goals[j].ID })
	for _, g := range goals {
		goalCreatedAt := ""
		gNode := &TreeNode{
			Kind:        TreeNodeGoal,
			ID:          g.GetID(),
			Label:       g.Title,
			URI:         g.GetURI(),
			SubKind:     g.Status,
			Description: g.Description,
			Status:      g.Status,
			Data: map[string]interface{}{
				"id":          g.ID,
				"title":       g.Title,
				"status":      g.Status,
				"dueDate":     g.DueDate,
				"createdAt":   goalCreatedAt,
				"description": g.Description,
			},
		}
		goalMap[g.ID] = gNode
	}
	var rootGoals []*TreeNode
	for _, g := range goals {
		gNode := goalMap[g.ID]
		parentID := ""
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			parentID = g.ID[:idx]
		}
		if parentID != "" && parentID != g.ID {
			if parent, ok := goalMap[parentID]; ok {
				parent.Children = append(parent.Children, gNode)
			} else {
				rootGoals = append(rootGoals, gNode)
			}
		} else {
			rootGoals = append(rootGoals, gNode)
		}
	}
	goalsNode.Children = rootGoals

	ticketCh := make(chan model.Ticket)
	ticketError := make(chan error, 1)
	go func() { ticketError <- ticketspkg.StreamTickets(ctx, nil, nil, nil, ticketCh) }()
	var tickets []model.Ticket
	for t := range ticketCh {
		tickets = append(tickets, t)
	}
	if err := <-ticketError; err != nil {
		return nil, err
	}
	for ti := range tickets {
		t := &tickets[ti]
		ticketStarted := t.GetDateStarted().Format(time.RFC3339)
		ticketFinished := ""
		if finished := t.GetDateFinished(); finished != nil {
			ticketFinished = finished.Format(time.RFC3339)
		}
		tNode := &TreeNode{
			Kind:        TreeNodeTicket,
			ID:          t.GetID(),
			Label:       t.Title,
			URI:         t.GetURI(),
			Description: t.Description,
			Year:        t.Year,
			Month:       t.Month,
			Day:         t.Day,
			Status:      string(t.Status),
			Data: map[string]interface{}{
				"year":     float64(t.Year),
				"month":    float64(t.Month),
				"day":      float64(t.Day),
				"slug":     t.Slug,
				"title":    t.Title,
				"status":   string(t.Status),
				"started":  ticketStarted,
				"finished": ticketFinished,
				"prompt":   t.Description,
				"summary":  t.Summary,
				"goalId":   t.Goal,
			},
		}
		if t.Goal != "" {
			if gNode, ok := goalMap[t.Goal]; ok {
				gNode.Children = append(gNode.Children, tNode)
				continue
			}
		}
		goalsNode.Children = append(goalsNode.Children, tNode)
	}
	root.Children = append(root.Children, goalsNode)

	draftsNode := &TreeNode{Kind: TreeNodeCategory, ID: "drafts", Label: "✍️Drafts", URI: "repo://drafts"}
	if drafts != nil {
		for _, d := range drafts {
			dNode := &TreeNode{
				Kind:  TreeNodeDraft,
				ID:    d.GetID(),
				Label: d.ID,
				URI:   d.GetURI(),
				Data:  map[string]interface{}{"id": d.ID, "slug": d.ID},
			}
			draftsNode.Children = append(draftsNode.Children, dNode)
		}
	}
	root.Children = append(root.Children, draftsNode)

	policiesNode := &TreeNode{Kind: TreeNodeCategory, ID: "policies", Label: "🛡️Policies", URI: "repo://policies"}
	sort.Slice(policies, func(i, j int) bool { return policies[i].ID < policies[j].ID })
	for _, p := range policies {
		policyId := model.EmojiText(model.EmojiPolicy) + workspace.Flat(strings.TrimPrefix(p.ID, "/"))
		pNode := &TreeNode{
			Kind:        TreeNodePolicy,
			ID:          fmt.Sprintf("%s/%s", model.EmojiText(model.EmojiPolicy), p.ID),
			Label:       p.Name,
			URI:         "repo://policy/" + policyId,
			Description: p.Description,
			Data:        map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description},
		}
		pNode.Children = buildPolicyEntityKindTree(p.Groups)
		policiesNode.Children = append(policiesNode.Children, pNode)
	}
	root.Children = append(root.Children, policiesNode)

	contributorsNode := &TreeNode{Kind: TreeNodeCategory, ID: "contributors", Label: "🧑️‍💻️Contributors", URI: "repo://contributors"}
	sort.Slice(contributors, func(i, j int) bool { return contributors[i].Alias < contributors[j].Alias })
	for _, c := range contributors {
		label := c.Alias
		if c.Name != "" {
			label = c.Name + " (" + c.Alias + ")"
		}
		contributorData := map[string]interface{}{
			"alias":   c.Alias,
			"aliases": c.Aliases,
			"github":  c.Github,
			"githubs": c.Githubs,
			"name":    c.Name,
			"names":   c.Names,
			"email":   c.Email,
			"emails":  c.Emails,
		}
		cNode := &TreeNode{
			Kind:        TreeNodeContributor,
			ID:          c.GetID(),
			Label:       label,
			URI:         c.GetURI(),
			Contributor: c.Alias,
			Data:        contributorData,
		}
		contributorsNode.Children = append(contributorsNode.Children, cNode)
	}
	root.Children = append(root.Children, contributorsNode)

	checkpointsNode := &TreeNode{Kind: TreeNodeCategory, ID: "checkpoints", Label: "🔀️Checkpoints", URI: "repo://checkpoints"}
	for ci := range checkpoints {
		c := &checkpoints[ci]
		cNode := &TreeNode{
			Kind:  TreeNodeCheckpoint,
			ID:    c.GetID(),
			Label: c.SHA[:8] + " " + c.Title,
			URI:   c.GetURI(),
			Year:  checkpointDatePart(c.Date, 0, 4),
			Month: checkpointDatePart(c.Date, 5, 7),
			Day:   checkpointDatePart(c.Date, 8, 10),
			Data: map[string]interface{}{"sha": c.SHA, "message": c.Title, "authorId": func() string {
				if c.AuthorID != nil {
					return *c.AuthorID
				}
				return ""
			}()},
		}
		checkpointsNode.Children = append(checkpointsNode.Children, cNode)
	}
	root.Children = append(root.Children, checkpointsNode)

	sessionsNode := &TreeNode{Kind: TreeNodeCategory, ID: "sessions", Label: "⚪️Sessions", URI: "repo://sessions"}
	sort.Slice(sessions, func(i, j int) bool { return sessions[i].StartedAt > sessions[j].StartedAt })
	for si := range sessions {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		s := &sessions[si]
		kindEmoji := contributorspkg.SessionKindEmoji(s.Kind)
		label := kindEmoji + " " + s.UUID
		if s.Client != "" {
			label += " (" + s.Client + ")"
		}
		sNode := &TreeNode{
			Kind:    TreeNodeSession,
			ID:      s.GetID(),
			Label:   label,
			URI:     s.GetURI(),
			SubKind: string(s.Kind),
			Year:    s.Year,
			Month:   s.Month,
			Day:     s.Day,
			Status:  string(s.Kind),
			Data: map[string]interface{}{
				"uuid":       s.UUID,
				"kind":       string(s.Kind),
				"client":     s.Client,
				"startedAt":  s.StartedAt,
				"year":       float64(s.Year),
				"month":      float64(s.Month),
				"day":        float64(s.Day),
				"checkpoint": s.Checkpoint,
			},
		}
		sessionsNode.Children = append(sessionsNode.Children, sNode)
	}
	root.Children = append(root.Children, sessionsNode)

	PropagateParentIDs(root, "", DefaultArtifactIdentifier{})
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	return root, nil
}

// 💿️PropagateParentIDs holds the data fields for a PropagateParentIDs record.
func PropagateParentIDs(node *TreeNode, parentArtifactID string, identifier ArtifactIdentifier) {
	if node.Data == nil {
		node.Data = map[string]interface{}{}
	}
	entityKind := TreeNodeKindToEntityKind(node.Kind)
	currentID := ""
	if entityKind != "" && node.Data != nil {
		node.Data["parentId"] = parentArtifactID
		currentID = identifier.ArtifactID(entityKind, node.Data)
	} else {
		currentID = parentArtifactID
	}
	for _, child := range node.Children {
		PropagateParentIDs(child, currentID, identifier)
	}
}

// 📑️buildSectionTreeNode holds the data fields for a buildSectionTreeNode record.
func buildSectionTreeNode(s *model.Section) *TreeNode {
	sNode := &TreeNode{
		Kind:  TreeNodeSection,
		ID:    s.GetID(),
		Label: s.Name,
		URI:   s.GetURI(),
		Data:  map[string]interface{}{"path": s.Path, "name": s.Name, "startLine": float64(s.StartLine), "endLine": float64(s.EndLine)},
	}
	for di := range s.Definitions {
		d := &s.Definitions[di]
		dNode := &TreeNode{
			Kind:    TreeNodeDefinition,
			ID:      d.GetID(),
			Label:   d.Name,
			URI:     d.GetURI(),
			SubKind: string(d.Kind),
			Data:    map[string]interface{}{"path": d.FilePath, "name": d.Name, "kind": string(d.Kind), "startLine": float64(d.StartLine), "endLine": float64(d.EndLine)},
		}
		sNode.Children = append(sNode.Children, dNode)
	}
	for ci := range s.Children {
		child := &s.Children[ci]
		sNode.Children = append(sNode.Children, buildSectionTreeNode(child))
	}
	return sNode
}

// 🏗️buildStatuteTree holds the data fields for a buildStatuteTree record.
func BuildStatuteTreeForRepo(kinds []model.Statute) []*TreeNode {
	type treeEntry struct {
		node     *TreeNode
		children map[string]*treeEntry
	}
	rootEntries := make(map[string]*treeEntry)
	orderedRootKeys := []string{}

	for _, k := range kinds {
		parts := strings.Split(string(k), "/")
		current := rootEntries
		currentOrdered := &orderedRootKeys
		for i, part := range parts {
			if _, ok := current[part]; !ok {
				isLeaf := i == len(parts)-1
				meta := k.Info()
				n := &TreeNode{
					Kind:  TreeNodeStatute,
					ID:    string(k),
					Label: part,
					URI:   meta.GetURI(),
				}
				if isLeaf {
					n.Description = meta.Reason
					priorityIcon := "🟢️"
					if meta.Priority == model.BreachPriorityHigh {
						priorityIcon = "🔴️"
					} else if meta.Priority == model.BreachPriorityMedium {
						priorityIcon = "🟡️"
					}
					n.Label = priorityIcon + part
					if meta.Autofixable {
						n.SubKind = "autofixable"
					}
					n.Data = map[string]interface{}{
						"id":          string(k),
						"priority":    string(meta.Priority),
						"autofixable": meta.Autofixable,
						"reason":      meta.Reason,
						"solution":    meta.Solution,
					}
				} else {
					prefix := strings.Join(parts[:i+1], "/")
					n.ID = "breachCategory:" + prefix
					n.URI = "repo://statute/" + workspace.Flat(prefix)
					n.Kind = TreeNodeCategory
				}
				current[part] = &treeEntry{node: n, children: make(map[string]*treeEntry)}
				*currentOrdered = append(*currentOrdered, part)
			}
			entry := current[part]
			current = entry.children
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			currentOrdered = &childKeys
		}
	}

	var buildNodes func(entries map[string]*treeEntry, keys []string) []*TreeNode
	buildNodes = func(entries map[string]*treeEntry, keys []string) []*TreeNode {
		seen := map[string]bool{}
		var result []*TreeNode
		for _, key := range keys {
			if seen[key] {
				continue
			}
			seen[key] = true
			entry := entries[key]
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			sort.Strings(childKeys)
			entry.node.Children = buildNodes(entry.children, childKeys)
			result = append(result, entry.node)
		}
		return result
	}

	return buildNodes(rootEntries, orderedRootKeys)
}

// 🔷️buildTerritoryTree holds the data fields for a buildTerritoryTree record.
func buildTerritoryTree(groups []model.Territory) []*TreeNode {
	var result []*TreeNode
	for _, g := range groups {
		groupNode := &TreeNode{
			Kind:        TreeNodeCategory,
			ID:          fmt.Sprintf("%s%s", model.EmojiText(model.EmojiTerritory), g.Name),
			Label:       g.Name,
			URI:         "repo://territory/" + g.GetID(),
			Description: g.Description,
			SubKind:     "territory",
			Data: map[string]interface{}{
				"name":        g.Name,
				"description": g.Description,
				"scopes":      g.Scopes,
			},
		}
		for _, k := range g.Kinds {
			meta := k.Info()
			priorityIcon := "🟢️"
			if meta.Priority == model.BreachPriorityHigh {
				priorityIcon = "🔴️"
			} else if meta.Priority == model.BreachPriorityMedium {
				priorityIcon = "🟡️"
			}
			label := priorityIcon + workspace.StatutePathToIdValue(string(k))
			vkNode := &TreeNode{
				Kind:        TreeNodeStatute,
				ID:          meta.GetID(),
				Label:       label,
				URI:         meta.GetURI(),
				Description: meta.Reason,
				Data: map[string]interface{}{
					"id":          string(k),
					"priority":    string(meta.Priority),
					"autofixable": meta.Autofixable,
					"reason":      meta.Reason,
					"solution":    meta.Solution,
				},
			}
			if meta.Autofixable {
				vkNode.SubKind = "autofixable"
			}
			groupNode.Children = append(groupNode.Children, vkNode)
		}
		groupNode.Children = append(groupNode.Children, buildTerritoryTree(g.Groups)...)
		result = append(result, groupNode)
	}
	return result
}

// #endregion 🩻️Monorepo Tree
