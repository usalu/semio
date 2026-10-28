

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
body_text = _manim_module.body_text
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
window_chip = _manim_module.window_chip
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
de_num = _manim_module.de_num
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing
room_section = _manim_module.room_section
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
meter = _manim_module.meter
set_meter = _manim_module.set_meter

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — self.add()'ed on every beat.
TITLE_DE = "Systemauslegung"

# Mid-screen anchor for rooms / ducts (clear of title + formula/caption).
CONTENT_CENTER = UP * 0.1


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``3_heating/2_conduction/scene_2.py``): a dim ``P_PRIMARY`` footnote
    that never competes with the diagram. The formula panel sits on the
    bottom edge, so this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Running example
# 🧮 The hygiene airflow q_v,R and the duct velocity v_m belong to the Lüftung video (chapter 2 ``2_ventilation/2_luftmenge``,
# chapter 5 ``2_ventilation/5_mechanische_lueftung``). The solar load is part 4's ``A · F_F · F_V · g_tot · I_S,max`` with
# the displayed factors of ``4_solar_radiation`` Beat 5 on the 30 m² of glazing of the Heating example house
# (Heating Modul 2); ρ_a, c_p and Δθ are Beat 2's own values.
Q_V_R_HYGIENE = 220.0 / 3600.0
V_M = 2.5
RHO_AIR, CP_AIR, D_THETA = 1.2, 1.0, 7.0
A_GLAZING, F_F, F_V, G_TOT, I_S_MAX = 30.0, 0.49, 0.15, 0.60, 800.0
W_SOLAR = round(A_GLAZING * F_F * F_V * G_TOT * I_S_MAX, -1)
Q_V_R_SOLAR = W_SOLAR / 1000.0 / (RHO_AIR * CP_AIR * D_THETA)
FACTOR = Q_V_R_SOLAR / Q_V_R_HYGIENE
A_HYGIENE, A_SOLAR = Q_V_R_HYGIENE / V_M, Q_V_R_SOLAR / V_M
R_HYGIENE, R_SOLAR = (float(np.sqrt(a / np.pi)) for a in (A_HYGIENE, A_SOLAR))

AIR = semio_color("secondary")
#endregion


#region Shared visual motifs

def _build_room(center=CONTENT_CENTER, width=7.4, height=3.55):
    """🏠 Physical Fundamentals room section — slabs, glazed left wall, solid right wall."""
    r = room_section(np.array(center, dtype=float), w=width, h=height)
    r["room"] = r["group"]
    return r


