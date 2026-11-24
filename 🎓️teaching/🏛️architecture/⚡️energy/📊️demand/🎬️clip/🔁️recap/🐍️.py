

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
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
SAFE_TOP = _manim_module.SAFE_TOP
CONTENT_TOP = _manim_module.CONTENT_TOP
chip = _manim_module.chip
stacked_bar = _manim_module.stacked_bar
side_labels = _manim_module.side_labels
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
flow_animation = _manim_module.flow_animation
house_section = _manim_module.house_section
staffage = _manim_module.staffage
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
sun_glyph = _manim_module.sun_glyph
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
pulse_flashes = _manim_module.pulse_flashes
ripples = _manim_module.ripples
open_window = _manim_module.open_window
thermometer_glyph = _manim_module.thermometer_glyph
clock_glyph = _manim_module.clock_glyph
meter = _manim_module.meter
set_meter = _manim_module.set_meter
bind_meter = _manim_module.bind_meter
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
label_number = _manim_module.label_number
dim_arrow = _manim_module.dim_arrow
highlight_param = _manim_module.highlight_param
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
load_vo_timing = _manim_module.load_vo_timing

# 🗣️ Measured clause durations live in vo_timing.json.
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ One persistent chapter title — written once on Beat 1, re-added on later beats.
TITLE_DE = "Die Energiebilanz des Gebäudes"

# Carried over unchanged from the Ventilation, Heating and Cooling series so a viewer who
# watched those recognises each flow by its colour before reading its label.
LOSS_BLUE = semio_color("secondary")     # Transmission — 3_heating/2_conduction
VENT_BLUE = semio_color("secondary")     # Lüftung — 2_ventilation/
SOLAR_YELLOW = semio_color("warning")  # Solare Gewinne — 3_heating/5_solar_heat_gain
INT_ORANGE = semio_color("tertiary")    # Interne Gewinne — 3_heating/4_internal_heat_gain
ETA_GREEN = semio_color("success")     # Ausnutzungsgrad
DEMAND_RED = semio_color("primary")    # Was die Anlage liefern muss

HOUSE_SCALE = 0.8


#region Example house
# One set of numbers for the whole chapter, taken from the running example (areas, U-values, volume,
# air change, heat capacity of air), so every bar and readout below is a product of drawn quantities.
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
SURFACE_SLUGS = ("wall", "roof", "windows", "floor")
_ENVELOPE = {name: (area, u) for name, area, u, _delta in _example.ENVELOPE}
_GROUND_DELTA = {name: delta for name, _a, _u, delta in _example.ENVELOPE}["Boden"]
_SURFACE_COLORS = (("Außenwand", LOSS_BLUE), ("Dach", semio_color("info")), ("Fenster", P_CYAN), ("Boden", P_TEAL))


def _surfaces_at(d_theta: float):
    """🧱 The four envelope surfaces at an indoor-outdoor difference; the floor always sees the ground at its own difference."""
    return [(n, _ENVELOPE[n][0], _ENVELOPE[n][1], _GROUND_DELTA if n == "Boden" else d_theta, c) for n, c in _SURFACE_COLORS]


def _eta_heat(gamma: float, a: float = 2.0) -> float:
    """🔋 Utilisation factor of the gains η = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_g / Q_l (DIN V 4108-6, 6.5)."""
    return a / (a + 1.0) if abs(gamma - 1.0) < 1e-9 else (1.0 - gamma ** a) / (1.0 - gamma ** (a + 1.0))


def _eta_cool(gamma: float, a: float = 2.0) -> float:
    """❄️ Utilisation factor of the losses η_c = (1 − γ⁻ᵃ) / (1 − γ⁻⁽ᵃ⁺¹⁾) with γ = Q_g / Q_l."""
    return a / (a + 1.0) if abs(gamma - 1.0) < 1e-9 else (1.0 - gamma ** -a) / (1.0 - gamma ** -(a + 1.0))


D_THETA_MONTH, D_THETA_JAN, D_THETA_JUL = 10.0, _example.D_THETA_WINTER_K, 6.0
D_THETA = D_THETA_MONTH
SURFACES = _surfaces_at(D_THETA_MONTH)
A_HUELLE = sum(a for _n, a, _u, _d, _c in SURFACES)
DU_WB = 0.05
V_NET, N_AIR, RHO_C = _example.V_NET_M3, _example.N_AIR_PER_H, _example.C_LUFT_WH_M3_K
T_MONTH = 744.0
PHI_SURFACES = [u * a * d for _n, a, u, d, _c in SURFACES]
PHI_WB = DU_WB * A_HUELLE * D_THETA
PHI_V = V_NET * N_AIR * RHO_C * D_THETA
BAR_UNIT = 3.0 / (sum(PHI_SURFACES) + PHI_WB + PHI_V)


def _loss_kwh(d_theta: float) -> float:
    """📉 Monthly losses Q_l = Q_T + Q_V in kWh."""
    phi = sum(u * a * d for _n, a, u, d, _c in _surfaces_at(d_theta)) + DU_WB * A_HUELLE * d_theta + V_NET * N_AIR * RHO_C * d_theta
    return phi * T_MONTH / 1000.0


A_WINDOW, G_VALUE, F_S = _ENVELOPE["Fenster"][0], _example.G_PERP, _example.F_S_WINTER
F_C, F_F, F_W = _example.F_C_NONE, _example.F_F, _example.F_W
I_S, I_S_JAN, I_S_JUL = 65.0, 26.0, 54.0
Q_I_SPEC, A_NET = 5.0, 60.0
Q_INTERN = Q_I_SPEC * A_NET * T_MONTH / 1000.0


def _solar_kwh(i_s: float) -> float:
    """☀️ Solar gains Q_S = A · I_s · g⊥ · F_S · F_C · F_F · F_W in kWh, every factor from the running example."""
    return A_WINDOW * G_VALUE * F_S * F_C * F_F * F_W * i_s


Q_SOLAR = _solar_kwh(I_S)
Q_L, Q_G = _loss_kwh(D_THETA_MONTH), Q_SOLAR + Q_INTERN
ETA_H = _eta_heat(Q_G / Q_L)
Q_USABLE = ETA_H * Q_G
Q_L_JAN, Q_G_JAN = _loss_kwh(D_THETA_JAN), _solar_kwh(I_S_JAN) + Q_INTERN
ETA_JAN = _eta_heat(Q_G_JAN / Q_L_JAN)
Q_USABLE_JAN = ETA_JAN * Q_G_JAN
Q_L_JUL, Q_G_JUL = _loss_kwh(D_THETA_JUL), _solar_kwh(I_S_JUL) + Q_INTERN
ETA_C = _eta_cool(Q_G_JUL / Q_L_JUL)
#endregion


#region Shared visual motifs
# The through-line house: the Physical Fundamentals section every Heating and
# Cooling video draws, now carrying the four balance flows across one dashed
# envelope. Every beat of this chapter builds on the same geometry.

def _pts(*coords):
    """📍 Points on the z = 0 plane."""
    return [np.array([x, y, 0.0]) for x, y in coords]


def _home(center, scale: float = HOUSE_SCALE):
    """🏠 Section house plus the geometry every flow, person and label hangs on."""
    center = np.array(center, dtype=float)
    hs = house_section(center, scale)
    bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
    eave_l, eave_r = tl + LEFT * 0.26 * scale, tr + RIGHT * 0.26 * scale
    inner_peak = peak + DOWN * 0.12
    air = Polygon(
        bl + RIGHT * 0.08 + UP * 0.02, br + LEFT * 0.08 + UP * 0.02, tr + LEFT * 0.08 + DOWN * 0.06,
        inner_peak, tl + RIGHT * 0.08 + DOWN * 0.06,
        stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.0,
    ).set_z_index(-1)
    return {
        "hs": hs, "group": hs["group"], "air": air, "scale": scale, "cx": float(center[0]),
        "x_l": float(bl[0]), "x_r": float(br[0]), "floor_y": float(bl[1]),
        "mid_y": float(hs["level_1"].get_center()[1]), "top_y": float(tl[1]),
        "peak": peak, "eave_l": eave_l, "eave_r": eave_r,
        "lo": hs["windows"][0], "up": hs["windows"][1],
    }


def _roof(H, side: str, u: float):
    """📐 Point on a roof slope (``u`` runs eave → ridge) and the unit normal pointing out of the house."""
    eave = H["eave_l"] if side == "l" else H["eave_r"]
    p = eave + (H["peak"] - eave) * u
    d = (H["peak"] - eave) if side == "l" else (eave - H["peak"])
    d = d / np.linalg.norm(d)
    return p, np.array([-d[1], d[0], 0.0])


def _envelope(H, off: float = 0.26, color=P_PRIMARY):
    """⭕ Dashed thermal envelope hugging the house — the boundary every balance flow crosses."""
    nl, nr = _roof(H, "l", 0.0)[1], _roof(H, "r", 0.0)[1]
    bottom = H["floor_y"] - 0.22
    corners = [
        np.array([H["x_l"] - off, bottom, 0.0]), np.array([H["x_l"] - off, H["top_y"], 0.0]),
        H["eave_l"] + nl * off, H["peak"] + UP * off * 1.15, H["eave_r"] + nr * off,
        np.array([H["x_r"] + off, H["top_y"], 0.0]), np.array([H["x_r"] + off, bottom, 0.0]),
    ]
    return DashedVMobject(Polygon(*corners, color=color, stroke_width=2.6), num_dashes=70, dashed_ratio=0.55)


def _metre(H) -> float:
    """🧮 Manim units per real metre at the storey scale."""
    return room_metre(H["mid_y"] - H["floor_y"])


def _stand(H, x: float):
    """🧍 Breathing thermal occupant at true scale, feet on the slab's top edge at ``x``."""
    return staffage_motion("shared-person-winter", unit=_metre(H), anchor=np.array([x, H["floor_y"], 0.0]))


def _heater(H):
    """♨️ Thermal panel radiator seen from the front at the same scale as the storey, like the house sections of primary energy and practice."""
    return staffage("shared-radiator-front-winter", unit=_metre(H), anchor=np.array([H["x_r"] - 0.8, H["floor_y"], 0.0]))


def _gains(H):
    """💡 Occupant, pendant lamp, desk and laptop — the internal sources inside the house."""
    cx, fy, my = H["cx"], H["floor_y"], H["mid_y"]
    unit = _metre(H)
    person = _stand(H, cx + 0.05)
    lamp_group = staffage("shared-lamp-winter", unit=unit)
    lamp_group.shift(np.array([cx - 0.9, my, 0.0]) - lamp_group.get_top())
    desk_x = H["hs"]["x_l"] + 0.3 * unit
    desk = staffage("shared-desk", unit=unit, anchor=np.array([desk_x, fy, 0.0]))
    laptop = staffage("shared-laptop-winter", unit=unit, anchor=np.array([desk_x + 0.3 * unit, fy + 0.72 * unit, 0.0]))
    return {
        "person": person, "lamp": lamp_group, "device": VGroup(desk, laptop),
        "sources": VGroup(person, lamp_group, desk, laptop),
        "warm": [person.get_center() + UP * 0.08, laptop.get_center()],
        "bulbs": [lamp_group.get_bottom() + UP * 0.06],
    }


