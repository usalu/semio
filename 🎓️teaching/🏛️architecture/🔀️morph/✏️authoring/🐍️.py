"""✏️ Morph authoring: list, show, validate, edit and plan the declarative transition documents the player and the exporter consume.

Every command reads and writes the persisted `🔀️transitions/🔣️.json` files themselves, through the same `📋️spec` validator and canonical serialization as playback and export.
See [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901) for the `set` addressing.
"""

from __future__ import annotations

import argparse
import ast
import copy
import importlib
import json
import os
import re
import sys
import tempfile
from pathlib import Path
from typing import NamedTuple

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

sys.dont_write_bytecode = True

spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
profiles = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")

TEACHING = "🎓️teaching"
DOCUMENT = Path("🔀️transitions") / "🔣️.json"
CURRICULUM = Path("🏛️architecture") / "⚡️energy" / "🎬️clip" / "🔣️.json"
SKIPPED = frozenset({".git", "node_modules", "__pycache__", ".🧬semio", ".venv", "target"})
LANGUAGES = ("en", "de")
PURPOSES = ("review", "publication")
REQUEST_VERSION = 1

MESSAGES = {
    "language-required": ("--language en|de is required; no default language is assumed.", "--language en|de ist erforderlich; es wird keine Standardsprache angenommen."),
    "unknown-id": ("transition {id!r} is not declared in any transitions document; run `morph list` for the ids.", "Übergang {id!r} ist in keinem Übergangsdokument deklariert; `morph list` zeigt die Ids."),
    "duplicate-id": ("transition id {id!r} is declared in {first} and {second}; ids must be unique across the course.", "Übergangs-Id {id!r} steht in {first} und {second}; Ids müssen im ganzen Kurs eindeutig sein."),
    "format": ("file is not in canonical form (2-space JSON, stable key order, trailing newline); run `morph format <path>`.", "Datei ist nicht kanonisch formatiert (JSON mit 2 Leerzeichen, feste Schlüsselreihenfolge, abschliessender Zeilenumbruch); `morph format <Pfad>` ausführen."),
    "unresolved": ("scene {scene} in {script} plays transition {id!r}, which no transitions document of its clip folders declares; declare it in {where}.", "Szene {scene} in {script} spielt den Übergang {id!r}, den kein Übergangsdokument ihrer Clip-Ordner deklariert; in {where} deklarieren."),
    "script": ("{script} cannot be analysed: {reason}.", "{script} lässt sich nicht analysieren: {reason}."),
    "pointer": ("invalid JSON pointer {pointer!r}: {reason}; use /timing/duration or /source/view.", "ungültiger JSON-Pointer {pointer!r}: {reason}; z. B. /timing/duration oder /source/view verwenden."),
    "refused": ("edit refused, {path} is unchanged:", "Änderung abgelehnt, {path} bleibt unverändert:"),
    "written": ("wrote {path}", "geschrieben: {path}"),
    "unchanged": ("{path} already holds this value; nothing written", "{path} enthält diesen Wert bereits; nichts geschrieben"),
    "valid": ("{count} transitions document(s) valid", "{count} Übergangsdokument(e) gültig"),
    "unplayed": ("warning: transition {id!r} in {path} is not played by any curriculum scene", "Warnung: Übergang {id!r} in {path} wird von keiner Szene des Lehrplans gespielt"),
    "played-unknown": ("{file} records transition {id!r}, which no transitions document declares; the scene ran against a stale or foreign document.", "{file} verzeichnet den Übergang {id!r}, den kein Übergangsdokument deklariert; die Szene lief gegen ein veraltetes oder fremdes Dokument."),
    "played-duration": ("{file} records {id!r} with duration {recorded}s but the document declares {declared}s.", "{file} verzeichnet {id!r} mit Dauer {recorded}s, das Dokument deklariert {declared}s."),
    "profile": ("unknown profile {profile!r}; use one of {names}.", "unbekanntes Profil {profile!r}; erlaubt sind {names}."),
    "purpose": ("a publication request must use the final profile, got {profile!r}; preview output can never satisfy a final request.", "Eine Veröffentlichungsanfrage muss das Profil final verwenden, erhalten {profile!r}; Vorschauausgabe kann nie eine finale Anfrage erfüllen."),
    "bounds": ("bounds must be four numbers x0,y0,x1,y1, got {value!r}.", "Grenzen müssen vier Zahlen x0,y0,x1,y1 sein, erhalten {value!r}."),
    "bounds-pair": ("--source-bounds and --target-bounds must be given together.", "--source-bounds und --target-bounds müssen zusammen angegeben werden."),
    "fail": ("{reason}", "{reason}"),
}

