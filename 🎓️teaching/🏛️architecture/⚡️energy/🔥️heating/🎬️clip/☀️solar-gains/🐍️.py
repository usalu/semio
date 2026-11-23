"""☀️ Heating Module 5 — Solarer Wärmegewinn.

One south window carries the whole chain Q̇_S = I_s · A · g_⊥ · F_F · F_W · F_S · F_C
(DIN V 4108-6, 6.4): every factor is read from the drawing (dimension arrows, frame
face, counted ray packets, sunlit glass height) and counts up from zero while it grows.
Formulas and units are typeset with the ``math_*`` helpers from
``manim_visuals``; German ``caption_bar`` / ``hold_for`` keep VO sync. Houses,
rooms, sun, moon, straight rays, light pulses and heat ripples are the Physical
Fundamentals glyphs, so light and heat keep moving while each subtitle is read.
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


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
section_material = _manim_module.section_material
section_assembly = _manim_module.section_assembly
PASTEL_WHITE = _manim_module.PASTEL_WHITE
PASTEL_CYAN = _manim_module.PASTEL_CYAN
PASTEL_TEAL = _manim_module.PASTEL_TEAL
PASTEL_ORANGE = _manim_module.PASTEL_ORANGE
PASTEL_YELLOW = _manim_module.PASTEL_YELLOW
PASTEL_RED = _manim_module.PASTEL_RED
PASTEL_BLUE = _manim_module.PASTEL_BLUE
PASTEL_GREEN = _manim_module.PASTEL_GREEN
highlight_param = _manim_module.highlight_param
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
radiation_ray = _manim_module.radiation_ray
tr = _manim_module.tr
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
staffage = _manim_module.staffage
staffage_height = _manim_module.staffage_height
staffage_parts = _manim_module.staffage_parts
lamella_faces = _manim_module.lamella_faces
dim_arrow = _manim_module.dim_arrow
math_text = _manim_module.math_text
math_row = _manim_module.math_row
math_readout = _manim_module.math_readout
math_panel = _manim_module.math_panel
fit_band = _manim_module.fit_band
SAFE_BOTTOM = _manim_module.SAFE_BOTTOM
math_label = _manim_module.math_label
label_number = _manim_module.label_number
house_section = _manim_module.house_section
window_glyph = _manim_module.window_glyph
emphasize_casement = _manim_module.emphasize_casement
room_section = _manim_module.room_section
sun_glyph = _manim_module.sun_glyph
moon_glyph = _manim_module.moon_glyph
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
ripples = _manim_module.ripples
pulse_flashes = _manim_module.pulse_flashes
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
flow_animation = _manim_module.flow_animation

# 🗣️ Timing follows German captions (reading floor in hold_for).

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 5: Solarer Wärmegewinn"

COLOR_G = PASTEL_YELLOW
COLOR_A = PASTEL_CYAN
COLOR_WIN = PASTEL_CYAN
COLOR_FRAME = semio_color("gray-600")
COLOR_FF = PASTEL_ORANGE
COLOR_GVAL = PASTEL_GREEN
COLOR_REFL = semio_color("light-gray")
COLOR_GLASS_HEAT = PASTEL_RED
COLOR_FSH = PASTEL_BLUE
COLOR_HEAT = PASTEL_ORANGE
COLOR_SLAB = semio_color("gray-400")
COLOR_MASS = PASTEL_CYAN
COLOR_C = PASTEL_GREEN
COLOR_FC = PASTEL_TEAL
COLOR_FW = PASTEL_RED

SYM_QS = r"\dot{Q}_{\mathrm{S}}"
SYM_QS_E = r"Q_{\mathrm{S}}"
SYM_IS = r"I_{\mathrm{s}}"
SYM_G = r"g_{\perp}"
SYM_FF = r"F_{\mathrm{F}}"
SYM_FW = r"F_{\mathrm{W}}"
SYM_FS = r"F_{\mathrm{S}}"
SYM_FC = r"F_{\mathrm{C}}"
UNIT_TAIL = rf"\quad[{SYM_QS}] = \mathrm{{W}}"

#region Example window
# The one south window of the shared example house (Modul 2/3/5, final):
# 1,2 m × 1,6 m, clear winter day I_s = 400 W/m², glass share F_F = 0,70 from the
# frame face width, g_⊥ = 0,60 from the energy split at the glass, F_S from the sunlit
# glass height under an overhang sized for 0,80 at the winter sun, F_C = 0,15 with the
# blind lowered and F_W = 0,90 for non-perpendicular incidence (DIN V 4108-6).
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
WIN_W, WIN_H = _example.WINDOW_W_M, _example.WINDOW_H_M
WIN_A = _example.WINDOW_A_M2
G_WINTER = _example.I_S_WINTER_W_M2
FF_GLASS = _example.F_F
FW_INCIDENCE = _example.F_W
FRAME_FACE = ((WIN_W + WIN_H) - np.sqrt((WIN_W + WIN_H) ** 2 - 4 * WIN_W * WIN_H * (1 - FF_GLASS))) / 4
SHARE_R, SHARE_T, SHARE_IN, SHARE_OUT = _example.G_SHARE_REFLECTED, _example.G_SHARE_DIRECT, _example.G_SHARE_SECONDARY_IN, _example.G_SHARE_SECONDARY_OUT
SHARE_ABS = SHARE_IN + SHARE_OUT
G_VALUE = _example.G_PERP
ROOM_DEPTH, ROOM_H, SILL_H = 5.0, 2.6, 0.6
ELEV_SUMMER, ELEV_WINTER = 60.0, 20.0
ROOF_OVERHANG = 1.07
FS_WINTER = _example.F_S_WINTER
FC_WINTER = _example.F_C_NONE
AWNING_DEPTH = (1 - FS_WINTER) * WIN_H / np.tan(np.radians(ELEV_WINTER))
FS_SUMMER = max(0.0, 1 - AWNING_DEPTH * np.tan(np.radians(ELEV_SUMMER)) / WIN_H)
PHI_GA = G_WINTER * WIN_A
PHI_SOLAR = PHI_GA * G_VALUE * FF_GLASS * FW_INCIDENCE * FS_WINTER * FC_WINTER
SLAB_L, SLAB_B, SLAB_D = 5.0, 4.0, 0.20
RHO_BETON, C_BETON, DT_SLAB = 2400.0, 1000.0, 2.0
#endregion


#region Shared
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the other Heating
    modules. Module 5's solar-gain chain — Q̇_S = I_s · A · g_⊥ · F_F · F_W · F_S · F_C —
    is the DIN V 4108-6 method (6.4 Solare Wärmegewinne, Anhang B.4 for F_C); the g-value beat
    cites DIN EN 410 next to it. Added in absolute frame coordinates.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Physical Fundamentals glyphs
# Sun, rays, light pulses and heat ripples come from the Physical Fundamentals
# vocabulary in ``manim_visuals``; these wrappers only fix the module's size,
# pastel colours and pulse pacing.
SUN_SCALE = 0.42


def _sun(pos):
    """🌞 Physical Fundamentals sun at the size every solar beat uses."""
    return sun_glyph(np.array(pos, dtype=float), PASTEL_YELLOW).scale(SUN_SCALE)


def _soft_ray(start, end, *, width: float = 2.2, opacity: float = 0.85, fade: bool = True):
    """🌅 Straight sun ray that fades in from transparent at its sun-side end."""
    ray = radiation_ray(start, end, color=COLOR_G, stroke_width=width)
    return ray.set_stroke(opacity=[0.0, opacity, opacity] if fade else opacity)


def _parallel_starts(sun_c, hits, gap: float = 0.5):
    """📐 Shared sun direction onto ``hits`` and each ray's start ``gap`` clear of the sun's centre."""
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array(h, dtype=float) for h in hits]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    return aim, [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]


def _pulses(paths, rt: float, *, color=COLOR_G, width: float = 5.0, every: float = 1.6, lead: float = 0.3):
    """⚡ Light pulses along ray polylines, as many rounds as fit into ``rt`` seconds.

    Each pulse starts ``lead`` of the way along the first leg, where a ray that fades in from its sun side is
    already visible, so no pulse is born in empty sky.
    """
    trimmed = [[np.asarray(p[0]) + lead * (np.asarray(p[1]) - np.asarray(p[0])), *p[1:]] for p in paths]
    return pulse_flashes(trimmed, color, repeats=max(1, int(rt / every)), width=width)
#endregion


#region Readouts into the panel
def _freeze(*mobs):
    """🧊 Stop live readouts from redrawing once their value has settled."""
    for mob in mobs:
        mob.clear_updaters()


def _wipe(scene, *mobs):
    """🧹 Zero fill/stroke opacity on every drawable leaf, then drop from the scene.

    ``FadeOut`` on nested ``math_text`` / ``Text`` leaves Cairo glyphs painted — parent
    VGroup opacity does not cascade to Text leaves. Explicit leaf wipe is required.
    https://docs.manim.community/en/stable/reference/manim.mobject.mobject.Mobject.html#manim.mobject.mobject.Mobject.family_members_with_points
    """
    alive = [m for m in mobs if m is not None]
    if not alive:
        return
    for mob in alive:
        mob.clear_updaters()
        for leaf in mob.family_members_with_points():
            leaf.set_fill(opacity=0)
            leaf.set_stroke(opacity=0)
            leaf.set_opacity(0)
        mob.set_opacity(0)
    scene.remove(*alive)


def _fly_into(scene, stage, prefix, panel, sources, *, run_time: float = 1.2):
    """🧲 Fly copies of on-stage readouts into their formula slots, then reveal the rest of the panel.

    Same idea as ``_fill_panel`` in ``1_physical_fundamentals/scene_1.py``.
    https://docs.manim.community/en/stable/reference/manim.animation.transform.FadeTransform.html
    """
    row, box, items = panel
    copies = {}
    for key, src in sources.items():
        copy = src.copy()
        copy.clear_updaters()
        for part in copy.get_family():
            part._layout_zone = "formula"
        copies[key] = copy
    scene.play(*[copy.animate.move_to(items[key]) for key, copy in copies.items()], run_time=run_time * 0.6)
    scene.play(*[stage.transition(f"{prefix}.{key}", source=copies[key], target=items[key]) for key in copies], run_time=run_time * 0.4)
    rest = VGroup(*[m for m in row.submobjects if all(m is not items[key] for key in sources)])
    scene.play(FadeIn(rest), Create(box), run_time=0.6)
#endregion


#region Sun geometry
# The sun is so far away that its rays reach the facade parallel. A room
# section is a dict of screen coordinates (facade plane ``x_w``, back wall
# ``x_b``, floor ``y_f``, sill ``y_s``, window top ``y_t``, ceiling ``y_c``);
# shades are axis-aligned boxes outside the facade whose lower outer edge
# decides where the grazing ray crosses the glass.
def _room_geo(x_w: float, y_f: float, s: float) -> dict:
    """📏 Screen geometry of the example room — facade plane ``x_w``, floor ``y_f``, ``s`` units per metre."""
    return {
        "x_w": x_w, "x_b": x_w - ROOM_DEPTH * s, "y_f": y_f, "y_s": y_f + SILL_H * s,
        "y_t": y_f + (SILL_H + WIN_H) * s, "y_c": y_f + ROOM_H * s, "s": s, "wall": 0.12, "slab": 0.16,
    }


def _block(x0, x1, y0, y1, *, color=PASTEL_WHITE, opacity=0.14):
    """🧱 Filled axis-aligned building element in the Physical Fundamentals slab style."""
    return Polygon([x0, y0, 0], [x1, y0, 0], [x1, y1, 0], [x0, y1, 0],
                   color=PASTEL_WHITE, stroke_width=2, fill_color=color, fill_opacity=opacity)


def _section_room(geo: dict, *, roof_tip=None) -> dict:
    """🏠 Room section: floor and ceiling slab (optionally running out as a roof overhang), back wall, glazed facade."""
    w, sl = geo["wall"], geo["slab"]
    tip = geo["x_w"] if roof_tip is None else roof_tip
    shell = VGroup(
        _block(geo["x_b"] - w, geo["x_w"], geo["y_f"] - sl, geo["y_f"]),
        _block(geo["x_b"] - w, tip, geo["y_c"], geo["y_c"] + sl),
        _block(geo["x_b"] - w, geo["x_b"], geo["y_f"], geo["y_c"]),
        _block(geo["x_w"] - w, geo["x_w"], geo["y_f"], geo["y_s"]),
        _block(geo["x_w"] - w, geo["x_w"], geo["y_t"], geo["y_c"]),
    )
    win = emphasize_casement(window_glyph(
        geo["x_w"] - w / 2, geo["y_s"], geo["y_t"], depth=0.85 * (geo["y_t"] - geo["y_s"]),
        color=COLOR_WIN, wall=w, inward=-1))
    glass = win["group"]
    ground = Line([geo["x_w"], geo["y_f"], 0], [6.6, geo["y_f"], 0], color=P_PRIMARY, stroke_width=2.2)
    return {
        "shell": shell, "glass": glass, "window": win, "ground": ground, "group": VGroup(ground, shell, glass),
        "roof_box": (geo["x_b"] - w, tip, geo["y_c"], geo["y_c"] + sl),
    }


def _sun_dir(elev: float):
    """🧭 Unit vector from the facade toward the sun at ``elev`` degrees (sun on the right)."""
    a = np.radians(elev)
    return np.array([np.cos(a), np.sin(a), 0.0])


def _lit_top(elev: float, geo: dict, edges) -> float:
    """🔝 Highest sunlit glass point — window top or where the ray grazing a shade's lower outer edge meets the facade."""
    t = np.tan(np.radians(elev))
    return min([geo["y_t"]] + [y0 - (x1 - geo["x_w"]) * t for x1, y0 in edges])


