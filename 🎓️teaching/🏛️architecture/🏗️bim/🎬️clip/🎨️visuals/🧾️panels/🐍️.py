"""🧾 Concept panels of the BIM clips: class boxes and trees, property tables, STEP text, model cards and identity chips."""

from __future__ import annotations

import importlib

import numpy as np

_drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, SERV, DATA, GOOD, WARN = (getattr(_drafting, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "SERV", "DATA", "GOOD", "WARN"))
S_SMALL, S_LABEL, S_BODY, W_THIN, W_EDGE, W_HAIR = (getattr(_drafting, name) for name in ("S_SMALL", "S_LABEL", "S_BODY", "W_THIN", "W_EDGE", "W_HAIR"))
P, mix, place, segments = _drafting.P, _drafting.mix, _drafting.place, _drafting.segments

ROLE_COLORS = {"guid": DATA, "class": ARCH, "name": INK, "tag": INK, "predefined": WARN, "pset": GOOD, "property": GOOD, "value": INK,
               "placement": MUTED, "representation": MUTED, "profile": STRUCT, "extrusion": STRUCT, "depth": STRUCT, "material": STRUCT,
               "number": MUTED, "reference": _theme.color("gray-600")}
CARD_NAMES = {"architecture": ("Architecture", "Architektur"), "structure": ("Structure", "Tragwerk"), "services": ("Services", "Haustechnik")}
BASIS_NAMES = {"sourced": ("sourced", "belegt"), "assumed": ("assumed", "angenommen"), "inferred": ("inferred", "gefolgert")}


def frame_box(width: float, height: float, color=MUTED, *, fill: float = 0.9, radius: float = 0.06):
    """🔲 Dark rounded panel frame."""
    return _engine.RoundedRectangle(corner_radius=radius, width=width, height=height, stroke_color=color, stroke_width=W_THIN,
                                    fill_color=_art.CHROME_FILL, fill_opacity=fill)


