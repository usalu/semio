"""🌬️ Ventilation Chapter 6 — Übergabe an Heizung und Kühlung.

Beat 1 collects the seven quantities of the video (V, n, V̇ = q_v,R, c_Luft, Δθ, Δx, η_WRG) on one board
and lets the formula chain grow out of the cards; Beat 2 hands the result over: Heizung takes the winter
sensible flow as Φ_V, Kühlung takes sensible plus latent flow, the airflow, the night purge and the cold recovery.

The numbers are the running example of the whole video — source of truth: Ch 2 (V, n, V̇), Ch 3 (c_Luft, Δθ, Δx,
Q̇_V,sens, Q̇_V,lat), Ch 5 (η_WRG, duct).
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
beat_subtitle = _manim_module.beat_subtitle
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
flow_animation = _manim_module.flow_animation
chip = _manim_module.chip
dim_chip = _manim_module.dim_chip
math_label = _manim_module.math_label
math_row = _manim_module.math_row
math_readout = _manim_module.math_readout
de_num = _manim_module.de_num
highlight_param = _manim_module.highlight_param
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing
house_section = _manim_module.house_section
open_window = _manim_module.open_window

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "⏱️timing" / "🔣️.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Übergabe an Heizung und Kühlung"

#region Running example
V_NET, N_RATE = 440.0, 0.5
V_DOT = V_NET * N_RATE
C_LUFT = 0.34
DT_WINTER, DT_SUMMER = 20.0, 6.0
DX_GKG, R_KJKG, RHO_AIR = 4.5, 2450.0, 1.2
ETA_WRG = 0.8
Q_WINTER = round(V_DOT * C_LUFT * DT_WINTER, -1)
Q_SUMMER = round(V_DOT * C_LUFT * DT_SUMMER, -1)
Q_LATENT = round(V_DOT / 3600.0 * RHO_AIR * R_KJKG * 1000.0 * DX_GKG / 1000.0, -1)
Q_WINTER_WRG = round(Q_WINTER * (1.0 - ETA_WRG), -1)
Q_SUMMER_WRG = round(Q_SUMMER * (1.0 - ETA_WRG), -1)

VENT_BLUE = "#60A5FA"
COL_V, COL_N, COL_VDOT, COL_C = P_ORANGE, P_CYAN, VENT_BLUE, P_GREEN
COL_DT, COL_DX, COL_ETA = P_YELLOW, P_BLUE, "#A78BFA"
COL_HEAT, COL_COOL, COL_REC = P_ORANGE, P_CYAN, P_TEAL
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _streams(streams, rt: float, *, speed: float = 0.45, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _sym_card(sym_src, name, fmt, color, at, *, width: float = 1.84, height: float = 1.50):
    """🪪 Symbol card — the quantity's symbol, its name and its running-example value counted up by ``tracker``.

    The frame stays on the board as a dim slot until the narration names the quantity.
    """
    at = np.array(at, dtype=float)
    frame = RoundedRectangle(width=width, height=height, corner_radius=0.1, color=color, stroke_width=1.8,
                             fill_color=P_DEEP_DARK, fill_opacity=0.82).move_to(at)
    frame.set_stroke(opacity=0.28).set_z_index(-1)
    sym = math_label(sym_src, at + UP * 0.22, size=28, color=color)
    label = body_text(name, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL).move_to(at + DOWN * 0.08)
    tracker = ValueTracker(0.0)
    value = math_readout(lambda: fmt(tracker.get_value()), at + DOWN * 0.50, size=LABEL_FONT_SIZE - 1, color=P_WHITE,
                         edge="center")
    return {"frame": frame, "sym": sym, "name": label, "value": value, "tracker": tracker,
            "static": VGroup(sym, label), "color": color}


def _place_row(parts, baseline_y, x_left, *, size: float = 24):
    """📝 Typeset row whose left edge sits on ``x_left`` and whose baseline is ``baseline_y``."""
    row, items = math_row(parts, font_size=size)
    row.shift(np.array([x_left - row.get_left()[0], baseline_y - row.base.get_center()[1], 0.0]))
    return row, items


def _unkeyed(row, items):
    """🧩 Everything in a typeset row that is not a named card term — operators, equals sign, result symbol."""
    named = [id(m) for m in items.values()]
    return VGroup(*[m for m in row if id(m) not in named])


def _hand_over(prefix, quantity_src, color):
    """🔖 Hand-over chip naming the quantity the later video takes over."""
    label = body_text(prefix, font_size=LABEL_FONT_SIZE + 1, color=color)
    quantity = math_label(quantity_src, size=BODY_FONT_SIZE, color=color)
    content = VGroup(label, quantity).arrange(RIGHT, buff=0.14)
    box = SurroundingRectangle(content, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8)
    return VGroup(box, content)


def _bar_stack(x, base_y, colours, *, unit, width: float = 0.55, opacities=None):
    """📊 Vertical stack whose segment heights follow value trackers in watts — a heat flow as a drawn length."""
    values = [ValueTracker(0.0) for _ in colours]
    opacities = opacities or [0.8] * len(colours)

    def build():
        out, cursor = VGroup(), base_y
        for v, colour, opacity in zip(values, colours, opacities):
            h = v.get_value() * unit
            if h > 0.004:
                out.add(Rectangle(width=width, height=h, color=colour, stroke_width=1.4, fill_color=colour,
                                  fill_opacity=opacity).move_to(np.array([x, cursor + h / 2, 0.0])))
            cursor += h
        return out

    return {"mob": always_redraw(build), "values": values,
            "total": lambda: float(sum(v.get_value() for v in values)),
            "top": lambda: base_y + sum(v.get_value() for v in values) * unit}


def _series_chain(y):
    """⛓️ The five videos of the series as chips on one line — Lüftung is the stop we are at."""
    entries = (("Physikalische Grundlagen", P_YELLOW), ("Lüftung", VENT_BLUE), ("Heizung", COL_HEAT),
               ("Kühlung", COL_COOL), ("Energiebilanz", P_TEAL))
    chips = VGroup(*[chip(text, colour, font_size=LABEL_FONT_SIZE) for text, colour in entries])
    chips.arrange(RIGHT, buff=0.62)
    chips.move_to(np.array([0.0, y, 0.0]))
    arrows = VGroup(*[Arrow(a.get_right() + RIGHT * 0.04, b.get_left() + LEFT * 0.04, buff=0, color=P_WHITE,
                            stroke_width=2.2, max_tip_length_to_length_ratio=0.5, tip_length=0.14)
                      for a, b in zip(chips[:-1], chips[1:])])
    return chips, arrows

#endregion


#region Beat 1 – Zusammenfassung

CARD_Y = 1.28
X_FORMULA, X_WINTER, X_SUMMER = -6.55, 1.55, 4.75
Y_HEAD = 0.10
Y_ROWS = (-0.46, -0.96, -1.46, -1.96)

CARD_SPECS = (
    ("v", r"V", "Volumen", lambda k: rf"{de_num(V_NET * k)}\,\mathrm{{m^{{3}}}}", COL_V),
    ("n", r"n", "Luftwechsel", lambda k: rf"{de_num(N_RATE * k, 1)}\,\mathrm{{h^{{-1}}}}", COL_N),
    ("vdot", r"\dot{V}=q_{v,\mathrm{R}}", "Volumenstrom", lambda k: rf"{de_num(V_DOT * k)}\,\mathrm{{m^{{3}}/h}}", COL_VDOT),
    ("c", r"c_{\mathrm{Luft}}", "Wärmekapazität", lambda k: rf"{de_num(C_LUFT * k, 2)}\,\mathrm{{Wh/(m^{{3}}K)}}", COL_C),
    ("dt", r"\Delta\theta", "Winter / Sommer",
     lambda k: rf"{de_num(DT_WINTER * k)}\,/\,{de_num(DT_SUMMER * k)}\,\mathrm{{K}}", COL_DT),
    ("dx", r"\Delta x", "Feuchte, Sommer", lambda k: rf"{de_num(DX_GKG * k, 1)}\,\mathrm{{g/kg}}", COL_DX),
    ("eta", r"\eta_{\mathrm{WRG}}", "Rückgewinnung", lambda k: de_num(ETA_WRG * k, 1), COL_ETA),
)


class Beat1_Zusammenfassung(Scene):
    NARRATION = [
        ("intro",
         "Seven quantities carry the whole ventilation topic — here they lie on one board.",
         "Sieben Größen tragen das ganze Thema Lüftung — hier liegen sie auf einer Tafel."),
        ("v",
         "The volume V: 440 cubic metres of room air in the house.",
         "Das Volumen V: 440 Kubikmeter Raumluft im Haus."),
        ("n",
         "The air change rate n: 0.5 per hour — half the volume every hour.",
         "Die Luftwechselrate n: 0,5 pro Stunde — das halbe Volumen in jeder Stunde."),
        ("vdot",
         "Together they give the volume flow V dot: 220 cubic metres per hour. Cooling writes it q-v-R.",
         "Zusammen ergeben sie den Volumenstrom V-Punkt: 220 Kubikmeter pro Stunde — die Kühlung schreibt q-v-R."),
        ("c",
         "Every cubic metre carries heat: c Luft is 0.34 watt-hours per cubic metre and kelvin.",
         "Jeder Kubikmeter trägt Wärme: c-Luft beträgt 0,34 Wattstunden je Kubikmeter und Kelvin."),
        ("dt",
         "Delta theta is the driving temperature difference: 20 kelvin in winter, 6 kelvin in summer.",
         "Delta-Theta ist die Triebkraft: 20 Kelvin im Winter, 6 Kelvin im Sommer."),
        ("sens",
         "Volume flow, heat capacity and temperature difference give the sensible heat flow: 1500 watts in winter, 450 in summer.",
         "Daraus folgt der fühlbare Wärmestrom: 1500 Watt im Winter, 450 Watt im Sommer."),
        ("dx",
         "In summer the moisture difference delta x adds to it: 4.5 grams per kilogram.",
         "Im Sommer kommt die Feuchte hinzu: Delta-x beträgt 4,5 Gramm je Kilogramm."),
        ("lat",
         "With density and the heat of vaporisation it becomes a latent heat flow of 810 watts.",
         "Mit Dichte und Verdampfungswärme wird daraus ein latenter Wärmestrom von 810 Watt."),
        ("eta",
         "Heat recovery with eta equal 0.8 returns four fifths of the sensible part.",
         "Die Wärmerückgewinnung mit Eta gleich 0,8 holt vier Fünftel des fühlbaren Anteils zurück."),
        ("wrg",
         "What remains: 300 watts in winter, 90 watts in summer — the latent 810 watts stay untouched.",
         "Übrig bleiben 300 Watt im Winter und 90 Watt im Sommer — die latenten 810 Watt bleiben unberührt."),
        ("close",
         "Heating takes over the winter column, Cooling the summer column.",
         "Die Heizung übernimmt die Winterspalte, die Kühlung die Sommerspalte."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Sieben Größen, eine Kette", title)
        din = _din_ref("DIN EN 12831-1 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        xs = [-5.7 + 1.9 * i for i in range(7)]
        cards = {key: _sym_card(sym, name, fmt, colour, np.array([x, CARD_Y, 0.0]))
                 for (key, sym, name, fmt, colour), x in zip(CARD_SPECS, xs)}
        frames = VGroup(*[c["frame"] for c in cards.values()])

        head_w = body_text("Winter · Heizung", font_size=LABEL_FONT_SIZE, color=COL_HEAT).move_to(
            np.array([X_WINTER, Y_HEAD, 0.0]))
        head_s = body_text("Sommer · Kühlung", font_size=LABEL_FONT_SIZE, color=COL_COOL).move_to(
            np.array([X_SUMMER, Y_HEAD, 0.0]))
        rule = Line(np.array([-6.6, -0.14, 0.0]), np.array([6.2, -0.14, 0.0]), color=P_WHITE, stroke_width=1.2,
                    stroke_opacity=0.35)

        self.play(LaggedStart(*[FadeIn(f, shift=DOWN * 0.1) for f in frames], lag_ratio=0.12), run_time=1.4)
        hold_for(self, N, "intro", used=0.3 + 1.4)

        def reveal(key, run_time=1.2):
            card = cards[key]
            self.add(card["value"])
            return [card["frame"].animate.set_stroke(opacity=1.0), FadeIn(card["static"], shift=UP * 0.08),
                    card["tracker"].animate.set_value(1.0)]

        for key in ("v", "n"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row1, it1 = _place_row([("vdot", r"\dot{V}", COL_VDOT), (None, "=", None), ("v", "V", COL_V), (None, r"\cdot", None),
                                ("n", "n", COL_N)], Y_ROWS[0], X_FORMULA)
        k1 = ValueTracker(0.0)
        res1 = math_readout(lambda: rf"=\;{de_num(V_DOT * k1.get_value())}\,\mathrm{{m^{{3}}/h}}",
                            np.array([row1.get_right()[0] + 0.12, Y_ROWS[0], 0.0]), size=24, color=COL_VDOT, edge="left")
        caption = swap_caption(self, caption, subtitle_text(N, "vdot"))
        self.play(FadeIn(rule), FadeIn(head_w), FadeIn(head_s), run_time=0.6)
        self.add(res1)
        self.play(TransformFromCopy(cards["v"]["sym"], it1["v"]), TransformFromCopy(cards["n"]["sym"], it1["n"]),
                  FadeIn(it1["vdot"]), FadeIn(_unkeyed(row1, it1)), k1.animate.set_value(1.0), run_time=1.6)
        self.add(cards["vdot"]["value"])
        self.play(cards["vdot"]["frame"].animate.set_stroke(opacity=1.0), TransformFromCopy(it1["vdot"], cards["vdot"]["sym"]),
                  FadeIn(cards["vdot"]["name"]), cards["vdot"]["tracker"].animate.set_value(1.0), run_time=1.2)
        hold_for(self, N, "vdot")

        caption = swap_caption(self, caption, subtitle_text(N, "c"))
        self.play(*reveal("c"), run_time=1.2)
        hold_for(self, N, "c")

        caption = swap_caption(self, caption, subtitle_text(N, "dt"))
        self.play(*reveal("dt"), run_time=1.2)
        hold_for(self, N, "dt")

        row2, it2 = _place_row([("q", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), ("c", r"c_{\mathrm{Luft}}", COL_C), (None, r"\cdot", None),
                                ("dt", r"\Delta\theta", COL_DT)], Y_ROWS[1], X_FORMULA)
        k2 = ValueTracker(0.0)
        res2w = math_readout(lambda: rf"{de_num(Q_WINTER * k2.get_value())}\,\mathrm{{W}}",
                             np.array([X_WINTER, Y_ROWS[1], 0.0]), size=BODY_FONT_SIZE, color=COL_HEAT, edge="center")
        res2s = math_readout(lambda: rf"{de_num(Q_SUMMER * k2.get_value())}\,\mathrm{{W}}",
                             np.array([X_SUMMER, Y_ROWS[1], 0.0]), size=BODY_FONT_SIZE, color=COL_COOL, edge="center")
        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        self.add(res2w, res2s)
        self.play(*[TransformFromCopy(cards[k]["sym"], it2[k]) for k in ("vdot", "c", "dt")], FadeIn(it2["q"]),
                  FadeIn(_unkeyed(row2, it2)), k2.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "sens")

        caption = swap_caption(self, caption, subtitle_text(N, "dx"))
        self.play(*reveal("dx"), run_time=1.2)
        hold_for(self, N, "dx")

        row3, it3 = _place_row([("q", r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), (None, r"\rho", P_WHITE), (None, r"\cdot", None), (None, "r", P_WHITE),
                                (None, r"\cdot", None), ("dx", r"\Delta x", COL_DX)], Y_ROWS[2], X_FORMULA)
        k3 = ValueTracker(0.0)
        res3s = math_readout(lambda: rf"{de_num(Q_LATENT * k3.get_value())}\,\mathrm{{W}}",
                             np.array([X_SUMMER, Y_ROWS[2], 0.0]), size=BODY_FONT_SIZE, color=COL_DX, edge="center")
        none_w = math_label("–", np.array([X_WINTER, Y_ROWS[2], 0.0]), size=BODY_FONT_SIZE, color=P_TEAL)
        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        self.add(res3s)
        self.play(*[TransformFromCopy(cards[k]["sym"], it3[k]) for k in ("vdot", "dx")], FadeIn(it3["q"]),
                  FadeIn(_unkeyed(row3, it3)), FadeIn(none_w), k3.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "lat")

        caption = swap_caption(self, caption, subtitle_text(N, "eta"))
        self.play(*reveal("eta"), run_time=1.2)
        hold_for(self, N, "eta")

        row4, it4 = _place_row([("q", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, r"\cdot", None),
                                ("eta", r"(1-\eta_{\mathrm{WRG}})", COL_ETA)], Y_ROWS[3], X_FORMULA)
        k4 = ValueTracker(0.0)
        res4w = math_readout(lambda: rf"{de_num(Q_WINTER_WRG * k4.get_value())}\,\mathrm{{W}}",
                             np.array([X_WINTER, Y_ROWS[3], 0.0]), size=BODY_FONT_SIZE, color=COL_HEAT, edge="center")
        res4s = math_readout(lambda: rf"{de_num(Q_SUMMER_WRG * k4.get_value())}\,\mathrm{{W}}\,+\,{de_num(Q_LATENT)}\,\mathrm{{W}}",
                             np.array([X_SUMMER, Y_ROWS[3], 0.0]), size=BODY_FONT_SIZE, color=COL_COOL, edge="center")
        caption = swap_caption(self, caption, subtitle_text(N, "wrg"))
        self.add(res4w, res4s)
        self.play(TransformFromCopy(cards["eta"]["sym"], it4["eta"]), FadeIn(it4["q"]), FadeIn(_unkeyed(row4, it4)),
                  k4.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "wrg")

        box_w = Rectangle(width=2.75, height=2.46, color=COL_HEAT, stroke_width=2.2).move_to(np.array([X_WINTER, -0.95, 0.0]))
        box_s = Rectangle(width=3.05, height=2.46, color=COL_COOL, stroke_width=2.2).move_to(np.array([X_SUMMER, -0.95, 0.0]))
        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        self.play(Create(box_w), Create(box_s), run_time=1.2)
        hold_for(self, N, "close")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion


#region Beat 2 – Übergabe an Heizung und Kühlung

HOUSE_AT, HOUSE_SCALE = np.array([0.0, -0.10, 0.0]), 0.80
BAR_UNIT = 0.0013
X_BAR_W, X_BAR_S = -2.55, 2.55
X_COL_W, X_COL_S = -6.70, 3.02
Y_HAND = 2.12
ROW_Y = (1.42, 0.96, 0.50, 0.04, -0.42, -0.88)
Y_CHAIN = -2.12


class Beat2_UebergabeHeizungKuehlung(Scene):
    NARRATION = [
        ("intro",
         "The same house is ventilated all year round — one airflow, but two different bills.",
         "Dasselbe Haus wird das ganze Jahr gelüftet — ein Luftstrom, aber zwei verschiedene Rechnungen."),
        ("winter",
         "In winter the exhaust air carries heat out: 1500 watts at a temperature difference of 20 kelvin.",
         "Im Winter trägt die Abluft Wärme hinaus: 1500 Watt bei 20 Kelvin Temperaturdifferenz."),
        ("heizung",
         "Heating takes this flow over as the ventilation term phi V of its heating demand.",
         "Die Heizung übernimmt diesen Strom als Lüftungsterm Phi-V des Heizwärmebedarfs."),
        ("wrg_winter",
         "With heat recovery only 300 watts remain to be covered.",
         "Mit Wärmerückgewinnung bleiben davon nur 300 Watt zu decken."),
        ("summer",
         "In summer the direction reverses: 450 watts sensible and, in addition, 810 watts latent.",
         "Im Sommer kehrt sich die Richtung um: 450 Watt fühlbar und zusätzlich 810 Watt latent."),
        ("kuehlung",
         "Cooling takes both parts as ventilation load, and the airflow sizes its system.",
         "Die Kühlung übernimmt beide Anteile als Lüftungslast — der Luftstrom dimensioniert ihre Anlage."),
        ("extras",
         "Night ventilation discharges the storage mass, and cold recovery cuts the sensible part from 450 to 90 watts.",
         "Nachtlüftung entlädt die Speichermasse, die Kälterückgewinnung senkt den fühlbaren Anteil von 450 auf 90 Watt."),
        ("serie",
         "That completes ventilation: Heating and Cooling follow, and the energy balance closes the series.",
         "Damit ist die Lüftung erklärt: Es folgen Heizung und Kühlung, die Energiebilanz schließt die Serie."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Was Heizung und Kühlung übernehmen", title)
        din = _din_ref("DIN EN 12831-1 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hs = house_section(HOUSE_AT, scale=HOUSE_SCALE)
        bl, br = hs["bottom_left"], hs["bottom_right"]
        base_y = float(bl[1])
        lo_win, up_win = hs["windows"]
        x_w, x_r = float(lo_win["x"]), float(br[0])
        y_lo, y_up, y_mid = float(lo_win["center"][1]), float(up_win["center"][1]), float(hs["level_1"].get_center()[1])
        f = HOUSE_SCALE / 0.62
        circuit = [smooth_path([np.array([x, y + dy, 0.0]) for x, y in (
            (x_w - 0.50, y_lo), (x_w - 0.20, y_lo + 0.01), (x_w + 0.30 * f, y_lo), (x_w + 0.75 * f, y_lo + 0.05 * f),
            (x_r - 0.42 * f, y_lo + 0.12 * f), (x_r - 0.28 * f, y_mid + 0.02), (x_r - 0.45 * f, y_up - 0.12 * f),
            (x_w + 0.75 * f, y_up + 0.04 * f), (x_w + 0.30 * f, y_up), (x_w - 0.20, y_up - 0.01), (x_w - 0.50, y_up))])
            for dy in (-0.05, 0.05)]
        guides = flow_guides(circuit, VENT_BLUE, opacity=0.22)

        def breeze(season):
            colours = {"neutral": (VENT_BLUE, VENT_BLUE), "winter": (P_BLUE, COL_HEAT), "summer": (P_RED, COL_COOL)}[season]

            def build(rt):
                return [_streams([(circuit, *colours)], rt, speed=0.35, waves=5)]
            return build

        winter = _bar_stack(X_BAR_W, base_y, [COL_HEAT, COL_REC], unit=BAR_UNIT, opacities=[0.8, 0.42])
        summer = _bar_stack(X_BAR_S, base_y, [COL_COOL, COL_REC, COL_DX], unit=BAR_UNIT, opacities=[0.8, 0.42, 0.8])
        w0, w1 = winter["values"]
        s0, s1, s2 = summer["values"]
        ground = VGroup(*[Line(np.array([x - 0.45, base_y, 0.0]), np.array([x + 0.45, base_y, 0.0]), color=P_TEAL, stroke_width=2.2)
                          for x in (X_BAR_W, X_BAR_S)])

        hand_w = _hand_over("→ Heizung:", r"\Phi_{\mathrm{V}}", COL_HEAT).move_to(np.array([X_COL_W + 1.55, Y_HAND, 0.0]))
        hand_s = _hand_over("→ Kühlung:", r"\dot{Q}_{\mathrm{L}}", COL_COOL).move_to(np.array([X_COL_S + 1.55, Y_HAND, 0.0]))
        hand_w.align_to(np.array([X_COL_W, 0.0, 0.0]), LEFT)
        hand_s.align_to(np.array([X_COL_S, 0.0, 0.0]), LEFT)

        def row(src, y, x_left, color, *, size: float = 20):
            return math_label(src, np.array([x_left, y, 0.0]), size=size, color=color, edge="left")

        def live(fn, y, x_left, color, *, size: float = 20):
            return math_readout(fn, np.array([x_left, y, 0.0]), size=size, color=color, edge="left")

        w_dt = row(r"\Delta\theta=20\,\mathrm{K}", ROW_Y[0], X_COL_W, COL_DT)
        w_flow = live(lambda: rf"\Phi_{{\mathrm{{V}}}}=\dot{{Q}}_{{\mathrm{{V,sens}}}}={de_num(w0.get_value() + w1.get_value())}\,\mathrm{{W}}",
                      ROW_Y[1], X_COL_W, COL_HEAT)
        w_term = row(r"\text{Lüftungsterm der Heizlast}", ROW_Y[2], X_COL_W, P_WHITE)
        w_wrg = live(lambda: rf"\text{{mit WRG:}}\;\Phi_{{\mathrm{{V}}}}={de_num(w0.get_value())}\,\mathrm{{W}}", ROW_Y[3], X_COL_W, COL_ETA)

        s_dt = row(r"\Delta\theta=6\,\mathrm{K}", ROW_Y[0], X_COL_S, COL_DT)
        s_sens = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}}={de_num(s0.get_value() + s1.get_value())}\,\mathrm{{W}}",
                      ROW_Y[1], X_COL_S, COL_COOL)
        s_lat = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}}={de_num(s2.get_value())}\,\mathrm{{W}}", ROW_Y[2], X_COL_S, COL_DX)
        s_air = row(r"\text{Auslegung:}\;q_{v,\mathrm{R}}=0{,}061\,\mathrm{m^{3}/s}", ROW_Y[3], X_COL_S, COL_VDOT)
        s_night = row(r"\text{Nachtlüftung}\to\text{Speicher}", ROW_Y[4], X_COL_S, P_WHITE)
        s_rec = live(lambda: rf"\text{{mit WRG:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}}={de_num(s0.get_value())}\,\mathrm{{W}}",
                     ROW_Y[5], X_COL_S, COL_ETA)

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(*[open_window(w, run_time=0.8) for w in hs["windows"]], FadeIn(guides), run_time=0.8)
        self.play(*breeze("neutral")(1.0), run_time=1.0)
        hold_for(self, N, "intro", used=0.3 + 1.2 + 0.8 + 1.0, during=breeze("neutral"))

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.add(winter["mob"], w_flow)
        self.play(FadeIn(hand_w), FadeIn(w_dt), FadeIn(ground[0]), w0.animate.set_value(Q_WINTER), *breeze("winter")(1.6),
                  run_time=1.6)
        hold_for(self, N, "winter", during=breeze("winter"))

        caption = swap_caption(self, caption, subtitle_text(N, "heizung"))
        ring = SurroundingRectangle(hand_w, color=COL_HEAT, buff=0.08, corner_radius=0.1, stroke_width=3)
        self.play(Create(ring), FadeIn(w_term), *breeze("winter")(0.8), run_time=0.8)
        hold_for(self, N, "heizung", during=breeze("winter"))

        caption = swap_caption(self, caption, subtitle_text(N, "wrg_winter"))
        self.add(w_wrg)
        self.play(FadeOut(ring), w0.animate.set_value(Q_WINTER_WRG), w1.animate.set_value(Q_WINTER - Q_WINTER_WRG),
                  *breeze("winter")(1.6), run_time=1.6)
        hold_for(self, N, "wrg_winter", during=breeze("winter"))

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        self.add(summer["mob"], s_sens, s_lat)
        self.play(FadeIn(hand_s), FadeIn(s_dt), FadeIn(ground[1]), s0.animate.set_value(Q_SUMMER), *breeze("summer")(1.4),
                  run_time=1.4)
        self.play(s2.animate.set_value(Q_LATENT), *breeze("summer")(1.4), run_time=1.4)
        hold_for(self, N, "summer", during=breeze("summer"))

        caption = swap_caption(self, caption, subtitle_text(N, "kuehlung"))
        ring = SurroundingRectangle(hand_s, color=COL_COOL, buff=0.08, corner_radius=0.1, stroke_width=3)
        self.play(Create(ring), FadeIn(s_air), *breeze("summer")(0.8), run_time=0.8)
        hold_for(self, N, "kuehlung", during=breeze("summer"))

        caption = swap_caption(self, caption, subtitle_text(N, "extras"))
        self.add(s_rec)
        self.play(FadeOut(ring), FadeIn(s_night), *breeze("summer")(0.8), run_time=0.8)
        self.play(s0.animate.set_value(Q_SUMMER_WRG), s1.animate.set_value(Q_SUMMER - Q_SUMMER_WRG), *breeze("summer")(1.6),
                  run_time=1.6)
        hold_for(self, N, "extras", during=breeze("summer"))

        chips, arrows = _series_chain(Y_CHAIN)
        caption = swap_caption(self, caption, subtitle_text(N, "serie"))
        self.play(LaggedStart(*[FadeIn(c, shift=RIGHT * 0.1) for c in chips], lag_ratio=0.2),
                  LaggedStart(*[FadeIn(a) for a in arrows], lag_ratio=0.2), run_time=1.6)
        dims = [a for i, c in enumerate(chips) if i != 1 for a in dim_chip(c, 0.55)]
        self.play(*dims, chips[1][0].animate.set_fill(VENT_BLUE, opacity=0.22).set_stroke(width=3.2), *breeze("summer")(1.0),
                  run_time=1.0)
        hold_for(self, N, "serie", during=breeze("summer"))

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion
