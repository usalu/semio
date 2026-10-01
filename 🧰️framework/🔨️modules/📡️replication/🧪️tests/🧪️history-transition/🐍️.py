"""🐍️ Independent (non-Rust, non-TypeScript) encoder of `semio.history.transition` payloads.

Written from the wire grammar alone (`tag varint | variant fields in declaration order`), with its own BLAKE3
(the reference algorithm, no library) for the trunk alternative id of a document and the content-addressed transition id
`transition-{hex16(blake3(hlc.actor varint | hlc.physical_ms varint | hlc.logical varint | payload bytes))}` — the
free-form actor string never enters the id. It generates the language-agnostic corpus `../../🔗️causal/🧫️fixtures/🧫️history-transition/🔣️.json` that the Rust codec
(`the_language_agnostic_fixture_matches_the_codec_byte_for_byte`) and the TypeScript encoder + Ajv test
(`../🧪️history-transition/🟦️.ts`) both check byte for byte.

Run from the repository root: `python3 "<this file>"` verifies the committed corpus equals this encoder's output
(exit 1 on any difference); `python3 "<this file>" --write` regenerates it.

@see ../../🔗️causal/🧬️schema/🔣️history-transition/🔣️.json
@see ../../🔗️causal/🔀️transition/🦀️.rs
"""

from __future__ import annotations

# region 🔖️Imports
import json
import struct
import sys
from pathlib import Path

# endregion 🔖️Imports

# region 🔖️Blake3
IV = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19]
PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]
CHUNK_START, CHUNK_END, PARENT, ROOT = 1, 2, 4, 8
M32 = 0xFFFFFFFF


def rotr(x: int, n: int) -> int:
    """🔄️ 32-bit right rotation."""
    return ((x >> n) | (x << (32 - n))) & M32


def mix(s: list[int], a: int, b: int, c: int, d: int, x: int, y: int) -> None:
    """🌀️ The quarter-round G."""
    s[a] = (s[a] + s[b] + x) & M32
    s[d] = rotr(s[d] ^ s[a], 16)
    s[c] = (s[c] + s[d]) & M32
    s[b] = rotr(s[b] ^ s[c], 12)
    s[a] = (s[a] + s[b] + y) & M32
    s[d] = rotr(s[d] ^ s[a], 8)
    s[c] = (s[c] + s[d]) & M32
    s[b] = rotr(s[b] ^ s[c], 7)


def compress(cv: list[int], m: list[int], counter: int, block_len: int, flags: int) -> list[int]:
    """🗜️ Seven rounds over one 64-byte block."""
    s = list(cv) + IV[:4] + [counter & M32, (counter >> 32) & M32, block_len, flags]
    for _ in range(7):
        mix(s, 0, 4, 8, 12, m[0], m[1]); mix(s, 1, 5, 9, 13, m[2], m[3]); mix(s, 2, 6, 10, 14, m[4], m[5]); mix(s, 3, 7, 11, 15, m[6], m[7])
        mix(s, 0, 5, 10, 15, m[8], m[9]); mix(s, 1, 6, 11, 12, m[10], m[11]); mix(s, 2, 7, 8, 13, m[12], m[13]); mix(s, 3, 4, 9, 14, m[14], m[15])
        m = [m[i] for i in PERMUTATION]
    return [s[i] ^ s[i + 8] for i in range(8)] + [s[i + 8] ^ cv[i] for i in range(8)]


def block_words(block: bytes) -> list[int]:
    """🧱️ Sixteen little-endian words of a zero-padded block."""
    return list(struct.unpack("<16I", block.ljust(64, b"\0")))


def chunk_output(data: bytes, counter: int) -> tuple:
    """📦️ The last block's compression inputs of one chunk (at most 1024 bytes)."""
    cv = IV
    blocks = [data[i:i + 64] for i in range(0, len(data), 64)] or [b""]
    for index, block in enumerate(blocks):
        flags = (CHUNK_START if index == 0 else 0) | (CHUNK_END if index == len(blocks) - 1 else 0)
        if index == len(blocks) - 1:
            return (cv, block_words(block), counter, len(block), flags)
        cv = compress(cv, block_words(block), counter, 64, flags)[:8]
    raise AssertionError("unreachable")


