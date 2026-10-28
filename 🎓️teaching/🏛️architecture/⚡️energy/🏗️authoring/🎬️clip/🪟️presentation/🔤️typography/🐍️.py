"""🔤 Measure bundled Anta and align native text to actual CSS font and baseline metrics."""

from __future__ import annotations

import importlib
import math
import struct
from functools import cache
from pathlib import Path

_ROOT = Path(__file__).resolve().parent
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")


@cache
def _font() -> tuple:
    """📦 Read the exact bundled Anta SFNT tables without a runtime font library."""
    path = _theme.font_sources()[_theme.FONT_SANS][0]
    data = path.read_bytes()
    tables = {data[offset:offset + 4].decode(): struct.unpack_from(">II", data, offset + 8)[0]
              for offset in range(12, 12 + struct.unpack_from(">H", data, 4)[0] * 16, 16)}
    return data, tables


def _u16(data, offset):
    """🔢 Read an unsigned OpenType table word."""
    return struct.unpack_from(">H", data, offset)[0]


def _i16(data, offset):
    """➖ Read a signed OpenType table word."""
    return struct.unpack_from(">h", data, offset)[0]


@cache
def _glyph(codepoint: int) -> int:
    """🗺️ Resolve actual format 4 and 12 cmap entries to bundled glyph identifiers."""
    data, tables = _font()
    base = tables["cmap"]
    for record in range(_u16(data, base + 2)):
        platform, encoding, relative = struct.unpack_from(">HHI", data, base + 4 + record * 8)
        if platform != 0 and not (platform == 3 and encoding in (1, 10)):
            continue
        offset = base + relative
        kind = _u16(data, offset)
        if kind == 12:
            for index in range(struct.unpack_from(">I", data, offset + 12)[0]):
                start, end, glyph = struct.unpack_from(">III", data, offset + 16 + index * 12)
                if start <= codepoint <= end:
                    return glyph + codepoint - start
        elif kind == 4 and codepoint <= 0xffff:
            count = _u16(data, offset + 6) // 2
            for index in range(count):
                end = _u16(data, offset + 14 + index * 2)
                start = _u16(data, offset + 16 + count * 2 + index * 2)
                if not start <= codepoint <= end:
                    continue
                delta = _i16(data, offset + 16 + count * 4 + index * 2)
                address = offset + 16 + count * 6 + index * 2
                distance = _u16(data, address)
                glyph = _u16(data, address + distance + (codepoint - start) * 2) if distance else codepoint
                return (glyph + delta) & 0xffff if glyph or not distance else 0
    return 0


def _coverage(data, offset, glyph):
    """🎯 Locate a glyph in OpenType coverage ranges."""
    kind, count = struct.unpack_from(">HH", data, offset)
    if kind == 1:
        for index in range(count):
            if _u16(data, offset + 4 + index * 2) == glyph:
                return index
    elif kind == 2:
        for index in range(count):
            start, end, coverage = struct.unpack_from(">HHH", data, offset + 4 + index * 6)
            if start <= glyph <= end:
                return coverage + glyph - start
    return None


def _class(data, offset, glyph):
    """🏷️ Resolve pair-positioning class definitions."""
    kind = _u16(data, offset)
    if kind == 1:
        start, count = struct.unpack_from(">HH", data, offset + 2)
        return _u16(data, offset + 6 + (glyph - start) * 2) if start <= glyph < start + count else 0
    if kind == 2:
        for index in range(_u16(data, offset + 2)):
            start, end, value = struct.unpack_from(">HHH", data, offset + 4 + index * 6)
            if start <= glyph <= end:
                return value
    return 0


def _value(data, offset, mask):
    """↔️ Read placement and advance adjustments while skipping device-table references."""
    result = [0, 0, 0, 0]
    for bit in range(8):
        if mask & (1 << bit):
            if bit < 4:
                result[bit] = _i16(data, offset)
            offset += 2
    return result, offset


def _pair(data, offset, left, right):
    """🔗 Apply actual glyph-pair or class-pair positioning from one GPOS subtable."""
    kind, relative, first_format, second_format = struct.unpack_from(">HHHH", data, offset)
    covered = _coverage(data, offset + relative, left)
    if covered is None:
        return None
    if kind == 1:
        start = offset + _u16(data, offset + 10 + covered * 2)
        cursor = start + 2
        for _ in range(_u16(data, start)):
            second = _u16(data, cursor)
            first, cursor = _value(data, cursor + 2, first_format)
            other, cursor = _value(data, cursor, second_format)
            if second == right:
                return first, other
    elif kind == 2:
        first_class = _class(data, offset + _u16(data, offset + 8), left)
        second_class = _class(data, offset + _u16(data, offset + 10), right)
        columns = _u16(data, offset + 14)
        size = 2 * (first_format.bit_count() + second_format.bit_count())
        cursor = offset + 16 + (first_class * columns + second_class) * size
        first, cursor = _value(data, cursor, first_format)
        other, _ = _value(data, cursor, second_format)
        return first, other
    return None


@cache
def _kern_lookups() -> tuple:
    """⚙️ Select the font-provided kern feature instead of inventing a spacing table."""
    data, tables = _font()
    if "GPOS" not in tables:
        return ()
    base = tables["GPOS"]
    features = base + _u16(data, base + 6)
    indices = set()
    for index in range(_u16(data, features)):
        record = features + 2 + index * 6
        if data[record:record + 4] == b"kern":
            feature = features + _u16(data, record + 4)
            indices.update(_u16(data, feature + 4 + item * 2) for item in range(_u16(data, feature + 2)))
    lookups = base + _u16(data, base + 8)
    result = []
    for index in sorted(indices):
        lookup = lookups + _u16(data, lookups + 2 + index * 2)
        kind, flags, count = struct.unpack_from(">HHH", data, lookup)
        subtables = []
        for item in range(count):
            subtable = lookup + _u16(data, lookup + 6 + item * 2)
            actual_kind = kind
            if kind == 9:
                actual_kind = _u16(data, subtable + 2)
                subtable += struct.unpack_from(">I", data, subtable + 4)[0]
            if actual_kind != 2:
                raise ValueError("Bundled Anta kern feature has unsupported positioning")
            subtables.append(subtable)
        result.append(tuple(subtables))
    return tuple(result)


