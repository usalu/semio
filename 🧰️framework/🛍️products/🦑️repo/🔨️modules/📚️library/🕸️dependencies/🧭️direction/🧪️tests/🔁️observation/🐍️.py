"""🐍️ Independent oracle of the dependency direction re-observation: replays each scripted world with its own loop, decides concurrent edits by set difference of two inventories and prints the verdicts without sharing a line with the implementation."""
import json
import sys


class Moved(Exception):
    def __init__(self, reason, path):
        super().__init__(path)
        self.reason, self.path = reason, path


class Refused(Exception):
    pass


class PassFailed(Exception):
    pass


def replay(maximum, events):
    queue = list(events)
    runs = [0]

    def observe():
        event = queue.pop(0)
        if "observeFails" in event:
            if event["observeFails"] == "ENOENT":
                raise Moved("vanished", event.get("path", "inventory"))
            raise Refused()
        return event["observe"]

    def run():
        event = queue.pop(0)
        runs[0] += 1
        if event["run"] != "ok":
            raise PassFailed()

    attempt = 0
    while True:
        attempt += 1
        try:
            first = observe()
            try:
                run()
                return {"result": "ok", "runs": runs[0]}
            except PassFailed:
                second = observe()
                gone = [name for name in first if name not in second]
                added = [name for name in second if name not in first]
                if gone:
                    raise Moved("vanished", gone[0])
                if added:
                    raise Moved("appeared", added[0])
                raise
        except Moved as moved:
            if attempt >= maximum:
                return {"result": "exhausted", "runs": runs[0], "reason": moved.reason, "path": moved.path}
        except (PassFailed, Refused):
            return {"result": "failed", "runs": runs[0]}


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        fixture = json.load(handle)
    with open(sys.argv[2], encoding="utf-8") as handle:
        maximum = json.load(handle)["definitions"]["Protocol"]["const"]["profiles"]["build"]["maximumAttempts"]
    print(json.dumps({row["id"]: replay(maximum, row["events"]) for row in fixture["scenarios"]}, ensure_ascii=False))


main()
