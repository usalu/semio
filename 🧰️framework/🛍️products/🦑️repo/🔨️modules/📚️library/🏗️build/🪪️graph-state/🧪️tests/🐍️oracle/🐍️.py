import hashlib
import json
import sys

MASK = (1 << 256) - 1


def term(path, value=None):
    data = path if value is None else path + "\0" + value
    return int(hashlib.sha256(data.encode("utf-8")).hexdigest(), 16)


def state(frame, rows):
    names = 0
    content = 0
    files = 0
    content_files = 0
    for path, kind, sha in rows:
        if kind == "ignored":
            continue
        names = (names + term(path)) & MASK
        files += 1
        if kind == "content":
            content = (content + term(path, sha)) & MASK
            content_files += 1
    names_hex = format(names, "064x")
    content_hex = format(content, "064x")
    extra = [[key, frame["extra"][key]] for key in sorted(frame["extra"])]
    digest = hashlib.sha256(
        json.dumps(
            ["semio-graph-state-v1", frame["kind"], frame["tooling"], frame["implementation"], frame["platform"], extra, names_hex, content_hex, files, content_files],
            separators=(",", ":"),
            ensure_ascii=False,
        ).encode("utf-8")
    ).hexdigest()
    return {"names": names_hex, "content": content_hex, "files": files, "contentFiles": content_files, "digest": digest}


def generated(lines):
    return hashlib.sha256(json.dumps(["semio-graph-state-v1", "generated", lines], separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()


payload = json.load(sys.stdin)
if "generated" in payload:
    print(json.dumps([generated(["%s\0%s" % (path, sha) for path, sha in sorted(batch, key=lambda row: row[0].encode("utf-8"))]) for batch in payload["generated"]], separators=(",", ":")))
else:
    print(json.dumps([state(batch["frame"], batch["rows"]) for batch in payload["batches"]], separators=(",", ":")))
