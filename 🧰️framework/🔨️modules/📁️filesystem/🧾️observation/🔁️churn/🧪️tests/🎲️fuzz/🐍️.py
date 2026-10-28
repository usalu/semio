#!/usr/bin/env python3
"""🐍️ Churn fuzz harness (independent of the TypeScript library): a deterministic peer that renames, deletes, rewrites and tears files of a fixture workspace while a fixture build (🟨️.mjs, run under the restart supervisor of the library under test) observes it.

  🐍️.py run  --lib ROOT --variant NAME --seeds A:B [--mode sync|timed] [--jobs 4] --out DIR [--ops 6] [--naive]
  🐍️.py peer --work DIR --seed N --mode sync|timed --ops N        (started by `run`)

The peer's schedule is a pure function of the seed. In `sync` mode every operation is bound to a tick the build raises at fixed points (start, impl, before each file hash, hashed), so
the interleaving is exactly reproducible; in `timed` mode operations fire at wall-clock offsets. The oracle is the list of tree states the peer produced: a build that wrote a manifest that
equals none of them consumed a MIX of two tree states; one that equals a state that only existed halfway through a rewrite consumed a PARTIAL write.
"""
import argparse
import hashlib
import json
import os
import random
import shutil
import subprocess
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
DIRS, FILES = 6, 8


def digest_of(model):
    manifest = {path: hashlib.sha256(body).hexdigest() for path, body in sorted(model.items())}
    return hashlib.sha256(json.dumps(manifest, separators=(",", ":"), sort_keys=True).encode()).hexdigest()


def body_of(path, version):
    return (("%s version %d\n" % (path, version)) * (2 + (sum(path.encode()) + version) % 5)).encode()


def make_workspace(work):
    shutil.rmtree(work, ignore_errors=True)
    for sub in ("src", "impl", "ctl", "out", "tmp"):
        os.makedirs(os.path.join(work, sub))
    for name in ("AGENTS.md", "package.json"):
        open(os.path.join(work, name), "w").write("{}")
    model = {}
    for directory in range(DIRS):
        os.makedirs(os.path.join(work, "src", "d%d" % directory))
        for number in range(FILES):
            relative = "d%d/f%d.txt" % (directory, number)
            model[relative] = body_of(relative, 0)
            open(os.path.join(work, "src", relative), "wb").write(model[relative])
    open(os.path.join(work, "impl", "protocol.json"), "w").write('{"version": 0}')
    open(os.path.join(work, "impl", "a.mjs"), "w").write('export const a = "a1";\n')
    open(os.path.join(work, "impl", "b.mjs"), "w").write('import {lstatSync} from "node:fs";\nimport {a} from "./a.mjs";\nglobalThis.__tick?.("impl-b");\nconst peer = lstatSync(new URL("./a.mjs", import.meta.url));\nexport const b = a + ":" + peer.size;\n')
    open(os.path.join(work, "impl", "main.mjs"), "w").write('import {b} from "./b.mjs";\nexport const version = () => "impl:" + b.split(":")[0];\n')
    return model


KINDS = [("rewrite", 4), ("torn", 3), ("delete", 3), ("rename", 4), ("swap", 2), ("replace-dir", 1), ("move-dir", 2), ("create", 2), ("impl-rename", 2), ("protocol-edit", 3), ("touch", 1), ("delete-dir", 1)]


