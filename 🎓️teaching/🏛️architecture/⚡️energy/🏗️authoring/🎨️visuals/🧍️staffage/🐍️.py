"""🧍 Handcrafted staffage casts as Manim mobjects: the course draws people, furniture, devices and building parts only from the staffage archive.

A cast is a named, figure-only rendering listed in `🔣️.json`. The TypeScript exporter renders missing casts in one batch into a content-addressed cache,
and this module turns the frames into transformed VGroups with the world origin on `anchor` and `unit` Manim units per metre.

@see ../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts — the cast exporter
@see ../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json — the manifest schema
"""

from __future__ import annotations

import contextlib
import errno
import hashlib
import ast
import json
import os
import re
import shutil
import subprocess
import signal
import sys
import threading
import time
import uuid
from collections import OrderedDict
from functools import lru_cache
from pathlib import Path
from typing import Callable, NamedTuple

import numpy as np
from manim import ORIGIN, VectorizedPoint, VGroup, VMobject

_WORKSPACE = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
_SOURCES = _WORKSPACE / "🎓️teaching/🏛️architecture/🧍️staffage"
_EXPORTER = _SOURCES / "🔨️modules/📤️export/🎭️cast/🟦️.ts"
_MANIFEST = Path(__file__).with_name("🔣️.json")
_INDEX = "🔣️.json"
_VIEWPOINTS = ("elevation", "front", "plan", "axonometric")
_VIEWS = ("dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments")
_THEMES = ("dark", "light")
_SCALES = (500, 200, 100, 50)
_TRANSPORT_LAYERS = ("parcels", "pathlines", "streamlines", "condensation")
_CAST_KEYS = frozenset(("asset", "pose", "clip", "view", "viewpoint", "scale", "fps", "environment", "timeline", "thermalSpanK", "mirror", "theme", "transport", "transportAgeS", "transportStrokeMm", "transportTrailS"))
_BUNDLED = ("🗂️catalog/🖨️print-profiles/🔣️.json",)
_KEY_LENGTH = 32
_PUBLISH_ATTEMPTS = 8
_PUBLISH_PAUSE = 0.05
_PRUNE_AGE_S = 24 * 3600
_WAIT_POLL_S = 0.1
_WAIT_REPORT_S = 1.0
_CONTENDED = frozenset({errno.EACCES, errno.EAGAIN, errno.EWOULDBLOCK, errno.EDEADLK, getattr(errno, "EDEADLOCK", errno.EDEADLK)})
_STROKE_UNIT = 0.01
_MIN_STROKE = 1.0
_LINE_SUFFIX = "-line"
_GROUP_MATRIX = re.compile(r'<g id="staffage-cast" transform="matrix\(([^)]*)\)')
_GROUP_TRANSLATE = re.compile(r'<g id="staffage-cast" transform="translate\(([^)]*)\)')
_SHAPE = re.compile(r'<g id="shape-([^"]+)"[^>]*>\s*(?:<title>[^<]*</title>\s*)?<path d="([^"]+)"')
_COMMAND = re.compile(r"([MLHVZmlhvz])([^MLHVZmlhvz]*)")
_NUMBER = re.compile(r"-?\d*\.?\d+(?:e-?\d+)?")
_PATH = re.compile(r"<path\b([^>]*?)/?>")
_ATTRIBUTE = re.compile(r'([\w:-]+)="([^"]*)"')
_PATH_COMMAND = re.compile(r"([A-Za-z])([^A-Za-z]*)")
_PATH_NUMBER = re.compile(r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?")
_CLOSE_TOLERANCE = 1e-4
_FRAME_CACHE = 256
_FRAMES: OrderedDict[tuple[str, int, int], "_Frame"] = OrderedDict()
_SLOT_COUNTS: dict[Path, int] = {}
_INDEXES: dict[Path, dict] = {}
_CHECKED: set[Path] = set()
_MANIFESTS: dict[Path, tuple[int, dict]] = {}


def _cache_root() -> Path:
    """📂 Resolve the staffage cache beside the other teaching caches."""
    base = os.environ.get("SEMIO_TEACHING_CACHE_ROOT")
    return (_WORKSPACE / base if base else _WORKSPACE / ".🧬semio/🎓️teaching/⚡️cache") / "staffage"


def _repo() -> str:
    """🧰 Resolve the host's provisioned command without assuming a platform-specific binary."""
    configured = os.environ.get("SEMIO_REPO_BINARY") or shutil.which("repo")
    if configured:
        return configured
    marker = _WORKSPACE / ".🧬semio/🦑️repo/native-repo-path"
    if marker.is_file():
        native = Path(marker.read_text(encoding="utf-8").strip())
        if native.is_file():
            return str(native)
    raise FileNotFoundError("The repository command is unavailable; start this render through repo")


def _manifest() -> dict:
    """📜 Read the cast manifest again whenever a migration edits it."""
    path = _MANIFEST
    stamp = path.stat().st_mtime_ns
    cached = _MANIFESTS.get(path)
    if cached is None or cached[0] != stamp:
        document = json.loads(path.read_text(encoding="utf-8"))
        if document.get("version") != 1:
            raise ValueError(f"Unsupported cast manifest version {document.get('version')!r}")
        cached = _MANIFESTS[path] = (stamp, document)
    return cached[1]


def _entry(cast: str) -> dict:
    """🔎 Look up one cast entry and name the known casts when it is missing."""
    casts = _manifest()["casts"]
    if cast not in casts:
        raise KeyError(f"Unknown cast {cast!r}; add it to {_MANIFEST}; known casts: {', '.join(sorted(casts))}")
    return casts[cast]


@lru_cache(maxsize=None)
def _source_digest(root: Path = _SOURCES) -> str:
    """🔐 Hash every exporter and schema source file and the bundled print profiles once per provenance checkpoint, in path order."""
    digest = hashlib.sha256()
    files = sorted([file for folder in ("🔨️modules", "🧬️schema") for file in (root / folder).rglob("*") if file.is_file() and "node_modules" not in file.parts]
                   + [root / name for name in _BUNDLED if (root / name).is_file()], key=lambda file: file.relative_to(root).as_posix())
    for file in files:
        digest.update(file.relative_to(root).as_posix().encode("utf-8") + b"\0" + _lf(file.read_bytes()) + b"\0")
    return digest.hexdigest()


def _lf(data: bytes) -> bytes:
    """↵ Turn CRLF into LF, so a checkout with `core.autocrlf` (Windows) and one without (Linux, macOS, devcontainer) address the same cache entry."""
    return data.replace(b"\r\n", b"\n")


@lru_cache(maxsize=None)
def _catalog_bytes(slug: str, root: Path = _SOURCES) -> bytes:
    """🗂️ Read the catalog file of an asset, with LF line endings."""
    matches = sorted((root / "🗂️catalog").glob(f"*/{slug}/{_INDEX}"), key=lambda file: file.as_posix())
    if not matches:
        raise FileNotFoundError(f"Asset {slug!r} is not in the staffage catalog {root / '🗂️catalog'}")
    return _lf(matches[0].read_bytes())


@lru_cache(maxsize=None)
def _presets(root: Path = _SOURCES) -> dict[str, dict]:
    """🌤️ Read the environment presets by id once per provenance checkpoint."""
    return {preset["id"]: preset for file in sorted((root / "🗂️catalog").glob(f"*environments*/{_INDEX}")) for preset in json.loads(file.read_text(encoding="utf-8"))["presets"]}


def _number(value: float) -> str:
    """🔢 Print a number the way ECMAScript does: integral values without a fraction, shortest round-trip digits, `e-7` instead of `e-07`."""
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise TypeError(f"not a number: {value!r}")
    if float(value).is_integer() and abs(value) < 1e21:
        return str(int(value))
    text = repr(float(value))
    if "e" in text:
        mantissa, exponent = text.split("e")
        return f"{mantissa}e{'+' if int(exponent) > 0 else '-'}{abs(int(exponent))}"
    return text


def canonical_json(value) -> str:
    """🧾 Serialise JSON canonically: sorted keys, no whitespace, non-ASCII kept; byte-identical to `canonicalJson` of the TypeScript exporter."""
    if isinstance(value, dict):
        return "{" + ",".join(f"{json.dumps(key, ensure_ascii=False)}:{canonical_json(value[key])}" for key in sorted(value)) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical_json(item) for item in value) + "]"
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        return _number(value)
    return json.dumps(value, ensure_ascii=False)


