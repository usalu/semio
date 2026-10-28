"""🔤 Render semio typography from bundled faces with consistent glyph coverage and spacing."""

from __future__ import annotations

import importlib
import sys
import struct
from pathlib import Path
from threading import RLock

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

from functools import lru_cache as _cache

_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").configure_manim()
_REGISTERED_FONTS: set[Path] = set()


_PANGO_FONT_LOCK = RLock()
_PANGO_ATTACHED_FONTS: set[str] = set()
_PANGO_TEXT_RENDERER = None


def _render_pango_text(*args, **kwargs):
    """🔤 Attach each native Windows face once to Pango's shared rendering map."""
    import manimpango

    with _PANGO_FONT_LOCK:
        registered = manimpango.registered_fonts
        pending = {font for font in registered if font.type == "win32"}
        redundant = {font for font in pending if font.path in _PANGO_ATTACHED_FONTS}
        registered.difference_update(redundant)
        try:
            result = _PANGO_TEXT_RENDERER(*args, **kwargs)
            _PANGO_ATTACHED_FONTS.update(font.path for font in pending)
            return result
        finally:
            registered.update(redundant)


def _install_pango_font_reuse() -> None:
    """🪟 Keep repeated Windows font attachment within the typography interface."""
    global _PANGO_TEXT_RENDERER
    if sys.platform != "win32" or _PANGO_TEXT_RENDERER is not None:
        return
    import manimpango

    _PANGO_TEXT_RENDERER = manimpango.text2svg
    manimpango.text2svg = _render_pango_text


def register_fonts(font: str | None = None) -> None:
    """📦 Register requested bundled outlines through the existing Pango interface."""
    import manimpango

    with _PANGO_FONT_LOCK:
        for path in _theme.font_sources()[font or _theme.FONT_SANS]:
            if path in _REGISTERED_FONTS:
                continue
            if not path.is_file() or not manimpango.register_font(str(path)):
                raise RuntimeError(f"Cannot register bundled semio font: {path}")
            _REGISTERED_FONTS.add(path)


#region Text glyph coverage and kerning (ManimCommunity/manim#2844)
# Manim rasterises a ``Text`` at the given ``font_size`` and only then scales the
# SVG down by ``TEXT_MOB_SCALE_FACTOR``. Below ~30 px Pango quantises every glyph
# advance to a whole pixel, so the letters in a 17–24 px caption or label sit at
# visibly uneven gaps — the "our fonts render badly" look this project kept
# chasing across revisions. Root cause and fix are in
# https://github.com/ManimCommunity/manim/issues/2844 : rasterise the text at a
# comfortably large size, where Pango has the sub-pixel room to space glyphs
# correctly, then scale the finished mobject down to whatever size the scene
# asked for. The kerning is decided at the large size and survives the scale.
#
# Applied once, by wrapping ``Text.__init__``, so it covers every path with no
# per-scene edits: ``body_text()`` and the shared helpers below, the bare
# ``Text(...)`` calls still scattered through the Heating ``scene_*.py`` files,
# and ``6_final_calculation/scene_6.py``. A scene that sizes a
# ``Text`` by an explicit ``width=``/``height=`` box is left untouched (scaling
# it afterwards would fight that box).
_SYMBOL_CANDIDATES: tuple[str, ...] = (_theme.FONT_MATH, _theme.FONT_SANS, _theme.FONT_SERIF, _theme.FONT_MONO, "Noto Emoji")


@_cache(maxsize=1)
def _font_digest() -> str:
    """🧾 Invalidate text SVGs when typography code or bundled outlines change."""
    from hashlib import sha256

    digest = sha256()
    for path in (Path(__file__), *_theme.source_files()):
        digest.update(path.relative_to(_WORKSPACE_ROOT).as_posix().encode())
        digest.update(path.read_bytes())
    return digest.hexdigest()


