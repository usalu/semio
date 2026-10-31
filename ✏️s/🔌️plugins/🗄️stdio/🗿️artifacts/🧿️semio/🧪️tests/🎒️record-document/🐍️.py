"""🧪️ Language-agnostic laws of the shared record-document codec, Python side. `🟦️.ts` beside this file runs the same laws over the same `🧫️fixtures/🔣️.json`.

The Rust record pack of `stdio.semio` is the oracle: every committed `*.pack.semio` it wrote must decode to the JSON snapshot it came from and re-encode
to the same bytes, and the primitives (CRC-32C, BLAKE3, deflate) must agree with committed vectors and with the system zlib.

Run it with `python3 <this file>` or `pytest <this file>`.
"""

from __future__ import annotations

# region 🔖️Imports
import importlib.util
import json
import os
import struct
import sys
import unittest
import zlib

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))


def load_codec():
    """🧩️ Loads the codec module from its taxonomy path and registers it under an importable name."""
    path = os.path.join(ROOT, "🚪️io", "💾️binary", "🎒️record-document", "🐍️.py")
    spec = importlib.util.spec_from_file_location("record_document", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["record_document"] = module
    spec.loader.exec_module(module)
    return module


codec = load_codec()
with open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), "r", encoding="utf-8") as handle:
    FIXTURE = json.load(handle)

# endregion 🔖️Imports


# region 🔖️Primitives
def pattern(size: int) -> bytes:
    """🧵️ The fixture's deterministic byte pattern: byte `i` is `(i * 31 + 7) mod 256`."""
    return bytes((index * 31 + 7) & 0xFF for index in range(size))


