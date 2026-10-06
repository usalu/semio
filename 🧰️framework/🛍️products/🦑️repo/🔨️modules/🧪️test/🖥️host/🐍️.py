#!/usr/bin/env python3
"""🧪️ Python native host of the repository test platform.

Invoked as ``python3 🐍️.py [--local-source <directory>] --plan <plan.json> --out <results.jsonl> --adapter <🐍️component.py>``.

The repository's root ``pyproject.toml`` is compose-scoped, so non-compose Python tests are NOT
discovered through it. This host owns its own configuration: it loads exactly the adapter the
coordinator names, executes exactly the planned scenarios, and emits the owned result stream. It
never parses a feature file.
"""

from __future__ import annotations

# region 🔖️Imports
import argparse
import hashlib
import importlib.util
import json
import os
import re
import shutil
import sys
import time
import traceback
from typing import Any, Callable, Dict, List, Optional

# endregion 🔖️Imports


# region 🔖️Digest
def digest(payload: Optional[bytes]) -> str:
    """#⃣ The coordinator's content digest: sha256, hex, truncated to 32 characters."""
    return hashlib.sha256(payload if payload is not None else b"").hexdigest()[:32]


def content_digest(path: str) -> str:
    """#⃣ The FULL ``sha256:<64 hex>`` content address of a produced file.

    Protocol v2 addresses fixture blobs and result artifacts by content, and a truncated digest is
    not a content address — the store's whole safety argument is that a blob's name IS its content.
    """
    with open(path, "rb") as handle:
        return "sha256:" + hashlib.sha256(handle.read()).hexdigest()


# endregion 🔖️Digest


# region 🔖️Adapter
class Outcome:
    """🎯️ What one scenario handler returns: an artifact BUNDLE plus the compared projection.

    ``production_dispatch`` is set only by a SUBJECT handler that actually invoked production
    dispatch. Its ABSENCE is how a vector-replay adapter is detected — a replayed expectation and a
    computed one are otherwise indistinguishable on the wire.
    """

    def __init__(
        self,
        projection: Any,
        raw: Optional[bytes] = None,
        diagnostics: Optional[List[Dict[str, str]]] = None,
        artifacts: Optional[List[Dict[str, str]]] = None,
        production_dispatch: Optional[Dict[str, Any]] = None,
    ) -> None:
        self.projection = projection
        self.raw = raw
        self.diagnostics = diagnostics or []
        self.artifacts = artifacts or []
        self.production_dispatch = production_dispatch

    def artifact(self, role: str, path: str, media_type: str) -> "Outcome":
        """📦️ Adds one produced file to the bundle under its role."""
        self.artifacts.append({"role": role, "path": path, "mediaType": media_type})
        return self

    def dispatched(self, operation: str, bridge_version: int) -> "Outcome":
        """🏭️ Records that this outcome came out of PRODUCTION dispatch, not a committed vector."""
        self.production_dispatch = {"invoked": True, "operation": operation, "bridgeVersion": bridge_version}
        return self


#: 🧫️ The platform's one fixture-URI grammar — the Python twin of `TEST_INPUT_URI_RE` in `🧪️test/🟦️.ts`.
FIXTURE_URI = re.compile(r"\b(shared|local|asset|schema)://([^\s\"'`,;)\]]+)")


