"""🧍 Guard: course clips draw people, furniture, objects and devices only from handcrafted staffage.

The forbidden names, the scanned tree and the forbidden paths live in ``🔣️staffage_only.json`` so any language can run the same scan.
"""

import ast
from functools import cache
import importlib
import json
import re
from pathlib import Path
import unittest

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️staffage_only.json").read_text(encoding="utf-8"))
ROOT = next(p for p in Path(__file__).resolve().parents if (p / CASES["root"]).is_dir()) / CASES["root"]


@cache
def clip_sources():
    """📜 Every clip script under the teaching root, outside tests and tool folders."""
    return sorted(
        path for path in ROOT.rglob(CASES["file_name"])
        if CASES["clip_folder"] in path.parts and not set(CASES["skip_folders"]) & set(path.parts)
    )


def ast_names(source):
    """🌳 Every identifier, attribute and imported name the parser sees in ``source``."""
    names = set()
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.Name):
            names.add(node.id)
        elif isinstance(node, ast.Attribute):
            names.add(node.attr)
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names.add(node.name)
        elif isinstance(node, ast.alias):
            names.update({node.name, node.asname or node.name})
    return names


def cast_names(source):
    """🔎 Every literal cast name a staffage call receives as first argument or as ``on_cast``/``off_cast``."""
    names = []
    for node in ast.walk(ast.parse(source)):
        if not isinstance(node, ast.Call):
            continue
        callee = node.func.id if isinstance(node.func, ast.Name) else node.func.attr if isinstance(node.func, ast.Attribute) else None
        arguments = list(node.args[:1]) if callee in CASES["cast_calls"] else []
        arguments += [keyword.value for keyword in node.keywords if keyword.arg in ("on_cast", "off_cast")]
        names += [argument.value for argument in arguments if isinstance(argument, ast.Constant) and isinstance(argument.value, str)]
    return names


def manifest_casts():
    """🗂️ The cast manifest the clips draw from."""
    return json.loads((Path(V.__file__).parent / CASES["manifest"]).read_text(encoding="utf-8"))["casts"]


def clip_key(path):
    """🔑 Path of a clip script below the architecture folder, the key allowlists use."""
    return path.relative_to(ROOT / CASES["rule_a"]["scope"]).as_posix()


def called_names(node):
    """📞 Names of every function and method called anywhere inside ``node``."""
    return {
        call.func.id if isinstance(call.func, ast.Name) else call.func.attr
        for call in ast.walk(node)
        if isinstance(call, ast.Call) and isinstance(call.func, (ast.Name, ast.Attribute))
    }


def head_nouns(slug, stop_words):
    """🏷️ Words of a catalog slug that name a thing: every ``-`` part outside the stop list."""
    return [part for part in slug.split("-") if part not in stop_words]


def catalog_slugs():
    """🔖 Slug of every catalog asset folder."""
    spec = CASES["rule_a"]["object_nouns"]
    return sorted({path.name for path in (ROOT / spec["catalog"]).glob(spec["slug_depth"]) if path.is_dir()})


def derived_nouns():
    """🧬 Object nouns derived from the catalog slugs."""
    stop = set(CASES["rule_a"]["object_nouns"]["stop_words"])
    return {noun for slug in catalog_slugs() for noun in head_nouns(slug, stop)}


def object_noun_pattern():
    """🔤 Regex of every object noun: the derived ones plus the extra nouns that have no asset yet."""
    nouns = sorted(derived_nouns() | set(CASES["rule_a"]["object_nouns"]["extra_nouns"]))
    return re.compile(r"(?:^|_)(?:" + "|".join(map(re.escape, nouns)) + r")s?(?:_|$)")


