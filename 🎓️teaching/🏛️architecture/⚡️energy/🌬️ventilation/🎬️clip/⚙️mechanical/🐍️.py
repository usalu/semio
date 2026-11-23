"""⚙️ Ventilation Chapter 5 — Mechanische Lüftung und Wärmerückgewinnung.

Fan-driven ventilation in four beats: the three base types, the plate heat
exchanger with ONE definition of the temperature change ratio η for winter and
summer, the system comparison, and the duct sized from the hygiene airflow.
This chapter owns η_WRG = 0,8 (winter 1500 W → 300 W, summer sensible
450 W → 90 W, latent 810 W untouched) and the duct numbers of the running
example (q_v,R = 0,061 m³/s, v_m = 2,5 m/s, A ≈ 0,0244 m², r ≈ 0,088 m).
The airflow itself (V̇ = 220 m³/h) and the heat flows 1500 W / 450 W / 810 W are
owned by Chapters 2 and 3 and mirrored here as module constants.
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


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
from types import SimpleNamespace

_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
P_PRIMARY = _manim_module.P_PRIMARY
section_material = _manim_module.section_material
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
SAFE_TOP = _manim_module.SAFE_TOP
SAFE_BOTTOM = _manim_module.SAFE_BOTTOM
SAFE_BOTTOM_FORMULA = _manim_module.SAFE_BOTTOM_FORMULA
fit_band = _manim_module.fit_band
smooth_path = _manim_module.smooth_path
def flow_guides(*args, visible: bool = False, **kwargs):
    """🧭 Particle-path guides — kept, temporarily hidden; particles stay visible."""
    return _manim_module.flow_guides(*args, visible=visible, **kwargs)
flow_animation = _manim_module.flow_animation
meter = _manim_module.meter
bind_meter = _manim_module.bind_meter
cross_mark = _manim_module.cross_mark
note_line = _manim_module.note_line
highlight_param = _manim_module.highlight_param
math_label = _manim_module.math_label
math_panel = _manim_module.math_panel
math_readout = _manim_module.math_readout
label_number = _manim_module.label_number
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
tr = _manim_module.tr
_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
unit_text = _manim_module.unit_text
load_vo_timing = _manim_module.load_vo_timing
room_section = _manim_module.room_section
window_glyph = _manim_module.window_glyph
emphasize_casement = _manim_module.emphasize_casement
open_window = _manim_module.open_window
window_opening = _manim_module.window_opening
room_metre = _manim_module.room_metre
staffage_motion = _manim_module.staffage_motion
staffage_height = _manim_module.staffage_height
fitted_staffage = _manim_module.fitted_staffage
_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
TRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
color_ramp = _manim_module.color_ramp

# 🗣️ Measured clause durations live in vo_timing.json.
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent chapter title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Mechanische Lüftung und Wärmerückgewinnung"


#region Shared numbers
_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
Q_V_WINTER, Q_V_SENS_SUMMER, Q_V_LAT_SUMMER = 1500.0, 450.0, 810.0
Q_V_LAT_WINTER = round(_example.Q_LAT_WINTER_W, -1)
ETA_WRG = 0.8
V_DOT_M3H = 220.0
Q_VR = V_DOT_M3H / 3600.0
V_M = 2.5
THETA_I_WINTER, THETA_E_WINTER = 20.0, 0.0
THETA_I_SUMMER, THETA_E_SUMMER = 26.0, 32.0
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``3_heating/2_conduction/scene_2.py``): a dim ``P_PRIMARY`` footnote
    that never competes with the diagram. The chapter title is wide, so the
    footnote sits one line lower, level with the beat subtitle.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30).shift(DOWN * 0.6)
    return ref
#endregion


#region Shared visual motifs

def _room(center, width, height, color=P_WHITE, t: float = 0.16, *,
          window_side="none", window_y=None, window_h=1.15):
    """🏠 Closed room material around declared openings, with slabs centred on the outline."""
    inner_h = height - t
    floor_y = float(center[1]) - inner_h / 2
    mid = float(center[1]) if window_y is None else window_y
    opening = (0.30, 0.88) if window_side == "none" else ((mid - window_h / 2 - floor_y) / inner_h, (mid + window_h / 2 - floor_y) / inner_h)
    section = room_section(center, w=width - t, h=inner_h, wall=t, slab=t,
                           window=opening, window_side=window_side)
    body = VGroup(section["ceiling"], *[m for m in section["shell"] if m is not section["floor"] and m is not section["ceiling"]])
    return section_material(VGroup(body, section["floor"]))


def _fan(pos, radius=0.24):
    """🌀 Handcrafted duct fan in thermal view, fitted into a square of ``2 * radius``; it stands still until ``_spin`` starts it."""
    return fitted_staffage("ventilation-mechanical-duct-fan", width=2 * radius, height=2 * radius, rate=0.0, loop=True).move_to(pos)


PENDULUM_PHASES = 6
PENDULUM_PHASE_S = 70.0


def _mvhr(pos, height=0.62):
    """♻️ Handcrafted heat-recovery unit in thermal view, ``height`` tall, centred on ``pos``; its impeller turns once ``_spin`` starts it."""
    return fitted_staffage("ventilation-mechanical-mvhr", height=height, rate=0.0, loop=True).move_to(pos)


def _pendulum_unit(pos, size=0.5):
    """🔄 Handcrafted decentralised push-pull fan with its ceramic core in thermal view, fitted into a square of ``size``; ``_pendulum_half`` drives its clip."""
    return fitted_staffage("ventilation-mechanical-pendulum-fan", width=size, height=size, rate=0.0).move_to(pos)


def _pendulum_half(unit, half: int, rt: float):
    """↔️ Play one 70 s phase of the pendulum clip in ``rt`` seconds: the rotor turns one way and back while the core stores or gives up heat; from the seventh phase on the last two repeat."""
    index = half if half < PENDULUM_PHASES else PENDULUM_PHASES - 2 + half % 2
    start = index * PENDULUM_PHASE_S
    return UpdateFromAlphaFunc(unit, lambda m, a: m.seek(start + a * PENDULUM_PHASE_S), run_time=rt, rate_func=linear)


def _spin(fans, rt: float):
    """▶️ Start the rotor of every ``_fan`` and hold for ``rt`` seconds."""
    for fan in fans:
        fan.rate = 1.0
    return [Wait(rt)]


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _port(pos, color=P_WHITE):
    """🕳️ Passive wall opening — the uncontrolled half of a single-fan system."""
    return Rectangle(
        width=0.16, height=0.50, color=color, stroke_width=2.6,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    ).move_to(pos)


def _stand(room_c, room_h, x, cast: str = "shared-person-pathlines", t: float = 0.16):
    """🧍 A 1.75 m breathing occupant (thermal staffage clip) in a 2.5 m clear room, anchored to the slab top."""
    unit = room_metre(room_h - t)
    floor_y = float(room_c[1]) - room_h / 2 + t / 2
    return staffage_motion(cast, unit=unit, anchor=np.array([x, floor_y, 0.0]))


def _math_chip(src: str, color: str):
    """🔖 Boxed typeset label — hand-over chips and inline annotations with real subscripts."""
    label = math_label(src, size=BODY_FONT_SIZE, color=color)
    return window_chip(label, color=color)


_AIR_TEMP = color_ramp([(0.0, P_BLUE), (0.4, P_CYAN), (0.7, P_ORANGE), (1.0, P_RED)])


def _air_color(theta: float):
    """🌡️ Particle colour for an air temperature in °C — cold blue to hot red over 0 … 32 °C."""
    return _AIR_TEMP(theta / 32.0)


def _live_flow(streams, rt: float, *, speed: float = 0.8, waves: int = 4, radius: float = 0.065):
    """🎨 Particle streams coloured by ``color_fn(t)`` along the path every frame — supply air changes colour in the core."""
    dots, meta = VGroup(), []
    for paths, color_fn in streams:
        for path in paths:
            for w in range(waves):
                dot = Ellipse(width=radius * 2.8, height=radius * 1.15, stroke_width=0, fill_opacity=0.0)
                dots.add(dot.move_to(path.point_from_proportion(0.0)))
                meta.append([path, w / waves, color_fn, 0.0])

    def update(group, alpha):
        for dot, m in zip(group, meta):
            path, offset, color_fn, angle = m
            t = (alpha * speed * rt + offset) % 1.0
            a, b = path.point_from_proportion(max(0.0, t - 0.02)), path.point_from_proportion(min(1.0, t + 0.02))
            m[3] = float(np.arctan2(b[1] - a[1], b[0] - a[0]))
            dot.rotate(m[3] - angle).move_to(path.point_from_proportion(t))
            dot.set_fill(color_fn(t), opacity=0.92 * max(0.0, min(1.0, t / 0.08, (1.0 - t) / 0.10)))

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


def _live_value(fn):
    """🔗 Read-only stand-in for a ``ValueTracker`` whose value is computed from the drawn picture — drives ``bind_meter``."""
    return SimpleNamespace(get_value=fn)


def _temp_port(title: str, value_fn, at, color):
    """🏷️ Port tag of a duct — name over a live temperature read from the drawn exchanger."""
    box = RoundedRectangle(width=1.3, height=0.84, corner_radius=0, color=color, stroke_width=_theme_module.CHROME_BORDER,
                           fill_color=_theme_module.CHROME_FILL, fill_opacity=0.82).move_to(at)
    name = Text(title, font_size=LABEL_FONT_SIZE, color=color).move_to(box.get_center() + UP * 0.24)
    read = math_readout(lambda: rf"{label_number(value_fn())}\,°\mathrm{{C}}", box.get_center() + DOWN * 0.17,
                        size=BODY_FONT_SIZE, color=P_WHITE, edge="center")
    return VGroup(box, name), read


#endregion


#region Beat1 – The three mechanical base types
class Beat1_Grundtypen(Scene):
    NARRATION = [
        ("intro",
         "Fan-assisted ventilation comes in three basic types. They differ in one thing only: which of the two air streams gets a fan.",
         "Ventilatorgestützte Lüftung hat drei Grundtypen — sie\nunterscheiden sich nur darin, wer einen Ventilator bekommt."),
        ("exhaust",
         "Exhaust-only: a fan high up extracts air; fresh air enters unfiltered through a passive vent low on the opposite wall, with slight room suction.",
         "Die Abluftanlage saugt oben mechanisch ab — die Zuluft strömt\nungefiltert unten gegenüber nach, im Raum herrscht leichter Unterdruck."),
        ("supply",
         "The supply-only system does the opposite: treated air is pushed in low and leaks out high through joints, at slight positive pressure.",
         "Die Zuluftanlage macht es umgekehrt: aufbereitete Luft wird unten\neingeblasen und entweicht oben über Fugen — Überdruck."),
        ("balanced",
         "The balanced system ducts both streams: supply low, extract high on the opposite side. The air crosses the occupied zone.",
         "Die Zu-/Abluftanlage führt beide Ströme: Zuluft unten, Abluft oben gegenüber — die Luft durchquert den Aufenthaltsbereich."),
        ("verdict",
         "Only the third type controls both air streams, enabling filtering, humidity control and heat recovery in both seasons.",
         "Nur der dritte Typ hat beide Ströme unter Kontrolle — erst dann\nsind Filter, Feuchte und WRG möglich, im Winter wie im Sommer."),
    ]

    CENTERS = (
        np.array([-4.35, 0.35, 0.0]),
        np.array([0.0, 0.35, 0.0]),
        np.array([4.35, 0.35, 0.0]),
    )
    ROOM_W, ROOM_H = 3.50, 1.90

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Three basic mechanical types", "Drei mechanische Grundtypen"))
        din = _din_ref(_references.cite("din-en-16798-3"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        c0, c1, c2 = self.CENTERS
        shells = [_room(c, self.ROOM_W, self.ROOM_H) for c in self.CENTERS]
        rooms = VGroup(*[s[0] for s in shells])
        floors = VGroup(*[s[1] for s in shells])
        people = VGroup(*[_stand(c, self.ROOM_H, c[0]) for c in self.CENTERS])
        y_lo, y_hi = c0[1] - 0.52, c0[1] + 0.58
        half = self.ROOM_W / 2

        headers = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], 1.98, 0.0]))
            for t, c, cx in (
                (tr("Extract-only system", "Abluftanlage"), P_ORANGE, c0),
                (tr("Supply-only system", "Zuluftanlage"), P_CYAN, c1),
                (tr("Supply/extract system", "Zu-/Abluftanlage"), P_GREEN, c2),
            )
        ])

        def _duct_arrow(start, end, color):
            return Arrow(start, end, buff=0, color=color, stroke_width=4,
                         max_tip_length_to_length_ratio=0.26)

        def _low_left(c):
            return np.array([c[0] - half, y_lo, 0.0])

        def _high_right(c):
            return np.array([c[0] + half, y_hi, 0.0])

        def _crossing(c):
            start, end = _low_left(c) + RIGHT * 0.10, _high_right(c) + LEFT * 0.10
            return VGroup(*[
                smooth_path([
                    start,
                    np.array([c[0] - 0.75, y_lo + 0.30 + 0.45 * dy, 0.0]),
                    np.array([c[0] + 0.75, y_hi - 0.30 + 0.45 * dy, 0.0]),
                    end,
                ])
                for dy in (-0.5, 0.0, 0.5)
            ])

        ald = _port(_low_left(c0))
        exh_fan = _fan(_high_right(c0))
        exh_out = _duct_arrow(_high_right(c0) + RIGHT * 0.30, _high_right(c0) + RIGHT * 0.68, P_ORANGE)
        exh_paths = _crossing(c0)

        sup_fan = _fan(_low_left(c1))
        sup_in = _duct_arrow(_low_left(c1) + LEFT * 0.68, _low_left(c1) + LEFT * 0.30, P_CYAN)
        leak = _port(_high_right(c1))
        sup_paths = _crossing(c1)

        bal_sup = _fan(_low_left(c2))
        bal_exh = _fan(_high_right(c2))
        bal_in = _duct_arrow(_low_left(c2) + LEFT * 0.68, _low_left(c2) + LEFT * 0.30, P_CYAN)
        bal_out = _duct_arrow(_high_right(c2) + RIGHT * 0.30, _high_right(c2) + RIGHT * 0.68, P_ORANGE)
        bal_paths = _crossing(c2)

        signs = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], -0.86, 0.0]))
            for (t, c), cx in zip((("−", P_ORANGE), ("+", P_CYAN), ("=", P_GREEN)), self.CENTERS)
        ])
        sign_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.18, 0.0]))
            for (t, c), cx in zip(
                ((tr("Negative pressure", "Unterdruck"), P_ORANGE), (tr("Positive pressure", "Überdruck"), P_CYAN), (tr("balanced", "ausgeglichen"), P_GREEN)),
                self.CENTERS,
            )
        ])
        port_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.54, 0.0]))
            for (t, c), cx in zip(
                ((tr("Unfiltered outdoor air", "Außenluft ungefiltert"), P_WHITE),
                 (tr("Exhaust through leaks", "Abluft über Fugen"), P_WHITE),
                 (tr("both flows ducted", "beide Ströme geführt"), P_GREEN)),
                self.CENTERS,
            )
        ])

        def _zone_tag(text, color, c, high):
            return Text(text, font_size=LABEL_FONT_SIZE, color=color).move_to(
                np.array([c[0] + (half - 0.7 if high else 0.7 - half), 1.55 if high else -0.92, 0.0]))

        zone_out = [_zone_tag(tr("extract high", "Abluft oben"), P_ORANGE, c, True) for c in (c0, c2)]
        zone_in = [_zone_tag(tr("supply low", "Zuluft unten"), P_CYAN, c, False) for c in (c1, c2)]

        verdict = Text(
            tr("only with both flows ducted → filtering, humidity control, heat recovery possible", "erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich"),
            font_size=BODY_FONT_SIZE, color=P_GREEN,
        ).move_to(np.array([0.0, -2.18, 0.0]))

        scaffold = VGroup(rooms, floors, people, headers, exh_fan, exh_out, ald, sup_fan, sup_in, leak,
                          bal_sup, bal_exh, bal_in, bal_out, signs, sign_tags, port_tags, verdict, *zone_out, *zone_in)
        fit_band(scaffold)

        running = {"streams": [], "fans": []}

        def machines(rt):
            if not running["streams"]:
                return []
            return [_streams(running["streams"], rt, speed=0.8), *_spin(running["fans"], rt)]

        self.play(
            LaggedStart(*[Create(r) for r in rooms], lag_ratio=0.22),
            LaggedStart(*[Create(f) for f in floors], lag_ratio=0.22),
            LaggedStart(*[FadeIn(p) for p in people], lag_ratio=0.22),
            LaggedStart(*[FadeIn(h, shift=DOWN * 0.1) for h in headers], lag_ratio=0.22),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0)

        for key, new_fans, extras, guide, stream, sign in (
            ("exhaust", [exh_fan], [GrowArrow(exh_out), FadeIn(ald), FadeIn(port_tags[0]), FadeIn(zone_out[0])],
             (exh_paths, P_ORANGE), (exh_paths, P_WHITE, P_ORANGE), 0),
            ("supply", [sup_fan], [GrowArrow(sup_in), FadeIn(leak), FadeIn(port_tags[1]), FadeIn(zone_in[0])],
             (sup_paths, P_CYAN), (sup_paths, P_CYAN, P_WHITE), 1),
            ("balanced", [bal_sup, bal_exh], [GrowArrow(bal_in), GrowArrow(bal_out), FadeIn(port_tags[2]), FadeIn(zone_out[1]), FadeIn(zone_in[1])],
             (bal_paths, P_GREEN), (bal_paths, P_CYAN, P_ORANGE), 2),
        ):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(*[FadeIn(f) for f in new_fans], *extras, Create(flow_guides(*guide)), *machines(1.4),
                      run_time=1.4)
            running["streams"].append(stream)
            running["fans"].extend(new_fans)
            self.play(*machines(2.4), run_time=2.4)
            self.play(FadeIn(signs[sign], scale=1.2), FadeIn(sign_tags[sign]), *machines(0.8), run_time=0.8)
            hold_for(self, self.NARRATION, key, during=machines)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verdict"))
        pick = SurroundingRectangle(
            VGroup(rooms[2], headers[2], signs[2], sign_tags[2], port_tags[2], bal_in, bal_out),
            color=P_GREEN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5,
        )
        self.play(Create(pick), FadeIn(verdict, shift=UP * 0.12), *machines(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "verdict", during=machines)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Heat recovery: one definition of η for winter and summer
class Beat2_Waermerueckgewinnung(Scene):
    NARRATION = [
        ("intro",
         "Once both streams are ducted, they can be run past each other in a plate heat exchanger without ever mixing.",
         "Sobald beide Ströme geführt sind, laufen sie im Platten-\nwärmeübertrager aneinander vorbei, ohne sich zu mischen."),
        ("winter",
         "In winter the twenty-degree exhaust air gives heat to the zero-degree outdoor air, and the supply air rises to sixteen degrees.",
         "Im Winter gibt die 20 °C warme Abluft Wärme an die 0 °C\nkalte Außenluft ab — die Zuluft steigt auf 16 °C."),
        ("formula",
         r"The temperature change ratio $\eta$ divides the achieved supply-air warming by the available exhaust-to-outdoor temperature difference.",
         r"Der Temperaturänderungsgrad $\eta$: erreichte Erwärmung der Zuluft, geteilt durch die verfügbare Differenz Abluft − Außenluft."),
        ("winter_bars",
         r"Only the share $1-\eta$ stays as a loss: at $\eta = 0.8$, $1500\,\mathrm{W}$ shrink to $300\,\mathrm{W}$.",
         r"Nur der Anteil $1-\eta$ bleibt als Verlust: bei $\eta = 0{,}8$ sinken $1500\,\mathrm{W}$ auf $300\,\mathrm{W}$."),
        ("summer",
         r"Summer: $32\,\mathrm{°C}$ outdoors, $26\,\mathrm{°C}$ exhaust; about $5\,\mathrm{K}$ of $6\,\mathrm{K}$ recovered. The same ratio turns heat recovery into cold recovery.",
         r"Im Sommer gilt dieselbe Definition: Außenluft $32\,\mathrm{°C}$, Abluft $26\,\mathrm{°C}$ — fast $5\,\mathrm{K}$ von $6\,\mathrm{K}$, die WRG wird zur Kälterückgewinnung."),
        ("summer_bars",
         r"The sensible load $\dot{Q}_{V,sens}$ drops from $450\,\mathrm{W}$ to about $90\,\mathrm{W}$.",
         r"Die fühlbare Last $\dot{Q}_{V,sens}$ sinkt von $450\,\mathrm{W}$ auf rund $90\,\mathrm{W}$."),
        ("latent",
         r"A plate exchanger transfers sensible heat only. The latent part $\dot{Q}_{V,lat}$ stays in both seasons: $810\,\mathrm{W}$ in summer, $770\,\mathrm{W}$ in winter.",
         r"Der Plattenübertrager überträgt nur fühlbare Wärme: der latente Anteil $\dot{Q}_{V,lat}$ bleibt in beiden Jahreszeiten — $810\,\mathrm{W}$ im Sommer, $770\,\mathrm{W}$ im Winter."),
        ("latent_use",
         "Cooling must dehumidify it. The heating load counts by convention only the sensible part; humidifying would have to pay the winter share.",
         "Die Kühltechnik muss entfeuchten. Die Heizlast rechnet nach Konvention nur den fühlbaren Teil; Befeuchten müsste den Winteranteil bezahlen."),
        ("close",
         r"One definition, two seasons: heating calculates with $300\,\mathrm{W}$, cooling with $90\,\mathrm{W}$ sensible plus $810\,\mathrm{W}$ latent.",
         r"Eine Definition, zwei Jahreszeiten: die Heizung rechnet mit $300\,\mathrm{W}$, die Kühltechnik mit $90\,\mathrm{W}$ fühlbar plus $810\,\mathrm{W}$ latent."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Heat recovery in a plate heat exchanger", "Wärmerückgewinnung im Plattenübertrager"))
        din = _din_ref(_references.cite("din-en-308"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        season = ValueTracker(0.0)
        gain = ValueTracker(0.0)

        def t_aul():
            return THETA_E_WINTER + (THETA_E_SUMMER - THETA_E_WINTER) * season.get_value()

        def t_abl():
            return THETA_I_WINTER + (THETA_I_SUMMER - THETA_I_WINTER) * season.get_value()

        def t_zul():
            return t_aul() + gain.get_value() * (t_abl() - t_aul())

        def t_fol():
            return t_abl() - gain.get_value() * (t_abl() - t_aul())

        core_c = np.array([-2.55, 0.95, 0.0])
        y_sup, y_exh = 1.45, 0.45
        x_in, x_out = -5.0, -0.1
        core = Rectangle(width=2.8, height=1.8, color=P_WHITE, stroke_width=3.0).move_to(core_c)
        plate = Line(core.get_left(), core.get_right(), color=P_TEAL, stroke_width=3.0)
        fins = VGroup(*[
            Line(np.array([x, core_c[1] - 0.09, 0.0]), np.array([x, core_c[1] + 0.09, 0.0]), color=P_TEAL, stroke_width=1.6)
            for x in np.linspace(core_c[0] - 1.1, core_c[0] + 1.1, 9)
        ])
        stubs = VGroup(*[
            Line(np.array([xa, y + dy, 0.0]), np.array([xb, y + dy, 0.0]), color=P_WHITE, stroke_width=1.6).set_stroke(opacity=0.7)
            for y in (y_sup, y_exh) for dy in (-0.22, 0.22)
            for xa, xb in ((x_in, core.get_left()[0]), (core.get_right()[0], x_out))
        ])
        fans = [_fan(np.array([-4.45, y_sup, 0.0]), radius=0.17),
                _fan(np.array([-0.62, y_exh, 0.0]), radius=0.17)]
        core_tag = Text(tr("Plate heat exchanger", "Plattenwärmeübertrager"), font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(np.array([core_c[0], -0.22, 0.0]))

        supply = VGroup(smooth_path([np.array([x_in, y_sup, 0.0]), np.array([-3.0, y_sup, 0.0]),
                                     np.array([-2.0, y_sup, 0.0]), np.array([x_out, y_sup, 0.0])]))
        exhaust = VGroup(smooth_path([np.array([x_out, y_exh, 0.0]), np.array([-2.0, y_exh, 0.0]),
                                      np.array([-3.0, y_exh, 0.0]), np.array([x_in, y_exh, 0.0])]))

        def heat_arrows(up: bool):
            ya, yb = (y_exh + 0.14, y_sup - 0.14) if up else (y_sup - 0.14, y_exh + 0.14)
            return VGroup(*[
                Arrow(np.array([x, ya, 0.0]), np.array([x, yb, 0.0]), buff=0, color=P_YELLOW, stroke_width=3.5,
                      max_tip_length_to_length_ratio=0.3)
                for x in (core_c[0] - 0.8, core_c[0], core_c[0] + 0.8)
            ])

        arrows_up, arrows_down = heat_arrows(True), heat_arrows(False)

        aul_box, aul_read = _temp_port(tr("Outdoor air", "Außenluft"), t_aul, np.array([-5.95, y_sup, 0.0]), P_WHITE)
        zul_box, zul_read = _temp_port(tr("Supply air", "Zuluft"), t_zul, np.array([0.75, y_sup, 0.0]), P_CYAN)
        abl_box, abl_read = _temp_port(tr("Extract air", "Abluft"), t_abl, np.array([0.75, y_exh, 0.0]), P_TEAL)
        fol_box, fol_read = _temp_port(tr("Exhaust air", "Fortluft"), t_fol, np.array([-5.95, y_exh, 0.0]), P_ORANGE)

        def stream_color(t_from, t_to):
            def fn(t):
                f = float(np.clip((t - 0.21) / 0.58, 0.0, 1.0))
                f = f * f * (3.0 - 2.0 * f)
                return _air_color(t_from() + f * (t_to() - t_from()))
            return fn

        air = [([supply[0]], stream_color(t_aul, t_zul)), ([exhaust[0]], stream_color(t_abl, t_fol))]
        running = []

        def exchanger(rt):
            if not running:
                return []
            return [_live_flow(air, rt, speed=0.35, waves=7), *_spin(fans, rt)]

        chip_winter = _math_chip(r"\text{Winter}", P_BLUE).move_to(np.array([core_c[0], 2.2, 0.0]))
        chip_summer = _math_chip(tr(r"\text{Summer}", r"\text{Sommer}"), P_ORANGE).move_to(chip_winter)

        eta_gauge = meter("η", length=3.0, thickness=0.3, color=P_GREEN, vertical=False)
        eta_gauge["group"] = VGroup(eta_gauge["track"], eta_gauge["fill"])
        eta_gauge["group"].move_to(np.array([core_c[0] - 0.4, -0.72, 0.0]))
        bind_meter(eta_gauge, gain)
        eta_read = math_readout(lambda: rf"\eta = {label_number(gain.get_value(), 2)}",
                                eta_gauge["track"].get_right() + RIGHT * 0.25, size=BODY_FONT_SIZE, color=P_GREEN)

        def eta_panel(value_src):
            row, box, items = math_panel([
                ("eta", r"\eta", P_YELLOW), (None, "=", P_WHITE),
                ("frac",
                 rf"\frac{{\textcolor{{{P_CYAN}}}{{\theta_{{\mathrm{{ZUL}}}} - \theta_{{\mathrm{{AUL}}}}}}}}"
                 rf"{{\textcolor{{{P_TEAL}}}{{\theta_{{\mathrm{{ABL}}}} - \theta_{{\mathrm{{AUL}}}}}}}}",
                 P_WHITE),
                (None, "=", P_WHITE),
                ("val", value_src, P_YELLOW),
            ], color=P_YELLOW)
            items["num"], items["den"] = items["frac"][0][0], items["frac"][0][2]
            return row, box, items

        def loss_panel(value_src):
            return math_panel([
                ("q", r"\dot{Q}_{\mathrm{V,WRG}}", P_RED), (None, "=", P_WHITE),
                ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_RED), (None, r"\cdot", P_WHITE),
                ("eta", r"(1-\eta)", P_GREEN), (None, "=", P_WHITE),
                ("val", value_src, P_RED),
            ], color=P_GREEN)

        self.play(Create(core), Create(plate), FadeIn(fins), Create(stubs), FadeIn(core_tag), FadeIn(chip_winter),
                  run_time=1.6)
        self.play(FadeIn(fans[0]), FadeIn(fans[1]), Create(flow_guides(supply, P_WHITE, opacity=0.2)),
                  Create(flow_guides(exhaust, P_WHITE, opacity=0.2)),
                  FadeIn(aul_box), FadeIn(zul_box), FadeIn(abl_box), FadeIn(fol_box), run_time=1.0)
        self.add(aul_read, zul_read, abl_read, fol_read)
        running.append(True)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6, during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.play(LaggedStart(*[GrowArrow(a) for a in arrows_up], lag_ratio=0.25),
                  gain.animate(rate_func=smooth).set_value(ETA_WRG), *exchanger(3.2), run_time=3.2)
        hold_for(self, N, "winter", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        eq, eq_box, items = eta_panel(tr(r"\frac{16\,\mathrm{K}}{20\,\mathrm{K}} = 0.8", r"\frac{16\,\mathrm{K}}{20\,\mathrm{K}} = 0{,}8"))
        self.add(eta_read)
        self.play(FadeIn(eta_gauge["group"]), FadeIn(eq), Create(eq_box), *exchanger(1.2), run_time=1.2)
        ring_num = highlight_param(items, "num", color=P_CYAN)
        self.play(Create(ring_num), Indicate(zul_box, color=P_CYAN), Indicate(aul_box, color=P_WHITE),
                  gain.animate(rate_func=smooth).set_value(0.4), *exchanger(1.4), run_time=1.4)
        self.play(FadeOut(ring_num), gain.animate(rate_func=smooth).set_value(ETA_WRG), *exchanger(1.2), run_time=1.2)
        ring_den = highlight_param(items, "den", color=P_TEAL)
        self.play(Create(ring_den), Indicate(abl_box, color=P_TEAL), Indicate(aul_box, color=P_WHITE),
                  *exchanger(1.0), run_time=1.0)
        hold_for(self, N, "formula", during=exchanger)
        self.play(FadeOut(ring_den), *exchanger(0.25), run_time=0.25)

        bx0, unit = 2.35, 0.0029
        y_w, y_s = 1.0, -0.75

        def seg(x_from, width, color, y, opacity=0.78):
            return Rectangle(width=max(width, 0.002), height=0.40, color=color, fill_color=color, fill_opacity=opacity,
                             stroke_width=1.2).move_to(np.array([x_from + width / 2, y, 0.0]))

        def head(text, color, y):
            label = Text(text, font_size=LABEL_FONT_SIZE, color=color)
            return label.move_to(np.array([bx0 + label.width / 2, y + 0.95, 0.0]))

        def name(src, x, y, color):
            return math_label(src, np.array([x, y + 0.42, 0.0]), size=LABEL_FONT_SIZE, color=color)

        def value(src, x, y, color):
            return math_label(src, np.array([x, y - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=color)

        caption = swap_caption(self, caption, subtitle_text(N, "winter_bars"))
        full_w = Q_V_WINTER * unit
        w_head = head(tr("Winter · Heating", "Winter · Heizung"), P_ORANGE, y_w)
        w_name = name(r"\dot{Q}_{\mathrm{V,sens}}", bx0 + full_w / 2, y_w, P_RED)
        w_full = seg(bx0, full_w, P_RED, y_w)
        wf = ValueTracker(0.0)
        w_read = math_readout(lambda: rf"{label_number(Q_V_WINTER * wf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + full_w / 2, y_w - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_RED,
                              edge="center")
        self.add(w_read)
        self.play(FadeIn(w_head), FadeIn(w_name), GrowFromEdge(w_full, LEFT), wf.animate.set_value(1.0),
                  *exchanger(1.4), run_time=1.4)
        self.remove(w_read)
        w_val = value(rf"{label_number(Q_V_WINTER)}\,\mathrm{{W}}", bx0 + full_w / 2, y_w, P_RED)
        self.add(w_val)
        self.play(*exchanger(1.2), run_time=1.2)
        w_green = seg(bx0, full_w * ETA_WRG, P_GREEN, y_w)
        w_rest = seg(bx0 + full_w * ETA_WRG, full_w * (1 - ETA_WRG), P_RED, y_w)
        w_green_val = value(rf"−{label_number(Q_V_WINTER * ETA_WRG)}\,\mathrm{{W}}", bx0 + full_w * ETA_WRG / 2, y_w, P_GREEN)
        w_rest_val = value(rf"{label_number(Q_V_WINTER * (1 - ETA_WRG))}\,\mathrm{{W}}",
                           bx0 + full_w * (ETA_WRG + (1 - ETA_WRG) / 2), y_w, P_RED)
        lp = loss_panel(r"300\,\mathrm{W}")
        self.play(stage.transition("ventilation.mechanical.heat-recovery.winter-bar-share", source=w_full, target=w_rest), GrowFromEdge(w_green, LEFT),
                  stage.transition("ventilation.mechanical.heat-recovery.winter-value-remaining", source=w_val, target=w_rest_val), FadeIn(w_green_val),
                  stage.transition("ventilation.mechanical.heat-recovery.winter-formula-to-loss", source=eq, target=lp[0]), stage.transition("ventilation.mechanical.heat-recovery.winter-box-to-loss", source=eq_box, target=lp[1]),
                  *exchanger(2.0), run_time=2.0)
        hold_for(self, N, "winter_bars", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        eq2, eq2_box, items2 = eta_panel(tr(r"\frac{5\,\mathrm{K}}{6\,\mathrm{K}} \approx 0.8", r"\frac{5\,\mathrm{K}}{6\,\mathrm{K}} \approx 0{,}8"))
        self.play(stage.transition("ventilation.mechanical.heat-recovery.season-chip-summer", source=chip_winter, target=chip_summer),
                  stage.transition("ventilation.mechanical.heat-recovery.season-arrows-down", source=arrows_up, target=arrows_down),
                  season.animate(rate_func=smooth).set_value(1.0),
                  stage.transition("ventilation.mechanical.heat-recovery.loss-to-efficiency-formula", source=lp[0], target=eq2), stage.transition("ventilation.mechanical.heat-recovery.loss-to-efficiency-box", source=lp[1], target=eq2_box),
                  *exchanger(3.0), run_time=3.0)
        ring_val = highlight_param(items2, "val", color=P_YELLOW)
        self.play(Create(ring_val), Indicate(zul_box, color=P_CYAN), Indicate(abl_box, color=P_TEAL), *exchanger(1.2),
                  run_time=1.2)
        hold_for(self, N, "summer", during=exchanger)
        self.play(FadeOut(ring_val), *exchanger(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "summer_bars"))
        sens_w, lat_w = Q_V_SENS_SUMMER * unit, Q_V_LAT_SUMMER * unit
        s_head = head(tr("Summer · Cooling", "Sommer · Kühltechnik"), P_CYAN, y_s)
        s_name = name(r"\dot{Q}_{\mathrm{V,sens}}", bx0 + sens_w / 2, y_s, P_RED)
        l_name = name(r"\dot{Q}_{\mathrm{V,lat}}", bx0 + sens_w + lat_w / 2, y_s, P_CYAN)
        s_full = seg(bx0, sens_w, P_RED, y_s)
        l_full = seg(bx0 + sens_w, lat_w, P_CYAN, y_s)
        sf = ValueTracker(0.0)
        s_read = math_readout(lambda: rf"{label_number(Q_V_SENS_SUMMER * sf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + sens_w / 2, y_s - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_RED,
                              edge="center")
        l_read = math_readout(lambda: rf"{label_number(Q_V_LAT_SUMMER * sf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + sens_w + lat_w / 2, y_s - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN,
                              edge="center")
        self.add(s_read, l_read)
        self.play(FadeIn(s_head), FadeIn(s_name), FadeIn(l_name), GrowFromEdge(s_full, LEFT), GrowFromEdge(l_full, LEFT),
                  sf.animate.set_value(1.0), *exchanger(1.4), run_time=1.4)
        self.remove(s_read, l_read)
        s_val = value(rf"{label_number(Q_V_SENS_SUMMER)}\,\mathrm{{W}}", bx0 + sens_w / 2, y_s, P_RED)
        l_val = value(rf"{label_number(Q_V_LAT_SUMMER)}\,\mathrm{{W}}", bx0 + sens_w + lat_w / 2, y_s, P_CYAN)
        self.add(s_val, l_val)
        self.play(*exchanger(1.0), run_time=1.0)
        s_green = seg(bx0, sens_w * ETA_WRG, P_GREEN, y_s)
        s_rest = seg(bx0 + sens_w * ETA_WRG, sens_w * (1 - ETA_WRG), P_RED, y_s)
        s_green_val = value(rf"−{label_number(Q_V_SENS_SUMMER * ETA_WRG)}\,\mathrm{{W}}", bx0 + 0.02, y_s, P_GREEN)
        s_rest_val = value(rf"{label_number(Q_V_SENS_SUMMER * (1 - ETA_WRG))}\,\mathrm{{W}}", bx0 + sens_w + 0.02, y_s, P_RED)
        sp = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,sens,WRG}}", P_RED), (None, "=", P_WHITE),
            ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_RED), (None, r"\cdot", P_WHITE),
            ("eta", r"(1-\eta)", P_GREEN), (None, r"\approx", P_WHITE),
            ("val", r"90\,\mathrm{W}", P_RED),
        ], color=P_GREEN)
        self.play(stage.transition("ventilation.mechanical.heat-recovery.summer-bar-share", source=s_full, target=s_rest), GrowFromEdge(s_green, LEFT),
                  stage.transition("ventilation.mechanical.heat-recovery.summer-value-remaining", source=s_val, target=s_rest_val), FadeIn(s_green_val),
                  stage.transition("ventilation.mechanical.heat-recovery.efficiency-to-sensible-formula", source=eq2, target=sp[0]), stage.transition("ventilation.mechanical.heat-recovery.efficiency-to-sensible-box", source=eq2_box, target=sp[1]),
                  *exchanger(2.0), run_time=2.0)
        hold_for(self, N, "summer_bars", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "latent"))
        keep = Text(tr("unchanged", "unverändert"), font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK).move_to(
            np.array([bx0 + sens_w + lat_w / 2, y_s, 0.0]))
        lp2 = math_panel([
            ("sens", r"\dot{Q}_{\mathrm{V,sens}}\cdot(1-\eta)", P_RED), (None, "+", P_WHITE),
            ("lat", r"\dot{Q}_{\mathrm{V,lat}}", P_CYAN), (None, "=", P_WHITE),
            ("val", r"90\,\mathrm{W} + 810\,\mathrm{W}", P_WHITE),
        ], color=P_CYAN)
        self.play(stage.transition("ventilation.mechanical.heat-recovery.sensible-to-latent-formula", source=sp[0], target=lp2[0]), stage.transition("ventilation.mechanical.heat-recovery.sensible-to-latent-box", source=sp[1], target=lp2[1]), FadeIn(keep),
                  Indicate(l_full, color=P_CYAN, scale_factor=1.06), *exchanger(1.6), run_time=1.6)
        hold_for(self, N, "latent", used=1.6, during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "latent_use"))
        hold_for(self, N, "latent_use", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        self.play(Indicate(w_rest, color=P_ORANGE, scale_factor=1.15), Indicate(s_rest, color=P_CYAN, scale_factor=1.3),
                  *exchanger(1.6), run_time=1.6)
        hold_for(self, N, "close", during=exchanger)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Ventilation systems compared
class Beat3_Anlagenvergleich(Scene):
    NARRATION = [
        ("window",
         r"Window ventilation is a free air change without any recovery: the full ventilation loss of $1500\,\mathrm{W}$ remains, $\eta = 0$.",
         r"Fensterlüftung ist freier Luftwechsel ohne Rückgewinnung — der volle Lüftungsverlust von $1500\,\mathrm{W}$ bleibt, $\eta = 0$."),
        ("central",
         r"A central unit with heat recovery reaches $\eta \approx 0.8 \dots 0.9$ and cuts the loss to between $300\,\mathrm{W}$ and $150\,\mathrm{W}$.",
         r"Eine zentrale Anlage mit WRG erreicht $\eta \approx 0{,}8 \dots 0{,}9$ — der Verlust sinkt auf $300\,\mathrm{W}$ bis $150\,\mathrm{W}$."),
        ("decentral",
         "Decentralised ceramic push-pull units store the heat alternately and recover seventy to ninety percent, room by room.",
         "Dezentrale Keramik-Pendellüfter speichern die Wärme abwechselnd\nund gewinnen raumweise 70 bis 90 % zurück."),
        ("outro",
         r"Recovery leaves only the share $1-\eta$ of the winter loss: at $\eta = 0.9$ just $10\,\%$. In summer it recovers cold.",
         r"Die WRG lässt nur den Anteil $1-\eta$ des Winterverlusts übrig: bei $\eta = 0{,}9$ nur $10\,\%$. Im Sommer gewinnt sie Kälte zurück."),
        ("passivhaus",
         "Open windows recover nothing. The Passivhaus uses ducts and a heat-recovery unit as its main ventilation path.",
         "Ein offenes Fenster gewinnt nichts zurück — das Passivhaus führt\ndie Luft deshalb über die WRG, nicht über das Fenster."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Ventilation systems compared", "Lüftungssysteme im Vergleich"))
        din = _din_ref(_references.cite("din-1946-6"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "window"))
        self.play(FadeIn(caption), run_time=0.3)

        N = self.NARRATION
        col_w, col_h = 4.3, 4.6
        pivot = np.array([0.0, -0.25, 0.0])
        room_w, room_h, slab = 2.3, 1.4, 0.13

        def place(*mobs):
            for m in mobs:
                m.scale(0.85, about_point=pivot).shift(UP * 0.35)

        def path(*pts):
            return smooth_path([np.array([x, y, 0.0]) for x, y in pts])

        def legend(lines, color, scale, at):
            return VGroup(*[Text(t, font_size=LABEL_FONT_SIZE, color=color) for t in lines]).arrange(DOWN, buff=0.15)\
                .scale(scale).move_to(at)

        def loss_bar(lo, hi, color):
            width = 1.6
            track = Rectangle(width=width, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK, fill_opacity=1.0)
            upper = Rectangle(width=width * hi, height=0.2, stroke_width=0, fill_color=color, fill_opacity=0.35)
            lower = Rectangle(width=width * lo, height=0.2, stroke_width=0, fill_color=color, fill_opacity=0.9)
            watts = (rf"\dot{{Q}}_{{\mathrm{{V}}}} = {label_number(Q_V_WINTER * lo)}\,\mathrm{{W}}" if lo == hi
                     else rf"\dot{{Q}}_{{\mathrm{{V}}}} = {label_number(Q_V_WINTER * lo)}\ldots{label_number(Q_V_WINTER * hi)}\,\mathrm{{W}}")
            label = math_label(watts, size=LABEL_FONT_SIZE, color=color)
            label.next_to(track, RIGHT, buff=0.15)
            return {"track": track, "upper": upper, "lower": lower, "group": VGroup(track, label)}

        def panel(px, color, fill, main, sub, eta_src, bar):
            card = RoundedRectangle(width=col_w, height=col_h, corner_radius=0, color=color, fill_color=fill,
                                    fill_opacity=0.85).move_to([px, -0.25, 0])
            hdr = VGroup(Text(main, font_size=LABEL_FONT_SIZE, color=color),
                         Text(sub, font_size=LABEL_FONT_SIZE, color=GREY_A)).arrange(DOWN, buff=0.05)
            hdr.move_to(card.get_top() + DOWN * 0.45)
            eta_lbl = math_label(eta_src, size=LABEL_FONT_SIZE, color=color)
            box = VGroup(eta_lbl, bar["group"]).arrange(DOWN, buff=0.12).move_to(card.get_bottom() + UP * 0.45)
            for fill in (bar["upper"], bar["lower"]):
                fill.move_to(bar["track"].get_center()).align_to(bar["track"], LEFT)
            room = room_section(np.array([px + 0.35, -0.85, 0.0]), w=room_w, h=room_h, wall=0.13, slab=slab,
                                window=(0.3, 0.75))
            for part in room["shell"]:
                part.set_stroke(P_WHITE).set_fill(P_WHITE, opacity=1)
            for win in room["windows"]:
                emphasize_casement(win, color=P_CYAN)
            height = 0.7 * room_h
            person = staffage_motion("shared-person-pathlines", unit=height / staffage_height("shared-person-pathlines"),
                                     anchor=np.array([px + 0.95, room["y_f"], 0.0]))
            return card, hdr, box, room, person

        p1 = -4.4
        bar1 = loss_bar(1.0, 1.0, P_RED)
        card1, hdr1, box1_stat, room1, person1 = panel(
            p1, P_RED, semio_color("dark-8-9"), tr("Window ventilation", "Fensterlüftung"), tr("natural, no heat recovery", "freie Lüftung ohne WRG"), r"\eta_{\mathrm{WRG}} = 0", bar1)
        x_l, y_f, y_c, lo, hi = room1["x_l"], room1["y_f"], room1["y_c"], room1["win_lo"], room1["win_hi"]
        win1 = room1["windows"][0]
        in1 = [path((x_l - 1.05, lo - 0.05), (x_l - 0.4, lo + 0.08), (x_l + 0.3, lo + 0.06), (x_l + 0.8, y_f + 0.15),
                    (p1 + 0.5, y_f + 0.2))]
        out1 = [path((p1 + 0.55, y_c - 0.35), (p1 + 0.3, y_c - 0.15), (x_l + 0.45, hi - 0.06), (x_l - 0.35, hi - 0.04),
                     (x_l - 1.0, hi + 0.35))]
        lbl_in1 = legend([tr("Cold air", "Kaltluft"), "(0 °C)"], P_BLUE, 0.75, [p1 - 1.4, -1.55, 0])
        lbl_out1 = legend([tr("Warm air", "Warmluft"), "(20 °C)"], P_ORANGE, 0.75, [p1 - 1.3, 0.45, 0])
        col1 = VGroup(card1, hdr1, room1["shell"], win1["group"], person1, lbl_in1, lbl_out1, box1_stat)

        p2 = 0.0
        bar2 = loss_bar(0.1, 0.2, P_GREEN)
        card2, hdr2, box2_stat, room2, person2 = panel(
            p2, P_BLUE, semio_color("dark-8-9"), tr("Central HRV", "Zentrale WRG"), tr("central ventilation unit", "zentrales Lüftungsgerät"), tr(r"\eta_{\mathrm{WRG}} \approx 0.8 \ldots 0.9", r"\eta_{\mathrm{WRG}} \approx 0{,}8 \ldots 0{,}9"), bar2)
        y_c2 = room2["y_c"]
        mvhr_box = _mvhr(np.array([p2 + 0.15, 0.6, 0.0]))
        lbl_mvhr = Text(tr("Central HRV", "WRG Zentral"), font_size=LABEL_FONT_SIZE, color=P_YELLOW).scale(0.65)
        lbl_mvhr.next_to(mvhr_box, UP, buff=0.08)
        x_s, x_e = mvhr_box.get_left()[0] + 0.1, mvhr_box.get_right()[0] - 0.1
        ducts2 = VGroup(*[Line([x + dx, mvhr_box.get_bottom()[1], 0], [x + dx, y_c2, 0], color=P_WHITE, stroke_width=1.4,
                               stroke_opacity=0.7) for x in (x_s, x_e) for dx in (-0.09, 0.09)])
        fresh2 = [path((p2 - 1.95, 1.0), (p2 - 1.0, 0.95), (mvhr_box.get_left()[0] + 0.02, 0.62))]
        fort2 = [path((mvhr_box.get_right()[0] - 0.02, 0.62), (p2 + 1.5, 0.95), (p2 + 1.95, 1.05))]
        sup2 = [path((x_s, 0.33), (x_s, -0.3), (p2 - 0.6, -0.85), (p2 - 0.62, -1.32))]
        mix2 = [path((p2 - 0.62, -1.32), (p2 - 0.05, -1.22), (p2 + 0.65, -0.85), (p2 + 1.3, -0.34))]
        ext2 = [path((p2 + 1.3, -0.34), (p2 + 0.9, -0.27), (x_e, -0.1), (x_e, 0.33))]
        lbl_supply = legend([tr("preheated", "vorgewärmte"), tr("supply air (16 °C)", "Zuluft (16 °C)")], P_GREEN, 0.62, [p2 - 1.53, -0.75, 0])
        col2 = VGroup(card2, hdr2, room2["shell"], room2["glass"], person2, ducts2, mvhr_box, lbl_mvhr, lbl_supply,
                      box2_stat)

        p3 = 4.4
        bar3 = loss_bar(0.1, 0.3, P_GREEN)
        card3, hdr3, box3_stat, room3, person3 = panel(
            p3, P_GREEN, semio_color("dark-8-9"), tr("Decentral HRV", "Dezentrale WRG"), tr("Reversing fan with ceramic core", "Pendellüfter mit Keramik"), tr(r"\eta_{\mathrm{WRG}} \approx 0.7 \ldots 0.9", r"\eta_{\mathrm{WRG}} \approx 0{,}7 \ldots 0{,}9"), bar3)
        x_l3, gx3 = room3["x_l"], room3["glass_x"]
        y_u = 0.5 * (room3["win_hi"] + room3["y_c"])
        unit3 = _pendulum_unit(np.array([gx3, y_u, 0.0]))
        ab3 = [path((p3 + 0.6, -0.95), (x_l3 + 0.45, y_u - 0.05), (gx3, y_u), (p3 - 1.4, y_u + 0.2), (p3 - 1.9, 0.12))]
        zu3 = [path((p3 - 1.9, -0.02), (p3 - 1.4, y_u - 0.12), (gx3, y_u), (x_l3 + 0.45, y_u - 0.1), (p3 + 0.6, -1.0))]
        lbl_cyc1 = legend([tr("70 s exhaust", "70 s Abluft"), tr("(storing)", "(Speichern)")], P_ORANGE, 0.65, [p3 - 1.35, 0.45, 0])
        lbl_cyc2 = legend([tr("70 s supply", "70 s Zuluft"), f"({unit_text('16', '°C')})"], P_GREEN, 0.65, [p3 - 1.52, -1.05, 0])
        col3 = VGroup(card3, hdr3, room3["shell"], room3["glass"], person3, unit3, lbl_cyc1, lbl_cyc2, box3_stat)

        paths = VGroup(*in1, *out1, *fresh2, *fort2, *sup2, *mix2, *ext2, *ab3, *zu3)
        fills = VGroup(*[b[k] for b in (bar1, bar2, bar3) for k in ("upper", "lower")])
        place(VGroup(col1, col2, col3), paths, fills)
        win1["hinge"] = pivot + 0.85 * (win1["hinge"] - pivot) + UP * 0.35
        guides1 = VGroup(flow_guides(in1, P_BLUE, opacity=0.2), flow_guides(out1, P_ORANGE, opacity=0.2))
        guides2 = VGroup(flow_guides(fresh2, P_BLUE, opacity=0.2), flow_guides(fort2, GREY_B, opacity=0.2),
                         flow_guides(sup2, P_GREEN, opacity=0.2), flow_guides(mix2, P_GREEN, opacity=0.2),
                         flow_guides(ext2, P_ORANGE, opacity=0.2))
        guides3 = VGroup(flow_guides(ab3, P_ORANGE, opacity=0.2, tips=False))
        active = []
        pendulum = {"out": True, "half": 0}

        def window_air(rt):
            return [_streams([(in1, P_BLUE, P_ORANGE), (out1, P_ORANGE)], rt, speed=0.5, waves=4)]

        def central_air(rt):
            return [_streams([(fresh2, P_BLUE), (fort2, GREY_B), (sup2, P_GREEN), (mix2, P_GREEN, P_ORANGE), (ext2, P_ORANGE)], rt,
                             speed=0.55, waves=3), *_spin([mvhr_box], rt)]

        def pendulum_air(rt):
            n = max(1, int(round(rt / 2.2)))
            flows, turns = [], []
            for _ in range(n):
                out = pendulum["out"]
                flow = _streams([(ab3, P_ORANGE)] if out else [(zu3, P_BLUE, P_GREEN)], rt / n, speed=0.55, waves=4)
                flow.run_time = rt / n
                turn = _pendulum_half(unit3, pendulum["half"], rt / n)
                flows.append(flow)
                turns.append(turn)
                pendulum["out"] = not out
                pendulum["half"] += 1
            return [Succession(*flows), Succession(*turns)]

        def systems(rt):
            return [a for fn in active for a in fn(rt)]

        def grow(bar):
            return [GrowFromEdge(bar["upper"], LEFT), GrowFromEdge(bar["lower"], LEFT)]

        def reveal(col, bar, *extra, rt):
            self.play(FadeIn(col), *extra, *systems(rt), run_time=rt)
            self.play(*grow(bar), *systems(1.0), run_time=1.0)

        reveal(col1, bar1, rt=1.0)
        self.play(open_window(win1, run_time=0.8), FadeIn(guides1), run_time=0.8)
        active.append(window_air)
        self.play(*systems(1.0), run_time=1.0)
        hold_for(self, N, "window", used=4.6, during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "central"))
        reveal(col2, bar2, FadeIn(guides2), rt=1.3)
        active.append(central_air)
        hold_for(self, N, "central", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "decentral"))
        reveal(col3, bar3, FadeIn(guides3), rt=1.3)
        active.append(pendulum_air)
        hold_for(self, N, "decentral", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "outro"))
        hold_for(self, N, "outro", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "passivhaus"))
        verdict = note_line(tr("Passive house: heat-recovery ventilation replaces windows", "Passivhaus: Zu-/Abluft mit WRG — Fensterlüftung entfällt als Hauptweg"), color=P_ORANGE)
        mark = cross_mark(size=0.2).move_to(win1["hinge"])
        ring_bad = SurroundingRectangle(col1, color=P_RED, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5)
        ring_good = SurroundingRectangle(VGroup(col2, col3), color=P_GREEN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5)
        self.play(FadeIn(verdict, shift=DOWN * 0.1), Create(ring_bad), Create(ring_good), Create(mark), *systems(1.2), run_time=1.2)
        hold_for(self, N, "passivhaus", during=systems)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Duct cross-section from the hygiene airflow
class Beat4_Kanalquerschnitt(Scene):
    NARRATION = [
        ("intro",
         r"The system has to deliver the hygiene airflow of $0.061\,\mathrm{m^3/s}$. Which duct cross-section does that take?",
         r"Die Anlage muss den Hygieneluftstrom von $0{,}061\,\mathrm{m^3/s}$ fördern — welchen Kanalquerschnitt braucht sie dafür?"),
        ("continuity",
         r"Continuity links the three quantities: the volume flow $q_{v,R}$ is the mean velocity $v_m$ times the cross-sectional area $A$.",
         r"Die Kontinuität verknüpft sie: Der Volumenstrom $q_{v,R}$ ist die mittlere Geschwindigkeit $v_m$ mal die Querschnittsfläche $A$."),
        ("vm",
         r"To keep the duct quiet, the mean velocity is limited to about $2.5\,\mathrm{m/s}$. This small duct is far too fast.",
         r"Für einen leisen Kanal wird die Geschwindigkeit auf etwa $2{,}5\,\mathrm{m/s}$ begrenzt — der kleine Kanal hier ist viel zu schnell."),
        ("area",
         r"For the same flow, more area means less speed. At $2.5\,\mathrm{m/s}$: $A = q_{v,R} / v_m \approx 0.0244\,\mathrm{m^2}$.",
         r"Mehr Fläche bremst denselben Strom: bei $2{,}5\,\mathrm{m/s}$ ist $A = q_{v,R} / v_m \approx 0{,}0244\,\mathrm{m^2}$."),
        ("radius",
         r"For a round duct the radius is $r = \sqrt{A/\pi}$: about $9\,\mathrm{cm}$, a diameter of roughly $18\,\mathrm{cm}$.",
         r"Beim runden Kanal ist $r = \sqrt{A/\pi}$: rund $9\,\mathrm{cm}$, also ein Durchmesser von etwa $18\,\mathrm{cm}$."),
        ("cooling",
         "Cooling technology will need several times more air for its load. The method stays the same, only the duct gets bigger.",
         "Die Kühltechnik braucht für ihre Last ein Vielfaches an Luft — dieselbe Methode, nur ein größerer Kanal."),
        ("handover",
         r"Heating takes over $\eta$ for its ventilation loss $\dot{Q}_V$, and cooling technology takes the airflow for its duct sizing.",
         r"Die Heizung übernimmt $\eta$ für ihren Lüftungsverlust $\dot{Q}_V$, die Kühltechnik den Luftstrom für ihre Kanalauslegung."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("From airflow to duct cross-section", "Vom Luftstrom zum Kanalquerschnitt"))
        din = _din_ref(_references.cite("vdi-2087"))
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        units_per_m, r_start = 12.0, 0.62
        r_final = float(np.sqrt(Q_VR / V_M / np.pi)) * units_per_m
        cy, disc_x, tube_x0 = 0.55, -2.6, -6.5
        disc_c = np.array([disc_x, cy, 0.0])
        r = ValueTracker(r_start)
        shown = ValueTracker(0.0)

        def area():
            return np.pi * (r.get_value() / units_per_m) ** 2

        def vel():
            return Q_VR / area()

        air_opacity = 0.28

        def tube():
            rr = r.get_value()
            # One closed air path: full barrel through the disc centre + right semicircle face.
            air = VMobject(stroke_width=0, fill_color=P_BLUE, fill_opacity=air_opacity)
            arc = Arc(radius=rr, start_angle=PI / 2, angle=-PI, arc_center=disc_c)
            air.set_points_as_corners([
                np.array([tube_x0, cy + rr, 0.0]),
                np.array([disc_x, cy + rr, 0.0]),
            ])
            air.append_points(arc.points.copy())
            air.add_line_to(np.array([tube_x0, cy - rr, 0.0]))
            air.close_path()
            air.set_fill(P_BLUE, opacity=air_opacity).set_stroke(width=0)
            walls = [
                Line(np.array([tube_x0, cy + s * rr, 0.0]), np.array([disc_x, cy + s * rr, 0.0]),
                     color=P_TEAL, stroke_width=4)
                for s in (-1, 1)
            ]
            return VGroup(air, *walls)

        def disc():
            return Circle(radius=r.get_value(), color=P_TEAL, stroke_width=4, fill_opacity=0.0).move_to(disc_c)

        duct_wall = always_redraw(tube)
        duct_disc = always_redraw(disc)
        a_tag = always_redraw(lambda: math_label("A", disc_c, size=FORMULA_FONT_SIZE, color=P_BLUE))

        def lanes(rr):
            return [smooth_path([np.array([tube_x0 + 0.15, cy + k * rr, 0.0]), np.array([(tube_x0 + disc_x) / 2, cy + k * rr, 0.0]),
                                 np.array([disc_x - 0.08, cy + k * rr, 0.0])]) for k in (-0.55, 0.0, 0.55)]

        def duct_air(rt, speed=None):
            pace = (0.18 + 0.12 * min(vel() / V_M, 4.0)) if speed is None else speed
            # Same hue as the air fill, stroke-like streaks — avoid a second translucent colour stacking on the fill.
            return [flow_animation([(lanes(r.get_value()), P_TEAL)], waves=2, radius=0.045, cycles=pace * rt,
                                     streak=True, peak_opacity=0.0, stroke_width=1.6)]

        rx = 1.25

        def line_at(y):
            return np.array([rx, y, 0.0])

        q_read = math_readout(lambda: rf"q_{{v,\mathrm{{R}}}} = {label_number(Q_VR * shown.get_value(), 3)}\,\frac{{\mathrm{{m^{{3}}}}}}{{\mathrm{{s}}}}",
                              line_at(1.95), size=BODY_FONT_SIZE, color=P_YELLOW)
        v_read = math_readout(lambda: rf"v_{{m}} = {label_number(vel(), 1)}\,\frac{{\mathrm{{m}}}}{{\mathrm{{s}}}}", line_at(1.25),
                              size=BODY_FONT_SIZE, color=P_CYAN)
        a_read = math_readout(lambda: rf"A = {label_number(area(), 4)}\,\mathrm{{m^{{2}}}}", line_at(0.55),
                              size=BODY_FONT_SIZE, color=P_BLUE)
        r_read = math_readout(lambda: rf"r = {label_number(r.get_value() / units_per_m, 3)}\,\mathrm{{m}}", line_at(-0.15),
                              size=BODY_FONT_SIZE, color=P_YELLOW)
        d_read = math_readout(lambda: rf"d = 2\,r = {label_number(200 * r.get_value() / units_per_m)}\,\mathrm{{cm}}",
                              line_at(-0.85), size=BODY_FONT_SIZE, color=P_WHITE)

        v_gauge = meter("m/s", length=2.4, thickness=0.3, color=P_GREEN, vertical=False)
        v_gauge["group"] = VGroup(v_gauge["track"], v_gauge["fill"])
        v_gauge["group"].move_to(np.array([5.45, 1.25, 0.0]))
        v_scale = 8.0
        bind_meter(v_gauge, _live_value(lambda: min(vel() / v_scale, 1.0)))
        v_gauge["fill"].add_updater(lambda m: m.set_fill(P_RED if vel() > V_M * 1.001 else P_GREEN))
        limit_x = v_gauge["track"].get_left()[0] + 0.04 + (V_M / v_scale) * v_gauge["span"]
        limit = VGroup(
            Line(np.array([limit_x, 1.25 - 0.26, 0.0]), np.array([limit_x, 1.25 + 0.26, 0.0]), color=P_YELLOW, stroke_width=3),
            math_label(tr(r"2.5", r"2{,}5"), np.array([limit_x, 1.25 - 0.48, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW),
        )

        def cont_panel():
            return math_panel([
                ("qvr", r"q_{v,\mathrm{R}}", P_YELLOW), (None, "=", P_WHITE),
                ("vm", r"v_{m}", P_CYAN), (None, r"\cdot", P_WHITE),
                ("A", "A", P_BLUE),
            ], color=P_YELLOW)

        def area_panel():
            return math_panel([
                ("A", "A", P_BLUE), (None, "=", P_WHITE),
                ("frac", rf"\frac{{\textcolor{{{P_YELLOW}}}{{q_{{v,\mathrm{{R}}}}}}}}{{\textcolor{{{P_CYAN}}}{{v_{{m}}}}}}", P_WHITE),
                (None, "=", P_WHITE),
                ("val", tr(r"\frac{0.061\,\frac{\mathrm{m^{3}}}{\mathrm{s}}}{2.5\,\frac{\mathrm{m}}{\mathrm{s}}} \approx 0.0244\,\mathrm{m^{2}}", r"\frac{0{,}061\,\frac{\mathrm{m^{3}}}{\mathrm{s}}}{2{,}5\,\frac{\mathrm{m}}{\mathrm{s}}} \approx 0{,}0244\,\mathrm{m^{2}}"), P_BLUE),
            ], color=P_BLUE)

        def radius_panel():
            return math_panel([
                ("r", "r", P_YELLOW), (None, "=", P_WHITE),
                ("root", rf"\sqrt{{\frac{{\textcolor{{{P_BLUE}}}{{A}}}}{{\pi}}}}", P_WHITE),
                (None, "=", P_WHITE),
                ("val", tr(r"\sqrt{\frac{0.0244\,\mathrm{m^{2}}}{\pi}} \approx 0.088\,\mathrm{m}", r"\sqrt{\frac{0{,}0244\,\mathrm{m^{2}}}{\pi}} \approx 0{,}088\,\mathrm{m}"), P_YELLOW),
            ], color=P_YELLOW)

        self.add(duct_wall, duct_disc, a_tag)
        self.add(q_read)
        self.play(shown.animate(rate_func=smooth).set_value(1.0), *duct_air(2.6, 0.5), run_time=2.6)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6, during=duct_air)

        caption = swap_caption(self, caption, subtitle_text(N, "continuity"))
        cp, cp_box, cp_items = cont_panel()
        self.play(FadeIn(cp), Create(cp_box), *duct_air(1.0), run_time=1.0)
        rings = [highlight_param(cp_items, key, color=color) for key, color in
                 (("qvr", P_YELLOW), ("vm", P_CYAN), ("A", P_BLUE))]
        self.add(v_read, a_read)
        self.play(*[Create(g) for g in rings], *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "continuity", during=duct_air)
        self.play(*[FadeOut(g) for g in rings], *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "vm"))
        self.play(FadeIn(v_gauge["group"]), FadeIn(limit), *duct_air(1.2), run_time=1.2)
        ring_v = SurroundingRectangle(v_read, color=P_RED, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        self.play(Create(ring_v), *duct_air(1.2), run_time=1.2)
        hold_for(self, N, "vm", during=duct_air)
        self.play(FadeOut(ring_v), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "area"))
        ap, ap_box, ap_items = area_panel()
        self.play(stage.transition("ventilation.mechanical.duct.continuity-to-area-formula", source=cp, target=ap), stage.transition("ventilation.mechanical.duct.continuity-to-area-box", source=cp_box, target=ap_box), *duct_air(1.2), run_time=1.2)
        self.play(r.animate(rate_func=smooth).set_value(r_final), *duct_air(3.4), run_time=3.4)
        ring_a = SurroundingRectangle(a_read, color=P_BLUE, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        ring_ok = SurroundingRectangle(v_read, color=P_GREEN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        self.play(Create(ring_a), Create(ring_ok), *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "area", during=duct_air)
        self.play(FadeOut(ring_a), FadeOut(ring_ok), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "radius"))
        rp, rp_box, rp_items = radius_panel()
        radius_line = always_redraw(lambda: Line(disc_c, disc_c + RIGHT * r.get_value(), color=P_YELLOW, stroke_width=4))
        radius_dot = Dot(disc_c, radius=0.06, color=P_YELLOW)
        r_tag = always_redraw(lambda: math_label("r", disc_c + RIGHT * r.get_value() / 2 + UP * 0.2, size=BODY_FONT_SIZE,
                                                 color=P_YELLOW))
        self.remove(a_tag)
        self.add(r_read, d_read)
        self.play(stage.transition("ventilation.mechanical.duct.area-to-radius-formula", source=ap, target=rp), stage.transition("ventilation.mechanical.duct.area-to-radius-box", source=ap_box, target=rp_box), FadeIn(radius_dot),
                  Create(radius_line), FadeIn(r_tag), *duct_air(1.6), run_time=1.6)
        ring_root = highlight_param(rp_items, "root", color=P_BLUE)
        self.play(Create(ring_root), Indicate(duct_disc, color=P_BLUE), *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "radius", during=duct_air)
        self.play(FadeOut(ring_root), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "cooling"))
        r_big = 1.6 * r_final
        ghost = VGroup(
            DashedVMobject(Circle(radius=r_big, color=P_CYAN, stroke_width=3).move_to(disc_c), num_dashes=40),
            *[DashedLine(np.array([tube_x0, cy + s * r_big, 0.0]), np.array([disc_x, cy + s * r_big, 0.0]), color=P_CYAN,
                         stroke_width=3) for s in (-1, 1)],
        )
        ghost_tag = Text(tr("Cooling: more air, larger duct", "Kühltechnik: mehr Luft, größerer Kanal"), font_size=LABEL_FONT_SIZE, color=P_CYAN)
        ghost_tag.move_to(np.array([-2.6, cy - r_big - 0.3, 0.0]))
        self.play(FadeOut(rp), FadeOut(rp_box), Create(ghost), FadeIn(ghost_tag), *duct_air(1.8), run_time=1.8)
        hold_for(self, N, "cooling", during=duct_air)

        caption = swap_caption(self, caption, subtitle_text(N, "handover"))
        heat_chip = _math_chip(tr(r"\text{→ Heating:}\;\eta_{\mathrm{WRG}}\;\to\;\dot{Q}_{\mathrm{V}}", r"\text{→ Heizung:}\;\eta_{\mathrm{WRG}}\;\to\;\dot{Q}_{\mathrm{V}}"), P_ORANGE)
        cool_chip = _math_chip(tr(r"\text{→ Cooling:}\;q_{v,\mathrm{R}}\;\to\;A,\,r", r"\text{→ Kühltechnik:}\;q_{v,\mathrm{R}}\;\to\;A,\,r"), P_CYAN)
        VGroup(heat_chip, cool_chip).arrange(RIGHT, buff=0.5).move_to(np.array([0.9, -1.95, 0.0]))
        self.play(FadeIn(heat_chip, shift=UP * 0.1), FadeIn(cool_chip, shift=UP * 0.1), *duct_air(1.2), run_time=1.2)
        hold_for(self, N, "handover", during=duct_air)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
