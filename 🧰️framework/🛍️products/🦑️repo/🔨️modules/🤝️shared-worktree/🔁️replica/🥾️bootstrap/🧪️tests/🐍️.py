#!/usr/bin/env python3
"""🐍️ Independent oracle of the bootstrap archive of the shared worktree fleet: it reads `<store>/v1/bootstrap/worktree.tar` with Python's standard `tarfile` and `manifest.json` with `json`, and shares no code with `../🟦️.ts`.

usage: python3 -I oracle.py check STORE [EXPECTED.json]
       checks that manifest.json has exactly version, hlc, files, bytes and sha256, that the sha256 is the hash of the tar, that the tar is a multiple of 512 bytes
       and ends with two zero blocks, that every member is a regular file with a safe NFC name, that files and bytes equal the members, and, when EXPECTED.json
       (a list of {"path","bytes","sha256","executable"}) is given, that names, sizes, content hashes and executable bits are exactly those; exit 1 with one line per problem.
"""
import hashlib
import json
import os
import re
import sys
import tarfile
import unicodedata

BLOCK = 512


def digest_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_members(path, problems):
    found = {}
    with tarfile.open(path, "r:") as archive:
        for member in archive:
            name = member.name
            if not member.isreg():
                problems.append("not a regular file: " + name)
                continue
            parts = name.split("/")
            if name.startswith("/") or "\\" in name or any(part in ("", ".", "..") for part in parts):
                problems.append("unsafe name: " + name)
            if unicodedata.normalize("NFC", name) != name:
                problems.append("name is not NFC: " + name)
            if name in found:
                problems.append("duplicate name: " + name)
            digest = hashlib.sha256()
            count = 0
            stream = archive.extractfile(member)
            for chunk in iter(lambda: stream.read(1 << 20), b""):
                digest.update(chunk)
                count += len(chunk)
            if count != member.size:
                problems.append("size differs from the content: " + name)
            found[name] = (member.size, digest.hexdigest(), bool(member.mode & 0o111))
    return found


def check(store, expected_path):
    problems = []
    base = os.path.join(store, "v1", "bootstrap")
    with open(os.path.join(base, "manifest.json"), encoding="utf-8") as handle:
        manifest = json.load(handle)
    if sorted(manifest) != ["bytes", "files", "hlc", "sha256", "version"]:
        problems.append("manifest keys: " + ",".join(sorted(manifest)))
        return problems
    if manifest["version"] != 1:
        problems.append("manifest version")
    if not re.fullmatch(r"[0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9-]{0,39}", manifest["hlc"]):
        problems.append("manifest hlc")
    if not re.fullmatch(r"[0-9a-f]{64}", manifest["sha256"]):
        problems.append("manifest sha256 shape")
    tar_path = os.path.join(base, "worktree.tar")
    if digest_file(tar_path) != manifest["sha256"]:
        problems.append("the sha256 of the tar differs from manifest.json")
    size = os.path.getsize(tar_path)
    if size % BLOCK != 0 or size < 2 * BLOCK:
        problems.append("the tar is not a multiple of 512 bytes")
    else:
        with open(tar_path, "rb") as handle:
            handle.seek(size - 2 * BLOCK)
            if handle.read() != bytes(2 * BLOCK):
                problems.append("the tar does not end with two zero blocks")
    found = read_members(tar_path, problems)
    if manifest["files"] != len(found):
        problems.append("files: manifest says %d, the tar holds %d" % (manifest["files"], len(found)))
    if manifest["bytes"] != sum(item[0] for item in found.values()):
        problems.append("bytes: manifest says %d, the members hold %d" % (manifest["bytes"], sum(item[0] for item in found.values())))
    if expected_path is not None:
        with open(expected_path, encoding="utf-8") as handle:
            expected = {item["path"]: (item["bytes"], item["sha256"], item["executable"]) for item in json.load(handle)}
        for name in sorted(set(expected) - set(found)):
            problems.append("missing from the tar: " + name)
        for name in sorted(set(found) - set(expected)):
            problems.append("unexpected in the tar: " + name)
        for name in sorted(set(found) & set(expected)):
            if found[name] != expected[name]:
                problems.append("differs: %s %r != %r" % (name, found[name], expected[name]))
    return problems


def main(argv):
    if len(argv) not in (3, 4) or argv[1] != "check":
        sys.stderr.write(__doc__)
        return 2
    problems = check(argv[2], argv[3] if len(argv) == 4 else None)
    for problem in problems:
        sys.stderr.write(problem + "\n")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