HELP = {
    "description": ("Author the declarative morph transitions of the course; every command edits exactly the data playback and export consume.", "Die deklarativen Morph-Übergänge des Kurses bearbeiten; jeder Befehl ändert genau die Daten, die Wiedergabe und Export nutzen."),
    "root": ("workspace root (default: the repository containing this module)", "Arbeitsbereich-Wurzel (Standard: das Repository dieses Moduls)"),
    "list": ("list all transitions with id, intent, recipe and duration", "alle Übergänge mit Id, Absicht, Rezept und Dauer auflisten"),
    "show": ("print one transition as persisted", "einen Übergang wie gespeichert ausgeben"),
    "validate": ("validate transitions documents and curriculum references; exit 1 on any problem", "Übergangsdokumente und Lehrplanverweise prüfen; Exit 1 bei jedem Problem"),
    "format": ("rewrite valid documents in canonical form", "gültige Dokumente kanonisch neu schreiben"),
    "set": ("validated edit of one value addressed by a JSON pointer inside the transition; the value is JSON, otherwise a plain string", "validierte Änderung eines Werts per JSON-Pointer innerhalb des Übergangs; der Wert ist JSON, sonst eine einfache Zeichenkette"),
    "plan": ("print the resolved plan (frames, placement, level of detail) without rendering", "den aufgelösten Plan (Bilder, Platzierung, Detailstufe) ausgeben, ohne zu rendern"),
    "request": ("persist an export request with an explicit profile", "eine Exportanfrage mit explizitem Profil speichern"),
    "paths": ("files or folders; default: every transitions document in the repository", "Dateien oder Ordner; Standard: jedes Übergangsdokument im Repository"),
    "format_paths": ("files or folders to rewrite; required, a repository-wide rewrite is never implied", "umzuschreibende Dateien oder Ordner; erforderlich, ein Umschreiben des ganzen Repositorys wird nie angenommen"),
    "played": ("runtime record `transitions.json` written beside a scene's work.json; every recorded id and duration must match the documents", "Laufzeitprotokoll `transitions.json` neben work.json einer Szene; jede verzeichnete Id und Dauer muss zu den Dokumenten passen"),
    "id": ("transition id", "Übergangs-Id"),
    "pointer": ("JSON pointer inside the transition, e.g. /timing/duration", "JSON-Pointer im Übergang, z. B. /timing/duration"),
    "value": ("new value as JSON (\"smooth\", 1.5, null, [[\"a\",\"b\"]]) or plain string", "neuer Wert als JSON (\"smooth\", 1.5, null, [[\"a\",\"b\"]]) oder einfache Zeichenkette"),
    "profile": ("render profile; required, no default", "Render-Profil; erforderlich, kein Standard"),
    "fps": ("output frame rate (default: the profile's)", "Ausgabe-Bildrate (Standard: die des Profils)"),
    "start": ("transition start in scene seconds (default 0)", "Beginn des Übergangs in Szenensekunden (Standard 0)"),
    "source_bounds": ("logical source bounds x0,y0,x1,y1 for the placement affine", "logische Quellgrenzen x0,y0,x1,y1 für die Platzierungs-Affinität"),
    "target_bounds": ("logical target bounds x0,y0,x1,y1 for the placement affine", "logische Zielgrenzen x0,y0,x1,y1 für die Platzierungs-Affinität"),
    "frames": ("also print the owner and progress of every frame", "zusätzlich Besitzer und Fortschritt jedes Bildes ausgeben"),
    "json": ("machine-readable JSON output", "maschinenlesbare JSON-Ausgabe"),
    "ids": ("transition ids to export", "zu exportierende Übergangs-Ids"),
    "purpose": ("review (preview or final) or publication (final only)", "review (Vorschau oder final) oder publication (nur final)"),
    "output": ("request file to write", "zu schreibende Anfragedatei"),
}


