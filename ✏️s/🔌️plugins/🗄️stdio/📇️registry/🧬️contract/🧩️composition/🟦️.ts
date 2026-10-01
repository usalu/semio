/** 🧩️ Admits owner-authored Stdio composition contributions without a concrete roster. */
export type CompositionAppV1 = { owner: string; order: number; variant: string; role: "editor" | "viewer"; type: string; factory: string; mutationRoster: boolean };
export type CompositionPlaygroundV1 = { owner: string; variant: string; app: string; aliases: string[]; ports: { react: number; wgpu: number } };
export type CompositionContributionV1 = { schema: "semio.stdio.composition-contribution/v1"; artifact: string; package: string; order: number; selections: string[]; apps: CompositionAppV1[]; nativeReceipts: Record<string, string>[]; openTargets: Record<string, string>[]; catalogFeatures: string[]; playgrounds: CompositionPlaygroundV1[] };

const atom = /^[a-z][a-z0-9-]*$/;
const rust = /^[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z_][A-Za-z0-9_]*)*$/;
const variant = /^[A-Za-z_][A-Za-z0-9_]*$/;
const receiptFields = ["artifact", "factory_id", "descriptor_codec_id", "runtime_capability_id", "artifact_kind", "artifact_schema", "extension", "pack_schema_hash", "protocol_source_sha256", "protocol_path", "definition_path"];

function object(value: unknown, fields: readonly string[]): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== fields.length || fields.some((field) => !Object.hasOwn(value, field))) throw new Error("composition contribution has an invalid closed object");
  return value as Record<string, unknown>;
}

function text(value: unknown, pattern: RegExp): string {
  if (typeof value !== "string" || !pattern.test(value)) throw new Error("composition contribution has an invalid identifier");
  return value;
}

function order(value: unknown): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("composition contribution has an invalid order");
  return value;
}

function array(value: unknown): unknown[] {
  if (!Array.isArray(value)) throw new Error("composition contribution has an invalid array");
  return value;
}

/** 🧬️ Validates the closed contribution schema before any source or manifest publication. */
export function admitCompositionContributionV1(value: unknown): CompositionContributionV1 {
  const row = object(value, ["schema", "artifact", "package", "order", "selections", "apps", "nativeReceipts", "openTargets", "catalogFeatures", "playgrounds"]);
  if (row.schema !== "semio.stdio.composition-contribution/v1") throw new Error("composition contribution schema is unsupported");
  const selections = array(row.selections).map((value) => text(value, atom));
  if (new Set(selections).size !== selections.length) throw new Error("composition contribution repeats a selection");
  const apps = array(row.apps).map((value): CompositionAppV1 => {
    const app = object(value, ["owner", "order", "variant", "role", "type", "factory", "mutationRoster"]);
    if (app.role !== "editor" && app.role !== "viewer") throw new Error("composition app role is unsupported");
    if (typeof app.mutationRoster !== "boolean") throw new Error("composition app mutation roster is invalid");
    return { owner: text(app.owner, atom), order: order(app.order), variant: text(app.variant, variant), role: app.role, type: text(app.type, rust), factory: text(app.factory, rust), mutationRoster: app.mutationRoster };
  });
  const nativeReceipts = array(row.nativeReceipts).map((value) => {
    const receipt = object(value, receiptFields);
    for (const field of receiptFields) if (typeof receipt[field] !== "string" || !(receipt[field] as string).length) throw new Error("composition native receipt has an invalid field");
    if (!/^[0-9a-f]{64}$/.test(receipt.protocol_source_sha256 as string)) throw new Error("composition native receipt has an invalid source proof");
    return receipt as Record<string, string>;
  });
  const openTargets = array(row.openTargets).map((value) => {
    const target = object(value, ["artifactKind", "artifactSchema", "packSchemaHash", "factoryId", "extension", "role", "surfaceId", "protocolSourceSha256"]);
    if (Object.values(target).some((value) => typeof value !== "string" || !value.length)) throw new Error("composition open target has an invalid field");
    return target as Record<string, string>;
  });
  const catalogFeatures = array(row.catalogFeatures).map((value) => text(value, atom));
  if (new Set(catalogFeatures).size !== catalogFeatures.length) throw new Error("Composition contribution repeats a catalog feature");
  const playgrounds = array(row.playgrounds).map((value): CompositionPlaygroundV1 => {
    const playground = object(value, ["owner", "variant", "app", "aliases", "ports"]);
    const ports = object(playground.ports, ["react", "wgpu"]);
    for (const value of Object.values(ports)) if (typeof value !== "number" || !Number.isInteger(value) || value < 1 || value > 65535) throw new Error("Invalid contribution playground port");
    const aliases = array(playground.aliases).map((value) => text(value, /^.+$/));
    if (new Set(aliases).size !== aliases.length) throw new Error("Repeated contribution playground alias");
    return { owner: text(playground.owner, atom), variant: text(playground.variant, atom), app: text(playground.app, /^.+$/), aliases, ports: ports as { react: number; wgpu: number } };
  });
  return { schema: row.schema, artifact: text(row.artifact, atom), package: text(row.package, atom), order: order(row.order), selections, apps, nativeReceipts, openTargets, catalogFeatures, playgrounds };
}

/** 📇️ Selects present contributions in authored order and refuses conflicting authority. */
export function selectCompositionContributionsV1(values: readonly unknown[], selection: string): CompositionContributionV1[] {
  const rows = values.map(admitCompositionContributionV1);
  for (const key of ["artifact", "package", "order"] as const) if (new Set(rows.map((row) => row[key])).size !== rows.length) throw new Error(`composition contributions repeat ${key}`);
  const apps = rows.flatMap((row) => row.apps);
  for (const key of ["variant", "order"] as const) if (new Set(apps.map((app) => `${app.owner}:${app[key]}`)).size !== apps.length) throw new Error(`composition apps repeat ${key}`);
  for (const key of ["factory_id", "descriptor_codec_id"] as const) {
    const receipts = rows.flatMap((row) => row.nativeReceipts);
    if (new Set(receipts.map((row) => row[key])).size !== receipts.length) throw new Error(`composition native receipts repeat ${key}`);
  }
  const playgrounds = rows.flatMap((row) => row.playgrounds);
  for (const key of ["variant", "app"] as const) if (new Set(playgrounds.map((entry) => `${entry.owner}:${entry[key]}`)).size !== playgrounds.length) throw new Error(`Composition playgrounds repeat ${key}`);
  return rows.filter((row) => row.selections.includes(selection)).sort((a, b) => a.order - b.order);
}
