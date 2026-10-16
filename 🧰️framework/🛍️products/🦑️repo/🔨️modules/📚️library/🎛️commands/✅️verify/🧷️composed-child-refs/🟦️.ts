/** 🧭️ Composed-child reference asset gate behind `verify composed-child-refs`.
 * @see 📜️script.ts
 */
import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

/** 🧪️Aggregates lint + generated-catalog freshness + region/host-contract script lints (`gate`, the cheap pre-`ticket_close` step every refactor session runs), plus the full test suite for the top-level `verify` verb. */
/** 🧸️One composed-child row of a committed `*.dsl.semio` document whose target names a different id than the row's own `child_id`. */
export type ComposedChildRefRow = { path: string; slot: string; childId: string; targetArtifactId: string; targetUri: string; waiver?: string };

/**
 * 🧾️The only rows `verify composed-child-refs` tolerates. `🌍️gis`'s durable three-store assembly
 * deliberately pins its member documents to fixed ids (`gismap-drawing`, `gismap-value`) that are
 * declared as constants in the framework's own durable-group composition
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`), in
 * `✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json` as `const`, and in `🌎️hub`'s committed fixtures — so the
 * parent's per-handle `child_id` and the member's stable document id are two different things on
 * purpose. The consequence is stated, not hidden: a durable-group child in that shape can never pass
 * `ChildRestoreProjection`, so gis cannot take a whole-document `Effect::LoadDocument` until that
 * fork is resolved by the durable-group owner. Each waiver names the id it protects.
 */
const COMPOSED_CHILD_REF_WAIVERS: { targetArtifactId: string; reason: string }[] = [
  { targetArtifactId: "gismap-drawing", reason: "gis durable-group member id, declared const in store/🗄️durable-group, gis 🧬️schema/🔣️.json and 🌎️hub fixtures" },
  { targetArtifactId: "gismap-value", reason: "gis durable-group member id, declared const in store/🗄️durable-group, gis 🧬️schema/🔣️.json and 🌎️hub fixtures" },
];

/** 🚫️Directory names the composed-child-ref walk never descends into — caches and build output carry copies of the same assets. */
const COMPOSED_CHILD_REF_SKIPPED_DIRECTORIES = new Set([".git", "node_modules", "dist", "target", "⚡️cache", "🗑️generated"]);

/** 📚️Every committed `*.dsl.semio` document in the workspace, in walk order. */
function verifyComposedChildRefAssets(root: string): string[] {
  const found: string[] = [];
  const walk = (directory: string): void => {
    let entries;
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.isDirectory()) {
        if (!COMPOSED_CHILD_REF_SKIPPED_DIRECTORIES.has(entry.name)) walk(join(directory, entry.name));
      } else if (entry.name.endsWith(".dsl.semio")) found.push(join(directory, entry.name));
    }
  };
  walk(root);
  return found;
}

/** 🔢️Count of the documents the law scanned, so a zero-breach run still proves it looked at something. */
export function verifyComposedChildRefAssetCount(root: string): number {
  return verifyComposedChildRefAssets(root).length;
}

/**
 * 🔎️Scans every committed `*.dsl.semio` for child rows of the DSL shape
 * `<slot>=[<hex child_id>,<hex target uri>]` that `store::os_store::artifact_child::artifact_child_to_record` writes, and
 * returns the rows whose target uri names an artifact id other than the row's own `child_id`.
 * Rows matching `COMPOSED_CHILD_REF_WAIVERS` come back carrying their waiver reason rather than
 * being dropped, so the census always shows the whole truth.
 */
export function verifyComposedChildRefRows(root: string): ComposedChildRefRow[] {
  const rows: ComposedChildRefRow[] = [];
  const decode = (hex: string): string | undefined => {
    if (hex.length % 2 !== 0) return undefined;
    try {
      return Buffer.from(hex, "hex").toString("utf8");
    } catch {
      return undefined;
    }
  };
  for (const path of verifyComposedChildRefAssets(root)) {
    let text;
    try {
      text = readFileSync(path, "utf8");
    } catch {
      continue;
    }
    for (const line of text.split("\n")) {
      const match = /^([A-Za-z0-9_]+)=\[([0-9a-f]+),([0-9a-f]+)\]$/.exec(line.trim());
      if (match === null) continue;
      const [, slot, childHex, targetHex] = match;
      if (slot === undefined || childHex === undefined || targetHex === undefined) continue;
      const childId = decode(childHex);
      const targetUri = decode(targetHex);
      if (childId === undefined || targetUri === undefined || !targetUri.includes("!")) continue;
      const targetArtifactId = targetUri.slice(0, targetUri.indexOf("!"));
      if (targetArtifactId === childId) continue;
      const waiver = COMPOSED_CHILD_REF_WAIVERS.find((row) => row.targetArtifactId === targetArtifactId)?.reason;
      rows.push({ path: relative(root, path), slot, childId, targetArtifactId, targetUri, waiver });
    }
  }
  return rows;
}
