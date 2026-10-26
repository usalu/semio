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


import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
scene_title = _manim_module.scene_title
play_scene_title = _manim_module.play_scene_title
TITLE_RUN_TIME = _manim_module.TITLE_RUN_TIME
beat_subtitle = _manim_module.beat_subtitle
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
body_text = _manim_module.body_text
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
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
de_num = _manim_module.de_num
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
flow_guides = _manim_module.flow_guides
dim_arrow = _manim_module.dim_arrow
chip = _manim_module.chip
meter = _manim_module.meter
set_meter = _manim_module.set_meter
stacked_bar = _manim_module.stacked_bar
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing
house_section = _manim_module.house_section
open_window = _manim_module.open_window
radiator = _manim_module.radiator
sun_glyph = _manim_module.sun_glyph
thermometer_glyph = _manim_module.thermometer_glyph
ripples = _manim_module.ripples

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "⏱️timing" / "🔣️.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent chapter title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Was die Luft trägt"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the Heating and Cooling series: a dim ``P_TEAL``
    footnote that never competes with the diagram.
    """
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
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

AIR = "#60A5FA"
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
    return chip("Winter" if winter else "Sommer", HEIZUNG if winter else KUEHLUNG)


def _handover_chip(prefix: str, src: str, color: str):
    """🔗 Hand-over chip with a typeset quantity — names what the later video takes from this chapter."""
    parts = VGroup(body_text(prefix, font_size=LABEL_FONT_SIZE + 2, color=color),
                   math_label(src, size=LABEL_FONT_SIZE + 2, color=color)).arrange(RIGHT, buff=0.12)
    box = SurroundingRectangle(parts, color=color, corner_radius=0.1, buff=0.1, stroke_width=1.8)
    return VGroup(box, parts)
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
         "One cubic metre of air has a mass of about 1.2 kilograms.",
         "Ein Kubikmeter Luft hat eine Masse von etwa 1,2 Kilogramm."),
        ("coil",
         "A heating element warms that cubic metre from below.",
         "Ein Heizelement erwärmt diesen Kubikmeter von unten."),
        ("heat",
         "From 20 to 21 degrees it takes 1.2 times 1005, that is 1206 joules.",
         "Von 20 auf 21 °C braucht es 1,2 mal 1005, also 1206 Joule."),
        ("c_luft",
         "The energy per cubic metre and kelvin is the volumetric heat capacity of air, c Luft — about 0.34 watt-hours.",
         "Die Energie je Kubikmeter und Kelvin ist die volumenbezogene Wärmekapazität der Luft, c-Luft — etwa 0,34 Wattstunden."),
        ("spell",
         "Heating and Cooling spell the same quantity differently: c Luft in watt-hours, or rho times c p in joules.",
         "Heizung und Kühlung schreiben dieselbe Größe verschieden: c-Luft in Wh/(m³·K) oder ρ mal c-p in J/(m³·K)."),
        ("perk",
         "So 220 cubic metres per hour carry about 75 watts for every kelvin of temperature difference.",
         "Damit tragen 220 m³/h je Kelvin Temperaturunterschied rund 75 Watt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Wärmekapazität der Luft", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "cube"))
        self.play(FadeIn(caption), run_time=0.3)

        #region cube
        c_center = UP * 0.55
        cube = _cube_faces(c_center, 1.0, P_BLUE, warm=False)
        cube_label = math_label(r"1\,\mathrm{m^{3}}", size=BODY_FONT_SIZE).move_to(cube[0].get_center() + UP * 0.15)
        self.play(FadeIn(cube, shift=UP * 0.3), FadeIn(cube_label), run_time=2.0)
        hold_for(self, N, "cube", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 2.0)
        #endregion

        #region mass
        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        xl = -6.45
        mass = ValueTracker(0.0)
        rho_lbl = math_label(rf"\rho = {de_num(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}}", np.array([xl, 1.75, 0]),
                             size=BODY_FONT_SIZE, color=P_CYAN, edge="left")
        m_read = math_readout(lambda: rf"m = \rho \cdot 1\,\mathrm{{m^{{3}}}} = {de_num(mass.get_value(), 2)}\,\mathrm{{kg}}",
                              np.array([xl, 1.2, 0]), size=BODY_FONT_SIZE, color=P_CYAN)
        self.play(FadeIn(rho_lbl), run_time=0.5)
        self.add(m_read)
        self.play(mass.animate.set_value(RHO_AIR), Indicate(cube, color=P_CYAN, scale_factor=1.04), run_time=1.4)
        hold_for(self, N, "mass")
        #endregion

        #region coil and warming
        caption = swap_caption(self, caption, subtitle_text(N, "coil"))
        coil = radiator([0, -0.84, 0], color=P_RED).scale(1.05)
        coil_label = body_text("Heizelement", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(coil, DOWN, buff=0.1)
        heat_spots = [np.array([x, coil.get_top()[1] + 0.03, 0]) for x in (-0.36, 0.0, 0.36)]
        rise = [smooth_path([np.array([s * x0, y0, 0]), np.array([s * (x0 - 0.08), 0.15, 0]), np.array([s * x0, y1, 0])])
                for s in (-1, 1) for x0, y0, y1 in ((0.55, -0.05, 0.62), (0.2, -0.26, 0.45))]

        def warming(rt, convect=True):
            heat = [ripples(heat_spots, r_max=0.42, rings=2, cycles=rt / 1.2, color=P_RED)]
            return heat + ([flow_animation([(rise, P_BLUE, P_ORANGE)], waves=3, cycles=0.45 * rt)] if convect else [])

        cp_lbl = math_label(rf"c_{{p}} = {de_num(CP_AIR)}\,\mathrm{{J/(kg\,K)}}", np.array([xl, 0.55, 0]),
                            size=BODY_FONT_SIZE, color=P_RED, edge="left")
        self.play(Create(coil), FadeIn(coil_label), FadeIn(cp_lbl), run_time=1.5)
        hold_for(self, N, "coil", during=lambda rt: warming(rt, convect=False))

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        theta = ValueTracker(20.0)
        thermo = thermometer_glyph(np.array([-1.65, -0.22, 0]), height=1.8, color=P_RED, level=0.35)
        thermo["column"].add_updater(lambda m: thermo["level"].set_value(0.35 + 0.45 * (theta.get_value() - 20.0)))
        th_read = math_readout(lambda: rf"\theta = {de_num(theta.get_value(), 1)}\,°\mathrm{{C}}", np.array([xl, -0.1, 0]),
                               size=BODY_FONT_SIZE, color=P_ORANGE)
        q_read = math_readout(
            lambda: rf"Q = {de_num(RHO_AIR, 1)} \cdot {de_num(CP_AIR)} \cdot {de_num(theta.get_value() - 20.0, 1)} = "
                    rf"{de_num(RHO_AIR * CP_AIR * (theta.get_value() - 20.0))}\,\mathrm{{J}}",
            np.array([xl, -0.75, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        wh_read = math_readout(
            lambda: rf"= {de_num(RHO_AIR * CP_AIR * (theta.get_value() - 20.0) / 3600.0, 3)}\,\mathrm{{Wh}}",
            np.array([xl + 0.42, -1.3, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        self.play(FadeIn(thermo["group"]), FadeIn(thermo["column"]), *warming(0.4, convect=False), run_time=0.4)
        self.add(th_read, q_read, wh_read)
        warm = _cube_faces(c_center, 1.0, P_ORANGE, warm=True)
        self.play(*[Transform(a, b) for a, b in zip(cube, warm)], theta.animate.set_value(21.0), *warming(3.0),
                  run_time=3.0, rate_func=linear)
        hold_for(self, N, "heat", during=warming)
        #endregion

        #region c_Luft
        caption = swap_caption(self, caption, subtitle_text(N, "c_luft"))
        xr = 1.55
        c_name = body_text("Volumenbezogene Wärmekapazität", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        c_name.move_to([xr + c_name.width / 2, 1.75, 0])
        c_def = math_label(r"c_{\mathrm{Luft}} = \frac{\rho \cdot c_{p}}{3600\,\mathrm{s/h}}", np.array([xr, 1.0, 0]),
                           size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        c_j = math_label(rf"= {de_num(RHO_AIR * CP_AIR)}\,\mathrm{{J/(m^{{3}}K)}}", np.array([xr + 0.75, 0.3, 0]),
                         size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        c_wh = math_label(rf"\approx {de_num(C_LUFT, 2)}\,\mathrm{{Wh/(m^{{3}}K)}}", np.array([xr + 0.75, -0.3, 0]),
                          size=BODY_FONT_SIZE, color=P_GREEN, edge="left")
        q_snap = math_label(rf"{de_num(RHO_AIR * CP_AIR)}\,\mathrm{{J}}", size=BODY_FONT_SIZE, color=P_YELLOW)
        q_snap.move_to(q_read.get_right() + LEFT * q_snap.width / 2)
        wh_snap = math_label(rf"{de_num(RHO_AIR * CP_AIR / 3600.0, 3)}\,\mathrm{{Wh}}", size=BODY_FONT_SIZE, color=P_YELLOW)
        wh_snap.move_to(wh_read.get_right() + LEFT * wh_snap.width / 2)
        self.play(FadeIn(c_name), FadeIn(c_def), *warming(0.7), run_time=0.7)
        self.play(TransformFromCopy(q_snap, c_j), *warming(0.9), run_time=0.9)
        self.play(TransformFromCopy(wh_snap, c_wh), *warming(0.9), run_time=0.9)
        hold_for(self, N, "c_luft", during=warming)
        #endregion

        #region two spellings
        caption = swap_caption(self, caption, subtitle_text(N, "spell"))
        _hide(self, cube, cube_label, rho_lbl, m_read, coil, coil_label, cp_lbl, thermo["group"], thermo["column"],
              th_read, q_read, wh_read, c_name, c_def, c_j, c_wh)
        heiz_row, heiz = math_row([
            ("v", r"\dot{V}\,[\mathrm{m^{3}/h}]", AIR), (None, r"\cdot", P_WHITE),
            ("c", r"c_{\mathrm{Luft}}\,[\mathrm{Wh/(m^{3}\,K)}]", P_GREEN), (None, r"=\,\mathrm{W/K}", P_WHITE),
        ], font_size=FORMULA_FONT_SIZE - 5)
        kuehl_row, kuehl = math_row([
            ("q", r"q_{v,\mathrm{R}}\,[\mathrm{m^{3}/s}]", AIR), (None, r"\cdot", P_WHITE),
            ("c", r"\rho \cdot c_{p}\,[\mathrm{J/(m^{3}\,K)}]", P_GREEN), (None, r"=\,\mathrm{W/K}", P_WHITE),
        ], font_size=FORMULA_FONT_SIZE - 5)
        chip_h, chip_k = chip("Heizung", HEIZUNG), chip("Kühlung", KUEHLUNG)
        line_h = VGroup(chip_h, heiz_row).arrange(RIGHT, buff=0.45)
        line_k = VGroup(chip_k, kuehl_row).arrange(RIGHT, buff=0.45)
        both = VGroup(line_h, line_k).arrange(DOWN, aligned_edge=LEFT, buff=0.55).move_to(UP * 1.25)
        both.set_x(0)
        self.play(FadeIn(line_h, shift=RIGHT * 0.2), run_time=0.8)
        self.play(FadeIn(line_k, shift=RIGHT * 0.2), run_time=0.8)
        ring_h = highlight_param(heiz, "c", color=P_GREEN)
        ring_k = highlight_param(kuehl, "c", color=P_GREEN)
        self.play(Create(ring_h), Create(ring_k), run_time=0.6)
        hold_for(self, N, "spell")
        #endregion

        #region watts per kelvin
        caption = swap_caption(self, caption, subtitle_text(N, "perk"))
        k = ValueTracker(0.0)
        perk = math_readout(
            lambda: rf"\dot{{V}} \cdot c_{{\mathrm{{Luft}}}} = {de_num(V_DOT * k.get_value())} \cdot {de_num(C_LUFT, 2)} "
                    rf"\approx {de_num(V_DOT * C_LUFT * k.get_value())}\,\mathrm{{W/K}}",
            np.array([0.0, -0.4, 0]), size=FORMULA_FONT_SIZE, color=P_YELLOW, edge="center")
        row, box, items = math_panel([
            ("c", r"c_{\mathrm{Luft}}", P_GREEN), (None, "=", P_WHITE),
            (None, r"\frac{\rho \cdot c_{p}}{3600\,\mathrm{s/h}}", P_WHITE),
            (None, rf"\approx {de_num(C_LUFT, 2)}\,\mathrm{{Wh/(m^{{3}}\,K)}}", P_GREEN),
        ])
        self.add(perk)
        self.play(_ramp(k, 1.0), FadeOut(VGroup(ring_h, ring_k)), Create(box), FadeIn(row), run_time=1.8)
        hold_for(self, N, "perk")
        #endregion

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
         "The driving force is the temperature difference delta theta, indoor minus outdoor. In winter that is 20 minus 0, so 20 kelvin.",
         "Treiber ist die Temperaturdifferenz Delta-Theta, innen minus außen. Im Winter sind das 20 minus 0, also 20 Kelvin."),
        ("formula",
         "The sensible ventilation heat flow is volume flow times c Luft times delta theta, in watts.",
         "Der sensible Lüftungswärmestrom ist Volumenstrom mal c-Luft mal Delta-Theta, in Watt."),
        ("v",
         "The volume flow is V times n: 440 times 0.5 gives 220 cubic metres per hour.",
         "Der Volumenstrom ist V mal n: 440 mal 0,5 ergibt 220 Kubikmeter pro Stunde."),
        ("c",
         "And c Luft is the heat capacity of air: 0.34 watt-hours per cubic metre and kelvin.",
         "Und c-Luft ist die Wärmekapazität der Luft: 0,34 Wattstunden je Kubikmeter und Kelvin."),
        ("winter",
         "In winter: 220 times 0.34 times 20 — about 1500 watts leave with the air. Heating counts this as ventilation loss.",
         "Im Winter: 220 mal 0,34 mal 20 — rund 1500 Watt gehen mit der Luft verloren. Die Heizung rechnet das als Lüftungsverlust."),
        ("summer",
         "In summer: 26 minus 32, so delta theta is minus 6 kelvin and the result minus 450 watts — a gain. We quote the magnitudes 6 K and 450 W.",
         "Im Sommer: 26 minus 32, also Delta-Theta minus 6 Kelvin und minus 450 Watt — ein Gewinn. Genannt werden die Beträge 6 K und 450 W."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Sensible Wärme", title)
        din = _din_ref("DIN EN 12831-1 · VDI 2078")
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

        #region house and winter streams
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
        #endregion

        #region temperatures and delta theta
        caption = swap_caption(self, caption, subtitle_text(N, "delta"))
        tx_i, tx_e, ty = -1.25, -0.3, -0.38

        def tag(x, tracker, src, color, word):
            thermo = thermometer_glyph(np.array([x, ty, 0.0]), height=1.35, color=color, level=0.0)
            thermo["column"].add_updater(
                lambda m: thermo["level"].set_value(float(np.clip((tracker.get_value() + 5) / 45, 0.02, 1.0))))
            head = math_label(src, np.array([x, ty + 1.35 + 0.35, 0.0]), size=BODY_FONT_SIZE, color=color)
            read = math_readout(lambda: rf"{de_num(tracker.get_value())}\,°\mathrm{{C}}", np.array([x, ty - 0.5, 0.0]),
                                size=BODY_FONT_SIZE, color=color, edge="center")
            where = body_text(word, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([x, ty - 0.86, 0.0]))
            return {"static": VGroup(thermo["group"], thermo["column"], head, where), "read": read}

        t_i = tag(tx_i, th_i, r"\theta_{\mathrm{i}}", P_ORANGE, "innen")
        t_e = tag(tx_e, th_e, r"\theta_{\mathrm{e}}", P_CYAN, "außen")
        dt_name = _name(r"\Delta\theta = \theta_{\mathrm{i}} - \theta_{\mathrm{e}}", "Temperaturdifferenz", P_YELLOW)
        dt_row = _row(dt_name, x0, y_d, 2.9, P_YELLOW, lambda: abs(d()) / 25.0)
        dt_read = math_readout(lambda: rf"\Delta\theta = {de_num(d() + 0.0)}\,\mathrm{{K}}",
                               np.array([dt_row["x_end"] + 0.25, y_d, 0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        self.play(FadeIn(t_i["static"]), FadeIn(t_e["static"]), FadeIn(dt_row["group"]), run_time=0.7)
        self.add(t_i["read"], t_e["read"], dt_read)
        self.play(_ramp(th_i, THETA_I_W), *air(1.6), run_time=1.6)
        hold_for(self, N, "delta", during=air)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,sens}}", SENS), (None, "=", P_WHITE),
            ("v", r"\dot{V}", AIR), (None, r"\cdot", P_WHITE), ("c", r"c_{\mathrm{Luft}}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_YELLOW), (None, r"\;[\mathrm{W}]", P_WHITE),
        ])
        rest = VGroup(*[m for m in row if m is not items["dt"]])
        self.play(TransformFromCopy(dt_name[0], items["dt"]), *air(1.0), run_time=1.0)
        self.play(FadeIn(rest), Create(box), *air(0.6), run_time=0.6)
        hold_for(self, N, "formula", during=air)
        #endregion

        #region V̇ and c_Luft
        caption = swap_caption(self, caption, subtitle_text(N, "v"))
        v_row = _row(_name(r"\dot{V} = V \cdot n", "Volumenstrom", AIR), x0, y_v, 2.9, AIR, lambda: v.get_value() / 300.0)
        v_read = math_readout(lambda: rf"\dot{{V}} = {de_num(v.get_value())}\,\mathrm{{m^{{3}}/h}}",
                              np.array([v_row["x_end"] + 0.25, y_v, 0]), size=BODY_FONT_SIZE, color=AIR)
        ring = highlight_param(items, "v", color=AIR)
        _show_row(self, v_row, v_read, run_time=0.5)
        self.play(_ramp(v, V_DOT), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "v", during=air)
        self.play(FadeOut(ring), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "c"))
        c_row = _row(_name(r"c_{\mathrm{Luft}}", "Wärmekapazität der Luft", P_GREEN), x0, y_c, 2.9, P_GREEN,
                     lambda: c.get_value() / 0.5)
        c_read = math_readout(lambda: rf"c_{{\mathrm{{Luft}}}} = {de_num(c.get_value(), 2)}\,\mathrm{{Wh/(m^{{3}}K)}}",
                              np.array([c_row["x_end"] + 0.25, y_c, 0]), size=BODY_FONT_SIZE, color=P_GREEN)
        ring = highlight_param(items, "c", color=P_GREEN)
        _show_row(self, c_row, c_read, run_time=0.5)
        self.play(_ramp(c, C_LUFT), Create(ring), *air(1.2), run_time=1.2)
        hold_for(self, N, "c", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region winter result
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        q_name = _name(r"\dot{Q}_{\mathrm{V,sens}}", "Verlust", SENS)
        q_row = _row(q_name, x0, y_q, 2.9, SENS, lambda: abs(q()) / 1600.0)
        q_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} \approx {de_num(round(q(), -1))}\,\mathrm{{W}}",
                              np.array([q_row["x_end"] + 0.25, y_q, 0]), size=BODY_FONT_SIZE, color=SENS)
        ring = highlight_param(items, "q", color=SENS)
        _show_row(self, q_row, q_read, run_time=0.5)
        self.play(_ramp(kq, 1.0), Create(ring), *air(1.8), run_time=1.8)
        hold_for(self, N, "winter", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region summer
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        chip_s = _season_chip("summer")
        chip_s.move_to(chip_w.get_center() + LEFT * (chip_w.width - chip_s.width) / 2)
        gain = body_text("Gewinn", font_size=LABEL_FONT_SIZE, color=SENS).move_to(q_name[1])
        gain.align_to(q_name[1], LEFT)
        season["now"] = "summer"
        self.play(ReplacementTransform(chip_w, chip_s), Transform(q_name[1], gain), ReplacementTransform(guides, _guides([(house, "summer")])), _ramp(th_i, THETA_I_S),
                  _ramp(th_e, THETA_E_S), *air(3.2), run_time=3.2)
        hold_for(self, N, "summer", during=air)
        #endregion

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
         "At 32 degrees and 50 percent, outdoor air holds 15 grams of water per kilogram, the room at 26 degrees and 50 percent only 10.5 — delta x is 4.5 grams per kilogram.",
         "Bei 32 °C und 50 % enthält die Außenluft 15 g Wasser je kg, die Raumluft bei 26 °C und 50 % nur 10,5 g — Delta-x beträgt 4,5 g/kg."),
        ("formula",
         "The latent ventilation load is density times latent heat r times delta x times volume flow.",
         "Die latente Lüftungslast ist Dichte mal Verdampfungswärme r mal Delta-x mal Volumenstrom."),
        ("water",
         "Per cubic metre that is 1.2 times 4.5, so 5.4 grams of water that must be condensed.",
         "Je Kubikmeter sind das 1,2 mal 4,5, also 5,4 Gramm Wasser, die kondensiert werden müssen."),
        ("r",
         "Changing phase involves energy: r is about 2450 kilojoules per kilogram — condensing releases it, and it must be removed again.",
         "Der Phasenwechsel braucht Energie: r beträgt etwa 2450 Kilojoule je Kilogramm — beim Kondensieren wird sie frei und muss abgeführt werden."),
        ("qv",
         "The air flow of 220 cubic metres per hour is 0.061 cubic metres per second.",
         "Der Luftstrom von 220 Kubikmetern pro Stunde sind 0,061 Kubikmeter pro Sekunde."),
        ("result",
         "Together: about 810 watts of latent load — more than the 450 watts of sensible heat.",
         "Zusammen: rund 810 Watt latente Last — mehr als die 450 Watt sensible Wärme."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Latente Last durch Feuchte", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        x_out, x_in, kw, kr = (ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0))
        kq, kv = ValueTracker(0.0), ValueTracker(0.0)
        x0, y_w, y_r, y_v, y_q = -1.2, 1.68, 0.80, -0.08, -0.93

        #region house and humid air
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
            return RoundedRectangle(corner_radius=0.1, width=0.9, height=col_h, color=P_WHITE, stroke_width=2).move_to(
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

        heads = VGroup(head(cx_o, "Außenluft", rf"{THETA_E_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{r.F.}}"),
                       head(cx_i, "Raumluft", rf"{THETA_I_S:.0f}\,°\mathrm{{C}}\;\cdot\;50\,\%\;\mathrm{{r.F.}}"))
        read_o = math_readout(lambda: rf"x = {de_num(x_out.get_value(), 1)}\,\mathrm{{g/kg}}", np.array([cx_o, y_b - 0.3, 0]),
                              size=BODY_FONT_SIZE, color=LATENT, edge="center")
        read_i = math_readout(lambda: rf"x = {de_num(x_in.get_value(), 1)}\,\mathrm{{g/kg}}", np.array([cx_i, y_b - 0.3, 0]),
                              size=BODY_FONT_SIZE, color=LATENT, edge="center")
        self.play(Create(cols), FadeIn(heads), *air(0.8), run_time=0.8)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.9 + 0.8, during=air)
        #endregion

        #region moisture content outdoors and indoors
        caption = swap_caption(self, caption, subtitle_text(N, "dx"))
        self.add(base_o, extra_o, base_i, read_o, read_i)
        self.play(_ramp(x_out, X_OUT), _ramp(x_in, X_IN), *air(2.4), run_time=2.4)
        y_hi, y_lo = level(X_OUT), level(X_IN)
        guide_o = DashedLine(np.array([cx_o + 0.45, y_hi, 0]), np.array([bx + 0.3, y_hi, 0]), color=P_BLUE, stroke_width=1.6)
        guide_i = DashedLine(np.array([cx_i - 0.45, y_lo, 0]), np.array([bx - 0.3, y_lo, 0]), color=P_BLUE, stroke_width=1.6)
        guide_o_full = DashedLine(np.array([cx_o + 0.45, y_lo, 0]), np.array([bx - 0.3, y_lo, 0]), color=P_BLUE, stroke_width=1.6)
        brace = dim_arrow(np.array([bx, y_lo, 0]), np.array([bx, y_hi, 0]), color=P_BLUE)
        dx_lbl = math_label(rf"\Delta x = {de_num(D_X, 1)}\,\mathrm{{g/kg}}", np.array([bx + 0.3, (y_lo + y_hi) / 2, 0]),
                            size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        self.play(Create(guide_o), Create(guide_o_full), Create(brace), FadeIn(dx_lbl), *air(1.0), run_time=1.0)
        hold_for(self, N, "dx", during=air)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,lat}}", LATENT), (None, "=", P_WHITE),
            ("rho", r"\rho", P_WHITE), (None, r"\cdot", P_WHITE), ("r", "r", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dx", r"\Delta x", P_BLUE), (None, r"\cdot", P_WHITE), ("qv", r"q_{v,\mathrm{R}}", AIR),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])
        rest = VGroup(*[m for m in row if m is not items["dx"]])
        self.play(TransformFromCopy(dx_lbl, items["dx"]), *air(1.0), run_time=1.0)
        self.play(FadeIn(rest), Create(box), *air(0.6), run_time=0.6)
        hold_for(self, N, "formula", during=air)
        #endregion

        #region water per cubic metre
        caption = swap_caption(self, caption, subtitle_text(N, "water"))
        w_name = _name(r"\rho \cdot \Delta x", "Wasser je m³", P_BLUE)
        w_row = _row(w_name, x0, y_w, 3.3, P_BLUE, lambda: kw.get_value() * WATER_PER_M3 / 6.0)
        w_read = math_readout(lambda: rf"\rho \cdot \Delta x = {de_num(WATER_PER_M3 * kw.get_value(), 1)}\,\mathrm{{g/m^{{3}}}}",
                              np.array([w_row["x_end"] + 0.25, y_w, 0]), size=BODY_FONT_SIZE, color=P_BLUE)
        _hide(self, cols, heads, base_o, extra_o, base_i, read_o, read_i, guide_o, guide_i, guide_o_full, brace, dx_lbl)
        ring = highlight_param(items, "dx", color=P_BLUE)
        _show_row(self, w_row, w_read, run_time=0.5)
        self.play(_ramp(kw, 1.0), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "water", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region latent heat of vaporization
        caption = swap_caption(self, caption, subtitle_text(N, "r"))
        r_row = _row(_name("r", "Verdampfungswärme", P_ORANGE), x0, y_r, 3.3, P_ORANGE, lambda: kr.get_value() * R_LAT / 3000.0)
        r_read = math_readout(lambda: rf"r = {de_num(R_LAT * kr.get_value())}\,\mathrm{{kJ/kg}}",
                              np.array([r_row["x_end"] + 0.25, y_r, 0]), size=BODY_FONT_SIZE, color=P_ORANGE)
        ring = highlight_param(items, "r", color=P_ORANGE)
        _show_row(self, r_row, r_read, run_time=0.5)
        self.play(_ramp(kr, 1.0), Create(ring), *air(1.4), run_time=1.4)
        hold_for(self, N, "r", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region volume flow
        caption = swap_caption(self, caption, subtitle_text(N, "qv"))
        v_row = _row(_name(r"q_{v,\mathrm{R}} = \dot{V}/3600", "Volumenstrom", AIR), x0, y_v, 3.3, AIR, lambda: kv.get_value() * Q_V_R / 0.1)
        v_read = math_readout(lambda: rf"q_{{v,\mathrm{{R}}}} = {de_num(Q_V_R * kv.get_value(), 3)}\,\mathrm{{m^{{3}}/s}}",
                              np.array([v_row["x_end"] + 0.25, y_v, 0]), size=BODY_FONT_SIZE, color=AIR)
        ring = highlight_param(items, "qv", color=AIR)
        _show_row(self, v_row, v_read, run_time=0.5)
        self.play(_ramp(kv, 1.0), Create(ring), *air(1.2), run_time=1.2)
        hold_for(self, N, "qv", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region result
        caption = swap_caption(self, caption, subtitle_text(N, "result"))
        q_row = _row(_name(r"\dot{Q}_{\mathrm{V,lat}}", "Latente Last", LATENT), x0, y_q, 3.3, LATENT,
                     lambda: kq.get_value() * Q_LAT / 1000.0)
        q_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} \approx {de_num(round(Q_LAT * kq.get_value(), -1))}\,\mathrm{{W}}",
                              np.array([q_row["x_end"] + 0.25, y_q, 0]), size=BODY_FONT_SIZE, color=LATENT)
        tick_x = x0 + 0.04 + (3.3 - 0.08) * W_SUMMER / 1000.0
        tick = Line(np.array([tick_x, y_q - 0.2, 0]), np.array([tick_x, y_q + 0.2, 0]), color=SENS, stroke_width=3)
        tick_lbl = math_label(rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {de_num(W_SUMMER)}\,\mathrm{{W}}", np.array([tick_x, y_q - 0.34, 0]),
                              size=LABEL_FONT_SIZE - 2, color=SENS)
        ring = highlight_param(items, "q", color=LATENT)
        _show_row(self, q_row, q_read, run_time=0.5)
        self.play(_ramp(kq, 1.0), Create(ring), *air(1.8), run_time=1.8)
        self.play(Create(tick), FadeIn(tick_lbl), *air(0.6), run_time=0.6)
        hold_for(self, N, "result", during=air)
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

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
         "In winter 20 kelvin drive 1500 watts of sensible heat out. Latent heat is not part of the heating load.",
         "Im Winter treiben 20 Kelvin 1500 Watt sensible Wärme hinaus. Die latente Wärme gehört nicht zur Heizlast."),
        ("sens",
         "In summer only 6 kelvin drive 450 watts of sensible heat in.",
         "Im Sommer treiben nur 6 Kelvin 450 Watt sensible Wärme herein."),
        ("lat",
         "On top comes the latent load of 810 watts from the moisture — together 1260 watts.",
         "Dazu kommt die latente Last von 810 Watt aus der Feuchte — zusammen 1260 Watt."),
        ("use",
         "Heating takes the winter value as its ventilation loss, Cooling takes both summer shares as its ventilation load.",
         "Die Heizung übernimmt den Winterwert als Lüftungsverlust, die Kühlung beide Sommeranteile als Lüftungslast."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Dieselbe Formel, zwei Jahreszeiten", title)
        din = _din_ref("DIN EN 12831-1 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        unit, y_base, bar_w = 1.9 / 1500.0, -0.95, 1.0

        #region two blocks
        house_w = _house(np.array([-4.9, -0.17, 0.0]), 0.6, 0.95)
        house_s = _house(np.array([2.3, -0.17, 0.0]), 0.6, 0.95)
        chip_w, chip_s = _season_chip("winter"), _season_chip("summer")
        chip_w.move_to(np.array([-4.5, 2.35, 0.0]))
        chip_s.move_to(np.array([3.3, 2.35, 0.0]))
        row, box, items = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V}}", P_WHITE), (None, "=", P_WHITE),
            ("s", r"\dot{Q}_{\mathrm{V,sens}}", SENS), (None, "+", P_WHITE), ("l", r"\dot{Q}_{\mathrm{V,lat}}", LATENT),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])
        self.play(Create(house_w["group"]), Create(house_s["group"]), FadeIn(chip_w), FadeIn(chip_s), run_time=1.4)
        self.play(*_open_both(house_w), *_open_both(house_s), Create(_guides([(house_w, "winter"), (house_s, "summer")])),
                  run_time=0.8)

        def air(rt):
            return _air([(house_w, "winter"), (house_s, "summer")], rt)

        self.play(FadeIn(row), Create(box), *air(0.8), run_time=0.8)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.8 + 0.8, during=air)
        #endregion

        def bar(x, segments):
            return stacked_bar(segments, base=np.array([x, y_base, 0.0]), width=bar_w, unit=unit)

        def side(bars, k, x, src, driver, color):
            mid = bars["anchors"][k]
            label = VGroup(math_label(src, size=LABEL_FONT_SIZE, color=color),
                           math_label(driver, size=LABEL_FONT_SIZE, color=P_YELLOW if "theta" in driver else P_BLUE)).arrange(
                DOWN, buff=0.1, aligned_edge=LEFT)
            label.move_to(np.array([x + label.width / 2, mid[1], 0.0]))
            return label

        #region winter bar
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        bars_w = bar(-2.9, [("sens", W_WINTER, SENS)])
        lab_w = side(bars_w, 0, -2.25, rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {de_num(W_WINTER)}\,\mathrm{{W}}",
                     rf"\Delta\theta = {de_num(THETA_I_W - THETA_E_W)}\,\mathrm{{K}}", SENS)
        self.play(GrowFromEdge(bars_w["bars"][0], DOWN), *air(1.6), run_time=1.6)
        note_w = VGroup(body_text("latent: nicht Teil", font_size=LABEL_FONT_SIZE - 2, color=LATENT),
                        body_text("der Heizlast", font_size=LABEL_FONT_SIZE - 2, color=LATENT)).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        note_w.set_opacity(0.75).move_to(np.array([-2.25 + note_w.width / 2, -0.62, 0.0]))
        self.play(FadeIn(lab_w), FadeIn(note_w), *air(0.5), run_time=0.5)
        hold_for(self, N, "winter", during=air)
        #endregion

        #region summer bar
        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        bars_s = bar(4.2, [("sens", W_SUMMER, SENS), ("lat", W_LATENT, LATENT)])
        lab_s = side(bars_s, 0, 4.85, rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} = {de_num(W_SUMMER)}\,\mathrm{{W}}",
                     rf"\Delta\theta = {de_num(THETA_E_S - THETA_I_S)}\,\mathrm{{K}}\;\text{{(Gewinn)}}", SENS)
        self.play(GrowFromEdge(bars_s["bars"][0], DOWN), *air(1.2), run_time=1.2)
        self.play(FadeIn(lab_s), *air(0.5), run_time=0.5)
        hold_for(self, N, "sens", during=air)

        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        lab_l = side(bars_s, 1, 4.85, rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} = {de_num(W_LATENT)}\,\mathrm{{W}}",
                     rf"\Delta x = {de_num(D_X, 1)}\,\mathrm{{g/kg}}", LATENT)
        self.play(GrowFromEdge(bars_s["bars"][1], DOWN), *air(1.4), run_time=1.4)
        self.play(FadeIn(lab_l), *air(0.5), run_time=0.5)
        hold_for(self, N, "lat", during=air)
        #endregion

        #region hand-over
        caption = swap_caption(self, caption, subtitle_text(N, "use"))
        to_h = _handover_chip("→ Heizung:", r"\Phi_{\mathrm{V}}", HEIZUNG)
        to_k = _handover_chip("→ Kühlung:", r"\dot{Q}_{\mathrm{L}}", KUEHLUNG)
        to_h.move_to(np.array([-4.5, 1.72, 0.0]))
        to_k.move_to(np.array([3.3, 1.72, 0.0]))
        self.play(FadeIn(to_h, shift=DOWN * 0.15), *air(0.8), run_time=0.8)
        self.play(FadeIn(to_k, shift=DOWN * 0.15), *air(0.8), run_time=0.8)
        hold_for(self, N, "use", during=air)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
