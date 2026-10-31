"""🗃️ Own bounded opaque templates without renderer or external runtime dependencies."""
from collections import OrderedDict
from contextlib import contextmanager
from dataclasses import dataclass

MAX_INTEGER = 9007199254740991


def _integer(value):
    if type(value) not in (int, float) or not 0 <= value <= MAX_INTEGER or int(value) != value:
        raise ValueError("Expected a nonnegative safe integer")
    return int(value)


def _key(value):
    if not isinstance(value, str) or not 1 <= len(value) <= 4096 or any(0xD800 <= ord(char) <= 0xDFFF for char in value):
        raise ValueError("Expected a bounded Unicode scalar string key")


@dataclass(frozen=True)
class CacheCost:
    """⚖️ Measured resource rows retained by one opaque template."""
    members: int
    points: int

    def __post_init__(self):
        for name in ("members", "points"):
            object.__setattr__(self, name, _integer(getattr(self, name)))


@dataclass(frozen=True)
class CachePolicy:
    """📏 Independent retained-resource and scalar-history limits."""
    max_entries: int = 8
    max_members: int = 1000000
    max_points: int = 10000000
    max_history: int = 64

    def __post_init__(self):
        for name in ("max_entries", "max_members", "max_points", "max_history"):
            object.__setattr__(self, name, _integer(getattr(self, name)))


class CacheCancelled(Exception):
    """🛑 Caller cancellation at a resource lifecycle checkpoint."""


class TemplateCache:
    """📦 Retain canonical templates within independently accounted resource budgets."""

    def __init__(self, policy=None):
        self._entries, self._history = OrderedDict(), OrderedDict()
        self._members = self._points = 0
        self._reserved = CacheCost(0, 0)
        self._busy = False
        self._counts = dict.fromkeys(("hits", "misses", "evictions", "bypasses", "factory_calls", "clone_calls", "underestimates"), 0)
        self.configure(CachePolicy() if policy is None else policy)

    def _idle(self):
        if self._busy:
            raise RuntimeError("Cache callbacks cannot mutate or reenter their cache")

    def _count(self, name):
        self._counts[name] = min(MAX_INTEGER, self._counts[name] + 1)

    def _drop(self, key):
        self._members -= self._entries[key][1].members
        self._points -= self._entries[key][1].points
        del self._entries[key]
        self._count("evictions")

    def _enabled(self):
        return min(self._policy.max_entries, self._policy.max_members, self._policy.max_points) > 0

    def _space(self, cost, incoming=1):
        while self._entries and (len(self._entries) > self._policy.max_entries - incoming or self._members > self._policy.max_members - cost.members or self._points > self._policy.max_points - cost.points):
            self._drop(next(iter(self._entries)))

    def _remember(self, key, cost):
        self._history.pop(key, None)
        self._history[key] = cost
        while len(self._history) > self._policy.max_history:
            self._history.popitem(last=False)

    def configure(self, policy):
        """🎚️ Validate a complete immutable policy before immediately applying its bounds."""
        self._idle()
        if not isinstance(policy, CachePolicy):
            raise TypeError("Expected CachePolicy")
        self._policy = policy
        self._space(CacheCost(0, 0), 0)
        if not self._enabled():
            while self._entries:
                self._drop(next(iter(self._entries)))
        while len(self._history) > policy.max_history:
            self._history.popitem(last=False)

    def stats(self):
        """📊 Return detached resource counts and scalar keys in oldest-first order."""
        return {**self._counts, "entries": len(self._entries), "members": self._members, "points": self._points, "keys": tuple(self._entries), "history_keys": tuple(self._history), "reserved_members": self._reserved.members, "reserved_points": self._reserved.points}

    def release(self):
        """🧹 Drop owned templates and cost history without touching returned client values."""
        self._idle()
        self._entries.clear()
        self._history.clear()
        self._members = self._points = 0

    def invalidate(self, key):
        """🔄 Invalidate a template and every historical reservation for its exact key."""
        self._idle()
        _key(key)
        if key in self._entries:
            self._drop(key)
        self._history.pop(key, None)

    def resolve(self, key, factory, measure, clone, *, reservation=None, valid=None, cancelled=None):
        """🏭 Evict before lazy construction, then admit actual cost before cloning."""
        self._idle()
        _key(key)
        if reservation is not None and not isinstance(reservation, CacheCost):
            raise TypeError("Expected CacheCost reservation")
        self._busy = True
        def checkpoint():
            if cancelled is not None and cancelled():
                raise CacheCancelled("Template request cancelled")
        try:
            checkpoint()
            if key in self._entries:
                if valid is None or valid(self._entries[key][0]):
                    self._count("hits")
                    self._entries.move_to_end(key)
                    if key in self._history:
                        self._history.move_to_end(key)
                    self._count("clone_calls")
                    result = clone(self._entries[key][0])
                    checkpoint()
                    return result
                self._drop(key)
                self._history.pop(key, None)
            self._count("misses")
            wanted = reservation if reservation is not None else self._history.get(key)
            if wanted is None:
                while self._entries:
                    self._drop(next(iter(self._entries)))
                wanted = CacheCost(self._policy.max_members, self._policy.max_points)
            self._history.pop(key, None)
            self._reserved = CacheCost(min(wanted.members, self._policy.max_members), min(wanted.points, self._policy.max_points))
            self._space(self._reserved)
            if not self._enabled():
                while self._entries:
                    self._drop(next(iter(self._entries)))
            checkpoint()
            self._count("factory_calls")
            value = factory()
            checkpoint()
            cost = measure(value)
            if not isinstance(cost, CacheCost):
                raise TypeError("Measurement must return CacheCost")
            checkpoint()
            if cost.members > wanted.members or cost.points > wanted.points:
                self._count("underestimates")
            self._reserved = CacheCost(0, 0)
            if not self._enabled() or cost.members > self._policy.max_members or cost.points > self._policy.max_points:
                self._count("bypasses")
                self._remember(key, cost)
                return value
            self._space(cost)
            self._count("clone_calls")
            result = clone(value)
            checkpoint()
            self._entries[key] = (value, cost)
            self._members += cost.members
            self._points += cost.points
            self._remember(key, cost)
            return result
        finally:
            self._reserved = CacheCost(0, 0)
            self._busy = False


class CacheOwner:
    """🧭 Restore borrowed outer contexts and release only newly owned inner contexts."""

    def __init__(self):
        self.current = None

    @contextmanager
    def scope(self, *, cache=None, policy=None):
        """🔐 Own a fresh scope unless an explicit cache is borrowed."""
        if cache is not None and (not isinstance(cache, TemplateCache) or policy is not None):
            raise TypeError("Borrow a TemplateCache or configure an owned policy")
        previous = self.current
        chosen = TemplateCache(policy) if cache is None else cache
        self.current = chosen
        try:
            yield chosen
        finally:
            self.current = previous
            if cache is None:
                chosen.release()
