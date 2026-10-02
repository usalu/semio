#!/usr/bin/env python3
"""🪪️ Oracle of the id shapes and the handle policy: python-jsonschema over the normative schema, held to the Unicode Character Database.

No string a client chose may reach an event before it has its shape. Ids are judged by python-jsonschema's
Draft 7 validator against the normative ``Command`` and ``Query`` definitions (``Id`` is 32 lowercase hex,
``Slug`` the kebab-case pattern). A handle is normalized as the contract states — runs of ``White_Space``
(the 25 code points of the Unicode property, written out here) collapse to one space and are trimmed, the
typographic apostrophe U+2019 becomes the apostrophe — and the result is judged by the same validator
against ``$defs/Handle`` (its alphabet pattern and its 1…64 code points); input over 256 code points is
refused unread. The key is Python's own ``str.lower`` of the display and the stream id of a handle the hex
of the key's UTF-8 bytes.

The validator is not trusted alone. The alphabet it accepts is recomputed from the Unicode Character Database
(``unicodedata``): exactly the upper- and lowercase letters of Basic Latin, Latin-1 Supplement, Latin
Extended-A, Latin Extended-B and Latin Extended Additional without a compatibility decomposition, the ASCII
digits, the space and ``'._-``. Every member must be a starter that NFC leaves unchanged, and every accepted
display must equal its NFC form — that is the whole argument why the cores need no normalizer: nothing
outside NFC is spellable in the alphabet.

Python's ``re`` lets ``$`` match before a final line feed while ECMA 262, the dialect JSON Schema patterns are
written in, does not; an id or slug ending in a line feed is therefore refused here before the validator sees it.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🪪️identity-shapes/🔣️.json
@see https://www.unicode.org/reports/tr15/ — Unicode normalization forms
"""

# region 🔖️Imports
import json
import os
import re
import unicodedata

import jsonschema

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧬️schema", "🔣️.json")
VECTORS = "shared://🪪️identity-shapes/🔣️.json"
WHITE_SPACE = frozenset([*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B), 0x2028, 0x2029, 0x202F, 0x205F, 0x3000])
LATIN_BLOCKS = [(0x0000, 0x007F), (0x0080, 0x00FF), (0x0100, 0x017F), (0x0180, 0x024F), (0x1E00, 0x1EFF)]
HANDLE_INPUT_MAX = 256
IDS = ("id", "learner", "run", "quiz", "task")
SCALARS =[point for point in range(0x110000) if not 0xD800 <= point <= 0xDFFF]


def schema():
    """📜️ The normative contract."""
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        return json.load(handle)


def validator(definition):
    """⚖️ python-jsonschema's Draft 7 validator of one definition."""
    return jsonschema.Draft7Validator({**schema(), "$ref": "#/$defs/%s" % definition})


def paths(definition, document):
    """📍️ The members the contract refuses: the first path member of every complaint of the validator about the shape the document's ``type`` names, and every id or slug that ends in a line feed."""
    contract = schema()
    shape = next(option for option in contract["$defs"][definition]["oneOf"] if option["properties"]["type"]["const"] == document["type"])
    complaints = jsonschema.Draft7Validator({"$defs": contract["$defs"], **shape}).iter_errors(document)
    feeds = [name for name in IDS if isinstance(document.get(name), str) and document[name].endswith("\n")]
    return sorted({*feeds, *(str(error.absolute_path[0]) if error.absolute_path else "$" for error in complaints)})


def collapsed(handle):
    """🧽️ A handle with its White_Space runs collapsed to one space and trimmed, and U+2019 as the apostrophe."""
    words = "".join(" " if ord(character) in WHITE_SPACE else character for character in handle).split(" ")
    return " ".join(word for word in words if word).replace("’", "'")


def letter(character):
    """🔤️ Whether the Unicode Character Database makes a character a handle letter."""
    point = ord(character)
    return any(low <= point <= high for low, high in LATIN_BLOCKS) and unicodedata.category(character) in ("Lu", "Ll") and unicodedata.normalize("NFKC", character) == character


def member(character):
    """🧮️ Whether the Unicode Character Database puts a character into the handle alphabet."""
    return letter(character) or character in "0123456789 '._-"