def revision_of(data: bytes) -> str:
    """🔖 Name a revision: `sha256:` and the hex digest of the bytes with CRLF turned into LF."""
    return "sha256:" + hashlib.sha256(_lf(data)).hexdigest()


def asset_revision(slug: str, root: Path = _SOURCES) -> str:
    """🔖 Return the content hash of an asset's catalog file."""
    return revision_of(_catalog_bytes(slug, root))


def preset_revision(preset: dict) -> str:
    """🔖 Return the content hash of an environment preset in canonical JSON."""
    return "sha256:" + hashlib.sha256(canonical_json(preset).encode("utf-8")).hexdigest()


def _preset_revisions(entry: dict, root: Path = _SOURCES) -> dict[str, str]:
    """🌤 Return the revision of every environment preset a cast names, failing on an unknown one."""
    presets = _presets(root)
    slugs = sorted({*([entry["environment"]] if "environment" in entry else []), *(key["environment"] for key in entry.get("timeline", []))})
    unknown = [slug for slug in slugs if slug not in presets]
    if unknown:
        raise KeyError(f"unknown environment preset {', '.join(unknown)}; presets: {', '.join(sorted(presets))}")
    return {slug: preset_revision(presets[slug]) for slug in slugs}


def entry_issues(entry, path: str = "cast") -> list[str]:
    """✅ List every violation of the cast schema in one manifest entry, with the messages of the TypeScript twin; empty when the entry is valid."""
    if not isinstance(entry, dict):
        return [f"{path}: must be an object"]
    issues: list[str] = []
    slug = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
    is_slug = lambda value: isinstance(value, str) and len(value) <= 64 and slug.match(value) is not None
    positive = lambda value: isinstance(value, (int, float)) and not isinstance(value, bool) and value > 0
    for key in entry:
        if key not in _CAST_KEYS:
            issues.append(f"{path}: unknown property {key}")
    for key in ("asset", "pose", "clip", "environment"):
        if key in entry and not is_slug(entry[key]):
            issues.append(f"{path}/{key}: must be a slug")
    if "asset" not in entry:
        issues.append(f"{path}: asset is required")
    if "theme" not in entry:
        issues.append(f"{path}: theme is required")
    elif entry["theme"] not in _THEMES:
        issues.append(f"{path}/theme: must be one of {', '.join(_THEMES)}")
    layers = entry.get("transport")
    if "transport" in entry and not (isinstance(layers, list) and all(layer in _TRANSPORT_LAYERS for layer in layers) and len(set(layers)) == len(layers)):
        issues.append(f"{path}/transport: must be unique layers of {', '.join(_TRANSPORT_LAYERS)}")
    for key in ("transportAgeS", "transportStrokeMm", "transportTrailS"):
        if key in entry and not positive(entry[key]):
            issues.append(f"{path}/{key}: must be a positive number")
        if key in entry and "transport" not in entry:
            issues.append(f"{path}: {key} needs transport layers")
    if "transportAgeS" in entry and "clip" not in entry:
        issues.append(f"{path}: transportAgeS needs a clip")
    if "transport" in entry and "transportAgeS" not in entry:
        issues.append(f"{path}: transport layers need transportAgeS, the explicit start of the field")
    if "transport" in entry and entry.get("view") != "thermal":
        issues.append(f"{path}: transport layers exist for the thermal view only")
    if entry.get("view") not in _VIEWS:
        issues.append(f"{path}/view: must be one of {', '.join(_VIEWS)}")
    if entry.get("viewpoint") not in _VIEWPOINTS:
        issues.append(f"{path}/viewpoint: is required and must be one of {', '.join(_VIEWPOINTS)}")
    if entry.get("mirror") is True and entry.get("viewpoint") != "elevation":
        issues.append(f"{path}: mirror needs viewpoint elevation, a mirrored front or plan would fake a view")
    if entry.get("scale") not in _SCALES or isinstance(entry.get("scale"), bool):
        issues.append(f"{path}/scale: is required and must be one of {', '.join(map(str, _SCALES))}")
    fps = entry.get("fps")
    if "fps" in entry and not (isinstance(fps, int) and not isinstance(fps, bool) and 1 <= fps <= 120):
        issues.append(f"{path}/fps: must be an integer in 1..120")
    if "thermalSpanK" in entry and not positive(entry["thermalSpanK"]):
        issues.append(f"{path}/thermalSpanK: must be a positive number of kelvin")
    if "thermalSpanK" in entry and entry.get("view") != "thermal":
        issues.append(f"{path}: thermalSpanK exists for the thermal view only")
    if "mirror" in entry and not isinstance(entry["mirror"], bool):
        issues.append(f"{path}/mirror: must be a boolean")
    if ("pose" in entry) == ("clip" in entry):
        issues.append(f"{path}: give exactly one of pose and clip")
    if ("environment" in entry or "timeline" in entry) and entry.get("view") != "thermal":
        issues.append(f"{path}: environment and timeline exist for the thermal view only")
    if "environment" in entry and "timeline" in entry:
        issues.append(f"{path}: environment and timeline exclude each other")
    if "fps" in entry and not ("clip" in entry or "timeline" in entry):
        issues.append(f"{path}/fps: needs a clip or a timeline")
    if "fps" not in entry and ("clip" in entry or "timeline" in entry):
        issues.append(f"{path}/fps: is required for a clip or a timeline")
    if entry.get("view") == "thermal" and not ("environment" in entry or "timeline" in entry):
        issues.append(f"{path}: the thermal view needs an environment or a timeline")
    if "timeline" in entry:
        timeline = entry["timeline"]
        if not isinstance(timeline, list) or not timeline:
            issues.append(f"{path}/timeline: must be a non-empty array")
        else:
            for position, key in enumerate(timeline):
                at = f"{path}/timeline/{position}"
                if not isinstance(key, dict):
                    issues.append(f"{at}: must be an object")
                    continue
                issues.extend(f"{at}: unknown property {name}" for name in key if name not in ("t", "environment", "ease"))
                if not (isinstance(key.get("t"), (int, float)) and not isinstance(key.get("t"), bool) and key["t"] >= 0):
                    issues.append(f"{at}/t: must be a number >= 0")
                if not is_slug(key.get("environment")):
                    issues.append(f"{at}/environment: must be a slug")
                if key.get("ease") not in ("step", "linear"):
                    issues.append(f"{at}/ease: is required and must be step or linear")
    return issues


