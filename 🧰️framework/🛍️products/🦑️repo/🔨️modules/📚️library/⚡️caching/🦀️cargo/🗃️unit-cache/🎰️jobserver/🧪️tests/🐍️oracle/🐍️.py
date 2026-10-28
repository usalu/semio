"""🐍️ Independent oracle of the job-token pool grammar. stdin: the fixture ({cases, closedPools}); stdout: {endpoints, patches}. Written from the GNU make manual (POSIX jobserver, the `fifo:PATH` form of make 4.4) and the behaviour of Rust's `jobserver` crate, not from the TypeScript module."""
import json
import re
import sys

VARIABLES = ["CARGO_MAKEFLAGS", "MAKEFLAGS", "MFLAGS"]
CENSUS = "SEMIO_UNIT_CACHE_JOBSERVER_FDS"
LAST_DESCRIPTOR = 63


def auth_value(environment):
    chosen = next((name for name in VARIABLES if name in environment), None)
    if chosen is None:
        return None, None
    text = environment[chosen]
    for flag in ("--jobserver-auth=", "--jobserver-fds="):
        index = text.rfind(flag)
        if index >= 0:
            return chosen, text[index + len(flag):].split(" ")[0]
    return chosen, None


def endpoint(case):
    environment, platform, fifos = case["environment"], case["platform"], set(case["fifos"])
    variable, value = auth_value(environment)
    if variable is None or value is None:
        return {"kind": "none"}

    def closed(reason):
        return {"kind": "closed", "variable": variable, "reason": reason}

    if platform == "win32":
        return closed("malformed") if value == "" else {"kind": "semaphore", "name": value}
    head, colon, rest = value.partition(":")
    if head == "fifo":
        if rest == "" or not colon:
            return closed("malformed")
        return {"kind": "fifo", "path": rest} if rest in fifos else closed("missing-fifo")
    pair = value.split(",", 1)
    if len(pair) != 2 or not all(re.fullmatch(r"[+-]?[0-9]+", part) for part in pair):
        return closed("malformed")
    read, write = int(pair[0]), int(pair[1])
    if read < 0 or write < 0:
        return closed("disabled")
    if max(read, write) > LAST_DESCRIPTOR:
        return closed("out-of-range")
    census = re.fullmatch(r"([0-9]+),([0-9]+)", environment.get(CENSUS, ""))
    if census and int(census.group(1)) == read and int(census.group(2)) == write:
        return {"kind": "pipe", "read": read, "write": write}
    return closed("not-inherited")


def patch(row):
    result = {"NUM_JOBS": "1"}
    for name in VARIABLES:
        if name not in row["environment"]:
            continue
        words = [w for w in row["environment"][name].split(" ") if w and not w.startswith(("--jobserver-auth=", "--jobserver-fds=")) and not re.fullmatch(r"-j[0-9]*", w)]
        words.append("-j1")
        if row["empty"] is not None:
            words += ["--jobserver-fds=fifo:" + row["empty"], "--jobserver-auth=fifo:" + row["empty"]]
        result[name] = " ".join(words)
    return result


fixture = json.load(sys.stdin)
json.dump({"endpoints": [endpoint(case) for case in fixture["cases"]], "patches": [patch(row) for row in fixture["closedPools"]]}, sys.stdout)
