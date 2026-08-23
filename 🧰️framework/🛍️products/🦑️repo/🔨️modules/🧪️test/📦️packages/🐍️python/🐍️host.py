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


# endregion 🔖️Digest


# region 🔖️Adapter
class Outcome:
    """🎯️ What one scenario handler returns: the raw artifact and the compared projection."""

    def __init__(self, projection: Any, raw: Optional[bytes] = None, diagnostics: Optional[List[Dict[str, str]]] = None) -> None:
        self.projection = projection
        self.raw = raw
        self.diagnostics = diagnostics or []


class Context:
    """🧭️ Everything one scenario handler is given: its plan slice, fixtures and work directory."""

    def __init__(self, plan: Dict[str, Any], scenario: Dict[str, Any], role: str, repo_root: str) -> None:
        self.plan = plan
        self.scenario = scenario
        self.role = role
        self.repo_root = repo_root
        self.work_dir = plan["workDir"]

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
            "testId": "%s::%s::%s::%s::%s" % (plan["owner"], plan["case"], scenario["id"], plan["implementation"], plan["role"]),
            "owner": plan["owner"],
            "case": plan["case"],
            "scenario": scenario["id"],
            "implementation": plan["implementation"],
            "role": plan["role"],
            "level": scenario["level"],
            "seed": scenario.get("seed", ""),
            "featureHash": plan.get("featureHash", ""),
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