def primitive_objects(source):
    """🧱 Object factories built from primitives or physical pictograms without rendered staffage, as (name, line)."""
    rule = {**CASES["rule_a"], **CASES["rule_c"]}
    noun = object_noun_pattern()
    rendered = set(rule["staffage_calls"]) - set(rule["metadata_calls"])
    found = []
    for node in ast.walk(ast.parse(source)):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) or not noun.search(node.name):
            continue
        units = composition_units(node, rule)
        returns = [unit for unit in units if unit.startswith("<return@")]
        def calls_from(unit):
            calls, seen, pending = set(), {unit}, [unit]
            while pending:
                for expression in units[pending.pop()]:
                    calls |= called_names(expression)
                    refs = {target_key(item) for item in ast.walk(expression) if isinstance(item, (ast.Name, ast.Attribute))}
                    new = refs & units.keys() - seen
                    pending.extend(new)
                    seen |= new
            return calls
        call_sets = map(calls_from, returns) if returns else [called_names(node)]
        primitive = any(calls & set(rule["primitives"]) and not calls & rendered for calls in call_sets)
        pictogram = any(
            isinstance(call, ast.Call) and call_name(call) == "icon_glyph" and call.args
            and isinstance(call.args[0], ast.Constant) and call.args[0].value in rule["physical_icons"]
            for call in ast.walk(node)
        )
        if primitive or pictogram:
            found.append((node.name, node.lineno))
    return found


def physical_pictograms(source):
    """🏭 Lines that render a physical-object icon, including module-level icon maps."""
    tree = ast.parse(source)
    constants = string_constants(tree)
    mappings = {
        target.id: {key.value: value.value for key, value in zip(node.value.keys, node.value.values)
                    if isinstance(key, ast.Constant) and isinstance(value, ast.Constant) and isinstance(value.value, str)}
        for node in tree.body if isinstance(node, ast.Assign) and isinstance(node.value, ast.Dict)
        for target in node.targets if isinstance(target, ast.Name)
    }
    found = []
    for call in (node for node in ast.walk(tree) if isinstance(node, ast.Call) and call_name(node) == "icon_glyph"):
        arg = call.args[0] if call.args else next((keyword.value for keyword in call.keywords if keyword.arg == "kind"), None)
        if isinstance(arg, ast.Constant) and isinstance(arg.value, str):
            values = {arg.value}
        elif isinstance(arg, ast.Name):
            values = {constants.get(arg.id)}
        elif isinstance(arg, ast.Subscript) and isinstance(arg.value, ast.Name):
            mapping = mappings.get(arg.value.id, {})
            values = {mapping.get(arg.slice.value)} if isinstance(arg.slice, ast.Constant) else set(mapping.values())
        else:
            values = set()
        if values & set(CASES["rule_a"]["physical_icons"]):
            found.append(call.lineno)
    return sorted(found)


def call_name(call):
    """🪪 Name of the function or method a call invokes, ``None`` for anything else."""
    return call.func.id if isinstance(call.func, ast.Name) else call.func.attr if isinstance(call.func, ast.Attribute) else None


def target_key(node):
    """🎯 Dotted name of an assignment or ``add`` target, ``None`` when it is no plain name."""
    if isinstance(node, ast.Name):
        return node.id
    if isinstance(node, ast.Attribute):
        base = target_key(node.value)
        return base and f"{base}.{node.attr}"
    return target_key(node.value) if isinstance(node, ast.Subscript) else None


def function_defs(tree):
    """🗃️ Every function with its qualified name (``Class.method``), nested functions included."""
    def visit(node, prefix):
        for child in ast.iter_child_nodes(node):
            if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)):
                yield f"{prefix}{child.name}", child
                yield from visit(child, f"{prefix}{child.name}.")
            elif isinstance(child, ast.ClassDef):
                yield from visit(child, f"{prefix}{child.name}.")
            else:
                yield from visit(child, prefix)
    return list(visit(tree, ""))


