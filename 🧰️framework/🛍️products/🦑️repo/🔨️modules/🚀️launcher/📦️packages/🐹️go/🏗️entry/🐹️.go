// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏗️entry plans the process route of one repo command and starts the implementation: build verbs, the OS workspace, and plain scripts all run through the registry's runtime.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
)

// #region 🏗️Entry

// 🧭️Plan is the process route of one argument vector: its kind and the unchanged arguments the route receives.
type Plan struct {
	Kind      string
	Arguments []string
}

// 🏗️BuildVerbsAt reads the verbs that enter the owned build implementation from the neutral operations catalogue named by the registry, so no verb list is kept by hand.
func BuildVerbsAt(root string, registry *Registry) ([]string, error) {
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(registry.Routing.Catalogue)))
	if err != nil {
		return nil, err
	}
	var catalogue struct {
		Operations []struct {
			Name string `json:"name"`
		} `json:"operations"`
	}
	if err := json.Unmarshal(raw, &catalogue); err != nil {
		return nil, err
	}
	verbs := make([]string, 0, len(catalogue.Operations))
	for _, operation := range catalogue.Operations {
		verbs = append(verbs, operation.Name)
	}
	return verbs, nil
}

var (
	verbsLock  sync.Mutex
	verbsDone  bool
	verbsFound []string
	verbsRoot  string
)

// 🔗️BindRoot makes the build verbs come from the repository around dir instead of the working directory; test binaries whose working directory is a sandbox bind their origin.
func BindRoot(dir string) {
	verbsLock.Lock()
	defer verbsLock.Unlock()
	verbsRoot, verbsDone, verbsFound = dir, false, nil
}

// 🏗️BuildVerbs answers the build verbs of the repository around the bound directory or the working directory, read once; it answers none outside a repository.
func BuildVerbs() []string {
	verbsLock.Lock()
	defer verbsLock.Unlock()
	if verbsDone {
		return verbsFound
	}
	verbsDone = true
	start := verbsRoot
	if start == "" {
		var err error
		if start, err = StartDirectory(os.Getenv, os.Getwd); err != nil {
			return nil
		}
	}
	root, err := ResolveRoot(start)
	if err != nil {
		return nil
	}
	if registry, err := LoadRegistry(root); err == nil {
		verbsFound, _ = BuildVerbsAt(root, registry)
	}
	return verbsFound
}

// 📜️ScriptExtensions mark an argument as a script that the runtime runs.
var ScriptExtensions = []string{".ts", ".mjs", ".js"}

// 🎚️ScriptOptions are the neutral options that may precede a script; the runtime maps them to its own flags.
var ScriptOptions = []string{"watch", "cwd"}

// 📍️DashboardArguments is the authored run target that `repo dashboard` expands to.
var DashboardArguments = []string{"run", "@semio-tech/repo-dashboard-rs:run", "--output=stream", "--"}

