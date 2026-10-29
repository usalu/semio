// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Search, list, query, export commands and tree filter flag binding.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	search "github.com/usalu/semio/repo/search"
	"sort"
	"strings"
)

// ⌨️searchCommand holds the data fields for a searchCommand record.
func searchCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "search [query]",
		Short: "Search monorepo tree",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			ctx := cmd.Context()
			progress := search.NewProgressReporter(cmd.ErrOrStderr())

			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}

			buildOpts := TreeBuildOptions{
				IncludeSections: filter.OnlyKinds[TreeNodeSection] ||
					filter.OnlyKinds[TreeNodeDefinition],
			}
			buildOpts.Progress = progress
			tree, err := BuildMonorepoTreeCached(ctx, buildOpts)
			if err != nil {
				return err
			}
			limit, _ := cmd.Flags().GetInt("limit")
			if limit == 0 {
				limit = 100
			}
			tree, err = SearchMonorepoTreeWithOptions(ctx, tree, filter.Query, TreeSearchOptions{Limit: limit, Filter: &filter, Progress: progress})
			if err != nil {
				return err
			}
			tree = FilterMonorepoTree(tree, &filter)

			var output string
			switch {
			case config.IsJSON():
				encoded, err := json.Marshal(tree)
				if err != nil {
					return err
				}
				output = string(encoded) + "\n"
			case config.IsMarkdown():
				output = RenderMonorepoTreeMarkdown(tree)
			default:
				output = RenderMonorepoTree(tree)
			}
			fmt.Fprint(cmd.OutOrStdout(), output)
			return nil
		},
	}
	cmd.Flags().Int("limit", 100, "Maximum ranked search hits (1–10000)")
	bindTreeFlags(cmd)
	return cmd
}

// 🌳️flattenTreeNodes holds the data fields for a flattenTreeNodes record.
func flattenTreeNodes(node *TreeNode, out *[]*TreeNode) {
	if node.Kind != TreeNodeCategory {
		*out = append(*out, node)
	}
	for _, c := range node.Children {
		flattenTreeNodes(c, out)
	}
}

// ⬛️listCommand holds the data fields for a listCommand record.
func listCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "list [query]",
		Short: "Stream a flat list of monorepo items",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			ctx := cmd.Context()
			progress := search.NewProgressReporter(cmd.ErrOrStderr())
			sorted, _ := cmd.Flags().GetBool("sorted")
			limit, _ := cmd.Flags().GetInt("limit")
			filter := buildTreeFilterFromFlags(cmd)
			if len(args) > 0 {
				filter.Query = args[0]
			}
			if queryFlag, _ := cmd.Flags().GetString("query"); queryFlag != "" && filter.Query == "" {
				filter.Query = queryFlag
			}
			buildOpts := TreeBuildOptions{
				IncludeSections: filter.OnlyKinds[TreeNodeSection] ||
					filter.OnlyKinds[TreeNodeDefinition],
			}
			buildOpts.Progress = progress
			tree, err := BuildMonorepoTreeCached(ctx, buildOpts)
			if err != nil {
				return err
			}
			searchLimit := limit
			if searchLimit == 0 {
				searchLimit = 100
			}
			tree, err = SearchMonorepoTreeWithOptions(ctx, tree, filter.Query, TreeSearchOptions{Limit: searchLimit, Filter: &filter, Progress: progress})
			if err != nil {
				return err
			}
			tree = FilterMonorepoTree(tree, &filter)
			var nodes []*TreeNode
			flattenTreeNodes(tree, &nodes)
			if sorted {
				sort.Slice(nodes, func(i, j int) bool { return nodes[i].ID < nodes[j].ID })
			}
			if limit > 0 && len(nodes) > limit {
				nodes = nodes[:limit]
			}
			switch {
			case config.IsJSON():
				for _, n := range nodes {
					entityKind := treeNodeKindToEntityKind(n.Kind)
					if entityKind == "" {
						entityKind = string(n.Kind)
					}
					if n.Data != nil {
						wrapped := map[string]interface{}{entityKind: n.Data}
						encoded, err := json.Marshal(wrapped)
						if err != nil {
							continue
						}
						fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
					} else {
						encoded, err := json.Marshal(n)
						if err != nil {
							continue
						}
						fmt.Fprintln(cmd.OutOrStdout(), string(encoded))
					}
				}
			case config.IsMarkdown():
				for _, n := range nodes {
					entityKind := treeNodeKindToEntityKind(n.Kind)
					if entityKind == "" {
						entityKind = string(n.Kind)
					}
					if n.Data != nil {
						fmt.Fprintln(cmd.OutOrStdout(), renderEntityMarkdown(entityKind, n.Data))
					} else {
						label := n.Label
						if n.URI != "" {
							label = "[" + n.Label + "](" + n.URI + ")"
						}
						fmt.Fprintln(cmd.OutOrStdout(), "- "+label)
					}
				}
			default:
				for _, n := range nodes {
					entityKind := treeNodeKindToEntityKind(n.Kind)
					if entityKind != "" && n.Data != nil {
						fmt.Fprintln(cmd.OutOrStdout(), renderEntityHuman(entityKind, n.Data, false))
					} else {
						label := n.Label
						if n.URI != "" {
							label = "[" + n.Label + "](" + n.URI + ")"
						}
						fmt.Fprintln(cmd.OutOrStdout(), label)
					}
				}
			}
			return nil
		},
	}
	cmd.Flags().Int("limit", 0, "Maximum number of items to output (0 = unlimited)")
	bindTreeFlags(cmd)
	cmd.Flags().Bool("sorted", false, "Collect all results and sort by ID before output")
	return cmd
}

