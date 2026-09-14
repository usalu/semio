// 🐹️ Go side of the golden-tree analysis case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"sort"
	"strings"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	host "semio.tech/repo/test"
)

// region 🔖️Tree

// goldenTree is the data table of the-golden-tree-breaches, as the feature file states it.
var goldenTree = []string{
	"shared://📁️some/📁️folder/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file/🐍️.py",
	"shared://📁️some/📁️folder/🧪️file/🔷️.cs",
	"shared://📁️some/📁️folder/🧪️file/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file-fixable/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file-fixed/🐍️.py",
	"shared://📁️some/📁️folder/🧪️file-fixed/🐹️.go",
	"shared://📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
	"shared://📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file-invalid/🐍️.py",
	"shared://📁️some/📁️folder/🧪️file-invalid/🐹️.go",
	"shared://📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
	"shared://📁️some/📁️folder/🧪️file-invalid/🟦️.tsx",
}

// cleanTree is the data table of the-clean-sources-are-clean, as the feature file states it.
var cleanTree = []string{
	"shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
	"shared://📁️some/📁️folder/🧪️file-fixed/🐍️.py",
	"shared://📁️some/📁️folder/🧪️file-fixed/🐹️.go",
	"shared://📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
	"shared://📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
}

// tree reads the declared fixture URIs as repository-relative sources, ordered by path.
func tree(ctx *host.Context, uris []string) (*statutes.SourceSet, error) {
	files := make([]statutes.SourceFile, 0, len(uris))
	for _, uri := range uris {
		path, err := ctx.Fixture(uri)
		if err != nil {
			return nil, err
		}
		content, err := os.ReadFile(path)
		if err != nil {
			return nil, fmt.Errorf("cannot read %s: %w", uri, err)
		}
		files = append(files, statutes.SourceFile{Path: strings.TrimPrefix(uri, "shared://"), Content: string(content)})
	}
	sort.Slice(files, func(i, j int) bool { return files[i].Path < files[j].Path })
	return statutes.NewSourceSet(files), nil
}

// render renders one breach as the tuple both implementations must agree on.
func render(breach model.Breach) string {
	return fmt.Sprintf("%s|%s|%s|%d|%d|%t|%s", breach.ID, string(breach.Kind), breach.Scope, breach.Line, breach.Column, statutes.IsAutofixable(breach.Kind), breach.Summary)
}

// renderAll renders every breach of one analysis.
func renderAll(breachs []model.Breach) []string {
	rows := make([]string, 0, len(breachs))
	for _, breach := range breachs {
		rows = append(rows, render(breach))
	}
	return rows
}

// endregion 🔖️Tree

// region 🔖️Scenarios

func theGoldenTreeBreaches(ctx *host.Context) (host.Outcome, error) {
	sources, err := tree(ctx, goldenTree)
	if err != nil {
		return host.Outcome{}, err
	}
	rendered := renderAll(statutes.Analyze(sources))
	data, err := ctx.FixtureBytes("local://🔣️breaches.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var golden struct {
		Breachs []string `json:"breachs"`
	}
	if err := json.Unmarshal(data, &golden); err != nil {
		return host.Outcome{}, err
	}
	if len(golden.Breachs) != len(rendered) {
		return host.Outcome{}, fmt.Errorf("analysis drifted from the reviewed golden: %d breaches produced, %d expected", len(rendered), len(golden.Breachs))
	}
	for index := range rendered {
		if rendered[index] != golden.Breachs[index] {
			return host.Outcome{}, fmt.Errorf("analysis drifted from the reviewed golden at %d: produced %s, expected %s", index, rendered[index], golden.Breachs[index])
		}
	}
	return host.Outcome{Projection: map[string]any{"breachs": rendered}}, nil
}

func theCleanSourcesAreClean(ctx *host.Context) (host.Outcome, error) {
	sources, err := tree(ctx, cleanTree)
	if err != nil {
		return host.Outcome{}, err
	}
	rendered := renderAll(statutes.Analyze(sources))
	if len(rendered) != 0 {
		return host.Outcome{}, fmt.Errorf("a repaired source raised %d breaches", len(rendered))
	}
	return host.Outcome{Projection: map[string]any{"breachs": rendered}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-golden-tree-breaches", theGoldenTreeBreaches).
		Subject("the-clean-sources-are-clean", theCleanSourcesAreClean)
}

// endregion 🔖️Registration
