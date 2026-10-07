/** 🧩️ Admits owner-authored Stdio composition contributions without a concrete roster. */
export type CompositionAppV1 = { owner: string; order: number; variant: string; role: "editor" | "viewer"; type: string; factory: string; mutationRoster: boolean; laws: { editing: string; sqliteSnapshot: string } | null };
export type CompositionPlaygroundV1 = { owner: string; variant: string; app: string; aliases: string[]; ports: { react: number; wgpu: number } };
export type CompositionContributionV1 = { schema: "semio.stdio.composition-contribution/v1"; artifact: string; package: string; order: number; selections: string[]; apps: CompositionAppV1[]; nativeFactories: { factoryId: string; definitionPath: string; protocolPath: string }[]; openTargets: Record<string, string>[]; catalogFeatures: string[]; playgrounds: CompositionPlaygroundV1[]; mutationDescriptors: { name: string; snapshot: string; mutation: string }[]; mutationCoordinates: { artifact: string; standard: string; subset: string; surface: string; owner: string; prefix: string }[] };

const atom = /^[a-z][a-z0-9-]*$/;
const rust = /^[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z_][A-Za-z0-9_]*)*$/;
const variant = /^[A-Za-z_][A-Za-z0-9_]*$/;

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
  const row = object(value, ["schema", "artifact", "package", "order", "selections", "apps", "nativeFactories", "openTargets", "catalogFeatures", "playgrounds", "mutationDescriptors", "mutationCoordinates"]);
  if (row.schema !== "semio.stdio.composition-contribution/v1") throw new Error("composition contribution schema is unsupported");
  const selections = array(row.selections).map((value) => text(value, atom));
  if (new Set(selections).size !== selections.length) throw new Error("composition contribution repeats a selection");
  const apps = array(row.apps).map((value): CompositionAppV1 => {
    const app = object(value, ["owner", "order", "variant", "role", "type", "factory", "mutationRoster", "laws"]);
    if (app.role !== "editor" && app.role !== "viewer") throw new Error("composition app role is unsupported");
    if (typeof app.mutationRoster !== "boolean") throw new Error("composition app mutation roster is invalid");
    const law = app.role === "editor" ? object(app.laws, ["editing", "sqliteSnapshot"]) : null;
    if (app.role === "viewer" && app.laws !== null) throw new Error("Viewer contribution cannot author editor laws");
    const laws = law ? { editing: text(law.editing, variant), sqliteSnapshot: text(law.sqliteSnapshot, variant) } : null;
    return { owner: text(app.owner, atom), order: order(app.order), variant: text(app.variant, variant), role: app.role, type: text(app.type, rust), factory: text(app.factory, rust), mutationRoster: app.mutationRoster, laws };
  });
  const nativeFactories = array(row.nativeFactories).map((value) => {
    const factory = object(value, ["factoryId", "definitionPath", "protocolPath"]);
    for (const field of Object.values(factory)) if (typeof field !== "string" || !field.length) throw new Error("Invalid native factory export selection");
    return factory as CompositionContributionV1["nativeFactories"][number];
  });
  const openTargets = array(row.openTargets).map((value) => {
    const target = object(value, ["factoryId", "role", "surfaceId"]);
    if (Object.values(target).some(value => typeof value !== "string" || !value.length)) throw new Error("Invalid owner-authored open target selection");
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
  const mutationDescriptors = array(row.mutationDescriptors).map((value) => {
    const descriptor = object(value, ["name", "snapshot", "mutation"]);
    return { name: text(descriptor.name, variant), snapshot: text(descriptor.snapshot, rust), mutation: text(descriptor.mutation, rust) };
  });
  const mutationCoordinates = array(row.mutationCoordinates).map((value) => {
    const coordinate = object(value, ["artifact", "standard", "subset", "surface", "owner", "prefix"]);
    for (const field of ["artifact", "standard", "subset", "owner"]) text(coordinate[field], /^.+$/);
    for (const field of ["surface", "prefix"]) if (typeof coordinate[field] !== "string") throw new Error("Invalid contribution mutation coordinate");
    return coordinate as CompositionContributionV1["mutationCoordinates"][number];
  });
  return { schema: row.schema, artifact: text(row.artifact, atom), package: text(row.package, atom), order: order(row.order), selections, apps, nativeFactories, openTargets, catalogFeatures, playgrounds, mutationDescriptors, mutationCoordinates };
}

/** 📇️ Selects present contributions in authored order and refuses conflicting authority. */
export function selectCompositionContributionsV1(values: readonly unknown[], selection: string): CompositionContributionV1[] {
  const rows = values.map(admitCompositionContributionV1);
  for (const key of ["artifact", "package", "order"] as const) if (new Set(rows.map((row) => row[key])).size !== rows.length) throw new Error(`composition contributions repeat ${key}`);
  const apps = rows.flatMap((row) => row.apps);
  const lawNames = apps.flatMap(app => app.laws ? [app.laws.editing, app.laws.sqliteSnapshot] : []);
  if (new Set(lawNames).size !== lawNames.length) throw new Error("Composition editors repeat an authored law name");
  for (const key of ["variant", "order"] as const) if (new Set(apps.map((app) => `${app.owner}:${app[key]}`)).size !== apps.length) throw new Error(`composition apps repeat ${key}`);
  const factories = rows.flatMap(row => row.nativeFactories);
  if (new Set(factories.map(row => row.factoryId)).size !== factories.length) throw new Error("Composition native factories repeat their identity");
  const playgrounds = rows.flatMap((row) => row.playgrounds);
  for (const key of ["variant", "app"] as const) if (new Set(playgrounds.map((entry) => `${entry.owner}:${entry[key]}`)).size !== playgrounds.length) throw new Error(`Composition playgrounds repeat ${key}`);
  const descriptors = rows.flatMap(row => row.mutationDescriptors);
  if (new Set(descriptors.map(entry => `${entry.snapshot}:${entry.mutation}`)).size !== descriptors.length) throw new Error("Repeated contribution mutation descriptor");
  const coordinates = rows.flatMap(row => row.mutationCoordinates);
  if (new Set(coordinates.map(entry => `${entry.artifact}:${entry.standard}:${entry.subset}:${entry.surface}`)).size !== coordinates.length) throw new Error("Repeated contribution mutation coordinate");
  return rows.filter((row) => row.selections.includes(selection)).sort((a, b) => a.order - b.order);
}

export type CompositionNativeReceiptV1 = { artifact: string; factory_id: string; descriptor_codec_id: string; runtime_capability_id: string; artifact_kind: string; artifact_schema: string; extension: string; pack_schema_hash: string; protocol_source_sha256: string; protocol_path: string; definition_path: string };
export type CompositionNativeExportV1 = { descriptorCodecId: string; factory: { factory_id: string; runtime_capability_id: string; artifact_kind: string; artifact_schema: string; extension: string; pack_schema_hash: string } };

/** 🪪️ Resolves selected owner exports and independent source proof without copying opaque identity facts. */
export function resolveCompositionNativeFactoriesV1(contribution: CompositionContributionV1, readExport: (definition: string, factory: string) => CompositionNativeExportV1, sourceProof: (protocol: string) => string): CompositionNativeReceiptV1[] {
  return contribution.nativeFactories.map(selection => {
    const exported = object(readExport(selection.definitionPath, selection.factoryId), ["descriptorCodecId", "factory"]);
    const factory = object(exported.factory, ["factory_id", "runtime_capability_id", "artifact_kind", "artifact_schema", "extension", "pack_schema_hash"]);
    if (factory.factory_id !== selection.factoryId || Object.values(factory).some(value => typeof value !== "string" || !value.length) || typeof exported.descriptorCodecId !== "string" || !exported.descriptorCodecId.length) throw new Error("Invalid owned native factory export");
    const source = sourceProof(selection.protocolPath);
    if (!/^[0-9a-f]{64}$/.test(source)) throw new Error("Invalid independent native protocol proof");
    return { artifact: contribution.artifact, ...factory as CompositionNativeExportV1["factory"], descriptor_codec_id: exported.descriptorCodecId, protocol_source_sha256: source, protocol_path: selection.protocolPath, definition_path: selection.definitionPath };
  });
}

/** 🚪️ Resolves explicit open targets through their exact selected native factory authority. */
export function resolveCompositionOpenTargetsV1(contributions: readonly CompositionContributionV1[], receipts: readonly CompositionNativeReceiptV1[]): Record<string, string>[] {
  for (const key of ["factory_id", "descriptor_codec_id"] as const) if (new Set(receipts.map(row => row[key])).size !== receipts.length) throw new Error("Ambiguous selected native factory export");
  const targets = contributions.flatMap(row => row.openTargets).map(target => {
    const receipt = receipts.find(row => row.factory_id === target.factoryId);
    if (!receipt) throw new Error("Open target has no selected native factory authority");
    return { artifactKind: receipt.artifact_kind, artifactSchema: receipt.artifact_schema, packSchemaHash: receipt.pack_schema_hash, factoryId: receipt.factory_id, extension: receipt.extension, role: target.role!, surfaceId: target.surfaceId!, protocolSourceSha256: receipt.protocol_source_sha256 };
  });
  if (new Set(targets.map(target => `${target.factoryId}:${target.role}:${target.surfaceId}`)).size !== targets.length) throw new Error("Repeated owner-authored open target");
  return targets;
}