type boolFlagSpec struct {
	Name  string
	Usage string
}

type boolFlagPairSpec struct {
	OnlyName  string
	NoName    string
	OnlyUsage string
	NoUsage   string
}

// 🔘️bindBoolFlags MUST register all boolean flags from the input specs.
func bindBoolFlags(cmd *command.Command, specs []boolFlagSpec) {
	for _, spec := range specs {
		cmd.Flags().Bool(spec.Name, false, spec.Usage)
	}
}

// 🚩️bindOnlyNoFlagPairs MUST register only/no boolean flag pairs from the input specs.
func bindOnlyNoFlagPairs(cmd *command.Command, specs []boolFlagPairSpec) {
	for _, spec := range specs {
		cmd.Flags().Bool(spec.OnlyName, false, spec.OnlyUsage)
		cmd.Flags().Bool(spec.NoName, false, spec.NoUsage)
	}
}

func bindTreeFlags(cmd *command.Command) {
	bindOnlyNoFlagPairs(cmd, []boolFlagPairSpec{
		{OnlyName: "only-technology", NoName: "no-technology", OnlyUsage: "Only show technologies", NoUsage: "Exclude technologies"},
		{OnlyName: "only-bundle", NoName: "no-bundle", OnlyUsage: "Only show bundles", NoUsage: "Exclude bundles"},
		{OnlyName: "only-folder", NoName: "no-folder", OnlyUsage: "Only show folders", NoUsage: "Exclude folders"},
		{OnlyName: "only-file", NoName: "no-file", OnlyUsage: "Only show files", NoUsage: "Exclude files"},
		{OnlyName: "only-section", NoName: "no-section", OnlyUsage: "Only show sections", NoUsage: "Exclude sections"},
		{OnlyName: "only-definition", NoName: "no-definition", OnlyUsage: "Only show definitions", NoUsage: "Exclude definitions"},
		{OnlyName: "only-goal", NoName: "no-goal", OnlyUsage: "Only show goals", NoUsage: "Exclude goals"},
		{OnlyName: "only-ticket", NoName: "no-ticket", OnlyUsage: "Only show tickets", NoUsage: "Exclude tickets"},
		{OnlyName: "only-draft", NoName: "no-draft", OnlyUsage: "Only show drafts", NoUsage: "Exclude drafts"},
		{OnlyName: "only-policy", NoName: "no-policy", OnlyUsage: "Only show policies", NoUsage: "Exclude policies"},
		{OnlyName: "only-contributor", NoName: "no-contributor", OnlyUsage: "Only show contributors", NoUsage: "Exclude contributors"},
		{OnlyName: "only-checkpoint", NoName: "no-checkpoint", OnlyUsage: "Only show checkpoints", NoUsage: "Exclude checkpoints"},
		{OnlyName: "only-statute", NoName: "no-statute", OnlyUsage: "Only show statutes", NoUsage: "Exclude statutes"},
		{OnlyName: "only-todo", NoName: "no-todo", OnlyUsage: "Only show todos", NoUsage: "Exclude todos"},
		{OnlyName: "only-breach", NoName: "no-breach", OnlyUsage: "Only show breaches", NoUsage: "Exclude breaches"},
		{OnlyName: "only-library", NoName: "no-library", OnlyUsage: "Only show library bundles", NoUsage: "Exclude library bundles"},
		{OnlyName: "only-schema", NoName: "no-schema", OnlyUsage: "Only show schema bundles", NoUsage: "Exclude schema bundles"},
		{OnlyName: "only-binary", NoName: "no-binary", OnlyUsage: "Only show binary bundles", NoUsage: "Exclude binary bundles"},
		{OnlyName: "only-client", NoName: "no-client", OnlyUsage: "Only show client bundles", NoUsage: "Exclude client bundles"},
		{OnlyName: "only-site", NoName: "no-site", OnlyUsage: "Only show site bundles", NoUsage: "Exclude site bundles"},
		{OnlyName: "only-assets", NoName: "no-assets", OnlyUsage: "Only show asset bundles", NoUsage: "Exclude asset bundles"},
		{OnlyName: "only-organization", NoName: "no-organization", OnlyUsage: "Only show organization folders", NoUsage: "Exclude organization folders"},
		{OnlyName: "only-required", NoName: "no-required", OnlyUsage: "Only show required folders", NoUsage: "Exclude required folders"},
		{OnlyName: "only-code", NoName: "no-code", OnlyUsage: "Only show code files", NoUsage: "Exclude code files"},
		{OnlyName: "only-script", NoName: "no-script", OnlyUsage: "Only show script files", NoUsage: "Exclude script files"},
		{OnlyName: "only-config", NoName: "no-config", OnlyUsage: "Only show config files", NoUsage: "Exclude config files"},
		{OnlyName: "only-lab", NoName: "no-lab", OnlyUsage: "Only show lab files", NoUsage: "Exclude lab files"},
		{OnlyName: "only-docs", NoName: "no-docs", OnlyUsage: "Only show docs files", NoUsage: "Exclude docs files"},
		{OnlyName: "only-resource", NoName: "no-resource", OnlyUsage: "Only show resource files", NoUsage: "Exclude resource files"},
		{OnlyName: "only-template", NoName: "no-template", OnlyUsage: "Only show template files", NoUsage: "Exclude template files"},
		{OnlyName: "only-license", NoName: "no-license", OnlyUsage: "Only show license files", NoUsage: "Exclude license files"},
		{OnlyName: "only-implementation", NoName: "no-implementation", OnlyUsage: "Only show implementation definitions", NoUsage: "Exclude implementation definitions"},
		{OnlyName: "only-interface", NoName: "no-interface", OnlyUsage: "Only show interface definitions", NoUsage: "Exclude interface definitions"},
		{OnlyName: "only-constant", NoName: "no-constant", OnlyUsage: "Only show constant definitions", NoUsage: "Exclude constant definitions"},
	})

	bindBoolFlags(cmd, []boolFlagSpec{
		{Name: "only-open", Usage: "Only show open items"},
		{Name: "only-closed", Usage: "Only show closed items"},
		{Name: "open", Usage: "Only show open items"},
		{Name: "closed", Usage: "Only show closed items"},
	})

	cmd.Flags().IntSlice("only-year", nil, "Only show years")
	cmd.Flags().IntSlice("no-year", nil, "Exclude years")
	cmd.Flags().IntSlice("only-month", nil, "Only show months")
	cmd.Flags().IntSlice("no-month", nil, "Exclude months")
	cmd.Flags().IntSlice("only-day", nil, "Only show days")
	cmd.Flags().IntSlice("no-day", nil, "Exclude days")

	cmd.Flags().StringSlice("only-contributor-name", nil, "Only show specific contributors")
	cmd.Flags().StringSlice("no-contributor-name", nil, "Exclude specific contributors")
	cmd.Flags().StringSlice("only-policy-name", nil, "Only show specific policies")
	cmd.Flags().StringSlice("no-policy-name", nil, "Exclude specific policies")
	cmd.Flags().String("query", "", "Full-text search query")
}

