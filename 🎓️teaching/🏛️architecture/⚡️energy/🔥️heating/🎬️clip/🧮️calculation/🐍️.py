

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))
import os
_theme_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
semio_color = _theme_module.color
UI_SPACING = _theme_module.UI_SPACING

import numpy as np
from manim import *


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
body_text = _manim_module.body_text
BODY_FONT = _manim_module.BODY_FONT
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
room_metre = _manim_module.room_metre
P_PRIMARY = _manim_module.P_PRIMARY
section_material = _manim_module.section_material
section_assembly = _manim_module.section_assembly
PASTEL_CYAN = _manim_module.PASTEL_CYAN
PASTEL_PURPLE = _manim_module.PASTEL_PURPLE
PASTEL_PINK = _manim_module.PASTEL_PINK
PASTEL_WHITE = _manim_module.PASTEL_WHITE
PASTEL_ORANGE = _manim_module.PASTEL_ORANGE
PASTEL_YELLOW = _manim_module.PASTEL_YELLOW
PASTEL_RED = _manim_module.PASTEL_RED
PASTEL_BLUE = _manim_module.PASTEL_BLUE
PASTEL_GREEN = _manim_module.PASTEL_GREEN
PASTEL_GREY = _manim_module.PASTEL_GREY
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
unit_text = _manim_module.unit_text
begin_vo_beat = _manim_module.begin_vo_beat
math_label = _manim_module.math_label
math_row = _manim_module.math_row
math_readout = _manim_module.math_readout
math_panel = _manim_module.math_panel
label_number = _manim_module.label_number
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
ripples = _manim_module.ripples
pulse_flashes = _manim_module.pulse_flashes
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
house_section = _manim_module.house_section
window_glyph = _manim_module.window_glyph
open_window = _manim_module.open_window
room_section = _manim_module.room_section
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
staffage = _manim_module.staffage
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
sun_glyph = _manim_module.sun_glyph
moon_glyph = _manim_module.moon_glyph
thermometer_glyph = _manim_module.thermometer_glyph

# 🗣️ Timing follows German captions (reading floor in hold_for).


#region Shared numbers
#region Envelope and ventilation (Modul 2 / Modul 3)
_PHI_T = _example.PHI_T_W
_H_T = _example.H_T_W_K
_PHI_V = _example.PHI_V_W
_H_V = _example.H_V_W_K
_PHI_LOSS = _example.PHI_LOSS_W
#endregion

#region Annual balance
_SEASON_DAYS = _example.SEASON_DAYS
_GT = _example.GT_K_D_PER_A
_THETA_I = _example.THETA_I_WINTER_C
_THETA_HG = _example.THETA_HG_C
_F_GT = 0.024 * _GT
_Q_LOSS = _example.Q_LOSS_KWH_PER_A
_Q_LOSS_SHOWN = _example.Q_LOSS_SHOWN_KWH_PER_A
_Q_SOL, _Q_INT = _example.Q_SOL_KWH_PER_A, _example.Q_INT_KWH_PER_A
DESK_SURFACE_M = 0.72
_Q_GAIN = _Q_SOL + _Q_INT
#endregion

#region Utilization day (Scene4)
_DAY_T = np.linspace(0.0, 24.0, 241)
_MASS_SHARE = 0.7


def _loss_kw(t):
    """🥶 Loss power of the example house over a clear winter day — H · (θ_i − θ_e(t))."""
    return (_H_T + _H_V) / 1000.0 * (_example.THETA_I_WINTER_C - (_example.THETA_E_SEASON_MEAN_C + 4.0 * np.cos(2.0 * np.pi * (t - 15.0) / 24.0)))


def _internal_kw(t):
    """🧑‍🍳 Internal gains: base load plus morning and evening occupancy peaks."""
    return 0.35 + 0.25 * np.exp(-((t - 7.5) / 1.2) ** 2) + 0.45 * np.exp(-((t - 19.0) / 2.0) ** 2)


def _solar_kw(t, peak):
    """🌞 Solar gains through the south windows between sunrise and sunset."""
    return peak * np.clip(np.sin(np.pi * (t - 8.0) / 8.5), 0.0, None) ** 1.5 * ((t > 8.0) & (t < 16.5))


def _gain_kw(peak, mass):
    """⚖️ Gain curve; ``mass`` stores part of the surplus and releases it after sunset."""
    gain = _internal_kw(_DAY_T) + _solar_kw(_DAY_T, peak)
    surplus = np.maximum(0.0, gain - _loss_kw(_DAY_T))
    shape = np.exp(-((_DAY_T - 19.5) / 2.0) ** 2)
    shape /= np.trapezoid(shape, _DAY_T)
    stored = _MASS_SHARE * mass
    return gain - stored * surplus + stored * np.trapezoid(surplus, _DAY_T) * shape


def _utilization(peak, mass, upto: float = 24.0):
    """🧮 ``(A_nutz, A_Gewinn, η)`` integrated from the sampled curves up to hour ``upto``; η is over the whole day."""
    gain = _gain_kw(peak, mass)
    used = np.minimum(gain, _loss_kw(_DAY_T))
    cut = _DAY_T <= upto + 1e-9
    if cut.sum() < 2:
        return 0.0, 0.0, 0.0
    a_used = float(np.trapezoid(used[cut], _DAY_T[cut]))
    a_gain = float(np.trapezoid(gain[cut], _DAY_T[cut]))
    return a_used, a_gain, a_used / float(np.trapezoid(gain, _DAY_T))


def _solve_solar_peak(target: float = _example.ETA_UTILISATION) -> float:
    """🎯 Solar peak that makes the light house reach ``target`` utilization."""
    lo, hi = 2.0, 6.0
    for _ in range(60):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if _utilization(mid, 0.0)[2] > target else (lo, mid)
    return (lo + hi) / 2


_SOLAR_PEAK = _solve_solar_peak()
_ETA = _example.ETA_UTILISATION
_Q_USE = _example.Q_USE_KWH_PER_A
_Q_H = _example.Q_H_KWH_PER_A
#endregion

#region System losses (DIN V 18599-5)
_CE_SHARE = 0.05
_PIPE_M, _PIPE_W_PER_M, _PIPE_H = 30.0, 10.0, 3000.0
_STORE_KWH_D, _STORE_D = 1.8, 200.0
_ETA_G = 0.95
_Q_CE = round(_CE_SHARE * _Q_H, -2)
_Q_D = _PIPE_M * _PIPE_W_PER_M * _PIPE_H / 1000.0
_Q_S = _STORE_KWH_D * _STORE_D
_Q_BEFORE_G = _Q_H + _Q_CE + _Q_D + _Q_S
_Q_G = _Q_BEFORE_G * (1.0 / _ETA_G - 1.0)
_Q_E = _Q_BEFORE_G + _Q_G
#endregion
#endregion


#region Shared helpers
def _heading(text: str):
    """🏷️ Place the instructional calculation heading in the shared content row."""
    return section_heading(text)


def _label(text: str, *, size: float = LABEL_FONT_SIZE, color=PASTEL_GREY):
    """🔤 Set a label in the canonical face and label hierarchy."""
    return body_text(text, font_size=size, color=color)


def _slot(mob):
    """📍 Left baseline point of a typeset box — where a live readout replaces it."""
    return np.array([mob.get_left()[0], mob.base.get_center()[1], 0.0])


def _freeze(readout):
    """🧊 Stop a live readout so it can morph like a static label."""
    readout.clear_updaters()
    return readout


def _year_theta_e():
    """🌡️ Daily mean outdoor temperature θ_e of the example year in °C — its heating days (θ_e below the heating limit) add up to exactly G_{t,20/15} (VDI 3807 Blatt 1)."""
    return _example.year_outdoor_profile()
#endregion


#region Handcrafted staffage
def _cast(cast: str, height: float, *, motion: bool = False, **kwargs):
    """🧍 Handcrafted staffage cast scaled to ``height`` manim units; animated in the thermal view when ``motion``."""
    build = staffage_motion if motion else staffage
    return build(cast, unit=height / staffage_height(cast), **kwargs)


def _pendant(top, metre: float):
    """💡 Handcrafted pendant lamp at the scene metre, hanging below ``top`` with its bulb point.

    https://github.com/ManimCommunity/manim
    """
    lamp = _cast("shared-lamp-winter", staffage_height("shared-lamp-winter") * metre)
    lamp.move_to(np.array(top, dtype=float) + DOWN * lamp.height / 2)
    return {"group": lamp, "bulb": Dot(lamp.get_bottom() + UP * 0.1 * lamp.height, radius=0.001).set_opacity(0)}
#endregion


#region Physical Fundamentals glyphs


