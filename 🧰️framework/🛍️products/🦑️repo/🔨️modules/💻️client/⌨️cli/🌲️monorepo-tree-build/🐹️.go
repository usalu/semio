// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Monorepo tree builder.

// #endregion 🧲️Header

package client

import (
	"context"
	"errors"
	"fmt"
	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"
)

// 🌳️TreeBuildOptions holds the data fields for a tree build options record.
type TreeBuildOptions struct {
	IncludeSections bool
	Progress        func(search.Progress) `json:"-"`
}

// 🏢️BuildMonorepoTree MUST assemble the monorepo tree from the available context data.
// 🏢️BuildMonorepoTree constructs and returns the monorepo tree structure.
func BuildMonorepoTree(ctx context.Context, opts ...TreeBuildOptions) (*TreeNode, error) {
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
	var technologies []Technology
	var allFolders []Folder
	var allFiles []File
	InvalidateTechnologyCache()
	technologies = LoadTechnologies()
	boundaries := []model.ArtifactBoundary{}
	for _, technology := range technologies {
		for _, bundle := range technology.Bundles {
			boundaries = append(boundaries, model.ArtifactBoundary{Root: bundle.Root, ID: bundle.GetID()})
		}
	}
	layout := model.NewArtifactLayout(GetRootDir(), boundaries)
	streamOptions := StreamOptions{Layout: layout, Progress: func(value StreamProgress) {
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
		entries := make(chan Folder)
		completed := make(chan error, 1)
		go func() { completed <- StreamFolders(ctx, "", entries, streamOptions) }()
		for entry := range entries {
			allFolders = append(allFolders, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan File)
		completed := make(chan error, 1)
		go func() { completed <- StreamFiles(ctx, "", entries, streamOptions) }()
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
	codebaseNode := &TreeNode{Kind: TreeNodeCategory, ID: "codebase", Label: emojiText(EmojiCodebase) + "Codebase", URI: "repo://codebase"}
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })

	type folderEntry struct {
		folder *Folder
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
			absPath := filepath.Join(GetRootDir(), f.Path)
			content, err := ReadTextFile(absPath)
			if err == nil {
				sections := ParseSections(content, f.Path)
				definitions := ParseDefinitions(content, f.Path)
				sections = HydrateSectionsWithDefinitions(sections, definitions)
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

			sortTreeChildren(bNode)
			pNode.Children = append(pNode.Children, bNode)
		}
		codebaseNode.Children = append(codebaseNode.Children, pNode)
	}
	sortTreeChildren(codebaseNode)
	return codebaseNode, ctx.Err()

}

// 🧬️buildRepositoryDomains refreshes live and persistent domain projections for each request.
func buildRepositoryDomains(ctx context.Context, codebaseNode *TreeNode, options TreeBuildOptions) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	root := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{codebaseNode}}
	var goals []*Goal
	var drafts []*Draft
	var policies []PolicyDef
	var contributors []Contributor
	var checkpoints []Checkpoint
	var sessions []Session
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
		entries := make(chan *Goal)
		completed := make(chan error, 1)
		go func() { completed <- StreamGoals(ctx, entries) }()
		for entry := range entries {
			goals = append(goals, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan PolicyDef)
		completed := make(chan error, 1)
		go func() { completed <- StreamPolicies(ctx, entries) }()
		for entry := range entries {
			policies = append(policies, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan Contributor)
		completed := make(chan error, 1)
		go func() { completed <- StreamContributors(ctx, entries) }()
		for entry := range entries {
			contributors = append(contributors, entry)
		}
		fail(<-completed)
	}()
	go func() {
		defer wg.Done()
		entries := make(chan Session)
		completed := make(chan error, 1)
		go func() { completed <- StreamSessions(ctx, entries) }()
		for entry := range entries {
			sessions = append(sessions, entry)
		}
		fail(<-completed)
	}()
	go func() { defer wg.Done(); var err error; drafts, err = ListDrafts(); fail(err) }()
	go func() { defer wg.Done(); limit := 100; checkpoints = LoadCheckpoints(&limit) }()
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

	ticketCh := make(chan Ticket)
	ticketError := make(chan error, 1)
	go func() { ticketError <- StreamTickets(ctx, nil, nil, nil, ticketCh) }()
	var tickets []Ticket
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
		policyId := emojiText(EmojiPolicy) + Flat(strings.TrimPrefix(p.ID, "/"))
		pNode := &TreeNode{
			Kind:        TreeNodePolicy,
			ID:          fmt.Sprintf("%s/%s", emojiText(EmojiPolicy), p.ID),
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
			Year:  c.Date.Year(),
			Month: int(c.Date.Month()),
			Day:   c.Date.Day(),
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
		kindEmoji := SessionKindEmoji(s.Kind)
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

	PropagateParentIDs(root, "")
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	return root, nil
}

// 💿️PropagateParentIDs holds the data fields for a PropagateParentIDs record.
func PropagateParentIDs(node *TreeNode, parentArtifactID string) {
	if node.Data == nil {
		node.Data = map[string]interface{}{}
	}
	entityKind := treeNodeKindToEntityKind(node.Kind)
	currentID := ""
	if entityKind != "" && node.Data != nil {
		node.Data["parentId"] = parentArtifactID
		currentID = GetArtifactID(entityKind, node.Data)
	} else {
		currentID = parentArtifactID
	}
	for _, child := range node.Children {
		PropagateParentIDs(child, currentID)
	}
}

// 📑️buildSectionTreeNode holds the data fields for a buildSectionTreeNode record.
func buildSectionTreeNode(s *Section) *TreeNode {
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
