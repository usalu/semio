"""🌡️ Independent Python oracle of the host capacity argument rules (standard library only): it reads options before the argument separator, decides which operation, how many jobs and which action kinds an argument list asks for, and rewrites the parallelism option to the granted jobs. Requests arrive as JSON on stdin."""
import json, re, sys


def option(args, name, short=None):
    last, index = None, 0
    while index < len(args) and args[index] != "--":
        argument = args[index]
        if argument == "--" + name or (short is not None and argument == "-" + short):
            value = args[index + 1] if index + 1 < len(args) else None
            if value is not None and not value.startswith("-"):
                last = value
                index += 1
        elif argument.startswith("--" + name + "=") or (short is not None and argument.startswith("-" + short + "=")):
            last = argument[argument.index("=") + 1:]
        index += 1
    return last


def number(text):
    if text is None:
        return float("nan")
    if text.strip() == "":
        return 0.0
    if re.fullmatch(r"\s*[+-]?\d+(\.\d+)?\s*", text):
        return float(text)
    return float("nan")


def classify(workers, target):
    for rule in workers["classification"]:
        if "continuous" in rule:
            if rule["continuous"] is False:
                return rule["kind"]
        elif re.search(rule["pattern"], target):
            return rule["kind"]
    return workers["defaultKind"]


def ask(policy, workers, args, environment):
    command = args[0] if args else None
    if command is None or command not in [row["name"] for row in policy["operations"]]:
        return None
    head = args[:args.index("--")] if "--" in args else args
    if any(argument in ("--help", "-h") for argument in head):
        return None
    requested = option(head, "parallelism") if command == "cargo" else option(head, "parallel")
    fallback = None if command == "cargo" else environment.get("SEMIO_BUILD_PARALLEL")
    parsed = number(requested if requested is not None else fallback)
    explicit = parsed == parsed and parsed.is_integer() and 0 < parsed <= 2 ** 53 - 1
    if command == "cargo":
        kinds = []
    else:
        named = [head[1].split(":")[1] if len(head[1].split(":")) > 1 else None] if head[:1] == ["run"] and len(head) > 1 else ([] if head[:1] == ["run"] else (option(head, "targets", "t") or option(head, "target") or "").split(","))
        if head[:1] == ["run"] and len(head) <= 1:
            named = []
        kinds = [classify(workers, target) for target in named if isinstance(target, str) and target]
    return {"operation": command, "preferredJobs": int(parsed) if explicit else "host", "kinds": kinds}


def granted(args, operation, jobs):
    name = "--parallelism" if operation == "cargo" else "--parallel"
    boundary = args.index("--") if "--" in args else None
    head = args[:boundary] if boundary is not None else args
    tail = args[boundary:] if boundary is not None else []
    kept, index = [], 0
    while index < len(head):
        argument = head[index]
        if argument == name:
            index += 2
            continue
        if argument.startswith(name + "="):
            index += 1
            continue
        kept.append(argument)
        index += 1
    return kept + [name + "=" + str(jobs)] + tail


def flag(head, name):
    for index, argument in enumerate(head):
        if argument == "--" + name:
            return head[index + 1] if index + 1 < len(head) else None
        if argument.startswith("--" + name + "="):
            return argument[len(name) + 3:]
    return None


def positive(value, name):
    if value is None:
        return None
    parsed = number(value)
    if not (parsed == parsed and parsed.is_integer() and 1 <= parsed <= 2 ** 53 - 1):
        raise ValueError("Capacity option --" + name + " requires a positive integer")
    return int(parsed)


def command(args, policy):
    try:
        if not args or args[0] not in ("status", "run"):
            return "Expected repo capacity status|run"
        if args[0] == "status":
            return None
        boundary = args.index("--") if "--" in args else None
        head = args[1:boundary] if boundary is not None else args[1:]
        tail = args[boundary + 1:] if boundary is not None else []
        if not tail:
            return "Capacity run requires a command after --"
        memory, cpu, jobs = positive(flag(head, "memory"), "memory"), positive(flag(head, "cpu"), "cpu"), positive(flag(head, "jobs"), "jobs")
        operation = flag(head, "operation")
        operation = "command" if operation is None else operation
        if memory is None and operation not in [row["name"] for row in policy["operations"]]:
            return "Unknown capacity operation: " + operation
        return None
    except ValueError as error:
        return str(error)


def main():
    request = json.load(sys.stdin)
    if request["op"] == "ask":
        print(json.dumps([ask(request["policy"], request["workers"], row["args"], row["environment"]) for row in request["rows"]]))
    elif request["op"] == "command":
        print(json.dumps([command(row, request["policy"]) for row in request["rows"]]))
    else:
        print(json.dumps([granted(row["args"], row["operation"], row["jobs"]) for row in request["rows"]]))


main()
