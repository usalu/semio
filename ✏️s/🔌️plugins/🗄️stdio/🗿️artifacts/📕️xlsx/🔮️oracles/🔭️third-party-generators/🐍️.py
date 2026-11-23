"""🐍️ Replayable third-party generator of the `set-relationship-base` fixture pairs of `s.stdio.xlsx@ecma-376/{🔒️strict,🌉️transitional}`.

The `before` package is written by `openpyxl` (locked in `uv.lock`, group `test`) with fixed properties and a fixed zip clock. The `after` package is the same
package with every relationship `Type` of every `*.rels` part moved from the Transitional base to the Strict base by `lxml`; nothing here imports this
repository's codecs or mutation semantics.

  repo tools exec uv -- run --frozen --no-dev --group test --python <3.14 interpreter> python <this file> verify
  repo tools exec uv -- run --frozen --no-dev --group test --python <3.14 interpreter> python <this file> generate

`verify` re-derives every pair and compares it with the committed bytes (byte equality, then part-wise semantic equality), and replays the five openpyxl-written
grid pairs of the base subset (`insert-sheet`, `remove-sheet`, `rename-sheet`, `set-cell`, `remove-cell`) by rebuilding each package with openpyxl and comparing the
workbook as openpyxl reads it (their container bytes carry the clock of the original run, so only the read-back is comparable). It also re-derives the base subset's
unreferenced shared-string package: `lxml` appends one `<si>` that no worksheet cell references, and `openpyxl.reader.strings.read_string_table` (which walks every
`si`, referenced or not) is the committed pool. `generate` rewrites the relationship-base fixtures and their `testEvidence` rows. Run from anywhere: the repository
root is found from this file's location.
"""

import hashlib
import io
import json
import sys
import zipfile
from datetime import datetime
from pathlib import Path

import openpyxl
from lxml import etree

ARTIFACT = Path(__file__).resolve().parents[2]
SUBSETS = ARTIFACT / "🏅️standards" / "🔖️ecma-376" / "🪆️subsets"
TRANSITIONAL_BASE = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
STRICT_BASE = "http://purl.oclc.org/ooxml/officeDocument/relationships"
CLOCK = (2026, 1, 1, 0, 0, 0)
FIXTURE = "⚓️set-relationship-base-applied"
MEDIA = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
CLASSES = {"strict": "🔒️strict", "transitional": "🌉️transitional"}
MUTATION = {"kind": "set-relationship-base", "params": {"base": STRICT_BASE}}


def repack(parts: list[tuple[str, bytes]]) -> bytes:
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(parts):
            info = zipfile.ZipInfo(name, CLOCK)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o600 << 16
            archive.writestr(info, data)
    return buffer.getvalue()


def read_parts(package: bytes) -> list[tuple[str, bytes]]:
    with zipfile.ZipFile(io.BytesIO(package)) as archive:
        return [(info.filename, archive.read(info.filename)) for info in archive.infolist()]


def build_before() -> bytes:
    workbook = openpyxl.Workbook()
    sheet = workbook.active
    sheet.title = "Sheet1"
    sheet["A1"] = "relationship-base"
    sheet["B1"] = 1
    workbook.properties.created = datetime(2026, 1, 1)
    workbook.properties.modified = datetime(2026, 1, 1)
    buffer = io.BytesIO()
    workbook.save(buffer)
    return repack(pin_modified(read_parts(buffer.getvalue())))


def pin_modified(parts: list[tuple[str, bytes]]) -> list[tuple[str, bytes]]:
    pinned = []
    for name, data in parts:
        if name == "docProps/core.xml":
            root = etree.fromstring(data)
            for element in root.iter("{http://purl.org/dc/terms/}modified"):
                element.text = "2026-01-01T00:00:00Z"
            data = etree.tostring(root)
        pinned.append((name, data))
    return pinned


def move_base(parts: list[tuple[str, bytes]], source: str, target: str) -> list[tuple[str, bytes]]:
    moved = []
    for name, data in parts:
        if name.endswith(".rels"):
            root = etree.fromstring(data)
            for relationship in root:
                kind = relationship.get("Type", "")
                if kind.startswith(source):
                    relationship.set("Type", target + kind[len(source):])
            data = etree.tostring(root)
        moved.append((name, data))
    return moved


