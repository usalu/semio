"""🔋️ The Energy Pack schema digest, Python side: the same fixture `🔣️.json` that `🟦️.ts` checks with @noble/hashes. The canonical schema bytes are rebuilt from the fields, hashed by the independent BLAKE3 of the stdio-semio record-document codec, cross-checked by @noble/hashes through bun when available, and must equal the digest constant the Rust pack carries. Run with `python3 <this file>`."""

import importlib.util
import json
import os
import re
import shutil
import subprocess
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
REPOSITORY = os.path.abspath(os.path.join(HERE, *[".."] * 14))
CODEC = os.path.join(REPOSITORY, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio", "🚪️io", "💾️binary", "🎒️record-document", "🐍️.py")
RUST = os.path.join(HERE, "..", "🦀️.rs")
with open(os.path.join(HERE, "🔣️.json"), encoding="utf-8") as handle:
    LAW = json.load(handle)
SHAPES = {"value": 0}


def load_codec():
    spec = importlib.util.spec_from_file_location("record_document", CODEC)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def canonical(fields):
    out = bytearray([1, len(fields)])
    for field in fields:
        key = field["key"].encode("utf-8")
        out += bytes([field["id"], len(key)]) + key + bytes([SHAPES[field["shape"]], 14])
    return bytes(out)


class EnergySchemaDigest(unittest.TestCase):
    def test_the_canonical_bytes_are_rebuilt_from_the_fields(self):
        self.assertEqual(list(canonical(LAW["fields"])), LAW["canonicalSchemaBytes"])

    def test_the_independent_blake3_hashes_them_to_the_committed_digest(self):
        self.assertEqual(load_codec().blake3(bytes(LAW["canonicalSchemaBytes"])).hex(), LAW["schemaHash"])

    def test_the_rust_constant_is_the_committed_digest(self):
        with open(RUST, encoding="utf-8") as handle:
            numbers = re.search(r"pub const SCHEMA_HASH:\[u8;32\]=\[([0-9,]+)\]", handle.read())
        self.assertIsNotNone(numbers)
        self.assertEqual(bytes(int(number) for number in numbers.group(1).split(",")).hex(), LAW["schemaHash"])

    @unittest.skipUnless(shutil.which("bun"), "bun is absent")
    def test_noble_hashes_agree(self):
        script = 'import {blake3} from "@noble/hashes/blake3.js";process.stdout.write(Buffer.from(blake3(Uint8Array.from(JSON.parse(process.argv[1])))).toString("hex"))'
        done = subprocess.run(["bun", "-e", script, json.dumps(LAW["canonicalSchemaBytes"])], cwd=HERE, capture_output=True, text=True)
        if done.returncode != 0:
            self.skipTest("@noble/hashes is not resolvable here")
        self.assertEqual(done.stdout, LAW["schemaHash"])


if __name__ == "__main__":
    unittest.main()
