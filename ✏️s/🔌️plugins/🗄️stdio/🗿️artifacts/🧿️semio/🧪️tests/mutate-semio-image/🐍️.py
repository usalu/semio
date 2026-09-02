"""🐍️ Independent Python implementation of the `s.stdio.semio.image` carrier and its thirteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. Two independent producers meet in this file, and they
answer two different halves of the question:

* **Pillow (PIL 11.3, littleCMS)** is a genuine third party and it speaks the PAYLOAD. Every RGBA8
  sample this case moves was decoded out of the real committed animated GIF by Pillow, the ICC
  profile `set-icc` attaches is a real sRGB profile littleCMS emitted, and `identity-round-trip`
  hands the re-decoded planes back to Pillow so an independent raster library — not this file —
  states the geometry, the mode and the extremes of what our codec produced. Pillow does not read
  `.dsl.semio` and has no opinion about mutation verbs, and that boundary is named rather than
  blurred.
* **This module** is the second IMPLEMENTATION of the carrier and the vocabulary, which no third
  party speaks. It was written from the committed specification documents alone:
  - the envelope — `semio <schema>.dsl v<version>` preamble for text, and the
    `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
    specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section,
    the carrier's normative description;
  - the DSL body is the committed grammar
    `../../🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
    (`document = artifact-mark schema-line width-line height-line colorspace-line bit-depth-line
    icc-line frames-line metadata-line`, `colorspace = r|a|g|y|i`, `option-hex`, hex-encoded
    scalars);
  - the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio`
    together with its Kaitai mirror `…/💾️binary/🥋️.ksy`, which — unusually for this family
    — describes the trailing chain completely: *"icc (presence u8 + optional length-prefixed
    bytes), frames (varint count + per-frame delay_ms u32 LE + varint-length-prefixed rgba8 bytes),
    metadata (varint count + per-entry varint-length-prefixed key/value UTF-8)"*. Nothing had to be
    reverse-engineered from bytes here, and `pack_bytes` re-encoding the committed swatch file byte
    for byte is what proves the reading right;
  - the thirteen verbs, their argument lists and their JSON wire form are the committed grammar
    `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, the committed JSON schema
    `…/🧬️mutations/🔣️.json` and the committed per-kind specification vectors under
    `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`, which pin each verb's before/after semantics.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
DSL_PREAMBLE = "semio s.stdio.semio.image.dsl v1"
PACK_TOKEN = "s.stdio.semio.image.pack v1"
PACK_FORMAT = 1

#: 🌈️ `colorspace = "r" | "a" | "g" | "y" | "i"` from the DSL grammar, in the ordinal order the
#: Kaitai mirror spells out for the pack frame: *"0=Rgb 1=Rgba 2=Grayscale 3=GrayscaleAlpha
#: 4=Indexed"*. The committed swatch carries `colorspace=a` in its DSL and `0x01` in its pack, which
#: pins the two spellings against each other.
COLORSPACE_ORDER = ("rgb", "rgba", "grayscale", "grayscaleAlpha", "indexed")
COLORSPACE_LETTER = {"rgb": "r", "rgba": "a", "grayscale": "g", "grayscaleAlpha": "y", "indexed": "i"}
LETTER_COLORSPACE = {letter: name for name, letter in COLORSPACE_LETTER.items()}

KINDS = (
    "no-mutation",
    "set-snapshot",
    "set-dimensions",
    "set-colorspace",
    "set-bit-depth",
    "set-icc",
    "insert-frame",
    "remove-frame",
    "move-frame",
    "set-frame-delay",
    "set-frame-pixels",
    "set-metadata-entry",
    "remove-metadata-entry",
)

ARTIFACT_DSL = "local://🗣️.dsl.semio"
ARTIFACT_PACK = "local://🎒️artifact.pack.semio"


def hex_of_text(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction, for a string leaf."""
    return text.encode("utf-8").hex()


