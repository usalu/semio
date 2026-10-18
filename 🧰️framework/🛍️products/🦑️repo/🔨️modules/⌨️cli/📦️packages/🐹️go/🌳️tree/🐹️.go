// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌳️tree carries the filesystem tree source and the tree command of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	graphqlpkg "github.com/usalu/semio/repo/graphql"
	languagespkg "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	treepkg "github.com/usalu/semio/repo/tree"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🌳️TreeSource

// 🗂️FsTreeSource is the treepkg.TreeSource the `tree` verb projects from: every record already read
// out of the repository through the GraphQL repo context, the twin of the Rust `FsTreeSource`.
type FsTreeSource struct {
	TechnologyRecords  []treepkg.TechnologyRecord
	FolderRecords      []treepkg.FolderRecord
	FileRecords        []treepkg.FileRecord
	GoalSeeds          []treepkg.GoalRecord
	TicketSeeds        []treepkg.TicketRecord
	DraftRecords       []treepkg.DraftRecord
	PolicyRecords      []treepkg.PolicyRecord
	ContributorRecords []treepkg.ContributorRecord
	CheckpointRecords  []treepkg.CheckpointRecord
	Root               string
	IncludeSections    bool
}

// 🧹️NormalizeTreeScope MUST return the scope as the record paths spell it.
// 🧹️NormalizeTreeScope returns one scope with forward slashes, no leading or trailing separator, and
// the empty string for the repository root.
func NormalizeTreeScope(scope string) string {
	scope = strings.Trim(strings.TrimSpace(strings.ReplaceAll(scope, "\\", "/")), "/")
	if scope == "." {
		return ""
	}
	return scope
}

// 🎯️PathWithinScope MUST return true only when the path is the scope itself or lies below it.
// 🎯️PathWithinScope reports whether a record path belongs to a scope.
func PathWithinScope(path string, scope string) bool {
	return scope == "" || path == scope || strings.HasPrefix(path, scope+"/")
}

// 🔤️derefString returns the pointed-to string, or the empty string when there is none.
func derefString(value *string) string {
	if value == nil {
		return ""
	}
	return *value
}

