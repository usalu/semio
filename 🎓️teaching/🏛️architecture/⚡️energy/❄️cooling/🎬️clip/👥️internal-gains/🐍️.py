

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


_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
apply_scene_style = _manim_module.apply_scene_style
section_heading = _manim_module.section_heading
BEAT_SUBTITLE_FADE = _manim_module.BEAT_SUBTITLE_FADE
SUBTITLE_FONT_SIZE = _manim_module.SUBTITLE_FONT_SIZE
BODY_FONT_SIZE = _manim_module.BODY_FONT_SIZE
LABEL_FONT_SIZE = _manim_module.LABEL_FONT_SIZE
FORMULA_FONT_SIZE = _manim_module.FORMULA_FONT_SIZE
_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
SINGLE_METRE, ROW_METRE, GRID_METRE, LAPTOP_GRID_METRE, CARD_METRE, CARD_LAMP_METRE = 0.68, 0.47, 0.19, 1.5, 0.6, 1.1
P_PRIMARY = _manim_module.P_PRIMARY
window_frame = _manim_module.window_frame
P_DEEP_DARK = _manim_module.P_DEEP_DARK
P_WHITE = _manim_module.P_WHITE
P_CYAN = _manim_module.P_CYAN
P_TEAL = _manim_module.P_TEAL
P_ORANGE = _manim_module.P_ORANGE
P_YELLOW = _manim_module.P_YELLOW
P_RED = _manim_module.P_RED
P_BLUE = _manim_module.P_BLUE
watt_anchor = _manim_module.watt_anchor
highlight_param = _manim_module.highlight_param
house_section = _manim_module.house_section
room_section = _manim_module.room_section
staffage = _manim_module.staffage
staffage_motion = _manim_module.staffage_motion
droplets = _manim_module.droplets
ripples = _manim_module.ripples
smooth_path = _manim_module.smooth_path
flow_animation = _manim_module.flow_animation
pulse_flashes = _manim_module.pulse_flashes
math_label = _manim_module.math_label
math_readout = _manim_module.math_readout
math_panel = _manim_module.math_panel
label_number = _manim_module.label_number
caption_bar = _manim_module.caption_bar
swap_caption = _manim_module.swap_caption
hold_for = _manim_module.hold_for
subtitle_text = _manim_module.subtitle_text
SAFE_BOTTOM = _manim_module.SAFE_BOTTOM
set_vo_language = _manim_module.set_vo_language
tr = _manim_module.tr
unit_text = _manim_module.unit_text
load_vo_timing = _manim_module.load_vo_timing

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(_Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Interne Wärmegewinne"


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


#region Shared helpers

_CAST = "cooling-internal-gains-"
_SHARED = {"desk": "shared-desk", "lamp": "shared-lamp-summer"}
_DESK_TOP_M = 0.72
_CHAIR_DX_M = -0.4
_ACTIVITY_UNIT = 0.69
_LAMP_TOP_M = 0.85

_HEAT_M = {
    "seated-worker": {"chest": (0.15, 0.95), "head": (0.12, 1.3), "head_top": (0.1, 1.43), "mouth": (0.22, 1.27), "laptop": (0.8, 0.8)},
    "sleeper": {"chest": (1.1, 0.5)},
    "walker": {"chest": (0.3, 1.3)},
    "jogger": {"chest": (1.3, 1.3)},
    "standing": {"chest": (0.0, 1.25)},
    "desk": {"heat": (0.6, 0.95)},
    "laptop": {"heat": (-0.03, 0.12)},
    "server-rack": {"heat": (0.0, 1.1)},
    "lamp": {"bulb": (0.0, 0.08)},
}


def _figure(name, *, unit, at, moving=True, **kw):
    """🧍 One cast of this clip at a world anchor, animated through the staffage clip loop or held as one frame."""
    build = staffage_motion if moving else staffage
    return build(_SHARED.get(name, _CAST + name), unit=unit, anchor=np.array([at[0], at[1], 0.0]), **kw)


def _spot(name, point, *, unit, at):
    """🌡️ A heat-relevant point of a cast, in metres from its anchor, mapped into the scene."""
    dx, dy = _HEAT_M[name][point]
    return np.array([at[0] + dx * unit, at[1] + dy * unit, 0.0])


def _lamp(room, x, unit, state="lamp"):
    """💡 A pendant lamp, lit or dimmed, hanging from the ceiling slab and the point of its bulb."""
    top = (x, room["y_c"] - _LAMP_TOP_M * unit)
    return _figure(state, unit=unit, at=top, moving=False), _spot("lamp", "bulb", unit=unit, at=top)


def _seated(room, x):
    """🪑 A seated office worker at the desk of the cast, on the room scale and feet on the slab."""
    unit = room["h"] / 2.5
    at = (x, room["y_f"])
    return {
        "group": _figure("seated-worker", unit=unit, at=at),
        **{key: _spot("seated-worker", key, unit=unit, at=at) for key in _HEAT_M["seated-worker"]},
    }


def _workstation(room, x):
    """🖥️ One workstation — desk and a swivel chair — standing on the room floor, and the desk's heat point."""
    unit = room["h"] / 2.5
    at = (x, room["y_f"])
    desk = _figure("desk", unit=unit, at=at, moving=False)
    chair = _figure("office-chair", unit=unit, at=(x + _CHAIR_DX_M * unit, room["y_f"]), moving=False)
    return VGroup(chair, desk), _spot("desk", "heat", unit=unit, at=at)


def _office_furniture(room):
    """🏢 Two workstations standing on the room floor."""
    group, warm = VGroup(), []
    for x in (room["x_l"] + 0.9, room["center"][0] + 0.4):
        station, heat = _workstation(room, x)
        group.add(station)
        warm.append(heat)
    return {"group": group, "warm": warm}


_ACTIVITY_CAST = {"sleep": "sleeper", "sit": "seated-worker", "walk": "walker", "sprint": "jogger"}
_ACTIVITY_CENTER_M = {"sleeper": 1.1, "seated-worker": 0.45, "walker": 0.3, "jogger": 1.3}


def _activity_figure(kind, *, unit, at):
    """🏃 A thermal figure per metabolic activity — asleep, seated, walking, running — and its chest heat point."""
    name = _ACTIVITY_CAST[kind]
    at = (at[0] - _ACTIVITY_CENTER_M.get(name, 0.0) * unit, at[1])
    return _figure(name, unit=unit, at=at), _spot(name, "chest", unit=unit, at=at)

#endregion


#region Beat 1 — Office room / enclosed envelope

class Beat1_OfficeRoom(Scene):
    NARRATION = [
        ("intro",
         "Before the sun even touches the building, heat is already produced inside — by the internal sources.",
         "Schon bevor die Sonne das Gebäude trifft, entsteht innen Wärme — aus den internen Quellen."),
        ("facade",
         "Start from the commercial facade, then zoom into the insulated office envelope.",
         "Beginnen wir bei der Gewerbefassade und zoomen in die gedämmte Bürohülle."),
        ("interior",
         "Inside sits a commercial workplace — desks and chairs — sealed by insulation.",
         "Drinnen liegt ein gewerblicher Arbeitsplatz — Tische und Stühle — dicht gedämmt."),
        ("outro",
         "In offices and lecture halls, internal heat is often the primary cooling load.",
         "In Büros und Hörsälen ist die interne Wärme oft die dominante Kühllast."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("The enclosed environment", "Die umschlossene Umgebung"))
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        facade = house_section(np.array([0.0, -0.55, 0.0]))["group"]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "facade"))
        self.play(FadeIn(facade), run_time=1.4)
        self.play(facade.animate.scale(1.35).move_to(DOWN * 0.15), run_time=2.0)
        hold_for(self, self.NARRATION, "facade", used=3.4 + 0.35)

        room = room_section(np.array([0.0, -0.2, 0.0]), w=6.6, h=2.5)
        insulation_box = SurroundingRectangle(room["shell"], buff=_theme_module.UI_SPACING, corner_radius=0, color=P_ORANGE,
                                              stroke_width=_theme_module.CHROME_BORDER)
        office = _office_furniture(room)
        interior_label = Text(tr("Commercial work area", "Gewerblicher Arbeitsbereich"), font_size=BODY_FONT_SIZE, color=P_CYAN)
        interior_label.move_to(np.array([0.95, room["y_c"] - 0.45, 0.0]))
        office_env_label = Text(tr("Office envelope", "Bürohülle"), font_size=SUBTITLE_FONT_SIZE, color=P_WHITE)
        office_env_label.next_to(insulation_box, UP, buff=0.12)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "interior"))
        freeze_drawables(facade)
        freeze_drawables(room["shell"])
        self.play(ReplacementTransform(facade, room["shell"]), FadeIn(room["glass"]), Create(insulation_box),
                  run_time=1.8)
        self.add(room["air"])
        self.play(LaggedStart(*[FadeIn(m) for m in office["group"]], lag_ratio=0.15), FadeIn(interior_label),
                  FadeIn(office_env_label), run_time=1.6)
        hold_for(self, self.NARRATION, "interior", used=3.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.1), run_time=0.8)
        hold_for(self, self.NARRATION, "outro", during=lambda rt: [
            ripples(office["warm"], r_max=0.5, color=P_ORANGE, cycles=max(1.0, rt / 1.3))])

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 — Human factor

