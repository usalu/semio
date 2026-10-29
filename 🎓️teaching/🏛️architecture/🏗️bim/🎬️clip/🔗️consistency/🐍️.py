"""🎬 Identity and Consistency: one GlobalId carries the roof slab of the Villa Savoye through a leak, a design change, a stale analysis and its resolution."""

import importlib
import math
import sys
import textwrap
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_PREFIX = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
_d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
_analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧮️analysis.🐍️")
_core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV = _visuals.INK, _visuals.MUTED, _visuals.ARCH, _visuals.STRUCT, _visuals.SERV
DATA, GOOD, WARN, GLASS = _visuals.DATA, _visuals.GOOD, _visuals.WARN, _visuals.GLASS
S_SMALL, S_LABEL, S_BODY = _d.S_SMALL, _d.S_LABEL, _d.S_BODY
BG, CONCRETE_FILL = _d.BACKGROUND, _visuals.CONCRETE
LINK, K_KN = "LK-ROOF", _analysis.ARROW_KN
K_PAIR = K_KN * 1.3
AXON_AT, AXON_SCALE = (-3.4, 0.3, 0), 0.155
SECTION_AT, SECTION_SCALE = (-4.0, 0.15, 0), 0.27
DETAIL_AT, DETAIL_SCALE, DETAIL_WIDTH = (2.4, -1.95, 0), 4.4, 1.9
STANDARD = "IFC4X3_ADD2 · ISO 16739-1:2024"
AUTHORS = {"Atelier (architecture)": ("Atelier (architecture)", "Atelier (Architektur)"), "Structural engineer": ("Structural engineer", "Tragwerksplaner"),
           "Heating engineer": ("Heating engineer", "Heizungsplaner")}


