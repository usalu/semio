// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident is the thin client of the resident repo service: for a plain task command it asks the service of this worktree before any runtime is provisioned, streams its answer, and steps aside silently whenever the service cannot answer.

// #endregion 🧲️Header

package launcher

import (
	"bufio"
	"bytes"
	"context"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"io"
	"os"
	"os/signal"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"time"
)

// #region 🛰️Resident

const residentFrameLimit = 16 << 20

// 🔌️residentDial opens the endpoint of the service; the platform members replace it, tests inject their own.
var residentDial = residentDialEndpoint

// 📤️residentStdout and residentStderr receive the streamed output of the service byte for byte; tests replace them.
var residentStdout, residentStderr io.Writer = os.Stdout, os.Stderr

// 🔢️ResidentWorktreeID names a worktree root: twelve hex digits of the digest the service computes from the same realpath.
func ResidentWorktreeID(root string) string {
	sum := sha256.Sum256([]byte("semio-repo-service-v1\x00" + root))
	return hex.EncodeToString(sum[:])[:12]
}

// 📍️ResidentEndpoints are the runtime directory, endpoint and key file of one worktree and user; a unix socket path stays below 96 bytes.
type ResidentEndpoints struct{ Directory, Endpoint, Key string }

// 📍️ResidentEndpointsFor computes the endpoint exactly as the service does.
func ResidentEndpointsFor(root string, env Environment, goos string, uid int, temp string) ResidentEndpoints {
	id := ResidentWorktreeID(root)
	owner := strconv.Itoa(uid)
	if goos == "windows" {
		base := env("LOCALAPPDATA")
		if base == "" {
			base = temp
		}
		directory := filepath.Join(base, "semio", "repo", id)
		return ResidentEndpoints{Directory: directory, Endpoint: `\\.\pipe\semio-repo-` + owner + "-" + id, Key: filepath.Join(directory, "key")}
	}
	base := env("XDG_RUNTIME_DIR")
	if base == "" {
		base = strings.TrimRight(temp, "/")
	}
	directory := filepath.Join(base, "semio-repo-"+owner, id)
	if len(filepath.Join(directory, "s")) > 96 {
		directory = filepath.Join("/tmp", "semio-repo-"+owner, id)
	}
	return ResidentEndpoints{Directory: directory, Endpoint: filepath.Join(directory, "s"), Key: filepath.Join(directory, "key")}
}

func residentCanonical(value any) string {
	switch typed := value.(type) {
	case map[string]any:
		keys := make([]string, 0, len(typed))
		for key := range typed {
			keys = append(keys, key)
		}
		sort.Strings(keys)
		var out strings.Builder
		out.WriteByte('{')
		for index, key := range keys {
			if index > 0 {
				out.WriteByte(',')
			}
			out.WriteString(residentQuote(key))
			out.WriteByte(':')
			out.WriteString(residentCanonical(typed[key]))
		}
		out.WriteByte('}')
		return out.String()
	case []any:
		parts := make([]string, len(typed))
		for index, item := range typed {
			parts[index] = residentCanonical(item)
		}
		return "[" + strings.Join(parts, ",") + "]"
	default:
		return residentQuote(typed)
	}
}

func residentQuote(value any) string {
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	_ = encoder.Encode(value)
	return strings.TrimRight(buffer.String(), "\n")
}

func residentMac(key []byte, data string) []byte {
	mac := hmac.New(sha256.New, key)
	mac.Write([]byte(data))
	return mac.Sum(nil)
}

// 🏷️ResidentMint mints a capability token for caveats given as decoded JSON values; the MAC chain covers their canonical form.
func ResidentMint(key []byte, id string, caveats []any) string {
	mac := residentMac(key, id)
	for _, caveat := range caveats {
		mac = residentMac(mac, residentCanonical(caveat))
	}
	payload := `{"id":` + residentQuote(id) + `,"caveats":` + residentQuote(caveats) + `}`
	encode := base64.RawURLEncoding.EncodeToString
	return "v1." + encode([]byte(payload)) + "." + encode(mac)
}