// 🔍️queryCommand holds the data fields for a queryCommand record.
func queryCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "query [keywords]",
		Short: "Keyword search across monorepo resources",
		Args:  command.ArbitraryArgs,
		RunE: func(cmd *command.Command, args []string) error {
			ctx := cmd.Context()
			progress := search.NewProgressReporter(cmd.ErrOrStderr())
			query := strings.Join(args, " ")
			tree, err := BuildMonorepoTreeCached(ctx, TreeBuildOptions{IncludeSections: true, Progress: progress})
			if err != nil {
				return err
			}
			matched, err := searchMonorepoTreeWithCache(ctx, tree, query)
			if err != nil {
				return err
			}
			var nodes []*TreeNode
			flattenTreeNodes(matched, &nodes)
			for _, n := range nodes {
				id := n.ID
				if id == "" {
					id = n.Label
				}
				fmt.Fprintln(cmd.OutOrStdout(), id)
			}
			return nil
		},
	}
	return cmd
}

// 🧹️buildTreeFilterFromFlags holds the data fields for a buildTreeFilterFromFlags record.
func buildTreeFilterFromFlags(cmd *command.Command) TreeFilter {
	filter := TreeFilter{
		OnlyKinds:       make(map[TreeNodeKind]bool),
		ExcludeKinds:    make(map[TreeNodeKind]bool),
		OnlySubKinds:    make(map[TreeNodeKind][]string),
		ExcludeSubKinds: make(map[TreeNodeKind][]string),
	}

	kindFlags := []struct {
		onlyFlag string
		noFlag   string
		kind     TreeNodeKind
	}{
		{"only-technology", "no-technology", TreeNodeTechnology},
		{"only-bundle", "no-bundle", TreeNodeBundle},
		{"only-folder", "no-folder", TreeNodeFolder},
		{"only-file", "no-file", TreeNodeFile},
		{"only-section", "no-section", TreeNodeSection},
		{"only-definition", "no-definition", TreeNodeDefinition},
		{"only-goal", "no-goal", TreeNodeGoal},
		{"only-ticket", "no-ticket", TreeNodeTicket},
		{"only-draft", "no-draft", TreeNodeDraft},
		{"only-policy", "no-policy", TreeNodePolicy},
		{"only-contributor", "no-contributor", TreeNodeContributor},
		{"only-checkpoint", "no-checkpoint", TreeNodeCheckpoint},
		{"only-statute", "no-statute", TreeNodeStatute},
		{"only-todo", "no-todo", TreeNodeTodo},
		{"only-breach", "no-breach", TreeNodeBreach},
	}

	for _, kf := range kindFlags {
		if v, _ := cmd.Flags().GetBool(kf.onlyFlag); v {
			filter.OnlyKinds[kf.kind] = true
		}
		if v, _ := cmd.Flags().GetBool(kf.noFlag); v {
			filter.ExcludeKinds[kf.kind] = true
		}
	}

	type subKindFlag struct {
		onlyFlag string
		noFlag   string
		kind     TreeNodeKind
		value    string
	}
	subKindFlags := []subKindFlag{
		{"only-library", "no-library", TreeNodeBundle, string(BundleKindLibrary)},
		{"only-schema", "no-schema", TreeNodeBundle, string(BundleKindSchema)},
		{"only-binary", "no-binary", TreeNodeBundle, string(BundleKindBinary)},
		{"only-client", "no-client", TreeNodeBundle, string(BundleKindUI)},
		{"only-site", "no-site", TreeNodeBundle, string(BundleKindSite)},
		{"only-assets", "no-assets", TreeNodeBundle, string(BundleKindAssets)},
		{"only-organization", "no-organization", TreeNodeFolder, string(FolderKindOrganization)},
		{"only-code", "no-code", TreeNodeFile, FileKindCode},
		{"only-script", "no-script", TreeNodeFile, FileKindScript},
		{"only-config", "no-config", TreeNodeFile, FileKindConfig},
		{"only-lab", "no-lab", TreeNodeFile, FileKindLab},
		{"only-docs", "no-docs", TreeNodeFile, FileKindDocs},
		{"only-resource", "no-resource", TreeNodeFile, FileKindResource},
		{"only-template", "no-template", TreeNodeFile, FileKindTemplate},
		{"only-license", "no-license", TreeNodeFile, FileKindLicense},
		{"only-implementation", "no-implementation", TreeNodeDefinition, string(DefinitionKindImplementation)},
		{"only-interface", "no-interface", TreeNodeDefinition, string(DefinitionKindInterface)},
		{"only-constant", "no-constant", TreeNodeDefinition, string(DefinitionKindConstant)},
	}

	for _, sf := range subKindFlags {
		if v, _ := cmd.Flags().GetBool(sf.onlyFlag); v {
			filter.OnlySubKinds[sf.kind] = append(filter.OnlySubKinds[sf.kind], sf.value)
		}
		if v, _ := cmd.Flags().GetBool(sf.noFlag); v {
			filter.ExcludeSubKinds[sf.kind] = append(filter.ExcludeSubKinds[sf.kind], sf.value)
		}
	}

	if v, _ := cmd.Flags().GetBool("only-open"); v {
		filter.OnlyStatus = "open"
	} else if v, _ := cmd.Flags().GetBool("open"); v {
		filter.OnlyStatus = "open"
	}
	if v, _ := cmd.Flags().GetBool("only-closed"); v {
		filter.OnlyStatus = "closed"
	} else if v, _ := cmd.Flags().GetBool("closed"); v {
		filter.OnlyStatus = "closed"
	}

	filter.OnlyYears, _ = cmd.Flags().GetIntSlice("only-year")
	filter.ExcludeYears, _ = cmd.Flags().GetIntSlice("no-year")
	filter.OnlyMonths, _ = cmd.Flags().GetIntSlice("only-month")
	filter.ExcludeMonths, _ = cmd.Flags().GetIntSlice("no-month")
	filter.OnlyDays, _ = cmd.Flags().GetIntSlice("only-day")
	filter.ExcludeDays, _ = cmd.Flags().GetIntSlice("no-day")

	filter.OnlyContributors, _ = cmd.Flags().GetStringSlice("only-contributor-name")
	filter.ExcludeContributors, _ = cmd.Flags().GetStringSlice("no-contributor-name")
	filter.OnlyPolicies, _ = cmd.Flags().GetStringSlice("only-policy-name")
	filter.ExcludePolicies, _ = cmd.Flags().GetStringSlice("no-policy-name")
	filter.Query, _ = cmd.Flags().GetString("query")

	return filter
}

// 📤️exportCommand holds the data fields for a exportCommand record.
func exportCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "export [output]",
		Short: "Export repo data to an event log",
		Long:  `Export repo data as deterministic append-only events for replay.`,
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			outputPath := ""
			if len(args) > 0 {
				outputPath = args[0]
			}
			repoRoot := config.Repo
			if repoRoot == "" {
				repoRoot = findRepoRoot(".")
			}
			ctx := NewRepoContext(repoRoot)
			result, err := ExportToEventLogContext(cmd.Context(), outputPath, ctx, nil)
			if err != nil {
				return err
			}
			jsonBytes, err := json.MarshalIndent(result, "", "  ")
			if err != nil {
				return err
			}
			cmd.Println(string(jsonBytes))
			return nil
		},
	}
}
