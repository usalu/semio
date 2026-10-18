// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Bundle and folder commands with stream flag binding.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"sort"
	"strings"
)

// 📦️bundleCommand holds the data fields for a bundleCommand record.
func bundleCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "bundle", Short: "Bundle management commands"}
	listCmd := &command.Command{
		Use:   "list",
		Short: "List bundles",
		RunE: func(cmd *command.Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = getBundlesWithOpenTickets()
			}

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle list"}

				bundleChan := make(chan Bundle)
				go func() {
					StreamBundles(context.Background(), bundleChan, opts)
				}()

				for b := range bundleChan {

					if statusFilter != nil {
						hasOpenTicket := bundlesWithOpenTickets[b.Name]
						if *statusFilter == "open" && !hasOpenTicket {
							continue
						}
						if *statusFilter == "closed" && hasOpenTicket {
							continue
						}
					}

					data, err := json.Marshal(map[string]interface{}{"bundle": b})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "bundle list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	bindStatusFlags(listCmd)

	treeCmd := &command.Command{
		Use:   "tree",
		Short: "Show bundle tree",
		RunE: func(cmd *command.Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = getBundlesWithOpenTickets()
			}
			_ = bundlesWithOpenTickets

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle tree"}

				bundleChan := make(chan Bundle)
				go func() {
					StreamBundles(context.Background(), bundleChan, opts)
				}()

				var bundles []Bundle
				for b := range bundleChan {
					bundles = append(bundles, b)
				}

				sort.Slice(bundles, func(i, j int) bool {
					return bundles[i].Name < bundles[j].Name
				})

				if config.IsMarkdown() {
					var sb strings.Builder
					for _, b := range bundles {
						bMap := map[string]interface{}{}
						bBytes, _ := json.Marshal(b)
						json.Unmarshal(bBytes, &bMap)
						line := renderEntityMarkdown("bundle", bMap)
						sb.WriteString(line + "\n")
					}
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "bundle tree", Data: data}
				} else {
					stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: "."}
					for i, b := range bundles {
						connector := "├️─️─️ "
						if i == len(bundles)-1 {
							connector = "└️─️─️ "
						}
						stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: connector + b.Name}
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)
	bindStatusFlags(treeCmd)

	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

// 💠️folderCommand holds the data fields for a folderCommand record.
func folderCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "folder", Short: "Folder management commands"}
	createCmd := &command.Command{
		Use:   "create",
		Short: "Create a folder",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "Folder path")
	moveCmd := &command.Command{
		Use:   "move",
		Short: "Move a folder",
		Args:  command.MaximumNArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &command.Command{
		Use:   "delete",
		Short: "Delete a folder",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderDelete($path: String!) { folderDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "Folder path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

// 🔳️bindStreamFlags holds the data fields for a bindStreamFlags record.
func bindStreamFlags(cmd *command.Command) {
	bindBoolFlags(cmd, []boolFlagSpec{
		{Name: "show-ignored", Usage: "Show ignored folders and files"},
		{Name: "show-generated", Usage: "Show generated folders and files"},
	})

	bindOnlyNoFlagPairs(cmd, []boolFlagPairSpec{
		{OnlyName: "only-code", NoName: "no-code", OnlyUsage: "Only show code files", NoUsage: "Exclude code files"},
		{OnlyName: "only-script", NoName: "no-script", OnlyUsage: "Only show script files", NoUsage: "Exclude script files"},
		{OnlyName: "only-config", NoName: "no-config", OnlyUsage: "Only show config files", NoUsage: "Exclude config files"},
		{OnlyName: "only-lab", NoName: "no-lab", OnlyUsage: "Only show lab files", NoUsage: "Exclude lab files"},
		{OnlyName: "only-docs", NoName: "no-docs", OnlyUsage: "Only show docs files", NoUsage: "Exclude docs files"},
		{OnlyName: "only-resource", NoName: "no-resource", OnlyUsage: "Only show resource files", NoUsage: "Exclude resource files"},
		{OnlyName: "only-template", NoName: "no-template", OnlyUsage: "Only show template files", NoUsage: "Exclude template files"},
		{OnlyName: "only-license", NoName: "no-license", OnlyUsage: "Only show license files", NoUsage: "Exclude license files"},
		{OnlyName: "only-library", NoName: "no-library", OnlyUsage: "Only show library bundles", NoUsage: "Exclude library bundles"},
		{OnlyName: "only-schema", NoName: "no-schema", OnlyUsage: "Only show schema bundles", NoUsage: "Exclude schema bundles"},
		{OnlyName: "only-binary", NoName: "no-binary", OnlyUsage: "Only show binary bundles", NoUsage: "Exclude binary bundles"},
		{OnlyName: "only-client", NoName: "no-client", OnlyUsage: "Only show client bundles", NoUsage: "Exclude client bundles"},
		{OnlyName: "only-site", NoName: "no-site", OnlyUsage: "Only show site bundles", NoUsage: "Exclude site bundles"},
		{OnlyName: "only-assets", NoName: "no-assets", OnlyUsage: "Only show asset bundles", NoUsage: "Exclude asset bundles"},
		{OnlyName: "only-organization", NoName: "no-organization", OnlyUsage: "Only show organization folders", NoUsage: "Exclude organization folders"},
		{OnlyName: "only-required", NoName: "no-required", OnlyUsage: "Only show required folders", NoUsage: "Exclude required folders"},
		{OnlyName: "only-implementation", NoName: "no-implementation", OnlyUsage: "Only show implementation definitions", NoUsage: "Exclude implementation definitions"},
		{OnlyName: "only-interface", NoName: "no-interface", OnlyUsage: "Only show interface definitions", NoUsage: "Exclude interface definitions"},
		{OnlyName: "only-constant", NoName: "no-constant", OnlyUsage: "Only show constant definitions", NoUsage: "Exclude constant definitions"},
	})

	cmd.Flags().IntSlice("no-year", nil, "Exclude years")
	cmd.Flags().IntSlice("only-year", nil, "Only show years")
	cmd.Flags().IntSlice("no-month", nil, "Exclude months")
	cmd.Flags().IntSlice("only-month", nil, "Only show months")
	cmd.Flags().IntSlice("no-day", nil, "Exclude days")
	cmd.Flags().IntSlice("only-day", nil, "Only show days")

	cmd.Flags().StringSlice("no-contributor", nil, "Exclude contributors")
	cmd.Flags().StringSlice("only-contributor", nil, "Only show contributors")
	cmd.Flags().StringSlice("no-policy", nil, "Exclude policies")
	cmd.Flags().StringSlice("only-policy", nil, "Only show policies")
	cmd.Flags().StringSlice("no-breach", nil, "Exclude statutes")
	cmd.Flags().StringSlice("only-breach", nil, "Only show statutes")

	cmd.Flags().String("filter", "", "Filter string")
	cmd.Flags().String("query", "", "Full-text search query")
	cmd.Flags().Bool("regex", false, "Use regex for filter")
	cmd.Flags().Bool("match-case", false, "Match case for filter")
	cmd.Flags().Bool("match-whole-word", false, "Match whole word for filter")
}

// 🔲️bindStatusFlags holds the data fields for a bindStatusFlags record.
func bindStatusFlags(cmd *command.Command) {
	cmd.Flags().Bool("open", false, "Show only open items")
	cmd.Flags().Bool("closed", false, "Show only closed items")
	cmd.Flags().String("status", "", "Filter by status (open or closed)")
}

// ▪️getStatusFilter holds the data fields for a getStatusFilter record.
func getStatusFilter(cmd *command.Command) *string {
	open, _ := cmd.Flags().GetBool("open")
	closed, _ := cmd.Flags().GetBool("closed")
	status, _ := cmd.Flags().GetString("status")

	if open {
		s := "open"
		return &s
	}
	if closed {
		s := "closed"
		return &s
	}
	if status != "" {
		return &status
	}
	return nil
}

// 📬️getBundlesWithOpenTickets holds the data fields for a getBundlesWithOpenTickets record.
func getBundlesWithOpenTickets() map[string]bool {
	bundlesWithOpenTickets := make(map[string]bool)
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return bundlesWithOpenTickets
	}

	for _, t := range tickets {
		if t.Status != TicketStatusOpen {
			continue
		}
		for _, iter := range t.Interactions {
			for _, f := range iter.Files {
				parts := strings.Split(f.Path, "/")
				if len(parts) >= 2 && strings.HasPrefix(parts[0], "@") {
					bundleName := parts[0] + "/" + parts[1]
					bundlesWithOpenTickets[bundleName] = true
				}
			}
		}
	}
	return bundlesWithOpenTickets
}

// ▫️getStreamOptions holds the data fields for a getStreamOptions record.
func getStreamOptions(cmd *command.Command) StreamOptions {
	showIgnored, _ := cmd.Flags().GetBool("show-ignored")
	showGenerated, _ := cmd.Flags().GetBool("show-generated")

	var excludeKinds []string
	if v, _ := cmd.Flags().GetBool("no-code"); v {
		excludeKinds = append(excludeKinds, FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("no-script"); v {
		excludeKinds = append(excludeKinds, FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("no-config"); v {
		excludeKinds = append(excludeKinds, FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("no-lab"); v {
		excludeKinds = append(excludeKinds, FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("no-docs"); v {
		excludeKinds = append(excludeKinds, FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("no-resource"); v {
		excludeKinds = append(excludeKinds, FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("no-template"); v {
		excludeKinds = append(excludeKinds, FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("no-license"); v {
		excludeKinds = append(excludeKinds, FileKindLicense)
	}

	var includeKinds []string
	if v, _ := cmd.Flags().GetBool("only-code"); v {
		includeKinds = append(includeKinds, FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("only-script"); v {
		includeKinds = append(includeKinds, FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("only-config"); v {
		includeKinds = append(includeKinds, FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("only-lab"); v {
		includeKinds = append(includeKinds, FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("only-docs"); v {
		includeKinds = append(includeKinds, FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("only-resource"); v {
		includeKinds = append(includeKinds, FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("only-template"); v {
		includeKinds = append(includeKinds, FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("only-license"); v {
		includeKinds = append(includeKinds, FileKindLicense)
	}

	var excludeBundleKinds []BundleKind
	if v, _ := cmd.Flags().GetBool("no-library"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("no-schema"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("no-binary"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("no-client"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("no-site"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("no-assets"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindAssets)
	}

	var includeBundleKinds []BundleKind
	if v, _ := cmd.Flags().GetBool("only-library"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("only-schema"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("only-binary"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("only-client"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("only-site"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("only-assets"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindAssets)
	}

	var excludeFolderKinds []FolderKind
	if v, _ := cmd.Flags().GetBool("no-organization"); v {
		excludeFolderKinds = append(excludeFolderKinds, FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("no-required"); v {
		excludeFolderKinds = append(excludeFolderKinds, FolderKindRequired)
	}

	var includeFolderKinds []FolderKind
	if v, _ := cmd.Flags().GetBool("only-organization"); v {
		includeFolderKinds = append(includeFolderKinds, FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("only-required"); v {
		includeFolderKinds = append(includeFolderKinds, FolderKindRequired)
	}

	var excludeDefinitionKinds []DefinitionKind
	if v, _ := cmd.Flags().GetBool("no-implementation"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("no-interface"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("no-constant"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindConstant)
	}

	var includeDefinitionKinds []DefinitionKind
	if v, _ := cmd.Flags().GetBool("only-implementation"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("only-interface"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("only-constant"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindConstant)
	}

	excludeYears, _ := cmd.Flags().GetIntSlice("no-year")
	includeYears, _ := cmd.Flags().GetIntSlice("only-year")
	excludeMonths, _ := cmd.Flags().GetIntSlice("no-month")
	includeMonths, _ := cmd.Flags().GetIntSlice("only-month")
	excludeDays, _ := cmd.Flags().GetIntSlice("no-day")
	includeDays, _ := cmd.Flags().GetIntSlice("only-day")

	excludeContributors, _ := cmd.Flags().GetStringSlice("no-contributor")
	includeContributors, _ := cmd.Flags().GetStringSlice("only-contributor")
	excludePolicies, _ := cmd.Flags().GetStringSlice("no-policy")
	includePolicies, _ := cmd.Flags().GetStringSlice("only-policy")
	excludeBreachs, _ := cmd.Flags().GetStringSlice("no-breach")
	includeBreachs, _ := cmd.Flags().GetStringSlice("only-breach")

	filter, _ := cmd.Flags().GetString("filter")
	query, _ := cmd.Flags().GetString("query")
	regex, _ := cmd.Flags().GetBool("regex")
	matchCase, _ := cmd.Flags().GetBool("match-case")
	matchWholeWord, _ := cmd.Flags().GetBool("match-whole-word")

	return StreamOptions{
		ShowIgnored:            showIgnored,
		ShowGenerated:          showGenerated,
		ExcludeKinds:           excludeKinds,
		IncludeKinds:           includeKinds,
		ExcludeBundleKinds:     excludeBundleKinds,
		IncludeBundleKinds:     includeBundleKinds,
		ExcludeFolderKinds:     excludeFolderKinds,
		IncludeFolderKinds:     includeFolderKinds,
		ExcludeDefinitionKinds: excludeDefinitionKinds,
		IncludeDefinitionKinds: includeDefinitionKinds,
		ExcludeYears:           excludeYears,
		IncludeYears:           includeYears,
		ExcludeMonths:          excludeMonths,
		IncludeMonths:          includeMonths,
		ExcludeDays:            excludeDays,
		IncludeDays:            includeDays,
		ExcludeContributors:    excludeContributors,
		IncludeContributors:    includeContributors,
		ExcludePolicies:        excludePolicies,
		IncludePolicies:        includePolicies,
		ExcludeBreachs:         excludeBreachs,
		IncludeBreachs:         includeBreachs,
		Filter:                 filter,
		Query:                  query,
		Regex:                  regex,
		MatchCase:              matchCase,
		MatchWholeWord:         matchWholeWord,
	}
}
