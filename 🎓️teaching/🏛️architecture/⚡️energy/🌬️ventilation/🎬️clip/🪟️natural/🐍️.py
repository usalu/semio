"""🌬️ Lüftung Kapitel 4 — natürliche Lüftung, im Winter wie im Sommer.

Fenster, Querlüftung, Auftrieb und Nachtlüftung bewegen Luft ohne Ventilator. Dieselbe Öffnung
dient im Winter dem kurzen Stoßlüften und im Sommer dem Spülen; die Grenzen der freien Lüftung
(keine Filterung, nicht regelbar, keine Wärmerückgewinnung) führen zur mechanischen Lüftung.
Laufendes Beispiel: 20 °C innen, 0 °C außen im Winter (Δθ = 20 K), 26 °C innen, 32 °C außen im Sommer.
"""

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))


_theme_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")

import numpy as np
from manim import *

from pathlib import Path as _Path


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
reference_chip = _manim_module.reference_chip
section_material = _manim_module.section_material
section_assembly = _manim_module.section_assembly
window_chip = _manim_module.window_chip
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
SAFE_TOP = _manim_module.SAFE_TOP
SAFE_BOTTOM = _manim_module.SAFE_BOTTOM
SAFE_BOTTOM_FORMULA = _manim_module.SAFE_BOTTOM_FORMULA
fit_band = _manim_module.fit_band
symbol_token = _manim_module.symbol_token
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
meter = _manim_module.meter
bind_meter = _manim_module.bind_meter
recolor_meter = _manim_module.recolor_meter
chip = _manim_module.chip
cross_mark = _manim_module.cross_mark
dim_arrow = _manim_module.dim_arrow
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
label_number = _manim_module.label_number
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
load_vo_timing = _manim_module.load_vo_timing
sun_glyph = _manim_module.sun_glyph
moon_glyph = _manim_module.moon_glyph
ripples = _manim_module.ripples
flow_animation = _manim_module.flow_animation
window_glyph = _manim_module.window_glyph
clock_glyph = _manim_module.clock_glyph
tracked_clock = _manim_module.tracked_clock
room_section = _manim_module.room_section
window_opening = _manim_module.window_opening
open_window = _manim_module.open_window
staffage = _manim_module.staffage
room_metre = _manim_module.room_metre
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
TRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"

_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Natürliche Lüftung"



#region Shared visual motifs

def _room(center, width, height, color=P_WHITE, t: float = 0.16, *,
          window_side="none", window_y=None, window_h=1.15):
    """🏠 Closed room material around declared openings, with slabs centred on the outline."""
    inner_h = height - t
    floor_y = float(center[1]) - inner_h / 2
    mid = float(center[1]) if window_y is None else window_y
    opening = (0.30, 0.88) if window_side == "none" else ((mid - window_h / 2 - floor_y) / inner_h, (mid + window_h / 2 - floor_y) / inner_h)
    section = room_section(center, w=width - t, h=inner_h, wall=t, slab=t,
                           window=opening, window_side=window_side)
    body = VGroup(section["ceiling"], *[m for m in section["shell"] if m is not section["floor"] and m is not section["ceiling"]])
    return section_material(VGroup(body, section["floor"]))


def _window(pos, height=1.15, width=0.16, color=P_CYAN, inward=1):
    """🪟 Recessed sash projected inward within a structural wall aperture."""
    win = window_glyph(float(pos[0]), float(pos[1]) - height / 2, float(pos[1]) + height / 2,
                       depth=0.8 * height, wall=width, color=color, inward=inward)
    win["sash"].become(window_opening(win))
    win["group"].window = win
    return win["group"]


def _stand(room_c, room_h, x, cast: str = "shared-person-pathlines", t: float = 0.16):
    """🧍 A 1.75 m breathing occupant (thermal staffage clip) in a 2.5 m clear room, anchored to the slab top."""
    unit = room_metre(room_h - t)
    floor_y = float(room_c[1]) - room_h / 2 + t / 2
    return staffage_motion(cast, unit=unit, anchor=np.array([x, floor_y, 0.0]))


def _badge(title, value, color=P_PRIMARY):
    """🏷️ Compact callout card for design levers / comfort states."""
    import re

    temp = re.match(r"^(\d+)\s*°C(?:\s*·\s*(.+))?$", value)
    if temp:
        value_mob = math_label(rf"{temp.group(1)}\,\mathrm{{°C}}", size=BODY_FONT_SIZE, color=P_WHITE)
        if temp.group(2):
            value_mob = VGroup(
                value_mob,
                Text(temp.group(2), font_size=LABEL_FONT_SIZE, color=P_WHITE),
            ).arrange(RIGHT, buff=0.1)
    else:
        value_mob = Text(value, font_size=BODY_FONT_SIZE, color=P_WHITE)
    body = VGroup(
        Text(title, font_size=LABEL_FONT_SIZE, color=color),
        value_mob,
    ).arrange(DOWN, buff=0.06)
    box = SurroundingRectangle(body, color=color, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER)
    return VGroup(box, body)


def _hand_chip(text: str, src: str, color: str):
    """🔖 Hand-over chip — german lead-in plus a typeset quantity, boxed in the later video's colour."""
    label = VGroup(Text(text, font_size=LABEL_FONT_SIZE, color=color),
                   math_label(src, size=BODY_FONT_SIZE, color=color)).arrange(RIGHT, buff=_theme_module.UI_SPACING)
    return window_chip(label, color=color)


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)



def _open_sash(window, *, run_time: float = 0.9, close: bool = False):
    """🚪 Project the shared sash open or closed around its recessed hinge."""
    return open_window(window, run_time=run_time, close=close)


def _vbar(x: float, y0: float, height: float, frac, *, width: float = 0.26, color=P_CYAN):
    """📊 Vertical fill gauge from ``y0`` upward; ``frac()`` returns 0…1 every frame."""
    track = Rectangle(width=width, height=height, color=P_WHITE, stroke_width=2)
    track.set_fill(P_DEEP_DARK, opacity=1.0).move_to([x, y0 + height / 2, 0])

    def fill():
        h = max(0.004, float(np.clip(frac(), 0, 1)) * (height - 0.06))
        return Rectangle(width=width - 0.06, height=h, stroke_width=0, fill_color=color,
                         fill_opacity=0.85).move_to([x, y0 + 0.03 + h / 2, 0])

    return VGroup(track, always_redraw(fill))


def _remember(mob):
    """🧠 Keep the authored fill and stroke opacity of every member so ``_level`` can fade without flattening it."""
    mob.authored = [(m.get_fill_opacity(), m.get_stroke_opacity()) for m in mob.family_members_with_points()]
    return mob


def _level(mob, level: float):
    """🔆 Show ``mob`` at ``level`` (0…1) of its authored opacity."""
    for m, (fill, stroke) in zip(mob.family_members_with_points(), mob.authored):
        m.set_fill(opacity=fill * level).set_stroke(opacity=stroke * level)


def _power(rad, on: bool):
    """🔥 Cross-fade the radiator between its warm (rated) and cold (off) thermal staffage."""
    shown, hidden = (rad[0], rad[1]) if on else (rad[1], rad[0])
    return [UpdateFromAlphaFunc(shown, lambda m, a: _level(m, a)), UpdateFromAlphaFunc(hidden, lambda m, a: _level(m, 1.0 - a))]


def _vent_room(ox: float, floor_y: float, k: float, title: str, color):
    """🛋️ Room section with exterior wall, window opening, radiator, thermostat and the reveal (Laibung).

    ``ox`` is the inner face of the back wall, ``k`` scene units per metre. The
    room is 4 m wide and 2,5 m high; the window sits 0,9 m … 2,2 m above the floor
    in a 0,35 m exterior wall with the radiator below it.
    """
    w, h, t = 4.0 * k, 2.5 * k, 0.35 * k
    x_in, x_out = ox + w, ox + w + t
    sill, lintel, ceil = floor_y + 0.9 * k, floor_y + 2.2 * k, floor_y + h
    floor = section_material(Rectangle(width=w + t + 0.18 * k, height=0.2 * k))
    floor.move_to([ox - 0.18 * k + (w + t + 0.18 * k) / 2, floor_y - 0.10 * k, 0])
    ceiling = floor.copy().move_to([floor.get_center()[0], ceil + 0.10 * k, 0])
    back = section_material(Rectangle(width=0.18 * k, height=h)).move_to([ox - 0.09 * k, floor_y + h / 2, 0])
    brust = section_material(Rectangle(width=t, height=sill - floor_y)).move_to([(x_in + x_out) / 2, (floor_y + sill) / 2, 0])
    sturz = section_material(Rectangle(width=t, height=ceil - lintel)).move_to([(x_in + x_out) / 2, (lintel + ceil) / 2, 0])
    laibung = Rectangle(width=t, height=lintel - sill, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.14)
    laibung.move_to([(x_in + x_out) / 2, (sill + lintel) / 2, 0])
    window = window_glyph((x_in + x_out) / 2, sill, lintel, depth=0.9 * k, wall=t, inward=-1, color=P_CYAN)
    rad = VGroup(*[_remember(staffage(cast, unit=k, anchor=np.array([x_in, floor_y + 0.04 * k, 0.0])))
                   for cast in ("ventilation-natural-radiator-on-mirrored", "ventilation-natural-radiator-off-mirrored")])
    _level(rad[1], 0.0)
    valve = VectorizedPoint([x_in - 0.09 * k, floor_y + 0.125 * k, 0])
    head = math_label(title, size=BODY_FONT_SIZE, color=color)
    head.move_to([ox + w / 2, ceil + 0.36, 0])
    material = section_assembly(floor, ceiling, back, brust, sturz)
    return {
        "x_in": x_in, "x_out": x_out, "sill": sill, "lintel": lintel, "ceil": ceil, "floor": floor_y,
        "ox": ox, "w": w, "k": k, "rad": rad, "valve": valve, "window": window, "sash": window["sash"],
        "sturz": sturz, "laibung": laibung, "head": head,
        "material": material, "shell": VGroup(material, laibung),
        "group": VGroup(material, laibung, window["group"], rad, valve),
    }


def _valve_dial(room, opening):
    """🌡️ Thermostat dial mark — turns with ``opening()`` from closed (0) to fully open (1)."""
    def mark():
        c = room["valve"].get_center()
        a = PI / 2 - 0.9 * PI * float(np.clip(opening(), 0, 1))
        return Line(c, c + 0.06 * np.array([np.cos(a), np.sin(a), 0]), color=P_YELLOW, stroke_width=2)
    return always_redraw(mark)


def _energy_row(x0: float, y: float, unit: float, parts, *, height: float = 0.5):
    """🧱 Horizontal stacked bar redrawn every frame from ``(value_fn, colour)`` parts in Wh."""
    def build():
        row, cursor = VGroup(), x0
        for fn, colour in parts:
            w = max(0.0, float(fn())) * unit
            if w > 0.004:
                row.add(Rectangle(width=w, height=height, stroke_width=1.2, color=colour, fill_color=colour,
                                  fill_opacity=0.72).move_to([cursor + w / 2, y, 0]))
            cursor += w
        return row
    return always_redraw(build)


#endregion


#region Shared numbers
"""🔢 Laufendes Beispiel — Zahlenquellen: Kapitel 2 (V, n), Kapitel 3 (c_Luft, Δθ_Winter, Δθ_Sommer).

c_Luft = ρ·c_p/3600 = 1,2 kg/m³ · 1005 J/(kg·K) / (3600 J/Wh) ≈ 0,34 Wh/(m³·K); Winter 20 °C innen gegen 0 °C außen (Δθ = 20 K),
Sommer 26 °C innen gegen 32 °C außen am Tag und 18 °C in der Nacht.
"""
RHO_AIR, CP_AIR = 1.2, 1005.0
C_LUFT = round(RHO_AIR * CP_AIR / 3600.0, 2)
D_THETA = 20.0
V_NET, N_RATE = 440.0, 0.5
T_IN_WINTER, T_OUT_WINTER = 20.0, 0.0
T_IN_SUMMER, T_OUT_NIGHT, T_OUT_DAY = 26.0, 18.0, 32.0
G_ACC = 9.81


def _rho_air(theta: float) -> float:
    """⚖️ Dichte trockener Luft bei ``theta`` °C aus dem idealen Gasgesetz, ρ = p/(R·T) mit p = 101325 Pa.

    https://de.wikipedia.org/wiki/Dichte_(Luft)
    """
    return 101325.0 / (287.05 * (273.15 + theta))


CD_OPENING, A_OPENING = 0.6, 0.1


