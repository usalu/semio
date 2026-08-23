// 🧪️ `semio.tech/repo/test` — the Go native host of the repository test platform.
//
// The taxonomy filename `🐹️component.go` is not a name `go test` discovers, and committing a second
// `_test.go` wrapper next to it would create a duplicate test hierarchy. Instead the coordinator
// materializes a cache-local module whose generated entrypoint calls RunMain with the committed
// adapter's registration. Nothing here parses a feature file — the plan is the whole contract.
package test

//#region 🔖️Protocol

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"time"
)

// Fixture is one immutable fixture the coordinator resolved for this case.
type Fixture struct {
	URI    string `json:"uri"`
	Scope  string `json:"scope"`
	Name   string `json:"name"`
	Path   string `json:"path"`
	Digest string `json:"digest"`
}

// Step is one Given/When/Then step of a planned scenario.
type Step struct {
	Keyword string `json:"keyword"`
	Text    string `json:"text"`
}

// Scenario is one planned scenario, already expanded and level-filtered by the coordinator.
type Scenario struct {
	ID    string `json:"id"`
	Name  string `json:"name"`
	Level string `json:"level"`
	Mode  string `json:"mode"`
	Seed  string `json:"seed"`
	Steps []Step `json:"steps"`
}

// Plan is the owned execution plan one host receives.
type Plan struct {
	Owner          string     `json:"owner"`
	Case           string     `json:"case"`
	Capability     string     `json:"capability"`
	Comparison     string     `json:"comparison"`
	FeatureHash    string     `json:"featureHash"`
	Level          string     `json:"level"`
	Role           string     `json:"role"`
	Implementation string     `json:"implementation"`
	WorkDir        string     `json:"workDir"`
	OutputDir      string     `json:"outputDir"`
	ResultsPath    string     `json:"resultsPath"`
	Fixtures       []Fixture  `json:"fixtures"`
	Scenarios      []Scenario `json:"scenarios"`
}

// Diagnostic is one message attached to a result.
type Diagnostic struct {
	Severity string `json:"severity"`
	Message  string `json:"message"`
	Detail   string `json:"detail,omitempty"`
}

// Output is the artifact half of a result: hashes, cache-relative paths, and the projection.
type Output struct {
	RawHash        string `json:"rawHash"`
	ProjectionHash string `json:"projectionHash"`
	RawPath        string `json:"rawPath,omitempty"`
	ProjectionPath string `json:"projectionPath,omitempty"`
	Projection     any    `json:"projection"`
}

// Result is the single record shape every native host emits, one per (scenario, implementation, role).
type Result struct {
	TestID         string       `json:"testId"`
	Owner          string       `json:"owner"`
	Case           string       `json:"case"`
	Scenario       string       `json:"scenario"`
	Implementation string       `json:"implementation"`
	Role           string       `json:"role"`
	Level          string       `json:"level"`
	Status         string       `json:"status"`
	DurationMs     float64      `json:"durationMs"`
	Seed           string       `json:"seed,omitempty"`
	FeatureHash    string       `json:"featureHash,omitempty"`
	Output         Output       `json:"output"`
	Diagnostics    []Diagnostic `json:"diagnostics"`
}

//#endregion 🔖️Protocol

//#region 🔖️Digest

// Digest is the coordinator's content digest: sha256, hex, truncated to 32 characters.
func Digest(input []byte) string {
	sum := sha256.Sum256(input)
	return hex.EncodeToString(sum[:])[:32]
}

//#endregion 🔖️Digest

//#region 🔖️Adapter

// Context is everything one scenario handler is given.
type Context struct {
	Plan     *Plan
	Scenario *Scenario
	Role     string
	RepoRoot string
	WorkDir  string
}

// Fixture resolves a declared fixture URI to an absolute path; an undeclared URI is an error.
func (c *Context) Fixture(uri string) (string, error) {
	for _, fixture := range c.Plan.Fixtures {
		if fixture.URI == uri {
			return filepath.Join(c.RepoRoot, fixture.Path), nil
		}
	}
	return "", fmt.Errorf("fixture %s is not part of this plan — declare it in the feature file", uri)
}

// FixtureBytes reads a declared fixture.
func (c *Context) FixtureBytes(uri string) ([]byte, error) {
	path, err := c.Fixture(uri)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(path)
}

// CopyFixture copies an immutable fixture into the work directory and returns the mutable copy.
func (c *Context) CopyFixture(uri string, as string) (string, error) {
	bytes, err := c.FixtureBytes(uri)
	if err != nil {
		return "", err
	}
	if as == "" {
		source, _ := c.Fixture(uri)
		as = filepath.Base(source)
	}
	if err := os.MkdirAll(c.WorkDir, 0o755); err != nil {
		return "", err
	}
	target := filepath.Join(c.WorkDir, as)
	return target, os.WriteFile(target, bytes, 0o644)
}

// Seed is the scenario's deterministic seed.
func (c *Context) Seed() int64 {
	value, _ := strconv.ParseInt(c.Scenario.Seed, 10, 64)
	return value
}

// Outcome is what a scenario handler returns: the raw artifact and the compared projection.
type Outcome struct {
	Raw         []byte
	Projection  any
	Diagnostics []Diagnostic
}

// Handler runs one scenario in one role.
type Handler func(ctx *Context) (Outcome, error)

