

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))
_animation_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎞️animation.🐍️")
freeze_drawables = _animation_module.freeze_drawables
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
SUBTITLE_FONT_SIZE = _manim_module.SUBTITLE_FONT_SIZE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
P_GREEN = _manim_module.P_GREEN
symbol_token = _manim_module.symbol_token
watt_anchor = _manim_module.watt_anchor
equation_row = _manim_module.equation_row
formula_panel = _manim_module.formula_panel
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_readout = _manim_module.math_readout
math_panel = _manim_module.math_panel
de_num = _manim_module.de_num
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing
house_section = _manim_module.house_section
room_section = _manim_module.room_section
sun_glyph = _manim_module.sun_glyph
sun_rays = _manim_module.sun_rays
shine = _manim_module.shine
radiation_ray = _manim_module.radiation_ray
ripples = _manim_module.ripples
pulse_flashes = _manim_module.pulse_flashes

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Solare Einstrahlung"

# Mid-screen anchor for facade / charts / sections.
CONTENT_CENTER = UP * 0.25


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


#region Shared visual motifs
_heating_solar = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.☀️solar-gains.🐍️")


def solar_cooling_example() -> dict[str, float]:
    """🪟 Apply the earlier heating window to the summer design irradiance and external blind."""
    values = {"area": _heating_solar.WIN_A, "glass_share": _heating_solar.FF_GLASS,
              "glass_transmittance": _heating_solar.G_VALUE, "shading": 0.15, "irradiance": 800.0}
    values["load"] = values["area"] * values["glass_share"] * values["glass_transmittance"] * values["shading"] * values["irradiance"]
    return values


def _sun(pos):
    """🌞 Physical Fundamentals sun at the size every Cooling part uses."""
    return sun_glyph(np.array(pos, dtype=float)).scale(0.42)


def _parallel_rays(sun_c, hits, gap: float = 0.5):
    """☀️ Straight parallel rays from the sun onto ``hits``, each starting ``gap`` clear of the sun's centre."""
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array(h, dtype=float) for h in hits]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    starts = [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]
    return aim, starts


def _house_sun_rays(house, sun_c):
    """🔆 Two parallel rays through each window of ``house``, landing on that storey's floor."""
    floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
    sets = [sun_rays(sun_c, w["x"], [w["center"][1] + 0.09, w["center"][1] - 0.09], fy, gap=0.5)
            for w, fy in zip(house["windows"], floors)]
    paths = [[s, h, p] for r in sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]
    return sets, paths


def _bell(x, x0, x1, amp):
    """📈 Half-sine daily irradiance profile between sunrise x0 and sunset x1."""
    if x <= x0 or x >= x1:
        return 0.0
    return amp * np.sin(PI * (x - x0) / (x1 - x0))


#region 3D sun path
# One solar model drives both halves of Beat 1: the sun's 3D position over the
# cube house AND the orientation power curves on the chart — the curves are
# literally drawn by the moving sun, never hand-tuned bells.
def _cav_proj(x, y, z):
    """📐 Cavalier projection — south face true, north depth recedes up-right."""
    return np.array([x + 0.42 * y, z + 0.26 * y, 0.0])


def _sun_angles(h: float):
    """🌞 Solar elevation α and azimuth φ (0 = Süd, −Ost/+West) for hour ``h``."""
    t = np.clip((h - 6.0) / 12.0, 0.0, 1.0)
    alpha = np.radians(62.0) * np.sin(np.pi * t)
    phi = np.radians((h - 12.0) / 6.0 * 90.0)
    return alpha, phi


def _facade_direct(name: str, h: float) -> float:
    """☀️ Direct-beam share (0–1) on one orientation at hour ``h``."""
    if h <= 6.0 or h >= 18.0:
        return 0.0
    a, p = _sun_angles(h)
    direct = {
        "Horiz": np.sin(a),
        "S": np.cos(a) * np.cos(p),
        "O": np.cos(a) * -np.sin(p),
        "W": np.cos(a) * np.sin(p),
        "N": 0.0,
    }[name]
    return max(0.0, float(direct))


def _sun_atten(h: float) -> float:
    """🌫️ Air-mass attenuation of the direct beam — kills the sunrise spike a
    pure cosine model would paint onto the east facade."""
    a, _ = _sun_angles(h)
    return float(np.exp(-0.18 / max(np.sin(a), 0.02)))