class Beat2_HumanFactor(Scene):
    NARRATION = [
        ("intro",
         "A person at a desk is a heat source in the room.",
         "Eine Person am Schreibtisch ist eine Wärmequelle im Raum."),
        ("modes",
         "Heat leaves as radiation, convection, and respiration — both sensible and latent.",
         "Wärme geht als Strahlung, Konvektion und Atmung ab — fühlbar und latent."),
        ("anchor",
         "A single seated adult emits about one hundred twenty watts.",
         "Eine sitzende Person gibt etwa einhundertzwanzig Watt ab."),
        ("activities",
         "How much heat the body emits depends on activity — it rises from sleeping to desk work, walking and high-performance sport.",
         "Wie viel Wärme der Körper abgibt, hängt von der Aktivität ab — sie steigt vom Schlafen über Büroarbeit und Gehen bis zum Hochleistungssport."),
        ("activity_scale",
         "On one watt scale: about seventy watts asleep, one hundred twenty at the desk, three hundred walking, up to nine hundred in sport.",
         "Auf einer Watt-Skala: rund siebzig Watt im Schlaf, hundertzwanzig am Schreibtisch, dreihundert beim Gehen, bis neunhundert beim Sport."),
        ("formula",
         "We calculate Q-dot Personen as n times the specific emission q-dot p, in watts.",
         "Wir berechnen Q-Punkt Personen als n mal die spezifische Abgabe q-Punkt p, in Watt."),
        ("n",
         "n is the number of occupants in the room.",
         "n ist die Anzahl der Personen im Raum."),
        ("qp",
         "q-dot p is roughly one hundred twenty watts per person at desk work.",
         "q-Punkt p liegt bei etwa einhundertzwanzig Watt pro Person bei Büroarbeit."),
        ("scale",
         "Fifty people times one hundred twenty watts is six thousand watts, continuously.",
         "Fünfzig Personen mal einhundertzwanzig Watt sind dauerhaft sechstausend Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Human metabolic heat", "Menschliche Stoffwechselwärme"))
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        room = room_section(np.array([-0.9, 0.0, 0.0]), w=6.6, h=3.4)
        seated = _seated(room, -1.55)
        office_setup = seated["group"]
        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(office_setup), run_time=1.6)

        chest = seated["chest"]
        head, head_top = seated["head"], seated["head_top"]
        mouth = seated["mouth"]
        lbl_rad = Text(tr("Radiation", "Strahlung"), font_size=BODY_FONT_SIZE, color=P_ORANGE)
        lbl_rad.next_to(np.array([chest[0] - 0.85, chest[1] + 0.05, 0.0]), LEFT, buff=0.0)
        plume = [smooth_path([head_top + UP * 0.08 + RIGHT * dx, head_top + UP * 0.45 + RIGHT * (0.12 + dx),
                              head_top + UP * 0.85 + LEFT * (0.05 - dx),
                              np.array([head[0] + 0.1 + dx, room["y_c"] - 0.08, 0.0])])
                 for dx in (-0.08, 0.08)]
        lbl_conv = Text(tr("Convection", "Konvektion"), font_size=BODY_FONT_SIZE, color=P_ORANGE)
        lbl_conv.next_to(np.array([head[0] - 0.3, head_top[1] + 0.65, 0.0]), LEFT, buff=0.0)
        breath_warm = [smooth_path([mouth + RIGHT * 0.06, mouth + RIGHT * 0.6 + UP * 0.12, mouth + RIGHT * 1.25 + UP * 0.32])]
        breath_wet = [smooth_path([mouth + RIGHT * 0.06 + DOWN * 0.04, mouth + RIGHT * 0.65 + DOWN * 0.1,
                                   mouth + RIGHT * 1.25 + DOWN * 0.18])]
        lbl_sens = Text(tr("sensible", "fühlbar"), font_size=LABEL_FONT_SIZE, color=P_RED)
        lbl_sens.next_to(breath_warm[0].get_end(), RIGHT, buff=0.15)
        lbl_lat = Text("latent", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        lbl_lat.next_to(breath_wet[0].get_end(), RIGHT, buff=0.15)
        lbl_resp = Text(tr("Respiration", "Atmung"), font_size=BODY_FONT_SIZE, color=P_RED)
        lbl_resp.next_to(lbl_sens, UP, buff=0.18).align_to(lbl_sens, LEFT)
        drops = droplets(breath_wet[0].get_end() + LEFT * 0.35 + DOWN * 0.04, n=5, spread=(0.25, 0.05), seed=5)

        def body_heat(rt):
            cyc = max(1.0, rt / 1.4)
            return [ripples([chest], r_max=0.7, color=P_ORANGE, cycles=cyc),
                    flow_animation([(plume, P_ORANGE, P_RED)], waves=4, radius=0.055, cycles=cyc),
                    flow_animation([(breath_warm, P_RED, P_ORANGE)], waves=3, radius=0.05, cycles=cyc, streak=False),
                    flow_animation([(breath_wet, P_BLUE)], waves=3, radius=0.045, cycles=cyc, streak=False)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "modes"))
        self.play(ripples([chest], r_max=0.7, color=P_ORANGE, cycles=1.2), FadeIn(lbl_rad), run_time=1.4)
        self.play(flow_animation([(plume, P_ORANGE, P_RED)], waves=4, radius=0.055, cycles=1.0), FadeIn(lbl_conv),
                  run_time=1.4)
        self.play(*body_heat(1.4)[2:], FadeIn(lbl_sens), FadeIn(lbl_resp), FadeIn(lbl_lat),
                  FadeIn(drops, lag_ratio=0.2), run_time=1.4)
        hold_for(self, self.NARRATION, "modes", during=body_heat)

        badge = watt_anchor(120, compare="person", title=tr("Body heat", "Körperwärme"))
        badge.scale(0.85).next_to(room["wall_r"], RIGHT, buff=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "anchor"))
        self.play(FadeIn(badge, shift=LEFT * 0.25), room["air"].animate.set_fill(P_RED, opacity=0.08), *body_heat(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "anchor", during=body_heat)

        desk_group = VGroup(room["group"], office_setup, lbl_rad, lbl_conv, lbl_sens, lbl_lat, lbl_resp, drops, badge)
        self.play(FadeOut(desk_group), run_time=0.7)

        acts = [
            (tr("Sleeping", "Schlafen"), 70, "sleep", P_TEAL),
            (tr("Office work", "Büroarbeit"), 120, "sit", P_CYAN),
            (tr("Walking", "Gehen"), 300, "walk", P_YELLOW),
            (tr("Intense exercise", "Hochleistungssport"), 900, "sprint", P_RED),
        ]
        base_y = -0.75
        xs = [-4.8, -1.7, 1.3, 4.3]
        figures, trackers, readouts, names, heat_spots = [], [], [], [], []
        for (name, watts, pose, color), x in zip(acts, xs):
            fig, heat_at = _activity_figure(pose, unit=_ACTIVITY_UNIT, at=(x, base_y))
            head_y = base_y + (0.25 if pose == "sleep" else 1.25)
            heat_spots.append((heat_at, 0.3 + watts / 1300))
            trk = ValueTracker(0.0)
            read = math_readout(
                lambda trk=trk: rf"{label_number(trk.get_value())}\,\mathrm{{W}}",
                np.array([x, head_y + 0.75 + watts / 1600, 0.0]),
                size=BODY_FONT_SIZE, color=color, edge="center",
            )
            label = Text(name, font_size=LABEL_FONT_SIZE, color=color)
            label.move_to([x, base_y - 0.42, 0.0])
            figures.append(fig); trackers.append(trk); readouts.append(read)
            names.append(label)

        def activity_heat(rt, upto=4):
            cyc = max(1.0, rt / 1.2)
            return [ripples([spot], r_max=r, color=P_ORANGE, cycles=cyc) for spot, r in heat_spots[:upto]]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "activities"))
        for k, (fig, trk, read, label, (name, watts, pose, color)) in enumerate(
            zip(figures, trackers, readouts, names, acts),
        ):
            self.add(read)
            self.play(FadeIn(fig), FadeIn(label), trk.animate.set_value(watts), *activity_heat(1.1, k + 1),
                      run_time=1.1)
        hold_for(self, self.NARRATION, "activities", during=activity_heat)

        static_reads = VGroup(*[
            math_label(rf"{label_number(watts)}\,\mathrm{{W}}",
                       at=np.array([x, base_y + (0.25 if pose == "sleep" else 1.25) + 0.75 + watts / 1600, 0.0]),
                       size=BODY_FONT_SIZE, color=color, edge="center")
            for (name, watts, pose, color), x in zip(acts, xs)
        ])
        self.add(static_reads)
        for read in readouts:
            self.remove(read)

        scale_y = -0.85
        def w_to_x(w):
            return -5.2 + (w / 900.0) * 10.4
        scale_line = Line([w_to_x(0), scale_y, 0], [w_to_x(900), scale_y, 0], color=P_WHITE, stroke_width=2.5)
        ticks = VGroup()
        tick_labels = VGroup()
        for w in (0, 300, 600, 900):
            x = w_to_x(w)
            ticks.add(Line([x, scale_y - 0.09, 0], [x, scale_y + 0.09, 0], color=P_WHITE, stroke_width=2))
            tick_labels.add(math_label(label_number(w), at=np.array([x, scale_y - 0.42, 0.0]),
                                       size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="center"))
        unit_label = math_label(r"\mathrm{W}", at=np.array([w_to_x(900) + 0.45, scale_y - 0.42, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="center")
        dots = VGroup(*[
            Dot([w_to_x(watts), scale_y, 0], radius=0.09, color=color)
            for (name, watts, pose, color) in acts
        ])
        name_targets = []
        read_targets = []
        for i, ((name, watts, pose, color), label) in enumerate(zip(acts, names)):
            x = w_to_x(watts)
            if i == 0:
                read_targets.append(np.array([x, scale_y - 0.85, 0.0]))
                name_targets.append(np.array([x, scale_y - 1.18, 0.0]))
            else:
                read_targets.append(np.array([x, scale_y + 0.82, 0.0]))
                name_targets.append(np.array([x, scale_y + 0.46, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "activity_scale"))
        self.play(
            Create(scale_line), Create(ticks), FadeIn(tick_labels), FadeIn(unit_label),
            *[FadeTransform(fig, dot) for fig, dot in zip(figures, dots)],
            *[label.animate.move_to(t) for label, t in zip(names, name_targets)],
            *[read.animate.move_to(t) for read, t in zip(static_reads, read_targets)],
            run_time=2.2,
        )
        buero_ring = Circle(radius=0.17, color=P_ORANGE, stroke_width=3).move_to(dots[1].get_center())
        self.play(Create(buero_ring), run_time=0.5)
        hold_for(self, self.NARRATION, "activity_scale", used=2.7 + 0.35)

        scale_assembly = VGroup(
            scale_line, ticks, tick_labels, unit_label, dots, buero_ring,
            *names, *static_reads,
        )
        self.play(scale_assembly.animate.scale(0.82).shift(UP * 1.75), run_time=0.8)

        row, box, items = math_panel([
            ("qp_tot", r"\dot{Q}_{\text{Pers}}", P_ORANGE), (None, "=", P_WHITE),
            ("n", "n", P_CYAN), (None, r"\cdot", P_WHITE),
            ("qp", r"\dot{q}_{p}", P_ORANGE), (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ])
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        n_people = VGroup(*[
            _figure("standing", unit=ROW_METRE, at=(-2.4 + i * 1.15, -0.6))
            for i in range(3)
        ])
        n_tr = ValueTracker(0.0)
        n_read = math_readout(
            lambda: rf"n = {label_number(n_tr.get_value())}",
            np.array([1.7, -0.2, 0.0]),
            size=BODY_FONT_SIZE, color=P_CYAN, edge="left",
        )
        qp_fly = None

        for key, color in (("n", P_CYAN), ("qp", P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            if key == "n":
                self.add(n_read)
                self.play(Create(ring), run_time=0.35)
                self.play(
                    LaggedStart(*[FadeIn(p) for p in n_people], lag_ratio=0.25),
                    n_tr.animate.set_value(len(n_people)),
                    run_time=1.3,
                )
                used_n = 0.35 + 1.3
            else:
                qp_fly = static_reads[1].copy()
                self.play(
                    Create(ring),
                    qp_fly.animate.next_to(box, UP, buff=0.14).set_x(items["qp"].get_center()[0]),
                    run_time=0.6,
                )
                used_n = 0.6
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=used_n + 0.35)
            fades = [FadeOut(ring)]
            if qp_fly is not None and key == "qp":
                fades.append(FadeOut(qp_fly))
            self.play(*fades, run_time=0.25)

        self.play(
            FadeOut(scale_assembly), FadeOut(row), FadeOut(box), FadeOut(n_people), FadeOut(n_read),
            run_time=0.7,
        )

        single = _figure("standing", unit=SINGLE_METRE, at=(0.0, -0.95), moving=False)
        single_label = Text(f"{tr('Single person', 'Einzelperson')} ({unit_text('120', 'W')})", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        single_label.next_to(single, DOWN, buff=0.28)
        self.play(FadeIn(single), FadeIn(single_label), run_time=0.9)

        hall_shift = UP * 0.4
        hall_outline = Rectangle(
            width=9.6, height=3.0, stroke_color=semio_color("dark-5-7"), stroke_width=2,
            fill_color=P_DEEP_DARK, fill_opacity=0.85,
        ).move_to(UP * 0.15 + hall_shift)
        tier_y = np.linspace(-1.15, 0.95, 5) + hall_shift[1]
        tier_lines = VGroup(*[
            Line(start=[-4.5, y, 0], end=[4.5, y, 0], stroke_color=semio_color("dark-5-7"), stroke_width=1, stroke_opacity=0.6)
            for y in tier_y
        ])
        grid_icons = VGroup()
        for y_pos in tier_y:
            for c in range(10):
                x_pos = -4.05 + c * 0.9
                grid_icons.add(_figure("standing", unit=GRID_METRE, at=(x_pos, y_pos + 0.03), moving=False))

        n_hall = ValueTracker(0.0)
        n_hall_read = math_readout(
            lambda: rf"n = {label_number(n_hall.get_value())}",
            np.array([-6.2, 0.85, 0.0]),
            size=LABEL_FONT_SIZE, color=P_CYAN, edge="left",
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "scale"))
        self.play(FadeOut(single_label), Create(hall_outline), Create(tier_lines), run_time=0.8)
        self.add(n_hall_read)
        self.play(
            FadeTransform(single, grid_icons[0]),
            LaggedStart(*[TransformFromCopy(grid_icons[0], icon) for icon in grid_icons[1:]], lag_ratio=0.03),
            n_hall.animate.set_value(len(grid_icons)),
            run_time=2.0,
        )
        hall_anchor = watt_anchor(6000, compare="toaster", title=tr("Total heat", "Gesamtwärme"), row=True)
        hall_anchor.next_to(hall_outline, DOWN, buff=0.3)
        clearance = (SAFE_BOTTOM + 0.15) - hall_anchor.get_bottom()[1]
        if clearance > 0:
            hall_anchor.shift(UP * clearance)
        hall_heat_at = [icon.get_center() + UP * 0.05 for icon in grid_icons]

        def hall_heat(rt):
            return [ripples(hall_heat_at, r_max=0.32, rings=2, color=P_ORANGE, cycles=max(1.0, rt / 1.3))]

        self.play(*hall_heat(2.4), hall_outline.animate.set_fill(P_RED, opacity=0.18), FadeIn(hall_anchor),
                  run_time=2.4)
        hold_for(self, self.NARRATION, "scale", during=hall_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 — Devices and lighting

class Beat3_DevicesLighting(Scene):
    NARRATION = [
        ("intro",
         "Every laptop, server rack and light fixture converts electrical power straight into heat.",
         "Jedes Notebook, jedes Serverrack und jede Leuchte wandelt elektrische Leistung direkt in Wärme um."),
        ("devices",
         "Plug loads follow Q-dot Geräte equals the sum of P el times the usage factor f N, in watts.",
         "Steckerlasten: Q-Punkt Geräte gleich Summe aus P el mal Nutzungsfaktor f N, in Watt."),
        ("pel",
         "P el is the electrical power drawn by each device.",
         "P el ist die elektrische Leistung jedes Geräts."),
        ("fn",
         "f N accounts for the fact that not every device runs at once.",
         "f N berücksichtigt, dass nicht jedes Gerät gleichzeitig läuft."),
        ("fn_usage",
         "The usage sets f N: in an office about seven of ten devices run, in a server room all of them, in a meeting room only three.",
         "Die Nutzung bestimmt f N: im Büro laufen etwa sieben von zehn Geräten, im Serverraum alle, im Besprechungsraum nur drei."),
        ("lights",
         "Ceiling lighting adds Q-dot Licht equals the sum of P Licht times the coincidence factor f g, in watts.",
         "Deckenlicht: Q-Punkt Licht gleich Summe aus P Licht mal Gleichzeitigkeit f g, in Watt."),
        ("pl",
         "P Licht is the installed electrical power of the luminaires.",
         "P Licht ist die installierte elektrische Leistung der Leuchten."),
        ("fg",
         "f g is the coincidence factor — the share of luminaires that actually run at the same time.",
         "f g ist der Gleichzeitigkeitsfaktor — der Anteil der gleichzeitig betriebenen Leuchten."),
        ("outro",
         "Together, devices and lights bake the room from the inside out.",
         "Zusammen heizen Geräte und Licht den Raum von innen heraus auf."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Equipment, plug loads and lighting", "Geräte, Steckerlasten und Beleuchtung"))
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        room = room_section(np.array([0.0, 0.25, 0.0]), w=8.6, h=2.6)
        unit = room["h"] / 2.5
        desk_x = -3.7
        desk = _figure("desk", unit=unit, at=(desk_x, room["y_f"]), moving=False)
        desk_y = room["y_f"] + _DESK_TOP_M * unit
        laptop_at = (desk_x + 1.3 * unit, desk_y)
        laptop = _figure("laptop", unit=unit, at=laptop_at, moving=False)
        rack_unit = room["h"] / 2.5
        rack_at = (2.6, room["y_f"])
        server_rack = _figure("server-rack", unit=rack_unit, at=rack_at, moving=False)
        devices = VGroup(desk, laptop, server_rack)
        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), run_time=0.8)
        self.play(LaggedStart(FadeIn(desk), FadeIn(laptop, shift=DOWN * 0.15), FadeIn(server_rack, shift=DOWN * 0.15),
                              lag_ratio=0.35), run_time=1.4)

        plug_y = room["y_f"] + 0.12
        cables = [smooth_path([np.array([room["x_r"] - 0.05, plug_y, 0.0]), np.array([0.0, plug_y, 0.0]),
                               np.array([-1.9, plug_y, 0.0]), np.array([laptop_at[0], desk_y + 0.02, 0.0])]),
                  smooth_path([np.array([room["x_r"] - 0.05, plug_y, 0.0]), np.array([3.4, plug_y, 0.0]),
                               server_rack.get_bottom() + RIGHT * 0.15])]
        cable_lines = VGroup(*[c.copy().set_stroke(P_CYAN, width=1.3, opacity=0.4) for c in cables])
        warm = [_spot("laptop", "heat", unit=unit, at=laptop_at), _spot("server-rack", "heat", unit=rack_unit, at=rack_at)]

        def device_heat(rt):
            cyc = max(1.0, rt / 1.3)
            return [flow_animation([(cables, P_CYAN, P_ORANGE)], waves=4, radius=0.045, cycles=cyc, streak=False),
                    ripples(warm, r_max=0.75, color=P_ORANGE, cycles=cyc)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "devices"))
        self.play(Create(cable_lines), run_time=0.6)
        self.play(*device_heat(2.0), room["air"].animate.set_fill(P_RED, opacity=0.08), run_time=2.0)
        devices_group = VGroup(room["group"], devices, cable_lines)
        self.play(devices_group.animate.scale(0.78).shift(UP * 0.45), run_time=0.8)
        warm[:] = [laptop.get_center() + UP * 0.08, server_rack.get_center() + UP * 0.1]
        cables[:] = [cl.copy() for cl in cable_lines]

        row_d, box_d, items_d = math_panel([
            ("qg", r"\dot{Q}_{\text{Geräte}}", P_CYAN), (None, "=", P_WHITE),
            (None, r"\Sigma", P_WHITE),
            ("pel", r"P_{el}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("fn", r"f_{N}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ])
        self.play(FadeIn(row_d), Create(box_d), *device_heat(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "devices", during=device_heat)

        for key, color in (("pel", P_CYAN), ("fn", P_YELLOW)):
            ring = highlight_param(items_d, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, during=device_heat)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(devices_group), run_time=0.5)
        fn_ring = highlight_param(items_d, "fn", color=P_YELLOW)
        usages = [(tr("Office", "Büro"), 7, P_CYAN, -3.9), (tr("Server room", "Serverraum"), 10, P_ORANGE, 0.0), (tr("Meeting", "Besprechung"), 3, P_TEAL, 3.9)]
        usage_groups = VGroup()
        usage_anim_specs = []
        for name, lit, color, cx in usages:
            label = Text(name, font_size=BODY_FONT_SIZE, color=color)
            depth = label.height + 2 * _theme_module.UI_SPACING
            frame = window_frame(3.4, 2.4, title_width=label.width + 2 * _theme_module.UI_SPACING,
                                 title_depth=depth, color=color).move_to([cx, 0.5, 0])
            label.move_to([frame.get_left()[0] + _theme_module.UI_SPACING + label.width / 2,
                           frame.get_top()[1] - depth / 2, 0])
            icons = VGroup()
            for i in range(10):
                r, c = divmod(i, 5)
                icons.add(_figure("laptop-closed", unit=LAPTOP_GRID_METRE, at=(cx - 1.1 + c * 0.55, 0.72 - r * 0.6), moving=False))
            trk = ValueTracker(0.0)
            read = math_readout(
                lambda trk=trk: rf"f_{{N}} = {label_number(trk.get_value(), 1)}",
                np.array([cx, -0.30, 0.0]), size=BODY_FONT_SIZE, color=color, edge="center",
            )
            usage_groups.add(VGroup(frame, label, icons))
            usage_anim_specs.append((icons, lit, color, trk, cx, read))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fn_usage"))
        usage_opened = []
        self.play(Create(fn_ring), FadeIn(usage_groups), run_time=0.9)
        for icons, lit, color, trk, panel_cx, read in usage_anim_specs:
            self.add(read)
            opened = [_figure("laptop", unit=LAPTOP_GRID_METRE, at=(panel_cx - 1.1 + i % 5 * 0.55, 0.72 - i // 5 * 0.6), moving=False)
                      for i in range(lit)]
            usage_opened.extend(opened)
            self.play(
                LaggedStart(*[AnimationGroup(FadeOut(icons[i]), FadeIn(opened[i])) for i in range(lit)], lag_ratio=0.12),
                trk.animate.set_value(lit / 10),
                run_time=1.1,
            )
        hold_for(self, self.NARRATION, "fn_usage", used=0.9 + 3 * 1.1 + 0.35)

        usage_reads = [read for *_rest, read in usage_anim_specs]
        self.play(
            FadeOut(usage_groups), *[FadeOut(o) for o in usage_opened], *[FadeOut(r) for r in usage_reads],
            FadeOut(fn_ring), FadeOut(row_d), FadeOut(box_d),
            run_time=0.7,
        )

        lroom = room_section(np.array([0.0, 0.3, 0.0]), w=7.4, h=2.7)
        lamp_unit = lroom["h"] / 2.5 * 1.6
        lamps = [_lamp(lroom, x, lamp_unit) for x in (-1.8, 1.8)]
        bulbs = [bulb for _, bulb in lamps]
        fixture_l, fixture_r = lamps[0][0], lamps[1][0]
        fixtures = VGroup(fixture_l, fixture_r)
        dimmed_r = _lamp(lroom, 1.8, lamp_unit, "lamp-dimmed")[0]

        def light_rays(bulb):
            return VGroup(*[Line(bulb, np.array([bulb[0] + dx, lroom["y_f"] + 0.02, 0.0]), color=P_YELLOW,
                                 stroke_width=1.8, stroke_opacity=0.6) for dx in (-1.1, -0.4, 0.4, 1.1)])

        rays_l, rays_r = light_rays(bulbs[0]), light_rays(bulbs[1])
        lit = {"l": True, "r": True}

        def light_heat(rt):
            on = [b for b, k in zip(bulbs, ("l", "r")) if lit[k]]
            paths = [[b, r.get_end()] for b, r_set, k in zip(bulbs, (rays_l, rays_r), ("l", "r")) if lit[k]
                     for r in r_set]
            return [ripples(on, r_max=0.8, color=P_RED, down=True, cycles=max(1.0, rt / 1.3)),
                    pulse_flashes(paths, P_YELLOW, repeats=max(1, int(rt / 1.6)), width=3.5)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lights"))
        self.add(lroom["air"])
        self.play(FadeIn(lroom["shell"]), FadeIn(lroom["glass"]), FadeIn(fixtures), run_time=1.2)
        self.play(LaggedStart(*[Create(r) for r in (*rays_l, *rays_r)], lag_ratio=0.08), run_time=1.2)
        self.play(*light_heat(1.4), lroom["air"].animate.set_fill(P_RED, opacity=0.08), run_time=1.4)

        lighting_group = VGroup(lroom["group"], fixtures, rays_l, rays_r, dimmed_r)
        self.play(lighting_group.animate.scale(0.82).shift(UP * 0.4), run_time=0.7)
        bulbs[:] = [rays_l[0].get_start(), rays_r[0].get_start()]

        row_l, box_l, items_l = math_panel([
            ("ql", r"\dot{Q}_{\text{Licht}}", P_YELLOW), (None, "=", P_WHITE),
            (None, r"\Sigma", P_WHITE),
            ("pl", r"P_{\text{Licht}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("fg", r"f_{g}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ])
        self.play(FadeIn(row_l), Create(box_l), *light_heat(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "lights", during=light_heat)

        for key, color in (("pl", P_CYAN), ("fg", P_YELLOW)):
            ring = highlight_param(items_l, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            if key == "fg":
                fg_tr = ValueTracker(1.0)
                fg_read = math_readout(
                    lambda: rf"f_{{g}} = {label_number(fg_tr.get_value(), 1)}",
                    lambda: fixtures.get_center() + DOWN * 0.95,
                    size=BODY_FONT_SIZE, color=P_YELLOW, edge="center",
                )
                self.add(fg_read)
                lit["r"] = False
                self.play(
                    FadeOut(fixture_r), FadeIn(dimmed_r),
                    rays_r.animate.set_stroke(opacity=0.0),
                    fg_tr.animate.set_value(0.5), *light_heat(1.4),
                    run_time=1.4,
                )
                hold_for(self, self.NARRATION, key, during=light_heat)
                self.remove(fg_read)
                self.play(FadeOut(ring), run_time=0.25)
            else:
                hold_for(self, self.NARRATION, key, during=light_heat)
                self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=light_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 4 — Cumulative internal load

class Beat4_CumulativeLoad(Scene):
    NARRATION = [
        ("intro",
         "In building physics we sum every internal source into one cooling load, Q-dot internal.",
         "In der Bauphysik summieren wir alle internen Quellen zur Kühllast Q-Punkt intern."),
        ("sources",
         "People, plug loads and lighting are the three terms on the right-hand side.",
         "Personen, Steckerlasten und Beleuchtung sind die drei Terme auf der rechten Seite."),
        ("formula",
         "Q-dot i equals Q-dot Personen plus Q-dot Geräte plus Q-dot Licht, all in watts.",
         "Q-Punkt i gleich Q-Punkt Personen plus Q-Punkt Geräte plus Q-Punkt Licht, alles in Watt."),
        ("qi",
         "Q-dot i is the total internal cooling load the system must remove.",
         "Q-Punkt i ist die gesamte interne Kühllast, die das System abführen muss."),
        ("outro",
         "On a hot summer day the cooling system must remove this internal heat on top — even with blinds drawn.",
         "An einem heißen Sommertag muss die Kühlung diese interne Wärme zusätzlich abführen — selbst bei geschlossenen Jalousien."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Sum of the internal gains", "Summe der internen Gewinne"))
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        def mini_person(center=ORIGIN):
            return _figure("standing", unit=CARD_METRE, at=(0.0, 0.0)).move_to(center)

        def mini_laptop(center=ORIGIN):
            return _figure("laptop", unit=CARD_METRE, at=(0.0, 0.0), moving=False).move_to(center)

        def mini_lamp(center=ORIGIN):
            return _figure("lamp", unit=CARD_LAMP_METRE, at=(0.0, 0.0), moving=False).move_to(center)

        def source_card(icon, title_de, term, color, center):
            label = Text(title_de, font_size=BODY_FONT_SIZE, color=color)
            depth = label.height + 2 * _theme_module.UI_SPACING
            frame = window_frame(2.15, 2.1, title_width=label.width + 2 * _theme_module.UI_SPACING,
                                 title_depth=depth, color=color).move_to(center)
            label.move_to([frame.get_left()[0] + _theme_module.UI_SPACING + label.width / 2,
                           frame.get_top()[1] - depth / 2, 0])
            term_t = math_label(term, size=BODY_FONT_SIZE, color=P_WHITE)
            term_t.next_to(frame.get_bottom(), UP, buff=0.14)
            icon.move_to((label.get_bottom() + term_t.get_top()) / 2)
            return VGroup(frame, label, icon, term_t)

        c_p = LEFT * 3.8 + UP * 0.85
        c_e = ORIGIN + UP * 0.85
        c_l = RIGHT * 3.8 + UP * 0.85
        card_p = source_card(mini_person(), tr("People", "Personen"), r"\dot{Q}_{\text{Pers}}", P_ORANGE, c_p)
        card_e = source_card(mini_laptop(), tr("Equipment", "Geräte"), r"\dot{Q}_{\text{Geräte}}", P_CYAN, c_e)
        card_l = source_card(mini_lamp(), tr("Lighting", "Beleuchtung"), r"\dot{Q}_{\text{Licht}}", P_YELLOW, c_l)
        plus_1 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_p + c_e) / 2)
        plus_2 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_e + c_l) / 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sources"))
        self.play(
            LaggedStart(FadeIn(card_p), FadeIn(card_e), FadeIn(card_l), lag_ratio=0.2),
            run_time=1.3,
        )
        self.play(FadeIn(plus_1), FadeIn(plus_2), run_time=0.5)

        def card_heat(rt):
            spots = [card_p[2].get_center() + UP * 0.08, card_e[2].get_center() + UP * 0.05]
            cyc = max(1.0, rt / 1.3)
            return [ripples(spots, r_max=0.38, color=P_ORANGE, cycles=cyc),
                    ripples([card_l[2].get_center() + DOWN * 0.1], r_max=0.38, color=P_RED, down=True, cycles=cyc)]

        hold_for(self, self.NARRATION, "sources", during=card_heat)

        self.play(
            VGroup(card_p, card_e, card_l, plus_1, plus_2).animate.shift(UP * 0.35).scale(0.92),
            run_time=0.7,
        )

        row, box, items = math_panel([
            ("qi", r"\dot{Q}_{i}", P_CYAN), (None, "=", P_WHITE),
            ("pers", r"\dot{Q}_{\text{Pers}}", P_ORANGE), (None, "+", P_WHITE),
            ("ger", r"\dot{Q}_{\text{Geräte}}", P_CYAN), (None, "+", P_WHITE),
            ("licht", r"\dot{Q}_{\text{Licht}}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_PRIMARY),
        ])
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", during=card_heat)

        card_of = {"pers": card_p, "ger": card_e, "licht": card_l}
        for key, color in (("pers", P_ORANGE), ("ger", P_CYAN), ("licht", P_YELLOW), ("qi", P_CYAN)):
            ring = highlight_param(items, key, color=color)
            pulse = ([card_of[key][0].animate(rate_func=there_and_back).set_stroke(width=5.5)]
                     if key in card_of else [])
            self.play(Create(ring), *pulse, run_time=0.5)
            if key == "qi":
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "qi"))
                hold_for(self, self.NARRATION, "qi", during=card_heat)
            else:
                self.play(*card_heat(0.45), run_time=0.45)
            self.play(FadeOut(ring), run_time=0.22)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=card_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 5 — Mitigation & smart design

class Beat5_Mitigation(Scene):
    NARRATION = [
        ("intro",
         "Smart design cuts the load before the chiller must remove it.",
         "Intelligentes Design senkt die Last, bevor die Kälteanlage sie abführen muss."),
        ("high",
         "High plug and lighting loads first heat the office interior.",
         "Hohe Stecker- und Lichtlasten heizen zuerst den Büroraum."),
        ("dim",
         "Controls dim lights to thirty percent and plug loads to forty percent.",
         "Steuerung dimmt Licht auf dreißig und Steckerlasten auf vierzig Prozent."),
        ("outro",
         "Less internal heat gain means a smaller cooling load.",
         "Weniger interne Wärmegewinne bedeuten eine kleinere Kühllast."),
    ]

    def construct(self):
        apply_scene_style(self)

        subtitle = section_heading(tr("Mitigation & smart design", "Minderung & intelligentes Design"))
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        C_HEAT_HOT = P_RED
        C_COOL = P_CYAN
        C_COOL_DIM = P_TEAL
        C_YELLOW = P_YELLOW

        room_w, room_h = 7.0, 2.25
        rs = room_section(np.array([0.0, 0.35, 0.0]), w=room_w, h=room_h, slab=0.2)
        room = rs["shell"]
        warm_fill = rs["air"]
        y_f = rs["y_f"]
        seated = _seated(rs, -0.95)
        unit = rs["h"] / 2.5
        laptop_heat = seated["laptop"]
        laptop_lbl = Text(tr("Device", "Gerät"), font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        laptop_lbl.move_to(laptop_heat + UP * 0.55 + RIGHT * 0.4)
        fixture, bulb_c = _lamp(rs, 2.55, unit * 1.4)
        fixture_dim = _lamp(rs, 2.55, unit * 1.4, "lamp-dimmed")[0]
        beam = VGroup(*[Line(bulb_c, np.array([bulb_c[0] + dx, y_f + 0.02, 0.0]), color=C_YELLOW, stroke_width=1.8,
                             stroke_opacity=0.6) for dx in (-0.65, -0.22, 0.22, 0.65)])
        light_lbl = Text(tr("Lighting", "Beleuchtung"), font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_lbl.next_to(fixture, LEFT, buff=0.3).shift(UP * 0.12)
        window_group = rs["glass"]
        chair = seated["group"]

        _slider_half = 0.52
        light_label = Text(tr("Light", "Licht"), font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        light_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_YELLOW, stroke_width=5)
        light_knob = Dot(color=C_YELLOW, radius=0.1)
        light_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_YELLOW)

        plug_label = Text(tr("Plug", "Stecker"), font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        plug_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        plug_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_HEAT_HOT, stroke_width=5)
        plug_knob = Dot(color=C_HEAT_HOT, radius=0.1)
        plug_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)

        ctrl_panel = RoundedRectangle(
            width=room_w, height=0.9, corner_radius=0,
            color=C_COOL_DIM, stroke_width=2,
            fill_color=_theme_module.CHROME_FILL, fill_opacity=0.95,
        ).next_to(room, DOWN, buff=0.18)

        ctrl_title = Text(tr("Smart load control", "Smarte Laststeuerung"), font_size=LABEL_FONT_SIZE, color=C_COOL)
        ctrl_title.move_to(ctrl_panel.get_top() + DOWN * 0.18)

        light_track.move_to(ctrl_panel.get_center() + LEFT * 2.0 + DOWN * 0.12)
        light_fill.put_start_and_end_on(light_track.get_start(), light_track.get_end())
        light_knob.move_to(light_track.get_end())
        light_label.next_to(light_track, LEFT, buff=0.2)
        light_pct.next_to(light_track, RIGHT, buff=0.26)

        plug_track.move_to(ctrl_panel.get_center() + RIGHT * 1.9 + DOWN * 0.12)
        plug_fill.put_start_and_end_on(plug_track.get_start(), plug_track.get_end())
        plug_knob.move_to(plug_track.get_end())
        plug_label.next_to(plug_track, LEFT, buff=0.2)
        plug_pct.next_to(plug_track, RIGHT, buff=0.26)

        self.add(warm_fill)
        self.play(FadeIn(room), run_time=1.0)
        self.play(
            FadeIn(window_group),
            FadeIn(chair),
            run_time=1.0,
        )
        self.play(
            FadeIn(laptop_lbl),
            FadeIn(fixture),
            FadeIn(beam),
            FadeIn(light_lbl),
            run_time=0.9,
        )

        light_on = ValueTracker(1.0)
        plug_on = ValueTracker(1.0)
        heat_spots = {"laptop": laptop_heat, "person": seated["chest"]}

        def room_heat(rt):
            cyc = max(1.0, rt / 1.3)
            anims = [ripples([heat_spots["person"]], r_max=0.55, color=P_ORANGE, cycles=cyc),
                     ripples([heat_spots["laptop"]], r_max=0.3 + 0.5 * plug_on.get_value(), color=C_HEAT_HOT, cycles=cyc),
                     ripples([bulb_c], r_max=0.25 + 0.6 * light_on.get_value(), color=C_HEAT_HOT, down=True, cycles=cyc)]
            if light_on.get_value() > 0.5:
                anims.append(pulse_flashes([[bulb_c, r.get_end()] for r in beam], C_YELLOW,
                                           repeats=max(1, int(rt / 1.6)), width=3.5))
            return anims

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "high"))
        self.play(warm_fill.animate.set_fill(C_HEAT_HOT, opacity=0.2), *room_heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "high", during=room_heat)

        self.play(
            FadeIn(ctrl_panel),
            FadeIn(ctrl_title),
            FadeIn(light_label), Create(light_track), Create(light_fill),
            FadeIn(light_knob), FadeIn(light_pct),
            FadeIn(plug_label), Create(plug_track), Create(plug_fill),
            FadeIn(plug_knob), FadeIn(plug_pct),
            run_time=1.1,
        )

        light_t, plug_t = 0.30, 0.40
        light_target = light_track.point_from_proportion(light_t)
        plug_target = plug_track.point_from_proportion(plug_t)
        light_share = ValueTracker(100.0)
        plug_share = ValueTracker(100.0)
        light_pct_anchor = light_pct.get_left()
        plug_pct_anchor = plug_pct.get_left()
        light_pct_live = always_redraw(lambda: Text(
            f"{light_share.get_value():.0f}%", font_size=LABEL_FONT_SIZE, color=C_YELLOW,
        ).move_to(light_pct_anchor, aligned_edge=LEFT))
        plug_pct_live = always_redraw(lambda: Text(
            f"{plug_share.get_value():.0f}%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT,
        ).move_to(plug_pct_anchor, aligned_edge=LEFT))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dim"))
        self.remove(light_pct, plug_pct)
        self.add(light_pct_live, plug_pct_live)
        self.play(
            light_knob.animate.move_to(light_target),
            plug_knob.animate.move_to(plug_target),
            UpdateFromAlphaFunc(
                light_fill,
                lambda m, a: m.put_start_and_end_on(
                    light_track.get_start(),
                    light_track.point_from_proportion(1.0 - a * (1.0 - light_t)),
                ),
            ),
            UpdateFromAlphaFunc(
                plug_fill,
                lambda m, a: m.put_start_and_end_on(
                    plug_track.get_start(),
                    plug_track.point_from_proportion(1.0 - a * (1.0 - plug_t)),
                ),
            ),
            light_share.animate.set_value(light_t * 100),
            plug_share.animate.set_value(plug_t * 100),
            beam.animate.set_stroke(opacity=0.18),
            FadeOut(fixture), FadeIn(fixture_dim),
            warm_fill.animate.set_fill(C_COOL, opacity=0.08),
            light_on.animate.set_value(light_t), plug_on.animate.set_value(plug_t),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "dim", during=room_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=room_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