class Plays(NamedTuple):
    """🎬 Static facts about the transitions one scene plays."""

    called: frozenset
    patterns: tuple
    constants: frozenset


class Fail(Exception):
    """⛔ A refused operation carrying the problems to show."""

    def __init__(self, problems):
        self.problems = tuple(problems)
        super().__init__("\n".join(self.problems))


def say(language, key, **values):
    """💬 One catalogue message in the explicit language."""
    return MESSAGES[key][LANGUAGES.index(language)].format(**values)


def problem(path, code, message):
    """🚩 A spec problem for findings outside the schema."""
    return spec.Problem(path, code, message)


def teaching(root):
    """🎓 The teaching folder of a workspace root."""
    return Path(root) / TEACHING


def discover(folder):
    """🔭 Every transitions document below a folder, sorted."""
    found = []
    for current, names, files in os.walk(folder):
        names[:] = sorted(name for name in names if name not in SKIPPED)
        if Path(current).name == DOCUMENT.parent.name and DOCUMENT.name in files:
            found.append(Path(current) / DOCUMENT.name)
    return sorted(found)


def expand(paths, root):
    """📂 Resolve file and folder arguments to transitions documents; no arguments mean the whole teaching folder."""
    if not paths:
        return discover(teaching(root))
    return [q for item in map(Path, paths) for q in (discover(item) if item.is_dir() else [item])]


def read(path):
    """📖 Raw JSON of a file; raise `Fail` with a located message."""
    try:
        return json.loads(Path(path).read_text(encoding="utf-8"), parse_constant=lambda name: (_ for _ in ()).throw(ValueError(f"{name} is not a finite number")))
    except OSError as error:
        raise Fail([f"{path}: $ [file] cannot read: {error.strerror or error}"]) from error
    except ValueError as error:
        raise Fail([f"{path}: $ [json] not valid JSON: {error}"]) from error


def write(path, text):
    """💾 Atomically replace a file with `text` (temporary sibling, flush, rename)."""
    path = Path(path)
    descriptor, temporary = tempfile.mkstemp(dir=path.parent, prefix=".morph-", suffix=".tmp")
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
    except BaseException:
        Path(temporary).unlink(missing_ok=True)
        raise


def label(path, root):
    """🏷 Display path relative to the root when possible, with forward slashes."""
    try:
        return Path(path).resolve().relative_to(Path(root).resolve()).as_posix()
    except ValueError:
        return Path(path).as_posix()


def load_all(paths):
    """🗃 Raw documents of all paths as `{path: raw}`; unreadable ones raise `Fail`."""
    return {path: read(path) for path in paths}


def duplicates(documents, language, root):
    """👯 One problem per id declared twice across documents."""
    seen, out = {}, []
    for path, raw in documents.items():
        for item in raw.get("transitions", ()) if isinstance(raw, dict) else ():
            ident = item.get("id") if isinstance(item, dict) else None
            if not isinstance(ident, str):
                continue
            if ident in seen and seen[ident] != path:
                out.append((path, problem(f"$.transitions[id={ident}]", "duplicate-id", say(language, "duplicate-id", id=ident, first=label(seen[ident], root), second=label(path, root)))))
            seen.setdefault(ident, path)
    return out