class Panels:
    """🧾 Scene mixin: every concept widget of the clips."""

    def class_box(self, name: str, rows, point=(0, 0, 0), *, color=ARCH, width: float = 3.2, grey=(), size=S_SMALL, anchor="c"):
        """📦 UML-style class box: coloured header, rows below; ``grey`` row texts are drawn as inherited."""
        items = [(row, MUTED if row in grey else INK) if isinstance(row, str) else row for row in rows]
        head = self.text(name, (0, 0, 0), size=S_LABEL, color=color, width=width - 0.2)
        pitch = 0.3
        height = 0.46 + (len(items) * pitch + 0.14 if items else 0.0)
        frame = frame_box(width, height, color)
        group = _engine.VGroup(frame, head.move_to(frame.get_top() + np.array([0, -0.23, 0])))
        rule = segments([(frame.get_corner(_engine.UL) + (0, -0.46, 0), frame.get_corner(_engine.UR) + (0, -0.46, 0))], color, W_THIN) if items else None
        labels = []
        for index, (row, tone) in enumerate(items):
            label = self.text(row, frame.get_corner(_engine.UL) + np.array([0.14, -0.46 - 0.07 - pitch * (index + 0.5), 0]), size=size, color=tone, width=width - 0.28, anchor="l")
            labels.append(label)
        if rule is not None:
            group.add(rule)
        group.add(*labels)
        group.frame, group.header, group.rows = frame, head, labels
        return place(group, point, anchor)

    def class_tree(self, chains, point=(0, 0, 0), *, highlight=(), width: float = 6.4, expand: str | None = None, color=ARCH, leaf_attrs: int = 2, pitch: float = 0.58):
        """🌳 IFC inheritance tree. ``chains`` is one ``class_chain`` result, or a list of them (leaf classes share the trunk and sit side by side on the last row).
        ``leaf_attrs`` caps the attributes listed under a leaf (the rest reads ``+n``). Parts: ``tree.chips[cls]`` (frame + name), ``tree.rows[cls]`` (chip + own attributes), ``tree.attr[(cls, attribute)]``, ``tree.arrows``.
        ``expand`` names a leaf class whose full attribute box (inherited grey, own bright) stands to the right as ``tree.box`` with ``tree.box_rows[attribute]``."""
        chains = [chains] if chains and isinstance(chains[0], dict) else list(chains)
        chains = [chain if isinstance(chain, list) else _model.class_chain(chain) for chain in chains]
        tree = _engine.VGroup()
        tree.chips, tree.attr, tree.rows, tree.box, tree.box_rows = {}, {}, {}, None, {}
        depth_nodes: dict[int, list] = {}
        for chain in chains:
            for depth, node in enumerate(chain):
                if node["class"] not in [n["class"] for n in depth_nodes.setdefault(depth, [])]:
                    depth_nodes[depth].append(node)
        parents = {chain[i]["class"]: chain[i - 1]["class"] for chain in chains for i in range(1, len(chain))}
        arrows = _engine.VGroup()

        def build(node, leaf_row):
            lit = node["class"] in highlight
            tone = DATA if lit else color
            chip = self.text(node["class"], (0, 0, 0), size=S_LABEL, color=tone)
            frame = _engine.RoundedRectangle(corner_radius=0.05, width=chip.width + 0.28, height=0.36, stroke_color=tone, stroke_width=W_EDGE if lit else W_THIN,
                                             fill_color=_art.CHROME_FILL, fill_opacity=0.95)
            chip.move_to(frame)
            row = _engine.VGroup(frame, chip)
            attrs = _engine.VGroup()
            shown = node["adds"] if not leaf_row or len(node["adds"]) <= leaf_attrs else node["adds"][:leaf_attrs]
            for name in shown:
                label = self.text(name, (0, 0, 0), size=S_SMALL, color=INK)
                tree.attr[(node["class"], name)] = label
                attrs.add(label)
            if len(shown) < len(node["adds"]):
                attrs.add(self.text(f"+{len(node['adds']) - len(shown)}", (0, 0, 0), size=S_SMALL, color=MUTED))
            if len(attrs):
                attrs.arrange(_engine.RIGHT if not leaf_row else _engine.DOWN, buff=0.18)
            return row, frame, attrs, tone

        for depth, nodes in sorted(depth_nodes.items()):
            y = -depth * pitch
            leaf_row = len(nodes) > 1
            built = [build(node, leaf_row) for node in nodes]
            if leaf_row:
                widths = [max(frame.width, attrs.width if len(attrs) else 0.0) + 0.3 for _, frame, attrs, _ in built]
                x = -width / 2
                for (row, frame, attrs, tone), node, w in zip(built, nodes, widths):
                    centre = x + w / 2
                    x += w
                    row.shift(np.array([centre, y, 0.0]) - frame.get_center())
                    if len(attrs):
                        place(attrs, frame.get_bottom() + np.array([0, -0.1, 0]), "u")
                        row.add(attrs)
            else:
                for (row, frame, attrs, tone), node in zip(built, nodes):
                    row.shift(np.array([-width / 2 + frame.width / 2, y, 0.0]) - frame.get_center())
                    if len(attrs):
                        place(attrs, frame.get_right() + np.array([0.2, 0, 0]), "l")
                        row.add(attrs)
            for (row, frame, attrs, tone), node in zip(built, nodes):
                tree.chips[node["class"]] = row[:2]
                tree.rows[node["class"]] = row
                tree.add(row)
                parent = parents.get(node["class"])
                if parent is not None:
                    up = tree.chips[parent][0]
                    top = np.array([frame.get_center()[0] if leaf_row else up.get_left()[0] + 0.3, frame.get_top()[1], 0.0])
                    end = np.array([up.get_left()[0] + 0.3 if not leaf_row else up.get_left()[0] + 0.3 + 0.0, up.get_bottom()[1], 0.0])
                    path = [top, end] if not leaf_row else [top, np.array([top[0], (top[1] + end[1]) / 2, 0.0]), np.array([end[0], (top[1] + end[1]) / 2, 0.0]), end]
                    arrows.add(_engine.VGroup(_drafting.polyline(path, tone, W_THIN),
                                              _drafting.polygon([end, end + (-0.07, -0.13, 0), end + (0.07, -0.13, 0)], _art.CHROME_FILL, 1.0, tone, W_THIN)))
        tree.arrows = arrows
        tree.add_to_back(arrows)
        if expand is not None:
            chain = next(ch for ch in chains if ch[-1]["class"] == expand)
            inherited = [name for node in chain[:-1] for name in node["adds"]]
            own = chain[-1]["adds"]
            box = self.class_box(expand, [(name, MUTED) for name in inherited] + [(name, INK) for name in own], (0, 0, 0),
                                 color=DATA if expand in highlight else color, width=2.9)
            place(box, (tree.get_right()[0] + 0.3, 0.2), "ul")
            tree.box = box
            tree.box_rows = {name: label for name, label in zip(inherited + own, box.rows)}
            tree.add(box)
        return place(tree, point, "c")

    def property_table(self, rows, point=(0, 0, 0), *, width: float = 6.8, title: str | None = None, basis: bool = True, size=S_SMALL, color=GOOD):
        """📋 Pset table. Rows are ``(name, value, unit, basis)`` tuples or dicts; numbers are formatted with the language rules, strings pass through.
        ``table.rows[i]`` holds ``name``, ``value``, ``basis`` parts; ``table.cell(i, "value")`` returns one."""
        normal = []
        for row in rows:
            if isinstance(row, dict):
                row = (row["name"], row["value"], row.get("unit", ""), row.get("basis", "sourced"))
            name, value, unit, kind = (list(row) + ["", "sourced"])[:4]
            text = value if isinstance(value, str) else self.quantity(value, unit, 2 if abs(value) < 100 else 1)
            if isinstance(value, str) and unit:
                text = value + _drafting.NARROW + unit
            normal.append((name, text, kind))
        pitch = 0.34
        height = len(normal) * pitch + 0.2 + (0.42 if title else 0)
        frame = frame_box(width, height, color)
        table = _engine.VGroup(frame)
        top = frame.get_top()[1] - 0.1
        if title:
            head = self.text(title, (frame.get_left()[0] + 0.14, top - 0.2, 0), size=S_LABEL, color=color, anchor="l", width=width - 0.3)
            table.add(head)
            table.add(segments([(P((frame.get_left()[0], top - 0.42)), P((frame.get_right()[0], top - 0.42)))], color, W_THIN))
            top -= 0.42
        table.rows, table.title = [], None
        basis_x = frame.get_right()[0] - 0.14
        basis_w = max((self.text(self.tr(*BASIS_NAMES[kind]), (0, 0, 0), size=size).width for kind in ("sourced", "assumed")), default=0.0) if basis else 0.0
        value_x = basis_x - (basis_w + 0.3 if basis else 0)
        for index, (name, text, kind) in enumerate(normal):
            y = top - pitch * (index + 0.5) - 0.0
            room = width - 0.5 - (basis_w + 0.3 if basis else 0.0) - 1.2
            name_label = self.text(name, (frame.get_left()[0] + 0.14, y - 0.08, 0), size=size, color=INK, anchor="lb", width=room)
            value_label = self.text(text, (value_x, y - 0.08, 0), size=size, color=DATA, anchor="rb", width=1.2)
            row = _engine.VGroup(name_label, value_label)
            row.name, row.value = name_label, value_label
            if basis:
                en, de = BASIS_NAMES[kind]
                tone = GOOD if kind == "sourced" else WARN
                tag = self.text(self.tr(en, de), (basis_x, y - 0.08, 0), size=size, color=tone, anchor="rb")
                row.add(tag)
                row.basis = tag
            table.rows.append(row)
            table.add(row)
        table.frame = frame
        table.cell = lambda index, column: getattr(table.rows[index], column)
        return place(table, point, "c")

    def step_panel(self, excerpt, point=(0, 0, 0), *, width: float = 6.6, wrap=None, highlight_roles=(), only=None, size=S_SMALL, color=MUTED):
        """⌨️ ISO 10303-21 excerpt in monospace; tokens coloured by role. Lines wrap after a comma and continue indented.
        ``excerpt`` is ``step_excerpt(...)`` or a list of lines; ``wrap`` is the column count (default: as many as fit ``width``; ``False`` never wraps);
        ``only`` keeps source lines by index, by entity name (``excerpt["entities"]``) or by role. ``panel.rows[i]`` are display rows (one ``Text`` each), ``panel.runs`` every coloured run,
        ``panel.token(match, role=None, nth=0)`` the glyph group of one token (``match`` is a substring of the source line or its index)."""
        prototype = self.mono("0" * 20, (0, 0, 0), size=size)
        advance, pitch = prototype.width / 20, prototype.height + 0.17
        pad = 0.16
        columns = 10 ** 6 if wrap is False else wrap if isinstance(wrap, int) and not isinstance(wrap, bool) else int((width - 2 * pad) / advance)
        source = list(excerpt["lines"] if isinstance(excerpt, dict) else excerpt)
        spans = list(excerpt.get("spans", [])) if isinstance(excerpt, dict) else []
        entities = excerpt.get("entities", {}) if isinstance(excerpt, dict) else {}
        keep = None if only is None else {number for number in range(len(source)) if any(
            item == number if isinstance(item, int) else entities.get(item) == number if item in entities
            else any(span["role"] == item and span["line"] == number for span in spans) for item in only)}
        displayed = []
        for number, line in enumerate(source):
            if keep is not None and number not in keep:
                continue
            roles = [None] * len(line)
            for span in spans:
                if span["line"] == number:
                    for column in range(span["start"], min(span["end"], len(line))):
                        roles[column] = span["role"]
            start, first = 0, True
            while start < len(line) or first:
                room = columns - (0 if first else 2)
                stop = len(line)
                if len(line) - start > room:
                    stop = start + room
                    cut = line.rfind(",", start, stop)
                    stop = cut + 1 if cut > start + room // 3 else stop
                displayed.append((number, ("" if first else "  ") + line[start:stop], [None] * (0 if first else 2) + roles[start:stop]))
                start, first = stop, False
                if not line:
                    break
        panel = _engine.VGroup()
        panel.runs, panel.rows = [], []
        frame = frame_box(width if wrap is not False else max(width, max((len(t) for _, t, _ in displayed), default=1) * advance + 2 * pad), len(displayed) * pitch + 0.24, MUTED)
        panel.frame = frame
        panel.add(frame)
        x0, y0 = frame.get_left()[0] + pad, frame.get_top()[1] - 0.12 - pitch / 2
        for row_index, (number, text, roles) in enumerate(displayed):
            body = text.lstrip(" ")
            indent = len(text) - len(body)
            roles = roles[indent:]
            if not body:
                panel.rows.append(_engine.VGroup())
                continue
            groups, begin = [], 0
            for index in range(1, len(body) + 1):
                if index == len(body) or roles[index] != roles[begin]:
                    groups.append((begin, index, roles[begin]))
                    begin = index
            t2c = {f"[{a}:{b}]": ROLE_COLORS[role] for a, b, role in groups if role in ROLE_COLORS}
            _fonts.register_fonts(_theme.FONT_MONO)
            row = _engine.Text(body, font=_theme.FONT_MONO, font_size=size, color=color, t2c=t2c, disable_ligatures=True)
            row.descent = _drafting.descent(size, True) if _drafting.DESCENDERS & set(body) else 0.0
            _drafting.place(row, (x0 + indent * advance, y0 - row_index * pitch - 0.07, 0), "lb")
            glyphs = list(row.submobjects)
            visible = [i for i, c in enumerate(body) if not c.isspace()]
            index_of = dict(zip(visible, range(len(glyphs)))) if len(glyphs) == len(visible) else {i: i for i in range(len(glyphs))}
            for a, b, role in groups:
                members = [glyphs[index_of[i]] for i in range(a, b) if i in index_of and index_of[i] < len(glyphs)]
                if not members:
                    continue
                run = _engine.VGroup(*members)
                run.role, run.source_line, run.source_text = role, number, source[number]
                if highlight_roles and role not in highlight_roles:
                    run.set_opacity(0.35)
                panel.runs.append(run)
            panel.rows.append(row)
            panel.add(row)

        def token(match, role=None, nth: int = 0):
            found = [run for run in panel.runs if (match in run.source_text if isinstance(match, str) else run.source_line == match) and (role is None or run.role == role)]
            return found[nth]

        panel.token, panel.advance, panel.pitch = token, advance, pitch
        return place(panel, point, "c")

    def model_card(self, model_id: str, point=(0, 0, 0), *, model=None, width: float = 3.9, thumbnail: bool = True, scale: float = 0.08):
        """🗂️ Card of one partial model: coloured name, author, revision badge, schema chip, element total and a discipline axon.
        Parts: ``card.header``, ``card.rev``, ``card.schema``, ``card.total``, ``card.thumbnail``, ``card.frame``."""
        info = _model.partial_model(model_id, model) if model is not None else _model.partial_model(model_id)
        tone = _drafting.DISCIPLINES[model_id]
        en, de = CARD_NAMES[model_id]
        name = info["name"][self.language()] if isinstance(info.get("name"), dict) else self.tr(en, de)
        height = 3.7 if thumbnail else 1.5
        frame = frame_box(width, height, tone)
        card = _engine.VGroup(frame)
        top = frame.get_top()[1]
        header = self.text(name, (frame.get_left()[0] + 0.16, top - 0.28, 0), size=S_BODY, color=tone, anchor="l", width=width * 0.6)
        rev = self.badge(f"rev {info['revision']}", (frame.get_right()[0] - 0.14, top - 0.28, 0), color=tone, anchor="r")
        author = self.text(str(info["author"]), (frame.get_left()[0] + 0.16, top - 0.64, 0), size=S_SMALL, color=INK, anchor="l", width=width - 0.3)
        schema = self.badge(info["schema_id"], (frame.get_left()[0] + 0.16, top - 0.98, 0), color=MUTED, filled=False, anchor="l")
        count = sum(info["counts_by_class"].values()) if "counts_by_class" in info else len(info["element_guids"])
        total = self.text(self.tr(f"{count} objects", f"{count} Objekte"), (frame.get_right()[0] - 0.14, top - 0.98, 0), size=S_SMALL, color=INK, anchor="r")
        card.add(header, rev, author, schema, total)
        card.header, card.rev, card.schema, card.total, card.frame, card.thumbnail, card.info = header, rev, schema, total, frame, None, info
        if thumbnail:
            thumb = self.villa_axon((frame.get_center()[0], frame.get_bottom()[1] + 1.15, 0), scale, model=model, disciplines=(model_id,))
            card.thumbnail = thumb
            card.add(thumb)
        return place(card, point, "c")

    def guid_chip(self, key: str, point=(0, 0, 0), *, color=DATA, model=None, anchor="c", label: bool = True):
        """🆔 Chip showing the 22-character IFC GlobalId of an element in monospace; ``chip.id`` is the id text."""
        guid = _model.global_id(key) if model is None else _model.element(key, model)["guid"]
        ident = self.mono(guid, (0, 0, 0), size=S_SMALL, color=color)
        head = self.text("GlobalId" + (f"  {key}" if label else ""), (0, 0, 0), size=S_SMALL, color=MUTED, anchor="l")
        frame = frame_box(max(ident.width, head.width) + 0.32, 0.72, color)
        chip = _engine.VGroup(frame, head.move_to(frame.get_corner(_engine.UL) + np.array([0.14, -0.2, 0]), aligned_edge=_engine.LEFT),
                              ident.move_to(frame.get_corner(_engine.DL) + np.array([0.16 + ident.width / 2, 0.22, 0])))
        chip.frame, chip.id, chip.head, chip.guid = frame, ident, head, guid
        return place(chip, point, anchor)
