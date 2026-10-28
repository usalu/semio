"""🐍️ Independent Python model of the lease rank table for the Nx path (no TypeScript code shared): classifies resources by the authored policy, replays the declared Nx acquisition sequences against the out-of-order and hold-and-wait rules, and scans the sources for the facts the declarations rest on.
usage: python3 -I model.py <policy.json> <sequences.json> <library-root>  ->  JSON on stdout."""
import json, re, sys


def slash(path):
    return path.replace("\\", "/").rstrip("/")


class Policy:
    def __init__(self, document):
        self.document = document
        self.exact, self.prefixes = {}, []
        for row in document["classes"]:
            for match in row["match"]:
                if match["kind"] == "exact":
                    self.exact[match["value"]] = row
                else:
                    self.prefixes.append((match["value"], row))
        self.prefixes.sort(key=lambda item: -len(item[0]))
        self.grace = document["holdAndWait"]["graceMilliseconds"]

    def classify(self, resource):
        if resource in self.exact:
            return self.exact[resource]
        for prefix, row in self.prefixes:
            if resource.startswith(prefix):
                return row
        return {"id": "unclassified", "rank": 0}

    def allowed(self, held, request):
        source, target = self.classify(held["resource"])["id"], self.classify(request["resource"])["id"]
        for edge in self.document["allowed"]:
            if edge["from"] != source or edge["to"] != target:
                continue
            binding = edge.get("binding")
            if not binding:
                return True
            suffix = held["resource"][len(binding["fromPrefix"]):]
            if held["directory"] == request["directory"] and held["resource"].startswith(binding["fromPrefix"]) and request["resource"].startswith(binding["toPrefix"]) and suffix and suffix == request["resource"][len(binding["toPrefix"]):]:
                return True
        return False

    def medium(self, held, request):
        if held["mode"] != "shared":
            return False
        return any(held["resource"].startswith(item["prefix"]) and (slash(request["directory"]) + "/").startswith(slash(held["resource"][len(item["prefix"]):]) + "/") for item in self.document["media"])

    def out_of_order(self, held, request):
        wanted = self.classify(request["resource"])
        if wanted["rank"] <= 0 or wanted.get("inert"):
            return None
        if request["mode"] == "shared" and any(row["mode"] == "shared" and row["directory"] == request["directory"] and row["resource"] == request["resource"] for row in held):
            return None
        later = [row for row in held if not self.medium(row, request) and not self.classify(row["resource"]).get("inert") and self.classify(row["resource"])["rank"] > wanted["rank"] and not self.allowed(row, request)]
        return min(later, key=lambda row: row["atMs"]) if later else None

    def hold_and_wait(self, held, request, waited):
        if waited < self.grace or self.classify(request["resource"]).get("inert"):
            return None
        blocking = [row for row in held if row["mode"] == "exclusive" and row["resource"] != request["resource"] and not self.classify(row["resource"]).get("inert") and not self.allowed(row, request)]
        return min(blocking, key=lambda row: row["atMs"]) if blocking else None


def replay(policy, directory, steps):
    held, order, wait = [], [], []
    for index, step in enumerate(steps):
        if step["op"] == "release":
            at = next(i for i, row in enumerate(held) if row["resource"] == step["resource"])
            del held[at]
            continue
        request = {"directory": directory, "resource": step["resource"], "mode": step["mode"]}
        found = policy.out_of_order(held, request)
        if found:
            order.append({"held": found["resource"], "requested": step["resource"]})
        found = policy.hold_and_wait(held, request, step.get("waitedMilliseconds", 0))
        if found:
            wait.append({"held": found["resource"], "requested": step["resource"]})
        held.append({**request, "atMs": index + 1})
    return {"outOfOrder": order, "holdAndWait": wait}


PIN_FILES = ["⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts", "🏗️build/🕸️graph/🟦️.ts"]
CALL = re.compile(r"acquire(?:Queued)?ResourceLease\(\{([^{}]*)\}\)")


def pins(root):
    rows = []
    for name in PIN_FILES:
        text = open(root + "/" + name, encoding="utf8").read()
        for body in CALL.findall(text):
            if "resource:resource(" not in body and 'resource:"build-graph:"' not in body:
                continue
            mode = re.search(r'mode:"(shared|exclusive)"', body)
            timeout = re.search(r"timeoutMs:(\d+)", body)
            rows.append({"file": name, "mode": mode.group(1), "timeoutMs": int(timeout.group(1)) if timeout else None})
    return rows


SLOT_FILE = "🕸️graph/🏁️startup/🚏️admission/🟦️.ts"


def slot_requests(root):
    text = open(root + "/⚡️caching/" + SLOT_FILE, encoding="utf8").read()
    return [{"mode": re.search(r'mode:"(shared|exclusive)"', body).group(1), "timeoutMs": int(re.search(r"timeoutMs:(\d+)", body).group(1)) if re.search(r"timeoutMs:(\d+)", body) else None} for body in CALL.findall(text)]


SOURCES = [
    ("🕸️graph/🏁️startup/🚦️service/🟦️.ts", "function withNxDaemonServiceV1"),
    ("🕸️graph/🏁️startup/🚦️service/🟦️.ts", "function withNxDaemonServiceBirthV1"),
    ("🕸️graph/🏁️startup/🚦️service/🟦️.ts", "function withNxDaemonJournalReaderV1"),
    ("🚀️bootstrap/📜️script.ts", "async runWithPreparedToolingStore("),
]
BOUNDARY = re.compile(r"\n(?:export |/\*\*|  private |  async |  static )")


def binding_order(root):
    rows = {}
    for name, marker in SOURCES:
        text = open(root + "/⚡️caching/" + name, encoding="utf8").read()
        start = text.index(marker)
        end = BOUNDARY.search(text, start + len(marker))
        body = text[start:end.start() if end else len(text)]
        use, bind = body.find("withNxToolingUseV1("), re.search(r"\bbind\(", body)
        rows[marker] = {"toolUseBeforeBind": use >= 0 and bind is not None and use < bind.start()}
    return rows


def main():
    policy = Policy(json.load(open(sys.argv[1], encoding="utf8")))
    document = json.load(open(sys.argv[2], encoding="utf8"))
    print(json.dumps({"replays": {row["id"]: replay(policy, document["directory"], row["steps"]) for row in document["sequences"]}, "pins": pins(sys.argv[3]), "slots": slot_requests(sys.argv[3]), "binding": binding_order(sys.argv[3])}, ensure_ascii=False))


main()
