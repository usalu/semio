"""🏗️ Structural mode of the staffage bridge: loads, reactions, member forces and moment diagrams of the staffage archive for scenes, in newtons and newton-metres.

Three sources feed a scene, all derived from geometry and stated loading: the `structure` block of a rendered `forces` or `moments` cast (`cast_structure`), a case document solved by the library solver (`solve_case`), and the EN 1991-1-1 load cases (`load_cases`, `check_imposed`).
A moment here is a moment of force in N·m (displayed kN·m), never a momentum. Axial force is positive in tension and negative in compression; compression is drawn warm, tension cool, zero grey (`ramp_color`).

@see ../../../../../🧍️staffage/🔨️modules/🏗️structure/🟦️.ts — the solver
@see ../../../../../🧍️staffage/🔨️modules/🏗️structure/🧮️solve/🟦️.ts — the case command behind `solve_case`
@see ../../../../../🧍️staffage/🔨️modules/🏗️structure/📋️load-cases/🔣️.json — the load cases
@see ../../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json — `$defs/Structure`
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import tempfile
from decimal import ROUND_HALF_UP, Context, Decimal
from functools import lru_cache
from pathlib import Path

_WORKSPACE = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
_SOURCES = _WORKSPACE / "🎓️teaching/🏛️architecture/🧍️staffage"
_MODULE = _SOURCES / "🔨️modules/🏗️structure"
_LOAD_CASES = _MODULE / "📋️load-cases/🔣️.json"
_SOLVER = _MODULE / "🧮️solve/🟦️.ts"
_INDEX = "🔣️.json"
_PREFIXES = {"N": ((1e3, "kN"), (0.0, "N")), "N·m": ((1e3, "kN·m"), (0.0, "N·m"))}
_STATUS_TOLERANCE = 1e-9
_SOLVE_TIMEOUT_S = 120


def _precision(value: float, significant: int) -> Decimal:
    """🎯 The magnitude of a number rounded half up to `significant` digits, as JavaScript `toPrecision` does."""
    return Context(prec=significant, rounding=ROUND_HALF_UP).create_decimal(Decimal(abs(value)))


def _places(magnitude: Decimal, significant: int) -> int:
    """🔢 Decimal places that keep `significant` digits of a magnitude, never negative."""
    return max(0, significant - 1) if magnitude == 0 else max(0, significant - 1 - magnitude.adjusted())


def quantity_parts(value: float, unit: str, language: str, significant: int = 3) -> tuple[float, str, str]:
    """🧾 A force (`N`) or a moment (`N·m`) with the sensible prefix: kN and kN·m from 1000, the same digits and separators as the library formatter (`formatQuantity`); returns the scaled value, the symbol and the text with a no-break space."""
    if unit not in _PREFIXES:
        raise ValueError(f"unit {unit!r} is not one of {sorted(_PREFIXES)}")
    if language not in ("en", "de"):
        raise ValueError(f"language {language!r} is not 'en' or 'de'")
    threshold, symbol = next(entry for entry in _PREFIXES[unit] if float(_precision(value, significant)) >= entry[0])
    scaled = value / (1 if threshold == 0 else threshold)
    places = 0 if value == 0 else _places(_precision(scaled, significant), significant)
    fixed = f"{Decimal(scaled).quantize(Decimal(1).scaleb(-places), rounding=ROUND_HALF_UP):f}"
    text = fixed.replace("-", "") if float(fixed) == 0 else fixed
    return float(fixed), symbol, f"{text.replace('.', ',') if language == 'de' else text} {symbol}"


def format_quantity(value: float, unit: str, language: str, significant: int = 3) -> str:
    """🧾 `quantity_parts(...)[2]`: `1.25 kN`, `12,5 kN·m`."""
    return quantity_parts(value, unit, language, significant)[2]


@lru_cache(maxsize=None)
def load_cases() -> dict:
    """📋 The EN 1991-1-1 imposed loads and material densities of the library, validated for the fields a scene reads."""
    document = json.loads(_LOAD_CASES.read_text(encoding="utf-8"))
    if document.get("version") != 1:
        raise ValueError(f"unsupported load cases version {document.get('version')!r} in {_LOAD_CASES}")
    for key, quantity in (("occupancy", "areaKNPerM2"), ("materials", "densityKNPerM3")):
        for row in document[key]:
            if not (isinstance(row.get(quantity), (int, float)) and row[quantity] > 0 and row.get("id") and row.get("table")):
                raise ValueError(f"{_LOAD_CASES}: {key} row {row.get('id')!r} needs an id, a table and a positive {quantity}")
    return document


def occupancy(identifier: str) -> dict:
    """👥 The occupancy load `identifier`; the error names the known ids."""
    found = next((row for row in load_cases()["occupancy"] if row["id"] == identifier), None)
    if found is None:
        raise KeyError(f"unknown occupancy load {identifier!r}; known: {', '.join(row['id'] for row in load_cases()['occupancy'])}")
    return found


def material(identifier: str) -> dict:
    """🧱 The material `identifier`; the error names the known ids."""
    found = next((row for row in load_cases()["materials"] if row["id"] == identifier), None)
    if found is None:
        raise KeyError(f"unknown material {identifier!r}; known: {', '.join(row['id'] for row in load_cases()['materials'])}")
    return found


def check_imposed(value_kn_m2: float, category: str) -> dict:
    """✅ Compare a model imposed load with the characteristic value of its EN 1991-1-1 category: `status` is `equal`, `above` (conservative) or `below` (unconservative)."""
    row = occupancy(category)
    standard = row["areaKNPerM2"]
    status = "equal" if abs(value_kn_m2 - standard) <= _STATUS_TOLERANCE * max(1.0, standard) else "above" if value_kn_m2 > standard else "below"
    return {"category": row["category"], "id": row["id"], "title": row["title"], "table": row["table"], "standardKNPerM2": standard, "modelKNPerM2": value_kn_m2, "status": status}


def weight_n(mass_kg: float, gravity: float | None = None) -> float:
    """⚖ The weight of a mass in newtons, `W = m·g`."""
    return mass_kg * (load_cases()["gravityMS2"] if gravity is None else gravity)


def simple_beam(span_m: float, line_n_per_m: float, point_n: float = 0.0, point_at_m: float = 0.0) -> dict:
    """🌉 Closed form of a simply supported span under a uniform line load and one point load: both reactions and the largest sagging moment in N·m."""
    left = line_n_per_m * span_m / 2 + point_n * (span_m - point_at_m) / span_m
    right = line_n_per_m * span_m / 2 + point_n * point_at_m / span_m
    moment = lambda x: left * x - line_n_per_m * x * x / 2 - (point_n * (x - point_at_m) if x > point_at_m else 0.0)
    zero_shear = [x for x in ((left / line_n_per_m, (left - point_n) / line_n_per_m) if line_n_per_m > 0 else ()) if 0 < x < span_m]
    return {"leftN": left, "rightN": right, "maxMomentNm": max(moment(x) for x in (point_at_m, *zero_shear))}


def cantilever(length_m: float, line_n_per_m: float, tip_n: float = 0.0) -> dict:
    """🧗 Closed form of a cantilever under a uniform line load and a tip load, both downward: the wall reaction in N and the clamping moment in N·m (hogging)."""
    return {"reactionN": line_n_per_m * length_m + tip_n, "momentNm": line_n_per_m * length_m**2 / 2 + tip_n * length_m}


def strip_case(name: str, span_m: float, line_n_per_m: float, thickness_m: float, width_m: float = 1.0, elastic_modulus_pa: float = 33e9, gravity: float | None = None) -> dict:
    """🗜 A case document of a simply supported slab strip (pin and roller) under a uniform line load, for `solve_case`: the line load enters as the uniform mass `w·L/g` of the strip."""
    g = load_cases()["gravityMS2"] if gravity is None else gravity
    return {
        "case": name, "gravityMS2": g, "dynamics": "quasi-static", "kind": "structural",
        "materials": [{"id": "concrete", "elasticModulusPa": elastic_modulus_pa}], "root": {"x": 0, "y": 0, "angleDeg": 0},
        "bones": [{"id": "strip", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": span_m, "mass": {"kg": line_n_per_m * span_m / g, "comX": span_m / 2, "comY": 0, "distribution": "uniform"},
                   "section": {"areaM2": width_m * thickness_m, "secondMomentM4": width_m * thickness_m**3 / 12, "extremeFibreM": thickness_m / 2, "material": "concrete"}}],
        "anchors": [{"id": "pin", "bone": "strip", "x": 0, "y": 0, "kind": "support", "support": "pin"}, {"id": "roller", "bone": "strip", "x": span_m, "y": 0, "kind": "support", "support": "roller", "normalDeg": 90}],
        "contacts": [], "supports": ["pin", "roller"], "loads": [], "scenarios": [],
    }


def _repo() -> str:
    """🧰 Resolve the host's provisioned command without assuming a platform-specific binary."""
    configured = os.environ.get("SEMIO_REPO_BINARY") or shutil.which("repo")
    if configured:
        return configured
    marker = _WORKSPACE / ".🧬semio/🦑️repo/native-repo-path"
    if marker.is_file() and Path(marker.read_text(encoding="utf-8").strip()).is_file():
        return marker.read_text(encoding="utf-8").strip()
    raise FileNotFoundError("The repository command is unavailable; start this through repo")


