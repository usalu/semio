"""📥️ Independent Python oracle of the pinned Nx package acquisition (standard library only): it builds the gzip tar of a package with tarfile and gzip, derives its sha512 integrity, lists the entries a correct decoder must return and classifies a lock file row with regular expressions. Requests arrive as JSON on stdin."""
import base64, gzip, hashlib, io, json, re, sys, tarfile

LOCK_NAME = re.compile(r"nx@[0-9]+\.[0-9]+\.[0-9]+")
INTEGRITY = re.compile(r"sha512-[A-Za-z0-9+/]{86}==")


def archive(entries, tamper=False):
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode="w", format=tarfile.USTAR_FORMAT) as tar:
        for entry in entries:
            data = entry["text"].encode("utf8")
            info = tarfile.TarInfo(entry["path"])
            info.size, info.mode, info.mtime = len(data), entry.get("mode", 0o644), 0
            tar.addfile(info, io.BytesIO(data))
    out = io.BytesIO()
    with gzip.GzipFile(fileobj=out, mode="wb", mtime=0) as handle:
        handle.write(raw.getvalue())
    data = bytearray(out.getvalue())
    integrity = "sha512-" + base64.b64encode(hashlib.sha512(bytes(data)).digest()).decode()
    if tamper:
        data[-12] ^= 1
    return {"base64": base64.b64encode(bytes(data)).decode(), "integrity": integrity, "length": len(data),
            "entries": [{"path": e["path"].split("/", 1)[1], "text": e["text"], "executable": bool(e.get("mode", 0o644) & 0o111)} for e in entries]}


def lock_valid(lock):
    row = ((lock or {}).get("packages") or {}).get("nx") if isinstance(lock, dict) else None
    return isinstance(row, list) and len(row) == 4 and isinstance(row[0], str) and bool(LOCK_NAME.fullmatch(row[0])) and row[1] == "" and isinstance(row[3], str) and bool(INTEGRITY.fullmatch(row[3]))


request = json.load(sys.stdin)
if request["op"] == "archive":
    print(json.dumps(archive(request["entries"], request.get("tamper", False))))
else:
    print(json.dumps([lock_valid(lock) for lock in request["locks"]]))