def scene_plays(script, scene):
    """🎭 What the class `scene` of a script plays, found statically without importing it: `called` ids and f-string `patterns` passed to `.transition`/`.cut`, plus every string `constants` of the class (ids and namespace prefixes handed to helpers); None when the class is absent."""
    tree = ast.parse(Path(script).read_text(encoding="utf-8"))
    node = next((n for n in ast.walk(tree) if isinstance(n, ast.ClassDef) and n.name == scene), None)
    if node is None:
        return None
    called, patterns, constants = set(), [], set()
    for item in ast.walk(node):
        if isinstance(item, ast.Constant) and isinstance(item.value, str):
            constants.add(item.value)
        if isinstance(item, ast.Call) and isinstance(item.func, ast.Attribute) and item.func.attr in ("transition", "cut") and item.args:
            first = item.args[0]
            if isinstance(first, ast.Constant) and isinstance(first.value, str):
                called.add(first.value)
            elif isinstance(first, ast.JoinedStr):
                patterns.append(re.compile("".join(re.escape(v.value) if isinstance(v, ast.Constant) else ".+" for v in first.values)))
    return Plays(frozenset(called), tuple(patterns), frozenset(constants))


def plays_id(plays, ident):
    """🎬 Whether the scene plays `ident`: named literally, matched by an f-string pattern, or reached through a namespace prefix constant handed to a helper."""
    return ident in plays.called or ident in plays.constants or any(p.fullmatch(ident) for p in plays.patterns) or any(ident.startswith(c + ".") for c in plays.constants if c.count(".") >= 2)


def unresolved(plays, declared):
    """❓ Sorted descriptions of called ids and patterns that no declared id satisfies."""
    return sorted({i for i in plays.called if i not in declared} | {p.pattern for p in plays.patterns if not any(p.fullmatch(i) for i in declared)})


def curriculum_segments(root, curriculum=None):
    """📜 `(script, scene)` of every curriculum segment, in order, de-duplicated."""
    path = Path(curriculum) if curriculum else teaching(root) / CURRICULUM
    if not path.is_file():
        return []
    videos = json.loads(path.read_text(encoding="utf-8"))["videos"]
    return list(dict.fromkeys((s["script"], s["scene"]) for v in videos for c in v.get("chapters", ()) for s in c.get("segments", ())))


def declared_for(script, root, documents):
    """📍 Ids declared by the documents of the script's folder and its ancestors up to the teaching folder."""
    top = teaching(root).resolve()
    ids, folder = set(), Path(script).resolve().parent
    while folder == top or top in folder.parents:
        document = (folder / DOCUMENT).resolve()
        raw = documents.get(document)
        if raw is not None:
            ids |= {t["id"] for t in raw.get("transitions", ()) if isinstance(t, dict) and isinstance(t.get("id"), str)}
        if folder == top:
            break
        folder = folder.parent
    return ids


def references(documents, root, language, curriculum=None):
    """🔗 Problems of scenes that call undeclared ids, plus the set of declared ids some scene plays."""
    base = teaching(root)
    resolved = {Path(p).resolve(): raw for p, raw in documents.items()}
    everything = {t["id"] for raw in documents.values() for t in raw.get("transitions", ()) if isinstance(t, dict) and isinstance(t.get("id"), str)}
    out, played = [], set()
    for script, scene in curriculum_segments(root, curriculum):
        file = base / script
        try:
            plays = scene_plays(file, scene) if file.is_file() else None
        except (SyntaxError, UnicodeDecodeError, OSError) as error:
            out.append((file, problem("$", "script", say(language, "script", script=script, reason=error))))
            continue
        if plays is None:
            continue
        declared = declared_for(file, root, resolved)
        played |= {i for i in everything if plays_id(plays, i)}
        out += [(file, problem(f"$.{scene}", "unresolved-transition", say(language, "unresolved", scene=scene, script=script, id=ident, where=label(file.parent / DOCUMENT, root)))) for ident in unresolved(plays, declared)]
    return out, played


