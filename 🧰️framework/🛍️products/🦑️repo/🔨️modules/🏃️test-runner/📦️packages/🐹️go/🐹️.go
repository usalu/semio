// 🏃️ Package testrunner is the Go half of the repo test-runner domain.
//
// It carries the process port every planned invocation goes through and its replaying double,
// alongside the runner behaviour the `go-split` moved out of the pre-split godfile.
package testrunner

import (
	fmt "fmt"
	strings "strings"
)

// #region ⚙️Process Port

// 🎬️ProcessRequest is one process a plan wants to run.
type ProcessRequest struct {
	// 🏷️Program is the executable name.
	Program string
	// 📜️Args are the arguments after the program name.
	Args []string
	// 📁️Cwd is the working directory.
	Cwd string
	// 🌱️Env is the environment overlay.
	Env map[string]string
}

// 📤️ProcessOutput is what a process answered.
type ProcessOutput struct {
	// 🔢️Status is the exit status.
	Status int
	// 📝️Stdout is standard output.
	Stdout string
	// 📝️Stderr is standard error.
	Stderr string
}

// ⚙️ProcessRunner is the only door from a plan to an operating system. It is the same shape as the
// `ProcessRunner` trait in the Rust twin and the one 🧩️providers will own for every shelled-out
// command in this repository.
type ProcessRunner interface {
	// ▶️Run executes one process to completion.
	Run(request ProcessRequest) (ProcessOutput, error)
}

// 🎞️RecordedTranscript is one recorded (argv, cwd) → output pair.
type RecordedTranscript struct {
	// 📜️Argv is the argv this transcript answers, program name first.
	Argv []string
	// 📁️Cwd is the working directory it answers for; empty matches any.
	Cwd string
	// 📤️Output is the recorded answer.
	Output ProcessOutput
}

// 🎞️RecordedProcessRunner replays committed transcripts and never touches the machine.
type RecordedProcessRunner struct {
	// 🎞️Transcripts are matched in order.
	Transcripts []RecordedTranscript
}

// ▶️Run answers from the first transcript whose argv matches and whose cwd matches or is empty.
func (runner *RecordedProcessRunner) Run(request ProcessRequest) (ProcessOutput, error) {
	argv := append([]string{request.Program}, request.Args...)
	for _, transcript := range runner.Transcripts {
		if !equalArgv(transcript.Argv, argv) {
			continue
		}
		if transcript.Cwd != "" && normalizeSeparators(transcript.Cwd) != normalizeSeparators(request.Cwd) {
			continue
		}
		return transcript.Output, nil
	}
	return ProcessOutput{}, fmt.Errorf("no recorded transcript for %s", strings.Join(argv, " "))
}

