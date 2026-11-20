from pathlib import Path
import json
import sys

print(json.dumps(Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()))