def node(data: bytes, counter: int) -> tuple:
    """🌳️ A subtree's output: a chunk, or a parent over the largest power-of-two left subtree."""
    if len(data) <= 1024:
        return chunk_output(data, counter)
    left = 1 << (((len(data) + 1023) // 1024 - 1).bit_length() - 1)
    return (IV, compress(*node(data[:left * 1024], counter))[:8] + compress(*node(data[left * 1024:], counter + left))[:8], 0, 64, PARENT)


def blake3(data: bytes) -> bytes:
    """#⃣ The 32-byte BLAKE3 hash."""
    cv, m, counter, block_len, flags = node(data, 0)
    return struct.pack("<8I", *compress(cv, m, counter, block_len, flags | ROOT)[:8])


assert blake3(b"").hex() == "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
assert blake3(b"abc").hex() == "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
# endregion 🔖️Blake3

# region 🔖️Grammar
SUPERSEDE_SCOPE_MAX_BYTES = 256
SUPERSEDE_PAYLOAD_MAX_BYTES = 262_144


def varint(n: int) -> bytes:
    """🔢️ Unsigned LEB128."""
    out = bytearray()
    while True:
        b = n & 0x7F
        n >>= 7
        if n:
            out.append(b | 0x80)
        else:
            out.append(b)
            return bytes(out)


def s(v: str) -> bytes:
    """🔤️ `varint length | utf-8 bytes`."""
    b = v.encode("utf-8")
    return varint(len(b)) + b


def opt(v: str | None) -> bytes:
    """❔️ `0` or `1 | string`."""
    return b"\x00" if v is None else b"\x01" + s(v)


def ids(v: list[str]) -> bytes:
    """🆔️ `varint count | string*`."""
    return varint(len(v)) + b"".join(s(i) for i in v)


def replacement(r: dict) -> bytes:
    """♻️ `0 | schema string | payload bytes` for an input, `1` for a withdrawal."""
    if r["kind"] == "input":
        payload = bytes.fromhex(r["payloadHex"])
        return b"\x00" + s(r["schema"]) + varint(len(payload)) + payload
    if r["kind"] == "withdrawn":
        return b"\x01"
    raise ValueError(r["kind"])


def encode(t: dict) -> bytes:
    """🎯️ One transition payload."""
    k = t["kind"]
    if k == "revert":
        return varint(0) + ids(t["mutationIds"])
    if k == "reinstate":
        return varint(1) + ids(t["mutationIds"])
    if k == "commit":
        out = varint(2) + s(t["checkpointId"]) + opt(t["parentId"]) + s(t["changeId"]) + ids(t["mutationIds"]) + opt(t["description"]) + s(t["savedAt"])
        out += varint(len(t["authors"])) + b"".join(s(a["id"]) + s(a["name"]) + opt(a["avatar"]) for a in t["authors"])
        return out + opt(t["message"]) + s(t["timestamp"]) + opt(t["lineId"])
    if k == "branch":
        return varint(3) + s(t["alternativeId"]) + s(t["name"]) + s(t["checkpointId"])
    if k == "checkout":
        return varint(4) + s(t["checkpointId"]) + opt(t["alternativeId"])
    if k == "repin":
        return varint(5) + s(t["checkpointId"]) + s(t["pinnedCheckpointId"]) + varint(len(t["pins"])) + b"".join(s(p["childUri"]) + s(p["checkpointId"]) for p in t["pins"])
    if k == "supersede":
        return varint(6) + opt(t["scope"]) + varint(len(t["inputs"])) + b"".join(s(i["target"]) + replacement(i["replacement"]) for i in t["inputs"])
    raise ValueError(k)


ID_CLOCK = {"actor": 2557761449, "physicalMs": 1790622765027, "logical": 3}


def transition_id(payload: bytes) -> str:
    """🪪️ The content-addressed id of `payload` authored at `ID_CLOCK`."""
    material = varint(ID_CLOCK["actor"]) + varint(ID_CLOCK["physicalMs"]) + varint(ID_CLOCK["logical"]) + varint(len(payload)) + payload
    return "transition-" + blake3(material)[:8].hex()


KINDS = ["revert", "reinstate", "commit", "branch", "checkout", "repin", "supersede"]


def admits(shape: str, kind: str) -> bool:
    """🗂️ Whether a history of `shape` holds a `kind` transition: a document every kind, a config only undo and redo."""
    return shape == "document" or kind in ("revert", "reinstate")


def trunk_id(document_id: str) -> str:
    """🌳️ The id of `document_id`'s trunk, the implicit root alternative: `trunk-{hex16(blake3(str "semio.history.trunk" | str document_id))}`."""
    return "trunk-" + blake3(s("semio.history.trunk") + s(document_id))[:8].hex()


# endregion 🔖️Grammar

# region 🔖️Corpus
def corpus() -> dict:
    """🧫️ Every accepted and malformed case, in a stable order."""
    long_id = "op-" + "x" * 140
    accepted = [
        ("revert-one", {"kind": "revert", "mutationIds": ["op-a-1"]}),
        ("revert-many", {"kind": "revert", "mutationIds": ["op-a-1", "op-a-2", "op-a-3"]}),
        ("reinstate-long-id", {"kind": "reinstate", "mutationIds": [long_id]}),
        ("commit-full", {"kind": "commit", "checkpointId": "checkpoint-2", "parentId": "checkpoint-1", "changeId": "change-2", "mutationIds": ["op-a-1", "op-b-1"], "description": "Tiles and source", "savedAt": "2026-09-19T12:00:00Z", "authors": [{"id": "actor-a", "name": "Ada", "avatar": "https://example.test/ada.png"}, {"id": "actor-b", "name": "Bé 🧪", "avatar": None}], "message": "merge both replicas", "timestamp": "1789819200000", "lineId": "alternative-2"}),
        ("commit-minimal", {"kind": "commit", "checkpointId": "checkpoint-1", "parentId": None, "changeId": "change-1", "mutationIds": [], "description": None, "savedAt": "", "authors": [], "message": None, "timestamp": "", "lineId": None}),
        ("branch", {"kind": "branch", "alternativeId": "alternative-2", "name": "Variant B", "checkpointId": "checkpoint-1"}),
        ("checkout-alternative", {"kind": "checkout", "checkpointId": "checkpoint-1", "alternativeId": "alternative-2"}),
        ("checkout-plain", {"kind": "checkout", "checkpointId": "checkpoint-2", "alternativeId": None}),
        ("repin", {"kind": "repin", "checkpointId": "checkpoint-2", "pinnedCheckpointId": "checkpoint-2-pinned", "pins": [{"childUri": "semio://child/a", "checkpointId": "child-checkpoint-4"}, {"childUri": "semio://child/b", "checkpointId": "child-checkpoint-1"}]}),
        ("supersede-input", {"kind": "supersede", "scope": None, "inputs": [{"target": "op-a-1", "replacement": {"kind": "input", "schema": "demo/v1", "payloadHex": "010205"}}]}),
        ("supersede-withdrawn-scoped", {"kind": "supersede", "scope": "alternative-2", "inputs": [{"target": "op-b-1", "replacement": {"kind": "withdrawn"}}]}),
        ("supersede-many", {"kind": "supersede", "scope": None, "inputs": [{"target": "op-a-1", "replacement": {"kind": "input", "schema": "puzzle.2d", "payloadHex": "0103" + "ff" * 130}}, {"target": "op-b-1", "replacement": {"kind": "withdrawn"}}, {"target": "op-c-1", "replacement": {"kind": "input", "schema": "demo/v1", "payloadHex": ""}}]}),
    ]
    revert = encode(accepted[0][1])
    withdraw_a = s("op-a-1") + b"\x01"
    malformed = [
        ("empty", b"", "truncated"),
        ("unknown-tag", varint(7), "unknown transition tag 7"),
        ("trailing-bytes", revert + b"\x00", "trailing bytes"),
        ("truncated-id", revert[:-1], "truncated"),
        ("invalid-option-tag", varint(4) + s("checkpoint-1") + b"\x02", "invalid option tag 2"),
        ("id-count-exceeds-payload", varint(0) + varint(1000), "id count exceeds payload"),
        ("supersede-no-input", varint(6) + opt(None) + varint(0), "supersede names no input"),
        ("supersede-repeated-target", varint(6) + opt(None) + varint(2) + withdraw_a + withdraw_a, "supersede repeats target op-a-1"),
        ("supersede-scope-too-long", varint(6) + opt("x" * (SUPERSEDE_SCOPE_MAX_BYTES + 1)) + varint(1) + withdraw_a, "supersede scope exceeds 256 bytes"),
        ("supersede-payload-too-long", varint(6) + opt(None) + varint(1) + s("op-a-1") + b"\x00" + s("demo/v1") + varint(SUPERSEDE_PAYLOAD_MAX_BYTES + 1), "supersede payload exceeds 262144 bytes"),
        ("supersede-invalid-replacement-tag", varint(6) + opt(None) + varint(1) + s("op-a-1") + b"\x02", "invalid replacement tag 2"),
        ("supersede-input-count-exceeds-payload", varint(6) + opt(None) + varint(1000), "input count exceeds payload"),
        ("supersede-truncated-payload", varint(6) + opt(None) + varint(1) + s("op-a-1") + b"\x00" + s("demo/v1") + varint(3) + b"\x01", "truncated"),
    ]
    return {
        "schema": "semio.history.transition.v1",
        "diffSchema": "semio.history.transition",
        "idClock": ID_CLOCK,
        "cases": [{"id": i, "payloadHex": encode(t).hex(), "expect": {"outcome": "accepted", "transition": t, "transitionId": transition_id(encode(t))}} for i, t in accepted]
        + [{"id": i, "payloadHex": b.hex(), "expect": {"outcome": "malformed", "detail": d}} for i, b, d in malformed],
        "trunks": [{"documentId": d, "alternativeId": trunk_id(d)} for d in ["doc", "doc-supersede-fold", "Grüße 🧪 document"]],
        "shapes": [{"shape": shape, "admits": [k for k in KINDS if admits(shape, k)]} for shape in ["document", "config"]],
    }


# endregion 🔖️Corpus

# region 🔖️Main
FIXTURE = Path(__file__).resolve().parent.parent.parent / "🔗️causal" / "🧫️fixtures" / "🧫️history-transition" / "🔣️.json"


def main(argv: list[str]) -> int:
    """🚦️ Verifies (default) or rewrites (`--write`) the committed corpus."""
    text = json.dumps(corpus(), indent=2, ensure_ascii=False) + "\n"
    if "--write" in argv:
        FIXTURE.write_text(text, encoding="utf-8")
        return 0
    if FIXTURE.read_text(encoding="utf-8") != text:
        sys.stderr.write(f"{FIXTURE} differs from the independent encoder's corpus; rerun with --write\n")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
# endregion 🔖️Main