func equalArgv(left []string, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

func normalizeSeparators(path string) string {
	return strings.ReplaceAll(path, "\\", "/")
}

// #endregion ⚙️Process Port

// #region 🪪️IdentityPending

// 🪪️EntityIdentity is the identifier vocabulary the scope resolver needs from 🪪️identity. It is
// declared here because that module does not export it yet; when it does, this collapses to an alias.
type EntityIdentity interface {
	Flat(text string) string
	PathFromUriPath(uriPath string) string
	IdToUri(id string) string
}

// 🗣️LanguageTable is the file-extension to language mapping the scope resolver needs from 🗣️languages.
type LanguageTable interface {
	LanguageOf(filePath string) (string, bool)
}

// 🪪️PendingIdentity is the ported-from-Go default for both ports.
type PendingIdentity struct{}

// 🔤️Flat keeps alphanumerics and every non-ASCII rune, then lowercases.
func (PendingIdentity) Flat(text string) string { return Flat(text) }

// 🛤️PathFromUriPath decodes %20 in every segment.
func (PendingIdentity) PathFromUriPath(uriPath string) string { return PathFromUriPath(uriPath) }

// 🔗️IdToUri answers the empty string until 🪪️identity owns the entity emoji table.
func (PendingIdentity) IdToUri(string) string { return "" }

// 🗣️LanguageOf answers the language a testable extension maps to.
func (PendingIdentity) LanguageOf(filePath string) (string, bool) {
	return LanguageOfExtension(filePath)
}

// 🔤️Flat keeps alphanumerics and every non-ASCII rune, then lowercases.
func Flat(text string) string {
	var builder strings.Builder
	for _, value := range text {
		if (value >= '0' && value <= '9') || (value >= 'a' && value <= 'z') || (value >= 'A' && value <= 'Z') || value > 0x7F {
			builder.WriteRune(value)
		}
	}
	return strings.ToLower(builder.String())
}

// 🛤️PathFromUriPath decodes %20 per segment.
func PathFromUriPath(uriPath string) string {
	segments := strings.Split(uriPath, "/")
	for index, segment := range segments {
		segments[index] = strings.ReplaceAll(segment, "%20", " ")
	}
	return strings.Join(segments, "/")
}

// 🗣️LanguageOfExtension is the extension table the planner needs, a subset of 🗣️languages limited to
// testable languages.
func LanguageOfExtension(filePath string) (string, bool) {
	lower := strings.ToLower(filePath)
	index := strings.LastIndex(lower, ".")
	if index < 0 {
		return "", false
	}
	switch lower[index+1:] {
	case "go":
		return "go", true
	case "rs":
		return "rust", true
	case "cs":
		return "csharp", true
	case "ts", "tsx":
		return "typescript", true
	case "js", "jsx", "mjs", "cjs":
		return "javascript", true
	case "py":
		return "python", true
	case "rb":
		return "ruby", true
	}
	return "", false
}

// #endregion 🪪️IdentityPending

// #region 🧮️Paths

// 🧭️NormalizeSeparators forward-slashes every path this domain stores or compares.
func NormalizeSeparators(value string) string { return strings.ReplaceAll(value, "\\", "/") }

// 📏️IsAbsolutePath accepts both POSIX roots and Windows drive roots so one fixture runs on every host.
func IsAbsolutePath(value string) bool {
	value = NormalizeSeparators(value)
	if strings.HasPrefix(value, "/") {
		return true
	}
	if len(value) < 3 || value[1] != ':' || value[2] != '/' {
		return false
	}
	return (value[0] >= 'a' && value[0] <= 'z') || (value[0] >= 'A' && value[0] <= 'Z')
}

// 🔗️JoinPath joins and cleans, dropping `.` segments and resolving `..`.
func JoinPath(base string, relative string) string {
	base = NormalizeSeparators(base)
	relative = NormalizeSeparators(relative)
	if base == "" {
		return CleanPath(relative)
	}
	if relative == "" {
		return CleanPath(base)
	}
	return CleanPath(strings.TrimRight(base, "/") + "/" + relative)
}

// 🧹️CleanPath collapses separators and `.`/`..` segments while keeping any root prefix.
func CleanPath(value string) string {
	value = NormalizeSeparators(value)
	prefix, rest := "", value
	if strings.HasPrefix(value, "/") {
		prefix, rest = "/", value[1:]
	} else if IsAbsolutePath(value) {
		prefix, rest = value[:3], value[3:]
	}
	segments := make([]string, 0, 8)
	for _, segment := range strings.Split(rest, "/") {
		switch segment {
		case "", ".":
		case "..":
			if len(segments) > 0 && segments[len(segments)-1] != ".." {
				segments = segments[:len(segments)-1]
			} else if prefix == "" {
				segments = append(segments, "..")
			}
		default:
			segments = append(segments, segment)
		}
	}
	body := strings.Join(segments, "/")
	if prefix != "" {
		return prefix + body
	}
	if body == "" {
		return "."
	}
	return body
}

// 🏷️BaseName is the final segment of a path.
func BaseName(value string) string {
	value = NormalizeSeparators(value)
	trimmed := strings.TrimRight(value, "/")
	if trimmed == "" {
		return value
	}
	if index := strings.LastIndex(trimmed, "/"); index >= 0 {
		return trimmed[index+1:]
	}
	return trimmed
}

// 📁️DirName is everything before the final segment.
func DirName(value string) string {
	cleaned := CleanPath(value)
	index := strings.LastIndex(cleaned, "/")
	if index == 0 {
		return "/"
	}
	if index > 0 {
		return cleaned[:index]
	}
	return "."
}

// ↔️RelativePath is target expressed relative to base, or target unchanged when it is not below it.
func RelativePath(base string, target string) string {
	base = CleanPath(base)
	target = CleanPath(target)
	if base == target {
		return "."
	}
	prefix := strings.TrimRight(base, "/") + "/"
	if strings.HasPrefix(target, prefix) {
		return target[len(prefix):]
	}
	return target
}

// #endregion 🧮️Paths
