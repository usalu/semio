"""Independent derivation, in Python's standard library only, of the slot parts and the slot id of a rustc command line, and of the portable text of the unit-cache fleet tier. usage: python3 -I oracle.py slots|portable (JSON on stdin, JSON on stdout)."""
import hashlib
import json
import os
import re
import sys


def codegen(args):
    rows = {}
    i = 0
    while i < len(args):
        a = args[i]
        value = None
        if a in ("-C", "--codegen"):
            i += 1
            value = args[i] if i < len(args) else None
        elif a.startswith("-C") and len(a) > 2:
            value = a[2:]
        if value is not None:
            k, _, v = value.partition("=")
            rows[k] = v
        i += 1
    return rows


def many(args, flag):
    rows = []
    i = 0
    while i < len(args):
        a = args[i]
        if a == flag:
            i += 1
            rows.append(args[i] if i < len(args) else "")
        elif a.startswith(flag + "="):
            rows.append(a[len(flag) + 1:])
        i += 1
    return rows


def first(args, flag):
    rows = many(args, flag)
    return rows[0] if rows else None


def parts_of(vector):
    args, env = vector["arguments"], vector["environment"]
    flags = codegen(args)
    cfgs = many(args, "--cfg")
    features = sorted(re.sub(r'^"|"$', "", c[len("feature="):]) for c in cfgs if c.startswith("feature="))
    others = sorted(c for c in cfgs if not c.startswith("feature="))
    manifest = env.get("CARGO_MANIFEST_DIR")
    member = ""
    if manifest and manifest != vector["cwd"] and os.path.isabs(manifest):
        rel = os.path.relpath(manifest, vector["cwd"])
        if not rel.startswith(".."):
            member = "#" + rel.replace("\\", "/")
    incremental = flags.get("incremental") is not None
    profile = ";".join([
        "opt=" + flags.get("opt-level", ""), "debug=" + flags.get("debuginfo", ""), "da=" + flags.get("debug-assertions", ""),
        "oc=" + flags.get("overflow-checks", ""), "lto=" + flags.get("lto", ""), "panic=" + flags.get("panic", ""),
        "cgu=" + flags.get("codegen-units", ""), "inc=" + ("1" if incremental else "0"),
    ])
    types = sorted(t for v in many(args, "--crate-type") for t in v.split(",")) or ["bin"]
    emit = sorted(set(p.split("=")[0] for v in many(args, "--emit") for p in v.split(",")))
    return [
        vector["rustc"], first(args, "--target") or vector["host"], profile,
        env.get("CARGO_PKG_NAME", "") + "@" + env.get("CARGO_PKG_VERSION", "") + member,
        first(args, "--crate-name") or "", ",".join(types) + "/" + ",".join(emit), ",".join(features), ",".join(others),
    ]


def slot_of(parts):
    return hashlib.sha256(json.dumps(["semio-slot-v1", "rust-unit"] + parts, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()


def portable(request):
    roots = sorted(request["roots"], key=lambda r: -len(r["path"]))
    alternatives = []
    lookup = {}
    for root in roots:
        for form in {root["path"], root["path"].replace("\\", "/"), root["path"].replace("\\", "\\\\")}:
            if form not in lookup:
                lookup[form] = root["token"]
                alternatives.append(re.escape(form))
    alternatives.sort(key=lambda x: -len(x))
    pattern = re.compile("(?:" + "|".join(alternatives) + ")(?![A-Za-z0-9._-])")
    return [pattern.sub(lambda m: lookup[m.group(0)], text) for text in request["texts"]]


if __name__ == "__main__":
    request = json.load(sys.stdin)
    if sys.argv[1] == "slots":
        out = []
        for vector in request:
            parts = parts_of(vector)
            out.append({"parts": parts, "slot": slot_of(parts)})
        json.dump(out, sys.stdout, ensure_ascii=False)
    elif sys.argv[1] == "portable":
        json.dump(portable(request), sys.stdout, ensure_ascii=False)
