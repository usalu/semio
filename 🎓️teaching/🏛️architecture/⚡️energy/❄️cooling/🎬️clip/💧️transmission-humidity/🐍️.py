

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
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
de_num = _manim_module.de_num
house_section = _manim_module.house_section
sun_glyph = _manim_module.sun_glyph
radiation_ray = _manim_module.radiation_ray
ripples = _manim_module.ripples
pulse_flashes = _manim_module.pulse_flashes
clock_glyph = _manim_module.clock_glyph
droplets = _manim_module.droplets
open_window = _manim_module.open_window
smooth_path = _manim_module.smooth_path
flow_guides = _manim_module.flow_guides
flow_animation = _manim_module.flow_animation
dim_arrow = _manim_module.dim_arrow
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
set_vo_language = _manim_module.set_vo_language
load_vo_timing = _manim_module.load_vo_timing

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "⏱️timing" / "🔣️.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module titles — animated once, self.add()'ed on later beats.
TITLE_OPAQUE_DE = "Transmissionswärme: Opake Bauteile"
TITLE_LAST_DE = "Lüftungslast im Sommer"

# Mid-screen anchor for house/diagram content (clear of title + formula/caption).
CONTENT_CENTER = DOWN * 0.1


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``3_heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
    that never competes with the diagram. The formula panel sits on the
    bottom edge, so this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Running example
# 🧮 The numbers of Beat 3 are read from the Lüftung video, never redefined: q_v,R from chapter 2
# (``2_ventilation/2_luftmenge``), c_Luft, Δθ, Δx and r from chapter 3 (``2_ventilation/3_waermestrom_luft``), η_WRG from
# chapter 5 (``2_ventilation/5_mechanische_lueftung``). The opaque load is Beat 1's U · A · Δθ_eq on the roof and
# the outer wall of the Heating example house (Heating Modul 2: 89 m² · 0,20 and 209 m² · 0,24 W/(m²·K)).
V_DOT = 220.0
Q_V_R = V_DOT / 3600.0
RHO_AIR, C_LUFT, R_LAT = 1.2, 0.34, 2450.0
D_THETA, D_X = 6.0, 4.5
ETA_WRG = 0.8
UA_OPAQUE = 89.0 * 0.20 + 209.0 * 0.24
D_THETA_EQ = 10.0
Q_SENS = V_DOT * C_LUFT * D_THETA
Q_LAT = RHO_AIR * R_LAT * D_X * Q_V_R
W_OPAQUE, W_SENS, W_LAT = (round(v, -1) for v in (UA_OPAQUE * D_THETA_EQ, Q_SENS, Q_LAT))
W_SENS_WRG = round(Q_SENS * (1.0 - ETA_WRG), -1)

AIR = "#60A5FA"
SENS = P_RED
LATENT = P_CYAN
OPAQUE = P_YELLOW
#endregion


#region Shared visual motifs

