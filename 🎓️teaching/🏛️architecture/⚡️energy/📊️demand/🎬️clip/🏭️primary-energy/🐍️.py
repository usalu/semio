

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
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
room_metre = _manim_module.room_metre
P_PRIMARY = _manim_module.P_PRIMARY
reference_chip = _manim_module.reference_chip
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
staffage = _manim_module.staffage
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
fitted_staffage = _manim_module.fitted_staffage
BOILER_RATE = 60.0
BOILER_CLEARANCE_M = 0.6
sun_glyph = _manim_module.sun_glyph
math_label = _manim_module.math_label
math_row = _manim_module.math_row
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
label_number = _manim_module.label_number
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
load_vo_timing = _manim_module.load_vo_timing

_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Primärenergie & das GModG"

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
E_E_GAS = 1.04 * 1.08 * 1.06 * 1.18   # ≈ 1,40
E_E_WP = 1.04 * 1.08 * 1.06 * 0.26    # ≈ 0,31
Q_END_GAS = round(Q_NUTZ * E_E_GAS)   # 140 kWh/(m²·a)
Q_END_WP = round(Q_NUTZ * E_E_WP)     #  31 kWh/(m²·a)
F_P_GAS, F_P_STROM = 1.1, 1.8          # GModG Anlage 4, nicht erneuerbarer Anteil
F_P_HOLZ, F_P_UMWELT = 0.2, 0.0
# From the *displayed* final energy, so the multiplication on screen checks out
# by hand; half-up because round() is banker's rounding.
Q_PRIM_GAS = int(Q_END_GAS * F_P_GAS + 0.5)    # 154 kWh/(m²·a)
Q_PRIM_WP = int(Q_END_WP * F_P_STROM + 0.5)    #  56 kWh/(m²·a)
E_P_GAS = Q_PRIM_GAS / Q_NUTZ                  # 1,54
E_P_WP = Q_PRIM_WP / Q_NUTZ                    # 0,56
Q_LOSS_GAS = Q_END_GAS - Q_NUTZ                #  40 kWh/(m²·a) Anlagenverluste
WP_GEN_OUT = Q_NUTZ * 1.04 * 1.08 * 1.06       # 119 kWh/(m²·a) Erzeugerabgabe
Q_AMBIENT_WP = round(Q_NUTZ * (1.0 - Q_END_WP * 1.0 / WP_GEN_OUT))  #  74 kWh/(m²·a) Umweltwärme-Anteil der Nutzenergie
Q_POWER_SHARE = Q_NUTZ - Q_AMBIENT_WP          #  26 kWh/(m²·a) Strom-Anteil der Nutzenergie
Q_CHAIN_GAS = Q_PRIM_GAS - Q_END_GAS           #  14 kWh/(m²·a) Vorkette
Q_CHAIN_WP = Q_PRIM_WP - Q_END_WP              #  25 kWh/(m²·a) Vorkette
# Reference-building value is building-specific; this one is a schematic
# example. The 55 % share is the GEG § 15 requirement for new residential builds.
Q_REF, REF_SHARE = 120, 0.55
Q_LIMIT = int(Q_REF * REF_SHARE + 0.5)          #  66 kWh/(m²·a)
EE_SHARE = 0.65

UNIT = r"\frac{\mathrm{kWh}}{\mathrm{m^{2}\,a}}"

def _de(value: float, digits: int = 2) -> str:
    """🔢 Decimal comma in braces for narration TeX."""
    return f"{value:.{digits}f}".replace(".", "{,}")


def _eq(product: float) -> str:
    """🟰 '=' only when the shown product is exact — 140 · 1,1 = 154 but 31 · 1,8 ≈ 56."""
    return "=" if abs(product - round(product)) < 1e-9 else r"\approx"


def _math_note(tex: str, color: str = P_PRIMARY):
    """💭 One-line aside with typeset symbols on the reserved band under the beat subtitle."""
    return math_label(tex, np.array([0.0, _manim_module.NOTE_BAND_Y, 0.0]), size=LABEL_FONT_SIZE, color=color)
#endregion


#region Shared visual motifs

#region Buildings and people
def _stand(floor_y, unit, x):
    """🧍 Breathing occupant at the room scale."""
    return staffage_motion("shared-person-winter", unit=unit, anchor=np.array([x, floor_y, 0.0]))

def _house(center_x, floor_y, s):
    """🏠 Physical Fundamentals house section standing on ``floor_y``."""
    return house_section(np.array([center_x, floor_y + 1.2 * s, 0.0]), scale=s)


def _furnish(hs, s, person_dx, rad_dx, *, person=True):
    """🛋️ Ground-floor occupant and radiator inside a ``house_section`` — returns both and their warm spot."""
    floor_y, cx = float(hs["bottom_left"][1]), float(hs["center"][0])
    unit = room_metre(hs["w_height"] / 2)
    heater = staffage("shared-radiator-front-winter", unit=unit,
                      anchor=np.array([cx + rad_dx, floor_y, 0.0]))
    figure = _stand(floor_y, unit, cx + person_dx) if person else VGroup()
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


def _derrick(base, unit):
    """🛢️ Animated oil pumpjack at the supplied physical metre."""
    return staffage_motion("demand-primary-energy-pumpjack", unit=unit, anchor=_pt(base), rate=1.0, loop=True)


def _power_plant(base, unit):
    """🏭 Container generator representing conversion at the supplied metre."""
    return staffage("shared-diesel-generator-rated", unit=unit, anchor=_pt(base))


def _pump(center, unit):
    """🌀 Circulation pump at the building scale."""
    return staffage_motion("shared-pump-run-high", unit=unit, rate=1.0).move_to(_pt(center))


def _flame(center, h=0.5, color=P_ORANGE):
    """🔥 An irregular ink tongue with a tapered split tip."""
    profile = _manim_module.icon_glyph("flame", color=color)
    profile.scale(h / profile.height)
    return profile.move_to(_pt(center))


def _tree(base, h=0.5):
    """🌳 Handcrafted oak, ``h`` tall with its foot on ``base``."""
    cast = "demand-primary-energy-tree"
    return staffage(cast, unit=h / staffage_height(cast), anchor=_pt(base))


def _boiler(floor, unit):
    """⛽ Wall-hung boiler with its mounting clearance measured from the floor datum."""
    return staffage_motion("shared-boiler-heating-cycle", unit=unit, anchor=_pt(floor) + UP * BOILER_CLEARANCE_M * unit, rate=BOILER_RATE)