class PrimitiveLaws(unittest.TestCase):
    def test_the_fixture_declares_its_schema(self) -> None:
        self.assertEqual(FIXTURE["schema"], "semio.stdio.record-document-laws/v1")

    def test_crc32c_matches_the_published_vectors(self) -> None:
        for row in FIXTURE["crc32c"]:
            self.assertEqual(codec.crc32c(bytes.fromhex(row["hex"])), row["crc"], row["hex"])

    def test_blake3_matches_the_third_party_vectors_across_chunk_and_tree_boundaries(self) -> None:
        for row in FIXTURE["blake3"]:
            self.assertEqual(codec.blake3(pattern(row["size"])).hex(), row["hash"], "size %d" % row["size"])

    def test_varints_round_trip_and_stay_canonical(self) -> None:
        for value in (0, 1, 127, 128, 300, 16383, 16384, 2**32, 2**63 - 1, 2**64 - 1):
            self.assertEqual(codec.Reader(codec.write_varint(value)).varint(), value)
        for value in (0, 1, -1, 63, -64, 2**62, -(2**63)):
            self.assertEqual(codec.Reader(codec.write_zigzag(value)).zigzag(), value)
        self.assertEqual(codec.write_zigzag(-1), b"\x01")
        self.assertEqual(codec.write_varint(300), b"\xac\x02")

    def test_deflate_reproduces_every_segment_the_rust_writer_produced(self) -> None:
        for row in FIXTURE["deflate"]:
            self.assertEqual(codec.deflate(bytes.fromhex(row["rawHex"])).hex(), row["deflatedHex"], row["source"])

    def test_deflate_output_is_read_back_by_the_system_zlib(self) -> None:
        for size in (0, 1, 3, 258, 259, 1000, 70000):
            raw = pattern(size) + bytes(size // 2)
            self.assertEqual(zlib.decompress(codec.deflate(raw), -15), raw, "size %d" % size)

    def test_a_corrupted_segment_is_refused_by_its_crc(self) -> None:
        frame = codec.encode_segment(codec.KIND_SYMBOLS, b"\x01\x01a")
        self.assertEqual(codec.decode_segment(codec.Reader(frame))[1], b"\x01\x01a")
        damaged = bytearray(frame)
        damaged[3] ^= 0x01
        with self.assertRaises(ValueError):
            codec.decode_segment(codec.Reader(bytes(damaged)))


# endregion 🔖️Primitives


# region 🔖️Packs
def read(path: str) -> bytes:
    with open(os.path.join(ROOT, *path.split("/")), "rb") as handle:
        return handle.read()


def spec_of(subset: dict):
    return codec.load_record_spec(os.path.join(ROOT, *subset["spec"].split("/")))


class SubsetLaws(unittest.TestCase):
    def test_every_exported_record_spec_hashes_to_the_hash_it_carries(self) -> None:
        for subset in FIXTURE["subsets"]:
            self.assertEqual(spec_of(subset).envelope, subset["envelope"], subset["folder"])

    def test_every_committed_rust_pack_decodes_and_re_encodes_to_its_own_bytes(self) -> None:
        count = 0
        for subset in FIXTURE["subsets"]:
            spec = spec_of(subset)
            for pack in subset["packs"]:
                data = read(pack)
                self.assertEqual(codec.encode_pack(spec, codec.decode_pack(spec, data)), data, pack)
                self.assertEqual(codec.encode_pack(spec, codec.decode_pack(spec, data, omit_nulls=True)), data, pack)
                count += 1
        self.assertEqual(count, FIXTURE["packCount"])

    def test_decoded_packs_equal_the_snapshot_json_the_rust_derive_wrote(self) -> None:
        pairs = 0
        for subset in FIXTURE["subsets"]:
            spec = spec_of(subset)
            for pair in subset.get("snapshots", []):
                expected = json.loads(read(pair["json"]).decode("utf-8"))
                self.assertEqual(codec.decode_pack(spec, read(pair["pack"]), narrow_f32=True), expected, pair["pack"])
                self.assertEqual(codec.encode_pack(spec, expected), read(pair["pack"]), pair["pack"])
                pairs += 1
        self.assertGreaterEqual(pairs, 1)

    def test_a_pack_of_another_subset_is_refused(self) -> None:
        first, second = FIXTURE["subsets"][0], FIXTURE["subsets"][1]
        with self.assertRaises(ValueError):
            codec.decode_pack(spec_of(first), read(second["packs"][0]))

    def test_a_flipped_body_byte_is_refused(self) -> None:
        subset = FIXTURE["subsets"][0]
        data = bytearray(read(subset["packs"][0]))
        data[len(data) // 2] ^= 0x40
        with self.assertRaises(ValueError):
            codec.decode_pack(spec_of(subset), bytes(data))

    def test_a_document_that_names_an_unknown_key_or_variant_is_refused(self) -> None:
        subset = next(row for row in FIXTURE["subsets"] if row["folder"].endswith("model"))
        spec = spec_of(subset)
        document = codec.decode_pack(spec, read(subset["packs"][0]))
        with self.assertRaises(ValueError):
            codec.encode_pack(spec, {**document, "unknown": 1})
        with self.assertRaises(ValueError):
            codec.encode_pack(spec, {**document, "spatial": [{**document["spatial"][0], "kind": "galaxy"}]})


# endregion 🔖️Packs

# region 🔖️Hostile
HOSTILE_VALUE_HEX = "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030303016300001d75fe6e04034b62636414146067e78c8f2f28ca2fc98f8f676164674ce461646162674eca4c67f90f05f5ec2ce59929a970fe7f4676e69cfc7266243e6b716e624e0e0b2b3b6b5a4e7e62092b03187cb007003a8adaca0103302e6348addadbcc79f048cc5ecb99c92f2dfe2c8f5ec35bbc5480cd7a99d49d951b9fae38a51dcca8c0cd0004498c0c0c00d1e7c4d10000007aa3646053504b464f4f543101000000010000007e0000000000000038000000000000001101000000000000a5fd061bc8a594b70e83de0ccdff6158fac82066ef6db399f4ff289b53d97e020000000000000000b66f0ae9"


def hostile_spec():
    """🌾️ A two-record spec (a dynamic `value` field and a `table` field) hashed from its own graph."""
    graph = [
        [
            {"id": 1, "key": "v", "optional": False, "flatten": False, "shape": {"kind": "value"}},
            {"id": 2, "key": "rows", "optional": True, "flatten": False, "shape": {"kind": "table", "record": 1}},
        ],
        [{"id": 1, "key": "n", "optional": False, "flatten": False, "shape": {"kind": "uint"}}],
    ]
    document = {"schema": "semio.pack.record-spec/v1", "envelope": "hostile", "schemaHash": "", "graph": graph}
    document["schemaHash"] = codec.RecordSpec(document).schema_hash().hex()
    return codec.RecordSpec(document)


def hostile_pack(spec, body: bytes) -> bytes:
    return codec.build_pack(spec.schema_hash(), bytes([0]), body, [])


class HostileLaws(unittest.TestCase):
    def test_an_inflated_segment_is_capped_at_the_size_it_declares(self) -> None:
        compressor = zlib.compressobj(9, zlib.DEFLATED, -15)
        bomb = compressor.compress(bytes(1 << 24)) + compressor.flush()
        self.assertLess(len(bomb), 1 << 15)
        with self.assertRaises(ValueError):
            codec.inflate(bomb, 16)
        with self.assertRaises(ValueError):
            codec.inflate(bomb, 2**40)
        self.assertEqual(len(codec.inflate(bomb, 1 << 24)), 1 << 24)

    def test_a_varint_wider_than_64_bits_is_refused(self) -> None:
        with self.assertRaises(ValueError):
            codec.Reader(bytes([0x80] * 9 + [0x02])).varint()
        self.assertEqual(codec.Reader(bytes([0xFF] * 9 + [0x01])).varint(), 2**64 - 1)

    def test_a_table_that_declares_more_rows_than_the_bytes_can_hold_is_refused(self) -> None:
        spec = hostile_spec()
        body = bytes([1, 2, 0x14]) + codec.write_varint(2**40) + codec.write_varint(0)
        with self.assertRaises(ValueError):
            codec.decode_record(spec, hostile_pack(spec, body))

    def test_dynamic_values_keep_a_proto_key_and_every_64_bit_integer_exact_byte_equal_to_typescript(self) -> None:
        spec = hostile_spec()
        value = {"__proto__": 1, "a": [2], "big": 2**63 - 1, "wide": 2**64 - 1, "low": -(2**63), "small": 5, "float": codec.Float(1.0)}
        inner = codec.encode_record(spec, {1: ("value", value)})
        self.assertEqual(inner.hex(), HOSTILE_VALUE_HEX)
        back = codec.decode_record(spec, inner)[1][1]
        self.assertEqual(list(back), list(value))
        self.assertEqual([back["big"], back["wide"], back["low"], back["small"]], [2**63 - 1, 2**64 - 1, -(2**63), 5])
        self.assertEqual(codec.encode_record(spec, {1: ("value", back)}).hex(), HOSTILE_VALUE_HEX)


# endregion 🔖️Hostile


if __name__ == "__main__":
    unittest.main()
