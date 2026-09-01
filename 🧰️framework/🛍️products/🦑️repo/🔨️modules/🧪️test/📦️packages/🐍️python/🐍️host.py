#!/usr/bin/env python3
"""🧪️ Python native host of the repository test platform.

Invoked as ``python3 🐍️host.py --plan <plan.json> --out <results.jsonl> --adapter <🐍️component.py>``.

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


class Context:
    """🧭️ Everything one scenario handler is given: its plan slice, fixtures and work directory."""

    def __init__(self, plan: Dict[str, Any], scenario: Dict[str, Any], role: str, repo_root: str) -> None:
        self.plan = plan
        self.scenario = scenario
        self.role = role
        self.repo_root = repo_root
        self.work_dir = plan["workDir"]
        self.artifact_dir = plan.get("artifactDir") or os.path.join(plan["workDir"], "📦️artifacts")

    def artifact(self, role: str, filename: str) -> str:
        """📦️ Absolute path to write one named result artifact to, creating parent directories."""
        directory = os.path.join(self.artifact_dir, role)
        os.makedirs(directory, exist_ok=True)
        return os.path.join(directory, filename)

    def target(self) -> Dict[str, str]:
        """🪆️ The smallest owning subset this case is scoped to; a handler must not invent one."""
        target = self.plan.get("target")
        if not target:
            raise KeyError("case %s declares no subset target — Protocol v2 scopes every mutation case to its smallest owning subset" % self.plan["case"])
        return target

    def fixture(self, uri: str) -> str:
        """🧫️ Absolute path of a declared fixture; an undeclared URI is an error, never a default."""
        for entry in self.plan.get("fixtures", []):
            if entry["uri"] == uri:
                return os.path.join(self.repo_root, entry["path"])
        raise KeyError("fixture %s is not part of this plan — declare it in the feature file" % uri)

    def fixture_bytes(self, uri: str) -> bytes:
        """🧫️ Bytes of a declared fixture."""
        with open(self.fixture(uri), "rb") as handle:
            return handle.read()

    def subject_raw_bytes(self, implementation: str) -> bytes:
        """📥️ Bytes a subject host produced for an oracle that declares ``@oracle-input-subject-raw``."""
        path = self.plan.get("subjectRawInputs", {}).get(implementation)
        if not path:
            raise AssertionError("scenario %s has no raw subject output from %s; run its subject phase before this byte-decoding oracle" % (self.scenario["id"], implementation))
        with open(path, "rb") as handle:
            return handle.read()

    def copy_fixture(self, uri: str, as_name: Optional[str] = None) -> str:
        """🧫️ Copies an immutable fixture into the work directory and returns the mutable copy."""
        source = self.fixture(uri)
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
        """🔮️ Registers the reference-implementation handler for one scenario."""
        self._handlers[scenario + "::oracle"] = handler
        return self

    def subject(self, scenario: str, handler: Callable[[Context], Outcome]) -> "Adapter":
        """🎯️ Registers this repository's handler for one scenario."""
        self._handlers[scenario + "::subject"] = handler
        return self

    def handler(self, scenario: str, role: str) -> Optional[Callable[[Context], Outcome]]:
        """🔎️ The registered handler for one (scenario, role), or ``None``."""
        return self._handlers.get(scenario + "::" + role)


# endregion 🔖️Adapter


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
    args = parser.parse_args(argv)

    with open(args.plan, "r", encoding="utf-8") as handle:
        plan = json.load(handle)
    repo_root = _repo_root_from(plan["workDir"])
    os.makedirs(plan["workDir"], exist_ok=True)
    os.makedirs(plan["outputDir"], exist_ok=True)
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
        handler = adapter.handler(scenario["id"], plan["role"])
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