class Peer:
    def __init__(self, work, seed, mode, count):
        self.work, self.seed, self.mode = work, seed, mode
        self.rng = random.Random(seed)
        self.model = {}
        for directory in range(DIRS):
            for number in range(FILES):
                relative = "d%d/f%d.txt" % (directory, number)
                self.model[relative] = body_of(relative, 0)
        self.lock = threading.RLock()
        self.busy = set()
        self.threads = []
        self.states = [digest_of(self.model)]
        self.torn = []
        self.applied = []
        self.version = 0
        schedule = random.Random(seed * 7919 + 13)
        kinds = [kind for kind, weight in KINDS for _ in range(weight)]
        self.ops = sorted(({"kind": schedule.choice(kinds), "at": schedule.choice([schedule.randint(0, 3), schedule.randint(0, 55), schedule.randint(0, 55)]), "atMs": schedule.randint(150, 800)} for _ in range(count)), key=lambda row: row["atMs" if mode == "timed" else "at"])

    def src(self, relative):
        return os.path.join(self.work, "src", relative)

    def record(self, torn=False):
        with self.lock:
            (self.torn if torn else self.states).append(digest_of(self.model))

    def pick(self):
        names = sorted(name for name in self.model if name not in self.busy)
        return self.rng.choice(names) if names else None

    def apply(self, op):
        with self.lock:
            try:
                self.apply_locked(op)
            except OSError as error:
                self.applied.append({"kind": op["kind"], "error": str(error)[-120:]})
                self.resync()

    def resync(self):
        model = {}
        for directory in sorted(os.listdir(os.path.join(self.work, "src"))):
            base = os.path.join(self.work, "src", directory)
            if os.path.isdir(base):
                for name in sorted(os.listdir(base)):
                    if os.path.isfile(os.path.join(base, name)):
                        model[directory + "/" + name] = open(os.path.join(base, name), "rb").read()
        self.model = model
        self.record()

    def apply_locked(self, op):
        kind, detail = op["kind"], {}
        self.version += 1
        if kind in ("rewrite", "torn", "delete", "rename", "touch", "swap", "replace-dir") and not self.pick():
            return
        if kind == "rewrite":
            name = self.pick()
            temporary = os.path.join(self.work, "tmp", "rewrite")
            body = body_of(name, self.version)
            open(temporary, "wb").write(body)
            os.replace(temporary, self.src(name))
            self.model[name] = body
            self.record()
            detail = {"file": name}
        elif kind == "torn":
            name = self.pick()
            body = body_of(name, self.version)
            handle = open(self.src(name), "wb")
            self.model[name] = b""
            self.busy.add(name)
            self.record(torn=True)

            def finish(handle=handle, name=name, body=body):
                time.sleep(0.15)
                with self.lock:
                    handle.write(body)
                    handle.close()
                    self.model[name] = body
                    self.busy.discard(name)
                    self.record()
            thread = threading.Thread(target=finish)
            thread.start()
            self.threads.append(thread)
            detail = {"file": name}
        elif kind == "delete":
            name = self.pick()
            os.remove(self.src(name))
            del self.model[name]
            self.record()
            detail = {"file": name}
        elif kind == "rename":
            name = self.pick()
            target = "d%d/r%d.txt" % (self.rng.randrange(DIRS), self.version)
            os.makedirs(os.path.dirname(self.src(target)), exist_ok=True)
            os.rename(self.src(name), self.src(target))
            self.model[target] = self.model.pop(name)
            self.record()
            detail = {"from": name, "to": target}
        elif kind == "swap":
            first, second = self.pick(), self.pick()
            if first == second:
                return
            temporary = os.path.join(self.work, "tmp", "swap")
            os.rename(self.src(first), temporary)
            body = self.model.pop(first)
            self.record()
            os.rename(self.src(second), self.src(first))
            self.model[first] = self.model.pop(second)
            self.record()
            os.rename(temporary, self.src(second))
            self.model[second] = body
            self.record()
            detail = {"a": first, "b": second}
        elif kind == "replace-dir":
            name = self.pick()
            os.remove(self.src(name))
            del self.model[name]
            self.record()
            os.mkdir(self.src(name))
            detail = {"file": name}
        elif kind == "move-dir":
            directory = "d%d" % self.rng.randrange(DIRS)
            moved = {path: body for path, body in self.model.items() if path.startswith(directory + "/")}
            if not moved:
                return
            parked = os.path.join(self.work, "tmp", directory + ".moved")
            os.rename(self.src(directory), parked)
            for path in moved:
                del self.model[path]
            self.record()

            def restore(directory=directory, moved=moved, parked=parked):
                time.sleep(0.2)
                with self.lock:
                    os.makedirs(self.src(directory), exist_ok=True)
                    for path in moved:
                        os.rename(os.path.join(parked, os.path.basename(path)), self.src(path))
                    os.rmdir(parked)
                    self.model.update(moved)
                    self.record()
            thread = threading.Thread(target=restore)
            thread.start()
            self.threads.append(thread)
            detail = {"dir": directory}
        elif kind == "create":
            name = "d%d/n%d.txt" % (self.rng.randrange(DIRS), self.version)
            body = body_of(name, self.version)
            os.makedirs(os.path.dirname(self.src(name)), exist_ok=True)
            open(self.src(name), "wb").write(body)
            self.model[name] = body
            self.record()
            detail = {"file": name}
        elif kind == "impl-rename":
            path = os.path.join(self.work, "impl", "a.mjs")
            if not os.path.exists(path):
                return
            os.rename(path, path + ".away")

            def back(path=path):
                time.sleep(0.2)
                with self.lock:
                    os.rename(path + ".away", path)
            thread = threading.Thread(target=back)
            thread.start()
            self.threads.append(thread)
        elif kind == "protocol-edit":
            path = os.path.join(self.work, "impl", "protocol.json")
            temporary = os.path.join(self.work, "tmp", "protocol")
            open(temporary, "w").write(json.dumps({"version": self.version}))
            os.replace(temporary, path)
            detail = {"version": self.version}
        elif kind == "touch":
            name = self.pick()
            now = time.time()
            os.utime(self.src(name), (now, now))
            detail = {"file": name}
        elif kind == "delete-dir":
            directory = "d%d" % self.rng.randrange(DIRS)
            victims = [path for path in self.model if path.startswith(directory + "/")]
            if not victims:
                return
            shutil.rmtree(self.src(directory))
            for path in victims:
                del self.model[path]
            self.record()
            detail = {"dir": directory}
        self.applied.append({"kind": kind, **detail})

    def serve(self):
        ctl = os.path.join(self.work, "ctl")
        started, seen, tick, pending = time.time(), set(), 0, list(self.ops)
        while not os.path.exists(os.path.join(ctl, "stop")):
            if self.mode == "timed":
                while pending and pending[0]["atMs"] <= (time.time() - started) * 1000:
                    self.apply(pending.pop(0))
            else:
                names = sorted((name for name in os.listdir(ctl) if name.startswith("req-") and name not in seen), key=lambda name: os.stat(os.path.join(ctl, name)).st_mtime_ns)
                for name in names:
                    seen.add(name)
                    tick += 1
                    while pending and pending[0]["at"] <= tick:
                        self.apply(pending.pop(0))
                    open(os.path.join(ctl, "ack-" + name[4:]), "w").close()
            time.sleep(0.001)
        for thread in self.threads:
            thread.join()
        json.dump({"seed": self.seed, "applied": self.applied, "states": self.states, "torn": self.torn, "final": digest_of(self.model), "pending": len(pending)}, open(os.path.join(self.work, "peer.json"), "w"))


