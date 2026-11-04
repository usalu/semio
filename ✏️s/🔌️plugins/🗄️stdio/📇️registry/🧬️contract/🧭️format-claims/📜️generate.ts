/** 🧭️ Derives `stdio-format-claims.json`: every published stdio format (the same row `source_format_descriptors` publishes, owner revision first) with the dialect coordinate of its owner editor. Driven by the `s` project targets `generate-stdio-format-claims` and `check-stdio-format-claims`. */
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";

type Claim = { namespace: string; value: string };
type Representation = { id: string; standard: string; mimes: string[]; extensions: string[]; aliases: string[]; owner?: boolean; sniff?: { window: number; contains: string[] } };
type Definition = { artifact: string; id: string; standards: { id: string; revision: string }[]; representations: Representation[]; runtime_capabilities: { category: string; claims: Claim[] }[] };

/** 📍 Repository-relative path of the generated claims. */
export const STDIO_FORMAT_CLAIMS_PATH = "✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json";

/** 🧮️ The exact generated text for one repository root. */
export function stdioFormatClaimsTextV1(root: string): string {
  const artifacts = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
  const editors = new Set((JSON.parse(readFileSync(join(root, "🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🧫️fixtures/✏️editor-catalog/🔣️.json"), "utf8")) as { editorApps: string[] }).editorApps);
  const shortIdOf = (item: Representation): string => {
    const first = item.extensions[0];
    if (first === undefined || !first.startsWith(".") || first.length < 2) throw new Error(`${item.id}: the first extension must be a dotted name to serve as the short format id`);
    return first.slice(1);
  };
  const claimKey = (claims: Claim[]): string => claims.map((claim) => `${claim.namespace}:${claim.value}`).sort().join("|");
  const rows: Record<string, unknown>[] = [];
  const excluded: { artifact: string; reason: string }[] = [];
  for (const directory of readdirSync(artifacts).sort()) {
    const path = join(artifacts, directory, "📜️artifact-definition.json");
    if (!existsSync(path)) continue;
    const definition = JSON.parse(readFileSync(path, "utf8")) as Definition;
    for (const capability of definition.runtime_capabilities.filter((item) => item.category === "representation")) {
      const wanted = claimKey(capability.claims);
      const members = definition.representations.filter((item) => claimKey([...item.mimes.map((value) => ({ namespace: "mime", value })), ...item.extensions.map((value) => ({ namespace: "extension", value }))]) === wanted).sort((left, right) => Number(right.owner === true) - Number(left.owner === true) || (left.id < right.id ? -1 : 1));
      const owner = members[0];
      if (owner === undefined) throw new Error(`${definition.id}: representation capability without a representation`);
      const dashed = (text: string): string => text.replaceAll(".", "-");
      const coordinateOf = (item: Representation): string | undefined => {
        const revision = definition.standards.find((standard) => standard.id === item.standard)?.revision;
        return [...editors].map((app) => app.slice(0, app.indexOf("#"))).find((coordinate) => coordinate.startsWith(`${definition.id}@`) && coordinate.endsWith("/*") && dashed(coordinate.slice(definition.id.length + 1, -2)) === dashed(revision ?? ""));
      };
      const artifactRef = coordinateOf(owner);
      if (artifactRef === undefined) {
        excluded.push({ artifact: definition.artifact, reason: "no owner editor in the editor-catalog fixture" });
        continue;
      }
      const revisions = members.flatMap((item) => {
        const coordinate = coordinateOf(item);
        return coordinate === undefined ? [] : [{ kindId: item.id, artifactRef: coordinate, sniff: item.sniff ?? null }];
      });
      rows.push({ kindId: owner.id, shortId: shortIdOf(owner), aliases: [...owner.aliases].sort(), mimes: [...owner.mimes].sort(), extensions: owner.extensions, artifactRef, ...(members.length > 1 ? { revisions } : {}) });
    }
  }
  const roster = (JSON.parse(readFileSync(join(root, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json"), "utf8")) as { identities: string[] }).identities;
  const covered = new Set(rows.map((row) => String(row.kindId).split(".")[2]));
  const missing = roster.filter((identity) => !covered.has(identity));
  if (missing.length > 0) throw new Error(`roster artifacts without a format claim row (add an owner editor or exclude them explicitly): ${missing.join(", ")}`);
  const unique = [...new Map(excluded.map((entry) => [entry.artifact, entry])).values()].sort((left, right) => (left.artifact < right.artifact ? -1 : 1));
  return `${JSON.stringify({ schema: "semio.stdio.format-claims/v1", formats: rows, excluded: unique }, null, 2)}\n`;
}