def normalize_handle(handle, judge):
    """🪪️ ``{display, key, actor}`` of a handle the contract admits, or ``None``; an admitted display must be spelled in the alphabet of the database and be its own NFC form."""
    if len(handle) > HANDLE_INPUT_MAX:
        return None
    display = collapsed(handle)
    if not judge.is_valid(display):
        return None
    if not all(member(character) for character in display) or unicodedata.normalize("NFC", display) != display:
        raise AssertionError("the schema admits %r, which the Unicode Character Database keeps out of the alphabet or out of NFC" % display)
    key = display.lower()
    return {"display": display, "key": key, "actor": key.encode("utf-8").hex()}


def ranges(points):
    """📏️ Ascending code points as ``XXXX`` or ``XXXX-YYYY`` words."""
    found = []
    for point in points:
        if found and found[-1][1] == point - 1:
            found[-1][1] = point
        else:
            found.append([point, point])
    return " ".join("%04X" % low if low == high else "%04X-%04X" % (low, high) for low, high in found)


def alphabet_of(pattern):
    """🔡️ Every scalar value the contract's pattern admits between two letters, and the lowercase of every admitted character that changes."""
    members = [point for point in SCALARS if (found := pattern.search("a%sa" % chr(point))) is not None and found.end() == 3]
    database = [point for point in SCALARS if member(chr(point))]
    if members != database:
        raise AssertionError("the schema admits %s, the Unicode Character Database %s" % (ranges(members), ranges(database)))
    for point in members:
        if unicodedata.combining(chr(point)) != 0 or unicodedata.normalize("NFC", chr(point)) != chr(point):
            raise AssertionError("U+%04X is not a starter that NFC leaves unchanged" % point)
    folds = {"%04X" % point: chr(point).lower() for point in members if chr(point).lower() != chr(point)}
    return {"members": ranges(members), "folds": folds}


def rejection(vector, handles):
    """🚫️ Why a command or query is refused for its shapes — ``id-invalid`` for an id or slug, ``handle-invalid`` for a handle query — or ``None``."""
    document = vector["document"]
    broken = paths(vector["definition"], document)
    if any(path in IDS for path in broken):
        return "id-invalid"
    if vector["definition"] == "Query" and document.get("type") == "handle":
        return "handle-invalid" if broken or normalize_handle(document["handle"], handles) is None else None
    if broken:
        raise AssertionError("shapes/%s: the vector breaks %r, which no shape rule names" % (vector["id"], broken))
    return None


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def held_to(scenario, produced, expected):
    """🤝️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if value != expected[key]:
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def handles(ctx):
    """🏷️ Every committed handle normalized, or refused."""
    judge = validator("Handle")
    vectors = committed(ctx)["handles"]
    return held_to("handles", {vector["id"]: normalize_handle(vector["handle"], judge) for vector in vectors}, {vector["id"]: vector["expected"] for vector in vectors})


def alphabet(ctx):
    """🔠️ The alphabet of a handle and the lowercase of its letters, over every Unicode scalar value."""
    produced = alphabet_of(re.compile(schema()["$defs"]["Handle"]["pattern"]))
    judge = validator("Handle")
    for point in [*range(0x0000, 0x2500), *range(0x1E00, 0x2100), *range(0xFE00, 0x10000)]:
        if not 0xD800 <= point <= 0xDFFF and judge.is_valid("a%sa" % chr(point)) != member(chr(point)):
            raise AssertionError("python-jsonschema and the Unicode Character Database disagree on U+%04X" % point)
    return held_to("alphabet", {"alphabet": produced}, {"alphabet": committed(ctx)["alphabet"]})


def shapes(ctx):
    """🛃️ Every committed command and query, refused for its shapes or not."""
    judge = validator("Handle")
    vectors = committed(ctx)["shapes"]
    return held_to("shapes", {vector["id"]: rejection(vector, judge) for vector in vectors}, {vector["id"]: vector["expected"] for vector in vectors})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: python-jsonschema over the normative schema and the Unicode Character Database are the reference, the owned validators the subjects."""
    return Adapter("python").oracle("handles", handles).oracle("alphabet", alphabet).oracle("shapes", shapes)


# endregion 🔖️Registration