def _gain_heat(G, rt: float, *, color=INT_ORANGE, r_max: float = 0.42):
    """🔥 Long-wave heat from every internal source — ripples from person and laptop, down from the lamp."""
    cycles = max(1.0, rt / 1.3)
    return [ripples(G["warm"], r_max=r_max, color=color, cycles=cycles),
            ripples(G["bulbs"], r_max=r_max * 0.6, color=color, down=True, cycles=cycles)]


def _paths(H, reach: float = 1.15):
    """🧭 Guide curves of the balance flows — each one crosses the envelope exactly where its physics does."""
    xl, xr, fy, my = H["x_l"], H["x_r"], H["floor_y"], H["mid_y"]
    lo_y, up_y = H["lo"]["center"][1], H["up"]["center"][1]
    walls = [smooth_path(_pts((xr - 0.6, y - 0.04), (xr, y), (xr + reach * 0.5, y + 0.04), (xr + reach, y + 0.1)))
             for y in (my - 0.2, my + 0.68)]
    roofs = []
    for u in (0.3, 0.72):
        p, n = _roof(H, "r", u)
        roofs.append(smooth_path([p - n * 0.6, p - n * 0.1, p + n * 0.4, p + n * 0.8]))
    pl, nl = _roof(H, "l", 0.55)
    return {
        "trans": walls + roofs, "wall": walls, "roof": roofs,
        "win": [smooth_path(_pts((xl + 0.55, y), (xl, y + 0.02), (xl - reach * 0.5, y - 0.02), (xl - reach, y + 0.06)))
                for y in (lo_y, up_y)],
        "floor": [smooth_path(_pts((x, fy + 0.4), (x, fy + 0.05), (x + 0.04, fy - 0.18), (x, fy - 0.38)))
                  for x in (H["cx"] - 0.5, H["cx"] + 0.5)],
        "vent_in": [smooth_path(_pts((xl - reach, lo_y - 0.1), (xl - reach * 0.55, lo_y - 0.03), (xl, lo_y),
                                     (xl + 0.5, lo_y - 0.12), (xl + 0.9, lo_y - 0.4)))],
        "vent_out": [smooth_path([pl - nl * 0.85, pl - nl * 0.2, pl + nl * 0.4, pl + nl * 0.8])],
        "supply": [smooth_path(_pts((xr + reach, fy + 0.2), (xr + reach * 0.5, fy + 0.2), (xr, fy + 0.2), (xr - 0.2, fy + 0.2)))],
    }


def _stream(paths, color, rt: float, *, color_end=None, period: float = 1.5, waves: int = 3):
    """💨 One coloured particle stream for ``rt`` seconds."""
    return flow_animation([(list(paths), color, color_end)], waves=waves, radius=0.06, cycles=max(1.0, rt / period))


def _solar(H, sun_c, windows=("up",)):
    """☀️ Sun plus parallel straight rays through the chosen windows onto the floor behind each."""
    lands = {"up": H["mid_y"], "lo": H["floor_y"]}
    sets = [sun_rays(sun_c, H[k]["x"], [H[k]["center"][1] + 0.09, H[k]["center"][1] - 0.09], lands[k], gap=0.5)
            for k in windows]
    return {"sun": sun_glyph(sun_c).scale(0.42), "sets": sets, "group": VGroup(*[r["group"] for r in sets]),
            "paths": [[s, h, p] for r in sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]}


def _shine(S, lag: float = 0.25):
    """🔦 Draw every ray of the sun, outside the glass first, then on through it."""
    return AnimationGroup(*[shine(r, lag=lag) for r in S["sets"]])


def _sunshine(S, rt: float):
    """⚡ Light pulses running down the rays while a subtitle is read."""
    return pulse_flashes(S["paths"], SOLAR_YELLOW, repeats=max(1, int(rt / 1.5)))


def _overhangs(H):
    """🪟 Exterior shading slats above each window — the outer end lower than the wall end."""
    return VGroup(*[Line(np.array([w["x"], w["lintel"] + 0.05, 0.0]), np.array([w["x"] - 0.42, w["lintel"] - 0.03, 0.0]),
                         color=P_WHITE, stroke_width=3.2) for w in (H["lo"], H["up"])])


def _tag(name: str, sym: str, color):
    """🏷️ Flow label — a short word plus the balance symbol that later lands in the formula."""
    word = body_text(name, font_size=LABEL_FONT_SIZE, color=color)
    symbol = math_label(sym, size=BODY_FONT_SIZE, color=color)
    return VGroup(word, symbol).arrange(RIGHT, buff=0.14)


def _sym(sym: str, color):
    """🔣 Bare balance symbol that travels from its flow into the formula."""
    return math_label(sym, size=BODY_FONT_SIZE, color=color)


def _lead(tag, point, *, side: str, color):
    """➖ Thin leader from a tag's inner edge to the physical element it names."""
    start = tag.get_right() + RIGHT * 0.08 if side == "left" else tag.get_left() + LEFT * 0.08
    return Line(start, np.array(point, dtype=float), color=color, stroke_width=1.4, stroke_opacity=0.55)


def _merge(stage, transition, sources, target):
    """🔀 Morph several flow symbols into one formula term — the first becomes it, the rest fold into it and are returned to remove."""
    anims = [stage.transition(transition, source=sources[0], target=target)]
    anims += [src.animate.move_to(target).scale(0.4).set_opacity(0) for src in sources[1:]]
    return anims, list(sources[1:])


def _surfaces(H):
    """🧱 Heavy colour overlays on the four envelope surfaces — the areas that become U · A."""
    hs = H["hs"]

    def thick(mob, color):
        return mob.copy().set_color(color).set_fill(opacity=0.85).set_stroke(width=1.5, opacity=0.95)

    colors = {n: c for n, _a, _u, _d, c in SURFACES}
    return {
        "Außenwand": VGroup(*[thick(hs["walls"][i], colors["Außenwand"]) for i in (0, 1)]),
        "Dach": thick(hs["roof"][0], colors["Dach"]),
        "Fenster": VGroup(*[Line(np.array([w["x"], w["sill"], 0.0]), np.array([w["x"], w["lintel"], 0.0]),
                                 color=colors["Fenster"], stroke_width=7.5) for w in hs["windows"]]),
        "Boden": thick(hs["floor"], colors["Boden"]),
    }


def _card(title: str, color, specs, *, x: float = 4.5, top: float = 1.85, width: float = 2.3, gap: float = 0.44):
    """🪪 Factor card — a framed title over live readouts that count up from the physical picture."""
    frame = RoundedRectangle(
        width=width, height=0.66 + gap * len(specs), corner_radius=0, color=color, stroke_width=_theme_module.CHROME_BORDER,
        fill_color=_theme_module.CHROME_FILL, fill_opacity=0.85,
    )
    frame.move_to(np.array([x + width / 2, top - frame.height / 2, 0.0])).set_z_index(-2)
    head_at = np.array([x + width / 2, top - 0.3, 0.0])
    rows = [math_readout(fn, np.array([x + 0.18, top - 0.62 - gap * (k + 0.55), 0.0]), size=LABEL_FONT_SIZE,
                         color=c, edge="left") for k, (fn, c) in enumerate(specs)]
    return {"frame": frame, "head_at": head_at, "rows": rows}


def _gauge(sym: str, color, at, tracker, *, length: float = 1.3, digits: int = 2):
    """📏 Horizontal 0–1 factor gauge — a typeset symbol, a track that fills with the tracker and its live value."""
    g = meter("x", length=length, thickness=0.2, color=color, vertical=False)
    g["track"].move_to(np.array(at, dtype=float))
    set_meter(g, 0.0)
    bind_meter(g, tracker)
    label = math_label(sym, size=BODY_FONT_SIZE, color=color).next_to(g["track"], LEFT, buff=0.14)
    read = math_readout(lambda: label_number(tracker.get_value(), digits),
                        np.array([g["track"].get_right()[0] + 0.14, float(at[1]) - 0.08, 0.0]),
                        size=LABEL_FONT_SIZE + 1, color=color, edge="left")
    return {"track": g["track"], "fill": g["fill"], "label": label, "read": read,
            "static": VGroup(g["track"], g["fill"], label)}


def _thermo_pair(x: float, bottom: float, *, height: float = 1.5, gap: float = 0.8):
    """🌡️ Indoor and outdoor thermometer — the Δθ span is the gap between their two column tops, read live."""
    t_in = thermometer_glyph(np.array([x, bottom, 0.0]), height=height, color=INT_ORANGE, level=0.0)
    t_out = thermometer_glyph(np.array([x + gap, bottom, 0.0]), height=height, color=LOSS_BLUE, level=0.0)
    names = VGroup(body_text(tr("inside", "innen"), font_size=LABEL_FONT_SIZE, color=INT_ORANGE),
                   body_text(tr("outside", "außen"), font_size=LABEL_FONT_SIZE, color=LOSS_BLUE))
    names[0].move_to(np.array([x, bottom - 0.33, 0.0]))
    names[1].move_to(np.array([x + gap, bottom - 0.33, 0.0]))
    span = height - 0.12

    def top(t):
        return bottom + 0.05 + span * float(np.clip(t["level"].get_value(), 0.0, 1.0))

    def d_theta():
        return 40.0 * (t_in["level"].get_value() - t_out["level"].get_value())

    arrow = always_redraw(lambda: dim_arrow(
        np.array([x + gap / 2, top(t_out), 0.0]), np.array([x + gap / 2, top(t_out) + max(0.06, top(t_in) - top(t_out)), 0.0]),
        color=P_BLUE))
    return {"in": t_in, "out": t_out, "names": names, "arrow": arrow, "d_theta": d_theta, "x_mid": x + gap / 2,
            "static": VGroup(t_in["group"], t_out["group"], names), "top": top, "bottom": bottom}


def _veil(row):
    """🫥 Hide every piece of a typeset formula row so it can be revealed term by term."""
    row.set_opacity(0)
    return row


def _terms(row, lo: int, hi: int):
    """🔡 Reveal formula pieces ``lo`` to ``hi`` (exclusive) of a veiled row, in reading order."""
    return [m.animate.set_opacity(1) for m in row.submobjects[lo:hi]]
#endregion


#region Beat 1 – Bilanzgrenze & Energieerhaltung

