// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Header, comment, and requirements policies.

// #endregion 🧲️Header

package client

import (
	"fmt"
	glob "github.com/usalu/semio/repo/client/internal/glob"
	"path/filepath"
	"regexp"
	"strings"
)

// ✔️CheckPolicies MUST run all applicable policies and aggregate breachs.
// ✔️CheckPolicies validates the policies and returns any breachs.
func CheckPolicies(scope Scope, bundles []Bundle, policyIDs []string) ([]Breach, error) {
	ctx := NewPolicyContext(scope, bundles)
	return CheckPoliciesWithContext(ctx, policyIDs)
}

// 🟨CheckPoliciesWithContext MUST run all applicable policies and aggregate breachs.
// 🔑CheckPoliciesWithContext validates the policies with context and returns any breachs.
func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Breach, error) {
	var breachs []Breach
	var policiesToRun []PolicyDef
	if len(policyIDs) > 0 {
		for _, p := range policies {
			for _, id := range policyIDs {
				if p.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range policies {
			if matchesScope(p.Scopes, ctx.Scope) {
				policiesToRun = append(policiesToRun, p)
			}
		}
	}
	for _, policy := range policiesToRun {
		policyBreachs := policy.Run(ctx)
		breachs = append(breachs, policyBreachs...)
	}
	return breachs, nil
}

// 🔭matchesScope holds the data fields for a matchesScope record.
func matchesScope(policyScopes []string, targetScope Scope) bool {
	for _, pattern := range policyScopes {
		if pattern == "*" || pattern == "**/*" {
			return true
		}
		if strings.HasPrefix(pattern, "compose") {
			if targetScope.Kind == ScopeRepo || (targetScope.Kind == ScopeTechnology && strings.HasPrefix(targetScope.TechnologyName, pattern)) {
				return true
			}
		}
		if targetScope.Kind == ScopeRepo && strings.HasPrefix(pattern, "**/*.") {
			return true
		}
		if targetScope.FilePath != "" {
			normalizedTarget := NormalizePath(targetScope.FilePath)
			normalizedPattern := NormalizePath(pattern)
			if matched, _ := glob.Match(normalizedPattern, normalizedTarget); matched {
				return true
			}
		}
	}
	return false
}

// 🟩headerPolicy holds the data fields for a headerPolicy record.
func headerPolicy(ctx *PolicyContext) []Breach {
	var breachs []Breach
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
		language := GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing header region in %s", file),
				BreachCodeFileMissingHeaderRegion,
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
				BreachCodeFileMissingContributors,
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
				BreachCodeFileMissingLicense,
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
					BreachCodeFileWrongLicense,
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
			if isSpecText(commentText) {
				continue
			}
			if strings.HasPrefix(commentText, "TODO:") {
				continue
			}
			hasSummary = true
			break
		}
		if !hasSummary && !isTestOrBenchmarkFile(file) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing summary in header of %s", file),
				BreachCodeFileMissingSummary,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🧪️isTestOrBenchmarkFile reports whether a path conventionally contains tests or benchmarks.
func isTestOrBenchmarkFile(file string) bool {
	if DeriveFileKind(filepath.Base(file)) == FileKindLab {
		return true
	}
	normalized := strings.ToLower(filepath.ToSlash(file))
	if strings.Contains(normalized, "/tests/") || strings.Contains(normalized, ".tests/") ||
		strings.Contains(normalized, "/test/") || strings.Contains(normalized, ".test/") ||
		strings.Contains(normalized, "/benchmark/") || strings.Contains(normalized, ".benchmark/") {
		return true
	}
	name := filepath.Base(normalized)
	return strings.HasSuffix(name, "_test.go") ||
		strings.HasSuffix(name, ".test.ts") ||
		strings.HasSuffix(name, ".test.tsx") ||
		strings.HasSuffix(name, ".test.js") ||
		strings.HasSuffix(name, ".test.jsx") ||
		strings.HasSuffix(name, ".spec.ts") ||
		strings.HasSuffix(name, ".spec.tsx") ||
		strings.HasSuffix(name, ".spec.js") ||
		strings.HasSuffix(name, ".spec.jsx") ||
		strings.HasPrefix(name, "test_") ||
		strings.Contains(name, "benchmark")
}

// 📤isExportedDefinition holds the data fields for a isExportedDefinition record.
func isExportedDefinition(name string, line string, langName string) bool {
	return true
}

// 🟦requiresDefinitionRequirements holds the data fields for a requiresDefinitionRequirements record.
func requiresDefinitionRequirements(line string, langName string) bool {
	trimmed := strings.TrimSpace(line)
	switch langName {
	case "typescript":
		noExport := strings.TrimPrefix(trimmed, "export ")
		noExport = strings.TrimLeft(noExport, "async abstract declare default ")
		return strings.HasPrefix(noExport, "function ") || strings.HasPrefix(noExport, "class ")
	case "go":
		return strings.HasPrefix(trimmed, "func ")
	case "python":
		return strings.HasPrefix(trimmed, "def ") || strings.HasPrefix(trimmed, "class ") || strings.HasPrefix(trimmed, "async def ")
	case "csharp":
		return !strings.Contains(trimmed, " interface ") && !strings.Contains(trimmed, " enum ")
	case "rust":
		noPrefix := strings.TrimPrefix(trimmed, "pub ")
		if strings.HasPrefix(noPrefix, "(") {
			idx := strings.Index(noPrefix, ") ")
			if idx >= 0 {
				noPrefix = strings.TrimSpace(noPrefix[idx+2:])
			}
		}
		return strings.HasPrefix(noPrefix, "fn ") || strings.HasPrefix(noPrefix, "struct ") || strings.HasPrefix(noPrefix, "impl ") || strings.HasPrefix(noPrefix, "trait ")
	}
	return true
}

// 🟫commentPolicy holds the data fields for a commentPolicy record.
func commentPolicy(ctx *PolicyContext) []Breach {
	var breachs []Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := GetLanguage(file)
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
func requirementsPolicy(ctx *PolicyContext) []Breach {
	var breachs []Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		lines := strings.Split(content, "\n")
		sections := ctx.Sections(file)
		prefix := language.CommentPrefix()
		var headerSection *Section
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
								BreachCodeRequirementsSyntax,
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
					if !isSpecText(commentText) {
						continue
					}
					if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
							BreachCodeRequirementsSyntax,
							fmt.Sprintf("%s#Header", file), i, 0, commentText))
					}
				}
			}
		}
		var checkSectionRequirements func(s Section)
		checkSectionRequirements = func(s Section) {
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
				if !isSpecText(commentText) {
					break
				}
				if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
						BreachCodeRequirementsSyntax,
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
