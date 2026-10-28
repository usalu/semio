"""🎨 Match canonical CSS surface glass over an explicitly uniform backdrop."""

from __future__ import annotations

import importlib
import math
import re
from pathlib import Path

_ROOT = Path(__file__).resolve().parent
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")


def _rgb(value) -> list[float]:
    """🌈 Read one opaque sRGB backdrop without selecting a host-dependent color profile."""
    if isinstance(value, str) and re.fullmatch(r"#[0-9a-fA-F]{6}", value):
        return [int(value[index:index + 2], 16) for index in (1, 3, 5)]
    if isinstance(value, (tuple, list)) and len(value) == 3 and all(
            not isinstance(channel, bool) and isinstance(channel, (int, float)) and
            math.isfinite(channel) and 0 <= channel <= 255 for channel in value):
        return list(value)
    raise ValueError("Surface backdrops must be six-digit RGB colors or three finite 0-255 channels")


def _hex(rgb) -> str:
    """🔢 Quantize final output channels with CSS-compatible half-up rounding."""
    return "#" + "".join(f"{math.floor(max(0, min(255, value)) + 0.5):02x}" for value in rgb)


def _linear(channel: float) -> float:
    """📉 Decode the sRGB transfer function before perceptual color interpolation."""
    channel /= 255
    return channel / 12.92 if channel <= 0.04045 else ((channel + 0.055) / 1.055) ** 2.4


def _encoded(channel: float) -> float:
    """📈 Encode linear RGB into the gamma-encoded CSS sRGB output domain."""
    return 255 * (12.92 * channel if channel <= 0.0031308 else 1.055 * channel ** (1 / 2.4) - 0.055)


def _oklab(rgb) -> tuple:
    """🧪 Convert linear sRGB through the standard Oklab cone-response matrices."""
    red, green, blue = map(_linear, rgb)
    cube_root = lambda value: math.copysign(abs(value) ** (1 / 3), value)
    long = cube_root(0.4122214708 * red + 0.5363325363 * green + 0.0514459929 * blue)
    medium = cube_root(0.2119034982 * red + 0.6806995451 * green + 0.1073969566 * blue)
    short = cube_root(0.0883024619 * red + 0.2817188376 * green + 0.6299787005 * blue)
    return (0.2104542553 * long + 0.7936177850 * medium - 0.0040720468 * short,
            1.9779984951 * long - 2.4285922050 * medium + 0.4505937099 * short,
            0.0259040371 * long + 0.7827717662 * medium - 0.8086757660 * short)


def _from_oklab(lab) -> list[float]:
    """🧫 Convert a perceptual CSS mixture back to bounded encoded sRGB channels."""
    lightness, axis_a, axis_b = lab
    long = (lightness + 0.3963377774 * axis_a + 0.2158037573 * axis_b) ** 3
    medium = (lightness - 0.1055613458 * axis_a - 0.0638541728 * axis_b) ** 3
    short = (lightness - 0.0894841775 * axis_a - 1.2914855480 * axis_b) ** 3
    linear = (4.0767416621 * long - 3.3077115913 * medium + 0.2309699292 * short,
              -1.2684380046 * long + 2.6097574011 * medium - 0.3413193965 * short,
              -0.0041960863 * long - 0.7034186147 * medium + 1.7076147010 * short)
    return [max(0, min(255, _encoded(channel))) for channel in linear]


def _token_color(value: dict) -> list[float]:
    """🏷️ Resolve the canonical appearance contract before applying surface-level formulas."""
    if "token" in value:
        return _rgb(_theme.color(value["token"]))
    first, second, ratio = value["mix"]
    return [a * ratio + b * (1 - ratio) for a, b in zip(_rgb(_theme.color(first)), _rgb(_theme.color(second)))]


def surface_paint(level: str, parent=None, *, appearance: str = "dark") -> dict:
    """🪟 Resolve Oklab tint, CSS backdrop saturation and sRGB alpha over one uniform parent."""
    levels = _theme.TOKENS["levels"]
    if level not in levels["names"] or appearance not in ("light", "dark"):
        raise ValueError("An explicit canonical surface level and appearance are required")
    depth = levels["names"].index(level)
    chrome = _theme.TOKENS["appearances"][appearance]["chrome"]
    base, foreground = (_token_color(chrome[key]) for key in ("base", "foreground"))
    background = base if parent is None else _rgb(parent)
    fraction = depth * levels["shadeStepPercent"] / 100
    rgb = base if depth == 0 else _from_oklab([
        a * (1 - fraction) + b * fraction for a, b in zip(_oklab(base), _oklab(foreground))])
    alpha = max(0, min(1, 1 - depth * levels["glassAlphaStep"]))
    saturate = levels["glassSaturate"]
    luminance = sum(channel * weight for channel, weight in zip(background, (0.213, 0.715, 0.072)))
    backdrop = [max(0, min(255, luminance + saturate * (channel - luminance))) for channel in background]
    composite = [channel * alpha + behind * (1 - alpha) for channel, behind in zip(rgb, backdrop)]
    return {"level": level, "appearance": appearance, "fill": _hex(rgb), "rgb": rgb,
            "alpha": alpha, "blur_px": depth * levels["glassBlurStepPx"], "saturate": saturate,
            "parent_rgb": background, "backdrop_rgb": backdrop,
            "composite_rgb": [math.floor(channel + 0.5) for channel in composite], "composite": _hex(composite)}


def source_files() -> tuple[Path, ...]:
    """🧬 Bind surface paint identity to the token contract and actual CSS glass formulas."""
    ui = _theme.ROOT / "🧰️framework/🔨️modules/🖱️ui"
    return (Path(__file__), _ROOT / "🧬️schema.json", _theme.TOKEN_PATH,
            ui / "🎨️styling/🖌️ui/🎨️.css",
            ui / "🔨️modules/🌈️surface-presentation/🟦️.ts")