def _cache_root() -> Path:
    """📂 The solved-case cache beside the other teaching caches."""
    base = os.environ.get("SEMIO_TEACHING_CACHE_ROOT")
    return (_WORKSPACE / base if base else _WORKSPACE / ".🧬semio/🎓️teaching/⚡️cache") / "structure"


def _solver_digest() -> str:
    """🔐 Hash the solver sources, so a changed solver never answers from the cache."""
    digest = hashlib.sha256()
    for file in sorted(path for path in _MODULE.rglob("*") if path.is_file() and path.suffix in (".ts", ".json")):
        digest.update(file.relative_to(_MODULE).as_posix().encode("utf-8") + b"\0" + file.read_bytes().replace(b"\r\n", b"\n") + b"\0")
    return digest.hexdigest()


@lru_cache(maxsize=None)
def _solve(canonical: str) -> str:
    """🧮 Solve one canonical case document through the library command, cached on disk by content."""
    key = hashlib.sha256((canonical + _solver_digest()).encode("utf-8")).hexdigest()[:32]
    target = _cache_root() / f"{key}.json"
    if not target.is_file():
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=target.parent) as folder:
            source, out = Path(folder) / "case.json", Path(folder) / "out.json"
            source.write_text(canonical, encoding="utf-8")
            done = subprocess.run([_repo(), "tools", "exec", "bun", "--", str(_SOLVER), "--case", str(source), "--out", str(out)], cwd=_WORKSPACE, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=_SOLVE_TIMEOUT_S)
            if done.returncode != 0 or not out.is_file():
                raise RuntimeError(f"the structural solver failed for case {json.loads(canonical).get('case')!r}: {done.stderr.strip()[-600:]}")
            os.replace(out, target)
    return target.read_text(encoding="utf-8")


