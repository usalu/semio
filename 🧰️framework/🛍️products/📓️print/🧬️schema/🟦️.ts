/** 📊️ Typed twin of `🧬️schema/🔣️.json`: the print visualization catalogue contract.
 *
 * Schema-first — `🔣️.json` is the single source of truth and the only thing `ajv` validates against;
 * this module restates it for TypeScript consumers and carries the pure readers the generator and the
 * coverage checks share. No runtime dependency: `ajv` is a test-only oracle.
 *
 * @see 🖼️assets/🔣️viz-catalog.json — the catalogue this contract describes
 * @see 🖼️assets/📊️viz-taxonomy.md — the handcrafted taxonomy the catalogue covers
 */

//#region 🔖️Contract
/** 🌍 A user-visible string in both supported document languages; there is no default language. */
export type LocalizedText = { readonly en: string; readonly de: string };

/** 🗣️ The document languages the print product renders. */
export const VIZ_LANGUAGES = ["en", "de"] as const;
export type VizLanguage = (typeof VIZ_LANGUAGES)[number];

/** 🗂️ What a catalogue entry renders. */
export const VIZ_ENTRY_KINDS = ["mark", "chart", "layout", "axis", "scale"] as const;
export type VizEntryKind = (typeof VIZ_ENTRY_KINDS)[number];

/** 🎛️ One option value of a family; LaTeX receives it as a plain `key=value` token. */
export type VizOptionValue = string | number | boolean;

/** 📇 One chart kind: a family plus its default options, covering one or more taxonomy leaves. */
export type VizCatalogEntry = {
  readonly id: string;
  readonly slug: string;
  readonly title: LocalizedText;
  readonly kind: VizEntryKind;
  readonly namespace: string;
  readonly family: string;
  readonly options: Readonly<Record<string, VizOptionValue>> & { readonly variant: string };
  readonly data: string;
  readonly covers: readonly string[];
};

/** 📚️ The catalogue document as stored in `🖼️assets/🔣️viz-catalog.json`. */
export type VizCatalog = { readonly schemaVersion: 1; readonly kinds: readonly VizCatalogEntry[] };

/** 🎚️ One declared option of a family. */
export type VizFamilyOption = {
  readonly type?: "string" | "number" | "integer" | "boolean";
  readonly default?: VizOptionValue;
  readonly enum?: readonly string[];
  readonly description: LocalizedText;
};

/** 🧩 One family's vocabulary: its owning agent and every option key it accepts. */
export type VizFamilyOptions = { readonly owner: string; readonly options: Readonly<Record<string, VizFamilyOption>> };

/** 🧫 A named demo table every catalogue entry may point at through its `data` key. */
export type VizDemoTable = { readonly name: string; readonly columns: readonly string[]; readonly description: LocalizedText };

/** 🔬 One line of `\jobname.probe.jsonl` written by `semio-viz-probe.sty`. */
export type VizProbeRecord = { readonly case: string; readonly scenario: string; readonly key: string; readonly values: readonly (number | string)[] };

/** 🧬️ The schema document, including the `x-` annotations that carry the per-family vocabulary. */
export type VizSchemaDocument = {
  readonly $id: string;
  readonly "x-semio-family-options": Readonly<Record<string, VizFamilyOptions>>;
  readonly "x-semio-demo-tables": readonly VizDemoTable[];
};

/** 🍃 One terminal entry of the handcrafted taxonomy, mirrored into `🖼️assets/🔣️viz-taxonomy.json`. */
export type VizTaxonomyLeaf = {
  readonly id: string;
  readonly slug: string;
  readonly title: string;
  readonly kind: VizEntryKind;
  readonly family: string;
  readonly section: string;
};
//#endregion 🔖️Contract

//#region 🔖️Readers
const LEAF_PATTERN = /^- (.+) `([^`]+)` (mark|chart|layout|axis|scale)$/;

/** 🍃️ Reads the terminal visualization identifiers and titles from the taxonomy source. */
export function parseVizTaxonomy(md: string): readonly { readonly section: string; readonly group: string; readonly slug: string; readonly title: string; readonly kind: VizEntryKind }[] {
  const leaves: { section: string; group: string; slug: string; title: string; kind: VizEntryKind }[] = [];
  let section = "";
  let group = "General";
  for (const line of md.split(/\n/)) {
    const heading = line.match(/^##\s+(\d+)/);
    if (heading) { section = heading[1]!; group = "General"; continue; }
    const subheading = line.match(/^###\s+(.*?)\s*$/);
    if (subheading) { group = subheading[1]!; continue; }
    const leaf = line.match(LEAF_PATTERN);
    if (leaf && section) leaves.push({ section, group, slug: leaf[2]!, title: leaf[1]!, kind: leaf[3] as VizEntryKind });
  }
  return leaves;
}

/** 🏷️ Resolves a catalogue title in the requested document language. */
export function vizTitle(entry: VizCatalogEntry, language: VizLanguage): string {
  return entry.title[language];
}

/** 🔑 Renders one entry's options as the LaTeX key-value list the family receives. */
export function vizOptionList(options: Readonly<Record<string, VizOptionValue>>): string {
  return Object.entries(options).map(([key, value]) => `${key}=${vizOptionValue(value)}`).join(",");
}

/** 🔤️ One option value as l3keys reads it: a value that carries a comma or an equals sign is braced,
 * because `\keys_set` splits an unbraced list on commas and would turn one key into several. */
export function vizOptionValue(value: VizOptionValue): string {
  const text = String(value);
  return /[,=]/.test(text) ? `{${text}}` : text;
}
//#endregion 🔖️Readers
