// 🧪️ `semio.tech/repo/test` — the Go native host of the repository test platform.
//
// The taxonomy filename `🐹️.go` is not a name `go test` discovers, and committing a second
// `_test.go` wrapper next to it would create a duplicate test hierarchy. Instead the coordinator
// materializes a cache-local module whose generated entrypoint calls RunMain with the committed
// adapter's registration. Nothing here parses a feature file — the plan is the whole contract.
package test