class Context:
    """🧭️ Everything one scenario handler is given: its plan slice, fixtures and work directory."""

    def __init__(self, plan: Dict[str, Any], scenario: Dict[str, Any], role: str, repo_root: str) -> None:
        self.plan = plan
        self.scenario = scenario
        self.role = role
        self.repo_root = repo_root
        self.work_dir = plan["workDir"]
        self.artifact_dir = plan.get("artifactDir") or os.path.join(plan["workDir"], "📦️artifacts")

    def row(self) -> str:
        """🪆️ The Examples row id this scenario expands, or an error for a plain scenario."""
        outline = self.scenario.get("outlineOf") or ""
        if not outline or not self.scenario["id"].startswith(outline + "-"):
            raise AssertionError("scenario %s expands no Scenario Outline row" % self.scenario["id"])
        return self.scenario["id"][len(outline) + 1 :]

    def doc_string(self) -> str:
        """📜️ The scenario's first doc string — the feature-owned input vector (the twin of the Rust runner's)."""
        for step in self.scenario["steps"]:
            if step.get("docString") is not None:
                return step["docString"]
        raise AssertionError("scenario %s carries no doc string" % self.scenario["id"])

    def doc_json(self) -> Any:
        """📜️ The scenario's first doc string, parsed as JSON."""
        return json.loads(self.doc_string())

    def artifact(self, role: str, filename: str) -> str:
        """📦️ Absolute path to write one named result artifact to — ``<artifact_dir>/<scenario id>/<role>/<filename>``, so a
        scenario's artifacts never overwrite another's — creating parent directories."""
        directory = os.path.join(self.artifact_dir, self.scenario["id"], role)
        os.makedirs(directory, exist_ok=True)
        return os.path.join(directory, filename)

    def target(self) -> Dict[str, str]:
        """🪆️ The smallest owning subset this case is scoped to; a handler must not invent one."""
        target = self.plan.get("target")
        if not target:
            raise KeyError("case %s declares no subset target — Protocol v2 scopes every mutation case to its smallest owning subset" % self.plan["case"])
        return target

    def input(self, uri: str) -> str:
        """🧫️ Absolute path of a declared fixture; an undeclared URI is an error, never a default."""
        for entry in self.plan.get("inputs", []):
            if entry["uri"] == uri:
                return os.path.join(self.repo_root, entry["path"])
        raise KeyError("fixture %s is not part of this plan — declare it in the feature file" % uri)

    def step_input_uris(self) -> List[str]:
        """🔗️ Every fixture URI the scenario's steps name — step text and data-table cells, in step order, whatever
        scheme the feature uses. The feature is the single place a vector path is written down."""
        return [match.group(0) for step in self.scenario["steps"] for text in [step.get("text", "")] + [cell for row in (step.get("dataTable") or []) for cell in row] for match in FIXTURE_URI.finditer(text)]

    def input_bytes(self, uri: str) -> bytes:
        """🧫️ Bytes of a declared fixture."""
        with open(self.input(uri), "rb") as handle:
            return handle.read()

    def subject_raw_bytes(self, implementation: str) -> bytes:
        """📥️ Bytes THIS scenario's subject host produced in ``implementation``, for an ``@oracle-input-subject-raw`` oracle."""
        path = self.plan.get("subjectRawInputs", {}).get(self.scenario["id"], {}).get(implementation)
        if not path:
            raise AssertionError("scenario %s has no raw subject output from %s; run its subject phase before this byte-decoding oracle" % (self.scenario["id"], implementation))
        with open(path, "rb") as handle:
            return handle.read()

    def copy_input(self, uri: str, as_name: Optional[str] = None) -> str:
        """🧫️ Copies an immutable fixture into the work directory and returns the mutable copy."""
        source = self.input(uri)
        os.makedirs(self.work_dir, exist_ok=True)
        target = os.path.join(self.work_dir, as_name or os.path.basename(source))
        shutil.copyfile(source, target)
        return target

    @property
    def seed(self) -> int:
        """🎲️ Deterministic seed declared by the scenario's ``@seed-…`` tag."""
        try:
            return int(self.scenario.get("seed") or 0)
        except ValueError:
            return 0


class Adapter:
    """🧭️ One implementation's registration for a case: which scenarios it serves, in which roles."""

    def __init__(self, implementation: str = "python") -> None:
        self.implementation = implementation
        self._handlers: Dict[str, Callable[[Context], Outcome]] = {}

    def oracle(self, scenario: str, handler: Callable[[Context], Outcome]) -> "Adapter":
        """🔮️ Registers the reference-implementation handler for one scenario id, or for a Scenario
        Outline's base id (``@id-<base>``), which then serves every row the feature expands."""
        self._handlers[scenario + "::oracle"] = handler
        return self

    def subject(self, scenario: str, handler: Callable[[Context], Outcome]) -> "Adapter":
        """🎯️ Registers this repository's handler for one scenario id, or for a Scenario Outline's base id."""
        self._handlers[scenario + "::subject"] = handler
        return self

    def handler(self, scenario: Dict[str, Any], role: str) -> Optional[Callable[[Context], Outcome]]:
        """🔎️ The handler registered for the scenario's own id, else for its outline's base id, or ``None``."""
        exact = self._handlers.get(scenario["id"] + "::" + role)
        if exact is not None or not scenario.get("outlineOf"):
            return exact
        return self._handlers.get(scenario["outlineOf"] + "::" + role)


# endregion 🔖️Adapter


