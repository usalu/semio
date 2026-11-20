"""🌬️ Ventilation chapter 3 — Was die Luft trägt (heat flow of the air stream, season-neutral).

One example house runs through every beat: its air flow V̇ = V · n = 220 m³/h (chapter 2) carries sensible heat
Q̇_V,sens = V̇ · c_Luft · Δθ with a signed Δθ and, in summer, latent heat Q̇_V,lat = ρ · r · Δx · q_v,R. This chapter
owns c_Luft, the winter and summer Δθ and the latent Δx of the running example; Heizung (winter, sensible) and
Kühlung (summer, sensible plus latent) use them later. Every shown quantity counts up from 0 and every result is
read from the drawn meters.

Source of truth for V, n and V̇ is chapter 2 (``2_luftmenge/scene_2.py``); the values are mirrored in
``Running example`` below.
"""

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))


_theme_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
semio_color = _theme_module.color

import numpy as np
from manim import *

from pathlib import Path as _Path


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
body_text = _manim_module.body_text
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
window_chip = _manim_module.window_chip
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_row = _manim_module.math_row
math_readout = _manim_module.math_readout
math_panel = _manim_module.math_panel
label_number = _manim_module.label_number
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
def flow_guides(*args, visible: bool = False, **kwargs):
    """🧭 Particle-path guides — kept, temporarily hidden; particles stay visible."""
    return _manim_module.flow_guides(*args, visible=visible, **kwargs)
dim_arrow = _manim_module.dim_arrow
chip = _manim_module.chip
meter = _manim_module.meter
set_meter = _manim_module.set_meter
stacked_bar = _manim_module.stacked_bar
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
load_vo_timing = _manim_module.load_vo_timing
house_section = _manim_module.house_section
open_window = _manim_module.open_window
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
staffage = _manim_module.staffage
staffage_height = _manim_module.staffage_height
sun_glyph = _manim_module.sun_glyph
thermometer_glyph = _manim_module.thermometer_glyph
ripples = _manim_module.ripples