func contains(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

func isScript(argument string) bool {
	for _, extension := range ScriptExtensions {
		if strings.HasSuffix(argument, extension) {
			return true
		}
	}
	return false
}

func scriptIndex(argv []string) int {
	index := 0
	for index < len(argv) {
		switch {
		case argv[index] == "--watch":
			index++
		case argv[index] == "--cwd":
			index += 2
		case strings.HasPrefix(argv[index], "--cwd="):
			index++
		default:
			if isScript(argv[index]) {
				return index
			}
			return -1
		}
	}
	return -1
}

// 🎯️PlanRoute plans with the build verbs of the surrounding repository.
func PlanRoute(argv []string) Plan { return PlanRouteWith(BuildVerbs(), argv) }

// 🎯️PlanRouteWith selects the process route before any domain flag parsing and never rewrites the arguments of the build, workspace, script and domain routes.
func PlanRouteWith(verbs []string, argv []string) Plan {
	arguments := append([]string{}, argv...)
	if len(argv) >= 4 && argv[0] == "dev" && argv[1] == "mcp" && argv[2] == "stdio" && argv[3] == "os" {
		return Plan{"workspace", arguments}
	}
	prefix := 0
	for prefix < len(argv) {
		if argv[prefix] == "--repo" && prefix+1 < len(argv) {
			prefix += 2
		} else if strings.HasPrefix(argv[prefix], "--repo=") {
			prefix++
		} else {
			break
		}
	}
	if prefix > 0 && prefix < len(argv) && argv[prefix] == "mcp" {
		return Plan{"mcp", append(append([]string{}, argv[:prefix]...), argv[prefix+1:]...)}
	}
	if len(argv) > 0 && argv[0] == "mcp" {
		return Plan{"mcp", arguments[1:]}
	}
	if len(argv) > 0 && argv[0] == "dashboard" {
		return Plan{"build", append(append([]string{}, DashboardArguments...), argv[1:]...)}
	}
	if len(argv) == 0 || argv[0] == "--help" || argv[0] == "-h" {
		return Plan{"build", arguments}
	}
	if scriptIndex(argv) >= 0 {
		return Plan{"script", arguments}
	}
	index := 0
	for index < len(argv) {
		if argv[index] == "--tool" {
			index += 2
			continue
		}
		if strings.HasPrefix(argv[index], "--tool=") {
			index++
			continue
		}
		if strings.HasPrefix(argv[index], "--service=") {
			index++
			continue
		}
		break
	}
	if index > 0 || (index < len(argv) && contains(verbs, argv[index])) {
		return Plan{"build", arguments}
	}
	return Plan{"domain", arguments}
}

// 🧩️LaunchArguments builds the runtime's argument vector: executable, its fixed leading arguments, the mapped script options, the script, the entry prefix and the caller's arguments.
func LaunchArguments(executable string, rt Runtime, flags []string, script string, prefix, args []string) []string {
	argv := append(append([]string{executable}, rt.Launch.Before...), flags...)
	argv = append(argv, script)
	argv = append(argv, prefix...)
	return append(argv, args...)
}

// 🎚️ScriptArguments maps the neutral leading options of a script route to the runtime's flags and splits the script from its arguments.
func ScriptArguments(rt Runtime, args []string) (flags []string, script string, rest []string, ok bool) {
	index := scriptIndex(args)
	if index < 0 {
		return nil, "", nil, false
	}
	for position := 0; position < index; position++ {
		name, value := strings.TrimPrefix(args[position], "--"), ""
		if before, after, found := strings.Cut(name, "="); found {
			name, value = before, after
		} else if name == "cwd" {
			position++
			value = args[position]
		}
		for _, flag := range rt.Launch.Options[name] {
			flags = append(flags, strings.ReplaceAll(flag, "{value}", value))
		}
	}
	return flags, args[index], args[index+1:], true
}

// 🛤️WithRuntimeOnPath prepends directories to the PATH entry of an environment list and applies fixed variables.
func WithRuntimeOnPath(environ []string, directories []string, fixed map[string]string) []string {
	key := "PATH"
	for _, entry := range environ {
		if name, _, found := strings.Cut(entry, "="); found && strings.EqualFold(name, "PATH") && runtime.GOOS == "windows" {
			key = name
		}
	}
	joined := strings.Join(directories, string(os.PathListSeparator))
	out := make([]string, 0, len(environ)+len(fixed)+1)
	seen := false
	for _, entry := range environ {
		name, value, found := strings.Cut(entry, "=")
		if found && name == key {
			seen = true
			if value != "" {
				entry = key + "=" + joined + string(os.PathListSeparator) + value
			} else {
				entry = key + "=" + joined
			}
		}
		if _, overridden := fixed[name]; found && overridden {
			continue
		}
		out = append(out, entry)
	}
	if !seen {
		out = append(out, key+"="+joined)
	}
	for name, value := range fixed {
		out = append(out, name+"="+value)
	}
	return out
}

// 🔁️RunChild starts the runtime as a child that shares the console, forwards interrupts and settles with its exit code; a non-zero exit code is returned as ExitError. Windows hands over this way, and a caller that continues after the runtime finished (the help of the full command) selects it as its HandOff.
func RunChild(executable string, argv []string, env []string) error {
	command := exec.Command(executable, argv[1:]...)
	command.Stdin, command.Stdout, command.Stderr = os.Stdin, os.Stdout, os.Stderr
	command.Env = env
	interrupts := make(chan os.Signal, 2)
	signal.Notify(interrupts, os.Interrupt)
	defer signal.Stop(interrupts)
	if err := command.Start(); err != nil {
		return err
	}
	done := make(chan struct{})
	defer close(done)
	go func() {
		for {
			select {
			case received := <-interrupts:
				_ = command.Process.Signal(received)
			case <-done:
				return
			}
		}
	}()
	err := command.Wait()
	var exited *exec.ExitError
	if errors.As(err, &exited) {
		code := exited.ExitCode()
		if code < 0 {
			code = 130
		}
		return ExitError{Code: code}
	}
	return err
}

// 🏠️Host holds every effect the launcher has so that tests replace them; Forward is the hook of a resident build service: it answers handled=true with the exit code when it ran the command, and handled=false (silently) to continue with the normal start.
type Host struct {
	Env     Environment
	Environ func() []string
	Stdout  func(text string)
	Stderr  func(text string)
	Getwd   func() (string, error)
	System  func() []string
	Target  func() (platform, arch string, musl bool)
	HandOff func(executable string, argv []string, env []string) error
	Provide func(ProvisionRequest) ProvisionRequest
	Self    func() string
	Forward func(ctx context.Context, root, kind string, args []string) (handled bool, code int)
}

// 🏠️DefaultHost wires the host to the real process.
func DefaultHost() Host {
	host := Host{
		Env:     os.Getenv,
		Environ: os.Environ,
		Stdout:  func(text string) { _, _ = os.Stdout.WriteString(text + "\n") },
		Stderr:  func(text string) { _, _ = os.Stderr.WriteString(text + "\n") },
		Getwd:   os.Getwd,
		System:  systemLanguages,
		Target:  HostTarget,
		HandOff: handOff,
		Self:    func() string { path, _ := os.Executable(); return path },
	}
	host.Forward = host.ForwardResident
	return host
}

// 🗣️speaker chooses the language lazily so that the silent hot path never reads the language protocol.
type speaker struct {
	host     Host
	root     string
	registry *Registry
	args     []string
	language string
	refused  bool
}

func (s *speaker) choose() bool {
	if s.language != "" {
		return true
	}
	if s.refused {
		return false
	}
	protocol, err := LoadLanguageProtocol(s.root, s.registry)
	if err != nil {
		s.refused = true
		s.host.Stderr(builtin(Builtin["languageProtocol"], map[string]string{"path": s.registry.Language.Protocol}))
		return false
	}
	resolution, refusal := protocol.ResolveLanguage(s.registry.Languages, s.args, s.host.Env, s.host.System)
	if refusal != nil {
		s.refused = true
		s.host.Stderr(s.registry.Refusal(refusal))
		return false
	}
	s.language = resolution.Language
	return true
}

func (s *speaker) text(id string, values map[string]string) string {
	return s.registry.Say(s.language, id, values)
}

func (s *speaker) fail(err error) int {
	var failure Failure
	if asFailure(err, &failure) {
		if failure.ID == "cancelled" {
			if s.choose() {
				s.host.Stderr(s.text("cancelled", nil))
			}
			return 130
		}
		if s.choose() {
			s.host.Stderr(s.text(failure.ID, failure.Values))
		}
		return 2
	}
	if s.choose() {
		s.host.Stderr(s.text("unexpected", map[string]string{"reason": err.Error()}))
	}
	return 1
}

// 🚀️Execute starts the entry named by the plan kind (build, workspace) or a script through the registry runtime; it returns only on failure, on Windows after the child exited, or in tests.
func (h Host) Execute(ctx context.Context, root, kind string, args []string) int {
	registry, err := LoadRegistry(root)
	if err != nil {
		return h.ReportRegistry(err)
	}
	talk := &speaker{host: h, root: root, registry: registry, args: args}
	if kind == "script" && h.namesVersionOfBuildEntry(root, registry, args) {
		return h.Version(args)
	}
	var entry Entry
	script, prefix, rest, flags := "", []string(nil), args, []string(nil)
	if kind != "script" {
		found, ok := registry.Implementation.Entries[kind]
		if !ok {
			return talk.failWith("entryUnknown", map[string]string{"name": kind, "declared": strings.Join(sortedKeys(registry.Implementation.Entries), ", ")})
		}
		entry, script, prefix = found, filepath.Join(root, filepath.FromSlash(found.Script)), found.Prefix
		if !isPlainFile(script) {
			return talk.failWith("entryMissing", map[string]string{"path": script})
		}
	}
	if h.Forward != nil && kind == "build" {
		if handled, code := h.Forward(ctx, root, kind, args); handled {
			return code
		}
	}
	name, rt, err := registry.Pick(entry, h.Env)
	if err != nil {
		var unknown UnknownError
		asUnknown(err, &unknown)
		return talk.failWith("runtimeUnknown", map[string]string{"name": unknown.Name, "declared": strings.Join(unknown.Declared, ", ")})
	}
	if kind == "script" {
		var ok bool
		if flags, script, rest, ok = ScriptArguments(rt, args); !ok {
			return talk.failWith("entryMissing", map[string]string{"path": strings.Join(args, " ")})
		}
	}
	platform, arch, musl := h.Target()
	request := ProvisionRequest{Root: root, Name: name, Registry: registry, Runtime: rt, Platform: platform, Arch: arch, Musl: musl, Env: h.Env, Progress: func(id string, values map[string]string) {
		if talk.choose() {
			h.Stderr(talk.text(id, values))
		}
	}}
	if h.Provide != nil {
		request = h.Provide(request)
	}
	if _, present, locateFailure := request.Locate(); !present && (locateFailure != nil || !talk.choose()) {
		if locateFailure != nil {
			return talk.fail(locateFailure)
		}
		return 2
	}
	install, err := request.Ensure(ctx)
	if err != nil {
		return talk.fail(err)
	}
	argv := LaunchArguments(install.Executable, rt, flags, script, prefix, rest)
	directories := []string{install.Directory}
	if self := h.Self(); self != "" && filepath.Dir(self) != install.Directory {
		directories = append(directories, filepath.Dir(self))
	}
	if err := h.HandOff(install.Executable, argv, WithRuntimeOnPath(h.Environ(), directories, rt.Launch.Environment)); err != nil {
		var exited ExitError
		if asExit(err, &exited) {
			return exited.Code
		}
		return talk.failWith("handOffFailed", map[string]string{"path": install.Executable, "reason": err.Error()})
	}
	return 0
}

// 🔖️namesVersionOfBuildEntry tells whether a script route starts the build entry itself with its prefix and a version flag (`bun repo --version` runs `repo <entry script> repo --version`): the launcher answers it, like `repo --version`, instead of the entry, which has no version operation.
func (h Host) namesVersionOfBuildEntry(root string, registry *Registry, args []string) bool {
	index := scriptIndex(args)
	entry, found := registry.Implementation.Entries["build"]
	if index < 0 || !found || len(args) < index+3 || len(entry.Prefix) != 1 || args[index+1] != entry.Prefix[0] || (args[index+2] != "--version" && args[index+2] != "-V") {
		return false
	}
	start := root
	if cwd, err := h.Getwd(); err == nil {
		start = cwd
	}
	script := args[index]
	if !filepath.IsAbs(script) {
		script = filepath.Join(start, script)
	}
	return filepath.Clean(script) == filepath.Join(root, filepath.FromSlash(entry.Script))
}

func (s *speaker) failWith(id string, values map[string]string) int {
	return s.fail(Failure{ID: id, Values: values})
}

// #endregion 🏗️Entry