// 🪪️ResidentAgentID is the agent identity a client announces: the value cut to the characters a token may carry, else the fallback.
func ResidentAgentID(value, fallback string) string {
	if value == "" {
		value = fallback
	}
	clean := make([]rune, 0, 64)
	for _, character := range value {
		if len(clean) == 64 {
			break
		}
		if character >= 'a' && character <= 'z' || character >= 'A' && character <= 'Z' || character >= '0' && character <= '9' || strings.ContainsRune("._:-", character) {
			clean = append(clean, character)
		} else {
			clean = append(clean, '_')
		}
	}
	if len(clean) == 0 {
		return "default"
	}
	return string(clean)
}

// 🎫️ResidentLauncherCapability is the default token of a local launcher: every class but administration, bound to its worktree and agent, for one minute.
func ResidentLauncherCapability(key []byte, id string, now time.Time, worktree, agent string) string {
	return ResidentMint(key, id, []any{
		map[string]any{"k": "scope", "v": []any{"status", "query", "run", "watch"}},
		map[string]any{"k": "notAfter", "v": float64(now.UnixMilli() + 60000)},
		map[string]any{"k": "worktree", "v": worktree},
		map[string]any{"k": "agent", "v": agent},
	})
}

var residentFlagPrefixes = []string{"--service=", "--lang="}

var residentFlagsExact = []string{"--output=stream", "--no-color"}

var residentVerbs = []string{"run", "test", "run-many"}

// ✅ResidentEligible is true for plain task commands: a verb, at least one target and only flags that cannot change what a task does.
func ResidentEligible(args []string) bool {
	index := 0
	for index < len(args) && strings.HasPrefix(args[index], "--service=") {
		index++
	}
	if index >= len(args) || !contains(residentVerbs, args[index]) {
		return false
	}
	targets := 0
	for _, argument := range args[index+1:] {
		if argument == "--" {
			return false
		}
		if !strings.HasPrefix(argument, "-") {
			targets++
			continue
		}
		allowed := contains(residentFlagsExact, argument)
		for _, prefix := range residentFlagPrefixes {
			if strings.HasPrefix(argument, prefix) && len(argument) > len(prefix) {
				allowed = true
			}
		}
		if !allowed {
			return false
		}
	}
	return targets > 0
}

// 🧭️ResidentMode reads the service mode from `--service=` (anywhere before `--`) or SEMIO_BUILD_SERVICE; an unknown value answers off so that the normal path reports it.
func ResidentMode(args []string, env Environment) string {
	mode := env("SEMIO_BUILD_SERVICE")
	for _, argument := range args {
		if argument == "--" {
			break
		}
		if strings.HasPrefix(argument, "--service=") {
			mode = strings.TrimPrefix(argument, "--service=")
		}
	}
	switch mode {
	case "", "auto":
		return "auto"
	case "off", "require":
		return mode
	}
	return "off"
}

var residentMessages = map[string]map[string]string{
	"en": {"require": "--service=require: no resident service answered ({reason})."},
	"de": {"require": "--service=require: kein residenter Dienst hat geantwortet ({reason})."},
}

var residentBound = []string{"SEMIO_BUILD_TOOL", "SEMIO_BUILD_MODE", "SEMIO_BUILD_WORKERS", "SEMIO_BUILD_GRAPH_POLICY"}

type residentFrame struct {
	T       string            `json:"t"`
	Stream  string            `json:"stream"`
	Text    string            `json:"text"`
	B64     string            `json:"b64"`
	Exit    int               `json:"exit"`
	Code    string            `json:"code"`
	Retry   string            `json:"retry"`
	Message map[string]string `json:"message"`
}

// 🛰️ForwardResident hands an eligible build command to the resident service; handled=false means the caller continues with the normal path and nothing was printed.
func (h Host) ForwardResident(ctx context.Context, root, kind string, args []string) (bool, int) {
	busy := false
	handled, code := h.forwardResidentOnce(ctx, root, kind, args, &busy)
	if !busy {
		return handled, code
	}
	registry, err := LoadRegistry(root)
	if err != nil {
		return handled, code
	}
	policy, err := LoadRestartPolicy(root, registry)
	if err != nil {
		return handled, code
	}
	languages := residentLanguages(h, root, args)
	for attempt := 0; busy && attempt < policy.Maximum; attempt++ {
		h.Stderr(policy.Notice(languages, attempt))
		if !restartSleep(ctx, policy.Wait(attempt)) {
			return true, 130
		}
		busy = false
		handled, code = h.forwardResidentOnce(ctx, root, kind, args, &busy)
	}
	return handled, code
}

