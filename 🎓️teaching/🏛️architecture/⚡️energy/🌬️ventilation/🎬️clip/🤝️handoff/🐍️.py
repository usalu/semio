"""🌬️ Ventilation Chapter 6 — Übergabe an Heizung und Kühltechnik.

Beat 1 collects the seven quantities of the video (V, n, V̇ = q_v,R, c_Luft, Δθ, Δx, η_WRG) on one board
and lets the formula chain grow out of the cards; Beat 2 hands the result over: Heizung takes the winter
sensible flow as Q̇_V, Kühltechnik takes sensible plus latent flow, the airflow, the night purge and the cold recovery.

The numbers are the running example of the whole video — source of truth: Ch 2 (V, n, V̇), Ch 3 (c_Luft, Δθ, Δx,
Q̇_V,sens, Q̇_V,lat), Ch 5 (η_WRG, duct).
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
import sys as _sys


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
body_text = _manim_module.body_text
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
window_chip = _manim_module.window_chip
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
label_number = _manim_module.label_number
highlight_param = _manim_module.highlight_param
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

_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Übergabe an Heizung und Kühltechnik"

#region Running example
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
V_NET, N_RATE = _example.V_NET_M3, _example.N_AIR_PER_H
V_DOT = V_NET * N_RATE
C_LUFT = _example.C_LUFT_WH_M3_K
DT_WINTER, DT_SUMMER = _example.D_THETA_WINTER_K, _example.D_THETA_SUMMER_K
DX_WINTER_GKG, DX_SUMMER_GKG = _example.D_X_WINTER_G_KG, _example.D_X_G_KG
R_KJKG, RHO_AIR = _example.R_LAT_KJ_KG, _example.RHO_AIR_KG_M3
ETA_WRG = _example.ETA_WRG
Q_WINTER = round(_example.Q_SENS_WINTER_W, -1)
Q_SUMMER = round(_example.Q_SENS_SUMMER_W, -1)
Q_LAT_WINTER = round(_example.Q_LAT_WINTER_W, -1)
Q_LAT_SUMMER = round(_example.Q_LAT_SUMMER_W, -1)
Q_WINTER_WRG = round(Q_WINTER * (1.0 - ETA_WRG), -1)
Q_SUMMER_WRG = round(Q_SUMMER * (1.0 - ETA_WRG), -1)
Q_SUM_WINTER, Q_SUM_SUMMER = Q_WINTER + Q_LAT_WINTER, Q_SUMMER + Q_LAT_SUMMER
Q_SUM_WINTER_WRG, Q_SUM_SUMMER_WRG = Q_WINTER_WRG + Q_LAT_WINTER, Q_SUMMER_WRG + Q_LAT_SUMMER

VENT_BLUE = semio_color("secondary")
COL_V, COL_N, COL_VDOT, COL_C = P_ORANGE, P_CYAN, VENT_BLUE, P_GREEN
COL_DT, COL_DX, COL_ETA = P_YELLOW, P_BLUE, semio_color("info")
COL_HEAT, COL_COOL, COL_REC = P_ORANGE, P_CYAN, P_TEAL
COL_RHO, COL_R0 = P_WHITE, P_RED
CARD_W, CARD_H = 1.42, 1.40
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _streams(streams, rt: float, *, speed: float = 0.45, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _sym_card(sym_src, name, fmt, color, at, *, width: float = CARD_W, height: float = CARD_H):
    """🪪 Symbol card — the quantity's symbol, its name and its running-example value counted up by ``tracker``.

    The frame stays on the board as a dim slot until the narration names the quantity.
    """
    at = np.array(at, dtype=float)
    frame = RoundedRectangle(width=width, height=height, corner_radius=0, color=color, stroke_width=_theme_module.CHROME_BORDER,
                             fill_color=_theme_module.CHROME_FILL, fill_opacity=0.82).move_to(at)
    frame.set_stroke(opacity=0.28).set_z_index(-1)
    sym = math_label(sym_src, at + UP * 0.22, size=26, color=color)
    label = body_text(name, font_size=LABEL_FONT_SIZE - 2, color=P_PRIMARY)
    label.scale_to_fit_width(min(label.width, width - 0.12)).move_to(at + DOWN * 0.10)
    tracker = ValueTracker(0.0)
    value = math_readout(lambda: fmt(tracker.get_value()), at + DOWN * 0.48, size=LABEL_FONT_SIZE - 4, color=P_WHITE,
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
    content = VGroup(label, quantity).arrange(RIGHT, buff=_theme_module.UI_SPACING)
    return window_chip(content, color=color)


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
    entries = ((tr("Physical fundamentals", "Physikalische Grundlagen"), P_YELLOW), (tr("Ventilation", "Lüftung"), VENT_BLUE), (tr("Heating", "Heizung"), COL_HEAT),
               (tr("Cooling", "Kühltechnik"), COL_COOL), (tr("Energy balance", "Energiebilanz"), P_TEAL))
    chips = VGroup(*[chip(text, colour, font_size=LABEL_FONT_SIZE) for text, colour in entries])
    chips.arrange(RIGHT, buff=0.62)
    chips.move_to(np.array([0.0, y, 0.0]))
    arrows = VGroup(*[Arrow(a.get_right() + RIGHT * 0.04, b.get_left() + LEFT * 0.04, buff=0, color=P_WHITE,
                            stroke_width=2.2, max_tip_length_to_length_ratio=0.5, tip_length=0.14)
                      for a, b in zip(chips[:-1], chips[1:])])
    return chips, arrows

#endregion


#region Beat 1 – Zusammenfassung

CARD_Y, CARD_STEP = 1.36, 1.5
X_FORMULA, X_WINTER, X_SUMMER = -6.55, 1.45, 4.85
Y_HEAD = 0.22
Y_ROWS = (-0.34, -0.80, -1.26, -1.72, -2.18)

_CARD_EN = {"Volumen": "Volume", "Luftwechsel": "Air change", "Volumenstrom": "Volume flow", "Dichte": "Density",
            "Wärmekapazität": "Heat capacity", "Winter / Sommer": "Winter / Summer", "Feuchtedifferenz": "Humidity diff.",
            "Verdampfungsenthalpie": "Vaporisation enthalpy", "Rückgewinnung": "Recovery"}
CARD_SPECS = (
    ("v", r"V", "Volumen", lambda k: rf"{label_number(V_NET * k)}\,\mathrm{{m^{{3}}}}", COL_V),
    ("n", r"n", "Luftwechsel", lambda k: rf"{label_number(N_RATE * k, 1)}\,\mathrm{{h^{{-1}}}}", COL_N),
    ("vdot", r"\dot{V}", "Volumenstrom", lambda k: rf"{label_number(V_DOT * k)}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{h}}}}", COL_VDOT),
    ("rho", r"\rho", "Dichte", lambda k: rf"{label_number(RHO_AIR * k, 1)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}}", COL_RHO),
    ("c", r"c_{\mathrm{Luft}}", "Wärmekapazität", lambda k: rf"{label_number(C_LUFT * k, 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}K}}}}", COL_C),
    ("dt", r"\Delta\theta", "Winter / Sommer",
     lambda k: rf"{label_number(DT_WINTER * k)}\,/\,{label_number(DT_SUMMER * k)}\,\mathrm{{K}}", COL_DT),
    ("dx", r"\Delta x", "Feuchtedifferenz",
     lambda k: rf"{label_number(DX_WINTER_GKG * k, 1)}\,/\,{label_number(DX_SUMMER_GKG * k, 1)}\,\frac{{\mathrm{{g}}}}{{\mathrm{{kg}}}}", COL_DX),
    ("r0", r"r_{0}", "Verdampfungsenthalpie", lambda k: rf"{label_number(R_KJKG * k)}\,\frac{{\mathrm{{kJ}}}}{{\mathrm{{kg}}}}", COL_R0),
    ("eta", r"\eta_{\mathrm{WRG}}", "Rückgewinnung", lambda k: label_number(ETA_WRG * k, 1), COL_ETA),
)


class Beat1_Zusammenfassung(Scene):
    NARRATION = [
        ("intro",
         "Nine quantities carry the whole ventilation topic — here they lie on one board.",
         "Neun Größen tragen das ganze Thema Lüftung — hier liegen sie auf einer Tafel."),
        ("v",
         r"The volume $V$: $440\,\mathrm{m^3}$ of room air in the house.",
         r"Das Volumen $V$: $440\,\mathrm{m^3}$ Raumluft im Haus."),
        ("n",
         r"The air change rate $n$: $0.5\,\mathrm{h^{-1}}$ — half the volume every hour.",
         r"Die Luftwechselrate $n$: $0{,}5\,\mathrm{h^{-1}}$ — das halbe Volumen in jeder Stunde."),
        ("vdot",
         r"Together they give the volume flow $\dot{V} = V \cdot n = 220\,\mathrm{m^3/h}$. Cooling writes it $q_{v,R}$.",
         r"Zusammen ergeben sie den Volumenstrom $\dot{V} = V \cdot n = 220\,\mathrm{m^3/h}$. Die Kühltechnik schreibt $q_{v,R}$."),
        ("rho",
         r"Air has mass: its density is $\rho \approx 1.2\,\mathrm{kg/m^3}$ at $20\,\mathrm{°C}$.",
         r"Luft hat Masse: ihre Dichte beträgt $\rho \approx 1{,}2\,\mathrm{kg/m^3}$ bei $20\,\mathrm{°C}$."),
        ("c",
         r"Every cubic metre carries heat: $c_{Luft} = \rho \cdot c_p \approx 0.34\,\mathrm{Wh/(m^3K)}$.",
         r"Jeder Kubikmeter trägt Wärme: $c_{Luft} = \rho \cdot c_p \approx 0{,}34\,\mathrm{Wh/(m^3K)}$."),
        ("dt",
         r"The temperature difference $\Delta\theta$ drives the sensible flow: $20\,\mathrm{K}$ on the example winter day, $6\,\mathrm{K}$ on the example summer day.",
         r"Die Temperaturdifferenz $\Delta\theta$ treibt den fühlbaren Strom: $20\,\mathrm{K}$ am Beispiel-Wintertag, $6\,\mathrm{K}$ am Beispiel-Sommertag."),
        ("sens",
         r"Volume flow, heat capacity and temperature difference give the sensible heat flow $\dot{Q}_{V,sens}$: $1500\,\mathrm{W}$ in winter, $450\,\mathrm{W}$ in summer.",
         r"Daraus folgt der fühlbare Wärmestrom $\dot{Q}_{V,sens}$: $1500\,\mathrm{W}$ im Winter, $450\,\mathrm{W}$ im Sommer."),
        ("dx",
         r"Air also carries moisture. The humidity difference $\Delta x$ is $4.3\,\mathrm{g/kg}$ in winter and $4.5\,\mathrm{g/kg}$ in summer.",
         r"Luft trägt auch Feuchte. Die Feuchtedifferenz $\Delta x$ beträgt im Winter $4{,}3\,\mathrm{g/kg}$ und im Sommer $4{,}5\,\mathrm{g/kg}$."),
        ("r0",
         r"Water vapour carries energy: the enthalpy of vaporisation of water is $r_0 \approx 2450\,\mathrm{kJ/kg}$.",
         r"Wasserdampf trägt Energie: die Verdampfungsenthalpie von Wasser beträgt $r_0 \approx 2450\,\mathrm{kJ/kg}$."),
        ("lat",
         r"Together they give the latent heat flow $\dot{Q}_{V,lat} = \dot{V} \cdot \rho \cdot r_0 \cdot \Delta x$: $770\,\mathrm{W}$ in winter, $810\,\mathrm{W}$ in summer.",
         r"Damit wird daraus der latente Wärmestrom $\dot{Q}_{V,lat} = \dot{V} \cdot \rho \cdot r_0 \cdot \Delta x$: $770\,\mathrm{W}$ im Winter, $810\,\mathrm{W}$ im Sommer."),
        ("sum",
         r"Sensible plus latent is the sum $\dot{Q}_V$: $2270\,\mathrm{W}$ in winter, $1260\,\mathrm{W}$ in summer.",
         r"Fühlbar plus latent ergibt die Summe $\dot{Q}_V$: $2270\,\mathrm{W}$ im Winter, $1260\,\mathrm{W}$ im Sommer."),
        ("eta",
         r"A plate exchanger with $\eta = 0.8$ returns four fifths of the sensible part only.",
         r"Der Plattenübertrager mit $\eta = 0{,}8$ holt vier Fünftel des fühlbaren Anteils zurück — und nur den."),
        ("wrg",
         r"What remains: $300\,\mathrm{W} + 770\,\mathrm{W} = 1070\,\mathrm{W}$ in winter, $90\,\mathrm{W} + 810\,\mathrm{W} = 900\,\mathrm{W}$ in summer.",
         r"Übrig bleiben im Winter $300\,\mathrm{W} + 770\,\mathrm{W} = 1070\,\mathrm{W}$, im Sommer $90\,\mathrm{W} + 810\,\mathrm{W} = 900\,\mathrm{W}$."),
        ("conv",
         r"By convention the heating load counts only the sensible part, $\dot{Q}_{V,sens}$; the winter latent part costs energy only where air is humidified.",
         r"Die Heizlast rechnet nach Konvention nur den fühlbaren Teil $\dot{Q}_{V,sens}$; der latente Winteranteil kostet nur dort Energie, wo befeuchtet wird."),
        ("close",
         r"So heating takes the sensible winter values, cooling technology the sums $\dot{Q}_V$ of the summer values.",
         r"Die Heizung übernimmt also die fühlbaren Winterwerte, die Kühltechnik die Summen $\dot{Q}_V$ der Sommerwerte."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Nine quantities, one chain", "Neun Größen, eine Kette"))
        din = _din_ref(_references.cite("din-en-12831-1", "vdi-2078"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        xs = [-6.0 + CARD_STEP * i for i in range(len(CARD_SPECS))]
        cards = {key: _sym_card(sym, tr(_CARD_EN[name], name), fmt, colour, np.array([x, CARD_Y, 0.0]))
                 for (key, sym, name, fmt, colour), x in zip(CARD_SPECS, xs)}
        frames = VGroup(*[c["frame"] for c in cards.values()])

        head_w = body_text(tr("Winter · Heating", "Winter · Heizung"), font_size=LABEL_FONT_SIZE, color=COL_HEAT).move_to(
            np.array([X_WINTER, Y_HEAD, 0.0]))
        head_s = body_text(tr("Summer · Cooling", "Sommer · Kühltechnik"), font_size=LABEL_FONT_SIZE, color=COL_COOL).move_to(
            np.array([X_SUMMER, Y_HEAD, 0.0]))
        rule = Line(np.array([-6.6, Y_HEAD - 0.24, 0.0]), np.array([6.6, Y_HEAD - 0.24, 0.0]), color=P_WHITE, stroke_width=1.2,
                    stroke_opacity=0.35)

        def watts(amount, tracker, x, y, color):
            return math_readout(lambda: rf"{label_number(amount * tracker.get_value())}\,\mathrm{{W}}",
                                np.array([x, y, 0.0]), size=BODY_FONT_SIZE, color=color, edge="center")

        def watts_sum(first, second, tracker, x, y, color):
            return math_readout(
                lambda: rf"{label_number(first * tracker.get_value())}\,\mathrm{{W}}\,+\,{label_number(second * tracker.get_value())}\,\mathrm{{W}}\,=\,{label_number((first + second) * tracker.get_value())}\,\mathrm{{W}}",
                np.array([x, y, 0.0]), size=LABEL_FONT_SIZE + 1, color=color, edge="center")

        self.play(LaggedStart(*[FadeIn(f, shift=DOWN * 0.1) for f in frames], lag_ratio=0.1), run_time=1.4)
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
        res1 = math_readout(lambda: rf"=\;{label_number(V_DOT * k1.get_value())}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{h}}}}",
                            np.array([row1.get_right()[0] + 0.12, Y_ROWS[0], 0.0]), size=22, color=COL_VDOT, edge="left")
        caption = swap_caption(self, caption, subtitle_text(N, "vdot"))
        self.play(FadeIn(rule), FadeIn(head_w), FadeIn(head_s), run_time=0.6)
        self.add(res1)
        self.play(stage.transition("ventilation.handoff.summary.vdot-v", source=cards["v"]["sym"], target=it1["v"]), stage.transition("ventilation.handoff.summary.vdot-n", source=cards["n"]["sym"], target=it1["n"]),
                  FadeIn(it1["vdot"]), FadeIn(_unkeyed(row1, it1)), k1.animate.set_value(1.0), run_time=1.6)
        self.add(cards["vdot"]["value"])
        self.play(cards["vdot"]["frame"].animate.set_stroke(opacity=1.0), stage.transition("ventilation.handoff.summary.card-vdot", source=it1["vdot"], target=cards["vdot"]["sym"]),
                  FadeIn(cards["vdot"]["name"]), cards["vdot"]["tracker"].animate.set_value(1.0), run_time=1.2)
        hold_for(self, N, "vdot")

        for key in ("rho", "c", "dt"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row2, it2 = _place_row([("q", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), ("c", r"c_{\mathrm{Luft}}", COL_C), (None, r"\cdot", None),
                                ("dt", r"\Delta\theta", COL_DT)], Y_ROWS[1], X_FORMULA, size=22)
        k2 = ValueTracker(0.0)
        res2w, res2s = watts(Q_WINTER, k2, X_WINTER, Y_ROWS[1], COL_HEAT), watts(Q_SUMMER, k2, X_SUMMER, Y_ROWS[1], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        self.add(res2w, res2s)
        self.play(*[stage.transition(f"ventilation.handoff.summary.sens-{k}", source=cards[k]["sym"], target=it2[k]) for k in ("vdot", "c", "dt")], FadeIn(it2["q"]),
                  FadeIn(_unkeyed(row2, it2)), k2.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "sens")

        for key in ("dx", "r0"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row3, it3 = _place_row([("q", r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), ("rho", r"\rho", COL_RHO), (None, r"\cdot", None), ("r0", r"r_{0}", COL_R0),
                                (None, r"\cdot", None), ("dx", r"\Delta x", COL_DX)], Y_ROWS[2], X_FORMULA, size=22)
        k3 = ValueTracker(0.0)
        res3w, res3s = watts(Q_LAT_WINTER, k3, X_WINTER, Y_ROWS[2], COL_DX), watts(Q_LAT_SUMMER, k3, X_SUMMER, Y_ROWS[2], COL_DX)
        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        self.add(res3w, res3s)
        self.play(*[stage.transition(f"ventilation.handoff.summary.lat-{k}", source=cards[k]["sym"], target=it3[k]) for k in ("vdot", "rho", "r0", "dx")], FadeIn(it3["q"]),
                  FadeIn(_unkeyed(row3, it3)), k3.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "lat")

        row4, it4 = _place_row([("q", r"\dot{Q}_{\mathrm{V}}", P_WHITE), (None, "=", None), ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE),
                                (None, "+", None), ("lat", r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE)], Y_ROWS[3], X_FORMULA, size=22)
        k4 = ValueTracker(0.0)
        res4w, res4s = watts(Q_SUM_WINTER, k4, X_WINTER, Y_ROWS[3], COL_HEAT), watts(Q_SUM_SUMMER, k4, X_SUMMER, Y_ROWS[3], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        self.add(res4w, res4s)
        self.play(FadeIn(row4), k4.animate.set_value(1.0), run_time=1.4)
        hold_for(self, N, "sum")

        caption = swap_caption(self, caption, subtitle_text(N, "eta"))
        self.play(*reveal("eta"), run_time=1.2)
        hold_for(self, N, "eta")

        row5, it5 = _place_row([("q", r"\dot{Q}_{\mathrm{V,WRG}}", P_WHITE), (None, "=", None),
                                (None, r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, r"\cdot", None),
                                ("eta", r"(1-\eta_{\mathrm{WRG}})", COL_ETA), (None, "+", None),
                                (None, r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE)], Y_ROWS[4], X_FORMULA, size=22)
        k5 = ValueTracker(0.0)
        res5w = watts_sum(Q_WINTER_WRG, Q_LAT_WINTER, k5, X_WINTER - 0.05, Y_ROWS[4], COL_HEAT)
        res5s = watts_sum(Q_SUMMER_WRG, Q_LAT_SUMMER, k5, X_SUMMER + 0.05, Y_ROWS[4], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "wrg"))
        self.add(res5w, res5s)
        self.play(stage.transition("ventilation.handoff.summary.wrg-eta", source=cards["eta"]["sym"], target=it5["eta"]), FadeIn(it5["q"]), FadeIn(_unkeyed(row5, it5)),
                  k5.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "wrg")

        ring_sens = SurroundingRectangle(res2w, color=COL_HEAT, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        ring_lat = DashedVMobject(SurroundingRectangle(res3w, color=COL_DX, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3), num_dashes=24)
        note = body_text(tr("heating load: sensible only (convention)", "Heizlast: nur fühlbar (Konvention)"), font_size=LABEL_FONT_SIZE - 2, color=COL_HEAT)
        note.scale_to_fit_width(min(note.width, 2.15)).move_to(np.array([3.3, (Y_ROWS[1] + Y_ROWS[2]) / 2, 0.0]))
        caption = swap_caption(self, caption, subtitle_text(N, "conv"))
        self.play(Create(ring_sens), Create(ring_lat), FadeIn(note), run_time=1.2)
        hold_for(self, N, "conv")

        ring_sum = SurroundingRectangle(res4s, color=COL_COOL, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        self.play(Create(ring_sum), Indicate(res4s, color=COL_COOL, scale_factor=1.15), run_time=1.2)
        hold_for(self, N, "close")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion


#region Beat 2 – Übergabe an Heizung und Kühltechnik

HOUSE_AT, HOUSE_SCALE = np.array([0.0, -0.10, 0.0]), 0.80
BAR_UNIT = 0.0013
X_BAR_W, X_BAR_S = -2.55, 2.55
X_COL_W, X_COL_S = -6.70, 3.02
Y_HAND = 2.12
ROW_Y = (1.42, 0.96, 0.50, 0.04, -0.42, -0.88, -1.34)
Y_CHAIN = -2.12


class Beat2_UebergabeHeizungKuehlung(Scene):
    NARRATION = [
        ("intro",
         "The same house is ventilated all year round — one airflow, but two different bills.",
         "Dasselbe Haus wird das ganze Jahr gelüftet — ein Luftstrom, aber zwei verschiedene Rechnungen."),
        ("winter",
         r"On the example winter day the exhaust air carries heat out: $\dot{Q}_{V,sens} = 1500\,\mathrm{W}$ at $\Delta\theta = 20\,\mathrm{K}$ — and moisture, too: $\dot{Q}_{V,lat} = 770\,\mathrm{W}$.",
         r"Am Beispiel-Wintertag trägt die Abluft Wärme hinaus: $\dot{Q}_{V,sens} = 1500\,\mathrm{W}$ bei $\Delta\theta = 20\,\mathrm{K}$ — und auch Feuchte: $\dot{Q}_{V,lat} = 770\,\mathrm{W}$."),
        ("heizung",
         r"Heating takes the sensible flow as the ventilation term $\dot{Q}_V$ of its load; the latent part is not counted by convention.",
         r"Die Heizung übernimmt den fühlbaren Strom als Lüftungsterm $\dot{Q}_V$ der Heizlast; Latentes zählt nach Konvention nicht."),
        ("wrg_winter",
         r"With heat recovery only $300\,\mathrm{W}$ of the sensible part remain to be covered.",
         r"Mit Wärmerückgewinnung bleiben vom fühlbaren Teil nur $300\,\mathrm{W}$ zu decken."),
        ("summer",
         r"In summer the direction reverses: $\dot{Q}_{V,sens} = 450\,\mathrm{W}$ and, in addition, $\dot{Q}_{V,lat} = 810\,\mathrm{W}$.",
         r"Im Sommer kehrt sich die Richtung um: $\dot{Q}_{V,sens} = 450\,\mathrm{W}$ und zusätzlich $\dot{Q}_{V,lat} = 810\,\mathrm{W}$."),
        ("kuehlung",
         r"Cooling technology takes both parts, their sum $\dot{Q}_V = 1260\,\mathrm{W}$, as ventilation load, and the airflow sizes its system.",
         r"Die Kühltechnik übernimmt beide Anteile, ihre Summe $\dot{Q}_V = 1260\,\mathrm{W}$, als Lüftungslast — der Luftstrom dimensioniert ihre Anlage."),
        ("extras",
         r"Night ventilation discharges the storage mass, and cold recovery cuts the sensible part from $450\,\mathrm{W}$ to $90\,\mathrm{W}$.",
         r"Nachtlüftung entlädt die Speichermasse, die Kälterückgewinnung senkt den fühlbaren Anteil von $450\,\mathrm{W}$ auf $90\,\mathrm{W}$."),
        ("serie",
         "That completes ventilation: heating and cooling technology follow, and the energy balance closes the series.",
         "Damit ist die Lüftung erklärt: Es folgen Heizung und Kühltechnik, die Energiebilanz schließt die Serie."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("What heating and cooling technology take over", "Was Heizung und Kühltechnik übernehmen"))
        din = _din_ref(_references.cite("din-en-12831-1", "vdi-2078"))
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

        winter = _bar_stack(X_BAR_W, base_y, [COL_HEAT, COL_REC, COL_DX], unit=BAR_UNIT, opacities=[0.8, 0.42, 0.8])
        summer = _bar_stack(X_BAR_S, base_y, [COL_COOL, COL_REC, COL_DX], unit=BAR_UNIT, opacities=[0.8, 0.42, 0.8])
        w0, w1, w2 = winter["values"]
        s0, s1, s2 = summer["values"]
        ground = VGroup(*[Line(np.array([x - 0.45, base_y, 0.0]), np.array([x + 0.45, base_y, 0.0]), color=P_PRIMARY, stroke_width=2.2)
                          for x in (X_BAR_W, X_BAR_S)])

        hand_w = _hand_over(tr("→ Heating:", "→ Heizung:"), r"\dot{Q}_{\mathrm{V,sens}}", COL_HEAT).move_to(np.array([X_COL_W + 1.55, Y_HAND, 0.0]))
        hand_s = _hand_over(tr("→ Cooling:", "→ Kühltechnik:"), r"\dot{Q}_{\mathrm{V}}", COL_COOL).move_to(np.array([X_COL_S + 1.55, Y_HAND, 0.0]))
        hand_w.align_to(np.array([X_COL_W, 0.0, 0.0]), LEFT)
        hand_s.align_to(np.array([X_COL_S, 0.0, 0.0]), LEFT)

        def row(src, y, x_left, color, *, size: float = 20):
            return math_label(src, np.array([x_left, y, 0.0]), size=size, color=color, edge="left")

        def live(fn, y, x_left, color, *, size: float = 20):
            return math_readout(fn, np.array([x_left, y, 0.0]), size=size, color=color, edge="left")

        w_dt = row(r"\Delta\theta=20\,\mathrm{K}", ROW_Y[0], X_COL_W, COL_DT)
        w_flow = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(w0.get_value() + w1.get_value())}\,\mathrm{{W}}",
                      ROW_Y[1], X_COL_W, COL_HEAT)
        w_term = row(tr(r"\text{Ventilation term }\dot{Q}_{\mathrm{V}}\text{ of the load}", r"\text{Lüftungsterm }\dot{Q}_{\mathrm{V}}\text{ der Heizlast}"), ROW_Y[2], X_COL_W, P_WHITE)
        w_wrg = live(lambda: tr(rf"\text{{with HRV:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(w0.get_value())}\,\mathrm{{W}}", rf"\text{{mit WRG:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(w0.get_value())}\,\mathrm{{W}}"), ROW_Y[3], X_COL_W, COL_ETA)
        w_lat = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}}={label_number(w2.get_value())}\,\mathrm{{W}}", ROW_Y[4], X_COL_W, COL_DX)
        w_lat_note = row(tr(r"\text{not in the load}", r"\text{nicht in der Heizlast}"), ROW_Y[5], X_COL_W, COL_DX, size=17)

        s_dt = row(r"\Delta\theta=6\,\mathrm{K}", ROW_Y[0], X_COL_S, COL_DT)
        s_sens = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(s0.get_value() + s1.get_value())}\,\mathrm{{W}}",
                      ROW_Y[1], X_COL_S, COL_COOL)
        s_lat = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}}={label_number(s2.get_value())}\,\mathrm{{W}}", ROW_Y[2], X_COL_S, COL_DX)
        s_sum = live(lambda: rf"\dot{{Q}}_{{\mathrm{{V}}}}={label_number(s0.get_value() + s1.get_value() + s2.get_value())}\,\mathrm{{W}}",
                     ROW_Y[3], X_COL_S, COL_COOL)
        s_air = row(tr(r"\text{Design:}\;q_{v,\mathrm{R}}=0.061\,\frac{\mathrm{m^{3}}}{\mathrm{s}}", r"\text{Auslegung:}\;q_{v,\mathrm{R}}=0{,}061\,\frac{\mathrm{m^{3}}}{\mathrm{s}}"), ROW_Y[4], X_COL_S, COL_VDOT)
        s_night = row(tr(r"\text{Night ventilation}\to\text{storage}", r"\text{Nachtlüftung}\to\text{Speicher}"), ROW_Y[5], X_COL_S, P_WHITE)
        s_rec = live(lambda: tr(rf"\text{{with HRV:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(s0.get_value())}\,\mathrm{{W}}", rf"\text{{mit WRG:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}}={label_number(s0.get_value())}\,\mathrm{{W}}"),
                     ROW_Y[6], X_COL_S, COL_ETA)

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(*[open_window(w, run_time=0.8) for w in hs["windows"]], FadeIn(guides), run_time=0.8)
        self.play(*breeze("neutral")(1.0), run_time=1.0)
        hold_for(self, N, "intro", used=0.3 + 1.2 + 0.8 + 1.0, during=breeze("neutral"))

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.add(winter["mob"], w_flow)
        self.play(FadeIn(hand_w), FadeIn(w_dt), FadeIn(ground[0]), w0.animate.set_value(Q_WINTER), *breeze("winter")(1.6),
                  run_time=1.6)
        self.add(w_lat)
        self.play(w2.animate.set_value(Q_LAT_WINTER), *breeze("winter")(1.4), run_time=1.4)
        hold_for(self, N, "winter", during=breeze("winter"))

        caption = swap_caption(self, caption, subtitle_text(N, "heizung"))
        ring = SurroundingRectangle(hand_w, color=COL_HEAT, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        self.play(Create(ring), FadeIn(w_term), FadeIn(w_lat_note), *breeze("winter")(0.8), run_time=0.8)
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
        self.play(s2.animate.set_value(Q_LAT_SUMMER), *breeze("summer")(1.4), run_time=1.4)
        hold_for(self, N, "summer", during=breeze("summer"))

        caption = swap_caption(self, caption, subtitle_text(N, "kuehlung"))
        ring = SurroundingRectangle(hand_s, color=COL_COOL, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        self.add(s_sum)
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
