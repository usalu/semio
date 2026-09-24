// 🐹️ Go side of the MCP client identity case, written against the same frozen vocabulary vectors as
// the Rust adapter — pairwise equivalence between two independently written implementations is the
// whole evidence this case has, so neither adapter reads the other's source.
package adapter

import (
	"encoding/json"
	"fmt"

	repo "github.com/usalu/semio/repo/providers"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

const identityVectors = "shared://🪪️mcp-client-kind-parse/🪪️client-kinds.json"

type identityVectorFile struct {
	Inputs          []string `json:"inputs"`
	Kinds           []string `json:"kinds"`
	ResolvedClients []string `json:"resolvedClients"`
}

func loadIdentityVectors(ctx *host.Context) (identityVectorFile, error) {
	var file identityVectorFile
	raw, err := ctx.FixtureBytes(identityVectors)
	if err != nil {
		return file, err
	}
	if err := json.Unmarshal(raw, &file); err != nil {
		return file, fmt.Errorf("client identity vectors are not valid JSON: %w", err)
	}
	return file, nil
}

func parsedKind(input string) string {
	kind, err := repo.ParseMcpClientKind(input)
	if err != nil {
		return "refused"
	}
	return string(kind)
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func everyAcceptedSpellingParsesToOneKind(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIdentityVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := []string{}
	for _, input := range file.Inputs {
		projection = append(projection, fmt.Sprintf("%q=%s", input, parsedKind(input)))
	}
	return host.Outcome{Projection: projection}, nil
}

func eachKindNamesOneServerAndOneHookClient(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIdentityVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := map[string]any{}
	for _, slug := range file.Kinds {
		kind, parseErr := repo.ParseMcpClientKind(slug)
		if parseErr != nil {
			return host.Outcome{}, parseErr
		}
		projection[slug] = map[string]any{"server": repo.McpServerName(kind), "hookClient": repo.HookClientForMcpKind(kind)}
	}
	return host.Outcome{Projection: projection}, nil
}

func aResolvedClientMapsBackToAKind(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIdentityVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	projection := []string{}
	for _, slug := range file.ResolvedClients {
		projection = append(projection, fmt.Sprintf("%q=%s", slug, string(repo.McpKindFromResolvedClient(slug))))
	}
	return host.Outcome{Projection: projection}, nil
}

func anUnknownSpellingIsRefused(ctx *host.Context) (host.Outcome, error) {
	file, err := loadIdentityVectors(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	refused := []string{}
	for _, input := range file.Inputs {
		if parsedKind(input) == "refused" {
			refused = append(refused, input)
		}
	}
	message := ""
	if _, unknownErr := repo.ParseMcpClientKind("unknown"); unknownErr != nil {
		message = unknownErr.Error()
	}
	return host.Outcome{Projection: map[string]any{"refused": refused, "messageForUnknown": message}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("every-accepted-spelling-parses-to-one-kind", everyAcceptedSpellingParsesToOneKind).
		Subject("each-kind-names-one-server-and-one-hook-client", eachKindNamesOneServerAndOneHookClient).
		Subject("a-resolved-client-maps-back-to-a-kind", aResolvedClientMapsBackToAKind).
		Subject("an-unknown-spelling-is-refused", anUnknownSpellingIsRefused)
}

// endregion 🔖️Registration