def build_pair() -> tuple[bytes, bytes]:
    before = build_before()
    after = repack(move_base(read_parts(before), TRANSITIONAL_BASE, STRICT_BASE))
    return before, after


def audit(before: bytes, after: bytes) -> None:
    openpyxl.load_workbook(io.BytesIO(before))
    unchanged = dict(read_parts(before))
    moved = dict(read_parts(after))
    assert unchanged.keys() == moved.keys(), "the part inventory must not change"
    rels = 0
    for name, data in moved.items():
        if not name.endswith(".rels"):
            assert data == unchanged[name], f"{name}: a non-relationship part changed"
            continue
        for relationship in etree.fromstring(data):
            assert not relationship.get("Type").startswith(TRANSITIONAL_BASE), f"{name}: a Transitional base remains"
            rels += 1
        assert any(r.get("Type").startswith(STRICT_BASE) for r in etree.fromstring(data)), f"{name}: no Strict base"
    assert rels > 0 and unchanged != moved, "the pair must differ"


def sha(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def evidence(subset: str, before: bytes, after: bytes) -> dict:
    stem = f"../🧪️tests/🧫️fixtures/{FIXTURE}"
    return {
        "id": "set-relationship-base-applied",
        "class": "third-party-generated",
        "target": {"artifact": "s.stdio.xlsx", "standard": "ecma-376", "subset": subset},
        "mutation": "set-relationship-base",
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "degree"},
        "files": [
            {"role": "expected-before-xlsx", "path": f"{stem}/⬅️before.xlsx", "mediaType": MEDIA, "sha256": sha(before), "bytes": len(before)},
            {"role": "expected-after-xlsx", "path": f"{stem}/➡️after.xlsx", "mediaType": MEDIA, "sha256": sha(after), "bytes": len(after)},
        ],
        "generator": {
            "oracle": f"openpyxl-xlsx-ecma-376-{subset}-mutate-reader",
            "packageVersion": f"openpyxl@{openpyxl.__version__} + lxml@{etree.LXML_VERSION[0]}.{etree.LXML_VERSION[1]}.{etree.LXML_VERSION[2]}",
            "engineFamily": "openpyxl",
            "engineVersion": f"openpyxl@{openpyxl.__version__}",
            "command": "repo tools exec uv -- run --frozen --no-dev --group test python ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/🔭️third-party-generators/🐍️.py generate",
            "platform": "any",
        },
        "provenance": {
            "source": "generated",
            "license": "MIT (openpyxl) + BSD-3-Clause (lxml)",
            "attribution": "before written by openpyxl; after is the before with every *.rels relationship Type moved from the Transitional to the Strict officeDocument base by lxml",
            "security": "scanned-clean",
            "privacy": "no-personal-data",
        },
        "comparisonProfile": "exact-bytes-v1",
        "reproducible": True,
        "family": "structural",
        "notes": "Every relationship Type of every *.rels part moves from the Transitional officeDocument relationship base to the Strict one; no other part changes.",
    }


GRID_FIXTURES = SUBSETS / "🧱️base" / "🧪️tests" / "🧫️fixtures"


def grid_workbook(sheets: dict[str, dict[str, object]]) -> bytes:
    workbook = openpyxl.Workbook()
    workbook.remove(workbook.active)
    for title, cells in sheets.items():
        sheet = workbook.create_sheet(title)
        for coordinate, value in cells.items():
            sheet[coordinate] = value
    buffer = io.BytesIO()
    workbook.save(buffer)
    return buffer.getvalue()


def read_grid(package: bytes) -> dict[str, dict[str, object]]:
    workbook = openpyxl.load_workbook(io.BytesIO(package))
    return {sheet.title: {cell.coordinate: cell.value for row in sheet.iter_rows() for cell in row if cell.value is not None} for sheet in workbook}


GRID_RECIPES = {
    "➕️insert-sheet-applied": ({"Sheet1": {}}, {"Sheet1": {}, "Sheet2": {}}),
    "➖️remove-sheet-applied": ({"Sheet1": {}, "Sheet2": {}}, {"Sheet1": {}}),
    "🏷️rename-sheet-applied": ({"Sheet1": {}}, {"Renamed": {}}),
    "✍️set-cell-applied": ({"Sheet": {"A1": "Hello", "B1": 1}}, {"Sheet": {"A1": "Hello", "B1": 2}}),
    "🧽️remove-cell-applied": ({"Sheet": {"A1": "Hello", "D1": "Temp"}}, {"Sheet": {"A1": "Hello"}}),
}