def runtime(records, documents, language):
    """⏱ Problems of scene runtime records (`transitions.json` written by the stage) against the declared documents: unknown ids and durations that differ."""
    declared = {t["id"]: t for raw in documents.values() for t in raw.get("transitions", ()) if isinstance(t, dict) and "id" in t}
    out = []
    for path in map(Path, records):
        for item in read(path).get("transitions", ()):
            known = declared.get(item.get("id"))
            if known is None:
                out.append((path, problem("$.transitions", "played-unknown", say(language, "played-unknown", file=path.name, id=item.get("id")))))
            elif abs(float(item.get("duration", -1)) - float(known["timing"]["duration"])) > 1e-9:
                out.append((path, problem("$.transitions", "played-duration", say(language, "played-duration", file=path.name, id=item["id"], recorded=item.get("duration"), declared=known["timing"]["duration"]))))
    return out


def findings(documents, root, language, curriculum=None, check_format=True, whole=True):
    """🔍 Located problems of documents: schema/semantics, format, duplicate ids and curriculum references."""
    out = []
    for path, raw in documents.items():
        found = spec.validate(raw)
        out += [(path, p) for p in found]
        if not found and check_format and Path(path).is_file() and Path(path).read_text(encoding="utf-8") != spec.dumps(raw):
            out.append((path, problem("$", "format", say(language, "format"))))
    out += duplicates(documents, language, root)
    return out + (references(documents, root, language, curriculum)[0] if whole else [])


def show_findings(items, root, stream):
    """📣 Print `path: pointer [code] message` lines."""
    for path, item in items:
        print(f"{label(path, root)}: {item}", file=stream)


def index(documents):
    """🗂 `{id: (path, position)}` of every declared transition."""
    return {t["id"]: (path, i) for path, raw in documents.items() for i, t in enumerate(raw.get("transitions", ())) if isinstance(t, dict) and "id" in t}


def find(documents, ident, language):
    """🎯 `(path, position)` of a transition id or `Fail`."""
    where = index(documents).get(ident)
    if where is None:
        raise Fail([say(language, "unknown-id", id=ident)])
    return where


def tokens(pointer, language):
    """✂ Unescaped reference tokens of a non-empty JSON pointer."""
    if not pointer.startswith("/") or pointer == "/":
        raise Fail([say(language, "pointer", pointer=pointer, reason="must start with / and name at least one key")])
    return [t.replace("~1", "/").replace("~0", "~") for t in pointer[1:].split("/")]


def assign(transition, pointer, value, language):
    """✍ Set `value` at `pointer` inside a copy of a transition; list indexes must exist (`-` appends)."""
    keys, node = tokens(pointer, language), transition
    for key in keys[:-1]:
        node = step(node, key, pointer, language)
    last = keys[-1]
    if isinstance(node, dict):
        node[last] = value
    elif isinstance(node, list):
        if last == "-":
            node.append(value)
        else:
            node[index_of(node, last, pointer, language)] = value
    else:
        raise Fail([say(language, "pointer", pointer=pointer, reason="the parent is a plain value")])


def index_of(node, key, pointer, language):
    """🔢 A valid list position from a pointer token."""
    if not key.isdigit() or int(key) >= len(node):
        raise Fail([say(language, "pointer", pointer=pointer, reason=f"list index {key!r} is outside 0..{len(node) - 1}")])
    return int(key)


def step(node, key, pointer, language):
    """👣 Descend one pointer token."""
    if isinstance(node, dict) and key in node:
        return node[key]
    if isinstance(node, list):
        return node[index_of(node, key, pointer, language)]
    raise Fail([say(language, "pointer", pointer=pointer, reason=f"{key!r} does not exist")])


def coerce(text):
    """🔤 JSON value of a command-line text, or the text itself."""
    try:
        return json.loads(text)
    except ValueError:
        return text


def edit(documents, ident, pointer, value, root, language, curriculum=None):
    """🧪 Apply an edit to copies; return `(path, text)` or raise `Fail` listing every new problem."""
    path, position = find(documents, ident, language)
    updated = {**documents, path: copy.deepcopy(documents[path])}
    assign(updated[path]["transitions"][position], pointer, value, language)
    before = {(str(p), q.code, q.path) for p, q in findings(documents, root, language, curriculum, check_format=False)}
    after = [(p, q) for p, q in findings(updated, root, language, curriculum, check_format=False) if (str(p), q.code, q.path) not in before]
    if after:
        raise Fail([say(language, "refused", path=label(path, root))] + [f"  {label(p, root)}: {q}" for p, q in after])
    return path, spec.dumps(updated[path])


def window_of(transition, start):
    """🪟 Timeline window of a transition."""
    timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
    return timeline, timeline.Window(start, transition.timing.duration)


def parse_bounds(text, language):
    """📦 Placement bounds from `x0,y0,x1,y1`."""
    placement = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📍️placement.🐍️")
    try:
        values = [float(v) for v in text.split(",")]
        if len(values) != 4:
            raise ValueError
        return placement.Bounds((values[0], values[1]), (values[2], values[3]))
    except (ValueError, placement.PlacementError) as error:
        raise Fail([say(language, "bounds", value=text)]) from error


def plan(transition, profile_name, language, *, fps=None, start=0.0, source_bounds=None, target_bounds=None, frames=False):
    """🗺 The resolved plan of one transition under a profile, computed without rendering."""
    if profile_name not in profiles.NAMES:
        raise Fail([say(language, "profile", profile=profile_name, names=list(profiles.NAMES))])
    chosen = profiles.profile(profile_name)
    rate = float(fps if fps is not None else chosen.fps)
    timeline, window = window_of(transition, start)
    schedule = timeline.Schedule(rate)
    span = timeline.frames(window, schedule)
    terminal = timeline.terminal(window, schedule)
    placed = transition.placement
    affine = None
    if (source_bounds is None) != (target_bounds is None):
        raise Fail([say(language, "bounds-pair")])
    if source_bounds is not None:
        placement = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📍️placement.🐍️")
        try:
            result = placement.affine(parse_bounds(source_bounds, language), parse_bounds(target_bounds, language), placed)
        except placement.PlacementError as error:
            raise Fail([str(error)]) from error
        affine = {"scale": list(result.scale), "translation": list(result.translation)}
    first = timeline.first(window, schedule)
    rows = [{"frame": f, "owner": (s := timeline.sample(window, schedule, f)).owner, "progress": s.progress} for f in (range(span[0], span[1] + 1) if span else [first])] if frames else None

    def endpoint(e):
        return {"entity": e.entity, "lod": chosen.lod, "revision": e.revision, "sampling": e.sampling, "view": e.view, "scale": e.scale, "pose": e.pose, "scenario": e.scenario}

    return {
        "id": transition.id, "profile": profile_name, "fingerprint": chosen.fingerprint, "morph": chosen.morph, "fps": rate, "start": start,
        "duration": transition.timing.duration, "easing": transition.timing.easing,
        "frames": {"count": timeline.count(window, schedule), "first": first, "terminal": terminal, "source": None if span is None or span[1] == span[0] else [span[0], span[1] - 1], "boundaryTime": timeline.boundary(window, schedule)},
        "work": {"fullMorph": chosen.morph == "full", "recipe": transition.recipe if chosen.morph == "full" else None},
        "placement": {"space": placed.space, "anchor": placed.anchor, "fit": placed.fit, "uniform": placed.uniform, "affine": affine},
        "lod": {"source": endpoint(transition.source), "target": endpoint(transition.target)},
        **({"samples": rows} if rows is not None else {}),
    }


