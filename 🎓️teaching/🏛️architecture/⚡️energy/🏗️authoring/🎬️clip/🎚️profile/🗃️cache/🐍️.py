"""🗃️ Bounded LRU cache whose keys come from Job.key, so entries are profile scoped."""

from __future__ import annotations

import hashlib
import importlib
import json
import os
import threading
import weakref
from collections import OrderedDict
from typing import Any, Callable

_instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
_LIVE: "weakref.WeakSet[BoundedCache]" = weakref.WeakSet()


class BoundedCache:
    """🧺 Evicts least recently used entries beyond `max_entries` or `max_bytes`."""

    def __init__(self, namespace: str, *, max_entries: int, max_bytes: int | None = None, sizeof: Callable[[Any], int] | None = None):
        if max_entries < 1 or (max_bytes is not None and max_bytes < 1):
            raise ValueError("A cache needs positive bounds")
        if (max_bytes is None) != (sizeof is None):
            raise ValueError("Byte bounds need an explicit sizeof; entries-only caches pass neither")
        max_bytes = float("inf") if max_bytes is None else max_bytes
        sizeof = sizeof or (lambda _value: 0)
        self.namespace, self.max_entries, self.max_bytes, self._sizeof = namespace, max_entries, max_bytes, sizeof
        self._items: OrderedDict[str, tuple[Any, int]] = OrderedDict()
        self._bytes = 0
        self._lock = threading.RLock()
        _LIVE.add(self)

    def get(self, key: str) -> Any | None:
        """🔍 Return a cached value and mark it recently used."""
        with self._lock:
            item = self._items.get(key)
            if item is None:
                return None
            self._items.move_to_end(key)
        _instrument.count(f"cache.hit[{self.namespace}]")
        return item[0]

    def put(self, key: str, value: Any) -> None:
        """📥 Store a value; values larger than the byte bound are not retained."""
        size = int(self._sizeof(value))
        evicted = 0
        with self._lock:
            if key in self._items:
                self._bytes -= self._items.pop(key)[1]
            if size > self.max_bytes:
                return
            self._items[key] = (value, size)
            self._bytes += size
            while len(self._items) > self.max_entries or self._bytes > self.max_bytes:
                self._bytes -= self._items.popitem(last=False)[1][1]
                evicted += 1
        _instrument.count(f"cache.create[{self.namespace}]")
        if evicted:
            _instrument.count(f"cache.evict[{self.namespace}]", evicted)

    def clear(self) -> None:
        """🧹 Drop every entry."""
        with self._lock:
            self._items.clear()
            self._bytes = 0

    def get_or_build(self, key: str, build: Callable[[], Any]) -> Any:
        """🏭 Return the cached value or build, store and return it; callers copy mutable values before changing them."""
        value = self.get(key)
        if value is None:
            value = build()
            self.put(key, value)
        return value

    def discard(self, key: str) -> None:
        """🗑️ Drop one entry if present."""
        with self._lock:
            if key in self._items:
                self._bytes -= self._items.pop(key)[1]

    def stats(self) -> dict:
        """📈 Current entries and bytes against their bounds."""
        with self._lock:
            return self._stats()

    def _stats(self) -> dict:
        return {"entries": len(self._items), "bytes": self._bytes, "maxEntries": self.max_entries, "maxBytes": None if self.max_bytes == float("inf") else self.max_bytes}


def registry() -> dict[str, dict]:
    """🧾 Entries and bytes per namespace of every live cache, summed when a namespace has several instances (lifecycle audits)."""
    found: dict[str, dict] = {}
    for item in list(_LIVE):
        stats = item.stats()
        total = found.setdefault(item.namespace, {"entries": 0, "bytes": 0, "maxEntries": 0, "maxBytes": 0, "instances": 0})
        for name in ("entries", "bytes", "maxEntries"):
            total[name] += stats[name]
        total["maxBytes"] = None if total["maxBytes"] is None or stats["maxBytes"] is None else total["maxBytes"] + stats["maxBytes"]
        total["instances"] += 1
    return dict(sorted(found.items()))


def clear_all() -> None:
    """🧹 Empty every live cache (profile switch, tests)."""
    for item in list(_LIVE):
        item.clear()


def scoped_key(*parts) -> str:
    """🗝️ The `Job.key` of the active profile; standalone tools and tests without a Job get a key under the explicit `unscoped` scope instead of a borrowed profile."""
    profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
    if os.environ.get(profile.PROFILE_ENV):
        return profile.job().key(*parts)
    return hashlib.sha256(json.dumps(["unscoped", *parts], sort_keys=True, default=str, separators=(",", ":")).encode()).hexdigest()


def mobject_bytes(mobject) -> int:
    """📐 Approximate retained bytes of a Manim mobject family: point arrays plus a fixed overhead per member."""
    return sum(getattr(member, "points", ()).nbytes + 512 for member in mobject.get_family())


def _immutable(*_arguments, **_keywords):
    raise TypeError("Cached data is immutable; copy it with dict(...) or list(...) before changing it")


class FrozenDict(dict):
    """🧊 A dict that refuses every mutation, compares and serialises like a dict, and survives copy and pickle."""

    __setitem__ = __delitem__ = clear = pop = popitem = setdefault = update = __ior__ = _immutable

    def __reduce__(self):
        return (FrozenDict, (dict(self),))


class FrozenList(list):
    """🧊 A list that refuses every mutation, compares and serialises like a list, and survives copy and pickle."""

    __setitem__ = __delitem__ = __iadd__ = __imul__ = append = extend = insert = pop = remove = clear = sort = reverse = _immutable

    def __reduce__(self):
        return (FrozenList, (list(self),))


def freeze(value):
    """🧊 Recursively replace dicts and lists by frozen ones (tuples and scalars stay), so a cached result can be shared without copying."""
    if isinstance(value, dict):
        return FrozenDict({key: freeze(item) for key, item in value.items()})
    if isinstance(value, list):
        return FrozenList(freeze(item) for item in value)
    if isinstance(value, tuple):
        return tuple(freeze(item) for item in value)
    return value
