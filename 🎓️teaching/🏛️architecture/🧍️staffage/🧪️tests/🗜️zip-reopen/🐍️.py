"""🐍️ Oracle for the PNG-sequence ZIP: python's zipfile lists, CRC-checks and reads it back."""
import json
import sys
import zipfile

with zipfile.ZipFile(sys.argv[1]) as archive:
    names = archive.namelist()
    print(json.dumps({
        "bad": archive.testzip(),
        "names": names,
        "stored": all(info.compress_type == zipfile.ZIP_STORED for info in archive.infolist()),
        "manifest": json.loads(archive.read("manifest.json")),
        "signatures": [archive.read(name)[:8].hex() for name in names if name.endswith(".png")],
    }))