def cast_key(entry: dict, root: Path = _SOURCES) -> str:
    """🔑 Derive the content address of a cast from its validated entry (every view, scale, state, clock and scenario field), the revision of its asset, the revision of every environment preset it names and the exporter sources."""
    issues = entry_issues(entry, f"cast entry for {entry.get('asset')!r}")
    if issues:
        raise ValueError("invalid cast entry\n" + "\n".join(issues))
    digest = hashlib.sha256()
    digest.update(json.dumps(entry, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8") + b"\0")
    digest.update(asset_revision(entry["asset"], root).encode("ascii") + b"\0")
    digest.update(json.dumps(_preset_revisions(entry, root), sort_keys=True, separators=(",", ":")).encode("ascii") + b"\0")
    digest.update(_source_digest(root).encode("ascii"))
    return digest.hexdigest()[:_KEY_LENGTH]


def _directory(cast: str) -> Path:
    """📁 Name the cache folder of a cast."""
    return _cache_root() / cast_key(_entry(cast)) / cast


class _Tree:
    """🌳 One exporter process and everything it starts: a Windows job object that dies with its handle, a POSIX session that is signalled as a group."""

    def __init__(self, command: list[str]) -> None:
        self.job = None
        if sys.platform == "win32":
            self.job = self._job()
            flags = subprocess.CREATE_NEW_PROCESS_GROUP | (0x4 if self.job is not None else 0)
            self.process = subprocess.Popen(command, cwd=_WORKSPACE, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace", creationflags=flags)
            try:
                self._assign()
            finally:
                if flags & 0x4:
                    self._resume()
        else:
            self.process = subprocess.Popen(command, cwd=_WORKSPACE, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace", start_new_session=True)

    @staticmethod
    def _job():
        """🪝 Create a job object that kills every member when its last handle closes."""
        import ctypes
        from ctypes import wintypes

        class Basic(ctypes.Structure):
            _fields_ = [("PerProcessUserTimeLimit", ctypes.c_int64), ("PerJobUserTimeLimit", ctypes.c_int64), ("LimitFlags", wintypes.DWORD), ("MinimumWorkingSetSize", ctypes.c_size_t), ("MaximumWorkingSetSize", ctypes.c_size_t), ("ActiveProcessLimit", wintypes.DWORD), ("Affinity", ctypes.c_size_t), ("PriorityClass", wintypes.DWORD), ("SchedulingClass", wintypes.DWORD)]

        class Extended(ctypes.Structure):
            _fields_ = [("Basic", Basic), ("IoInfo", ctypes.c_uint64 * 6), ("ProcessMemoryLimit", ctypes.c_size_t), ("JobMemoryLimit", ctypes.c_size_t), ("PeakProcessMemoryUsed", ctypes.c_size_t), ("PeakJobMemoryUsed", ctypes.c_size_t)]

        kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        kernel.CreateJobObjectW.restype = wintypes.HANDLE
        kernel.CreateJobObjectW.argtypes = [ctypes.c_void_p, wintypes.LPCWSTR]
        kernel.SetInformationJobObject.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD]
        kernel.AssignProcessToJobObject.argtypes = [wintypes.HANDLE, wintypes.HANDLE]
        kernel.TerminateJobObject.argtypes = [wintypes.HANDLE, wintypes.UINT]
        kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        handle = kernel.CreateJobObjectW(None, None)
        limits = Extended()
        limits.Basic.LimitFlags = 0x2000
        if not handle or not kernel.SetInformationJobObject(handle, 9, ctypes.byref(limits), ctypes.sizeof(limits)):
            return None
        return kernel, handle

    def _assign(self) -> None:
        """🔗 Put the process into the job; without a job the tree is killed with `taskkill`."""
        if self.job is not None:
            kernel, handle = self.job
            if not kernel.AssignProcessToJobObject(handle, int(self.process._handle)):
                self.close()

    def _resume(self) -> None:
        """▶️ Let the process, created suspended so that it cannot start a child before it is in the job, run."""
        import ctypes

        ctypes.WinDLL("ntdll").NtResumeProcess(ctypes.c_void_p(int(self.process._handle)))

    def stop(self, grace: float = 3.0) -> None:
        """🛑 Ask the tree to stop, then kill every member that is still running after `grace` seconds."""
        try:
            if sys.platform == "win32":
                self.process.send_signal(signal.CTRL_BREAK_EVENT)
            else:
                os.killpg(self.process.pid, signal.SIGTERM)
            self.process.wait(timeout=grace)
            self._drain(time.monotonic() + grace)
        except (OSError, subprocess.TimeoutExpired, ValueError):
            pass
        self.kill()

    def _drain(self, deadline: float) -> None:
        """⏳ On POSIX wait until no member of the session is left, so a child that outlives the shim gets its grace to remove its temporary folder."""
        while sys.platform != "win32" and time.monotonic() < deadline:
            try:
                os.killpg(self.process.pid, 0)
            except OSError:
                return
            time.sleep(0.05)

    def kill(self) -> None:
        """💥 Kill every member of the tree."""
        if sys.platform == "win32":
            if self.job is not None:
                self.job[0].TerminateJobObject(self.job[1], 1)
            else:
                subprocess.run(["taskkill", "/PID", str(self.process.pid), "/T", "/F"], capture_output=True, check=False)
        else:
            try:
                os.killpg(self.process.pid, signal.SIGKILL)
            except OSError:
                pass
        try:
            self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            pass

    def close(self) -> None:
        """🔒 Release the job handle, which kills any member left in it."""
        if self.job is not None:
            self.job[0].CloseHandle(self.job[1])
            self.job = None


@contextlib.contextmanager
def _terminable():
    """📴 Turn SIGTERM and SIGHUP (POSIX) or SIGBREAK (Windows) of the render into a `SystemExit` while the exporter runs, so a killed or closed terminal stops the exporter tree at once instead of leaving it to the parent watchdog; the previous handlers return afterwards. Only the main thread may install handlers."""
    names = ("SIGTERM", "SIGBREAK") if sys.platform == "win32" else ("SIGTERM", "SIGHUP")
    previous: dict[int, object] = {}

    def leave(number, frame):
        raise SystemExit(128 + number)

    if threading.current_thread() is threading.main_thread():
        for name in names:
            number = getattr(signal, name)
            try:
                previous[number] = signal.signal(number, leave)
            except (OSError, ValueError):
                pass
    try:
        yield
    finally:
        for number, handler in previous.items():
            signal.signal(number, handler)


def _run(command: list[str], progress: Callable[[str], None] | None = None) -> None:
    """🔧 Run the exporter; every stderr line goes to stderr with a `[staffage]` prefix and to `progress`, and a cancelled render (any `BaseException`, `KeyboardInterrupt` and `SystemExit` included) stops the whole process tree."""
    with _terminable():
        _supervise(command, progress)


def _supervise(command: list[str], progress: Callable[[str], None] | None) -> None:
    """👁️ Start the exporter tree and wait for it."""
    tree = _Tree(command)
    process = tree.process
    tail: list[str] = []
    failure: list[BaseException] = []

    def pump() -> None:
        for line in process.stderr:
            tail[:] = (tail + [line.rstrip()])[-12:]
            sys.stderr.write(f"[staffage] {line}")
            sys.stderr.flush()
            if progress is not None and not failure:
                try:
                    progress(line.rstrip())
                except BaseException as error:
                    failure.append(error)

    reader = threading.Thread(target=pump, daemon=True)
    reader.start()
    try:
        while True:
            if failure:
                raise failure[0]
            try:
                code = process.wait(timeout=0.1)
                break
            except subprocess.TimeoutExpired:
                continue
        reader.join()
        if failure:
            raise failure[0]
    except BaseException:
        tree.stop()
        reader.join(timeout=5)
        raise
    finally:
        tree.close()
        process.stderr.close()
    if code != 0:
        raise RuntimeError("Staffage export failed:\n" + "\n".join(tail))


def _publish(source: Path, target: Path) -> None:
    """📮 Rename a rendered cast into place, trying again while Windows reports a transient lock (a virus scanner or indexer holds a just written file), and keeping the cast of another renderer that won the race."""
    for attempt in range(1, _PUBLISH_ATTEMPTS + 1):
        try:
            source.rename(target)
            return
        except OSError:
            if (target / _INDEX).is_file():
                return
            if attempt == _PUBLISH_ATTEMPTS:
                raise
            time.sleep(_PUBLISH_PAUSE * attempt)


def _prune(root: Path, live: set[str], now: float | None = None) -> None:
    """🧹 Remove the cache folders of keys that no cast of the manifest has any more, and abandoned render folders, once a day has passed since they were last written, so the cache does not grow with every source edit."""
    now = time.time() if now is None else now
    try:
        folders = [folder for folder in root.iterdir() if folder.name not in live]
    except OSError:
        return
    for folder in folders:
        try:
            if folder.is_dir() and now - folder.stat().st_mtime > _PRUNE_AGE_S:
                shutil.rmtree(folder, ignore_errors=True)
        except OSError:
            continue


class _Lock:
    """🔏 An exclusive operating-system lock on one file (`msvcrt.locking` on Windows, `fcntl.flock` on POSIX): the system drops it when the owner exits or dies, so a crashed renderer never leaves a stale lock. A file system without locks degrades to no lock."""

    def __init__(self, path: Path) -> None:
        self.path = path
        self.handle = None

    def acquire(self) -> bool:
        """🚦 Take the lock without waiting; false while another process or thread holds it."""
        self.path.parent.mkdir(parents=True, exist_ok=True)
        handle = open(self.path, "a+b")
        try:
            if sys.platform == "win32":
                import msvcrt

                handle.seek(0)
                msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl

                fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError as error:
            handle.close()
            return error.errno not in _CONTENDED
        self.handle = handle
        return True

    def release(self) -> None:
        """🔓 Give the lock back."""
        handle, self.handle = self.handle, None
        if handle is None:
            return
        try:
            if sys.platform == "win32":
                import msvcrt

                handle.seek(0)
                msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
        except OSError:
            pass
        handle.close()


def _lock(cast: str) -> _Lock:
    """🏷️ Name the lock of one cast beside its cache folder, so renderers of the same source and entry share it."""
    return _Lock(_directory(cast).parent / f".lock-{cast}")


def _wait(pending: list[str], progress: Callable[[str], None] | None, since: list[float]) -> None:
    """⏱️ Sleep one poll while other renderers export the `pending` casts and tell `progress` (which may raise to cancel) about it once per second."""
    now = time.monotonic()
    if not since or now - since[0] >= _WAIT_REPORT_S:
        since[:] = [now]
        if progress is not None:
            progress(f"[wait] {','.join(pending)}")
    time.sleep(_WAIT_POLL_S)


def _render(casts: list[str], folders: dict[str, Path], progress: Callable[[str], None] | None) -> None:
    """🎬 Export `casts` in one exporter call and publish each into the cache."""
    root = _cache_root()
    temporary = root / f".tmp-{uuid.uuid4().hex[:12]}"
    temporary.mkdir(parents=True)
    try:
        batch = temporary / "🔣️.json"
        batch.write_text(json.dumps({"version": 1, "casts": {cast: _entry(cast) for cast in casts}}, ensure_ascii=False), encoding="utf-8")
        _run([_repo(), "tools", "exec", "--light", "bun", "--", str(_EXPORTER), "--manifest", str(batch), "--ids", ",".join(casts), "--out", str(temporary / "out"), "--parent", str(os.getpid())], progress)
        for cast in casts:
            folders[cast].parent.mkdir(parents=True, exist_ok=True)
            _publish(temporary / "out" / cast, folders[cast])
    finally:
        shutil.rmtree(temporary, ignore_errors=True)


def ensure_casts(ids: list[str] | tuple[str, ...] | None = None, progress: Callable[[str], None] | None = None) -> dict[str, Path]:
    """📦 Render every missing cast of `ids` (default: the whole manifest) in one exporter call and return the cache folder of each; a cast that another process is already exporting is waited for instead of exported twice. `progress` receives each exporter stderr line (`[pid]`, `[cast i/n] id`, `[done/total]`) and `[wait] ids` while waiting, and may raise to cancel."""
    wanted = list(dict.fromkeys(ids if ids is not None else _manifest()["casts"]))
    folders = {cast: _directory(cast) for cast in wanted}
    pending = [cast for cast, folder in folders.items() if not (folder / _INDEX).is_file()]
    rendered = bool(pending)
    since: list[float] = []
    while pending:
        held: dict[str, _Lock] = {}
        try:
            for cast in sorted(pending):
                lock = _lock(cast)
                if not lock.acquire():
                    continue
                if (folders[cast] / _INDEX).is_file():
                    lock.release()
                else:
                    held[cast] = lock
            if held:
                _render([cast for cast in pending if cast in held], folders, progress)
        finally:
            for lock in held.values():
                lock.release()
        pending = [cast for cast in pending if cast not in held and not (folders[cast] / _INDEX).is_file()]
        if pending and not held:
            _wait(pending, progress, since)
    if rendered:
        _prune(_cache_root(), {cast_key(entry) for entry in _manifest()["casts"].values()})
    for cast, folder in folders.items():
        drawn, named = _index(folder)["viewpoint"], _manifest()["casts"][cast]["viewpoint"]
        if drawn != named:
            raise ValueError(f"cast {cast!r} is named {named!r} in the manifest but its rendered index {folder / _INDEX} drew {drawn!r}")
        if folder not in _CHECKED:
            _provenance(cast, _index(folder))
            _CHECKED.add(folder)
    return folders


def casts_in(sources, manifest: dict | None = None, root: Path = _SOURCES) -> dict[str, str]:
    """🔗 Map every cast id a set of Python sources names (string literals, and every id behind a literal ending in `-`) to its content key, so a clip key changes with the catalog asset, the exporter and the viewpoint of each cast it draws."""
    for cached in (_source_digest, _catalog_bytes, _presets):
        cached.cache_clear()
    casts = (manifest if manifest is not None else _manifest())["casts"]
    named: set[str] = set()
    for source in sources:
        if Path(source).suffix != ".py" or not Path(source).is_file():
            continue
        try:
            tree = ast.parse(Path(source).read_text(encoding="utf-8"))
        except SyntaxError:
            continue
        for node in ast.walk(tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                if node.value in casts:
                    named.add(node.value)
                elif node.value.endswith("-"):
                    named.update(name for name in casts if name.startswith(node.value))
    return {name: cast_key(casts[name], root) for name in sorted(named)}


def expected_provenance(cast: str, root: Path = _SOURCES) -> dict:
    """🔖 Derive what the provenance of a rendered cast must say from its manifest entry, the catalog asset and the environment presets: every dimension except the clock, which the exporter measures."""
    entry = _entry(cast)
    found = {
        "version": 1,
        "asset": {"id": entry["asset"], "revision": asset_revision(entry["asset"], root)},
        "view": {"style": entry["view"], "viewpoint": entry["viewpoint"], "theme": entry["theme"], "mirror": entry.get("mirror", False)},
        "scale": {"denominator": entry["scale"]},
        "state": {"kind": "clip", "id": entry["clip"]} if "clip" in entry else {"kind": "pose", "id": entry["pose"]},
    }
    if entry["view"] == "thermal":
        revisions = _preset_revisions(entry, root)
        scenario = {"kind": "thermal", "spanK": entry.get("thermalSpanK"), "transport": entry.get("transport", [])}
        if "timeline" in entry:
            scenario["timeline"] = [{"t": key["t"], "id": key["environment"], "ease": key["ease"], "revision": revisions[key["environment"]]} for key in sorted(entry["timeline"], key=lambda key: key["t"])]
        else:
            scenario["environment"] = {"id": entry["environment"], "revision": revisions[entry["environment"]]}
    else:
        scenario = {"kind": "structural" if entry["view"] in ("forces", "moments") else "geometry"}
    return {**found, "scenario": scenario}


def _provenance(cast: str, index: dict, root: Path = _SOURCES) -> dict:
    """🔖 Check the provenance of a rendered cast against its manifest entry, the catalog asset, the environment presets and its own index, and return it; a missing or differing dimension raises."""
    found = index.get("provenance")
    if not isinstance(found, dict):
        raise ValueError(f"cast {cast!r}: its rendered index has no provenance record; export it again")
    expected = expected_provenance(cast, root)
    for section, named in expected.items():
        drawn = found.get(section)
        if section == "asset" and isinstance(drawn, dict):
            drawn = {key: drawn.get(key) for key in named}
        if section == "scenario" and isinstance(drawn, dict) and named["kind"] == "structural":
            drawn = {"kind": drawn.get("kind")}
        if drawn != named:
            raise ValueError(f"cast {cast!r}: provenance {section} is {drawn!r} but the manifest and the catalog say {named!r}; export it again")
    clock = found.get("clock")
    wanted = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "frames": len(index.get("frames", []))}
    if not isinstance(clock, dict) or any(clock.get(key) != value for key, value in wanted.items()) or "offsetS" not in clock:
        raise ValueError(f"cast {cast!r}: provenance clock is {clock!r} but its index says {wanted!r}; export it again")
    return found


_DRAWN: list[dict] = []
_RECORDING = [True]


@contextlib.contextmanager
def unrecorded():
    """🙈 Draw casts for measuring only: inside this block they are not listed by `drawn_casts`."""
    _RECORDING.append(False)
    try:
        yield
    finally:
        _RECORDING.pop()


def staffage_provenance(cast: str, rate: float | None = None) -> dict:
    """🔖 Return the checked provenance record of a cast for a scene: asset id and revision, view, authored scale, pose or clip, clock and scenario; `rate` adds the scene's playback rate to the clock."""
    record = json.loads(json.dumps(_index(ensure_casts([cast])[cast])["provenance"]))
    if rate is not None:
        record["clock"]["rate"] = rate
    return record


def drawn_casts() -> list[dict]:
    """📋 Return the provenance of every cast drawn in this process since `clear_drawn_casts`, in drawing order, each with its cast id and key."""
    return json.loads(json.dumps(_DRAWN))


def clear_drawn_casts() -> None:
    """🧹 Forget the casts drawn so far, for example at the start of a scene."""
    _DRAWN.clear()


def _drawn(cast: str, rate: float | None) -> dict:
    """📌 Record a drawn cast and return its provenance."""
    record = staffage_provenance(cast, rate)
    if _RECORDING[-1]:
        _DRAWN.append({"cast": cast, "key": ensure_casts([cast])[cast].parent.name, **record})
    return record


def _index(folder: Path) -> dict:
    """🧾 Read the index of a rendered cast once per process; `viewpoint` says which view was drawn (`elevation`, `front`, `plan` or `axonometric`)."""
    if folder not in _INDEXES:
        index = json.loads((folder / _INDEX).read_text(encoding="utf-8"))
        if index.get("viewpoint") not in _VIEWPOINTS:
            raise ValueError(f"cast index {folder / _INDEX} names the missing or unknown viewpoint {index.get('viewpoint')!r}")
        _INDEXES[folder] = index
    return _INDEXES[folder]


class _Shape(NamedTuple):
    """🔷 The style of one drawn path, with its colours as hex strings or `None` and its stroke width in paper millimetres."""

    fill: str | None
    fill_opacity: float
    stroke: str | None
    stroke_opacity: float
    stroke_mm: float


class _Frame(NamedTuple):
    """🖼️ One parsed frame: every path as 4n Bezier points in world metres (y up, origin on the feet), concatenated, with the span of each path."""

    points: np.ndarray
    spans: tuple[tuple[int, int], ...]
    shapes: tuple[_Shape, ...]


def _colour(value: str | None, default: str | None) -> str | None:
    """🎨 Read an SVG paint as `#RRGGBB`, or `None` for `none`. A `staffage-pattern` paint is the print texture laid over the base fill the exporter writes just before it, so it reads as `none` and the base colour stays."""
    if value is None:
        return default
    value = value.strip()
    if value == "none" or re.fullmatch(r"url\(#staffage-pattern-[a-z]+-[0-9a-fA-F]{6}\)", value):
        return None
    if re.fullmatch(r"#[0-9a-fA-F]{6}", value):
        return value.upper()
    if re.fullmatch(r"#[0-9a-fA-F]{3}", value):
        return "#" + "".join(2 * digit for digit in value[1:]).upper()
    raise ValueError(f"unsupported SVG paint {value!r}")


def _outline(data: str) -> np.ndarray:
    """✏️ Turn path data (`M`, `L`, `H`, `V`, `C`, `Z`, absolute, a run of one command written once) into Manim's 4-point cubic segments, with straight lines as thirds and a closing line unless already closed."""
    segments: list[tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]] = []
    pen = start = np.zeros(2)

    def line(end: np.ndarray) -> None:
        segments.append((pen, (2 * pen + end) / 3, (pen + 2 * end) / 3, end))

    for command, numbers in _PATH_COMMAND.findall(data):
        values = [float(value) for value in _PATH_NUMBER.findall(numbers)]
        pairs = [np.array(values[i:i + 2]) for i in range(0, len(values) - 1, 2)]
        if command == "M":
            pen = start = pairs[0]
            pairs = pairs[1:]
            command = "L"
        if command == "L":
            for end in pairs:
                line(end)
                pen = end
        elif command == "H":
            for x in values:
                end = np.array([x, pen[1]])
                line(end)
                pen = end
        elif command == "V":
            for y in values:
                end = np.array([pen[0], y])
                line(end)
                pen = end
        elif command == "C":
            for i in range(0, len(pairs) - 2, 3):
                segments.append((pen, pairs[i], pairs[i + 1], pairs[i + 2]))
                pen = pairs[i + 2]
        elif command == "Z":
            if np.linalg.norm(pen - start) > _CLOSE_TOLERANCE:
                line(start)
            pen = start
        else:
            raise ValueError(f"unsupported path command {command!r}")
    return np.array(segments, dtype=float).reshape(-1, 2) if segments else np.empty((0, 2))


def _parse_frame(text: str, index: dict) -> _Frame:
    """🧩 Parse the paths of one cast frame straight into world-metre Bezier points and styles, without an SVG library."""
    matrix = np.array([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    found = _GROUP_MATRIX.search(text)
    if found:
        matrix = np.array([float(value) for value in found.group(1).replace(",", " ").split()])
    elif (shift := _GROUP_TRANSLATE.search(text)):
        matrix[4:] = [float(value) for value in shift.group(1).replace(",", " ").split()]
    origin_x, origin_y = index["originMm"]
    per_metre = index["mmPerMetre"]
    blocks: list[np.ndarray] = []
    spans: list[tuple[int, int]] = []
    shapes: list[_Shape] = []
    count = 0
    for tag in _PATH.findall(text):
        attributes = dict(_ATTRIBUTE.findall(tag))
        points = _outline(attributes.get("d", ""))
        if not len(points):
            continue
        x = matrix[0] * points[:, 0] + matrix[2] * points[:, 1] + matrix[4]
        y = matrix[1] * points[:, 0] + matrix[3] * points[:, 1] + matrix[5]
        blocks.append(np.column_stack(((x - origin_x) / per_metre, (origin_y - y) / per_metre)))
        spans.append((count, count + len(points)))
        count += len(points)
        opacity = float(attributes.get("opacity", 1.0))
        stroke = _colour(attributes.get("stroke"), None)
        shapes.append(_Shape(_colour(attributes.get("fill"), "#000000"), float(attributes.get("fill-opacity", 1.0)) * opacity, stroke,
                             float(attributes.get("stroke-opacity", 1.0)) * opacity, float(attributes.get("stroke-width", 1.0)) if stroke else 0.0))
    return _Frame(np.concatenate(blocks) if blocks else np.empty((0, 2)), tuple(spans), tuple(shapes))


def _frame(folder: Path, frame: int) -> _Frame:
    """🗃️ Return a parsed frame from a bounded least-recently-used cache, so a long clip never holds more than a few hundred frames."""
    index = _index(folder)
    path = folder / index["frames"][frame]
    stat = path.stat()
    key = (str(path), stat.st_mtime_ns, stat.st_size)
    cached = _FRAMES.get(key)
    if cached is not None:
        _FRAMES.move_to_end(key)
        return cached
    for attempt in range(3):
        try:
            parsed = _parse_frame(path.read_text(encoding="utf-8"), index)
            break
        except OSError:
            if attempt == 2:
                raise
    _FRAMES[key] = parsed
    while len(_FRAMES) > _FRAME_CACHE:
        _FRAMES.popitem(last=False)
    return parsed


def _slot_count(folder: Path) -> int:
    """🔢 Count the most paths any frame of a cast draws, reading each distinct frame file once."""
    if folder not in _SLOT_COUNTS:
        names = dict.fromkeys(_index(folder)["frames"])
        _SLOT_COUNTS[folder] = max(((folder / name).read_bytes().count(b"<path") for name in names), default=0)
    return _SLOT_COUNTS[folder]


def _style(shape: _Shape, factor: float, color: str | None, opacity: float) -> tuple[str | None, float, str | None, float, float]:
    """🖌️ Resolve the Manim fill, stroke and width of a path; the stroke width follows the physical scale and never drops below one."""
    width = max(shape.stroke_mm * factor / _STROKE_UNIT, _MIN_STROKE) if shape.stroke_mm > 0 else 0.0
    return shape.fill, shape.fill_opacity * opacity, color or shape.stroke, shape.stroke_opacity * opacity, width


class _Path(VMobject):
    """〰️ One drawn path; a path the drawing leaves unfilled (an open stroke) keeps a fill opacity of zero when the group is faded in with `set_opacity`, so open strokes never gain a fill."""

    unfilled = False

    def set_fill(self, color=None, opacity=None, family=True):
        """🫥 Set the fill, holding the opacity at zero for an unfilled path."""
        return super().set_fill(color, 0.0 if self.unfilled and opacity is not None else opacity, family)


def _apply(member: _Path, style: tuple[str | None, float, str | None, float, float]) -> None:
    """🪄 Give a path its fill and stroke."""
    fill, fill_opacity, stroke, stroke_opacity, width = style
    member.unfilled = fill is None
    if fill is not None:
        member.set_fill(color=fill, opacity=fill_opacity)
    else:
        member.set_fill(opacity=0.0)
    if stroke is not None and width > 0:
        member.set_stroke(color=stroke, width=width, opacity=stroke_opacity)
    else:
        member.set_stroke(width=0.0, opacity=0.0)


def _flipped(parsed: _Frame, mirror: bool) -> _Frame:
    """🪞 Mirror a parsed frame about the vertical axis of the world origin, exactly the flip the cast exporter applies to a `mirror` cast, so no pixel changes and the asymmetric transport air layer flips with the figure."""
    return parsed._replace(points=parsed.points * np.array([-1.0, 1.0])) if mirror else parsed


def _bounds(index: dict, key: str, mirror: bool) -> dict:
    """🧱 Return the bounds of a cast in world metres, flipped about the world origin when `mirror` is set."""
    box = index[key]
    return {"minX": -box["maxX"], "minY": box["minY"], "maxX": -box["minX"], "maxY": box["maxY"]} if mirror else box


def _place(folder: Path, frame: int, unit: float, anchor, color: str | None, opacity: float, mirror: bool = False) -> VGroup:
    """📐 Map paper millimetres of a frame to world metres times `unit` with the world origin on `anchor`."""
    index = _index(folder)
    parsed = _flipped(_frame(folder, frame), mirror)
    factor = unit / index["mmPerMetre"]
    anchor = np.asarray(anchor, dtype=float)
    recolour = color if color is not None and index["view"].endswith(_LINE_SUFFIX) else None
    members = []
    for (low, high), shape in zip(parsed.spans, parsed.shapes):
        member = _Path()
        member.set_points(np.column_stack((anchor[0] + parsed.points[low:high, 0] * unit, anchor[1] + parsed.points[low:high, 1] * unit, np.full(high - low, anchor[2]))))
        _apply(member, _style(shape, factor, recolour, opacity))
        members.append(member)
    return VGroup(*members)


def staffage_height(cast: str) -> float:
    """📏 Return the height of a cast in metres: the figure, never the air layer of a transport cast."""
    return float(_index(ensure_casts([cast])[cast])["heightM"])


def staffage_width(cast: str) -> float:
    """📏 Return the horizontal paper extent of a cast in metres, the figure without the air layer of a transport cast: the width an elevation or plan figure spans and the depth a front figure spans, read from the exporter's bounds."""
    index = _index(ensure_casts([cast])[cast])
    box = index.get("figureBoundsM", index["boundsM"])
    return float(box["maxX"] - box["minX"])


def staffage_parts(cast: str, prefix: str = "") -> dict[str, np.ndarray]:
    """🔩 Return the filled polygons of the first frame whose shape id starts with `prefix`, as (n, 2) arrays in world metres (y up, origin on the feet), keyed by the shape id without its `shape-` prefix.

    Clips derive attachment points (ray starts, labels) from these instead of hard-coding positions; only straight `M`/`L`/`H`/`V`/`Z` outlines are supported.
    """
    return _parts(ensure_casts([cast])[cast], prefix)


def _parts(folder: Path, prefix: str) -> dict[str, np.ndarray]:
    """🧬 Read the filled polygons of the first frame in `folder` in world metres."""
    index = _index(folder)
    cast = index["cast"]
    text = (folder / index["frames"][0]).read_text(encoding="utf-8")
    matrix = np.array([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    found = _GROUP_MATRIX.search(text)
    if found:
        matrix = np.array([float(value) for value in found.group(1).replace(",", " ").split()])
    elif (shift := _GROUP_TRANSLATE.search(text)):
        matrix[4:] = [float(value) for value in shift.group(1).replace(",", " ").split()]
    origin_x, origin_y = index["originMm"]
    scale = index["mmPerMetre"]
    parts: dict[str, np.ndarray] = {}
    for name, path in _SHAPE.findall(text):
        if not name.startswith(prefix):
            continue
        points: list[tuple[float, float]] = []
        x = y = 0.0
        for command, numbers in _COMMAND.findall(path):
            values = [float(value) for value in _NUMBER.findall(numbers)]
            if command in "MLl":
                for step in zip(values[0::2], values[1::2]):
                    x, y = (x + step[0], y + step[1]) if command == "l" else step
                    points.append((x, y))
                continue
            if command == "H":
                x = values[0]
            elif command == "V":
                y = values[0]
            elif command not in "Zz":
                raise ValueError(f"unsupported path command {command!r} in shape {name!r} of cast {cast!r}")
            if command not in "Zz":
                points.append((x, y))
        placed = np.array([(matrix[0] * px + matrix[2] * py + matrix[4], matrix[1] * px + matrix[3] * py + matrix[5])
                           for px, py in points])
        parts[name] = np.column_stack(((placed[:, 0] - origin_x) / scale, (origin_y - placed[:, 1]) / scale))
    return parts


def staffage(cast: str, *, unit: float, anchor=ORIGIN, color: str | None = None, opacity: float = 1.0,
             window: tuple[float, float, float, float] | None = None, mirror: bool = False) -> VGroup:
    """🎭 Draw the first frame of a cast with the world origin (the feet on the ground) on `anchor` and `unit` Manim units per metre; `mirror` flips it about the vertical axis of the origin, pixel for pixel what a `mirror` cast of the manifest would draw.

    `window` = (min x, min y, max x, max y) in world metres keeps only the drawing elements fully inside it, for example the blind of a façade component without its wall stubs.
    """
    folder = ensure_casts([cast])[cast]
    group = _place(folder, 0, unit, anchor, color, opacity, mirror)
    group = group if window is None else _crop(group, unit, anchor, window)
    group = _Figure(group, folder, unit, anchor, mirror) if window is None and "figureBoundsM" in _index(folder) else group
    group.provenance = _drawn(cast, None)
    return group


class _FigureBounded(VGroup):
    """🪜 A group whose width, height, top, bottom and centre follow its `probes` when `_figure_only` is set: the air layer of a transport cast reaches far beyond the person and must not size, place or align the group."""

    _figure_only = False
    probes: list

    def get_points_defining_boundary(self):
        """🧵 The probes alone for a figure-bounded group."""
        if self._figure_only:
            return np.array([probe.get_center() for probe in self.probes])
        return super().get_points_defining_boundary()

    def length_over_dim(self, dim):
        """🔭 The extent of the probes alone for a figure-bounded group."""
        if self._figure_only:
            points = self.get_points_defining_boundary()
            return float(points[:, dim].max() - points[:, dim].min())
        return super().length_over_dim(dim)


class _Figure(_FigureBounded):
    """🪵 A static transport cast bounded by its figure alone."""

    _figure_only = True

    def __init__(self, group: VGroup, folder: Path, unit: float, anchor, mirror: bool = False):
        super().__init__(*group)
        bounds = _bounds(_index(folder), "figureBoundsM", mirror)
        anchor = np.asarray(anchor, dtype=float)
        self.probes = [VectorizedPoint(anchor + np.array([x, y, 0.0]) * unit) for x, y in ((bounds["minX"], bounds["minY"]), (bounds["maxX"], bounds["maxY"]))]
        self.add(*self.probes)


def _crop(group: VGroup, unit: float, anchor, window: tuple[float, float, float, float]) -> VGroup:
    """✂️ Keep the drawing elements of a placed frame that lie fully inside `window` (world metres)."""
    low, high = np.array(window[:2]) - 1e-6, np.array(window[2:]) + 1e-6
    origin = np.asarray(anchor, dtype=float)[:2]
    inside = []
    for member in group.family_members_with_points():
        points = (member.get_all_points()[:, :2] - origin) / unit
        if (points.min(axis=0) >= low).all() and (points.max(axis=0) <= high).all():
            inside.append(member)
    return VGroup(*inside)


class _SceneClock:
    """⏱️ The scene time of the frame being drawn, in seconds of video: Manim's `renderer.time` while a scene updates its mobjects. A motion is a pure function of this value, so waits, plays, animated groups, skipped plays and partial renders show the same frame at the same instant."""

    __slots__ = ("now",)

    def __init__(self) -> None:
        self.now = 0.0


SCENE_CLOCK = _SceneClock()


def playback_frame(seconds: float, fps: float, count: int, duration: float, loop: bool) -> int:
    """🎬 The frame of a rendered cast at cast time `seconds`: `floor(t · fps + 1e-9)`; a loop wraps in whole frames (`round(duration · fps)` per period) after the floor and any other cast holds its last frame. The twin of `playbackFrame` in `🧍️staffage/🔨️modules/🎞️animation/🟦️.ts`; both read `🧫️fixtures/🎬️playback-clock/🔣️.json`."""
    if count <= 1 or not duration > 0:
        return 0
    step = int(np.floor(seconds * fps + 1e-9))
    if not loop:
        return min(count - 1, max(step, 0))
    period = max(1, int(np.floor(duration * fps + 0.5)))
    return step % period


def _install_scene_clock() -> None:
    """🔌 Publish `renderer.time` to `SCENE_CLOCK` before every update of a scene's mobjects."""
    from manim import Scene

    if getattr(Scene.update_mobjects, "_scene_clock", False):
        return
    update_mobjects, update_to_time = Scene.update_mobjects, Scene.update_to_time

    def publish(self) -> None:
        SCENE_CLOCK.now = float(self.renderer.time)

    def clocked_update_mobjects(self, dt):
        publish(self)
        return update_mobjects(self, dt)

    def clocked_update_to_time(self, t):
        publish(self)
        return update_to_time(self, t)

    clocked_update_mobjects._scene_clock = True
    Scene.update_mobjects = clocked_update_mobjects
    Scene.update_to_time = clocked_update_to_time


_install_scene_clock()


class _Source:
    """🗄️ Where a motion reads its frames from; it has no `__dict__`, so Manim's play-hash sees only its type and the group's own content key keeps the hash small and deterministic."""

    __slots__ = ("folder", "unit", "anchor", "factor", "color", "mirror")

    def __init__(self, folder: Path, unit: float, anchor: np.ndarray, factor: float, color: str | None, mirror: bool = False):
        self.mirror = mirror
        self.folder = folder
        self.unit = unit
        self.anchor = anchor
        self.factor = factor
        self.color = color


class _Motion(_FigureBounded):
    """🎞️ A cast that swaps its frame from scene time; moving, scaling, rotating and fading the group carries over to every later frame.

    The family never changes: a fixed set of slot paths takes the points and style of the current frame (unused slots collapse to an invisible point), so animations that pair families stay valid.
    Frames are parsed on demand into a bounded cache and then written into the slots, so a clip of thousands of frames costs the memory of a few hundred.
    """

    def __init__(self, folder: Path, unit: float, anchor, rate: float, color: str | None, window=None, mirror: bool = False, loop: bool = False):
        super().__init__()
        index = _index(folder)
        bounds = _bounds(index, "figureBoundsM" if "figureBoundsM" in index else "boundsM", mirror)
        self._figure_only = "figureBoundsM" in index
        x0, y0, x1, y1 = window or (bounds["minX"], bounds["minY"], bounds["maxX"], bounds["maxY"])
        anchor = np.asarray(anchor, dtype=float)
        corners = [anchor + np.array([x, y, 0.0]) * unit for x, y in ((x0, y0), (x1, y0), (x0, y1))]
        self.window = window
        self._source = _Source(folder, unit, anchor, unit / index["mmPerMetre"], color if color is not None and index["view"].endswith(_LINE_SUFFIX) else None, mirror)
        self.content = f"{folder.parent.name}/{folder.name}" + ("~mirror" if mirror else "")
        self.frames = tuple(index["frames"])
        self.slots = [_Path() for _ in range(_slot_count(folder))]
        self.fps = float(index["fps"])
        self.loops = bool(index["loop"]) or loop
        self.duration = float(index["durationS"])
        self._rate = rate
        self._cast_start = 0.0
        self._anchor = None
        self.elapsed = 0.0
        self.frame = 0
        self.probes = [VectorizedPoint(corner) for corner in corners]
        self.probes[0].set_fill(opacity=1.0)
        self._base = self._frame_of(self.probes)
        self.add(*self.slots, *self.probes)
        self._show(0)
        self.add_updater(lambda group, dt: group.sync())

    @staticmethod
    def _frame_of(probes) -> tuple[np.ndarray, np.ndarray]:
        """🧭 Express the probes as an origin and a 2x2 basis."""
        origin = probes[0].get_center()[:2]
        return origin, np.column_stack([probes[1].get_center()[:2] - origin, probes[2].get_center()[:2] - origin])

    def _show(self, frame: int) -> None:
        """🔁 Load the pristine frame into the slots, carried through the current transform and opacity of the group."""
        source = self._source
        origin, basis = self._frame_of(self.probes)
        base_origin, base_basis = self._base
        matrix = basis @ np.linalg.inv(base_basis)
        fade = float(self.probes[0].get_fill_opacity())
        parsed = _flipped(_frame(source.folder, frame), source.mirror)
        carried = (parsed.points * source.unit + source.anchor[:2] - base_origin) @ matrix.T + origin
        for position, slot in enumerate(self.slots):
            if position >= len(parsed.spans) or not self._inside(parsed, position):
                slot.set_points(np.tile(self.probes[0].get_center(), (4, 1)))
                slot.set_fill(opacity=0.0)
                slot.set_stroke(width=0.0, opacity=0.0)
                continue
            low, high = parsed.spans[position]
            slot.set_points(np.column_stack((carried[low:high], np.full(high - low, source.anchor[2]))))
            _apply(slot, _style(parsed.shapes[position], source.factor, source.color, fade))
        self.frame = frame

    def _inside(self, parsed: _Frame, position: int) -> bool:
        """🪟 Whether the path at `position` of a frame lies fully inside the window of the cast (cast metres), or no window is set."""
        if self.window is None:
            return True
        low, high = parsed.spans[position]
        points = parsed.points[low:high]
        x0, y0, x1, y1 = self.window
        return bool((points[:, 0] >= x0 - 1e-6).all() and (points[:, 0] <= x1 + 1e-6).all()
                    and (points[:, 1] >= y0 - 1e-6).all() and (points[:, 1] <= y1 + 1e-6).all())

    @property
    def rate(self) -> float:
        """⚡ Cast seconds per scene second."""
        return self._rate

    @rate.setter
    def rate(self, rate: float) -> None:
        if self._anchor is not None:
            self._cast_start = self.elapsed
            self._anchor = SCENE_CLOCK.now
        self._rate = rate

    def _place(self, seconds: float) -> None:
        """📍 Show the frame of cast time `seconds`."""
        self.elapsed = seconds
        frame = playback_frame(seconds, self.fps, len(self.frames), self.duration, self.loops)
        if frame != self.frame:
            self._show(frame)

    def sync(self) -> "_Motion":
        """⏱️ Show the frame of the current scene time: cast time is `rate` times the scene time since the first sync (or the last seek), a pure function of `SCENE_CLOCK` that no play, wait or removal changes."""
        now = SCENE_CLOCK.now
        if self._anchor is None:
            self._anchor = now
        self._place(self._cast_start + self._rate * (now - self._anchor))
        return self

    def seek(self, seconds: float) -> "_Motion":
        """⏩ Jump to `seconds` of cast time at the current scene time; the clip plays on from there."""
        self._cast_start = seconds
        if self._anchor is not None:
            self._anchor = SCENE_CLOCK.now
        self._place(seconds)
        return self

    def restart(self) -> "_Motion":
        """🔁 Start the clip again from its first frame at the current scene time."""
        return self.seek(0.0)


def staffage_motion(cast: str, *, unit: float, anchor=ORIGIN, rate: float = 1.0, color: str | None = None,
                    window: tuple[float, float, float, float] | None = None, mirror: bool = False, loop: bool = False) -> VGroup:
    """🚶 Play a clip or transient cast through scene time at `rate` times real time from the scene time it first appears, looping when the clip loops or `loop` repeats a clip that does not (wrapping at its duration); `window` keeps only the paths fully inside it (cast metres), and the group is bounded by it; `mirror` flips every frame about the vertical axis of the origin."""
    motion = _Motion(ensure_casts([cast])[cast], unit, anchor, rate, color, window, mirror, loop)
    motion.provenance = _drawn(cast, rate)
    return motion
