// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎯️test_definitions carries the test definition collection and selector matching.

// #endregion 🧲️Header

package codebase

import (
	regexp "regexp"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
)

// #region 🎯️TestDefinitions

func CollectTestDefinitionsFromContent(content string, normalized string) []model.Definition {
	definitions := languages.ParseDefinitions(content, normalized)
	var tests []model.Definition
	seen := map[string]bool{}
	for i := range definitions {
		if definitions[i].Kind == "" && statutes.IsTestOrBenchmarkFile(normalized) {
			definitions[i].Kind = model.DefinitionKindTest
		}
		if definitions[i].Kind != model.DefinitionKindTest {
			continue
		}
		id := definitions[i].GetID()
		if id == "" || seen[id] {
			continue
		}
		seen[id] = true
		tests = append(tests, definitions[i])
	}
	if len(tests) > 0 {
		return tests
	}
	return fallbackTestDefinitionsFromContent(content, normalized)
}

func fallbackTestDefinitionsFromContent(content string, normalized string) []model.Definition {
	patterns := []*regexp.Regexp{
		regexp.MustCompile(`(?m)^[\t ]*func[\t ]+(Test[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*def[\t ]+(test_[A-Za-z0-9_]+)\s*\(`),
		regexp.MustCompile(`(?m)^[\t ]*(?:it|test)\s*\(?\s*['"]([^'"]+)['"]`),
		regexp.MustCompile(`(?m)^[\t ]*public[\t ]+function[\t ]+(test[A-Za-z0-9_]+)\s*\(`),
	}
	var tests []model.Definition
	seen := map[string]bool{}
	for _, pattern := range patterns {
		matches := pattern.FindAllStringSubmatch(content, -1)
		for _, match := range matches {
			if len(match) < 2 {
				continue
			}
			name := strings.TrimSpace(match[1])
			if name == "" {
				continue
			}
			definition := model.Definition{Name: name, Kind: model.DefinitionKindTest, FilePath: normalized}
			id := definition.GetID()
			if id == "" || seen[id] {
				continue
			}
			seen[id] = true
			tests = append(tests, definition)
		}
	}
	return tests
}

func TestSelectorMatchesDefinitionName(selector string, definitionName string) bool {
	selector = strings.TrimSpace(selector)
	selector = strings.Trim(selector, `"'`)
	if selector == "" || definitionName == "" {
		return false
	}
	selectorLower := strings.ToLower(selector)
	definitionLower := strings.ToLower(definitionName)
	if selectorLower == definitionLower || strings.Contains(definitionLower, selectorLower) {
		return true
	}
	pattern, err := regexp.Compile(selector)
	if err == nil && pattern.MatchString(definitionName) {
		return true
	}
	quoted, quoteErr := regexp.Compile(regexp.QuoteMeta(selector))
	return quoteErr == nil && quoted.MatchString(definitionName)
}

// #endregion 🎯️TestDefinitions
