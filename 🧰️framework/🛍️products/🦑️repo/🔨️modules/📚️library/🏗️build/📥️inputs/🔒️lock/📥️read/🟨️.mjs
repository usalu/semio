import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
import { cargoLockClosure } from "../🟨️.mjs";

/** 🔒️ Hashes the selected package's locked resolution without admitting unrelated registry entries. */
export function selectedCargoLockDigest(manifest, records = new Map()) {
  const parser = createRequire(import.meta.url)("@iarna/toml"), read = path => { if (!records.has(path)) records.set(path, parser.parse(readFileSync(path, "utf8"))); return records.get(path); };
  const packagePath = resolve(manifest), selected = read(packagePath);
  if (!selected.package?.name) throw Error("Selected Cargo package manifest required");
  let root = selected.package.workspace ? resolve(dirname(packagePath), selected.package.workspace) : dirname(packagePath);
  if (!selected.package.workspace) {
    while (!read(join(root, "Cargo.toml")).workspace) {
      const parent = dirname(root);
      if (parent === root) throw Error("Selected Cargo workspace owner required");
      root = parent;
      while (!existsSync(join(root, "Cargo.toml"))) {
        const ancestor = dirname(root);
        if (ancestor === root) throw Error("Selected Cargo workspace owner required");
        root = ancestor;
      }
    }
  }
  if (!existsSync(join(root, "Cargo.lock"))) return createHash("sha256").update("missing\0Cargo.lock").digest("hex");
  const workspace = read(join(root, "Cargo.toml")), version = selected.package.version?.workspace ? workspace.workspace?.package?.version : selected.package.version;
  return createHash("sha256").update(JSON.stringify(cargoLockClosure(read(join(root, "Cargo.lock")), [{ name: selected.package.name, version }]))).digest("hex");
}