def _room_hit(p, elev: float, geo: dict):
    """🎯 Where a ray entering the facade at ``p`` lands — on the floor or, if it runs past the depth, on the back wall."""
    t = np.tan(np.radians(elev))
    run = (p[1] - geo["y_f"]) / t
    depth = geo["x_w"] - geo["x_b"]
    if run <= depth:
        return np.array([geo["x_w"] - run, geo["y_f"], 0.0])
    return np.array([geo["x_b"], p[1] - depth * t, 0.0])


def _sun_patch(elev: float, geo: dict, edges=()) -> list:
    """🟨 Sunlit section polygon between the ray over the lit glass top and the ray over the sill, clipped by floor and back wall.

    The patch can only fall away from the window, so the upper back corner of
    the room is never reached by direct sun.
    """
    top = _lit_top(elev, geo, edges)
    if top <= geo["y_s"] + 1e-3:
        return []
    a = np.array([geo["x_w"], top, 0.0])
    b = np.array([geo["x_w"], geo["y_s"], 0.0])
    ha, hb = _room_hit(a, elev, geo), _room_hit(b, elev, geo)
    on_wall = ha[0] <= geo["x_b"] + 1e-6
    on_floor = hb[1] <= geo["y_f"] + 1e-6 and hb[0] > geo["x_b"] + 1e-6
    corner = [np.array([geo["x_b"], geo["y_f"], 0.0])] if on_wall and on_floor else []
    return [a, ha, *corner, hb, b]


def _patch_mob(points, color=PASTEL_ORANGE):
    """🟧 Faint sunlit shaft plus the sunlit floor and back-wall stretch drawn bold."""
    if len(points) < 3:
        return VGroup()
    fill = Polygon(*points, stroke_width=0, fill_color=COLOR_G, fill_opacity=0.12)
    lit = VMobject(stroke_color=color, stroke_width=6, stroke_opacity=0.9)
    lit.set_points_as_corners(points[1:-1])
    return VGroup(fill, lit)


def _box_exit(p, u, box):
    """📦 Distance along ``u`` at which the ray from ``p`` leaves an axis-aligned box, or ``None`` if it misses."""
    lo, hi = 0.0, np.inf
    for pc, uc, a, b in ((p[0], u[0], box[0], box[1]), (p[1], u[1], box[2], box[3])):
        if abs(uc) < 1e-9:
            if not a <= pc <= b:
                return None
            continue
        t0, t1 = sorted(((a - pc) / uc, (b - pc) / uc))
        lo, hi = max(lo, t0), min(hi, t1)
    return hi if hi >= lo else None


RAY_REACH = 2.3


def _ray_paths(elev: float, geo: dict, boxes, heights, sun_at) -> list:
    """🧵 Polyline of every parallel ray at one elevation — sun side, then the facade or shade it stops at, then the room.

    ``heights`` are the facade-plane heights the rays aim at; every ray starts at
    most ``RAY_REACH`` out from the facade, so the parallel bundle never reads
    wider than the sun.
    """
    u = _sun_dir(elev)
    paths = []
    for h in heights:
        p = np.array([geo["x_w"], h, 0.0])
        hits = [t for t in (_box_exit(p, u, b) for b in boxes) if t is not None]
        start = p + u * min(RAY_REACH, max(float(np.dot(np.asarray(sun_at) - p, u)) - 0.6, 0.3))
        if hits:
            paths.append([start, p + u * max(hits)])
        elif geo["y_s"] <= h <= geo["y_t"]:
            paths.append([start, p, _room_hit(p, elev, geo)])
        else:
            paths.append([start, p])
    return paths


def _sun_rays(elev: float, geo: dict, boxes, heights, sun_at):
    """🔆 Parallel straight rays at one elevation — bright and fading in outside, dimmer past the glass."""
    return VGroup(*[
        VGroup(_soft_ray(path[0], path[1], width=1.8, opacity=0.75),
               *([_soft_ray(path[1], path[2], width=1.8, opacity=0.5, fade=False)] if len(path) > 2 else []))
        for path in _ray_paths(elev, geo, boxes, heights, sun_at)
    ])


def _patch_spots(points, geo: dict) -> list:
    """♨️ Ripple sources on a sun patch — ``(point, facing)`` on the lit floor and the lit back wall."""
    if len(points) < 3:
        return []
    lit = points[1:-1]
    floor = [q for q in lit if q[1] <= geo["y_f"] + 1e-6]
    wall = [q for q in lit if q[0] <= geo["x_b"] + 1e-6]
    spots = [(interpolate(floor[0], floor[-1], f) + UP * 0.03, None) for f in (0.3, 0.75)] if len(floor) > 1 else []
    if len(wall) > 1:
        spots.append(((wall[0] + wall[-1]) / 2 + RIGHT * 0.03, 0.0))
    return spots
