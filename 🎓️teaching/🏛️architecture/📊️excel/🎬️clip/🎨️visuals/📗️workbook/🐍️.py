"""📗 Microsoft 365 Excel look in vector form: window, ribbon, formula bar, grid, reference frames, trace arrows, error tags, tabs and charts.

Strings and palette: ``🔣️.json`` (schema ``🧬️schema/🔣️.schema.json``). Reference: https://support.microsoft.com/office/overview-of-formulas-in-excel-ecfdc708-9162-49e8-b993-c311f47ca173
"""

from __future__ import annotations

import importlib
import json
import math
import re
import weakref
from pathlib import Path

import numpy as np

_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_engine = importlib.import_module("manim")

P, mix, place, segments, polygon, polyline, region = (getattr(_drafting, name) for name in ("P", "mix", "place", "segments", "polygon", "polyline", "region"))
S_SMALL, S_LABEL, W_THIN, W_EDGE, W_HAIR = _drafting.S_SMALL, _drafting.S_LABEL, _drafting.W_THIN, _drafting.W_EDGE, _drafting.W_HAIR

DATA = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
REFERENCE_COLORS = tuple(DATA["reference_colors"])
ERRORS = DATA["errors"]

PAPER, GRID, HEADER, HEADER_INK, CELL_INK = "#FFFFFF", "#D4D4D4", "#F2F2F2", "#595959", "#1F1F1F"
TITLE_GREEN, GREEN, GREEN_DARK, SELECT_TINT, SELECT_HEADER = "#217346", "#107C41", "#0B5A2F", "#DCEBE2", "#D2E5DA"
RIBBON, TABSTRIP, STATUS, EDGE, TRACE, CHART_BLUE, TIP = "#F3F3F3", "#E9E9E9", "#EFEFEF", "#BFBFBF", "#1F4FD8", "#4472C4", "#FFFFE1"

TITLE_H, RIBBON_H, FORMULA_H, HEADER_H, TABS_H, STATUS_H = 0.30, 0.30, 0.36, 0.28, 0.34, 0.26
CHROME_H = TITLE_H + RIBBON_H + FORMULA_H + HEADER_H + TABS_H + STATUS_H
ROW_HEADER_W = 0.46
ERROR_CODES = frozenset(ERRORS)

_CELL = re.compile(r"\$?([A-Z]{1,3})\$?(\d+)$")
_TOKEN = re.compile(r"\"(?:[^\"]|\"\")*\"|(?<![\w.$])(?P<ref>(?:(?:'[^']+'|[A-Za-z_][\w.]*)!)?\$?[A-Z]{1,3}\$?\d+(?::\$?[A-Z]{1,3}\$?\d+)?)(?![\w(!])")
_CAP: dict = {}


def column_index(letters: str) -> int:
    """🔤 Column letters to a 1-based index: A=1 · Z=26 · AA=27."""
    index = 0
    for char in letters.upper():
        index = index * 26 + ord(char) - 64
    return index


def column_letters(index: int) -> str:
    """🔠 1-based column index to letters."""
    letters = ""
    while index > 0:
        index, rest = divmod(index - 1, 26)
        letters = chr(65 + rest) + letters
    return letters


def split_address(address: str) -> tuple[int, int]:
    """📍 ``$B$3`` to ``(2, 3)`` as (column, row)."""
    match = _CELL.match(address.strip())
    if match is None:
        raise ValueError(f"not a cell address: {address!r}")
    return column_index(match.group(1)), int(match.group(2))


def cell_address(column: int, row: int) -> str:
    """🏷️ ``(2, 3)`` to ``B3``."""
    return f"{column_letters(column)}{row}"


def parse_range(a1: str) -> tuple[int, int, int, int]:
    """🔲 ``B2:D4`` or ``B2`` to normalised ``(col1, row1, col2, row2)``."""
    first, _, last = a1.partition(":")
    (c1, r1), (c2, r2) = split_address(first), split_address(last or first)
    return min(c1, c2), min(r1, r2), max(c1, c2), max(r1, r2)


def split_reference(reference: str) -> tuple[str | None, str]:
    """📑 ``'Type A'!$B$3`` to ``('Type A', 'B3')``; no sheet gives ``None``."""
    sheet, bang, rest = reference.rpartition("!")
    return (sheet.strip("'").replace("''", "'") if bang else None), rest.replace("$", "")


def reference_tokens(formula: str) -> list[tuple[str, str]]:
    """🧩 Split a formula into ``("text"|"ref", string)`` runs; strings and function names never count as references."""
    tokens, position = [], 0
    for match in _TOKEN.finditer(formula):
        if match.group("ref") is None:
            continue
        if match.start() > position:
            tokens.append(("text", formula[position:match.start()]))
        tokens.append(("ref", match.group("ref")))
        position = match.end()
    if position < len(formula):
        tokens.append(("text", formula[position:]))
    return tokens


def reference_colors(formula: str) -> dict[str, str]:
    """🌈 Excel's editing colours in order of first appearance: the same reference keeps its colour."""
    colors: dict[str, str] = {}
    for kind, text in reference_tokens(formula):
        key = text.replace("$", "")
        if kind == "ref" and key not in colors:
            colors[key] = REFERENCE_COLORS[len(colors) % len(REFERENCE_COLORS)]
    return colors


def error_name(code: str, language: str) -> str:
    """⚠️ The localized error label: ``#REF!`` is ``#BEZUG!`` in German Excel."""
    return ERRORS[code][language]


def _label(text: str, size: float, color: str, *, weight: str | None = None, slant: str | None = None, width: float | None = None):
    """🔤 One sans string with a descent attribute; squeezed only when wider than ``width``."""
    kwargs = {key: value for key, value in (("weight", weight), ("slant", slant)) if value}
    if "(" in text or ")" in text:
        _fonts.register_fonts(_theme.FONT_SERIF)
        kwargs["t2f"] = {"(": _theme.FONT_SERIF, ")": _theme.FONT_SERIF}
    if any(char.isalnum() for char in text):
        shape = _fonts.body_text(text, font_size=size, color=color, **kwargs)
        shape.descent = _drafting.descent(size) if _drafting.DESCENDERS & set(text) else 0.0
    else:
        full = _fonts.body_text("H" + text + "H", font_size=size, color=color, **kwargs)
        shape = _engine.VGroup(*full.submobjects[1:-1])
        shape.descent = full.submobjects[0].get_bottom()[1] - shape.get_bottom()[1]
    if width and shape.width > width:
        shape.descent *= width / shape.width
        shape.scale_to_fit_width(width)
    return shape


def _clip(text: str, room: float, size: float, weight: str | None = None) -> str:
    """✂️ The longest start of ``text`` that fits ``room`` once an ellipsis is appended."""
    keep = max(1, int(len(text) * room / max(_label(text, size, CELL_INK, weight=weight).width, 1e-6)) - 1)
    while keep > 1 and _label(text[:keep].rstrip() + "…", size, CELL_INK, weight=weight).width > room:
        keep -= 1
    return text[:keep].rstrip() + "…"