def _facade_irradiance(name: str, h: float) -> float:
    """📈 I in W/m² on one orientation: attenuated direct beam plus diffuse share."""
    if h <= 6.0 or h >= 18.0:
        return 0.0
    a, _ = _sun_angles(h)
    return 950.0 * _sun_atten(h) * _facade_direct(name, h) + 110.0 * float(np.sin(a))
#endregion

#endregion


#region Beat1 – Maximum Solar Irradiance (I_S,max)
class Beat1_SolarIrradiance(Scene):
    NARRATION = [
        ("intro",
         "Now we tackle the most significant summer heat source: direct solar radiation.",
         "Die größte sommerliche Wärmequelle: direkte Sonnenstrahlung."),
        ("irradiance",
         "Everything starts with the maximum solar irradiance, I S max, measured in watts per square meter.",
         "Alles beginnt mit I S max — der maximalen Bestrahlungsstärke in Watt pro Quadratmeter."),
        ("chart",
         "Sun: east–south–west. Facade power peaks east in the morning, south at noon, west in the afternoon; north stays low.",
         "Die Sonne wandert von Ost über Süd nach West und zeichnet die Kurven:\nOst am Morgen, Süd am Mittag, West am Nachmittag, Nord bleibt niedrig."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading("Direkte Sonnenstrahlung")
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        house = house_section(np.array([-3.75, -0.3, 0.0]), scale=0.78)
        building = house["group"]
        sun_pos = np.array([-6.45, 0.75, 0.0])
        sun = _sun(sun_pos)
        ray_sets, ray_paths = _house_sun_rays(house, sun_pos)
        light_beam = VGroup(*[r["group"] for r in ray_sets])

        def sunshine(rt):
            return [pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)))]

        irr_anchor = watt_anchor(800, compare="vacuum", title="Bestrahlungsstärke je m²")
        irr_anchor.scale(0.6).move_to(np.array([-0.55, -0.1, 0.0]))

        axes = Axes(
            x_range=[6, 18, 3],
            y_range=[0, 900, 300],
            x_length=5.0,
            y_length=3.0,
            axis_config={"color": P_WHITE, "stroke_width": 2},
            tips=False,
        ).move_to(RIGHT * 3.7 + CONTENT_CENTER + UP * 0.2)

        x_labels = VGroup(*[
            Text(str(h), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(h, 0), DOWN, buff=0.14)
            for h in (6, 9, 12, 15, 18)
        ])
        y_labels = VGroup(*[
            Text(str(v), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(6, v), LEFT, buff=0.14)
            for v in (300, 600, 900)
        ])
        y_axis_name = math_label(r"\mathrm{W/m^{2}}", size=BODY_FONT_SIZE, color=P_PRIMARY)
        y_axis_name.next_to(axes.c2p(6, 900), UP, buff=0.16).shift(LEFT * 0.15)
        x_axis_name = Text("Sonnenzeit", font_size=BODY_FONT_SIZE, color=P_PRIMARY)
        x_axis_name.next_to(axes.c2p(12, 0), DOWN, buff=0.36)

        # ── 3D-Sonnenbahn über dem Würfelhaus (Kavalierprojektion) ──
        cube_c = np.array([-4.2, -1.3, 0.0])
        cs, ch = 0.95, 1.15

        def CV(x, y, z):
            return cube_c + _cav_proj(x, y, z)

        a0, b0, c0, d0 = CV(-cs, -cs, 0), CV(cs, -cs, 0), CV(cs, cs, 0), CV(-cs, cs, 0)
        a1, b1, c1, d1 = CV(-cs, -cs, ch), CV(cs, -cs, ch), CV(cs, cs, ch), CV(-cs, cs, ch)
        faces = {
            "S": Polygon(a0, b0, b1, a1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "O": Polygon(b0, c0, c1, b1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "W": Polygon(d0, a0, a1, d1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "N": Polygon(c0, d0, d1, c1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "Horiz": Polygon(a1, b1, c1, d1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
        }
        solid_edges = VGroup(*[
            Line(p, q, color=P_WHITE, stroke_width=2.4)
            for p, q in ((a0, b0), (b0, c0), (a0, a1), (b0, b1), (c0, c1),
                         (a1, b1), (b1, c1), (c1, d1), (d1, a1))
        ])
        hidden_edges = VGroup(*[
            DashedLine(p, q, color=P_WHITE, stroke_width=1.4, dash_length=0.1, stroke_opacity=0.45)
            for p, q in ((c0, d0), (d0, a0), (d0, d1))
        ])
        # Only the two visible facades carry a tag; W and N face away from the
        # viewer and are named by the compass instead.
        face_tags = VGroup(
            Text("S", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to((a0 + b0) / 2 + RIGHT * 0.45 + UP * 0.24),
            Text("O", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to((b0 + c0 + c1 + b1) / 4),
        )
        north_dir = _cav_proj(0, 1, 0)
        north_dir = north_dir / np.linalg.norm(north_dir)
        compass_base = np.array([-6.55, 1.05, 0.0])
        compass = VGroup(
            Arrow(compass_base, compass_base + north_dir * 0.75, buff=0,
                  color=P_TEAL, stroke_width=2.5, max_tip_length_to_length_ratio=0.22),
            Text("N", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(compass_base + north_dir * 1.0),
        )
        cube = VGroup(*faces.values(), solid_edges, hidden_edges, face_tags)

        def sun_screen(h):
            a, p = _sun_angles(h)
            r = 2.5
            pos3 = (r * np.cos(a) * -np.sin(p), r * np.cos(a) * -np.cos(p), r * np.sin(a))
            return cube_c + _cav_proj(*pos3)

        sun_path = DashedVMobject(
            VMobject(color=P_YELLOW, stroke_width=1.6, stroke_opacity=0.5).set_points_smoothly(
                [sun_screen(h) for h in np.linspace(6.0, 18.0, 25)]
            ),
            num_dashes=40,
        )
        # Tags sit clear of the sun disc even when the sun parks on an end point.
        path_tags = VGroup(
            Text("Ost", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(6.0), DOWN, buff=0.48),
            Text("Süd", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(12.0), UP, buff=0.48),
            Text("West", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(18.0), DOWN, buff=0.48),
        )

        h_tr = ValueTracker(6.0)
        for name, face in faces.items():
            face.add_updater(lambda m, name=name: m.set_fill(
                P_YELLOW,
                opacity=0.75 * _sun_atten(h_tr.get_value()) * _facade_direct(name, h_tr.get_value()),
            ))

        colors = {"Horiz": P_YELLOW, "O": P_ORANGE, "S": P_CYAN, "W": P_GREEN, "N": P_TEAL}
        live_curves = VGroup(*[
            always_redraw(lambda n=n: axes.plot(
                lambda x: _facade_irradiance(n, x),
                x_range=[6.0, max(6.06, min(18.0, h_tr.get_value()))],
                color=colors[n], stroke_width=2.8,
            ))
            for n in colors
        ])
        time_cursor = always_redraw(lambda: DashedLine(
            axes.c2p(np.clip(h_tr.get_value(), 6.0, 18.0), 0),
            axes.c2p(np.clip(h_tr.get_value(), 6.0, 18.0), 900),
            color=P_WHITE, stroke_width=1.2, dash_length=0.08, stroke_opacity=0.35,
        ))
        label_spots = {"Horiz": (12, 860), "O": (6.6, 650), "S": (12, 540), "W": (17.4, 650), "N": (9.0, 210)}
        curve_labels = VGroup(*[
            Text(n, font_size=BODY_FONT_SIZE, color=colors[n]).move_to(axes.c2p(*label_spots[n]))
            for n in colors
        ])

        south_peak = int(round(max(float(_facade_irradiance("S", h)) for h in np.linspace(6.0, 18.0, 90))))
        peak_tr = ValueTracker(0.0)
        eq_row, eq_box, eq_items = math_panel([
            ("i", r"I_{S,max}", P_YELLOW), (None, "=", P_WHITE),
            (None, rf"{de_num(south_peak)}", P_YELLOW), (None, r"\;[\mathrm{W/m^{2}}]", P_PRIMARY),
        ], size=BODY_FONT_SIZE, color=P_YELLOW)

        self.play(Create(building), run_time=1.6)
        self.play(FadeIn(sun, scale=0.7), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.6 + 1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "irradiance"))
        self.play(*[shine(r) for r in ray_sets], run_time=1.4)
        self.play(FadeIn(irr_anchor, shift=DOWN * 0.1), *sunshine(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "irradiance", during=sunshine)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "chart"))
        self.play(Create(axes), run_time=1.0)
        self.play(
            LaggedStart(*[FadeIn(m) for m in (*x_labels, *y_labels, y_axis_name, x_axis_name)], lag_ratio=0.06),
            run_time=0.9,
        )
        # Facade morphs into the cube house; the sun shrinks onto its 3D path.
        self.play(FadeOut(light_beam), FadeOut(irr_anchor), run_time=0.5)
        freeze_drawables(building)
        self.play(
            ReplacementTransform(building, cube), Create(sun_path),
            sun.animate.scale(0.55).move_to(sun_screen(6.0)),
            run_time=1.5,
        )
        self.play(FadeIn(path_tags), FadeIn(compass), run_time=0.5)
        sun.add_updater(lambda m: m.move_to(sun_screen(h_tr.get_value())))
        i_read = math_readout(
            lambda: rf"I_{{S,max}} = {de_num(peak_tr.get_value())}",
            np.array([2.15, 2.28, 0.0]),
            size=LABEL_FONT_SIZE, color=P_CYAN, edge="left",
        )

        def _raise_peak(_mob):
            peak_tr.set_value(max(peak_tr.get_value(), _facade_irradiance("S", h_tr.get_value())))

        i_read.add_updater(_raise_peak)
        self.add(live_curves, time_cursor, i_read)
        self.play(h_tr.animate.set_value(18.0), run_time=8.0, rate_func=linear)
        i_read.remove_updater(_raise_peak)
        peak_tr.set_value(south_peak)
        sun.clear_updaters()
        for face in faces.values():
            face.clear_updaters()

        static_curves = VGroup(*[
            axes.plot(lambda x, n=n: _facade_irradiance(n, x), x_range=[6.0, 18.0],
                      color=colors[n], stroke_width=2.8)
            for n in colors
        ])
        self.remove(live_curves, time_cursor, i_read)
        self.add(static_curves)
        self.play(
            LaggedStart(*[FadeIn(lbl) for lbl in curve_labels], lag_ratio=0.12),
            FadeOut(path_tags),
            run_time=0.9,
        )

        eq_rest = VGroup(*[m for m in eq_row.submobjects if m is not eq_items["i"]])
        self.play(Create(eq_box), FadeIn(eq_rest), run_time=1.0)
        self.play(ReplacementTransform(sun.copy(), eq_items["i"]), run_time=1.4)

        ring = highlight_param(eq_items, "i", color=P_YELLOW)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "chart", used=16.75 + 0.5 + 0.35)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion



#region Beat5 – Final Solar Cooling Load (Q̇_S,tr)
class Beat5_SolarCoolingLoad(Scene):
    NARRATION = [
        ("intro",
         "Solar load: irradiance × gross area × three dimensionless factors—frame, shading, glass transmittance.",
         "Mit dem Fenster aus dem Heizvideo: Bestrahlungsstärke mal Fläche, Glasanteil, Verschattung und g-Wert."),
        ("result",
         "This is Q-dot S,tr — the solar cooling load through the glazing.",
         "Das ist Q-Punkt-S,tr — die solare Kühllast durch die Verglasung."),
        ("meaning",
         "It is the precise thermal wattage our mechanical system must actively remove to prevent the room from overheating.",
         "Genau diese Leistung muss die Anlage aktiv abführen."),
    ]

    def construct(self):
        apply_scene_style(self)
        example = solar_cooling_example()

        subtitle = section_heading("Solare Kühllast")
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room = room_section(np.array([0.0, 0.25, 0.0]), w=11.0, h=2.7, window=(0.3, 0.88))
        sun_pos = np.array([-6.45, 1.45, 0.0])
        sun = _sun(sun_pos)
        ys = np.linspace(room["win_hi"] - 0.18, room["win_lo"] + 0.18, 4)
        rays = sun_rays(sun_pos, room["glass_x"], ys, room["y_f"], gap=0.5)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        warm_spots = [p + UP * 0.03 for p in rays["lands"]]

        def solar_load(rt, r_max=0.75):
            return [ripples(warm_spots, r_max=r_max, color=P_ORANGE, cycles=max(1.0, rt / 1.3)),
                    pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)))]

        eq_row, eq_box, eq_items = math_panel([
            ("q", r"\dot{Q}_{S,tr}", P_YELLOW), (None, "=", P_WHITE),
            ("A", "A", P_BLUE), (None, r"\cdot", P_WHITE),
            ("ff", r"F_{f}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("fv", r"F_{sh}", P_TEAL), (None, r"\cdot", P_WHITE),
            ("g", r"g", P_RED), (None, r"\cdot", P_WHITE),
            ("i", r"I_{S,max}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ], color=P_YELLOW, buff=0.12)

        self.play(Create(room["group"]), run_time=1.8)
        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(shine(rays), run_time=1.6)
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.08), *solar_load(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8 + 0.7 + 1.6 + 1.6,
                 during=solar_load)

        ff_tr = ValueTracker(1.0)
        fv_tr = ValueTracker(1.0)
        g_tr = ValueTracker(0.0)
        i_tr = ValueTracker(0.0)
        read_at = np.array([room["x_r"] - 0.3, room["y_c"] - 1.3, 0.0])
        gx = room["glass_x"]
        win_mid_y = (room["win_hi"] + room["win_lo"]) / 2
        frame = Rectangle(
            width=0.28, height=(room["win_hi"] - room["win_lo"]) + 0.16,
            color=P_WHITE, stroke_width=2.4,
        ).move_to(np.array([gx, win_mid_y, 0.0]))
        slats = VGroup(*[
            Line(
                np.array([gx - 0.46, y, 0.0]),
                np.array([gx - 0.14, y - 0.12, 0.0]),
                color=P_TEAL, stroke_width=3,
            )
            for y in np.linspace(room["win_lo"] + 0.16, room["win_hi"] - 0.16, 5)
        ])
        ff_target = example["glass_share"]

        def _factor_read(src, color):
            return math_readout(src, read_at, size=BODY_FONT_SIZE, color=color, edge="right")

        area = math_label(rf"A = {de_num(example['area'], 2)}\,\mathrm{{m^2}}", read_at + UP * 0.6, size=BODY_FONT_SIZE, color=P_BLUE, edge="right")
        self.play(FadeIn(area), run_time=0.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        self.play(Create(eq_box), FadeIn(eq_row), Indicate(room["glass"], color=P_BLUE), *solar_load(0.8), run_time=0.8)
        ff_read = _factor_read(lambda: rf"F_{{f}} = {de_num(ff_tr.get_value(), 2)}", P_WHITE)
        self.add(ff_read)
        self.play(Create(frame), ff_tr.animate.set_value(ff_target), *solar_load(1.0), run_time=1.0)
        self.play(FadeOut(ff_read), run_time=0.2)
        fv_read = _factor_read(lambda: rf"F_{{sh}} = {de_num(fv_tr.get_value(), 2)}", P_TEAL)
        self.add(fv_read)
        slats.shift(UP * 1.1).set_opacity(0)
        self.add(slats)
        self.play(
            slats.animate.shift(DOWN * 1.1).set_opacity(1),
            fv_tr.animate.set_value(example["shading"]),
            rays["group"].animate.set_stroke(opacity=0.22),
            run_time=1.2,
        )
        self.play(FadeOut(fv_read), run_time=0.2)
        g_read = _factor_read(lambda: rf"g = {de_num(g_tr.get_value(), 2)}", P_RED)
        self.add(g_read)
        self.play(
            room["glass"].animate.set_fill(P_ORANGE, opacity=0.45),
            g_tr.animate.set_value(example["glass_transmittance"]),
            *solar_load(1.0),
            run_time=1.0,
        )
        self.play(FadeOut(g_read), run_time=0.2)
        i_read = _factor_read(lambda: rf"I_{{S,max}} = {de_num(i_tr.get_value())}", P_YELLOW)
        self.add(i_read)
        self.play(Indicate(sun, color=P_YELLOW), i_tr.animate.set_value(example["irradiance"]), *solar_load(1.0), run_time=1.0)
        self.play(FadeOut(i_read), run_time=0.2)
        result = math_label(rf"\dot{{Q}}_{{S,tr}} = {de_num(example['load'], 1)}\,\mathrm{{W}}", read_at + DOWN * 0.7, size=BODY_FONT_SIZE, color=P_YELLOW, edge="right")
        self.play(FadeIn(result), run_time=0.5)
        ring = highlight_param(eq_items, "q", color=P_YELLOW)
        self.play(Create(ring), room["air"].animate.set_fill(P_RED, opacity=0.16), *solar_load(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "result", used=0.8 + 1.0 + 0.2 + 1.2 + 0.2 + 1.0 + 0.2 + 1.0 + 0.2 + 0.8 + 0.35,
                 during=solar_load)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        hold_for(self, self.NARRATION, "meaning", during=lambda rt: solar_load(rt, r_max=0.95))
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