class Beat1_Bilanzgrenze(Scene):
    NARRATION = [
        ("intro",
         r"In three videos we calculated single heat flows. Now we assemble them into one balance.",
         r"In drei Videos haben wir einzelne Wärmeströme berechnet. Jetzt fügen wir sie zu einer Bilanz zusammen."),
        ("huelle",
         r"A balance needs a boundary: the thermal envelope.",
         r"Eine Bilanz braucht eine Grenze: die thermische Hülle."),
        ("fluesse",
         r"Everything that crosses it over the year is an energy flow we can add up.",
         r"Alles, was sie übers Jahr kreuzt, ist ein Energiefluss, den wir aufsummieren."),
        ("erhaltung",
         r"Energy is conserved — so at a constant indoor temperature, everything that flows out must be replaced.",
         r"Energie bleibt erhalten: Bei konstanter Innentemperatur muss alles ersetzt werden, was hinausfließt."),
        ("transmission",
         r"Through walls, roof, windows and floor, heat leaves by transmission, the energy $Q_T$.",
         r"Durch Wände, Dach, Fenster und Boden verlässt Wärme das Gebäude als Transmission, die Energie $Q_T$."),
        ("lueftung",
         r"With the exhaust air, heat leaves as ventilation loss $Q_V$.",
         r"Mit der Abluft verlässt Wärme das Gebäude als Lüftungsverlust $Q_V$."),
        ("sonne",
         r"The sun brings heat in for free: the solar gains $Q_S$.",
         r"Die Sonne bringt kostenlos Wärme herein: die solaren Gewinne $Q_S$."),
        ("intern",
         r"People, appliances and lighting add internal gains $Q_i$.",
         r"Personen, Geräte und Beleuchtung liefern interne Gewinne $Q_i$."),
        ("rest",
         r"What the free gains don't cover, the heating has to deliver.",
         r"Was die freien Gewinne nicht decken, muss die Heizung liefern."),
        ("bedarf",
         r"That remainder is the heating demand $Q_h$ — the useful energy the heating must hand to the room.",
         r"Dieser Rest ist der Heizwärmebedarf $Q_h$ — die Nutzenergie, die die Heizung an den Raum abgeben muss."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("System boundary & energy conservation", "Bilanzgrenze & Energieerhaltung"))
        din = reference_chip(_references.cite("!din-v-4108-6"), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([0.3, 0.0, 0.0]), 0.9)
        G = _gains(H)
        P = _paths(H)
        S = _solar(H, np.array([-4.3, 1.3, 0.0]))
        envelope = _envelope(H)
        legend = {"t": np.array([5.3, 1.3, 0.0]), "v": np.array([5.3, 0.7, 0.0]),
                  "s": np.array([-5.3, 1.3, 0.0]), "i": np.array([-5.3, 0.7, 0.0])}
        tag_t = _tag("Transmission", r"Q_{T}", LOSS_BLUE)
        tag_v = _tag(tr("Ventilation", "Lüftung"), r"Q_{V}", VENT_BLUE)
        tag_s = _tag(tr("Solar gains", "Solare Gewinne"), r"Q_{S}", SOLAR_YELLOW)
        tag_i = _tag(tr("Internal gains", "Interne Gewinne"), r"Q_{i}", INT_ORANGE)
        tag_t.move_to(np.array([4.6, 0.55, 0.0]))
        tag_v.move_to(np.array([-4.45, -0.95, 0.0]))
        tag_s.move_to(S["sun"].get_center() + np.array([0.0, -0.75, 0.0]))
        tag_i.move_to(np.array([4.5, -0.5, 0.0]))
        trans_g = flow_guides(P["trans"], LOSS_BLUE, opacity=0.2)
        vent_g = flow_guides(P["vent_in"] + P["vent_out"], VENT_BLUE, opacity=0.2)
        lead_t = _lead(tag_t, P["trans"][0].point_from_proportion(0.85), side="right", color=LOSS_BLUE)
        lead_v = _lead(tag_v, P["vent_in"][0].point_from_proportion(0.1), side="left", color=VENT_BLUE)
        lead_i = _lead(tag_i, G["person"].get_center() + RIGHT * 0.12 + UP * 0.2, side="right", color=INT_ORANGE)

        def trans_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt, color_end=LOSS_BLUE)]

        def vent_flows(rt):
            return [_stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]

        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.4)
        self.play(LaggedStart(*[FadeIn(m) for m in G["sources"]], lag_ratio=0.2),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.0,
                 during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "huelle"))
        self.play(Create(envelope), run_time=1.6)
        hold_for(self, self.NARRATION, "huelle")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fluesse"))
        self.play(Indicate(envelope, color=P_YELLOW, scale_factor=1.0), run_time=1.4)
        hold_for(self, self.NARRATION, "fluesse")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "erhaltung"))
        self.play(Indicate(H["air"], color=P_ORANGE, scale_factor=1.0), run_time=1.4)
        hold_for(self, self.NARRATION, "erhaltung")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "transmission"))
        self.play(Create(trans_g), FadeIn(tag_t), Create(lead_t), *trans_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "transmission", during=trans_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lueftung"))
        self.play(FadeOut(VGroup(trans_g, tag_t[0], lead_t)), tag_t[1].animate.move_to(legend["t"]),
                  run_time=0.6)
        self.play(Create(vent_g), FadeIn(tag_v), Create(lead_v), open_window(H["lo"], run_time=0.8),
                  *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "lueftung", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sonne"))
        self.play(FadeOut(VGroup(vent_g, tag_v[0], lead_v)), tag_v[1].animate.move_to(legend["v"]), run_time=0.6)
        self.play(FadeIn(S["sun"], scale=0.7), _shine(S), FadeIn(tag_s), run_time=1.6)
        hold_for(self, self.NARRATION, "sonne", during=lambda rt: [_sunshine(S, rt)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "intern"))
        self.play(FadeOut(VGroup(S["sun"], S["group"], tag_s[0])), tag_s[1].animate.move_to(legend["s"]),
                  run_time=0.6)
        self.play(FadeIn(tag_i), Create(lead_i), *_gain_heat(G, 1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "intern", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rest"))
        rad = _heater(H)
        tag_h = _tag(tr("Heating demand", "Heizwärmebedarf"), r"Q_{h}", DEMAND_RED)
        tag_h.move_to(np.array([4.95, H["floor_y"] + 0.2, 0.0]))
        lead_h = _lead(tag_h, P["supply"][0].point_from_proportion(0.05), side="right", color=DEMAND_RED)
        heat_g = flow_guides(P["supply"], DEMAND_RED, opacity=0.3)
        rad_spots = [rad.get_top() + UP * 0.02]

        def heating(rt):
            return [_stream(P["supply"], DEMAND_RED, rt, period=1.2),
                    ripples(rad_spots, r_max=0.5, color=DEMAND_RED, cycles=max(1.0, rt / 1.3))]

        self.play(FadeOut(VGroup(tag_i[0], lead_i)), tag_i[1].animate.move_to(legend["i"]), run_time=0.6)
        self.play(FadeIn(rad), Create(heat_g), FadeIn(tag_h), Create(lead_h), *heating(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "rest", during=heating)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf"))
        row, box, items = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "+", P_WHITE),
            ("quelle", r"Q_{g}", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("senke", r"Q_{l}", LOSS_BLUE), (None, r"\quad Q\;\mathrm{in}\;\frac{\mathrm{kWh}}{\mathrm{a}}", P_PRIMARY),
        ])
        glue = [m for m in row if all(m is not v for v in items.values())]
        sink_anims, sink_spare = _merge(stage, "demand.recap.balance.sink-symbol-into-senke", [tag_t[1], tag_v[1]], items["senke"])
        src_anims, src_spare = _merge(stage, "demand.recap.balance.source-symbol-into-quelle", [tag_s[1], tag_i[1]], items["quelle"])
        self.play(
            Create(box), *[FadeIn(g) for g in glue],
            *sink_anims, *src_anims, stage.transition("demand.recap.balance.heat-symbol-into-qh", source=tag_h[1], target=items["qh"]),
            FadeOut(VGroup(tag_h[0], lead_h, heat_g)), run_time=1.6,
        )
        self.remove(*sink_spare, *src_spare)
        ring = highlight_param(items, "qh", color=DEMAND_RED)
        self.play(Create(ring), run_time=0.6)
        hold_for(self, self.NARRATION, "bedarf")

        self.play(FadeOut(VGroup(caption, ring, box, row, envelope, rad, G["sources"], H["group"], H["air"])),
                  run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Die Verlustseite (Q_T + Q_V)

class Beat2_Verlustseite(Scene):
    NARRATION = [
        ("intro",
         r"The loss side first — and it is exactly the two mechanisms the Heating and Ventilation videos derived.",
         r"Zuerst die Verlustseite — genau die beiden Mechanismen aus den Videos Heizen und Lüftung."),
        ("transmission",
         r"Transmission: the heat flow $\Phi$ through each surface, as load $\dot{Q}_T$, times time gives the energy $Q_T$.",
         r"Transmission: Der Wärmestrom $\Phi$ durch jede Fläche, als Last $\dot{Q}_T$, mal Zeit ergibt die Energie $Q_T$."),
        ("u",
         r"The U-value $U$ states how many watts flow through one square metre per kelvin: here $0.24\;\mathrm{W/(m^2K)}$.",
         r"Der U-Wert $U$ gibt an, wie viele Watt je Quadratmeter und Kelvin fließen: hier $0{,}24\;\mathrm{W/(m^2K)}$."),
        ("a",
         r"The area $A$ tells how large the element is — here the exterior wall with $209\;\mathrm{m^2}$.",
         r"Die Fläche $A$ sagt, wie groß das Bauteil ist — hier die Außenwand mit $209\;\mathrm{m^2}$."),
        ("dt",
         r"The temperature difference $\Delta\theta = \theta_i - \theta_e$ between inside and outside — here $10\;\mathrm{K}$ in a transition month.",
         r"Die Temperaturdifferenz $\Delta\theta = \theta_i - \theta_e$ zwischen innen und außen — hier $10\;\mathrm{K}$ in einem Übergangsmonat."),
        ("t",
         r"And $t$ is the time: one month has $744\;\mathrm{h}$.",
         r"Und $t$ ist die Zeit: Ein Monat hat $744\;\mathrm{h}$."),
        ("wand",
         r"The exterior wall: $209\;\mathrm{m^2}\cdot 0.24\;\mathrm{W/(m^2K)}\cdot 10\;\mathrm{K}$ is about $500\;\mathrm{W}$.",
         r"Die Außenwand: $209\;\mathrm{m^2}\cdot 0{,}24\;\mathrm{W/(m^2K)}\cdot 10\;\mathrm{K}$ ergeben rund $500\;\mathrm{W}$."),
        ("dach",
         r"The roof: $89\;\mathrm{m^2}$ at $0.20\;\mathrm{W/(m^2K)}$ — well insulated, so only about $180\;\mathrm{W}$.",
         r"Das Dach: $89\;\mathrm{m^2}$ mit $0{,}20\;\mathrm{W/(m^2K)}$ — gut gedämmt, daher nur etwa $180\;\mathrm{W}$."),
        ("fenster",
         r"The windows: only $30\;\mathrm{m^2}$, but $1.10\;\mathrm{W/(m^2K)}$ — that is about $330\;\mathrm{W}$.",
         r"Die Fenster: nur $30\;\mathrm{m^2}$, aber $1{,}10\;\mathrm{W/(m^2K)}$ — das sind etwa $330\;\mathrm{W}$."),
        ("boden",
         r"The floor: $80\;\mathrm{m^2}$ at $0.30\;\mathrm{W/(m^2K)}$, and the ground stays $10\;\mathrm{K}$ below the room all year — $240\;\mathrm{W}$.",
         r"Der Boden: $80\;\mathrm{m^2}$ mit $0{,}30\;\mathrm{W/(m^2K)}$, und das Erdreich bleibt das ganze Jahr $10\;\mathrm{K}$ kälter als der Raum — $240\;\mathrm{W}$."),
        ("bruecken",
         r"Thermal bridges — edges, corners and joints — let additional heat escape.",
         r"Wärmebrücken — Kanten, Ecken und Anschlüsse — lassen zusätzlich Wärme entweichen."),
        ("zuschlag",
         r"We add them as a lump surcharge $\Delta U_{\mathrm{WB}}$ on every U-value.",
         r"Wir erfassen sie pauschal als Zuschlag $\Delta U_{\mathrm{WB}}$ auf jeden U-Wert."),
        ("wb_wert",
         r"That is $0.05\;\mathrm{W/(m^2K)}$ when a thermal-bridge check is available, more without one.",
         r"Mit Wärmebrückennachweis sind es $0{,}05\;\mathrm{W/(m^2K)}$, ohne Nachweis deutlich mehr."),
        ("lueftung",
         r"Ventilation, as the Ventilation video built it, carries the second stream out: the load $\dot{Q}_V$.",
         r"Die Lüftung, wie das Lüftungs-Video sie aufgebaut hat, trägt den zweiten Strom hinaus: die Last $\dot{Q}_V$."),
        ("luftwechsel",
         r"The air change rate $n$ states how often the room air is exchanged per hour — here $0.5\;\mathrm{h^{-1}}$.",
         r"Der Luftwechsel $n$ gibt an, wie oft pro Stunde die Raumluft ausgetauscht wird — hier $0{,}5\;\mathrm{h^{-1}}$."),
        ("volumen",
         r"The air volume $V$ of the building — $440\;\mathrm{m^3}$ — is what gets exchanged.",
         r"Das Luftvolumen $V$ des Gebäudes — $440\;\mathrm{m^3}$ — wird dabei ausgetauscht."),
        ("waermekap",
         r"$\rho\,c_p = 0.34\;\mathrm{Wh/(m^3K)}$ is the heat capacity of air per cubic metre; it turns the air flow into heat per kelvin.",
         r"$\rho\,c_p = 0{,}34\;\mathrm{Wh/(m^3K)}$ ist die Wärmekapazität der Luft je Kubikmeter; sie macht aus dem Luftstrom Wärme je Kelvin."),
        ("dichte",
         r"It belongs to $\rho \approx 1.2\;\mathrm{kg/m^3}$ at $20\;^{\circ}\mathrm{C}$; colder air is denser, $1.27\;\mathrm{kg/m^3}$ at $5\;^{\circ}\mathrm{C}$, but the standard keeps the fixed value.",
         r"Sie gehört zu $\rho \approx 1{,}2\;\mathrm{kg/m^3}$ bei $20\;^{\circ}\mathrm{C}$; kältere Luft ist dichter, $1{,}27\;\mathrm{kg/m^3}$ bei $5\;^{\circ}\mathrm{C}$, die Norm rechnet aber mit dem festen Wert."),
        ("lueft_block",
         r"With $\Delta\theta$ and $t$ as before, this gives the second block of the losses, $Q_V$.",
         r"Mit $\Delta\theta$ und $t$ wie zuvor ergibt sich der zweite Block der Verluste, $Q_V$."),
        ("senke_teile",
         r"Transmission $Q_T$ covers four surfaces plus thermal bridges; ventilation $Q_V$ is a block of its own.",
         r"Die Transmission $Q_T$ umfasst vier Flächen und die Wärmebrücken, die Lüftung $Q_V$ bildet einen eigenen Block."),
        ("summe",
         r"Together they form the loss side of the balance, $Q_l = Q_T + Q_V$ — all the heat the building loses.",
         r"Zusammen bilden sie die Verlustseite der Bilanz, $Q_l = Q_T + Q_V$ — alle Wärme, die das Gebäude verliert."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The loss side: transmission & ventilation", "Die Verlustseite: Transmission & Lüftung"))
        din = reference_chip(_references.cite(("din-v-4108-6", "verluste")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([-3.7, 0.0, 0.0]), 0.9)
        P = _paths(H, reach=0.8)
        hl = _surfaces(H)
        envelope = _envelope(H)
        by_surface = {"Außenwand": P["wall"], "Dach": P["roof"], "Fenster": P["win"], "Boden": P["floor"]}
        colors = {n: c for n, _a, _u, _d, c in SURFACES}

        segments = [(tr({"Außenwand": "Exterior wall", "Dach": "Roof", "Fenster": "Windows", "Boden": "Floor"}[n], n), phi, c) for (n, _a, _u, _d, c), phi in zip(SURFACES, PHI_SURFACES)]
        segments += [(tr("Thermal bridges", "Wärmebrücken"), PHI_WB, P_ORANGE), (tr("Ventilation", "Lüftung"), PHI_V, VENT_BLUE)]
        stack = stacked_bar(segments, base=np.array([1.35, -1.2, 0.0]), width=1.0, unit=BAR_UNIT)
        labels, leaders = side_labels(
            [(a, name, col) for a, (name, _v, col) in zip(stack["anchors"], segments)],
            x=2.3, align="left", min_gap=0.28, pad=0.06,
        )
        ledger = DashedVMobject(Rectangle(width=1.0, height=3.0, color=P_WHITE, stroke_width=1.6, stroke_opacity=0.5)
                                .move_to(np.array([1.35, -1.2 + 1.5, 0.0])), num_dashes=40)
        sym_t = _sym(r"Q_{T}", LOSS_BLUE).next_to(P["wall"][0].get_end(), RIGHT, buff=0.12)
        sym_v = _sym(r"Q_{V}", VENT_BLUE).next_to(P["vent_in"][0].get_start(), LEFT, buff=0.1)

        def vent_flows(rt):
            return [_stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]

        def loss_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt), *vent_flows(rt)]

        def on(name, rt):
            return [_stream(by_surface[name], colors[name], rt)]

        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.2)
        self.play(Create(envelope), H["air"].animate.set_fill(P_ORANGE, opacity=0.07),
                  open_window(H["lo"], run_time=0.8), FadeIn(sym_t), FadeIn(sym_v), *loss_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.6, during=loss_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "transmission"))
        row_t, box_t, it_t = math_panel([
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "=", P_WHITE),
            (None, r"\Sigma\,(", P_ORANGE), ("u", r"U", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", r"A", P_CYAN), (None, r")", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_PRIMARY), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        self.add(_veil(row_t))
        self.play(Create(box_t), *_terms(row_t, 0, 3), FadeOut(sym_v),
                  *[_stream(P["trans"], LOSS_BLUE, 1.6)], run_time=1.6)
        hold_for(self, self.NARRATION, "transmission", during=lambda rt: [_stream(P["trans"], LOSS_BLUE, rt)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "u"))
        u_t, a_t = ValueTracker(0.0), ValueTracker(0.0)
        ex = _card("", LOSS_BLUE, [
            (lambda: rf"U = {label_number(u_t.get_value(), 2)}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}\,K}}}}", P_ORANGE),
            (lambda: rf"A = {label_number(a_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
        ])
        ex_head = body_text(tr("Exterior wall", "Außenwand"), font_size=LABEL_FONT_SIZE, color=LOSS_BLUE).move_to(ex["head_at"])
        ring_u = highlight_param(it_t, "u", color=P_ORANGE)
        ring_a = highlight_param(it_t, "a", color=P_CYAN)
        self.add(ex["rows"][0])
        self.play(Create(hl["Außenwand"]), FadeIn(VGroup(ex["frame"], ex_head)), Create(ring_u),
                  *_terms(row_t, 3, 4), u_t.animate.set_value(0.24), *on("Außenwand", 1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "u", during=lambda rt: on("Außenwand", rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "a"))
        self.add(ex["rows"][1])
        self.play(stage.transition("demand.recap.loss.ring-u-to-a", source=ring_u, target=ring_a), *_terms(row_t, 4, 7), a_t.animate.set_value(209.0),
                  *on("Außenwand", 1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "a", during=lambda rt: on("Außenwand", rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dt"))
        th = _thermo_pair(-1.2, -1.15, height=2.4, gap=1.5)
        dth_read = math_readout(lambda: rf"\Delta\theta = {label_number(th['d_theta']())}\,\mathrm{{K}}",
                                lambda: np.array([th["x_mid"] + 1.15, (th["top"](th["in"]) + th["top"](th["out"])) / 2, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        ring_dt = highlight_param(it_t, "dt", color=P_BLUE)
        self.play(FadeOut(VGroup(H["group"], H["air"], envelope, sym_t, hl["Außenwand"], ex["frame"], ex_head)),
                  *[FadeOut(r) for r in ex["rows"]], stage.transition("demand.recap.loss.ring-a-to-dt", source=ring_a, target=ring_dt), run_time=0.8)
        self.add(th["in"]["column"], th["out"]["column"], dth_read)
        self.play(FadeIn(th["static"]), *_terms(row_t, 7, 9),
                  th["in"]["level"].animate.set_value(0.75), th["out"]["level"].animate.set_value(0.5), run_time=1.6)
        self.add(th["arrow"])
        hold_for(self, self.NARRATION, "dt")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "t"))
        clk = clock_glyph(np.array([-1.3, -0.1, 0.0]), r=0.85, color=P_PRIMARY)
        t_tr = ValueTracker(0.0)
        t_card = _card("", P_PRIMARY, [(lambda: rf"t = {label_number(t_tr.get_value())}\,\mathrm{{h}}", P_PRIMARY)], x=0.3, top=0.65)
        t_head = body_text(tr("Hours per month", "Stunden im Monat"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(t_card["head_at"])
        ring_t = highlight_param(it_t, "t", color=P_PRIMARY)
        self.add(*t_card["rows"])
        self.play(FadeOut(th["static"]), *[FadeOut(m) for m in (th["in"]["column"], th["out"]["column"], dth_read, th["arrow"])],
                  stage.transition("demand.recap.loss.ring-dt-to-t", source=ring_dt, target=ring_t), run_time=0.6)
        self.play(FadeIn(clk["group"]), FadeIn(VGroup(t_card["frame"], t_head)), *_terms(row_t, 9, 12), run_time=0.8)
        self.play(Rotate(clk["hand"], angle=-TAU * 3, about_point=clk["center"], rate_func=linear),
                  t_tr.animate.set_value(T_MONTH), run_time=1.6)
        hold_for(self, self.NARRATION, "t", during=lambda rt: [
            Rotate(clk["hand"], angle=-TAU * rt / 0.5, about_point=clk["center"], rate_func=linear)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wand"))
        u_s, a_s, d_s = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        card = _card("", P_ORANGE, [
            (lambda: rf"U = {label_number(u_s.get_value(), 2)}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}\,K}}}}", P_ORANGE),
            (lambda: rf"A = {label_number(a_s.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"\Delta\theta = {label_number(d_s.get_value())}\,\mathrm{{K}}", P_BLUE),
            (lambda: rf"\Phi = {label_number(u_s.get_value() * a_s.get_value() * d_s.get_value())}\,\mathrm{{W}}", P_WHITE),
        ])
        heads = [body_text(tr({"Außenwand": "Exterior wall", "Dach": "Roof", "Fenster": "Windows", "Boden": "Floor"}[n], n), font_size=LABEL_FONT_SIZE, color=c).move_to(card["head_at"]) for n, _a, _u, _d, c in SURFACES]
        self.add(*card["rows"])
        self.play(FadeOut(VGroup(clk["group"], t_card["frame"], t_head, ring_t)), *[FadeOut(r) for r in t_card["rows"]],
                  Create(H["group"]), Create(envelope), run_time=1.4)
        self.add(ledger)
        previous = None
        for k, (name, area, u, d_k, color) in enumerate(SURFACES):
            if k:
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, ("wand", "dach", "fenster", "boden")[k]))
            u_s.set_value(0.0)
            a_s.set_value(0.0)
            d_s.set_value(0.0)
            enter = [FadeIn(heads[0]), FadeIn(card["frame"])] if k == 0 else [stage.transition(f"demand.recap.loss.surface-head-{SURFACE_SLUGS[k - 1]}-to-{SURFACE_SLUGS[k]}", source=heads[k - 1], target=heads[k])]
            leave = [FadeOut(hl[previous])] if previous else []
            self.play(Create(hl[name]), *leave, *enter, *on(name, 1.2), run_time=1.2)
            self.play(u_s.animate.set_value(u), a_s.animate.set_value(area), d_s.animate.set_value(d_k),
                      *on(name, 1.8), run_time=1.8)
            self.play(stage.transition(f"demand.recap.loss.surface-{SURFACE_SLUGS[k]}-into-stack", source=hl[name], target=stack["bars"][k]), FadeIn(labels[k]), Create(leaders[k]),
                      *on(name, 1.2), run_time=1.2)
            hold_for(self, self.NARRATION, ("wand", "dach", "fenster", "boden")[k], during=lambda rt, n=name: on(n, rt))
            previous = name

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bruecken"))
        xl, xr = H["x_l"], H["x_r"]
        spots = [H["hs"]["bottom_left"], H["hs"]["bottom_right"], H["hs"]["top_left"], H["hs"]["top_right"],
                 np.array([xl, H["mid_y"], 0.0]), np.array([xr, H["mid_y"], 0.0]), H["peak"],
                 *[np.array([w["x"], y, 0.0]) for w in H["hs"]["windows"] for y in (w["sill"], w["lintel"])]]
        dots = VGroup(*[Dot(np.array(p, dtype=float), radius=0.075, color=P_ORANGE) for p in spots]).set_z_index(5)
        self.play(FadeOut(VGroup(card["frame"], heads[-1], hl["Boden"])), *[FadeOut(r) for r in card["rows"]],
                  LaggedStart(*[GrowFromCenter(d) for d in dots], lag_ratio=0.1), run_time=1.6)
        sparks = lambda rt: [ripples([np.array(p, dtype=float) for p in spots[:7]], r_max=0.3, color=P_ORANGE,
                                     cycles=max(1.0, rt / 1.4))]
        hold_for(self, self.NARRATION, "bruecken", during=sparks)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zuschlag"))
        du_t = ValueTracker(0.0)
        du_g = meter(tr("Thermal bridges", "Wärmebrücken"), length=2.2, thickness=0.26, color=P_ORANGE, vertical=False)
        du_g["group"].move_to(np.array([5.65, 1.2, 0.0]))
        bind_meter(du_g, du_t)
        du_read = math_readout(lambda: rf"\Delta U_{{WB}} = {label_number(du_t.get_value() * 0.15, 2)}",
                               np.array([5.65, 0.52, 0.0]), size=LABEL_FONT_SIZE + 1, color=P_ORANGE, edge="center")
        du_unit = math_label(r"\frac{\mathrm{W}}{\mathrm{m^{2}K}}", np.array([5.65, 0.16, 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        row_t2, box_t2, it_t2 = math_panel([
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "=", P_WHITE),
            (None, r"\Sigma\,(", P_ORANGE), ("u", r"(U+\Delta U_{WB})", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", r"A", P_CYAN), (None, r")", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_PRIMARY), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        ring_u2 = highlight_param(it_t2, "u", color=P_ORANGE)
        self.add(du_read)
        self.play(FadeIn(du_g["group"]), FadeIn(du_unit), Create(ring_u2),
                  stage.transition("demand.recap.loss.formula-box-with-bridges", source=box_t, target=box_t2), stage.transition("demand.recap.loss.formula-with-bridges", source=row_t, target=row_t2), run_time=1.6)
        hold_for(self, self.NARRATION, "zuschlag", during=sparks)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wb_wert"))
        self.play(du_t.animate.set_value(1 / 3), run_time=1.4)
        self.play(stage.transition("demand.recap.loss.bridge-dots-into-stack-bar", source=dots, target=stack["bars"][4]), FadeIn(labels[4]), Create(leaders[4]), run_time=1.4)
        hold_for(self, self.NARRATION, "wb_wert")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lueftung"))
        n_t, v_t, rc_t = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        v_card = _card("", VENT_BLUE, [
            (lambda: rf"n = {label_number(n_t.get_value(), 1)}\,\mathrm{{h^{{-1}}}}", P_YELLOW),
            (lambda: rf"V = {label_number(v_t.get_value())}\,\mathrm{{m^{{3}}}}", P_YELLOW),
            (lambda: rf"\rho\,c_{{p}} = {label_number(rc_t.get_value(), 2)}\,\frac{{\mathrm{{Wh}}}}{{\mathrm{{m^{{3}}\,K}}}}", P_GREEN),
            (lambda: rf"\dot{{Q}}_{{V}} = {label_number(rc_t.get_value() * n_t.get_value() * v_t.get_value() * D_THETA)}\,\mathrm{{W}}", P_WHITE),
        ])
        v_head = body_text(tr("Ventilation", "Lüftung"), font_size=LABEL_FONT_SIZE, color=VENT_BLUE).move_to(v_card["head_at"])
        vol = H["air"].copy().set_fill(VENT_BLUE, opacity=0.3).set_z_index(0)
        row_v, box_v, it_v = math_panel([
            ("qv", r"Q_{V}", VENT_BLUE), (None, "=", P_WHITE),
            ("n", r"n", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("v", r"V", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("rc", r"\rho\,c_{p}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta", P_BLUE), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_PRIMARY), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        ring_n = highlight_param(it_v, "n", color=P_YELLOW)
        ring_v = highlight_param(it_v, "v", color=P_YELLOW)
        ring_rc = highlight_param(it_v, "rc", color=P_GREEN)
        self.add(_veil(row_v))
        self.play(FadeOut(VGroup(du_g["group"], du_unit, ring_u2, row_t2)), FadeOut(du_read),
                  stage.transition("demand.recap.loss.formula-box-to-ventilation", source=box_t2, target=box_v), FadeIn(sym_v), *_terms(row_v, 0, 2), *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "lueftung", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "luftwechsel"))
        self.add(v_card["rows"][0])
        self.play(FadeIn(VGroup(v_card["frame"], v_head)), Create(ring_n), *_terms(row_v, 2, 3),
                  n_t.animate.set_value(N_AIR), *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "luftwechsel", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "volumen"))
        self.add(v_card["rows"][1])
        self.play(FadeIn(vol), stage.transition("demand.recap.loss.ring-n-to-v", source=ring_n, target=ring_v), *_terms(row_v, 3, 5),
                  v_t.animate.set_value(V_NET), *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "volumen", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waermekap"))
        self.add(v_card["rows"][2])
        self.play(stage.transition("demand.recap.loss.ring-v-to-rc", source=ring_v, target=ring_rc), *_terms(row_v, 5, 7), rc_t.animate.set_value(RHO_C),
                  *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "waermekap", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dichte"))
        rho_note = math_label(tr(r"\rho \approx 1.2\,\frac{\mathrm{kg}}{\mathrm{m^{3}}}\;\text{at}\;20\,^{\circ}\mathrm{C}",
                                 r"\rho \approx 1{,}2\,\frac{\mathrm{kg}}{\mathrm{m^{3}}}\ (20\,^{\circ}\mathrm{C})"),
                              np.array([5.65, -1.05, 0.0]), size=LABEL_FONT_SIZE, color=P_GREEN)
        self.play(FadeIn(rho_note), *vent_flows(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "dichte", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lueft_block"))
        self.add(v_card["rows"][3])
        self.play(FadeOut(ring_rc), *_terms(row_v, 7, 12), *vent_flows(1.4), run_time=1.4)
        self.play(stage.transition("demand.recap.loss.ventilation-volume-into-stack", source=vol, target=stack["bars"][5]), FadeIn(labels[5]), Create(leaders[5]),
                  *vent_flows(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "lueft_block", during=vent_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "senke_teile"))
        shift = LEFT * 1.4
        bars_s = stack["bars"].copy().shift(shift)
        brace_t = Brace(VGroup(*bars_s[:5]), LEFT, color=LOSS_BLUE, buff=0.12)
        brace_v = Brace(bars_s[5], LEFT, color=VENT_BLUE, buff=0.12)
        brace_all = Brace(bars_s, LEFT, color=LOSS_BLUE, buff=0.95)
        lab_t = _sym(r"Q_{T}", LOSS_BLUE).next_to(brace_t, LEFT, buff=0.1)
        lab_v = _sym(r"Q_{V}", VENT_BLUE).next_to(brace_v, LEFT, buff=0.1)
        lab_all = _sym(r"Q_{l}", LOSS_BLUE).next_to(brace_all, LEFT, buff=0.1)
        scale_group = VGroup(stack["bars"], labels, leaders, ledger)
        row_s, box_s, it_s = math_panel([
            ("senke", r"Q_{l}", LOSS_BLUE), (None, "=", P_WHITE),
            ("qt", r"Q_{T}", LOSS_BLUE), (None, "+", P_WHITE),
            ("qv", r"Q_{V}", VENT_BLUE), (None, r"\quad Q\;\mathrm{in}\;\frac{\mathrm{kWh}}{\mathrm{a}}", P_PRIMARY),
        ])
        glue = [m for m in row_s if all(m is not v for v in it_s.values())]
        self.play(FadeOut(VGroup(H["group"], H["air"], envelope, vol, sym_v, v_card["frame"], v_head, rho_note, row_v, box_v)),
                  *[FadeOut(r) for r in v_card["rows"]],
                  scale_group.animate.shift(shift), run_time=1.4)
        self.play(GrowFromCenter(brace_t), GrowFromCenter(brace_v), FadeIn(lab_t), FadeIn(lab_v), run_time=1.4)
        hold_for(self, self.NARRATION, "senke_teile")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        self.play(GrowFromCenter(brace_all), FadeIn(lab_all), run_time=1.2)
        self.play(Create(box_s), *[FadeIn(g) for g in glue],
                  stage.transition("demand.recap.loss.label-sink-into-sum", source=lab_all, target=it_s["senke"]), stage.transition("demand.recap.loss.label-transmission-into-sum", source=lab_t, target=it_s["qt"]),
                  stage.transition("demand.recap.loss.label-ventilation-into-sum", source=lab_v, target=it_s["qv"]), run_time=1.4)
        hold_for(self, self.NARRATION, "summe")

        self.play(FadeOut(VGroup(caption, box_s, row_s, brace_t, brace_v, brace_all, lab_t, lab_v, lab_all, scale_group)),
                  run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 3 – Die Gewinnseite & der Ausnutzungsgrad

class Beat3_Gewinnseite(Scene):
    NARRATION = [
        ("intro",
         r"Now the other pan of the scale: the heat the building gets for free.",
         r"Jetzt die andere Waagschale: die Wärme, die das Gebäude kostenlos bekommt."),
        ("solar",
         r"Solar gains $Q_S$: sunlight enters the building through the glazing.",
         r"Solare Gewinne $Q_S$: Sonnenstrahlung dringt durch die Verglasung ins Gebäude."),
        ("fenster",
         r"The window area $A$ — here $30\;\mathrm{m^2}$ — decides how much radiation enters.",
         r"Die Fensterfläche $A$ — hier $30\;\mathrm{m^2}$ — bestimmt, wie viel Strahlung hereinkommt."),
        ("einstrahlung",
         r"The irradiation $I_s$ on that facade supplies the energy per square metre: here $65\;\mathrm{kWh/m^2}$ in this spring month.",
         r"Die Einstrahlung $I_s$ auf diese Fassade liefert die Energie je Quadratmeter: hier $65\;\mathrm{kWh/m^2}$ in diesem Frühjahrsmonat."),
        ("gwert",
         r"The total solar energy transmittance $g_\perp$ says which share of the radiation passes the glass — here $0.6$.",
         r"Der Gesamtenergiedurchlassgrad $g_\perp$ sagt, welcher Anteil der Strahlung durch das Glas gelangt — hier $0{,}6$."),
        ("fs",
         r"The shading factor $F_S$ accounts for overhangs and neighbouring buildings — here $0.8$, as in the heating video.",
         r"Der Verschattungsfaktor $F_S$ berücksichtigt Überhänge und Nachbargebäude — hier $0{,}8$ wie im Heizungsvideo."),
        ("weitere",
         r"Sun protection $F_C = 1$, frame share $F_F = 0.70$, oblique incidence $F_W = 0.90$ — as in the heating video.",
         r"Sonnenschutz $F_C = 1$, Rahmenanteil $F_F = 0{,}70$, schräger Einfall $F_W = 0{,}90$ — wie im Heizungsvideo."),
        ("solar_block",
         r"Multiplied, that is the solar heat gain $Q_S = 590\;\mathrm{kWh}$.",
         r"Multipliziert ergibt das den solaren Wärmegewinn $Q_S = 590\;\mathrm{kWh}$."),
        ("intern",
         r"Internal gains $Q_i$: people, appliances and lighting all give off heat.",
         r"Interne Gewinne $Q_i$: Personen, Geräte und Beleuchtung geben Wärme ab."),
        ("q_i",
         r"DIN V 4108-6 assumes $q_i = 5\;\mathrm{W/m^2}$ for residential use.",
         r"DIN V 4108-6 setzt für Wohnnutzung $q_i = 5\;\mathrm{W/m^2}$ an."),
        ("a_n",
         r"That applies per square metre of useful floor area $A_N$ — here $60\;\mathrm{m^2}$.",
         r"Das gilt je Quadratmeter Nutzfläche $A_N$ — hier $60\;\mathrm{m^2}$."),
        ("t_i",
         r"Over the $744\;\mathrm{h}$ of the month this adds up to $Q_i = 223\;\mathrm{kWh}$.",
         r"Über die $744\;\mathrm{h}$ des Monats ergibt das $Q_i = 223\;\mathrm{kWh}$."),
        ("eta",
         r"But not every free kilowatt-hour helps.",
         r"Doch nicht jede freie Kilowattstunde hilft."),
        ("eta_h",
         r"The utilisation factor $\eta$ depends on the ratio $\gamma = Q_g / Q_l$ of gains to losses.",
         r"Der Ausnutzungsgrad $\eta$ hängt vom Verhältnis $\gamma = Q_g / Q_l$ aus Gewinnen und Verlusten ab."),
        ("wert",
         r"Here it is $\eta \approx 0.86$: that share of the free heat is actually used.",
         r"Hier ist $\eta \approx 0{,}86$: Diesen Anteil der freien Wärme nutzt das Gebäude tatsächlich."),
        ("ueberhitzung",
         r"Gains that arrive when no heat is needed only overheat the room.",
         r"Gewinne, die ohne Wärmebedarf anfallen, überhitzen nur den Raum."),
        ("bilanz",
         r"The usable gains $\eta \cdot Q_g$ cover part of the losses $Q_l$ — the heating adds the rest.",
         r"Die nutzbaren Gewinne $\eta \cdot Q_g$ decken einen Teil der Verluste $Q_l$ — den Rest liefert die Heizung."),
        ("bilanz_qh",
         r"That remainder is the heating demand $Q_h = Q_l - \eta \cdot Q_g$ — useful energy at the room.",
         r"Dieser Rest ist der Heizwärmebedarf $Q_h = Q_l - \eta \cdot Q_g$ — Nutzenergie am Raum."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The gain side & the utilisation factor", "Die Gewinnseite & der Ausnutzungsgrad"))
        din = reference_chip(_references.cite(("din-v-4108-6", "gewinne")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        H = _home(np.array([-2.5, 0.0, 0.0]), 0.9)
        P = _paths(H, reach=0.8)
        G = _gains(H)
        S = _solar(H, np.array([-5.9, 1.35, 0.0]), ("up", "lo"))
        envelope = _envelope(H)
        slats = _overhangs(H)

        base_y, unit, width = -1.0, 2.9 / Q_L, 1.0
        x_sink, x_gain = 0.95, 2.35

        def block(x, y0, value, color, opacity=0.85):
            h = value * unit
            return Rectangle(width=width, height=h, color=color, fill_color=color, fill_opacity=opacity,
                             stroke_width=1.2).move_to(np.array([x, base_y + y0 * unit + h / 2, 0.0]))

        sink = block(x_sink, 0.0, Q_L, LOSS_BLUE, 0.55)
        ground = Line(np.array([x_sink - 0.75, base_y, 0.0]), np.array([x_gain + 0.75, base_y, 0.0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        cap_sink = math_label(r"Q_{l}", np.array([x_sink, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain = math_label(r"Q_{g}", np.array([x_gain, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=SOLAR_YELLOW)

        def loss_flows(rt):
            return [_stream(P["trans"], LOSS_BLUE, rt),
                    _stream(P["vent_in"], VENT_BLUE, rt, color_end=P_ORANGE),
                    _stream(P["vent_out"], P_ORANGE, rt, color_end=VENT_BLUE)]

        self.add(H["air"])
        self.play(Create(H["group"]), run_time=1.2)
        self.play(Create(envelope), Create(ground), GrowFromEdge(sink, DOWN), FadeIn(cap_sink),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), open_window(H["lo"], run_time=0.8),
                  *loss_flows(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.4, during=loss_flows)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        aw_t, is_t, g_t, fs_t, qs_t = [ValueTracker(0.0) for _ in range(5)]
        g_g = _gauge(r"g_{\perp}", P_ORANGE, np.array([5.45, 1.85, 0.0]), g_t)
        fs_g = _gauge(r"F_{S}", P_TEAL, np.array([5.45, 1.85, 0.0]), fs_t)
        s_card = _card("", P_YELLOW, [
            (lambda: rf"A = {label_number(aw_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"I_{{s}} = {label_number(is_t.get_value())}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{m^{{2}}}}}}", P_YELLOW),
            (lambda: rf"g_{{\perp}} = {label_number(g_t.get_value(), 2)}", P_ORANGE),
            (lambda: rf"F_{{S}} = {label_number(fs_t.get_value(), 2)}", P_TEAL),
            (lambda: rf"Q_{{S}} = {label_number(qs_t.get_value())}\,\mathrm{{kWh}}", SOLAR_YELLOW),
        ], top=1.45)
        glass = VGroup(*[Line(np.array([w["x"], w["sill"], 0.0]), np.array([w["x"], w["lintel"], 0.0]),
                              color=P_CYAN, stroke_width=7.5) for w in (H["lo"], H["up"])])
        patches = VGroup(*[Rectangle(width=0.55, height=0.07, stroke_width=0, fill_color=SOLAR_YELLOW, fill_opacity=0.9)
                           .move_to(np.array([float(np.mean([p[0] for p in r["lands"]])), y + 0.04, 0.0]))
                           for r, y in zip(S["sets"], (H["mid_y"], H["floor_y"]))])
        solar_block = block(x_gain, 0.0, Q_SOLAR, SOLAR_YELLOW)
        row_s, box_s, it_s = math_panel([
            ("qs", r"Q_{S}", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("a", r"A", P_CYAN), (None, r"\cdot", P_WHITE),
            ("i", r"I_{s}", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("g", r"g_{\perp}", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("f", r"F_{S}", P_TEAL), (None, r"\cdot", P_WHITE), (None, r"F_{C}", P_TEAL), (None, r"\cdot", P_WHITE),
            (None, r"F_{F}", P_TEAL), (None, r"\cdot", P_WHITE), (None, r"F_{W}", P_TEAL),
            (None, rf"\quad F_{{C}}={label_number(F_C)},\;F_{{F}}={label_number(F_F, 2)},\;F_{{W}}={label_number(F_W, 2)}", P_PRIMARY),
        ])
        ring_a = highlight_param(it_s, "a", color=P_CYAN)
        ring_i = highlight_param(it_s, "i", color=P_YELLOW)
        ring_g = highlight_param(it_s, "g", color=P_ORANGE)
        ring_f = highlight_param(it_s, "f", color=P_TEAL)
        sunny = lambda rt: [_sunshine(S, rt)]
        self.add(_veil(row_s))
        self.play(FadeIn(S["sun"], scale=0.7), _shine(S), Create(glass), Create(box_s), *_terms(row_s, 0, 2),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "solar", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fenster"))
        self.add(s_card["rows"][0])
        self.play(FadeIn(s_card["frame"]), Create(ring_a), *_terms(row_s, 2, 3), aw_t.animate.set_value(A_WINDOW),
                  *sunny(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "fenster", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "einstrahlung"))
        self.add(s_card["rows"][1])
        self.play(stage.transition("demand.recap.gain.ring-a-to-i", source=ring_a, target=ring_i), *_terms(row_s, 3, 5), is_t.animate.set_value(I_S),
                  *sunny(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "einstrahlung", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gwert"))
        self.add(s_card["rows"][2])
        self.play(FadeIn(g_g["static"]), stage.transition("demand.recap.gain.ring-i-to-g", source=ring_i, target=ring_g), *_terms(row_s, 5, 7),
                  g_t.animate.set_value(G_VALUE), *sunny(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "gwert", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fs"))
        self.add(s_card["rows"][3])
        self.play(FadeOut(g_g["static"]), FadeIn(fs_g["static"]), Create(slats), stage.transition("demand.recap.gain.ring-g-to-f", source=ring_g, target=ring_f),
                  *_terms(row_s, 7, 9), fs_t.animate.set_value(F_S), *sunny(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "fs", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "weitere"))
        self.play(FadeOut(fs_g["static"]), FadeOut(ring_f), *_terms(row_s, 9, 16), *sunny(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "weitere", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar_block"))
        self.add(s_card["rows"][4])
        self.play(FadeIn(patches), *sunny(1.0), run_time=1.0)
        self.play(stage.transition("demand.recap.gain.patches-into-solar-block", source=patches, target=solar_block), FadeIn(cap_gain), qs_t.animate.set_value(Q_SOLAR),
                  *sunny(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "solar_block", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "intern"))
        q_t, an_t, t_t, qi_t = [ValueTracker(0.0) for _ in range(4)]
        i_card = _card("", INT_ORANGE, [
            (lambda: rf"q_{{i}} = {label_number(q_t.get_value())}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}}}}}", P_ORANGE),
            (lambda: rf"A_{{N}} = {label_number(an_t.get_value())}\,\mathrm{{m^{{2}}}}", P_CYAN),
            (lambda: rf"t = {label_number(t_t.get_value())}\,\mathrm{{h}}", P_PRIMARY),
            (lambda: rf"Q_{{i}} = {label_number(qi_t.get_value())}\,\mathrm{{kWh}}", INT_ORANGE),
        ], top=1.85)
        floors = VGroup(*[Line(np.array([H["x_l"] + 0.12, y + 0.04, 0.0]), np.array([H["x_r"] - 0.12, y + 0.04, 0.0]),
                               color=P_CYAN, stroke_width=6) for y in (H["floor_y"], H["mid_y"])])
        heat_pack = VGroup(*[Rectangle(width=0.5, height=0.07, stroke_width=0, fill_color=INT_ORANGE, fill_opacity=0.9)
                             .move_to(np.array([G["person"].get_center()[0], y + 0.1, 0.0])) for y in (H["floor_y"], H["mid_y"])])
        intern_block = block(x_gain, 0.0, Q_INTERN, INT_ORANGE)
        row_i, box_i, it_i = math_panel([
            ("qi", r"Q_{i}", INT_ORANGE), (None, "=", P_WHITE),
            ("q", r"q_{i}", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", r"A_{N}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("t", r"t", P_PRIMARY), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        ring_q = highlight_param(it_i, "q", color=P_ORANGE)
        ring_an = highlight_param(it_i, "a", color=P_CYAN)
        ring_t = highlight_param(it_i, "t", color=P_PRIMARY)
        self.add(_veil(row_i))
        self.play(FadeOut(VGroup(s_card["frame"], glass, slats, patches, S["sun"], S["group"], row_s)),
                  *[FadeOut(r) for r in s_card["rows"]],
                  stage.transition("demand.recap.gain.formula-box-to-internal", source=box_s, target=box_i), *_terms(row_i, 0, 2),
                  LaggedStart(*[FadeIn(m) for m in G["sources"]], lag_ratio=0.2), run_time=1.6)
        hold_for(self, self.NARRATION, "intern", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "q_i"))
        self.add(i_card["rows"][0])
        self.play(FadeIn(i_card["frame"]), Create(ring_q), *_terms(row_i, 2, 3), q_t.animate.set_value(Q_I_SPEC),
                  *_gain_heat(G, 1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "q_i", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "a_n"))
        self.add(i_card["rows"][1])
        self.play(stage.transition("demand.recap.gain.ring-q-to-an", source=ring_q, target=ring_an), Create(floors), *_terms(row_i, 3, 5),
                  an_t.animate.set_value(A_NET), *_gain_heat(G, 1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "a_n", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "t_i"))
        self.add(i_card["rows"][2])
        self.play(stage.transition("demand.recap.gain.ring-an-to-t", source=ring_an, target=ring_t), FadeOut(floors), *_terms(row_i, 5, 8),
                  t_t.animate.set_value(T_MONTH), *_gain_heat(G, 1.6), run_time=1.6)
        self.add(i_card["rows"][3])
        self.play(FadeOut(ring_t), FadeIn(heat_pack), run_time=0.6)
        self.play(stage.transition("demand.recap.gain.heat-pack-into-internal-block", source=heat_pack, target=intern_block), qi_t.animate.set_value(Q_INTERN),
                  solar_block.animate.move_to(np.array([x_gain, base_y + (Q_INTERN + Q_SOLAR / 2) * unit, 0.0])),
                  *_gain_heat(G, 1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "t_i", during=lambda rt: _gain_heat(G, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        self.play(FadeOut(VGroup(i_card["frame"], heat_pack, G["sources"], H["group"], H["air"], envelope, box_i, row_i)),
                  *[FadeOut(r) for r in i_card["rows"]], run_time=1.0)
        self.play(Indicate(VGroup(intern_block, solar_block), color=P_WHITE, scale_factor=1.0), run_time=1.4)
        hold_for(self, self.NARRATION, "eta")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta_h"))
        eta_t = ValueTracker(0.0)
        eta_g = _gauge(r"\eta", ETA_GREEN, np.array([-3.2, 0.1, 0.0]), eta_t, length=2.4)
        ratio = VGroup(math_label(r"Q_{g}", size=BODY_FONT_SIZE, color=SOLAR_YELLOW),
                       math_label(r"/", size=BODY_FONT_SIZE, color=P_WHITE),
                       math_label(r"Q_{l}", size=BODY_FONT_SIZE, color=LOSS_BLUE)).arrange(RIGHT, buff=0.2)
        ratio.move_to(np.array([-3.2, 1.2, 0.0]))
        usable = block(x_gain, 0.0, Q_USABLE, ETA_GREEN)
        waste = block(x_gain, Q_USABLE, Q_G - Q_USABLE, P_RED, 0.6)
        spill = DashedLine(np.array([x_gain - 0.65, base_y + Q_G * unit, 0.0]),
                           np.array([x_gain + 0.65, base_y + Q_G * unit, 0.0]), color=P_RED, stroke_width=2)
        spill_note = body_text(tr("Overheating", "Überhitzung"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([3.95, base_y + (Q_USABLE + (Q_G - Q_USABLE) / 2) * unit, 0.0]))
        row_e, box_e, it_e = math_panel([
            ("eta", r"\eta", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{g}", SOLAR_YELLOW),
            (None, rf"\approx {label_number(ETA_H, 2)}\cdot {label_number(Q_G)}\,\mathrm{{kWh}} \approx {label_number(Q_USABLE)}\,\mathrm{{kWh}}", P_WHITE),
        ])
        ring_eta = highlight_param(it_e, "eta", color=ETA_GREEN)
        self.add(_veil(row_e), eta_g["read"])
        self.play(FadeIn(eta_g["static"]), FadeIn(ratio), Create(box_e), *_terms(row_e, 0, 3), Create(ring_eta),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "eta_h")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wert"))
        self.play(eta_t.animate.set_value(ETA_H), *_terms(row_e, 3, 4), run_time=1.6)
        self.play(stage.transition("demand.recap.gain.blocks-into-usable-and-waste", source=VGroup(intern_block, solar_block), target=VGroup(usable, waste)), Create(spill),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "wert")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ueberhitzung"))
        self.add(H["air"])
        self.play(FadeOut(VGroup(eta_g["static"], ratio, ring_eta)), FadeOut(eta_g["read"]), run_time=0.8)
        self.play(Create(H["group"]), Create(envelope), FadeIn(G["sources"]),
                  H["air"].animate.set_fill(P_RED, opacity=0.16), FadeIn(spill_note),
                  *_gain_heat(G, 1.6, color=P_RED), run_time=1.6)
        hold_for(self, self.NARRATION, "ueberhitzung", during=lambda rt: _gain_heat(G, rt, color=P_RED))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        rad = _heater(H)
        supply = P["supply"]
        demand = block(x_gain, Q_USABLE, Q_L - Q_USABLE, DEMAND_RED)
        level = DashedLine(np.array([x_sink + 0.65, base_y + Q_L * unit, 0.0]),
                           np.array([x_gain + 0.65, base_y + Q_L * unit, 0.0]), color=P_WHITE, stroke_width=1.8,
                           stroke_opacity=0.55)
        lab_use = math_label(rf"\eta\cdot Q_{{g}}", np.array([3.0, base_y + Q_USABLE / 2 * unit - 0.07, 0.0]),
                             size=LABEL_FONT_SIZE + 1, color=ETA_GREEN, edge="left")
        lab_dem = math_label(r"Q_{h}", np.array([3.0, base_y + (Q_USABLE + (Q_L - Q_USABLE) / 2) * unit - 0.07, 0.0]),
                             size=BODY_FONT_SIZE, color=DEMAND_RED, edge="left")
        row_b, box_b, it_b = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", r"Q_{l}", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", r"\eta", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{g}", SOLAR_YELLOW), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        ring_h = highlight_param(it_b, "qh", color=DEMAND_RED)
        warm_in = lambda rt: [_stream(supply, DEMAND_RED, rt, period=1.2),
                              ripples([rad.get_top() + UP * 0.02], r_max=0.5, color=DEMAND_RED, cycles=max(1.0, rt / 1.3))]
        self.play(FadeOut(VGroup(spill, spill_note, waste, G["sources"])),
                  H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=0.8)
        self.play(FadeIn(rad), stage.transition("demand.recap.gain.radiator-into-demand", source=rad, target=demand), Create(level), *warm_in(1.6), run_time=1.6)
        self.play(FadeIn(lab_use), FadeIn(lab_dem), *warm_in(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "bilanz", during=warm_in)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz_qh"))
        self.play(stage.transition("demand.recap.gain.formula-box-to-balance", source=box_e, target=box_b), stage.transition("demand.recap.gain.formula-to-balance", source=row_e, target=row_b),
                  stage.transition("demand.recap.gain.demand-label-into-qh", source=lab_dem, target=it_b["qh"]), *warm_in(1.4), run_time=1.4)
        self.play(Create(ring_h), *warm_in(0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "bilanz_qh", during=warm_in)

        self.play(FadeOut(VGroup(caption, ring_h, box_b, row_b, sink, cap_sink, cap_gain, usable, demand, lab_use, lab_dem,
                                 level, ground, rad, envelope, H["group"], H["air"])), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Dieselbe Bilanz im Sommer

class Beat4_WinterUndSommer(Scene):
    NARRATION = [
        ("intro",
         r"The same balance runs all year — only the two pans change height.",
         r"Dieselbe Bilanz läuft das ganze Jahr — nur die beiden Waagschalen ändern ihre Höhe."),
        ("januar",
         r"In January it is cold outside, as on our example winter day: $\Delta\theta = 20\;\mathrm{K}$, and the sun stands low.",
         r"Im Januar ist es draußen kalt wie am Beispiel-Wintertag, $\Delta\theta = 20\;\mathrm{K}$, und die Sonne steht tief."),
        ("januar_schalen",
         r"So the loss pan $Q_l$ is high and the gain pan is small — almost all gains are used, $\eta \approx 0.98$.",
         r"Deshalb ist die Verlustschale $Q_l$ hoch und die Gewinnschale klein — fast alle Gewinne werden genutzt, $\eta \approx 0{,}98$."),
        ("januar_luecke",
         r"The gap between them is the heating demand $Q_h$.",
         r"Die Lücke dazwischen ist der Heizwärmebedarf $Q_h$."),
        ("juli",
         r"In July it flips: the sun stands high and the temperature difference $\Delta\theta$ shrinks to $6\;\mathrm{K}$.",
         r"Im Juli kippt es: Die Sonne steht hoch, und die Temperaturdifferenz $\Delta\theta$ schrumpft auf $6\;\mathrm{K}$."),
        ("juli_schalen",
         r"The losses shrink and count only with $\eta_c \approx 0.53$, while sun and internal loads deliver more heat.",
         r"Die Verluste schrumpfen und zählen nur mit $\eta_c \approx 0{,}53$, während Sonne und interne Lasten mehr Wärme liefern."),
        ("juli_ueberschuss",
         r"Now the gains exceed the usable losses $\eta_c \cdot Q_l$, and the surplus opens a gap on the other side.",
         r"Jetzt übersteigen die Gewinne die nutzbaren Verluste $\eta_c \cdot Q_l$, und der Überschuss öffnet eine Lücke auf der anderen Seite."),
        ("juli_kuehlen",
         r"That surplus is the cooling demand $Q_c = Q_g - \eta_c \cdot Q_l$ — heat that has to be removed.",
         r"Dieser Überschuss ist der Kühlbedarf $Q_c = Q_g - \eta_c \cdot Q_l$ — Wärme, die wir abführen müssen."),
        ("beide",
         r"Same building, same physics, opposite sign.",
         r"Gleiches Gebäude, gleiche Physik, umgekehrtes Vorzeichen."),
        ("groessen",
         r"Heating counted the annual demand $Q_h$, cooling the design load $\dot{Q}_C$ — two quantities, one envelope.",
         r"Heizen zählte den Jahresbedarf $Q_h$, Kühlen die Auslegungslast $\dot{Q}_C$ — zwei Größen, eine Hülle."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)

        subtitle = section_heading(tr("The same balance, two seasons", "Dieselbe Bilanz, zwei Jahreszeiten"))
        din = reference_chip(_references.cite(("din-v-4108-6", "ausnutzung"), ("din-ts-18599-2", "nutzenergie")), subtitle)
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        season, climate, eta_t = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)

        def mix(a, b, t):
            return a + (b - a) * t.get_value()

        base_y, unit, width = -1.0, 2.9 / Q_L_JAN, 1.0
        x_loss, x_gain = -1.2, 1.0
        mid_x = (x_loss + x_gain) / 2

        def loss_h():
            return mix(Q_L_JAN, ETA_C * Q_L_JUL, season)

        def gain_h():
            return mix(Q_USABLE_JAN, Q_G_JUL, season)

        def column(x, height, color_fn, opacity):
            def build():
                h = max(0.02, height() * unit)
                c = color_fn()
                return Rectangle(width=width, height=h, color=c, fill_color=c, fill_opacity=opacity,
                                 stroke_width=1.2).move_to(np.array([x, base_y + h / 2, 0.0]))
            return always_redraw(build)

        loss_col = column(x_loss, loss_h, lambda: LOSS_BLUE, 0.55)
        gain_col = column(x_gain, gain_h,
                          lambda: interpolate_color(ManimColor(ETA_GREEN), ManimColor(SOLAR_YELLOW), season.get_value()), 0.85)

        def gap():
            lo, hi = sorted((loss_h(), gain_h()))
            y0, y1 = base_y + lo * unit, base_y + hi * unit
            if y1 - y0 < 0.08:
                return VGroup()
            return VGroup(
                Rectangle(width=x_gain - x_loss - width, height=y1 - y0, stroke_width=0, fill_color=DEMAND_RED,
                          fill_opacity=0.3).move_to(np.array([mid_x, (y0 + y1) / 2, 0.0])),
                DashedLine(np.array([x_loss + width / 2, y1, 0.0]), np.array([x_gain - width / 2, y1, 0.0]),
                           color=P_WHITE, stroke_width=1.6, stroke_opacity=0.6),
                DashedLine(np.array([x_loss + width / 2, y0, 0.0]), np.array([x_gain - width / 2, y0, 0.0]),
                           color=P_WHITE, stroke_width=1.6, stroke_opacity=0.6),
                dim_arrow(np.array([mid_x + 0.2, y0, 0.0]), np.array([mid_x + 0.2, y1, 0.0]), color=DEMAND_RED),
            )

        gap_w, gap_s = always_redraw(gap), always_redraw(gap)
        ground = Line(np.array([x_loss - 0.75, base_y, 0.0]), np.array([x_gain + 0.75, base_y, 0.0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        cap_loss = math_label(r"Q_{l}", np.array([x_loss, base_y - 0.38, 0.0]), size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain = math_label(r"\eta\cdot Q_{g}", np.array([x_gain, base_y - 0.38, 0.0]),
                              size=BODY_FONT_SIZE, color=ETA_GREEN)
        cap_loss_k = math_label(r"\eta_{c}\cdot Q_{l}", np.array([x_loss, base_y - 0.38, 0.0]),
                                size=BODY_FONT_SIZE, color=LOSS_BLUE)
        cap_gain_k = math_label(r"Q_{g}", np.array([x_gain, base_y - 0.38, 0.0]),
                                size=BODY_FONT_SIZE, color=SOLAR_YELLOW)
        gap_lab_h = math_label(r"Q_{h}", np.array([mid_x - 0.2, base_y + (Q_L_JAN + Q_USABLE_JAN) / 2 * unit - 0.07, 0.0]),
                               size=BODY_FONT_SIZE, color=DEMAND_RED)
        gap_lab_k = math_label(r"Q_{c}", np.array([mid_x - 0.2, base_y + 0.5 * (ETA_C * Q_L_JUL + Q_G_JUL) * unit - 0.07, 0.0]),
                               size=BODY_FONT_SIZE, color=DEMAND_RED)

        sun_at = np.array([-5.15, 1.4, 0.0])

        def sun_mob():
            m = climate.get_value()
            sun = sun_glyph(sun_at).scale(0.28 + 0.2 * m, about_point=sun_at)
            sun.set_opacity(0.55 + 0.45 * m)
            return sun

        sun_live = always_redraw(sun_mob)
        season_chip = chip(tr("January", "Januar"), LOSS_BLUE).move_to(np.array([4.9, 1.7, 0.0]))
        result_chip = chip(tr("Heating demand", "Heizwärmebedarf"), DEMAND_RED).move_to(np.array([4.9, 1.0, 0.0]))
        season_chip_j = chip(tr("July", "Juli"), P_RED).move_to(season_chip)
        result_chip_k = chip(tr("Cooling demand", "Kühlbedarf"), DEMAND_RED).move_to(result_chip)
        eta_g = _gauge(r"\eta", ETA_GREEN, np.array([4.9, 0.1, 0.0]), eta_t)
        card = _card("", P_PRIMARY, [
            (lambda: rf"\Delta\theta = {label_number(mix(D_THETA_JAN, D_THETA_JUL, climate))}\,\mathrm{{K}}", P_BLUE),
            (lambda: rf"I_{{s}} = {label_number(mix(I_S_JAN, I_S_JUL, climate))}\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{m^{{2}}}}}}", P_YELLOW),
        ], x=-6.3, top=0.85)

        row_w, box_w, it_w = math_panel([
            ("qh", r"Q_{h}", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", r"Q_{l}", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", r"\eta", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("quelle", r"Q_{g}", SOLAR_YELLOW), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        row_k, box_k, it_k = math_panel([
            ("qk", r"Q_{c}", DEMAND_RED), (None, "=", P_WHITE),
            ("quelle", r"Q_{g}", SOLAR_YELLOW), (None, "−", P_WHITE),
            ("eta", r"\eta_{c}", ETA_GREEN), (None, r"\cdot", P_WHITE),
            ("senke", r"Q_{l}", LOSS_BLUE), (None, r"\quad Q\;\mathrm{in}\;\mathrm{kWh}", P_PRIMARY),
        ])
        ring_h = highlight_param(it_w, "qh", color=DEMAND_RED)
        ring_k = highlight_param(it_k, "qk", color=DEMAND_RED)

        self.play(Create(ground), FadeIn(cap_loss), FadeIn(cap_gain), FadeIn(loss_col), FadeIn(gain_col), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "januar"))
        self.play(FadeIn(season_chip), FadeIn(card["frame"]), FadeIn(sun_live), *[FadeIn(r) for r in card["rows"]],
                  run_time=1.4)
        hold_for(self, self.NARRATION, "januar")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "januar_schalen"))
        self.add(eta_g["read"])
        self.play(Indicate(cap_loss, color=P_WHITE), run_time=1.0)
        self.play(Indicate(cap_gain, color=P_WHITE), FadeIn(eta_g["static"]), eta_t.animate.set_value(ETA_JAN), run_time=1.4)
        hold_for(self, self.NARRATION, "januar_schalen")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "januar_luecke"))
        self.play(FadeIn(gap_w), FadeIn(gap_lab_h), FadeIn(result_chip), run_time=1.4)
        self.play(Create(box_w), FadeIn(row_w), Create(ring_h), run_time=1.2)
        hold_for(self, self.NARRATION, "januar_luecke")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "juli"))
        self.play(FadeOut(VGroup(gap_lab_h, ring_h, box_w, row_w)), FadeOut(gap_w),
                  FadeOut(result_chip), run_time=0.8)
        self.play(climate.animate.set_value(1.0), stage.transition("demand.recap.seasons.season-chip-to-summer", source=season_chip, target=season_chip_j),
                  run_time=2.0, rate_func=smooth)
        hold_for(self, self.NARRATION, "juli")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "juli_schalen"))
        self.play(season.animate.set_value(1.0), eta_t.animate.set_value(ETA_C),
                  stage.transition("demand.recap.seasons.shell-loss-to-summer", source=cap_loss, target=cap_loss_k), stage.transition("demand.recap.seasons.shell-gain-to-summer", source=cap_gain, target=cap_gain_k),
                  run_time=2.6, rate_func=smooth)
        hold_for(self, self.NARRATION, "juli_schalen")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "juli_ueberschuss"))
        self.play(FadeIn(gap_s), FadeIn(gap_lab_k), run_time=1.6)
        hold_for(self, self.NARRATION, "juli_ueberschuss")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "juli_kuehlen"))
        self.play(FadeIn(result_chip_k), run_time=0.8)
        self.play(Create(box_k), FadeIn(row_k), Create(ring_k), run_time=1.2)
        hold_for(self, self.NARRATION, "juli_kuehlen")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "beide"))
        win_chip = chip(tr("Winter: losses > usable gains", "Winter: Verluste > nutzbare Gewinne"), LOSS_BLUE, font_size=LABEL_FONT_SIZE).move_to(np.array([4.6, -0.2, 0.0]))
        sum_chip = chip(tr("Summer: gains > usable losses", "Sommer: Gewinne > nutzbare Verluste"), P_RED, font_size=LABEL_FONT_SIZE).move_to(np.array([4.6, -0.9, 0.0]))
        self.play(FadeOut(VGroup(card["frame"], eta_g["static"], ring_k)), *[FadeOut(r) for r in card["rows"]],
                  FadeOut(eta_g["read"]), run_time=0.8)
        self.play(FadeIn(win_chip), FadeIn(sum_chip), run_time=1.4)
        hold_for(self, self.NARRATION, "beide")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "groessen"))
        H = _home(np.array([-1.6, 0.0, 0.0]), 0.9)
        envelope = _envelope(H)
        sign_h = chip(tr("Heating: annual demand", "Heizen: Jahresbedarf"), DEMAND_RED).move_to(np.array([4.4, 0.5, 0.0]))
        sign_k = chip(tr("Cooling: design load", "Kühlen: Auslegungslast"), P_CYAN).move_to(np.array([4.4, -0.3, 0.0]))
        self.play(FadeOut(VGroup(box_k, row_k, season_chip_j, result_chip_k, win_chip, sum_chip, cap_loss_k, cap_gain_k,
                                 gap_lab_k, ground)),
                  *[FadeOut(m) for m in (loss_col, gain_col, gap_s, sun_live)], run_time=1.0)
        self.add(H["air"])
        self.play(Create(H["group"]), Create(envelope), H["air"].animate.set_fill(P_ORANGE, opacity=0.07), run_time=1.4)
        self.play(FadeIn(sign_h), FadeIn(sign_k), Indicate(envelope, color=P_PRIMARY, scale_factor=1.0), run_time=1.4)
        hold_for(self, self.NARRATION, "groessen")

        self.play(FadeOut(VGroup(caption, sign_h, sign_k, envelope, H["group"], H["air"])), run_time=0.6)
        self.wait(0.4)

#endregion
