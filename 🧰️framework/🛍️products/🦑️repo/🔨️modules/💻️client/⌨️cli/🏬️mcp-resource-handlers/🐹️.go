// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP resource handlers.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	mcp "github.com/usalu/semio/repo/client/internal/mcp"
	"strings"
)

// 📩️handleRepoResource holds the data fields for a handleRepoResource record.
func handleRepoResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Repo { repo { id name bundles { id } tickets { id } policies { id } contributors { id } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 💿️handleBundlesResource holds the data fields for a handleBundlesResource record.
func handleBundlesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Bundles { repo { bundles { id name root sourceRoot projectType tags kind } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📦️handleBundleResource holds the data fields for a handleBundleResource record.
func handleBundleResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid bundle URI: %s", request.Params.URI)
	}
	// Resolve bundle name from ID by walking bundles
	bundles := LoadBundles()
	norm := func(s string) string {
		r := strings.ReplaceAll(s, "\uFE0E", "")
		return strings.ReplaceAll(r, "\uFE0F", "")
	}
	var bundleName string
	for _, b := range bundles {
		if norm(b.GetID()) == norm(id) {
			bundleName = b.Name
			break
		}
	}
	if bundleName == "" {
		bundleName = id
	}
	query := `query Bundle($id: String!) { bundle(name: $id) { id name root sourceRoot projectType tags kind } }`
	result, err := gql(query, map[string]interface{}{"id": bundleName})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFoldersResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Folders { repo { folders { id path name kind } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFolderResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	path := IdToPath(id)
	query := `query Folder($path: String!) { folder(path: $path) { id path name kind parent { path } children { path name kind } files { path name kind } breachs { id } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFilesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Files { repo { files { id path name kind extension } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📄️handleFileResource holds the data fields for a handleFileResource record.
func handleFileResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	path := IdToPath(id)
	query := `query File($path: String!) { file(path: $path) { id path name kind extension folder { path } bundle { name } breachs { id } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleSectionsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	path := IdToPath(id)
	query := `query Sections($path: String!) { file(path: $path) { sections { id path name range { start end } definitions { name } children { name } } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📑️handleSectionResource holds the data fields for a handleSectionResource record.
func handleSectionResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid section URI: %s", request.Params.URI)
	}
	path := IdToPath(id)
	sectionSlugs := IdToSectionPath(id)
	if len(sectionSlugs) == 0 {
		return nil, fmt.Errorf("invalid section URI: %s", request.Params.URI)
	}
	sectionPath := make([]string, len(sectionSlugs))
	for i, s := range sectionSlugs {
		sectionPath[i] = TitleizeSlug(s)
	}

	query := `query Section($path: String!, $sectionPath: [String!]!) { section(path: $path, sectionPath: $sectionPath) { id path name range { start end } } }`
	result, err := gql(query, map[string]interface{}{"path": path, "sectionPath": sectionPath})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📖️handleDefinitionsResource holds the data fields for a handleDefinitionsResource record.
func handleDefinitionsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	path := IdToPath(id)
	query := `query Definitions($path: String!) { file(path: $path) { definitions { id name kind range { start end } } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔷️handleDefinitionResource holds the data fields for a handleDefinitionResource record.
func handleDefinitionResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid definition URI: %s", request.Params.URI)
	}
	path := IdToPath(id)
	defName := IdToDefinitionName(id)
	if defName == "" {
		return nil, fmt.Errorf("invalid definition URI: %s", request.Params.URI)
	}
	name := TitleizeSlug(defName)

	query := `query Definition($path: String!, $name: String!) { definition(path: $path, name: $name) { id name kind range { start end } } }`
	result, err := gql(query, map[string]interface{}{"path": path, "name": name})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🎫️handleTicketsResource holds the data fields for a handleTicketsResource record.
func handleTicketsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Tickets { repo { tickets { id slug title status prompt interactions { prompt } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔶️handleTicketResource holds the data fields for a handleTicketResource record.
func handleTicketResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid ticket URI: %s", request.Params.URI)
	}
	// Parse ticket ID to extract year/month/day/slug
	year, month, day, slug := ticketIdToParts(id)
	if slug == "" {
		return nil, fmt.Errorf("invalid ticket ID: %s", id)
	}

	query := `query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id slug title status prompt interactions { prompt dates { started finished } } } }`
	result, err := gql(query, map[string]interface{}{"year": year, "month": month, "day": day, "slug": slug})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleGoalsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	goals, err := ListGoals()
	if err != nil {
		return nil, err
	}
	bytes, err := json.Marshal(goals)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(string(bytes))
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// ⛳️handleGoalResource holds the data fields for a handleGoalResource record.
func handleGoalResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid goal URI: %s", request.Params.URI)
	}
	// Convert emoji goal ID back to goal path
	goalPath := composeIDToGoalPath(id)
	goals, err := ListGoals()
	if err != nil {
		return nil, err
	}
	for _, g := range goals {
		if g.ID == goalPath {
			bytes, err := json.Marshal(g)
			if err != nil {
				return nil, err
			}
			yaml, err := jsonToYaml(string(bytes))
			if err != nil {
				return nil, err
			}
			return []mcp.ResourceContents{
				mcp.TextResourceContents{
					URI:      request.Params.URI,
					MIMEType: "text/plain",
					Text:     yaml,
				},
			}, nil
		}
	}
	return nil, fmt.Errorf("goal not found: %s", goalPath)
}

// 🔹️handlePoliciesResource holds the data fields for a handlePoliciesResource record.
func handlePoliciesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Policies { repo { policies { id description breachs { id } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 📜️handlePolicyResource holds the data fields for a handlePolicyResource record.
func handlePolicyResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid policy URI: %s", request.Params.URI)
	}
	query := `query Policy($id: String!) { policy(id: $id) { id description breachs { id } } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔸️handleStatutesResource holds the data fields for a handleStatutesResource record.
func handleStatutesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Statutes { repo { statutes { id priority autofixable reason solution } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔺️handleStatuteResource holds the data fields for a handleStatuteResource record.
func handleStatuteResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid statute URI: %s", request.Params.URI)
	}
	query := `query Statute($id: String!) { statute(id: $id) { id priority autofixable reason solution } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🤝️handleContributorsResource holds the data fields for a handleContributorsResource record.
func handleContributorsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Contributors { repo { contributors { id emails name contributions { checkpoints { id } tickets { id } } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 🔻️handleContributorResource holds the data fields for a handleContributorResource record.
func handleContributorResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := UriToId(request.Params.URI)
	if id == "" {
		return nil, fmt.Errorf("invalid contributor URI: %s", request.Params.URI)
	}
	query := `query Contributor($id: String!) { contributor(id: $id) { id emails name contributions { checkpoints { id } tickets { id } } } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

// 💾️handleCheckpointsResource holds the data fields for a handleCheckpointsResource record.
func handleCheckpointsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Checkpoints { repo { checkpoints { id sha title date } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleCheckpointResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	return nil, fmt.Errorf("checkpoint resource not implemented")
}