def composition_units(function, rule):
    """🧩 What a function composes, as ``{unit: [expressions]}``: every variable with what is assigned or added to it, plus each ``VGroup``, ``return`` and ``self.add``."""
    units, stack = {}, list(function.body)
    while stack:
        node = stack.pop()
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            continue
        stack.extend(ast.iter_child_nodes(node))
        if isinstance(node, ast.Assign):
            targets = [element for target in node.targets for element in (target.elts if isinstance(target, (ast.Tuple, ast.List)) else [target])]
            for target in filter(target_key, targets):
                units.setdefault(target_key(target), []).append(node.value)
        elif isinstance(node, (ast.AugAssign, ast.AnnAssign)) and node.value is not None and target_key(node.target):
            units.setdefault(target_key(node.target), []).append(node.value)
        elif isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and node.value.func.attr in rule["collect_methods"]:
            owner = target_key(node.value.func.value)
            units.setdefault(f"<add@{node.lineno}>" if owner in (None, "self") else owner, []).append(ast.Tuple(elts=node.value.args, ctx=ast.Load()))
        elif isinstance(node, ast.Call) and call_name(node) in rule["group_calls"]:
            units.setdefault(f"<group@{node.lineno}:{node.col_offset}>", []).append(node)
        elif isinstance(node, ast.Return) and node.value is not None:
            units.setdefault(f"<return@{node.lineno}>", []).append(node.value)
    return units


def composed_objects(source):
    """🪢 Functions that compose three or more distinct primitives into one group without staffage and outside charts, tables and axes, as qualified names."""
    rule, found = {**CASES["rule_a"], **CASES["rule_c"]}, set()
    for qualified, function in function_defs(ast.parse(source)):
        units = composition_units(function, rule)

        def facts(expression):
            names = {call_name(node) for node in ast.walk(expression) if isinstance(node, ast.Call)}
            return names, {target_key(node) for node in ast.walk(expression) if isinstance(node, (ast.Name, ast.Attribute))} - {None}

        def closure(unit):
            names, seen, pending = set(), {unit}, [unit]
            while pending:
                for expression in units[pending.pop()]:
                    called, refs = facts(expression)
                    names |= called
                    pending += list(refs & units.keys() - seen)
                    seen |= refs & units.keys()
            return names

        flagged = {
            unit: names for unit in units
            for names in [closure(unit)]
            if len(names & set(rule["primitives"])) >= rule["minimum_primitives"] and not names & set(rule["staffage_calls"]) and not names & set(rule["context_calls"])
        }
        if any(not any(ref in flagged and ref != unit for expression in units[unit] for ref in facts(expression)[1]) for unit in flagged):
            found.add(qualified)
    return sorted(found)


def string_constants(tree):
    """📌 Module-level names bound to a plain string."""
    return {
        target.id: node.value.value
        for node in tree.body if isinstance(node, ast.Assign) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str)
        for target in node.targets if isinstance(target, ast.Name)
    }


def string_parts(node, constants):
    """🧵 Leading text of a string expression as (text, complete): constants, module constants, ``+`` chains and f-strings; the rest is dynamic."""
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value, True
    if isinstance(node, ast.Name) and node.id in constants:
        return constants[node.id], True
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Add):
        left, complete = string_parts(node.left, constants)
        if not complete:
            return left, False
        right, right_complete = string_parts(node.right, constants)
        return left + right, right_complete
    if isinstance(node, ast.JoinedStr):
        text = ""
        for part in node.values:
            piece, complete = string_parts(part.value if isinstance(part, ast.FormattedValue) else part, constants)
            text += piece
            if not complete:
                return text, False
        return text, True
    return "", False


def resolved_casts(source, manifest, scope=None):
    """🔬 Manifest ids a source (or one syntax node of it) names: literals, names of string constants, and every id behind a constant prefix."""
    tree = ast.parse(source)
    constants = string_constants(tree)
    found = set()
    for call in (node for node in ast.walk(scope or tree) if isinstance(node, ast.Call)):
        for argument in [*call.args, *(keyword.value for keyword in call.keywords)]:
            text, complete = string_parts(argument, constants)
            if complete and text in manifest:
                found.add(text)
            elif not complete and text.endswith("-"):
                found.update(name for name in manifest if name.startswith(text))
    return found


