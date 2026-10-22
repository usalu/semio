// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧱️executor carries the executor, the resolver, the root-bound executor and the CLI entry points.

// #endregion 🧲️Header

package graphql

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	sync "sync"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📦️Utils

var (
	executorMutex  sync.Mutex
	activeExecutor *Executor
	executorRoot   string
)

// 🔗️boundExecutor returns the executor of the explicitly bound repository root and rebuilds it whenever the root changed.
// 🔗️It refuses to answer while no root is bound, so no query can fall back to the repository the process happens to run in.
func boundExecutor() (*Executor, error) {
	root, bound := workspace.LookupRootDir()
	if !bound {
		return nil, fmt.Errorf("repository root is not bound: call workspace.BindWorkingDirectory or workspace.SetRootDir first")
	}
	executorMutex.Lock()
	defer executorMutex.Unlock()
	if activeExecutor == nil || executorRoot != root {
		built, err := NewExecutorWithContext(root, NewRepoContext(root))
		if err != nil {
			activeExecutor, executorRoot = nil, ""
			return nil, fmt.Errorf("failed to initialize GraphQL executor: %w", err)
		}
		activeExecutor, executorRoot = built, root
	}
	return activeExecutor, nil
}

// #endregion 📦️Utils

// #region 💡️GraphQL Resolver

// 💿️Resolver holds the data fields for a resolver record.
type Resolver struct {
	RootDir string
	Ctx     model.RepoContext
}

// 🔷️NewResolver MUST initialize all required fields and return a valid resolver.
// 🆕️NewResolver creates and returns a new resolver instance.
func NewResolver(rootDir string) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: NewRepoContext(rootDir)}
}

// 📝️NewResolverWithContext MUST initialize all required fields and return a valid resolver with context.
// 🔤️NewResolverWithContext creates and returns a new resolver with context instance.
func NewResolverWithContext(rootDir string, ctx model.RepoContext) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: ctx}
}

// 🔶️context holds the data fields for a context record.
func (r *Resolver) context() model.RepoContext {
	return r.Ctx
}

// #endregion 💡️GraphQL Resolver

// #region 🧱️GraphQL Executor

/** 🧱️ One executable schema bound to one repository context. */
type Executor struct {
	resolver *Resolver
	schema   Schema
}

/** 🔷️ Binds the repo schema to a filesystem-backed context rooted at one directory. */
func NewExecutor(rootDir string) (*Executor, error) {
	resolved, err := workspace.SelectRepoRoot(".", &rootDir)
	if err != nil { return nil, err }
	return NewExecutorWithContext(resolved, NewRepoContext(resolved))
}

/** 🔷️ Binds the repo schema to a context. */
func NewExecutorWithContext(rootDir string, ctx model.RepoContext) (*Executor, error) {
	resolver := NewResolverWithContext(rootDir, ctx)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{resolver: resolver, schema: schema}, nil
}

/** 🗺️ The schema this executor serves. */
func (e *Executor) Schema() Schema { return e.schema }

// 🧭️Repository returns this executor's bound repository port.
func (e *Executor) Repository() model.RepoContext { return e.resolver.context() }

/** ⚡️ Parses and executes one request, returning the `data` payload. */
func (e *Executor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	result := Do(Params{
		Context:        ctx,
		Schema:         e.schema,
		Repo:           e.resolver.context(),
		RequestString:  query,
		VariableValues: variables,
	})
	if len(result.Errors) > 0 {
		return nil, fmt.Errorf("graphql errors: [%s]", result.Errors[0].Error())
	}
	return result.Data, nil
}

/** 📋️ Executes one request and renders the `data` payload the way the CLI prints it. */
func (e *Executor) ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error) {
	data, err := e.Execute(ctx, query, variables)
	if err != nil {
		return "", err
	}
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", err
	}
	return string(jsonBytes), nil
}

/** 🔍️ Reports whether a request string is a document this grammar accepts. */
func (e *Executor) ValidateQuery(query string) error {
	return Validate(query)
}

/** 📨️ The operation kind of a request string, without executing it. */
func (e *Executor) GetOperationType(query string) (string, error) {
	return OperationType(query)
}

// #endregion 🧱️GraphQL Executor

// #region 🎙️GraphQL

// 💿️gql holds the data fields for a gql record.
func Gql(query string, variables map[string]interface{}) (string, error) {
	bound, err := boundExecutor()
	if err != nil {
		return "", err
	}
	return bound.ExecuteJSON(context.Background(), query, variables)
}

// #endregion 🎙️GraphQL

// #region 🌦️GraphQL Helpers

// 💿️printGQL holds the data fields for a printGQL record.
func PrintGQL(query string, variables map[string]interface{}) error {
	result, err := Gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// #endregion 🌦️GraphQL Helpers

// #region 🔊️Cli

// 💿️repoResolverInstance holds the data fields for a repoResolverInstance record.
var RepoResolverInstance *Resolver

// #endregion 🔊️Cli

// #region 🔌️GraphQL Ports

// 🔌️init wires the repository port 📐️model declares but cannot implement.
func init() {
	model.LookupRepoContext = NewRepoContext
}

// #endregion 🔌️GraphQL Ports