def text_of_hex(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction, for a string leaf."""
    return bytes.fromhex(hexed).decode("utf-8")


def strip_preamble(text: str) -> str:
    """📜️ Splits the mandatory text envelope preamble off and checks it names this artifact."""
    line, _, body = text.partition("\n")
    if line != DSL_PREAMBLE:
        raise AssertionError("the text envelope preamble is %r, expected %r" % (line, DSL_PREAMBLE))
    return body


# endregion 🔖️Carrier


# region 🔖️Dsl
def read_field(body: str, name: str) -> tuple:
    """📜️ Reads one `name=value` line off the front of the body, in the grammar's fixed order."""
    line, _, rest = body.partition("\n")
    prefix = name + "="
    if not line.startswith(prefix):
        raise AssertionError("expected a %r line, found %r" % (name, line))
    return line[len(prefix) :], rest


def split_top_level(text: str) -> list:
    """📜️ Splits a bracket list body on the commas that sit at bracket depth zero."""
    parts = []
    depth = 0
    current = []
    for character in text:
        if character == "[":
            depth += 1
        elif character == "]":
            depth -= 1
        if character == "," and depth == 0:
            parts.append("".join(current))
            current = []
            continue
        current.append(character)
    tail = "".join(current)
    if tail != "" or parts:
        parts.append(tail)
    return parts


def strip_brackets(text: str, what: str) -> str:
    """📜️ Unwraps one `[...]` group, refusing anything the grammar does not admit there."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("%s must be a bracketed group, found %r" % (what, text[:40]))
    return text[1:-1]


def parse_option_hex(text: str, what: str):
    """📜️ `option-hex = "[" "0" "]" | "[" "1" "," hex "]"` — absent, or present with a payload."""
    inner = strip_brackets(text, what)
    parts = split_top_level(inner)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return list(bytes.fromhex(parts[1]))
    raise AssertionError("%s is not a well-formed option-hex: %r" % (what, text[:40]))


def print_option_hex(value) -> str:
    """📜️ `option-hex` in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % bytes(value).hex()


def parse_dsl(text: str) -> dict:
    """📜️ The committed DSL grammar, read into the snapshot's own structural shape."""
    body = strip_preamble(text)
    schema_hex, body = read_field(body, "schema")
    width, body = read_field(body, "width")
    height, body = read_field(body, "height")
    colorspace, body = read_field(body, "colorspace")
    bit_depth, body = read_field(body, "bitDepth")
    icc, body = read_field(body, "icc")
    frames, body = read_field(body, "frames")
    metadata, body = read_field(body, "metadata")
    if body != "":
        raise AssertionError("the document carries trailing content after its metadata line: %r" % body[:40])
    if colorspace not in LETTER_COLORSPACE:
        raise AssertionError("unknown colorspace letter %r" % colorspace)
    return {
        "schema": text_of_hex(schema_hex),
        "width": int(width),
        "height": int(height),
        "colorspace": LETTER_COLORSPACE[colorspace],
        "bitDepth": int(bit_depth),
        "frames": [parse_frame(entry) for entry in split_top_level(strip_brackets(frames, "frames"))],
        "icc": parse_option_hex(icc, "icc"),
        "metadata": [parse_metadata_entry(entry) for entry in split_top_level(strip_brackets(metadata, "metadata"))],
    }


def parse_frame(text: str) -> dict:
    """📜️ `frame = "[" INT "," hex "]"` — the delay in milliseconds and the RGBA8 plane."""
    parts = split_top_level(strip_brackets(text, "frame"))
    if len(parts) != 2:
        raise AssertionError("a frame must carry a delay and a plane, found %r" % text[:40])
    return {"delayMs": int(parts[0]), "rgba8": list(bytes.fromhex(parts[1]))}


def parse_metadata_entry(text: str) -> dict:
    """📜️ `entry = "[" hex "," hex "]"` — one key/value pair, both hex-encoded UTF-8."""
    parts = split_top_level(strip_brackets(text, "metadata entry"))
    if len(parts) != 2:
        raise AssertionError("a metadata entry must carry a key and a value, found %r" % text[:40])
    return {"key": text_of_hex(parts[0]), "value": text_of_hex(parts[1])}


def print_dsl(document: dict) -> str:
    """📜️ The committed DSL grammar in the writing direction, line for line in its declared order."""
    frames = ",".join("[%d,%s]" % (frame["delayMs"], bytes(frame["rgba8"]).hex()) for frame in document["frames"])
    metadata = ",".join("[%s,%s]" % (hex_of_text(entry["key"]), hex_of_text(entry["value"])) for entry in document["metadata"])
    return "\n".join(
        [
            DSL_PREAMBLE,
            "schema=%s" % hex_of_text(document["schema"]),
            "width=%d" % document["width"],
            "height=%d" % document["height"],
            "colorspace=%s" % COLORSPACE_LETTER[document["colorspace"]],
            "bitDepth=%d" % document["bitDepth"],
            "icc=%s" % print_option_hex(document["icc"]),
            "frames=[%s]" % frames,
            "metadata=[%s]" % metadata,
        ]
    )


# endregion 🔖️Dsl


# region 🔖️Pack
def read_varint(data: bytes, at: int) -> tuple:
    """🔢️ LEB128, the `varint` the protocol's `schema_len` and every count/length below uses."""
    value = 0
    shift = 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if byte & 0x80 == 0:
            return value, at
        shift += 7


def write_varint(value: int) -> bytes:
    """🔢️ LEB128 in the writing direction."""
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
            continue
        out.append(byte)
        return bytes(out)


def read_blob(data: bytes, at: int) -> tuple:
    """🔢️ A varint-length-prefixed byte run — the protocol's `write_str_lp` wire."""
    length, at = read_varint(data, at)
    return data[at : at + length], at + length


def write_blob(payload: bytes) -> bytes:
    """🔢️ A varint-length-prefixed byte run in the writing direction."""
    return write_varint(len(payload)) + payload


def parse_pack(data: bytes) -> dict:
    """🎒️ The committed binary envelope and the pack frame the Kaitai mirror describes."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the binary envelope magic is %r, expected %r" % (data[:8], BINARY_MAGIC))
    token_length = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_length].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("the binary envelope token is %r, expected %r" % (token, PACK_TOKEN))
    at = 12 + token_length
    if data[at] != PACK_FORMAT:
        raise AssertionError("the pack format byte is %d, expected %d" % (data[at], PACK_FORMAT))
    at += 1
    schema, at = read_blob(data, at)
    width = int.from_bytes(data[at : at + 4], "little")
    height = int.from_bytes(data[at + 4 : at + 8], "little")
    colorspace = data[at + 8]
    bit_depth = data[at + 9]
    at += 10
    if colorspace >= len(COLORSPACE_ORDER):
        raise AssertionError("the pack colorspace ordinal %d is outside the declared enumeration" % colorspace)
    present = data[at]
    at += 1
    icc = None
    if present == 1:
        blob, at = read_blob(data, at)
        icc = list(blob)
    elif present != 0:
        raise AssertionError("the icc presence byte is %d, expected 0 or 1" % present)
    count, at = read_varint(data, at)
    frames = []
    for _ in range(count):
        delay = int.from_bytes(data[at : at + 4], "little")
        at += 4
        plane, at = read_blob(data, at)
        frames.append({"delayMs": delay, "rgba8": list(plane)})
    count, at = read_varint(data, at)
    metadata = []
    for _ in range(count):
        key, at = read_blob(data, at)
        value, at = read_blob(data, at)
        metadata.append({"key": key.decode("utf-8"), "value": value.decode("utf-8")})
    if at != len(data):
        raise AssertionError("the pack frame ends %d bytes before its envelope does" % (len(data) - at))
    return {
        "schema": schema.decode("utf-8"),
        "width": width,
        "height": height,
        "colorspace": COLORSPACE_ORDER[colorspace],
        "bitDepth": bit_depth,
        "frames": frames,
        "icc": icc,
        "metadata": metadata,
    }


def pack_bytes(document: dict) -> bytes:
    """🎒️ The pack frame in the writing direction, inside the shared binary envelope."""
    body = bytearray()
    body.append(PACK_FORMAT)
    body += write_blob(document["schema"].encode("utf-8"))
    body += int(document["width"]).to_bytes(4, "little")
    body += int(document["height"]).to_bytes(4, "little")
    body.append(COLORSPACE_ORDER.index(document["colorspace"]))
    body.append(int(document["bitDepth"]))
    if document["icc"] is None:
        body.append(0)
    else:
        body.append(1)
        body += write_blob(bytes(document["icc"]))
    body += write_varint(len(document["frames"]))
    for frame in document["frames"]:
        body += int(frame["delayMs"]).to_bytes(4, "little")
        body += write_blob(bytes(frame["rgba8"]))
    body += write_varint(len(document["metadata"]))
    for entry in document["metadata"]:
        body += write_blob(entry["key"].encode("utf-8"))
        body += write_blob(entry["value"].encode("utf-8"))
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
#: 🏷️ The JSON wire form is serde's internally-tagged shape with camelCase VARIANT names, exactly as
#: the committed vectors and `…/🧬️mutations/🔣️.json` spell it. That renames variants and not
#: their fields, so a struct-variant's own keys stay snake_case (`bit_depth`, `delay_ms`) while a
#: NESTED snapshot or frame payload keeps its own camelCase (`bitDepth`, `delayMs`) — the committed
#: vectors carry exactly that mixed shape.
TAG_TO_KIND = {
    "noMutation": "no-mutation",
    "setSnapshot": "set-snapshot",
    "setDimensions": "set-dimensions",
    "setColorspace": "set-colorspace",
    "setBitDepth": "set-bit-depth",
    "setIcc": "set-icc",
    "insertFrame": "insert-frame",
    "removeFrame": "remove-frame",
    "moveFrame": "move-frame",
    "setFrameDelay": "set-frame-delay",
    "setFramePixels": "set-frame-pixels",
    "setMetadataEntry": "set-metadata-entry",
    "removeMetadataEntry": "remove-metadata-entry",
}

def kind_of(mutation: dict) -> str:
    """🏷️ The kebab-case kind a wire payload names, refusing anything outside the vocabulary."""
    tag = mutation.get("mutation")
    if tag not in TAG_TO_KIND:
        raise AssertionError("%r is not one of this subset's thirteen declared verbs" % (tag,))
    return TAG_TO_KIND[tag]


def frame_at(document: dict, index: int, verb: str) -> dict:
    """🎞️ The frame a positional verb addresses, refusing an index the document does not hold."""
    if index < 0 or index >= len(document["frames"]):
        raise AssertionError("%s addresses frame %d of a %d-frame document" % (verb, index, len(document["frames"])))
    return document["frames"][index]


def value_of(document: dict, key: str):
    """🏷️ The value a metadata key currently carries, or `None` when the document has no such key."""
    for entry in document["metadata"]:
        if entry["key"] == key:
            return entry["value"]
    return None


def apply_mutation(document: dict, mutation: dict) -> dict:
    """▶️ One verb applied to a document, returning the resulting document.

    Each arm is the behaviour its committed `(before, mutation, after)` specification vector states:
    `set-snapshot` replaces every slot but the schema id, the four scalar setters write their one
    field, `insert-frame`/`remove-frame`/`move-frame`/`set-frame-delay`/`set-frame-pixels` address
    the frame list by POSITION, and the two metadata verbs address it by KEY — `set-metadata-entry`
    rewriting an existing key in place and appending an unknown one.
    """
    kind = kind_of(mutation)
    result = json.loads(json.dumps(document))
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        replacement = json.loads(json.dumps(mutation["snapshot"]))
        replacement["schema"] = document["schema"]
        return replacement
    if kind == "set-dimensions":
        result["width"] = int(mutation["width"])
        result["height"] = int(mutation["height"])
        return result
    if kind == "set-colorspace":
        if mutation["colorspace"] not in COLORSPACE_LETTER:
            raise AssertionError("%r is not one of the five declared colorspaces" % mutation["colorspace"])
        result["colorspace"] = mutation["colorspace"]
        return result
    if kind == "set-bit-depth":
        result["bitDepth"] = int(mutation["bit_depth"])
        return result
    if kind == "set-icc":
        result["icc"] = None if mutation["icc"] is None else list(mutation["icc"])
        return result
    if kind == "insert-frame":
        index = int(mutation["index"])
        if index < 0 or index > len(result["frames"]):
            raise AssertionError("insert-frame addresses position %d of a %d-frame document" % (index, len(result["frames"])))
        result["frames"].insert(index, json.loads(json.dumps(mutation["frame"])))
        return result
    if kind == "remove-frame":
        index = int(mutation["index"])
        frame_at(result, index, "remove-frame")
        result["frames"].pop(index)
        return result
    if kind == "move-frame":
        source = int(mutation["from"])
        target = int(mutation["to"])
        frame_at(result, source, "move-frame")
        if target < 0 or target >= len(result["frames"]):
            raise AssertionError("move-frame targets position %d of a %d-frame document" % (target, len(result["frames"])))
        result["frames"].insert(target, result["frames"].pop(source))
        return result
    if kind == "set-frame-delay":
        index = int(mutation["index"])
        frame_at(result, index, "set-frame-delay")["delayMs"] = int(mutation["delay_ms"])
        return result
    if kind == "set-frame-pixels":
        index = int(mutation["index"])
        frame_at(result, index, "set-frame-pixels")["rgba8"] = list(mutation["rgba8"])
        return result
    if kind == "set-metadata-entry":
        for entry in result["metadata"]:
            if entry["key"] == mutation["key"]:
                entry["value"] = mutation["value"]
                return result
        result["metadata"].append({"key": mutation["key"], "value": mutation["value"]})
        return result
    if value_of(result, mutation["key"]) is None:
        raise AssertionError("remove-metadata-entry addresses the key %r, which the document does not carry" % mutation["key"])
    result["metadata"] = [entry for entry in result["metadata"] if entry["key"] != mutation["key"]]
    return result


def inverse_mutation(document: dict, mutation: dict) -> dict:
    """↩️ The verb's own inverse against the document it is about to be applied to.

    Every arm restores from the BEFORE document rather than guessing: the scalar setters re-set the
    value the document carried, `remove-frame` re-inserts the frame it is about to drop at the
    position it held, `move-frame` moves back, and `set-metadata-entry` either rewrites the old
    value or removes the key it introduced.
    """
    kind = kind_of(mutation)
    if kind in ("no-mutation", "set-snapshot"):
        return {"mutation": "noMutation"} if kind == "no-mutation" else {"mutation": "setSnapshot", "snapshot": json.loads(json.dumps(document))}
    if kind == "set-dimensions":
        return {"mutation": "setDimensions", "width": document["width"], "height": document["height"]}
    if kind == "set-colorspace":
        return {"mutation": "setColorspace", "colorspace": document["colorspace"]}
    if kind == "set-bit-depth":
        return {"mutation": "setBitDepth", "bit_depth": document["bitDepth"]}
    if kind == "set-icc":
        return {"mutation": "setIcc", "icc": None if document["icc"] is None else list(document["icc"])}
    if kind == "insert-frame":
        return {"mutation": "removeFrame", "index": int(mutation["index"])}
    if kind == "remove-frame":
        index = int(mutation["index"])
        return {"mutation": "insertFrame", "index": index, "frame": json.loads(json.dumps(frame_at(document, index, "remove-frame")))}
    if kind == "move-frame":
        return {"mutation": "moveFrame", "from": int(mutation["to"]), "to": int(mutation["from"])}
    if kind == "set-frame-delay":
        index = int(mutation["index"])
        return {"mutation": "setFrameDelay", "index": index, "delay_ms": frame_at(document, index, "set-frame-delay")["delayMs"]}
    if kind == "set-frame-pixels":
        index = int(mutation["index"])
        return {"mutation": "setFramePixels", "index": index, "rgba8": list(frame_at(document, index, "set-frame-pixels")["rgba8"])}
    previous = value_of(document, mutation["key"])
    if kind == "set-metadata-entry":
        if previous is None:
            return {"mutation": "removeMetadataEntry", "key": mutation["key"]}
        return {"mutation": "setMetadataEntry", "key": mutation["key"], "value": previous}
    if previous is None:
        raise AssertionError("remove-metadata-entry addresses the key %r, which the document does not carry" % mutation["key"])
    return {"mutation": "setMetadataEntry", "key": mutation["key"], "value": previous}


# endregion 🔖️Mutations


# region 🔖️Pillow
def pillow_report(document: dict) -> dict:
    """🖼️ What the third-party raster library states about the planes this document carries.

    Pillow reconstructs each frame as an `RGBA` image of the document's own declared width and
    height and reports its mode, its size, its extrema and its colour count — facts about the actual
    samples, computed by a library that has never seen this repository's codec. A plane whose length
    does not match `width * height * 4` cannot be reconstructed at all, which is how a geometry the
    samples do not support fails here rather than passing quietly.
    """
    from PIL import Image

    frames = []
    for index, frame in enumerate(document["frames"]):
        expected = document["width"] * document["height"] * 4
        plane = bytes(frame["rgba8"])
        if expected == 0 or len(plane) != expected:
            frames.append({"index": index, "planeBytes": len(plane), "declaredBytes": expected, "reconstructable": False})
            continue
        image = Image.frombytes("RGBA", (document["width"], document["height"]), plane)
        frames.append(
            {
                "index": index,
                "planeBytes": len(plane),
                "declaredBytes": expected,
                "reconstructable": True,
                "mode": image.mode,
                "size": list(image.size),
                "extrema": [list(band) for band in image.getextrema()],
                "colours": len(image.getcolors(maxcolors=1 << 22) or []),
            }
        )
    return {"library": "pillow", "frames": frames}


# endregion 🔖️Pillow


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own doc string. The Python `Context` exposes the raw plan, not a helper."""
    for step in ctx.scenario.get("steps", []):
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("%s declares no doc string" % ctx.scenario["id"])


def step_uris(ctx: Context, scheme: str) -> list:
    """🧫️ Every fixture URI of one scheme the scenario's steps name, in step order."""
    found = []
    for step in ctx.scenario.get("steps", []):
        for token in step["text"].split():
            if token.startswith(scheme):
                found.append(token)
    return found


def fixture_json(ctx: Context, uri: str):
    """🧫️ A declared fixture read as JSON."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


def artifact(ctx: Context) -> dict:
    """🎞️ The real committed artifact, parsed through this implementation's own DSL reader."""
    return parse_dsl(ctx.fixture_bytes(ARTIFACT_DSL).decode("utf-8"))


def projection_of(document: dict) -> dict:
    """🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own
    structural JSON shape, field for field, with every RGBA8 sample present rather than summarised."""
    return {
        "schema": document["schema"],
        "width": document["width"],
        "height": document["height"],
        "colorspace": document["colorspace"],
        "bitDepth": document["bitDepth"],
        "frames": [{"delayMs": frame["delayMs"], "rgba8": list(frame["rgba8"])} for frame in document["frames"]],
        "icc": None if document["icc"] is None else list(document["icc"]),
        "metadata": [{"key": entry["key"], "value": entry["value"]} for entry in document["metadata"]],
    }


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real derived artifact by this implementation alone, with the
    resulting planes handed to Pillow so the third party — and not this file — states what they are."""
    document = artifact(ctx)
    mutation = fixture_json(ctx, step_uris(ctx, "local://🦠️")[0])
    applied = apply_mutation(document, mutation)
    return Outcome({"document": projection_of(applied), "raster": pillow_report(applied)})


def inverse(ctx: Context) -> Outcome:
    """↩️ The metamorphic inverse law on the real artifact: the verb followed by its OWN computed
    inverse must restore the artifact exactly, frame order and every sample included."""
    document = artifact(ctx)
    mutation = fixture_json(ctx, step_uris(ctx, "local://🦠️")[0])
    undo = inverse_mutation(document, mutation)
    mutated = apply_mutation(document, mutation)
    restored = apply_mutation(mutated, undo)
    if restored != document:
        raise AssertionError("%s: undoing the mutation did not restore the artifact" % ctx.scenario["id"])
    return Outcome({"mutated": projection_of(mutated), "restored": projection_of(restored)})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    statement of what the verb means, independent of both implementations."""
    uris = step_uris(ctx, "asset://")
    before = fixture_json(ctx, uris[0])
    mutation = fixture_json(ctx, uris[1]) if len(uris) > 2 else json.loads(doc_string(ctx))
    expected = fixture_json(ctx, uris[2]) if len(uris) > 2 else before
    applied = apply_mutation(before, mutation)
    if applied != expected:
        raise AssertionError("%s: the applied snapshot is not the committed after-snapshot" % ctx.scenario["id"])
    return Outcome(projection_of(applied))


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real artifact, each re-emitted from the parsed document,
    with the reconstructed planes handed to Pillow for an independent statement of what they are.

    🔒️ The BYTE half of the identity law. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` its binary twin, so reproducing both files byte for byte is the CORRECT answer here
    and a must-differ tripwire would be exactly backwards. What stops that from being a codec
    agreeing with itself is that the Rust subject reproduces the same two files from its own reading
    of the same grammar, and the digests of what each side emitted are what the runner compares.
    """
    dsl_bytes = ctx.fixture_bytes(ARTIFACT_DSL)
    parsed = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(parsed).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("identity-round-trip: re-printing the parsed artifact did not reproduce the committed DSL file")
    pack = ctx.fixture_bytes(ARTIFACT_PACK)
    unpacked = parse_pack(pack)
    if unpacked != parsed:
        raise AssertionError("identity-round-trip: the committed binary twin decodes to a different image than the committed text artifact")
    repacked = pack_bytes(parsed)
    if repacked != pack:
        raise AssertionError("identity-round-trip: re-encoding the parsed artifact did not reproduce the committed pack file")
    return Outcome(
        {
            "document": projection_of(parsed),
            "raster": pillow_report(parsed),
            "dslDigest": digest(printed),
            "packDigest": digest(repacked),
            "dslLength": len(printed),
            "packLength": len(repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the Python host calls, keyed by FULL expanded scenario id."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