// 🎯️LoadFsTreeSource MUST read every record the monorepo tree projects.
// 🎯️LoadFsTreeSource reads the records of one repository-relative subtree: every folder and file at
// or below the scope, every bundle rooted there and the technologies that still own one. A scope
// names a place in the codebase, so the aggregates that have no place — goals, tickets, drafts,
// policies, contributors, checkpoints — are not part of a scoped projection. An empty scope is the
// whole repository and loads all of them.
func LoadFsTreeSource(ctx model.RepoContext, includeSections bool, scope string) *FsTreeSource {
	scope = NormalizeTreeScope(scope)
	source := &FsTreeSource{Root: ctx.GetRootDir(), IncludeSections: includeSections}
	bundles := ctx.GetBundles()
	for _, technology := range ctx.GetTechnologies() {
		record := treepkg.TechnologyRecord{
			ID:      technology.GetID(),
			Name:    technology.Name,
			URI:     "repo://technology/" + technology.GetID(),
			Kind:    string(technology.Kind),
			Emoji:   technology.Emoji,
			Bundles: []treepkg.BundleRecord{},
		}
		for _, bundle := range bundles {
			if bundle.TechnologyName != technology.Name || !PathWithinScope(bundle.Root, scope) {
				continue
			}
			record.Bundles = append(record.Bundles, treepkg.BundleRecord{
				ID:         bundle.GetID(),
				Name:       bundle.Name,
				URI:        "repo://bundle/" + bundle.GetID(),
				Kind:       string(bundle.Kind),
				Emoji:      bundle.Emoji,
				Root:       bundle.Root,
				SourceRoot: bundle.SourceRoot,
			})
		}
		if scope != "" && len(record.Bundles) == 0 {
			continue
		}
		source.TechnologyRecords = append(source.TechnologyRecords, record)
	}
	for _, folder := range ctx.GetFolders() {
		if !PathWithinScope(folder.Path, scope) {
			continue
		}
		source.FolderRecords = append(source.FolderRecords, treepkg.FolderRecord{ID: folder.ID, Path: folder.Path, Name: folder.Name, URI: folder.URI, Kind: string(folder.Kind), ParentID: derefString(folder.ParentID)})
	}
	for _, file := range ctx.GetFiles() {
		if !PathWithinScope(file.Path, scope) {
			continue
		}
		source.FileRecords = append(source.FileRecords, treepkg.FileRecord{ID: file.ID, Path: file.Path, Name: file.Name, URI: file.URI, Kind: file.Kind, ParentID: derefString(file.FolderID)})
	}
	if scope != "" {
		return source
	}
	goals, _ := ctx.GetGoals()
	for _, goal := range goals {
		source.GoalSeeds = append(source.GoalSeeds, treepkg.GoalRecord{ID: goal.ID, Title: goal.Title, URI: "repo://goal/" + goal.ID, Status: goal.Status, DueDate: goal.Dates.Due, Description: goal.Description})
	}
	tickets, _ := ctx.GetTickets(nil, nil, nil, nil)
	for _, ticket := range tickets {
		source.TicketSeeds = append(source.TicketSeeds, treepkg.TicketRecord{
			ID:          fmt.Sprintf("%d/%d/%d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug),
			Slug:        ticket.Slug,
			Title:       ticket.Title,
			URI:         "repo://ticket/" + ticket.Slug,
			Description: ticket.Description,
			Summary:     ticket.Summary,
			Goal:        ticket.Goal,
			Parent:      ticket.Parent,
			Year:        ticket.Year,
			Month:       ticket.Month,
			Day:         ticket.Day,
			Status:      string(ticket.Status),
		})
	}
	drafts, _ := ctx.GetDrafts()
	for _, draft := range drafts {
		source.DraftRecords = append(source.DraftRecords, treepkg.DraftRecord{ID: draft.ID, URI: "repo://draft/" + draft.ID})
	}
	for _, policy := range ctx.GetPolicies() {
		source.PolicyRecords = append(source.PolicyRecords, treepkg.PolicyRecord{ID: policy.ID, Name: policy.Name, Description: derefString(policy.Description), Groups: policy.Groups})
	}
	contributors, _ := ctx.GetContributors()
	for _, contributor := range contributors {
		email := ""
		if len(contributor.Emails) > 0 {
			email = contributor.Emails[0]
		}
		source.ContributorRecords = append(source.ContributorRecords, treepkg.ContributorRecord{
			ID:      contributor.Alias,
			Alias:   contributor.Alias,
			URI:     "repo://contributor/" + contributor.Alias,
			Name:    contributor.Name,
			Aliases: []string{},
			Github:  contributor.Github,
			Githubs: []string{},
			Names:   []string{},
			Email:   email,
			Emails:  contributor.Emails,
		})
	}
	limit := 100
	checkpoints, _ := ctx.GetCheckpoints(&limit)
	for _, checkpoint := range checkpoints {
		source.CheckpointRecords = append(source.CheckpointRecords, treepkg.CheckpointRecord{ID: checkpoint.ID, Sha: checkpoint.SHA, Title: checkpoint.Title, URI: "repo://checkpoint/" + checkpoint.SHA, AuthorID: derefString(checkpoint.AuthorID)})
	}
	return source
}

// 🧪️Technologies MUST return the technologies with their bundles, in load order.
func (s *FsTreeSource) Technologies() []treepkg.TechnologyRecord { return s.TechnologyRecords }

// 📁️Folders MUST return every folder of the projection.
func (s *FsTreeSource) Folders() []treepkg.FolderRecord { return s.FolderRecords }

// 📄️Files MUST return every file of the projection.
func (s *FsTreeSource) Files() []treepkg.FileRecord { return s.FileRecords }

// 🎯️GoalRecords MUST return every goal of the projection.
func (s *FsTreeSource) GoalRecords() []treepkg.GoalRecord { return s.GoalSeeds }

// 🎫️TicketRecords MUST return every ticket of the projection.
func (s *FsTreeSource) TicketRecords() []treepkg.TicketRecord { return s.TicketSeeds }

// ✍️Drafts MUST return every draft of the projection.
func (s *FsTreeSource) Drafts() []treepkg.DraftRecord { return s.DraftRecords }

// 🛡️Policies MUST return every policy of the projection.
func (s *FsTreeSource) Policies() []treepkg.PolicyRecord { return s.PolicyRecords }

// 🧑️Contributors MUST return every contributor of the projection.
func (s *FsTreeSource) Contributors() []treepkg.ContributorRecord { return s.ContributorRecords }

// 🔀️Checkpoints MUST return every checkpoint of the projection.
func (s *FsTreeSource) Checkpoints() []treepkg.CheckpointRecord { return s.CheckpointRecords }

// ⚪️Sessions MUST return every agent session of the projection.
func (s *FsTreeSource) Sessions() []treepkg.SessionRecord { return nil }

// 📑️Sections MUST return the parsed sections of one file, read only when sections are requested.
func (s *FsTreeSource) Sections(filePath string) []treepkg.SectionRecord {
	if !s.IncludeSections {
		return nil
	}
	content, err := os.ReadFile(filepath.Join(s.Root, filePath))
	if err != nil {
		return nil
	}
	sections := languagespkg.ParseSections(string(content), filePath)
	records := make([]treepkg.SectionRecord, 0, len(sections))
	for i := range sections {
		records = append(records, convertSectionRecord(&sections[i], filePath))
	}
	return records
}

// 🔁️convertSectionRecord projects one parsed section into the record the tree module reads.
func convertSectionRecord(section *model.Section, filePath string) treepkg.SectionRecord {
	path := section.Path
	if path == "" {
		path = filePath + "#" + section.Name
	}
	record := treepkg.SectionRecord{ID: section.ID, Path: path, Name: section.Name, StartLine: section.StartLine, EndLine: section.EndLine}
	record.Definitions = make([]treepkg.DefinitionRecord, 0, len(section.Definitions))
	for _, definition := range section.Definitions {
		record.Definitions = append(record.Definitions, treepkg.DefinitionRecord{ID: definition.ID, Name: definition.Name, Kind: string(definition.Kind), FilePath: filePath, StartLine: definition.StartLine, EndLine: definition.EndLine})
	}
	record.Children = make([]treepkg.SectionRecord, 0, len(section.Children))
	for i := range section.Children {
		record.Children = append(record.Children, convertSectionRecord(&section.Children[i], filePath))
	}
	return record
}

// #endregion 🌳️TreeSource

// #region 🌳️Tree Command

// 🌳️treeCommand builds the `tree` verb: the four trees 🌳️tree projects, each rendered through the
// persistent format flag. Every subcommand builds through the tree module's own ports, so the two
// implementations project the same document from the same repository.
func treeCommand(factory EngineFactory, config *Config) *Command {
	_ = factory
	root := &Command{Use: "tree", Short: "Show a repository tree"}
	monorepo := &Command{
		Use:   "monorepo [query]",
		Short: "Show the monorepo tree",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}
			scope, _ := cmd.Flags().GetString("scope")
			includeSections := filter.OnlyKinds[treepkg.TreeNodeSection] || filter.OnlyKinds[treepkg.TreeNodeDefinition]
			source := LoadFsTreeSource(graphqlpkg.NewRepoContext(repoRootOf(config)), includeSections, scope)
			node := treepkg.BuildMonorepoTree(source, treepkg.TreeBuildOptions{IncludeSections: includeSections})
			if filter.Query != "" {
				node = treepkg.SearchTreeInMemory(node, filter.Query)
			}
			return writeTreeNode(cmd, config, treepkg.FilterMonorepoTree(node, &filter))
		},
	}
	monorepo.Flags().String("scope", "", "Restrict the projection to this repository-relative path")
	bindTreeFlags(monorepo)
	goal := &Command{
		Use:   "goal",
		Short: "Show the goal and ticket tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			source := LoadFsTreeSource(graphqlpkg.NewRepoContext(repoRootOf(config)), false, "")
			roots := treepkg.BuildGoalTree(source.GoalRecords(), source.TicketRecords())
			if config.IsJSON() {
				encoded, err := json.Marshal(roots)
				if err != nil {
					return err
				}
				fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
				return nil
			}
			format := treepkg.TreeRenderFormatText
			if config.IsMarkdown() {
				format = treepkg.TreeRenderFormatMarkdown
			}
			fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderGoalTreeNodes(roots, format, treepkg.DefaultEntityRenderer{}))
			return nil
		},
	}
	statute := &Command{
		Use:   "statute",
		Short: "Show the statute tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			return writeTreeForest(cmd, config, treepkg.BuildStatuteTree(treepkg.DeclaredStatutes(), treepkg.DeclaredStatuteCatalog{}))
		},
	}
	territory := &Command{
		Use:   "territory",
		Short: "Show the territory tree",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			return writeTreeForest(cmd, config, treepkg.BuildTerritoryTree(treepkg.DeclaredTerritories(), treepkg.DeclaredStatuteCatalog{}))
		},
	}
	root.AddCommand(monorepo)
	root.AddCommand(goal)
	root.AddCommand(statute)
	root.AddCommand(territory)
	return root
}

// 🏠️repoRootOf resolves the repository root one invocation runs against.
func repoRootOf(config *Config) string {
	if config != nil && config.Repo != "" {
		return config.Repo
	}
	return workspace.GetRootDir()
}

// 🌲️writeTreeForest renders a forest under the same anonymous category root the monorepo tree carries.
func writeTreeForest(cmd *Command, config *Config, roots []*treepkg.TreeNode) error {
	node := treepkg.NewTreeNode(treepkg.TreeNodeCategory, "", ".", "")
	node.Children = roots
	return writeTreeNode(cmd, config, node)
}

// 🖨️writeTreeNode writes one tree node in the requested format, exactly as `search` renders one.
func writeTreeNode(cmd *Command, config *Config, node *treepkg.TreeNode) error {
	switch {
	case config.IsJSON():
		encoded, err := json.Marshal(node)
		if err != nil {
			return err
		}
		fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
	case config.IsMarkdown():
		fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderMonorepoTreeMarkdown(node, treepkg.DefaultEntityRenderer{}))
	default:
		fmt.Fprint(cmd.OutOrStdout(), treepkg.RenderMonorepoTree(node, treepkg.DefaultEntityRenderer{}))
	}
	return nil
}

// #endregion 🌳️Tree Command