# region 🩹️SnapshotPatch
def patched_snapshot(snapshot: Any, patch: Dict[str, Any]) -> Any:
    """🩹️ A ``patch-snapshot`` row's ONE pointer operation (``set``, ``insert`` with an optional object member
    ``index``, ``remove``, ``move``, ``rename``, ``splice`` over array items / UTF-8 bytes of text / object members)
    applied to a reference's OWN snapshot reading, written from RFC 6901 alone — the Python twin of the Rust host's
    ``law::patched_snapshot``. Returns a new value; a pointer the reading lacks is an AssertionError.
    """
    document = json.loads(json.dumps(snapshot))

    def segments(key: str) -> List[str]:
        text = patch.get(key, "")
        if text == "":
            return []
        if not text.startswith("/"):
            raise AssertionError("patch %s %r is not an RFC 6901 pointer" % (key, text))
        return [segment.replace("~1", "/").replace("~0", "~") for segment in text[1:].split("/")]

    def node(path: List[str]) -> Any:
        current = document
        for segment in path:
            if isinstance(current, dict) and segment in current:
                current = current[segment]
            elif isinstance(current, list) and segment.isdigit() and int(segment) < len(current):
                current = current[int(segment)]
            else:
                raise AssertionError("pointer segment %r is absent" % segment)
        return current

    def take(path: List[str]) -> Any:
        if not path:
            raise AssertionError("a removal addresses the document root")
        parent, key = node(path[:-1]), path[-1]
        if isinstance(parent, dict) and key in parent:
            return parent.pop(key)
        if isinstance(parent, list) and key.isdigit() and int(key) < len(parent):
            return parent.pop(int(key))
        raise AssertionError("removal target %r is absent" % key)

    def insert(path: List[str], value: Any, index: Optional[int]) -> None:
        if not path:
            raise AssertionError("an insertion addresses the document root")
        parent, key = node(path[:-1]), path[-1]
        if isinstance(parent, dict) and key not in parent:
            items = list(parent.items())
            position = len(items) if index is None else min(index, len(items))
            items.insert(position, (key, value))
            parent.clear()
            parent.update(items)
        elif isinstance(parent, list) and (key == "-" or (key.isdigit() and int(key) <= len(parent))):
            parent.insert(len(parent) if key == "-" else int(key), value)
        else:
            raise AssertionError("insertion at %r needs an absent object member or an array index" % key)

    operation = patch.get("operation")
    if operation == "set":
        path = segments("path")
        if not path:
            return json.loads(json.dumps(patch["value"]))
        node(path)
        parent = node(path[:-1])
        parent[int(path[-1]) if isinstance(parent, list) else path[-1]] = patch["value"]
    elif operation == "insert":
        insert(segments("path"), patch["value"], patch.get("index"))
    elif operation == "remove":
        take(segments("path"))
    elif operation == "move":
        insert(segments("path"), take(segments("from")), patch.get("index"))
    elif operation == "rename":
        path = segments("path")
        parent = node(path[:-1])
        if not isinstance(parent, dict) or path[-1] not in parent or (patch["key"] != path[-1] and patch["key"] in parent):
            raise AssertionError("rename of %r to %r does not fit its object" % (path[-1], patch["key"]))
        items = [(patch["key"] if name == path[-1] else name, value) for name, value in parent.items()]
        parent.clear()
        parent.update(items)
    elif operation == "splice":
        path, offset, remove, value = segments("path"), patch["offset"], patch["remove"], patch["value"]
        target = node(path)
        if isinstance(target, list) and isinstance(value, list) and offset + remove <= len(target):
            target[offset:offset + remove] = value
        elif isinstance(target, dict) and isinstance(value, dict) and offset + remove <= len(target):
            items = list(target.items())
            items[offset:offset + remove] = list(value.items())
            target.clear()
            target.update(items)
        elif isinstance(target, str) and isinstance(value, str):
            encoded = target.encode("utf-8")
            boundary = lambda at: at == len(encoded) or (at < len(encoded) and encoded[at] & 0xC0 != 0x80)
            if offset + remove > len(encoded) or not boundary(offset) or not boundary(offset + remove):
                raise AssertionError("a text splice must address UTF-8 character boundaries")
            replaced = (encoded[:offset] + value.encode("utf-8") + encoded[offset + remove:]).decode("utf-8")
            if not path:
                return replaced
            parent = node(path[:-1])
            parent[int(path[-1]) if isinstance(parent, list) else path[-1]] = replaced
        else:
            raise AssertionError("splice range or value does not fit the addressed container")
    else:
        raise AssertionError("unknown snapshot patch operation %r" % operation)
    return document


