import { selectedCargoLockDigest } from "../🔒️lock/📥️read/🟨️.mjs";
import { createHash } from "node:crypto";
import { readFileSync, lstatSync } from "node:fs";
import { isAbsolute, resolve, relative, join } from "node:path";

/** 🧮️ Hashes current bytes and absence from an exact source inventory independently of graph serialization. */
export function sourceClosureDigest(workspaceRoot, value, files = new Map()) {
  if (!value || Object.keys(value).length !== 2 || value.version !== 1 || !Array.isArray(value.files) || new Set(value.files).size !== value.files.length) throw Error("Exact source closure required");
  const digest = createHash("sha256");
  for (const path of [...value.files].sort()) {
    if (typeof path !== "string" || !path || path.length > 256 || path.includes("\0") || /^[A-Za-z]:/u.test(path) || isAbsolute(path) || path.includes("\\") || path.split("/").some(part => ["", ".", ".."].includes(part))) throw Error("Source input must be repository relative");
    const absolute = resolve(workspaceRoot, path);
    if (relative(workspaceRoot, absolute).startsWith("..")) throw Error("Source input escapes workspace");
    digest.update(path).update("\0");
    try {
      const status = lstatSync(absolute);
      if (!status.isFile() && !status.isSymbolicLink()) throw Error(`Source input must be a file: ${path}`);
      if (!files.has(absolute)) files.set(absolute, createHash("sha256").update(readFileSync(absolute)).digest("hex"));
      digest.update("file\0").update(files.get(absolute)).update("\0");
    } catch (error) {
      if (!error || typeof error !== "object" || !("code" in error) || !["ENOENT", "ENOTDIR"].includes(String(error.code))) throw error;
      digest.update("missing\0");
    }
  }
  return digest.digest("hex");
}

/** 🏗️ Fills private exact-source hash inputs in process before the native Nx hash batch. */
export function sourceClosureTaskEnvironments(workspaceRoot, plans, environments) {
  const values = new Map(), files = new Map(), lockRecords = new Map(), result = {};
  const prefix = "env:SEMIO_SOURCE_CLOSURE_";
  for (const [id, instructions] of Object.entries(plans)) {
    const environment = { ...environments[id] };
    for (const instruction of instructions) {
      if (instruction.startsWith("env:SEMIO_CARGO_LOCK_")) {
        const key = instruction.slice("env:SEMIO_CARGO_LOCK_".length);
        if (!/^[a-f0-9]{64}$/u.test(key)) throw Error("Invalid Cargo lock input identity");
        const name = instruction.slice(4);
        if (!values.has(name)) {
          const bytes = readFileSync(join(workspaceRoot, ".🧬semio/🦑️repo/⚡️cache/nx-lock-closures", `${key}.json`));
          if (createHash("sha256").update(bytes).digest("hex") !== key) throw Error("Cargo lock inventory identity changed");
          const contract = JSON.parse(bytes);
          if (Object.keys(contract).length !== 2 || contract.version !== 1 || typeof contract.manifest !== "string" || contract.manifest.length > 256 || contract.manifest.includes("\0") || /^[A-Za-z]:/u.test(contract.manifest) || isAbsolute(contract.manifest) || contract.manifest.includes("\\") || contract.manifest.split("/").some(part => ["", ".", ".."].includes(part))) throw Error("Cargo lock inventory must be workspace relative");
          values.set(name, selectedCargoLockDigest(join(workspaceRoot, contract.manifest), lockRecords));
        }
        environment[name] = values.get(name);
        continue;
      }
      if (!instruction.startsWith(prefix)) continue;
      const key = instruction.slice(prefix.length);
      if (!/^[a-f0-9]{64}$/u.test(key)) throw Error("Invalid exact source input identity");
      if (!values.has(key)) {
        const manifest = join(workspaceRoot, ".🧬semio/🦑️repo/⚡️cache/nx-input-closures", `${key}.json`), bytes = readFileSync(manifest);
        if (createHash("sha256").update(bytes).digest("hex") !== key) throw Error("Exact source inventory identity changed");
        values.set(key, sourceClosureDigest(workspaceRoot, JSON.parse(bytes), files));
      }
      environment[instruction.slice(4)] = values.get(key);
    }
    result[id] = environment;
  }
  return result;
}

/** 🔐️ Refuses promotion if any exact selected source changed while the task ran. */
export function sourceClosureReceiptHolds(workspaceRoot, receipt) {
  if (receipt === undefined) return true;
  if (!receipt || Object.keys(receipt).length !== 2 || receipt.version !== 1 || !receipt.values || typeof receipt.values !== "object" || Array.isArray(receipt.values)) throw Error("Exact source receipt required");
  const names = Object.keys(receipt.values);
  if (names.some(name => !/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}$/u.test(name) || !/^[a-f0-9]{64}$/u.test(receipt.values[name]))) throw Error("Exact source receipt identity refused");
  const current = sourceClosureTaskEnvironments(workspaceRoot, { selected: names.map(name => `env:${name}`) }, { selected: {} }).selected;
  return names.every(name => current[name] === receipt.values[name]);
}