@_cache(maxsize=None)
def _font_cmaps(path: Path) -> tuple[bytes, tuple[int, ...]]:
    """🗺️ Read Unicode cmap records directly from a bundled OpenType outline."""
    data = path.read_bytes()
    tables = struct.unpack_from(">H", data, 4)[0]
    offset = next(struct.unpack_from(">I", data, record + 8)[0] for record in range(12, 12 + tables * 16, 16) if data[record:record + 4] == b"cmap")
    count = struct.unpack_from(">H", data, offset + 2)[0]
    records = [struct.unpack_from(">HHI", data, offset + 4 + index * 8) for index in range(count)]
    return data, tuple(dict.fromkeys(offset + subtable for platform, encoding, subtable in records if platform == 0 or (platform == 3 and encoding in (1, 10))))


def _cmap_supports(path: Path, codepoint: int) -> bool:
    """🧮 Resolve format 4 and 12 glyph indices without system fallback fonts."""
    data, offsets = _font_cmaps(path)
    for offset in offsets:
        format_id = struct.unpack_from(">H", data, offset)[0]
        if format_id == 12:
            count = struct.unpack_from(">I", data, offset + 12)[0]
            for index in range(count):
                first, last, glyph = struct.unpack_from(">III", data, offset + 16 + index * 12)
                if first <= codepoint <= last and glyph + codepoint - first:
                    return True
        elif format_id == 4 and codepoint <= 0xFFFF:
            count = struct.unpack_from(">H", data, offset + 6)[0] // 2
            for index in range(count):
                last = struct.unpack_from(">H", data, offset + 14 + index * 2)[0]
                first = struct.unpack_from(">H", data, offset + 16 + count * 2 + index * 2)[0]
                if not first <= codepoint <= last:
                    continue
                delta = struct.unpack_from(">h", data, offset + 16 + count * 4 + index * 2)[0]
                address = offset + 16 + count * 6 + index * 2
                distance = struct.unpack_from(">H", data, address)[0]
                glyph = struct.unpack_from(">H", data, address + distance + (codepoint - first) * 2)[0] if distance else codepoint
                if (not distance or glyph) and (glyph + delta) & 0xFFFF:
                    return True
    return False


@_cache(maxsize=None)
def _font_supports(font: str, character: str) -> bool:
    """🔎 Query the requested bundled family instead of a fallback-aware system face."""
    return any(_cmap_supports(path, ord(character)) for path in _theme.font_sources().get(font, ()))


@_cache(maxsize=1)
def _symbol_fonts() -> tuple[str, ...]:
    """∑ Use the registered bundled families without host-dependent font enumeration."""
    return _SYMBOL_CANDIDATES


def _glyph_fallbacks(font: str, text: str) -> dict[str, str]:
    """↪️ Substitute only characters absent from the requested face."""
    result: dict[str, str] = {}
    for character in sorted(set(text)):
        if character.isspace() or _font_supports(font, character):
            continue
        fallback = next((candidate for candidate in _symbol_fonts() if _font_supports(candidate, character)), None)
        if fallback is None:
            raise RuntimeError(f"No bundled semio font covers U+{ord(character):04X} for {font}")
        register_fonts(fallback)
        result[character] = fallback
    return result


_KERNING_SAFE_FONT_SIZE: float = 60.0
_TEXT_CANVAS_MIN_SIZE: tuple[int, int] = (854, 480)
_TEXT_KERNING_PATCHED: bool = False
_TEXT_KERNING_INIT = None


