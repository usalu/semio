// 🐹️ Go side of the section parsing case. Reads every fixture through the production parsers so the
// projection is produced by production code, never restated in the adapter.
package adapter

import (
	"os"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	host "semio.tech/repo/test"
)

// #region 🔖️Projection

var markerFixtures = []string{
	"🟦️sample.ts",
	"🐹️sample.go",
	"🐍️sample.py",
	"🔷️sample.cs",
	"🦀️sample.rs",
	"💎️sample.rb",
	"🐚️sample.sh",
	"📢️sample.toml",
	"🤸️sample.yaml",
	"🕌️sample.sql",
	"🎙️sample.graphql",
}

func ranged(sections []model.Section) []any {
	out := make([]any, 0, len(sections))
	for i := range sections {
		out = append(out, map[string]any{
			"name":       sections[i].Name,
			"emoji":      sections[i].Emoji,
			"startLine":  sections[i].StartLine,
			"endLine":    sections[i].EndLine,
			"startIndex": sections[i].StartIndex,
			"endIndex":   sections[i].EndIndex,
			"children":   ranged(sections[i].Children),
		})
	}
	return out
}

func started(sections []model.Section) []any {
	out := make([]any, 0, len(sections))
	for i := range sections {
		out = append(out, map[string]any{
			"name":       sections[i].Name,
			"startLine":  sections[i].StartLine,
			"startIndex": sections[i].StartIndex,
			"children":   started(sections[i].Children),
		})
	}
	return out
}

func read(ctx *host.Context, name string) (string, error) {
	path, err := ctx.Fixture("shared://" + name)
	if err != nil {
		return "", err
	}
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

func markerRegionsAcrossLanguages(ctx *host.Context) (host.Outcome, error) {
	projection := map[string]any{}
	for _, name := range markerFixtures {
		content, err := read(ctx, name)
		if err != nil {
			return host.Outcome{}, err
		}
		projection[name] = ranged(languages.ParseSections(content, name))
	}
	return host.Outcome{Projection: projection}, nil
}

func markdownHeadingRanges(ctx *host.Context) (host.Outcome, error) {
	content, err := read(ctx, "📰️sample.md")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: ranged(languages.ParseMarkdownSectionsInternal(content))}, nil
}

func jsonObjectKeyTree(ctx *host.Context) (host.Outcome, error) {
	content, err := read(ctx, "🔣️sample.json")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: started(languages.ParseJSONSections(content))}, nil
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("marker-regions-across-languages", markerRegionsAcrossLanguages).
		Subject("markdown-heading-ranges", markdownHeadingRanges).
		Subject("json-object-key-tree", jsonObjectKeyTree)
}

// #endregion 🔖️Registration