def _stack_chain(height_m: float, t_out: float, t_in: float) -> dict:
    """⬆️ Auftriebskette Δρ(θ) → Δp = Δρ · g · h → v = √(2Δp/ρ) → V̇ = C_d · A · v, in kg/m³, Pa, m/s und m³/s.

    https://de.wikipedia.org/wiki/Kamineffekt
    """
    rho_e, rho_i = _rho_air(t_out), _rho_air(t_in)
    drho = rho_e - rho_i
    dp = height_m * G_ACC * drho
    v = float(np.sqrt(2.0 * dp / RHO_AIR))
    vdot = CD_OPENING * A_OPENING * v
    return {"rho_e": rho_e, "rho_i": rho_i, "drho": drho, "dp": dp, "v": v, "vdot": vdot, "vdot_h": vdot * 3600.0}


#endregion


#region Beat1 – The window is a dial, not a switch
class Beat1_Fensterregeln(Scene):
    NARRATION = [
        ("intro",
         "Natural ventilation is adjustable, in winter as in summer. A single facade opening already gives you a dial, not a switch.",
         "Natürliche Lüftung ist einstellbar, Winter wie Sommer —\neine Fassadenöffnung ist ein Regler, kein Schalter."),
        ("closed",
         "Fully closed, the air change rate drops to almost nothing. Heat and moisture given off by the occupants build up and comfort collapses.",
         "Ganz geschlossen sinkt der Luftwechsel fast auf null —\nWärme und Feuchte stauen sich, der Komfort bricht ein."),
        ("partial",
         "Open a narrow gap and the air change rate climbs into the comfort band: enough fresh air, and still no draught.",
         "Ein schmaler Spalt hebt den Luftwechsel ins Komfortband —\ngenug Frischluft, keine Zugluft."),
        ("cooler",
         "On a summer night the outdoor air is cooler than the room. Open wide: the very same window now flushes heat out of the space.",
         "In der Sommernacht ist die Außenluft kühler als der Raum —\nweit öffnen, das Fenster spült jetzt Wärme hinaus."),
        ("hotter",
         "On a summer afternoon the outdoor air is hotter. Throttle back: a wide opening would import heat instead of removing it.",
         "Am Sommernachmittag ist es draußen heißer — drosseln,\neine weite Öffnung würde Wärme hereinholen."),
        ("winter",
         "Winter air is colder. Open wide but briefly: each minute loses heat, so duration matters as much as opening width.",
         "Im Winter ist die Außenluft immer kälter — weit, aber nur kurz\nöffnen: jede Minute trägt Wärme hinaus."),
        ("rule",
         "In both seasons, set window opening and duration for the occupants and the outdoor temperature.",
         "Die Regel gilt in beiden Jahreszeiten: das Fenster nach den Menschen,\nder Außentemperatur und der Öffnungsdauer einstellen."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The window is a control", "Das Fenster ist ein Regler"))
        din = reference_chip(_references.cite("din-1946-6"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-1.55, 0.15, 0.0])
        shell = _room(room_c, 5.6, 2.5, window_side="left", window_y=0.30, window_h=1.42)
        room, floor = shell[0], shell[1]
        wash = Rectangle(
            width=5.5, height=2.4, stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
        ).move_to(room_c)
        person = _stand(room_c, 2.5, room_c[0] + 0.55)

        win_h = 1.42
        window = window_glyph(room_c[0] - 2.8, 0.30 - win_h / 2, 0.30 + win_h / 2,
                              depth=0.8 * win_h, wall=0.16, color=P_CYAN)
        frame, sash, gap = window["reveal"], window["sash"], window["reference"]
        opening = ValueTracker(0.0)

        def _shape_window(_m=None):
            sash.become(window_opening(window, fraction=opening.get_value()))

        _shape_window()
        sash.add_updater(_shape_window)

        def _open_lbl(text, color):
            return Text(text, font_size=LABEL_FONT_SIZE, color=color).next_to(
                frame, LEFT, buff=0.22, aligned_edge=DOWN)

        open_pct = _open_lbl(tr("Opening 0 %", "Öffnung 0 %"), P_CYAN)
        open_mid = _open_lbl(tr("Opening 35 %", "Öffnung 35 %"), P_CYAN)
        open_wide = _open_lbl(tr("Opening 85 %", "Öffnung 85 %"), P_CYAN)
        open_thin = _open_lbl(tr("Opening 15 %", "Öffnung 15 %"), P_ORANGE)

        outdoor_cool = _badge(tr("Summer night", "Sommernacht"), "18 °C", P_BLUE)
        outdoor_cool.move_to(np.array([-5.60, 1.55, 0.0]))
        outdoor_hot = _badge(tr("Summer day", "Sommertag"), "32 °C", P_RED).move_to(outdoor_cool)
        outdoor_winter = _badge("Winter", "0 °C", P_CYAN).move_to(outdoor_cool)

        def _inside(deg, color):
            tag = math_label(tr(rf"\text{{indoor}}\;{deg}\,\mathrm{{°C}}", rf"\text{{innen}}\;{deg}\,\mathrm{{°C}}"), size=LABEL_FONT_SIZE, color=color)
            return tag.move_to(room_c + np.array([2.0, 0.88, 0.0]))

        inside_summer, inside_winter = _inside(26, P_ORANGE), _inside(20, P_ORANGE)

        scaffold = VGroup(room, floor, wash, person, frame, sash, gap, open_pct, outdoor_cool, inside_summer)
        fit_band(scaffold)

        stuffy_spots = [person.get_top() + UP * 0.05, person.get_top() + LEFT * 1.6 + DOWN * 0.6]

        def stuffy(rt, r_max=0.6, color=P_RED):
            return [ripples(stuffy_spots, r_max=r_max, color=color, cycles=rt / 1.4)]

        air = meter(tr("Air change n", "Luftwechsel n"), length=2.5, thickness=0.55, color=P_CYAN)
        air["group"].move_to(np.array([3.45, 0.15, 0.0]))
        rate = ValueTracker(0.03)
        bind_meter(air, rate)

        track = air["track"]
        band_lo = track.get_bottom()[1] + 0.04 + 0.30 * air["span"]
        band_hi = track.get_bottom()[1] + 0.04 + 0.60 * air["span"]
        comfort = Rectangle(
            width=0.55, height=band_hi - band_lo, stroke_width=0,
            fill_color=P_GREEN, fill_opacity=0.22,
        ).move_to(np.array([track.get_center()[0], (band_lo + band_hi) / 2, 0.0]))
        comfort_lbl = Text(tr("Comfort", "Komfort"), font_size=LABEL_FONT_SIZE, color=P_GREEN)
        comfort_lbl.next_to(comfort, RIGHT, buff=0.18)

        n_low = math_label(tr(r"n \approx 0.2\,\mathrm{h^{-1}}", r"n \approx 0{,}2\,\mathrm{h^{-1}}"), size=BODY_FONT_SIZE, color=P_RED)
        n_low.next_to(track, DOWN, buff=0.26)
        n_mid = math_label(r"n \approx 2\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_GREEN).move_to(n_low)
        n_high = math_label(r"n \approx 6\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_BLUE).move_to(n_low)
        n_thin = math_label(r"n \approx 1\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_ORANGE).move_to(n_low)
        open_short = _open_lbl(tr("Opening 85 %", "Öffnung 85 %"), P_CYAN)
        n_short = math_label(r"n \approx 6\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_BLUE).move_to(n_low)

        tag_flush = Text(tr("flush", "spülen"), font_size=LABEL_FONT_SIZE, color=P_BLUE)
        tag_flush.next_to(track, LEFT, buff=0.22).shift(UP * 0.75)
        tag_throttle = Text(tr("throttle", "drosseln"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(tag_flush)
        tag_short = Text(tr("short", "kurz"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(tag_flush)

        gentle = VGroup(
            smooth_path([
                frame.get_right() + RIGHT * 0.05 + DOWN * 0.25,
                room_c + LEFT * 1.4 + DOWN * 0.55,
                room_c + RIGHT * 0.3 + DOWN * 0.15,
            ]),
            smooth_path([
                room_c + RIGHT * 0.3 + UP * 0.30,
                room_c + LEFT * 1.4 + UP * 0.62,
                frame.get_right() + RIGHT * 0.05 + UP * 0.48,
            ]),
        )
        rule = Text(tr("Window = control: temperature and duration", "Fenster = Regler: Temperatur und Dauer"), font_size=BODY_FONT_SIZE, color=P_YELLOW)
        rule.move_to(np.array([room_c[0], -1.78, 0.0]))

        def draft(rt, speed=0.8, color=P_CYAN, color_end=None, waves=3):
            return [_streams([(gentle, color, color_end)], rt, speed=speed, waves=waves)]

        self.add(wash)
        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.play(FadeIn(frame), FadeIn(sash), FadeIn(gap), FadeIn(open_pct), FadeIn(inside_summer), run_time=0.8)
        self.play(FadeIn(air["group"]), FadeIn(comfort), FadeIn(comfort_lbl), FadeIn(n_low), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.8 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "closed"))
        self.play(wash.animate.set_fill(P_RED, opacity=0.24), *stuffy(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "closed", during=stuffy)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "partial"))
        self.play(
            opening.animate.set_value(0.35),
            rate.animate.set_value(0.45),
            stage.transition("ventilation.natural.window-rules.opening-partial", source=open_pct, target=open_mid),
            stage.transition("ventilation.natural.window-rules.rate-partial", source=n_low, target=n_mid),
            recolor_meter(air, P_GREEN),
            wash.animate.set_fill(P_CYAN, opacity=0.07),
            *stuffy(1.6, 0.35),
            run_time=1.6,
        )
        self.play(Create(flow_guides(gentle, P_CYAN)), run_time=0.6)
        self.play(*draft(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "partial", during=draft)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cooler"))
        self.play(FadeIn(outdoor_cool), *draft(0.6), run_time=0.6)
        self.play(
            opening.animate.set_value(0.85),
            rate.animate.set_value(0.85),
            stage.transition("ventilation.natural.window-rules.opening-wide", source=open_mid, target=open_wide),
            stage.transition("ventilation.natural.window-rules.rate-wide", source=n_mid, target=n_high),
            recolor_meter(air, P_BLUE),
            wash.animate.set_fill(P_BLUE, opacity=0.14),
            FadeIn(tag_flush), *draft(1.5, 1.2, P_BLUE, P_ORANGE, 4),
            run_time=1.5,
        )
        self.play(*draft(2.2, 1.2, P_BLUE, P_ORANGE, 4), run_time=2.2)
        hold_for(self, self.NARRATION, "cooler", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "hotter"))
        self.play(
            stage.transition("ventilation.natural.window-rules.outdoor-hot", source=outdoor_cool, target=outdoor_hot),
            opening.animate.set_value(0.15),
            rate.animate.set_value(0.18),
            stage.transition("ventilation.natural.window-rules.opening-thin", source=open_wide, target=open_thin),
            stage.transition("ventilation.natural.window-rules.rate-thin", source=n_high, target=n_thin),
            recolor_meter(air, P_ORANGE),
            wash.animate.set_fill(P_RED, opacity=0.12),
            stage.transition("ventilation.natural.window-rules.tag-throttle", source=tag_flush, target=tag_throttle),
            *draft(1.6, 0.45, P_ORANGE, None, 2),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "hotter", during=lambda rt: draft(rt, 0.45, P_ORANGE, None, 2))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        minutes = ValueTracker(0.0)
        dial = tracked_clock(np.array([2.15, -0.45, 0.0]), 0.36, minutes, color=P_CYAN)
        dial_read = math_readout(
            lambda: rf"t \approx {label_number(minutes.get_value())}\,\mathrm{{min}}",
            np.array([2.15, -1.02, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="center",
        )
        self.play(
            stage.transition("ventilation.natural.window-rules.outdoor-winter", source=outdoor_hot, target=outdoor_winter),
            stage.transition("ventilation.natural.window-rules.inside-winter", source=inside_summer, target=inside_winter),
            opening.animate.set_value(0.85),
            rate.animate.set_value(0.85),
            stage.transition("ventilation.natural.window-rules.opening-short", source=open_thin, target=open_short),
            stage.transition("ventilation.natural.window-rules.rate-short", source=n_thin, target=n_short),
            recolor_meter(air, P_BLUE),
            wash.animate.set_fill(P_BLUE, opacity=0.14),
            stage.transition("ventilation.natural.window-rules.tag-short", source=tag_throttle, target=tag_short),
            FadeIn(dial["group"]), FadeIn(dial_read),
            *draft(1.5, 1.2, P_BLUE, P_ORANGE, 4),
            run_time=1.5,
        )
        self.play(minutes.animate.set_value(5.0), *draft(2.4, 1.2, P_BLUE, P_ORANGE, 4), run_time=2.4, rate_func=linear)
        hold_for(self, self.NARRATION, "winter", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rule"))
        band_ring = SurroundingRectangle(comfort, color=P_GREEN, buff=0.06, stroke_width=3, corner_radius=0)
        self.play(FadeIn(rule), Create(band_ring), *draft(1.0, 1.2, P_BLUE, P_ORANGE, 4), run_time=1.0)
        hold_for(self, self.NARRATION, "rule", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))
        sash.clear_updaters()
        air["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Purge ventilation versus tilted window
ROOM_A, ROOM_H, ROOM_D = 4.0 * 5.0, 2.5, 5.0
T_STOSS, T_KIPP = 5.0, 60.0
P_RAD, RAD_SHARE = 600.0, 0.5
M_LAIBUNG, C_MASONRY, DT_LAIBUNG = 160.0, 0.84, 5.0


class Beat2_StossOderKipp(Scene):
    """⏱️ Same 50 m³ of fresh air: wide-open window in 5 min versus tilted window in 60 min — and what each costs."""

    NARRATION = [
        ("room",
         r"Winter case: a room of $20\,\mathrm{m^2}$ and $2.5\,\mathrm{m}$ height holds $V = 50\,\mathrm{m^3}$ of air; inside $\theta_i = 20\,\mathrm{°C}$, outside $\theta_e = 0\,\mathrm{°C}$.",
         r"Winterfall: Ein Raum mit $20\,\mathrm{m^2}$ und $2{,}5\,\mathrm{m}$ Höhe enthält $V = 50\,\mathrm{m^3}$ Luft. Innen $\theta_i = 20\,\mathrm{°C}$, außen $\theta_e = 0\,\mathrm{°C}$."),
        ("open",
         "Two ways to air it: the thermostat turned down and the window wide open — or the window tilted.",
         "Zwei Arten zu lüften: Thermostat zu und Fenster weit auf — oder Fenster gekippt."),
        ("five",
         r"After five minutes the open window has exchanged all the air; the tilted one ($n \approx 1\,\mathrm{h^{-1}}$) only $\frac{5\,\mathrm{min}}{60\,\mathrm{min}} \approx 8\,\%$.",
         r"Nach fünf Minuten ist beim offenen Fenster alle Luft getauscht, beim gekippten ($n \approx 1\,\mathrm{h^{-1}}$) erst $\frac{5\,\mathrm{min}}{60\,\mathrm{min}} \approx 8\,\%$."),
        ("stoss",
         r"Purge ventilation only reheats the air: $0.34\,\mathrm{Wh/(m^3K)} \cdot 50\,\mathrm{m^3} \cdot 20\,\mathrm{K} = 340\,\mathrm{Wh}$. The walls stay warm.",
         r"Beim Stoßlüften wird nur die Luft nachgeheizt: $0{,}34\,\mathrm{Wh/(m^3K)} \cdot 50\,\mathrm{m^3} \cdot 20\,\mathrm{K} = 340\,\mathrm{Wh}$. Die Wände bleiben warm."),
        ("hour",
         r"The tilted window needs about an hour for one full air change: $n \approx 1\,\mathrm{h^{-1}}$.",
         r"Das gekippte Fenster braucht für einen vollen Luftwechsel etwa eine Stunde: $n \approx 1\,\mathrm{h^{-1}}$."),
        ("thermostat",
         r"Cold air falls onto the thermostat below the window; the valve opens and half of $600\,\mathrm{W}$ flows straight out: $300\,\mathrm{Wh}$.",
         r"Kalte Luft fällt auf das Thermostat unter dem Fenster. Das Ventil öffnet, die Hälfte von $600\,\mathrm{W}$ geht direkt hinaus: $300\,\mathrm{Wh}$."),
        ("laibung",
         r"Reveal and lintel cool by about $5\,\mathrm{K}$; reheating $160\,\mathrm{kg}$ of masonry costs about $190\,\mathrm{Wh}$.",
         r"Laibung und Sturz kühlen um etwa $5\,\mathrm{K}$ ab. $160\,\mathrm{kg}$ Mauerwerk wieder zu erwärmen kostet rund $190\,\mathrm{Wh}$."),
        ("sum",
         r"For the same $50\,\mathrm{m^3}$ of fresh air: $340\,\mathrm{Wh}$ purge, about $830\,\mathrm{Wh}$ tilted.",
         r"Für dieselben $50\,\mathrm{m^3}$ Frischluft: $340\,\mathrm{Wh}$ beim Stoßlüften, rund $830\,\mathrm{Wh}$ bei Kipplüftung."),
        ("rule",
         "So in winter, open the window wide for a short time instead of tilting it.",
         "Darum im Winter: kurz und weit öffnen statt kippen."),
        ("summer",
         r"Summer: $\Delta\theta = 6\,\mathrm{K}$ gives only about $100\,\mathrm{Wh}$ of air heat, no radiator or reveal. Tilt moves little air: open wide, briefly, in cool hours.",
         r"Im Sommer entfallen Heizkörper und Laibung: bei $\Delta\theta = 6\,\mathrm{K}$ nur etwa $100\,\mathrm{Wh}$."
         "\nGekippt tauscht kaum Luft — weit und kurz, in den kühlen Stunden."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION
        vol = ROOM_A * ROOM_H
        e_air = C_LUFT * vol * D_THETA
        e_rad = P_RAD * (T_KIPP / 60.0) * RAD_SHARE
        e_lb = round(M_LAIBUNG * C_MASONRY * DT_LAIBUNG / 3.6, -1)
        e_kipp = e_air + e_rad + e_lb

        subtitle = section_heading(tr("Burst ventilation or tilted window", "Stoßlüften oder Kipplüftung"))
        din = reference_chip(_references.cite("din-1946-6"))
        source = reference_chip(_references.cite(("maas-1995", "kap8")) + tr(": n = 0.56 … 1.38 1/h", ": n = 0,56 … 1,38 1/h")).next_to(din, DOWN, buff=0.08, aligned_edge=RIGHT)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "room"))
        self.play(FadeIn(caption), run_time=0.3)

        k, fy = 0.85, -0.45
        rs = _vent_room(-5.9, fy, k, tr(r"\text{Burst ventilation}", r"\text{Stoßlüften}"), P_CYAN)
        rk = _vent_room(1.3, fy, k, tr(r"\text{Tilted window}", r"\text{Kipplüftung}"), P_ORANGE)
        note = math_label(rf"\text{{Winter}}\quad V = 4\,\mathrm{{m}} \cdot {label_number(ROOM_D)}\,\mathrm{{m}} \cdot {label_number(ROOM_H, 1)}\,\mathrm{{m}} = "
                          rf"{label_number(vol)}\,\mathrm{{m^{{3}}}} \quad\quad \Delta\theta = {label_number(D_THETA)}\,\mathrm{{K}}",
                          size=LABEL_FONT_SIZE, color=P_WHITE).move_to([-0.3, 2.45, 0])
        t_s, t_k = ValueTracker(0.0), ValueTracker(0.0)
        valve_s, valve_k = ValueTracker(0.6), ValueTracker(0.6)
        cool = ValueTracker(0.0)
        clocks = {}
        for room, clk, col in ((rs, t_s, P_CYAN), (rk, t_k, P_ORANGE)):
            clocks[id(room)] = tracked_clock([room["ox"] + 0.64, room["ceil"] - 0.5, 0], 0.32, clk, color=col)
        reads = VGroup(*[
            math_readout(lambda clk=clk: rf"t = {label_number(clk.get_value())}\,\mathrm{{min}}",
                         np.array([room["ox"] + 0.64, room["ceil"] - 1.12, 0]), size=LABEL_FONT_SIZE, color=col, edge="center")
            for room, clk, col in ((rs, t_s, P_CYAN), (rk, t_k, P_ORANGE))
        ])
        frac_s = lambda: min(1.0, t_s.get_value() / T_STOSS)
        frac_k = lambda: min(1.0, t_k.get_value() / T_KIPP)
        gauges = VGroup(*[_vbar(r["ox"] - 0.5, fy, r["ceil"] - fy, f, color=P_CYAN) for r, f in ((rs, frac_s), (rk, frac_k))])
        pct = VGroup(*[
            math_readout(lambda f=f: rf"{label_number(100 * f())}\,\%", np.array([room["ox"] - 0.5, room["ceil"] + 0.24, 0]),
                         size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
            for room, f in ((rs, frac_s), (rk, frac_k))
        ])
        dials = VGroup(_valve_dial(rs, valve_s.get_value), _valve_dial(rk, valve_k.get_value))
        flows, state = {}, {"air": [], "heat": [rs, rk]}

        def running(rt):
            streams = [st for key in state["air"] for st in flows[key]]
            heat = [ripples([r["rad"][0].get_top() + UP * 0.03 for r in state["heat"]], r_max=0.3, rings=2,
                            cycles=rt / 1.3, color=P_RED)] if state["heat"] else []
            return ([_streams(streams, rt, speed=0.55, waves=5)] if streams else []) + heat

        self.play(FadeIn(rs["group"]), FadeIn(rk["group"]), FadeIn(rs["head"]), FadeIn(rk["head"]), FadeIn(note),
                  *running(1.4), run_time=1.4)
        self.play(*[FadeIn(c["group"]) for c in clocks.values()], FadeIn(reads), FadeIn(gauges), FadeIn(pct),
                  FadeIn(dials), *running(0.8), run_time=0.8)
        hold_for(self, N, "room", used=2.2, during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "open"))
        state["heat"] = [rk]
        self.play(valve_s.animate.set_value(0.0), *_power(rs["rad"], False), *running(0.8), run_time=0.8)
        self.play(_open_sash(rs["window"]), open_window(rk["window"], fraction=0.17, tilt=True), *running(0.9),
                  run_time=0.9)
        hold_for(self, N, "open", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "five"))
        unit, bx = 8.0 / 830.0, -4.3
        yr_s, yr_k = -1.2, -1.88
        row_s = _energy_row(bx, yr_s, unit, [(lambda: e_air * frac_s(), P_CYAN)])
        row_k = _energy_row(bx, yr_k, unit, [
            (lambda: e_air * frac_k(), P_CYAN),
            (lambda: P_RAD * RAD_SHARE * t_k.get_value() / 60.0, P_RED),
            (lambda: e_lb * cool.get_value(), P_YELLOW),
        ])
        row_labels = VGroup(
            Text(tr("Burst ventilation", "Stoßlüften"), font_size=LABEL_FONT_SIZE, color=P_CYAN),
            Text(tr("Tilted window", "Kipplüftung"), font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        )
        for lbl, y in zip(row_labels, (yr_s, yr_k)):
            lbl.move_to([bx - 0.18 - lbl.width / 2, y, 0])

        def total_k():
            return e_air * frac_k() + P_RAD * RAD_SHARE * t_k.get_value() / 60.0 + \
                e_lb * cool.get_value()

        tot_s = math_readout(lambda: rf"{label_number(e_air * frac_s())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + e_air * frac_s() * unit + 0.15, yr_s - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=P_CYAN)
        tot_k = math_readout(lambda: rf"{label_number(total_k())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + total_k() * unit + 0.15, yr_k - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=P_ORANGE)
        n_kipp_box = window_chip(VGroup(
            math_label(rf"n_{{\mathrm{{{tr('tilt', 'Kipp')}}}}} \approx 1\,\mathrm{{h^{{-1}}}}", size=LABEL_FONT_SIZE, color=P_ORANGE),
            math_label(r"\frac{5\,\mathrm{min}}{60\,\mathrm{min}} \approx 8\,\%", size=LABEL_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.12), color=P_ORANGE).move_to([-0.55, 1.05, 0])
        self.play(FadeIn(row_labels), FadeIn(source), *running(0.4), run_time=0.4)
        self.add(row_s, row_k, tot_s, tot_k)

        def ext(room, x_off, y):
            return np.array([room["x_out"] + x_off, y, 0])

        def inn(room, x_off, y):
            return np.array([room["x_in"] - x_off, y, 0])

        mid_s = (rs["x_in"] + rs["x_out"]) / 2
        stoss_paths_out = [smooth_path([inn(rs, 1.4, y - 0.15), np.array([mid_s, y, 0]), ext(rs, 0.9, y + 0.3)])
                           for y in (0.98, 1.12, 1.26)]
        stoss_paths_in = [smooth_path([ext(rs, 0.9, y - 0.1), np.array([mid_s, y, 0]), inn(rs, 0.6, y - 0.15),
                                       inn(rs, 1.5, fy + 0.75), inn(rs, 2.4, fy + 0.15)]) for y in (0.52, 0.66, 0.8)]
        mid_k = (rk["x_in"] + rk["x_out"]) / 2
        gap_y = rk["lintel"] - 0.06
        kipp_out = [smooth_path([inn(rk, 0.9, gap_y - 0.25), np.array([mid_k, gap_y, 0]), ext(rk, 0.8, gap_y + 0.3)])]
        kipp_in = [smooth_path([ext(rk, 0.7, gap_y - 0.1), np.array([mid_k + 0.03, gap_y - 0.02, 0]),
                                inn(rk, 0.12, gap_y - 0.25), inn(rk, 0.1, rk["valve"].get_center()[1] + 0.1)])]
        heat_out = [smooth_path([np.array([rk["rad"].get_center()[0], rk["rad"][0].get_top()[1] + 0.05, 0]),
                                 inn(rk, 0.35, rk["sill"] + 0.45), np.array([mid_k, gap_y + 0.01, 0]),
                                 ext(rk, 0.8, gap_y + 0.38)])]
        flows.update({"s": [(stoss_paths_out, P_ORANGE), (stoss_paths_in, P_BLUE)],
                      "k": [(kipp_out, P_ORANGE), (kipp_in, P_BLUE)], "h": [(heat_out, P_RED)]})
        state["air"] = ["s", "k"]
        self.play(t_s.animate.set_value(T_STOSS), t_k.animate.set_value(T_STOSS), valve_k.animate.set_value(0.75),
                  cool.animate.set_value(T_STOSS / T_KIPP), FadeIn(n_kipp_box), *running(4.0), run_time=4.0, rate_func=linear)
        hold_for(self, N, "five", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "stoss"))
        air_s_lbl = math_label(tr(rf"\text{{Air}}\; {label_number(e_air)}\,\mathrm{{Wh}}", rf"\text{{Luft}}\; {label_number(e_air)}\,\mathrm{{Wh}}"), size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        air_s_lbl.move_to([bx + e_air * unit / 2, yr_s, 0])
        stoss_eq = math_label(
            rf"Q = c_{{\mathrm{{Luft}}}} \cdot V \cdot \Delta\theta = {label_number(C_LUFT, 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}\,K}}}} \cdot {label_number(vol)}\,\mathrm{{m^{{3}}}} \cdot {label_number(D_THETA)}\,\mathrm{{K}} = {label_number(e_air)}\,\mathrm{{Wh}}",
            size=LABEL_FONT_SIZE, color=P_CYAN)
        stoss_eq.scale_to_fit_width(min(stoss_eq.width, 6.2)).move_to([3.35, yr_s + 0.05, 0])
        state["air"] = ["k"]
        self.play(_open_sash(rs["window"], run_time=1.0, close=True), valve_s.animate.set_value(0.6),
                  *_power(rs["rad"], True), FadeIn(air_s_lbl), FadeIn(stoss_eq), FadeOut(n_kipp_box), *running(1.0), run_time=1.0)
        state["heat"] = [rs, rk]
        hold_for(self, N, "stoss", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))

        def laibung_color():
            return interpolate_color(ManimColor(P_ORANGE), ManimColor(P_BLUE), float(cool.get_value()))

        rk["sturz"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.7))
        rk["laibung"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.3))
        state["air"] = ["k", "h"]
        self.play(t_k.animate.set_value(T_KIPP), valve_k.animate.set_value(1.0), cool.animate.set_value(1.0),
                  *running(6.0), run_time=6.0, rate_func=linear)
        hold_for(self, N, "hour", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "thermostat"))
        rad_lbl = math_label(rf"\dot{{Q}} = {label_number(P_RAD)}\,\mathrm{{W}} \cdot {label_number(100 * RAD_SHARE)}\,\%",
                             np.array([rk["ox"] + 0.15, fy + 0.2, 0]), size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        seg_rad = math_label(tr(rf"\text{{Radiator}}\; {label_number(e_rad)}\,\mathrm{{Wh}}", rf"\text{{Heizkörper}}\; {label_number(e_rad)}\,\mathrm{{Wh}}"), size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_rad.move_to([bx + (e_air + e_rad / 2) * unit, yr_k, 0])
        seg_air = math_label(tr(rf"\text{{Air}}\; {label_number(e_air)}\,\mathrm{{Wh}}", rf"\text{{Luft}}\; {label_number(e_air)}\,\mathrm{{Wh}}"), size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_air.move_to([bx + e_air * unit / 2, yr_k, 0])
        self.play(Indicate(rk["valve"], color=P_YELLOW, scale_factor=1.8), FadeIn(rad_lbl), FadeIn(seg_air),
                  FadeIn(seg_rad), *running(1.0), run_time=1.0)
        hold_for(self, N, "thermostat", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "laibung"))
        lb_lbl = math_label(rf"\Delta\theta = {label_number(DT_LAIBUNG)}\,\mathrm{{K}}", np.array([rk["x_out"] + 0.2, rk["sill"] + 0.05, 0]),
                            size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        seg_lb = math_label(tr(rf"\text{{Reveal}}\; {label_number(e_lb)}\,\mathrm{{Wh}}", rf"\text{{Laibung}}\; {label_number(e_lb)}\,\mathrm{{Wh}}"), size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_lb.scale_to_fit_width(min(seg_lb.width, e_lb * unit - 0.12)).move_to([bx + (e_air + e_rad + e_lb / 2) * unit, yr_k, 0])
        self.play(Indicate(VGroup(rk["sturz"], rk["laibung"]), color=P_BLUE, scale_factor=1.05), FadeIn(lb_lbl),
                  FadeIn(seg_lb), *running(1.0), run_time=1.0)
        hold_for(self, N, "laibung", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        tot_k.clear_updaters()
        tot_k_final = math_label(rf"\approx {label_number(round(e_kipp, -1))}\,\mathrm{{Wh}}", size=LABEL_FONT_SIZE, color=P_ORANGE)
        tot_k_final.move_to(tot_k.get_left() + RIGHT * tot_k_final.width / 2)
        ratio = math_label(rf"\approx {label_number(e_kipp / e_air, 1)} \times", size=BODY_FONT_SIZE, color=P_ORANGE)
        ratio.next_to(tot_k_final, RIGHT, buff=0.3)
        self.play(stage.transition("ventilation.natural.purge-or-tilt.tilted-total-final", source=tot_k, target=tot_k_final), *running(0.6), run_time=0.6)
        self.play(stage.transition("ventilation.natural.purge-or-tilt.ratio-from-purge-row", source=row_s, target=ratio), *running(0.8), run_time=0.8)
        hold_for(self, N, "sum", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "rule"))
        rule = Text(tr("open wide briefly instead of tilting", "kurz und weit öffnen statt kippen"), font_size=BODY_FONT_SIZE, color=P_GREEN)
        rule_box = SurroundingRectangle(rule, color=P_GREEN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=_theme_module.CHROME_BORDER)
        VGroup(rule, rule_box).move_to([3.3, yr_s, 0])
        self.play(FadeOut(rad_lbl), FadeOut(stoss_eq), FadeIn(rule), Create(rule_box), *running(0.8), run_time=0.8)
        hold_for(self, N, "rule", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        e_sum = C_LUFT * vol * (T_OUT_DAY - T_IN_SUMMER)
        winter_rows = VGroup(row_s, row_k, tot_s, tot_k_final, ratio, air_s_lbl, seg_air, seg_rad, seg_lb,
                             row_labels, lb_lbl, source)
        sun =sun_glyph(np.array([bx - 0.9, yr_s - 0.3, 0.0])).scale(0.32)
        sum_bar = Rectangle(width=e_sum * unit, height=0.5, stroke_width=1.2, color=P_YELLOW, fill_color=P_YELLOW,
                            fill_opacity=0.72).move_to([bx + e_sum * unit / 2, yr_s - 0.3, 0])
        sum_head = math_label(tr(r"\text{Summer}\quad \Delta\theta = 6\,\mathrm{K}", r"\text{Sommer}\quad \Delta\theta = 6\,\mathrm{K}"), size=LABEL_FONT_SIZE, color=P_YELLOW)
        sum_head.move_to([bx + sum_head.width / 2, yr_s + 0.28, 0])
        sum_val = math_label(rf"{label_number(C_LUFT, 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}\,K}}}} \cdot {label_number(vol)}\,\mathrm{{m^{{3}}}} \cdot {label_number(T_OUT_DAY - T_IN_SUMMER)}\,\mathrm{{K}} \approx {label_number(round(e_sum, -1))}\,\mathrm{{Wh}}",
                             size=LABEL_FONT_SIZE, color=P_YELLOW)
        sum_val.next_to(sum_bar, RIGHT, buff=0.2)
        self.play(FadeOut(winter_rows), *running(0.6), run_time=0.6)
        self.play(FadeIn(sun), GrowFromEdge(sum_bar, LEFT), FadeIn(sum_head), FadeIn(sum_val), *running(1.2), run_time=1.2)
        hold_for(self, N, "summer", during=running)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Cross ventilation: pressure difference and both openings
class Beat3_Querlueftung(Scene):
    NARRATION = [
        ("intro",
         "For a stronger flow that is still entirely natural, open two opposite facades.",
         "Für stärkeren, weiterhin natürlichen Luftstrom:\nzwei gegenüberliegende Fassaden öffnen."),
        ("pressure",
         "Wind presses on the windward face and pulls on the leeward face. That pressure difference, not a fan, is what drives air through the room.",
         "Wind drückt auf der Luvseite und saugt auf der Leeseite —\ndiese Druckdifferenz treibt die Luft, kein Ventilator."),
        ("formula",
         r"How much air gets through is set by an effective area $A_{eff}$ that combines both openings.",
         r"Wie viel Luft durchkommt, bestimmt eine wirksame Fläche $A_{eff}$ aus beiden Öffnungen."),
        ("a1",
         r"$A_1$ is the inlet opening on the windward facade.",
         r"$A_1$ ist die Zuluftöffnung auf der Luvseite."),
        ("a2",
         r"$A_2$ is the outlet opening on the leeward facade.",
         r"$A_2$ ist die Abluftöffnung auf der Leeseite."),
        ("limit",
         "The smaller series opening governs. Shrinking the outlet chokes cross flow even with the inlet wide open.",
         "Weil beide in Reihe wirken, bestimmt die kleinere Öffnung:\nverkleinert man den Auslass, bricht der Durchzug ein."),
        ("summer",
         "So size both sides together. In summer this flush dumps a lot of heat on windy nights, but it does not replace a design cooling plant by itself.",
         "Also beide Seiten gemeinsam dimensionieren. Im Sommer führt die\nSpülung nachts viel Wärme ab, ersetzt aber keine Kälteanlage."),
        ("winter",
         "Winter wind pressure draws cold air through and quickly loses heat: ventilate briefly, then close the openings.",
         "Im Winter zieht derselbe Winddruck kalte Luft durch den Raum —\nDurchzug kostet schnell Wärme: kurz lüften, danach schließen."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Cross ventilation: both openings count", "Querlüftung: beide Öffnungen zählen"))
        din = reference_chip(_references.cite("din-en-16798-7"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([0.0, 0.55, 0.0])
        shell = _room(room_c, 6.4, 2.2, window_side="both", window_y=0.55)
        room, floor = shell[0], shell[1]
        left_win = _window(np.array([room_c[0] - 3.2, 0.55, 0.0]), height=1.15)
        right_win = _window(np.array([room_c[0] + 3.2, 0.55, 0.0]), height=1.15, inward=-1)

        a1_lbl = math_label(r"A_{1}", size=BODY_FONT_SIZE, color=P_GREEN).next_to(left_win, LEFT, buff=0.18)
        a2_lbl = math_label(r"A_{2}", size=BODY_FONT_SIZE, color=P_ORANGE).next_to(right_win, RIGHT, buff=0.18)

        gusts = VGroup(*[
            Arrow(
                np.array([-5.75, y, 0.0]), np.array([-4.45, y, 0.0]),
                buff=0, color=P_CYAN, stroke_width=4,
                max_tip_length_to_length_ratio=0.16,
            )
            for y in (0.55, 1.05, 1.55)
        ])
        wind_lbl = Text("Wind", font_size=BODY_FONT_SIZE, color=P_CYAN).next_to(gusts, UP, buff=0.16)

        luv = _badge(tr("Windward", "Luv"), tr("+ overpressure", "+ Überdruck"), P_CYAN)
        luv.move_to(np.array([-5.05, -0.05, 0.0]))
        lee = _badge(tr("Leeward", "Lee"), tr("− underpressure", "− Unterdruck"), P_ORANGE)
        lee.move_to(np.array([5.05, -0.05, 0.0]))

        aeff_wide = math_label(tr(r"A_{eff} \approx 0.35\,\mathrm{m^{2}}", r"A_{eff} \approx 0{,}35\,\mathrm{m^{2}}"), size=BODY_FONT_SIZE, color=P_CYAN)
        aeff_wide.move_to(np.array([0.0, 2.12, 0.0]))
        aeff_thin = math_label(tr(r"A_{eff} \approx 0.10\,\mathrm{m^{2}}", r"A_{eff} \approx 0{,}10\,\mathrm{m^{2}}"), size=BODY_FONT_SIZE, color=P_RED).move_to(aeff_wide)
        tip = Text(tr("Summer: purge before the chiller", "Sommer: Spülung vor der Kältemaschine"), font_size=BODY_FONT_SIZE, color=P_YELLOW)
        tip.move_to(aeff_wide)
        tip_winter = Text(tr("Winter: ventilate briefly, then close", "Winter: kurz lüften, danach schließen"), font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(aeff_wide)
        wash = Rectangle(
            width=6.3, height=2.1, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.0,
        ).move_to(room_c)

        scaffold = VGroup(room, floor, left_win, right_win, a1_lbl, a2_lbl,
                          gusts, wind_lbl, luv, lee, aeff_wide)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        left_x, right_x = room.get_left()[0], room.get_right()[0]

        def _cross(rows):
            return VGroup(*[
                smooth_path([
                    np.array([left_x, y0, 0.0]),
                    np.array([-1.5, y1, 0.0]),
                    np.array([1.5, y2, 0.0]),
                    np.array([2.5, y3, 0.0]),
                    np.array([right_x, y4, 0.0]),
                ])
                for y0, y1, y2, y3, y4 in rows
            ])

        cross = _cross([(0.92, 0.92, 0.92, 0.92, 0.92), (0.56, 0.56, 0.56, 0.56, 0.56), (0.20, 0.20, 0.20, 0.20, 0.20)])
        cross_thin = _cross([(0.92, 0.92, 0.88, 0.76, 0.64), (0.56, 0.56, 0.56, 0.56, 0.56), (0.20, 0.20, 0.24, 0.36, 0.46)])

        eq, eq_box, items = math_panel([
            ("aeff", r"A_{eff}", P_CYAN), (None, "=", P_WHITE),
            ("root",
             rf"\frac{{1}}{{\sqrt{{\textcolor{{{P_GREEN}}}{{\frac{{1}}{{A_{{1}}^{{2}}}}}} + "
             rf"\textcolor{{{P_ORANGE}}}{{\frac{{1}}{{A_{{2}}^{{2}}}}}}}}}}",
             P_WHITE),
            (None, r"\quad A_{eff}\;\text{in}\;\mathrm{m^{2}}", P_PRIMARY),
        ], color=P_CYAN)
        _inner = items["root"][0][2][0][1]
        items["a1"], items["a2"] = _inner[0], _inner[2]

        self.play(Create(room), Create(floor), run_time=1.3)
        self.play(FadeIn(left_win), FadeIn(right_win), FadeIn(a1_lbl), FadeIn(a2_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "pressure"))
        self.play(
            LaggedStart(*[GrowArrow(a) for a in gusts], lag_ratio=0.18),
            FadeIn(wind_lbl), FadeIn(luv), FadeIn(lee),
            run_time=1.5,
        )
        def through(rt, paths=cross, color=P_GREEN, speed=0.85, waves=4, end=None):
            return [_streams([(paths, color, end)], rt, speed=speed, waves=waves)]

        guides = flow_guides(cross, P_GREEN)
        self.play(Create(guides), run_time=0.7)
        self.play(*through(2.8), run_time=2.8)
        hold_for(self, self.NARRATION, "pressure", during=through)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        rest = VGroup(*[
            m for m in eq.submobjects
            if m is not items["a1"] and m is not items["a2"]
        ])
        self.play(Create(eq_box), FadeIn(rest), FadeIn(aeff_wide), *through(0.9), run_time=0.9)
        self.play(
            stage.transition("ventilation.natural.cross-ventilation.a1-into-formula", source=a1_lbl, target=items["a1"]),
            stage.transition("ventilation.natural.cross-ventilation.a2-into-formula", source=a2_lbl, target=items["a2"]),
            *through(1.2),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "formula", during=through)

        for key, win, color in (("a1", left_win, P_GREEN), ("a2", right_win, P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), Indicate(win, color=color, scale_factor=1.2), *through(0.55), run_time=0.55)
            hold_for(self, self.NARRATION, key, during=through)
            self.play(FadeOut(ring), *through(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "limit"))
        a2_ring = highlight_param(items, "a2", color=P_RED)
        guides_thin = flow_guides(cross_thin, P_RED)
        self.play(
            open_window(right_win.window, fraction=0.26),
            stage.transition("ventilation.natural.cross-ventilation.aeff-thin", source=aeff_wide, target=aeff_thin),
            stage.transition("ventilation.natural.cross-ventilation.guides-thin", source=guides, target=guides_thin),
            Create(a2_ring),
            run_time=1.3,
        )
        self.play(*through(2.6, cross_thin, P_RED, 0.3, 2), run_time=2.6)
        guides = flow_guides(cross, P_GREEN)
        self.play(
            open_window(right_win.window),
            stage.transition("ventilation.natural.cross-ventilation.aeff-wide-again", source=aeff_thin, target=aeff_wide),
            stage.transition("ventilation.natural.cross-ventilation.guides-wide-again", source=guides_thin, target=guides),
            FadeOut(a2_ring),
            run_time=1.1,
        )
        self.play(*through(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "limit", during=through)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summer"))
        self.play(stage.transition("ventilation.natural.cross-ventilation.tip-summer", source=aeff_wide, target=tip), *through(0.9, color=P_BLUE, end=P_ORANGE), run_time=0.9)
        hold_for(self, self.NARRATION, "summer", during=lambda rt: through(rt, color=P_BLUE, end=P_ORANGE))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.add(wash)
        wash.set_z_index(-1)
        self.play(stage.transition("ventilation.natural.cross-ventilation.tip-winter", source=tip, target=tip_winter), wash.animate.set_fill(P_BLUE, opacity=0.14),
                  *through(1.0, speed=1.2, color=P_BLUE, end=P_ORANGE), run_time=1.0)
        self.play(*through(2.0, speed=1.2, color=P_BLUE, end=P_ORANGE), run_time=2.0)
        hold_for(self, self.NARRATION, "winter", during=lambda rt: through(rt, speed=1.2, color=P_BLUE, end=P_ORANGE))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Stack effect: height makes the pressure
S_LOW = _stack_chain(2.5, T_OUT_NIGHT, T_IN_SUMMER)
S_TALL = _stack_chain(4.4, T_OUT_NIGHT, T_IN_SUMMER)
S_WINTER = _stack_chain(4.4, T_OUT_WINTER, T_IN_WINTER)


class Beat4_Auftrieb(Scene):
    NARRATION = [
        ("intro",
         "When the wind drops, buoyancy remains: warm air is lighter, rises and leaves through a high opening; cool air enters through a low one.",
         "Fällt der Wind aus, bleibt der Auftrieb: warme Luft ist leichter, steigt und geht durch eine hohe Öffnung, kühle strömt durch eine tiefe nach."),
        ("shaft",
         "A stairwell or a chimney-like shaft carries the warm air up to the high outlet opening, so the flow keeps running even on a still day.",
         "Treppenhaus oder Schacht (Kamin) führt die warme Luft zur hohen Abluftöffnung — auch an windstillen Tagen."),
        ("density",
         r"Why does it rise? Warm air is lighter: the density $\rho(\theta)$ falls as the temperature $\theta$ rises.",
         r"Warum steigt sie? Warme Luft ist leichter: die Dichte $\rho(\theta)$ sinkt mit steigender Temperatur $\theta$."),
        ("drho",
         r"Outside ($\theta_e = 18\,\mathrm{°C}$) air has $\rho = 1.212\,\mathrm{kg/m^3}$, inside ($\theta_i = 26\,\mathrm{°C}$) $1.180\,\mathrm{kg/m^3}$: $\Delta\rho = 0.032\,\mathrm{kg/m^3}$.",
         r"Außen ($\theta_e = 18\,\mathrm{°C}$) hat Luft $\rho = 1{,}212\,\mathrm{kg/m^3}$, innen ($\theta_i = 26\,\mathrm{°C}$) $1{,}180\,\mathrm{kg/m^3}$: $\Delta\rho = 0{,}032\,\mathrm{kg/m^3}$."),
        ("pressure",
         r"Over the height $h$ between inlet and outlet this becomes a pressure: $\Delta p = \Delta\rho \cdot g \cdot h = 0.8\,\mathrm{Pa}$ for $h = 2.5\,\mathrm{m}$.",
         r"Über die Höhe $h$ zwischen Zu- und Abluftöffnung wird daraus ein Druck: $\Delta p = \Delta\rho \cdot g \cdot h = 0{,}8\,\mathrm{Pa}$ bei $h = 2{,}5\,\mathrm{m}$."),
        ("speed",
         r"The pressure accelerates the air: $v = \sqrt{2\,\Delta p/\rho}$ gives $1.15\,\mathrm{m/s}$ with $\rho \approx 1.2\,\mathrm{kg/m^3}$ (20 °C).",
         r"Der Druck beschleunigt die Luft: $v = \sqrt{2\,\Delta p/\rho}$ ergibt $1{,}15\,\mathrm{m/s}$ mit $\rho \approx 1{,}2\,\mathrm{kg/m^3}$ (20 °C)."),
        ("flow",
         r"Speed times open area is the flow: $\dot{V} = C_d \cdot A \cdot v = 0.069\,\mathrm{m^3/s} \approx 250\,\mathrm{m^3/h}$ for $C_d \approx 0.6$ and $A = 0.1\,\mathrm{m^2}$.",
         r"Geschwindigkeit mal Öffnungsfläche ist der Volumenstrom: $\dot{V} = C_d \cdot A \cdot v = 0{,}069\,\mathrm{m^3/s} \approx 250\,\mathrm{m^3/h}$ bei $C_d \approx 0{,}6$ und $A = 0{,}1\,\mathrm{m^2}$."),
        ("taller",
         r"Raise the outlet and $h$ grows: the shaft lifts it from $2.5\,\mathrm{m}$ to $4.4\,\mathrm{m}$, so $\Delta p$ rises to $1.4\,\mathrm{Pa}$ and the flow speeds up.",
         r"Höherer Auslass, größeres $h$: der Schacht hebt es von $2{,}5\,\mathrm{m}$ auf $4{,}4\,\mathrm{m}$, $\Delta p$ steigt auf $1{,}4\,\mathrm{Pa}$, der Strom wird schneller."),
        ("winter",
         r"Winter: $\Delta\theta = 20\,\mathrm{K}$ instead of $8\,\mathrm{K}$ gives the same shaft $3.8\,\mathrm{Pa}$, about $2.7$ times the pressure. Buoyancy is strongest in the cold.",
         r"Im Winter: $\Delta\theta = 20\,\mathrm{K}$ statt $8\,\mathrm{K}$ ergibt im selben Schacht $3{,}8\,\mathrm{Pa}$, das $2{,}7$-fache. Bei Kälte wirkt der Auftrieb am stärksten."),
        ("comfort",
         "That is why inlets sit low, near the people, and outlets sit high at the roof: comfort without a fan.",
         "Deshalb sitzen Zuluftöffnungen tief bei den Menschen und Abluftöffnungen hoch am Dach — Komfort ohne Ventilator."),
    ]

    SHAFT_X = -1.60
    SHAFT_HW = 0.42
    TOP_LOW = 0.75
    TOP_HIGH = 2.05

    def _stack_paths(self, inlet, room_top, shaft_top):
        """〰️ Inlet → occupied zone → up the shaft → out at the top."""
        return VGroup(*[
            smooth_path([
                inlet + RIGHT * 0.06 + UP * (0.06 * i),
                np.array([-3.20, -0.62 + 0.10 * i, 0.0]),
                np.array([self.SHAFT_X, room_top + 0.10, 0.0]),
                np.array([self.SHAFT_X, shaft_top - 0.08, 0.0]),
            ])
            for i in range(3)
        ])

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Stack effect: height creates pressure", "Auftrieb: Höhe erzeugt Druck"))
        din = reference_chip(_references.cite("din-en-16798-7"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-2.90, -0.45, 0.0])
        shell = _room(room_c, 4.2, 1.7, window_side="left", window_y=-0.85, window_h=0.60)
        room, floor = shell[0], shell[1]
        room_top = room.get_top()[1]
        inlet = Rectangle(width=0.16, height=0.60, color=P_CYAN, stroke_width=2.5, fill_opacity=0.0)
        inlet.move_to(np.array([room_c[0] - 2.1, -0.85, 0.0]))
        inlet_arrow = Arrow(inlet.get_center() + LEFT * 0.95, inlet.get_center() + LEFT * 0.12, buff=0, color=P_CYAN,
                            stroke_width=4, max_tip_length_to_length_ratio=0.28)
        inlet_lbl = Text(tr("Inlet opening", "Zuluftöffnung"), font_size=LABEL_FONT_SIZE, color=P_CYAN)
        inlet_lbl.next_to(inlet_arrow, UP, buff=0.12).align_to(inlet_arrow.get_left() + LEFT * 0.9, LEFT)
        person = _stand(room_c, 1.7, -1.75)

        shaft_walls = VGroup(*[
            Line(
                np.array([self.SHAFT_X + s * self.SHAFT_HW, room_top, 0.0]),
                np.array([self.SHAFT_X + s * self.SHAFT_HW, self.TOP_LOW, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            )
            for s in (-1, 1)
        ])
        ceiling_gap = Rectangle(width=2 * self.SHAFT_HW - 0.07, height=0.18, stroke_width=0, fill_color=P_DEEP_DARK,
                                fill_opacity=1.0).move_to(np.array([self.SHAFT_X, room_top - 0.08, 0.0]))
        outlet = DashedLine(
            np.array([self.SHAFT_X - self.SHAFT_HW, self.TOP_LOW, 0.0]),
            np.array([self.SHAFT_X + self.SHAFT_HW, self.TOP_LOW, 0.0]),
            color=P_ORANGE, stroke_width=3.0, dash_length=0.09,
        )
        outlet_arrow = Arrow(outlet.get_center() + UP * 0.05, outlet.get_center() + UP * 0.62, buff=0, color=P_ORANGE,
                             stroke_width=4, max_tip_length_to_length_ratio=0.3)
        outlet_lbl = Text(tr("Outlet opening", "Abluftöffnung"), font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        outlet_lbl.next_to(outlet, LEFT, buff=0.25)
        outlet_group = VGroup(outlet, outlet_arrow, outlet_lbl)

        def _legend(season, season_color, t_out, t_in):
            def line(sym, t, rho, tag, c):
                return VGroup(
                    math_label(rf"\rho(\theta_{{{sym}}}) = {label_number(rho, 3)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}}", size=LABEL_FONT_SIZE, color=c),
                    VGroup(math_label(rf"\theta_{{{sym}}} = {label_number(t)}\,\mathrm{{°C}}", size=LABEL_FONT_SIZE, color=P_WHITE),
                           Text(tag, font_size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.16),
                ).arrange(DOWN, aligned_edge=LEFT, buff=0.06)
            rows = VGroup(
                Text(season, font_size=LABEL_FONT_SIZE, color=season_color),
                line("e", t_out, _rho_air(t_out), tr("outside · cool · heavy", "außen · kühl · schwer"), P_BLUE),
                line("i", t_in, _rho_air(t_in), tr("inside · warm · light", "innen · warm · leicht"), P_ORANGE),
            ).arrange(DOWN, aligned_edge=LEFT, buff=0.14)
            box = SurroundingRectangle(rows, color=P_PRIMARY, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER)
            return VGroup(box, rows).move_to(np.array([2.35, 1.20, 0.0]))

        legend = _legend(tr("Summer night", "Sommernacht"), P_BLUE, T_OUT_NIGHT, T_IN_SUMMER)
        legend_winter = _legend("Winter", P_CYAN, T_OUT_WINTER, T_IN_WINTER)
        legend_rows = legend[1]

        scaffold = VGroup(room, floor, inlet, inlet_arrow, inlet_lbl, person, shaft_walls, outlet_group, legend)
        fit_band(scaffold, bottom=-0.98)

        warm_spots = [person.get_top() + UP * 0.05, np.array([-2.30, floor.get_top()[1] + 0.02, 0.0])]
        stack = {"paths": None, "speed": 0.65}

        def warmth(rt, r_max=0.55):
            return [ripples(warm_spots, r_max=r_max, color=P_ORANGE, cycles=rt / 1.4)]

        def rising(rt):
            return [_streams([(stack["paths"], P_CYAN, P_ORANGE)], rt, speed=stack["speed"]), *warmth(rt, 0.4)]

        height, t_out_tr, t_in_tr = ValueTracker(2.5), ValueTracker(T_OUT_NIGHT), ValueTracker(T_IN_SUMMER)

        def chain():
            return _stack_chain(height.get_value(), t_out_tr.get_value(), t_in_tr.get_value())

        x_read = 4.45
        out_dp = math_readout(lambda: rf"\Delta p = {label_number(chain()['dp'], 1)}\,\mathrm{{Pa}}",
                              np.array([x_read, 1.80, 0.0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        out_v = math_readout(lambda: rf"v = {label_number(chain()['v'], 2)}\,\frac{{\mathrm{{m}}}}{{\mathrm{{s}}}}",
                             np.array([x_read, 1.20, 0.0]), size=BODY_FONT_SIZE, color=P_CYAN)
        out_q = math_readout(lambda: rf"\dot{{V}} = {label_number(round(chain()['vdot_h'], -1))}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{h}}}}",
                             np.array([x_read, 0.60, 0.0]), size=BODY_FONT_SIZE, color=P_GREEN)

        def case_tag(en, de):
            return Text(tr(en, de), font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([6.9, 2.85, 0.0]), aligned_edge=RIGHT)

        case_low = case_tag("Summer night · h = 2.5 m", "Sommernacht · h = 2,5 m")
        case_tall = case_tag("Summer night · h = 4.4 m", "Sommernacht · h = 4,4 m")
        case_winter = case_tag("Winter · h = 4.4 m", "Winter · h = 4,4 m")

        def ru(value, digits=0):
            return label_number(value, digits)

        def panel_density(s):
            kgm3 = r"\,\frac{\mathrm{kg}}{\mathrm{m^{3}}}"
            return math_panel([
                ("drho", r"\Delta\rho", P_BLUE), (None, "=", P_WHITE),
                ("diff", r"\rho(\theta_{e}) - \rho(\theta_{i})", P_BLUE), (None, "=", P_WHITE),
                ("val", rf"{ru(s['rho_e'], 3)}{kgm3} - {ru(s['rho_i'], 3)}{kgm3} = {ru(s['drho'], 3)}{kgm3}", P_YELLOW),
            ], color=P_BLUE)

        def panel_dp(s, h):
            return math_panel([
                ("dp", r"\Delta p", P_YELLOW), (None, "=", P_WHITE),
                ("drho", r"\Delta\rho", P_BLUE), (None, r"\cdot", P_WHITE),
                ("g", "g", P_WHITE), (None, r"\cdot", P_WHITE),
                ("h", "h", P_CYAN), (None, "=", P_WHITE),
                ("val", rf"{ru(s['drho'], 3)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}} \cdot {ru(G_ACC, 2)}\,\frac{{\mathrm{{m}}}}{{\mathrm{{s^{{2}}}}}} \cdot {ru(h, 1)}\,\mathrm{{m}} = {ru(s['dp'], 1)}\,\mathrm{{Pa}}", P_YELLOW),
            ], color=P_YELLOW)

        def panel_v(s):
            return math_panel([
                ("v", "v", P_CYAN), (None, "=", P_WHITE),
                ("root", r"\sqrt{2\,\Delta p\,/\,\rho}", P_WHITE), (None, "=", P_WHITE),
                ("val", rf"\sqrt{{2 \cdot {ru(s['dp'], 1)}\,\mathrm{{Pa}}\,/\,{ru(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}}}} ={ru(s['v'], 2)}\,\mathrm{{m/s}}", P_CYAN),
            ], color=P_CYAN, size=0.85 * FORMULA_FONT_SIZE, edge_buff=1.86)

        def panel_q(s):
            return math_panel([
                ("q", r"\dot{V}", P_GREEN), (None, "=", P_WHITE),
                ("cd", r"C_{d}", P_WHITE), (None, r"\cdot", P_WHITE),
                ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
                ("v", "v", P_CYAN), (None, "=", P_WHITE),
                ("val", rf"{ru(CD_OPENING, 1)} \cdot {ru(A_OPENING, 1)}\,\mathrm{{m^{{2}}}} \cdot {ru(s['v'], 2)}\,\frac{{\mathrm{{m}}}}{{\mathrm{{s}}}} = {ru(s['vdot'], 3)}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{s}}}} \approx {ru(round(s['vdot_h'], -1))}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{h}}}}", P_GREEN),
            ], color=P_GREEN)

        def swap_panel(old, new):
            return [FadeOut(old[0]), FadeOut(old[1]), FadeIn(new[0]), FadeIn(new[1])]

        self.play(Create(room), Create(floor), Create(inlet), GrowArrow(inlet_arrow), FadeIn(inlet_lbl), FadeIn(person), run_time=1.4)
        self.play(*warmth(1.1), run_time=1.1)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.1, during=warmth)

        caption = swap_caption(self, caption, subtitle_text(N, "shaft"))
        self.play(Create(shaft_walls), FadeIn(ceiling_gap), Create(outlet), GrowArrow(outlet_arrow), FadeIn(outlet_lbl), *warmth(1.2), run_time=1.2)
        stack["paths"] = stack_low = self._stack_paths(inlet.get_center(), room_top, self.TOP_LOW)
        guides_low = flow_guides(stack_low, P_ORANGE)
        self.play(Create(guides_low), *warmth(0.6), run_time=0.6)
        self.play(*rising(2.6), run_time=2.6)
        hold_for(self, N, "shaft", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "density"))
        panel = panel_density(S_LOW)
        self.play(FadeIn(legend), *rising(1.2), run_time=1.2)
        hold_for(self, N, "density", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "drho"))
        self.play(FadeIn(panel[0]), Create(panel[1]), Indicate(legend_rows[1], color=P_BLUE), Indicate(legend_rows[2], color=P_ORANGE),
                  *rising(1.4), run_time=1.4)
        hold_for(self, N, "drho", during=rising)

        h_dim = dim_arrow(
            np.array([-0.20, inlet.get_center()[1], 0.0]),
            np.array([-0.20, self.TOP_LOW, 0.0]),
            color=P_CYAN,
        )
        h_tok = math_label(tr(r"h = 2.5\,\mathrm{m}", r"h = 2{,}5\,\mathrm{m}"), size=LABEL_FONT_SIZE, color=P_CYAN).rotate(PI / 2)
        h_tok.next_to(h_dim, LEFT, buff=0.10)
        h_tok_tall = math_label(tr(r"h = 4.4\,\mathrm{m}", r"h = 4{,}4\,\mathrm{m}"), size=LABEL_FONT_SIZE, color=P_CYAN).rotate(PI / 2)

        caption = swap_caption(self, caption, subtitle_text(N, "pressure"))
        new = panel_dp(S_LOW, 2.5)
        self.play(*swap_panel(panel, new), Create(h_dim), FadeIn(h_tok), FadeIn(out_dp), FadeIn(case_low), *rising(1.2), run_time=1.2)
        panel = new
        ring_h = highlight_param(panel[2], "h", color=P_CYAN)
        self.play(Create(ring_h), *rising(0.8), run_time=0.8)
        hold_for(self, N, "pressure", during=rising)
        self.play(FadeOut(ring_h), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "speed"))
        new = panel_v(S_LOW)
        stack["speed"] = 0.9
        self.play(*swap_panel(panel, new), FadeIn(out_v), *rising(1.2), run_time=1.2)
        panel = new
        ring_v = highlight_param(panel[2], "root", color=P_CYAN)
        self.play(Create(ring_v), *rising(0.8), run_time=0.8)
        hold_for(self, N, "speed", during=rising)
        self.play(FadeOut(ring_v), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "flow"))
        new = panel_q(S_LOW)
        a_lbl = math_label(rf"A = {ru(A_OPENING, 1)}\,\mathrm{{m^{{2}}}}", size=LABEL_FONT_SIZE, color=P_CYAN)
        a_lbl.next_to(inlet_arrow, DOWN, buff=0.12).align_to(inlet_arrow.get_left() + LEFT * 0.9, LEFT)
        self.play(*swap_panel(panel, new), FadeIn(out_q), FadeIn(a_lbl), Indicate(inlet, color=P_CYAN, scale_factor=1.3), *rising(1.2), run_time=1.2)
        panel = new
        ring_q = highlight_param(panel[2], "a", color=P_CYAN)
        self.play(Create(ring_q), *rising(0.8), run_time=0.8)
        hold_for(self, N, "flow", during=rising)
        self.play(FadeOut(ring_q), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "taller"))
        rise = self.TOP_HIGH - self.TOP_LOW
        h_tok_tall.move_to(h_tok.get_center() + UP * rise / 2)
        tall_dim = dim_arrow(
            np.array([-0.20, inlet.get_center()[1], 0.0]),
            np.array([-0.20, self.TOP_HIGH, 0.0]),
            color=P_CYAN,
        )
        new = panel_dp(S_TALL, 4.4)
        self.play(
            *[
                wall.animate.put_start_and_end_on(
                    wall.get_start(), wall.get_end() + UP * rise,
                )
                for wall in shaft_walls
            ],
            outlet_group.animate.shift(UP * rise),
            stage.transition("ventilation.natural.stack-effect.height-dimension-taller", source=h_dim, target=tall_dim),
            stage.transition("ventilation.natural.stack-effect.height-token-taller", source=h_tok, target=h_tok_tall),
            FadeOut(guides_low),
            FadeOut(case_low), FadeIn(case_tall),
            height.animate.set_value(4.4),
            *swap_panel(panel, new),
            run_time=1.6,
        )
        panel = new
        stack["paths"] = stack_high = self._stack_paths(inlet.get_center(), room_top, self.TOP_HIGH)
        stack["speed"] = 1.1
        self.play(Create(flow_guides(stack_high, P_ORANGE)), run_time=0.5)
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, N, "taller", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        stack["speed"] = 1.8
        new = panel_dp(S_WINTER, 4.4)
        self.play(
            stage.transition("ventilation.natural.stack-effect.legend-winter", source=legend, target=legend_winter),
            FadeOut(case_tall), FadeIn(case_winter),
            t_out_tr.animate.set_value(T_OUT_WINTER), t_in_tr.animate.set_value(T_IN_WINTER),
            *swap_panel(panel, new),
            *rising(1.6),
            run_time=1.6,
        )
        panel = new
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, N, "winter", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "comfort"))
        low_ring = VGroup(
            SurroundingRectangle(VGroup(inlet, inlet_arrow, inlet_lbl, a_lbl), color=P_CYAN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5),
            SurroundingRectangle(person, color=P_CYAN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5),
        )
        high_ring = SurroundingRectangle(VGroup(outlet, outlet_arrow), color=P_ORANGE, buff=_theme_module.UI_SPACING,
                                         corner_radius=0, stroke_width=2.5)
        self.play(Create(low_ring), Create(high_ring), *rising(1.0), run_time=1.0)
        self.play(Indicate(person, scale_factor=1.0), *rising(0.9), run_time=0.9)
        hold_for(self, N, "comfort", during=rising)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Night purge over a two-day summer profile
class Beat5_Nachtlueftung(Scene):
    NARRATION = [
        ("intro",
         "Summer comfort lives on night ventilation once the envelope is shaded. Here are two summer days, with the outdoor temperature in yellow.",
         "Sommerkomfort lebt von der Nachtlüftung, sobald die Hülle\nverschattet ist — hier zwei Sommertage, Außentemperatur in Gelb."),
        ("shade",
         "By day the sun hits the facade. External shading stops the radiation before the glass; the rest reaches the thermal mass.",
         "Tagsüber trifft die Sonne die Fassade. Außenliegende Verschattung hält die Strahlung vor dem Glas ab; der Rest erreicht die Speichermasse."),
        ("day",
         "By day the outdoor air is hotter than the room, so the windows stay shut and the structure soaks up the heat it cannot reject.",
         "Tagsüber ist es draußen heißer als drinnen — die Fenster\nbleiben zu, und die Bauteile nehmen die Wärme auf."),
        ("without",
         "Without night ventilation that stored heat is never released. Each day starts warmer than the last, and the room drifts out of the comfort band.",
         "Ohne Nachtlüftung wird die gespeicherte Wärme nie\nabgegeben — der Raum verlässt das Komfortband."),
        ("night",
         "After sunset, cooler outdoor air opens the opportunity: open wide and flush the thermal mass with cool air.",
         "Nach Sonnenuntergang fällt die Außenluft unter die\nRaumtemperatur — jetzt weit öffnen und kühl spülen."),
        ("result",
         "The mass is discharged overnight, so the second day peaks about four kelvin lower, back inside the comfort band, with no chiller at all.",
         "Über Nacht entlädt sich die Masse — der zweite Tag liegt\nrund vier Kelvin tiefer, ganz ohne Kältemaschine."),
        ("schedule",
         "So in summer night ventilation is a schedule, not a setting: wide open at night, closed and shaded by day.",
         "Im Sommer ist Nachtlüftung ein Zeitplan, keine Einstellung:\nnachts weit offen, tagsüber zu und verschattet."),
        ("winter",
         "Winter reverses the schedule: thermal mass retains daytime warmth. Close windows at night and air briefly by day.",
         "Im Winter kehrt sich der Zeitplan um: die Masse hält die Tageswärme —\nnachts bleiben die Fenster zu, tagsüber wird kurz gelüftet."),
    ]

    @staticmethod
    def _outdoor(t):
        return 27.5 + 6.0 * np.sin(2 * np.pi * (t - 8.0) / 24.0)

    @staticmethod
    def _indoor_sealed(t):
        return 26.8 + 1.3 * np.sin(2 * np.pi * (t - 12.0) / 24.0) + 0.035 * t

    @staticmethod
    def _indoor_purged(t):
        return 24.6 + 1.2 * np.sin(2 * np.pi * (t - 12.0) / 24.0)

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Night ventilation: discharging the thermal mass", "Nachtlüftung: den Speicher entladen"))
        din = reference_chip(_references.cite("din-4108-2"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = Axes(
            x_range=[0, 48, 12], y_range=[18, 34, 4],
            x_length=7.2, y_length=2.8,
            axis_config={"color": P_WHITE, "stroke_width": 2.2, "include_tip": False},
            tips=False,
        )
        axes.move_to(np.array([-1.35, 0.60, 0.0]))

        x_ticks = VGroup(*[
            Text(s, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(v, 18), DOWN, buff=0.14)
            for v, s in ((0, "0"), (12, "12"), (24, "24"), (36, "36"), (48, "48"))
        ])
        x_unit = math_label(r"t\;\text{in}\;\mathrm{h}", size=LABEL_FONT_SIZE, color=P_PRIMARY)
        x_unit.next_to(axes.c2p(48, 18), DOWN, buff=0.14).shift(RIGHT * 0.62)
        y_ticks = VGroup(*[
            Text(s, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(0, v), LEFT, buff=0.14)
            for v, s in ((22, "22"), (26, "26"), (30, "30"), (34, "34"))
        ])
        y_unit = math_label(r"\theta\;\text{in}\;\mathrm{°C}", size=LABEL_FONT_SIZE, color=P_PRIMARY)
        y_unit.next_to(axes.c2p(0, 34), UP, buff=0.14).shift(RIGHT * 0.45)

        def _zone(t0, t1, v0, v1, color, opacity):
            p0, p1 = axes.c2p(t0, v0), axes.c2p(t1, v1)
            return Rectangle(
                width=abs(p1[0] - p0[0]), height=abs(p1[1] - p0[1]),
                stroke_width=0, fill_color=color, fill_opacity=opacity,
            ).move_to((p0 + p1) / 2)

        comfort = _zone(0, 48, 22, 26, P_GREEN, 0.12)
        comfort_lbl = Text(tr("Comfort band", "Komfortband"), font_size=LABEL_FONT_SIZE, color=P_GREEN)
        comfort_lbl.next_to(axes.c2p(0, 22), LEFT, buff=0.14).shift(DOWN * 0.30)
        nights = VGroup(_zone(20, 30, 18, 34, P_BLUE, 0.16), _zone(44, 48, 18, 34, P_BLUE, 0.16))
        night_lbl = Text(tr("Night", "Nacht"), font_size=LABEL_FONT_SIZE, color=P_BLUE)
        night_lbl.next_to(nights[0], UP, buff=0.10)
        warn = Text(tr("leaves the comfort band", "verlässt das Komfortband"), font_size=LABEL_FONT_SIZE, color=P_RED)
        warn.next_to(axes, UP, buff=0.22).set_x(axes.get_center()[0])

        outdoor = axes.plot(self._outdoor, x_range=[0, 48], color=P_YELLOW, stroke_width=3)
        sealed = axes.plot(self._indoor_sealed, x_range=[0, 48], color=P_RED, stroke_width=3)
        purged = axes.plot(self._indoor_purged, x_range=[0, 48], color=P_CYAN, stroke_width=3)

        def _key(color, text):
            swatch = Line(LEFT * 0.22, RIGHT * 0.22, color=color, stroke_width=4)
            return VGroup(swatch, Text(text, font_size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.16)

        keys = VGroup(
            _key(P_YELLOW, tr("Outdoor air", "Außenluft")),
            _key(P_RED, tr("indoor — without", "innen — ohne")),
            _key(P_CYAN, tr("indoor — with", "innen — mit")),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.18)
        keys.move_to(np.array([4.85, -0.55, 0.0]))

        room_c = np.array([5.05, 1.10, 0.0])
        icon = Rectangle(width=2.3, height=1.3, stroke_width=0).move_to(room_c)
        icon_room = _room(room_c, 2.3, 1.3, t=0.12, window_side="both", window_y=room_c[1] + 0.28, window_h=0.45)
        mass = Rectangle(
            width=2.05, height=0.42, stroke_width=0,
            fill_color=P_RED, fill_opacity=0.0,
        ).move_to(icon.get_bottom() + UP * 0.24)
        mass_lbl = Text(tr("Thermal mass", "Speichermasse"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY)
        mass_lbl.next_to(icon, UP, buff=0.12)
        mass_hot = Text("27 °C", font_size=BODY_FONT_SIZE, color=P_RED).move_to(mass)
        mass_cool = Text("22 °C", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(mass)
        icon_win = VGroup(
            _window(icon.get_left() + UP * 0.28, height=0.45, width=0.12),
            _window(icon.get_right() + UP * 0.28, height=0.45, width=0.12, inward=-1),
        )
        win_c = icon.get_left() + UP * 0.28
        sky_pos = win_c + np.array([-1.05, 1.0, 0.0])
        sun = sun_glyph(sky_pos).scale(0.32)
        blind_c = win_c + LEFT * 0.32
        blind = VGroup(
            Rectangle(width=0.5, height=0.05, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.9).move_to(win_c + np.array([-0.32, 0.39, 0.0])),
            *[Line(blind_c + np.array([-0.12, dy, 0.0]), blind_c + np.array([0.12, dy - 0.07, 0.0]), color=P_ORANGE, stroke_width=3.2)
              for dy in np.linspace(0.30, -0.28, 6)],
        )
        shade_lbl = Text(tr("Shading", "Verschattung"), font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        shade_lbl.scale(0.85).next_to(blind, DOWN, buff=0.12)
        shade_lbl.shift(RIGHT * (blind_c[0] + 0.12 - shade_lbl.get_right()[0]))
        ray_dir = (blind_c + UP * 0.1 - sky_pos) / np.linalg.norm(blind_c + UP * 0.1 - sky_pos)
        ray_normal = np.array([-ray_dir[1], ray_dir[0], 0.0])
        rays = VGroup(*[
            Arrow(sky_pos + ray_dir * 0.36 + ray_normal * k, sky_pos + ray_dir * (np.linalg.norm(blind_c + UP * 0.1 - sky_pos) - 0.10) + ray_normal * k,
                  buff=0, color=P_YELLOW, stroke_width=3.5, max_tip_length_to_length_ratio=0.3)
            for k in (-0.2, 0.0, 0.2)
        ])
        leak = Arrow(win_c + RIGHT * 0.12, mass.get_top() + LEFT * 0.55, buff=0, color=P_YELLOW, stroke_width=2.0,
                     max_tip_length_to_length_ratio=0.25).set_stroke(opacity=0.7)
        sun_group = VGroup(sun, rays, blind, shade_lbl, leak)
        moon = VGroup(
            moon_glyph(sky_pos).scale(0.85),
            *[
                Dot(sky_pos + RIGHT * dx + UP * dy, radius=0.03, color=P_WHITE, fill_opacity=0.85)
                for dx, dy in ((0.52, 0.16), (-0.34, 0.42), (0.28, -0.40))
            ],
        )

        schedule = VGroup(
            _badge(tr("Night", "Nacht"), tr("open wide", "weit öffnen"), P_BLUE),
            _badge(tr("Day", "Tag"), tr("close + shade", "schließen + verschatten"), P_ORANGE),
        ).arrange(RIGHT, buff=0.45)
        schedule.move_to(np.array([-1.35, -2.05, 0.0]))
        schedule_winter = VGroup(
            _badge(tr("Night", "Nacht"), tr("closed", "geschlossen"), P_BLUE),
            _badge(tr("Day", "Tag"), tr("burst briefly", "kurz stoßlüften"), P_ORANGE),
        ).arrange(RIGHT, buff=0.45).move_to(schedule)
        season_lbl = Text(tr("Summer", "Sommer"), font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(schedule, LEFT, buff=0.3)
        season_lbl_winter = Text("Winter", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(season_lbl)
        mass_warm = Text("21 °C", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(mass)

        chart = VGroup(axes, x_ticks, x_unit, y_ticks, y_unit, comfort, comfort_lbl,
                       keys, icon, icon_room, mass, mass_lbl, icon_win, schedule, season_lbl, sun_group)
        fit_band(chart)

        mass_spots = [mass.get_top() + LEFT * 0.6, mass.get_top() + RIGHT * 0.6]

        def stored(rt, color=P_RED):
            return [ripples(mass_spots, r_max=0.32, color=color, cycles=rt / 1.3)]

        self.play(Create(axes), FadeIn(x_ticks), FadeIn(y_ticks), FadeIn(x_unit), FadeIn(y_unit), run_time=1.6)
        self.play(FadeIn(comfort), FadeIn(comfort_lbl), run_time=0.7)
        self.play(Create(outdoor), FadeIn(keys[0]), run_time=2.2)
        self.play(Create(icon_room), FadeIn(icon_win), FadeIn(mass_lbl), FadeIn(sun), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.6 + 0.7 + 2.2 + 1.1)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "shade"))
        self.add(mass)
        self.play(LaggedStart(*[GrowArrow(a) for a in rays], lag_ratio=0.2), FadeIn(blind), FadeIn(shade_lbl), run_time=1.4)
        self.play(GrowArrow(leak), run_time=0.8)
        hold_for(self, self.NARRATION, "shade", used=1.4 + 0.8)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "day"))
        self.play(mass.animate.set_fill(P_RED, opacity=0.55), FadeIn(mass_hot), *stored(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "day", during=stored)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "without"))
        self.play(Create(sealed), FadeIn(keys[1]), *stored(2.2), run_time=2.2)
        self.play(FadeIn(warn, shift=DOWN * 0.1), Indicate(sealed, color=P_RED, scale_factor=1.0), *stored(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "without", during=stored)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "night"))
        self.play(
            FadeIn(nights), FadeIn(night_lbl), FadeOut(warn),
            FadeOut(sun_group, shift=UP * 0.3), FadeIn(moon, shift=DOWN * 0.3),
            run_time=1.0,
        )
        sweep = VGroup(
            smooth_path([
                icon.get_left() + UP * 0.28,
                icon.get_center() + DOWN * 0.02,
                icon.get_right() + UP * 0.28,
            ]),
        )
        def purge(rt):
            return [_streams([(sweep, P_BLUE, P_ORANGE)], rt, speed=1.0, waves=4), *stored(rt, P_ORANGE)]

        self.play(Create(flow_guides(sweep, P_BLUE)), run_time=0.5)
        self.play(*purge(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "night", during=purge)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        self.play(
            Create(purged), FadeIn(keys[2]),
            mass.animate.set_fill(P_BLUE, opacity=0.45),
            stage.transition("ventilation.natural.night-ventilation.mass-cooled", source=mass_hot, target=mass_cool),
            _streams([(sweep, P_BLUE, P_ORANGE)], 2.2, speed=1.0, waves=4),
            run_time=2.2,
        )
        peak = 42.0
        gain = dim_arrow(
            axes.c2p(peak, self._indoor_purged(peak)),
            axes.c2p(peak, self._indoor_sealed(peak)),
            color=P_YELLOW,
        )
        gain_lbl = Text("≈ 4 K", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        gain_lbl.next_to(gain, DOWN, buff=0.32).shift(LEFT * 0.38)
        self.play(Create(gain), FadeIn(gain_lbl), run_time=1.0)
        hold_for(self, self.NARRATION, "result", used=2.2 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "schedule"))
        self.play(FadeIn(schedule, shift=UP * 0.12), FadeIn(season_lbl), FadeOut(moon), FadeIn(sun), FadeIn(blind), FadeIn(rays),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "schedule", used=1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.play(
            stage.transition("ventilation.natural.night-ventilation.schedule-winter", source=schedule, target=schedule_winter),
            stage.transition("ventilation.natural.night-ventilation.season-winter", source=season_lbl, target=season_lbl_winter),
            mass.animate.set_fill(P_ORANGE, opacity=0.5),
            stage.transition("ventilation.natural.night-ventilation.mass-warm", source=mass_cool, target=mass_warm),
            *stored(1.4, P_ORANGE),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "winter", used=1.4 + 0.35, during=lambda rt: stored(rt, P_ORANGE))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Where free ventilation runs out
class Beat6_GrenzenDerFreienLueftung(Scene):
    NARRATION = [
        ("intro",
         "Free ventilation assumes usable outdoor air. When that fails, it has no answer in summer or winter.",
         "Bisher galt eine Annahme: die Außenluft ist brauchbar. Wenn nicht,\nhat die freie Lüftung in Sommer wie Winter keine Antwort."),
        ("hot",
         "On a hot, humid summer afternoon the incoming air carries heat and moisture into the room instead of taking them out.",
         "An einem heißen, schwülen Sommernachmittag trägt die Zuluft\nWärme und Feuchte herein statt heraus."),
        ("cold",
         "In winter it is the other way round: air at zero degrees streams in and the room loses heat.",
         "Im Winter ist es umgekehrt: Luft mit 0 °C strömt herein,\nder Raum verliert Wärme."),
        ("filter",
         "An open window also has no filter. Pollen, dust and street noise come in together with the air.",
         "Ein offenes Fenster hat auch keinen Filter — Pollen,\nStaub und Straßenlärm kommen mit."),
        ("control",
         "Wind and buoyancy vary all day: weather sets this flow, so it cannot be sized for a heating or cooling load.",
         "Der Volumenstrom folgt dem Wetter, nicht dem Entwurf — auf eine\nHeiz- oder Kühllast lässt er sich nicht auslegen."),
        ("recovery",
         r"At $\eta = 0$, exhaust energy escapes through the opening. No heat is recovered: winter ventilation loses all $1500\,\mathrm{W}$.",
         r"Bei $\eta = 0$ entweicht die Abluftenergie durch die Öffnung. Keine Rückgewinnung: im Winter gehen volle $1500\,\mathrm{W}$ verloren."),
        ("handover",
         "Fan-assisted ventilation with heat recovery closes these four gaps. The next chapter underpins heating and cooling technology.",
         "Mechanische Lüftung mit WRG schließt diese vier Lücken. Das nächste Kapitel legt die Basis für Heizung und Kühltechnik."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Limits of natural ventilation", "Grenzen der freien Lüftung"))
        din = reference_chip(_references.cite("din-en-16798-7"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-3.40, 0.55, 0.0])
        shell = _room(room_c, 4.4, 2.2, window_side="left", window_y=0.55, window_h=1.0)
        room, floor = shell[0], shell[1]
        wash = Rectangle(
            width=4.3, height=2.1, stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
        ).move_to(room_c)
        win = _window(np.array([room_c[0] - 2.2, 0.55, 0.0]), height=1.0)
        person = _stand(room_c, 2.2, -2.25)

        outdoor = _badge(tr("Summer afternoon", "Sommernachmittag"), tr("32 °C · humid", "32 °C · schwül"), P_RED)
        outdoor.move_to(np.array([-3.40, 2.15, 0.0]))
        outdoor_cold = _badge("Winter", tr("0 °C · cold", "0 °C · kalt"), P_CYAN).move_to(outdoor)

        inflow = VGroup(
            smooth_path([
                win.get_right() + RIGHT * 0.05 + UP * 0.20,
                room_c + LEFT * 0.9 + UP * 0.35,
                room_c + RIGHT * 0.9 + DOWN * 0.10,
            ]),
        )
        outflow = VGroup(
            smooth_path([
                room_c + RIGHT * 0.9 + UP * 0.45,
                room_c + LEFT * 0.9 + UP * 0.72,
                win.get_right() + RIGHT * 0.05 + UP * 0.62,
            ]),
        )

        rng = np.random.default_rng(9)
        grime = VGroup(*[
            Dot(
                room_c + np.array([float(rng.uniform(-1.7, 1.7)), float(rng.uniform(-0.7, 0.8)), 0.0]),
                radius=0.045, color=P_WHITE, fill_opacity=0.55, stroke_width=0,
            )
            for _ in range(14)
        ])
        grime_tag = Text(tr("Pollen · Dust", "Pollen · Staub"), font_size=LABEL_FONT_SIZE, color=P_WHITE)
        grime_tag.next_to(room, DOWN, buff=0.16)

        lost = Arrow(
            win.get_left() + LEFT * 0.10 + UP * 0.62,
            win.get_left() + LEFT * 0.95 + UP * 0.62,
            buff=0, color=P_ORANGE, stroke_width=4, max_tip_length_to_length_ratio=0.24,
        )
        lost_tag = Text(tr("unused", "ungenutzt"), font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        lost_tag.next_to(lost, UP, buff=0.55).set_x(lost_tag.width / 2 - 6.8)

        gaps = VGroup(*[
            VGroup(cross_mark(), Text(t, font_size=BODY_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.22)
            for t in (
                tr("Outdoor air too hot, humid or cold", "Außenluft zu heiß, schwül oder kalt"),
                tr("no filtering", "keine Filterung"),
                tr("volume flow not controllable", "Volumenstrom nicht regelbar"),
                tr("no heat recovery (η = 0)", "keine Rückgewinnung (η = 0)"),
            )
        ]).arrange(DOWN, aligned_edge=LEFT, buff=0.34)
        gaps.move_to(np.array([4.05, 0.45, 0.0]))

        lost_eta = math_label(r"\eta = 0", size=BODY_FONT_SIZE, color=P_ORANGE)
        lost_eta.next_to(lost, DOWN, buff=0.14).set_x(lost_tag.get_x())
        q_loss = V_NET * N_RATE * C_LUFT * D_THETA
        loss_read = math_label(
            rf"\text{{Winter:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}} \approx {label_number(round(q_loss, -2))}\,\mathrm{{W}}",
            size=LABEL_FONT_SIZE, color=P_CYAN,
        ).move_to(np.array([4.05, -1.05, 0.0]))

        handover = VGroup(
            chip(tr("→ mechanical ventilation · next chapter", "→ mechanische Lüftung · nächstes Kapitel"), P_PRIMARY, font_size=LABEL_FONT_SIZE),
            _hand_chip(tr("→ Heating:", "→ Heizung:"), r"\dot{Q}_{\mathrm{V}}", P_ORANGE),
            _hand_chip(tr("→ Cooling:", "→ Kühltechnik:"), r"\dot{Q}_{\mathrm{V,sens}} + \dot{Q}_{\mathrm{V,lat}}", P_CYAN),
        ).arrange(RIGHT, buff=0.3)
        handover.move_to(np.array([0.0, -1.95, 0.0]))

        scaffold = VGroup(room, floor, wash, win, person, outdoor, grime_tag, gaps, handover, loss_read)
        fit_band(scaffold)

        vol = meter(tr("Volume flow", "Volumenstrom"), length=2.1, thickness=0.52, color=P_CYAN)
        vol["group"].move_to(np.array([0.3, 0.45, 0.0]))
        rate = ValueTracker(0.50)
        bind_meter(vol, rate)

        air = {"in": (P_CYAN,), "out": (P_CYAN,), "speed": 0.9}

        def exchange(rt):
            return [_streams([(inflow, *air["in"]), (outflow, *air["out"])], rt, speed=air["speed"])]

        self.add(wash)
        self.play(Create(room), Create(floor), FadeIn(win), FadeIn(person), run_time=1.3)
        self.play(Create(flow_guides(inflow, P_CYAN)), Create(flow_guides(outflow, P_CYAN)), run_time=0.7)
        self.play(*exchange(1.8), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.7 + 1.8, during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "hot"))
        air["in"] = (P_RED, P_ORANGE)
        self.play(FadeIn(outdoor), wash.animate.set_fill(P_RED, opacity=0.22), *exchange(1.1), run_time=1.1)
        self.play(*exchange(2.0), run_time=2.0)
        self.play(FadeIn(gaps[0], shift=RIGHT * 0.2), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "hot", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cold"))
        air["in"] = (P_BLUE, P_ORANGE)
        self.play(stage.transition("ventilation.natural.limits.outdoor-cold", source=outdoor, target=outdoor_cold), wash.animate.set_fill(P_BLUE, opacity=0.16),
                  *exchange(1.2), run_time=1.2)
        self.play(*exchange(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "cold", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "filter"))
        self.play(
            LaggedStart(*[FadeIn(d, scale=0.4) for d in grime], lag_ratio=0.06),
            FadeIn(grime_tag), FadeIn(gaps[1], shift=RIGHT * 0.2), *exchange(1.8),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "filter", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "control"))
        self.play(FadeIn(vol["group"]), *exchange(0.7), run_time=0.7)
        for level in (0.82, 0.22, 0.95, 0.38):
            air["speed"] = 0.4 + 1.2 * level
            self.play(rate.animate.set_value(level), *exchange(0.5), run_time=0.5)
        self.play(FadeIn(gaps[2], shift=RIGHT * 0.2), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "control", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "recovery"))
        air["out"] = (P_ORANGE,)
        self.play(GrowArrow(lost), FadeIn(lost_tag), FadeIn(lost_eta), *exchange(0.9), run_time=0.9)
        self.play(*exchange(1.8), run_time=1.8)
        self.play(FadeIn(gaps[3], shift=RIGHT * 0.2), FadeIn(loss_read, shift=UP * 0.1), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "recovery", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "handover"))
        self.play(Indicate(gaps, color=P_RED, scale_factor=1.04), *exchange(1.0), run_time=1.0)
        self.play(LaggedStart(*[FadeIn(h, shift=UP * 0.14) for h in handover], lag_ratio=0.25),
                  *exchange(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "handover", during=exchange)
        vol["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
