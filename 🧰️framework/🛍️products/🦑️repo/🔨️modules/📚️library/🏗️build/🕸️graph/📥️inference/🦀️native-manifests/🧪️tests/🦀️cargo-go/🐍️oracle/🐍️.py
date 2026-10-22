"""🦀 Independent Python oracle of Cargo path dependency discovery (standard library only): given a parsed manifest and the parsed workspace table it lists the path dependencies the way Cargo resolves aliases, target tables and workspace inheritance. Requests arrive as JSON on stdin."""
import json, sys


def dependencies(manifest, workspace):
    out = []
    tables = [manifest] + list((manifest.get("target") or {}).values())
    for table in tables:
        for kind in ("dependencies", "build-dependencies", "dev-dependencies"):
            for alias, declaration in (table.get(kind) or {}).items():
                if not isinstance(declaration, dict):
                    continue
                inherited = declaration.get("workspace") is True
                dependency = (workspace.get("dependencies") or {}).get(alias) if inherited else declaration
                if isinstance(dependency, dict) and dependency.get("path"):
                    out.append({"name": dependency.get("package", alias), "path": dependency["path"], "workspace": inherited, "kind": kind})
    return out


print(json.dumps([dependencies(r["manifest"], r["workspace"]) for r in json.load(sys.stdin)["requests"]], ensure_ascii=False))
