"""🎚️ Tower drawing detail resolved centrally from the lod of the active render profile; clip code never chooses it. See 🔣️.lod.json and 🔣️.lod.schema.json."""
import importlib
import json
import os
from pathlib import Path

_profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
_TABLE = json.loads(Path(__file__).with_name("🔣️.lod.json").read_text(encoding="utf-8"))["lods"]


def lod() -> str:
    """🔎 The lod of the profile named by the environment; a library call outside any render (no profile chosen) draws the authored full detail."""
    name = os.environ.get(_profile.PROFILE_ENV)
    return _profile.profile(name).lod if name else "full"


def segments(authored: int) -> int:
    """✂️ Authored capsule corner segments limited by the lod's cap."""
    cap = _TABLE[lod()]["towerSegments"]
    return int(authored) if cap is None else min(int(authored), cap)


def core_assembly() -> str:
    """🗼 Core assembly detail (`full` or `exterior`) for the lod."""
    return _TABLE[lod()]["coreAssembly"]


def template_cache() -> tuple[int, int]:
    """🗃️ Maximum resident whole-tower templates and the total facets they may hold for the lod."""
    row = _TABLE[lod()]
    return row["templateEntries"], row["templateFacets"]