def describe(result):
    """📝 Human-readable lines of a plan."""
    f, p = result["frames"], result["placement"]
    lines = [f"{result['id']}  profile={result['profile']} ({result['fingerprint']}) morph={result['morph']} fps={result['fps']:g}", f"duration={result['duration']}s easing={result['easing']} start={result['start']}s frames={f['count']}"]
    lines.append(f"source frames {f['source'][0]}..{f['source'][1]}, target from frame {f['terminal']} (boundary {f['boundaryTime']:.6g}s)" if f["source"] else f"target at frame {f['terminal']} only (boundary {f['boundaryTime']:.6g}s)")
    lines.append(f"work: full morph {'recipe ' + result['work']['recipe'] if result['work']['fullMorph'] else 'none (swap)'}")
    lines.append(f"placement: space={p['space']} anchor={p['anchor']} fit={p['fit']} uniform={p['uniform']}" + (f" affine scale={p['affine']['scale']} translation={p['affine']['translation']}" if p["affine"] else ""))
    for side in ("source", "target"):
        e = result["lod"][side]
        lines.append(f"{side}: {e['entity']} lod={e['lod']} rev={e['revision']} sampling={e['sampling']} view={e['view']} scale={e['scale']} pose={e['pose']} scenario={e['scenario']}")
    lines += [f"frame {s['frame']}: {s['owner']} p={s['progress']:.4f}" for s in result.get("samples", ())]
    return lines


def request(documents, ids, profile_name, purpose, language):
    """📦 A validated export request that persists its explicit profile."""
    known = index(documents)
    missing = [i for i in ids if i not in known]
    if missing:
        raise Fail([say(language, "unknown-id", id=i) for i in missing])
    if profile_name not in profiles.NAMES:
        raise Fail([say(language, "profile", profile=profile_name, names=list(profiles.NAMES))])
    if purpose == "publication" and profile_name != "final":
        raise Fail([say(language, "purpose", profile=profile_name)])
    return {"version": REQUEST_VERSION, "purpose": purpose, "profile": profile_name, "fingerprint": profiles.fingerprint(profile_name), "language": language, "transitions": sorted(set(ids))}


def request_text(value):
    """🧬 Canonical text of an export request."""
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def run(args, root, language):
    """🏃 Execute one parsed command; return the exit code."""
    out = sys.stdout
    curriculum = args.curriculum
    if args.command == "list":
        documents = load_all(discover(teaching(root)))
        rows = [(t["id"], t["intent"], t["recipe"], t["timing"]["duration"], label(p, root)) for p, raw in documents.items() for t in raw["transitions"]]
        print(json.dumps([dict(zip(("id", "intent", "recipe", "duration", "file"), r)) for r in rows], indent=2, ensure_ascii=False) if args.json else "\n".join("\t".join(map(str, r)) for r in rows), file=out)
        return 0
    if args.command == "show":
        documents = load_all(discover(teaching(root)))
        path, position = find(documents, args.id, language)
        print(f"# {label(path, root)}", file=out)
        print(json.dumps(documents[path]["transitions"][position], indent=2, ensure_ascii=False), file=out)
        return 0
    if args.command in ("validate", "format"):
        documents = load_all(expand(args.paths, root))
        if args.command == "format":
            bad = [(p, q) for p, raw in documents.items() for q in spec.validate(raw)]
            if bad:
                show_findings(bad, root, sys.stderr)
                return 1
            for path, raw in documents.items():
                text = spec.dumps(raw)
                if path.read_text(encoding="utf-8") != text:
                    write(path, text)
                    print(say(language, "written", path=label(path, root)), file=out)
            return 0
        found = findings(documents, root, language, curriculum, whole=not args.paths) + runtime(args.played, load_all(discover(teaching(root))), language)
        if found:
            show_findings(found, root, sys.stderr)
            return 1
        _, played = references(documents, root, language, curriculum)
        if not args.paths:
            for ident, (path, _) in sorted(index(documents).items()):
                if ident not in played and curriculum_segments(root, curriculum):
                    print(say(language, "unplayed", id=ident, path=label(path, root)), file=sys.stderr)
        print(say(language, "valid", count=len(documents)), file=out)
        return 0
    if args.command == "set":
        documents = load_all(discover(teaching(root)))
        path, text = edit(documents, args.id, args.pointer, coerce(args.value), root, language, curriculum)
        if path.read_text(encoding="utf-8") == text:
            print(say(language, "unchanged", path=label(path, root)), file=out)
            return 0
        write(path, text)
        print(say(language, "written", path=label(path, root)), file=out)
        return 0
    if args.command == "preview-plan":
        documents = load_all(discover(teaching(root)))
        path, position = find(documents, args.id, language)
        transition = spec.parse(documents[path]).transitions[position]
        result = plan(transition, args.profile, language, fps=args.fps, start=args.start, source_bounds=args.source_bounds, target_bounds=args.target_bounds, frames=args.frames)
        print(json.dumps(result, indent=2, ensure_ascii=False) if args.json else "\n".join(describe(result)), file=out)
        return 0
    documents = load_all(discover(teaching(root)))
    value = request(documents, args.ids, args.profile, args.purpose, language)
    write(Path(args.output), request_text(value))
    print(say(language, "written", path=args.output), file=out)
    return 0


