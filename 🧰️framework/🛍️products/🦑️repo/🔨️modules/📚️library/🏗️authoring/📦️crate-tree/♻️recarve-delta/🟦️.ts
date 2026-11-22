/** ♻️ Re-carve delta: reduces the writes of a re-carve to what actually changes, so a carved artifact that gains no source is left byte for byte as it is and a source that is added only touches the crates and wiring it needs. */
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { CarveFileWrite } from "../🟦️.ts";
import type { ArtifactCratePlan, PlannedCrate } from "../🗺️plan/🧾️model/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

type Item = { lead: string[]; line: string; children: Item[] | null };

/** 🧱️ The items of generated wiring text as an order-free tree: attribute and comment lines stay with the item they lead, blocks nest, siblings are sorted. Two texts that declare the same modules and re-exports in any order and any indentation have the same canonical form. */
export function canonicalWiring(text: string): string {
  const lines = text.split("\n").map((line) => line.trim()).filter(Boolean);
  let at = 0;
  const parse = (): Item[] => {
    const items: Item[] = [];
    let lead: string[] = [];
    while (at < lines.length) {
      const line = lines[at]!;
      if (line.startsWith("}")) return items;
      at += 1;
      if (line.startsWith("//!")) { items.push({ lead: [], line, children: null }); continue; }
      if (line.endsWith("{")) { const children = parse(); at += 1; items.push({ lead, line, children }); lead = []; }
      else if (line.endsWith(";")) { items.push({ lead, line, children: null }); lead = []; }
      else lead.push(line);
    }
    return items;
  };
  const render = (items: readonly Item[]): string[] => items.map((item) => [...item.lead, item.line, ...(item.children ? ["{", ...render(item.children), "}"] : [])].join("\n")).sort();
  return render(parse()).join("\n");
}

const wiringFiles = (plan: ArtifactCratePlan): Set<string> => new Set([plan.rootFile, ...plan.crates.flatMap((crate) => [join(crate.packageDir, "🦀️.rs"), join(crate.packageDir, "🪞️mirror/🦀️.rs")])]);

function sorted(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sorted);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value as Record<string, unknown>).sort(([a], [b]) => (a < b ? -1 : 1)).map(([key, entry]) => [key, sorted(entry)]));
  return value;
}

/** ⚖️ Whether a planned file text says what the file on disk already says: manifests compare as parsed tables, generated wiring as canonical trees, every other source modulo whitespace. */
export function equivalentText(path: string, planned: string, disk: string, wiring: boolean): boolean {
  if (planned === disk) return true;
  if (path.endsWith(".toml")) {
    try { return JSON.stringify(sorted(runtimeParseTomlV1(planned))) === JSON.stringify(sorted(runtimeParseTomlV1(disk))); } catch { return false; }
  }
  if (path.endsWith(".rs")) return wiring ? canonicalWiring(planned) === canonicalWiring(disk) : planned.replace(/\s+/gu, "") === disk.replace(/\s+/gu, "");
  return false;
}

const sameSet = (left: readonly string[], right: readonly string[]): boolean => left.length === right.length && left.every((entry) => right.includes(entry));

/** 🔒️ Whether a planned crate is a carved crate that keeps what it owns and needs no sibling its manifest lacks: its files are left as they are. */
export function crateUnchanged(plan: ArtifactCratePlan, crate: PlannedCrate): boolean {
  const known = plan.carved?.crates.find((entry) => entry.signed && entry.name === crate.name);
  if (!known) return false;
  const names = crate.dependsOn.map((id) => plan.crates.find((entry) => entry.id === id)!.name);
  const planned = known.recorded ? [...new Set(crate.slices.flatMap((slice) => slice.decls.map((decl) => decl.eff.join("::"))))] : crate.attach.map((decl) => decl.eff.join("::"));
  return sameSet(known.recorded ?? known.owned, planned) && names.every((name) => known.dependsOn.includes(name));
}

/** 🪟️ The signed carved crates that no longer own a declaration but were not merged into another crate (pure re-export layers): they stay in the layout untouched and keep their facade dependency row. */
export function retainedCrates(plan: ArtifactCratePlan): readonly { name: string; packageDir: string }[] {
  const absorbed = new Set(plan.crates.flatMap((crate) => crate.supersedes ?? []));
  return (plan.carved?.crates ?? []).filter((carved) => carved.signed && !absorbed.has(carved.name) && !plan.crates.some((crate) => crate.name === carved.name));
}

/** 🗑️ The carved crates a re-carve no longer produces: their identity was merged into another crate to break a cycle, or they were hand-written and the planner now owns their declarations. */
export function supersededCrates(plan: ArtifactCratePlan): readonly { name: string; packageDir: string }[] {
  const kept = new Set(retainedCrates(plan).map((crate) => crate.name));
  return (plan.carved?.crates ?? []).filter((carved) => !kept.has(carved.name) && !plan.crates.some((crate) => crate.name === carved.name));
}

/** ✂️ Drops every write that leaves its file as it is: crates that keep their declarations are not rewritten, and a planned text equivalent to the file on disk is not written again. */
export function deltaWrites(plan: ArtifactCratePlan, writes: readonly CarveFileWrite[]): CarveFileWrite[] {
  if (!plan.carved) return [...writes];
  const files = ["Cargo.toml", "🦀️.rs", "🪞️mirror/🦀️.rs"];
  const keep = new Set(plan.crates.filter((crate) => crateUnchanged(plan, crate)).flatMap((crate) => files.map((file) => join(crate.packageDir, file))));
  const wiring = wiringFiles(plan);
  const gone = supersededCrates(plan).flatMap((crate) => files.map((file) => join(crate.packageDir, file)));
  const out = writes.filter((write) => {
    if (write.action === "delete") return true;
    if (keep.has(write.path)) return false;
    const absolute = resolve(plan.repoRoot, write.path);
    return !existsSync(absolute) || !equivalentText(write.path, write.content, readFileSync(absolute, "utf8"), wiring.has(write.path));
  });
  const deletes = gone.filter((path) => existsSync(resolve(plan.repoRoot, path)) && !out.some((write) => write.path === path)).map((path): CarveFileWrite => ({ path, action: "delete", content: "" }));
  return [...out, ...deletes];
}