def _wrap(text: str, size: float, room: float) -> list[str]:
    """↩️ Greedy wrap at operators and separators (never inside a reference): the longest start that fits ``room`` per line, found by bisection over break points."""
    lines = []
    while text:
        if _advance(text, size) <= room:
            lines.append(text)
            break
        breaks = [i + 1 for i, char in enumerate(text) if char in ",;(+-*/&=<>" and 0 < i + 1 < len(text)] or [max(1, len(text) // 2)]
        low, high, best = 0, len(breaks) - 1, breaks[0]
        while low <= high:
            middle = (low + high) // 2
            if _advance(text[:breaks[middle]], size) <= room:
                best, low = breaks[middle], middle + 1
            else:
                high = middle - 1
        lines.append(text[:best])
        text = text[best:]
    return lines


def _cap(size: float) -> float:
    """🔡 Cap height of the shared sans at one size."""
    if size not in _CAP:
        _CAP[size] = _fonts.body_text("H", font_size=size, color=CELL_INK).height
    return _CAP[size]


def _put(text: str, x: float, cy: float, align: str, size: float, color: str = CELL_INK, **kwargs):
    """📌 A string on a common baseline: ``l`` starts, ``r`` ends and ``c`` centres at ``x``; vertically centred on ``cy`` by cap height."""
    shape = _label(text, size, color, **kwargs)
    return place(shape, (x, cy - _cap(size) / 2, 0), {"l": "bl", "r": "br", "c": "b"}[align])


def _advance(text: str, size: float) -> float:
    """📏 Horizontal advance of a string: measured in chunks of 60 characters at 16.2 against a bar (so trailing spaces count) and scaled, because pango drops glyphs of very wide texts."""
    bar = _label("|", S_SMALL, CELL_INK).width
    return sum(_label(text[i:i + 60] + "|", S_SMALL, CELL_INK).width - bar for i in range(0, max(len(text), 1), 60)) * size / S_SMALL


def _box(left: float, top: float, right: float, bottom: float, fill: str | None = None, stroke: str | None = None, width: float = W_THIN, opacity: float = 1.0):
    """▭ An axis-aligned rectangle from screen edges."""
    return polygon([(left, top), (right, top), (right, bottom), (left, bottom)], fill, opacity, stroke, width)


def _ring(left: float, top: float, right: float, bottom: float) -> list:
    """⬜ Corner list of a rectangle."""
    return [(left, top), (right, top), (right, bottom), (left, bottom)]


def formula_group(text: str, *, at=(0, 0, 0), size: float = S_LABEL, colors: dict | None = None, ink: str = CELL_INK, caret: bool = False):
    """🎨 A formula as coloured runs on one baseline; reference tokens take ``colors`` (``reference_colors`` by default). ``at`` is the left baseline point."""
    colors = reference_colors(text) if colors is None else colors
    group = _engine.VGroup()
    group.refs = []
    start = P(at)
    offset = 0
    for kind, token in reference_tokens(text):
        tone = colors.get(token.replace("$", ""), ink) if kind == "ref" else ink
        if token.strip():
            lead = len(token) - len(token.lstrip())
            shape = place(_label(token.strip(), size, tone), start + np.array([_advance(text[:offset + lead], size), 0, 0]), "bl")
            group.add(shape)
            if kind == "ref":
                group.refs.append((token, tone, shape))
        offset += len(token)
    if caret:
        x = start[0] + _advance(text, size) + 0.03
        group.add(segments([((x, start[1] - 0.06), (x, start[1] + _cap(size) + 0.06))], ink, W_EDGE))
    group.width_total = _advance(text, size)
    return group


def _frame(rect, color: str, *, dashed: bool = False, handles: bool = True):
    """🔳 A coloured reference frame: tint, border and the four corner handles Excel shows while editing."""
    left, top, right, bottom = rect
    border = polyline(_ring(left, top, right, bottom), color, 2.8, closed=True, dash=0.1 if dashed else None)
    parts = [_box(left, top, right, bottom, color, None, opacity=0.0 if dashed else 0.1), border]
    if handles and not dashed:
        parts += [_box(x - 0.04, y + 0.04, x + 0.04, y - 0.04, color, PAPER, 1.0) for x, y in _ring(left, top, right, bottom)]
    group = _engine.VGroup(*parts)
    group.border = border
    return group


def _tabs(sheets, active: str, *, at, width: float | None, size: float = S_SMALL, titles: dict | None = None):
    """🗂️ The compact sheet tab strip: the active tab white with a green underline, scroll arrows left and a plus right; ``titles`` maps sheet name to tab text."""
    titles = titles or {}
    labels = [_label(titles.get(name, name), size, CELL_INK, weight="BOLD" if name == active else None) for name in sheets]
    widths = [item.width + 0.3 for item in labels]
    total = sum(widths) + 1.5
    width = width or total
    left, cy = at[0] - width / 2, at[1]
    strip = _engine.VGroup(_box(left, cy + TABS_H / 2, left + width, cy - TABS_H / 2, TABSTRIP), segments([((left, cy + TABS_H / 2), (left + width, cy + TABS_H / 2))], EDGE, W_THIN))
    for flip, x in ((-1, left + 0.22), (1, left + 0.52)):
        strip.add(polygon([(x + flip * 0.05, cy), (x - flip * 0.05, cy + 0.07), (x - flip * 0.05, cy - 0.07)], HEADER_INK, 1.0))
    sheets, room = list(sheets), width - 0.9 - 0.5
    names = list(sheets)
    while len(names) > 1 and sum(widths[sheets.index(n)] for n in names[:names.index(active) + 1]) > room:
        names.pop(0)
    kept, spent = [], 0.0
    for n in names:
        spent += widths[sheets.index(n)]
        if spent <= room:
            kept.append((n, labels[sheets.index(n)], widths[sheets.index(n)]))
    x = left + 0.9
    tabs = {}
    for name, item, extent in kept:
        on = name == active
        body = _engine.VGroup(_box(x, cy + TABS_H / 2, x + extent, cy - TABS_H / 2, PAPER if on else None, None, opacity=1.0 if on else 0.0))
        if on:
            body.add(segments([((x, cy - TABS_H / 2 + 0.02), (x + extent, cy - TABS_H / 2 + 0.02))], GREEN, 4.2))
        text = _put(titles.get(name, name), x + extent / 2, cy + (0.015 if on else 0.0), "c", size, GREEN if on else HEADER_INK, weight="BOLD" if on else None)
        body.add(text)
        body.text = text
        tabs[name] = body
        strip.add(body)
        if not on:
            strip.add(segments([((x + extent, cy + 0.09), (x + extent, cy - 0.09))], EDGE, W_HAIR))
        x += extent
    plus = _engine.VGroup(_engine.Circle(radius=0.1, stroke_color=HEADER_INK, stroke_width=W_THIN, fill_opacity=0).move_to(P((x + 0.3, cy))),
                          segments([((x + 0.24, cy), (x + 0.36, cy)), ((x + 0.3, cy - 0.06), (x + 0.3, cy + 0.06))], HEADER_INK, W_THIN))
    strip.add(plus)
    strip.tabs, strip.plus = tabs, plus
    return strip


def _model():
    """🧮 The Excel engine module."""
    return importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")


class EngineView:
    """🔌 The window's read interface over an XL-engine workbook: ``sheets``, ``file_name``, ``formula``, ``value``, ``number_format``, ``column_width``; errors read as their code, bilingual labels in the window language."""

    def __init__(self, workbook, language: str, values: dict | None = None):
        self.workbook, self.language = workbook, language
        self.values = _model().evaluate(workbook) if values is None else values
        self.sheets, self.file_name = tuple(workbook.order()), workbook.file

    def formula(self, sheet: str, addr: str):
        item = self.workbook.cell(sheet, addr)
        return item.formula if item is not None and item.formula else None

    def value(self, sheet: str, addr: str):
        item = self.workbook.cell(sheet, addr)
        if item is None:
            return None
        if item.labels:
            return item.labels[self.language]
        value = self.values.get((self.workbook.sheet(sheet).name, addr))
        return value.code if isinstance(value, _model().XlError) else value

    def number_format(self, sheet: str, addr: str):
        item = self.workbook.cell(sheet, addr)
        return item.format if item is not None and item.format else "General"

    def column_width(self, sheet: str, col: str) -> float:
        return float(self.workbook.sheet(sheet).widths.get(col, 8.43))

    def tab_title(self, sheet: str) -> str:
        return _model().tab_name(self.workbook, sheet, self.language)


def as_view(workbook, language: str, values: dict | None = None):
    """🔌 Anything the window can read: an engine workbook, an object with ``xl_view()``, or a ready view."""
    if hasattr(workbook, "xl_view"):
        return workbook.xl_view()
    return EngineView(workbook, language, values) if hasattr(workbook, "order") and hasattr(workbook, "names") else workbook


class Glide(_engine.Animation):
    """🛝 Morph parts of a window in place. It never adds anything to the scene (the parts keep their place in the window's drawing order, so the tint stays under the text and the frames above), it does not suspend the window's live parts, and the whole window counts as moving so Cairo draws it in one pass."""

    def __init__(self, window, anchor, pairs, **kwargs):
        self.window = window
        self.parts = [_engine.Transform(part, target) for part, target in pairs]
        super().__init__(anchor, introducer=True, **kwargs)

    @property
    def animations(self) -> list:
        """🎞️ The animation list of a group, so scenes may splice it into a step."""
        return [self]

    def _setup_scene(self, scene):
        return None

    def get_all_mobjects(self):
        return (self.window, self.mobject)

    def begin(self):
        for part in self.parts:
            part.begin()
        super().begin()

    def interpolate_mobject(self, alpha):
        for part in self.parts:
            part.interpolate(alpha)

    def finish(self):
        for part in self.parts:
            part.finish()
        super().finish()

    def clean_up_from_scene(self, scene):
        for part in self.parts:
            part.clean_up_from_scene(scene)
        super().clean_up_from_scene(scene)


class Window(_engine.VGroup):
    """🪟 A drawn Excel window over one sheet of a workbook view: ``cell(a1)``, ``range(a1)``, ``rect(a1)``, ``select(a1)``, ``edit(formula)``, ``bind(a1, fn)``."""

    def __init__(self, scene, view, sheet: str, *, at, width: float, height: float, cols: str, rows: tuple, selected: str, show: str, language: str, ribbon: str, editing: str | None):
        super().__init__()
        self._scene = weakref.ref(scene)
        self.view, self.sheet, self.language, self.show, self.ribbon_key = view, sheet, language, show, ribbon
        self.state = {"selected": selected, "editing": editing, "rev": 0}
        first, last = (column_index(part) for part in cols.split(":"))
        self.cols, self.rows = list(range(first, last + 1)), list(range(rows[0], rows[1] + 1))
        self.x0, self.top, self.size_box = at[0] - width / 2, at[1] + height / 2, (width, height)
        self.row_h = (height - CHROME_H) / len(self.rows)
        self.grid_top = self.top - TITLE_H - RIBBON_H - FORMULA_H - HEADER_H
        self.font = S_LABEL if self.row_h >= 0.34 else S_SMALL
        raw = [max(float(view.column_width(sheet, column_letters(c))), 1.0) for c in self.cols]
        scale = (width - ROW_HEADER_W) / sum(raw)
        self.col_x = [self.x0 + ROW_HEADER_W]
        for item in raw:
            self.col_x.append(self.col_x[-1] + item * scale)
        self.cells, self.styles = {}, getattr(view, "style", None)
        self._build()

    @property
    def scene(self):
        """🎬 The owning scene, held weakly so animations can deep-copy the window."""
        return self._scene()

    @property
    def right(self) -> float:
        """➡️ Right edge of the window."""
        return self.x0 + self.size_box[0]

    @property
    def bottom(self) -> float:
        """⬇️ Bottom edge of the window."""
        return self.top - self.size_box[1]

    @property
    def grid_bottom(self) -> float:
        """🔻 Bottom edge of the last visible row."""
        return self.grid_top - self.row_h * len(self.rows)

    def visible(self, a1: str) -> bool:
        """👁️ Whether any part of the range is inside the visible columns and rows."""
        c1, r1, c2, r2 = parse_range(a1)
        return c2 >= self.cols[0] and c1 <= self.cols[-1] and r2 >= self.rows[0] and r1 <= self.rows[-1]

    def rect(self, a1: str) -> tuple[float, float, float, float]:
        """📐 Screen ``(left, top, right, bottom)`` of a cell or range, clamped to the visible grid."""
        c1, r1, c2, r2 = parse_range(a1)
        c1, c2 = max(c1, self.cols[0]), min(c2, self.cols[-1])
        r1, r2 = max(r1, self.rows[0]), min(r2, self.rows[-1])
        return (self.col_x[c1 - self.cols[0]], self.grid_top - (r1 - self.rows[0]) * self.row_h,
                self.col_x[c2 - self.cols[0] + 1], self.grid_top - (r2 - self.rows[0] + 1) * self.row_h)

    def point(self, a1: str, anchor: str = "c") -> np.ndarray:
        """📍 Screen point of a cell: centre, or ``l r u d`` combinations for its edges and corners."""
        left, top, right, bottom = self.rect(a1)
        return P(((left + right) / 2 if "l" not in anchor and "r" not in anchor else left if "l" in anchor else right,
                  (top + bottom) / 2 if "u" not in anchor and "d" not in anchor else top if "u" in anchor else bottom))

    def cell(self, a1: str):
        """🔲 The drawn cell: an invisible box plus its text; raises ``KeyError`` when outside the visible grid."""
        return self.cells[a1.replace("$", "")]

    def range(self, a1: str):
        """🔳 All visible cells of a range as one group, with ``.box`` for its rectangle."""
        c1, r1, c2, r2 = parse_range(a1)
        group = _engine.VGroup(*[self.cells[cell_address(c, r)] for r in range(r1, r2 + 1) for c in range(c1, c2 + 1) if cell_address(c, r) in self.cells])
        group.box = _engine.Rectangle(width=1, height=1, stroke_width=0, fill_opacity=0)
        left, top, right, bottom = self.rect(a1)
        group.box.stretch_to_fit_width(right - left).stretch_to_fit_height(top - bottom).move_to(P(((left + right) / 2, (top + bottom) / 2)))
        return group

    def active_address(self) -> str:
        """🎯 The active cell: the first cell of the selection."""
        return self.state["selected"].split(":")[0].replace("$", "")

    def raw_text(self, value) -> str:
        """🧾 A stored value as typed: General format, error labels and TRUE / FALSE in the window language."""
        if isinstance(value, str):
            return error_name(value, self.language) if value in ERROR_CODES else value
        if isinstance(value, bool):
            return DATA["ui"]["true" if value else "false"][self.language]
        return self.scene.xl_display(value, "General", self.language)

    def text_of(self, a1: str, show: str | None = None) -> tuple[str, str] | None:
        """🔤 Displayed string and alignment of one cell in ``values`` or ``formulas`` view, or ``None`` when empty."""
        show = show or self.show
        formula, value = self.view.formula(self.sheet, a1), self.view.value(self.sheet, a1)
        if show == "formulas" and formula:
            return self.scene.xl_localize(formula, self.language), "l"
        if value is None:
            return None
        if isinstance(value, str):
            return self.raw_text(value), "c" if value in ERROR_CODES else "l"
        if isinstance(value, bool):
            return self.raw_text(value), "c"
        fmt = (self.view.number_format(self.sheet, a1) or "General") if show == "values" else "General"
        return self.scene.xl_display(value, fmt, self.language), "r"

    def _room(self, a1: str, align: str) -> float:
        """↔️ Width a cell text may use: its own cell, plus the empty cells to its right (left-aligned) or the same number of empty cells on both sides (centred)."""
        left, _, right, _ = self.rect(a1)
        column, row = split_address(a1)

        def reach(step: int) -> float:
            end = right if step > 0 else left
            for index in range(column + step, self.cols[-1] + 1 if step > 0 else self.cols[0] - 1, step):
                if self.text_of(cell_address(index, row)) is not None:
                    break
                end = self.col_x[index - self.cols[0] + (1 if step > 0 else 0)]
            return abs(end - (right if step > 0 else left))

        extra = reach(1) if align == "l" else min(reach(1), reach(-1)) * 2 if align == "c" else 0.0
        return right - left + extra - 0.22

    def _cell_label(self, a1: str, text: str, align: str):
        """🏷️ A cell string on the common baseline. Left text and error codes overflow into empty neighbours like Excel; centred text that still does not fit is scaled to fit; left text is cut with an ellipsis and numbers become ###."""
        left, top, right, bottom = self.rect(a1)
        style = self.styles(self.sheet, a1) if self.styles else {}
        align = style.get("align", align)
        room, weight = self._room(a1, align), "BOLD" if style.get("bold") else None
        x = {"l": left + 0.1, "r": right - 0.12, "c": (left + right) / 2}[align]
        fits = _label(text, self.font, CELL_INK, weight=weight).width <= room
        if not fits and align == "r":
            text = "###"
        elif not fits and align == "l":
            text = _clip(text, room, self.font, weight)
        label = _put(text, x, (top + bottom) / 2, align, self.font, style.get("color", CELL_INK), weight=weight)
        if align == "c" and label.width > room:
            label.scale_to_fit_width(room).set_x(x)
        if label.get_right()[0] > right - 0.02 or label.get_left()[0] < left + 0.02:
            return _engine.VGroup(_box(min(left, label.get_left()[0] - 0.1) + 0.03, top - 0.02, max(right, label.get_right()[0] + 0.1), bottom + 0.02, PAPER), label)
        return label

    def _build(self):
        """🧱 Compose shadow, paper, fills, gridlines, selection, cells, headers, overlays and chrome in draw order."""
        width, height = self.size_box
        shadow = _box(self.x0 + 0.07, self.top, self.right + 0.07, self.bottom - 0.09, "#000000", None, opacity=0.4)
        paper = _box(self.x0, self.top, self.right, self.bottom, PAPER, EDGE, W_THIN)
        self.add(shadow, paper)
        fills = _engine.VGroup()
        for row in self.rows:
            for col in self.cols:
                fill = (self.styles(self.sheet, cell_address(col, row)) if self.styles else {}).get("fill")
                if fill:
                    fills.add(_box(*self.rect(cell_address(col, row)), fill))
        gx, gy = self.col_x, [self.grid_top - i * self.row_h for i in range(len(self.rows) + 1)]
        self.gridlines = segments([((x, gy[0]), (x, gy[-1])) for x in gx[1:]] + [((gx[0], y), (gx[-1], y)) for y in gy[1:]], GRID, W_HAIR)
        self.tint, self.active = self._selection_shapes(self.state["selected"])
        self.add(fills, self.tint, self.gridlines)
        for row in self.rows:
            for col in self.cols:
                a1 = cell_address(col, row)
                holder = _engine.VGroup(_box(*self.rect(a1), None, None, 0, 0))
                content = self.text_of(a1)
                holder.text, holder.string, holder.bound = None, None, False
                if content:
                    holder.text, holder.string = self._cell_label(a1, *content), content[0]
                    holder.add(holder.text)
                self.cells[a1] = holder
                self.add(holder)
        self.headers = self.scene.live(self._headers, lambda: self.state["selected"])
        self.frames = self.scene.live(self._edit_frames, lambda: (self.state["editing"], self.state["rev"]))
        self.add(self.headers, self.frames, self.active)
        self.overlay = self.scene.live(self._edit_overlay, lambda: (self.state["editing"], self.state["selected"], self.state["rev"]))
        self.add(self.overlay)
        self.title = self._title_bar()
        self.ribbon = self._ribbon()
        self.name_box, self.formula_bar = self._formula_bar()
        titles = {name: self.view.tab_title(name) for name in self.view.sheets} if hasattr(self.view, "tab_title") else {}
        self.tabs_strip = _tabs(self.view.sheets, self.sheet, at=((self.x0 + self.right) / 2, self.bottom + STATUS_H + TABS_H / 2), width=width, titles=titles)
        self.tabs = self.tabs_strip.tabs
        self.status = self.scene.live(self._status_bar, lambda: (self.state["selected"], self.state["rev"]))
        self.add(self.title, self.ribbon, self.formula_bar, self.name_box, self.tabs_strip, self.status)

    def _selection_shapes(self, a1: str):
        """🟩 Tint with a hole for the active cell, and the thick green border with its fill handle."""
        outer = self.rect(a1)
        hole = self.rect(a1.split(":")[0])
        tint = region([_ring(*outer), _ring(*hole)], SELECT_TINT, 0.9 if ":" in a1 else 0.0)
        left, top, right, bottom = outer
        border = polyline(_ring(left, top, right, bottom), GREEN, 3.4, closed=True)
        handle = _box(right - 0.045, bottom + 0.045, right + 0.045, bottom - 0.045, GREEN, PAPER, 1.4)
        return tint, _engine.VGroup(border, handle)

    def _headers(self):
        """🔢 Column letters and row numbers on grey bands; the selected ones sit on a green-tinted band with a green edge."""
        c1, r1, c2, r2 = parse_range(self.state["selected"])
        group = _engine.VGroup(_box(self.x0, self.grid_top + HEADER_H, self.right, self.grid_top, HEADER),
                               _box(self.x0, self.grid_top, self.x0 + ROW_HEADER_W, self.grid_bottom, HEADER))
        for i, col in enumerate(self.cols):
            if c1 <= col <= c2:
                group.add(_box(self.col_x[i], self.grid_top + HEADER_H, self.col_x[i + 1], self.grid_top, SELECT_HEADER))
        for i, row in enumerate(self.rows):
            if r1 <= row <= r2:
                group.add(_box(self.x0, self.grid_top - i * self.row_h, self.x0 + ROW_HEADER_W, self.grid_top - (i + 1) * self.row_h, SELECT_HEADER))
        group.add(segments([((self.x0, self.grid_top), (self.right, self.grid_top)), ((self.x0 + ROW_HEADER_W, self.grid_top + HEADER_H), (self.x0 + ROW_HEADER_W, self.grid_bottom))], EDGE, W_THIN))
        group.add(segments([((x, self.grid_top + HEADER_H), (x, self.grid_top)) for x in self.col_x[1:]], EDGE, W_HAIR))
        group.add(segments([((self.x0, self.grid_top - i * self.row_h), (self.x0 + ROW_HEADER_W, self.grid_top - i * self.row_h)) for i in range(1, len(self.rows) + 1)], EDGE, W_HAIR))
        group.add(polygon([(self.x0 + ROW_HEADER_W - 0.06, self.grid_top + 0.06), (self.x0 + ROW_HEADER_W - 0.06, self.grid_top + HEADER_H - 0.07), (self.x0 + 0.1, self.grid_top + 0.06)], "#C6C6C6", 1.0))
        for i, col in enumerate(self.cols):
            on = c1 <= col <= c2
            x = (self.col_x[i] + self.col_x[i + 1]) / 2
            group.add(_put(column_letters(col), x, self.grid_top + HEADER_H / 2, "c", self.font, GREEN_DARK if on else HEADER_INK, weight="BOLD" if on else None))
            if on:
                group.add(segments([((self.col_x[i], self.grid_top), (self.col_x[i + 1], self.grid_top))], GREEN, 3.0))
        for i, row in enumerate(self.rows):
            on = r1 <= row <= r2
            y = self.grid_top - (i + 0.5) * self.row_h
            group.add(_put(str(row), self.x0 + ROW_HEADER_W / 2, y, "c", self.font, GREEN_DARK if on else HEADER_INK, weight="BOLD" if on else None))
            if on:
                group.add(segments([((self.x0 + ROW_HEADER_W, y + self.row_h / 2), (self.x0 + ROW_HEADER_W, y - self.row_h / 2))], GREEN, 3.0))
        return group

    def _edit_frames(self):
        """🌈 Reference frames of the formula being edited, one colour per distinct reference on this sheet."""
        formula = self.state["editing"]
        group = _engine.VGroup()
        if formula is None:
            return group
        for key, color in reference_colors(formula).items():
            sheet, a1 = split_reference(key)
            if sheet in (None, self.sheet) and self.visible(a1):
                group.add(_frame(self.rect(a1), color))
        return group

    def _edit_overlay(self):
        """✍️ The active cell in edit mode: the coloured formula with caret inside a widened cell."""
        formula = self.state["editing"]
        if formula is None:
            return _engine.VGroup()
        left, top, right, bottom = self.rect(self.active_address())
        text = self.scene.xl_localize(formula, self.language)
        run = formula_group(text, at=(left + 0.08, (top + bottom) / 2 - _cap(self.font) / 2, 0), size=self.font, caret=True)
        extent = max(right, left + run.width_total + 0.3)
        if extent > self.right - 0.05:
            return _engine.VGroup(_box(left, top, right, bottom, PAPER, GREEN, 3.4))
        return _engine.VGroup(_box(left, top, extent, bottom, PAPER, GREEN, 3.4), run)

    def _title_bar(self):
        """🟢 Green title bar: app grid icon, file name, search pill and window buttons."""
        cy, size = self.top - TITLE_H / 2, S_SMALL
        group = _engine.VGroup(_box(self.x0, self.top, self.right, self.top - TITLE_H, TITLE_GREEN))
        for i in range(9):
            group.add(_engine.Dot(P((self.x0 + 0.2 + (i % 3) * 0.075, cy + 0.075 - (i // 3) * 0.075)), radius=0.017, color=PAPER))
        name = _put(getattr(self.view, "file_name", DATA["file"]),self.x0 + 0.55, cy, "l", size, PAPER, weight="BOLD")
        hint = _put(DATA["ui"]["search"][self.language], 0, cy, "l", size, PAPER)
        low, high = name.get_right()[0] + 0.3, self.right - 1.15
        pill_w = min(3.1, high - low)
        group.add(name)
        if pill_w >= hint.width + 0.62:
            centre = min(max((self.x0 + self.right) / 2 + 0.6, low + pill_w / 2), high - pill_w / 2)
            glass_x, pill = centre - pill_w / 2 + 0.2, _engine.RoundedRectangle(corner_radius=0.1, width=pill_w, height=TITLE_H - 0.09, stroke_width=0, fill_color="#FFFFFF", fill_opacity=0.22).move_to(P((centre, cy)))
            group.add(pill, place(hint, (centre - pill_w / 2 + 0.42, cy - _cap(size) / 2, 0), "bl"))
        elif high - low >= 0.4:
            glass_x = high - 0.2
        else:
            glass_x = None
        if glass_x is not None:
            group.add(_engine.Circle(radius=0.05, stroke_color=PAPER, stroke_width=W_EDGE, fill_opacity=0).move_to(P((glass_x, cy + 0.015))),
                      segments([((glass_x + 0.035, cy - 0.02), (glass_x + 0.08, cy - 0.065))], PAPER, W_EDGE))
        buttons = segments([((self.right - 0.92, cy), (self.right - 0.78, cy)), ((self.right - 0.5, cy - 0.06), (self.right - 0.5, cy + 0.06)), ((self.right - 0.5, cy + 0.06), (self.right - 0.38, cy + 0.06)),
                            ((self.right - 0.38, cy + 0.06), (self.right - 0.38, cy - 0.06)), ((self.right - 0.38, cy - 0.06), (self.right - 0.5, cy - 0.06)),
                            ((self.right - 0.2, cy - 0.06), (self.right - 0.08, cy + 0.06)), ((self.right - 0.2, cy + 0.06), (self.right - 0.08, cy - 0.06))], PAPER, W_EDGE)
        group.add(buttons)
        return group

    def _ribbon(self):
        """🎀 Ribbon tab names with the active one underlined, and the Share button."""
        top = self.top - TITLE_H
        cy = top - RIBBON_H / 2
        group = _engine.VGroup(_box(self.x0, top, self.right, top - RIBBON_H, RIBBON), segments([((self.x0, top - RIBBON_H), (self.right, top - RIBBON_H))], EDGE, W_THIN))
        group.tabs = {}
        x = self.x0 + 0.3
        wide = self.size_box[0] >= 9.0
        for entry in DATA["ribbon"]:
            text = _put(entry[self.language], x, cy, "l", S_LABEL, GREEN if entry["key"] == self.ribbon_key else CELL_INK, weight="BOLD" if entry["key"] == self.ribbon_key else None)
            if text.get_right()[0] > self.right - (1.7 if wide else 0.2):
                break
            group.add(text)
            group.tabs[entry["key"]] = text
            if entry["key"] == self.ribbon_key:
                group.add(segments([((text.get_left()[0] - 0.05, top - RIBBON_H + 0.03), (text.get_right()[0] + 0.05, top - RIBBON_H + 0.03))], GREEN, 3.6))
            x = text.get_right()[0] + 0.3
        if wide:
            share = _put(DATA["ui"]["share"][self.language], self.right - 0.5, cy, "r", S_SMALL, PAPER, weight="BOLD")
            button = _engine.RoundedRectangle(corner_radius=0.06, width=share.width + 0.34, height=RIBBON_H - 0.08, stroke_width=0, fill_color=GREEN, fill_opacity=1).move_to(P((share.get_center()[0], cy)))
            group.add(button, share)
        return group

    def _formula_bar(self):
        """🧮 Name Box with its drop-down, the cancel / enter / ``fx`` icons and the formula field."""
        top = self.top - TITLE_H - RIBBON_H
        cy = top - FORMULA_H / 2
        base = _engine.VGroup(_box(self.x0, top, self.right, top - FORMULA_H, PAPER), segments([((self.x0, top - FORMULA_H), (self.right, top - FORMULA_H))], EDGE, W_THIN))
        box_l, box_r = self.x0 + 0.1, self.x0 + 1.55
        base.add(_box(box_l, cy + 0.14, box_r, cy - 0.14, PAPER, EDGE, W_THIN), polygon([(box_r - 0.2, cy + 0.03), (box_r - 0.1, cy + 0.03), (box_r - 0.15, cy - 0.03)], HEADER_INK, 1.0))
        icons = segments([((box_r + 0.32, cy + 0.05), (box_r + 0.42, cy - 0.05)), ((box_r + 0.32, cy - 0.05), (box_r + 0.42, cy + 0.05))], "#9A9A9A", W_EDGE)
        icons.add(polyline([(box_r + 0.62, cy), (box_r + 0.68, cy - 0.06), (box_r + 0.8, cy + 0.07)], "#9A9A9A", W_EDGE))
        fx = _put("fx", box_r + 1.0, cy, "l", S_LABEL, HEADER_INK, slant="ITALIC", weight="BOLD")
        field_l = box_r + 1.5
        base.add(icons, fx, segments([((field_l - 0.06, cy + 0.14), (field_l - 0.06, cy - 0.14))], EDGE, W_THIN))
        self.field_left = field_l
        name_box = self.scene.live(lambda: _engine.VGroup(_put(self.state["selected"], box_l + 0.12, cy, "l", S_LABEL, CELL_INK)), lambda: self.state["selected"])
        formula_bar = self.scene.live(lambda: self._formula_content(cy), lambda: (self.state["selected"], self.state["editing"], self.state["rev"], self.show))
        bar = _engine.VGroup(base)
        bar.base = base
        return name_box, _engine.VGroup(bar, formula_bar)

    def _formula_content(self, cy: float):
        """📝 The formula bar text: the formula typed, else the active cell's formula or its stored constant; long text shrinks to 16.2 and then wraps into an expanded bar."""
        editing, address = self.state["editing"], self.active_address()
        raw = editing if editing is not None else self.view.formula(self.sheet, address)
        if raw is None:
            value = self.view.value(self.sheet, address)
            return _engine.VGroup() if value is None else self._field(self.raw_text(value), cy, {}, False)
        text = self.scene.xl_localize(raw, self.language)
        return self._field(text, cy, reference_colors(text) if editing is not None else {}, editing is not None)

    def _field(self, text: str, cy: float, colors: dict, caret: bool):
        """↕️ Place text in the formula field: one line at 19.2, else one line at 16.2, else wrapped at operators and separators into a panel that expands downward over the grid (Excel's expand chevron)."""
        room = self.right - self.field_left - 0.55
        size = S_LABEL if _advance(text, S_LABEL) <= room else S_SMALL
        lines = [text] if _advance(text, size) <= room else _wrap(text, size, room)
        chevron_x = self.right - 0.3
        up = len(lines) > 1
        chevron = polyline([(chevron_x - 0.07, cy + (-0.03 if up else 0.03)), (chevron_x, cy + (0.03 if up else -0.03)), (chevron_x + 0.07, cy + (-0.03 if up else 0.03))], HEADER_INK, W_EDGE)
        group = _engine.VGroup(chevron)
        pitch = _cap(size) + 0.17
        if up:
            floor = cy - FORMULA_H / 2 + 0.04 - pitch * (len(lines) - 1) - 0.14
            group.add(_box(self.field_left - 0.06, cy + FORMULA_H / 2 - 0.05, self.right - 0.08, floor, PAPER, EDGE, W_THIN))
            group.add(chevron)
        for index, line in enumerate(lines):
            group.add(formula_group(line, at=(self.field_left, cy - _cap(size) / 2 - index * pitch, 0), size=size, colors=colors, caret=caret and index == len(lines) - 1))
        group.lines = lines
        return group

    def _status_bar(self):
        """📊 Status bar: Ready and, for several selected cells, Average · Count · Sum of the selection."""
        cy = self.bottom + STATUS_H / 2
        group = _engine.VGroup(_box(self.x0, self.bottom + STATUS_H, self.right, self.bottom, STATUS), segments([((self.x0, self.bottom + STATUS_H), (self.right, self.bottom + STATUS_H))], EDGE, W_THIN))
        group.add(_put(DATA["ui"]["ready"][self.language], self.x0 + 0.2, cy, "l", S_SMALL, HEADER_INK))
        c1, r1, c2, r2 = parse_range(self.state["selected"])
        values = [self.view.value(self.sheet, cell_address(c, r)) for r in range(r1, r2 + 1) for c in range(c1, c2 + 1)]
        numbers = [v for v in values if isinstance(v, (int, float)) and not isinstance(v, bool)]
        if len([v for v in values if v is not None]) > 1 and numbers:
            number_format = self.view.number_format(self.sheet, self.active_address()) or "General"
            show = lambda v, fmt=number_format: self.scene.xl_display(v, fmt, self.language)
            parts = [("average", sum(numbers) / len(numbers)), ("count", len([v for v in values if v is not None])), ("sum", sum(numbers))]
            x = self.right - 2.0
            for key, value in reversed(parts):
                item = _put(f"{DATA['ui'][key][self.language]}: {show(value) if key != 'count' else self.scene.xl_display(value, 'General', self.language)}", x, cy, "r", S_SMALL, HEADER_INK)
                if item.get_left()[0] < self.x0 + 1.2:
                    break
                group.add(item)
                x = item.get_left()[0] - 0.35
        group.add(_put("100 %", self.right - 0.2, cy, "r", S_SMALL, HEADER_INK), segments([((self.right - 1.5, cy), (self.right - 0.95, cy))], EDGE, W_EDGE))
        return group

    def select(self, a1: str, **kwargs):
        """🎯 Move the selection; name box, headers, formula bar and status follow at once and the green border glides to the new range."""
        self.state["selected"] = a1.replace("$", "")
        tint, active = self._selection_shapes(self.state["selected"])
        return Glide(self, self.tint, [(self.tint, tint), (self.active, active)], **kwargs)

    def recalculate(self, workbook, values: dict | None = None, **kwargs):
        """♻️ Show a new workbook state: every cell whose text changed morphs into its new text (cells filled by ``bind`` stay live); returns one animation."""
        self.view = as_view(workbook, self.language, values)
        pairs = []
        for a1, holder in self.cells.items():
            content = self.text_of(a1)
            text = content[0] if content else None
            if holder.bound or text == holder.string:
                continue
            old, new = holder.text, self._cell_label(a1, *content) if content else None
            if old is not None and new is not None:
                pairs.append((old, new))
            elif new is not None:
                stub = new.copy().set_opacity(0)
                holder.add(stub)
                pairs.append((stub, new))
                holder.text = stub
            else:
                pairs.append((old, old.copy().set_opacity(0)))
                holder.text = None
            holder.string = text
        self.state["rev"] += 1
        return Glide(self, self.gridlines, pairs, **kwargs)

    def edit(self, formula: str | None) -> "Window":
        """✍️ Enter or leave edit mode with a canonical formula; colour frames and coloured tokens follow."""
        self.state["editing"] = formula
        self.state["rev"] += 1
        return self

    def bind(self, a1: str, fn, align: str = "r"):
        """🔗 Make a cell's text live: ``fn()`` returns the string and the label is re-typeset when it changes."""
        holder = self.cell(a1)
        live = self.scene.live(lambda: _engine.VGroup(self._cell_label(a1, fn(), align)) if fn() else _engine.VGroup(), fn)
        if holder.text is not None:
            holder.remove(holder.text)
        holder.text, holder.bound = live, True
        holder.add(live)
        return live

    def bind_value(self, a1: str, tracker, digits: int = 2, fmt: str | None = None, align: str = "r"):
        """🎚️ Show a ``ValueTracker`` in a cell, formatted by the cell's number format or by language digits."""
        fmt = fmt or self.view.number_format(self.sheet, a1)
        show = (lambda: self.scene.xl_display(tracker.get_value(), fmt, self.language)) if fmt else (lambda: self.scene.number(tracker.get_value(), digits))
        return self.bind(a1, show, align)


class Workbook:
    """📗 Scene mixin: Microsoft 365 Excel widgets (window, frames, fill handle, trace arrows, error tags, tabs, charts, formula text, cell close-up)."""

    def xl_localize(self, formula: str, language: str | None = None) -> str:
        """🌐 Canonical English formula to the displayed text: German names and ``;`` separators when de."""
        return importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️").localize(formula, language or self.language())

    def xl_display(self, value, number_format: str = "General", language: str | None = None) -> str:
        """🔢 A stored value as the cell shows it under its number format and language."""
        return importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️").display(value, number_format, language or self.language())

    def xl_window(self, workbook, sheet: str, *, at=(0, 0, 0), width: float = 12.4, height: float = 4.9, cols: str = "A:F", rows: tuple = (1, 10),
                  selected: str = "A1", show: str = "values", language: str | None = None, ribbon: str = "Home", editing: str | None = None, values: dict | None = None):
        """🪟 The Excel window over one sheet. Parts: ``title``, ``ribbon``, ``name_box``, ``formula_bar``, ``headers``, ``cells``, ``tabs``, ``status``; ``.cell(a1)`` ``.range(a1)`` ``.select(a1)`` ``.edit(f)`` ``.bind_value(a1, tracker)``."""
        language = language or self.language()
        return Window(self, as_view(workbook, language, values), sheet, at=P(at), width=width, height=height, cols=cols, rows=rows, selected=selected, show=show,
                      language=language, ribbon=ribbon, editing=editing)

    def xl_ref_frame(self, window: Window, a1: str, *, color: str | None = None, dashed: bool = False):
        """🔳 A coloured reference frame (or a dashed copy marquee) around a range; ``color`` defaults to the next editing colour."""
        if color is None:
            color = REFERENCE_COLORS[0]
        return _frame(window.rect(a1.replace("$", "")), color, dashed=dashed)

    def xl_fill_handle(self, window: Window, a1: str, *, to: str | None = None, color: str = GREEN):
        """➕ The fill handle square at the bottom-right of a cell; with ``to`` the dashed outline of the range it would fill and a plus cursor."""
        left, top, right, bottom = window.rect(a1.split(":")[0] if to is None else a1)
        group = _engine.VGroup(_box(right - 0.045, bottom + 0.045, right + 0.045, bottom - 0.045, color, PAPER, 1.4))
        group.handle = group[0]
        if to is not None:
            far = window.rect(to)
            group.outline = polyline(_ring(min(left, far[0]), max(top, far[1]), max(right, far[2]), min(bottom, far[3])), color, 2.4, closed=True, dash=0.1)
            x, y = far[2], far[3]
            group.cursor = segments([((x - 0.09, y), (x + 0.09, y)), ((x, y - 0.09), (x, y + 0.09))], CELL_INK, 3.0)
            group.add(group.outline, group.cursor)
        return group

    def xl_trace(self, window: Window, src: str, dst: str, *, kind: str = "precedent", color: str = TRACE):
        """➡️ Trace arrow: blue line from a dot in ``src`` (cell or range) to an arrowhead on ``dst``; ``kind`` is ``precedent`` or ``dependent`` (same drawing, recorded as ``.kind``). A source on another sheet gives a dotted black arrow from a sheet icon."""
        sheet, a1 = split_reference(src)
        end_rect = window.rect(dst.replace("$", ""))
        centre = np.array([end_rect[0] + 0.14, (end_rect[1] + end_rect[3]) / 2, 0.0])
        group = _engine.VGroup()
        group.kind = kind
        if sheet not in (None, window.sheet):
            color = CELL_INK
            icon_at = np.array([max(window.col_x[0] + 0.3, end_rect[0] - 1.0), centre[1], 0.0])
            icon = _engine.VGroup(_box(icon_at[0] - 0.13, icon_at[1] + 0.12, icon_at[0] + 0.13, icon_at[1] - 0.12, PAPER, color, W_EDGE),
                                  segments([((icon_at[0] - 0.13, icon_at[1]), (icon_at[0] + 0.13, icon_at[1])), ((icon_at[0], icon_at[1] + 0.12), (icon_at[0], icon_at[1] - 0.12))], color, W_HAIR))
            start = icon_at + np.array([0.15, 0, 0])
            group.add(icon)
            dash = 0.07
        else:
            src_rect = window.rect(a1)
            start = np.array([src_rect[0] + 0.14, (src_rect[1] + src_rect[3]) / 2, 0.0])
            if ":" in a1:
                group.add(polyline(_ring(*src_rect), color, 2.2, closed=True))
            group.add(_engine.Dot(start, radius=0.06, color=color))
            dash = None
        delta = start - centre
        reach = [((end_rect[2] if delta[0] > 0 else end_rect[0]) - centre[0]) / delta[0] if abs(delta[0]) > 1e-9 else math.inf,
                 ((end_rect[3] if delta[1] < 0 else end_rect[1]) - centre[1]) / delta[1] if abs(delta[1]) > 1e-9 else math.inf, 1.0]
        scale = min(reach)
        tip = centre + delta * scale
        direction = (tip - start) / max(np.linalg.norm(tip - start), 1e-6)
        normal = np.array([-direction[1], direction[0], 0.0])
        head = polygon([tip, tip - direction * 0.17 + normal * 0.075, tip - direction * 0.17 - normal * 0.075], color, 1.0)
        line = polyline([start, tip - direction * 0.14], color, 2.4, dash=dash)
        group.add(line, head)
        group.line, group.head = line, head
        return group

    def xl_error(self, window: Window, a1: str, code: str):
        """⚠️ Error tag on a cell: the green corner triangle, the yellow smart tag with ``!`` and the tooltip text in the window language."""
        left, top, right, bottom = window.rect(a1.replace("$", ""))
        cy = (top + bottom) / 2
        triangle = polygon([(left, top), (left + 0.11, top), (left, top - 0.11)], "#1E9E4A", 1.0)
        centre = (max(left - 0.2, window.x0 + ROW_HEADER_W + 0.2), cy)
        diamond = polygon([(centre[0], centre[1] + 0.13), (centre[0] + 0.13, centre[1]), (centre[0], centre[1] - 0.13), (centre[0] - 0.13, centre[1])], "#F6C700", 1.0, "#7A6200", W_EDGE)
        bang = _put("!", centre[0], centre[1], "c", S_SMALL, "#1F1F1F", weight="BOLD")
        message = _label(ERRORS[code]["tip"][window.language], S_SMALL, CELL_INK)
        tip_w, tip_h = message.width + 0.3, 0.38
        x = min(left - 0.1, window.right - tip_w - 0.1)
        edge = bottom - 0.04 if bottom - 0.08 - tip_h > window.grid_bottom - 0.01 else top + 0.04 + tip_h
        card = _box(x, edge, x + tip_w, edge - tip_h, TIP, "#7A7A7A", W_THIN)
        text = place(message, (x + 0.15, edge - tip_h / 2 - _cap(S_SMALL) / 2, 0), "bl")
        group = _engine.VGroup(triangle, diamond, bang, card, text)
        group.triangle, group.tag, group.tip = triangle, _engine.VGroup(diamond, bang), _engine.VGroup(card, text)
        return group

    def xl_tabs(self, sheets, active: str, *, at=(0, 0, 0), width: float | None = None, titles: dict | None = None):
        """🗂️ A standalone sheet tab strip: ``.tabs[name]`` per tab (keyed by canonical sheet name), the active one white with a green underline; ``titles`` maps names to tab text, e.g. ``engine.sheet_titles(language)``."""
        return _tabs(list(sheets), active, at=at, width=width, titles=titles)

    def xl_formula_text(self, formula: str, *, language: str | None = None, colors_by_ref: dict | None = None, at=(0, 0, 0), size: float = S_LABEL, ink: str = CELL_INK, caret: bool = False):
        """🎨 A canonical formula shown localized with every reference token in its frame colour; ``at`` is the left baseline point, ``.refs`` lists ``(token, colour, mobject)``."""
        text = self.xl_localize(formula, language)
        return formula_group(text, at=at, size=size, colors=colors_by_ref if colors_by_ref is not None else reference_colors(text), ink=ink, caret=caret)

    def xl_bar_chart(self, rows, *, at=(0, 0, 0), width: float = 4.8, height: float = 3.2, unit: str = "", title: str | None = None, decimals: int = 1, color: str = CHART_BLUE):
        """📊 An Excel clustered-column chart. ``rows`` is ``(label, value[, colour])`` with a number, a ``ValueTracker`` or a callable as value; trackers make the bars live."""
        entries = [(row[0], row[1], row[2] if len(row) > 2 else color) for row in rows]
        read = lambda v: float(v.get_value()) if hasattr(v, "get_value") else float(v() if callable(v) else v)
        dynamic = any(hasattr(v, "get_value") or callable(v) for _, v, _ in entries)
        build = lambda: self._chart(entries, read, at, width, height, unit, title, decimals)
        return self.live(build, lambda: tuple(round(read(v), 4) for _, v, _ in entries)) if dynamic else build()

    def _chart(self, entries, read, at, width, height, unit, title, decimals):
        """🧱 Draw one chart state: panel, title, gridlines, axis labels, columns, value labels and categories."""
        values = [read(v) for _, v, _ in entries]
        peak = max(max(values), 1e-9)
        magnitude = 10 ** math.floor(math.log10(peak / 4))
        step = next(m * magnitude for m in (1, 2, 2.5, 5, 10) if m * magnitude >= peak / 4)
        top_value = math.ceil(peak / step - 1e-9) * step
        left, top = at[0] - width / 2, at[1] + height / 2
        plot_l, plot_r = left + 0.95, left + width - 0.25
        plot_t, plot_b = top - (0.95 if title else 0.55), top - height + 0.55
        group = _engine.VGroup(_box(left, top, left + width, top - height, PAPER, "#D9D9D9", W_THIN))
        if title:
            group.add(_put(title, at[0], top - 0.27, "c", S_LABEL, HEADER_INK, weight="BOLD", width=width - 0.4))
        if unit:
            group.add(_put(unit, plot_l - 0.1, plot_t + 0.36, "r", S_SMALL, HEADER_INK))
        ticks = int(round(top_value / step))
        for i in range(ticks + 1):
            y = plot_b + (plot_t - plot_b) * i / ticks
            group.add(segments([((plot_l, y), (plot_r, y))], "#D9D9D9" if i else "#A6A6A6", W_THIN),
                      _put(self.number(step * i, next(d for d in range(4) if abs(step * 10 ** d - round(step * 10 ** d)) < 1e-6)), plot_l - 0.1, y, "r", S_SMALL, HEADER_INK))
        slot = (plot_r - plot_l) / len(entries)
        for i, ((label, _, tone), value) in enumerate(zip(entries, values)):
            x = plot_l + slot * (i + 0.5)
            bar_h = (plot_t - plot_b) * value / top_value
            bar = _box(x - slot * 0.3, plot_b + bar_h, x + slot * 0.3, plot_b, tone)
            group.add(bar, _put(self.number(value, decimals), x, plot_b + bar_h + 0.17, "c", S_SMALL, CELL_INK, width=slot * 0.95),
                      _put(label, x, plot_b - 0.22, "c", S_SMALL, HEADER_INK, width=slot * 0.95))
        return group

    def xl_cell_pop(self, window: Window, a1: str, *, at=None, width: float = 5.4):
        """🔍 Lift one cell out as a card: address, stored value, displayed text, formula and number format. Parts: ``card``, ``rows[name]``, ``leader``, ``cell_ring``.
        Without ``at`` the card stands beside the window when the frame has room (use a window of about 7 units), otherwise it covers the window's upper right."""
        a1 = a1.replace("$", "")
        ui, language, view = DATA["ui"], window.language, window.view
        formula, value, fmt = view.formula(window.sheet, a1), view.value(window.sheet, a1), view.number_format(window.sheet, a1)
        shown = window.text_of(a1, "values")
        stored = "" if value is None else window.raw_text(value)
        pitch, head = 0.44, 0.5
        height = head + 5 * pitch + 0.14
        beside = window.right + 0.3 + width / 2
        centre = P(at) if at is not None else np.array([beside, 0.0, 0.0]) if beside + width / 2 <= 6.9 else np.array([window.right - width / 2 - 0.3, window.top - 1.2 - height / 2, 0.0])
        left, top = centre[0] - width / 2, centre[1] + height / 2
        card = _engine.VGroup(_box(left + 0.05, top - 0.07, left + width + 0.05, top - height - 0.07, "#000000", None, opacity=0.4),
                              _box(left, top, left + width, top - height, PAPER, GREEN, 2.6), _box(left, top, left + width, top - head, TITLE_GREEN))
        card.add(_put(f"{ui['cell'][language]} {a1}", left + 0.2, top - head / 2, "l", S_LABEL, PAPER, weight="BOLD"))
        rows, x_value = {}, left + 0.5 + max(_label(ui[key][language], S_SMALL, CELL_INK).width for key in ('address', 'stored', 'shown', 'formula', 'format'))
        names = (("address", a1, None), ("stored", stored, None), ("shown", shown[0] if shown else "", None), ("formula", formula, "formula"), ("format", fmt, None))
        for i, (key, content, kind) in enumerate(names):
            y = top - head - pitch * (i + 0.5) - 0.07
            title = _put(ui[key][language], left + 0.2, y, "l", S_SMALL, HEADER_INK)
            if kind == "formula":
                body = formula_group(self.xl_localize(content, language), at=(x_value, y - _cap(S_LABEL) / 2, 0), size=S_LABEL) if content else _put(ui["constant"][language], x_value, y, "l", S_SMALL, HEADER_INK)
            else:
                body = _put(str(content) if content != "" else "—", x_value, y, "l", S_LABEL, CELL_INK, width=left + width - x_value - 0.15)
            rows[key] = _engine.VGroup(title, body)
            card.add(rows[key], segments([((left + 0.15, y - pitch / 2), (left + width - 0.15, y - pitch / 2))], GRID, W_HAIR) if i < 4 else _engine.VGroup())
        cell_rect = window.rect(a1)
        ring = polyline(_ring(*cell_rect), GREEN, 3.4, closed=True)
        anchor = np.array([cell_rect[2], (cell_rect[1] + cell_rect[3]) / 2, 0.0])
        leader = segments([(anchor, np.array([left, top - head / 2, 0.0]))], GREEN, W_EDGE)
        group = _engine.VGroup(leader, ring, card)
        group.card, group.rows, group.leader, group.cell_ring = card, rows, leader, ring
        return group
