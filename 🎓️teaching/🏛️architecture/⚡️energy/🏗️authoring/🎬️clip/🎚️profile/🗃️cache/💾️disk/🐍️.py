"""💾 Bounded on-disk caches: streamed atomic writes, LRU touch, and pruning by bytes and age with orphan and stale-temporary cleanup.

@see ../🐍️.py — the in-process counterpart `BoundedCache`
"""

from __future__ import annotations

import json
import os
import time
import uuid
from pathlib import Path
from typing import Callable, Iterable, Iterator

TEMPORARY_SUFFIX = ".tmp"
STALE_TEMPORARY_S = 3600.0


def temporary_for(path: Path) -> Path:
    """🏷️ A name no other process or job can pick for the same target."""
    return path.with_name(f".{path.name}.{os.getpid()}.{uuid.uuid4().hex[:12]}{TEMPORARY_SUFFIX}")


def publish_lines(path: Path, header: dict, rows: Iterable) -> int:
    """📤 Stream a header line and one JSON line per row into a unique temporary file and publish it atomically; returns the bytes written. Peak memory is one row, never the whole document."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = temporary_for(path)
    written = 0
    try:
        with temporary.open("w", encoding="utf-8", newline="\n") as handle:
            for record in _chain(header, iter(rows)):
                line = json.dumps(record, separators=(",", ":")) + "\n"
                handle.write(line)
                written += len(line)
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)
    return written


def _chain(header: dict, rows: Iterator) -> Iterator:
    yield header
    yield from rows


def read_lines(path: Path) -> Iterator:
    """📥 Yield the JSON records of a file written by `publish_lines`, one line at a time."""
    with Path(path).open("r", encoding="utf-8") as handle:
        for line in handle:
            yield json.loads(line)


def touch(path: Path) -> None:
    """👆 Mark an entry recently used so pruning evicts it last; a vanished file is ignored."""
    try:
        os.utime(path)
    except OSError:
        pass


def prune(folder: Path, *, max_bytes: int, max_age_s: float, suffixes: tuple[str, ...] = (".json",), orphan: Callable[[Path], bool] | None = None,
          keep: frozenset[str] = frozenset(), now: float | None = None) -> dict:
    """🧹 Remove stale temporaries, orphans (`orphan(path)` true: their source digest changed), entries older than `max_age_s` and then the least recently used until the folder holds at most `max_bytes`; names in `keep` survive everything but are counted. Returns what was removed and what remains."""
    folder = Path(folder)
    now = time.time() if now is None else now
    removed = {"temporary": 0, "orphans": 0, "aged": 0, "evicted": 0, "bytes": 0}
    entries: list[tuple[float, int, Path]] = []
    if not folder.is_dir():
        return {**removed, "entries": 0, "totalBytes": 0}
    for path in folder.iterdir():
        try:
            if not path.is_file():
                continue
            stat = path.stat()
        except OSError:
            continue
        if path.name.endswith(TEMPORARY_SUFFIX):
            if now - stat.st_mtime > STALE_TEMPORARY_S and _remove(path):
                removed["temporary"] += 1
                removed["bytes"] += stat.st_size
        elif path.suffix in suffixes:
            if path.name not in keep and orphan is not None and orphan(path) and _remove(path):
                removed["orphans"] += 1
                removed["bytes"] += stat.st_size
            elif path.name not in keep and now - stat.st_mtime > max_age_s and _remove(path):
                removed["aged"] += 1
                removed["bytes"] += stat.st_size
            else:
                entries.append((stat.st_mtime, stat.st_size, path))
    total = sum(size for _, size, _ in entries)
    for _, size, path in sorted(entries, key=lambda entry: (entry[0], entry[2].name)):
        if total <= max_bytes:
            break
        if path.name in keep or not _remove(path):
            continue
        total -= size
        removed["evicted"] += 1
        removed["bytes"] += size
    return {**removed, "entries": sum(1 for _, _, path in entries if path.exists()), "totalBytes": total}


def _remove(path: Path) -> bool:
    try:
        path.unlink()
    except FileNotFoundError:
        return True
    except OSError:
        return False
    return True
