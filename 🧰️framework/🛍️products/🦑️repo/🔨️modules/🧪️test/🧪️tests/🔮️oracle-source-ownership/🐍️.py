"""🔮️ Inspects the committed Norm adapters through the real Python test host."""

import ast
import importlib.util
import json
import pathlib
import sys


def inspect(root, control):
    """🧭️ Reports native import identity, registration and adapter declarations."""
    manifest = json.loads((root / control["manifest"]).read_text(encoding="utf-8"))
    declaration = next(row for row in manifest["oracleHostPackages"] if row["implementation"] == "python")
    sys.path.insert(0, str(root / declaration["path"]))
    spec = importlib.util.spec_from_file_location("semio_norm_owner_host", root / control["host"])
    host = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = host
    spec.loader.exec_module(host)
    rows = []
    for entry in control["adapters"]:
        path = root / entry["path"]
        tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
        selectors = [node.args[0].value for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "import_module" and node.args and isinstance(node.args[0], ast.Constant)]
        adapter = host._load_adapter(str(path))
        rows.append({"path": entry["path"], "selectors": selectors, "moduleFiles": [pathlib.Path(sys.modules[name].__file__).resolve().relative_to(root).as_posix() for name in selectors], "definitions": [node.name for node in tree.body if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))], "handlers": sorted(adapter._handlers)})
    return {"declaration": declaration, "adapters": rows}


if __name__ == "__main__":
    root = pathlib.Path(sys.argv[1]).resolve()
    control = json.loads(pathlib.Path(sys.argv[2]).read_text(encoding="utf-8"))
    print(json.dumps(inspect(root, control), ensure_ascii=False))