def _build_house(center=ORIGIN, scale: float = 1.0):
    """🏠 The Physical Fundamentals section house, keyed for the opaque-envelope beats.

    ``envelope`` holds the outer wall and roof lines that heat up; ``air`` tints the interior.
    """
    h = house_section(np.array(center, dtype=float) + UP * 0.05, scale)
    t = 0.1
    air = Polygon(
        h["bottom_left"] + RIGHT * t + UP * 0.02, h["bottom_right"] + LEFT * t + UP * 0.02,
        h["top_right"] + LEFT * t + DOWN * 0.06, h["roof_peak"] + DOWN * 0.16, h["top_left"] + RIGHT * t + DOWN * 0.06,
        stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    return {
        "house": h["group"], "air": air, "windows": h["windows"],
        "envelope": VGroup(*h["walls"][:4], h["roof"][0]), "roof": h["roof"][0],
        "bl": h["bottom_left"], "br": h["bottom_right"], "tl": h["top_left"], "tr": h["top_right"],
        "roof_peak": h["roof_peak"], "center": h["center"],
    }


def _build_sun(sun_pos):
    """☀️ Physical Fundamentals sun glyph."""
    return sun_glyph(sun_pos).scale(0.42)


def _parallel_rays(sun_pos, targets, *, gap: float = 0.45):
    """━ Straight parallel sun rays ending on ``targets`` — one shared direction from the sun."""
    sun_pos = np.array(sun_pos, dtype=float)
    targets = [np.array(t, dtype=float) for t in targets]
    aim = np.mean(targets, axis=0) - sun_pos
    aim /= np.linalg.norm(aim)
    starts = [t - aim * (float(np.dot(t - sun_pos, aim)) - gap) for t in targets]
    return VGroup(*[radiation_ray(a, b) for a, b in zip(starts, targets)]), [[a, b] for a, b in zip(starts, targets)]


#region Air through the openings
def _through(win, scale: float, reach: float, *, inward: bool, dy: float = 0.0):
    """➡️ Particle track through one opening — from the room out, or reversed from outside in."""
    x, y = float(win["x"]), float(win["center"][1])
    pts = [(x + 1.05 * scale, y - 0.06 + dy), (x + 0.35 * scale, y + dy), (x - 0.3 * scale, y + 0.03 + dy),
           (x - reach, y + 0.2 + dy)]
    if inward:
        pts = pts[::-1]
    return smooth_path([np.array([px, py, 0.0]) for px, py in pts])


def _summer_tracks(h, scale: float, reach: float):
    """🧭 Summer air tracks of the two openings — hot outdoor air in above, cooled room air out below, moisture in."""
    lo, up = h["windows"]
    return {"hot": [_through(up, scale, reach, inward=True)], "cool": [_through(lo, scale, reach, inward=False)],
            "wet": [_through(up, scale, reach, inward=True, dy=0.1)]}


def _summer_air(tracks, rt: float, *, speed: float = 0.6):
    """💨 Summer streams for ``rt`` seconds — the openings of Lüftung Kapitel 1 with the direction reversed."""
    return [flow_animation([(tracks["hot"], P_RED, P_ORANGE)], waves=3, cycles=speed * rt),
            flow_animation([(tracks["cool"], P_CYAN, P_BLUE)], waves=3, cycles=speed * rt),
            flow_animation([(tracks["wet"], LATENT)], waves=4, radius=0.05, cycles=speed * rt, streak=False)]
#endregion


#region Trackers
def _ramp(tracker, to: float, *, rate_func=smooth):
    """⏩ Move a tracker to ``to`` — one animation, so it can run beside streams and ripples."""
    start = tracker.get_value()
    return UpdateFromAlphaFunc(tracker, lambda m, a: m.set_value(start + (to - start) * a), rate_func=rate_func)
#endregion


#region Chips
def _back_chip(prefix: str, src: str | None = None):
    """🔗 Back-link chip in the Lüftung blue — names the chapter and quantity a value is taken from."""
    parts = [body_text(prefix, font_size=LABEL_FONT_SIZE + 1, color=AIR)]
    if src:
        parts.append(math_label(src, size=LABEL_FONT_SIZE + 1, color=AIR))
    content = VGroup(*parts).arrange(RIGHT, buff=0.12)
    box = SurroundingRectangle(content, color=AIR, corner_radius=0.1, buff=0.1, stroke_width=1.8)
    return VGroup(box, content)
#endregion

#endregion


#region Beat 1 – Transmission through opaque surfaces

class Beat1_TransmissionOpaque(Scene):
    NARRATION = [
        ("intro",
         "Next we look at the opaque envelope. In summer the heat flow reverses: hot surfaces drive heat inward.",
         "Als Nächstes die opake Hülle. Im Sommer kehrt sich der Wärmestrom um: heiße Oberflächen treiben Wärme nach innen."),
        ("sun",
         "A dark roof under midday sun absorbs solar radiation and heats the outer surface.",
         "Ein dunkles Dach absorbiert unter der Mittagssonne Strahlung und heizt die Außenfläche auf."),
        ("formula",
         "The opaque transmission load Q-dot T is U times A times the equivalent temperature difference Delta-theta eq, in watts.",
         "Die opake Transmissionslast Q-Punkt-T ist U mal A mal Delta-Theta-äquivalent — in Watt."),
        ("u",
         "U is the thermal transmittance in watts per square metre kelvin. On the design day a lower U cuts the inward heat flow.",
         "U ist der Wärmedurchgangskoeffizient in W/(m²·K). Am Auslegungstag senkt ein niedrigeres U den Wärmestrom nach innen."),
        ("a",
         "A is the opaque surface area in square metres.",
         "A ist die opake Bauteilfläche in Quadratmetern."),
        ("dt",
         "Delta-theta eq is the sol-air equivalent temperature difference — surface heating from radiation, not indoor-to-outdoor air Delta-theta.",
         "Delta-Theta-eq ist die äquivalente Temperaturdifferenz nach Sol-Air — solare Oberflächenerwärmung, nicht das Luft-Delta-Theta."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Opake Bauteile unter Sommerstrahlung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Content raised clear of formula panel + caption bar.
        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        self.play(Create(h["house"]), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sun"))
        self.play(FadeIn(sun_group, scale=0.7), run_time=1.2)

        opaque_left = Line(h["bl"], h["tl"], color=P_ORANGE, stroke_width=6)
        opaque_right = Line(h["br"], h["tr"], color=P_ORANGE, stroke_width=6)
        opaque_roof_l = Line(h["tl"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_roof_r = Line(h["tr"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_borders = VGroup(opaque_left, opaque_right, opaque_roof_l, opaque_roof_r)
        self.play(Create(opaque_borders), run_time=1.0)

        # Only the sun-facing surfaces: right roof slope and right wall.
        targets = [
            h["roof_peak"] + (h["tr"] - h["roof_peak"]) * 0.3,
            h["roof_peak"] + (h["tr"] - h["roof_peak"]) * 0.7,
            h["tr"] + (h["br"] - h["tr"]) * 0.3,
            h["tr"] + (h["br"] - h["tr"]) * 0.65,
        ]
        rays, ray_paths = _parallel_rays(sun_pos, targets)
        slope = h["tr"] - h["roof_peak"]
        roof_in = float(np.arctan2(-slope[0], slope[1]))
        hot_spots = [(t + np.array([0.0, -0.06, 0.0]), roof_in) for t in targets[:2]] + \
                    [(t + LEFT * 0.08, PI) for t in targets[2:]]

        def surface_heat(rt):
            cyc = max(1.0, rt / 1.3)
            return [pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.5)), width=4.0),
                    *[ripples([c], r_max=0.45, rings=2, color=P_RED, facing=f, cycles=cyc) for c, f in hot_spots]]

        self.add(h["air"])
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.15), run_time=1.5)
        self.play(
            opaque_borders.animate.set_color(P_RED),
            h["envelope"].animate.set_color(P_RED),
            h["air"].animate.set_fill(P_RED, opacity=0.12),
            *surface_heat(2.0),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "sun", during=surface_heat)

        row, box, items = math_panel([
            ("qt", r"\dot{Q}_{T}", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta_{eq}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])

        # U belongs to the sunlit right wall, A to the envelope band on the left —
        # two separate spots so the tokens never sit on each other.
        u_token = math_label("U", size=FORMULA_FONT_SIZE, color=P_ORANGE)
        u_token.move_to((h["tr"] + h["br"]) / 2 + LEFT * 0.42 + UP * 0.3)
        a_token = math_label("A", size=FORMULA_FONT_SIZE, color=P_CYAN)

        # A is the opaque envelope itself — shade the two walls and both roof
        # slopes as bands, not a floating box in the room.
        area_band = 0.18
        area_fill = VGroup(
            Polygon(
                h["bl"], h["tl"], h["tl"] + RIGHT * area_band, h["bl"] + RIGHT * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["br"], h["tr"], h["tr"] + LEFT * area_band, h["br"] + LEFT * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["tl"], h["roof_peak"], h["roof_peak"] + DOWN * area_band, h["tl"] + DOWN * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["tr"], h["roof_peak"], h["roof_peak"] + DOWN * area_band, h["tr"] + DOWN * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
        )
        a_token.move_to((h["tl"] + h["bl"]) / 2 + RIGHT * 0.45 + DOWN * 0.55)
        rest = VGroup(*[m for m in row.submobjects if m is not items["u"] and m is not items["a"]])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(LaggedStart(*[FadeIn(p) for p in area_fill], lag_ratio=0.12), run_time=0.8)
        self.play(
            ReplacementTransform(opaque_borders.copy(), u_token),
            ReplacementTransform(area_fill, a_token),
            run_time=1.4,
        )
        self.play(Create(box), FadeIn(rest), run_time=0.6)
        self.play(
            ReplacementTransform(u_token, items["u"]),
            ReplacementTransform(a_token, items["a"]),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "formula", during=surface_heat)

        for key, color in (("u", P_ORANGE), ("a", P_CYAN), ("dt", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), *surface_heat(0.5), run_time=0.5)
            hold_for(self, self.NARRATION, key, during=surface_heat)
            self.play(FadeOut(ring), run_time=0.3)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 – Thermal mass & time lag

class Beat2_TimeLag(Scene):
    NARRATION = [
        ("intro",
         "Because materials like concrete and brick have high thermal mass, they store this heat.",
         "Beton und Ziegel speichern Wärme — sie haben eine hohe thermische Masse."),
        ("clock",
         "They soak it up during the day and slowly release it into the room hours later.",
         "Tagsüber nehmen sie Wärme auf und geben sie erst Stunden später an den Raum ab."),
        ("peak",
         "So the thermal mass shifts the peak cooling load later — typically into the late afternoon, rarely as late as the evening.",
         "Deshalb verschiebt die Speichermasse die Spitzenkühllast nach hinten — typisch in den späten Nachmittag, selten erst in den Abend."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        self.add(title)
        subtitle = beat_subtitle("Phasenverschiebung (Time Lag)", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        h["envelope"].set_color(P_RED)

        row, box, items = math_panel([
            ("qt", r"\dot{Q}_{T}", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta_{eq}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])

        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        self.add(h["air"], h["house"], row, box, sun_group)
        wall_spots = [h["tl"] + (h["bl"] - h["tl"]) * f + RIGHT * 0.12 for f in (0.3, 0.7)] + \
                     [h["tr"] + (h["br"] - h["tr"]) * f + LEFT * 0.12 for f in (0.3, 0.7)]
        wall_dirs = [0.0, 0.0, PI, PI]

        def stored_heat(rt, r_max=0.35):
            cyc = max(1.0, rt / 1.4)
            return [ripples([c], r_max=r_max, rings=2, color=P_ORANGE, facing=f, cycles=cyc)
                    for c, f in zip(wall_spots, wall_dirs)]

        hold_for(self, self.NARRATION, "intro", during=stored_heat)

        clock = clock_glyph(np.array([-4.6, 1.3, 0.0]), r=0.48, color=P_ORANGE)
        clock_center = clock["center"]
        clock_group = clock["group"]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "clock"))
        self.play(Create(clock_group), run_time=1.0)

        dt_ring = highlight_param(items, "dt", color=P_ORANGE)
        self.play(Create(dt_ring), run_time=0.5)

        # Five hours on the dial: one hand turn per hour of time lag.
        self.play(
            Rotate(clock["hand"], angle=-TAU * 5, about_point=clock_center),
            sun_group.animate.shift(DOWN * 2.0 + RIGHT * 1.1).set_opacity(0.35),
            *stored_heat(3.0, r_max=0.45),
            run_time=3.0,
            rate_func=linear,
        )
        # 🕔 Five hours from noon lands at 17:00 — late afternoon, not evening.
        peak_tag = VGroup(
            Text("17 Uhr", font_size=BODY_FONT_SIZE, color=P_ORANGE),
            Text("später Nachmittag", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06).next_to(clock_group, DOWN, buff=0.14)
        late_tag = Text("21 Uhr Abend", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        late_tag.set_opacity(0.38).next_to(peak_tag, DOWN, buff=0.08)
        self.play(FadeIn(peak_tag), FadeIn(late_tag), run_time=0.5)
        hold_for(self, self.NARRATION, "clock", during=lambda rt: stored_heat(rt, r_max=0.5))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "peak"))
        self.play(
            dt_ring.animate.set_stroke(width=4),
            h["air"].animate.set_fill(opacity=0.25), *stored_heat(1.4, r_max=0.7),
            run_time=1.4,
        )
        self.play(h["air"].animate.set_fill(opacity=0.4), *stored_heat(1.3, r_max=0.85), run_time=1.3)
        self.play(
            h["air"].animate.set_fill(opacity=0.5),
            dt_ring.animate.set_stroke(width=5, color=P_RED), *stored_heat(1.3, r_max=0.95),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "peak", during=lambda rt: stored_heat(rt, r_max=0.95))

        self.play(FadeOut(dt_ring), FadeOut(caption), run_time=0.4)
        self.wait(0.5)

#endregion


#region Beat 3 – Lüftungslast im Sommer (bridge to the Lüftung video)

class Beat3_LueftungslastImSommer(Scene):
    """🌬️ Bridge beat: a short reminder of the summer air load from the Lüftung video (chapter 3: 450 W sensible +
    810 W latent) stacked next to the opaque transmission load, then η_WRG (chapter 5) and the window strategies (chapter 4).

    https://de.wikipedia.org/wiki/Kühllast
    """

    NARRATION = [
        ("intro",
         "The opaque envelope delivers about 680 watts here: 68 watts per kelvin times 10 kelvin. In summer the air adds a second load, explained in the ventilation video.",
         "Die opake Hülle liefert hier rund 680 W (68 W/K mal 10 K).\nIm Sommer kommt die Luft als zweite Last hinzu."),
        ("sens",
         "A short reminder from the ventilation video: 220 cubic metres per hour at 6 kelvin bring about 450 watts of sensible heat in through the reversed openings.",
         "Kurz zur Erinnerung aus dem Lüftungsvideo: 220 m³/h bei 6 K\nbringen rund 450 W sensible Wärme herein."),
        ("lat",
         "The moisture of the outdoor air adds about 810 watts of latent load.",
         "Die Feuchte der Außenluft bringt rund 810 W latente Last dazu."),
        ("sum",
         "Together the ventilation load Q-dot L is about 1260 watts, almost twice the 680 watts of the opaque envelope.",
         "Zusammen ergibt sich Q-Punkt-L von rund 1260 W —\nfast doppelt so viel wie die 680 W der opaken Hülle."),
        ("close",
         "A plate exchanger recovers only the sensible part; the 810 watts stay. Window, cross and night ventilation are in chapter 4.",
         "Der Plattentauscher holt nur den sensiblen Teil zurück, die 810 W bleiben.\nFenster-, Quer- und Nachtlüftung: Kapitel 4."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_LAST_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die Luft als zweite Last", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        scale, reach = 0.8, 0.96
        h = _build_house(np.array([-4.1, -0.05, 0.0]), scale)
        sun_pos = np.array([-6.2, 1.85, 0.0])
        sun = _build_sun(sun_pos)
        tracks = _summer_tracks(h, scale, reach)
        guides = VGroup(flow_guides(tracks["hot"], P_RED, opacity=0.3), flow_guides(tracks["cool"], P_CYAN, opacity=0.3))
        drops = droplets(np.array([-6.0, h["windows"][1]["center"][1] + 0.3, 0.0]), n=5, spread=(0.4, 0.05), seed=11)

        roof_spots = [h["roof_peak"] + (h["tl"] - h["roof_peak"]) * f for f in (0.35, 0.7)]
        rays, ray_paths = _parallel_rays(sun_pos, roof_spots)
        s = h["tl"] - h["roof_peak"]
        roof_in = float(np.arctan2(s[0], -s[1]))
        warm = [(p + np.array([0.0, -0.06, 0.0]), roof_in) for p in roof_spots]

        def roof_heat(rt):
            return [pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.5)), width=4.0),
                    *[ripples([c], r_max=0.4, rings=2, color=P_RED, facing=f, cycles=max(1.0, rt / 1.3)) for c, f in warm]]

        def air(rt):
            return _summer_air(tracks, rt)

        t_op, t_s, t_l = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        bars = (t_op, t_s, t_l)
        bar_x, bar_y0, bar_w, unit = 3.6, -1.25, 0.9, 3.1 / (W_OPAQUE + W_SENS + W_LAT)
        label_x = bar_x + bar_w / 2 + 0.2

        def seg(k):
            lo = bar_y0 + sum(t.get_value() for t in bars[:k]) * unit
            return lo, lo + bars[k].get_value() * unit

        def stack():
            out = VGroup()
            for k, color in enumerate((OPAQUE, SENS, LATENT)):
                lo, hi = seg(k)
                if hi - lo > 0.004:
                    out.add(Rectangle(width=bar_w, height=hi - lo, color=color, stroke_width=1.4, fill_color=color,
                                      fill_opacity=0.8).move_to(np.array([bar_x, (lo + hi) / 2, 0.0])))
            return out

        def mid(k, dy=0.0):
            return lambda: np.array([label_x, (seg(k)[0] + seg(k)[1]) / 2 + dy, 0.0])

        bar = always_redraw(stack)
        read_op = math_readout(lambda: rf"\dot{{Q}}_{{T}} \approx {de_num(round(t_op.get_value(), -1))}\,\mathrm{{W}}",
                               mid(0, 0.16), size=LABEL_FONT_SIZE + 1, color=OPAQUE)
        sub_op = math_label(rf"{de_num(UA_OPAQUE)}\,\mathrm{{W/K}} \cdot {de_num(D_THETA_EQ)}\,\mathrm{{K}}",
                            np.array([label_x, bar_y0 + W_OPAQUE * unit / 2 - 0.2, 0.0]), size=LABEL_FONT_SIZE - 2,
                            color=P_WHITE, edge="left")
        read_s = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,sens}}}} \approx {de_num(round(t_s.get_value(), -1))}\,\mathrm{{W}}",
                              mid(1, 0.12), size=LABEL_FONT_SIZE + 1, color=SENS)
        read_l = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{V,lat}}}} \approx {de_num(round(t_l.get_value(), -1))}\,\mathrm{{W}}",
                              mid(2), size=LABEL_FONT_SIZE + 1, color=LATENT)
        gain_tag = body_text("Wärmegewinn", font_size=LABEL_FONT_SIZE - 2, color=SENS)
        gain_tag.add_updater(lambda m: m.move_to(np.array([label_x + m.width / 2, (seg(1)[0] + seg(1)[1]) / 2 - 0.16, 0.0])))

        chip_y = 2.3

        #region envelope load and sun
        self.play(Create(h["house"]), FadeIn(sun, scale=0.7), run_time=1.4)
        self.add(h["air"], bar)
        self.play(
            LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.2),
            h["envelope"].animate.set_color(P_RED), h["air"].animate.set_fill(P_RED, opacity=0.12),
            _ramp(t_op, 0.4 * W_OPAQUE, rate_func=rush_into), *roof_heat(2.2), run_time=2.2,
        )
        self.add(read_op)
        self.play(_ramp(t_op, W_OPAQUE, rate_func=rush_from), *roof_heat(1.0), run_time=1.0)
        self.play(FadeIn(sub_op), *roof_heat(0.4), run_time=0.4)
        hold_for(self, N, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 2.2 + 1.0 + 0.4, during=roof_heat)
        #endregion

        #region sensible load — reminder from the Lüftung video
        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        chip_now = _back_chip("← Lüftung · Kapitel 3:", r"\dot{Q}_{\mathrm{V,sens}}").move_to(np.array([0.6, chip_y, 0.0]))
        self.play(*[open_window(w, run_time=0.9) for w in h["windows"]], Create(guides), FadeIn(drops), FadeIn(chip_now),
                  *roof_heat(0.9), run_time=0.9)
        self.play(_ramp(t_s, 0.4 * W_SENS, rate_func=rush_into), *air(0.7), run_time=0.7)
        self.add(read_s)
        self.play(_ramp(t_s, W_SENS, rate_func=rush_from), FadeIn(gain_tag), *air(1.1), run_time=1.1)
        hold_for(self, N, "sens", during=air)
        #endregion

        #region latent load — reminder from the Lüftung video
        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        new_chip = _back_chip("← Lüftung · Kapitel 3:", r"\dot{Q}_{\mathrm{V,lat}}").move_to(chip_now)
        self.play(FadeOut(chip_now), FadeIn(new_chip), *air(0.6), run_time=0.6)
        chip_now = new_chip
        self.play(_ramp(t_l, 0.4 * W_LAT, rate_func=rush_into), *air(0.7), run_time=0.7)
        self.add(read_l)
        self.play(_ramp(t_l, W_LAT, rate_func=rush_from), *air(1.1), run_time=1.1)
        hold_for(self, N, "lat", during=air)
        #endregion

        #region ventilation load next to the opaque load
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        sum_row, sum_box, sum_items = math_panel([
            ("l", r"\dot{Q}_{\mathrm{L}}", P_WHITE), (None, "=", P_WHITE),
            ("s", r"\dot{Q}_{\mathrm{V,sens}}", SENS), (None, "+", P_WHITE), ("lat", r"\dot{Q}_{\mathrm{V,lat}}", LATENT),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])
        lo_s, hi_l = seg(1)[0], seg(2)[1]
        brace = dim_arrow(np.array([bar_x - bar_w / 2 - 0.25, lo_s, 0.0]), np.array([bar_x - bar_w / 2 - 0.25, hi_l, 0.0]),
                          color=P_WHITE)
        mid_y = (lo_s + hi_l) / 2
        ql = math_label(rf"\dot{{Q}}_{{\mathrm{{L}}}} \approx {de_num(W_SENS + W_LAT)}\,\mathrm{{W}}",
                        np.array([bar_x - bar_w / 2 - 0.5, mid_y + 0.2, 0.0]), size=BODY_FONT_SIZE, color=P_WHITE, edge="right")
        ratio = math_label(rf"\dot{{Q}}_{{\mathrm{{L}}}} / \dot{{Q}}_{{T}} \approx {de_num((W_SENS + W_LAT) / W_OPAQUE, 1)}",
                           np.array([bar_x - bar_w / 2 - 0.5, mid_y - 0.2, 0.0]), size=LABEL_FONT_SIZE + 1, color=P_WHITE,
                           edge="right")
        new_chip = _back_chip("← Lüftung · Kapitel 3:", r"\dot{Q}_{\mathrm{V,sens}} + \dot{Q}_{\mathrm{V,lat}}").move_to(chip_now)
        self.play(FadeIn(sum_row), Create(sum_box), FadeOut(chip_now), FadeIn(new_chip), *air(0.8), run_time=0.8)
        chip_now = new_chip
        self.play(Create(brace), FadeIn(ql), FadeIn(ratio), *air(1.0), run_time=1.0)
        hold_for(self, N, "sum", during=air)
        #endregion

        #region recovery and window strategies
        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        chip_wrg = _back_chip("← Lüftung · Kapitel 5:", r"\eta_{\mathrm{WRG}}")
        chip_win = _back_chip("← Lüftung · Kapitel 4: Fenster · Quer · Nacht")
        chips = VGroup(chip_wrg, chip_win).arrange(RIGHT, buff=0.3).move_to(np.array([0.2, chip_y, 0.0]))
        for part in gain_tag.get_family():
            part.clear_updaters()
        self.play(FadeOut(brace), FadeOut(ql), FadeOut(ratio), FadeOut(gain_tag), FadeOut(chip_now), FadeIn(chips),
                  *air(0.6), run_time=0.6)
        self.play(_ramp(t_s, W_SENS_WRG), *air(2.0), run_time=2.0)
        hold_for(self, N, "close", during=air)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