def non_thermal_violations(source, manifest, allowed):
    """🌡️ Casts a source draws that are not in the thermal view and not allowlisted furniture."""
    return sorted(name for name in resolved_casts(source, manifest) if manifest[name]["view"] != "thermal" and name not in allowed)


def scene_spans(source, manifest):
    """🔭 The distinct thermal spans (``None`` = absolute range) of the thermal casts each scene class of a source draws (module-level code is one more group); what is on screen together is one scene, so more than one span in a group is a mismatch."""
    tree = ast.parse(source)
    functions = {node.name: node for node in tree.body if isinstance(node, ast.FunctionDef)}

    def reached(node, seen):
        for name in {child.id for child in ast.walk(node) if isinstance(child, ast.Name)} & functions.keys() - seen:
            seen.add(name)
            reached(functions[name], seen)
        return seen

    classes = [node for node in tree.body if isinstance(node, ast.ClassDef)]
    used = set().union(*(reached(node, set()) for node in classes)) if classes else set()
    rest = ast.Module(body=[node for node in tree.body if not isinstance(node, ast.ClassDef) and getattr(node, "name", None) not in used], type_ignores=[])
    groups = [*(ast.Module(body=[node, *(functions[name] for name in sorted(reached(node, set())))], type_ignores=[]) for node in classes), rest]
    spans = [{manifest[name].get("thermalSpanK") for name in resolved_casts(source, manifest, group) if manifest[name]["view"] == "thermal"} for group in groups]
    return [sorted(group, key=lambda span: (span is not None, span)) for group in spans if group]


def projection_mismatches(scene, views):
    """📐 Casts whose viewpoint cannot be drawn in a scene of the declared projection: a plan scene takes plan casts only, a vertical scene (section, elevation or diagram) takes elevation and front casts."""
    rule = CASES["scene_projections"]
    allowed = rule["plans"] if scene == "plan" else rule["vertical"]
    return sorted(name for name, view in views.items() if view not in allowed)


def mirror_calls(source):
    """🪞 Source text of every ``mirror=`` argument that is not the constant ``False`` and not a parameter of the enclosing function."""
    found = []

    def visit(node, parameters):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            parameters = parameters | {argument.arg for argument in [*node.args.args, *node.args.kwonlyargs]}
        if isinstance(node, ast.Call):
            for keyword in node.keywords:
                value = keyword.value
                if keyword.arg == "mirror" and not (isinstance(value, ast.Constant) and value.value is False) and not (isinstance(value, ast.Name) and value.id in parameters):
                    found.append(f"mirror={ast.unparse(value)}")
        for child in ast.iter_child_nodes(node):
            visit(child, parameters)

    visit(ast.parse(source), frozenset())
    return found


def cast_method_functions(source, methods):
    """📏 Qualified names of functions that call one of ``methods`` on a staffage figure, directly or through a variable bound to one."""
    calls = set(CASES["rule_a"]["staffage_calls"])

    def draws(expression):
        return any(isinstance(node, ast.Call) and call_name(node) in calls for node in ast.walk(expression))

    found = []
    for qualified, function in function_defs(ast.parse(source)):
        bound = {
            target.id for node in ast.walk(function) if isinstance(node, ast.Assign) and draws(node.value)
            for target in node.targets if isinstance(target, ast.Name)
        }
        if any(
            isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr in methods
            and (draws(node.func.value) or any(isinstance(inner, ast.Name) and inner.id in bound for inner in ast.walk(node.func.value)))
            for node in ast.walk(function)
        ):
            found.append(qualified)
    return found


def stretched_cast_functions(source):
    """📏 Functions that stretch a staffage figure: a cast keeps its real proportions."""
    return cast_method_functions(source, CASES["cast_rules"]["stretch_methods"])


def rotated_cast_functions(source):
    """🧭 Functions that rotate a staffage figure about its anchor: in a plan scene that is the physical yaw of the thing."""
    return cast_method_functions(source, CASES["cast_rules"]["rotate_methods"])