def install_text_kerning_fix() -> None:
    """🔧 Preserve supported faces, supply missing glyphs, and rasterise small text at a safe size."""
    global _TEXT_KERNING_PATCHED, _TEXT_KERNING_INIT
    from manim import Text

    if _TEXT_KERNING_PATCHED:
        if Text.__init__ is Text._original__init__:
            Text.__init__ = _TEXT_KERNING_INIT
        return

    from contextlib import nullcontext
    from hashlib import sha256
    from manim import DEFAULT_FONT_SIZE, Text, config, tempconfig

    _original_init = Text.__init__
    _original_hash = Text._text2hash

    def _canvas_hash(self, color):
        size = getattr(self, "_semio_text_canvas_size", None)
        key = sha256(f"{_original_hash(self, color)}|semio-fonts:{_font_digest()}".encode()).hexdigest()[:16]
        return sha256(f"{key}|text-canvas:{size[0]}x{size[1]}".encode()).hexdigest()[:16] if size else key

    def _kerned_init(self, *args, font_size: float | None = None, **kwargs):
        kwargs.setdefault("warn_missing_font", False)
        requested = kwargs.get("font") or _theme.FONT_SANS
        kwargs["font"] = requested
        if requested in _theme.font_sources():
            register_fonts(requested)
        for override in (kwargs.get("text2font") or kwargs.get("t2f") or {}).values():
            if override in _theme.font_sources():
                register_fonts(override)
        target = float(DEFAULT_FONT_SIZE if font_size is None else font_size)
        boxed = kwargs.get("height") is not None or kwargs.get("width") is not None
        render = target if (boxed or target >= _KERNING_SAFE_FONT_SIZE) else _KERNING_SAFE_FONT_SIZE
        text = args[0] if args else kwargs.get("text", "")
        font_key = "text2font" if "text2font" in kwargs else "t2f"
        overrides = kwargs.get(font_key) or {}
        protected: set[int] = set()
        for key in overrides:
            for start, end in self._find_indexes(key, text):
                protected.update(range(start, end))
        needed = "".join(character for index, character in enumerate(text) if index not in protected)
        fallbacks = _glyph_fallbacks(requested, needed)
        if fallbacks:
            automatic = {f"[{index}:{index + 1}]": fallbacks[character] for index, character in enumerate(text)
                         if index not in protected and character in fallbacks} if protected else fallbacks
            kwargs[font_key] = {**automatic, **overrides}
        size = (max(config.pixel_width, _TEXT_CANVAS_MIN_SIZE[0]), max(config.pixel_height, _TEXT_CANVAS_MIN_SIZE[1]))
        self._semio_text_canvas_size = size if size != (config.pixel_width, config.pixel_height) else None
        canvas = tempconfig({"pixel_width": size[0], "pixel_height": size[1]}) if self._semio_text_canvas_size else nullcontext()
        with canvas:
            _original_init(self, *args, font_size=render, **kwargs)
        if render != target:
            self.scale(target / render)

    Text._text2hash = _canvas_hash
    Text.__init__ = _kerned_init
    _TEXT_KERNING_INIT = _kerned_init
    _TEXT_KERNING_PATCHED = True


_install_pango_font_reuse()
install_text_kerning_fix()
#endregion


#region Resolve
def resolve_body_font() -> str:
    """🚀 Select semio's bundled sans face on every supported host."""
    register_fonts()
    return _theme.FONT_SANS


BODY_FONT: str = resolve_body_font()
#endregion


#region Scene helper
BODY_LINE_SPACING: float = 0.75
BODY_LINE_BUFF: float = _theme.UI_SPACING


def text_context() -> tuple:
    """🧬 Snapshot the active typography and canvas using the process-captured asset identity."""
    from manim import Text, config

    return (_font_digest(), BODY_FONT, BODY_FONT_SIZE, BODY_LINE_SPACING, _SYMBOL_CANDIDATES,
            _KERNING_SAFE_FONT_SIZE, _TEXT_CANVAS_MIN_SIZE, _TEXT_KERNING_PATCHED,
            config.pixel_width, config.pixel_height, config.frame_width, config.frame_height, str(config.renderer),
            id(vars(Text).get("__init__")), id(vars(Text).get("_text2hash")))


def apply_body_font() -> str:
    """✍️ Set Manim ``Text`` defaults: resolved face, line spacing, no ligatures, kerning fix."""
    from manim import Text

    install_text_kerning_fix()
    Text.set_default(
        font=BODY_FONT,
        line_spacing=BODY_LINE_SPACING,
        disable_ligatures=True,
        warn_missing_font=False,
    )
    return BODY_FONT