// 🛰️forwardResidentOnce is one attempt to ask the service; busy is set when the service refused with a later retry.
func (h Host) forwardResidentOnce(ctx context.Context, root, kind string, args []string, busy *bool) (bool, int) {
	if kind != "build" {
		return false, 0
	}
	mode := ResidentMode(args, h.Env)
	if mode == "off" || !ResidentEligible(args) {
		return false, 0
	}
	real := canonicalRoot(root)
	paths := ResidentEndpointsFor(real, h.Env, residentGOOS, os.Getuid(), os.TempDir())
	fail := func(reason string) (bool, int) {
		if mode != "require" {
			return false, 0
		}
		language := ""
		if registry, loadErr := LoadRegistry(root); loadErr == nil {
			if protocol, protocolErr := LoadLanguageProtocol(root, registry); protocolErr == nil {
				if resolution, refusal := protocol.ResolveLanguage(registry.Languages, args, h.Env, h.System); refusal == nil && residentMessages[resolution.Language] != nil {
					language = resolution.Language
				}
			}
		}
		fields := map[string]string{"reason": reason}
		if language == "" {
			h.Stderr(Format(residentMessages["en"]["require"], fields) + "\n" + Format(residentMessages["de"]["require"], fields))
			return true, 1
		}
		h.Stderr(Format(residentMessages[language]["require"], fields))
		return true, 1
	}
	if !residentTrusted(paths.Directory) {
		return fail("down")
	}
	key, err := os.ReadFile(paths.Key)
	if err != nil {
		return fail("key")
	}
	languages := residentLanguages(h, root, args)
	if languages == nil {
		return fail("language")
	}
	connection, err := residentDial(paths.Endpoint)
	if err != nil {
		return fail("down")
	}
	defer connection.Close()
	identifier := make([]byte, 8)
	_, _ = rand.Read(identifier)
	agent := ResidentAgentID(h.Env("SEMIO_AGENT_ID"), "launcher")
	token := h.Env("SEMIO_SERVICE_TOKEN")
	if token == "" {
		token = ResidentLauncherCapability(key, hex.EncodeToString(identifier), time.Now(), ResidentWorktreeID(real), agent)
	}
	environment := map[string]string{}
	for _, name := range residentBound {
		if value := h.Env(name); value != "" {
			environment[name] = value
		}
	}
	verb, targets := residentSplit(args)
	cwd, _ := os.Getwd()
	hello, _ := json.Marshal(map[string]any{"t": "hello", "v": 1, "client": map[string]any{"name": "repo-launcher", "pid": os.Getpid(), "lane": agent}, "languages": languages, "cap": token})
	request, _ := json.Marshal(map[string]any{"t": "request", "id": 1, "command": verb, "args": targets, "cwd": cwd, "env": environment})
	if _, err := connection.Write(append(append(hello, '\n'), append(request, '\n')...)); err != nil {
		return fail("write")
	}
	interrupts := make(chan os.Signal, 1)
	signal.Notify(interrupts, os.Interrupt)
	defer signal.Stop(interrupts)
	go func() {
		select {
		case <-interrupts:
			_, _ = connection.Write([]byte(`{"t":"cancel","id":1}` + "\n"))
		case <-ctx.Done():
		}
	}()
	reader := bufio.NewReaderSize(connection, 1<<16)
	accepted, wrote := false, false
	for {
		line, readErr := reader.ReadBytes('\n')
		if len(line) > residentFrameLimit {
			return fail("protocol")
		}
		if len(line) > 0 {
			var frame residentFrame
			if json.Unmarshal(line, &frame) != nil {
				return residentLost(fail, accepted || wrote, h, "protocol")
			}
			switch frame.T {
			case "accepted":
				accepted = true
			case "output":
				wrote = true
				text := frame.Text
				if frame.B64 != "" {
					decoded, _ := base64.StdEncoding.DecodeString(frame.B64)
					text = string(decoded)
				}
				target := residentStdout
				if frame.Stream == "stderr" {
					target = residentStderr
				}
				_, _ = io.WriteString(target, text)
			case "result":
				return true, frame.Exit
			case "refused":
				if frame.Retry == "fallback" {
					return fail(frame.Code)
				}
				text := ""
				for _, language := range languages {
					if text = frame.Message[language]; text != "" {
						break
					}
				}
				h.Stderr(text)
				if frame.Retry == "later" {
					*busy = true
					return true, 75
				}
				return true, 1
			}
		}
		if readErr != nil {
			return residentLost(fail, accepted || wrote, h, "closed")
		}
	}
}

