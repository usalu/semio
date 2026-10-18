// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🌊️stream carries the stream and status flag binding of the cli and the stream options.

// #endregion 🧲️Header

package cli

import (
	model "github.com/usalu/semio/repo/model"
)

// #region 🕸️Test Command

// 🔳️bindStreamFlags holds the data fields for a bindStreamFlags record.
func bindStreamFlags(cmd *Command) {
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
func bindStatusFlags(cmd *Command) {
	cmd.Flags().Bool("open", false, "Show only open items")
	cmd.Flags().Bool("closed", false, "Show only closed items")
	cmd.Flags().String("status", "", "Filter by status (open or closed)")
}

// ▪️getStatusFilter holds the data fields for a getStatusFilter record.
func getStatusFilter(cmd *Command) *string {
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

// ▫️getStreamOptions holds the data fields for a getStreamOptions record.
func getStreamOptions(cmd *Command) model.StreamOptions {
	showIgnored, _ := cmd.Flags().GetBool("show-ignored")
	showGenerated, _ := cmd.Flags().GetBool("show-generated")

	var excludeKinds []string
	if v, _ := cmd.Flags().GetBool("no-code"); v {
		excludeKinds = append(excludeKinds, model.FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("no-script"); v {
		excludeKinds = append(excludeKinds, model.FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("no-config"); v {
		excludeKinds = append(excludeKinds, model.FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("no-lab"); v {
		excludeKinds = append(excludeKinds, model.FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("no-docs"); v {
		excludeKinds = append(excludeKinds, model.FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("no-resource"); v {
		excludeKinds = append(excludeKinds, model.FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("no-template"); v {
		excludeKinds = append(excludeKinds, model.FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("no-license"); v {
		excludeKinds = append(excludeKinds, model.FileKindLicense)
	}

	var includeKinds []string
	if v, _ := cmd.Flags().GetBool("only-code"); v {
		includeKinds = append(includeKinds, model.FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("only-script"); v {
		includeKinds = append(includeKinds, model.FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("only-config"); v {
		includeKinds = append(includeKinds, model.FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("only-lab"); v {
		includeKinds = append(includeKinds, model.FileKindLab)
	}
	if v, _ := cmd.Flags().GetBool("only-docs"); v {
		includeKinds = append(includeKinds, model.FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("only-resource"); v {
		includeKinds = append(includeKinds, model.FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("only-template"); v {
		includeKinds = append(includeKinds, model.FileKindTemplate)
	}
	if v, _ := cmd.Flags().GetBool("only-license"); v {
		includeKinds = append(includeKinds, model.FileKindLicense)
	}

	var excludeBundleKinds []model.BundleKind
	if v, _ := cmd.Flags().GetBool("no-library"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("no-schema"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("no-binary"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("no-client"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("no-site"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("no-assets"); v {
		excludeBundleKinds = append(excludeBundleKinds, model.BundleKindAssets)
	}

	var includeBundleKinds []model.BundleKind
	if v, _ := cmd.Flags().GetBool("only-library"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("only-schema"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("only-binary"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("only-client"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("only-site"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("only-assets"); v {
		includeBundleKinds = append(includeBundleKinds, model.BundleKindAssets)
	}

	var excludeFolderKinds []model.FolderKind
	if v, _ := cmd.Flags().GetBool("no-organization"); v {
		excludeFolderKinds = append(excludeFolderKinds, model.FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("no-required"); v {
		excludeFolderKinds = append(excludeFolderKinds, model.FolderKindRequired)
	}

	var includeFolderKinds []model.FolderKind
	if v, _ := cmd.Flags().GetBool("only-organization"); v {
		includeFolderKinds = append(includeFolderKinds, model.FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("only-required"); v {
		includeFolderKinds = append(includeFolderKinds, model.FolderKindRequired)
	}

	var excludeDefinitionKinds []model.DefinitionKind
	if v, _ := cmd.Flags().GetBool("no-implementation"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("no-interface"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("no-constant"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, model.DefinitionKindConstant)
	}

	var includeDefinitionKinds []model.DefinitionKind
	if v, _ := cmd.Flags().GetBool("only-implementation"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("only-interface"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("only-constant"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, model.DefinitionKindConstant)
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

	return model.StreamOptions{
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

// #endregion 🕸️Test Command