def _vent_unit(pos, color=P_PRIMARY, width=1.55, height=0.3):
    """🔧 Ceiling grille seen in section — a slotted strip hanging from the slab."""
    body = Rectangle(width=width, height=height, color=color, stroke_width=1.8,
                     fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(pos)
    slots = VGroup(*[
        Line(body.get_bottom() + RIGHT * dx + UP * 0.05, body.get_top() + RIGHT * dx + DOWN * 0.05,
             color=color, stroke_width=1.3)
        for dx in np.linspace(-width / 2 + 0.16, width / 2 - 0.16, 6)
    ])
    return VGroup(body, slots)


def _supply_paths(supply_bottom, room_c, n=16, seed=3, half_w=2.2, half_h=0.9, margin=0.22):
    """➡️ Cool air enters from the supply grille and stays inside the room bounds."""
    rng = np.random.default_rng(seed)
    paths = VGroup()
    x_lo, x_hi = -half_w + margin, half_w - margin
    y_lo, y_hi = -half_h + margin, half_h - margin

    def _inside(pt):
        return np.array([
            float(np.clip(pt[0], room_c[0] + x_lo, room_c[0] + x_hi)),
            float(np.clip(pt[1], room_c[1] + y_lo, room_c[1] + y_hi)),
            0.0,
        ])

    for _ in range(n):
        start = _inside(np.array(supply_bottom, dtype=float) + np.array([
            float(rng.uniform(-0.32, 0.32)), -0.06, 0.0,
        ]))
        mid = _inside(room_c + np.array([
            float(rng.uniform(x_lo * 0.65, x_hi * 0.35)),
            float(rng.uniform(y_lo * 0.15, y_hi * 0.55)),
            0.0,
        ]))
        end = _inside(room_c + np.array([
            float(rng.uniform(x_lo * 0.8, x_hi * 0.8)),
            float(rng.uniform(y_lo * 0.85, y_hi * 0.05)),
            0.0,
        ]))
        path = smooth_path([start, mid, end])
        for i in range(len(path.points)):
            path.points[i, 0] = np.clip(path.points[i, 0], room_c[0] + x_lo, room_c[0] + x_hi)
            path.points[i, 1] = np.clip(path.points[i, 1], room_c[1] + y_lo, room_c[1] + y_hi)
        paths.add(path)
    return paths


def _back_chip(prefix: str, src: str | None = None, color: str = AIR):
    """🔗 Back-link chip — names the chapter or part and the quantity a value is taken from (Lüftung blue by default)."""
    parts = [body_text(prefix, font_size=LABEL_FONT_SIZE + 1, color=color)]
    if src:
        parts.append(math_label(src, size=LABEL_FONT_SIZE + 1, color=color))
    content = VGroup(*parts).arrange(RIGHT, buff=_theme_module.UI_SPACING)
    return window_chip(content, color=color)


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


def _ramp(tracker, to: float, *, rate_func=smooth):
    """⏩ Move a tracker to ``to`` — one animation, so it can run beside other animations."""
    start = tracker.get_value()
    return UpdateFromAlphaFunc(tracker, lambda m, a: m.set_value(start + (to - start) * a), rate_func=rate_func)


def _hide(scene, *mobs, run_time: float = 0.6):
    """🫥 Fade out what has been explained; live readouts stop first."""
    for mob in mobs:
        for part in mob.get_family():
            part.clear_updaters()
    scene.play(*[FadeOut(m) for m in mobs], run_time=run_time)

#endregion


#region Beat2 – Thermodynamic Volume-Flow Equation
class Beat2_VolumeFlowEquation(Scene):
    NARRATION = [
        ("intro",
         "The balanced supply and exhaust system from chapter 5 of the ventilation video removes the load with cooled supply air.",
         "Die Zu-/Abluftanlage aus Lüftung Kapitel 5 führt die Last mit gekühlter Zuluft ab."),
        ("formula",
         "Q-dot V = air density × specific heat capacity × Δθ (room − supply) × required airflow q v R.",
         "Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R."),
        ("air",
         "Density and heat capacity are the air properties you know from the ventilation video.",
         "Rho a und c p a sind die Stoffwerte der Luft, bekannt aus dem Lüftungsvideo."),
        ("dth",
         "Delta theta is the designed temperature lift between room air and cooled supply air — here 25 minus 18 equals 7 kelvin.",
         "Delta Theta ist der Temperaturhub zwischen Raumluft und gekühlter Zuluft — hier fünfundzwanzig minus achtzehn, also sieben Kelvin."),
        ("qvr",
         "And q v R is the free design variable: how much air we must move every second to carry the heat away.",
         "q v R ist die Entwurfsgröße — wie viel Luft pro Sekunde bewegt werden muss."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Konvektive Kühlleistung der Zuluft")
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        built = _build_room(center=LEFT * 2.6 + UP * 0.5, width=4.6, height=1.35)
        room, room_c = built["room"], built["center"]
        supply = _vent_unit(np.array([room_c[0] - 1.2, built["y_c"] - 0.13, 0.0]), color=P_CYAN, width=1.1, height=0.26)

        t_supply = VGroup(
            math_label(r"\theta_{Zu}", size=LABEL_FONT_SIZE, color=P_CYAN),
            math_label(r"18\,\mathrm{°C}", size=BODY_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.06)
        t_supply = VGroup(
            SurroundingRectangle(t_supply, color=P_CYAN, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER),
            t_supply,
        ).next_to(room, DOWN, buff=0.15).shift(LEFT * 1.0)

        t_room = VGroup(
            math_label(r"\theta_{Raum}", size=LABEL_FONT_SIZE, color=P_ORANGE),
            math_label(r"25\,\mathrm{°C}", size=BODY_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06)
        t_room = VGroup(
            SurroundingRectangle(t_room, color=P_ORANGE, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER),
            t_room,
        ).next_to(room, DOWN, buff=0.15).shift(RIGHT * 1.0)

        delta_line = math_label(r"25-18=7\,\mathrm{K}", size=BODY_FONT_SIZE, color=P_BLUE)
        delta_line.set_opacity(0)
        delta_body = VGroup(
            Text("Temperaturhub", font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
            delta_line,
        ).arrange(DOWN, buff=0.06)
        delta_card = VGroup(
            SurroundingRectangle(delta_body, color=P_BLUE, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER),
            delta_body,
        ).next_to(room, UP, buff=0.20).set_x(room_c[0])
        delta_tr = ValueTracker(0.0)
        delta_read = math_readout(
            lambda: rf"25-18={de_num(delta_tr.get_value())}\,\mathrm{{K}}",
            lambda: delta_line.get_center() + DOWN * 0.07,
            size=BODY_FONT_SIZE, color=P_BLUE, edge="center",
        )

        temp_arrow = Arrow(
            t_supply.get_right() + RIGHT * 0.08, t_room.get_left() + LEFT * 0.08,
            buff=0.04, stroke_width=4, max_tip_length_to_length_ratio=0.12, color=P_BLUE,
        )

        cool_paths = _supply_paths(
            supply.get_bottom(),
            room_c,
            n=14,
            seed=21,
            half_w=built["w"] / 2,
            half_h=built["h"] / 2,
            margin=0.25,
        )

        def cool_air(rt):
            return [flow_animation([(cool_paths, P_CYAN, P_BLUE)], waves=3, radius=0.055, cycles=rt / 2.6)]

        eq, eq_box, items = math_panel([
            ("qv", r"\dot{Q}_{V}", P_CYAN), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("cp", r"c_{p,a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dth", r"Δθ", P_BLUE), (None, r"\cdot", P_WHITE),
            ("qvr", r"q_{v,R}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ])

        card_bodies = [
            VGroup(
                Text("Luftdichte", font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                math_label(r"\rho_{a} = 1{,}2\,\mathrm{kg/m^{3}}", size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("spez. Wärmekapazität", font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                math_label(r"c_{p,a} = 1{,}0\,\mathrm{kJ/(kg\,K)}", size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Temperaturdifferenz", font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                math_label(r"Δθ = 7\,\mathrm{K}", size=BODY_FONT_SIZE, color=P_BLUE),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Volumenstrom", font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                math_label(r"q_{v,R}\;[\mathrm{m^{3}/s}]", size=BODY_FONT_SIZE, color=P_YELLOW),
            ).arrange(DOWN, buff=0.06),
        ]
        # Same-size frames so the 2×2 grid lines up. Tight padding keeps the grid
        # narrow enough to clear the room on the left without shrinking the text.
        card_w = max(b.width for b in card_bodies) + 0.32
        card_h = max(b.height for b in card_bodies) + 0.28
        cards = VGroup()
        for body in card_bodies:
            frame = RoundedRectangle(
                width=card_w, height=card_h, corner_radius=0,
                color=P_PRIMARY, stroke_width=_theme_module.CHROME_BORDER,
            )
            body.move_to(frame.get_center())
            cards.add(VGroup(frame, body))
        cards.arrange_in_grid(rows=2, cols=2, buff=(0.20, 0.20))
        # Right of the room, vertically centred on it (room right edge ≈ -0.2).
        cards.move_to(np.array([3.4, room_c[1], 0.0]))

        system_chip = _back_chip("← Lüftung · Kapitel 5: Zu-/Abluftsystem").move_to(np.array([3.4, 2.0, 0.0]))

        self.play(Create(room), FadeIn(supply), FadeIn(system_chip), run_time=1.5)
        self.play(FadeIn(t_supply), FadeIn(t_room), *cool_air(1.0), run_time=1.0)
        self.add(delta_read)
        self.play(
            FadeIn(temp_arrow), FadeIn(delta_card), delta_tr.animate.set_value(7),
            *cool_air(1.6), run_time=1.6,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.0 + 1.6, during=cool_air)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(eq_box), *cool_air(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", during=cool_air)

        for key, params, shown, color in (
            ("air", ("rho", "cp"), cards[:2], P_GREEN), ("dth", ("dth",), cards[2:3], P_BLUE),
            ("qvr", ("qvr",), cards[3:], P_YELLOW),
        ):
            rings = [highlight_param(items, k, color=color) for k in params]
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(*[Create(r) for r in rings], *[FadeIn(c) for c in shown], *cool_air(0.7), run_time=0.7)
            hold_for(self, self.NARRATION, key, during=cool_air)
            self.play(*[FadeOut(r) for r in rings], *cool_air(0.25), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.4)
        self.wait(0.5)
#endregion


#region Beat3 – Isolate Required Airflow q_v,R
class Beat3_IsolateAirflow(Scene):
    NARRATION = [
        ("intro",
         "We size the supply airflow so its cooling capacity matches the solar cooling load through the glazing from the last part.",
         "Wir legen den Zuluftstrom so aus, dass seine Kühlleistung der solaren Kühllast durch die Verglasung entspricht."),
        ("substitute",
         "Q-dot V = Q-dot S,tr: replace the left side, then isolate q v R. Divide the load by density × heat capacity × Δθ.",
         "Weil Q Punkt V gleich Q Punkt S t r ist, stellen wir um: q v R nach links, die Last geteilt durch Rho a mal c p a mal Delta Theta."),
        ("qvr",
         "The unknown on the left is required airflow q v R in m³/s, previously on the first equation’s right side.",
         "Links steht die gesuchte Größe q v R — dieselbe wie rechts in der ersten Formel."),
        ("qstr",
         "In the numerator sits Q-dot S,tr — it replaces Q-dot V because equilibrium made them equal.",
         "Im Zähler ersetzt Q Punkt S t r das Q Punkt V — im Gleichgewicht sind sie gleich."),
        ("rho",
         "The denominator keeps the air properties from the first formula — density and specific heat capacity.",
         "Im Nenner bleiben die Luftkennwerte der ersten Formel: Dichte und Wärmekapazität."),
        ("dth",
         "And the same temperature lift, seven Kelvin, closes the denominator.",
         "Und derselbe Temperaturhub von sieben Kelvin schließt den Nenner ab."),
        ("result",
         "That flow rate is the sizing target for the ventilation system.",
         "Dieser Volumenstrom ist der Auslegungswert der Lüftung."),
        ("load",
         "Part 4: 30 m² of glazing × 0.49 × 0.15 × 0.60 × 800 W/m² ≈ 1060 W.",
         "Aus Teil 4 kennen wir die Last: 30 m² Verglasung mal 0,49 mal 0,15\nmal 0,60 mal 800 W/m² ergeben rund 1060 W."),
        ("compare",
         "That needs 0.126 cubic metres per second, about twice the hygiene airflow of 0.061 cubic metres per second from the ventilation video.",
         "Nötig sind 0,126 m³/s: gut doppelt so viel wie 0,061 m³/s Hygieneluftstrom aus dem Lüftungsvideo."),
        ("duct",
         "The duct follows exactly as in chapter 5: A equals q v R over v m, 0.0505 square metres instead of 0.0244 — the same method, only bigger.",
         "Der Kanal folgt wie in Kapitel 5: A gleich q v R durch v m, also 0,0505 m²\nstatt 0,0244 m² — dieselbe Methode, nur größer."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Gleichgewicht: Kühlleistung = Kühllast")
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        beam = Line(LEFT * 2.8, RIGHT * 2.8, color=P_WHITE, stroke_width=4).move_to(UP * 1.15)
        pivot = Triangle(color=P_PRIMARY, fill_opacity=1).scale(0.18).rotate(PI).next_to(beam, DOWN, buff=0)
        left_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0,
            color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN, fill_opacity=0.12,
        ).next_to(beam.get_left(), DOWN, buff=0.35).shift(RIGHT * 0.9)
        right_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0,
            color=P_YELLOW, stroke_width=2.5, fill_color=P_YELLOW, fill_opacity=0.12,
        ).next_to(beam.get_right(), DOWN, buff=0.35).shift(LEFT * 0.9)
        left_txt = VGroup(
            math_label(r"\dot{Q}_{V}", size=BODY_FONT_SIZE, color=P_CYAN),
            Text("Kühlleistung", font_size=LABEL_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.08).move_to(left_pan)
        right_txt = VGroup(
            math_label(r"\dot{Q}_{S,tr}", size=BODY_FONT_SIZE, color=P_YELLOW),
            Text("Kühllast", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
        ).arrange(DOWN, buff=0.08).move_to(right_pan)
        eq_mark = Text("=", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to(beam.get_center() + DOWN * 0.55)
        balance = VGroup(beam, pivot, left_pan, right_pan, left_txt, right_txt, eq_mark)

        start_eq, start_box, start_items = math_panel([
            ("qv", r"\dot{Q}_{V}", P_CYAN), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("cp", r"c_{p,a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dth", r"Δθ", P_BLUE), (None, r"\cdot", P_WHITE),
            ("qvr", r"q_{v,R}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ], color=P_CYAN)
        start_panel = VGroup(start_box, start_eq)

        final_parts = [
            ("qvr", r"q_{v,R}", P_CYAN), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_YELLOW}}}{{\dot{{Q}}_{{S,tr}}}}}}"
             rf"{{\textcolor{{{P_GREEN}}}{{\rho_{{a}} \cdot c_{{p,a}}}} \cdot \textcolor{{{P_BLUE}}}{{Δθ}}}}",
             P_WHITE),
            (None, r"\;[\mathrm{m^{3}/s}]", P_PRIMARY),
        ]

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(
            Create(beam), FadeIn(pivot),
            FadeIn(left_pan), FadeIn(right_pan),
            FadeIn(left_txt), FadeIn(right_txt),
            run_time=2.0,
        )
        self.play(FadeIn(eq_mark), run_time=0.6)
        self.play(FadeIn(start_eq), Create(start_box), run_time=1.2)
        self.play(Indicate(left_txt, color=P_CYAN), Indicate(right_txt, color=P_YELLOW), run_time=1.2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "substitute"))
        self.play(FadeOut(balance), run_time=0.8)
        self.play(start_panel.animate.move_to(UP * 1.05), run_time=0.8)

        bridge = VGroup(
            math_label(r"\dot{Q}_{V} = \dot{Q}_{S,tr}", size=BODY_FONT_SIZE, color=P_YELLOW),
            math_label(r"↓\;\text{umstellen nach}\;q_{v,R}", size=LABEL_FONT_SIZE, color=P_PRIMARY),
        ).arrange(DOWN, buff=0.12)
        bridge.next_to(start_panel, DOWN, buff=0.28)

        final_eq, final_box, final_items = math_panel(final_parts, color=P_CYAN)
        # The one typeset fraction box: [numerator, bar, denominator, marker].
        final_frac = final_items["frac"][0]
        final_items["qstr"] = final_frac[0]
        final_items["rho"] = final_frac[2]
        final_items["dth"] = final_frac[2]
        self.play(FadeIn(bridge), run_time=0.7)
        self.play(FadeIn(final_eq), Create(final_box), run_time=1.2)
        hold_for(self, self.NARRATION, "substitute", used=0.8 + 0.8 + 0.7 + 1.2 + 0.35)

        start_links = {
            "qvr": ("qvr",),
            "qstr": ("qv",),
            "rho": ("rho", "cp"),
            "dth": ("dth",),
        }
        for key, color in (("qvr", P_CYAN), ("qstr", P_YELLOW), ("rho", P_GREEN), ("dth", P_BLUE)):
            ring = highlight_param(final_items, key, color=color)
            self.play(
                Create(ring),
                *[Indicate(start_items[sk], color=color, scale_factor=1.12) for sk in start_links[key]],
                run_time=0.55,
            )
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        hold_for(self, self.NARRATION, "result", used=0.35)

        #region solar load from part 4
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "load"))
        ff, fv, g, i = (ValueTracker(v) for v in (1.0, 1.0, 0.0, 0.0))

        def q_live():
            return A_GLAZING * ff.get_value() * fv.get_value() * g.get_value() * i.get_value()

        load_line = math_readout(
            lambda: (rf"\dot{{Q}}_{{S,tr}} = {de_num(A_GLAZING)}\,\mathrm{{m^{{2}}}} \cdot {de_num(ff.get_value(), 2)}"
                     rf" \cdot {de_num(fv.get_value(), 2)} \cdot {de_num(g.get_value(), 2)}"
                     rf" \cdot {de_num(i.get_value())}\,\mathrm{{W/m^{{2}}}} \approx {de_num(round(q_live(), -1))}\,\mathrm{{W}}"),
            np.array([-6.4, 0.9, 0.0]), size=FORMULA_FONT_SIZE, color=P_YELLOW, edge="left",
        )
        solar_chip = _back_chip("← Teil 4:", r"\dot{Q}_{S,tr}", P_YELLOW).move_to(np.array([-4.2, 2.3, 0.0]))
        self.play(FadeOut(start_panel), FadeOut(bridge), FadeIn(solar_chip), run_time=0.7)
        self.add(load_line)
        self.play(_ramp(ff, F_F), run_time=0.8)
        self.play(_ramp(fv, F_V), run_time=0.8)
        self.play(_ramp(g, G_TOT), run_time=0.8)
        self.play(_ramp(i, I_S_MAX), run_time=1.0)
        hold_for(self, self.NARRATION, "load", used=0.7 + 0.8 + 0.8 + 0.8 + 1.0 + 0.35)
        #endregion

        #region airflow against the hygiene airflow
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "compare"))
        x0, y_h, y_s, length, full = -6.2, 0.35, -0.6, 3.6, 0.14
        k_h, k_s = ValueTracker(0.0), ValueTracker(0.0)
        h_row = _row(_name(r"q_{v,\mathrm{R}}", "Hygiene", AIR), x0, y_h, length, AIR, lambda: k_h.get_value() * Q_V_R_HYGIENE / full)
        s_row = _row(_name(r"q_{v,\mathrm{R}}", "Solarlast", P_YELLOW), x0, y_s, length, P_YELLOW,
                     lambda: k_s.get_value() * Q_V_R_SOLAR / full)
        h_read = math_readout(lambda: rf"{de_num(Q_V_R_HYGIENE * k_h.get_value(), 3)}\,\mathrm{{m^{{3}}/s}}",
                              np.array([h_row["x_end"] + 0.25, y_h, 0.0]), size=BODY_FONT_SIZE, color=AIR)
        s_read = math_readout(lambda: rf"{de_num(Q_V_R_SOLAR * k_s.get_value(), 3)}\,\mathrm{{m^{{3}}/s}}",
                              np.array([s_row["x_end"] + 0.25, y_s, 0.0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        hygiene_chip = _back_chip("← Lüftung · Kapitel 2:", r"q_{v,\mathrm{R}}").move_to(np.array([-4.2, 2.3, 0.0]))
        factor = VGroup(
            math_label(rf"\frac{{{de_num(Q_V_R_SOLAR, 3)}}}{{{de_num(Q_V_R_HYGIENE, 3)}}} \approx {de_num(FACTOR, 1)}",
                       size=BODY_FONT_SIZE, color=P_ORANGE),
        )
        factor_box = SurroundingRectangle(factor, color=P_ORANGE, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER)
        factor_group = VGroup(factor_box, factor).move_to(np.array([2.6, (y_h + y_s) / 2, 0.0]))
        num_eq, num_box, _ = math_panel([
            ("qvr", r"q_{v,R}", P_CYAN), (None, "=", P_WHITE),
            ("frac", rf"\frac{{{de_num(W_SOLAR / 1000, 2)}\,\mathrm{{kW}}}}"
                     rf"{{{de_num(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}} \cdot {de_num(CP_AIR, 1)}\,\mathrm{{kJ/(kg\,K)}} \cdot {de_num(D_THETA)}\,\mathrm{{K}}}}",
             P_WHITE),
            (None, "=", P_WHITE), (None, rf"{de_num(Q_V_R_SOLAR, 3)}\,\mathrm{{m^{{3}}/s}}", P_CYAN),
        ], color=P_CYAN)
        self.play(FadeOut(load_line), ReplacementTransform(solar_chip, hygiene_chip),
                  ReplacementTransform(final_eq, num_eq), ReplacementTransform(final_box, num_box), run_time=0.9)
        self.play(FadeIn(h_row["group"], shift=UP * 0.1), FadeIn(s_row["group"], shift=UP * 0.1), run_time=0.6)
        self.add(h_read, s_read)
        self.play(_ramp(k_s, 1.0), run_time=1.4)
        self.play(_ramp(k_h, 1.0), run_time=1.2)
        self.play(FadeIn(factor_group, shift=LEFT * 0.2), run_time=0.7)
        hold_for(self, self.NARRATION, "compare", used=0.9 + 0.6 + 1.4 + 1.2 + 0.7 + 0.35)
        #endregion

        #region duct from the airflow
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "duct"))
        units_per_m, k_c = 9.0, ValueTracker(0.0)
        cy, cx_h, cx_s = 0.65, 2.1, 4.8
        disc_h = always_redraw(lambda: Circle(radius=max(0.01, R_HYGIENE * units_per_m * k_c.get_value()), color=AIR, stroke_width=3,
                                              fill_color=AIR, fill_opacity=0.3).move_to(np.array([cx_h, cy, 0.0])))
        disc_s = always_redraw(lambda: Circle(radius=max(0.01, R_SOLAR * units_per_m * k_c.get_value()), color=P_YELLOW,
                                              stroke_width=3, fill_color=P_YELLOW, fill_opacity=0.3).move_to(np.array([cx_s, cy, 0.0])))
        tag_h = VGroup(body_text("Hygiene", font_size=LABEL_FONT_SIZE, color=AIR),
                       math_readout(lambda: rf"A = {de_num(A_HYGIENE * k_c.get_value(), 4)}\,\mathrm{{m^{{2}}}}",
                                    np.array([cx_h, -1.12, 0.0]), size=LABEL_FONT_SIZE + 1, color=AIR, edge="center"))
        tag_h[0].move_to(np.array([cx_h, -0.72, 0.0]))
        tag_s = VGroup(body_text("Solarlast", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
                       math_readout(lambda: rf"A = {de_num(A_SOLAR * k_c.get_value(), 4)}\,\mathrm{{m^{{2}}}}",
                                    np.array([cx_s, -1.12, 0.0]), size=LABEL_FONT_SIZE + 1, color=P_YELLOW, edge="center"))
        tag_s[0].move_to(np.array([cx_s, -0.72, 0.0]))
        duct_chip = _back_chip("← Lüftung · Kapitel 5:", r"A = q_{v,\mathrm{R}} / v_{m}").move_to(np.array([-3.4, 2.3, 0.0]))
        area_eq, area_box, _ = math_panel([
            ("A", "A", P_BLUE), (None, "=", P_WHITE),
            ("frac", rf"\frac{{\textcolor{{{P_CYAN}}}{{q_{{v,R}}}}}}{{\textcolor{{{P_TEAL}}}{{v_{{m}}}}}}", P_WHITE),
            (None, "=", P_WHITE),
            ("val", rf"\frac{{{de_num(Q_V_R_SOLAR, 3)}\,\mathrm{{m^{{3}}/s}}}}{{{de_num(V_M, 1)}\,\mathrm{{m/s}}}} \approx {de_num(A_SOLAR, 4)}\,\mathrm{{m^{{2}}}}",
             P_BLUE),
        ], color=P_BLUE)
        self.play(ReplacementTransform(hygiene_chip, duct_chip), ReplacementTransform(num_eq, area_eq),
                  ReplacementTransform(num_box, area_box), FadeOut(factor_group), run_time=0.9)
        self.add(disc_h, disc_s, tag_h, tag_s)
        self.play(_ramp(k_c, 1.0), run_time=2.4)
        hold_for(self, self.NARRATION, "duct", used=0.9 + 2.4 + 0.35)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