def classify(work, code, stdout, stderr, wall):
    peer_path = os.path.join(work, "peer.json")
    if not os.path.exists(peer_path):
        return {"code": code, "wall": round(wall, 2), "applied": 0, "kinds": [], "outcome": "harness-error", "error": open(os.path.join(work, "peer.log")).read()[-300:]}
    peer = json.load(open(peer_path))
    row = {"code": code, "wall": round(wall, 2), "applied": len(peer["applied"]), "kinds": [item["kind"] for item in peer["applied"]]}
    result_path = os.path.join(work, "out", "result.json")
    if code == "timeout":
        row["outcome"] = "hang"
    elif code == 0 and os.path.exists(result_path):
        result = json.load(open(result_path))
        row.update(restarts=result["restarts"], attempts=result["attempts"])
        if result["digest"] in peer["torn"] and result["digest"] not in peer["states"]:
            row["outcome"] = "partial"
        elif result["digest"] in peer["states"]:
            row["outcome"] = "consistent"
        else:
            row["outcome"] = "mix"
    elif code == 75:
        notices = ("Waiting" in stderr or "Another process" in stderr or "Restarting" in stderr) and ("Warte" in stderr or "Ein anderer Prozess" in stderr or "Neustart" in stderr)
        row["outcome"] = "exhausted75" if notices else "bare75"
    else:
        lines = [line for line in (stderr + stdout).splitlines() if "rror" in line or "ENOENT" in line or "ENOTDIR" in line]
        row["outcome"] = "crash"
        row["error"] = (lines[0] if lines else (stderr or stdout)[-200:])[:240]
    return row


