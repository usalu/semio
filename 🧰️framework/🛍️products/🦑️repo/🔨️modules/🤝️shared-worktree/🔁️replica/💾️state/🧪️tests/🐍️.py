#!/usr/bin/env python3
"""💾️ Independent oracle of the binary replica state. It shares no code with `../🟦️.ts`: it reads `state.ckp` and `state.wal` with `struct` and `hashlib` only, recomputes the header check, the payload checksum, the path book hashes and the manifest digest of the shared files, and validates its own output with the third-party `jsonschema` library when it is installed.

usage: python3 -I oracle.py checkpoint FILE      prints the decoded checkpoint as JSON; exit 1 when the file is not a valid checkpoint
       python3 -I oracle.py wal FILE              prints generation, verdict, good offset and decoded records of a delta log as JSON
       python3 -I oracle.py vectors FIXTURE       replays the vectors of FIXTURE; exit 1 on the first disagreement
"""
import base64
import hashlib
import json
import os
import struct
import sys

MAGIC = b"SEMIOSNP"
WAL_MAGIC = b"SEMIOWAL"
REALM_BASE = ".cache/f/index-base"
REALM_ENTRIES = ".cache/f/index/"
WIDTH = {"u8": 1, "u16": 2, "u32": 4, "i32": 4, "f64": 8}
CODE = {"u8": "B", "u16": "H", "u32": "I", "i32": "i", "f64": "d"}
KINDS = {
    "meta": "f64", "fold": "u8", "book.starts": "f64", "book.lengths": "u16", "book.hashes": "u32", "book.folded": "u32", "book.pages": "u8",
    "files.flags": "u8", "files.ms": "f64", "files.counter": "u32", "files.origin": "u16", "files.bytes": "f64", "files.sha": "u8", "files.mtime": "f64", "files.seen": "f64", "files.targets": "u8",
    "queue.ids": "i32", "queue.flags": "u8", "queue.ms": "f64", "queue.counter": "u32", "queue.origin": "u16", "queue.bytes": "f64", "queue.sha": "u8", "queue.targets": "u8",
}
OPERATIONS = {1: "put", 2: "delete", 3: "link"}


class Corrupt(Exception):
    pass


def fnv(data):
    value = 2166136261
    for byte in data:
        value = ((value ^ byte) * 16777619) & 0xFFFFFFFF
    return value


def column(parts, name):
    raw = parts[name]
    kind = KINDS[name]
    count = len(raw) // WIDTH[kind]
    if count * WIDTH[kind] != len(raw):
        raise Corrupt("part " + name)
    return list(struct.unpack("<%d%s" % (count, CODE[kind]), raw))


def targets(raw):
    found = {}
    position = 0
    while position < len(raw):
        identifier, length = struct.unpack_from("<II", raw, position)
        found[identifier] = raw[position + 8:position + 8 + length].decode("utf-8")
        position += 8 + length
    return found


def read_checkpoint(data):
    if len(data) < 96 or data[:8] != MAGIC:
        raise Corrupt("header")
    if hashlib.sha256(data[:72]).digest()[:8] != data[72:80] or any(data[80:96]):
        raise Corrupt("header checksum")
    version, generation, paths, files, queue, parts_count = struct.unpack_from("<6I", data, 8)
    payload = struct.unpack_from("<Q", data, 32)[0]
    stored = data[40:72].hex()
    if version != 1 or payload != len(data) - 96:
        raise Corrupt("length")
    body = data[96:]
    if hashlib.sha256(body).hexdigest() != stored:
        raise Corrupt("checksum")
    directory = []
    for index in range(parts_count):
        entry = body[index * 32:(index + 1) * 32]
        name = entry[:24].split(b"\0", 1)[0].decode("ascii")
        directory.append((name, struct.unpack_from("<Q", entry, 24)[0]))
    position = parts_count * 32
    parts = {}
    for name, length in directory:
        parts[name] = body[position:position + length]
        position += length
    if position != len(body):
        raise Corrupt("directory")
    header = {"version": version, "generation": generation, "paths": paths, "files": files, "queue": queue, "parts": parts_count, "payloadBytes": payload, "sha256": stored}
    return header, directory, parts