@cache
def _glyph_metrics(glyph: int) -> tuple:
    """📐 Read advance width and true outline bounds, including composite glyph bounds."""
    data, tables = _font()
    count = _u16(data, tables["hhea"] + 34)
    advance = _u16(data, tables["hmtx"] + min(glyph, count - 1) * 4)
    short = _i16(data, tables["head"] + 50) == 0
    width = 2 if short else 4
    unpack = ">H" if short else ">I"
    start = struct.unpack_from(unpack, data, tables["loca"] + glyph * width)[0] * (2 if short else 1)
    end = struct.unpack_from(unpack, data, tables["loca"] + (glyph + 1) * width)[0] * (2 if short else 1)
    bounds = struct.unpack_from(">hhhh", data, tables["glyf"] + start + 2) if start != end else None
    return advance, bounds


def _layout(text: str) -> tuple:
    """🧱 Resolve exact glyph positions, advances and ink boxes from the bundled font."""
    data, tables = _font()
    glyphs = [_glyph(ord(character)) for character in text]
    if any(not glyph and not character.isspace() for character, glyph in zip(text, glyphs)):
        raise ValueError("The canonical Anta face does not cover this label")
    positions = [[0, 0, 0, 0] for _ in glyphs]
    for subtables in _kern_lookups():
        for index in range(len(glyphs) - 1):
            pair = next((value for subtable in subtables
                         if (value := _pair(data, subtable, glyphs[index], glyphs[index + 1])) is not None), None)
            if pair:
                for side in (0, 1):
                    positions[index + side] = [a + b for a, b in zip(positions[index + side], pair[side])]
    pen = 0
    boxes = []
    for character, glyph, (dx, dy, advance, _) in zip(text, glyphs, positions):
        width, bounds = _glyph_metrics(glyph)
        if bounds:
            left, bottom, right, top = bounds
            boxes.append((character, pen + dx + left, -dy - top, pen + dx + right, -dy - bottom))
        pen += width + advance
    return pen, boxes


def text_metrics(text: str, font_size: float = 11.2) -> dict:
    """📊 Return CSS-pixel advances and signed ink bounds relative to the alphabetic baseline."""
    if not isinstance(text, str) or "\n" in text or "\r" in text:
        raise ValueError("Typography metrics require one line of text")
    if isinstance(font_size, bool) or not isinstance(font_size, (int, float)) or not math.isfinite(font_size) or font_size <= 0:
        raise ValueError("Typography metrics require a positive finite font size")
    data, tables = _font()
    units = _u16(data, tables["head"] + 18)
    scale = font_size / units
    pen, boxes = _layout(text)
    ascent, descent = struct.unpack_from(">hh", data, tables["hhea"] + 4)
    return {
        "font_size": font_size, "advance": pen * scale,
        "ascent": ascent * scale, "descent": -descent * scale,
        "ink_left": min((box[1] for box in boxes), default=0) * scale,
        "ink_top": min((box[2] for box in boxes), default=0) * scale,
        "ink_right": max((box[3] for box in boxes), default=0) * scale,
        "ink_bottom": max((box[4] for box in boxes), default=0) * scale,
        "units_per_em": units,
    }


def source_files() -> tuple[Path, ...]:
    """🧾 Track metrics, native typography and the exact bundled outline in cache identity."""
    return (Path(__file__), _ROOT / "🧬️schema.json", _theme.TOKEN_PATH,
            *_theme.font_sources()[_theme.FONT_SANS],
            _ROOT.parents[2] / "🔤️fonts/🐍️.py")


@cache
def _native_glyph(character):
    """✒️ Cache native outlines while exact SFNT layout owns every painted glyph position."""
    fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
    return fonts.body_text(character, font_size=60, color=_theme.color("light"))


@cache
def _native_em_scale():
    """📍 Calibrate one uniform native outline scale against the same font reference glyph."""
    _, bounds = _glyph_metrics(_glyph(ord("H")))
    return _native_glyph("H").height / (bounds[3] - bounds[1])


def _native_text(value, x, y, unit, *, font_size=11.2, line_height=11.2, color=None):
    """🖌️ Align exact glyph positioning and native outlines to a centered CSS line box."""
    from manim import VGroup

    metrics = text_metrics(value, font_size)
    if not math.isfinite(line_height) or line_height <= 0 or not math.isfinite(unit) or unit <= 0:
        raise ValueError("Native typography needs positive finite line height and scale")
    native = VGroup()
    baseline = y + (metrics["ascent"] - metrics["descent"]) / 2
    scale = font_size / metrics["units_per_em"]
    _, boxes = _layout(value)
    for character, left, top, right, bottom in boxes:
        glyph = _native_glyph(character).copy()
        glyph.scale(scale * unit / _native_em_scale())
        center_x = x + (left + right) * scale / 2
        center_y = baseline + (top + bottom) * scale / 2
        glyph.move_to([(center_x - 800) * unit, (450 - center_y) * unit, 0])
        glyph.set_color(color or _theme.color("light"))
        native.add(glyph)
    native._css_metrics = {**metrics, "line_height": line_height}
    return native