def solve_case(document: dict) -> dict:
    """🧮 Solve a case document with the library solver: `summary` (reactions, loads, member extremes, residual, all in N and N·m) and per member every diagram `stations` entry `(s, x, y, axialN, shearN, momentNm)` and the `deflection` line."""
    return json.loads(_solve(json.dumps(document, sort_keys=True, separators=(",", ":"), ensure_ascii=False)))


def cast_structure(cast: str) -> dict:
    """🧾 The `structure` block of a rendered `forces` or `moments` cast: `units`, `colors` and one summary per frame; the cast is rendered first when it is missing."""
    staffage = __import__("importlib").import_module(__package__.rpartition(".")[0] + ".🐍️")
    folder = staffage.ensure_casts([cast])[cast]
    index = json.loads((folder / _INDEX).read_text(encoding="utf-8"))
    if "structure" not in index:
        raise ValueError(f"cast {cast!r} drew the view {index.get('view')!r}; only the views 'forces' and 'moments' carry a structure block")
    return index["structure"]


def _channel(color: str, at: int) -> int:
    return int(color[1 + 2 * at: 3 + 2 * at], 16)


def ramp_color(axial_n: float, reference_n: float, colors: dict) -> str:
    """🎨 The colour of a member under axial force `axial_n` (tension positive): compression warm, tension cool, zero grey, linear to the strong colour at `reference_n`; `reference_n` is the largest |N| of everything drawn together."""
    if reference_n <= 0:
        raise ValueError("reference_n must be positive")
    t = max(-1.0, min(1.0, axial_n / reference_n))
    weak, strong = (colors["compressionWeak"], colors["compressionStrong"]) if t < 0 else (colors["tensionWeak"], colors["tensionStrong"])
    u = abs(t)
    low, high, along = (colors["zero"], weak, u * 2) if u <= 0.5 else (weak, strong, (u - 0.5) * 2)
    return "#" + "".join(f"{round(_channel(low, i) + (_channel(high, i) - _channel(low, i)) * along):02x}" for i in range(3))