def entry_of(flag, ms, counter, origin, size, sha, mtime, seen, target, origins, local):
    operation = OPERATIONS[(flag >> 1) & 3]
    entry = {"hlc": "%013d-%06d-%s" % (int(ms), counter, origins[origin]), "operation": operation}
    if operation == "put":
        entry["sha256"] = sha.hex()
        entry["bytes"] = size
        entry["executable"] = (flag & 8) != 0
    elif operation == "link":
        entry["target"] = target
    if local:
        entry["mtimeMs"] = mtime
        if flag & 16:
            entry["seenMs"] = seen
    return entry


def decode_checkpoint(data):
    header, directory, parts = read_checkpoint(data)
    meta = column(parts, "meta")
    origins = parts["origins"].decode("utf-8")
    origins = origins.split("\n") if origins else []
    lengths = column(parts, "book.lengths")
    starts = column(parts, "book.starts")
    hashes = column(parts, "book.hashes")
    pages = parts["book.pages"]
    if len(lengths) != header["paths"]:
        raise Corrupt("paths")
    names = []
    for identifier, length in enumerate(lengths):
        start = int(starts[identifier])
        raw = pages[start:start + length]
        if fnv(raw) != hashes[identifier]:
            raise Corrupt("book hash of path %d" % identifier)
        names.append(raw.decode("utf-8"))
    flags = column(parts, "files.flags")
    ms = column(parts, "files.ms")
    counter = column(parts, "files.counter")
    origin = column(parts, "files.origin")
    size = column(parts, "files.bytes")
    mtime = column(parts, "files.mtime")
    seen = column(parts, "files.seen")
    sha = parts["files.sha"]
    held = targets(parts["files.targets"])
    files = []
    digest = bytearray(32)
    shared = 0
    for identifier, flag in enumerate(flags):
        if not flag & 1:
            continue
        entry = entry_of(flag, ms[identifier], counter[identifier], origin[identifier], size[identifier], sha[identifier * 32:identifier * 32 + 32], mtime[identifier], seen[identifier], held.get(identifier), origins, True)
        path = names[identifier]
        files.append(dict(entry, path=path))
        if entry["operation"] != "delete" and path != REALM_BASE and not path.startswith(REALM_ENTRIES):
            value = entry["sha256"] if entry["operation"] == "put" else entry["target"]
            term = hashlib.sha256((path + "\0" + entry["operation"] + "\0" + value).encode("utf-8")).digest()
            for index in range(32):
                digest[index] ^= term[index]
            shared += 1
    queue = []
    ids = column(parts, "queue.ids")
    qflags = column(parts, "queue.flags")
    qms = column(parts, "queue.ms")
    qcounter = column(parts, "queue.counter")
    qorigin = column(parts, "queue.origin")
    qsize = column(parts, "queue.bytes")
    qsha = parts["queue.sha"]
    qtargets = targets(parts["queue.targets"])
    for row, identifier in enumerate(ids):
        entry = entry_of(qflags[row], qms[row], qcounter[row], qorigin[row], qsize[row], qsha[row * 32:row * 32 + 32], 0, 0, qtargets.get(identifier), origins, False)
        queue.append(dict(entry, path=names[identifier]))
    memo = {name[5:]: base64.b64encode(raw).decode("ascii") for name, raw in parts.items() if name.startswith("memo:")}
    files.sort(key=lambda item: item["path"].encode("utf-8"))
    queue.sort(key=lambda item: item["path"].encode("utf-8"))
    return {
        "header": header,
        "parts": [{"name": name, "length": length} for name, length in directory],
        "origins": origins,
        "files": files,
        "queue": queue,
        "memo": memo,
        "manifest": {"digest": digest.hex(), "shared": shared, "stored": bytes(parts["fold"]).hex(), "storedShared": int(meta[0])},
    }


def text_of(payload, position, wide):
    size = 4 if wide else 2
    length = struct.unpack_from("<I" if wide else "<H", payload, position)[0]
    start = position + size
    if start + length > len(payload):
        raise Corrupt("record")
    return payload[start:start + length].decode("utf-8"), start + length


