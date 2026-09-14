// 🐹️ Go side of the scope identifier case.
package adapter

import (
	"os"

	languages "github.com/usalu/semio/repo/languages"
	host "semio.tech/repo/test"
)

// #region 🔖️Projection

type vector struct {
	kind       string
	path       string
	section    string
	definition string
}

var vectors = []vector{
	{"file", "a/b.ts", "", ""},
	{"section", "a/b.ts", "Outer.Inner", ""},
	{"definition", "a/b.ts", "Outer", "alpha"},
	{"definition", "a/b.ts", "", "alpha"},
}

var sources = []string{"🟦️sample.ts", "🐹️sample.go", "📰️sample.md"}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

func scopeIDGrammar(_ *host.Context) (host.Outcome, error) {
	out := make([]any, 0, len(vectors))
	for _, v := range vectors {
		out = append(out, map[string]any{
			"kind":        v.kind,
			"sectionPath": v.section,
			"definition":  v.definition,
			"id":          languages.BuildScopeID(v.kind, v.path, v.section, v.definition),
		})
	}
	return host.Outcome{Projection: out}, nil
}

func scopesOfASourceFile(ctx *host.Context) (host.Outcome, error) {
	projection := map[string]any{}
	for _, name := range sources {
		path, err := ctx.Fixture("shared://" + name)
		if err != nil {
			return host.Outcome{}, err
		}
		content, err := os.ReadFile(path)
		if err != nil {
			return host.Outcome{}, err
		}
		scopes := languages.BuildScopesForFile(name, string(content))
		out := make([]any, 0, len(scopes))
		for _, s := range scopes {
			out = append(out, map[string]any{
				"kind":        s.Kind,
				"id":          s.ID,
				"sectionPath": s.SectionPath,
				"definition":  s.Definition,
				"startLine":   s.StartLine,
				"endLine":     s.EndLine,
			})
		}
		projection[name] = out
	}
	return host.Outcome{Projection: projection}, nil
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("scope-id-grammar", scopeIDGrammar).
		Subject("scopes-of-a-source-file", scopesOfASourceFile)
}

// #endregion 🔖️Registration
