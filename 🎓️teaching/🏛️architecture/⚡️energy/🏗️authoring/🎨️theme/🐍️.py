"""🎨 Bind clip typography, palette and geometry to semio's canonical styling contract."""

from __future__ import annotations

import json
from functools import cache
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
TOKEN_PATH = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json"
ASSET_ROOT = ROOT / "🧰️framework/🔨️modules/🖼️assets"
TOKENS = json.loads(TOKEN_PATH.read_text(encoding="utf-8"))
FONT_SANS = TOKENS["fontStacks"]["sans"].split(",")[0].strip('" ')
FONT_SERIF = TOKENS["fontStacks"]["serif"].split(",")[0].strip('" ')
FONT_MONO = TOKENS["fontStacks"]["mono"].split(",")[0].strip('" ')
FONT_MATH = "Libertinus Math"
MATH_FONT_PATH = ROOT / "🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf"
UI_SPACING = float(TOKENS["spacing"]["compact"].removesuffix("rem")) * 0.5
UI_PIXEL = UI_SPACING / (float(TOKENS["spacing"]["compact"].removesuffix("rem")) * TOKENS["metrics"]["dom"]["rootRemPx"])
CHROME_BORDER = TOKENS["strokes"]["chromeBorderDefault"]


def color(token: str) -> str:
    """🖌️ Resolve a canonical semio palette role without a clip-local color table."""
    return TOKENS["colors"][token]


def px(value: float) -> float:
    """📏 Translate canonical UI pixels to the fixed teaching scene coordinate scale."""
    return value * UI_PIXEL


CHROME_FILL = color("dark-8-9")
CHROME_FOREGROUND = color("light")


def font_size(role: str) -> float:
    """🔎 Preserve semio's type hierarchy at the readable video presentation scale."""
    return TOKENS["metrics"]["typography"][role] * 1.5


@cache
def font_sources() -> dict[str, tuple[Path, ...]]:
    """📇 Select one native face per family so subset aliases cannot shadow body glyphs."""
    catalog = json.loads((ASSET_ROOT / "🔤️fonts/📇️catalog.json").read_text(encoding="utf-8"))
    sources = {}
    for family in catalog["families"]:
        subset = next(part for part in family["subsets"] if part["sourceSubset"] in ("latin", None))
        sources[family["family"]] = (ASSET_ROOT / "🔤️fonts" / family["directory"] / subset["directory"] / "📖️regular" / catalog["encodings"]["ttf"],)
    return {**sources, FONT_MATH: (MATH_FONT_PATH,)}


@cache
def font_files() -> tuple[Path, ...]:
    """🔠 Select bundled outline faces behind the native typography interface."""
    return tuple(path for paths in font_sources().values() for path in paths)


def source_files() -> tuple[Path, ...]:
    """🧬 Expose every style input that changes the appearance of a cached clip."""
    return (Path(__file__), TOKEN_PATH, ASSET_ROOT / "🔤️fonts/📇️catalog.json", *font_files())
