

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
import sys as _sys


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
window_card = _manim_module.window_card
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
SAFE_TOP = _manim_module.SAFE_TOP
SAFE_BOTTOM_FORMULA = _manim_module.SAFE_BOTTOM_FORMULA
CONTENT_TOP = _manim_module.CONTENT_TOP
note_line = _manim_module.note_line
highlight_param = _manim_module.highlight_param
dim_arrow = _manim_module.dim_arrow
chip = _manim_module.chip
meter = _manim_module.meter
bind_meter = _manim_module.bind_meter
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
ripples = _manim_module.ripples
house_section = _manim_module.house_section
person_glyph = _manim_module.person_glyph
radiator = _manim_module.radiator
sun_glyph = _manim_module.sun_glyph
math_label = _manim_module.math_label
math_row = _manim_module.math_row
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
de_num = _manim_module.de_num
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing

set_vo_language("de")
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Primärenergie & das GEG"

NUTZ_BLUE = semio_color("secondary")
END_RED = semio_color("danger")
PRIM_VIOLET = semio_color("info")
AMBIENT_GREEN = semio_color("success")
GAS_AMBER = semio_color("tertiary")
WOOD_BROWN = semio_color("info")

# The schematic worked example carried over from Chapter 4. The plant factors
# mirror ``4_system_losses/scene_4.py`` (E_UEBERGABE · E_VERTEILUNG ·
# E_SPEICHERUNG · E_ERZEUGUNG_*) so final energy here is the value Chapter 4
# puts on screen, not a separately typed number.
Q_NUTZ = 100
E_P_GAS = 1.04 * 1.08 * 1.06 * 1.18   # ≈ 1,40
E_P_WP = 1.04 * 1.08 * 1.06 * 0.26    # ≈ 0,31
Q_END_GAS = round(Q_NUTZ * E_P_GAS)   # 140 kWh/(m²·a)
Q_END_WP = round(Q_NUTZ * E_P_WP)     #  31 kWh/(m²·a)
F_P_GAS, F_P_STROM = 1.1, 1.8          # GEG Anlage 4, nicht erneuerbarer Anteil
F_P_HOLZ, F_P_UMWELT = 0.2, 0.0
# From the *displayed* final energy, so the multiplication on screen checks out
# by hand; half-up because round() is banker's rounding.
Q_PRIM_GAS = int(Q_END_GAS * F_P_GAS + 0.5)    # 154 kWh/(m²·a)
Q_PRIM_WP = int(Q_END_WP * F_P_STROM + 0.5)    #  56 kWh/(m²·a)
Q_LOSS_GAS = Q_END_GAS - Q_NUTZ                #  40 kWh/(m²·a) Anlagenverluste
Q_AMBIENT_WP = Q_NUTZ - Q_END_WP               #  69 kWh/(m²·a) Umweltwärme
Q_CHAIN_GAS = Q_PRIM_GAS - Q_END_GAS           #  14 kWh/(m²·a) Vorkette
Q_CHAIN_WP = Q_PRIM_WP - Q_END_WP              #  25 kWh/(m²·a) Vorkette
# Reference-building value is building-specific; this one is a schematic
# example. The 55 % share is the GEG § 15 requirement for new residential builds.
Q_REF, REF_SHARE = 120, 0.55
Q_LIMIT = int(Q_REF * REF_SHARE + 0.5)          #  66 kWh/(m²·a)
EE_SHARE = 0.65

UNIT = r"\mathrm{kWh/(m^{2}\,a)}"


def _eq(product: float) -> str:
    """🟰 '=' only when the shown product is exact — 140 · 1,1 = 154 but 31 · 1,8 ≈ 56."""
    return "=" if abs(product - round(product)) < 1e-9 else r"\approx"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