# 🗣️ Measured clause durations live in vo_timing.json.
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent chapter title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Was die Luft trägt"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the Heating and Cooling series: a dim ``P_PRIMARY``
    footnote that never competes with the diagram.
    """
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Running example
# 🧮 One house, one set of numbers. V, n and V̇ are owned by chapter 2 (``2_luftmenge``); c_Luft, the winter and
# summer Δθ and the latent Δx are owned here. Heating and Cooling read them back through the hand-over chips.
RHO_AIR, CP_AIR = 1.2, 1005.0
C_LUFT = round(RHO_AIR * CP_AIR / 3600.0, 2)
V_NET, N_RATE = 440.0, 0.5
V_DOT = V_NET * N_RATE
Q_V_R = V_DOT / 3600.0
THETA_I_W, THETA_E_W = 20.0, 0.0
THETA_I_S, THETA_E_S = 26.0, 32.0
X_OUT, X_IN = 15.0, 10.5
D_X = X_OUT - X_IN
R_LAT = 2450.0
WATER_PER_M3 = RHO_AIR * D_X
Q_SENS_W = V_DOT * C_LUFT * (THETA_I_W - THETA_E_W)
Q_SENS_S = V_DOT * C_LUFT * (THETA_E_S - THETA_I_S)
Q_LAT = RHO_AIR * R_LAT * D_X / 1000.0 * Q_V_R * 1000.0
W_WINTER, W_SUMMER, W_LATENT = (round(v, -1) for v in (Q_SENS_W, Q_SENS_S, Q_LAT))
_RUNNING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
D_X_W = _RUNNING.D_X_WINTER_G_KG
W_LATENT_WINTER = round(_RUNNING.Q_LAT_WINTER_W, -1)

AIR = semio_color("secondary")
SENS = P_RED
LATENT = P_CYAN
HEIZUNG = P_ORANGE
KUEHLUNG = P_CYAN
#endregion


#region Shared visual motifs
# 🧱 Pieces every beat of this chapter composes: the example house with its two openings, season streams, the
# factor rows (name above a 0→1 meter, live readout right of it) and typeset hand-over chips.

#region Geometry
def _cube_faces(center, size: float, color: str, *, warm: bool):
    """🧊 Isometric unit cube — three shaded faces around ``center``; ``warm`` deepens the fill."""
    c = np.array(center, dtype=float)
    top, tr, br = c + UP * size, c + (RIGHT * 0.866 + UP * 0.5) * size, c + (RIGHT * 0.866 + DOWN * 0.5) * size
    bot, bl, tl = c + DOWN * size, c + (LEFT * 0.866 + DOWN * 0.5) * size, c + (LEFT * 0.866 + UP * 0.5) * size
    fills = (0.45, 0.55, 0.65) if warm else (0.35, 0.45, 0.55)
    return VGroup(
        Polygon(c, tl, top, tr, fill_color=color, fill_opacity=fills[0], stroke_color=color, stroke_width=2),
        Polygon(c, tl, bl, bot, fill_color=color, fill_opacity=fills[1], stroke_color=color, stroke_width=2),
        Polygon(c, tr, br, bot, fill_color=color, fill_opacity=fills[2], stroke_color=color, stroke_width=2),
    )


def _house(center, scale: float, reach: float = 1.2):
    """🏠 Physical Fundamentals section house with both openings swung open; ``lo`` and ``up`` are its windows and
    ``reach`` is how far the outside end of a stream track extends in front of the wall."""
    hs = house_section(np.array(center, dtype=float), scale)
    lo, up = hs["windows"]
    return {"hs": hs, "lo": lo, "up": up, "scale": scale, "reach": reach * scale, "group": hs["group"]}


def _open_both(house, run_time: float = 0.9):
    """🚪 Swing both sashes into the room."""
    return [open_window(house["lo"], run_time=run_time), open_window(house["up"], run_time=run_time)]
#endregion


#region Air through the openings
def _through(win, scale: float, reach: float, *, inward: bool, dy: float = 0.0):
    """➡️ Particle track through one opening — from the room out, or reversed from outside in."""
    x, y = float(win["x"]), float(win["center"][1])
    pts = [(x + 1.05 * scale, y - 0.06 + dy), (x + 0.35 * scale, y + dy), (x - 0.3 * scale, y + 0.03 + dy),
           (x - reach, y + 0.2 + dy)]
    if inward:
        pts = pts[::-1]
    return smooth_path([np.array([px, py, 0.0]) for px, py in pts])


def _guides(houses):
    """🧭 Faint tipped streamlines of the season streams — the direction stays readable between particles."""
    lines = VGroup()
    for house, season in houses:
        winter = season == "winter"
        lines.add(flow_guides([_through(house["up"], house["scale"], house["reach"], inward=not winter)], P_RED, opacity=0.3),
                  flow_guides([_through(house["lo"], house["scale"], house["reach"], inward=winter)], P_CYAN, opacity=0.3))
    return lines


def _air(houses, rt: float, *, speed: float = 0.6, moist: bool = False):
    """💨 Season streams for ``rt`` seconds — ``houses`` are ``(house, season)``; the upper opening carries the hot air.

    Winter: warm room air leaves above, cold air enters below. Summer: the same openings, the streams reverse.
    The stream is thicker in winter because it carries more power.
    """
    streams, wet = [], []
    for house, season in houses:
        winter = season == "winter"
        waves = 6 if winter else 3
        up = [_through(house["up"], house["scale"], house["reach"], inward=not winter)]
        lo = [_through(house["lo"], house["scale"], house["reach"], inward=winter)]
        streams += [(up, P_RED, P_ORANGE, waves), (lo, P_CYAN, P_BLUE, waves)]
        if moist and not winter:
            wet.append(([_through(house["up"], house["scale"], house["reach"], inward=True, dy=0.1)], LATENT, 4))
    anims = [flow_animation([(paths, c1, c2)], waves=waves, cycles=speed * rt) for paths, c1, c2, waves in streams]
    anims += [flow_animation([(paths, color)], waves=waves, radius=0.05, cycles=speed * rt, streak=False)
              for paths, color, waves in wet]
    return anims
#endregion


#region Factor rows
def _name(src: str, word: str, color: str):
    """🏷️ Typeset symbol with a small German word beside it — the name above a factor meter."""
    return VGroup(math_label(src, size=BODY_FONT_SIZE, color=color),
                  body_text(word, font_size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.18, aligned_edge=DOWN)


def _row(name, x0: float, y: float, length: float, color: str, frac):
    """📏 Horizontal 0→1 meter at ``(x0, y)`` with its name above; ``frac()`` drives the fill every frame."""
    g = meter("·", length=length, thickness=0.26, color=color, vertical=False)
    g["track"].move_to(np.array([x0 + length / 2, y, 0.0]))
    set_meter(g, 0.0)
    g["fill"].add_updater(lambda m: set_meter(g, frac()))
    name.move_to(np.array([x0, y + 0.13 + 0.17 + name.height / 2, 0.0]))
    name.shift(RIGHT * (x0 - name.get_left()[0]))
    return {"track": g["track"], "fill": g["fill"], "name": name, "x_end": x0 + length, "y": y,
            "group": VGroup(g["track"], g["fill"], name)}


def _show_row(scene, row, readout, *, run_time: float = 0.6):
    """✨ Bring a factor row on screen: its meter and name fade in, the live readout starts counting."""
    scene.play(FadeIn(row["group"], shift=UP * 0.1), run_time=run_time)
    scene.add(readout)


def _ramp(tracker, to: float, *, rate_func=smooth):
    """⏩ Move a tracker to ``to`` — pass ``linear`` where the movement itself must stay proportional to time."""
    start = tracker.get_value()
    return UpdateFromAlphaFunc(tracker, lambda m, a: m.set_value(start + (to - start) * a), rate_func=rate_func)


def _hide(scene, *mobs, run_time: float = 0.7):
    """🫥 Fade out what has been explained so the next step starts on a clean screen; live readouts stop first."""
    for mob in mobs:
        for part in mob.get_family():
            part.clear_updaters()
    scene.play(*[FadeOut(m) for m in mobs], run_time=run_time)
#endregion


#region Chips
def _season_chip(season: str):
    """🔖 Season marker — Winter belongs to Heizung, Sommer to Kühlung, in the colour of its hand-over chip."""
    winter = season == "winter"
    return chip(tr("Example winter day", "Beispiel-Wintertag") if winter else tr("Example summer day", "Beispiel-Sommertag"), HEIZUNG if winter else KUEHLUNG)


def _handover_chip(prefix: str, src: str, color: str):
    """🔗 Hand-over chip with a typeset quantity — names what the later video takes from this chapter."""
    parts = VGroup(body_text(prefix, font_size=LABEL_FONT_SIZE + 2, color=color),
                   math_label(src, size=LABEL_FONT_SIZE + 2, color=color)).arrange(RIGHT, buff=_theme_module.UI_SPACING)
    return window_chip(parts, color=color)
#endregion
#endregion


#region Beat1 – Heat capacity of air
class Beat1_Luftwaerme(Scene):
    """🔋 One cubic metre of air (1,2 kg) warmed by 1 K: the energy counted from 0 is c_Luft."""

    NARRATION = [
        ("cube",
         "Take one cubic metre of air — the unit in which ventilation carries heat.",
         "Nehmen wir einen Kubikmeter Luft — die Einheit, in der die Lüftung Wärme transportiert."),
        ("mass",
         r"One cubic metre of air at $20\,°\mathrm{C}$ has a mass of about $\rho = 1.2\,\mathrm{kg/m^3}$. Colder air is denser: $1.27\,\mathrm{kg/m^3}$ at $5\,°\mathrm{C}$.",
         r"Ein Kubikmeter Luft bei $20\,°\mathrm{C}$ hat eine Masse von etwa $\rho = 1{,}2\,\mathrm{kg/m^3}$. Kältere Luft ist dichter: $1{,}27\,\mathrm{kg/m^3}$ bei $5\,°\mathrm{C}$."),
        ("coil",
         "A heating element warms that cubic metre from below.",
         "Ein Heizelement erwärmt diesen Kubikmeter von unten."),
        ("heat",
         r"From $20$ to $21\,°\mathrm{C}$ it takes $1.2\,\mathrm{kg} \cdot 1005\,\mathrm{J/(kg\,K)} \cdot 1\,\mathrm{K} = 1206\,\mathrm{J}$.",
         r"Von $20$ auf $21\,°\mathrm{C}$ braucht es $1{,}2\,\mathrm{kg} \cdot 1005\,\mathrm{J/(kg\,K)} \cdot 1\,\mathrm{K} = 1206\,\mathrm{J}$."),
        ("c_luft",
         r"The energy per cubic metre and kelvin is the volumetric heat capacity of air, $c_{\mathrm{Luft}}$.",
         r"Die Energie je Kubikmeter und Kelvin ist die volumenbezogene Wärmekapazität der Luft, $c_{\mathrm{Luft}}$."),
        ("norm",
         r"The norm calculates with the fixed value $0.34\,\mathrm{Wh/(m^3K)}$.",
         r"Die Norm rechnet mit dem festen Wert $0{,}34\,\mathrm{Wh/(m^3K)}$."),
        ("spell",
         r"The same physics comes in two unit sets: building-related units with $\mathrm{m^3/h}$ and $\mathrm{Wh/(m^3K)}$, or SI units with $\mathrm{m^3/s}$ and $\mathrm{J/(m^3K)}$.",
         r"Dieselbe Physik gibt es in zwei Einheitensätzen: gebäudebezogene Einheiten mit $\mathrm{m^3/h}$ und $\mathrm{Wh/(m^3K)}$ oder SI-Einheiten mit $\mathrm{m^3/s}$ und $\mathrm{J/(m^3K)}$."),
        ("variiert",
         "Unit usage varies, not only in the DIN. Both lines give the same result, apart from rounding.",
         "Der Einheitengebrauch variiert, nicht nur in der DIN. Beide Zeilen liefern dasselbe Ergebnis, bis auf Rundung."),
        ("perk",
         r"So $220\,\mathrm{m^3/h}$ carry about $75\,\mathrm{W/K}$, that is $75\,\mathrm{W}$ for every kelvin of temperature difference.",
         r"Damit tragen $220\,\mathrm{m^3/h}$ rund $75\,\mathrm{W/K}$, also $75\,\mathrm{W}$ je Kelvin Temperaturdifferenz."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Heat capacity of air", "Wärmekapazität der Luft"))
        din = _din_ref(_references.cite("din-en-12831-1"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "cube"))
        self.play(FadeIn(caption), run_time=0.3)

        c_center = UP * 0.55
        cube = _cube_faces(c_center, 1.0, P_BLUE, warm=False)
        cube_label = math_label(r"1\,\mathrm{m^{3}}", size=BODY_FONT_SIZE).move_to(cube[0].get_center() + UP * 0.15)
        self.play(FadeIn(cube, shift=UP * 0.3), FadeIn(cube_label), run_time=2.0)
        hold_for(self, N, "cube", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0)

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        xl, xq = -6.45, 1.55
        mass = ValueTracker(0.0)
        rho_lbl = math_label(rf"\rho \approx {label_number(RHO_AIR, 1)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}}\;(20\,°\mathrm{{C}})", np.array([xl, 2.3, 0]),
                             size=BODY_FONT_SIZE, color=P_CYAN, edge="left")
        rho_note = math_label(rf"{label_number(1.27, 2)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}}\;(5\,°\mathrm{{C}})", np.array([xl, 1.05, 0]),
                              size=BODY_FONT_SIZE, color=P_CYAN, edge="left")
        m_read = math_readout(lambda: rf"m = {label_number(RHO_AIR, 1)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}} \cdot 1\,\mathrm{{m^{{3}}}} = {label_number(mass.get_value(), 2)}\,\mathrm{{kg}}",
                              np.array([xl, 1.62, 0]), size=BODY_FONT_SIZE, color=P_CYAN)
        self.play(FadeIn(rho_lbl), run_time=0.5)
        self.add(m_read)
        self.play(mass.animate.set_value(RHO_AIR), Indicate(cube, color=P_CYAN, scale_factor=1.04), run_time=1.4)
        self.play(FadeIn(rho_note, shift=DOWN * 0.1), run_time=0.6)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "coil"))
        coil = staffage("ventilation-heat-flow-heater", unit=0.8 / staffage_height("ventilation-heat-flow-heater"), anchor=np.array([0.0, -1.34, 0.0]))
        coil_label = body_text(tr("Heating element", "Heizelement"), font_size=LABEL_FONT_SIZE, color=P_RED).next_to(coil, DOWN, buff=0.1)
        heat_spots = [np.array([x, coil.get_top()[1] + 0.03, 0]) for x in (-0.36, 0.0, 0.36)]
        rise = [smooth_path([np.array([s * x0, y0, 0]), np.array([s * (x0 - 0.08), 0.15, 0]), np.array([s * x0, y1, 0])])
                for s in (-1, 1) for x0, y0, y1 in ((0.55, -0.05, 0.62), (0.2, -0.26, 0.45))]

        def warming(rt, convect=True):
            heat = [ripples(heat_spots, r_max=0.42, rings=2, cycles=rt / 1.2, color=P_RED)]
            return heat + ([flow_animation([(rise, P_BLUE, P_ORANGE)], waves=3, cycles=0.45 * rt)] if convect else [])

        cp_lbl = math_label(rf"c_{{p}} = {label_number(CP_AIR)}\,\frac{{\mathrm{{J}}}}{{\mathrm{{kg\,K}}}}", np.array([xl, 0.55, 0]),
                            size=BODY_FONT_SIZE, color=P_RED, edge="left")
        self.play(Create(coil), FadeIn(coil_label), FadeIn(cp_lbl), run_time=1.5)
        hold_for(self, N, "coil", during=lambda rt: warming(rt, convect=False))

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        theta = ValueTracker(20.0)
        thermo = thermometer_glyph(np.array([-1.65, -0.22, 0]), height=1.8, color=P_RED, level=0.35)
        thermo["column"].add_updater(lambda m: thermo["level"].set_value(0.35 + 0.45 * (theta.get_value() - 20.0)))
        th_read = math_readout(lambda: rf"\theta = {label_number(theta.get_value(), 1)}\,°\mathrm{{C}}", np.array([xl, -0.1, 0]),
                               size=BODY_FONT_SIZE, color=P_ORANGE)
        q_read = math_readout(
            lambda: rf"Q = {label_number(RHO_AIR, 1)}\,\mathrm{{kg}} \cdot {label_number(CP_AIR)}\,\mathrm{{\frac{{J}}{{kg\,K}}}} \cdot {label_number(theta.get_value() - 20.0, 1)}\,\mathrm{{K}}",
            np.array([xq, -1.0, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        j_read = math_readout(
            lambda: rf"= {label_number(RHO_AIR * CP_AIR * (theta.get_value() - 20.0))}\,\mathrm{{J}}",
            np.array([xq + 0.42, -1.5, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        wh_read = math_readout(
            lambda: rf"= {label_number(RHO_AIR * CP_AIR * (theta.get_value() - 20.0) / 3600.0, 3)}\,\mathrm{{Wh}}",
            np.array([xq + 0.42, -2.0, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        self.play(FadeIn(thermo["group"]), FadeIn(thermo["column"]), *warming(0.4, convect=False), run_time=0.4)
        self.add(th_read, q_read, j_read, wh_read)
        warm = _cube_faces(c_center, 1.0, P_ORANGE, warm=True)
        self.play(stage.transition("ventilation.heat-flow.air-heat.cube-warms", source=cube, target=warm), theta.animate.set_value(21.0), *warming(3.0),
                  run_time=3.0, rate_func=linear)
        hold_for(self, N, "heat", during=warming)

        caption = swap_caption(self, caption, subtitle_text(N, "c_luft"))
        xr = 1.55
        c_name = body_text(tr("Volumetric heat capacity", "Volumenbezogene Wärmekapazität"), font_size=LABEL_FONT_SIZE, color=P_GREEN)
        c_name.move_to([xr + c_name.width / 2, 1.75, 0])
        c_def = math_label(r"c_{\mathrm{Luft}} = \frac{\rho \cdot c_{p}}{3600\,\frac{\mathrm{s}}{\mathrm{h}}}", np.array([xr, 1.0, 0]),
                           size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        c_j = math_label(rf"= {label_number(RHO_AIR * CP_AIR)}\,\frac{{\mathrm{{J}}}}{{\mathrm{{m^{{3}}K}}}}", np.array([xr + 0.75, 0.3, 0]),
                         size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        c_wh = math_label(rf"\approx {label_number(C_LUFT, 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}K}}}}", np.array([xr + 0.75, -0.3, 0]),
                          size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        q_snap = math_label(rf"{label_number(RHO_AIR * CP_AIR)}\,\mathrm{{J}}", size=BODY_FONT_SIZE, color=P_YELLOW)
        q_snap.move_to(j_read.get_right() + LEFT * q_snap.width / 2)
        wh_snap = math_label(rf"{label_number(RHO_AIR * CP_AIR / 3600.0, 3)}\,\mathrm{{Wh}}", size=BODY_FONT_SIZE, color=P_YELLOW)
        wh_snap.move_to(wh_read.get_right() + LEFT * wh_snap.width / 2)
        self.play(FadeIn(c_name), FadeIn(c_def), *warming(0.7), run_time=0.7)
        self.play(stage.transition("ventilation.heat-flow.air-heat.joules-per-m3-k", source=q_snap, target=c_j), *warming(0.9), run_time=0.9)
        self.play(stage.transition("ventilation.heat-flow.air-heat.wh-per-m3-k", source=wh_snap, target=c_wh), *warming(0.9), run_time=0.9)
        self.remove(q_snap, wh_snap)
        hold_for(self, N, "c_luft", during=warming)

        caption = swap_caption(self, caption, subtitle_text(N, "norm"))
        self.play(Indicate(c_wh, color=P_GREEN, scale_factor=1.08), *warming(1.0), run_time=1.0)
        hold_for(self, N, "norm", during=warming)

        caption = swap_caption(self, caption, subtitle_text(N, "spell"))
        _hide(self, warm, cube_label, rho_lbl, rho_note, m_read, coil, coil_label, cp_lbl, thermo["group"], thermo["column"],
              th_read, q_read, j_read, wh_read, c_name, c_def, c_j, c_wh)
        v_si = V_DOT / 3600.0

        def unit_block(head: str, formula: str, color: str):
            title = body_text(head, font_size=BODY_FONT_SIZE, color=color)
            line = math_label(formula, size=BODY_FONT_SIZE, color=P_WHITE)
            line.scale_to_fit_width(min(line.width, 11.6))
            frame = RoundedRectangle(width=12.2, height=title.height + line.height + 0.7, corner_radius=0, color=color,
                                     stroke_width=_theme_module.CHROME_BORDER)
            body = VGroup(title, line).arrange(DOWN, buff=0.22)
            return VGroup(frame, body.move_to(frame))

        block_h = unit_block(tr("Building-related units", "Gebäudebezogene Einheiten"),
                             rf"\dot{{V}} \cdot c_{{\mathrm{{Luft}}}} = {label_number(V_DOT)}\,\mathrm{{\frac{{m^3}}{{h}}}} \cdot {label_number(C_LUFT, 2)}\,\mathrm{{\frac{{Wh}}{{m^3\,K}}}}"
                             rf" \approx {label_number(V_DOT * C_LUFT)}\,\mathrm{{\frac{{W}}{{K}}}}", HEIZUNG)
        block_k = unit_block(tr("SI units", "SI-Einheiten"),
                             rf"q_{{v,\mathrm{{R}}}} \cdot \rho \cdot c_{{p}} = {label_number(v_si, 3)}\,\mathrm{{\frac{{m^3}}{{s}}}} \cdot {label_number(RHO_AIR, 1)}\,\mathrm{{\frac{{kg}}{{m^3}}}}"
                             rf" \cdot {label_number(CP_AIR)}\,\mathrm{{\frac{{J}}{{kg\,K}}}} \approx {label_number(v_si * RHO_AIR * CP_AIR)}\,\mathrm{{\frac{{W}}{{K}}}}", KUEHLUNG)
        both = VGroup(block_h, block_k).arrange(DOWN, buff=0.4).move_to(UP * 0.95)
        note = body_text(tr("Same physics, two unit sets (rounding: 0.335 ≈ 0.34). Unit usage varies, not only in the DIN.",
                            "Dieselbe Physik, zwei Einheitensätze (Rundung: 0,335 ≈ 0,34). Einheiten variieren, nicht nur in der DIN."),
                         font_size=LABEL_FONT_SIZE, color=P_PRIMARY).next_to(both, DOWN, buff=0.3)
        self.play(FadeIn(block_h, shift=RIGHT * 0.2), run_time=0.8)
        self.play(FadeIn(block_k, shift=RIGHT * 0.2), run_time=0.8)
        hold_for(self, N, "spell")

        caption = swap_caption(self, caption, subtitle_text(N, "variiert"))
        self.play(FadeIn(note), run_time=0.6)
        hold_for(self, N, "variiert")

        caption = swap_caption(self, caption, subtitle_text(N, "perk"))
        # Clear both unit cards first — the yellow perk line used to ramp in while SI was still fading (~1 s overlap).
        _hide(self, both, note, run_time=0.45)
        self.remove(both, note)
        k = ValueTracker(0.0)
        perk = math_readout(
            lambda: rf"\dot{{V}} \cdot c_{{\mathrm{{Luft}}}} = {label_number(V_DOT * k.get_value())}\,\mathrm{{\frac{{m^3}}{{h}}}} \cdot {label_number(C_LUFT, 2)}\,\mathrm{{\frac{{Wh}}{{m^3\,K}}}} "
                    rf"\approx {label_number(V_DOT * C_LUFT * k.get_value())}\,\mathrm{{\frac{{W}}{{K}}}}",
            np.array([0.0, 0.55, 0]), size=FORMULA_FONT_SIZE, color=P_YELLOW, edge="center")
        row, box, items = math_panel([
            ("c", r"c_{\mathrm{Luft}}", P_GREEN), (None, "=", P_WHITE),
            (None, rf"\frac{{{label_number(RHO_AIR, 1)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^3}}}} \cdot {label_number(CP_AIR)}\,\frac{{\mathrm{{J}}}}{{\mathrm{{kg\,K}}}}}}{{3600\,\frac{{\mathrm{{s}}}}{{\mathrm{{h}}}}}}", P_WHITE),
            (None, rf"\approx {label_number(C_LUFT, 2)}\,\mathrm{{\frac{{Wh}}{{m^3\,K}}}}", P_GREEN),
        ], edge_buff=2.15)
        self.add(perk)
        self.play(_ramp(k, 1.0), Create(box), FadeIn(row), run_time=1.8)
        hold_for(self, N, "perk")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion



#region Beat2 – Sensible heat flow
class Beat2_SensibleWaerme(Scene):
    """🌡️ Q̇_V,sens = V̇ · c_Luft · Δθ with a signed Δθ — every factor counted from 0, winter then summer."""

    NARRATION = [
        ("intro",
         "Sensible heat changes the air temperature: the exhaust air takes the room's heat out, the supply air brings the outdoor heat in.",
         "Sensible Wärme ändert die Lufttemperatur: Die Abluft nimmt Raumwärme mit, die Zuluft bringt Außenwärme herein."),
        ("delta",
         r"The driving force is the temperature difference $\Delta\theta$, indoor minus outdoor. On our example winter day with $\theta_e = 0\,°\mathrm{C}$ that is $20\,°\mathrm{C} - 0\,°\mathrm{C} = 20\,\mathrm{K}$.",
         r"Treiber ist die Temperaturdifferenz $\Delta\theta$, innen minus außen. Am Beispiel-Wintertag mit $\theta_e = 0\,°\mathrm{C}$ sind das $20\,°\mathrm{C} - 0\,°\mathrm{C} = 20\,\mathrm{K}$."),
        ("formula",
         r"The sensible ventilation heat flow is volume flow times $c_{\mathrm{Luft}}$ times $\Delta\theta$, in watts.",
         r"Der sensible Lüftungswärmestrom $\dot{Q}_{\mathrm{V,sens}}$ ist Volumenstrom mal $c_{\mathrm{Luft}}$ mal $\Delta\theta$, in Watt."),
        ("v",
         r"The volume flow is $V \cdot n$: $440\,\mathrm{m^3} \cdot 0.5\,\mathrm{h^{-1}} = 220\,\mathrm{m^3/h}$.",
         r"Der Volumenstrom ist $V \cdot n$: $440\,\mathrm{m^3} \cdot 0{,}5\,\mathrm{h^{-1}} = 220\,\mathrm{m^3/h}$."),
        ("c",
         r"And $c_{\mathrm{Luft}}$ is the heat capacity of air: $0.34\,\mathrm{Wh/(m^3K)}$.",
         r"Und $c_{\mathrm{Luft}}$ ist die Wärmekapazität der Luft: $0{,}34\,\mathrm{Wh/(m^3K)}$."),
        ("winter",
         r"In winter: $220\,\mathrm{m^3/h} \cdot 0.34\,\mathrm{Wh/(m^3K)} \cdot 20\,\mathrm{K}$ — about $1500\,\mathrm{W}$ leave with the air. Heating counts this as ventilation loss.",
         r"Im Winter: $220\,\mathrm{m^3/h} \cdot 0{,}34\,\mathrm{Wh/(m^3K)} \cdot 20\,\mathrm{K}$ — rund $1500\,\mathrm{W}$ gehen mit der Luft verloren. Die Heizung rechnet das als Lüftungsverlust."),
        ("summer",
         r"In summer: $26\,°\mathrm{C} - 32\,°\mathrm{C}$, so $\Delta\theta = -6\,\mathrm{K}$ and the result $-450\,\mathrm{W}$ — a gain. We quote the magnitudes $6\,\mathrm{K}$ and $450\,\mathrm{W}$.",
         r"Im Sommer: $26\,°\mathrm{C} - 32\,°\mathrm{C}$, also $\Delta\theta = -6\,\mathrm{K}$ und $-450\,\mathrm{W}$ — ein Gewinn. Genannt werden die Beträge $6\,\mathrm{K}$ und $450\,\mathrm{W}$."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Sensible heat", "Sensible Wärme"))
        din = _din_ref(_references.cite("din-en-12831-1", "vdi-2078"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        v, c, th_i, th_e, kq = (ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0), ValueTracker(THETA_E_W),
                                ValueTracker(0.0))
        x0, y_v, y_c, y_d, y_q = 0.35, 1.55, 0.70, -0.15, -1.00

        def d():
            return th_i.get_value() - th_e.get_value()

        def q():
            return v.get_value() * c.get_value() * d() * kq.get_value()

        house = _house(np.array([-3.85, -0.05, 0.0]), 0.85)
        season = {"now": "winter"}
        chip_w = _season_chip("winter")
        chip_w.move_to(np.array([-3.85, 2.3, 0.0]))
        guides = _guides([(house, "winter")])
        self.play(Create(house["group"]), FadeIn(chip_w), run_time=1.5)
        self.play(*_open_both(house), Create(guides), run_time=0.9)

        def air(rt):
            return _air([(house, season["now"])], rt)

        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.9, during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "delta"))
        tx_i, tx_e, ty = -1.25, -0.3, -0.38

        def tag(x, tracker, src, color, word):
            thermo = thermometer_glyph(np.array([x, ty, 0.0]), height=1.35, color=color, level=0.0)
            thermo["column"].add_updater(
                lambda m: thermo["level"].set_value(float(np.clip((tracker.get_value() + 5) / 45, 0.02, 1.0))))
            head = math_label(src, np.array([x, ty + 1.35 + 0.35, 0.0]), size=BODY_FONT_SIZE, color=color)
            read = math_readout(lambda: rf"{label_number(tracker.get_value())}\,°\mathrm{{C}}", np.array([x, ty - 0.5, 0.0]),
                                size=BODY_FONT_SIZE, color=color, edge="center")
            where = body_text(word, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([x, ty - 0.86, 0.0]))
            return {"static": VGroup(thermo["group"], thermo["column"], head, where), "read": read}

        t_i = tag(tx_i, th_i, r"\theta_{\mathrm{i}}", P_ORANGE, tr("indoor", "innen"))
        t_e = tag(tx_e, th_e, r"\theta_{\mathrm{e}}", P_CYAN, tr("outdoor", "außen"))
        dt_name = _name(r"\Delta\theta = \theta_{\mathrm{i}} - \theta_{\mathrm{e}}", tr("Temperature difference", "Temperaturdifferenz"), P_YELLOW)
        dt_row = _row(dt_name, x0, y_d, 2.9, P_YELLOW, lambda: abs(d()) / 25.0)
        dt_read = math_readout(lambda: rf"\Delta\theta = {label_number(d() + 0.0)}\,\mathrm{{K}}",
                               np.array([dt_row["x_end"] + 0.25, y_d, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        self.play(FadeIn(t_i["static"]), FadeIn(t_e["static"]), FadeIn(dt_row["group"]), run_time=0.7)
        self.add(t_i["read"], t_e["read"], dt_read)
        self.play(_ramp(th_i, THETA_I_W), *air(1.6), run_time=1.6)
        hold_for(self, N, "delta", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,sens}}", SENS), (None, "=", P_WHITE),
            ("v", r"\dot{V}", AIR), (None, r"\cdot", P_WHITE), ("c", r"c_{\mathrm{Luft}}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_YELLOW), (None, r"\;\text{in}\;\mathrm{W}", P_WHITE),
        ])
        rest = VGroup(*[m for m in row if m is not items["dt"]])
        self.play(stage.transition("ventilation.heat-flow.sensible.formula-dtheta", source=dt_name[0], target=items["dt"]), *air(1.0), run_time=1.0)
        self.play(FadeIn(rest), Create(box), *air(0.6), run_time=0.6)
        hold_for(self, N, "formula", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "v"))
        v_row = _row(_name(r"\dot{V} = V \cdot n", tr("Volume flow", "Volumenstrom"), AIR), x0, y_v, 2.9, AIR, lambda: v.get_value() / 300.0)
        v_read = math_readout(lambda: rf"\dot{{V}} = {label_number(v.get_value())}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{h}}}}",
                              np.array([v_row["x_end"] + 0.25, y_v, 0]), size=BODY_FONT_SIZE, color=AIR)
        ring = highlight_param(items, "v", color=AIR)
        _show_row(self, v_row, v_read, run_time=0.5)
        self.play(_ramp(v, V_DOT), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "v", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "c"))
        c_row = _row(_name(r"c_{\mathrm{Luft}}", tr("Heat capacity of air", "Wärmekapazität der Luft"), P_GREEN), x0, y_c, 2.9, P_GREEN,
                     lambda: c.get_value() / 0.5)
        c_read = math_readout(lambda: rf"c_{{\mathrm{{Luft}}}} = {label_number(c.get_value(), 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}K}}}}",
                              np.array([c_row["x_end"] + 0.25, y_c, 0]), size=BODY_FONT_SIZE, color=P_GREEN)
        ring = highlight_param(items, "c", color=P_GREEN)
        _show_row(self, c_row, c_read, run_time=0.5)
        self.play(_ramp(c, C_LUFT), Create(ring), *air(1.2), run_time=1.2)
        hold_for(self, N, "c", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        q_name = _name(r"\dot{Q}_{\mathrm{V,sens}}", tr("Loss", "Verlust"), SENS)
        q_row = _row(q_name, x0, y_q, 2.9, SENS, lambda: abs(q()) / 1600.0)
        q_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} \approx {label_number(round(q(), -1))}\,\mathrm{{W}}",
                              np.array([q_row["x_end"] + 0.25, y_q, 0]), size=BODY_FONT_SIZE, color=SENS)
        ring = highlight_param(items, "q", color=SENS)
        _show_row(self, q_row, q_read, run_time=0.5)
        self.play(_ramp(kq, 1.0), Create(ring), *air(1.8), run_time=1.8)
        hold_for(self, N, "winter", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        chip_s = _season_chip("summer")
        chip_s.move_to(chip_w.get_center() + LEFT * (chip_w.width - chip_s.width) / 2)
        gain = body_text(tr("Gain", "Gewinn"), font_size=LABEL_FONT_SIZE, color=SENS).move_to(q_name[1])
        gain.align_to(q_name[1], LEFT)
        season["now"] = "summer"
        self.play(stage.transition("ventilation.heat-flow.sensible.season-chip-summer", source=chip_w, target=chip_s), stage.transition("ventilation.heat-flow.sensible.label-gain", source=q_name[1], target=gain), stage.transition("ventilation.heat-flow.sensible.guides-summer", source=guides, target=_guides([(house, "summer")])), _ramp(th_i, THETA_I_S),
                  _ramp(th_e, THETA_E_S), *air(3.2), run_time=3.2)
        hold_for(self, N, "summer", during=air)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Latent load
class Beat3_LatenteLast(Scene):
    """💧 Q̇_V,lat = ρ · r · Δx · q_v,R — the moisture difference of summer air, counted from 0 into watts."""

    NARRATION = [
        ("intro",
         "Summer air carries more than heat: the humid outdoor air brings water vapour into the room.",
         "Sommerluft trägt mehr als Wärme: Die feuchte Außenluft bringt Wasserdampf in den Raum."),
        ("dx",
         r"Outside $32\,°\mathrm{C}$/$50\,\%$: $15\,\mathrm{g/kg}$; room $26\,°\mathrm{C}$: $10.5\,\mathrm{g/kg}$.\n$\Delta x = 4.5\,\mathrm{g/kg}$ — humidity drives the latent flow as $\Delta\theta$ drives the sensible one.",
         r"Außen $32\,°\mathrm{C}$/$50\,\%$: $15\,\mathrm{g/kg}$; Raum $26\,°\mathrm{C}$: $10{,}5\,\mathrm{g/kg}$.\n$\Delta x = 4{,}5\,\mathrm{g/kg}$ — Feuchte treibt den latenten Strom wie $\Delta\theta$ den sensiblen."),
        ("formula",
         r"The latent ventilation load $\dot{Q}_{\mathrm{V,lat}}$ is density $\rho$ times latent heat $r$ times $\Delta x$ times volume flow $q_{v,\mathrm{R}}$.",
         r"Die latente Lüftungslast $\dot{Q}_{\mathrm{V,lat}}$ ist Dichte $\rho$ mal Verdampfungswärme $r$ mal $\Delta x$ mal Volumenstrom $q_{v,\mathrm{R}}$."),
        ("water",
         r"Per cubic metre that is $1.2\,\mathrm{kg/m^3} \cdot 4.5\,\mathrm{g/kg} = 5.4\,\mathrm{g/m^3}$ of water that must be condensed.",
         r"Je Kubikmeter sind das $1{,}2\,\mathrm{kg/m^3} \cdot 4{,}5\,\mathrm{g/kg} = 5{,}4\,\mathrm{g/m^3}$ Wasser, die kondensiert werden müssen."),
        ("r",
         r"Changing phase involves energy: $r$ is about $2450\,\mathrm{kJ/kg}$ — condensing releases it, and it must be removed again.",
         r"Der Phasenwechsel benötigt $r \approx 2450\,\mathrm{kJ/kg}$. Kondensation setzt diese Energie frei; sie muss abgeführt werden."),
        ("qv",
         r"The air flow of $220\,\mathrm{m^3/h}$ is $0.061\,\mathrm{m^3/s}$.",
         r"Der Luftstrom von $220\,\mathrm{m^3/h}$ sind $0{,}061\,\mathrm{m^3/s}$."),
        ("result",
         r"Together: about $810\,\mathrm{W}$ of latent load — more than the $450\,\mathrm{W}$ of sensible heat.",
         r"Zusammen: rund $810\,\mathrm{W}$ latente Last — mehr als die $450\,\mathrm{W}$ sensible Wärme."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Latent load from moisture", "Latente Last durch Feuchte"))
        din = _din_ref(_references.cite("vdi-2078"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        x_out, x_in, kw, kr = (ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0))
        kq, kv = ValueTracker(0.0), ValueTracker(0.0)
        x0, y_w, y_r, y_v, y_q = -1.2, 1.68, 0.80, -0.08, -0.93

        house = _house(np.array([-4.1, -0.05, 0.0]), 0.8)
        sun = sun_glyph(np.array([-6.3, 1.65, 0.0])).scale(0.4)
        chip_s = _season_chip("summer")
        chip_s.move_to(np.array([-4.1, 2.3, 0.0]))
        self.play(Create(house["group"]), FadeIn(sun), FadeIn(chip_s), run_time=1.4)
        self.play(*_open_both(house), Create(_guides([(house, "summer")])), run_time=0.9)

        def air(rt):
            return _air([(house, "summer")], rt, moist=True)

        col_h, y_b, cx_o, cx_i, bx = 2.1, -0.95, -0.2, 4.2, 1.55

        def column(cx):
            return Rectangle(width=0.9, height=col_h, color=P_WHITE, stroke_width=_theme_module.CHROME_BORDER).move_to(
                np.array([cx, y_b + col_h / 2, 0.0]))

        def level(g):
            return y_b + 0.05 + (col_h - 0.1) * g / 16.0

        def fill(cx, lo, hi, color, opacity):
            h = max(0.004, level(hi()) - level(lo()))
            return Rectangle(width=0.8, height=h, stroke_width=0, fill_color=color, fill_opacity=opacity).move_to(
                np.array([cx, level(lo()) + h / 2, 0.0]))

        base_o = always_redraw(lambda: fill(cx_o, lambda: 0.0, lambda: min(x_out.get_value(), X_IN), LATENT, 0.55))
        extra_o = always_redraw(lambda: fill(cx_o, lambda: min(x_out.get_value(), X_IN), lambda: x_out.get_value(), P_BLUE, 0.9))
        base_i = always_redraw(lambda: fill(cx_i, lambda: 0.0, lambda: x_in.get_value(), LATENT, 0.55))
        cols = VGroup(column(cx_o), column(cx_i))

        def head(cx, word, src):
            return VGroup(body_text(word, font_size=LABEL_FONT_SIZE, color=P_WHITE),
                          math_label(src, size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(DOWN, buff=0.08).move_to(
                np.array([cx, y_b + col_h + 0.52, 0.0]))

        heads = VGroup(head(cx_o, tr("Outdoor air", "Außenluft"), tr(rf"{THETA_E_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{RH}}", rf"{THETA_E_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{r.F.}}")),
                       head(cx_i, tr("Room air", "Raumluft"), tr(rf"{THETA_I_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{RH}}", rf"{THETA_I_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{r.F.}}")))
        read_o = math_readout(lambda: rf"x = {label_number(x_out.get_value(), 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", np.array([cx_o, y_b - 0.3, 0]),
                              size=BODY_FONT_SIZE, color=LATENT, edge="center")
        read_i = math_readout(lambda: rf"x = {label_number(x_in.get_value(), 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", np.array([cx_i, y_b - 0.3, 0]),
                              size=BODY_FONT_SIZE, color=LATENT, edge="center")
        self.play(Create(cols), FadeIn(heads), *air(0.8), run_time=0.8)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.9 + 0.8, during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "dx"))
        self.add(base_o, extra_o, base_i, read_o, read_i)
        self.play(_ramp(x_out, X_OUT), _ramp(x_in, X_IN), *air(2.4), run_time=2.4)
        y_hi, y_lo = level(X_OUT), level(X_IN)
        guide_o = DashedLine(np.array([cx_o + 0.45, y_hi, 0]), np.array([bx + 0.3, y_hi, 0]), color=P_BLUE, stroke_width=1.6)
        guide_i = DashedLine(np.array([cx_i - 0.45, y_lo, 0]), np.array([bx - 0.3, y_lo, 0]), color=P_BLUE, stroke_width=1.6)
        guide_o_full = DashedLine(np.array([cx_o + 0.45, y_lo, 0]), np.array([bx - 0.3, y_lo, 0]), color=P_BLUE, stroke_width=1.6)
        brace = dim_arrow(np.array([bx, y_lo, 0]), np.array([bx, y_hi, 0]), color=P_BLUE)
        dx_lbl = math_label(rf"\Delta x = {label_number(D_X, 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", np.array([bx + 0.3, (y_lo + y_hi) / 2, 0]),
                            size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        self.play(Create(guide_o), Create(guide_o_full), Create(brace), FadeIn(dx_lbl), *air(1.0), run_time=1.0)
        hold_for(self, N, "dx", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,lat}}", LATENT), (None, "=", P_WHITE),
            ("rho", r"\rho", P_WHITE), (None, r"\cdot", P_WHITE), ("r", "r", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dx", r"\Delta x", P_BLUE), (None, r"\cdot", P_WHITE), ("qv", r"q_{v,\mathrm{R}}", AIR),
            (None, r"\;\text{in}\;\mathrm{W}", P_WHITE),
        ])
        rest = VGroup(*[m for m in row if m is not items["dx"]])
        self.play(stage.transition("ventilation.heat-flow.latent.formula-dx", source=dx_lbl, target=items["dx"]), *air(1.0), run_time=1.0)
        self.play(FadeIn(rest), Create(box), *air(0.6), run_time=0.6)
        hold_for(self, N, "formula", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "water"))
        w_name = _name(r"\rho \cdot \Delta x", tr("Water per m³", "Wasser je m³"), P_BLUE)
        w_row = _row(w_name, x0, y_w, 3.3, P_BLUE, lambda: kw.get_value() * WATER_PER_M3 / 6.0)
        w_read = math_readout(lambda: rf"\rho \cdot \Delta x = {label_number(RHO_AIR, 1)}\,\mathrm{{\frac{{kg}}{{m^3}}}} \cdot {label_number(D_X, 1)}\,\mathrm{{\frac{{g}}{{kg}}}} = {label_number(WATER_PER_M3 * kw.get_value(), 1)}\,\mathrm{{\frac{{g}}{{m^3}}}}",
                              np.array([w_row["x_end"] + 0.25, y_w, 0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        _hide(self, cols, heads, base_o, extra_o, base_i, read_o, read_i, guide_o, guide_i, guide_o_full, brace, dx_lbl)
        ring = highlight_param(items, "dx", color=P_BLUE)
        _show_row(self, w_row, w_read, run_time=0.5)
        self.play(_ramp(kw, 1.0), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "water", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "r"))
        r_row = _row(_name("r", tr("Heat of vaporisation", "Verdampfungswärme"), P_ORANGE), x0, y_r, 3.3, P_ORANGE, lambda: kr.get_value() * R_LAT / 3000.0)
        r_read = math_readout(lambda: rf"r = {label_number(R_LAT * kr.get_value())}\,\frac{{\mathrm{{kJ}}}}{{\mathrm{{kg}}}}",
                              np.array([r_row["x_end"] + 0.25, y_r, 0]), size=BODY_FONT_SIZE, color=P_ORANGE)
        ring = highlight_param(items, "r", color=P_ORANGE)
        _show_row(self, r_row, r_read, run_time=0.5)
        self.play(_ramp(kr, 1.0), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "r", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "qv"))
        v_row = _row(_name(r"q_{v,\mathrm{R}} = \dot{V}/3600", tr("Volume flow", "Volumenstrom"), AIR), x0, y_v, 3.3, AIR, lambda: kv.get_value() * Q_V_R / 0.1)
        v_read = math_readout(lambda: rf"q_{{v,\mathrm{{R}}}} = {label_number(Q_V_R * kv.get_value(), 3)}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{s}}}}",
                              np.array([v_row["x_end"] + 0.25, y_v, 0]), size=BODY_FONT_SIZE, color=AIR)
        ring = highlight_param(items, "qv", color=AIR)
        _show_row(self, v_row, v_read, run_time=0.5)
        self.play(_ramp(kv, 1.0), Create(ring), *air(1.2), run_time=1.2)
        hold_for(self, N, "qv", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "result"))
        q_row = _row(_name(r"\dot{Q}_{\mathrm{V,lat}}", tr("Latent load", "Latente Last"), LATENT), x0, y_q, 3.3, LATENT,
                     lambda: kq.get_value() * Q_LAT / 1000.0)
        q_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} \approx {label_number(round(Q_LAT * kq.get_value(), -1))}\,\mathrm{{W}}",
                              np.array([q_row["x_end"] + 0.25, y_q, 0]), size=BODY_FONT_SIZE, color=LATENT)
        tick_x = x0 + 0.04 + (3.3 - 0.08) * W_SUMMER / 1000.0
        tick = Line(np.array([tick_x, y_q - 0.2, 0]), np.array([tick_x, y_q + 0.2, 0]), color=SENS, stroke_width=3)
        tick_lbl = math_label(rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {label_number(W_SUMMER)}\,\mathrm{{W}}", np.array([tick_x, y_q - 0.34, 0]),
                              size=LABEL_FONT_SIZE - 2, color=SENS)
        ring = highlight_param(items, "q", color=LATENT)
        _show_row(self, q_row, q_read, run_time=0.5)
        self.play(_ramp(kq, 1.0), Create(ring), *air(1.8), run_time=1.8)
        self.play(Create(tick), FadeIn(tick_lbl), *air(0.6), run_time=0.6)
        hold_for(self, N, "result", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – One formula, two seasons
class Beat4_WinterUndSommer(Scene):
    """📊 The same house in both seasons — the sensible and latent flows stacked as bars, then handed over."""

    NARRATION = [
        ("intro",
         "One house, one formula, two seasons: the air carries heat in both, but in opposite directions.",
         "Ein Haus, eine Formel, zwei Jahreszeiten: Die Luft trägt in beiden Wärme, aber in entgegengesetzte Richtungen."),
        ("winter",
         r"On the example winter day $\Delta\theta = 20\,\mathrm{K}$ drives $1500\,\mathrm{W}$ of sensible heat out.",
         r"Am Beispiel-Wintertag treibt $\Delta\theta = 20\,\mathrm{K}$ rund $1500\,\mathrm{W}$ sensible Wärme hinaus."),
        ("winter_lat",
         r"Moisture leaves with the exhaust air as well, about $770\,\mathrm{W}$ latent, but by convention the heating load counts only the sensible part.",
         r"Auch Feuchte geht mit der Abluft verloren, rund $770\,\mathrm{W}$ latent, doch die Heizlast zählt per Konvention nur den sensiblen Anteil."),
        ("humid",
         "Humidifying the supply air would cost the latent part.",
         "Das Befeuchten der Zuluft würde den latenten Anteil kosten."),
        ("sens",
         r"In summer only $\Delta\theta = 6\,\mathrm{K}$ drives $450\,\mathrm{W}$ of sensible heat in.",
         r"Im Sommer treibt nur $\Delta\theta = 6\,\mathrm{K}$ rund $450\,\mathrm{W}$ sensible Wärme herein."),
        ("lat",
         r"On top comes the latent load of $810\,\mathrm{W}$ from the moisture — together $\dot{Q}_{\mathrm{V}} = 1260\,\mathrm{W}$.",
         r"Dazu kommt die latente Last von $810\,\mathrm{W}$ aus der Feuchte — zusammen $\dot{Q}_{\mathrm{V}} = 1260\,\mathrm{W}$."),
        ("use",
         r"Heating takes the winter value as its ventilation loss $\dot{Q}_{\mathrm{V}}$, Cooling takes both summer shares as its ventilation load $\dot{Q}_{\mathrm{V}}$.",
         r"Die Heizung übernimmt den Winterwert als Lüftungsverlust $\dot{Q}_{\mathrm{V}}$, die Kühltechnik beide Sommeranteile als Lüftungslast $\dot{Q}_{\mathrm{V}}$."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("The same formula, two seasons", "Dieselbe Formel, zwei Jahreszeiten"))
        din = _din_ref(_references.cite("din-en-12831-1", "vdi-2078"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        unit, y_base, bar_w = 1.9 / 1500.0, -0.95, 1.0

        house_w = _house(np.array([-4.9, -0.17, 0.0]), 0.6, 0.95)
        house_s = _house(np.array([2.3, -0.17, 0.0]), 0.6, 0.95)
        chip_w, chip_s = _season_chip("winter"), _season_chip("summer")
        chip_w.move_to(np.array([-4.5, 2.35, 0.0]))
        chip_s.move_to(np.array([3.3, 2.35, 0.0]))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V}}", P_WHITE), (None, "=", P_WHITE),
            ("s", r"\dot{Q}_{\mathrm{V,sens}}", SENS), (None, "+", P_WHITE), ("l", r"\dot{Q}_{\mathrm{V,lat}}", LATENT),
            (None, r"\;\text{in}\;\mathrm{W}", P_WHITE),
        ])
        self.play(Create(house_w["group"]), Create(house_s["group"]), FadeIn(chip_w), FadeIn(chip_s), run_time=1.4)
        self.play(*_open_both(house_w), *_open_both(house_s), Create(_guides([(house_w, "winter"), (house_s, "summer")])),
                  run_time=0.8)

        def air(rt):
            return _air([(house_w, "winter"), (house_s, "summer")], rt)

        self.play(FadeIn(row), Create(box), *air(0.8), run_time=0.8)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.8 + 0.8, during=air)

        def bar(x, segments):
            return stacked_bar(segments, base=np.array([x, y_base, 0.0]), width=bar_w, unit=unit)

        def side(bars, k, x, src, driver, color):
            mid = bars["anchors"][k]
            label = VGroup(math_label(src, size=LABEL_FONT_SIZE, color=color),
                           math_label(driver, size=LABEL_FONT_SIZE, color=P_YELLOW if "theta" in driver else P_BLUE)).arrange(
                DOWN, buff=0.1, aligned_edge=LEFT)
            label.move_to(np.array([x + label.width / 2, mid[1], 0.0]))
            return label

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        bars_w = bar(-2.9, [("sens", W_WINTER, SENS), ("lat", W_LATENT_WINTER, LATENT)])
        lab_w = side(bars_w, 0, -2.25, rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {label_number(W_WINTER)}\,\mathrm{{W}}",
                     rf"\Delta\theta = {label_number(THETA_I_W - THETA_E_W)}\,\mathrm{{K}}", SENS)
        self.play(GrowFromEdge(bars_w["bars"][0], DOWN), *air(1.6), run_time=1.6)
        note_w = VGroup(body_text(tr("by convention", "per Konvention"), font_size=LABEL_FONT_SIZE - 2, color=LATENT),
                        body_text(tr("not in the heating load", "nicht in der Heizlast"), font_size=LABEL_FONT_SIZE - 2, color=LATENT)).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        lab_wl = side(bars_w, 1, -2.25, rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} = {label_number(W_LATENT_WINTER)}\,\mathrm{{W}}",
                      rf"\Delta x = {label_number(D_X_W, 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", LATENT)
        note_w.set_opacity(0.75).next_to(lab_wl, DOWN, buff=0.12, aligned_edge=LEFT)
        self.play(FadeIn(lab_w), *air(0.5), run_time=0.5)
        hold_for(self, N, "winter", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "winter_lat"))
        self.play(GrowFromEdge(bars_w["bars"][1], DOWN), FadeIn(lab_wl), FadeIn(note_w), *air(1.2), run_time=1.2)
        hold_for(self, N, "winter_lat", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "humid"))
        hold_for(self, N, "humid", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        bars_s = bar(4.2, [("sens", W_SUMMER, SENS), ("lat", W_LATENT, LATENT)])
        lab_s = side(bars_s, 0, 4.85, rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {label_number(W_SUMMER)}\,\mathrm{{W}}",
                     tr(rf"\Delta\theta = {label_number(THETA_E_S - THETA_I_S)}\,\mathrm{{K}}\;\text{{(gain)}}", rf"\Delta\theta = {label_number(THETA_E_S - THETA_I_S)}\,\mathrm{{K}}\;\text{{(Gewinn)}}"), SENS)
        self.play(GrowFromEdge(bars_s["bars"][0], DOWN), *air(1.2), run_time=1.2)
        self.play(FadeIn(lab_s), *air(0.5), run_time=0.5)
        hold_for(self, N, "sens", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        lab_l = side(bars_s, 1, 4.85, rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} = {label_number(W_LATENT)}\,\mathrm{{W}}",
                     rf"\Delta x = {label_number(D_X, 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", LATENT)
        self.play(GrowFromEdge(bars_s["bars"][1], DOWN), *air(1.4), run_time=1.4)
        self.play(FadeIn(lab_l), *air(0.5), run_time=0.5)
        hold_for(self, N, "lat", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "use"))
        to_h = _handover_chip(tr("→ Heating:", "→ Heizung:"), r"\dot{Q}_{\mathrm{V}}", HEIZUNG)
        to_k = _handover_chip(tr("→ Cooling:", "→ Kühltechnik:"), r"\dot{Q}_{\mathrm{V}}", KUEHLUNG)
        to_h.move_to(np.array([-5.0, 1.72, 0.0]))
        to_k.move_to(np.array([3.3, 1.72, 0.0]))
        self.play(FadeIn(to_h, shift=DOWN * 0.15), *air(0.8), run_time=0.8)
        self.play(FadeIn(to_k, shift=DOWN * 0.15), *air(0.8), run_time=0.8)
        hold_for(self, N, "use", during=air)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