def run_seed(args, seed):
    work = os.path.join(args.out, "%s-%s-%d" % (args.variant, args.mode, seed))
    make_workspace(work)
    peer_log = open(os.path.join(work, "peer.log"), "w")
    peer = subprocess.Popen([sys.executable, "-I", os.path.abspath(__file__), "peer", "--work", work, "--seed", str(seed), "--mode", args.mode, "--ops", str(args.ops)], stdout=peer_log, stderr=peer_log)
    command = ["bun", os.path.join(HERE, "🟨️.mjs"), "--lib", args.lib, "--work", work, "--workMs", str(args.work_ms), "--scale", str(args.scale)] + (["--sync"] if args.mode == "sync" else []) + (["--naive"] if args.naive else [])
    started = time.time()
    build = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, cwd=work, start_new_session=True, env={**os.environ, "SEMIO_WORKSPACE_ROOT": work, "SEMIO_LANGUAGE": "", "LANG": "", "LC_ALL": "", "SEMIO_STALE_REEXEC": "0"})
    try:
        stdout, stderr = build.communicate(timeout=args.timeout)
        code = build.returncode
    except subprocess.TimeoutExpired:
        os.killpg(build.pid, 9)
        stdout, stderr = build.communicate()
        code = "timeout"
    try:
        os.killpg(build.pid, 9)
    except ProcessLookupError:
        pass
    wall = time.time() - started
    open(os.path.join(work, "ctl", "stop"), "w").close()
    peer.wait(timeout=30)
    row = classify(work, code, stdout, stderr, wall)
    row.update(seed=seed, variant=args.variant, mode=args.mode)
    open(os.path.join(work, "build.stderr"), "w").write(stderr)
    if row["outcome"] in ("consistent",) and not args.keep:
        shutil.rmtree(work, ignore_errors=True)
    return row


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["run", "peer"])
    parser.add_argument("--lib")
    parser.add_argument("--variant", default="v")
    parser.add_argument("--seeds", default="0:10")
    parser.add_argument("--mode", default="sync", choices=["sync", "timed"])
    parser.add_argument("--jobs", type=int, default=3)
    parser.add_argument("--out", default=".")
    parser.add_argument("--ops", type=int, default=6)
    parser.add_argument("--work")
    parser.add_argument("--seed", type=int)
    parser.add_argument("--work-ms", dest="work_ms", type=int, default=5)
    parser.add_argument("--scale", type=float, default=0.05)
    parser.add_argument("--timeout", type=int, default=90)
    parser.add_argument("--keep", action="store_true")
    parser.add_argument("--naive", action="store_true")
    args = parser.parse_args()
    if args.command == "peer":
        Peer(args.work, args.seed, args.mode, args.ops).serve()
        return
    first, last = (int(part) for part in args.seeds.split(":"))
    args.out, args.lib = os.path.abspath(args.out), os.path.abspath(args.lib)
    os.makedirs(args.out, exist_ok=True)
    with ThreadPoolExecutor(args.jobs) as pool:
        rows = list(pool.map(lambda seed: run_seed(args, seed), range(first, last)))
    path = os.path.join(args.out, "%s-%s.jsonl" % (args.variant, args.mode))
    with open(path, "w") as handle:
        for row in rows:
            handle.write(json.dumps(row, ensure_ascii=False) + "\n")
    counts = {}
    for row in rows:
        counts[row["outcome"]] = counts.get(row["outcome"], 0) + 1
    print(json.dumps({"variant": args.variant, "mode": args.mode, "seeds": len(rows), "outcomes": counts, "meanWall": round(sum(row["wall"] for row in rows) / len(rows), 2), "restarted": sum(1 for row in rows if row.get("restarts", 0) > 0), "retried": sum(1 for row in rows if row.get("attempts", 1) > 1)}))


if __name__ == "__main__":
    main()
