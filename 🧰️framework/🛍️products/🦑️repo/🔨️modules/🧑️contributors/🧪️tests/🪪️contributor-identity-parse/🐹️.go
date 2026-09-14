// 🐹️ Go side of the contributor identity case.
package adapter

import (
	"encoding/json"
	"fmt"

	contributors "github.com/usalu/semio/repo/contributors"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type contributorDocument struct {
	Directory string `json:"directory"`
	JSON      string `json:"json"`
}

type contributorDocumentFile struct {
	Schema    string                `json:"schema"`
	Documents []contributorDocument `json:"documents"`
}

type checkpointLogFile struct {
	Schema     string   `json:"schema"`
	Log        string   `json:"log"`
	Identities []string `json:"identities"`
	Malformed  []string `json:"malformed"`
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func authorLinesResolveToAliases(ctx *host.Context) (host.Outcome, error) {
	documentsData, err := ctx.FixtureBytes("shared://🧑️‍💻️contributor-documents.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var documentFile contributorDocumentFile
	if err := json.Unmarshal(documentsData, &documentFile); err != nil {
		return host.Outcome{}, err
	}
	logData, err := ctx.FixtureBytes("shared://🏁️checkpoint-log.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var logFile checkpointLogFile
	if err := json.Unmarshal(logData, &logFile); err != nil {
		return host.Outcome{}, err
	}

	seed := make([]contributors.StoredContributor, 0, len(documentFile.Documents))
	for _, entry := range documentFile.Documents {
		seed = append(seed, contributors.StoredContributor{Directory: entry.Directory, Document: entry.JSON})
	}
	store := contributors.NewMemoryContributorStore(seed)

	lines := append(append([]string{}, logFile.Identities...), logFile.Malformed...)
	identities := make([]string, 0, len(lines))
	aliases := make([]string, 0, len(lines))
	for _, line := range lines {
		name, email, ok := contributors.ParseContributorIdentity(line)
		if !ok {
			identities = append(identities, fmt.Sprintf("%s=-", line))
			aliases = append(aliases, fmt.Sprintf("%s=-", line))
			continue
		}
		identities = append(identities, fmt.Sprintf("%s=%s|%s", line, name, email))
		aliases = append(aliases, fmt.Sprintf("%s=%s", line, contributors.ResolveAuthorToAlias(store, name, email)))
	}

	people := contributors.ListContributors(store)
	list := make([]string, 0, len(people))
	authorLines := make([]string, 0, len(people)+len(lines))
	for _, person := range people {
		list = append(list, fmt.Sprintf("%s|%s|%s|%s|%d|%d", person.Alias, person.Github, person.Name, person.Email, len(person.Emails), len(person.Names)))
		authorLines = append(authorLines, fmt.Sprintf("%s <%s>", person.Name, person.Email))
	}
	authorLines = append(authorLines, lines...)
	authors := make([]string, 0, len(authorLines))
	for _, line := range authorLines {
		parsed := contributors.ParseGitAuthor(line)
		authors = append(authors, fmt.Sprintf("%s=%s|%s|%s", line, parsed.Name, parsed.Email, parsed.String()))
	}

	return host.Outcome{Projection: map[string]any{
		"identities":   identities,
		"aliases":      aliases,
		"contributors": list,
		"authors":      authors,
	}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("author-lines-resolve-to-aliases", authorLinesResolveToAliases)
}

// endregion 🔖️Registration