def apply_scene_style(scene) -> str:
    """🎬 Apply the shared semio background and bundled typography to a scene."""
    _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
    P_DEEP_DARK = _manim_module.P_DEEP_DARK

    scene.camera.background_color = P_DEEP_DARK
    return apply_body_font()


def body_text(text: str, *, font_size: float, color: str, **kwargs):
    """🔤 One on-screen string with the shared face — never a bare ``Text(...)`` in scenes."""
    from manim import Text

    return Text(
        text,
        font=BODY_FONT,
        font_size=font_size,
        color=color,
        disable_ligatures=True,
        **kwargs,
    )


def wrap_body_lines(text: str, *, font_size: float, max_width: float) -> list[str]:
    """📐 Word-wrap at ``max_width`` while honouring explicit ``\\n`` breaks; size stays fixed."""
    paragraphs = text.split("\n")
    wrapped: list[str] = []
    for para in paragraphs:
        words = para.split()
        if not words:
            continue
        current = words[0]
        for word in words[1:]:
            trial = f"{current} {word}"
            if body_text(trial, font_size=font_size, color=_theme.CHROME_FOREGROUND).width <= max_width:
                current = trial
            else:
                wrapped.append(current)
                current = word
        wrapped.append(current)
    return wrapped or [""]


def centered_body_text(
    text: str,
    *,
    font_size: float,
    color: str,
    line_buff: float | None = None,
    max_width: float | None = None,
):
    """📝 Center-aligned body copy; ``\\n`` / auto-wrap become a VGroup with a fixed vertical gap.

    A single Pango ``Text("a\\nb")`` left-aligns lines inside the SVG and inherits
    flaky vertical metrics — building one ``Text`` per line and arranging them
    with ``center=True`` is the stable path for titles, beat subtitles, and captions.
    """
    from manim import DOWN, VGroup

    if line_buff is None:
        line_buff = BODY_LINE_BUFF
    if max_width is None:
        lines = [part.strip() for part in text.split("\n") if part.strip()] or [""]
    else:
        lines = wrap_body_lines(text, font_size=font_size, max_width=max_width)
    mobs = [body_text(line, font_size=font_size, color=color) for line in lines]
    if len(mobs) == 1:
        return mobs[0]
    group = VGroup(*mobs)
    group.arrange(DOWN, buff=line_buff, center=True)
    return group
#endregion


#region Type scale
TITLE_FONT_SIZE: float = _theme.font_size("text2xlPx")
SUBTITLE_FONT_SIZE: float = _theme.font_size("textBasePx")
BODY_FONT_SIZE: float = _theme.font_size("textBasePx")
LABEL_FONT_SIZE: float = _theme.font_size("textSmPx")
FORMULA_FONT_SIZE: float = _theme.font_size("text2xlPx")
CAPTION_FONT_SIZE: float = _theme.font_size("textLgPx")
#endregion


#region Scene title
# Compact chapter tabs belong to the lesson window rather than a centered title screen.
TITLE_COLOR: str = _theme.CHROME_FOREGROUND


def section_heading(text: str):
    """📑 Place a standalone instructional heading in the first content row."""
    from manim import LEFT, UP

    visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
    label = centered_body_text(text, font_size=LABEL_FONT_SIZE, color=visuals.P_PRIMARY, max_width=13.0)
    heading = visuals.window_chip(label, color=visuals.P_PRIMARY)
    heading.to_edge(UP, buff=_theme.UI_SPACING * 2)
    heading.to_edge(LEFT, buff=_theme.UI_SPACING * 3)
    return heading


#endregion


#region Beat subtitle
# Same primary accent under the module title on every Heating/Cooling beat —
# font, size, colour, gap and x-centering must not drift per file.
BEAT_SUBTITLE_FADE: float = 0.3


#endregion