def _heat_pump(base, unit):
    """♻️ Outdoor heat-pump unit at the supplied physical metre."""
    return staffage_motion("shared-heat-pump-heating-modulation", unit=unit, anchor=_pt(base), rate=0.0)


METER_CASTS = {"strom": "shared-electricity-meter-run", "gas": "shared-gas-meter-run"}
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

    The upstream density is ``(kept + lost) / kept`` times the delivered density — exactly the factor ``f_P``
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


def _count(tracker, at, color, *, digits=0, size=None, edge="center", prefix="", suffix=""):
    """🔢 Live number that counts up from its tracker, with its unit as ``suffix``."""
    return math_readout(lambda: prefix + label_number(tracker.get_value(), digits) + suffix, at, size=size or LABEL_FONT_SIZE,
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

def _upstream_inset(ground):
    """🔭 The frame of the upstream chain with its caption below: a square outline on the shared frame owner, filled with the scene behind it.
    @see 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py window_frame
    """
    inset = _manim_module.window_frame(3.35, 1.15, color=P_PRIMARY, stroke_width=1.2, fill_opacity=0.0).move_to(np.array([-5.1, ground + 0.45, 0.0]))
    label = body_text(tr("Upstream: separate scale", "Vorkette: eigener Maßstab"), font_size=LABEL_FONT_SIZE - 4, color=P_PRIMARY).move_to(np.array([-5.1, ground - 0.28, 0.0]))
    return inset, label


class Beat1_DreiBilanzgrenzen(Scene):
    NARRATION = [
        ("intro",
         "Everything in this series has depended on one decision: where we draw the balance boundary.",
         "Alles in dieser Serie hing an einer Entscheidung: wo wir die Bilanzgrenze ziehen."),
        ("raum",
         r"Draw it around the room and you count useful energy, $Q_{h}$.",
         r"Zieht man sie um den Raum, zählt man Nutzenergie, $Q_{h}$."),
        ("heizwaerme",
         "That is the heat the balance said the room needs.",
         "Das ist die Wärme, die die Bilanz für den Raum verlangt."),
        ("gebaeude",
         r"Draw it around the building at the meter and you count final energy, $Q_{E}$.",
         r"Zieht man sie am Zähler um das Gebäude, zählt man Endenergie, $Q_{E}$."),
        ("verluste",
         "That includes the plant losses: the waste heat of the boiler counts as well.",
         "Dazu zählen die Anlagenverluste: Auch die Abwärme des Kessels wird mitgezählt."),
        ("quelle",
         r"Draw it all the way back to the source and you count primary energy, $Q_{P}$.",
         r"Zieht man sie bis zur Quelle, zählt man Primärenergie, $Q_{P}$."),
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
         r"That is why the Building Modernisation Act caps the primary energy demand $Q_{P} = Q_{E} \cdot f_{P}$.",
         r"Deshalb begrenzt das Gebäudemodernisierungsgesetz den Primärenergiebedarf $Q_{P} = Q_{E} \cdot f_{P}$."),
        ("vergleich",
         "Because the primary energy factor puts different energy carriers on one common scale.",
         "Denn der Primärenergiefaktor bringt verschiedene Energieträger auf einen gemeinsamen Maßstab."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("Three system boundaries", "Drei Bilanzgrenzen"))
        din = reference_chip(_references.cite(("din-ts-18599-1", "energiearten")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        revision_note = note_line(
            tr(f"Since 10/2025 {_references.designation('din-ts-18599-1')} · GModG § 20 still cites {_references.designation('din-v-18599-1')}",
               f"Seit 10/2025 {_references.designation('din-ts-18599-1')} · GModG § 20 nennt noch {_references.designation('din-v-18599-1')}"),
            color=P_PRIMARY,
        )
        law_note = note_line(
            tr("GModG since 29 July 2026, formerly GEG (Building Energy Act)", "GModG seit 29.07.2026, vormals GEG (Gebäudeenergiegesetz)"),
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

        ground = _ground(-2.2, 3.7, G)
        metre = room_metre(hs["w_height"] / 2)
        upstream_metre = 0.14
        inset, inset_label = _upstream_inset(G)
        annex = _outline(
            Line(np.array([wall_r, G + 0.95, 0.0]), np.array([wall_r + 1.05, G + 0.95, 0.0])),
            Line(np.array([wall_r + 1.05, G + 0.95, 0.0]), np.array([wall_r + 1.05, G, 0.0])),
        )
        boiler = _boiler(np.array([annex_x, G, 0.0]), metre)
        meter_glyph = fitted_staffage("shared-gas-meter-run", width=0.34, height=0.38, rate=1.0, loop=True).move_to(np.array([meter_x, y_ground, 0.0]))
        derrick = _derrick(np.array([-6.1, G]), upstream_metre)
        plant = _power_plant(np.array([-4.45, G]), upstream_metre)
        pump = _pump(np.array([-2.75, y_pipe]), metre)
        names = [
            body_text(text, font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET).move_to(np.array([x, G + 1.27, 0.0]))
            for text, x in ((tr("Extraction", "Gewinnung"), -5.9), (tr("Conversion", "Umwandlung"), -4.3), ("Transport", -2.75))
        ]
        loss_name = body_text(tr("System losses", "Anlagenverluste"), font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([4.95, G + 0.45, 0.0]))
        loss_arrow = Arrow(np.array([4.0, G + 0.45, 0.0]), np.array([annex_x + 0.4, G + 0.42, 0.0]), buff=0,
                           color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.25)

        path_up = smooth_path([(-5.9, y_pipe, 0.0), (-4.3, y_pipe, 0.0), (-2.75, y_pipe, 0.0), (-2.3, y_pipe, 0.0),
                               (-1.95, y_ground, 0.0), (meter_x, y_ground, 0.0)])
        path_end = smooth_path([(meter_x, y_ground, 0.0), (HX + 0.4, y_ground, 0.0), (annex_x, y_ground, 0.0),
                                tuple(boiler.get_bottom())])
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

        specs = ((tr("Room", "Raum"), tr("Useful energy", "Nutzenergie"), r"Q_{h}", NUTZ_BLUE), (tr("Meter", "Zähler"), tr("Final energy", "Endenergie"), r"Q_{E}", END_RED),
                 (tr("Source", "Quelle"), tr("Primary energy", "Primärenergie"), r"Q_{P}", PRIM_VIOLET))
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

        say("gewinnung", (1.2, [FadeIn(inset), FadeIn(inset_label), FadeIn(derrick), FadeIn(names[0])]))

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
            math_label(tr(r"\text{Plant}\;e_{E}", r"\text{Anlage}\;e_{E}"), np.array([-2.05, 1.22, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY),
            math_label(r"f_{P}", np.array([2.0, 1.1, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW),
        )
        row, box, items = math_panel([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{P}", P_YELLOW), ("cap", tr(r"\;\text{← limited by the GModG}", r"\;\text{← vom GModG begrenzt}"), P_PRIMARY),
        ])
        rings = {}
        scenery = (hs["group"], ground, inside["group"], meter_glyph, inset, inset_label, derrick, plant, pump, pipe_up, ring_source,
                   cards[2]["card"], *names)
        say("geg",
            (1.0, [FadeOut(m) for m in scenery]),
            (1.6, [FadeIn(rc["card"]) for rc in row_cards] + [GrowArrow(h) for h in hops] + [FadeIn(hop_names), FadeIn(law_note)]),
            (1.4, [Create(box), FadeIn(items["eq"]), FadeIn(items["dot"]), FadeIn(items["cap"]),
                   stage.transition("demand.primary-energy.boundaries.symbol-qp-into-formula", source=row_cards[2]["sym"], target=items["qp"]),
                   stage.transition("demand.primary-energy.boundaries.symbol-qe-into-formula", source=row_cards[1]["sym"], target=items["qe"]),
                   FadeIn(items["fp"], shift=UP * 0.2)]),
            (0.8, lambda: [Create(rings.setdefault("p", highlight_param(items, "qp", color=P_YELLOW)))]))

        carriers = [chip(text, colour, font_size=BODY_FONT_SIZE) for text, colour in (
            (tr("Natural gas", "Erdgas"), GAS_AMBER), (tr("Electricity", "Strom"), P_YELLOW), (tr("Wood", "Holz"), WOOD_BROWN), (tr("Ambient heat", "Umweltwärme"), AMBIENT_GREEN))]
        VGroup(*carriers).arrange(RIGHT, buff=0.6).move_to(np.array([0.0, 0.7, 0.0]))
        say("vergleich",
            (1.0, [FadeOut(rc["card"]) for rc in row_cards] + [FadeOut(h) for h in hops] + [FadeOut(hop_names),
                                                                                         FadeOut(rings["p"])]),
            (2.4, [LaggedStart(*[FadeIn(c, shift=UP * 0.15) for c in carriers], lag_ratio=0.45)]),
            (1.0, [Indicate(items["fp"], color=P_YELLOW, scale_factor=1.15)]))

        self.play(FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(law_note), *[FadeOut(c) for c in carriers], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Der Primärenergiefaktor

class Beat2_Primaerenergiefaktor(Scene):
    NARRATION = [
        ("intro",
         r"The step from final energy $Q_{E}$ to primary energy $Q_{P}$ is one multiplication.",
         r"Der Schritt von der Endenergie $Q_{E}$ zur Primärenergie $Q_{P}$ ist eine Multiplikation."),
        ("zahl",
         r"The multiplier, the primary energy factor $f_{P}$, is one number for each energy carrier.",
         r"Der Faktor, der Primärenergiefaktor $f_{P}$, ist eine Zahl je Energieträger."),
        ("bezug",
         "It says how many kilowatt-hours of primary energy one kilowatt-hour of final energy needs, so it is dimensionless.",
         "Er sagt, wie viele Kilowattstunden Primärenergie eine Kilowattstunde Endenergie braucht, und ist deshalb dimensionslos."),
        ("strom",
         r"Grid electricity carries the highest factor among household carriers: $f_{P} = 1.8$.",
         r"Netzstrom trägt unter den Haushalts-Energieträgern den höchsten Faktor: $f_{P} = 1{,}8$."),
        ("strom_mix",
         "Its generation mix still has a large fossil share.",
         "Sein Erzeugungsmix hat noch einen großen fossilen Anteil."),
        ("strom_geg",
         "The value is fixed by law: Annex 4 of the Building Modernisation Act.",
         "Der Wert ist gesetzlich festgelegt: Anlage 4 des Gebäudemodernisierungsgesetzes."),
        ("gas",
         r"Gas and oil sit just above one: $f_{P} = 1.1$.",
         r"Gas und Öl liegen knapp über eins: $f_{P} = 1{,}1$."),
        ("gas_kette",
         "The fuel itself is fossil, plus a small share for the supply chain.",
         "Der Brennstoff selbst ist fossil, dazu kommt ein kleiner Anteil für die Vorkette."),
        ("holz",
         r"Wood is counted at a fifth: $f_{P} = 0.2$.",
         r"Holz wird mit einem Fünftel angesetzt: $f_{P} = 0{,}2$."),
        ("holz_rest",
         "The remaining four fifths are renewable and do not count.",
         "Die übrigen vier Fünftel sind erneuerbar und zählen nicht."),
        ("umwelt",
         r"Ambient heat from air or ground counts as zero: $f_{P} = 0$.",
         r"Umweltwärme aus Luft oder Erdreich zählt als null: $f_{P} = 0$."),
        ("erneuerbar",
         "These are the non-renewable shares.",
         "Das sind die nicht erneuerbaren Anteile."),
        ("rechenwert",
         "They are fixed calculation values, not measurements of any individual delivery.",
         "Es sind festgelegte Rechenwerte, keine Messgrößen einzelner Lieferungen."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The non-renewable primary energy factor", "Der nicht erneuerbare Primärenergiefaktor"))
        din = reference_chip(_references.cite(("gmodg", "p22")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        X_NAME, X_SRC, X_START, X_METER, X_BAR, UNIT_LEN = -3.75, -3.2, -2.75, 0.35, 1.2, 1.8
        X_LOSS, V, BIG = -1.9, 1.5, 1.7
        rows = [
            ("strom", tr("Grid electricity", "Strom, netzbezogen"), F_P_STROM, P_YELLOW, 1.45),
            ("gas", tr("Natural gas / fuel oil", "Erdgas / Heizöl"), F_P_GAS, GAS_AMBER, 0.62),
            ("holz", tr("Wood", "Holz"), F_P_HOLZ, WOOD_BROWN, -0.21),
            ("umwelt", tr("Ambient heat", "Umgebungswärme"), F_P_UMWELT, AMBIENT_GREEN, -1.04),
        ]
        FP = {k: f for k, n, f, c, y in rows}
        COL = {k: c for k, n, f, c, y in rows}
        YS = {k: y for k, n, f, c, y in rows}

        def source_glyph(key, y):
            if key == "strom":
                return fitted_staffage("shared-diesel-generator-rated", width=0.5, height=0.48).move_to(np.array([X_SRC, y, 0.0]))
            if key == "gas":
                return _flame(np.array([X_SRC, y, 0.0]), 0.5, color=GAS_AMBER)
            if key == "holz":
                return _tree(np.array([X_SRC, y - 0.25, 0.0]), 0.5)
            return sun_glyph(np.array([X_SRC, y, 0.0])).scale(0.3)

        names = {k: body_text(n, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([X_NAME - 1.4, y, 0.0]))
                 .align_to(np.array([X_NAME, y, 0.0]), RIGHT) for k, n, f, c, y in rows}
        glyphs = {k: source_glyph(k, y) for k, n, f, c, y in rows}
        meters = {k: fitted_staffage(METER_CASTS[k], width=0.44, height=0.5, rate=1.0, loop=True).move_to(np.array([X_METER + 0.2, y, 0.0])) for k, n, f, c, y in rows if k in METER_CASTS}
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
            tag = body_text(tr("Upstream chain", "Vorkette"), font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET).move_to(
                np.array([X_BAR + UNIT_LEN + 0.7, y + 0.46, 0.0]))
            return VGroup(box, tag)

        fixed_tag = body_text(tr("set by law (Annex 4)", "gesetzlich festgelegt (Anlage 4)"), font_size=LABEL_FONT_SIZE - 2, color=P_PRIMARY)
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
                lambda t=trackers[k]: rf"f_{{P}}={label_number(t.get_value(), 1)}",
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
                tag = body_text(tr("renewable", "erneuerbar"), font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN)
                tag.move_to(np.array([X_BAR + (f + 1.0) / 2 * UNIT_LEN, y, 0.0]))
                renewable[k] = VGroup(rest, tag)

        def supply(key, rt):
            kept, lost = 20, int(round(20 * max(0.0, FP[key] - 1.0)))
            counted = int(round(20 * min(FP[key], 1.0)))
            return _supply_flow(YS[key], X_START, X_METER, X_LOSS, kept, lost, counted, COL[key], AMBIENT_GREEN, rt,
                                v=V)

        def lane(key):
            return [FadeIn(names[key]), FadeIn(glyphs[key]), FadeIn(rails[key]), *([FadeIn(meters[key])] if key in meters else [])]

        def unlane(key):
            return [FadeOut(rails[key]), *([FadeOut(meters[key])] if key in meters else [])]

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
            ("fp", r"f_{P}", P_YELLOW),
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
            ("fp", r"f_{P,\mathrm{nren}}", P_YELLOW), ("unit", tr(rf"\quad\text{{with}}\;Q\;\text{{in}}\;{UNIT}", rf"\quad\text{{mit}}\;Q\;\text{{in}}\;{UNIT}"), P_PRIMARY),
        ])
        say("erneuerbar",
            (1.0, unlane("umwelt")),
            (1.4, [Create(box2), *[stage.transition(f"demand.primary-energy.factor.formula-{k}-to-non-renewable", source=items[k], target=items2[k]) for k in ("qp", "eq", "qe", "dot", "fp")],
                   FadeIn(items2["unit"]), Indicate(VGroup(*reads.values()), color=P_YELLOW, scale_factor=1.06)]))

        note = _math_note(tr(r"f_{P}\;\text{per Annex 4: non-renewable share, dimensionless — a defined calculation value}", r"f_{P}\;\text{nach Anlage 4: nicht erneuerbarer Anteil, dimensionslos — festgelegter Rechenwert}"))
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
    """🏗️ One house on one plant: bars at the three boundaries, the plant between them, and the ``f_P`` gauge.

    Everything sits on the same ground line, so the two panels read as columns that can be compared by eye.
    """
    gas = kind == "gas"
    xs = {"h": px + 2.85, "plant": px + 3.95, "e": px + 5.0, "q": px + 6.1}
    y_b = G3 - 0.3
    hs = _house(px + 1.1, G3, S3)
    inside = _furnish(hs, S3, -0.55, 0.45)
    rad_x = float(inside["radiator"].get_center()[0])
    colour = GAS_AMBER if gas else AMBIENT_GREEN
    head = body_text(tr("Gas boiler", "Gaskessel") if gas else tr("Heat pump", "Wärmepumpe"), font_size=BODY_FONT_SIZE, color=colour)
    head.move_to(np.array([px + 1.1, 1.5, 0.0]))
    fan = None
    if gas:
        plant = _boiler(np.array([xs["plant"], G3, 0.0]), room_metre(hs["w_height"] / 2))
    else:
        plant = fan = _heat_pump(np.array([xs["plant"], G3 + 0.01, 0.0]), room_metre(hs["w_height"] / 2))
    meter_glyph = fitted_staffage("shared-gas-meter-run" if gas else "shared-electricity-meter-run", width=0.28, height=0.32, rate=1.0, loop=True).move_to(np.array([xs["e"], y_b, 0.0]))
    p_up = Line(np.array([xs["q"], y_b, 0.0]), np.array([xs["e"] + 0.14, y_b, 0.0]))
    metre = room_metre(hs["w_height"] / 2)
    connection_in = plant.get_bottom() + RIGHT * 0.08 * metre if gas else np.array([xs["plant"] + 0.2, G3 + 0.04, 0.0])
    connection_out = plant.get_bottom() + LEFT * 0.08 * metre if gas else np.array([xs["plant"] - 0.2, G3 + 0.04, 0.0])
    p_end = smooth_path([(xs["e"] - 0.14, y_b, 0.0), (connection_in[0], y_b, 0.0), tuple(connection_in)])
    p_use = smooth_path([tuple(connection_out), (connection_out[0], y_b, 0.0), (rad_x, y_b, 0.0),
                         (rad_x, G3 + 0.1, 0.0)])
    p_air = Line(np.array([xs["plant"], 0.72, 0.0]), np.array([xs["plant"], G3 + 0.62, 0.0]))
    sun = None if gas else sun_glyph(np.array([xs["plant"], 1.0, 0.0])).scale(0.3)
    trackers = {k: ValueTracker(0.0) for k in ("h", "e", "q")}
    plain = [(Q_NUTZ, NUTZ_BLUE)]
    split = [(Q_POWER_SHARE, P_YELLOW), (Q_AMBIENT_WP, AMBIENT_GREEN)]
    wp_end = [(Q_POWER_SHARE, P_YELLOW), (Q_END_WP - Q_POWER_SHARE, P_ORANGE)]
    stacks = {
        "e": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE)] if gas else wp_end,
        "q": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE), (Q_CHAIN_GAS, PRIM_VIOLET)] if gas
        else [*wp_end, (Q_CHAIN_WP, PRIM_VIOLET)],
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
            "num": math_readout(lambda t=trackers[key]: label_number(t.get_value()),
                                lambda k=key: np.array([xs[k], top(k) + 0.2, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
            "sym": math_readout(lambda s=symbol: s, lambda k=key: np.array([xs[k], top(k) + 0.58, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
        }

    kinds = {key: body_text(word, font_size=LABEL_FONT_SIZE - 5, color=stage[key][1]).move_to(np.array([xs[key] + dx, G3 - 0.67, 0.0]))
             for key, word, dx in (("h", tr("Useful energy", "Nutzenergie"), 0.0), ("e", tr("Final energy", "Endenergie"), -0.18),
                                   ("q", tr("Primary energy", "Primärenergie"), 0.1))}
    gauge = meter("f", length=1.6, thickness=0.26, color=P_YELLOW, vertical=False)
    gauge["group"].remove(gauge["label"])
    gauge["group"].move_to(np.array([px + 5.65, 1.85, 0.0]))
    share = ValueTracker(0.0)
    bind_meter(gauge, share)
    tick = Line(gauge["track"].get_top() + DOWN * 0.04, gauge["track"].get_bottom() + UP * 0.04,
                color=P_WHITE, stroke_width=1.6, stroke_opacity=0.7)
    gauge_read = math_readout(lambda: rf"f_{{P}}={label_number(2.0 * share.get_value(), 1)}",
                              np.array([px + 4.75, 1.78, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="right")
    return {
        "kind": kind, "xs": xs, "y_b": y_b, "hs": hs, "inside": inside, "ground": _ground(px, px + 6.6, G3),
        "head": head, "plant": plant, "fan": fan, "meter": meter_glyph, "sun": sun,
        "paths": {"up": p_up, "end": p_end, "use": p_use, "air": p_air}, "pipes": {"up": _pipes(p_up), "end": _pipes(p_end), "use": _pipes(p_use)},
        "trackers": trackers, "bars": bars, "reads": reads, "split": split, "plain": plain, "stacks": stacks,
        "gauge": gauge, "tick": tick, "share": share, "gauge_read": gauge_read, "colour": colour,
        "kinds": kinds, "static": {},
    }


class Beat3_Rechenbeispiel(Scene):
    NARRATION = [
        ("intro",
         "Now the whole chain on one building.",
         "Jetzt die ganze Kette an einem Gebäude."),
        ("nutz",
         rf"The same ${Q_NUTZ}\,\mathrm{{kWh/(m^2\,a)}}$ of useful energy $Q_{{h}}$ as in the last chapter.",
         rf"Dieselben ${Q_NUTZ}\,\mathrm{{kWh/(m^2\,a)}}$ Nutzenergie $Q_{{h}}$ wie im letzten Kapitel."),
        ("anlage",
         r"The plant turns that useful energy into final energy $Q_{E}$.",
         r"Die Anlage macht aus dieser Nutzenergie Endenergie $Q_{E}$."),
        ("end_gas",
         rf"The gas boiler draws ${Q_END_GAS}\,\mathrm{{kWh/(m^2\,a)}}$.",
         rf"Der Gaskessel nimmt ${Q_END_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ auf."),
        ("end_wp",
         rf"The heat pump needs only ${Q_END_WP}\,\mathrm{{kWh/(m^2\,a)}}$ of electricity, its final energy.",
         rf"Die Wärmepumpe braucht nur ${Q_END_WP}\,\mathrm{{kWh/(m^2\,a)}}$ Strom als Endenergie."),
        ("end_umwelt",
         rf"Of the useful energy, ${Q_AMBIENT_WP}\,\mathrm{{kWh/(m^2\,a)}}$ come free from the ambient air, and only ${Q_POWER_SHARE}\,\mathrm{{kWh/(m^2\,a)}}$ are driven by electricity.",
         rf"Von der Nutzenergie liefert die Umweltwärme der Luft ${Q_AMBIENT_WP}\,\mathrm{{kWh/(m^2\,a)}}$ kostenlos, nur ${Q_POWER_SHARE}\,\mathrm{{kWh/(m^2\,a)}}$ gehen auf den Strom zurück."),
        ("end_rest",
         r"The remaining $5\,\mathrm{kWh/(m^2\,a)}$ of electricity cover the losses of emission, distribution and storage.",
         r"Die übrigen $5\,\mathrm{kWh/(m^2\,a)}$ Strom decken die Verluste von Übergabe, Verteilung und Speicherung."),
        ("prim",
         r"Now apply the primary energy factors $f_{P}$.",
         r"Jetzt kommen die Primärenergiefaktoren $f_{P}$."),
        ("prim_gas",
         r"Gas is multiplied by $f_{P} = 1.1$.",
         r"Gas wird mit $f_{P} = 1{,}1$ multipliziert."),
        ("prim_wp",
         r"Electricity is multiplied by $f_{P} = 1.8$.",
         r"Strom wird mit $f_{P} = 1{,}8$ multipliziert."),
        ("ergebnis",
         rf"${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ against ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$ of primary energy $Q_{{P}}$.",
         rf"${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ gegen ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$ Primärenergie $Q_{{P}}$."),
        ("hoeher",
         "The electricity factor is higher, yet the heat pump ends up far lower.",
         "Der Stromfaktor ist höher — und die Wärmepumpe liegt trotzdem weit darunter."),
        ("warum",
         "Because the factor is applied to a final energy that was already more than three quarters smaller.",
         "Weil der Faktor auf eine Endenergie trifft, die schon gut drei Viertel kleiner war."),
        ("anlagenzahl",
         rf"Primary over useful energy is the primary-energy expenditure figure $e_{{P}} = e_{{E}}\cdot f_{{P}} = Q_{{P}}/Q_{{h}}$: ${E_P_GAS:.2f}$ against ${E_P_WP:.2f}$.",
         rf"Primärenergie durch Nutzenergie ist die primärenergetische Aufwandszahl $e_{{P}} = e_{{E}}\cdot f_{{P}} = Q_{{P}}/Q_{{h}}$: ${_de(E_P_GAS)}$ gegen ${_de(E_P_WP)}$."),
        ("effizienz",
         "Efficiency beats the factor.",
         "Effizienz schlägt den Faktor."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("One building, two systems", "Ein Gebäude, zwei Anlagen"))
        din = reference_chip(_references.cite(("gmodg", "p20-22"), ("din-ts-18599-1", "primaerenergie")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        schematic_note = note_line(tr("Simplified worked example — not a full DIN/TS 18599:2025-10 / GModG calculation", "Vereinfachtes Rechenbeispiel — keine vollständige Berechnung nach DIN/TS 18599:2025-10 / GModG"), color=P_PRIMARY)
        self.play(FadeIn(caption), FadeIn(schematic_note), run_time=0.3)

        gas, wp = _plant_panel(-6.85, "gas"), _plant_panel(0.1, "wp")
        panels = (gas, wp)

        legend_unit = math_label(rf"Q\;\text{{in}}\;{UNIT}", size=LABEL_FONT_SIZE, color=P_PRIMARY)
        words = {key: body_text(text, font_size=LABEL_FONT_SIZE - 2, color=colour)
                 for key, text, colour in (("nutz", tr("Useful energy", "Nutzenergie"), NUTZ_BLUE), ("loss", tr("System loss", "Anlagenverlust"), P_ORANGE),
                                           ("power", tr("Electricity", "Strom"), P_YELLOW),
                                           ("env", tr("Ambient heat", "Umweltwärme"), AMBIENT_GREEN), ("chain", tr("Upstream chain", "Vorkette"), PRIM_VIOLET))}
        VGroup(legend_unit, *words.values()).arrange(RIGHT, buff=0.45).move_to(np.array([0.0, 2.3, 0.0]))

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
            num = _freeze(self, p["reads"][key]["num"], label_number(value), (x, top_y + 0.2), color)
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
            streams.append(lambda rt, p=p: setattr(p["fan"], "rate", 1.0) or [])

        say("intro",
            (1.5, [*[Create(p["hs"]["group"]) for p in panels], *[Create(p["ground"]) for p in panels],
                   *[FadeIn(p["inside"]["group"]) for p in panels], *[FadeIn(p["head"]) for p in panels]]),
            first=True)

        say("nutz",
            (1.0, [stage.transition("demand.primary-energy.worked-example.note-to-unit-legend", source=schematic_note, target=legend_unit), FadeIn(words["nutz"]),
                   *[FadeIn(p["kinds"]["h"]) for p in panels]]),
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
                                anims=[FadeIn(words["loss"]), FadeIn(gas["kinds"]["e"]), gas["trackers"]["e"].animate.set_value(Q_END_GAS)])))
        freeze(gas, "e", base_src=r"Q_{E}", color=END_RED)

        end_streams(wp)
        say("end_wp",
            (1.6, lambda: shown(wp["bars"]["e"], wp["reads"]["e"]["num"], wp["reads"]["e"]["sym"],
                                anims=[FadeIn(wp["kinds"]["e"]), wp["trackers"]["e"].animate.set_value(Q_END_WP)])))
        freeze(wp, "e", base_src=r"Q_{E}", color=END_RED)

        ambient_streams(wp)
        split_bar = _bar_rects(Q_NUTZ, wp["split"], wp["xs"]["h"], G3, unit=U3)
        say("end_umwelt",
            (1.2, [FadeIn(wp["sun"]), FadeIn(words["env"]), FadeIn(words["power"])]),
            (1.6, [stage.transition("demand.primary-energy.worked-example.heat-bar-splits-ambient", source=wp["rect_h"], target=split_bar)]))
        wp["rect_h"] = split_bar

        say("end_rest", (1.2, [Indicate(wp["static"]["e"]["num"], color=P_YELLOW, scale_factor=1.15)]))

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
                                           FadeIn(p["pipes"]["up"]), FadeIn(p["kinds"]["q"])])),
                (2.0, [p["share"].animate.set_value(factor / 2.0), p["trackers"]["q"].animate.set_value(prim)]),
            ]

        say("prim_gas", *factor_step(gas))
        freeze(gas, "q", base_src=r"Q_{P}", color=PRIM_VIOLET)
        gas["static"]["f"] = _freeze(self, gas["gauge_read"], rf"f_{{P}}={label_number(F_P_GAS, 1)}",
                                     (gas["xs"]["e"] - 0.25, 1.78), P_YELLOW, edge="right")

        say("prim_wp", *factor_step(wp))
        freeze(wp, "q", base_src=r"Q_{P}", color=PRIM_VIOLET)
        wp["static"]["f"] = _freeze(self, wp["gauge_read"], rf"f_{{P}}={label_number(F_P_STROM, 1)}",
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

        unit_tail = rf"\,{UNIT}"
        parts = [
            ("g_qe", label_number(Q_END_GAS) + unit_tail, END_RED), ("g_dot", r"\cdot", P_WHITE),
            ("g_fp", label_number(F_P_GAS, 1), P_YELLOW), ("g_eq", _eq(Q_END_GAS * F_P_GAS), P_WHITE),
            ("g_qp", label_number(Q_PRIM_GAS) + unit_tail, PRIM_VIOLET), (None, r"\quad", P_WHITE),
            ("w_qe", label_number(Q_END_WP) + unit_tail, END_RED), ("w_dot", r"\cdot", P_WHITE),
            ("w_fp", label_number(F_P_STROM, 1), P_YELLOW), ("w_eq", _eq(Q_END_WP * F_P_STROM), P_WHITE),
            ("w_qp", label_number(Q_PRIM_WP) + unit_tail, PRIM_VIOLET),
        ]
        row, box, items = math_panel(parts)
        ep_note = _math_note(tr(rf"e_{{P}}=Q_{{P}}/Q_{{h}}:\;{E_P_GAS:.2f}\;\text{{(gas)}},\;{E_P_WP:.2f}\;\text{{(heat pump)}}\;\cdot\;e_{{P}}\;\text{{and}}\;f_{{P}}\;\text{{are dimensionless}}", rf"e_{{P}}=Q_{{P}}/Q_{{h}}:\;{_de(E_P_GAS)}\;\text{{(Gas)}},\;{_de(E_P_WP)}\;\text{{(Wärmepumpe)}}\;\cdot\;e_{{P}}\;\text{{und}}\;f_{{P}}\;\text{{sind dimensionslos}}"))
        say("anlagenzahl", (0.6, [FadeOut(legend_unit), *[FadeOut(w) for w in words.values()], *[FadeOut(m) for p in panels for m in p["kinds"].values()]]), (1.2, [FadeIn(ep_note)]))
        copies = []
        for p, tag, name in ((gas, "g", "gas"), (wp, "w", "heat-pump")):
            copies += [
                stage.transition(f"demand.primary-energy.worked-example.{name}-final-energy-into-formula", source=p["static"]["e"]["num"], target=items[f"{tag}_qe"]),
                stage.transition(f"demand.primary-energy.worked-example.{name}-factor-into-formula", source=p["static"]["f"], target=items[f"{tag}_fp"]),
                stage.transition(f"demand.primary-energy.worked-example.{name}-primary-energy-into-formula", source=p["static"]["q"]["num"], target=items[f"{tag}_qp"]),
                FadeIn(items[f"{tag}_dot"]), FadeIn(items[f"{tag}_eq"]),
            ]
        say("effizienz",
            (1.8, [Create(box), *copies]),
            (1.0, [Indicate(VGroup(items["g_qp"], items["w_qp"]), color=P_YELLOW, scale_factor=1.04)]))

        moved = {id(m) for p in panels for m in (p["static"]["e"]["num"], p["static"]["f"], p["static"]["q"]["num"])}
        everything = [
            caption, box, row, ghost_q, ghost_e, ep_note,
            *[m for p in panels for m in (
                p["ground"], p["meter"], p["head"], p["pipes"]["up"],
                p["rect_h"], p["bars"]["e"], p["bars"]["q"], p["gauge"]["track"], p["gauge"]["fill"], p["tick"],
                *[m for tag in p["static"].values() for m in (tag.values() if isinstance(tag, dict) else [tag]) if id(m) not in moved],
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
         rf"Its calculated primary energy $Q_{{P}}$ sets the benchmark: ${Q_REF}\,\mathrm{{kWh/(m^2\,a)}}$ in this schematic example.",
         rf"Dessen Primärenergiebedarf $Q_{{P}}$ setzt den Maßstab: im schematischen Beispiel ${Q_REF}\,\mathrm{{kWh/(m^2\,a)}}$."),
        ("anteil",
         rf"A new residential building may reach at most ${REF_SHARE:.2f}$ of it, that is ${Q_LIMIT}\,\mathrm{{kWh/(m^2\,a)}}$.",
         rf"Ein neues Wohngebäude darf höchstens das ${_de(REF_SHARE)}$-Fache davon erreichen, also ${Q_LIMIT}\,\mathrm{{kWh/(m^2\,a)}}$."),
        ("waermepumpe",
         rf"In this example the heat pump, ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$, clears the line.",
         rf"Im Beispiel unterschreitet die Wärmepumpe mit ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$ die Linie."),
        ("gaskessel",
         rf"The gas boiler, ${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$, misses it by more than a factor of two.",
         rf"Der Gaskessel verfehlt sie mit ${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ um mehr als das Doppelte."),
        ("huelle",
         r"The law also demands thermal protection of the envelope: a limit on the transmission heat loss per envelope area.",
         r"Daneben verlangt das Gesetz baulichen Wärmeschutz: eine Grenze für den Transmissionswärmeverlust je Hüllfläche."),
        ("erneuerbar",
         "The 65 percent renewable rule for new heating systems, formerly paragraph 71, was dropped with the 2026 reform.",
         "Die frühere 65-Prozent-Regel für neue Heizungen, Paragraf 71, ist mit der Reform 2026 entfallen."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The limit: the reference building", "Der Grenzwert: das Referenzgebäude"))
        din = reference_chip(_references.cite(("gmodg", "p71")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        design = _house(-5.25, G4, S4)
        reference = _house(-1.95, G4, S4)
        design_in = _furnish(design, S4, -0.55, 0.45)
        reference_in = _furnish(reference, S4, -0.55, 0.45)
        ground = _ground(-6.9, 6.7, G4)

        def dims(hs):
            tl, bl, top_r = hs["top_left"], hs["bottom_left"], hs["top_right"]
            vertical = dim_arrow(np.array([bl[0] - 0.4, bl[1], 0.0]), np.array([bl[0] - 0.4, tl[1], 0.0]), P_YELLOW)
            horizontal = dim_arrow(np.array([0.0, 0.0, 0.0]), np.array([0.0, top_r[0] - tl[0], 0.0]), P_YELLOW)
            horizontal.rotate(-PI / 2).move_to(np.array([(tl[0] + top_r[0]) / 2, hs["roof_peak"][1] + 0.28, 0.0]))
            return VGroup(vertical, horizontal)

        dims_d, dims_r = dims(design), dims(reference)
        envelope_d, envelope_r = _envelope(design, S4), _envelope(reference, S4)
        tag_d = body_text(tr("Your design", "Ihr Entwurf"), font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([-5.25, -0.95, 0.0]))
        tag_r = body_text(tr("Reference building", "Referenzgebäude"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([-1.95, -0.95, 0.0]))
        spec = chip(tr("Annex 1", "Anlage 1"), P_PRIMARY, font_size=LABEL_FONT_SIZE).move_to(np.array([-1.95, G4 + 1.1, 0.0]))

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
            lambda: rf"{label_number(t_share.get_value(), 2)}\cdot {Q_REF}\,{UNIT}",
            lambda: np.array([x_ref + ox.get_value(), G4 + Q_REF * U4 + 0.28, 0.0]),
            size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        y_limit = G4 + Q_LIMIT * U4
        limit_card = _card(tr("Limit in the example", "Grenzwert im Beispiel"), math_label(rf"{Q_LIMIT}\,{UNIT}", size=BODY_FONT_SIZE, color=P_RED),
                           P_RED)
        limit_card.move_to(np.array([4.9, 1.15, 0.0]))
        limit_leader = DashedLine(limit_card.get_bottom(), np.array([4.9, y_limit, 0.0]), color=P_RED,
                                  stroke_width=1.6, stroke_opacity=0.6)
        share_note = note_line(
            tr(f"New build (residential): max. {int(REF_SHARE * 100)} % of the reference value — shown schematically", f"Neubau (Wohngebäude): max. {int(REF_SHARE * 100)} % des Referenzwerts — hier schematisch"), color=P_RED)
        bar_gas = _bar_mob(t_gas, [(Q_LIMIT, GAS_AMBER), (Q_PRIM_GAS - Q_LIMIT, P_RED)], x_gas, G4, width=1.0, unit=U4)
        bar_wp = _bar_mob(t_wp, [(Q_PRIM_WP, AMBIENT_GREEN)], x_wp, G4, width=1.0, unit=U4)

        def caption_under(name, colour, tracker, x):
            label = body_text(name, font_size=LABEL_FONT_SIZE, color=colour).move_to(np.array([x - 0.3, -0.95, 0.0]))
            number = _count(tracker, lambda: np.array([label.get_center()[0], -1.36, 0.0]), colour, edge="center",
                            suffix=rf"\,{UNIT}")
            return label, number

        name_ref, num_ref = caption_under(tr("Reference", "Referenz"), PRIM_VIOLET, t_ref, x_ref + SHIFT)
        name_gas, num_gas = caption_under(tr("Gas boiler", "Gaskessel"), GAS_AMBER, t_gas, x_gas)
        name_wp, num_wp = caption_under(tr("Heat pump", "Wärmepumpe"), AMBIENT_GREEN, t_wp, x_wp)
        double_line = DashedLine(np.array([x_gas - 0.7, G4 + 2 * Q_LIMIT * U4, 0.0]),
                                 np.array([x_gas + 0.75, G4 + 2 * Q_LIMIT * U4, 0.0]), color=P_RED, stroke_width=1.8,
                                 stroke_opacity=0.55)
        double_tag = math_label(r"2\times", np.array([x_gas + 0.88, G4 + 2 * Q_LIMIT * U4 - 0.06, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        path_ref = smooth_path([(reference["bottom_right"][0] + 0.1, G4 - 0.2, 0.0),
                                (x_ref + SHIFT - 0.5, G4 - 0.2, 0.0)])
        pipe_ref = _pipes(path_ref)
        verdict = note_line(tr("in the example only the heat pump falls below the limit", "im Beispiel unterschreitet nur die Wärmepumpe den Grenzwert"), color=AMBIENT_GREEN)
        axis_label = math_label(tr(r"\text{Primary energy}\;Q_{P}\;\text{in}\;", r"\text{Primärenergie}\;Q_{P}\;\text{in}\;") + UNIT,
                                np.array([-6.7, 1.85, 0.0]), size=LABEL_FONT_SIZE, color=PRIM_VIOLET, edge="left")

        share_gauge = meter("e", length=1.0, thickness=0.2, color=AMBIENT_GREEN, vertical=False)
        share_gauge["group"].remove(share_gauge["label"])
        share_gauge["group"].move_to(np.array([3.0, 0.5, 0.0]))
        t_ee = ValueTracker(0.0)
        bind_meter(share_gauge, t_ee)
        ee_name = body_text(tr("renewable (until 2026)", "erneuerbar (bis 2026)"), font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(
            np.array([3.0, 0.8, 0.0]))
        ee_read = math_readout(lambda: rf"{label_number(100.0 * t_ee.get_value())}\,\%", np.array([3.65, 0.44, 0.0]),
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
            (1.8, [stage.transition("demand.primary-energy.reference.design-house-copies-to-reference", source=design["group"], target=reference["group"]),
                   FadeIn(reference_in["group"], shift=reference_in["group"].get_center() - design_in["group"].get_center()),
                   stage.transition("demand.primary-energy.reference.dimension-copies-to-reference", source=dims_d, target=dims_r), FadeIn(tag_r)]))

        say("anlage1", (1.4, [Create(envelope_r)]), (1.0, [FadeIn(spec)]))

        warm[:] = [reference_in["warm"]]
        active.append(lambda rt: [_flow(path_ref, PRIM_VIOLET, rt, v=1.0, per_unit=3.0)])
        say("grenze",
            (1.2, [FadeOut(m) for m in (design["group"], design_in["group"], dims_d, dims_r, tag_d, spec, envelope_r)]),
            (1.8, lambda: shown(bar_ref, num_ref, anims=[FadeIn(pipe_ref), FadeIn(name_ref), FadeIn(axis_label),
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
                 name_wp, num_ref, num_gas, num_wp, double_line, double_tag, verdict, axis_label)
        house_d = VGroup(design["group"], design_in["group"], envelope_d)
        house_d.shift(RIGHT * 5.25)
        row, box, items = math_panel([
            ("ht", r"H_{T}", P_CYAN),
            ("ht_ref", tr(r"\text{per envelope area in}\;\frac{\mathrm{W}}{\mathrm{m^{2}\,K}}\;\text{at most that of the reference building (§ 16)}", r"\text{je Hüllfläche in}\;\frac{\mathrm{W}}{\mathrm{m^{2}\,K}}\;\text{höchstens wie beim Referenzgebäude (§ 16)}"), P_PRIMARY)])
        ring_ht = highlight_param(items, "ht", color=P_CYAN)
        say("huelle",
            (1.2, [FadeOut(m) for m in chart]),
            (1.4, [Create(design["group"]), FadeIn(design_in["group"])]),
            (1.2, [Create(envelope_d)]),
            (1.4, [Create(box), FadeIn(items["ht_ref"]), stage.transition("demand.primary-energy.reference.envelope-into-ht", source=envelope_d, target=items["ht"]),
                   Indicate(envelope_d, color=P_CYAN, scale_factor=1.02)]),
            (0.5, [Create(ring_ht)]))

        row2, box2, items2 = math_panel([
            ("ee", r"65\,\%", AMBIENT_GREEN), ("ee_text", tr(r"\text{renewable for new heating systems (§ 71, until 2026) — repealed on 29 July 2026}", r"\text{erneuerbar für neue Heizungen (§ 71, bis 2026) — entfallen seit 29.07.2026}"), P_PRIMARY),
        ])
        strike = Line(items2["ee"].get_corner(DL), items2["ee"].get_corner(UR), color=P_RED, stroke_width=4)
        frozen = {}

        def freeze_ee():
            frozen["ee"] = _freeze(self, ee_read, rf"{label_number(100.0 * EE_SHARE)}\,\%", (3.65, 0.44), AMBIENT_GREEN,
                                   edge="left")
            return [Create(box2), stage.transition("demand.primary-energy.reference.share-reading-into-formula", source=frozen["ee"], target=items2["ee"]), FadeIn(items2["ee_text"])]

        say("erneuerbar",
            (1.0, [FadeOut(box), FadeOut(row), FadeOut(ring_ht)]),
            (1.6, lambda: shown(ee_read, anims=[FadeIn(share_gauge["track"]), FadeIn(share_gauge["fill"]),
                                                FadeIn(ee_name), t_ee.animate.set_value(EE_SHARE)])),
            (1.2, freeze_ee),
            (0.8, [Create(strike)]))

        self.play(
            FadeOut(caption), FadeOut(box2), FadeOut(row2), FadeOut(strike),
            *[FadeOut(m) for m in (design["group"], design_in["group"], ground, envelope_d,
                                   share_gauge["track"], share_gauge["fill"], ee_name, frozen["ee"])],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion
