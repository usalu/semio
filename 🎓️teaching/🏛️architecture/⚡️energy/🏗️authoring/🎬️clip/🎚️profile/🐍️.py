"""🎚️ Explicit preview and final render policies; see 🔣️.json and 🔣️.schema.json."""

from __future__ import annotations

import hashlib
import json
import os
import time
from dataclasses import dataclass
from pathlib import Path

PROFILE_ENV = "SEMIO_TEACHING_PROFILE"
JOB_ENV = "SEMIO_TEACHING_JOB"
FPS_ENV = "SEMIO_TEACHING_FPS"
RESOLUTION_ENV = "SEMIO_TEACHING_RESOLUTION"
NAMES = ("preview", "final")
LODS = ("full", "preview", "poster")
_FILE = Path(__file__).with_name("🔣️.json")
_BUDGETS = ("curveSamples", "heatmapResolution", "particles", "trails")


class ProfileError(ValueError):
    """🚫 A render profile is missing, unknown or malformed."""


@dataclass(frozen=True)
class Budgets:
    """📏 Upper bounds clip code scales its effect knobs with."""

    curve_samples: int | None
    heatmap_resolution: int | None
    particles: int | None
    trails: int | None


@dataclass(frozen=True)
class Profile:
    """🎚️ Immutable render policy; `fingerprint` hashes its JSON entry."""

    name: str
    morph: str
    lod: str
    width: int
    height: int
    fps: float
    budgets: Budgets
    antialias: bool
    shadows: bool
    cache_entries: int
    cache_bytes: int
    performance: tuple[tuple[str, float | int | None], ...]
    fingerprint: str

    def cap(self, budget: str, authored: int) -> int:
        """✂️ Limit an authored count by a named effect budget; a null budget keeps the authored count."""
        limit = getattr(self.budgets, budget)
        return int(authored) if limit is None else min(int(authored), limit)


@dataclass(frozen=True)
class Job:
    """🧾 One scene process: profile, identity, output schedule and seed."""

    profile: Profile
    id: str
    fps: float
    width: int
    height: int
    seed: int = 0

    def key(self, *parts) -> str:
        """🗝️ Cache key that can never collide across profiles."""
        return _key(self.profile.fingerprint, parts)

    def apply(self, config) -> None:
        """🧷 Set the Manim frame rate and pixel size once, at scene module import before any camera exists."""
        config.frame_rate, config.pixel_width, config.pixel_height = self.fps, self.width, self.height

    def verify(self, config) -> None:
        """🛡️ Fail loudly when Manim disagrees with the Job; the Job is the only source of truth."""
        actual = (float(config.frame_rate), int(config.pixel_width), int(config.pixel_height))
        if actual != (float(self.fps), self.width, self.height):
            raise ProfileError(f"Manim renders {actual[1]}x{actual[2]} at {actual[0]:g} fps but job {self.id} ({self.profile.name}) expects {self.width}x{self.height} at {self.fps:g} fps; render through `render --profile` or call job().apply(manim.config) at scene import.")


_JOB: Job | None = None


def _key(fingerprint: str, parts) -> str:
    return hashlib.sha256(json.dumps([fingerprint, *parts], sort_keys=True, default=str, separators=(",", ":")).encode()).hexdigest()


def _document() -> dict:
    return json.loads(_FILE.read_text(encoding="utf-8"))


def _build(name: str, entry: dict) -> Profile:
    budgets = entry["budgets"]
    if not isinstance(budgets, dict) or set(budgets) != set(_BUDGETS) or not all(budgets[key] is None or isinstance(budgets[key], int) and budgets[key] > 0 for key in _BUDGETS):
        raise ProfileError(f"Profile {name} needs positive integer or null budgets {list(_BUDGETS)}")
    output = entry["output"]
    if entry["morph"] not in ("swap", "full") or entry["lod"] not in LODS:
        raise ProfileError(f"Profile {name} needs morph swap|full and lod full|preview|poster")
    if output["width"] % 2 or output["height"] % 2 or output["width"] < 2 or output["height"] < 2 or not output["fps"] > 0:
        raise ProfileError(f"Profile {name} needs positive even dimensions and a positive frame rate")
    canonical = json.dumps(entry, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return Profile(name, entry["morph"], entry["lod"], output["width"], output["height"], float(output["fps"]),
                   Budgets(*(budgets[key] for key in _BUDGETS)), bool(entry["antialias"]), bool(entry["shadows"]),
                   entry["cache"]["maxEntries"], entry["cache"]["maxBytes"], tuple(sorted(entry["performance"].items())),
                   hashlib.sha256(canonical.encode("utf-8")).hexdigest())


def profile(name: str) -> Profile:
    """🔎 Resolve a profile by explicit name; never reads the environment."""
    profiles = _document()["profiles"]
    if name not in profiles:
        raise ProfileError(f"Unknown render profile {name!r}; expected one of {list(NAMES)}")
    return _build(name, profiles[name])


def fingerprint(name: str) -> str:
    """🧬 Hash of one profile's JSON entry."""
    return profile(name).fingerprint


def cache_key(name: str, *parts) -> str:
    """🗝️ Profile-scoped cache key for processes that do not hold a Job."""
    return _key(fingerprint(name), parts)


def active() -> Profile:
    """🎯 The profile of this scene process; fails when none was chosen."""
    name = os.environ.get(PROFILE_ENV)
    if not name:
        raise ProfileError(f"{PROFILE_ENV} is not set. Render through `render --profile preview|final`, or export {PROFILE_ENV}=preview|final for a direct scene render; there is no default profile.")
    return profile(name)


def job() -> Job:
    """🧾 Create the process-wide Job once and reuse it."""
    global _JOB
    if _JOB is None:
        chosen = active()
        resolution = os.environ.get(RESOLUTION_ENV)
        width, height = (int(part) for part in resolution.replace("x", ",").split(",")) if resolution else (chosen.width, chosen.height)
        fps = float(os.environ.get(FPS_ENV) or chosen.fps)
        _JOB = Job(chosen, os.environ.get(JOB_ENV) or f"{chosen.name}-{os.getpid()}-{time.time_ns():x}", fps, width, height)
    return _JOB


def reset() -> None:
    """🧹 Drop the cached Job (tests)."""
    global _JOB
    _JOB = None


def environment(name: str, *, job_id: str, fps: float | None = None, resolution: str | None = None) -> dict[str, str]:
    """🌱 Environment a parent process hands to its scene process."""
    chosen = profile(name)
    return {PROFILE_ENV: chosen.name, JOB_ENV: job_id, FPS_ENV: f"{fps if fps is not None else chosen.fps:g}",
            RESOLUTION_ENV: resolution or f"{chosen.width},{chosen.height}"}