#region Buildings and people
def _stand(floor_y, room_h, x, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the inner room height, feet on the floor line at ``x``."""
    height = 0.7 * room_h
    return person_glyph(np.array([x, floor_y + height / 2, 0.0]), color=color, scale=height / 0.66)


def _house(center_x, floor_y, s):
    """🏠 Physical Fundamentals house section standing on ``floor_y``."""
    return house_section(np.array([center_x, floor_y + 1.2 * s, 0.0]), scale=s)


def _furnish(hs, s, person_dx, rad_dx, *, person=True):
    """🛋️ Ground-floor occupant and radiator inside a ``house_section`` — returns both and their warm spot."""
    floor_y, cx = float(hs["bottom_left"][1]), float(hs["center"][0])
    heater = radiator(np.array([cx + rad_dx, floor_y, 0.0]))
    heater.set(height=0.26 * hs["w_height"] / 2)
    heater.move_to(np.array([cx + rad_dx, floor_y + heater.height / 2 + 0.02, 0.0]))
    figure = _stand(floor_y, 1.2 * s, cx + person_dx) if person else VGroup()
    return {"person": figure, "radiator": heater, "warm": heater.get_top() + UP * 0.03,
            "group": VGroup(figure, heater)}


def _envelope(hs, s, off=0.09):
    """🧥 Thick thermal-envelope outline hugging the house section — the building shell."""
    bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
    return VMobject(color=P_PRIMARY, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
        bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * s + off) + UP * 0.02, peak + UP * off,
        tr + RIGHT * (0.26 * s + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
    ])


def _ground(x0, x1, y):
    """🟫 Ground line with a few hatches."""
    line = Line(np.array([x0, y, 0.0]), np.array([x1, y, 0.0]), color=P_PRIMARY, stroke_width=2.2)
    hatches = VGroup(*[
        Line(p, p + (DOWN * 0.09 + LEFT * 0.06), color=P_PRIMARY, stroke_width=1.0)
        for p in [line.point_from_proportion(u) for u in np.linspace(0.02, 0.98, int((x1 - x0) / 0.33))]
    ])
    return VGroup(line, hatches)


def _ring(x0, x1, y0, y1, color, *, dashed=True):
    """⭕ One balance boundary — the dashed motif Chapter 1 drew around the house."""
    box = RoundedRectangle(corner_radius=0, width=x1 - x0, height=y1 - y0, color=color, stroke_width=2.6)
    box.move_to(np.array([(x0 + x1) / 2, (y0 + y1) / 2, 0.0]))
    return DashedVMobject(box, num_dashes=60, dashed_ratio=0.55) if dashed else box
#endregion


#region Plants and sources
def _pt(point):
    """📍 Point as a 3-vector."""
    return np.array([point[0], point[1], 0.0])


def _outline(*mobs, color=P_WHITE, width=1.8):
    """✏️ Shared line style of the plant glyphs."""
    for mob in mobs:
        mob.set_stroke(color=color, width=width)
    return VGroup(*mobs)


def _derrick(base, h=0.85):
    """🛢️ Extraction rig — lattice tower on the ground at ``base``."""
    b = _pt(base)
    apex = b + UP * h
    legs = [Line(b + LEFT * 0.3, apex), Line(b + RIGHT * 0.3, apex)]
    levels = [(0.0, 0.3), (0.3, 0.6), (0.6, 0.85)]
    zig = [Line(b + LEFT * 0.3 * (1 - lo) + UP * h * lo, b + RIGHT * 0.3 * (1 - hi) + UP * h * hi) for lo, hi in levels]
    rungs = [Line(b + LEFT * 0.3 * (1 - f) + UP * h * f, b + RIGHT * 0.3 * (1 - f) + UP * h * f) for f in (0.3, 0.6)]
    return _outline(*legs, *zig, *rungs, width=1.6)


def _power_plant(base, w=0.9, h=0.5):
    """🏭 Filled industrial elevation with sawtooth halls and tapered stacks."""
    profile = _manim_module.object_silhouette("factory", color=P_WHITE, width=w)
    profile.stretch_to_fit_height(h * 1.55)
    return VGroup(profile.move_to(_pt(base) + UP * profile.height / 2))


def _pump(center, r=0.17):
    """🌀 Compressor or fan — ring with three blades."""
    c = _pt(center)
    ring = Circle(radius=r).move_to(c)
    blades = VGroup(*[Line(c, c + RIGHT * r * 0.8).rotate(a, about_point=c) for a in (0.0, TAU / 3, 2 * TAU / 3)])
    _outline(ring, *blades, width=1.8)
    return VGroup(ring, blades)


def _flame(center, h=0.5, color=P_ORANGE):
    """🔥 An irregular ink tongue with a tapered split tip."""
    profile = _manim_module.object_silhouette("flame", color=color)
    profile.scale(h / profile.height)
    return profile.move_to(_pt(center))


def _tree(base, h=0.5):
    """🌳 Architectural tree entourage with a tapering trunk and natural branch forks."""
    profile = _manim_module.object_silhouette("tree", color=WOOD_BROWN)
    profile.scale(h / profile.height)
    return VGroup(profile.move_to(_pt(base) + UP * h / 2))


def _boiler(center):
    """🔥 Gas boiler — casing with its flame."""
    c = _pt(center)
    casing = RoundedRectangle(width=0.5, height=0.62, corner_radius=0.08).move_to(c)
    _outline(casing)
    return VGroup(casing, _flame(c + DOWN * 0.02, 0.36))


def _heat_pump(center):
    """♻️ Air-source heat pump — casing with a fan the outdoor air runs through."""
    c = _pt(center)
    casing = Rectangle(width=0.8, height=0.56).move_to(c)
    _outline(casing)
    fan = _pump(c + LEFT * 0.12, 0.2)
    louvres = VGroup(*[Line(c + RIGHT * 0.22 + UP * dy + LEFT * 0.08, c + RIGHT * 0.22 + UP * dy + RIGHT * 0.08)
                       for dy in (-0.14, 0.0, 0.14)])
    _outline(louvres, width=1.3)
    return {"group": VGroup(casing, fan, louvres), "fan": fan}


def _meter_glyph(center, r=0.17):
    """⏱️ Energy meter — dial with a needle, the point where final energy is counted."""
    c = _pt(center)
    dial = Circle(radius=r, color=P_WHITE, stroke_width=2.0, fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(c)
    needle = Line(c, c + (RIGHT * 0.8 + UP * 0.6) * r * 0.85, color=P_YELLOW, stroke_width=2.0)
    return VGroup(dial, needle)
#endregion


#region Flows and readouts
def _flow(path, color, rt, *, v=1.5, per_unit=1.5, color_end=None, radius=0.055):
    """💨 Particle stream along ``path`` whose pass count is a whole number, so consecutive plays loop without a jump."""
    length = max(0.4, path.get_arc_length())
    return flow_animation(
        [([path], color, color_end)], waves=max(2, round(length * per_unit)), radius=radius,
        cycles=max(1, round(v * rt / length)), streak=False,
    )


def _heat(spots, rt, r_max=0.45, color=P_ORANGE):
    """🌡️ Heat ripples that keep radiating for ``rt`` seconds."""
    return [ripples(spots, r_max=r_max, cycles=max(1.0, rt / 1.4), color=color)]


def _supply_flow(y, x0, x1, x_loss, kept, lost, counted, color_counted, color_other, rt, *, v=1.5, radius=0.055):
    """⛓️ One carrier's supply chain — ``kept`` particles reach the meter, ``lost`` ones dissipate at ``x_loss``.

    The upstream density is ``(kept + lost) / kept`` times the delivered density — exactly the factor ``f_p``
    for non-renewable carriers; ``counted`` of the ``kept`` particles are the non-renewable share.
    """
    total = kept + lost
    cycles = max(1, round(v * rt / (x1 - x0)))
    dots, meta, seen = VGroup(), [], 0
    for i in range(total):
        is_lost = (i + 1) * lost // total > i * lost // total
        if is_lost:
            colour = PRIM_VIOLET
        else:
            colour = color_counted if (seen + 1) * counted // max(1, kept) > seen * counted // max(1, kept) else color_other
            seen += 1
        dot = Dot(radius=radius, color=colour, stroke_width=0).set_fill(colour, opacity=0.0)
        dots.add(dot)
        meta.append((i, is_lost))

    def update(group, alpha):
        for dot, (i, is_lost) in zip(group, meta):
            t = (alpha * cycles + i / total) % 1.0
            x = x0 + (x1 - x0) * t
            opacity = min(1.0, t / 0.06)
            lift = 0.0
            if is_lost:
                spread = max(0.0, (x - x_loss) / 0.5)
                opacity *= max(0.0, 1.0 - spread)
                lift = spread * 0.3
            else:
                opacity *= min(1.0, (1.0 - t) / 0.06)
            dot.move_to(np.array([x, y + lift, 0.0]))
            dot.set_fill(opacity=0.92 * opacity)

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


def _pipes(*paths):
    """🚰 Pipe bands under the particle streams."""
    return VGroup(*[p.copy().set_stroke(P_WHITE, width=6, opacity=0.2).set_fill(opacity=0) for p in paths])


def _count(tracker, at, color, *, digits=0, size=None, edge="center", prefix=""):
    """🔢 Live number that counts up from its tracker."""
    return math_readout(lambda: prefix + de_num(tracker.get_value(), digits), at, size=size or LABEL_FONT_SIZE,
                        color=color, edge=edge)


def _freeze(scene, readout, src, at, color, *, size=None, edge="center"):
    """🧊 Replace a live readout by the same label typeset once, ready to be morphed."""
    static = math_label(src, np.array(at, dtype=float), size=size or LABEL_FONT_SIZE, color=color, edge=edge)
    scene.remove(readout)
    scene.add(static)
    return static
#endregion


#region Bars
def _bar_rects(value, segments, x, base_y, *, width=0.62, unit=0.01, opacity=0.8):
    """📊 Stacked rectangles for ``value`` — segments fill bottom to top as the value grows."""
    remain, cursor, group = max(0.0, value), 0.0, VGroup()
    for amount, colour in segments:
        take = min(amount, remain)
        if take <= 1e-6:
            break
        h = take * unit
        group.add(Rectangle(width=width, height=h, color=colour, fill_color=colour, fill_opacity=opacity,
                            stroke_width=1.2).move_to(np.array([x, base_y + cursor + h / 2, 0.0])))
        cursor += h
        remain -= take
    return group


def _bar_mob(tracker, segments, x, base_y, *, width=0.62, unit=0.01):
    """📈 Stacked bar whose height follows ``tracker`` every frame."""
    return always_redraw(lambda: _bar_rects(tracker.get_value(), segments, x, base_y, width=width, unit=unit))


def _ghost(x, base_y, value, *, width=0.62, unit=0.01, color=P_WHITE):
    """👻 Dashed outline of a bar height — what the other plant would have reached."""
    box = Rectangle(width=width, height=value * unit, color=color, stroke_width=1.8).move_to(
        np.array([x, base_y + value * unit / 2, 0.0]))
    return DashedVMobject(box, num_dashes=28, dashed_ratio=0.5).set_stroke(opacity=0.7)


def _card(caption, value, color, *, width=None):
    """🪧 Fuse the named quantity and its typeset value into the shared window card."""
    heading = body_text(caption, font_size=LABEL_FONT_SIZE, color=P_WHITE)
    return window_card(heading, value, color=color, width=width)
#endregion


#region Pacing
def _say(scene, narration, key, caption, *beats, during=None, first=False):
    """🎞️ One clause, one idea — swap its subtitle in, play each ``(run_time, anims)`` reveal in turn, then hold the rest of the clause.

    ``anims`` may be a callable, built only when its reveal starts (for rings around parts that move in first).

    ``first`` marks the beat's opening clause, whose caption is already on screen.
    """
    if not first:
        caption = swap_caption(scene, caption, subtitle_text(narration, key))
    spent = 0.3 if first else 0.0
    for run_time, anims in beats:
        scene.play(*(anims() if callable(anims) else anims), *(during(run_time) if during else []),
                   run_time=run_time)
        spent += run_time
    hold_for(scene, narration, key, used=spent, during=during)
    return caption
#endregion

#endregion


#region Beat 1 – Drei Bilanzgrenzen

class Beat1_DreiBilanzgrenzen(Scene):
    NARRATION = [
        ("intro",
         "Everything in this series has depended on one decision: where we draw the balance boundary.",
         "Alles in dieser Serie hing an einer Entscheidung: wo wir die Bilanzgrenze ziehen."),
        ("raum",
         "Draw it around the room and you count useful energy.",
         "Zieht man sie um den Raum, zählt man Nutzenergie."),
        ("heizwaerme",
         "That is the heat the balance said the room needs.",
         "Das ist die Wärme, die die Bilanz für den Raum verlangt."),
        ("gebaeude",
         "Draw it around the building at the meter and you count final energy.",
         "Zieht man sie am Zähler um das Gebäude, zählt man Endenergie."),
        ("verluste",
         "That includes the plant losses: the waste heat of the boiler counts as well.",
         "Dazu zählen die Anlagenverluste: Auch die Abwärme des Kessels wird mitgezählt."),
        ("quelle",
         "Draw it all the way back to the source and you count primary energy.",
         "Zieht man sie bis zur Quelle, zählt man Primärenergie."),
        ("gewinnung",
         "First comes extraction: the fuel has to be taken out of the ground.",
         "Zuerst die Gewinnung: Der Brennstoff muss aus dem Boden geholt werden."),
        ("umwandlung",
         "Then conversion: a plant turns it into the carrier that is delivered.",
         "Dann die Umwandlung: Eine Anlage macht daraus den gelieferten Energieträger."),
        ("transport",
         "Last comes transport, all the way to the meter.",
         "Zuletzt der Transport — bis zum Zähler."),
        ("geg",
         "That is why the GEG caps primary energy demand.",
         "Deshalb begrenzt das GEG den Primärenergiebedarf."),
        ("vergleich",
         "Because it puts different energy carriers on one common scale.",
         "Denn er bringt verschiedene Energieträger auf einen gemeinsamen Maßstab."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Drei Bilanzgrenzen")
        din = _din_ref("DIN V 18599-1:2018-09 · GEG § 20")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        revision_note = note_line(
            "Seit 2025 als DIN/TS 18599-1:2025-10 fortgeschrieben — GEG § 20 nutzt weiterhin 2018",
            color=P_PRIMARY,
        )
        self.play(FadeIn(caption), FadeIn(revision_note), run_time=0.3)

        G, S, HX = -0.95, 0.75, 0.45
        hs = _house(HX, G, S)
        inside = _furnish(hs, S, -0.65, 0.45)
        peak = float(hs["roof_peak"][1])
        wall_r = HX + 1.8 * S
        annex_x = wall_r + 0.5
        y_pipe, y_ground = G + 0.2, G - 0.22
        meter_x = -1.5

        ground = _ground(-6.8, 3.7, G)
        annex = _outline(
            Line(np.array([wall_r, G + 0.95, 0.0]), np.array([wall_r + 1.05, G + 0.95, 0.0])),
            Line(np.array([wall_r + 1.05, G + 0.95, 0.0]), np.array([wall_r + 1.05, G, 0.0])),
        )
        boiler = _boiler(np.array([annex_x, G + 0.4, 0.0]))
        meter_glyph = _meter_glyph(np.array([meter_x, y_ground, 0.0]))
        derrick = _derrick(np.array([-5.9, G]))
        plant = _power_plant(np.array([-4.3, G]), w=0.95, h=0.5)
        pump = _pump(np.array([-2.75, y_pipe]))
        names = [
            body_text(text, font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET).move_to(np.array([x, G + 1.1, 0.0]))
            for text, x in (("Gewinnung", -5.9), ("Umwandlung", -4.3), ("Transport", -2.75))
        ]
        loss_name = body_text("Anlagenverluste", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([4.95, G + 0.45, 0.0]))
        loss_arrow = Arrow(np.array([4.0, G + 0.45, 0.0]), np.array([annex_x + 0.4, G + 0.42, 0.0]), buff=0,
                           color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.25)

        path_up = smooth_path([(-5.9, y_pipe, 0.0), (-4.3, y_pipe, 0.0), (-2.75, y_pipe, 0.0), (-2.3, y_pipe, 0.0),
                               (-1.95, y_ground, 0.0), (meter_x, y_ground, 0.0)])
        path_end = smooth_path([(meter_x, y_ground, 0.0), (HX + 0.4, y_ground, 0.0), (annex_x, y_ground, 0.0),
                                (annex_x, G + 0.1, 0.0)])
        path_use = smooth_path([(annex_x + 0.3, G + 0.18, 0.0), (wall_r, G + 0.18, 0.0),
                                (inside["radiator"].get_right()[0] + 0.05, G + 0.18, 0.0)])
        pipe_up, pipe_end, pipe_use = (_pipes(p) for p in (path_up, path_end, path_use))

        ring_room = _ring(HX - 1.8 * S + 0.12, wall_r - 0.12, G + 0.05, G + 2.4 * S - 0.06, NUTZ_BLUE)
        ring_house = _ring(meter_x - 0.15, annex_x + 0.95, G - 0.36, peak + 0.12, END_RED)
        ring_source = _ring(-6.85, 3.8, G - 0.52, peak + 0.25, PRIM_VIOLET)

        def stage_card(caption_text, name, symbol, color, x, y):
            name_mob = body_text(name, font_size=LABEL_FONT_SIZE + 1, color=color)
            sym = math_label(symbol, size=BODY_FONT_SIZE, color=color)
            value = VGroup(name_mob, sym).arrange(RIGHT, buff=0.16)
            card = _card(caption_text, value, color, width=2.75)
            card.move_to(np.array([x, y, 0.0]))
            return {"card": card, "sym": sym}

        specs = (("Raum", "Nutzenergie", r"Q_{h}", NUTZ_BLUE), ("Zähler", "Endenergie", r"Q_{E}", END_RED),
                 ("Quelle", "Primärenergie", r"Q_{P}", PRIM_VIOLET))
        cards = [stage_card(*spec, 5.45, 0.4) for spec in specs]

        room_heat = lambda rt: _heat([inside["warm"]], rt, 0.4)
        active = [room_heat]

        def during(rt):
            anims = []
            for make in active:
                anims.extend(make(rt))
            return anims

        def say(key, *beats, first=False):
            nonlocal caption
            caption = _say(self, self.NARRATION, key, caption, *beats, during=during, first=first)

        say("intro",
            (1.6, [FadeOut(revision_note), Create(hs["group"]), Create(ground)]),
            (1.2, [FadeIn(inside["group"])]), first=True)

        say("raum", (1.4, [Create(ring_room), FadeIn(cards[0]["card"], shift=LEFT * 0.2)]))

        active.append(lambda rt: [_flow(path_use, NUTZ_BLUE, rt, v=0.9, per_unit=3.0)])
        say("heizwaerme", (1.2, [FadeIn(pipe_use)]))

        active[:] = [room_heat]
        active.append(lambda rt: [_flow(path_end, END_RED, rt, v=1.4, per_unit=1.5)])
        say("gebaeude",
            (1.0, [FadeOut(cards[0]["card"]), FadeOut(pipe_use), ring_room.animate.set_stroke(opacity=0.3)]),
            (1.4, [Create(ring_house), FadeIn(cards[1]["card"], shift=LEFT * 0.2), FadeIn(meter_glyph),
                   Create(annex), FadeIn(boiler), FadeIn(pipe_end)]))

        active.append(lambda rt: _heat([np.array([annex_x, G + 0.75, 0.0])], rt, 0.35))
        say("verluste", (1.2, [FadeIn(loss_name), GrowArrow(loss_arrow)]))

        active[:] = [room_heat]
        say("quelle",
            (1.0, [FadeOut(cards[1]["card"]), FadeOut(pipe_end), FadeOut(annex), FadeOut(boiler),
                   FadeOut(loss_name), FadeOut(loss_arrow), FadeOut(ring_room), FadeOut(ring_house)]),
            (1.4, [Create(ring_source), FadeIn(cards[2]["card"], shift=LEFT * 0.2)]))

        say("gewinnung", (1.2, [FadeIn(derrick), FadeIn(names[0])]))

        active.append(lambda rt: _heat([np.array([-4.3, G + 0.62, 0.0])], rt, 0.4))
        say("umwandlung", (1.2, [FadeIn(plant), FadeIn(names[1])]))

        active.append(lambda rt: [_flow(path_up, PRIM_VIOLET, rt, v=1.5, per_unit=1.8)])
        say("transport", (1.2, [FadeIn(pump), FadeIn(names[2]), FadeIn(pipe_up)]))

        active.clear()
        row_cards = [stage_card(*spec, x, 0.7) for spec, x in zip(specs, (-4.0, 0.0, 4.0))]
        hops = [
            Arrow(np.array([x0, 0.7, 0.0]), np.array([x1, 0.7, 0.0]), buff=0, color=P_WHITE, stroke_width=3,
                  max_tip_length_to_length_ratio=0.3)
            for x0, x1 in ((-2.55, -1.45), (1.45, 2.55))
        ]
        hop_names = VGroup(
            body_text("Anlage", font_size=LABEL_FONT_SIZE - 2, color=P_PRIMARY).move_to(np.array([-2.0, 1.05, 0.0])),
            math_label(r"f_{p}", np.array([2.0, 1.1, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW),
        )
        row, box, items = math_panel([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p}", P_YELLOW), ("cap", r"\;\text{← vom GEG begrenzt}", P_PRIMARY),
        ])
        rings = {}
        scenery = (hs["group"], ground, inside["group"], meter_glyph, derrick, plant, pump, pipe_up, ring_source,
                   cards[2]["card"], *names)
        say("geg",
            (1.0, [FadeOut(m) for m in scenery]),
            (1.6, [FadeIn(rc["card"]) for rc in row_cards] + [GrowArrow(h) for h in hops] + [FadeIn(hop_names)]),
            (1.4, [Create(box), FadeIn(items["eq"]), FadeIn(items["dot"]), FadeIn(items["cap"]),
                   TransformFromCopy(row_cards[2]["sym"], items["qp"]),
                   TransformFromCopy(row_cards[1]["sym"], items["qe"]),
                   FadeIn(items["fp"], shift=UP * 0.2)]),
            (0.8, lambda: [Create(rings.setdefault("p", highlight_param(items, "qp", color=P_YELLOW)))]))

        carriers = [chip(text, colour, font_size=BODY_FONT_SIZE) for text, colour in (
            ("Erdgas", GAS_AMBER), ("Strom", P_YELLOW), ("Holz", WOOD_BROWN), ("Umweltwärme", AMBIENT_GREEN))]
        VGroup(*carriers).arrange(RIGHT, buff=0.6).move_to(np.array([0.0, 0.7, 0.0]))
        say("vergleich",
            (1.0, [FadeOut(rc["card"]) for rc in row_cards] + [FadeOut(h) for h in hops] + [FadeOut(hop_names),
                                                                                         FadeOut(rings["p"])]),
            (2.4, [LaggedStart(*[FadeIn(c, shift=UP * 0.15) for c in carriers], lag_ratio=0.45)]),
            (1.0, [Indicate(items["fp"], color=P_YELLOW, scale_factor=1.15)]))

        self.play(FadeOut(caption), FadeOut(box), FadeOut(row), *[FadeOut(c) for c in carriers], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Der Primärenergiefaktor

class Beat2_Primaerenergiefaktor(Scene):
    NARRATION = [
        ("intro",
         "The step from final to primary energy is one multiplication.",
         "Der Schritt von End- zu Primärenergie ist eine Multiplikation."),
        ("zahl",
         "The multiplier is one number for each energy carrier.",
         "Der Faktor ist eine Zahl je Energieträger."),
        ("bezug",
         "It says how much primary energy one kilowatt-hour of final energy needs.",
         "Er sagt, wie viel Primärenergie eine Kilowattstunde Endenergie braucht."),
        ("strom",
         "Grid electricity carries a high factor among household carriers.",
         "Netzstrom trägt unter den Haushalts-Energieträgern einen hohen Faktor."),
        ("strom_mix",
         "Its generation mix still has a large fossil share.",
         "Sein Erzeugungsmix hat noch einen großen fossilen Anteil."),
        ("strom_geg",
         "The value is fixed in the GEG.",
         "Der Wert ist im GEG festgelegt."),
        ("gas",
         "Gas and oil sit just above one.",
         "Gas und Öl liegen knapp über eins."),
        ("gas_kette",
         "The fuel itself is fossil, plus a small share for the supply chain.",
         "Der Brennstoff selbst ist fossil, dazu kommt ein kleiner Anteil für die Vorkette."),
        ("holz",
         "Wood is counted at a fifth.",
         "Holz wird mit einem Fünftel angesetzt."),
        ("holz_rest",
         "The remaining four fifths are renewable and do not count.",
         "Die übrigen vier Fünftel sind erneuerbar und zählen nicht."),
        ("umwelt",
         "Ambient heat from air or ground counts as zero.",
         "Umweltwärme aus Luft oder Erdreich zählt als null."),
        ("erneuerbar",
         "These are the non-renewable shares.",
         "Das sind die nicht erneuerbaren Anteile."),
        ("rechenwert",
         "They are fixed calculation values, not measurements of any individual delivery.",
         "Es sind festgelegte Rechenwerte, keine Messgrößen einzelner Lieferungen."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Der nicht erneuerbare Primärenergiefaktor")
        din = _din_ref("GEG Anlage 4")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        X_NAME, X_SRC, X_START, X_METER, X_BAR, UNIT_LEN = -3.75, -3.2, -2.75, 0.35, 1.2, 1.8
        X_LOSS, V, BIG = -1.9, 1.5, 1.7
        rows = [
            ("strom", "Netzstrom", F_P_STROM, P_YELLOW, 1.45),
            ("gas", "Erdgas / Heizöl", F_P_GAS, GAS_AMBER, 0.62),
            ("holz", "Holz (Biomasse)", F_P_HOLZ, WOOD_BROWN, -0.21),
            ("umwelt", "Umweltwärme, Solar", F_P_UMWELT, AMBIENT_GREEN, -1.04),
        ]
        FP = {k: f for k, n, f, c, y in rows}
        COL = {k: c for k, n, f, c, y in rows}
        YS = {k: y for k, n, f, c, y in rows}

        def source_glyph(key, y):
            if key == "strom":
                return _power_plant(np.array([X_SRC, y - 0.24, 0.0]), w=0.5, h=0.3)
            if key == "gas":
                return _flame(np.array([X_SRC, y, 0.0]), 0.5, color=GAS_AMBER)
            if key == "holz":
                return _tree(np.array([X_SRC, y - 0.25, 0.0]), 0.5)
            return sun_glyph(np.array([X_SRC, y, 0.0])).scale(0.3)

        names = {k: body_text(n, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([X_NAME - 1.4, y, 0.0]))
                 .align_to(np.array([X_NAME, y, 0.0]), RIGHT) for k, n, f, c, y in rows}
        glyphs = {k: source_glyph(k, y) for k, n, f, c, y in rows}
        meters = {k: _meter_glyph(np.array([X_METER + 0.2, y, 0.0])) for k, n, f, c, y in rows}
        rails = {k: Line(np.array([X_START, y, 0.0]), np.array([X_METER, y, 0.0]), color=P_WHITE, stroke_width=6,
                         stroke_opacity=0.2) for k, n, f, c, y in rows}
        ref_line = DashedLine(np.array([X_BAR + UNIT_LEN, -1.4, 0.0]), np.array([X_BAR + UNIT_LEN, 1.8, 0.0]),
                              color=P_WHITE, stroke_width=1.8, stroke_opacity=0.6)
        ref_label = math_label(r"Q_{E}=1\,\mathrm{kWh}", np.array([X_BAR + UNIT_LEN - 0.1, 1.84, 0.0]),
                               size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")
        bar_axis = Line(np.array([X_BAR, -1.4, 0.0]), np.array([X_BAR, 1.8, 0.0]), color=P_WHITE, stroke_width=2,
                        stroke_opacity=0.5)

        def chain_marks(key):
            f, y = FP[key], YS[key]
            at = np.array([X_BAR + (1.0 + f) / 2 * UNIT_LEN, y, 0.0])
            box = DashedVMobject(Rectangle(width=(f - 1.0) * UNIT_LEN + 0.14, height=0.56, color=PRIM_VIOLET,
                                           stroke_width=2.2).move_to(at), num_dashes=24, dashed_ratio=0.55)
            tag = body_text("Vorkette", font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET).move_to(
                np.array([X_BAR + UNIT_LEN + 0.7, y + 0.46, 0.0]))
            return VGroup(box, tag)

        fixed_tag = body_text("im GEG festgelegt (Anlage 4)", font_size=LABEL_FONT_SIZE - 2, color=P_PRIMARY)
        fixed_tag.move_to(np.array([X_NAME + 0.35, YS["strom"] - 0.4, 0.0]))

        trackers = {k: ValueTracker(0.0) for k in FP}

        def factor_bar(key, y, colour):
            tracker = trackers[key]

            def build():
                v = max(0.0, tracker.get_value())
                group = VGroup()
                for lo, hi, fill in ((0.0, min(v, 1.0), colour), (1.0, v, PRIM_VIOLET)):
                    if hi - lo > 0.004:
                        group.add(Rectangle(width=(hi - lo) * UNIT_LEN, height=0.36, color=fill, fill_color=fill,
                                            fill_opacity=0.78, stroke_width=1.2).move_to(
                            np.array([X_BAR + (lo + hi) / 2 * UNIT_LEN, y, 0.0])))
                return group
            return always_redraw(build)

        bars = {k: factor_bar(k, y, c) for k, n, f, c, y in rows}
        reads = {
            k: math_readout(
                lambda t=trackers[k]: rf"f_{{p}}={de_num(t.get_value(), 1)}",
                lambda t=trackers[k], y=y: np.array([X_BAR + max(t.get_value(), 1.0) * UNIT_LEN + 0.22, y - 0.1, 0.0]),
                size=BODY_FONT_SIZE, color=c, edge="left")
            for k, n, f, c, y in rows
        }
        renewable = {}
        for k, n, f, c, y in rows:
            if f < 1.0:
                rest = Rectangle(width=(1.0 - f) * UNIT_LEN, height=0.36, color=AMBIENT_GREEN, stroke_width=1.8)
                rest.move_to(np.array([X_BAR + (f + 1.0) / 2 * UNIT_LEN, y, 0.0]))
                rest = DashedVMobject(rest, num_dashes=22, dashed_ratio=0.55)
                tag = body_text("erneuerbar", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN)
                tag.move_to(np.array([X_BAR + (f + 1.0) / 2 * UNIT_LEN, y, 0.0]))
                renewable[k] = VGroup(rest, tag)

        def supply(key, rt):
            kept, lost = 20, int(round(20 * max(0.0, FP[key] - 1.0)))
            counted = int(round(20 * min(FP[key], 1.0)))
            return _supply_flow(YS[key], X_START, X_METER, X_LOSS, kept, lost, counted, COL[key], AMBIENT_GREEN, rt,
                                v=V)

        def lane(key):
            return [FadeIn(names[key]), FadeIn(glyphs[key]), FadeIn(rails[key]), FadeIn(meters[key])]

        def unlane(key):
            return [FadeOut(rails[key]), FadeOut(meters[key])]

        def grow(key):
            self.add(bars[key], reads[key])
            return [trackers[key].animate.set_value(FP[key])]

        active = []

        def during(rt):
            return [make(rt) for make in active]

        def say(key, *beats, first=False):
            nonlocal caption
            caption = _say(self, self.NARRATION, key, caption, *beats, during=during, first=first)

        row, items = math_row([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p}", P_YELLOW),
        ], font_size=BODY_FONT_SIZE + 4)
        top = np.array([0.0, 2.32, 0.0])
        row.scale(BIG).move_to(np.array([0.0, 0.3, 0.0]))
        ring_fp = highlight_param(items, "fp", color=P_YELLOW)

        say("intro",
            (1.2, [FadeIn(items["qe"], shift=UP * 0.15)]),
            (1.2, [FadeIn(items["dot"]), FadeIn(items["fp"], shift=UP * 0.15)]),
            (1.2, [FadeIn(items["eq"]), FadeIn(items["qp"], shift=UP * 0.15)]), first=True)

        say("zahl",
            (1.0, [Create(ring_fp)]),
            (1.4, [FadeOut(ring_fp), row.animate.scale(1 / BIG).move_to(top)]))

        say("bezug", (1.4, [Create(bar_axis), Create(ref_line)]), (1.2, [FadeIn(ref_label)]))

        active.append(lambda rt: supply("strom", rt))
        say("strom", (1.2, lane("strom")), (1.8, lambda: grow("strom")))

        marks = chain_marks("strom")
        say("strom_mix", (1.4, [Create(marks[0]), FadeIn(marks[1])]))

        active.clear()
        say("strom_geg",
            (1.0, [*unlane("strom"), FadeOut(marks)]),
            (1.2, [FadeIn(fixed_tag)]))

        active.append(lambda rt: supply("gas", rt))
        say("gas",
            (1.0, [FadeOut(fixed_tag)] + lane("gas")),
            (1.6, lambda: grow("gas")))

        marks = chain_marks("gas")
        say("gas_kette", (1.4, [Create(marks[0]), FadeIn(marks[1])]))

        active.clear()
        active.append(lambda rt: supply("holz", rt))
        say("holz",
            (1.0, [*unlane("gas"), FadeOut(marks)] + lane("holz")),
            (1.6, lambda: grow("holz")))

        say("holz_rest", (1.4, [FadeIn(renewable["holz"])]))

        active.clear()
        active.append(lambda rt: supply("umwelt", rt))
        say("umwelt",
            (1.0, unlane("holz") + lane("umwelt")),
            (1.6, lambda: grow("umwelt") + [FadeIn(renewable["umwelt"])]))

        active.clear()
        row2, box2, items2 = math_panel([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p,\mathrm{n.\,ern.}}", P_YELLOW), ("unit", rf"\;[{UNIT}]", P_PRIMARY),
        ])
        say("erneuerbar",
            (1.0, unlane("umwelt")),
            (1.4, [Create(box2), *[ReplacementTransform(items[k], items2[k]) for k in ("qp", "eq", "qe", "dot", "fp")],
                   FadeIn(items2["unit"]), Indicate(VGroup(*reads.values()), color=P_YELLOW, scale_factor=1.06)]))

        note = note_line("f_p nach GEG Anlage 4: nicht erneuerbarer Anteil — festgelegter Rechenwert", color=P_PRIMARY)
        ring = highlight_param(items2, "fp", color=P_YELLOW)
        say("rechenwert", (1.2, [FadeIn(note), Create(ring)]))

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(box2), FadeOut(row2), FadeOut(note),
            FadeOut(VGroup(*names.values())), FadeOut(VGroup(*glyphs.values())),
            FadeOut(bar_axis), FadeOut(ref_line), FadeOut(ref_label),
            *[FadeOut(m) for m in list(bars.values()) + list(reads.values()) + list(renewable.values())],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Ein Gebäude, zwei Anlagen

G3, S3, U3 = -0.75, 0.55, 0.0100


def _plant_panel(px, kind):
    """🏠 One house on one plant: bars at the three boundaries, the plant between them, and the ``f_p`` gauge.

    Everything sits on the same ground line, so the two panels read as columns that can be compared by eye.
    """
    gas = kind == "gas"
    xs = {"h": px + 2.85, "plant": px + 3.95, "e": px + 5.0, "q": px + 6.1}
    y_b = G3 - 0.3
    hs = _house(px + 1.1, G3, S3)
    inside = _furnish(hs, S3, -0.55, 0.45)
    rad_x = float(inside["radiator"].get_center()[0])
    colour = GAS_AMBER if gas else AMBIENT_GREEN
    head = body_text("Gaskessel" if gas else "Wärmepumpe", font_size=BODY_FONT_SIZE, color=colour)
    head.move_to(np.array([px + 1.1, 1.5, 0.0]))
    fan = None
    if gas:
        plant = _boiler(np.array([xs["plant"], G3 + 0.36, 0.0]))
    else:
        heat_pump = _heat_pump(np.array([xs["plant"], G3 + 0.31, 0.0]))
        plant, fan = heat_pump["group"], heat_pump["fan"]
    meter_glyph = _meter_glyph(np.array([xs["e"], y_b, 0.0]), 0.14)
    p_up = Line(np.array([xs["q"], y_b, 0.0]), np.array([xs["e"] + 0.14, y_b, 0.0]))
    p_end = smooth_path([(xs["e"] - 0.14, y_b, 0.0), (xs["plant"] + 0.2, y_b, 0.0), (xs["plant"] + 0.2, G3 + 0.04, 0.0)])
    p_use = smooth_path([(xs["plant"] - 0.2, G3 + 0.04, 0.0), (xs["plant"] - 0.2, y_b, 0.0), (rad_x, y_b, 0.0),
                         (rad_x, G3 + 0.1, 0.0)])
    p_air = Line(np.array([xs["plant"], 0.72, 0.0]), np.array([xs["plant"], G3 + 0.62, 0.0]))
    sun = None if gas else sun_glyph(np.array([xs["plant"], 1.0, 0.0])).scale(0.3)
    trackers = {k: ValueTracker(0.0) for k in ("h", "e", "q")}
    plain = [(Q_NUTZ, NUTZ_BLUE)]
    split = [(Q_END_WP, NUTZ_BLUE), (Q_AMBIENT_WP, AMBIENT_GREEN)]
    stacks = {
        "e": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE)] if gas else [(Q_END_WP, NUTZ_BLUE)],
        "q": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE), (Q_CHAIN_GAS, PRIM_VIOLET)] if gas
        else [(Q_END_WP, NUTZ_BLUE), (Q_CHAIN_WP, PRIM_VIOLET)],
    }
    bars = {"h": _bar_mob(trackers["h"], plain, xs["h"], G3, unit=U3),
            "e": _bar_mob(trackers["e"], stacks["e"], xs["e"], G3, unit=U3),
            "q": _bar_mob(trackers["q"], stacks["q"], xs["q"], G3, unit=U3)}
    stage = {"h": (r"Q_{h}", NUTZ_BLUE), "e": (r"Q_{E}", END_RED), "q": (r"Q_{P}", PRIM_VIOLET)}

    def top(key):
        return G3 + max(0.0, trackers[key].get_value()) * U3

    reads = {}
    for key, (symbol, tint) in stage.items():
        reads[key] = {
            "num": math_readout(lambda t=trackers[key]: de_num(t.get_value()),
                                lambda k=key: np.array([xs[k], top(k) + 0.2, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
            "sym": math_readout(lambda s=symbol: s, lambda k=key: np.array([xs[k], top(k) + 0.58, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
        }

    gauge = meter("f", length=1.6, thickness=0.26, color=P_YELLOW, vertical=False)
    gauge["group"].remove(gauge["label"])
    gauge["group"].move_to(np.array([px + 5.65, 1.85, 0.0]))
    share = ValueTracker(0.0)
    bind_meter(gauge, share)
    tick = Line(gauge["track"].get_top() + DOWN * 0.04, gauge["track"].get_bottom() + UP * 0.04,
                color=P_WHITE, stroke_width=1.6, stroke_opacity=0.7)
    gauge_read = math_readout(lambda: rf"f_{{p}}={de_num(2.0 * share.get_value(), 1)}",
                              np.array([px + 4.75, 1.78, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="right")
    return {
        "kind": kind, "xs": xs, "y_b": y_b, "hs": hs, "inside": inside, "ground": _ground(px, px + 6.6, G3),
        "head": head, "plant": plant, "fan": fan, "meter": meter_glyph, "sun": sun,
        "paths": {"up": p_up, "end": p_end, "use": p_use, "air": p_air}, "pipes": {"up": _pipes(p_up), "end": _pipes(p_end), "use": _pipes(p_use)},
        "trackers": trackers, "bars": bars, "reads": reads, "split": split, "plain": plain, "stacks": stacks,
        "gauge": gauge, "tick": tick, "share": share, "gauge_read": gauge_read, "colour": colour,
        "static": {},
    }


class Beat3_Rechenbeispiel(Scene):
    NARRATION = [
        ("intro",
         "Now the whole chain on one building.",
         "Jetzt die ganze Kette an einem Gebäude."),
        ("nutz",
         "The same hundred kilowatt-hours of useful heat as in the last chapter.",
         "Dieselben hundert Kilowattstunden Nutzwärme wie im letzten Kapitel."),
        ("anlage",
         "The plant turns that useful heat into final energy.",
         "Die Anlage macht aus dieser Nutzwärme Endenergie."),
        ("end_gas",
         "The gas boiler draws a hundred and forty.",
         "Der Gaskessel nimmt hundertvierzig auf."),
        ("end_wp",
         "The heat pump needs only thirty-one of electricity.",
         "Die Wärmepumpe braucht nur einunddreißig Strom."),
        ("end_umwelt",
         "The rest comes free as ambient heat from the air.",
         "Den Rest liefert kostenlos die Umweltwärme aus der Luft."),
        ("prim",
         "Now apply the factors.",
         "Jetzt kommen die Faktoren."),
        ("prim_gas",
         "Gas is multiplied by one point one.",
         "Gas wird mit eins Komma eins multipliziert."),
        ("prim_wp",
         "Electricity is multiplied by one point eight.",
         "Strom wird mit eins Komma acht multipliziert."),
        ("ergebnis",
         "A hundred and fifty-four against fifty-six.",
         "Hundertvierundfünfzig gegen sechsundfünfzig."),
        ("hoeher",
         "The electricity factor is higher, yet the heat pump ends up far lower.",
         "Der Stromfaktor ist höher — und die Wärmepumpe liegt trotzdem weit darunter."),
        ("warum",
         "Because the factor is applied to a number that was already more than three quarters smaller.",
         "Weil der Faktor auf eine Zahl trifft, die schon gut drei Viertel kleiner war."),
        ("effizienz",
         "Efficiency beats the factor.",
         "Effizienz schlägt den Faktor."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Ein Gebäude, zwei Anlagen")
        din = _din_ref("GEG § 20 · DIN V 18599:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        schematic_note = note_line("Vereinfachtes Rechenbeispiel — keine vollständige DIN/GEG-Berechnung", color=P_PRIMARY)
        self.play(FadeIn(caption), FadeIn(schematic_note), run_time=0.3)

        gas, wp = _plant_panel(-6.85, "gas"), _plant_panel(0.1, "wp")
        panels = (gas, wp)

        legend_unit = math_label(rf"[{UNIT}]", size=LABEL_FONT_SIZE, color=P_PRIMARY)
        words = {key: body_text(text, font_size=LABEL_FONT_SIZE, color=colour)
                 for key, text, colour in (("nutz", "Nutzwärme", NUTZ_BLUE), ("loss", "Anlagenverlust", P_ORANGE),
                                           ("env", "Umweltwärme", AMBIENT_GREEN), ("chain", "Vorkette", PRIM_VIOLET))}
        legend = VGroup(legend_unit, *words.values()).arrange(RIGHT, buff=0.45).move_to(np.array([0.0, 2.3, 0.0]))

        streams = []
        houses_on = [True]

        def during(rt):
            anims = []
            if houses_on[0]:
                for p in panels:
                    anims.extend(_heat([p["inside"]["warm"]], rt, 0.35))
            for make in streams:
                anims.extend(make(rt))
            return anims

        def say(key, *beats, first=False):
            nonlocal caption
            caption = _say(self, self.NARRATION, key, caption, *beats, during=during, first=first)

        def shown(*mobs, anims):
            self.add(*mobs)
            return anims

        def freeze(p, key, *, base_src, color):
            x = p["xs"][key]
            value = p["trackers"][key].get_value()
            top_y = G3 + value * U3
            num = _freeze(self, p["reads"][key]["num"], de_num(value), (x, top_y + 0.2), color)
            sym = _freeze(self, p["reads"][key]["sym"], base_src, (x, top_y + 0.58), color)
            p["static"][key] = {"num": num, "sym": sym}

        def end_streams(p):
            streams.append(lambda rt, p=p: [_flow(p["paths"]["end"], END_RED, rt, v=1.3, per_unit=2.2)])
            if p["kind"] == "gas":
                xs = p["xs"]
                streams.append(lambda rt, p=p: [_flow(p["paths"]["use"], NUTZ_BLUE, rt, v=1.3, per_unit=2.2)])
                streams.append(lambda rt, xs=xs: _heat([np.array([xs["plant"], G3 + 0.75, 0.0])], rt, 0.3))
            else:
                streams.append(lambda rt, p=p: [_flow(p["paths"]["use"], NUTZ_BLUE, rt, v=1.3, per_unit=1.0)])

        def ambient_streams(p):
            streams.append(lambda rt, p=p: [_flow(p["paths"]["use"], AMBIENT_GREEN, rt, v=1.3, per_unit=2.3)])
            streams.append(lambda rt, p=p: [_flow(p["paths"]["air"], AMBIENT_GREEN, rt, v=0.9, per_unit=3.0)])
            streams.append(lambda rt, p=p: [Rotate(p["fan"][1], angle=-TAU * rt / 0.9,
                                                  about_point=p["fan"][0].get_center(), rate_func=linear)])

        say("intro",
            (1.5, [*[Create(p["hs"]["group"]) for p in panels], *[Create(p["ground"]) for p in panels],
                   *[FadeIn(p["inside"]["group"]) for p in panels], *[FadeIn(p["head"]) for p in panels]]),
            first=True)

        say("nutz",
            (1.0, [ReplacementTransform(schematic_note, legend_unit), FadeIn(words["nutz"])]),
            (1.6, lambda: shown(*[m for p in panels for m in (p["bars"]["h"], p["reads"]["h"]["num"],
                                                              p["reads"]["h"]["sym"])],
                                anims=[p["trackers"]["h"].animate.set_value(Q_NUTZ) for p in panels])))
        for p in panels:
            freeze(p, "h", base_src=r"Q_{h}", color=NUTZ_BLUE)
            rect = _bar_rects(Q_NUTZ, p["plain"], p["xs"]["h"], G3, unit=U3)
            self.remove(p["bars"]["h"])
            self.add(rect)
            p["rect_h"] = rect

        say("anlage",
            (1.6, [*[FadeIn(p["plant"]) for p in panels], *[FadeIn(p["meter"]) for p in panels],
                   *[FadeIn(p["pipes"][k]) for p in panels for k in ("end", "use")]]))

        end_streams(gas)
        say("end_gas",
            (2.0, lambda: shown(gas["bars"]["e"], gas["reads"]["e"]["num"], gas["reads"]["e"]["sym"],
                                anims=[FadeIn(words["loss"]), gas["trackers"]["e"].animate.set_value(Q_END_GAS)])))
        freeze(gas, "e", base_src=r"Q_{E}", color=END_RED)

        end_streams(wp)
        say("end_wp",
            (1.6, lambda: shown(wp["bars"]["e"], wp["reads"]["e"]["num"], wp["reads"]["e"]["sym"],
                                anims=[wp["trackers"]["e"].animate.set_value(Q_END_WP)])))
        freeze(wp, "e", base_src=r"Q_{E}", color=END_RED)

        ambient_streams(wp)
        split_bar = _bar_rects(Q_NUTZ, wp["split"], wp["xs"]["h"], G3, unit=U3)
        say("end_umwelt",
            (1.2, [FadeIn(wp["sun"]), FadeIn(words["env"])]),
            (1.6, [ReplacementTransform(wp["rect_h"], split_bar)]))
        wp["rect_h"] = split_bar

        streams.clear()
        houses_on[0] = False
        say("prim",
            (1.4, [FadeOut(m) for p in panels for m in (
                p["hs"]["group"], p["inside"]["group"], p["plant"], p["pipes"]["end"], p["pipes"]["use"])] +
                   [FadeOut(wp["sun"])]),
            (1.2, [FadeIn(words["chain"])]))

        def factor_step(p):
            streams.clear()
            streams.append(lambda rt, p=p: [_flow(p["paths"]["up"], PRIM_VIOLET, rt, v=1.0, per_unit=2.4)])
            factor = F_P_GAS if p is gas else F_P_STROM
            prim = Q_PRIM_GAS if p is gas else Q_PRIM_WP
            p["trackers"]["q"].set_value(Q_END_GAS if p is gas else Q_END_WP)
            return [
                (1.0, lambda: shown(p["bars"]["q"], p["reads"]["q"]["num"], p["reads"]["q"]["sym"], p["tick"],
                                    p["gauge"]["track"], p["gauge"]["fill"], p["gauge_read"],
                                    anims=[FadeIn(p["gauge"]["track"]), FadeIn(p["tick"]), FadeIn(p["gauge_read"]),
                                           FadeIn(p["pipes"]["up"])])),
                (2.0, [p["share"].animate.set_value(factor / 2.0), p["trackers"]["q"].animate.set_value(prim)]),
            ]

        say("prim_gas", *factor_step(gas))
        freeze(gas, "q", base_src=r"Q_{P}", color=PRIM_VIOLET)
        gas["static"]["f"] = _freeze(self, gas["gauge_read"], rf"f_{{p}}={de_num(F_P_GAS, 1)}",
                                     (gas["xs"]["e"] - 0.25, 1.78), P_YELLOW, edge="right")

        say("prim_wp", *factor_step(wp))
        freeze(wp, "q", base_src=r"Q_{P}", color=PRIM_VIOLET)
        wp["static"]["f"] = _freeze(self, wp["gauge_read"], rf"f_{{p}}={de_num(F_P_STROM, 1)}",
                                    (wp["xs"]["e"] - 0.25, 1.78), P_YELLOW, edge="right")
        streams.clear()

        rings = VGroup(
            SurroundingRectangle(VGroup(gas["static"]["q"]["num"]), color=P_RED, buff=_theme_module.UI_SPACING, corner_radius=0,
                                 stroke_width=2.6),
            SurroundingRectangle(VGroup(wp["static"]["q"]["num"]), color=AMBIENT_GREEN, buff=_theme_module.UI_SPACING, corner_radius=0,
                                 stroke_width=2.6),
        )
        ghost_q = _ghost(wp["xs"]["q"], G3, Q_PRIM_GAS, unit=U3)
        say("ergebnis", (1.4, [Create(rings), FadeIn(ghost_q)]))

        say("hoeher",
            (1.6, [Indicate(VGroup(wp["gauge"]["track"], wp["gauge"]["fill"], wp["static"]["f"]), color=P_YELLOW,
                            scale_factor=1.12)]))

        ghost_e = _ghost(wp["xs"]["e"], G3, Q_END_GAS, unit=U3)
        say("warum", (1.4, [FadeOut(rings), FadeIn(ghost_e)]))

        parts = [
            ("g_qe", de_num(Q_END_GAS), END_RED), ("g_dot", r"\cdot", P_WHITE),
            ("g_fp", de_num(F_P_GAS, 1), P_YELLOW), ("g_eq", _eq(Q_END_GAS * F_P_GAS), P_WHITE),
            ("g_qp", de_num(Q_PRIM_GAS), PRIM_VIOLET), (None, r"\quad", P_WHITE),
            ("w_qe", de_num(Q_END_WP), END_RED), ("w_dot", r"\cdot", P_WHITE),
            ("w_fp", de_num(F_P_STROM, 1), P_YELLOW), ("w_eq", _eq(Q_END_WP * F_P_STROM), P_WHITE),
            ("w_qp", de_num(Q_PRIM_WP), PRIM_VIOLET), ("unit", rf"\;{UNIT}", P_PRIMARY),
        ]
        row, box, items = math_panel(parts)
        copies = []
        for p, tag in ((gas, "g"), (wp, "w")):
            copies += [
                TransformFromCopy(p["static"]["e"]["num"], items[f"{tag}_qe"]),
                TransformFromCopy(p["static"]["f"], items[f"{tag}_fp"]),
                TransformFromCopy(p["static"]["q"]["num"], items[f"{tag}_qp"]),
                FadeIn(items[f"{tag}_dot"]), FadeIn(items[f"{tag}_eq"]),
            ]
        say("effizienz",
            (1.8, [Create(box), FadeIn(items["unit"]), *copies]),
            (1.0, [Indicate(VGroup(items["g_qp"], items["w_qp"]), color=P_YELLOW, scale_factor=1.15)]))

        everything = [
            caption, box, row, ghost_q, ghost_e, legend_unit, *words.values(),
            *[m for p in panels for m in (
                p["ground"], p["meter"], p["head"], p["pipes"]["up"],
                p["rect_h"], p["bars"]["e"], p["bars"]["q"], p["gauge"]["track"], p["gauge"]["fill"], p["tick"],
                *[m for tag in p["static"].values() for m in (tag.values() if isinstance(tag, dict) else [tag])],
            )],
        ]
        self.play(*[FadeOut(m) for m in everything], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Der Grenzwert: das Referenzgebäude

G4, S4, U4 = -0.45, 0.62, 0.0125


class Beat4_Referenzgebaeude(Scene):
    NARRATION = [
        ("intro",
         "So what does the law compare those numbers against?",
         "Womit vergleicht das Gesetz diese Zahlen?"),
        ("pauschal",
         "Not against a flat limit, but against a reference building with your own geometry.",
         "Nicht mit einem pauschalen Grenzwert, sondern mit einem Referenzgebäude Ihrer Geometrie."),
        ("referenz",
         "It takes the identical geometry.",
         "Es nimmt dieselbe Geometrie."),
        ("anlage1",
         "And it fills it with the technical reference specification from Annex 1: the reference building.",
         "Und es füllt sie mit der technischen Referenzausführung nach Anlage 1: das Referenzgebäude."),
        ("grenze",
         "Its calculated primary energy sets the benchmark.",
         "Dessen Primärenergiebedarf setzt den Maßstab."),
        ("anteil",
         "A new residential building may reach at most fifty-five percent of it.",
         "Ein neues Wohngebäude darf höchstens fünfundfünfzig Prozent davon erreichen."),
        ("waermepumpe",
         "In this example the heat pump clears the line.",
         "Im Beispiel unterschreitet die Wärmepumpe die Linie."),
        ("gaskessel",
         "The gas boiler misses it by more than a factor of two.",
         "Der Gaskessel verfehlt sie um mehr als das Doppelte."),
        ("huelle",
         "The GEG also demands thermal protection of the envelope.",
         "Daneben verlangt das GEG baulichen Wärmeschutz."),
        ("erneuerbar",
         "New heating systems must run on 65 percent renewable energy, with transition periods.",
         "Für neue Heizungen gilt — mit Übergangsfristen — ein Anteil von 65 Prozent erneuerbarer Energie."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Der Grenzwert: das Referenzgebäude")
        din = _din_ref("GEG §§ 15–17 · § 71 · Anlage 1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        design = _house(-5.25, G4, S4)
        reference = _house(-1.95, G4, S4)
        design_in = _furnish(design, S4, -0.55, 0.45)
        reference_in = _furnish(reference, S4, -0.55, 0.45)
        ground = _ground(-6.9, 6.7, G4)

        def dims(hs):
            tl, bl, tr = hs["top_left"], hs["bottom_left"], hs["top_right"]
            vertical = dim_arrow(np.array([bl[0] - 0.4, bl[1], 0.0]), np.array([bl[0] - 0.4, tl[1], 0.0]), P_YELLOW)
            horizontal = dim_arrow(np.array([0.0, 0.0, 0.0]), np.array([0.0, tr[0] - tl[0], 0.0]), P_YELLOW)
            horizontal.rotate(-PI / 2).move_to(np.array([(tl[0] + tr[0]) / 2, hs["roof_peak"][1] + 0.28, 0.0]))
            return VGroup(vertical, horizontal)

        dims_d, dims_r = dims(design), dims(reference)
        envelope_d, envelope_r = _envelope(design, S4), _envelope(reference, S4)
        tag_d = body_text("Ihr Entwurf", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([-5.25, -0.95, 0.0]))
        tag_r = body_text("Referenzgebäude", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([-1.95, -0.95, 0.0]))
        spec = chip("Anlage 1", P_PRIMARY, font_size=LABEL_FONT_SIZE).move_to(np.array([-1.95, G4 + 1.1, 0.0]))

        SHIFT, x_ref, x_wp, x_gas = 3.85, -3.4, -0.9, 1.6
        ox = ValueTracker(SHIFT)
        t_ref, t_share, t_gas, t_wp = (ValueTracker(0.0) for _ in range(4))
        bar_ref = always_redraw(lambda: _bar_rects(t_ref.get_value(), [(Q_REF, PRIM_VIOLET)], x_ref + ox.get_value(),
                                                   G4, width=1.0, unit=U4))
        fill = always_redraw(lambda: Rectangle(
            width=0.72, height=max(0.002, t_share.get_value() * t_ref.get_value() * U4), stroke_width=0,
            fill_color=P_RED, fill_opacity=0.85).move_to(np.array(
                [x_ref + ox.get_value(), G4 + t_share.get_value() * t_ref.get_value() * U4 / 2, 0.0])))
        limit_line = always_redraw(lambda: DashedLine(
            np.array([x_ref + ox.get_value() + 0.5, G4 + t_share.get_value() * t_ref.get_value() * U4, 0.0]),
            np.array([6.2, G4 + t_share.get_value() * t_ref.get_value() * U4, 0.0]),
            color=P_RED, stroke_width=2.6))
        share_read = math_readout(
            lambda: rf"{de_num(t_share.get_value(), 2)}\cdot {Q_REF}",
            lambda: np.array([x_ref + ox.get_value(), G4 + Q_REF * U4 + 0.28, 0.0]),
            size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        y_limit = G4 + Q_LIMIT * U4
        limit_card = _card("Grenzwert im Beispiel", math_label(rf"{Q_LIMIT}\,{UNIT}", size=BODY_FONT_SIZE, color=P_RED),
                           P_RED)
        limit_card.move_to(np.array([4.9, 1.15, 0.0]))
        limit_leader = DashedLine(limit_card.get_bottom(), np.array([4.9, y_limit, 0.0]), color=P_RED,
                                  stroke_width=1.6, stroke_opacity=0.6)
        share_note = note_line(
            f"Neubau (Wohngebäude): max. {int(REF_SHARE * 100)} % des Referenzwerts — hier schematisch", color=P_RED)
        bar_gas = _bar_mob(t_gas, [(Q_LIMIT, GAS_AMBER), (Q_PRIM_GAS - Q_LIMIT, P_RED)], x_gas, G4, width=1.0, unit=U4)
        bar_wp = _bar_mob(t_wp, [(Q_PRIM_WP, AMBIENT_GREEN)], x_wp, G4, width=1.0, unit=U4)

        def caption_under(name, colour, tracker, x):
            label = body_text(name, font_size=LABEL_FONT_SIZE, color=colour).move_to(np.array([x - 0.3, -0.95, 0.0]))
            number = _count(tracker, lambda: np.array([label.get_right()[0] + 0.1, -1.01, 0.0]), colour, edge="left")
            return label, number

        name_ref, num_ref = caption_under("Referenz", PRIM_VIOLET, t_ref, x_ref + SHIFT)
        name_gas, num_gas = caption_under("Gaskessel", GAS_AMBER, t_gas, x_gas)
        name_wp, num_wp = caption_under("Wärmepumpe", AMBIENT_GREEN, t_wp, x_wp)
        double_line = DashedLine(np.array([x_gas - 0.7, G4 + 2 * Q_LIMIT * U4, 0.0]),
                                 np.array([x_gas + 0.75, G4 + 2 * Q_LIMIT * U4, 0.0]), color=P_RED, stroke_width=1.8,
                                 stroke_opacity=0.55)
        double_tag = math_label(r"2\times", np.array([x_gas + 0.88, G4 + 2 * Q_LIMIT * U4 - 0.06, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        path_ref = smooth_path([(reference["bottom_right"][0] + 0.1, G4 - 0.2, 0.0),
                                (x_ref + SHIFT - 0.5, G4 - 0.2, 0.0)])
        pipe_ref = _pipes(path_ref)
        verdict = note_line("im Beispiel unterschreitet nur die Wärmepumpe den Grenzwert", color=AMBIENT_GREEN)

        share_gauge = meter("e", length=1.0, thickness=0.2, color=AMBIENT_GREEN, vertical=False)
        share_gauge["group"].remove(share_gauge["label"])
        share_gauge["group"].move_to(np.array([3.0, 0.5, 0.0]))
        t_ee = ValueTracker(0.0)
        bind_meter(share_gauge, t_ee)
        ee_name = body_text("erneuerbar", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(
            np.array([3.0, 0.8, 0.0]))
        ee_read = math_readout(lambda: rf"{de_num(100.0 * t_ee.get_value())}\,\%", np.array([3.65, 0.44, 0.0]),
                               size=LABEL_FONT_SIZE, color=AMBIENT_GREEN, edge="left")

        warm = [design_in["warm"]]
        active = [lambda rt: _heat(warm, rt, 0.35)]

        def during(rt):
            anims = []
            for make in active:
                anims.extend(make(rt))
            return anims

        def say(key, *beats, first=False):
            nonlocal caption
            caption = _say(self, self.NARRATION, key, caption, *beats, during=during, first=first)

        def shown(*mobs, anims):
            self.add(*mobs)
            return anims

        say("intro",
            (1.4, [Create(design["group"]), Create(ground), FadeIn(design_in["group"])]),
            (1.0, [FadeIn(tag_d)]), first=True)

        say("pauschal", (1.4, [Create(dims_d)]))

        warm.append(reference_in["warm"])
        say("referenz",
            (1.8, [TransformFromCopy(design["group"], reference["group"]),
                   TransformFromCopy(design_in["group"], reference_in["group"]),
                   TransformFromCopy(dims_d, dims_r), FadeIn(tag_r)]))

        say("anlage1", (1.4, [Create(envelope_r)]), (1.0, [FadeIn(spec)]))

        warm[:] = [reference_in["warm"]]
        active.append(lambda rt: [_flow(path_ref, PRIM_VIOLET, rt, v=1.0, per_unit=3.0)])
        say("grenze",
            (1.2, [FadeOut(m) for m in (design["group"], design_in["group"], dims_d, dims_r, tag_d, spec, envelope_r)]),
            (1.8, lambda: shown(bar_ref, num_ref, anims=[FadeIn(pipe_ref), FadeIn(name_ref),
                                                         t_ref.animate.set_value(Q_REF)])))

        active.clear()
        say("anteil",
            (1.0, [FadeOut(m) for m in (reference["group"], reference_in["group"], tag_r, pipe_ref)]),
            (1.4, [ox.animate.set_value(0.0), name_ref.animate.shift(LEFT * SHIFT)]),
            (1.6, lambda: shown(fill, limit_line, share_read, anims=[
                FadeIn(share_note), t_share.animate.set_value(REF_SHARE), FadeIn(limit_card), Create(limit_leader)])))

        say("waermepumpe",
            (1.0, [FadeOut(share_note), FadeIn(name_wp)]),
            (1.6, lambda: shown(bar_wp, num_wp, anims=[t_wp.animate.set_value(Q_PRIM_WP), FadeIn(verdict)])))

        say("gaskessel",
            (1.0, [FadeIn(name_gas)]),
            (1.6, lambda: shown(bar_gas, num_gas, anims=[t_gas.animate.set_value(Q_PRIM_GAS)])),
            (1.0, [Create(double_line), FadeIn(double_tag)]))

        chart = (bar_ref, fill, limit_line, share_read, limit_card, limit_leader, bar_gas, bar_wp, name_ref, name_gas,
                 name_wp, num_ref, num_gas, num_wp, double_line, double_tag, verdict)
        house_d = VGroup(design["group"], design_in["group"], envelope_d)
        house_d.shift(RIGHT * 5.25)
        row, box, items = math_panel([("ht", r"H′_{T}", P_CYAN), ("ht_ref", r"\text{(§ 16)}", P_PRIMARY)])
        ring_ht = highlight_param(items, "ht", color=P_CYAN)
        say("huelle",
            (1.2, [FadeOut(m) for m in chart]),
            (1.4, [Create(design["group"]), FadeIn(design_in["group"])]),
            (1.2, [Create(envelope_d)]),
            (1.4, [Create(box), FadeIn(items["ht_ref"]), TransformFromCopy(envelope_d, items["ht"]),
                   Indicate(envelope_d, color=P_CYAN, scale_factor=1.02)]),
            (0.5, [Create(ring_ht)]))

        row2, box2, items2 = math_panel([
            ("ee", r"65\,\%", AMBIENT_GREEN), ("ee_text", r"\text{erneuerbar für neue Heizungen (§ 71)}", P_PRIMARY),
        ])
        frozen = {}

        def freeze_ee():
            frozen["ee"] = _freeze(self, ee_read, rf"{de_num(100.0 * EE_SHARE)}\,\%", (3.65, 0.44), AMBIENT_GREEN,
                                   edge="left")
            return [Create(box2), TransformFromCopy(frozen["ee"], items2["ee"]), FadeIn(items2["ee_text"])]

        say("erneuerbar",
            (1.0, [FadeOut(box), FadeOut(row), FadeOut(ring_ht)]),
            (1.6, lambda: shown(ee_read, anims=[FadeIn(share_gauge["track"]), FadeIn(share_gauge["fill"]),
                                                FadeIn(ee_name), t_ee.animate.set_value(EE_SHARE)])),
            (1.2, freeze_ee))

        self.play(
            FadeOut(caption), FadeOut(box2), FadeOut(row2),
            *[FadeOut(m) for m in (design["group"], design_in["group"], ground, envelope_d,
                                   share_gauge["track"], share_gauge["fill"], ee_name, frozen["ee"])],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion
