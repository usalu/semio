// 🐹️ Go side of the malformed region case.
package adapter

import (
	"os"
	"strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	host "semio.tech/repo/test"
)

// #region 🔖️Projection

func flat(sections []model.Section) []any {
	out := make([]any, 0, len(sections))
	for i := range sections {
		out = append(out, map[string]any{
			"name":      sections[i].Name,
			"emoji":     sections[i].Emoji,
			"startLine": sections[i].StartLine,
			"endLine":   sections[i].EndLine,
			"children":  flat(sections[i].Children),
		})
	}
	return out
}

func parse(ctx *host.Context, name string) (map[string]any, error) {
	path, err := ctx.Fixture("local://" + name)
	if err != nil {
		return nil, err
	}
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	content := string(raw)
	return map[string]any{
		"lineCount": len(strings.Split(content, "\n")),
		"sections":  flat(languages.ParseSections(content, name)),
	}, nil
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

func unclosedRegionRunsToTheEnd(ctx *host.Context) (host.Outcome, error) {
	projection, err := parse(ctx, "❌️unclosed.ts")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: projection}, nil
}

func strayEndregionIsIgnored(ctx *host.Context) (host.Outcome, error) {
	projection, err := parse(ctx, "❌️stray-end.ts")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: projection}, nil
}

func unclaimedExtensionHasNoSections(ctx *host.Context) (host.Outcome, error) {
	projection, err := parse(ctx, "❌️unclaimed.unknown")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: projection}, nil
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("unclosed-region-runs-to-the-end", unclosedRegionRunsToTheEnd).
		Subject("stray-endregion-is-ignored", strayEndregionIsIgnored).
		Subject("unclaimed-extension-has-no-sections", unclaimedExtensionHasNoSections)
}

// #endregion 🔖️Registration