#endregion


#region Beat1 — Verlust zu Gewinn
class Beat1_VerlustZuGewinn(Scene):
    """🔄 Envelope heat loss next to free solar gain."""

    NARRATION = [
        ("loss",
         r"Outside heat leaves through the envelope as transmission loss $Q_{\mathrm{T}}$.",
         r"Wärme verlässt die Hülle als Transmissionsverlust $Q_{\mathrm{T}}$."),
        ("flip",
         "At the same time the sun supplies free heat.",
         "Gleichzeitig liefert die Sonne freie Wärme."),
        ("gain",
         r"The solar heat gain $Q_{\mathrm{S}}$ arrives at roof, walls, and windows; with the internal gains $Q_{\mathrm{i}}$ it forms the gains $Q_{\mathrm{g}}$.",
         r"Der solare Wärmegewinn $Q_{\mathrm{S}}$ trifft Dach, Wände und Fenster; mit den internen Gewinnen $Q_{\mathrm{i}}$ bildet er die Gewinne $Q_{\mathrm{g}}$."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("From heat loss to solar gain", "Von Wärmeverlust zu solarem Gewinn"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "loss"))

        house = house_section(np.array([-0.2, -0.62, 0.0]), scale=1.15)
        x_l, x_r = house["bottom_left"][0], house["bottom_right"][0]
        y_lo, y_hi = house["windows"][0]["center"][1], house["windows"][1]["center"][1]
        eave_l = house["top_left"] + LEFT * 0.26 * 1.15
        peak = house["roof_peak"]

        def roof_y(x):
            return eave_l[1] + (peak[1] - eave_l[1]) * (1 - abs(x - peak[0]) / (peak[0] - eave_l[0]))

        def leak(x_in, y_in, x_out, side):
            return smooth_path([[x_in, y_in, 0], [x_out, y_in + 0.04, 0], [x_out + side * 0.55, y_in + 0.22, 0],
                                [x_out + side * 1.0, y_in + 0.5, 0]])

        def roof_leak(x):
            side = np.sign(x - peak[0])
            y = roof_y(x)
            return smooth_path([[x - side * 0.25, y - 0.45, 0], [x, y, 0], [x + side * 0.3, y + 0.42, 0],
                                [x + side * 0.65, y + 0.72, 0]])

        loss_paths = [leak(x_l + 0.85, y, x_l, -1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [leak(x_r - 0.85, y, x_r, 1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [roof_leak(peak[0] + dx) for dx in (-1.05, 1.05)]
        loss_guides = flow_guides(loss_paths, PASTEL_CYAN, opacity=0.3, visible=False)
        loss_label = VGroup(
            Text(tr("Heat loss", "Wärmeverlust"), font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
            math_text(r"Q_{\mathrm{T}}", font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
        ).arrange(DOWN, buff=0.14).move_to([4.75, -0.35, 0])

        def leaking(rt):
            return [flow_animation([(loss_paths, PASTEL_ORANGE, PASTEL_CYAN)], waves=4, radius=0.06,
                                   cycles=max(1.0, rt / 1.3))]

        self.play(FadeIn(caption), Create(house["group"]), run_time=1.4)
        self.play(FadeIn(loss_label), *leaking(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "loss", used=1.4 + 1.2, during=leaking)

        sun_c = np.array([-5.45, 1.45, 0.0])
        sun = _sun(sun_c)

        def glowing(rt):
            return [UpdateFromAlphaFunc(sun, lambda m, a: m.set_fill(opacity=0.96 + 0.04 * np.cos(TAU * a)), rate_func=linear)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        self.play(FadeOut(loss_label), FadeIn(sun, scale=0.7), run_time=1.2)
        hold_for(self, self.NARRATION, "flip", during=glowing)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gain"))
        d = np.array([1.0, -0.42, 0.0]) / np.linalg.norm([1.0, -0.42])
        roof_hits = [np.array([x, roof_y(x), 0.0]) for x in (peak[0] - 1.55, peak[0] - 0.8)]
        wall_hit = np.array([x_l, house["level_1"].get_center()[1], 0.0])
        floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
        glass_hits = [np.array([x_l, w["center"][1] + dy, 0.0]) for w in house["windows"] for dy in (0.1, -0.1)]
        lands = [h + d * (fy - h[1]) / d[1] for h, fy in zip(glass_hits, np.repeat(floors, 2))]
        reach = 2.1
        outer = VGroup(*[_soft_ray(h - d * reach, h) for h in (*roof_hits, wall_hit, *glass_hits)])
        inner = VGroup(*[_soft_ray(h, p, width=2.0, opacity=0.5, fade=False) for h, p in zip(glass_hits, lands)])
        patches = VGroup(*[Line(lands[k], lands[k + 1], color=COLOR_G, stroke_width=6, stroke_opacity=0.8)
                           for k in (0, 2)])
        gain_paths = [[h - d * reach, h] for h in (*roof_hits, wall_hit)]
        gain_paths += [[h - d * reach, h, p] for h, p in zip(glass_hits, lands)]
        warm = [(lands[k] + lands[k + 1]) / 2 + UP * 0.03 for k in (0, 2)]
        gain_label = VGroup(
            Text(tr("Solar gain", "Solarer Gewinn"), font_size=BODY_FONT_SIZE, color=COLOR_G),
            math_text(SYM_QS_E, font_size=BODY_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, buff=0.14).move_to(loss_label)

        def sunshine(rt):
            return [_pulses(gain_paths, rt), ripples(warm, r_max=0.5, color=PASTEL_ORANGE, cycles=max(1.0, rt / 1.4)),
                    *glowing(rt)]

        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in outer], lag_ratio=0.1), FadeIn(gain_label),
                  run_time=1.3)
        balance = math_label(rf"Q_{{\mathrm{{g}}}} = {SYM_QS_E} + Q_{{\mathrm{{i}}}}", np.array([4.75, -1.25, 0.0]),
                             size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in inner], lag_ratio=0.1), FadeIn(patches),
                  FadeIn(balance), run_time=0.8)
        hold_for(self, self.NARRATION, "gain", during=sunshine)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 — Bestrahlung und Fläche
def _bands(x0, x1, y0, y1, t: float, **style):
    """🔲 Four filled bands of thickness ``t`` lining the rectangle ``x0..x1 × y0..y1`` — a wall or frame around a hole."""
    return VGroup(*[
        Rectangle(width=w, height=h, **style).move_to([cx, cy, 0])
        for w, h, cx, cy in (
            (x1 - x0, t, (x0 + x1) / 2, y1 - t / 2), (x1 - x0, t, (x0 + x1) / 2, y0 + t / 2),
            (t, y1 - y0 - 2 * t, x0 + t / 2, (y0 + y1) / 2), (t, y1 - y0 - 2 * t, x1 - t / 2, (y0 + y1) / 2),
        )
    ])


class Beat2_BestrahlungUndFlaeche(Scene):
    """🔢 Irradiance I_s, area A and frame reduction factor F_F — read off the drawn window as symbols, then one formula."""

    NARRATION = [
        ("aperture",
         "Start from the full window opening in the wall.",
         "Zuerst die gesamte Fensteröffnung in der Wand."),
        ("g",
         r"The irradiance $I_{\mathrm{s}}$ is the solar power that reaches each square metre of the facade, in $\mathrm{W/m^2}$.",
         r"Die Strahlungsintensität $I_{\mathrm{s}}$ ist die Sonnenleistung, die auf jeden Quadratmeter der Fassade trifft, in $\mathrm{W/m^2}$."),
        ("a",
         r"The area $A$ in $\mathrm{m^2}$ is the window opening that the radiation can hit.",
         r"Die Fläche $A$ in $\mathrm{m^2}$ ist die Fensteröffnung, auf die die Strahlung trifft."),
        ("ff",
         r"The frame blocks; only the glazed fraction $F_{\mathrm{F}}$ lets the radiation through.",
         r"Der Rahmen blockiert, nur der Glasanteil $F_{\mathrm{F}}$ lässt die Strahlung durch."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("Irradiance and window area", "Strahlungsintensität und Fensterfläche"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "aperture"))
        self.play(FadeIn(caption), run_time=0.3)

        s = 1.1
        oc = np.array([-2.9, 0.35, 0.0])
        ow, oh = WIN_W * s, WIN_H * s
        ox0, ox1, oy0, oy1 = oc[0] - ow / 2, oc[0] + ow / 2, oc[1] - oh / 2, oc[1] + oh / 2
        wall = VGroup(
            section_material(Rectangle(width=3.6, height=oc[1] + 1.65 - oy1)).move_to([oc[0], (oy1 + oc[1] + 1.65) / 2, 0]),
            section_material(Rectangle(width=3.6, height=oy0 - (oc[1] - 1.65))).move_to([oc[0], (oy0 + oc[1] - 1.65) / 2, 0]),
            section_material(Rectangle(width=1.8 - ow / 2, height=oh)).move_to([(oc[0] - 1.8 + ox0) / 2, oc[1], 0]),
            section_material(Rectangle(width=1.8 - ow / 2, height=oh)).move_to([(oc[0] + 1.8 + ox1) / 2, oc[1], 0]),
        ).set_fill(opacity=0.4)
        opening = Rectangle(width=ow, height=oh, color=COLOR_A, stroke_width=3).move_to(oc)
        self.play(Create(wall), Create(opening), run_time=1.3)
        hold_for(self, N, "aperture", used=1.3 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        sun_c = np.array([4.4, 1.6, 0.0])
        sun = _sun(sun_c)
        frame_t = FRAME_FACE * s
        frame_hits = [np.array(p) for p in ((ox1 - frame_t / 2, 0.95, 0), (oc[0] + 0.25, oy1 - frame_t / 2, 0),
                                            (oc[0] - 0.35, oy0 + frame_t / 2, 0), (ox0 + frame_t / 2, 0.2, 0))]
        glass_hits = [np.array(p) for p in ((-3.12, 0.92, 0), (-2.6, 0.78, 0), (-3.15, -0.12, 0),
                                            (-2.58, -0.2, 0), (-2.88, -0.28, 0))]
        aim, flat = _parallel_starts(sun_c, frame_hits + glass_hits, gap=0.55)
        starts = [sun_c + aim * 0.55 + 0.22 * (f - sun_c - aim * 0.55) for f in flat]
        rays = VGroup(*[_soft_ray(a, h, width=2.0) for a, h in zip(starts, frame_hits + glass_hits)])
        frame_rays, glass_rays = rays[:len(frame_hits)], rays[len(frame_hits):]
        all_paths = [[a, h] for a, h in zip(starts, frame_hits + glass_hits)]
        glass_paths = all_paths[len(frame_hits):]
        g_lbl = math_label(SYM_IS, sun_c + DOWN * 1.15, size=FORMULA_FONT_SIZE, color=COLOR_G)
        g_val = math_label(rf"{label_number(G_WINTER)}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}}}}}", sun_c + DOWN * 1.9,
                           size=BODY_FONT_SIZE, color=COLOR_G)

        def sunshine(rt):
            return [_pulses(all_paths, rt, every=1.8, lead=0.12)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.08), FadeIn(g_lbl), FadeIn(g_val),
                  run_time=1.6)
        hold_for(self, N, "g", during=sunshine)

        caption = swap_caption(self, caption, subtitle_text(N, "a"))
        area = Rectangle(width=ow, height=oh, stroke_width=0, fill_color=COLOR_A, fill_opacity=0.35).move_to(oc)
        a_lbl = math_label("A", oc, size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(GrowFromEdge(area, DOWN), FadeIn(a_lbl), sunshine(1.2)[0], run_time=1.2)
        hold_for(self, N, "a", during=sunshine)

        caption = swap_caption(self, caption, subtitle_text(N, "ff"))
        frame = _bands(ox0, ox1, oy0, oy1, frame_t, color=PASTEL_WHITE, stroke_width=1.5, fill_color=PASTEL_WHITE,
                       fill_opacity=0.45)
        glass = Rectangle(width=(WIN_W - 2 * FRAME_FACE) * s, height=(WIN_H - 2 * FRAME_FACE) * s,
                          color=COLOR_WIN, fill_color=COLOR_WIN, fill_opacity=0.15, stroke_width=2).move_to(oc)
        ff_lbl = math_label(SYM_FF, oc, size=FORMULA_FONT_SIZE, color=COLOR_FF)
        self.play(FadeOut(area), FadeIn(frame), run_time=0.7)
        self.play(stage.transition("heating.solar-gains.ff.glass-from-opening", source=opening, target=glass), stage.transition("heating.solar-gains.ff.area-label-to-frame-factor", source=a_lbl, target=ff_lbl),
                  frame_rays.animate.set_stroke(color=GREY_C, opacity=[0.0, 0.35, 0.35]), run_time=1.0)
        self.add(frame_rays, frame, glass, glass_rays)
        self.bring_to_front(ff_lbl)
        a_side = math_label("A", oc + LEFT * (ow / 2 + 0.45), size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(FadeIn(a_side), run_time=0.5)

        def through_glass(rt):
            return [_pulses(glass_paths, rt, every=1.8, lead=0.12), glass.animate.set_fill(COLOR_WIN, opacity=0.3)]

        panel = math_panel([
            ("phi", SYM_QS, PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g", SYM_IS, COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", SYM_FF, COLOR_FF), (None, UNIT_TAIL, PASTEL_WHITE),
        ])
        _fly_into(self, stage, "heating.solar-gains.ff.fly", panel, {"g": g_lbl, "a": a_side, "ff": ff_lbl})
        ring = highlight_param(panel[2], "ff", color=COLOR_FF)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "ff", during=through_glass)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 — g-Wert
def _glazing(x: float, y0: float, y1: float):
    """🪟 Double glazing in section: two panes, spacer and frame blocks."""
    panes = VGroup(*[
        Rectangle(width=0.07, height=y1 - y0, stroke_color=COLOR_WIN, stroke_width=2,
                  fill_color=COLOR_WIN, fill_opacity=0.15).move_to([x + dx, (y0 + y1) / 2, 0])
        for dx in (-0.13, 0.13)
    ])
    frames = VGroup(*[
        Rectangle(width=0.5, height=0.22, stroke_color=PASTEL_WHITE, stroke_width=2,
                  fill_color=PASTEL_WHITE, fill_opacity=0.3).move_to([x, y, 0])
        for y in (y0 - 0.11, y1 + 0.11)
    ])
    return panes, frames


class Beat3_GWert(Scene):
    """🌈 Energy split at the glazing: reflected, absorbed, transmitted — g_⊥ sits at the glazing and is what reaches the room."""

    NARRATION = [
        ("section",
         "A section through the glazing: outside on the left, inside on the right.",
         "Ein Schnitt durch die Verglasung: links außen, rechts innen."),
        ("split",
         "Of the solar radiation about 20 percent is reflected, 30 percent taken up by the glass and 50 percent let straight through.",
         "Von der Sonnenstrahlung werden rund 20 % reflektiert, 30 % vom Glas aufgenommen und 50 % direkt durchgelassen."),
        ("gval",
         r"The warm glass passes a part of its heat inwards: 50 plus 10 percent reach the room, so $g_{\perp} \approx 0.6$.",
         r"Das warme Glas gibt einen Teil seiner Wärme nach innen ab: 50 plus 10 % erreichen den Raum, also $g_{\perp} \approx 0{,}6$."),
        ("phi",
         r"The total solar energy transmittance $g_{\perp}$ is dimensionless and enters the equation as one more factor.",
         r"Der Gesamtenergiedurchlassgrad $g_{\perp}$ ist dimensionslos und kommt als weiterer Faktor in die Gleichung."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("Total solar energy transmittance g⊥", "Gesamtenergiedurchlassgrad g⊥"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar"), ("din-en-410", "g")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "section"))
        self.play(FadeIn(caption), run_time=0.3)

        panes, frames = _glazing(0.0, -1.35, 1.75)
        x_block_outer, x_block_inner = -0.165, 0.165
        side_out = Text(tr("outside", "außen"), font_size=BODY_FONT_SIZE, color=P_PRIMARY).move_to([-6.1, 1.9, 0])
        side_in = Text(tr("inside", "innen"), font_size=BODY_FONT_SIZE, color=P_PRIMARY).move_to([6.1, 2.1, 0])
        self.play(Create(panes), FadeIn(frames), FadeIn(side_out), FadeIn(side_in), run_time=1.2)
        hold_for(self, N, "section", used=1.2 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(N, "split"))
        sun_pos = np.array([-5.6, 1.3, 0.0])
        sun = _sun(sun_pos)
        hit_out = np.array([x_block_outer, 0.9, 0.0])
        d_in = hit_out - sun_pos
        d_in = d_in / np.linalg.norm(d_in)
        d_ref = np.array([-d_in[0], d_in[1], 0.0])
        d_glass = np.array([d_in[0], d_in[1] * 0.6, 0.0])
        d_glass = d_glass / np.linalg.norm(d_glass)
        hit_in = hit_out + d_glass * ((x_block_inner - x_block_outer) / d_glass[0])
        ray_end = hit_in + d_in * 6.0
        refl_end = hit_out + d_ref * 2.8

        incoming = radiation_ray(sun_pos + d_in * 0.45, hit_out, color=COLOR_G, stroke_width=2.6)
        reflected = radiation_ray(hit_out, refl_end, color=COLOR_REFL, stroke_width=2.2).set_stroke(opacity=0.75)
        glass_seg = radiation_ray(hit_out, hit_in, color=COLOR_G, stroke_width=1.8).set_stroke(opacity=0.55)
        transmitted = radiation_ray(hit_in, ray_end, color=COLOR_G, stroke_width=2.6)
        ray_path = [[sun_pos + d_in * 0.45, hit_out, hit_in, ray_end]]
        refl_path = [[sun_pos + d_in * 0.45, hit_out, refl_end]]

        in_lbl = Text("100 %", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
        in_lbl.next_to(sun_pos + d_in * 2.0, UP, buff=0.22)
        refl_lbl = Text(tr("reflected 20 %", "reflektiert 20 %"), font_size=LABEL_FONT_SIZE, color=COLOR_REFL)
        refl_lbl.move_to((hit_out + refl_end) / 2 + DOWN * 0.45)
        trans_lbl = Text(tr("transmitted 50 %", "durchgelassen 50 %"), font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
        trans_lbl.next_to(hit_in + d_in * 2.4, UP, buff=0.25)
        abs_lbl = Text(tr("in the glass 30 %", "im Glas 30 %"), font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT).move_to([0.0, 2.15, 0])

        def sunshine(rt):
            n = max(1, int(rt / 1.6))
            return [pulse_flashes(ray_path, COLOR_G, repeats=n, width=3.2),
                    pulse_flashes(refl_path, COLOR_REFL, repeats=n, width=2.4)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.5)
        self.play(Create(incoming, rate_func=linear), FadeIn(in_lbl), run_time=1.0)
        self.play(
            Create(reflected, rate_func=linear), Create(glass_seg, rate_func=linear),
            Create(transmitted, rate_func=linear), FadeIn(refl_lbl), FadeIn(trans_lbl),
            panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.7), FadeIn(abs_lbl),
            run_time=1.8,
        )
        hold_for(self, N, "split", during=sunshine)

        caption = swap_caption(self, caption, subtitle_text(N, "gval"))
        heat_y = -0.55
        warm_in = [np.array([x_block_inner, heat_y, 0.0])]

        def glass_heat(rt):
            return [*sunshine(rt), ripples(warm_in, r_max=0.5, color=COLOR_GLASS_HEAT, cycles=max(1.0, rt / 1.4), facing=0.0)]

        g_lbl = math_label(rf"{SYM_G} = {label_number(G_VALUE, 2)}", np.array([0.95, heat_y, 0.0]),
                           size=FORMULA_FONT_SIZE, color=COLOR_GVAL, edge="left")
        g_lead = Line([x_block_inner + 0.12, heat_y, 0], [g_lbl.get_left()[0] - 0.08, heat_y, 0], color=COLOR_GVAL, stroke_width=2.2)
        g_sum = Text(tr("50 % + 10 % = 60 % reach the room", "50 % + 10 % = 60 % erreichen den Raum"),
                     font_size=LABEL_FONT_SIZE, color=COLOR_GVAL).next_to(g_lbl, DOWN, buff=0.2, aligned_edge=LEFT)
        self.play(FadeIn(g_lbl, shift=RIGHT * 0.1), FadeIn(g_sum), Create(g_lead), panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.45),
                  *glass_heat(1.6), run_time=1.6)
        hold_for(self, N, "gval", during=glass_heat)

        caption = swap_caption(self, caption, subtitle_text(N, "phi"))
        g_sym = math_label(SYM_G, size=FORMULA_FONT_SIZE, color=COLOR_GVAL)
        g_sym.move_to(g_lbl.get_left() + RIGHT * g_sym.width / 2)
        panel = math_panel([
            ("phi", SYM_QS, PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", SYM_IS, COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("g", SYM_G, COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("ff", SYM_FF, COLOR_FF), (None, UNIT_TAIL, PASTEL_WHITE),
        ])
        _fly_into(self, stage, "heating.solar-gains.g.fly", panel, {"g": g_sym})
        ring = highlight_param(panel[2], "g", color=COLOR_GVAL)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "phi", during=glass_heat)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 — Saisonale Sonnenwinkel
class Beat4_SaisonaleWinkel(Scene):
    """🗓️ Parallel sun rays at 60° and 20°: the overhang blocks summer, winter light lands on floor and lower back wall."""

    NARRATION = [
        ("path",
         "The sun path climbs high in summer and stays low in winter.",
         "Die Sonnenbahn steht im Sommer hoch und im Winter flach."),
        ("summer",
         "The sun is so far away that its rays arrive parallel. At 60 degrees the roof overhang holds nearly all of them back.",
         "Die Sonne ist so weit weg, dass ihre Strahlen parallel einfallen. Bei 60° hält der Dachüberstand fast alle ab."),
        ("winter",
         "At 20 degrees the rays fall flat onto the floor and at most the lower back wall — the upper back corner gets no direct sun.",
         "Bei 20° fallen die Strahlen flach auf den Boden und höchstens unten auf die Rückwand — oben hinten kommt keine Sonne an."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("Seasonal sun angles", "Saisonale Sonnenwinkel"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "path"))
        self.play(FadeIn(caption), run_time=0.3)

        geo = _room_geo(1.0, -1.9, 1.05)
        tip = geo["x_w"] + ROOF_OVERHANG * geo["s"]
        room = _section_room(geo, roof_tip=tip)
        boxes = [room["roof_box"]]
        edges = [(tip, geo["y_c"])]
        heights = np.linspace(geo["y_f"] + 0.15, geo["y_c"] + geo["slab"] - 0.03, 10)
        hub = np.array([geo["x_w"], geo["y_f"], 0.0])
        radius = 3.9
        elev = ValueTracker(ELEV_SUMMER)

        def sun_at():
            return hub + radius * _sun_dir(elev.get_value())

        arc = DashedVMobject(Arc(
            radius=radius, start_angle=np.radians(ELEV_WINTER), angle=np.radians(ELEV_SUMMER - ELEV_WINTER),
            arc_center=hub, color=PASTEL_YELLOW, stroke_width=2,
        ), num_dashes=24)
        arc_label = Text(tr("Sun path", "Sonnenbahn"), font_size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        arc_label.next_to(hub + radius * _sun_dir(ELEV_WINTER), DR, buff=0.55)
        sun = _sun(sun_at())
        sun.add_updater(lambda m: m.move_to(sun_at()))
        elev_read = math_readout(lambda: tr(rf"\text{{Sun elevation}}\;{label_number(elev.get_value())}°", rf"\text{{Sonnenhöhe}}\;{label_number(elev.get_value())}°"),
                                 [4.3, 1.95, 0], size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer = Text(tr("Summer: overhang blocks steep rays", "Sommer: Überhang hält steile Strahlen ab"), font_size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer.move_to([-6.5 + tag_summer.width / 2, 1.95, 0])
        tag_winter = Text(tr("Winter: low rays reach the lower back wall", "Winter: flache Strahlen bis zur unteren Rückwand"), font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        tag_winter.move_to([-6.5 + tag_winter.width / 2, 1.95, 0])
        dark_note = Text(tr("upper back: no direct sun", "oben hinten: keine direkte Sonne"), font_size=LABEL_FONT_SIZE, color=GREY_B)
        dark_note.move_to([geo["x_b"] + 0.18 + dark_note.width / 2, geo["y_c"] - 0.3, 0])

        self.play(Create(room["group"]), Create(arc), FadeIn(arc_label), run_time=1.6)
        self.add(sun)
        self.play(FadeIn(sun, scale=0.6), FadeIn(elev_read), run_time=0.9)
        hold_for(self, N, "path", used=1.6 + 0.9 + 0.3,
                 during=lambda rt: [elev.animate(rate_func=there_and_back, run_time=rt).set_value(ELEV_WINTER)])

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        rays_now = _sun_rays(ELEV_SUMMER, geo, boxes, heights, sun_at())
        summer_paths = _ray_paths(ELEV_SUMMER, geo, boxes, heights, sun_at())
        patch_now = _patch_mob(_sun_patch(ELEV_SUMMER, geo, edges))
        self.play(LaggedStart(*[Create(r) for r in rays_now], lag_ratio=0.1), FadeIn(patch_now), FadeIn(tag_summer),
                  run_time=1.6)
        hold_for(self, N, "summer", during=lambda rt: [_pulses(summer_paths, rt, every=1.8)])

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        rays = always_redraw(lambda: _sun_rays(elev.get_value(), geo, boxes, heights, sun_at()))
        patch = always_redraw(lambda: _patch_mob(_sun_patch(elev.get_value(), geo, edges)))
        self.play(FadeOut(rays_now), FadeOut(patch_now), run_time=0.6)
        self.add(patch, rays)
        self.play(
            elev.animate.set_value(ELEV_WINTER),
            stage.transition("heating.solar-gains.season.tag-summer-to-winter", source=tag_summer, target=tag_winter),
            run_time=3.0, rate_func=smooth,
        )
        _freeze(rays, patch, sun, elev_read)
        winter_paths = _ray_paths(ELEV_WINTER, geo, boxes, heights, sun_at())
        spots = _patch_spots(_sun_patch(ELEV_WINTER, geo, edges), geo)

        def winter_sun(rt):
            cycles = max(1.0, rt / 1.4)
            return [_pulses(winter_paths, rt, every=1.8),
                    *[ripples([c], r_max=0.6, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]

        self.play(FadeIn(dark_note), *winter_sun(0.6), run_time=0.6)
        hold_for(self, N, "winter", during=winter_sun)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 — Verschattung
COLOR_BLIND = PASTEL_TEAL
FC_BLIND = _example.F_C_BLIND
BLIND_CAST = "shared-blind-lowered-mirrored"


def _blind_slats(to_scene):
    """🎚️ Sun-facing face of every lamella of the blind cast as (outer end, inner end) in scene coordinates, top to bottom.

    @see ../../../../🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json — the lamella polygons come from the rendered cast
    """
    return [(np.append(to_scene(outer), 0.0), np.append(to_scene(inner), 0.0)) for outer, inner in lamella_faces(staffage_parts(BLIND_CAST, "slat-"))]


def _first_slat_hit(slats, through, direction):
    """🔍 Where the ray along ``direction`` through ``through`` first reaches a lamella, with the lamella's index, or None."""
    best = None
    for index, (outer, inner) in enumerate(slats):
        axis = inner - outer
        matrix = np.array([[direction[0], -axis[0]], [direction[1], -axis[1]]])
        if abs(np.linalg.det(matrix)) < 1e-9:
            continue
        along, position = np.linalg.solve(matrix, (outer - through)[:2])
        if 0.0 <= position <= 1.0 and (best is None or along < best[0]):
            best = (float(along), index, outer + axis * position)
    return None if best is None else (best[2], best[1])


class Beat5_Verschattung(Scene):
    """⛱️ Vertical section: unshaded beam, a Raffstore that reflects it (F_C), the residue, then the roof overhang (F_S) — both on a 0…1 scale."""

    NARRATION = [
        ("intro",
         "A vertical section through window and facade.",
         "Ein Vertikalschnitt durch Fenster und Fassade."),
        ("unshaded",
         r"Without shading the full radiation reaches the glass and the room: $F_{\mathrm{C}} = 1$.",
         r"Ohne Sonnenschutz trifft die volle Strahlung Glas und Raum: $F_{\mathrm{C}} = 1$."),
        ("raffstore",
         "An external Raffstore drops in front. Its slats catch the radiation and send it back outside.",
         "Ein außenliegender Raffstore senkt sich davor. Seine Lamellen fangen die Strahlung ab und werfen sie zurück."),
        ("reduced",
         r"Only a small residue gets through — the sun-protection factor $F_{\mathrm{C}}$ falls to about 0.15.",
         r"Nur ein Restanteil kommt durch — der Sonnenschutzfaktor $F_{\mathrm{C}}$ sinkt auf etwa 0,15."),
        ("winter",
         r"In winter the blind stays up; the roof overhang shades the low sun only a little — shading factor $F_{\mathrm{S}}$ about 0.8.",
         r"Im Winter bleibt der Raffstore oben. Der Dachüberstand verschattet die flache Sonne nur wenig — Verschattungsfaktor $F_{\mathrm{S}}$ rund 0,8."),
        ("fsh",
         r"$F_{\mathrm{S}}$ and $F_{\mathrm{C}}$ are the last two factors of the solar equation.",
         r"$F_{\mathrm{S}}$ und $F_{\mathrm{C}}$ sind die letzten beiden Faktoren der solaren Gleichung."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("Sun protection and shading", "Sonnenschutz und Verschattung"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Exterior wall ~ between room slab and a thick masonry wall — same family as room_section.
        facade_t = 0.24
        glass_out, glass_in = -3.65, -3.65 + facade_t
        wall_cx = (glass_out + glass_in) / 2
        wall_top, wall_bottom = 1.55, -0.85
        lintel = section_material(Rectangle(width=facade_t, height=0.55)).move_to([wall_cx, wall_top + 0.275, 0])
        sill = section_material(Rectangle(width=facade_t, height=0.55)).move_to([wall_cx, wall_bottom - 0.275, 0])
        masonry = VGroup(lintel, sill).set_z_index(3)
        shade_win = emphasize_casement(window_glyph(
            wall_cx, wall_bottom, wall_top, depth=0.85 * (wall_top - wall_bottom),
            color=COLOR_WIN, wall=facade_t, inward=1))
        glazing = shade_win["group"]
        glazing.set_z_index(3)
        slab_left, room_end_x = glass_out, -0.7
        ceiling_y, floor_y = lintel.get_top()[1], sill.get_bottom()[1]
        t = 0.20
        room = VGroup(
            section_material(Rectangle(width=room_end_x + t - slab_left, height=t)).move_to(
                [(slab_left + room_end_x + t) / 2, ceiling_y + t / 2, 0]),
            section_material(Rectangle(width=room_end_x + t - slab_left, height=t)).move_to(
                [(slab_left + room_end_x + t) / 2, floor_y - t / 2, 0]),
            section_material(Rectangle(width=t, height=ceiling_y - floor_y)).move_to(
                [room_end_x + t / 2, (ceiling_y + floor_y) / 2, 0]),
        )
        lbl_out = Text(tr("outside", "außen"), font_size=BODY_FONT_SIZE, color=P_PRIMARY).move_to([-6.2, 0.35, 0])
        lbl_in = Text(tr("inside", "innen"), font_size=BODY_FONT_SIZE, color=P_PRIMARY).move_to([-1.55, 1.6, 0])
        material = section_assembly(masonry, room)
        self.play(FadeIn(material), Create(glazing), run_time=1.6)
        self.play(FadeIn(lbl_out), FadeIn(lbl_in), run_time=0.6)
        hold_for(self, N, "intro", used=2.2 + 0.3)

        d = np.array([0.75, -0.661, 0.0])
        sun_c = np.array([-6.25, 1.85, 0.0])
        sun = _sun(sun_c)
        glass_ys = [wall_top - 1.0 - i * 0.32 for i in range(6)]

        def ray(end, back=3.0, **kw):
            return _soft_ray(end - d * back, end, **kw).set_z_index(1)

        def through(start):
            t_hit = min(t for t in ((floor_y - start[1]) / d[1], (room_end_x - start[0]) / d[0]) if t > 0.05)
            return start + d * t_hit

        def warm_spots(ends):
            return [(e + UP * 0.03, None) if e[1] <= floor_y + 1e-6 else (e + LEFT * 0.03, PI) for e in ends]

        def inside(ys, **kw):
            return VGroup(*[_soft_ray([glass_in, y, 0], through(np.array([glass_in, y, 0.0])), fade=False, **kw)
                            .set_z_index(1) for y in ys])

        direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys])
        interior = inside(glass_ys[:5], width=2.4, opacity=0.6)
        direct_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(direct, interior)]
        direct_spots = warm_spots([s.get_end() for s in interior[::2]])

        sx0, sx1, sy = 1.3, 5.9, 0.35
        scale = VGroup(
            Line([sx0, sy, 0], [sx1, sy, 0], color=PASTEL_WHITE, stroke_width=2),
            *[Line([x, sy - 0.12, 0], [x, sy + 0.12, 0], color=PASTEL_WHITE, stroke_width=2) for x in (sx0, sx1)],
        )
        end_l = VGroup(math_label("0", size=BODY_FONT_SIZE), Text(tr("fully protected", "voll geschützt"), font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_l.next_to([sx0, sy, 0], DOWN, buff=0.25)
        end_r = VGroup(math_label("1", size=BODY_FONT_SIZE), Text(tr("no protection", "ohne Schutz"), font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_r.next_to([sx1, sy, 0], DOWN, buff=0.25)
        fsh = ValueTracker(1.0)

        def sx(v):
            return sx0 + v * (sx1 - sx0)

        marker = always_redraw(lambda: Triangle(stroke_width=0, fill_color=COLOR_G, fill_opacity=1.0)
                               .scale(0.15).rotate(PI).move_to([sx(fsh.get_value()), sy + 0.24, 0]))
        mode = ValueTracker(0.0)
        marker_lbl = math_readout(lambda: rf"{SYM_FC if mode.get_value() < 0.5 else SYM_FS} = {label_number(fsh.get_value(), 2)}",
                                  lambda: np.array([sx(fsh.get_value()), sy + 0.55, 0]),
                                  size=BODY_FONT_SIZE, color=COLOR_G, edge="center")

        def sunlit(paths, spots):
            def during(rt):
                cycles = max(1.0, rt / 1.4)
                return [_pulses(paths, rt, every=1.8),
                        *[ripples([c], r_max=0.5, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]
            return during

        caption = swap_caption(self, caption, subtitle_text(N, "unshaded"))
        self.play(FadeIn(sun, scale=0.7), run_time=0.6)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in interior], lag_ratio=0.1), run_time=1.2)
        self.play(Create(scale), FadeIn(end_l), FadeIn(end_r), *sunlit(direct_paths, direct_spots)(0.8), run_time=0.8)
        self.add(marker, marker_lbl)
        self.play(FadeIn(marker), run_time=0.4)
        hold_for(self, N, "unshaded", during=sunlit(direct_paths, direct_spots))

        caption = swap_caption(self, caption, subtitle_text(N, "raffstore"))
        cast_sill, cast_head = (staffage_parts(BLIND_CAST, name)[name] for name in ("block-sill", "block-head"))
        sill_top, head_bottom = float(cast_sill[:, 1].max()), float(cast_head[:, 1].min())
        unit = (wall_top - wall_bottom) / (head_bottom - sill_top)
        anchor = np.array([lintel.get_left()[0], wall_bottom - sill_top * unit, 0.0])

        def to_scene(point):
            return anchor[:2] + np.asarray(point) * unit

        blind = staffage(BLIND_CAST, unit=unit, anchor=anchor,
                         window=(-1.0, sill_top - 1e-3, 0.0, head_bottom + 1.0)).set_z_index(4)
        slats = _blind_slats(to_scene)
        strikes = []
        for y in glass_ys:
            glass_point = np.array([glass_out, y, 0.0])
            strike = _first_slat_hit(slats, glass_point, d)
            if strike is not None:
                strikes.append((glass_point, *strike))
        rail = to_scene(staffage_parts(BLIND_CAST, "bottom-rail")["bottom-rail"].min(axis=0))
        lbl_blind = Text(tr("Venetian blind", "Raffstore"), font_size=LABEL_FONT_SIZE, color=COLOR_BLIND)
        lbl_blind.move_to([rail[0] - 0.15 - lbl_blind.width / 2, rail[1] - 0.15, 0])

        def face_normal(index):
            outer, inner = slats[index]
            axis = (inner - outer) / np.linalg.norm(inner - outer)
            normal = np.array([-axis[1], axis[0], 0.0])
            return normal if float(np.dot(normal, -d)) > 0 else -normal

        def toward(glass_point, point):
            return 3.0 + float(np.dot(point - glass_point, d) / np.dot(d, d))

        blocked = VGroup(*[ray(point, back=toward(glass_point, point), width=2.4, opacity=0.8)
                           for glass_point, point, _ in strikes])
        reflected = VGroup(*[radiation_ray(point, point + (d - 2 * float(np.dot(d, face_normal(index))) * face_normal(index)) * 1.3,
                                           color=COLOR_G, stroke_width=2.2).set_stroke(opacity=0.55).set_z_index(5)
                             for _, point, index in strikes])
        bounce_paths = [[b.get_start(), b.get_end(), r.get_end()] for b, r in zip(blocked, reflected)]
        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), fsh.animate.set_value(0.0), run_time=1.6)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15),
                  stage.transition("heating.solar-gains.blind.rays-blocked", source=direct, target=blocked),
                  LaggedStart(*[Create(a, rate_func=linear) for a in reflected], lag_ratio=0.08), FadeOut(interior),
                  run_time=1.6)
        hold_for(self, N, "raffstore", during=sunlit(bounce_paths, []))

        caption = swap_caption(self, caption, subtitle_text(N, "reduced"))
        residual = VGroup(*[radiation_ray(slats[index][1], through(slats[index][1]), color=COLOR_G,
                                          stroke_width=1.4).set_stroke(opacity=0.32).set_z_index(1)
                            for _, _, index in strikes[:5]])
        rest_paths = [[r.get_start(), r.get_end()] for r in residual]

        def shaded(rt):
            return [*sunlit(bounce_paths, [])(rt), _pulses(rest_paths, rt, width=2.5, every=1.8, lead=0.0)]

        self.play(LaggedStart(*[Create(r) for r in residual], lag_ratio=0.1), fsh.animate.set_value(FC_BLIND),
                  run_time=1.8)
        hold_for(self, N, "reduced", during=shaded)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        overhang = section_material(Rectangle(width=1.25, height=0.20)).move_to([glass_out - 0.62, wall_top + 0.08, 0]).set_z_index(4)
        lbl_over = Text(tr("Roof overhang", "Dachüberstand"), font_size=LABEL_FONT_SIZE, color=COLOR_FRAME)
        lbl_over.move_to([overhang.get_right()[0] + 0.2 - lbl_over.width / 2, lintel.get_top()[1] + 0.25, 0])
        winter_direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys[1:]])
        winter_cut = ray(np.array([glass_out, glass_ys[0], 0.0]), back=3.0 - 1.45, width=2.4,
                         opacity=0.8).shift(-d * 1.45)
        winter_in = inside(glass_ys[1:5], width=2.4, opacity=0.6)
        winter_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(winter_direct, winter_in)]
        winter_paths.append([winter_cut.get_start(), winter_cut.get_end()])
        winter_spots = warm_spots([s.get_end() for s in winter_in[::2]])
        self.play(blind.animate.shift(UP * 3.0).set_opacity(0), FadeOut(lbl_blind), FadeOut(reflected),
                  FadeOut(residual), FadeOut(blocked), run_time=1.2)
        self.play(FadeIn(overhang), FadeIn(lbl_over), Create(winter_cut, rate_func=linear),
                  LaggedStart(*[Create(r, rate_func=linear) for r in winter_direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in winter_in], lag_ratio=0.1),
                  fsh.animate.set_value(FS_WINTER), mode.animate.set_value(1.0), run_time=1.6)
        winter_sun = sunlit(winter_paths, winter_spots)
        hold_for(self, N, "winter", during=winter_sun)

        caption = swap_caption(self, caption, subtitle_text(N, "fsh"))
        _freeze(marker, marker_lbl)
        sym = math_label(SYM_FS, size=BODY_FONT_SIZE, color=COLOR_G)
        sym.move_to(marker_lbl.get_left() + RIGHT * sym.width / 2)
        panel = math_panel([
            ("phi", SYM_QS, PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", SYM_IS, COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("g", SYM_G, COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("ff", SYM_FF, COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("fw", SYM_FW, COLOR_FW), (None, r"\cdot", PASTEL_WHITE),
            ("fsh", SYM_FS, COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("fc", SYM_FC, COLOR_FC), (None, UNIT_TAIL, PASTEL_WHITE),
        ], edge_buff=1.35)
        _fly_into(self, stage, "heating.solar-gains.fsh.fly", panel, {"fsh": sym})
        ring = highlight_param(panel[2], "fsh", color=COLOR_G)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "fsh", during=winter_sun)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 — Hauptgleichung
class Beat8_Hauptgleichung(Scene):
    """🧮 Master equation Q̇_S = I_s · A · g_⊥ · F_F · F_W · F_S · F_C (DIN V 4108-6), counted through with this module's window."""

    NARRATION = [
        ("assemble",
         r"Put it together: $\dot{Q}_{\mathrm{S}} = I_{\mathrm{s}} \cdot A \cdot g_{\perp} \cdot F_{\mathrm{F}} \cdot F_{\mathrm{W}} \cdot F_{\mathrm{S}} \cdot F_{\mathrm{C}}$ in watts.",
         r"Zusammen: $\dot{Q}_{\mathrm{S}} = I_{\mathrm{s}} \cdot A \cdot g_{\perp} \cdot F_{\mathrm{F}} \cdot F_{\mathrm{W}} \cdot F_{\mathrm{S}} \cdot F_{\mathrm{C}}$ in Watt."),
        ("chain",
         r"For this window $768\,\mathrm{W}$ hit the opening; after all factors about $232\,\mathrm{W}$ arrive in the room.",
         r"Für dieses Fenster treffen $768\,\mathrm{W}$ auf die Öffnung; nach allen Faktoren kommen rund $232\,\mathrm{W}$ im Raum an."),
        ("ring",
         r"Surroundings and sun protection decide how much sun counts: $F_{\mathrm{S}}$ and $F_{\mathrm{C}}$.",
         r"Umgebung und Sonnenschutz entscheiden, wie viel Sonne zählt: $F_{\mathrm{S}}$ und $F_{\mathrm{C}}$."),
        ("store",
         r"Mass only delays that power. Surplus solar heat overheats the room — the utilisation factor $\eta$ follows in the finale.",
         r"Masse verzögert diese Leistung nur. Überschüssige Sonnenwärme überhitzt den Raum — den Ausnutzungsgrad $\eta$ zeigt das Finale."),
        ("mass",
         r"Stored heat $Q = m \cdot c \cdot \Delta\theta$ — in joule.",
         r"Gespeicherte Wärme $Q = m \cdot c \cdot \Delta\theta$ — in Joule."),
        ("release",
         "More mass flattens the daily temperature swing and returns the warmth after sunset.",
         "Mehr Masse glättet den Tagesgang der Temperatur und gibt die Wärme nach Sonnenuntergang zurück."),
    ]

    def construct(self):
        stage = _morph.stage(self, TRANSITIONS)
        apply_scene_style(self)
        N = self.NARRATION

        subtitle = section_heading(tr("Main equation of solar gains", "Hauptgleichung der solaren Gewinne"))
        din = _din_ref(_references.cite(("din-v-4108-6", "solar")))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "assemble"))
        self.play(FadeIn(caption), run_time=0.3)

        token_src = [("g", SYM_IS, COLOR_G), ("a", "A", COLOR_A), ("gval", SYM_G, COLOR_GVAL), ("ff", SYM_FF, COLOR_FF),
                     ("fw", SYM_FW, COLOR_FW), ("fs", SYM_FS, COLOR_FSH), ("fsh", SYM_FC, COLOR_FC)]
        unit_src = [r"\frac{\mathrm{W}}{\mathrm{m^{2}}}", r"\mathrm{m^{2}}", r"\text{–}", r"\text{–}", r"\text{–}", r"\text{–}", r"\text{–}"]
        tokens = VGroup(*[math_text(src, font_size=FORMULA_FONT_SIZE, color=col) for _k, src, col in token_src])
        tokens.arrange(RIGHT, buff=0.6).move_to(ORIGIN)
        labs = VGroup(*[math_text(src, font_size=BODY_FONT_SIZE, color=col)
                        for src, (_k, _s, col) in zip(unit_src, token_src)])
        for lab, tok in zip(labs, tokens):
            lab.next_to(tok, DOWN, buff=0.2)
        VGroup(tokens, labs).move_to(DOWN * 0.2)
        unit_head = Text(tr("unit", "Einheit"), font_size=LABEL_FONT_SIZE, color=GREY_B).next_to(labs[0], LEFT, buff=0.5)

        self.play(
            LaggedStart(*[FadeIn(t, shift=DOWN * 0.15) for t in tokens], lag_ratio=0.12),
            LaggedStart(*[FadeIn(l) for l in labs], lag_ratio=0.12), FadeIn(unit_head),
            run_time=1.4,
        )

        row, items = math_row([
            ("phi", SYM_QS, PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g", SYM_IS, COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("gval", SYM_G, COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("ff", SYM_FF, COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("fw", SYM_FW, COLOR_FW), (None, r"\cdot", PASTEL_WHITE),
            ("fs", SYM_FS, COLOR_FSH), (None, r"\cdot", PASTEL_WHITE),
            ("fsh", SYM_FC, COLOR_FC), (None, UNIT_TAIL, PASTEL_WHITE),
        ], font_size=BODY_FONT_SIZE + 4, buff=0.1)
        row.next_to(subtitle, DOWN, buff=0.95)
        row.set_x(0)
        box = SurroundingRectangle(row, color=P_PRIMARY, buff=_theme_module.UI_SPACING * 2, corner_radius=0, stroke_width=2)
        # Free factor tokens must leave before the boxed row appears — FadeOut alone
        # leaves Cairo Text glyphs on stage that later sit on Q_Speicher.
        intro = VGroup(tokens, labs, unit_head)
        self.play(FadeOut(intro), run_time=0.55)
        _wipe(self, tokens, labs, unit_head)
        self.play(FadeIn(row), Create(box), run_time=1.05)
        self.add(row, box)
        hold_for(self, N, "assemble", used=1.4 + 0.55 + 1.05 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(N, "chain"))
        fac = {"g": ValueTracker(0.0), "a": ValueTracker(0.0), "gval": ValueTracker(1.0), "ff": ValueTracker(1.0),
               "fw": ValueTracker(1.0), "fs": ValueTracker(1.0), "fsh": ValueTracker(1.0)}

        def phi():
            return float(np.prod([t.get_value() for t in fac.values()]))

        def opening():
            return fac["g"].get_value() * fac["a"].get_value()

        def chain_src():
            v = {key: t.get_value() for key, t in fac.items()}
            return rf"{SYM_QS} = {label_number(opening())}\,\mathrm{{W}} \cdot " + r" \cdot ".join(
                rf"\textcolor{{{col}}}{{{label_number(v[key], 2)}}}" for key, col in (
                    ("gval", COLOR_GVAL), ("ff", COLOR_FF), ("fw", COLOR_FW), ("fs", COLOR_FSH), ("fsh", COLOR_FC))
            ) + rf" \approx {label_number(phi())}\,\mathrm{{W}}"

        def detail_src():
            return (rf"\textcolor{{{COLOR_G}}}{{{label_number(fac['g'].get_value())}}}\,\frac{{\mathrm{{W}}}}{{\mathrm{{m^{{2}}}}}} \cdot "
                    rf"\textcolor{{{COLOR_A}}}{{{label_number(fac['a'].get_value(), 2)}}}\,\mathrm{{m^{{2}}}} = {label_number(opening())}\,\mathrm{{W}}")

        chain = math_readout(chain_src, lambda: box.get_bottom() + DOWN * 0.55, size=BODY_FONT_SIZE,
                             color=PASTEL_WHITE, edge="center")
        detail = math_readout(detail_src, lambda: box.get_bottom() + DOWN * 1.05, size=LABEL_FONT_SIZE,
                              color=PASTEL_WHITE, edge="center")
        bar_w = 7.0
        track = always_redraw(lambda: Rectangle(width=bar_w, height=0.2, color=GREY_B, stroke_width=1.5).move_to(
            box.get_bottom() + DOWN * 1.65))
        bar = always_redraw(lambda: Rectangle(
            width=max(1e-3, bar_w * phi() / PHI_GA), height=0.2, stroke_width=0,
            fill_color=COLOR_HEAT, fill_opacity=0.9,
        ).move_to(box.get_bottom() + DOWN * 1.65 + LEFT * bar_w / 2, aligned_edge=LEFT))
        full = math_text(rf"{label_number(PHI_GA)}\,\mathrm{{W}}", font_size=LABEL_FONT_SIZE, color=GREY_B)
        full.add_updater(lambda m: m.next_to(track, RIGHT, buff=0.15))
        self.add(chain, detail)
        self.play(FadeIn(track), FadeIn(bar), FadeIn(full), run_time=0.4)
        factor_values = (("g", G_WINTER), ("a", WIN_A), ("gval", G_VALUE), ("ff", FF_GLASS), ("fw", FW_INCIDENCE),
                         ("fs", FS_WINTER), ("fsh", FC_WINTER))
        colors = {k: c for k, _s, c in token_src}
        for key, value in factor_values:
            ring = highlight_param(items, key, color=colors[key])
            self.play(Create(ring), fac[key].animate.set_value(value), run_time=0.8)
            self.play(FadeOut(ring), run_time=0.15)
        _freeze(chain, detail)
        hold_for(self, N, "chain")

        caption = swap_caption(self, caption, subtitle_text(N, "ring"))
        rings = [highlight_param(items, key, color=colors[key]) for key in ("fs", "fsh")]
        self.play(*[Create(r) for r in rings], run_time=0.5)
        hold_for(self, N, "ring")
        self.play(*[FadeOut(r) for r in rings], run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "store"))
        _wipe(self, tokens, labs, unit_head)
        track.clear_updaters()
        bar.clear_updaters()
        full.clear_updaters()
        self.play(FadeOut(track), FadeOut(bar), FadeOut(full), FadeOut(detail), run_time=0.35)
        _wipe(self, track, bar, full, detail)
        box.move_to(row.get_center())
        chain.next_to(box, DOWN, buff=0.18)
        solar = VGroup(row, box, chain)
        self.play(
            solar.animate.next_to(subtitle, DOWN, buff=0.16).set_x(0).scale(0.86),
            run_time=0.55,
        )
        box.move_to(row.get_center())
        chain.next_to(box, DOWN, buff=0.16)

        store_note = Text(
            tr("↓ stored in the thermal mass", "↓ gespeichert in der thermischen Masse"),
            font_size=BODY_FONT_SIZE, color=COLOR_HEAT,
        )
        row2, items2 = math_row([
            ("q", r"Q_{\mathrm{Speicher}}", COLOR_HEAT), (None, "=", PASTEL_WHITE),
            ("m", "m", COLOR_MASS), (None, r"\cdot", PASTEL_WHITE),
            ("c", "c", COLOR_C), (None, r"\cdot", PASTEL_WHITE),
            ("dt", r"\Delta\theta", COLOR_G), (None, r"\quad[Q_{\mathrm{Speicher}}] = \mathrm{J}", PASTEL_WHITE),
        ], font_size=BODY_FONT_SIZE)
        box2 = SurroundingRectangle(row2, color=COLOR_HEAT, buff=_theme_module.UI_SPACING * 2, corner_radius=0, stroke_width=2)
        legend = VGroup(
            math_text(tr(r"m\;\;\text{mass of the component in}\;\mathrm{kg}",
                         r"m\;\;\text{Masse des Bauteils in}\;\mathrm{kg}"), font_size=LABEL_FONT_SIZE, color=COLOR_MASS),
            math_text(tr(r"c\;\;\text{specific heat capacity in}\;\frac{\mathrm{J}}{\mathrm{kg\,K}}",
                         r"c\;\;\text{spez. Wärmekapazität in}\;\frac{\mathrm{J}}{\mathrm{kg\,K}}"),
                      font_size=LABEL_FONT_SIZE, color=COLOR_C),
            math_text(tr(r"\Delta\theta\;\;\text{temperature rise in}\;\mathrm{K}",
                         r"\Delta\theta\;\;\text{Temperaturanstieg in}\;\mathrm{K}"),
                      font_size=LABEL_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.20)
        # Side legend is safe now that free-factor tokens are wiped; keeps the band above the caption.
        mass_block = VGroup(VGroup(row2, box2), legend).arrange(RIGHT, buff=0.85, aligned_edge=UP)
        store_stack = VGroup(store_note, mass_block).arrange(DOWN, buff=0.28)
        store_stack.next_to(chain, DOWN, buff=0.28).set_x(0)
        fit_band(store_stack, top=min(chain.get_bottom()[1] - 0.18, 0.30), bottom=SAFE_BOTTOM + 0.55)
        store_stack.set_x(0)

        self.play(FadeIn(store_note, shift=DOWN * 0.1), run_time=0.55)
        self.play(FadeIn(row2), Create(box2), run_time=0.9)
        hold_for(self, N, "store")

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        self.play(
            LaggedStart(*[FadeIn(l, shift=RIGHT * 0.1) for l in legend], lag_ratio=0.2),
            run_time=1.0,
        )
        for key, col in (("m", COLOR_MASS), ("c", COLOR_C), ("dt", COLOR_G)):
            r = highlight_param(items2, key, color=col)
            self.play(Create(r), run_time=0.3)
            self.play(FadeOut(r), run_time=0.18)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "release"))
        ring_q = highlight_param(items2, "q", color=COLOR_HEAT)
        self.play(Create(ring_q), run_time=0.4)
        hold_for(self, N, "release")
        self.play(FadeOut(ring_q), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