def preparse(argv):
    """🌐 The explicit language before the full parser is built; exit 2 with a bilingual message when absent."""
    pre = argparse.ArgumentParser(add_help=False)
    pre.add_argument("--language", choices=LANGUAGES)
    known, _ = pre.parse_known_args(argv)
    if known.language is None:
        print(f"{MESSAGES['language-required'][0]}\n{MESSAGES['language-required'][1]}", file=sys.stderr)
        raise SystemExit(2)
    return known.language


def parser_for(language):
    """🧭 The full command parser with help in the explicit language."""
    h = lambda key: HELP[key][LANGUAGES.index(language)]
    parser = argparse.ArgumentParser(prog="morph", description=h("description"))
    parser.add_argument("--language", choices=LANGUAGES, required=True, help="en|de")
    parser.add_argument("--root", type=Path, default=_WORKSPACE_ROOT, help=h("root"))
    parser.add_argument("--curriculum", type=Path, help="curriculum 🔣️.json")
    commands = parser.add_subparsers(dest="command", required=True)
    listing = commands.add_parser("list", help=h("list"))
    listing.add_argument("--json", action="store_true", help=h("json"))
    commands.add_parser("show", help=h("show")).add_argument("id", help=h("id"))
    checking = commands.add_parser("validate", help=h("validate"))
    checking.add_argument("paths", nargs="*", help=h("paths"))
    checking.add_argument("--played", action="append", default=[], metavar="FILE", help=h("played"))
    commands.add_parser("format", help=h("format")).add_argument("paths", nargs="+", help=h("format_paths"))
    setter = commands.add_parser("set", help=h("set"))
    setter.add_argument("id", help=h("id"))
    setter.add_argument("pointer", help=h("pointer"))
    setter.add_argument("value", help=h("value"))
    planner = commands.add_parser("preview-plan", help=h("plan"))
    planner.add_argument("id", help=h("id"))
    planner.add_argument("--profile", required=True, choices=profiles.NAMES, help=h("profile"))
    planner.add_argument("--fps", type=float, help=h("fps"))
    planner.add_argument("--start", type=float, default=0.0, help=h("start"))
    planner.add_argument("--source-bounds", help=h("source_bounds"))
    planner.add_argument("--target-bounds", help=h("target_bounds"))
    planner.add_argument("--frames", action="store_true", help=h("frames"))
    planner.add_argument("--json", action="store_true", help=h("json"))
    asking = commands.add_parser("request", help=h("request"))
    asking.add_argument("ids", nargs="+", help=h("ids"))
    asking.add_argument("--profile", required=True, choices=profiles.NAMES, help=h("profile"))
    asking.add_argument("--purpose", required=True, choices=PURPOSES, help=h("purpose"))
    asking.add_argument("--output", required=True, help=h("output"))
    return parser


def main(argv=None):
    """🎬 Command-line entry: exit 0 success, 1 refused or invalid, 2 usage."""
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    argv = list(sys.argv[1:] if argv is None else argv)
    language = preparse(argv)
    args = parser_for(language).parse_args(argv)
    try:
        return run(args, args.root, language)
    except Fail as failure:
        print("\n".join(failure.problems), file=sys.stderr)
        return 1
    except spec.SpecError as error:
        print(error, file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