def verify_grid_pairs() -> int:
    failures = 0
    for directory, (before, after) in GRID_RECIPES.items():
        for name, expected in (("⬅️before.xlsx", before), ("➡️after.xlsx", after)):
            committed = read_grid((GRID_FIXTURES / directory / name).read_bytes())
            rebuilt = read_grid(grid_workbook(expected))
            if committed == rebuilt == expected:
                print(f"[base] {directory}/{name}: openpyxl replays the committed workbook")
            else:
                failures += 1
                print(f"[base] {directory}/{name}: DIFFERENT committed={committed} rebuilt={rebuilt}")
    return failures


SST = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
UNREFERENCED_VALUE = "Ökobau Referenzquelle 2024"
UNREFERENCED_DIR = GRID_FIXTURES / "🕳️unreferenced-shared-string"


def shared_string_pool(package: bytes) -> list[str]:
    """👁 openpyxl's own string-table reader: every `si`, including entries no cell references."""
    from openpyxl.reader.strings import read_string_table

    with zipfile.ZipFile(io.BytesIO(package)) as archive:
        xml = archive.read("xl/sharedStrings.xml")
    return [str(item) for item in read_string_table(io.BytesIO(xml))]


def worksheet_references(package: bytes, index: int) -> list[str]:
    """🔗️ Worksheet cells whose shared-string index is `index` (`<c t="s"><v>index</v></c>`)."""
    found = []
    with zipfile.ZipFile(io.BytesIO(package)) as archive:
        for name in archive.namelist():
            if not (name.startswith("xl/worksheets/") and name.endswith(".xml")):
                continue
            root = etree.fromstring(archive.read(name))
            for cell in root.iter(f"{{{SST}}}c"):
                if cell.get("t") != "s":
                    continue
                value = cell.find(f"{{{SST}}}v")
                if value is not None and (value.text or "").strip() == str(index):
                    found.append(f"{name}:{cell.get('r')}")
    return found


def append_unreferenced(package: bytes, value: str) -> bytes:
    """✍️ lxml appends one `si` and raises `uniqueCount`. `count` stays: nothing new references it."""
    rewritten = []
    for name, data in read_parts(package):
        if name == "xl/sharedStrings.xml":
            root = etree.fromstring(data)
            unique = int(root.get("uniqueCount"))
            item = etree.SubElement(root, f"{{{SST}}}si")
            text = etree.SubElement(item, f"{{{SST}}}t")
            text.set("{http://www.w3.org/XML/1998/namespace}space", "preserve")
            text.text = value
            root.set("uniqueCount", str(unique + 1))
            data = etree.tostring(root, xml_declaration=True, encoding="UTF-8", standalone=True)
        rewritten.append((name, data))
    return repack(rewritten)


def unreferenced_witness() -> tuple[bytes, dict]:
    """🕳️ The real workbook plus one unreferenced pool entry, and the pool openpyxl reads from it."""
    source = (GRID_FIXTURES / "📕️reuse-marketplaces.xlsx").read_bytes()
    baseline = shared_string_pool(source)
    package = append_unreferenced(source, UNREFERENCED_VALUE)
    pool = shared_string_pool(package)
    index = len(baseline)
    assert pool[:index] == baseline, "appending an si must not rewrite the entries already there"
    assert pool[index:] == [UNREFERENCED_VALUE], pool[index:]
    assert worksheet_references(package, index) == [], "the appended entry must stay unreferenced"
    from openpyxl import load_workbook

    visible = {
        sheet.title: cell.value
        for sheet in load_workbook(io.BytesIO(package))
        for row in sheet.iter_rows()
        for cell in row
        if cell.value == UNREFERENCED_VALUE
    }
    assert visible == {}, f"openpyxl's cell reader must not surface an unreferenced entry: {visible}"
    witness = {
        "reader": "openpyxl.reader.strings.read_string_table",
        "packageVersion": f"openpyxl@{openpyxl.__version__}",
        "baselineCount": len(baseline),
        "unreferencedIndex": index,
        "unreferencedValue": UNREFERENCED_VALUE,
        "sharedStrings": pool,
    }
    return package, witness