def snapshot_patch_inverse(snapshot: Any, patch: Dict[str, Any]) -> Dict[str, Any]:
    """↩️ The exact inverse of ONE ``patch`` taken against ``snapshot`` (the state it applies to), read from the patch
    semantics alone: a set restores the prior value, an insert removes what it added, a removal re-inserts the value
    (an object member at its former position), a move moves back, a rename renames back, a splice splices the removed
    units back.
    """
    def pointer(path: List[str]) -> str:
        return "".join("/" + segment.replace("~", "~0").replace("/", "~1") for segment in path)

    def segments(text: str) -> List[str]:
        return [] if text == "" else [segment.replace("~1", "/").replace("~0", "~") for segment in text[1:].split("/")]

    def value_at(path: List[str]) -> Any:
        current = snapshot
        for segment in path:
            current = current[int(segment)] if isinstance(current, list) else current[segment]
        return current

    def position(path: List[str]) -> Optional[int]:
        parent = value_at(path[:-1])
        return list(parent).index(path[-1]) if isinstance(parent, dict) else None

    operation = patch["operation"]
    path = segments(patch.get("path", ""))
    if operation == "set":
        return {"operation": "set", "path": patch["path"], "value": json.loads(json.dumps(value_at(path)))}
    if operation == "insert":
        parent = value_at(path[:-1])
        landed = path[:-1] + [str(len(parent))] if isinstance(parent, list) and path[-1] == "-" else path
        return {"operation": "remove", "path": pointer(landed)}
    if operation == "remove":
        inverse = {"operation": "insert", "path": patch["path"], "value": json.loads(json.dumps(value_at(path)))}
        member = position(path)
        return inverse if member is None else {**inverse, "index": member}
    if operation == "move":
        source = segments(patch["from"])
        moved = patched_snapshot(snapshot, {"operation": "remove", "path": patch["from"]})
        parent = path[:-1]
        container = moved
        for segment in parent:
            container = container[int(segment)] if isinstance(container, list) else container[segment]
        landed = parent + [str(len(container))] if isinstance(container, list) and path[-1] == "-" else path
        inverse = {"operation": "move", "from": pointer(landed), "path": patch["from"]}
        member = position(source)
        return inverse if member is None else {**inverse, "index": member}
    if operation == "rename":
        return {"operation": "rename", "path": pointer(path[:-1] + [patch["key"]]), "key": path[-1]}
    if operation == "splice":
        target = value_at(path)
        offset, remove = patch["offset"], patch["remove"]
        if isinstance(target, str):
            encoded = target.encode("utf-8")
            removed: Any = encoded[offset:offset + remove].decode("utf-8")
            inserted = len(patch["value"].encode("utf-8"))
        elif isinstance(target, dict):
            removed = dict(list(target.items())[offset:offset + remove])
            inserted = len(patch["value"])
        else:
            removed = target[offset:offset + remove]
            inserted = len(patch["value"])
        return {"operation": "splice", "path": patch["path"], "offset": offset, "remove": inserted, "value": json.loads(json.dumps(removed))}
    raise AssertionError("unknown snapshot patch operation %r" % operation)


# endregion 🩹️SnapshotPatch


# region 🔖️Runner
def _repo_root_from(start: str) -> str:
    directory = os.path.abspath(start)
    for _ in range(32):
        if os.path.exists(os.path.join(directory, "nx.json")) and os.path.exists(os.path.join(directory, "package.json")):
            return directory
        parent = os.path.dirname(directory)
        if parent == directory:
            break
        directory = parent
    return os.getcwd()


def _prioritize_local_source_paths(paths: List[str]) -> None:
    ordered: List[str] = []
    for path in paths:
        absolute = os.path.abspath(path)
        if not os.path.isdir(absolute):
            raise NotADirectoryError("declared local Python source is not a directory: %s" % path)
        if os.path.normcase(absolute) not in [os.path.normcase(entry) for entry in ordered]:
            ordered.append(absolute)
    declared = {os.path.normcase(path) for path in ordered}
    remainder = [path for path in sys.path if os.path.normcase(os.path.abspath(path or os.getcwd())) not in declared]
    sys.path[:] = ordered + remainder


