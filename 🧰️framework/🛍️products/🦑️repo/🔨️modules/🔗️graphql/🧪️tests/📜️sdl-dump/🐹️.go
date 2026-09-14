// 🐹️ Go side of the sdl-dump case. It reports the inventory of the schema the executor builds in
// code — never of the committed document, which is the oracle's input, not the subject's.
package adapter

import (
	graphql "github.com/usalu/semio/repo/graphql"
	host "semio.tech/repo/test"
)

// region 🔖️Scenarios

func servedSchemaMatchesTheCommittedSdl(_ *host.Context) (host.Outcome, error) {
	return host.Outcome{Projection: graphql.SchemaInventory(graphql.BuildSchema())}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("served-schema-matches-the-committed-sdl", servedSchemaMatchesTheCommittedSdl)
}

// endregion 🔖️Registration
