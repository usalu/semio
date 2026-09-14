// 🐹️ Go side of the definition parsing case.
package adapter

import (
	"errors"
	"os"
	"strings"

	languages "github.com/usalu/semio/repo/languages"
	host "semio.tech/repo/test"
)

// #region 🔖️Projection

func declarations(ctx *host.Context, uri string) ([]any, error) {
	path, err := ctx.Fixture(uri)
	if err != nil {
		return nil, err
	}
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	content := string(raw)
	lang := languages.GetLanguageByName("typescript")
	if lang == nil {
		return nil, errors.New("typescript is not registered")
	}
	ranges := lang.ParseDefinitions(content, strings.Split(content, "\n"))
	out := make([]any, 0, len(ranges))
	for _, r := range ranges {
		out = append(out, map[string]any{"name": r.Name, "startLine": r.Start, "kind": r.Kind})
	}
	return out, nil
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

func typescriptTopLevelDeclarations(ctx *host.Context) (host.Outcome, error) {
	projection, err := declarations(ctx, "shared://🟦️sample.ts")
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: projection}, nil
}

func callableConstIsAFunction(ctx *host.Context) (host.Outcome, error) {
	projection, err := declarations(ctx, "local://🔤️callables.ts")
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
		Subject("typescript-top-level-declarations", typescriptTopLevelDeclarations).
		Subject("callable-const-is-a-function", callableConstIsAFunction)
}

// #endregion 🔖️Registration
