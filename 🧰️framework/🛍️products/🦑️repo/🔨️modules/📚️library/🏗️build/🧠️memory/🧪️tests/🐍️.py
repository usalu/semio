"""🐍️ Independent model of the graph memory laws (stdlib only): the owner concurrency a cgroup headroom admits, the language choice and the message rendering. It never imports the JavaScript implementation.

usage: 🐍️.py expect FIXTURE   prints the expected verdicts of every table as JSON"""
import json, math, re, sys

MEGABYTE = 1048576


def language_oracle():
    """🗣️ The independent oracle of the one language protocol (`⚙️runtime/🗣️language`) and the protocol it reads."""
    import importlib.util
    import os
    root = os.path.dirname(os.path.abspath(__file__))
    while not os.path.isdir(os.path.join(root, "🧰️framework")):
        root = os.path.dirname(root)
    folder = os.path.join(root, "🧰️framework", "🔨️modules", "⚙️runtime", "🗣️language")
    spec = importlib.util.spec_from_file_location("language_oracle", os.path.join(folder, "🧪️tests", "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    with open(os.path.join(folder, "🔣️.json"), encoding="utf-8") as handle:
        return module, json.load(handle)


def concurrency(policy, row):
    """🚦️ How many owners of the learned size run at once on a cgroup: the headroom is the limit less the memory in use less the safety margin; one owner always runs, never more than the workers."""
    headroom = row["limitMegabytes"] * (1 - policy["safetyMarginBasisPoints"] / 10000) - row["currentMegabytes"]
    return max(1, min(row["workers"], math.floor(headroom / row["ownerMegabytes"])))


def admits(row):
    """🧮️ Whether one more owner starts: the first always, an unlearned kind never beside another, otherwise the headroom less the remaining estimates covers the new estimate."""
    if row["running"] == 0:
        return True
    if not row["learned"]:
        return False
    remaining = sum(max(0, owner["estimateMegabytes"] - owner["takenMegabytes"]) for owner in row["inflight"])
    return row["headroomMegabytes"] - remaining >= row["estimateMegabytes"]


def scenario(policy, row):
    """🚦️ Event model of one queue of owners that all take the same time: admit the head while the law allows, otherwise the oldest running owner completes and its kind learns the growth it took."""
    margin = row["limitMegabytes"] * policy["safetyMarginBasisPoints"] / 10000
    learned = dict(row["persisted"])
    queue, running, peak = list(row["tasks"]), [], 0
    while queue or running:
        if queue and len(running) < row["workers"]:
            head = queue[0]
            known = head["kind"] in learned
            live = sum(task["growthMegabytes"] for task in running)
            state = {
                "headroomMegabytes": row["limitMegabytes"] - margin - row["currentMegabytes"] - live,
                "running": len(running),
                "inflight": [{"estimateMegabytes": learned.get(task["kind"], row["guessMegabytes"]), "takenMegabytes": task["growthMegabytes"]} for task in running],
                "estimateMegabytes": learned[head["kind"]] if known else row["guessMegabytes"],
                "learned": known,
            }
            if admits(state):
                running.append(queue.pop(0))
                peak = max(peak, len(running))
                continue
        done = running.pop(0)
        learned[done["kind"]] = max(learned.get(done["kind"], 0), done["growthMegabytes"])
    return {"peak": peak, "learned": learned}


def languages(policy, environment):
    oracle, protocol = language_oracle()
    return oracle.notices(protocol, policy["languages"], environment)


def render(policy, messages, key, fields, environment):
    return [re.sub(r"\{([a-z]+)\}", lambda match: str(fields[match.group(1)]), messages[language][key]) for language in languages(policy, environment)]


def expect(fixture):
    policy, messages = fixture["policy"], fixture["messages"]
    return {
        "headroom": [concurrency(policy, row) for row in fixture["headroom"]],
        "languages": [languages(policy, row["environment"]) for row in fixture["languages"]],
        "messages": [render(policy, messages, row["key"], row["fields"], row["environment"]) for row in fixture["messageCases"]],
        "admission": [admits(row) for row in fixture["admission"]],
        "scenarios": [scenario(policy, row) for row in fixture["scenarios"]],
        "childHeap": {"bytes": policy["childHeapMegabytes"] * MEGABYTE, "grantIndependent": True},
    }


if __name__ == "__main__":
    command = sys.argv[1]
    fixture = json.load(open(sys.argv[2], encoding="utf8"))
    if command == "expect":
        print(json.dumps(expect(fixture), ensure_ascii=False))
    else:
        raise SystemExit("unknown command")