def _load_adapter(adapter_path: str) -> Adapter:
    sys.modules["semio_repo_test"] = sys.modules[__name__]
    spec = importlib.util.spec_from_file_location("semio_test_adapter", adapter_path)
    if spec is None or spec.loader is None:
        raise ImportError("cannot load adapter %s" % adapter_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    factory = getattr(module, "adapter", None)
    if factory is None:
        raise AttributeError("%s must define `def adapter() -> Adapter`" % adapter_path)
    return factory()


def run_main(argv: List[str]) -> int:
    """🚪️ Python host entry: load plan, load adapter, execute, emit JSONL."""
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--plan", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--adapter", required=True)
    parser.add_argument("--local-source", action="append", default=[])
    args = parser.parse_args(argv)

    with open(args.plan, "r", encoding="utf-8") as handle:
        plan = json.load(handle)
    repo_root = _repo_root_from(plan["workDir"])
    os.makedirs(plan["workDir"], exist_ok=True)
    os.makedirs(plan["outputDir"], exist_ok=True)
    _prioritize_local_source_paths(args.local_source)
    adapter = _load_adapter(args.adapter)

    lines: List[str] = []
    failed = False
    for scenario in plan.get("scenarios", []):
        started = time.time()
        result: Dict[str, Any] = {
            "schemaVersion": 2,
            "testId": "%s::%s::%s::%s::%s" % (plan["owner"], plan["case"], scenario["id"], plan["implementation"], plan["role"]),
            "baselineSha": plan.get("baselineSha", ""),
            "owner": plan["owner"],
            "case": plan["case"],
            "scenario": scenario["id"],
            "implementation": plan["implementation"],
            "role": plan["role"],
            "level": scenario["level"],
            "platform": plan.get("platform", ""),
            "seed": scenario.get("seed", ""),
            "featureHash": plan.get("featureHash", ""),
            "artifacts": [],
            "diagnostics": [],
        }
        handler = adapter.handler(scenario, plan["role"])
        if handler is None:
            failed = True
            result["status"] = "errored"
            result["output"] = {"rawHash": digest(None), "projectionHash": digest(None), "projection": None}
            result["diagnostics"] = [{"severity": "error", "message": "adapter has no %s registration for scenario %s" % (plan["role"], scenario["id"])}]
        else:
            try:
                outcome = handler(Context(plan, scenario, plan["role"], repo_root))
                payload = json.dumps(outcome.projection, sort_keys=False, separators=(",", ":")).encode("utf-8")
                result["status"] = "passed"
                result["output"] = {"rawHash": digest(outcome.raw), "projectionHash": digest(payload), "projection": outcome.projection}
                if outcome.raw is not None:
                    raw_path = os.path.join(plan["outputDir"], "%s.%s.raw" % (scenario["id"], plan["role"]))
                    with open(raw_path, "wb") as handle:
                        handle.write(outcome.raw)
                    result["output"]["rawPath"] = raw_path
                projection_path = os.path.join(plan["outputDir"], "%s.%s.projection.json" % (scenario["id"], plan["role"]))
                with open(projection_path, "wb") as handle:
                    handle.write(payload)
                result["output"]["projectionPath"] = projection_path
                # 📦️Every produced file is re-hashed HERE rather than trusted from the handler: the
                # digest a comparison stage keys on must describe the bytes that reached disk.
                result["artifacts"] = [
                    {
                        "role": artifact["role"],
                        "path": artifact["path"],
                        "mediaType": artifact["mediaType"],
                        "sha256": content_digest(artifact["path"]),
                        "bytes": os.path.getsize(artifact["path"]),
                    }
                    for artifact in outcome.artifacts
                ]
                if outcome.production_dispatch is not None:
                    result["productionDispatch"] = outcome.production_dispatch
                result["diagnostics"] = outcome.diagnostics
            except AssertionError as error:
                failed = True
                result["status"] = "failed"
                result["output"] = {"rawHash": digest(None), "projectionHash": digest(None), "projection": None}
                result["diagnostics"] = [{"severity": "error", "message": str(error), "detail": traceback.format_exc()}]
            except Exception as error:  # noqa: BLE001 — any host failure is a result, never a skip
                failed = True
                result["status"] = "errored"
                result["output"] = {"rawHash": digest(None), "projectionHash": digest(None), "projection": None}
                result["diagnostics"] = [{"severity": "error", "message": str(error), "detail": traceback.format_exc()}]
        result["durationMs"] = round((time.time() - started) * 1000.0, 3)
        lines.append(json.dumps(result))

    os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines) + ("\n" if lines else ""))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(run_main(sys.argv[1:]))
# endregion 🔖️Runner
