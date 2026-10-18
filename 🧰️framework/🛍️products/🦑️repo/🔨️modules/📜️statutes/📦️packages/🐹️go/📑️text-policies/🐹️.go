package statutes

import (
	fmt "fmt"
	regexp "regexp"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
)

// #region 📑️Text Policies

// 🟩headerPolicy holds the data fields for a headerPolicy record.
func headerPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	agplMarkers := []string{"GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *model.Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing header region in %s", file),
				model.BreachCodeFileMissingHeaderRegion,
				file, 0, 0, ""))
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		headerLines := strings.Split(headerContent, "\n")
		contributorPattern := regexp.MustCompile(`\d{4}\s+[\w\s]+<[\w.@-]+>`)
		hasContributors := false
		for _, line := range headerLines {
			if contributorPattern.MatchString(line) {
				hasContributors = true
				break
			}
		}
		if !hasContributors {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing contributors in header of %s", file),
				model.BreachCodeFileMissingContributors,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		}
		hasLicense := false
		for _, marker := range agplMarkers {
			if strings.Contains(headerContent, marker) {
				hasLicense = true
				break
			}
		}
		if !hasLicense {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing license in header of %s", file),
				model.BreachCodeFileMissingLicense,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		} else {
			wrongLicenses := []string{"MIT", "Apache", "BSD"}
			hasWrongLicense := false
			for _, wrong := range wrongLicenses {
				if strings.Contains(headerContent, wrong) {
					hasWrongLicense = true
					break
				}
			}
			if strings.Contains(headerContent, "GPL") && !strings.Contains(headerContent, "AGPL") && !strings.Contains(headerContent, "LGPL") {
				hasWrongLicense = true
			}
			if hasWrongLicense {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Wrong license in header of %s", file),
					model.BreachCodeFileWrongLicense,
					fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
			}
		}
		commentPrefix := language.CommentPrefix()
		hasSummary := false
		seenLicense := false
		licenseEnd := false
		for _, line := range headerLines {
			trimmed := strings.TrimSpace(line)
			if trimmed == "" {
				if seenLicense && !licenseEnd {
					licenseEnd = true
				}
				continue
			}
			if strings.HasPrefix(trimmed, commentPrefix+" #region") || strings.HasPrefix(trimmed, commentPrefix+" #endregion") {
				continue
			}
			if !strings.HasPrefix(trimmed, commentPrefix) {
				continue
			}
			commentText := strings.TrimSpace(strings.TrimPrefix(trimmed, commentPrefix))
			if commentText == "" {
				if seenLicense && !licenseEnd {
					licenseEnd = true
				}
				continue
			}

			if contributorPattern.MatchString(line) {
				continue
			}
			isLicenseLine := false
			for _, marker := range agplMarkers {
				if strings.Contains(line, marker) {
					seenLicense = true
					isLicenseLine = true
					break
				}
			}
			if isLicenseLine {
				continue
			}
			if !licenseEnd && seenLicense && (strings.Contains(commentText, "without") || strings.Contains(commentText, "program") || strings.Contains(commentText, "License") || strings.Contains(commentText, "http") || strings.Contains(commentText, "version") || strings.Contains(commentText, "terms") || strings.Contains(commentText, "WARRANTY") || strings.Contains(commentText, "PURPOSE") || strings.Contains(commentText, "General Public") || strings.Contains(commentText, "redistribute") || strings.Contains(commentText, "published") || strings.Contains(commentText, "Foundation") || strings.Contains(commentText, "received")) {
				continue
			}
			if IsSpecText(commentText) {
				continue
			}
			if strings.HasPrefix(commentText, "TODO:") {
				continue
			}
			hasSummary = true
			break
		}
		if !hasSummary && !IsTestOrBenchmarkFile(file) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing summary in header of %s", file),
				model.BreachCodeFileMissingSummary,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🟫commentPolicy holds the data fields for a commentPolicy record.
func commentPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsComments() {
			continue
		}

		lines := strings.Split(content, "\n")
		langBreachs := language.ScanComments(ctx, file, content, lines)
		breachs = append(breachs, langBreachs...)
	}
	return ctx.FilterIgnored(breachs)
}

// 💠truncate holds the data fields for a truncate record.
func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

// 🔳️requirementsPolicy holds the data fields for a requirementsPolicy record.
func requirementsPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		lines := strings.Split(content, "\n")
		sections := ctx.Sections(file)
		prefix := language.CommentPrefix()
		var headerSection *model.Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection != nil {
			requirementsChildFound := false
			for _, child := range headerSection.Children {
				if strings.ToLower(child.Name) == "requirements" {
					requirementsChildFound = true
					for i := child.StartLine + 1; i < child.EndLine && i <= len(lines); i++ {
						line := strings.TrimSpace(lines[i-1])
						if line == "" {
							continue
						}
						commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						if commentText == "" {
							continue
						}
						if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
								model.BreachCodeRequirementsSyntax,
								fmt.Sprintf("%s#Header/Requirements", file), i, 0, commentText))
						}
					}
					break
				}
			}
			if !requirementsChildFound {
				for i := headerSection.StartLine + 1; i < headerSection.EndLine && i <= len(lines); i++ {
					line := strings.TrimSpace(lines[i-1])
					if line == "" {
						continue
					}
					if !strings.HasPrefix(line, prefix) {
						continue
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
					if commentText == "" {
						continue
					}
					if !IsSpecText(commentText) {
						continue
					}
					if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
							model.BreachCodeRequirementsSyntax,
							fmt.Sprintf("%s#Header", file), i, 0, commentText))
					}
				}
			}
		}
		var checkSectionRequirements func(s model.Section)
		checkSectionRequirements = func(s model.Section) {
			if strings.ToLower(s.Name) == "header" {
				return
			}
			for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
				line := strings.TrimSpace(lines[i-1])
				if line == "" {
					continue
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				if !IsSpecText(commentText) {
					break
				}
				if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
						model.BreachCodeRequirementsSyntax,
						fmt.Sprintf("%s#%s", file, s.Name), i, 0, commentText))
				}
			}
			for _, child := range s.Children {
				checkSectionRequirements(child)
			}
		}
		for _, s := range sections {
			checkSectionRequirements(s)
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🔲️codePolicy holds the data fields for a codePolicy record.
func codePolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	breachs = append(breachs, headerPolicy(ctx)...)
	breachs = append(breachs, sectionPolicy(ctx)...)
	breachs = append(breachs, commentPolicy(ctx)...)
	breachs = append(breachs, requirementsPolicy(ctx)...)
	breachs = append(breachs, emojiPolicy(ctx)...)
	breachs = append(breachs, docsPolicy(ctx)...)
	return breachs
}

// #endregion 📑️Text Policies
