"""🦀 Independent model of the cargo grant rules.

Reads cases as JSON on stdin ({"lineage": [...], "jobs": [...], "caps": [...], "toolchains": [...]}) and answers every
case once more: the class names a Cargo command line learns its memory under, the jobs it asks for, its explicit job
flags capped at a grant, and the toolchain a rust-toolchain.toml pins (parsed by Python's own TOML parser, a third-party
implementation of the format, not the regular expression of the TypeScript code). It shares no code with the
TypeScript implementation.
"""
import json
import re
import sys
import tomllib


def own(arguments):
    return arguments[: arguments.index("--")] if "--" in arguments else arguments


def lineage(arguments):
    head = own(arguments)
    subcommand = next((a for a in head if not a.startswith("-") and not a.startswith("+")), "")
    part = None
    for i, a in enumerate(head):
        if a in ("-p", "--package") and i + 1 < len(head):
            part = head[i + 1]
        elif a.startswith("--package="):
            part = a[len("--package="):]
        elif a in ("--workspace", "--all"):
            part = "workspace"
        if part is not None:
            break
    result = ["cargo"]
    for name in (subcommand, part or ""):
        if name:
            result.append(result[-1] + ":" + re.sub(r"\s+", "_", name)[:120])
    return result


def jobs(arguments, environment, cores):
    head = own(arguments)
    value = None
    i = 0
    while i < len(head):
        a = head[i]
        if a in ("-j", "--jobs"):
            value = head[i + 1] if i + 1 < len(head) else None
            i += 1
        elif a.startswith("--jobs="):
            value = a[len("--jobs="):]
        elif re.fullmatch(r"-j\d+", a):
            value = a[2:]
        i += 1
    text = value if value is not None else environment.get("CARGO_BUILD_JOBS")
    try:
        parsed = int(text)
    except (TypeError, ValueError):
        parsed = 0
    return parsed if parsed > 0 else max(1, cores)


def cap(arguments, granted):
    result = list(arguments)
    end = len(own(arguments))

    def capped(text):
        try:
            value = int(text)
        except (TypeError, ValueError):
            value = 0
        return str(min(granted, value if value > 0 else granted))

    i = 0
    while i < end:
        a = result[i]
        if a in ("-j", "--jobs") and i + 1 < end:
            result[i + 1] = capped(result[i + 1])
            i += 1
        elif a.startswith("--jobs="):
            result[i] = "--jobs=" + capped(a[len("--jobs="):])
        elif re.fullmatch(r"-j\d+", a):
            result[i] = "-j" + capped(a[2:])
        i += 1
    return result


def toolchain(text, triple):
    channel = tomllib.loads(text).get("toolchain", {}).get("channel")
    if channel is None:
        return None
    if re.search(r"-(apple|pc|unknown)-", channel):
        return channel
    if re.fullmatch(r"(nightly|beta|stable)(-\d{4}-\d{2}-\d{2})?", channel):
        return channel + "-" + triple
    return channel


cases = json.load(sys.stdin)
print(json.dumps({
    "lineage": [lineage(c["arguments"]) for c in cases.get("lineage", [])],
    "jobs": [jobs(c["arguments"], c["environment"], c["cores"]) for c in cases.get("jobs", [])],
    "caps": [cap(c["arguments"], c["granted"]) for c in cases.get("caps", [])],
    "toolchains": [toolchain(c["file"], c["triple"]) for c in cases.get("toolchains", [])],
}))
