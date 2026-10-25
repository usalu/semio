#!/usr/bin/env python3
"""Independent model of the Nx pinned generation identity: sha256 over `<byte length>:<bytes>` frames of the identity files, `!<path>` for a missing file."""
import hashlib, json, os, sys

FILES = json.load(open(sys.argv[2]))["identityFiles"]
digest = hashlib.sha256()
for path in FILES:
    try:
        data = open(os.path.join(sys.argv[1], path), "rb").read()
    except OSError:
        data = ("!" + path).encode()
    digest.update(str(len(data)).encode() + b":" + data)
print(digest.hexdigest())