#region Buildings
def _house(center, scale: float = 1.0):
    """🏠 Physical Fundamentals section house in the pastel palette, plus its interior ``air`` for a warm tint."""
    h = house_section(np.array(center, dtype=float), scale=scale)
    h["ground"].set_color(P_PRIMARY)
    for w in h["windows"]:
        w["sash"].set_color(PASTEL_CYAN)
    t = 0.1 * scale
    air = Polygon(
        h["bottom_left"] + RIGHT * t + UP * 0.02, h["bottom_right"] + LEFT * t + UP * 0.02,
        h["top_right"] + LEFT * t + DOWN * 0.05, h["roof_peak"] + DOWN * 0.14 * scale,
        h["top_left"] + RIGHT * t + DOWN * 0.05,
        stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    h["air"] = air
    return h


def _envelope_paths(h, *, sides=("left", "right", "roof", "floor"), reach: float = 0.5, inset: float = 0.3):
    """🚪 Particle tracks from inside the house out through wall and window, roof slopes and floor slab."""
    s = h["w_width"] / 3.6
    bl, br, tl, tr, peak = h["bottom_left"], h["bottom_right"], h["top_left"], h["top_right"], h["roof_peak"]
    din, dout, hw = inset * s, reach * s, h["w_height"]
    paths = []
    if "left" in sides:
        paths += [Line(bl + UP * f * hw + RIGHT * din, bl + UP * f * hw + LEFT * dout) for f in (0.25, 0.75)]
    if "right" in sides:
        paths += [Line(br + UP * f * hw + LEFT * din, br + UP * f * hw + RIGHT * dout) for f in (0.25, 0.75)]
    if "roof" in sides:
        for a in (tl, tr):
            d = peak - a
            n = np.array([-d[1], d[0], 0.0]) / np.linalg.norm(d)
            n = n if n[1] > 0 else -n
            m = a + d * 0.5
            paths.append(Line(m - n * din, m + n * dout))
    if "floor" in sides:
        paths += [Line(bl + RIGHT * f * h["w_width"] + UP * din, bl + RIGHT * f * h["w_width"] + DOWN * dout * 0.5)
                  for f in (0.3, 0.7)]
    return paths


def _window_wall(x: float, y: float, *, height: float = 1.1, opening: float = 0.56, wall: float = 0.2):
    """🪟 Physical Fundamentals wall slab with a glazed opening — the sash opens into the room on the right."""
    lo, hi = y - opening / 2, y + opening / 2
    y0, y1 = y - height / 2, y + height / 2
    below = section_material(Rectangle(width=wall, height=lo - y0)).move_to(np.array([x, (y0 + lo) / 2, 0.0]))
    above = section_material(Rectangle(width=wall, height=y1 - hi)).move_to(np.array([x, (hi + y1) / 2, 0.0]))
    win = window_glyph(x, lo, hi, depth=0.42, color=PASTEL_CYAN)
    return {"x": x, "lo": lo, "hi": hi, "window": win, "group": VGroup(below, above, win["group"])}


def _exchange_paths(ww, *, reach: float = 0.75):
    """🌬️ Warm room air rising out through the top of an open window, cold air sinking in at the sill."""
    x, lo, hi = ww["x"], ww["lo"], ww["hi"]
    mid, q = (lo + hi) / 2, (hi - lo) / 4
    out = [smooth_path([np.array([x + reach, mid - q * 0.3, 0.0]), np.array([x + 0.25, hi - q * 0.7, 0.0]),
                        np.array([x - 0.2, hi - q * 0.6, 0.0]), np.array([x - reach, hi + q * 0.2, 0.0])])]
    inflow = [smooth_path([np.array([x - reach, lo + q * 0.2, 0.0]), np.array([x - 0.2, lo + q * 0.6, 0.0]),
                           np.array([x + 0.25, lo + q * 0.5, 0.0]), np.array([x + reach, lo - q * 0.6, 0.0])])]
    return out, inflow


def _gain_room(center, *, w: float = 1.5, h: float = 1.1):
    """🏛️ Small Physical Fundamentals room section in the pastel palette — glazed left wall, tintable ``air``."""
    room = room_section(center, w=w, h=h, wall=0.1, slab=0.1)
    section_material(room["shell"])
    room["glass"].set_color(PASTEL_CYAN)
    room["air"].set_z_index(-1)
    return room
#endregion


#region Energy flow
def _dim(mob, k: float):
    """🌗 Scale every fill and stroke opacity of a glyph by ``k`` — for crossfading sun and moon."""
    for m in mob.family_members_with_points():
        m.set_fill(opacity=m.get_fill_opacity() * k)
        m.set_stroke(opacity=m.get_stroke_opacity() * k)
    return mob


def _flow(streams, rt: float, *, speed: float = 0.7, waves: int = 3, radius: float = 0.05):
    """💨 Particle streams that keep moving for ``rt`` seconds at ``speed`` passes per second."""
    return flow_animation(streams, waves=waves, radius=radius, cycles=max(0.6, speed * rt))


def _heat(spots, rt: float, *, r_max: float = 0.35, color=PASTEL_ORANGE, rings: int = 3, down: bool = False,
          facing=None):
    """♨️ Long-wave heat ripples from warm ``spots`` that keep spreading for ``rt`` seconds."""
    return ripples(spots, r_max=r_max, rings=rings, color=color, down=down, facing=facing,
                   cycles=max(1.0, rt / 1.3))


def _pulses(paths, rt: float, color=PASTEL_YELLOW, *, width: float = 4.0):
    """⚡ Light pulses running along sun rays, repeated to fill ``rt`` seconds."""
    return pulse_flashes(paths, color, repeats=max(1, int(rt / 1.5)), width=width)
#endregion
#endregion


class ReviewingHeatLosses(Scene):
    NARRATION = [
        ("trans",
         r"From module 2: heat flows through the envelope as $\dot{Q}_{\mathrm{T}}$ — the heat flow $\Phi$ as a load: $\Sigma\, U \cdot A \cdot \Delta\theta$, about $2\,300\,\mathrm{W}$.",
         r"Aus Modul 2: Wärme fließt durch die Hülle als $\dot{Q}_{\mathrm{T}}$ — der Wärmestrom $\Phi$ als Last: $\Sigma\, U \cdot A \cdot \Delta\theta$, rund $2\,300\,\mathrm{W}$."),
        ("vent",
         r"From module 3 and the Ventilation video: the air change carries heat out as $\dot{Q}_{\mathrm{V}}$ — $n \cdot V \cdot \rho c_{\mathrm{p}} \cdot \Delta\theta$, about $1\,500\,\mathrm{W}$.",
         r"Aus Modul 3 und dem Lüftungsvideo: Der Luftwechsel trägt Wärme als $\dot{Q}_{\mathrm{V}}$ hinaus — $n \cdot V \cdot \rho c_{\mathrm{p}} \cdot \Delta\theta$, rund $1\,500\,\mathrm{W}$."),
        ("total",
         r"Both add up to about $3\,800\,\mathrm{W}$, the total heat-loss power $\dot{Q}_{\mathrm{l}}$ — after DIN V 4108-6.",
         r"Beide zusammen ergeben die Gesamtverlustleistung $\dot{Q}_{\mathrm{l}}$ von rund $3\,800\,\mathrm{W}$ — nach DIN V 4108-6."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "trans"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading(tr("Overview of heat losses", "Übersicht der Wärmeverluste"))

        ICY_BLUE = PASTEL_CYAN
        DEEP_BLUE = PASTEL_BLUE
        WARM = PASTEL_ORANGE
        X0 = -6.3

        trans_formula = math_label(r"\dot{Q}_{\mathrm{T}} = \Sigma\, U_{i} \cdot A_{i} \cdot \Delta\theta",
                                   np.array([X0, 2.15, 0.0]), size=28, color=ICY_BLUE, edge="left")
        house = _house(np.array([-5.25, 0.8, 0.0]), scale=0.4)
        house_paths = _envelope_paths(house, reach=1.1, inset=0.6)
        phi_t = ValueTracker(0.0)
        t_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{T}}}} \approx {label_number(phi_t.get_value())}\,\mathrm{{W}}",
                              np.array([-3.65, 0.85, 0.0]), size=26, color=ICY_BLUE)

        def leak(rt):
            return [_flow([(house_paths, WARM, ICY_BLUE)], rt, speed=0.6, radius=0.045)]

        self.add(house["air"])
        self.play(
            Write(title),
            FadeIn(trans_formula, shift=DOWN * 0.1),
            FadeIn(house["group"]),
            house["air"].animate.set_fill(opacity=0.14),
            run_time=2.0,
        )
        self.add(t_read)
        self.play(*leak(3.0), phi_t.animate.set_value(_PHI_T), run_time=3.0)
        hold_for(self, self.NARRATION, "trans", used=0.3 + 2.0 + 3.0, during=leak)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vent"))
        vent_formula = math_label(r"\dot{Q}_{\mathrm{V}} = n \cdot V \cdot \rho c_{\mathrm{p}} \cdot \Delta\theta",
                                  np.array([X0, -0.6, 0.0]), size=28, color=DEEP_BLUE, edge="left")
        vent = _window_wall(-5.25, -1.72)
        vent_out, vent_in = _exchange_paths(vent)
        phi_v = ValueTracker(0.0)
        v_read = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V}}}} \approx {label_number(phi_v.get_value())}\,\mathrm{{W}}",
                              np.array([-3.65, -1.72, 0.0]), size=26, color=DEEP_BLUE)

        def losses(rt):
            return [*leak(rt), _flow([(vent_out, WARM, ICY_BLUE), (vent_in, DEEP_BLUE, WARM)], rt, speed=0.55,
                                     waves=4)]

        self.play(FadeIn(vent_formula, shift=DOWN * 0.1), FadeIn(vent["group"]), *leak(1.4), run_time=1.4)
        self.play(open_window(vent["window"]), *leak(0.6), run_time=0.6)
        self.add(v_read)
        self.play(*losses(3.0), phi_v.animate.set_value(_PHI_V), run_time=3.0)
        hold_for(self, self.NARRATION, "vent", during=losses)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        left_side = VGroup(trans_formula, house["group"], vent_formula, vent["group"], t_read, v_read)
        brace = Brace(left_side, RIGHT, color=PASTEL_WHITE, buff=0.35)
        arrow = Arrow(brace.get_right(), brace.get_right() + RIGHT * 0.9, color=PASTEL_WHITE, buff=0.1, stroke_width=3)
        loss_desc = _label(tr("Total heat loss rate (DIN V 4108-6:2003-06)", "Gesamtverlustleistung (DIN V 4108-6:2003-06)"))
        loss_title = math_label(r"\dot{Q}_{\mathrm{l}}", size=36).next_to(loss_desc, UP, buff=0.15)
        loss_slot = math_label(rf"= {label_number(_PHI_LOSS)}\,\mathrm{{W}}", size=32).next_to(loss_desc, DOWN, buff=0.25)
        VGroup(loss_title, loss_desc, loss_slot).next_to(arrow, RIGHT, buff=0.25)
        tot = ValueTracker(0.0)
        tot_read = math_readout(lambda: rf"= {label_number(tot.get_value())}\,\mathrm{{W}}", _slot(loss_slot), size=32)

        self.play(GrowFromCenter(brace), *losses(2.0), run_time=2.0)
        self.play(GrowArrow(arrow), FadeIn(VGroup(loss_title, loss_desc), shift=RIGHT * 0.2), *losses(2.0),
                  run_time=2.0)
        flying = [_freeze(t_read).copy(), _freeze(v_read).copy()]
        self.add(tot_read, *flying)
        self.play(*[f.animate.move_to(loss_slot).set_opacity(0.0) for f in flying],
                  tot.animate.set_value(_PHI_LOSS), *losses(2.0), run_time=2.0)
        self.remove(*flying)
        hold_for(self, self.NARRATION, "total", during=losses)
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class Scene2(Scene):
    NARRATION = [
        ("phi",
         r"So the loss power is $\dot{Q}_{\mathrm{l}} = \dot{Q}_{\mathrm{T}} + \dot{Q}_{\mathrm{V}}$ — watts at one temperature difference.",
         r"Die Verlustleistung ist $\dot{Q}_{\mathrm{l}} = \dot{Q}_{\mathrm{T}} + \dot{Q}_{\mathrm{V}}$ — Watt bei einer Temperaturdifferenz."),
        ("climate",
         r"A design-day wattage is not a yearly kilowatt-hour. Per kelvin the house loses $H_{\mathrm{T}}$ and $H_{\mathrm{V}}$ watts, with $H = \dot{Q} / \Delta\theta$.",
         r"Eine Auslegungs-Wattzahl ist keine Jahres-Kilowattstunde. Pro Kelvin verliert das Haus $H_{\mathrm{T}}$ und $H_{\mathrm{V}}$ Watt, mit $H = \dot{Q} / \Delta\theta$."),
        ("gt_ref",
         r"The degree-day number needs an indoor reference: room temperature $\theta_{\mathrm{i}} = 20\,\mathrm{°C}$ and heating limit $15\,\mathrm{°C}$ — together $G_{t,20/15}$.",
         r"Die Gradtagzahl braucht eine Innentemperatur: Raumtemperatur $\theta_{\mathrm{i}} = 20\,\mathrm{°C}$ und Heizgrenze $15\,\mathrm{°C}$ — zusammen $G_{t,20/15}$."),
        ("gradtag",
         r"On every day with $\theta_{\mathrm{e}} < 15\,\mathrm{°C}$ the difference $\theta_{\mathrm{i}} - \theta_{\mathrm{e}}$ counts: the shaded area is about $3\,500\,\mathrm{Kd/a}$, kelvin-days per year.",
         r"An jedem Tag mit $\theta_{\mathrm{e}} < 15\,\mathrm{°C}$ zählt die Differenz $\theta_{\mathrm{i}} - \theta_{\mathrm{e}}$: Die gefärbte Fläche beträgt rund $3\,500\,\mathrm{Kd/a}$, Kelvintage pro Jahr."),
        ("annual",
         r"$Q_{\mathrm{l}} = (H_{\mathrm{T}} + H_{\mathrm{V}}) \cdot F_{Gt}$ with $F_{Gt} = 0.024\,\mathrm{kKh/(Kd)} \cdot G_{t,20/15}$ — about $16\,000\,\mathrm{kWh/a}$, after DIN V 4108-6.",
         r"$Q_{\mathrm{l}} = (H_{\mathrm{T}} + H_{\mathrm{V}}) \cdot F_{Gt}$ mit $F_{Gt} = 0{,}024\,\mathrm{kKh/(Kd)} \cdot G_{t,20/15}$ — rund $16\,000\,\mathrm{kWh/a}$, nach DIN V 4108-6."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "phi"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading(tr("From heat flow to annual energy", "Vom Wärmestrom zur Jahresenergie"))
        self.play(Write(title), run_time=0.8)

        ICY_BLUE = PASTEL_CYAN
        DEEP_BLUE = PASTEL_BLUE
        PURPLE = PASTEL_PURPLE
        WARM = PASTEL_ORANGE

        initial_eq, ie = math_row([
            ("loss", r"\dot{Q}_{\mathrm{l}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE),
            ("t", r"\dot{Q}_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PASTEL_WHITE), ("v", r"\dot{Q}_{\mathrm{V}}", DEEP_BLUE),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING)
        initial_eq.move_to(UP * 1.35)

        house = _house(np.array([-2.5, -0.05, 0.0]), scale=0.4)
        house_paths = _envelope_paths(house, reach=1.1, inset=0.6)
        wall_label = _label("Transmission", size=18, color=ICY_BLUE).next_to(house["floor"], DOWN, buff=0.3)
        wall_icon = VGroup(house["group"], wall_label)

        vent = _window_wall(2.5, 0.1)
        vent_out, vent_in = _exchange_paths(vent)
        window_label = _label(tr("Ventilation", "Lüftung"), size=18, color=DEEP_BLUE).move_to(np.array([2.5, wall_label.get_y(), 0.0]))
        window_icon = VGroup(vent["group"], window_label)

        def losses(rt):
            return [_flow([(house_paths, WARM, ICY_BLUE)], rt, speed=0.6, radius=0.045),
                    _flow([(vent_out, WARM, ICY_BLUE), (vent_in, DEEP_BLUE, WARM)], rt, speed=0.55, waves=4)]

        self.add(house["air"])
        self.play(FadeIn(initial_eq), FadeIn(wall_icon), FadeIn(window_icon), house["air"].animate.set_fill(opacity=0.14),
                  run_time=1.4)
        self.play(open_window(vent["window"]), *losses(0.6), run_time=0.6)
        self.play(*losses(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "phi", used=0.3 + 0.8 + 1.4 + 0.6 + 2.0, during=losses)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "climate"))
        multiplier_group, mg = math_row([
            ("arrow", r"\to", PURPLE), ("ht", r"H_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PURPLE),
            ("hv", r"H_{\mathrm{V}}", DEEP_BLUE),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING * 2)
        probe = VGroup(initial_eq.copy(), multiplier_group.copy()).arrange(RIGHT, buff=0.3).move_to(UP * 1.35)
        target_initial_pos = probe[0].get_center()
        multiplier_group.move_to(probe[1].get_center())
        multiplier_group.shift(UP * (initial_eq.base.get_center()[1] - multiplier_group.base.get_center()[1]))

        climate_label = math_label(
            tr(r"H = \frac{\dot{Q}}{\Delta\theta} \quad \text{in}\; \frac{\mathrm{W}}{\mathrm{K}} \quad \text{annual energy uses}\; H \cdot G_{t,20/15}"
               r"\text{, not the design load}",
               r"H = \frac{\dot{Q}}{\Delta\theta} \quad \text{in}\; \frac{\mathrm{W}}{\mathrm{K}} \quad \text{Jahresenergie nutzt}\; H \cdot G_{t,20/15}"
               r"\text{, nicht die Auslegungslast}"),
            size=18, color=PURPLE,
        )
        climate_label.next_to(probe, UP, buff=0.45)

        h_t = ValueTracker(0.0)
        h_v = ValueTracker(0.0)
        ht_read = math_readout(
            lambda: rf"H_{{\mathrm{{T}}}} = \frac{{{label_number(_PHI_T)}\,\mathrm{{W}}}}{{20\,\mathrm{{K}}}} = "
                    rf"{label_number(h_t.get_value())}\,\frac{{\mathrm{{W}}}}{{\mathrm{{K}}}}",
            np.array([wall_icon.get_center()[0], -1.6, 0.0]), size=20, color=ICY_BLUE, edge="center")
        hv_read = math_readout(
            lambda: rf"H_{{\mathrm{{V}}}} = \frac{{{label_number(_PHI_V)}\,\mathrm{{W}}}}{{20\,\mathrm{{K}}}} = "
                    rf"{label_number(h_v.get_value())}\,\frac{{\mathrm{{W}}}}{{\mathrm{{K}}}}",
            np.array([window_icon.get_center()[0], -1.6, 0.0]), size=20, color=DEEP_BLUE, edge="center")

        self.play(
            initial_eq.animate.move_to(target_initial_pos),
            FadeIn(multiplier_group),
            FadeIn(climate_label),
            *losses(1.8),
            run_time=1.8,
        )
        self.add(ht_read, hv_read)
        self.play(h_t.animate.set_value(_H_T), h_v.animate.set_value(_H_V), *losses(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "climate", during=losses)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gt_ref"))
        theta_e = _year_theta_e()
        heating = theta_e < _THETA_HG
        O = np.array([-5.7, -2.0, 0.0])
        XL, YL, T_LO, T_HI = 7.4, 2.6, -8.0, 26.0

        def cp(day, temp):
            return O + RIGHT * (day / 365 * XL) + UP * ((temp - T_LO) / (T_HI - T_LO) * YL)

        x_axis = Arrow(O, O + RIGHT * (XL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.05)
        y_axis = Arrow(O, O + UP * (YL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.08)
        y_name = math_label(r"\theta\;\text{in}\;\mathrm{°C}", size=LABEL_FONT_SIZE)
        y_name.next_to(y_axis.get_end(), RIGHT, buff=0.14)
        y_ticks = VGroup(*[
            VGroup(Line(cp(0, k) + LEFT * 0.06, cp(0, k) + RIGHT * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(k), cp(0, k) + LEFT * 0.14 + DOWN * 0.07, size=15, edge="right"))
            for k in (0, 10)
        ])
        months = tr("JFMAMJJASOND", "JFMAMJJASOND")
        starts = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365]
        m_marks = VGroup(*[Line(cp(d, T_LO) + DOWN * 0.06, cp(d, T_LO) + UP * 0.06, color=PASTEL_WHITE, stroke_width=2)
                           for d in starts[1:-1]])
        m_names = VGroup(*[_label(m, size=15, color=PASTEL_WHITE).move_to(cp((a + b) / 2, T_LO) + DOWN * 0.22)
                           for m, a, b in zip(months, starts[:-1], starts[1:])])
        curve = VMobject(color=ICY_BLUE, stroke_width=2.4).set_points_as_corners([cp(d + 0.5, v) for d, v in enumerate(theta_e)])
        e_name = math_label(r"\theta_{\mathrm{e}}", cp(292, theta_e[292]) + DOWN * 0.38 + RIGHT * 0.25, size=22, color=ICY_BLUE)
        i_line = Line(cp(0, _THETA_I), cp(365, _THETA_I), color=WARM, stroke_width=3)
        hg_line = DashedLine(cp(0, _THETA_HG), cp(365, _THETA_HG), color=PASTEL_YELLOW, stroke_width=2.4)
        i_name = math_label(rf"\theta_{{\mathrm{{i}}}} = {label_number(_THETA_I)}\,\mathrm{{°C}}",
                            np.array([2.2, cp(0, _THETA_I)[1], 0.0]), size=22, color=WARM, edge="left")
        i_note = _label(tr("room temperature", "Raumtemperatur"), size=15, color=WARM).next_to(i_name, RIGHT, buff=0.2)
        hg_name = math_label(tr(rf"\text{{heating limit}}\;{label_number(_THETA_HG)}\,\mathrm{{°C}}", rf"\text{{Heizgrenze}}\;{label_number(_THETA_HG)}\,\mathrm{{°C}}"),
                             np.array([2.2, cp(0, _THETA_HG)[1], 0.0]), size=22, color=PASTEL_YELLOW, edge="left")

        day = ValueTracker(0.0)
        heating_days = np.flatnonzero(heating)
        bars = VGroup(*[
            Rectangle(width=XL / 365 * 1.2, height=(_THETA_I - theta_e[d]) / (T_HI - T_LO) * YL, stroke_width=0,
                      fill_color=PURPLE, fill_opacity=0.0).move_to((cp(d + 0.5, _THETA_I) + cp(d + 0.5, theta_e[d])) / 2)
            for d in heating_days
        ])
        bars.add_updater(lambda group: [bar.set_fill(opacity=0.55 if d + 1 <= day.get_value() else 0.0)
                                        for bar, d in zip(group, heating_days)])

        def gt_now():
            return float(sum(_THETA_I - theta_e[d] for d in heating_days if d + 1 <= day.get_value()))

        gt_def = math_label(r"G_{t,20/15} = \Sigma\,(\theta_{\mathrm{i}} - \theta_{\mathrm{e}}) \cdot 1\,\mathrm{d}",
                            np.array([2.2, -0.95, 0.0]), size=20, color=PURPLE, edge="left")
        gt_note = math_label(tr(r"\text{only days with}\;\theta_{\mathrm{e}} < 15\,\mathrm{°C}", r"\text{nur Tage mit}\;\theta_{\mathrm{e}} < 15\,\mathrm{°C}"),
                             np.array([2.2, -1.35, 0.0]), size=18, color=GREY_A, edge="left")
        d_read = math_readout(lambda: tr(rf"\mathrm{{Day}}\; {int(day.get_value())}\; \text{{of}}\; 365", rf"\mathrm{{Tag}}\; {int(day.get_value())}\; \text{{von}}\; 365"),
                              np.array([2.2, -1.75, 0.0]), size=20, color=GREY_A)
        gt_read = math_readout(lambda: rf"G_{{t,20/15}} \approx {label_number(round(gt_now(), -1))}\,\frac{{\mathrm{{Kd}}}}{{\mathrm{{a}}}}",
                               np.array([2.2, -2.25, 0.0]), size=24, color=PURPLE)
        mark = always_redraw(lambda: Dot(cp(day.get_value(), theta_e[min(int(day.get_value()), 364)]), radius=0.07, color=PASTEL_WHITE))
        d0 = 20
        gap = DoubleArrow(cp(d0 + 0.5, _THETA_I), cp(d0 + 0.5, theta_e[d0]), buff=0, stroke_width=2.2, tip_length=0.12, color=PASTEL_WHITE)
        gap_name = math_label(r"\theta_{\mathrm{i}} - \theta_{\mathrm{e}}", cp(d0 + 0.5, (_THETA_I + theta_e[d0]) / 2) + RIGHT * 0.9,
                              size=18, color=PASTEL_WHITE)

        self.play(FadeOut(VGroup(wall_icon, window_icon, house["air"], ht_read, hv_read, climate_label)), run_time=0.6)
        self.play(GrowArrow(x_axis), GrowArrow(y_axis), FadeIn(y_name), FadeIn(y_ticks), FadeIn(m_marks),
                  FadeIn(m_names), Create(curve), FadeIn(e_name), run_time=1.6)
        self.play(Create(i_line), FadeIn(i_name), FadeIn(i_note), run_time=0.9)
        self.play(Create(hg_line), FadeIn(hg_name), run_time=0.9)
        hold_for(self, self.NARRATION, "gt_ref", used=0.6 + 1.6 + 0.9 + 0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gradtag"))
        self.add(bars, mark, d_read, gt_read)
        self.play(FadeIn(gt_def), FadeIn(gt_note), GrowFromCenter(gap), FadeIn(gap_name), run_time=1.0)
        self.play(day.animate.set_value(365), run_time=4.0, rate_func=linear)
        hold_for(self, self.NARRATION, "gradtag", used=1.0 + 4.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "annual"))
        consolidated_eq, ce = math_row([
            ("q", r"Q_{\mathrm{l}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE), ("lp", "(", PURPLE),
            ("ht", r"H_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PASTEL_WHITE), ("hv", r"H_{\mathrm{V}}", DEEP_BLUE),
            ("rp", ")", PURPLE), ("times", r"\cdot", PURPLE), ("gt", r"G_{t,20/15}", PURPLE),
            ("times2", r"\cdot", PURPLE), ("hd", r"24\,\frac{\mathrm{h}}{\mathrm{d}}", PURPLE),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING)
        consolidated_eq.move_to(UP * 1.35)
        wk = r"\,\frac{\mathrm{W}}{\mathrm{K}}"
        values, vi = math_row([
            ("eq", "=", PASTEL_WHITE),
            ("h", rf"({label_number(_H_T)}{wk} + {label_number(_H_V)}{wk})", PASTEL_WHITE),
            ("d1", r"\cdot", PASTEL_WHITE), ("gt", rf"{label_number(_GT)}\,\frac{{\mathrm{{Kd}}}}{{\mathrm{{a}}}}", PURPLE),
            ("d2", r"\cdot", PASTEL_WHITE), ("hd", r"24\,\frac{\mathrm{h}}{\mathrm{d}}", PASTEL_WHITE),
        ], font_size=BODY_FONT_SIZE, buff=UI_SPACING)
        values.move_to(UP * 0.35)
        q = ValueTracker(0.0)
        q_read = math_readout(
            lambda: rf"Q_{{\mathrm{{l}}}} \approx {label_number(round(q.get_value(), -2))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            np.array([0.0, -0.6, 0.0]), size=34, color=PASTEL_WHITE, edge="center")

        bars.clear_updaters()
        area_static = bars
        self.remove(mark)
        _freeze(gt_read)
        self.play(
            stage.transition("heating.calculation.balance.loss-equation", source=VGroup(ie["loss"], ie["eq"]), target=VGroup(ce["q"], ce["eq"])),
            stage.transition("heating.calculation.balance.loss-terms", source=VGroup(mg["ht"], mg["plus"], mg["hv"]), target=VGroup(ce["ht"], ce["plus"], ce["hv"])),
            stage.transition("heating.calculation.balance.open-bracket", source=ie["t"], target=ce["lp"]),
            stage.transition("heating.calculation.balance.close-bracket", source=ie["v"], target=ce["rp"]),
            stage.transition("heating.calculation.balance.arrow-to-times", source=mg["arrow"], target=ce["times"]),
            FadeOut(ie["plus"]),
            stage.transition("heating.calculation.balance.degree-day-area-to-symbol", source=area_static, target=ce["gt"]),
            FadeOut(VGroup(x_axis, y_axis, y_name, y_ticks, m_marks, m_names, curve, e_name, i_line, hg_line, i_name, i_note,
                           hg_name, gt_def, gt_note, d_read, gap, gap_name)),
            run_time=2.5,
        )
        self.remove(initial_eq, multiplier_group, *initial_eq.submobjects, *multiplier_group.submobjects)
        self.play(FadeIn(VGroup(ce["times2"], ce["hd"]), shift=LEFT * 0.2), run_time=0.8)
        self.play(FadeIn(VGroup(vi["eq"], vi["h"], vi["d1"], vi["d2"], vi["hd"])),
                  stage.transition("heating.calculation.balance.degree-day-readout-to-value", source=gt_read, target=vi["gt"]), run_time=1.2)
        self.add(q_read)
        self.play(q.animate.set_value(_Q_LOSS), run_time=2.2)

        f_gt = math_label(rf"F_{{Gt}} = {label_number(0.024, 3)}\,\frac{{\mathrm{{kKh}}}}{{\mathrm{{Kd}}}} \cdot G_{{t,20/15}} ={label_number(_F_GT)}\,\frac{{\mathrm{{kKh}}}}{{\mathrm{{a}}}}",
                          np.array([0.0, -1.3, 0.0]), size=26, color=PURPLE)
        f_src = _label(_references.cite("!din-v-4108-6") + "\n" + tr("Heating-period method · degree days per ", "Heizperiodenverfahren · Gradtagzahl nach ") + _references.cite("vdi-3807-1"),
                       size=15, color=GREY_A).move_to(DOWN * 1.8)
        self.play(FadeIn(f_gt), FadeIn(f_src), run_time=1)

        hold_for(self, self.NARRATION, "annual")
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class ReviewingHeatGains(Scene):
    NARRATION = [
        ("intro",
         "The losses are offset by free heat gains inside the building.",
         "Den Verlusten stehen freie Wärmegewinne im Gebäude gegenüber."),
        ("solar",
         r"From module 5: winter sun through the windows is $\dot{Q}_{\mathrm{S}}$; over the heating season it adds up to $Q_{\mathrm{S}}$, about $3\,000\,\mathrm{kWh/a}$.",
         r"Aus Modul 5: Wintersonne durch die Fenster ist $\dot{Q}_{\mathrm{S}}$ — über die Heizperiode summiert sich das zu $Q_{\mathrm{S}}$, rund $3\,000\,\mathrm{kWh/a}$."),
        ("internal",
         r"From module 4: people, devices and lights are $\dot{Q}_{\mathrm{i}}$; over the season they become $Q_{\mathrm{i}}$, about $2\,000\,\mathrm{kWh/a}$.",
         r"Aus Modul 4: Personen, Geräte und Licht sind $\dot{Q}_{\mathrm{i}}$ — über die Heizperiode werden daraus $Q_{\mathrm{i}}$, rund $2\,000\,\mathrm{kWh/a}$."),
        ("total",
         r"Together they form the gross heat gain $Q_{\mathrm{g}} = Q_{\mathrm{S}} + Q_{\mathrm{i}}$ of about $5\,000\,\mathrm{kWh/a}$ — free energy we can still use.",
         r"Zusammen bilden sie den Brutto-Wärmegewinn $Q_{\mathrm{g}} = Q_{\mathrm{S}} + Q_{\mathrm{i}}$ von $5\,000\,\mathrm{kWh/a}$ — freie Energie, die wir noch nutzen können."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        SOLAR_YELLOW = PASTEL_YELLOW
        INT_ORANGE = PASTEL_ORANGE
        TEXT_WHITE = semio_color("light")
        SUBTEXT_GREY = semio_color("light-5-9")

        title = _heading(tr("Overview of heat gains", "Übersicht der Wärmegewinne"))

        subtitle = _label(tr("Free heat gains (DIN V 4108-6:2003-06)", "Freie Wärmegewinne (DIN V 4108-6:2003-06)"), size=18, color=SUBTEXT_GREY)
        subtitle.next_to(title, DOWN, buff=0.15, aligned_edge=LEFT)

        FX, SOL_Y, INT_Y = 1.3, 0.95, -1.15

        sol_room = _gain_room(np.array([0.0, SOL_Y, 0.0]))
        sun_c = np.array([-1.45, SOL_Y + 0.65, 0.0])
        sun = sun_glyph(sun_c, SOLAR_YELLOW).scale(0.3)
        rays = sun_rays(sun_c, sol_room["glass_x"], [SOL_Y + 0.25, SOL_Y], sol_room["y_f"], gap=0.42)
        rays["group"].set_color(SOLAR_YELLOW)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        sun_icon = VGroup(sun, sol_room["group"], rays["group"])

        solar_text = VGroup(
            math_label(r"\dot{Q}_{\mathrm{S}} = I_{\mathrm{s}} \cdot A \cdot g_{\perp}", size=24, color=SOLAR_YELLOW),
            math_label(r"\cdot F_{\mathrm{F}} \cdot F_{\mathrm{W}} \cdot F_{\mathrm{S}} \cdot F_{\mathrm{C}}", size=24, color=SOLAR_YELLOW),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.1)
        solar_label = _label(tr("Solar heat gain (DIN V 4108-6:2003-06)", "Solarer Wärmegewinn (DIN V 4108-6:2003-06)"), size=15, color=SOLAR_YELLOW)
        solar_label.next_to(solar_text, DOWN, aligned_edge=LEFT, buff=0.1)
        solar_slot = math_label(rf"Q_{{\mathrm{{S}}}} \approx {label_number(_Q_SOL)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", size=24)
        solar_slot.next_to(solar_label, DOWN, aligned_edge=LEFT, buff=0.18)
        solar_eq_group = VGroup(solar_text, solar_label)
        VGroup(solar_eq_group, solar_slot).move_to(np.array([FX, SOL_Y, 0.0]), aligned_edge=LEFT)
        solar_group = VGroup(sun_icon, solar_eq_group, solar_slot)

        int_room = _gain_room(np.array([0.0, INT_Y, 0.0]))
        floor_y, ceil_y = int_room["y_f"], int_room["y_c"]
        person = _cast("shared-person-winter", staffage_height("shared-person-winter") * room_metre(int_room["h"]), motion=True,
                       anchor=np.array([-0.42, floor_y + 0.01, 0.0]))
        metre = room_metre(int_room["h"])
        lamp = _pendant(np.array([0.02, ceil_y, 0.0]), metre)
        desk_y = floor_y + DESK_SURFACE_M * metre
        desk_x = int_room["x_r"] - 0.04 - 1.65 * metre
        desk = _cast("shared-desk", staffage_height("shared-desk") * metre, color=PASTEL_WHITE,
                     anchor=np.array([desk_x, floor_y, 0.0]))
        laptop_top = _cast("heating-calculation-laptop", staffage_height("heating-calculation-laptop") * metre, anchor=np.array([desk_x + 1.15 * metre, desk_y, 0.0]))
        laptop = VGroup(desk, laptop_top)
        sources = VGroup(person, lamp["group"], laptop)
        person_icon = VGroup(int_room["group"], sources)
        warm_spots = [person.get_top() + DOWN * 0.12, laptop_top.get_center() + UP * 0.04]

        int_text = math_label(
            r"\dot{Q}_{\mathrm{i}} = \dot{Q}_{\mathrm{i,p}} + \dot{Q}_{\mathrm{i,e}} + \dot{Q}_{\mathrm{i,l}}",
            size=24, color=INT_ORANGE)
        int_label = _label(tr("Internal heat gain (DIN V 4108-6:2003-06)", "Interner Wärmegewinn (DIN V 4108-6:2003-06)"), size=15, color=INT_ORANGE)
        int_label.next_to(int_text, DOWN, aligned_edge=LEFT, buff=0.1)
        int_slot = math_label(rf"Q_{{\mathrm{{i}}}} \approx {label_number(_Q_INT)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", size=24)
        int_slot.next_to(int_label, DOWN, aligned_edge=LEFT, buff=0.18)
        int_eq_group = VGroup(int_text, int_label)
        VGroup(int_eq_group, int_slot).move_to(np.array([FX, INT_Y, 0.0]), aligned_edge=LEFT)
        internal_group = VGroup(person_icon, int_eq_group, int_slot)

        def sunshine(rt):
            return [_pulses(ray_paths, rt, SOLAR_YELLOW)]

        def body_heat(rt):
            return [_heat(warm_spots, rt, r_max=0.3, color=INT_ORANGE),
                    _heat([lamp["bulb"].get_center()], rt, r_max=0.3, color=PASTEL_RED, down=True)]

        gains_vgroup = VGroup(solar_group, internal_group)
        brace = Brace(gains_vgroup, direction=LEFT, color=TEXT_WHITE, buff=0.3)

        q_gain_main = math_label(r"Q_{\mathrm{g}}", size=38, color=TEXT_WHITE)
        q_gain_sub = _label(tr("Total gross heat gain", "Brutto-Gesamtwärmegewinn"), color=SUBTEXT_GREY)
        q_gain_slot = math_label(rf"= {label_number(_Q_GAIN)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", size=28)
        q_gain_box = VGroup(q_gain_main, q_gain_sub, q_gain_slot).arrange(DOWN, buff=0.14)
        q_gain_box.next_to(brace, LEFT, buff=0.3)

        q_sol = ValueTracker(0.0)
        q_int = ValueTracker(0.0)
        q_tot = ValueTracker(0.0)
        sol_read = math_readout(lambda: rf"Q_{{\mathrm{{S}}}} \approx {label_number(q_sol.get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                                _slot(solar_slot), size=24, color=SOLAR_YELLOW)
        int_read = math_readout(lambda: rf"Q_{{\mathrm{{i}}}} \approx {label_number(q_int.get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                                _slot(int_slot), size=24, color=INT_ORANGE)
        tot_read = math_readout(lambda: rf"= {label_number(q_tot.get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                                _slot(q_gain_slot), size=28, color=TEXT_WHITE)

        self.play(Write(title), FadeIn(subtitle, shift=DOWN * 0.2), FadeIn(sol_room["group"]),
                  FadeIn(int_room["group"]), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=0.3 + 1.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_eq_group, shift=RIGHT * 0.3), run_time=1.0)
        self.play(shine(rays, lag=0.25), sol_room["air"].animate.set_fill(SOLAR_YELLOW, opacity=0.1), run_time=1.2)
        self.add(sol_read)
        self.play(q_sol.animate.set_value(_Q_SOL), *sunshine(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "solar", during=sunshine)

        def gains(rt):
            return [*sunshine(rt), *body_heat(rt)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "internal"))
        self.play(LaggedStart(*[FadeIn(s) for s in sources], lag_ratio=0.25), FadeIn(int_eq_group, shift=RIGHT * 0.3),
                  int_room["air"].animate.set_fill(INT_ORANGE, opacity=0.1), *sunshine(1.6), run_time=1.6)
        self.add(int_read)
        self.play(q_int.animate.set_value(_Q_INT), *gains(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "internal", during=gains)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        self.play(Create(brace), *gains(1.5), run_time=1.5)

        self.play(FadeIn(q_gain_main), FadeIn(q_gain_sub, shift=LEFT * 0.2), *gains(1.5), run_time=1.5)
        flying = [_freeze(sol_read).copy(), _freeze(int_read).copy()]
        self.add(tot_read, *flying)
        self.play(*[Indicate(f, color=PASTEL_YELLOW, scale_factor=1.0) for f in flying], *gains(0.6), run_time=0.6)
        self.play(*[FadeOut(f) for f in flying], q_tot.animate.set_value(_Q_GAIN), *gains(1.2), run_time=1.2)
        self.remove(*flying)

        self.play(
            q_gain_main.animate.set_color(SOLAR_YELLOW),
            brace.animate.set_color(SOLAR_YELLOW),
            *gains(0.5),
            run_time=0.5,
        )
        self.play(
            q_gain_main.animate.set_color(TEXT_WHITE),
            brace.animate.set_color(TEXT_WHITE),
            *gains(0.5),
            run_time=0.5,
        )

        hold_for(self, self.NARRATION, "total", during=gains)
        self.play(FadeOut(caption), run_time=0.3)


class Scene4(Scene):
    NARRATION = [
        ("day",
         "A clear winter day: the house loses heat around the clock, while sun and occupants deliver gains mostly at midday.",
         "Ein klarer Wintertag: Das Haus verliert rund um die Uhr Wärme, Sonne und Bewohner liefern Gewinne vor allem mittags."),
        ("overheat",
         "Gains below the loss curve replace heating. The part above overheats the room and escapes unused.",
         "Gewinne unter der Verlustkurve ersetzen Heizwärme. Der Teil darüber überhitzt den Raum und entweicht ungenutzt."),
        ("ratio",
         r"$\eta$ is the green area divided by all gains — here about $0.90$.",
         r"$\eta$ ist die grüne Fläche geteilt durch alle Gewinne — hier rund $0{,}90$."),
        ("mass",
         r"Heavier thermal mass stores the surplus and releases it in the evening: the red area shrinks and $\eta$ rises to about $0.97$.",
         r"Mehr speicherfähige Masse nimmt den Überschuss auf und gibt ihn abends ab: Die rote Fläche schrumpft, $\eta$ steigt auf rund $0{,}97$."),
        ("house",
         r"Our example house is calculated with $\eta$ of about $0.90$.",
         r"Unser Beispielhaus rechnen wir mit $\eta$ von rund $0{,}90$."),
        ("formula",
         r"So we keep only the useful share: $\eta \cdot Q_{\mathrm{g}} = \eta \cdot (Q_{\mathrm{S}} + Q_{\mathrm{i}})$, about $4\,500\,\mathrm{kWh/a}$.",
         r"Deshalb behalten wir nur den nutzbaren Anteil: $\eta \cdot Q_{\mathrm{g}} = \eta \cdot (Q_{\mathrm{S}} + Q_{\mathrm{i}})$, also $4\,500\,\mathrm{kWh/a}$."),
        ("eta",
         r"$\eta$ is the utilisation factor — how much of those free gains actually cuts the heating demand.",
         r"$\eta$ ist der Ausnutzungsgrad — wie viel dieser freien Gewinne den Heizwärmebedarf wirklich senkt."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "day"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading(tr("The utilisation factor of heat gains", "Der Ausnutzungsgrad der Wärmegewinne"))
        self.add(title)

        GREEN = PASTEL_GREEN
        RED = PASTEL_RED
        LOSS_BLUE = PASTEL_CYAN
        GAIN_YELLOW = PASTEL_YELLOW

        O = np.array([-6.1, -1.75, 0.0])
        XL, YL, PMAX = 7.2, 3.3, 5.0

        def cp(hour, kw):
            return O + RIGHT * (hour / 24.0 * XL) + UP * (kw / PMAX * YL)

        x_axis = Arrow(O, O + RIGHT * (XL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.05)
        y_axis = Arrow(O, O + UP * (YL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.08)
        x_name = math_label(r"t\;\text{in}\;\mathrm{h}", size=LABEL_FONT_SIZE).next_to(x_axis.get_end(), DOWN, buff=0.14)
        x_name.align_to(x_axis.get_end(), RIGHT)
        y_name = math_label(r"P\;\text{in}\;\mathrm{kW}", size=LABEL_FONT_SIZE).next_to(y_axis.get_end(), RIGHT, buff=0.14)
        x_ticks = VGroup(*[
            VGroup(Line(cp(h, 0) + DOWN * 0.06, cp(h, 0) + UP * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(h), cp(h, 0) + DOWN * 0.3, size=15))
            for h in (6, 12, 18)
        ])
        y_ticks = VGroup(*[
            VGroup(Line(cp(0, k) + LEFT * 0.06, cp(0, k) + RIGHT * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(k), cp(0, k) + LEFT * 0.14 + DOWN * 0.07, size=15, edge="right"))
            for k in (2, 4)
        ])
        axes = VGroup(x_axis, y_axis, x_name, y_name, x_ticks, y_ticks)

        loss = _loss_kw(_DAY_T)
        loss_curve = VMobject(color=LOSS_BLUE, stroke_width=3).set_points_smoothly(
            [cp(h, p) for h, p in zip(_DAY_T[::4], loss[::4])])
        mass = ValueTracker(0.0)
        fill_to = ValueTracker(0.0)

        def gain_now():
            return _gain_kw(_SOLAR_PEAK, mass.get_value())

        def gain_curve():
            g = gain_now()
            return VMobject(color=GAIN_YELLOW, stroke_width=3).set_points_as_corners(
                [cp(h, p) for h, p in zip(_DAY_T, g)])

        def band(upper, lower, color):
            cut = _DAY_T <= fill_to.get_value() + 1e-9
            if cut.sum() < 2:
                return VMobject()
            hs = _DAY_T[cut]
            top = [cp(h, p) for h, p in zip(hs, upper[cut])]
            bottom = [cp(h, p) for h, p in zip(hs[::-1], lower[cut][::-1])]
            return Polygon(*top, *bottom, stroke_width=0, fill_color=color, fill_opacity=0.55)

        def green_area():
            return band(np.minimum(gain_now(), loss), np.zeros_like(loss), GREEN)

        def red_area():
            g = gain_now()
            return band(np.maximum(g, loss), loss, RED)

        gains = always_redraw(gain_curve)
        green = always_redraw(green_area)
        red = always_redraw(red_area)

        hour, sky_alpha = ValueTracker(0.0), ValueTracker(0.0)

        def sky():
            h, a = hour.get_value(), sky_alpha.get_value()
            light = float(np.clip((h - 7.6) / 0.8, 0.0, 1.0) * np.clip((16.9 - h) / 0.8, 0.0, 1.0))
            arc = float(np.sin(np.pi * np.clip((h - 8.0) / 8.5, 0.0, 1.0)))
            x = cp(h, 0.0)[0]
            sun = _dim(sun_glyph(np.array([x, 2.05 + 0.35 * arc, 0.0]), GAIN_YELLOW).scale(0.24), light * a)
            moon = _dim(moon_glyph(np.array([x, 2.35, 0.0]), PASTEL_WHITE).scale(0.75), (1.0 - light) * a)
            return VGroup(moon, sun)

        def new_day(run_time: float = 0.5):
            self.play(sky_alpha.animate.set_value(0.0), run_time=run_time / 2)
            hour.set_value(0.0)
            self.play(sky_alpha.animate.set_value(1.0), run_time=run_time / 2)

        def sweep(rt):
            return [hour.animate(rate_func=linear).set_value(24.0)]

        sky_body = always_redraw(sky)

        LX = 1.75
        legend = VGroup(
            VGroup(Line(ORIGIN, RIGHT * 0.4, color=LOSS_BLUE, stroke_width=3),
                   _label(tr("Heat loss of the house", "Wärmeverlust des Hauses"), color=LOSS_BLUE)),
            VGroup(Line(ORIGIN, RIGHT * 0.4, color=GAIN_YELLOW, stroke_width=3),
                   _label(tr("Gains: sun + internal", "Gewinne: Sonne + intern"), color=GAIN_YELLOW)),
            VGroup(Square(0.22, stroke_width=0, fill_color=GREEN, fill_opacity=0.7),
                   _label(tr("used, replaces heating", "genutzt, ersetzt Heizwärme"), color=GREEN)),
            VGroup(Square(0.22, stroke_width=0, fill_color=RED, fill_opacity=0.7),
                   _label(tr("Surplus: overheats, is lost", "Überschuss: überhitzt, geht verloren"), color=RED)),
        )
        for row in legend:
            row.arrange(RIGHT, buff=0.15)
        legend.arrange(DOWN, aligned_edge=LEFT, buff=0.16).move_to(np.array([LX, 1.45, 0.0]), aligned_edge=LEFT)
        legend.shift(UP * (1.75 - legend.get_top()[1]))

        def util():
            return _utilization(_SOLAR_PEAK, mass.get_value(), fill_to.get_value())

        a_gain_read = math_readout(
            lambda: rf"Q_{{\mathrm{{g}}}} = {label_number(util()[1], 1)}\,\mathrm{{kWh}}",
            np.array([LX, -0.3, 0.0]), size=22, color=GAIN_YELLOW)
        a_used_read = math_readout(
            lambda: rf"Q_{{\mathrm{{g,n}}}} = {label_number(util()[0], 1)}\,\mathrm{{kWh}}",
            np.array([LX, -0.82, 0.0]), size=22, color=GREEN)
        eta_read = math_readout(
            lambda: rf"\eta = \frac{{Q_{{\mathrm{{g,n}}}}}}{{Q_{{\mathrm{{g}}}}}} = "
                    rf"{label_number(util()[2], 2)}",
            np.array([LX, -1.58, 0.0]), size=26, color=GREEN)
        mass_tag = _label(tr("light construction", "leichte Bauweise"), size=18, color=GREY_A).move_to(np.array([LX, 0.14, 0.0]), aligned_edge=LEFT)

        self.add(sky_body)
        self.play(FadeIn(axes), Create(loss_curve), sky_alpha.animate.set_value(1.0), run_time=1.4)
        gain_static = gain_curve()
        self.play(Create(gain_static), FadeIn(legend[:2]), run_time=1.4)
        self.remove(gain_static)
        self.add(gains)
        hold_for(self, self.NARRATION, "day", used=0.3 + 1.4 + 1.4, during=sweep)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "overheat"))
        self.add(green, red)
        self.bring_to_front(loss_curve, gains)
        self.play(FadeIn(legend[2:]), run_time=0.6)
        new_day()
        self.add(a_gain_read, a_used_read, eta_read)
        self.play(fill_to.animate.set_value(24.0), *sweep(4.5), run_time=4.5, rate_func=linear)
        hold_for(self, self.NARRATION, "overheat")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ratio"))
        self.play(Circumscribe(eta_read, color=GREEN), run_time=1.2)
        hold_for(self, self.NARRATION, "ratio")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "mass"))
        heavy_tag = _label(tr("heavy construction, high thermal mass", "schwere Bauweise, viel Speichermasse"), size=18, color=GREY_A).move_to(mass_tag, aligned_edge=LEFT)
        self.play(FadeIn(mass_tag), run_time=0.4)
        new_day()
        self.play(mass.animate.set_value(1.0), stage.transition("heating.calculation.mass.light-to-heavy-tag", source=mass_tag, target=heavy_tag), *sweep(3.5), run_time=3.5)
        hold_for(self, self.NARRATION, "mass")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "house"))
        light_tag = _label(tr("Example house, medium construction", "Beispielhaus, mittlere Bauweise"), size=18, color=GREY_A).move_to(mass_tag, aligned_edge=LEFT)
        self.play(mass.animate.set_value(0.0), stage.transition("heating.calculation.mass.heavy-to-example-tag", source=heavy_tag, target=light_tag), run_time=2.5)
        hold_for(self, self.NARRATION, "house")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        green_static = green_area()
        self.remove(green)
        self.add(green_static)
        self.play(FadeOut(VGroup(axes, loss_curve, gains, red, legend, a_gain_read, a_used_read, eta_read, light_tag)),
                  sky_alpha.animate.set_value(0.0), run_time=0.8)
        self.remove(sky_body)

        initial_eq, ie = math_row([
            ("q", r"Q_{\mathrm{g}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE), ("sol", r"Q_{\mathrm{S}}", PASTEL_YELLOW),
            ("plus", "+", PASTEL_WHITE), ("int", r"Q_{\mathrm{i}}", PASTEL_ORANGE),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING * 2)
        initial_eq.move_to(UP * 1.1)

        self.play(FadeIn(initial_eq), run_time=1.2)
        self.wait(0.6)

        target_group, tg = math_row([
            ("eta0", r"\eta", GREEN), ("dot0", r"\cdot", GREEN), ("q", r"Q_{\mathrm{g}}", PASTEL_WHITE),
            ("eq", "=", PASTEL_WHITE), ("eta", r"\eta", GREEN),
            ("dot", r"\cdot", GREEN), ("lp", "(", GREEN), ("sol", r"Q_{\mathrm{S}}", PASTEL_YELLOW),
            ("plus", "+", PASTEL_WHITE), ("int", r"Q_{\mathrm{i}}", PASTEL_ORANGE), ("rp", ")", GREEN),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING)
        target_group.move_to(UP * 1.1)

        self.play(
            stage.transition("heating.calculation.use.heat-need-symbol", source=ie["q"], target=tg["q"]),
            ie["eq"].animate.move_to(tg["eq"]),
            ie["sol"].animate.move_to(tg["sol"]),
            ie["plus"].animate.move_to(tg["plus"]),
            ie["int"].animate.move_to(tg["int"]),
            run_time=1.2,
        )

        self.play(
            stage.transition("heating.calculation.use.eta-slot", source=green_static, target=tg["eta"]),
            FadeIn(tg["dot"]), FadeIn(tg["eta0"]), FadeIn(tg["dot0"]),
            FadeIn(tg["lp"], shift=RIGHT * 0.1),
            FadeIn(tg["rp"], shift=LEFT * 0.1),
            run_time=1.6,
        )
        q_use = ValueTracker(0.0)
        use_read = math_readout(
            lambda: rf"\eta \cdot Q_{{\mathrm{{g}}}} = {label_number(_ETA, 2)} \cdot {label_number(_Q_GAIN)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}} = "
                    rf"{label_number(q_use.get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            np.array([0.0, 0.05, 0.0]), size=28, color=PASTEL_WHITE, edge="center")
        self.add(use_read)
        self.play(q_use.animate.set_value(_Q_USE), run_time=2.0)
        hold_for(self, self.NARRATION, "formula")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        eta_box = SurroundingRectangle(tg["eta"], color=GREEN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=_theme_module.CHROME_BORDER)

        eta_title = VGroup(
            math_label(r"\eta\text{:}", size=22, color=GREEN),
            _label(tr("Utilisation factor of heat gains (DIN V 4108-6:2003-06)", "Ausnutzungsgrad der Wärmegewinne (DIN V 4108-6:2003-06)"), size=22, color=GREEN),
        ).arrange(RIGHT, buff=0.12, aligned_edge=DOWN)
        eta_title.move_to(DOWN * 0.8)

        eta_line1 = _label(tr("Gives the share of heat gains (0 to 100 %)", "Gibt den Anteil der Wärmegewinne an (0 bis 100 %),"), size=18)
        eta_line2 = _label(tr("that actually contributes to covering the heating demand.", "der tatsächlich zur Deckung des Heizwärmebedarfs beiträgt."), size=18)
        eta_desc = VGroup(eta_line1, eta_line2).arrange(DOWN, buff=0.12).next_to(eta_title, DOWN, buff=0.3)

        self.play(Create(eta_box), FadeIn(eta_title), run_time=1.0)
        self.play(FadeIn(eta_desc, shift=UP * 0.15), run_time=1.0)

        hold_for(self, self.NARRATION, "eta")
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class UltimateEnergyBalance(Scene):
    NARRATION = [
        ("balance",
         "Balance: losses leave the house, useful gains stay — the difference is the heating demand.",
         "Bilanz: Verluste gehen hinaus, nutzbare Gewinne bleiben — die Differenz ist der Heizwärmebedarf."),
        ("master",
         r"Heating demand $Q_{\mathrm{h}} = Q_{\mathrm{l}} - \eta \cdot Q_{\mathrm{g}}$: $16\,000\,\mathrm{kWh/a}$ minus $4\,500\,\mathrm{kWh/a}$ is about $11\,500\,\mathrm{kWh/a}$.",
         r"Heizwärmebedarf $Q_{\mathrm{h}} = Q_{\mathrm{l}} - \eta \cdot Q_{\mathrm{g}}$: $16\,000\,\mathrm{kWh/a}$ minus $4\,500\,\mathrm{kWh/a}$ ergibt rund $11\,500\,\mathrm{kWh/a}$."),
        ("expand",
         r"Expanded: transmission $Q_{\mathrm{T}}$ plus ventilation $Q_{\mathrm{V}}$, minus $\eta$ times solar $Q_{\mathrm{S}}$ plus internal $Q_{\mathrm{i}}$.",
         r"Ausgeschrieben: Transmission $Q_{\mathrm{T}}$ plus Lüftung $Q_{\mathrm{V}}$, minus $\eta$ mal Solar $Q_{\mathrm{S}}$ plus intern $Q_{\mathrm{i}}$."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "balance"))
        self.play(FadeIn(caption), run_time=0.3)

        title = section_heading(tr("The main equation of the heating demand (DIN V 4108-6:2003-06)", "Die Hauptgleichung des Heizwärmebedarfs (DIN V 4108-6:2003-06)"))

        LOSS_C, ETA_C, GAIN_C, QH_C = PASTEL_BLUE, PASTEL_GREEN, PASTEL_YELLOW, PASTEL_RED

        house = _house(np.array([-0.3, -0.5, 0.0]), scale=0.85)
        loss_paths = _envelope_paths(house, sides=("right", "roof", "floor"), reach=1.1, inset=0.45)
        sun_c = np.array([-4.4, 1.55, 0.0])
        sun = sun_glyph(sun_c, GAIN_C).scale(0.42)
        floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
        ray_sets = [sun_rays(sun_c, w["x"], [w["center"][1] + dy for dy in (0.1, -0.1)], fy, gap=0.55)
                    for w, fy in zip(house["windows"], floors)]
        for r in ray_sets:
            r["group"].set_color(GAIN_C)
        ray_paths = [[s, h, p] for r in ray_sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]
        floor_y = house["bottom_left"][1]
        person = _cast("shared-person-winter", staffage_height("shared-person-winter") * room_metre(house["w_height"] / 2), motion=True,
                       anchor=np.array([0.0, floor_y + 0.02, 0.0]))
        metre = room_metre(house["w_height"] / 2)
        lamp = _pendant(np.array([0.4, 0.95, 0.0]), metre)
        heater = _cast("shared-radiator-thermostat-front", staffage_height("shared-radiator-thermostat-front") * metre, motion=True,
                       anchor=np.array([0.82, floor_y + 0.05, 0.0]))
        scene_house = VGroup(house["group"], house["air"], sun, *[r["group"] for r in ray_sets], person,
                             lamp["group"], heater)

        def losses(rt):
            return [_flow([(loss_paths, PASTEL_ORANGE, LOSS_C)], rt, speed=0.55, radius=0.055)]

        def gains(rt):
            return [_pulses(ray_paths, rt, GAIN_C),
                    _heat([person.get_top() + DOWN * 0.12], rt, r_max=0.4),
                    _heat([lamp["bulb"].get_center()], rt, r_max=0.4, color=QH_C, down=True)]

        def heating(rt):
            return [_heat([heater.get_top() + UP * 0.03], rt, r_max=0.45, color=QH_C)]

        def balance(rt):
            return [*losses(rt), *gains(rt), *heating(rt)]

        self.add(house["air"])
        self.play(Write(title), FadeIn(house["group"]), house["air"].animate.set_fill(opacity=0.14), run_time=1.2)

        q_loss_tag = math_label(r"Q_{\mathrm{l}}", size=26, color=LOSS_C).move_to(np.array([3.4, -0.3, 0.0]))
        loss_v = ValueTracker(0.0)
        loss_read = math_readout(lambda: rf"{label_number(round(loss_v.get_value(), -1))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                                 lambda: q_loss_tag.get_top() + UP * 0.2, size=22, color=LOSS_C, edge="center")

        self.play(FadeIn(q_loss_tag, shift=RIGHT * 0.3), *losses(0.6), run_time=0.6)
        self.add(loss_read)
        self.play(loss_v.animate.set_value(_Q_LOSS_SHOWN), *losses(1.4), run_time=1.4)

        q_gain_tag, gt = math_row([
            ("eta", r"\eta", ETA_C), ("dot", r"\cdot", PASTEL_WHITE), ("g", r"Q_{\mathrm{g}}", GAIN_C),
        ], font_size=BODY_FONT_SIZE, buff=UI_SPACING)
        q_gain_tag.move_to(np.array([-4.6, -0.6, 0.0]))
        gain_v = ValueTracker(0.0)
        gain_read = math_readout(lambda: rf"{label_number(round(gain_v.get_value(), -1))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                                 lambda: q_gain_tag.get_top() + UP * 0.2, size=22, color=GAIN_C, edge="center")

        self.play(FadeIn(sun, scale=0.7), FadeIn(person), FadeIn(lamp["group"]), *losses(0.6), run_time=0.6)
        self.play(*[shine(r, lag=0.25) for r in ray_sets], *losses(0.8), run_time=0.8)
        self.play(FadeIn(q_gain_tag, shift=RIGHT * 0.3), *losses(0.6), *gains(0.6), run_time=0.6)
        self.add(gain_read)
        self.play(gain_v.animate.set_value(_Q_USE), *losses(1.4), *gains(1.4), run_time=1.4)
        self.play(FadeIn(heater), *balance(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "balance", used=0.3 + 1.2 + 0.6 + 1.4 + 0.6 + 0.8 + 0.6 + 1.4 + 1.0,
                 during=balance)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "master"))
        master_eq, me = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE), ("loss", r"Q_{\mathrm{l}}", LOSS_C),
            ("minus", "-", PASTEL_WHITE), ("eta", r"\eta", ETA_C), ("dot", r"\cdot", PASTEL_WHITE),
            ("gain", r"Q_{\mathrm{g}}", GAIN_C),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING)
        master_eq.move_to(UP * 1.2)

        numbers, nb = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE),
            ("loss", rf"{label_number(_Q_LOSS_SHOWN)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", LOSS_C),
            ("minus", "-", PASTEL_WHITE),
            ("gain", rf"{label_number(_Q_USE)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", GAIN_C), ("approx", r"\approx", PASTEL_WHITE),
            ("slot", rf"{label_number(_Q_H)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", QH_C),
        ], font_size=BODY_FONT_SIZE + 8, buff=UI_SPACING)
        numbers.move_to(UP * 0.1)
        q_h = ValueTracker(0.0)
        qh_read = math_readout(lambda: rf"{label_number(round(q_h.get_value(), -1))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
                               _slot(nb["slot"]), size=32, color=QH_C)

        self.play(
            FadeOut(scene_house),
            stage.transition("heating.calculation.master.loss-tag-to-term", source=q_loss_tag, target=me["loss"]),
            stage.transition("heating.calculation.master.gain-tag-to-term", source=q_gain_tag, target=VGroup(me["eta"], me["dot"], me["gain"])),
            FadeIn(VGroup(me["qh"], me["eq"], me["minus"])),
            stage.transition("heating.calculation.master.loss-readout-to-value", source=_freeze(loss_read), target=nb["loss"]),
            stage.transition("heating.calculation.master.gain-readout-to-value", source=_freeze(gain_read), target=nb["gain"]),
            FadeIn(VGroup(nb["qh"], nb["eq"], nb["minus"], nb["approx"])),
            run_time=1.5,
        )
        self.add(qh_read)
        self.play(q_h.animate.set_value(_Q_H), run_time=2.0)
        hold_for(self, self.NARRATION, "master")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "expand"))
        expanded_eq, ex = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE),
            ("loss", r"(Q_{\mathrm{T}} + Q_{\mathrm{V}})", LOSS_C), ("minus", "-", PASTEL_WHITE),
            ("eta", r"\eta", ETA_C), ("dot", r"\cdot", PASTEL_WHITE),
            ("gain", r"(Q_{\mathrm{S}} + Q_{\mathrm{i}})", GAIN_C),
        ], font_size=FORMULA_FONT_SIZE, buff=UI_SPACING)
        expanded_eq.move_to(DOWN * 1.1)

        self.play(FadeIn(VGroup(ex["qh"], ex["eq"]), shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(ex["loss"], shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(VGroup(ex["minus"], ex["eta"], ex["dot"]), shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(ex["gain"], shift=UP * 0.3), run_time=0.8)

        hold_for(self, self.NARRATION, "expand")

        shown = VGroup(*[nb[k] for k in ("qh", "eq", "loss", "minus", "gain", "approx")])
        self.play(FadeOut(VGroup(title, master_eq, shown, qh_read, expanded_eq, caption)), run_time=1.5)
        self.wait(0.5)
        #endregion


#region System losses scene
BOILER_RATE = 60.0
TANK_CHARGE_S = 900.0
_HOUSE_DY = 0.22


def _plant_house():
    """🏡 Physical Fundamentals section through the example house: heated storey, roof, unheated cellar with gas boiler,
    hot-water cylinder store, room thermostat and pipes — all handcrafted staffage."""
    PIPE = PASTEL_RED
    P = lambda x, y: np.array([x, y + _HOUSE_DY, 0.0])
    slab = lambda x0, x1, y0, y1: section_material(Rectangle(width=x1 - x0, height=y1 - y0)).move_to(P((x0 + x1) / 2, (y0 + y1) / 2))
    WIN = (0.65, 1.25)
    room_h, wall = 1.6, 0.35 * 1.6 / 2.5
    metre = room_metre(room_h)
    RAD_X, PIPE_Y, TANK_X = (-6.65 + wall / 2, -1.35 - wall / 2), -0.48, -4.9
    RAD_Y = -0.15 + metre * (0.11 + 0.6 / 2)
    parts = {}
    parts["room"] = Rectangle(width=5.3 - wall, height=room_h, stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.10
                              ).move_to(P(-4.0, 0.65))
    parts["cellar"] = Rectangle(width=5.3 - wall, height=1.3, stroke_width=0, fill_color=PASTEL_BLUE, fill_opacity=0.10
                                ).move_to(P(-4.0, -0.95))
    parts["walls"] = VGroup(
        *[slab(x - wall / 2, x + wall / 2, -0.15, WIN[0]) for x in (-6.65, -1.35)],
        *[slab(x - wall / 2, x + wall / 2, WIN[1], 1.45) for x in (-6.65, -1.35)],
        *[slab(x - wall / 2, x + wall / 2, -1.6, -0.3) for x in (-6.65, -1.35)],
        slab(-6.8, -1.2, -0.3, -0.15), slab(-6.8, -1.2, -1.72, -1.6),
    )
    parts["back_wall"] = parts["walls"][0]
    parts["roof"] = section_material(Polygon(P(-7.0, 1.45), P(-4.0, 2.4), P(-1.0, 1.45), P(-1.0, 1.29), P(-4.0, 2.24), P(-7.0, 1.29)))
    windows = [window_glyph(x, WIN[0] + _HOUSE_DY, WIN[1] + _HOUSE_DY, depth=0.3, color=PASTEL_CYAN, wall=wall, inward=1 if x < -4 else -1)
               for x in (-6.65, -1.35)]
    parts["windows"] = VGroup(*[w["group"] for w in windows])
    parts["radiators"] = VGroup(*[_cast("shared-radiator-thermostat", staffage_height("shared-radiator-thermostat") * metre, motion=True, anchor=P(x, -0.15), mirror=mirror)
                                 for x, mirror in zip(RAD_X, (False, True))])
    parts["thermostat"] = _cast("heating-calculation-thermostat", staffage_height("heating-calculation-thermostat") * metre, motion=True, rate=0.0, anchor=P(-6.42, -0.15))
    level = ValueTracker(0.9)
    store = _cast("shared-tank-charge-up", staffage_height("shared-tank-charge-up") * metre, motion=True, rate=0.0, anchor=P(TANK_X, -1.6))
    store.add_updater(lambda m: m.seek(float(np.clip(level.get_value(), 0.0, 1.0)) * TANK_CHARGE_S))
    tank = {"group": store, "fill": VGroup(), "frame": store, "cap": store, "level": level}
    parts["tank"] = tank
    parts["store"] = VGroup(store)
    parts["boiler"] = _cast("shared-boiler-heating-cycle", staffage_height("shared-boiler-heating-cycle") * metre, motion=True, rate=BOILER_RATE, anchor=P(-2.85, -1.6 + 0.7 * metre))
    flue_start = parts["boiler"].get_top()
    flue_top = P(0, 2.3)[1]
    parts["chimney"] = VGroup(
        section_material(Rectangle(width=0.14, height=flue_top - flue_start[1])).move_to(np.array([flue_start[0], (flue_top + flue_start[1]) / 2, 0.0])),
        section_material(Rectangle(width=0.24, height=0.05)).move_to(np.array([flue_start[0], flue_top + 0.025, 0.0])),
    )
    parts["material"] = section_assembly(parts["walls"], parts["roof"])
    rad_in = [P(x, RAD_Y - 0.1) for x in RAD_X]
    tank_top = tank["cap"].get_top()
    boiler_out, tank_in = parts["boiler"].get_left(), tank["frame"].get_right()
    plant_link = [boiler_out, np.array([tank_in[0], boiler_out[1], 0.0]), tank_in]
    run = [rad_in[0], P(RAD_X[0], PIPE_Y), P(RAD_X[1], PIPE_Y), rad_in[1]]
    parts["pipes"] = VGroup(
        VMobject(color=PIPE, stroke_width=4).set_points_as_corners(run),
        Line(tank_top, P(TANK_X, PIPE_Y), color=PIPE, stroke_width=4),
        VMobject(color=PIPE, stroke_width=4).set_points_as_corners(plant_link),
    )
    parts["flow_paths"] = [
        VMobject().set_points_as_corners(plant_link),
        VMobject().set_points_as_corners([tank_top, P(TANK_X, PIPE_Y), P(RAD_X[0], PIPE_Y), rad_in[0]]),
        VMobject().set_points_as_corners([P(TANK_X, PIPE_Y), P(RAD_X[1], PIPE_Y), rad_in[1]]),
    ]
    parts["pipe_spots"] = [P(x, PIPE_Y - 0.04) for x in (-5.55, -4.2, -3.75, -2.05)]
    parts["sleeves"] = VGroup(
        VMobject(color=PASTEL_WHITE, stroke_width=11, stroke_opacity=0.55).set_points_as_corners(
            [P(RAD_X[0], -0.32), P(RAD_X[0], PIPE_Y), P(RAD_X[1], PIPE_Y), P(RAD_X[1], -0.32)]),
        Line(tank_top, P(TANK_X, PIPE_Y), color=PASTEL_WHITE, stroke_width=11, stroke_opacity=0.55),
    )
    parts["labels"] = VGroup(
        _label(tr("Store", "Speicher"), size=15, color=PASTEL_YELLOW).move_to(P(-4.2, -1.1)),
        VGroup(_label(tr("Heat", "Wärme-"), size=15, color=PASTEL_ORANGE), _label(tr("generator", "erzeuger"), size=15, color=PASTEL_ORANGE)
               ).arrange(DOWN, buff=0.04).next_to(parts["boiler"], RIGHT, buff=0.14),
        VGroup(_label(tr("unheated", "unbeheizter"), size=15, color=GREY_A), _label(tr("basement", "Keller"), size=15, color=GREY_A)
               ).arrange(DOWN, buff=0.06).move_to(P(-5.9, -1.15)),
        _label(f"{tr('heated', 'beheizt')}, {unit_text('20', '°C')}", size=15, color=PASTEL_ORANGE).move_to(P(-4.7, 0.95)),
    )
    parts["thermo_label"] = _label("Thermostat", size=15, color=PASTEL_WHITE).next_to(parts["thermostat"], RIGHT, buff=0.32).shift(DOWN * 0.2)
    parts["pipe_label"] = _label(tr("30 m pipe", "30 m Rohr"), size=12, color=PIPE).move_to(P(-5.62, -0.66))
    parts["floor_y"] = -0.15 + _HOUSE_DY
    parts["room_height"] = room_h
    parts["person"] = _cast("shared-person-winter", staffage_height("shared-person-winter") * metre, motion=True, anchor=P(-3.35, -0.15))
    return parts



class AnlagenVerluste(Scene):
    NARRATION = [
        ("need",
         r"The heating demand $Q_{\mathrm{h}}$ of about $11\,500\,\mathrm{kWh/a}$ is what the rooms need.",
         r"Der Heizwärmebedarf $Q_{\mathrm{h}}$ von rund $11\,500\,\mathrm{kWh/a}$ ist das, was die Räume brauchen."),
        ("chain",
         "The system must deliver more: on the way from the boiler to the room, generator, store, pipes and radiators lose heat.",
         "Die Anlage muss mehr liefern: Auf dem Weg vom Kessel in den Raum verlieren Erzeuger, Speicher, Rohre und Heizkörper Wärme."),
        ("ce",
         "Emission: the radiator also heats the outer wall behind it and the thermostat regulates sluggishly — about five percent, roughly 600 kWh.",
         "Übergabe: Der Heizkörper erwärmt auch die Außenwand dahinter, das Thermostat regelt träge — rund 5 Prozent, also etwa 600 kWh."),
        ("d",
         "Distribution: 30 metres of pipe in the cold cellar lose 10 watts per metre over 3,000 hours, 900 kWh.",
         "Verteilung: 30 Meter Rohr im kalten Keller verlieren 10 Watt pro Meter über 3 000 Stunden — 900 kWh."),
        ("s",
         "Storage: the buffer store loses 1.8 kilowatt-hours per day on standby, over 200 heating days — 360 kilowatt-hours.",
         "Speicherung: Der Pufferspeicher verliert im Bereitschaftsbetrieb 1,8 kWh pro Tag, über 200 Heiztage 360 kWh."),
        ("g",
         "Generation: with an efficiency of 0.95, part of the fuel energy leaves through the flue, about 700 kWh.",
         "Erzeugung: Bei einem Wirkungsgrad von 0,95 geht ein Teil als Abgas durch den Schornstein — rund 700 kWh."),
        ("sum",
         r"Final energy $Q_{\mathrm{E}}$ is $Q_{\mathrm{h}}$ plus all system losses: about $14\,000\,\mathrm{kWh/a}$, after DIN/TS 18599-5.",
         r"Die Endenergie $Q_{\mathrm{E}}$ ist $Q_{\mathrm{h}}$ plus alle Anlagenverluste: rund $14\,000\,\mathrm{kWh/a}$, nach DIN/TS 18599-5."),
        ("tip",
         "So keep pipes and stores inside the heated envelope and insulate the pipes.",
         "Darum gehören Rohre und Speicher in die beheizte Hülle, und die Leitungen werden gedämmt."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        caption = caption_bar(subtitle_text(N, "need"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading(tr("System losses: from heating demand to final energy", "Anlagenverluste: vom Heizwärmebedarf zur Endenergie"))
        self.play(Write(title), run_time=0.8)

        QH_C, CE_C, D_C, S_C, G_C, E_C = PASTEL_RED, PASTEL_PINK, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_PURPLE, PASTEL_WHITE
        h = _plant_house()
        person = h["person"]
        rad_l = h["radiators"][0]
        glow = h["back_wall"].copy().set_stroke(width=0).set_fill(QH_C, opacity=0.0)
        wall_paths = [Line(np.array([rad_l.get_left()[0] - 0.02, y, 0.0]), np.array([-7.0, y, 0.0]))
                      for y in np.linspace(rad_l.get_bottom()[1] + 0.06, rad_l.get_top()[1] - 0.04, 3)]
        sc = h["tank"]["frame"].get_center()
        flue = [smooth_path([h["boiler"].get_top(), np.array([-2.85, 1.0 + _HOUSE_DY, 0.0]),
                             np.array([-2.85, 2.3 + _HOUSE_DY, 0.0]), np.array([-2.65, 2.6 + _HOUSE_DY, 0.0])])]
        on = set()

        def running(rt):
            anims = [_heat([person.get_top() + DOWN * 0.12], rt, r_max=0.3)]
            if "water" in on:
                anims.append(_flow([(h["flow_paths"], PASTEL_RED)], rt, speed=0.5, waves=6, radius=0.05))
            if "ce" in on:
                anims += [_flow([(wall_paths, PASTEL_RED, PASTEL_BLUE)], rt, speed=0.8, radius=0.045),
                          _heat([rad_l.get_left() + RIGHT * 0.02], rt, r_max=0.28, color=QH_C, facing=PI)]
            if "d" in on:
                anims.append(_heat(h["pipe_spots"], rt, r_max=0.24, color=D_C, rings=2, down=True))
            if "s" in on:
                anims += [_heat([sc + LEFT * 0.27], rt, r_max=0.24, color=S_C, rings=2, facing=PI),
                          _heat([sc + RIGHT * 0.27], rt, r_max=0.24, color=S_C, rings=2, facing=0.0)]
            if "g" in on:
                anims.append(_flow([(flue, PASTEL_ORANGE, PASTEL_GREY)], rt, speed=0.6, waves=6, radius=0.075))
            return anims

        BASE = np.array([0.35, -1.5, 0.0])
        BAR_W = 0.7
        UNIT = 3.5 / _Q_E
        vals = [ValueTracker(0.0) for _ in range(5)]
        colors = [QH_C, CE_C, D_C, S_C, G_C]

        def bar():
            group = VGroup()
            cursor = 0.0
            for v, c in zip(vals, colors):
                height = v.get_value() * UNIT
                if height > 1e-4:
                    group.add(Rectangle(width=BAR_W, height=height, stroke_width=1, stroke_color=c, fill_color=c,
                                        fill_opacity=0.8).move_to(BASE + UP * (cursor + height / 2)))
                cursor += height
            return group

        finals = [_Q_H, _Q_CE, _Q_D, _Q_S, _Q_G]
        mids = []
        cursor = 0.0
        for f in finals:
            mids.append(BASE + UP * (cursor + f * UNIT / 2) + RIGHT * BAR_W / 2)
            cursor += f * UNIT
        bar_live = always_redraw(bar)
        bar_floor = Line(BASE + LEFT * 0.55, BASE + RIGHT * 0.55, color=PASTEL_WHITE, stroke_width=2)

        CX = 1.55
        rows_y = [-0.95, -0.3, 0.32, 0.94, 1.56, 2.18]
        names = [tr("Heating demand", "Heizwärmebedarf"), tr("Emission: radiators, thermostat", "Übergabe: Heizkörper, Thermostat"), tr("Distribution: pipes in the basement", "Verteilung: Rohre im Keller"),
                 tr("Storage: standby", "Speicherung: Bereitschaft"), tr("Generation: flue gas, efficiency 0.95", "Erzeugung: Abgas, Wirkungsgrad 0,95"), tr("Final energy", "Endenergie")]
        row_colors = colors + [E_C]
        name_mobs = [_label(n, size=15, color=c).move_to(np.array([CX, y + 0.2, 0.0]), aligned_edge=LEFT)
                     for n, y, c in zip(names, rows_y, row_colors)]
        e_val = ValueTracker(0.0)
        formulas = [
            lambda: rf"Q_{{\mathrm{{h}}}} \approx {label_number(vals[0].get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            lambda: rf"Q_{{\mathrm{{h,ce}}}} = 5\,\% \cdot {label_number(_Q_H)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}} \approx {label_number(vals[1].get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            lambda: rf"Q_{{\mathrm{{h,d}}}} = {label_number(_PIPE_M)}\,\mathrm{{m}} \cdot {label_number(_PIPE_W_PER_M)}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m}}}} \cdot {label_number(_PIPE_H)}\,\mathrm{{h}} = "
                    rf"{label_number(vals[2].get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            lambda: rf"Q_{{\mathrm{{h,s}}}} = {label_number(1.8, 1)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{d}}}} \cdot 200\,\mathrm{{d}} = "
                    rf"{label_number(vals[3].get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            lambda: rf"Q_{{\mathrm{{h,g}}}} = {label_number(_Q_BEFORE_G)}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}} \cdot (1/{label_number(0.95, 2)} - 1) \approx "
                    rf"{label_number(vals[4].get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
            lambda: rf"Q_{{\mathrm{{E}}}} \approx {label_number(round(e_val.get_value(), -2))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}",
        ]
        readouts = [math_readout(fn, np.array([CX, y - 0.16, 0.0]), size=19, color=c)
                    for fn, y, c in zip(formulas, rows_y, row_colors)]
        leaders = [Line(np.array([CX - 0.1, y + 0.05, 0.0]), mid, color=c, stroke_width=1.4, stroke_opacity=0.6)
                   for y, mid, c in zip(rows_y[:5], mids, colors)]

        self.play(FadeIn(h["room"]), FadeIn(h["cellar"]), FadeIn(h["material"]),
                  FadeIn(h["windows"]), FadeIn(h["labels"][3]), FadeIn(person), Create(bar_floor), run_time=1.2)
        self.add(bar_live, readouts[0])
        self.play(FadeIn(name_mobs[0]), Create(leaders[0]), vals[0].animate.set_value(_Q_H),
                  h["room"].animate.set_fill(opacity=0.22), *running(2.0), run_time=2.0)
        hold_for(self, N, "need", used=0.3 + 0.8 + 1.2 + 2.0, during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "chain"))
        self.play(FadeIn(h["store"]), FadeIn(h["boiler"]), FadeIn(h["chimney"]), Create(h["pipes"]),
                  FadeIn(h["radiators"]), FadeIn(h["thermostat"]), FadeIn(h["labels"][:3]), *running(1.5),
                  run_time=1.5)
        on.add("water")
        self.play(*running(3.0), run_time=3.0)
        hold_for(self, N, "chain", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "ce"))
        self.add(glow)
        self.add(readouts[1])
        self.play(FadeIn(name_mobs[1]), FadeIn(h["thermo_label"]), Create(leaders[1]), *running(0.5), run_time=0.5)
        on.add("ce")
        self.play(vals[1].animate.set_value(_Q_CE), glow.animate.set_fill(opacity=0.55),
                  Wiggle(h["thermostat"], scale_value=1.4, n_wiggles=4), *running(3.0), run_time=3.0)
        hold_for(self, N, "ce", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "d"))
        self.add(readouts[2])
        self.play(FadeIn(name_mobs[2]), FadeIn(h["pipe_label"]), Create(leaders[2]), *running(0.5), run_time=0.5)
        on.add("d")
        self.play(vals[2].animate.set_value(_Q_D), *running(3.0), run_time=3.0)
        hold_for(self, N, "d", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "s"))
        self.add(readouts[3])
        self.play(FadeIn(name_mobs[3]), Create(leaders[3]), *running(0.5), run_time=0.5)
        on.add("s")
        self.play(vals[3].animate.set_value(_Q_S), h["tank"]["level"].animate.set_value(0.75), *running(3.0),
                  run_time=3.0)
        hold_for(self, N, "s", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        self.add(readouts[4])
        self.play(FadeIn(name_mobs[4]), Create(leaders[4]), *running(0.5), run_time=0.5)
        on.add("g")
        self.play(vals[4].animate.set_value(_Q_G), *running(3.0),
                  run_time=3.0)
        hold_for(self, N, "g", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        row, box, items = math_panel([
            ("e", r"Q_{\mathrm{E}}", E_C), (None, "=", PASTEL_WHITE), ("h", r"Q_{\mathrm{h}}", QH_C), (None, "+", PASTEL_WHITE),
            ("ce", r"Q_{\mathrm{h,ce}}", CE_C), (None, "+", PASTEL_WHITE), ("d", r"Q_{\mathrm{h,d}}", D_C),
            (None, "+", PASTEL_WHITE), ("s", r"Q_{\mathrm{h,s}}", S_C), (None, "+", PASTEL_WHITE),
            ("g", r"Q_{\mathrm{h,g}}", G_C), (None, r"\approx", PASTEL_WHITE),
            ("v", rf"{label_number(round(_Q_E, -2))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}", E_C),
        ], color=P_PRIMARY)
        sources = {
            key: math_label(src, _slot(r), size=21, color=c, edge="left")
            for key, src, r, c in zip(
                ("h", "ce", "d", "s", "g"),
                (r"Q_{\mathrm{h}}", r"Q_{\mathrm{h,ce}}", r"Q_{\mathrm{h,d}}", r"Q_{\mathrm{h,s}}", r"Q_{\mathrm{h,g}}"),
                readouts[:5], colors)
        }
        self.add(readouts[5])
        self.play(FadeIn(name_mobs[5]), e_val.animate.set_value(_Q_E), *running(1.8), run_time=1.8)
        copies = {k: s.copy() for k, s in sources.items()}
        for c in copies.values():
            for part in c.get_family():
                part._layout_zone = "formula"
        self.play(stage.transition("heating.calculation.system.readouts-into-sum", source=copies, target={k: items[k] for k in copies}), *running(1.2), run_time=1.2)
        rest = VGroup(*[m for m in row.submobjects if all(m is not items[k] for k in copies)])
        self.play(FadeIn(rest), Create(box), *running(0.6), run_time=0.6)
        hold_for(self, N, "sum", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "tip"))
        on.discard("d")
        self.play(Create(h["sleeves"]), FadeOut(h["pipe_label"]), *running(1.2), run_time=1.2)
        hold_for(self, N, "tip", during=running)
        self.play(FadeOut(caption), run_time=0.3)
        #endregion
#endregion


class FullFinalCalculationVideo(Scene):
    def construct(self):
        scenes = [
            ReviewingHeatLosses,
            Scene2,
            ReviewingHeatGains,
            Scene4,
            UltimateEnergyBalance,
            AnlagenVerluste,
        ]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3") for i in range(1, len(scenes) + 1)
        ]

        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            self.NARRATION = scene_cls.NARRATION
            begin_vo_beat(self, scene_cls.__name__)
            scene_cls.construct(self)
            self.clear()