func residentLost(fail func(string) (bool, int), started bool, h Host, reason string) (bool, int) {
	if started {
		h.Stderr("[repo] the resident service ended the connection before the command finished")
		return true, 70
	}
	return fail(reason)
}

func residentSplit(args []string) (string, []string) {
	index := 0
	for index < len(args) && strings.HasPrefix(args[index], "--service=") {
		index++
	}
	targets := []string{}
	for _, argument := range args[index+1:] {
		if !strings.HasPrefix(argument, "-") {
			targets = append(targets, argument)
		}
	}
	return args[index], targets
}

func residentLanguages(h Host, root string, args []string) []string {
	registry, err := LoadRegistry(root)
	if err != nil {
		return nil
	}
	protocol, err := LoadLanguageProtocol(root, registry)
	if err != nil {
		return nil
	}
	resolution, refusal := protocol.ResolveLanguage(registry.Languages, args, h.Env, h.System)
	if refusal != nil {
		return nil
	}
	switch resolution.Language {
	case "de":
		return []string{"de", "en"}
	case "en":
		return []string{"en", "de"}
	}
	return nil
}

// #region 🔁️Restart

// 📜️RestartPolicy is the part of the shared restart policy the launcher reads.
type RestartPolicy struct {
	Maximum int
	Waits   []time.Duration
	Texts   map[string]map[string]string
}

// 📥️LoadRestartPolicy reads the protocol constant of the policy file the registry names.
func LoadRestartPolicy(root string, registry *Registry) (RestartPolicy, error) {
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(registry.Restart.Policy)))
	if err != nil {
		return RestartPolicy{}, err
	}
	var document struct {
		Definitions struct {
			Protocol struct {
				Constant struct {
					Maximum     int `json:"maximumReexecutions"`
					LoadFailure struct {
						BackoffMs []int `json:"backoffMs"`
					} `json:"loadFailure"`
					Texts map[string]map[string]string `json:"texts"`
				} `json:"const"`
			} `json:"Protocol"`
		} `json:"$defs"`
	}
	if err = json.Unmarshal(raw, &document); err != nil {
		return RestartPolicy{}, err
	}
	constant := document.Definitions.Protocol.Constant
	if constant.Maximum < 1 || len(constant.LoadFailure.BackoffMs) == 0 || constant.Texts["en"]["busy"] == "" || constant.Texts["de"]["busy"] == "" {
		return RestartPolicy{}, errors.New("restart policy incomplete")
	}
	policy := RestartPolicy{Maximum: constant.Maximum, Texts: constant.Texts}
	for _, milliseconds := range constant.LoadFailure.BackoffMs {
		policy.Waits = append(policy.Waits, time.Duration(milliseconds)*time.Millisecond)
	}
	return policy, nil
}

// ⏱️Wait is the pause before restart number attempt (counted from zero); the last wait repeats.
func (p RestartPolicy) Wait(attempt int) time.Duration {
	return p.Waits[min(attempt, len(p.Waits)-1)]
}

// 📣️Notice is the busy notice before restart number attempt in each of the languages, preferred first.
func (p RestartPolicy) Notice(languages []string, attempt int) string {
	fields := map[string]string{"delay": strconv.FormatInt(p.Wait(attempt).Milliseconds(), 10), "attempt": strconv.Itoa(attempt + 1), "maximum": strconv.Itoa(p.Maximum)}
	if len(languages) == 0 {
		languages = []string{"en", "de"}
	}
	text := ""
	for _, language := range languages {
		if text != "" {
			text += "\n"
		}
		text += Format(p.Texts[language]["busy"], fields)
	}
	return text
}

// 😴restartSleep waits and reports false when the context ended first; tests replace it.
var restartSleep = func(ctx context.Context, wait time.Duration) bool {
	timer := time.NewTimer(wait)
	defer timer.Stop()
	select {
	case <-timer.C:
		return true
	case <-ctx.Done():
		return false
	}
}

// #endregion 🔁️Restart

// #endregion 🛰️Resident