def composed_cast_ids(source):
    """🧵 Source text of every staffage call whose first argument builds the cast id at run time (an f-string or a concatenation), which ``casts_in`` cannot key."""
    calls = set(CASES["rule_a"]["staffage_calls"])
    return [
        ast.unparse(node.args[0]) for node in ast.walk(ast.parse(source))
        if isinstance(node, ast.Call) and call_name(node) in calls and node.args and isinstance(node.args[0], (ast.JoinedStr, ast.BinOp))
    ]


def literal_units(source):
    """🔢 Source text of every ``unit=`` of a staffage call that holds a numeric literal and derives from no cast height, cast width or room metre."""
    rule = CASES["cast_rules"]["unit_rule"]
    found = []
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.Call) and call_name(node) in rule["calls"]:
            for keyword in node.keywords:
                inner = list(ast.walk(keyword.value))
                has_number = any(isinstance(item, ast.Constant) and isinstance(item.value, (int, float)) for item in inner)
                if keyword.arg == "unit" and has_number and not any(isinstance(item, ast.Call) and call_name(item) in rule["derived_from"] for item in inner):
                    found.append(ast.unparse(keyword.value))
    return found


@cache
def energy_clip_sources():
    """⚡️ Clip scripts of the energy course."""
    return [path for path in clip_sources() if CASES["rule_b"]["scope"] in path.parts]