def decode_records(payload):
    records = []
    position = 0
    while position < len(payload):
        kind = payload[position]
        position += 1
        if kind < 1 or kind > 6:
            raise Corrupt("kind")
        name, position = text_of(payload, position, False)
        if kind == 6:
            records.append({"kind": "memo", "name": name, "value": None})
            continue
        if kind == 5:
            length = struct.unpack_from("<I", payload, position)[0]
            position += 4
            records.append({"kind": "memo", "name": name, "value": base64.b64encode(payload[position:position + length]).decode("ascii")})
            position += length
            continue
        family = "files" if kind <= 2 else "queue"
        if kind % 2 == 0:
            records.append({"kind": family, "path": name, "entry": None})
            continue
        operation = OPERATIONS[payload[position]]
        flags = payload[position + 1]
        hlc_length = payload[position + 2]
        position += 3
        hlc = payload[position:position + hlc_length].decode("ascii")
        position += hlc_length
        mtime, seen, size = struct.unpack_from("<3d", payload, position)
        position += 24
        entry = {"hlc": hlc, "operation": operation}
        if operation == "put":
            entry["sha256"] = payload[position:position + 32].hex()
            entry["bytes"] = size
            entry["executable"] = (flags & 1) != 0
            position += 32
        elif operation == "link":
            entry["target"], position = text_of(payload, position, True)
        if flags & 4:
            entry["mtimeMs"] = mtime
        if flags & 2:
            entry["seenMs"] = seen
        records.append({"kind": family, "path": name, "entry": entry})
    return records


def scan_wal(data):
    if len(data) == 0:
        return {"generation": None, "verdict": "empty", "good": 0, "records": []}
    if len(data) < 16:
        return {"generation": None, "verdict": "torn", "good": 0, "records": []}
    version, generation = struct.unpack_from("<II", data, 8)
    if data[:8] != WAL_MAGIC or version != 1:
        return {"generation": None, "verdict": "corrupt", "good": 0, "records": []}
    records = []
    position = 16
    while position < len(data):
        if len(data) - position < 16:
            return {"generation": generation, "verdict": "torn", "good": position, "records": records}
        length, inverse = struct.unpack_from("<II", data, position)
        end = position + 16 + length
        if (length ^ inverse) != 0xFFFFFFFF:
            return {"generation": generation, "verdict": "torn" if not any(data[position:]) else "corrupt", "good": position, "records": records}
        if end > len(data):
            return {"generation": generation, "verdict": "torn", "good": position, "records": records}
        payload = data[position + 16:end]
        if hashlib.sha256(payload).digest()[:8] != data[position + 8:position + 16]:
            return {"generation": generation, "verdict": "torn" if end == len(data) else "corrupt", "good": position, "records": records}
        records.extend(decode_records(payload))
        position = end
    return {"generation": generation, "verdict": "clean", "good": position, "records": records}


def validate(value, definition):
    try:
        import jsonschema
    except ImportError:
        return True
    schema = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json"), encoding="utf-8"))
    jsonschema.validate(value, {"$schema": schema["$schema"], "definitions": schema["definitions"], "$ref": "#/definitions/" + definition})
    return True


def vectors(path):
    fixture = json.load(open(path, encoding="utf-8"))
    total = 0

    def check(condition, name):
        nonlocal total
        total += 1
        if not condition:
            print("MISMATCH " + name)
            sys.exit(1)

    for case in fixture["checkpoints"]:
        decoded = decode_checkpoint(base64.b64decode(case["base64"]))
        check(decoded == case["decoded"], "checkpoint " + case["name"])
        check(decoded["manifest"]["digest"] == decoded["manifest"]["stored"] and decoded["manifest"]["shared"] == decoded["manifest"]["storedShared"], "digest " + case["name"])
        check(validate(decoded, "Decoded"), "schema " + case["name"])
    for case in fixture["logs"]:
        scanned = scan_wal(base64.b64decode(case["base64"]))
        check(all(scanned[key] == case[key] for key in ("generation", "verdict", "good", "records")), "log " + case["name"])
    check(validate(fixture, "Fixture"), "fixture schema")
    print("ok %d vectors" % total)


def main():
    command, argument = sys.argv[1], sys.argv[2]
    if command == "vectors":
        vectors(argument)
        return
    data = open(argument, "rb").read()
    if command == "checkpoint":
        try:
            print(json.dumps(decode_checkpoint(data)))
        except (Corrupt, struct.error, UnicodeDecodeError, IndexError, KeyError):
            sys.exit(1)
    elif command == "wal":
        print(json.dumps(scan_wal(data)))
    else:
        sys.exit(2)


main()
