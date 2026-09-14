// 🐹️ Go side of the ranked search case.
package adapter

import (
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"

	search "github.com/usalu/semio/repo/search"
	host "semio.tech/repo/test"
)

// region 🔖️Vectors

type matchTerm struct {
	Term      string `json:"term"`
	Fuzziness int    `json:"fuzziness"`
}

type rankingVector struct {
	Name      string            `json:"name"`
	Documents map[string]string `json:"documents"`
	Deleted   []string          `json:"deleted"`
	Reindexed map[string]string `json:"reindexed"`
	Terms     []matchTerm       `json:"terms"`
	Size      int               `json:"size"`
}

type rankingVectorFile struct {
	Schema  string          `json:"schema"`
	Vectors []rankingVector `json:"vectors"`
}

func sortedKeys(values map[string]string) []string {
	keys := make([]string, 0, len(values))
	for key := range values {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	return keys
}

// endregion 🔖️Vectors

// region 🔖️Scenarios

func vectorsRankTheSameWay(ctx *host.Context) (host.Outcome, error) {
	data, err := ctx.FixtureBytes("shared://📡️ranking-vectors.json")
	if err != nil {
		return host.Outcome{}, err
	}
	var file rankingVectorFile
	if err := json.Unmarshal(data, &file); err != nil {
		return host.Outcome{}, err
	}
	rankings := make([]string, 0, len(file.Vectors))
	for _, vector := range file.Vectors {
		index, err := search.NewMemOnly(search.NewIndexMapping())
		if err != nil {
			return host.Outcome{}, err
		}
		for _, id := range sortedKeys(vector.Documents) {
			if err := index.Index(id, map[string]interface{}{"text": vector.Documents[id]}); err != nil {
				return host.Outcome{}, err
			}
		}
		for _, id := range vector.Deleted {
			if err := index.Delete(id); err != nil {
				return host.Outcome{}, err
			}
		}
		for _, id := range sortedKeys(vector.Reindexed) {
			if err := index.Index(id, map[string]interface{}{"text": vector.Reindexed[id]}); err != nil {
				return host.Outcome{}, err
			}
		}
		queries := make([]search.Query, 0, len(vector.Terms))
		for _, term := range vector.Terms {
			query := search.NewMatchQuery(term.Term)
			query.SetFuzziness(term.Fuzziness)
			queries = append(queries, query)
		}
		request := search.NewSearchRequest(search.NewConjunctionQuery(queries...))
		request.Size = vector.Size
		result, err := index.Search(request)
		if err != nil {
			return host.Outcome{}, err
		}
		hits := make([]string, 0, len(result.Hits))
		for _, hit := range result.Hits {
			hits = append(hits, fmt.Sprintf("%s@%s", hit.ID, strconv.FormatFloat(hit.Score, 'g', -1, 64)))
		}
		rankings = append(rankings, fmt.Sprintf("%s=%d:%s", vector.Name, result.Total, strings.Join(hits, ",")))
	}
	return host.Outcome{Projection: map[string]any{"rankings": rankings}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").Subject("vectors-rank-the-same-way", vectorsRankTheSameWay)
}

// endregion 🔖️Registration