// Adapter is one implementation's registration for a case.
type Adapter struct {
	Implementation string
	handlers       map[string]Handler
}

// NewAdapter starts a registration for the given implementation id.
func NewAdapter(implementation string) *Adapter {
	return &Adapter{Implementation: implementation, handlers: map[string]Handler{}}
}

// Oracle registers the reference-implementation handler for one scenario.
func (a *Adapter) Oracle(scenario string, handler Handler) *Adapter {
	a.handlers[scenario+"::oracle"] = handler
	return a
}

// Subject registers this repository's handler for one scenario.
func (a *Adapter) Subject(scenario string, handler Handler) *Adapter {
	a.handlers[scenario+"::subject"] = handler
	return a
}

//#endregion 🔖️Adapter

//#region 🔖️Runner

func flagValue(argv []string, flag string) string {
	for index, value := range argv {
		if value == flag && index+1 < len(argv) {
			return argv[index+1]
		}
	}
	return ""
}

func repoRootFrom(start string) string {
	dir := start
	for i := 0; i < 32; i++ {
		if _, err := os.Stat(filepath.Join(dir, "nx.json")); err == nil {
			if _, err := os.Stat(filepath.Join(dir, "package.json")); err == nil {
				return dir
			}
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	working, _ := os.Getwd()
	return working
}

// RunMain is the Go host entry: load the plan, execute every planned scenario, emit JSONL.
// A missing registration and an error are both results — never a silent skip.
func RunMain(adapter *Adapter) {
	argv := os.Args[1:]
	planPath := flagValue(argv, "--plan")
	outPath := flagValue(argv, "--out")
	if planPath == "" || outPath == "" {
		fmt.Fprintln(os.Stderr, "usage: host --plan <plan.json> --out <results.jsonl>")
		os.Exit(2)
	}
	source, err := os.ReadFile(planPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "cannot read plan %s: %v\n", planPath, err)
		os.Exit(2)
	}
	var plan Plan
	if err := json.Unmarshal(source, &plan); err != nil {
		fmt.Fprintf(os.Stderr, "malformed plan %s: %v\n", planPath, err)
		os.Exit(2)
	}
	repoRoot := repoRootFrom(plan.WorkDir)
	_ = os.MkdirAll(plan.WorkDir, 0o755)
	_ = os.MkdirAll(plan.OutputDir, 0o755)

	body := ""
	failed := false
	for index := range plan.Scenarios {
		scenario := &plan.Scenarios[index]
		started := time.Now()
		result := Result{
			TestID:         fmt.Sprintf("%s::%s::%s::%s::%s", plan.Owner, plan.Case, scenario.ID, plan.Implementation, plan.Role),
			Owner:          plan.Owner,
			Case:           plan.Case,
			Scenario:       scenario.ID,
			Implementation: plan.Implementation,
			Role:           plan.Role,
			Level:          scenario.Level,
			Seed:           scenario.Seed,
			FeatureHash:    plan.FeatureHash,
			Diagnostics:    []Diagnostic{},
		}
		handler, registered := adapter.handlers[scenario.ID+"::"+plan.Role]
		if !registered {
			failed = true
			result.Status = "errored"
			result.Output = Output{RawHash: Digest(nil), ProjectionHash: Digest(nil)}
			result.Diagnostics = append(result.Diagnostics, Diagnostic{Severity: "error", Message: fmt.Sprintf("adapter has no %s registration for scenario %s", plan.Role, scenario.ID)})
		} else {
			outcome, runErr := handler(&Context{Plan: &plan, Scenario: scenario, Role: plan.Role, RepoRoot: repoRoot, WorkDir: plan.WorkDir})
			if runErr != nil {
				failed = true
				result.Status = "failed"
				result.Output = Output{RawHash: Digest(nil), ProjectionHash: Digest(nil)}
				result.Diagnostics = append(result.Diagnostics, Diagnostic{Severity: "error", Message: runErr.Error()})
			} else {
				projectionBytes, _ := json.Marshal(outcome.Projection)
				result.Status = "passed"
				result.Output = Output{RawHash: Digest(outcome.Raw), ProjectionHash: Digest(projectionBytes), Projection: outcome.Projection}
				if outcome.Raw != nil {
					rawPath := filepath.Join(plan.OutputDir, scenario.ID+"."+plan.Role+".raw")
					_ = os.WriteFile(rawPath, outcome.Raw, 0o644)
					result.Output.RawPath = rawPath
				}
				projectionPath := filepath.Join(plan.OutputDir, scenario.ID+"."+plan.Role+".projection.json")
				_ = os.WriteFile(projectionPath, projectionBytes, 0o644)
				result.Output.ProjectionPath = projectionPath
				if outcome.Diagnostics != nil {
					result.Diagnostics = outcome.Diagnostics
				}
			}
		}
		result.DurationMs = float64(time.Since(started).Milliseconds())
		line, _ := json.Marshal(result)
		body += string(line) + "\n"
	}

	_ = os.MkdirAll(filepath.Dir(outPath), 0o755)
	if err := os.WriteFile(outPath, []byte(body), 0o644); err != nil {
		fmt.Fprintf(os.Stderr, "cannot write results %s: %v\n", outPath, err)
		os.Exit(2)
	}
	if failed {
		os.Exit(1)
	}
}

//#endregion 🔖️Runner