class Beat6_Consistency(_visuals.ClipScene):
    """📚 Animate identity and consistency on the Villa Savoye roof slab with explicit English and German narration."""

    TITLE = {"en": "Identity and Consistency", "de": "Identität und Konsistenz"}
    NARRATION = [
        ["identity",
         "A GlobalId names this roof slab in every exchange; the tag SL-R and the number #101 only work locally.",
         "Eine GlobalId benennt diese Dachplatte in jedem Austausch; Kürzel SL-R und Nummer #101 gelten nur lokal."],
        ["models",
         "Architecture and structure each keep this slab in their own model and revision, under one GlobalId.",
         "Architektur und Tragwerk führen diese Platte je im eigenen Modell und in eigener Revision, unter einer GlobalId."],
        ["metadata",
         "The slab record holds class, properties, author and revision; none of these is the identity.",
         "Der Datensatz der Platte hält Klasse, Eigenschaften, Urheber und Revision; keines davon ist die Identität."],
        ["leak",
         "Reports say water came through the flat roof each autumn; our teaching scenario starts from this leak.",
         "Berichte sagen, dass jeden Herbst Wasser durchs Flachdach kam; unser Lehrszenario beginnt bei diesem Leck."],
        ["leakpath",
         "The source distinguishes the original reconstructed state from later damage evidence. The illustrated water path is schematic, without an invented membrane layer.",
         "Die Quelle unterscheidet ursprünglichen rekonstruierten Zustand und spätere Schadensbelege. Der gezeigte Wasserweg ist schematisch, ohne erfundene Membranschicht."],
        ["change",
         "In an explicit repair scenario, a design event increases the reference paving reserve from 0.09 to 0.14 m; the slab keeps its GlobalId.",
         "In einem ausdrücklichen Reparaturszenario erhöht ein Entwurfsereignis die Bezugsreserve des Belags von 0,09 auf 0,14 m; die Platte behält ihre GlobalId."],
        ["kept",
         "Thickness and revision change; GlobalId, class and place in the villa stay what they were.",
         "Dicke und Revision ändern sich; GlobalId, Klasse und Platz in der Villa bleiben, wie sie waren."],
        ["load",
         "The added paving increases the area load; layer densities and thicknesses determine the displayed value.",
         "Zusätzlicher Belag erhöht die Flächenlast; Schichtdichten und Dicken bestimmen den angezeigten Wert."],
        ["grid",
         "Each roof support collects the slab nearest to it, excluding openings. The irregular cells have different areas.",
         "Jede Dachstütze trägt den nächstgelegenen Teil der Decke ohne Öffnungen. Die unregelmäßigen Zellen haben verschiedene Flächen."],
        ["derived",
         "The load on each support follows from its share of each roof zone and the corresponding area load.",
         "Die Last jeder Stütze folgt aus ihrem Anteil an jeder Dachzone und deren Flächenlast."],
        ["spread",
         "The paving change affects each support according to its share of the roof zone.",
         "Die Belagsänderung betrifft jede Stütze entsprechend ihrem Anteil an der Dachzone."],
        ["stale",
         "LK-ROOF links the analysis to the slab, but still holds the earlier revision and its earlier result.",
         "LK-ROOF bindet die Analyse an die Platte, hält aber noch die frühere Revision mit deren Ergebnis."],
        ["check",
         "A check compares the linked revision and stored values with the current model and derived results.",
         "Eine Prüfung vergleicht verknüpfte Revision und gespeicherte Werte mit dem aktuellen Modell und abgeleiteten Ergebnissen."],
        ["ids",
         "A requirement check (IDS) passes because the information exists; only the link check finds the old result.",
         "Eine Anforderungsprüfung (IDS) besteht, weil die Angabe existiert; nur die Verknüpfungsprüfung findet das alte Ergebnis."],
        ["report",
         "The finding goes to the structural engineer as a BCF topic that points to the slab by its GlobalId.",
         "Der Befund geht als BCF-Thema an den Tragwerksplaner und verweist über die GlobalId auf die Platte."],
        ["retranslate",
         "Re-translating generates surface members and supports from the current teaching model; a person reviews the assumptions.",
         "Die neue Übersetzung erzeugt Flächenelemente und Stützen aus dem aktuellen Lehrmodell; eine Person prüft die Annahmen."],
        ["resolve",
         "With the link updated, the analysis and model show matching reactions and the check finds no issue.",
         "Mit aktueller Verknüpfung zeigen Analyse und Modell gleiche Auflagerkräfte; die Prüfung findet kein Problem."],
        ["fork",
         "Two people edit revision 1 at once: both save revision 2 under the same GlobalId, with different paving.",
         "Zwei Personen ändern Revision 1 gleichzeitig: beide speichern Revision 2 unter derselben GlobalId, mit anderem Belag."],
        ["decide",
         "The GlobalId cannot merge competing paving edits; a person decides which edit the model keeps.",
         "Die GlobalId kann konkurrierende Belagsänderungen nicht verschmelzen; eine Person entscheidet, welche Änderung bleibt."],
        ["recap",
         "A schema gives data meaning, a GlobalId names the object, a revision dates it, and a link ties the analysis to it.",
         "Ein Schema gibt Daten Bedeutung, die GlobalId benennt das Objekt, die Revision datiert es, die Verknüpfung bindet die Analyse."],
    ]
    CONCEPTS = {
        "identity": ("One name for one object", "Ein Name für ein Objekt"),
        "models": ("Separate models, one identity", "Getrennte Modelle, eine Identität"),
        "metadata": ("Properties are not identity", "Eigenschaften sind keine Identität"),
        "leak": ("The trigger: a leaking roof", "Der Anlass: ein undichtes Dach"),
        "leakpath": ("Where the water gets in", "Wo das Wasser eindringt"),
        "change": ("A design event changes content", "Ein Entwurfsereignis ändert Inhalt"),
        "kept": ("Content changes, identity stays", "Inhalt ändert sich, Identität bleibt"),
        "load": ("Derived value: load per m²", "Abgeleiteter Wert: Last je m²"),
        "grid": ("Derived value: area per piloti", "Abgeleiteter Wert: Fläche je Piloti"),
        "derived": ("R = A · q follows the change", "R = A · q folgt der Änderung"),
        "spread": ("Every piloti follows differently", "Jeder Piloti folgt anders"),
        "stale": ("A link can go stale", "Eine Verknüpfung kann veralten"),
        "check": ("Check: expected against actual", "Prüfung: erwartet gegen tatsächlich"),
        "ids": ("Requirements are not consistency", "Anforderung ist keine Konsistenz"),
        "report": ("The finding becomes an issue", "Der Befund wird zum Thema"),
        "retranslate": ("Regenerate, then review", "Neu erzeugen, dann prüfen"),
        "resolve": ("Link updated, arrows match", "Verknüpfung aktuell, Pfeile gleich"),
        "fork": ("Two edits of one revision", "Zwei Änderungen einer Revision"),
        "decide": ("Same identity, different content", "Gleiche Identität, anderer Inhalt"),
        "recap": ("Schema, GlobalId, revision, link", "Schema, GlobalId, Revision, Verknüpfung"),
    }
    REFS = {"ids": "IDS 1.0 · " + STANDARD.split(" · ")[0], "report": "BCF-XML 3.0 · " + STANDARD.split(" · ")[0]}

    def construct(self):
        """▶️ Present this chapter through the native BIM visual interface."""
        self.present("consistency")

    def present_consistency(self):
        """🎞️ Run the beats of the clip in order."""
        self._prepare()
        for name in ("identity", "models", "metadata", "leak", "leakpath", "change", "kept", "load", "grid", "derived", "spread", "stale", "check", "ids",
                     "report", "retranslate", "resolve", "fork", "decide", "recap"):
            getattr(self, "_" + name)()

    def _prepare(self):
        """🧮 Read every model state the clip shows: base, changed, alternative and resolved."""
        base = _model.load_model()
        self.base = base
        self.geometry = _model.building_geometry(base)
        self.tag = _model.element(next(link["object_guid"] for link in base["links"] if link["id"] == LINK), base)["tag"]
        self.gid = _model.global_id(self.tag)
        self.event = _model.demo_event("roof_paving", base)
        self.new = _model.apply_event(self.event, base)
        self.alt = _model.apply_event(_model.demo_event("roof_paving_alt", base), base)
        self.resolved = _model.resolve_link(self.new, LINK)
        self.issues = _model.consistency_issues(self.new)
        self.link = next(link for link in base["links"] if link["id"] == LINK)
        self.link_new = next(link for link in self.resolved["links"] if link["id"] == LINK)
        self.state_cache = {}
        self.paving_old, self.paving_new, self.paving_alt = (_model.quantity(self.tag, "paving", state) for state in (base, self.new, self.alt))
        self.trib_old, self.trib_new, self.trib_alt = (_model.tributary(state, "roof") for state in (base, self.new, self.alt))
        self.cut_x = next(p["x"] for p in self.geometry["pilotis"] if p["tag"] == _model.role("column_example", base))
        roof_tags = {p["tag"] for p in self.trib_new["pilotis"]}
        self.column = [p["tag"] for p in sorted(self.geometry["pilotis"], key=lambda p: p["y"]) if p["tag"] in roof_tags and abs(p["x"] - self.cut_x) < 1e-6]
        self.reaction_old = {p["tag"]: p["reaction"] for p in self.trib_old["pilotis"]}
        self.reaction_new = {p["tag"]: p["reaction"] for p in self.trib_new["pilotis"]}
        self.show_tags = self.column[::2] or sorted(roof_tags)[:2]
        change = lambda tag: self.reaction_new[tag] - self.reaction_old[tag]
        self.focus, self.contrast = max(self.show_tags, key=change), min(self.show_tags, key=change)
        self.pair_tags = (self.focus,)
        self.pilotis = {p["tag"]: p for p in self.geometry["pilotis"]}
        self.zone_new = {z["id"]: z for z in self.trib_new["zones"]}
        self.zone_old = {z["id"]: z for z in self.trib_old["zones"]}
        self.zone_alt = {z["id"]: z for z in self.trib_alt["zones"]}
        self.q_old, self.q_new, self.q_alt = (zones["roof"]["area_load"] for zones in (self.zone_old, self.zone_new, self.zone_alt))
        self.cell = {p["tag"]: p for p in self.trib_new["pilotis"]}
        self.delta = {tag: self.reaction_new[tag] - self.reaction_old[tag] for tag in self.reaction_new}
        self.k_q = 0.62 / self.q_new
        self.analysis_old = _model.translate_structure(base)
        self.analysis_new = _model.translate_structure(self.resolved)
        self.stage = []
        self.concept_mob = self.ref_mob = self.legend_mob = self.ref_key = None
        for zone in (self.zone_old, self.zone_new, self.zone_alt):
            entry = zone["roof"]
            assert math.isclose(entry["area_load"], entry["load_breakdown"]["total"]["value"], rel_tol=1e-9)

    def revision(self, model):
        """🔢 Revision of the roof slab in one model state."""
        return _model.element(self.tag, model)["metadata"]["revision"]

    def model_revision(self, model_id, model):
        """🔢 Revision of one partial model in one model state."""
        return _model.partial_model(model_id, model)["revision"]

    def rev(self, value):
        """🔖 Localized revision word with its number."""
        return f"{self.tr('rev', 'Rev.')} {value}"

    def author(self, value):
        """👷 Localized author of a model record."""
        return AUTHORS[value][0 if self.language() == "en" else 1]

    def paving_state(self, paving):
        """♻️ Model state with the given paving thickness, cached by value."""
        key = round(paving, 4)
        if key not in self.state_cache:
            self.state_cache[key] = _model.apply_event({**self.event, "value": key}, self.base)
        return self.state_cache[key]

    def layers_of(self, model):
        """🥞 Roof build-up layers of one model state, bottom to top."""
        return _model.element(self.tag, model)["material"]["layers"]

    def name(self, record):
        """🗣️ Localized name of a model record."""
        return record["name"][self.language()]

    def star(self, basis):
        """✱ Asterisk mark of a teaching-model assumption."""
        return " *" if basis == "assumed" else ""

    def weights(self, paving):
        """⚖️ Weight per m² of every roof layer of the roof zone at one paving depth, bottom to top, then the imposed load: (id, name, thickness, density, weight)."""
        state = self.paving_state(paving)
        breakdown = next(zone for zone in _model.tributary(state, "roof")["zones"] if zone["id"] == "roof")["load_breakdown"]
        self.layer_basis = {key: (value["basis"], value["basis"]) for key, value in breakdown.items()}
        return [(key, value[self.language()], value.get("thickness"), value.get("density"), value["value"])
                for key, value in breakdown.items() if key != "total"]

    def q_of(self, paving):
        """⚖️ Roof load at one paving depth: the sum of the layer weights and the imposed load."""
        return sum(row[4] for row in self.weights(paving))

    def wrap(self, text, columns):
        """↩️ Break a sentence into lines of at most ``columns`` characters."""
        return textwrap.wrap(text, columns)

    def paragraph(self, lines, point, *, size=S_SMALL, color=INK, pitch=0.3, anchor="l"):
        """📝 Left-aligned lines of text, one per row."""
        return _engine.VGroup(*[self.text(line, (point[0], point[1] - pitch * index, 0), size=size, color=color, anchor=anchor) for index, line in enumerate(lines)])

    def panel(self, width, height, point, color=MUTED):
        """🔲 Dark panel frame."""
        return _panels.frame_box(width, height, color).move_to(_d.P(point))

    def card(self, rows, point, width, title=None, color=GOOD, value_width=3.0, mono=()):
        """🗂️ Key-value card; rows are ``(name, value)`` or ``(name, value, basis)`` and assumed values carry an asterisk."""
        pitch = 0.36
        height = len(rows) * pitch + 0.22 + (0.46 if title else 0.0)
        frame = self.panel(width, height, point, color)
        group = _engine.VGroup(frame)
        group.values = []
        y = frame.get_top()[1] - 0.12
        left, right = frame.get_left()[0] + 0.16, frame.get_right()[0] - 0.3
        if title:
            group.add(self.text(title, (left, y - 0.2, 0), size=S_LABEL, color=color, anchor="l", width=width - 0.3),
                      _d.segments([((frame.get_left()[0], y - 0.44, 0), (frame.get_right()[0], y - 0.44, 0))], color, _d.W_THIN))
            y -= 0.46
        for index, row in enumerate(rows):
            name, value, basis = (tuple(row) + (None,))[:3]
            at = y - pitch * (index + 0.5)
            shown = self.mono(value, (right, at, 0), size=S_SMALL, color=DATA, anchor="r") if name in mono else self.text(value, (right, at, 0), size=S_LABEL, color=DATA, anchor="r", width=value_width)
            group.add(self.text(name, (left, at, 0), size=S_LABEL, color=INK, anchor="l", width=width - value_width - 0.5), shown)
            if basis == "assumed":
                group.add(self.text("*", (right + 0.1, at, 0), size=S_LABEL, color=WARN, anchor="l"))
            group.values.append(shown)
        return group

    def formula(self, parts, point, *, color=DATA, size=S_LABEL, pad=0.2, anchor="c"):
        """🧮 One framed equation; ``parts`` is a list of ``(text, colour)`` terms laid out in a row."""
        row = _engine.VGroup(*[self.text(text, (0, 0, 0), size=size, color=tone) for text, tone in parts])
        row.arrange(_engine.RIGHT, buff=0.12)
        frame = self.panel(row.width + 2 * pad, row.height + 2 * pad, (0, 0, 0), color)
        group = _engine.VGroup(frame, row.move_to(frame))
        group.row, group.frame = row, frame
        return _d.place(group, point, anchor)

    def person(self, point, color, size=0.42):
        """🧑 A person glyph: head and shoulders."""
        x, y, _ = point
        head = _engine.Circle(radius=size * 0.26, stroke_width=0, fill_color=color, fill_opacity=1).move_to((x, y + size * 0.52, 0))
        body = _d.polygon(_d.circle_points((x, y - size * 0.1), size * 0.5, 16, 0, 180), color, 1.0, color, 0.5)
        return _engine.VGroup(head, body)

    def stack(self, lines, point, color, *, size=S_SMALL, pitch=0.3, pad=0.16):
        """🥞 A framed block of centred lines, each with its own colour."""
        rows = _engine.VGroup(*[self.text(text, (point[0], point[1] + pitch * (len(lines) - 1) / 2 - pitch * index, 0), size=size, color=tone) for index, (text, tone) in enumerate(lines)])
        frame = self.panel(rows.width + 2 * pad, pitch * len(lines) + pad, point, color)
        return _engine.VGroup(frame, rows)

    def section_mark(self, plan):
        """✂️ The cut of the later sections on the roof plan: a dash-dot line along axis 3 with an S chip and a view arrow at both ends, all inside the plan."""
        box = self.geometry["box"]
        lo, hi = plan.at(self.cut_x, 0.9, 0.0), plan.at(self.cut_x, box["width"] - 0.9, 0.0)
        group = _engine.VGroup(_d.polyline([lo, hi], INK, _d.W_THIN, dash=0.1))
        for end in (lo, hi):
            group.add(_d.polygon([end + (-0.34, 0.0, 0.0), end + (-0.2, 0.07, 0.0), end + (-0.2, -0.07, 0.0)], INK, 1.0, INK, 0.5), self.backed(self.text("S", end, size=S_SMALL, color=INK), INK))
        group.add(self.label("S = section", "S = Schnitt", np.array(plan.at(0.0, 0.0, 0.0)) + np.array([0.5, -0.95, 0.0]), size=S_SMALL, color=MUTED, anchor="l"))
        return group

    def backed(self, label, color=None):
        """🔲 A label on an opaque background chip so the drawing below never runs through it."""
        chip = _engine.RoundedRectangle(corner_radius=0.04, width=label.width + 0.12, height=label.height + 0.1, stroke_color=color or BG, stroke_width=_d.W_THIN if color else 0,
                                        fill_color=BG, fill_opacity=1).move_to(label)
        return _engine.VGroup(chip, label)

    def underline(self, token, color):
        """➖ A thick rule under a token."""
        y = token.get_bottom()[1] - 0.06
        return _d.segments([((token.get_left()[0], y, 0), (token.get_right()[0], y, 0))], color, _d.W_CUT)

    def step_number(self):
        """#️⃣ Local STEP instance number of the roof slab in its exchange fragment."""
        return next(row for row in _model.step_excerpt(self.tag)["lines"] if "IFCSLAB" in row).split("=")[0]

    def checkmark(self, point, color=GOOD):
        """✔️ A drawn check mark."""
        x, y, _ = point
        return _d.polyline([(x - 0.07, y, 0), (x - 0.02, y - 0.07, 0), (x + 0.09, y + 0.09, 0)], color, _d.W_CUT)

    def cross(self, point, color=WARN):
        """✖️ A drawn cross."""
        x, y, _ = point
        return _d.segments([((x - 0.08, y - 0.08, 0), (x + 0.08, y + 0.08, 0)), ((x - 0.08, y + 0.08, 0), (x + 0.08, y - 0.08, 0))], color, _d.W_CUT)

    def unequal(self, point, color=WARN):
        """≠ A drawn not-equal sign."""
        x, y, _ = point
        return _d.segments([((x - 0.3, y + 0.1, 0), (x + 0.3, y + 0.1, 0)), ((x - 0.3, y - 0.1, 0), (x + 0.3, y - 0.1, 0)), ((x - 0.12, y - 0.3, 0), (x + 0.12, y + 0.3, 0))], color, _d.W_CUT)

    def arrow(self, tip, length, color, *, up=False, width=4.5):
        """⬇️ A vertical load arrow of the given screen length ending at its tip."""
        length = max(length, 0.08)
        tail = np.array(tip, float) + np.array([0.0, -length if up else length, 0.0])
        return _engine.Arrow(tail, tip, buff=0, color=color, stroke_width=width, tip_length=min(0.18, length * 0.45),
                             max_tip_length_to_length_ratio=0.5, max_stroke_width_to_length_ratio=12)

    def foot(self, tag, point, scale, model):
        """📍 Screen position of a piloti foot in the villa axonometric."""
        p = self.pilotis[tag]
        return self.axon_point(p["x"], p["y"], 0.0, point, scale, model=model)

    def roof_spans(self):
        """📏 Intervals along the section cut where the roof lies, in metres of y."""
        return _core.scan(self.zone_new["roof"]["pieces"], self.cut_x, axis=0)

    def revision_badges(self, point, *, architecture, structure, analysis, analysis_color=STRUCT):
        """🔖 The three model badges of a row with the given revisions."""
        items = [(self.tr("architecture", "Architektur"), architecture, ARCH), (self.tr("structure", "Tragwerk"), structure, STRUCT),
                 (self.tr("analysis", "Analyse"), analysis, analysis_color)]
        row = _engine.VGroup(*[self.badge(f"{name} {self.rev(revision)}", (0, 0, 0), color=color, size=S_SMALL) for name, revision, color in items])
        return row.arrange(_engine.RIGHT, buff=0.14).move_to(point)

    def chrome(self, key, legend):
        """🧭 The concept chip next to the title, the standard reference at the top right and the asterisk legend."""
        out, new = [], []
        chip = _visuals._art.chip(self.tr(*self.CONCEPTS[key]), SERV)
        chip.move_to((-6.81 + chip.width / 2, 3.58, 0))
        if self.concept_mob is not None:
            out.append(self.concept_mob)
        self.concept_mob = chip
        new.append(chip)
        ref = self.REFS.get(key, STANDARD)
        if ref != self.ref_key:
            text = self.text(ref, (6.85, 3.58, 0), size=S_LABEL, color=SERV, anchor="r")
            if self.ref_mob is not None:
                out.append(self.ref_mob)
            self.ref_mob, self.ref_key = text, ref
            new.append(text)
        if legend and self.legend_mob is None:
            self.legend_mob = self.label("* teaching-model assumption, not a source value", "* Annahme im Lehrmodell, kein belegter Wert", (6.85, 3.2, 0), size=S_SMALL, color=MUTED, anchor="r")
            new.append(self.legend_mob)
        return out, new

    def show(self, key, new=(), keep=(), extra=(), track=(), seconds=3.0, legend=False):
        """🎞️ Complete the entrance before extra animations change live drawing topology."""
        keep_ids = {id(item) for item in keep}
        gone = [item for item in self.stage if id(item) not in keep_ids]
        out, fresh = self.chrome(key, legend)
        animations = []
        if gone or out:
            animations.append(_engine.FadeOut(*gone, *out))
        animations.append(_engine.LaggedStart(*[_engine.FadeIn(item) for item in (*fresh, *new)], lag_ratio=0.12))
        entrance = _engine.AnimationGroup(*animations)
        motion = _engine.Succession(entrance, _engine.AnimationGroup(*extra)) if extra else entrance
        self.stage = [*keep, *new, *track]
        self.step(key, motion, seconds=seconds)

    def section_view(self, model):
        """✂️ Section through piloti axis 3 with the roof slab lit."""
        return self.villa_section(SECTION_AT, SECTION_SCALE, model=model, highlight=(self.tag,), levels=False, level_lines=False)

    def storey_marks(self, section):
        """🔻 Storey names at the right of the section, without elevation numbers."""
        group = _engine.VGroup()
        for storey, en, de in (("ground", "GF", "EG"), ("first", "1F", "OG"), ("roof", "roof", "Dach")):
            point = section.at(self.cut_x, self.geometry["box"]["width"] + 1.9, self.geometry["storeys"][storey])
            group.add(self.level_mark(point, self.tr(en, de), color=MUTED, length=0.3))
        return group

    def build_up(self, model, point=DETAIL_AT, scale=DETAIL_SCALE, width=DETAIL_WIDTH, *, labels=True, dims=True, lit=None, grown_from=None, reach=6.9):
        """🥞 Magnified roof build-up: coloured layers of the model's layer set, swatch labels and the total thickness dimension."""
        layers = self.layers_of(model)
        x0, y = point[0] - width / 2, point[1]
        group = _engine.VGroup()
        group.spans, group.x0, group.span, group.base = {}, x0, width, point[1]
        wants = []
        for layer in layers:
            height = layer["thickness"]["value"] * scale
            ring = [(x0, y), (x0 + width, y), (x0 + width, y + height), (x0, y + height)]
            group.add(_d.polygon(ring, _core.layer_color(layer["id"]), 1.0, INK, 1.0))
            if layer["id"] == "structural":
                group.add(self.hatch(ring, MUTED, 0.1, 45, opacity=0.6))
            group.spans[layer["id"]] = (y, y + height)
            wants.append((layer, y + height / 2))
            y += height
        group.top = y
        if labels:
            last = -9.0
            for layer, mid in wants:
                at = max(mid, last + 0.28)
                last = at
                text = f"{self.name(layer)} · {self.quantity(layer['thickness']['value'], 'm', 2)}{self.star(layer['thickness'].get('basis'))}"
                group.add(_d.segments([((x0 + width, mid, 0), (x0 + width + 0.3, at, 0))], MUTED, _d.W_HAIR),
                          self.text(text, (x0 + width + 0.36, at, 0), size=S_SMALL, color=DATA if layer["id"] == lit else INK, anchor="l", width=reach - (x0 + width + 0.36)))
        if dims:
            total = sum(layer["thickness"]["value"] for layer in layers)
            group.add(self.dimension((x0, point[1], 0), (x0, point[1] + total * scale, 0), self.quantity(total, "m", 2) + " *", offset=0.28, color=DATA))
        if lit in group.spans:
            lo, hi = group.spans[lit]
            group.add(_engine.Rectangle(width=width, height=hi - lo, stroke_color=DATA, stroke_width=3.0, fill_opacity=0).move_to((point[0], (lo + hi) / 2, 0)))
            if grown_from is not None and hi - lo > grown_from * scale + 0.3:
                edge = lo + grown_from * scale
                group.add(_d.polyline([(x0, edge, 0), (x0 + width, edge, 0)], DATA, _d.W_EDGE, dash=0.1),
                          self.text("+ " + self.quantity(layers[[l["id"] for l in layers].index(lit)]["thickness"]["value"] - grown_from, "m", 2), (point[0], (edge + hi) / 2, 0), size=S_LABEL, color=DATA))
        return group

    def drops(self, clock, x_left, x_right, top):
        """💧 Rain falling onto the paving surface, as a function of the clock."""
        return _engine.VGroup(*[_engine.Dot((x_left + (x_right - x_left) * ((index * 0.61) % 1.0), top + 0.5 - 0.5 * ((clock + index * 0.37) % 1.0), 0), radius=0.045, color=GLASS)
                                for index in range(6)])

    def leak_path(self, detail):
        """💧 The water path through the build-up: points from the paving surface through a gap in the paving to the ceiling stain."""
        spans = detail.spans
        gap_x, entry_x = detail.x0 + 0.34 * detail.span, detail.x0 + 0.78 * detail.span
        wp0, wp1 = spans["paving"]
        c0 = spans["structural"][0]
        return [np.array(point, float) for point in ((entry_x, detail.top + 0.4, 0), (entry_x, wp1, 0), (gap_x, wp1, 0), (gap_x, wp0, 0), (gap_x - 0.12, c0, 0), (gap_x - 0.12, c0 - 0.26, 0))]

    def leak_marks(self, detail, flow):
        """🚰 Gap, water path, numbered stations, moving drop and ceiling stain on the magnified build-up."""
        path = self.leak_path(detail)
        wp0, wp1 = detail.spans["paving"]
        gap_x = path[2][0]
        group = _engine.VGroup(_engine.Rectangle(width=0.08, height=wp1 - wp0 + 0.03, stroke_width=0, fill_color=_d.BACKGROUND, fill_opacity=1).move_to((gap_x, (wp0 + wp1) / 2, 0)),
                               _engine.Circle(radius=0.17, stroke_color=SERV, stroke_width=_d.W_EDGE).move_to((gap_x, (wp0 + wp1) / 2, 0)),
                               _d.polyline(path, GLASS, _d.W_EDGE, dash=0.09))
        stain = _engine.Ellipse(width=0.95, height=0.2, stroke_color=GLASS, stroke_width=_d.W_THIN, fill_color=GLASS, fill_opacity=0.45).move_to((path[-1][0], path[-1][1] - 0.02, 0))
        group.add(stain)
        x_number = detail.x0 - 0.5
        for number, (index, reach) in enumerate(((0, path[0][0]), (3, gap_x - 0.19), (5, path[5][0] - 0.5)), 1):
            y = path[index][1] if index != 3 else (wp0 + wp1) / 2
            group.add(_d.polyline([(x_number + 0.15, y, 0), (reach, y, 0)], GLASS, _d.W_HAIR, dash=0.06),
                      self.badge(str(number), (x_number, y, 0), color=GLASS, size=S_SMALL))
        lengths = np.cumsum([0.0] + [float(np.linalg.norm(b - a)) for a, b in zip(path, path[1:])])

        def position(fraction):
            target = fraction * lengths[-1]
            index = max(0, min(int(np.searchsorted(lengths, target, side="right")) - 1, len(path) - 2))
            span = lengths[index + 1] - lengths[index]
            return path[index] + (path[index + 1] - path[index]) * ((target - lengths[index]) / span if span else 0.0)

        drop = self.live(lambda: _engine.VGroup(_engine.Dot(position(flow.get_value() % 1.0), radius=0.07, color=GLASS)))
        return _engine.VGroup(group, drop)

    def history_card(self):
        """📜 The historical report that starts the scenario."""
        years = self.base["building"]["years"]
        card = self.panel(6.4, 1.45, (3.7, 2.35))
        facts = _engine.VGroup(
            self.label(f"{years['design_start']}–{years['completed']} · Villa Savoye, Poissy", None, (0.7, 2.85, 0), size=S_LABEL, color=INK, anchor="l"),
            self.label("reports: water through the roof each autumn", "Berichte: jeden Herbst Wasser durchs Dach", (0.7, 2.43, 0), size=S_LABEL, color=GLASS, anchor="l", width=6.0),
            self.label("historical report · a teaching scenario follows", "historische Angabe · danach ein Lehrszenario", (0.7, 2.0, 0), size=S_SMALL, color=MUTED, anchor="l", width=6.0))
        return _engine.VGroup(card, facts)

    def zoom_lines(self, section, frame):
        """🔍 Ring on the section and two lines to the magnified frame."""
        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])
        centre = section.at(self.cut_x, (span[0] + span[1]) / 2, self.geometry["storeys"]["roof"])
        ring = _engine.Circle(radius=0.18, stroke_color=DATA, stroke_width=_d.W_EDGE).move_to(centre)
        return _engine.VGroup(ring, _d.segments([(centre + (0.16, 0.08, 0), (frame.get_left()[0], frame.get_top()[1], 0)), (centre + (0.16, -0.08, 0), (frame.get_left()[0], frame.get_bottom()[1], 0))],
                                                DATA, _d.W_THIN))

    def detail_frame(self, detail):
        """🖼️ Hairline frame of the magnified region, sized for the thickest build-up of the story."""
        final = sum(layer["thickness"]["value"] for layer in self.layers_of(self.new))
        left, right = detail.x0 - 1.45, detail.x0 + detail.span + 0.1
        y0, y1 = detail.base - 0.55, detail.base + final * DETAIL_SCALE + 0.5
        return _engine.Rectangle(width=right - left, height=y1 - y0, stroke_color=MUTED, stroke_width=_d.W_HAIR, fill_opacity=0).move_to(((left + right) / 2, (y0 + y1) / 2, 0))

    def story_chip(self, section):
        """🆔 The GlobalId chip above the section with a leader to the roof slab: the same identity through the whole story."""
        chip = self.guid_chip(self.tag, (-4.0, 2.5, 0), label=False)
        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])
        pointer = self.leader(chip, section.at(self.cut_x, max(self.roof_spans())[0] + 1.0, self.geometry["storeys"]["roof"]), color=DATA)
        return chip, pointer

    def _identity(self):
        """🆔 Beat 1: the GlobalId names the roof slab; the tag and the STEP number are local."""
        axon = self.villa_axon((-3.4, 0.3, 0), 0.155, model=self.base, highlight=(self.tag,))
        windows = self.label("teaching choice: ribbon windows on all faces", "Lehrannahme: Bandfenster an allen Seiten", (-6.9, 2.95, 0), size=S_SMALL, color=MUTED, anchor="l")
        tag_badge = self.badge(self.tag, (1.0, 2.8, 0), color=MUTED, filled=False, anchor="l")
        tag_note = self.label("short tag · for people", "Kürzel · für Menschen", (tag_badge.get_right()[0] + 0.15, 2.8, 0), size=S_LABEL, color=MUTED, anchor="l")
        kind = self.label("IfcRoot.GlobalId · IfcGloballyUniqueId", None, (1.0, 2.3, 0), size=S_LABEL, color=MUTED, anchor="l")
        chip = self.guid_chip(self.tag, (3.9, 1.5, 0), label=False)
        ident = chip.id
        chip.remove(ident)
        pointer = self.leader((chip.get_left()[0] - 0.1, chip.get_center()[1], 0), np.array(axon.at(self.geometry["box"]["length"], 0.0, self.geometry["storeys"]["roof"])), color=DATA)
        brace = _engine.Brace(ident, _engine.DOWN, color=DATA, buff=0.1)
        count = _engine.ValueTracker(0)
        number = self.live_label(lambda: f"{int(round(count.get_value()))} " + self.tr("characters", "Zeichen"), (3.9, 0.7, 0), size=S_BODY, color=DATA)
        heading = self.label("exchange file · STEP excerpt", "Austauschdatei · STEP-Auszug", (1.0, 0.1, 0), size=S_SMALL, color=MUTED, anchor="l")
        panel = self.step_panel(_model.step_excerpt(self.tag), (3.9, -0.7, 0), width=5.7, only=["guid"])
        local = self.underline(panel.token("IFCSLAB", "number"), WARN)
        global_ring = self.underline(panel.token("IFCSLAB", "guid"), DATA)
        notes = _engine.VGroup(self.label(f"{self.step_number()} · " + self.tr("number local to this file", "Nummer nur in dieser Datei"), None, (1.0, -1.7, 0), size=S_LABEL, color=WARN, anchor="l"),
                               self.label("GlobalId · " + self.tr("the same in every file", "in jeder Datei gleich"), None, (1.0, -2.15, 0), size=S_LABEL, color=DATA, anchor="l"))
        self.add(count)
        self.show("identity", new=[axon, windows, tag_badge, tag_note, kind, chip], extra=[
            _engine.Succession(_engine.Create(pointer),
                               _engine.AnimationGroup(_engine.AddTextLetterByLetter(ident, time_per_char=0.1), count.animate(rate_func=_engine.linear).set_value(len(self.gid)),
                                                      _engine.FadeIn(brace, number)),
                               _engine.FadeIn(heading, panel), _engine.AnimationGroup(_engine.Create(local), _engine.Create(global_ring), _engine.FadeIn(notes)))],
                  track=[pointer, brace, number, heading, panel, local, global_ring, notes], seconds=6.5)
        self.remove(ident)
        chip.add(ident)

    def _models(self):
        """🧱 Beat 2: the slab sits in the architecture and the structure model under one GlobalId."""
        axon = self.villa_axon((-3.9, -0.2, 0), 0.125, model=self.base, highlight=(self.tag,), explode=0.5)
        windows = self.label("teaching choice: ribbon windows on all faces", "Lehrannahme: Bandfenster an allen Seiten", (0.7, -2.45, 0), size=S_SMALL, color=MUTED, anchor="l")
        cards, leaders = _engine.VGroup(), _engine.VGroup()
        guid = _model.element(self.tag, self.base)["guid"]
        for index, (key, color) in enumerate((("architecture", ARCH), ("structure", STRUCT), ("services", SERV))):
            info = _model.partial_model(key, self.base)
            has = guid in info["element_guids"]
            count = sum(info["counts_by_class"].values()) if "counts_by_class" in info else len(info["element_guids"])
            card = self.card([(self.tr("author", "Urheber"), self.author(info["author"])),
                              (self.tr("objects · contains SL-R", "Objekte · enthält SL-R"), f"{count} · " + (self.tr("yes", "ja") if has else self.tr("no", "nein")))],
                             (3.7, 2.2 - 1.55 * index, 0), 6.0, f"{info['name'][self.language()]} · {self.rev(info['revision'])}", color if has else MUTED, value_width=2.8)
            cards.add(card)
            if has:
                leaders.add(self.leader((card[0].get_left()[0], card[0].get_center()[1], 0), axon.at(self.tag, layer=key), color=color))
        chip = self.guid_chip(self.tag, (-3.9, 2.55, 0), label=False)
        self.show("models", new=[axon, windows, chip, cards, leaders], seconds=4.0)

    def _metadata(self):
        """🏷️ Beat 3: class, properties, author and revision of the slab record."""
        element = _model.element(self.tag, self.base)
        storey = next(s for s in self.base["storeys"] if s["id"] == element["storey"])
        psets = element["psets"]["Pset_SlabCommon"]
        yes = lambda flag: self.tr("TRUE", "WAHR") if flag else self.tr("FALSE", "FALSCH")
        rows = [("IfcSlab · PredefinedType", element["predefined_type"]), (self.tr("Storey", "Geschoss"), self.name(storey)),
                ("Pset_SlabCommon.LoadBearing", yes(psets["LoadBearing"]["value"]), psets["LoadBearing"]["basis"]),
                ("Pset_SlabCommon.IsExternal", yes(psets["IsExternal"]["value"]), psets["IsExternal"]["basis"]),
                (self.tr("Author", "Urheber"), self.author(element["metadata"]["author"])), (self.tr("Revision (model field)", "Revision (Modellfeld)"), str(self.revision(self.base)))]
        card = self.card(rows, (3.9, 0.4, 0), 6.2, self.tr("Roof slab · record (can change)", "Dachplatte · Datensatz (kann sich ändern)"), GOOD, value_width=2.4)
        axon = self.villa_axon(AXON_AT, AXON_SCALE, model=self.base, highlight=(self.tag,))
        roof_z = self.geometry["storeys"]["roof"]
        box = self.geometry["box"]
        outline = _d.polyline([axon.at(x, y, roof_z) for x, y in ((0.0, 0.0), (box["length"], 0.0), (box["length"], box["width"]), (0.0, box["width"]))], GLASS, _d.W_CUT, closed=True)
        edge = np.array(axon.at(box["length"] / 2, 0.0, roof_z))
        over = np.array(axon.at(self.pilotis[self.focus]["x"], self.pilotis[self.focus]["y"], roof_z))
        left = card[0].get_left()[0]
        pointers = _engine.VGroup(outline, self.leader((left, card.values[2].get_center()[1], 0), over, color=GOOD), self.leader((left, card.values[3].get_center()[1], 0), edge, color=GLASS))
        chip = self.guid_chip(self.tag, (3.4, 2.6, 0), label=False)
        who = self.label("identity", "Identität", (chip.get_right()[0] + 0.2, 2.6, 0), size=S_LABEL, color=DATA, anchor="l")
        self.show("metadata", new=[axon, card, pointers, chip, who], legend=True, seconds=3.4)

    def _leak(self):
        """💧 Beat 4: the reported leak, shown in section and in the magnified build-up."""
        section = self.section_view(self.base)
        marks = self.storey_marks(section)
        chip, pointer = self.story_chip(section)
        history = self.history_card()
        detail = self.build_up(self.base, dims=False)
        frame = self.detail_frame(detail)
        title = self.label("Reference: 17 cm structure · 9 cm reserve", "Referenz: 17 cm Tragwerk · 9 cm Reserve", (detail.x0 + detail.span + 0.36, detail.top + 0.45, 0), size=S_SMALL, color=MUTED, anchor="l", width=3.0)
        zoom = self.zoom_lines(section, frame)
        clock = _engine.ValueTracker(0)
        clock.add_updater(lambda tracker, dt: tracker.increment_value(dt * 0.8))
        self.add(clock)
        rain = self.live(lambda: self.drops(clock.get_value(), detail.x0, detail.x0 + detail.span, detail.top))
        self.show("leak", new=[section, marks, chip, pointer, history, detail, frame, title, zoom, rain], seconds=3.4)
        self.section, self.storeys, self.chip, self.pointer = section, marks, chip, pointer
        self.detail, self.frame, self.zoom, self.clock, self.rain, self.detail_title = detail, frame, zoom, clock, rain, title

    def _leakpath(self):
        """🚰 Beat 5: the water passes paving and structural reserve, finds a gap in the paving and stains the ceiling below."""
        flow = _engine.ValueTracker(0)
        flow.add_updater(lambda tracker, dt: tracker.increment_value(dt * 0.3))
        self.add(flow)
        marks = self.leak_marks(self.detail, flow)
        legend = self.panel(6.4, 1.45, (3.7, 2.35), GLASS)
        rows = _engine.VGroup(*[self.label(*row, (0.7, 2.7 - 0.4 * index, 0), size=S_SMALL, color=color, anchor="l", width=5.9) for index, (row, color) in enumerate((
            (("1 · rain falls on the paving and seeps down", "1 · Regen fällt auf den Belag und sickert ein"), GLASS),
            (("2 · schematic failure path below the finish", "2 · schematischer Schadensweg unter dem Belag"), SERV),
            (("3 · seeps through the slab and stains the ceiling", "3 · sickert durch die Decke und fleckt sie"), GLASS)))])
        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])
        under = self.section.at(self.cut_x, (span[0] + span[1]) / 2 + 1.0, self.geometry["storeys"]["roof"] - 0.45)
        stain = _engine.Ellipse(width=0.6, height=0.1, stroke_color=GLASS, stroke_width=_d.W_THIN, fill_color=GLASS, fill_opacity=0.5).move_to(under)
        self.show("leakpath", new=[marks, legend, rows, stain], keep=[self.section, self.storeys, self.chip, self.pointer, self.zoom, self.detail, self.frame, self.rain, self.detail_title], seconds=4.4)
        self.flow, self.marks, self.stain = flow, marks, stain

    def event_card(self):
        """⚡ The design event as a card: type, object GlobalId and the new thickness."""
        group = _engine.VGroup(self.panel(6.4, 1.45, (3.7, 2.35), GOOD),
                               self.label("design event · teaching scenario", "Entwurfsereignis · Lehrszenario", (0.7, 2.85, 0), size=S_LABEL, color=GOOD, anchor="l"),
                               self.text(self.event["type"], (0.7, 2.5, 0), size=S_LABEL, color=DATA, anchor="l"),
                               self.label("= layer thickness changed", "= Schichtdicke geändert", (6.7, 2.5, 0), size=S_SMALL, color=MUTED, anchor="r"),
                               self.label("object", "Objekt", (0.7, 2.16, 0), size=S_SMALL, color=MUTED, anchor="l"),
                               self.mono(self.gid, (1.45, 2.16, 0), size=S_SMALL, color=DATA, anchor="l"),
                               self.label("paving layer · new thickness", "Belag · neue Dicke", (0.7, 1.84, 0), size=S_SMALL, color=MUTED, anchor="l"),
                               self.text(self.quantity(self.paving_new, "m", 2) + " *", (6.7, 1.84, 0), size=S_LABEL, color=DATA, anchor="r"))
        return group

    def _change(self):
        """⚡ Beat 6: after the repair the design event thickens the paving; the build-up grows live."""
        tracker = _engine.ValueTracker(self.paving_old)
        self.clock.clear_updaters()
        self.flow.clear_updaters()
        event = self.event_card()
        live = self.live(lambda: self.build_up(self.paving_state(tracker.get_value()), lit="paving", grown_from=self.paving_old), key=lambda: round(tracker.get_value(), 3))
        new_section = self.section_view(self.new)
        repaired = self.paragraph([self.tr("gap repaired", "Lücke repariert"), self.tr("new paving", "neuer Belag")], (self.detail.x0 + self.detail.span + 0.36, self.detail.base - 0.1, 0), color=GOOD, pitch=0.26)
        seal = _engine.Circle(radius=0.17, stroke_color=GOOD, stroke_width=_d.W_EDGE).move_to(self.marks[0][1].get_center())
        old_section = self.section
        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])
        base_z = self.geometry["storeys"]["roof"] + sum(layer["thickness"]["value"] for layer in self.layers_of(self.base) if layer["id"] != "structural")
        tip = new_section.at(self.cut_x, span[0] + 1.0, base_z)
        growth = _engine.VGroup(self.text("+" + self.quantity(self.paving_new - self.paving_old, "m", 2) + " *", tip + (0.0, 0.55, 0), size=S_LABEL, color=DATA), self.arrow(tip + (0.0, 0.02, 0), 0.2, DATA, width=3.0))
        self.show("change", new=[event, live, repaired, growth], keep=[old_section, self.storeys, self.chip, self.pointer, self.zoom, self.frame, self.detail_title],
                  extra=[_engine.Succession(_engine.Create(seal), tracker.animate(rate_func=_engine.smooth).set_value(self.paving_new),
                                            _engine.AnimationGroup(_engine.FadeTransform(old_section, new_section), _engine.Indicate(live, color=DATA)))],
                  track=[seal], seconds=5.0)
        self.stage = [new_section if item is old_section else item for item in self.stage]
        self.section = new_section

    def _kept(self):
        """♾️ Beat 7: what changed and what stayed the same in the slab record."""
        element_new = _model.element(self.tag, self.new)
        stays = self.card([("GlobalId", self.gid), ("IfcSlab · PredefinedType", element_new["predefined_type"]), (self.tr("tag", "Kürzel"), self.tag),
                           (self.tr("Storey", "Geschoss"), self.name(next(s for s in self.base["storeys"] if s["id"] == element_new["storey"])))],
                          (3.7, 1.5, 0), 6.4, self.tr("stays the same", "bleibt gleich"), GOOD, value_width=3.1, mono=("GlobalId",))
        total = lambda state: sum(layer["thickness"]["value"] for layer in self.layers_of(state))
        arrow = " → "
        changes = self.card([(self.tr("paving layer", "Belag"), self.quantity(self.paving_old, "m", 2) + arrow + self.quantity(self.paving_new, "m", 2), "assumed"),
                             (self.tr("build-up total", "Aufbau gesamt"), self.quantity(total(self.base), "m", 2) + arrow + self.quantity(total(self.new), "m", 2), "assumed"),
                             (self.tr("slab revision", "Revision der Platte"), f"{self.revision(self.base)}{arrow}{self.revision(self.new)}"),
                             (self.tr("model revisions", "Modellrevisionen"), f"{self.model_revision('architecture', self.base)}{arrow}{self.model_revision('architecture', self.new)}")],
                            (3.7, -1.15, 0), 6.4, self.tr("changes", "ändert sich"), WARN, value_width=3.1)
        link = self.leader((self.chip.get_right()[0], self.chip.get_center()[1], 0), (stays[0].get_left()[0], stays[0].get_center()[1] + 0.5, 0), color=GOOD)
        roof_z = self.geometry["storeys"]["roof"]
        target = self.section.at(self.cut_x, max(self.roof_spans())[0] + 1.0, roof_z)
        rows_left = stays[0].get_left()[0]
        ties = _engine.VGroup(*[self.leader((rows_left, value.get_center()[1], 0), target, color=GOOD, dot=index == 0) for index, value in enumerate(stays.values)],
                              self.leader((changes[0].get_left()[0], changes.values[0].get_center()[1], 0), self.section.at(self.cut_x, max(self.roof_spans())[1] - 1.0, roof_z), color=WARN))
        paving = _engine.VGroup(*[_d.polyline([self.section.at(self.cut_x, a, roof_z + 0.06), self.section.at(self.cut_x, b, roof_z + 0.06)], WARN, _d.W_CUT) for a, b in self.roof_spans()])
        self.show("kept", new=[stays, changes, link, ties, paving], keep=[self.section, self.chip, self.pointer], seconds=4.0)

    def load_table(self, paving):
        """⚖️ Weight table of the roof at one paving depth: swatch, layer, thickness × density = weight, then the sum."""
        group = _engine.VGroup()
        columns = (-0.55, 0.85, 2.6)
        heads = (self.tr("layer", "Schicht"), self.tr("thickness", "Dicke"), self.tr("density", "Dichte"), self.tr("weight", "Gewicht"))
        for x, head, anchor in zip((-4.05, *columns), heads, ("l", "r", "r", "r")):
            group.add(self.text(head, (x, 2.55, 0), size=S_SMALL, color=MUTED, anchor=anchor))
        rows = list(reversed(self.weights(paving)))
        for index, (layer, name, thickness, density, weight) in enumerate(rows):
            y = 2.1 - 0.4 * index
            tone = DATA if layer == "paving" else INK
            colour = GLASS if layer == "imposed" else _core.layer_color(layer)
            group.add(_d.polygon([(-4.3, y - 0.1, 0), (-4.1, y - 0.1, 0), (-4.1, y + 0.1, 0), (-4.3, y + 0.1, 0)], colour, 1.0, INK, 0.8),
                      self.text(name, (-4.0, y, 0), size=S_SMALL, color=tone, anchor="l"))
            if thickness is not None:
                group.add(self.text(self.quantity(thickness, "m", 2), (columns[0], y, 0), size=S_SMALL, color=tone, anchor="r"), self.text("× " + self.quantity(density, "kN/m³", 0), (columns[1], y, 0), size=S_SMALL, color=tone, anchor="r"))
                for column, basis in zip(columns[:2], self.layer_basis[layer]):
                    if basis == "assumed":
                        group.add(self.text("*", (column + 0.04, y, 0), size=S_SMALL, color=WARN, anchor="l"))
            group.add(self.text("= " + self.quantity(weight, "kN/m²", 2), (columns[2], y, 0), size=S_SMALL, color=tone, anchor="r"), self.text("*", (columns[2] + 0.1, y, 0), size=S_SMALL, color=WARN, anchor="l"))
        bottom = 2.1 - 0.4 * len(rows) + 0.1
        group.add(_d.segments([((-4.3, bottom, 0), (columns[2] + 0.05, bottom, 0))], MUTED, _d.W_THIN),
                  self.text(self.tr("roof paving load q", "Last der Dachbelag q"), (-4.3, bottom - 0.3, 0), size=S_LABEL, color=DATA, anchor="l"),
                  self.text("= " + self.quantity(sum(row[4] for row in rows), "kN/m²", 2), (columns[2], bottom - 0.3, 0), size=S_LABEL, color=DATA, anchor="r"))
        return group

    def load_formula(self, paving):
        """🧮 The sum of the layer weights with the substituted numbers."""
        rows = self.weights(paving)
        parts = [("q =", INK)]
        for index, row in enumerate(rows):
            parts += [(("+" if index else ""), INK), (self.number(row[4], 2), DATA if row[0] == "paving" else INK)]
        parts += [("=", INK), (self.quantity(sum(row[4] for row in rows), "kN/m²", 2), DATA)]
        return self.formula([part for part in parts if part[0]], (-1.2, -1.05, 0), size=S_LABEL)

    def load_stack(self, point, paving, *, scale=0.17, label=True, old=None, side=-1):
        """📊 Stacked bar of the layer weights, one segment per layer in the layer colour, with the total on top."""
        group = _engine.VGroup()
        x, y = point[0], point[1]
        for layer, name, thickness, density, weight in self.weights(paving):
            colour = GLASS if layer == "imposed" else _core.layer_color(layer)
            group.add(_d.polygon([(x - 0.5, y, 0), (x + 0.5, y, 0), (x + 0.5, y + weight * scale, 0), (x - 0.5, y + weight * scale, 0)], colour, 1.0, INK, 1.0))
            y += weight * scale
        if label:
            group.add(self.text("q = " + self.quantity(sum(row[4] for row in self.weights(paving)), "kN/m²", 2), (x, y + 0.28, 0), size=S_LABEL, color=DATA))
        if old is not None:
            edge = point[1] + old * scale
            group.add(_d.polyline([(x - 0.7, edge, 0), (x + 0.7, edge, 0)], MUTED, _d.W_EDGE, dash=0.1),
                      self.text(self.tr("before ", "vorher ") + self.quantity(old, "kN/m²", 2), (x + 0.75 * side, edge, 0), size=S_SMALL, color=MUTED, anchor="r" if side < 0 else "l"))
        return group

    def _load(self):
        """⚖️ Beat 8: the thicker paving weighs more: layer weights, the sum and the growing bar."""
        paving = _engine.ValueTracker(self.paving_old)
        key = lambda: round(paving.get_value(), 3)
        build = self.live(lambda: self.build_up(self.paving_state(paving.get_value()), (-5.9, -2.4, 0), 3.2, 1.2, labels=False, dims=False, lit="paving"), key=key)
        title = self.paragraph([self.tr("roof paving", "Dachbelag"), self.tr("build-up", "Dachaufbau")], (-5.9, -0.05, 0), color=MUTED, pitch=0.25, anchor="c")
        table = self.live(lambda: self.load_table(paving.get_value()), key=key)
        formula = self.live(lambda: self.load_formula(paving.get_value()), key=key)
        delta = self.formula([("Δq =", INK), (f"({self.number(self.paving_new, 2)} − {self.number(self.paving_old, 2)}) m", DATA), ("·", INK), (self.quantity(next(row[3] for row in self.weights(self.paving_new) if row[0] == "paving"), "kN/m³", 0), DATA), ("=", INK),
                              ("+" + self.quantity(self.q_new - self.q_old, "kN/m²", 2), GOOD)], (-1.2, -1.9, 0), color=GOOD)
        stack = self.live(lambda: self.load_stack((5.0, -2.3, 0), paving.get_value(), old=self.q_old), key=key)
        self.show("load", new=[build, title, table, formula, delta, stack], extra=[paving.animate(rate_func=_engine.smooth).set_value(self.paving_new)], seconds=5.0)
        self.paving_tracker = paving

    def dim_number(self, value):
        """📏 A dimension in metres with two decimals, or three when the value needs them."""
        return self.quantity(value, "m", 2 if abs(round(value, 2) - value) < 1e-9 else 3)

    def cell_box(self, tag):
        """📦 Bounding box of the tributary cell of one piloti: (x0, x1, y0, y1)."""
        ring = self.cell[tag]["cell"]
        xs, ys = [p[0] for p in ring], [p[1] for p in ring]
        return min(xs), max(xs), min(ys), max(ys)

    def mini_cell(self, tag, point, scale, color):
        """📐 One cell drawn large with its two dimensions and the piloti dot; the area is the product of the two dimensions."""
        x0, x1, y0, y1 = self.cell_box(tag)
        width, height = x1 - x0, y1 - y0
        
        centre = np.array(point, float)
        ring = [centre + ((x - (x0 + x1) / 2) * scale, (y - (y0 + y1) / 2) * scale, 0) for x, y in self.cell[tag]["cell"]]
        group = _engine.VGroup(_d.polygon(ring, color, 0.2, color, _d.W_EDGE), self.hatch(ring, color, 0.07, 45, opacity=0.8))
        pin = self.pilotis[tag]
        group.add(_engine.Dot(centre + ((pin["x"] - (x0 + x1) / 2) * scale, (pin["y"] - (y0 + y1) / 2) * scale, 0), radius=0.06, color=INK))
        group.add(self.dimension(ring[0], ring[1], self.dim_number(width), offset=-0.3, color=color), self.dimension(ring[1], ring[2], self.dim_number(height), offset=-0.3, color=color))
        return group

    def _grid(self):
        """🗺️ Beat 9: the roof plan with grid, north and scale; each piloti collects the area around it."""
        plan_at, scale = (-4.2, 0.3, 0), 0.18
        plan = self.villa_plan("roof", plan_at, scale, model=self.base, dims=("grid",), scalebar=True, label_sides=("bottom", "left"))
        cells = self.tributary_view(self.trib_new, plan_at, scale, projection="plan", level="roof", highlight=(self.focus, self.contrast), labels=False, model=self.new)
        north = self.north_arrow(plan.at(0.0, 0.0, 0.0) + np.array([-0.35, -0.95, 0.0]), 0.4)
        panels = _engine.VGroup()
        forms = _engine.VGroup()
        bounds = {tag: self.cell_box(tag) for tag in (self.focus, self.contrast)}
        num = lambda v: self.number(v, 2 if abs(round(v, 2) - v) < 1e-9 else 3)
        for tag, colour, x, wide, note in ((self.focus, DATA, 1.0, 4.3, ("nearest-support cell", "Zelle der nächsten Stütze")), (self.contrast, GLASS, 5.1, 3.5, ("slab minus openings", "Decke ohne Öffnungen"))):
            x0, x1, y0, y1 = bounds[tag]
            kind = {"edge": ("edge piloti", "Randpiloti"), "interior": ("interior piloti", "Innenpiloti"), "corner": ("corner piloti", "Eckpiloti")}[self.cell[tag]["kind"]]
            panels.add(self.panel(wide, 3.3, (x, 1.0, 0), colour), self.text(f"{tag} · " + self.tr(*kind), (x, 2.5, 0), size=S_LABEL, color=colour),
                       self.mini_cell(tag, (x - (0.3 if tag == self.focus else 0.0), 1.45, 0), 0.4 if tag == self.focus else 0.3, colour), self.label(*note, (x, -0.95, 0), size=S_SMALL, color=MUTED))
            forms.add(self.formula([("A =", INK), (self.quantity(self.cell[tag]["area"], "m²", 2), colour)], (x, -0.35, 0), color=colour, size=S_SMALL))
        lines = [(f"{self.tr('Roof paving (solarium level)', 'Dachbelag (Solariumebene)')} · {self.quantity(self.zone_new['roof']['area'], 'm²', 1)} *", GOOD),
                 (self.tr("= roof slab minus solarium and openings (model)", "= Dachplatte ohne Solarium und Öffnungen (Modell)"), MUTED),
                 (f"{self.name(self.zone_new['solarium'])} · {self.quantity(self.zone_new['solarium']['area'], 'm²', 1)} ({self.tr('sourced', 'belegt')})", GLASS),
                 (f"{self.tr('teaching grid', 'Lehrraster')} {self.quantity(self.geometry['grid']['spacing'], 'm', 2)} ({self.tr('sourced', 'belegt')})", MUTED),
                 (self.tr(f"{len(self.trib_new['pilotis'])} roof supports in the teaching reconstruction *", f"{len(self.trib_new['pilotis'])} Dachstützen in der Lehrrekonstruktion *"), MUTED)]
        zones = _engine.VGroup(*[self.text(text, (-0.3, -1.25 - 0.27 * index, 0), size=S_SMALL, color=tone, anchor="l") for index, (text, tone) in enumerate(lines)])
        mark = self.section_mark(plan)
        self.show("grid", new=[plan, cells, north, mark, panels, forms, zones], seconds=5.0)

    def _derived(self):
        """📈 Beat 10: the area load and the loads of the pilotis below the roof follow the paving depth."""
        alpha = _engine.ValueTracker(0)
        self.alpha = alpha
        section, storeys = self.section_view(self.new), self.geometry["storeys"]
        marks = self.storey_marks(section)
        zones = [(zid, self.zone_old[zid]["area_load"], self.zone_new[zid]["area_load"], colour) for zid, colour in (("roof", DATA), ("solarium", GLASS))]
        arrows = [(zid, y, q_old, q_new, colour) for zid, q_old, q_new, colour in zones
                  for first, last in _core.scan(self.zone_new[zid]["pieces"], self.cut_x, axis=0) for y in np.linspace(first, last, max(2, round((last - first) / 2.2) + 1))]
        mix = lambda old, new: old + (new - old) * alpha.get_value()
        udl = self.live(lambda: _engine.VGroup(*[self.arrow(section.at(self.cut_x, y, storeys["roof"]) + (0, 0.02, 0), mix(q_old, q_new) * self.k_q, colour, width=3.5)
                                                 for zid, y, q_old, q_new, colour in arrows]))
        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])
        q_label = self.live_label(lambda: "q = " + self.quantity(mix(0, self.q_new - self.q_old), "kN/m²", 2),
                                  (-6.9, section.at(self.cut_x, 0.0, storeys["roof"])[1] + 0.3 + self.q_new * self.k_q, 0), size=S_LABEL, color=DATA, anchor="l")
        feet = {tag: section.at(self.cut_x, self.pilotis[tag]["y"], 0.0) for tag in self.column}
        r_now = lambda tag: mix(self.reaction_old[tag], self.reaction_new[tag])
        reactions = self.live(lambda: _engine.VGroup(*[self.arrow(feet[tag], r_now(tag) * K_KN, GOOD, up=True) for tag in self.column]))
        ghosts = _engine.VGroup(*[_d.polyline([feet[tag] + (-0.12, -self.reaction_old[tag] * K_KN, 0), feet[tag] + (0.12, -self.reaction_old[tag] * K_KN, 0)], MUTED, _d.W_EDGE) for tag in self.column])
        values = self.live(lambda: _engine.VGroup(*[_engine.VGroup(self.text(tag, feet[tag] + (0, -r_now(tag) * K_KN - 0.2, 0), size=S_SMALL, color=MUTED), self.text(self.quantity(r_now(tag), "kN", 0), feet[tag] + (0, -r_now(tag) * K_KN - 0.45, 0), size=S_SMALL, color=GOOD)) for tag in self.column]))
        panel = self.derivation((3.6, 0.55, 0))
        self.show("derived", new=[section, marks, udl, q_label, reactions, ghosts, values, panel], extra=[alpha.animate(rate_func=_engine.smooth).set_value(1.0)], seconds=4.2)

    def derivation(self, point):
        """🧾 R = A · q for the focus and the contrast piloti, counted with the paving depth, with the change as a chip."""
        alpha = self.alpha
        mix = lambda old, new: old + (new - old) * alpha.get_value()
        group = _engine.VGroup(self.panel(6.6, 4.3, point, DATA))
        x0, top = point[0] - 3.1, point[1] + 1.85
        group.add(self.label(f"{self.trib_new['method']}, not FEM", f"{self.trib_new['method_de']}, keine FEM", (x0, top, 0), size=S_SMALL, color=MUTED, anchor="l"),
                  self.formula([("ΔR", GOOD), ("=", INK), ("A", DATA), ("·", INK), ("Δq", DATA)], (point[0], top - 0.6, 0), color=MUTED))
        for index, (tag, colour) in enumerate(((self.focus, DATA), (self.contrast, GLASS))):
            entry = self.cell[tag]["zones"]["roof"]
            area, r_old, r_new = self.cell[tag]["area"], self.reaction_old[tag], self.reaction_new[tag]
            y = top - 1.55 - 1.3 * index
            group.add(self.label(f"{self.tr('piloti', 'Piloti')} {tag}", None, (x0, y, 0), size=S_LABEL, color=colour, anchor="l"),
                      self.live(lambda area=area, colour=colour, y=y, r_old=r_old, r_new=r_new: self.formula(
                          [(f"= {self.quantity(area, 'm²', 2)} ·", INK), (self.quantity(mix(self.q_old, self.q_new), "kN/m²", 2), DATA), ("=", INK), (self.quantity(mix(0, r_new - r_old), "kN", 0), GOOD)],
                          (point[0] + 0.4, y - 0.42, 0), color=colour, size=S_SMALL), key=lambda: round(alpha.get_value(), 3)),
                      self.label(f"{self.tr('before', 'vorher')} {self.quantity(r_old, 'kN', 0)}", None, (point[0] + 1.6, y + 0.12, 0), size=S_SMALL, color=MUTED, anchor="r"),
                      self.badge("+" + self.quantity(r_new - r_old, "kN", 0), (point[0] + 3.0, y + 0.12, 0), color=GOOD, size=S_SMALL, anchor="r"))
        group.add(self.label("short bar = before the change", "kurzer Strich = vor der Änderung", (x0, point[1] - 1.85, 0), size=S_SMALL, color=MUTED, anchor="l"))
        return group

    def signed(self, value):
        """±️ A rounded change with its sign, a plain zero when nothing changes."""
        shown = round(value)
        return ("+" if shown > 0 else "") + self.number(shown, 0)

    def delta_cells(self, point, scale, picks):
        """🌡️ Plan of the roof with every tributary cell shaded by the change of its piloti reaction and the change written on it."""
        group = _engine.VGroup()
        box = self.geometry["box"]
        origin = np.array(point) - np.array([box["length"] / 2 * scale, box["width"] / 2 * scale, 0])
        to_screen = lambda ring: [origin + (x * scale, y * scale, 0) for x, y in ring]
        top = max(self.delta.values())
        slab = next(s for s in self.geometry["slabs"] if s["tag"] == self.tag)
        group.add(_d.region([to_screen(slab["outline"])] + [to_screen(v) for v in slab["voids"]], CONCRETE_FILL, 0.18, INK, _d.W_EDGE), *[_d.polyline(to_screen(v), MUTED, _d.W_THIN, closed=True, dash=0.08) for v in slab["voids"]])
        grid = self.geometry["grid"]
        for label, x in zip(grid["labels_x"], grid["x"]):
            group.add(self._bubble(label, origin + (x * scale, -0.3, 0)))
        for label, y in zip(grid["labels_y"], grid["y"]):
            group.add(self._bubble(label, origin + (-0.3, y * scale, 0)))
        group.add(self.north_arrow(origin + (-0.35, box["width"] * scale + 0.55, 0), 0.4), self.scale_bar(origin + (box["length"] * scale - 5.0 * scale * 2, -0.75, 0), scale, 5.0, steps=2))
        for entry in self.trib_new["pilotis"]:
            tag = entry["tag"]
            shade = 0.1 + 0.6 * self.delta[tag] / top
            for piece in entry["pieces"] or ([entry["cell"]] if entry["cell"] else []):
                group.add(_d.polygon(to_screen(piece), GOOD, shade, MUTED, _d.W_HAIR))
        for entry in self.trib_new["pilotis"]:
            tag = entry["tag"]
            at = origin + (entry["x"] * scale, entry["y"] * scale, 0)
            ring = _engine.Circle(radius=0.14, stroke_color=picks[tag], stroke_width=_d.W_EDGE).move_to(at) if tag in picks else None
            group.add(_engine.Dot(at, radius=0.035, color=INK), *([ring] if ring else []))
        return group

    def _spread(self):
        """🌡️ Beat 11: every piloti changes by its own roof area; the largest, a small one and one that stays at zero."""
        plan_at, scale = (-3.4, 0.15, 0), 0.18
        ranked = sorted(self.delta.items(), key=lambda item: -item[1])
        largest, smallest = ranked[0][0], min(ranked, key=lambda item: item[1])[0]
        picks = [(largest, DATA, self.tr("largest", "größte")), (self.focus, GOOD, ""), (self.contrast, GLASS, self.tr("sliver", "Streifen")), (smallest, MUTED, self.tr("under the terrace", "unter der Terrasse"))]
        cells = self.delta_cells(plan_at, scale, {tag: colour for tag, colour, _ in picks})
        card = self.panel(6.2, 2.55, (3.85, 1.65), MUTED)
        count_note = self.label("Modelled supports on both storeys *", "Modellierte Stützen beider Geschosse *", (0.95, 0.62, 0), size=S_SMALL, color=MUTED, anchor="l")
        rows = _engine.VGroup(self.label("change of the roof load on the piloti", "Änderung der Dachlast auf dem Piloti", (0.95, 2.65, 0), size=S_SMALL, color=MUTED, anchor="l"))
        for index, (tag, colour, note) in enumerate(picks):
            y = 2.25 - 0.4 * index
            area = self.cell[tag]["area"]
            rows.add(self.text(f"{tag} · {self.quantity(area, 'm²', 2)} · {self.signed(self.delta[tag])} kN" + (f" · {note}" if note else ""), (0.95, y, 0), size=S_LABEL, color=colour, anchor="l"))
        total_old, total_new = self.trib_old["total_reaction"], self.trib_new["total_reaction"]
        roof = sum(zone["area"] for zone in self.zone_new.values())
        sums = _engine.VGroup(
            self.formula([("Σ ΔR =", INK), (self.quantity(self.q_new - self.q_old, "kN/m²", 1), DATA), ("·", INK), (self.quantity(roof, "m²", 1) + " *", DATA), ("=", INK), (self.quantity(total_new - total_old, "kN", 0), GOOD)], (4.0, -0.35, 0), color=GOOD),
            self.formula([(self.quantity(total_old, "kN", 0), MUTED), ("→", INK), (self.quantity(total_new, "kN", 0), GOOD), (self.tr("on all pilotis", "auf allen Pilotis"), MUTED)], (4.0, -1.2, 0), color=MUTED, size=S_SMALL))
        top = max(self.delta.values())
        swatches = _engine.VGroup(*[_d.polygon([(2.9 + 0.32 * i, -2.1, 0), (3.22 + 0.32 * i, -2.1, 0), (3.22 + 0.32 * i, -1.85, 0), (2.9 + 0.32 * i, -1.85, 0)], GOOD, 0.1 + 0.6 * i / 4, MUTED, _d.W_HAIR) for i in range(5)])
        legend = _engine.VGroup(swatches, self.label("0 kN", None, (2.8, -1.97, 0), size=S_SMALL, color=MUTED, anchor="r"), self.label(self.signed(top) + " kN", None, (4.6, -1.97, 0), size=S_SMALL, color=MUTED, anchor="l"),
                                self.label("shade = change of the reaction", "Tönung = Änderung der Auflagerkraft", (4.0, -2.35, 0), size=S_SMALL, color=MUTED))
        self.show("spread", new=[cells, card, rows, count_note, sums, legend], seconds=5.0)

    def slab_corner(self, view_at, scale, model, level_z, *, side):
        """📍 The screen corner of a slab at one height that lies furthest to the left or right in the axonometric."""
        box = self.geometry["box"]
        corners = [np.array(self.axon_point(x, y, level_z, view_at, scale, model=model)) for x in (0.0, box["length"]) for y in (0.0, box["width"])]
        return max(corners, key=lambda p: p[0] * side)

    def link_object(self, left, right, point, *, current, revision):
        """🔗 The link LK-ROOF as an object: a pill with its revision, joined to the slab on both sides; orange when it lags behind the object."""
        colour = GOOD if current else WARN
        pill = self.badge(LINK, point, color=colour, size=S_LABEL)
        note = self.label(f"{self.tr('source', 'Quelle')} {self.rev(revision)}" + ('' if current else ' · ' + self.tr('out of date', 'veraltet')), None, np.array(point) + (0, -0.38 if current else 0.4, 0), size=S_SMALL, color=colour)
        lines = _engine.VGroup(_d.polyline([left, pill.get_left() + (-0.02, 0, 0)], colour, _d.W_EDGE, dash=0.0 if current else 0.12),
                               _d.polyline([pill.get_right() + (0.02, 0, 0), right], colour, _d.W_EDGE, dash=0.0 if current else 0.12),
                               _engine.Dot(left, radius=0.05, color=colour), _engine.Dot(right, radius=0.05, color=colour))
        group = _engine.VGroup(lines, pill, note)
        group.pill = pill
        return group

    def relabel(self, arrows, *, gap=0.2):
        """🏷️ Move the value label of every load arrow clear of the arrow tail."""
        for arrow in list(arrows.submobjects):
            label = arrow.submobjects[-1]
            if hasattr(label, "text"):
                arrow.remove(label)
                arrow.add(self._note(label.text, np.array(arrow.get_bottom()) + (0.0, -gap, 0.0), color=label.get_fill_color(), anchor="u"))
        return arrows

    def pair(self, *, stale, point_y=-0.25, scale=0.15, offset=3.45, arrows=True, k=K_PAIR):
        """🪞 The up-to-date model beside the linked analysis: the roof reactions of the shown pilotis and the link between the two slabs."""
        left_at, right_at = (-offset, point_y, 0), (offset, point_y, 0)
        analysis, reactions, state = (self.analysis_old, self.reaction_old, self.base) if stale else (self.analysis_new, self.reaction_new, self.resolved)
        model_view = self.tributary_view(self.trib_new, left_at, scale, projection="axon", highlight=tuple(self.pair_tags), level="roof", reactions=set(self.pair_tags) if arrows else False,
                                         arrow_scale=k, model=self.new)
        analysis_view = self.analysis_view(analysis, right_at, scale, projection="axon", loads={t: reactions[t] for t in self.pair_tags} if arrows else None, stale=stale,
                                           highlight=tuple(self.pair_tags), load_scale=k, revision=False, model=state)
        if arrows:
            self.relabel(model_view.arrows)
            self.relabel(analysis_view.loads)
        roof_z, mid_z = self.geometry["storeys"]["roof"], analysis["mid_surface"]["z"]
        a = self.slab_corner(left_at, scale, self.new, roof_z, side=1)
        b = self.slab_corner(right_at, scale, state, mid_z, side=-1)
        return model_view, analysis_view, a, b

    def _stale(self):
        """🟠 Beat 12: the linked analysis still holds the old revision and the old load."""
        scale, right_at = 0.15, (3.45, -0.25, 0)
        model_view, stale_view, a, b = self.pair(stale=True)
        gap = _engine.VGroup(*[_d.polyline([self.foot(tag, right_at, scale, self.base) + (0, -self.reaction_old[tag] * K_PAIR, 0),
                                            self.foot(tag, right_at, scale, self.base) + (0, -self.reaction_new[tag] * K_PAIR, 0)], GOOD, 2.6, dash=0.1) for tag in self.pair_tags])
        missing = self.reaction_new[self.focus] - self.reaction_old[self.focus]
        mid = self.foot(self.focus, right_at, scale, self.base) + (0.18, -self.reaction_old[self.focus] * K_PAIR + 0.2, 0)
        gap.add(self._note(f"+{self.quantity(missing, 'kN', 0)} " + self.tr("missing", "fehlen"), mid, size=S_LABEL, color=GOOD, anchor="l"))
        head_left = self.badge(f"{self.tr('architecture + structure', 'Architektur + Tragwerk')} · {self.rev(self.revision(self.new))}", (-3.45, 2.0, 0), color=ARCH, size=S_LABEL)
        head_right = self.badge(f"{self.tr('structural analysis', 'Tragwerksanalyse')} · {self.tr('uses', 'nutzt')} {self.rev(self.link['source_revision'])}", (3.45, 2.0, 0), color=WARN, size=S_LABEL)
        chip = self.guid_chip(self.tag, (0.0, 2.75, 0), label=False)
        leaders = _engine.VGroup(_d.polyline([chip.get_left() + (0.0, -0.15, 0), head_left.get_top()], DATA, _d.W_THIN), _d.polyline([chip.get_right() + (0.0, -0.15, 0), head_right.get_top()], DATA, _d.W_THIN))
        link = self.link_object(a, b, (0.0, 0.7, 0), current=False, revision=self.link["source_revision"])
        self.show("stale", new=[model_view, head_left, head_right, chip, leaders, stale_view, link, gap], seconds=4.6)
        self.views = (model_view, stale_view)

    def issue_rows(self):
        """🚦 Expected versus actual of the stale link: the model's issues for the revision, the roof load, one reaction and the sum."""
        values = {i["key"]: i for i in self.issues if i["code"] == "stale-value"}
        rows = [(f"{self.tr('link', 'Verknüpfung')} {LINK} · {self.tr('revision', 'Revision')}", str(i["expected"]), str(i["actual"])) for i in self.issues if i["code"] == "stale-link"]
        for key, name, unit, digits in (("roof_area_load", self.tr("roof paving load q", "Last der Dachbelag q"), "kN/m²", 2), (f"reaction_{self.focus}", f"{self.tr('reaction at', 'Auflagerkraft bei')} {self.focus}", "kN", 0),
                                        ("total_reaction", self.tr("sum of all reactions", "Summe aller Auflagerkräfte"), "kN", 0)):
            rows.append((name, self.quantity(values[key]["expected"], unit, digits), self.quantity(values[key]["actual"], unit, digits)))
        return rows

    def issue_table(self, rows, point, width, marks=()):
        """📋 Table of issue, expected and actual with a mismatch mark per row and an optional numbered badge that names the drawn element."""
        pitch = 0.36
        frame = self.panel(width, pitch * (len(rows) + 1) + 0.24, point, WARN)
        group = _engine.VGroup(frame)
        top = frame.get_top()[1] - 0.12 - pitch / 2
        left = frame.get_left()[0] + 0.2
        cols = [left, left + width * 0.48, left + width * 0.72]
        shift = 0.45 if marks else 0.0
        for x, head, color in zip(cols, (self.tr("what is compared", "Verglichen wird"), self.tr("expected (model now)", "erwartet (Modell jetzt)"), self.tr("actual (link)", "tatsächlich (Verknüpfung)")), (MUTED, GOOD, WARN)):
            group.add(self.text(head, (x + (shift if x == left else 0.0), top, 0), size=S_SMALL, color=color, anchor="l"))
        for index, row in enumerate(rows):
            y = top - pitch * (index + 1)
            group.add(self.text(row[0], (cols[0] + shift, y, 0), size=S_LABEL, color=INK, anchor="l"), self.text(row[1], (cols[1], y, 0), size=S_LABEL, color=GOOD, anchor="l"),
                      self.text(row[2], (cols[2], y, 0), size=S_LABEL, color=WARN, anchor="l"), self.unequal((frame.get_right()[0] - 0.35, y, 0)).scale(0.5))
            if marks:
                group.add(self.badge(marks[index], (cols[0] + 0.12, y, 0), color=DATA, size=S_SMALL))
        return group

    def _check(self):
        """🔎 Beat 13: the link record makes the stale result findable: expected against actual."""
        model_view, stale_view, a, b = self.pair(stale=True, point_y=1.8, scale=0.085, offset=4.4, arrows=True, k=K_PAIR * 0.6)
        link = self.link_object(a, b, (0.0, 2.1, 0), current=False, revision=self.link["source_revision"])
        record = self.label(f"{self.tr('link', 'Verknüpfung')} {LINK} · {self.tr('object', 'Objekt')} GlobalId = {self.gid} · {self.tr('source revision', 'Quell-Revision')} = {self.link['source_revision']}", None,
                            (0.0, -2.2, 0), size=S_SMALL, color=INK)
        table = self.issue_table(self.issue_rows(), (0.0, -0.75, 0), 11.2, marks=("1", "2", "3", "4"))
        badge = lambda text, point: self.badge(text, point, color=DATA, size=S_SMALL)
        arrow_model, arrow_analysis = model_view.arrows.submobjects[0], stale_view.loads.submobjects[0]
        tags = _engine.VGroup(badge("1", link.pill.get_bottom() + (0.0, -0.22, 0.0)), badge("2", model_view.get_top() + (0.0, 0.22, 0.0)),
                              badge("3", arrow_model.get_left() + (-0.22, 0.0, 0.0)), badge("3", arrow_analysis.get_left() + (-0.22, 0.0, 0.0)),
                              badge("4", model_view.get_left() + (-0.3, 0.0, 0.0)), badge("4", stale_view.get_right() + (0.3, 0.0, 0.0)))
        self.show("check", new=[model_view, stale_view, link, record, table, tags], seconds=4.0)

    def _ids(self):
        """✅ Beat 14: a requirement check (IDS) passes for the same slab while the link check fails."""
        result = _model.ids_check(self.new, "IfcSlab Pset_SlabCommon.LoadBearing exists")
        axon = self.villa_axon((0.0, 0.0, 0), 0.1, model=self.new, highlight=(self.tag,))
        ids = self.panel(4.4, 3.3, (-4.7, 0.5), GOOD)
        ids_rows = _engine.VGroup(self.label("IDS · requirement", "IDS · Anforderung", (-6.75, 1.85, 0), size=S_LABEL, color=GOOD, anchor="l"),
                                  self.label("applies to · IfcSlab", "gilt für · IfcSlab", (-6.75, 1.3, 0), size=S_LABEL, color=INK, anchor="l"),
                                  self.mono("Pset_SlabCommon.LoadBearing", (-6.75, 0.85, 0), size=S_SMALL, color=DATA, anchor="l"),
                                  self.label("cardinality · required", "Kardinalität · erforderlich", (-6.75, 0.4, 0), size=S_LABEL, color=INK, anchor="l"),
                                  self.label(f"{result['passed']} {self.tr('of', 'von')} {result['expected']} " + self.tr("slabs pass", "Platten bestehen"), None, (-6.75, -0.25, 0), size=S_BODY, color=GOOD, anchor="l"),
                                  self.label("checks that information exists", "prüft, dass Angaben vorhanden sind", (-6.75, -0.85, 0), size=S_SMALL, color=MUTED, anchor="l"))
        tick = self.checkmark((-2.85, -0.25, 0)).scale(1.8)
        link = self.panel(4.4, 3.3, (4.7, 0.5), WARN)
        issue = next(i for i in self.issues if i["code"] == "stale-link")
        link_rows = _engine.VGroup(self.label("link check · LK-ROOF", "Verknüpfungsprüfung · LK-ROOF", (2.75, 1.85, 0), size=S_LABEL, color=WARN, anchor="l"),
                                   self.label(f"{self.tr('object revision', 'Revision des Objekts')} · {issue['expected']}", None, (2.75, 1.3, 0), size=S_LABEL, color=GOOD, anchor="l"),
                                   self.label(f"{self.tr('link revision', 'Revision der Verknüpfung')} · {issue['actual']}", None, (2.75, 0.85, 0), size=S_LABEL, color=WARN, anchor="l"),
                                   self.label(f"q · {self.quantity(self.q_new, 'kN/m²', 2)} ≠ {self.quantity(self.q_old, 'kN/m²', 2)}", None, (2.75, 0.4, 0), size=S_LABEL, color=INK, anchor="l"),
                                   self.label("stale", "veraltet", (2.75, -0.25, 0), size=S_BODY, color=WARN, anchor="l"),
                                   self.label("checks that results are up to date", "prüft, dass Ergebnisse aktuell sind", (2.75, -0.85, 0), size=S_SMALL, color=MUTED, anchor="l"))
        cross = self.cross((6.55, -0.25, 0)).scale(1.8)
        roof_z = self.geometry["storeys"]["roof"]
        pointers = _engine.VGroup(self.leader(ids, self.slab_corner((0.0, 0.0, 0), 0.1, self.new, roof_z, side=-1), color=GOOD), self.leader(link, self.slab_corner((0.0, 0.0, 0), 0.1, self.new, roof_z, side=1), color=WARN))
        under = self.label("same slab, two different questions", "dieselbe Platte, zwei verschiedene Fragen", (0.0, -2.2, 0), size=S_LABEL, color=INK)
        self.show("ids", new=[axon, ids, ids_rows, tick, link, link_rows, cross, pointers, under], seconds=4.4)

    def _report(self):
        """📣 Beat 15: the finding becomes a BCF topic that points to the slab by its GlobalId."""
        axon = self.villa_axon(AXON_AT, AXON_SCALE, model=self.new, highlight=(self.tag,))
        author = self.author(_model.partial_model("structure", self.new)["author"])
        card = self.card([(self.tr("status", "Status"), self.tr("open", "offen")), (self.tr("assigned to", "zugewiesen an"), author), (self.tr("component", "Komponente"), "GlobalId"),
                          ("", self.gid), (self.tr("link", "Verknüpfung"), LINK)], (4.0, 0.9, 0), 6.0, self.tr("BCF topic · analysis uses rev 1", "BCF-Thema · Analyse nutzt Rev. 1"), WARN, value_width=3.2, mono=("",))
        thumb_frame = self.panel(2.4, 1.7, (4.0 - 1.8, -1.55), MUTED)
        thumb = self.villa_axon(thumb_frame.get_center() + (0.0, 0.0, 0.0), 0.05, model=self.new, highlight=(self.tag,))
        view_label = self.label("viewpoint", "Blickpunkt", thumb_frame.get_top() + (0.0, 0.15, 0), size=S_SMALL, color=MUTED)
        note = self.paragraph([self.tr("BCF carries issues,", "BCF trägt Themen,"), self.tr("not models", "keine Modelle")], (5.3, -1.45, 0), color=MUTED, pitch=0.26, anchor="c")
        pointer = self.leader((card[0].get_left()[0], card.values[3].get_center()[1], 0), axon.at(self.tag), color=WARN)
        edge = np.array(axon.at(self.geometry["box"]["length"], 0.0, self.geometry["storeys"]["roof"]))
        link_pointer = self.leader((card[0].get_left()[0], card.values[4].get_center()[1], 0), edge, color=GOOD)
        self.show("report", new=[axon, card, thumb_frame, thumb, view_label, note, pointer, link_pointer], seconds=4.0)

    def _retranslate(self):
        """🔄 Beat 16: the analysis model is regenerated from the slab and its assumptions are reviewed by a person."""
        counts = self.analysis_new["counts"]
        view = self.analysis_view(self.analysis_new, (-3.4, 0.25, 0), 0.19, projection="axon", highlight=("SL-1",), revision=False, model=self.resolved)
        trackers = [_engine.ValueTracker(0) for _ in range(3)]
        targets = (counts["surface_members"], counts["curve_members"], counts["slab_nodes"])
        names = (self.tr("surface members", "Flächenelemente"), self.tr("curve members (columns) *", "Stabelemente (Stützen) *"), self.tr("slab nodes", "Deckenknoten"))
        card = self.panel(6.4, 1.45, (3.7, 2.2), STRUCT)
        counters = _engine.VGroup(*[self.live_label(lambda t=t, n=n: f"{int(round(t.get_value()))} {n}", (0.6, 2.55 - 0.42 * index, 0), size=S_LABEL, color=STRUCT, anchor="l") for index, (t, n) in enumerate(zip(trackers, names))])
        self.add(*trackers)
        lines = []
        for item in [a for a in self.analysis_new["assumptions"] if a["id"] in ("mid-surface", "mesh-grid", "linear-elastic")]:
            lines.append((self.wrap(item[self.language()].replace(f"{counts['base_nodes']} base nodes", f"{counts['base_nodes']} base nodes *").replace(f"{counts['base_nodes']} Fußknoten", f"{counts['base_nodes']} Fußknoten *"), 44), True))
        lines.append((self.wrap(self.tr("not transferred: ", "nicht übertragen: ") + self.analysis_new["loss_groups"][0][self.language()], 44), False))
        pilotis = self.label("* Source-informed teaching reconstruction", "* Quellenbezogene Lehrrekonstruktion", (0.6, 1.33, 0), size=S_SMALL, color=MUTED, anchor="l")
        head = self.label("review · checked by a person", "Prüfung · von einer Person geprüft", (1.05, 0.86, 0), size=S_LABEL, color=GOOD, anchor="l")
        person = self.person((0.55, 0.83, 0), GOOD, 0.4)
        texts, boxes, ticks = _engine.VGroup(), _engine.VGroup(), _engine.VGroup()
        y = 0.44
        for rows, confirmed in lines:
            for number, row in enumerate(rows):
                texts.add(self.text(row, (1.2, y, 0), size=S_SMALL, color=INK if confirmed else MUTED, anchor="l"))
                y -= 0.26
            if confirmed:
                at = (0.85, y + 0.26 * (len(rows) + 1) / 2, 0)
                boxes.add(_engine.Square(side_length=0.2, stroke_color=GOOD, stroke_width=_d.W_THIN).move_to(at))
                ticks.add(self.checkmark(at))
            y -= 0.08
        self.show("retranslate", new=[view, card, counters, pilotis, head, person, texts, boxes], extra=[
            _engine.AnimationGroup(*[t.animate(rate_func=_engine.linear).set_value(v) for t, v in zip(trackers, targets)]),
            _engine.LaggedStart(*[_engine.Create(tick) for tick in ticks], lag_ratio=0.5)], track=[ticks], seconds=5.0)

    def _resolve(self):
        """✅ Beat 17: the link is updated to revision 2; the arrows match again and the check finds nothing."""
        model_view, fixed_view, a, b = self.pair(stale=False)
        head_left = self.badge(f"{self.tr('architecture + structure', 'Architektur + Tragwerk')} · {self.rev(self.revision(self.resolved))}", (-3.45, 2.0, 0), color=ARCH, size=S_LABEL)
        head_right = self.badge(f"{self.tr('structural analysis', 'Tragwerksanalyse')} · {self.rev(self.link_new['source_revision'])}", (3.45, 2.0, 0), color=GOOD, size=S_LABEL)
        chip = self.guid_chip(self.tag, (0.0, 2.75, 0), label=False)
        leaders = _engine.VGroup(_d.polyline([chip.get_left() + (0.0, -0.15, 0), head_left.get_top()], DATA, _d.W_THIN), _d.polyline([chip.get_right() + (0.0, -0.15, 0), head_right.get_top()], DATA, _d.W_THIN))
        link = self.link_object(a, b, (0.0, 0.7, 0), current=True, revision=self.link_new["source_revision"])
        issues = len(_model.consistency_issues(self.resolved))
        key = f"reaction_{self.focus}"
        model_now = next(p["reaction"] for p in _model.tributary(self.resolved, "roof")["pilotis"] if p["tag"] == self.focus)
        analysis_now, analysis_before = (analysis["used"]["roof-load"][key] for analysis in (self.analysis_new, self.analysis_old))
        sign = lambda a, b: ("=", GOOD) if math.isclose(a, b, abs_tol=0.5) else ("≠", WARN)
        revision = self.rev(self.revision(self.resolved))
        verdict = _engine.VGroup(
            self.stack([(f"{self.focus} · {revision}", MUTED), (f"{self.tr('analysis', 'Analyse')} {self.quantity(analysis_now, 'kN', 0)}", GOOD), (f"{sign(analysis_now, model_now)[0]} {self.tr('model', 'Modell')} {self.quantity(model_now, 'kN', 0)}", sign(analysis_now, model_now)[1])],
                       (0.0, -1.55, 0), GOOD),
            self.stack([(f"{self.rev(self.link['source_revision'])}: {self.quantity(analysis_before, 'kN', 0)} {sign(analysis_before, model_now)[0]} {self.quantity(model_now, 'kN', 0)}", WARN)], (0.0, -2.3, 0), MUTED))
        done = self.badge(f"{issues} " + self.tr("issues", "Probleme"), (-1.3, 1.5, 0), color=GOOD, size=S_LABEL)
        bcf = self.badge("BCF · " + self.tr("resolved", "gelöst"), (1.3, 1.5, 0), color=GOOD, size=S_LABEL)
        ties = _engine.VGroup(_d.polyline([link.pill.get_top() + (-0.1, 0.0, 0), done.get_bottom()], GOOD, _d.W_THIN), _d.polyline([link.pill.get_top() + (0.1, 0.0, 0), bcf.get_bottom()], GOOD, _d.W_THIN))

        self.show("resolve", new=[model_view, head_left, head_right, chip, leaders, fixed_view, link, verdict, done, bcf, ties], seconds=4.6)

    def edit_card(self, point, title, model, paving, color):
        """🧑 One edit of the slab: person, revision, paving depth, build-up and the unchanged GlobalId."""
        card = self.panel(5.0, 2.65, point, color)
        x, y = point[0], point[1]
        group = _engine.VGroup(card, self.person((x - 2.1, y + 0.82, 0), color, 0.42), self.text(title, (x - 1.75, y + 0.98, 0), size=S_LABEL, color=color, anchor="l"),
                               self.text(f"{self.rev(self.revision(model))} · {self.tr('paving', 'Belag')} {self.quantity(paving, 'm', 2)} *", (x - 2.3, y + 0.5, 0), size=S_LABEL, color=INK, anchor="l"),
                               self.build_up(model, (x - 1.7, y - 0.95, 0), 1.9, 1.0, labels=False, dims=False, lit="paving"),
                               self.mono(self.gid, (x - 0.3, y - 0.2, 0), size=S_SMALL, color=DATA, anchor="l"),
                               self.label("same GlobalId", "gleiche GlobalId", (x - 0.3, y - 0.55, 0), size=S_SMALL, color=MUTED, anchor="l"),
                               self.text("q = " + self.quantity(self.q_of(paving), "kN/m²", 2), (x - 0.3, y - 1.0, 0), size=S_LABEL, color=GOOD, anchor="l"))
        return group

    def _fork(self):
        """🍴 Beat 18: two people edit revision 1 at once; both get revision 2 and keep the GlobalId."""
        root = self.panel(4.6, 1.3, (0.0, 2.0), MUTED)
        root_group = _engine.VGroup(root, self.text(f"{self.rev(self.revision(self.base))} · {self.tr('paving', 'Belag')} {self.quantity(self.paving_old, 'm', 2)} *", (-2.1, 2.3, 0), size=S_LABEL, color=INK, anchor="l"),
                                    self.mono(self.gid, (-2.1, 1.8, 0), size=S_SMALL, color=DATA, anchor="l"),
                                    self.build_up(self.base, (1.55, 1.45, 0), 1.6, 0.9, labels=False, dims=False, lit="paving"))
        edit_a = self.edit_card((-3.8, -0.7, 0), self.tr("edit A · person 1", "Änderung A · Person 1"), self.new, self.paving_new, ARCH)
        edit_b = self.edit_card((3.8, -0.7, 0), self.tr("edit B · person 2", "Änderung B · Person 2"), self.alt, self.paving_alt, STRUCT)
        arrows = _engine.VGroup(_engine.Arrow((-0.9, 1.35, 0), (-3.6, 0.7, 0), buff=0, color=ARCH, stroke_width=3), _engine.Arrow((0.9, 1.35, 0), (3.6, 0.7, 0), buff=0, color=STRUCT, stroke_width=3))
        middle = _engine.VGroup(self.unequal((0.0, -0.5, 0)), self.label("different content", "anderer Inhalt", (0.0, -1.05, 0), size=S_SMALL, color=WARN, width=1.9))
        self.show("fork", new=[root_group, edit_a, edit_b, arrows, middle], seconds=4.4)

    def _decide(self):
        """⚖️ Beat 19: the GlobalId cannot merge two loads; a person decides."""
        stacks = _engine.VGroup(self.load_stack((-3.4, -2.3, 0), self.paving_new, scale=0.27, old=self.q_old), self.load_stack((3.4, -2.3, 0), self.paving_alt, scale=0.27, old=self.q_old, side=1))
        names = _engine.VGroup(self.label(f"{self.tr('edit A', 'Änderung A')} · {self.rev(self.revision(self.new))} · {self.tr('paving', 'Belag')} {self.quantity(self.paving_new, 'm', 2)} *", None, (-3.4, 2.75, 0), size=S_LABEL, color=ARCH),
                               self.label(f"{self.tr('edit B', 'Änderung B')} · {self.rev(self.revision(self.alt))} · {self.tr('paving', 'Belag')} {self.quantity(self.paving_alt, 'm', 2)} *", None, (3.4, 2.75, 0), size=S_LABEL, color=STRUCT),
                               self.mono(self.gid, (-3.4, 2.35, 0), size=S_SMALL, color=DATA), self.mono(self.gid, (3.4, 2.35, 0), size=S_SMALL, color=DATA))
        verdict = _engine.VGroup(self.unequal((0.0, 0.7, 0)), self.paragraph([self.tr("no automatic", "keine automatische"), self.tr("merge", "Zusammenführung")], (0.0, 0.2, 0), size=S_LABEL, color=MUTED, pitch=0.3, anchor="c"),
                                 self.person((0.0, -1.0, 0), GOOD, 0.6), self.label("a person decides", "eine Person entscheidet", (0.0, -1.5, 0), size=S_LABEL, color=GOOD, width=2.9))
        self.show("decide", new=[stacks, names, verdict], seconds=4.4)

    def _recap(self):
        """🏛️ Beat 20: the villa as separate models, the analysis card and the four things that tie them together."""
        explode, final = _engine.ValueTracker(0), 0.7
        at, scale = (-3.6, 0.1, 0), 0.105
        axon = self.live(lambda: self.villa_axon(at, scale, model=self.resolved, explode=explode.get_value()), key=lambda: round(explode.get_value(), 2))
        full = self.villa_axon(at, scale, model=self.resolved, explode=final)
        layers = _engine.VGroup(*[self.label(name, None, (-6.95, full.layers[key].get_center()[1], 0), size=S_SMALL, color=color, anchor="l")
                                  for key, name, color in (("architecture", self.tr("architecture", "Architektur"), ARCH), ("structure", self.tr("structure", "Tragwerk"), STRUCT), ("services", self.tr("services", "Haustechnik"), SERV))])
        tie = _engine.VGroup(*[self.leader(label, np.array(full.layers[key].get_left()) + (0.08, 0.0, 0.0), color=color, dot=False) for label, (key, color) in zip(layers, (("architecture", ARCH), ("structure", STRUCT), ("services", SERV)))])
        analysis = _engine.VGroup(self.panel(2.6, 0.8, (-3.6, -2.2), GOOD), self.label("structural analysis", "Tragwerksanalyse", (-3.6, -2.08, 0), size=S_SMALL, color=GOOD),
                                  self.label(self.rev(self.link_new["source_revision"]) + " · " + LINK, None, (-3.6, -2.35, 0), size=S_SMALL, color=GOOD))
        things = [("schema", self.tr("Schema", "Schema"), self.resolved["schema_id"], MUTED, full.at(self.tag, layer="architecture") + (1.0, 0.7, 0)),
                  ("globalid", "GlobalId", self.tr("names the object", "benennt das Objekt"), DATA, np.array(full.at(self.tag, layer="architecture"))),
                  ("revision", self.tr("Revision", "Revision"), self.tr("dates the content", "datiert den Inhalt"), WARN, np.array(full.at(self.tag, layer="structure"))),
                  ("link", self.tr("Link", "Verknüpfung") + f" {LINK}", self.tr("ties the analysis to the slab", "bindet die Analyse an die Platte"), GOOD, np.array(analysis[0].get_top()))]
        pills, pointers = _engine.VGroup(), _engine.VGroup()
        for index, (key, term, meaning, colour, target) in enumerate(things):
            y = 2.2 - 1.15 * index
            pill = self.badge(term, (1.6, y, 0), color=colour, size=S_LABEL, anchor="l")
            pills.add(pill, self.text(meaning, (1.6, y - 0.42, 0), size=S_LABEL, color=INK, anchor="l"))
            pointers.add(self.leader(pill, target, color=colour, dot=True))
        self.show("recap", new=[axon, layers, tie, analysis, pills, pointers], extra=[explode.animate(rate_func=_engine.smooth).set_value(final)], seconds=3.4)
        self.play(*[_engine.Indicate(pill, color=GOOD) for pill in pills[::2]], run_time=1.4)
                                                                                        