def verify_unreferenced() -> int:
    """🔬 The committed package and pool are exactly what openpyxl and lxml re-derive."""
    package, witness = unreferenced_witness()
    committed_package = UNREFERENCED_DIR / "📦️package.xlsx"
    committed_witness = UNREFERENCED_DIR / "🔣️.json"
    if not committed_package.is_file() or not committed_witness.is_file():
        print("[base] unreferenced shared string: fixture missing")
        return 1
    if committed_package.read_bytes() != package:
        print("[base] unreferenced shared string: package bytes differ from the lxml rebuild")
        return 1
    committed = json.loads(committed_witness.read_text(encoding="utf-8"))
    if committed != witness:
        print("[base] unreferenced shared string: openpyxl pool differs from the committed witness")
        return 1
    print(f"[base] unreferenced shared string: openpyxl reads index {witness['unreferencedIndex']} and no cell does")
    return 0


def generate_unreferenced() -> None:
    """📝 Writes the unreferenced-pool package and the openpyxl pool the Rust reader is held to."""
    package, witness = unreferenced_witness()
    UNREFERENCED_DIR.mkdir(parents=True, exist_ok=True)
    (UNREFERENCED_DIR / "📦️package.xlsx").write_bytes(package)
    (UNREFERENCED_DIR / "🔣️.json").write_text(json.dumps(witness, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"[base] wrote {UNREFERENCED_DIR}")


def manifest_path(subset: str) -> Path:
    return SUBSETS / CLASSES[subset] / "🔮️oracles" / "🔣️.json"


def fixture_directory(subset: str) -> Path:
    return SUBSETS / CLASSES[subset] / "🧪️tests" / "🧫️fixtures" / FIXTURE


def generate() -> int:
    before, after = build_pair()
    audit(before, after)
    for subset in CLASSES:
        directory = fixture_directory(subset)
        directory.mkdir(parents=True, exist_ok=True)
        (directory / "⬅️before.xlsx").write_bytes(before)
        (directory / "➡️after.xlsx").write_bytes(after)
        path = manifest_path(subset)
        original = path.read_text(encoding="utf-8")
        manifest = json.loads(original)
        assert json.dumps(manifest, indent=2, ensure_ascii=False) + "\n" == original, f"{path}: not canonically formatted"
        rows = manifest["testEvidence"]
        row = evidence(subset, before, after)
        at = next((index for index, existing in enumerate(rows) if existing["id"] == row["id"]), None)
        if at is None:
            rows.append(row)
        else:
            rows[at] = row
        path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"[{subset}] wrote {directory}")
    generate_unreferenced()
    return 0


def verify() -> int:
    before, after = build_pair()
    audit(before, after)
    failures = 0
    for subset in CLASSES:
        directory = fixture_directory(subset)
        for name, built in (("⬅️before.xlsx", before), ("➡️after.xlsx", after)):
            committed = (directory / name).read_bytes()
            if committed == built:
                print(f"[{subset}] {name}: byte-identical")
            elif sorted(read_parts(committed)) == sorted(read_parts(built)):
                print(f"[{subset}] {name}: part-wise identical, container bytes differ")
            else:
                failures += 1
                print(f"[{subset}] {name}: DIFFERENT")
        manifest = json.loads(manifest_path(subset).read_text(encoding="utf-8"))
        row = next((existing for existing in manifest["testEvidence"] if existing["id"] == "set-relationship-base-applied"), None)
        expected = [sha((directory / name).read_bytes()) for name in ("⬅️before.xlsx", "➡️after.xlsx")]
        if row is None or [file["sha256"] for file in row["files"]] != expected:
            failures += 1
            print(f"[{subset}] the testEvidence row is missing or does not match the committed bytes")
    failures += verify_grid_pairs()
    failures += verify_unreferenced()
    print("every pair replays" if failures == 0 else f"{failures} failure(s)")
    return 0 if failures == 0 else 1


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    sys.exit({"generate": generate, "verify": verify, "generate-unreferenced": lambda: generate_unreferenced() or 0}.get(command, lambda: print("usage: generate | verify | generate-unreferenced") or 2)())