class StaffageOnlyTest(unittest.TestCase):
    def test_mirror_calls_are_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_rules"]["mirror_call_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(mirror_calls(source), expected)

    def test_stretching_a_cast_is_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_rules"]["stretch_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(stretched_cast_functions(source), expected)

    def test_a_rotated_cast_is_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_rules"]["rotate_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(rotated_cast_functions(source), expected)

    def test_every_rotated_cast_is_a_documented_physical_yaw(self):
        rule, seen = CASES["cast_rules"]["plan_yaws"], set()
        for path in clip_sources():
            found = rotated_cast_functions(path.read_text(encoding="utf-8"))
            if not found:
                continue
            key = clip_key(path)
            seen.add(key)
            with self.subTest(path=key):
                self.assertEqual(sorted(found), sorted(rule.get(key, {})))
                self.assertTrue(all(reason.strip() for reason in rule[key].values()))
        self.assertEqual(set(rule) - seen, set())

    def test_composed_cast_ids_are_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_rules"]["composed_id_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(composed_cast_ids(source), expected)

    def test_no_clip_builds_a_cast_id_at_run_time(self):
        for path in clip_sources():
            with self.subTest(path=clip_key(path)):
                self.assertEqual(composed_cast_ids(path.read_text(encoding="utf-8")), [])

    def test_literal_units_are_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_rules"]["unit_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(literal_units(source), expected)

    def test_no_clip_sizes_a_cast_by_a_numeric_literal(self):
        for path in clip_sources():
            with self.subTest(path=clip_key(path)):
                self.assertEqual(literal_units(path.read_text(encoding="utf-8")), [])

    def test_no_clip_stretches_a_cast(self):
        for path in clip_sources():
            with self.subTest(path=clip_key(path)):
                self.assertEqual(stretched_cast_functions(path.read_text(encoding="utf-8")), [])

    def test_every_mirror_of_a_cast_is_named_and_justified(self):
        rule, manifest = CASES["cast_rules"], manifest_casts()
        for name, entry in manifest.items():
            with self.subTest(cast=name):
                self.assertEqual(bool(entry.get("mirror")), name.endswith(rule["mirror_suffix"]))
                if entry.get("mirror"):
                    self.assertTrue(rule["mirrored_assets"].get(entry["asset"], "").strip())
        for asset, reason in rule["mirrored_assets"].items():
            with self.subTest(asset=asset):
                self.assertTrue(reason.strip())
                self.assertTrue(any(entry["asset"] == asset and entry.get("mirror") for entry in manifest.values()))

    def test_every_call_site_mirror_is_declared_with_a_reason(self):
        rule, manifest = CASES["cast_rules"], manifest_casts()
        seen = set()
        for path in clip_sources():
            source = path.read_text(encoding="utf-8")
            if not mirror_calls(source):
                continue
            key = clip_key(path)
            seen.add(key)
            with self.subTest(path=key):
                declared = rule["mirrored_calls"].get(key)
                self.assertIsNotNone(declared)
                assets = {manifest[node.value]["asset"] for node in ast.walk(ast.parse(source)) if isinstance(node, ast.Constant) and isinstance(node.value, str) and node.value in manifest}
                for asset, reason in declared.items():
                    self.assertIn(asset, assets)
                    self.assertTrue(reason.strip())
        self.assertEqual(set(rule["mirrored_calls"]) - seen, set())

    def test_projection_guard_flags_a_cast_viewpoint_the_scene_projection_cannot_show(self):
        for case in CASES["scene_projections"]["fixture"]:
            with self.subTest(scene=case["scene"]):
                self.assertEqual(projection_mismatches(case["scene"], case["views"]), case["wrong"])

    def test_every_clip_declares_its_projection_and_its_casts_match_it(self):
        manifest, rule = manifest_casts(), CASES["scene_projections"]
        declared = set(rule["clips"])
        for path in clip_sources():
            casts = resolved_casts(path.read_text(encoding="utf-8"), manifest)
            if not casts:
                continue
            with self.subTest(path=clip_key(path)):
                self.assertIn(clip_key(path), declared)
                self.assertEqual(projection_mismatches(rule["clips"][clip_key(path)], {name: manifest[name]["viewpoint"] for name in casts}), [])
        for key in declared:
            with self.subTest(stale=key):
                self.assertTrue((ROOT / CASES["rule_a"]["scope"] / key).is_file())
                self.assertIn(rule["clips"][key], ("plan", "vertical"))

    def test_every_cast_names_its_viewpoint_and_mirrors_only_elevations(self):
        rule = CASES["scene_projections"]["mirror_fixture"]
        for name, entry in manifest_casts().items():
            with self.subTest(cast=name):
                self.assertIn(entry.get("viewpoint"), CASES["scene_projections"]["plans"] + CASES["scene_projections"]["vertical"])
                if entry.get("mirror"):
                    self.assertEqual(entry["viewpoint"], rule["ok"]["viewpoint"])
        for entry in rule["wrong"]:
            self.assertNotEqual(entry["viewpoint"], rule["ok"]["viewpoint"])

    def test_rule_a_flags_primitive_built_objects_and_passes_staffage_built_ones(self):
        for source, expected in CASES["rule_a"]["sources"].items():
            with self.subTest(source=source):
                self.assertEqual([name for name, _ in primitive_objects(source)], expected)

    def test_physical_pictograms_are_found_the_way_the_fixture_says(self):
        for source, expected in CASES["rule_a"]["pictogram_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(physical_pictograms(source), expected)

    def test_no_clip_renders_physical_pictograms(self):
        for path in clip_sources():
            with self.subTest(path=clip_key(path)):
                self.assertEqual(physical_pictograms(path.read_text(encoding="utf-8")), [])

    def test_rule_a_nouns_are_derived_from_the_catalog_slugs(self):
        spec = CASES["rule_a"]["object_nouns"]
        for slug, expected in CASES["rule_a"]["head_nouns"].items():
            with self.subTest(slug=slug):
                self.assertIn(slug, catalog_slugs())
                self.assertEqual(head_nouns(slug, set(spec["stop_words"])), expected)
                self.assertTrue(set(expected) <= derived_nouns())

    def test_rule_a_extra_nouns_have_no_asset_yet_and_stop_words_are_not_nouns(self):
        spec = CASES["rule_a"]["object_nouns"]
        every_part = {part for slug in catalog_slugs() for part in slug.split("-")}
        for noun in spec["extra_nouns"]:
            with self.subTest(extra=noun):
                self.assertNotIn(noun, derived_nouns())
        for word in spec["stop_words"]:
            with self.subTest(stop=word):
                self.assertIn(word, every_part)

    def test_rule_a_no_clip_builds_a_physical_object_from_primitives(self):
        allowed = CASES["rule_a"]["allowed"]
        for path in clip_sources():
            if CASES["rule_a"]["scope"] not in path.parts:
                continue
            for name, line in primitive_objects(path.read_text(encoding="utf-8")):
                with self.subTest(path=clip_key(path), function=name, line=line):
                    self.assertIn(f"{clip_key(path)}::{name}", allowed)

    def test_rule_a_allowlist_has_a_reason_and_no_stale_entries(self):
        found = {
            f"{clip_key(path)}::{name}"
            for path in clip_sources() if CASES["rule_a"]["scope"] in path.parts
            for name, _ in primitive_objects(path.read_text(encoding="utf-8"))
        }
        for entry, reason in CASES["rule_a"]["allowed"].items():
            with self.subTest(entry=entry):
                self.assertTrue(reason.strip())
                self.assertIn(entry, found)

    def test_rule_c_flags_three_primitives_composed_without_staffage_chart_or_table(self):
        for source, expected in CASES["rule_c"]["sources"].items():
            with self.subTest(source=source):
                self.assertEqual(composed_objects(source), expected)

    def test_rule_c_no_clip_composes_an_object_from_primitives(self):
        allowed = CASES["rule_c"]["allowed"]
        for path in clip_sources():
            if CASES["rule_a"]["scope"] not in path.parts:
                continue
            for name in composed_objects(path.read_text(encoding="utf-8")):
                with self.subTest(path=clip_key(path), function=name):
                    self.assertIn(f"{clip_key(path)}::{name}", allowed)

    def test_rule_c_allowlist_has_a_reason_and_no_stale_entries(self):
        found = {
            f"{clip_key(path)}::{name}"
            for path in clip_sources() if CASES["rule_a"]["scope"] in path.parts
            for name in composed_objects(path.read_text(encoding="utf-8"))
        }
        for entry, reason in CASES["rule_c"]["allowed"].items():
            with self.subTest(entry=entry):
                self.assertTrue(reason.strip())
                self.assertIn(entry, found)

    def test_rule_b_resolves_literals_constants_and_dynamic_prefixes(self):
        manifest = CASES["rule_b"]["manifest"]
        for source, expected in CASES["rule_b"]["resolve"].items():
            with self.subTest(source=source):
                self.assertEqual(sorted(resolved_casts(source, manifest)), expected)

    def test_rule_b_flags_non_thermal_casts_outside_the_furniture_allowlist(self):
        manifest, allowed = CASES["rule_b"]["manifest"], CASES["rule_b"]["vector_allowed"]
        for source, expected in CASES["rule_b"]["sources"].items():
            with self.subTest(source=source):
                self.assertEqual(non_thermal_violations(source, manifest, allowed), expected)

    def test_rule_b_every_energy_cast_is_thermal_or_allowlisted_furniture(self):
        manifest, allowed = manifest_casts(), CASES["rule_b"]["allowed"]
        for path in energy_clip_sources():
            with self.subTest(path=clip_key(path)):
                self.assertEqual(non_thermal_violations(path.read_text(encoding="utf-8"), manifest, allowed), [])

    def test_thermal_span_lists_the_distinct_spans_of_a_source(self):
        rule = CASES["thermal_span"]
        for source, expected in rule["sources"].items():
            with self.subTest(source=source):
                self.assertEqual(scene_spans(source, rule["manifest"]), expected)

    def test_thermal_span_every_scene_draws_its_thermal_casts_on_one_span(self):
        manifest = manifest_casts()
        for path in energy_clip_sources():
            for spans in scene_spans(path.read_text(encoding="utf-8"), manifest):
                with self.subTest(path=clip_key(path), spans=spans):
                    self.assertLessEqual(len(spans), 1)

    def test_rule_b_allowlist_names_existing_non_thermal_casts_with_a_reason(self):
        manifest = manifest_casts()
        for name, reason in CASES["rule_b"]["allowed"].items():
            with self.subTest(cast=name):
                self.assertTrue(reason.strip())
                self.assertIn(name, manifest)
                self.assertNotEqual(manifest[name]["view"], "thermal")

    def test_rule_b_every_watt_cast_is_thermal(self):
        manifest = manifest_casts()
        for name in V._WATT_CASTS.values():
            with self.subTest(cast=name):
                self.assertEqual(manifest[name]["view"], "thermal")


    def test_the_guard_flags_every_violation_by_regex_and_by_syntax_tree(self):
        pattern = re.compile(r"\b(" + "|".join(map(re.escape, CASES["forbidden_names"])) + r")\b")
        for source in CASES["violations"]:
            with self.subTest(source=source):
                self.assertTrue(pattern.search(source))
                self.assertTrue(ast_names(source) & set(CASES["forbidden_names"]))

    def test_the_guard_passes_handcrafted_calls_and_ignores_comments_in_the_tree(self):
        for source in CASES["clean"]:
            with self.subTest(source=source):
                self.assertEqual(ast_names(source) & set(CASES["forbidden_names"]), set())

    def test_literal_cast_names_are_found_the_way_the_fixture_says(self):
        for source, expected in CASES["cast_sources"].items():
            with self.subTest(source=source):
                self.assertEqual(cast_names(source), expected)

    def test_every_literal_cast_a_clip_or_the_library_draws_is_in_the_manifest(self):
        manifest = json.loads((Path(V.__file__).parent / CASES["manifest"]).read_text(encoding="utf-8"))["casts"]
        sources = [*clip_sources(), Path(V.__file__)]
        used = {name: path for path in sources for name in cast_names(path.read_text(encoding="utf-8"))}
        self.assertTrue(used)
        for name, path in used.items():
            with self.subTest(cast=name, path=path.relative_to(ROOT).as_posix()):
                self.assertIn(name, manifest)

    def test_every_watt_cast_is_in_the_manifest(self):
        manifest = json.loads((Path(V.__file__).parent / CASES["manifest"]).read_text(encoding="utf-8"))["casts"]
        for kind, name in V._WATT_CASTS.items():
            with self.subTest(kind=kind):
                self.assertIn(name, manifest)

    def test_scan_covers_the_course(self):
        self.assertGreaterEqual(len(clip_sources()), CASES["minimum_scanned_files"])

    def test_no_clip_names_an_ad_hoc_staffage_helper(self):
        pattern = re.compile(r"\b(" + "|".join(map(re.escape, CASES["forbidden_names"])) + r")\b")
        for path in clip_sources():
            with self.subTest(path=path.relative_to(ROOT).as_posix()):
                source = path.read_text(encoding="utf-8")
                self.assertEqual(pattern.findall(source), [])
                self.assertEqual(ast_names(source) & set(CASES["forbidden_names"]), set())

    def test_visuals_module_no_longer_defines_the_helpers(self):
        for name in CASES["forbidden_names"]:
            with self.subTest(name=name):
                self.assertFalse(hasattr(V, name))

    def test_manifest_has_no_orphan_casts(self):
        manifest = json.loads((Path(V.__file__).parent / CASES["manifest"]).read_text(encoding="utf-8"))["casts"]
        for cast in CASES["forbidden_casts"]:
            with self.subTest(cast=cast):
                self.assertNotIn(cast, manifest)

    def test_ad_hoc_staffage_assets_are_gone(self):
        for folder in CASES["forbidden_paths"]:
            with self.subTest(folder=folder):
                found = [p for p in ROOT.rglob(folder) if not set(CASES["skip_folders"]) & set(p.parts)]
                self.assertEqual(found, [])


if __name__ == "__main__":
    unittest.main()
