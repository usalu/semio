/** 🎯️ Native selection of the catalog verification: the native catalog selection types, its plan and preview, the registry schema validator and the selection oracle. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";

/** 🪪️ A registry build-planning identity, never an executable factory or verified catalog grant. */
export type NativeCatalogSelectionIdentityV1 = Readonly<{ pluginId: string; packageId: string; version: string }>;

export type NativeCatalogSelectionPackageV1 = NativeCatalogSelectionIdentityV1 & Readonly<{ dependencies: readonly NativeCatalogSelectionIdentityV1[]; receiptCount: number }>;

export type NativeCatalogProviderPreviewV1 = Readonly<{ pluginId: string; packageId: string; receiptCount: number }>;

export type NativeCatalogSelectionInputV1 = Readonly<{
  packages: readonly NativeCatalogSelectionPackageV1[];
  profiles: readonly Readonly<{ id: string; roots: readonly NativeCatalogSelectionIdentityV1[] }>[];
  availableProviders: readonly NativeCatalogProviderPreviewV1[];
}>;

/** 🧭️ Plans only a bounded dependency-first selected provider inventory; no bytes or authority are published. */
export function planNativeCatalogSelectionV1(input: NativeCatalogSelectionInputV1, profileId: string): readonly NativeCatalogSelectionPackageV1[] {
  const deny: () => never = () => { throw new Error("native catalog selection denied"); };
  const identity = (row: NativeCatalogSelectionIdentityV1): string => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.pluginId) || row.pluginId.length > 128 || row.packageId !== `semio:${row.pluginId}` || !/^[0-9]+\.[0-9]+\.[0-9]+$/.test(row.version) || row.version.length > 64) deny();
    return `${row.pluginId}/${row.packageId}/${row.version}`;
  };
  if (!input.packages.length || input.packages.length > 256 || !input.profiles.length || input.profiles.length > 256 || input.availableProviders.length > 256) deny();
  const packages = new Map<string, NativeCatalogSelectionPackageV1>();
  const owners = new Set<string>();
  for (const row of input.packages) {
    const key = identity(row);
    if (packages.has(key) || owners.has(row.packageId) || row.dependencies.length > 256 || !Number.isSafeInteger(row.receiptCount) || row.receiptCount < 1 || row.receiptCount > 16384) deny();
    if (new Set(row.dependencies.map(identity)).size !== row.dependencies.length) deny();
    packages.set(key, row);
    owners.add(row.packageId);
  }
  const providers = new Map<string, NativeCatalogProviderPreviewV1>();
  for (const row of input.availableProviders) {
    const key = `${row.pluginId}/${row.packageId}`;
    if (row.packageId !== `semio:${row.pluginId}` || providers.has(key) || !Number.isSafeInteger(row.receiptCount) || row.receiptCount < 1 || row.receiptCount > 16384) deny();
    providers.set(key, row);
  }
  if (new Set(input.profiles.map(row => row.id)).size !== input.profiles.length) deny();
  const profile = input.profiles.find(row => row.id === profileId);
  if (!profile || !profile.roots.length || profile.roots.length > 256 || new Set(profile.roots.map(identity)).size !== profile.roots.length) deny();
  const visiting = new Set<string>();
  const completed = new Set<string>();
  const selected: NativeCatalogSelectionPackageV1[] = [];
  const compare = (a: NativeCatalogSelectionIdentityV1, b: NativeCatalogSelectionIdentityV1): number => identity(a) < identity(b) ? -1 : identity(a) > identity(b) ? 1 : 0;
  let totalReceipts = 0;
  const visit = (key: string): void => {
    if (completed.has(key)) return;
    if (visiting.has(key)) deny();
    const row = packages.get(key);
    if (!row) deny();
    visiting.add(key);
    for (const dependency of [...row.dependencies].sort(compare)) visit(identity(dependency));
    const provider = providers.get(`${row.pluginId}/${row.packageId}`);
    if (!provider || provider.receiptCount !== row.receiptCount) deny();
    totalReceipts += row.receiptCount;
    if (totalReceipts > 16384) deny();
    visiting.delete(key);
    completed.add(key);
    selected.push(Object.freeze({ ...row, dependencies: Object.freeze(row.dependencies.map(dependency => Object.freeze({ ...dependency }))) }));
  };
  for (const root of [...profile.roots].sort(compare)) visit(identity(root));
  return Object.freeze(selected);
}

/** 🧪️ Exercises private preview staging; callback inventories are not native receipts or activation inputs. */
export async function previewNativeCatalogSelectionV1(
  plan: readonly NativeCatalogSelectionPackageV1[],
  preview: (identity: NativeCatalogSelectionIdentityV1) => Promise<NativeCatalogProviderPreviewV1>,
  checkpoint: () => void,
  preflight: () => void,
): Promise<readonly NativeCatalogProviderPreviewV1[]> {
  const staged: NativeCatalogProviderPreviewV1[] = [];
  for (const row of plan) {
    checkpoint();
    const result = await preview(row);
    checkpoint();
    if (result.pluginId !== row.pluginId || result.packageId !== row.packageId || result.receiptCount !== row.receiptCount) throw new Error("native catalog preview identity denied");
    staged.push(Object.freeze({ ...result }));
  }
  checkpoint();
  preflight();
  return Object.freeze(staged);
}

export type SchemaValidator = ((value: unknown) => boolean) & { readonly errors?: unknown };

/** 🧬️ Compiles one named export of the registry schema module against the repository draft-07 dialect. */
export async function registrySchemaValidator(exportId: string): Promise<SchemaValidator> {
  const { default: Ajv } = await import("ajv");
  const document = JSON.parse(readFileSync(join(import.meta.dirname, "..", "..", "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`registry schema: unknown export ${exportId}`);
  return validate as SchemaValidator;
}

/** 🧫️ Independent AJV/Kahn closure oracle over neutral data; never reads a live factory or production descriptor. */
export async function nativeCatalogSelectionOracleV1(): Promise<void> {
  const fixtureRoot = join(import.meta.dirname, "..", "..", "🧪️tests/🧫️fixtures/📦️native-catalog-selection");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validateInput = await registrySchemaValidator("NativeCatalogSelectionInputV1");
  const validate = (value: any): boolean => validateInput({ packages: value.packages, profiles: value.profiles, availableProviders: value.availableProviders });
  if (!validate(fixture)) throw new Error("native catalog domain input was denied");
  const neutral = (candidate: any, profileId: string, mutation: string): { code: string; receiptCount: number; calls: string[] } => {
    const denied = (calls: string[] = []) => ({ code: "denied", receiptCount: 0, calls });
    if (!validate(candidate)) return denied();
    const same = (a: any, b: any) => a.pluginId === b.pluginId && a.packageId === b.packageId && a.version === b.version;
    if (new Set(candidate.packages.map((item: any) => item.packageId)).size !== candidate.packages.length || new Set(candidate.availableProviders.map((item: any) => item.packageId)).size !== candidate.availableProviders.length) return denied();
    if ([...candidate.packages, ...candidate.availableProviders].some((item: any) => item.packageId !== `semio:${item.pluginId}`)) return denied();
    const profile = candidate.profiles.find((item: any) => item.id === profileId);
    if (!profile || new Set(profile.roots.map((item: any) => item.packageId)).size !== profile.roots.length) return denied();
    const wanted: any[] = [];
    const pending = [...profile.roots];
    while (pending.length) {
      const identity = pending.shift();
      const item = candidate.packages.find((item: any) => same(item, identity));
      if (!item) return denied();
      if (wanted.some(item => same(item, identity))) continue;
      wanted.push(item);
      pending.push(...item.dependencies);
    }
    const ordered: any[] = [];
    while (ordered.length < wanted.length) {
      const next = wanted.filter(item => !ordered.includes(item) && item.dependencies.every((dependency: any) => ordered.some(item => same(item, dependency)))).sort((a, b) => a.packageId < b.packageId ? -1 : a.packageId > b.packageId ? 1 : 0)[0];
      if (!next) return denied();
      const provider = candidate.availableProviders.find((item: any) => item.pluginId === next.pluginId && item.packageId === next.packageId);
      if (!provider || provider.receiptCount !== next.receiptCount) return denied();
      ordered.push(next);
    }
    const calls: string[] = [];
    for (const item of ordered) {
      if (mutation === "cancel-before-first") return denied(calls);
      calls.push(item.packageId);
      if (mutation === "unselected-result" || mutation === "cancel-after-first" || mutation === "selected-provider-failure" && item.pluginId === "vcs") return denied(calls);
    }
    if (mutation === "registry-conflict") return denied(calls);
    return { code: "planned", receiptCount: ordered.reduce((sum, item) => sum + item.receiptCount, 0), calls };
  };
  for (const row of fixture.cases) {
    const candidate = structuredClone(fixture);
    let profileId = row.profileId;
    const vcs = candidate.packages.find((item: any) => item.pluginId === "vcs");
    const stdio = candidate.packages.find((item: any) => item.pluginId === "stdio");
    const root = candidate.profiles.find((item: any) => item.id === "native-stdio-vcs-v1").roots[0];
    const vcsProvider = candidate.availableProviders.find((item: any) => item.pluginId === "vcs");
    switch (row.mutation) {
      case "unknown-profile": profileId = "unavailable"; break;
      case "duplicate-root": candidate.profiles[1].roots.push({ ...root }); break;
      case "missing-root": candidate.profiles[1].roots = []; break;
      case "missing-dependency": candidate.packages = candidate.packages.filter((item: any) => item.pluginId !== "stdio"); break;
      case "dependency-version": vcs.dependencies[0].version = "9.9.9"; break;
      case "cycle": stdio.dependencies.push({ ...root }); break;
      case "duplicate-package": candidate.packages.push({ ...stdio }); break;
      case "foreign-package": vcs.packageId = "semio:other"; break;
      case "duplicate-provider": candidate.availableProviders.push({ ...candidate.availableProviders[0] }); break;
      case "missing-provider": candidate.availableProviders = candidate.availableProviders.filter((item: any) => item.pluginId !== "vcs"); break;
      case "provider-package": vcsProvider.packageId = "semio:other"; break;
      case "provider-count": vcsProvider.receiptCount = 2; break;
    }
    const calls: string[] = [];
    let receiptCount = 0;
    let code = "denied";
    const independent = neutral(candidate, profileId, row.mutation);
    if (!isDeepStrictEqual(independent, { code: row.expectedCode, receiptCount: row.expectedReceiptCount, calls: row.expectedCalls })) throw new Error(`independent native selection case ${row.id} differs: ${JSON.stringify(independent)}`);
    try {
      const plan = planNativeCatalogSelectionV1(candidate, profileId);
      if (!Object.isFrozen(plan) || plan.some(item => !Object.isFrozen(item) || !Object.isFrozen(item.dependencies) || item.dependencies.some(dependency => !Object.isFrozen(dependency)))) throw new Error("selection inventory is mutable");
      const result = await previewNativeCatalogSelectionV1(plan, async selected => {
        calls.push(selected.packageId);
        if (row.mutation === "selected-provider-failure" && selected.pluginId === "vcs") throw new Error("selected preview unavailable");
        if (row.mutation === "unselected-result") return candidate.availableProviders[1];
        return candidate.availableProviders.find((provider: any) => provider.packageId === selected.packageId);
      }, () => {
        if (row.mutation === "cancel-before-first" || (row.mutation === "cancel-after-first" && calls.length === 1)) throw new Error("cancelled");
      }, () => {
        if (row.mutation === "registry-conflict") throw new Error("preview conflict");
      });
      receiptCount = result.reduce((sum, item) => sum + item.receiptCount, 0);
      code = "planned";
    } catch {}
    if (code !== row.expectedCode || receiptCount !== row.expectedReceiptCount || !isDeepStrictEqual(calls, row.expectedCalls)) throw new Error(`native selection case ${row.id} differs: ${JSON.stringify({ code, receiptCount, calls })}`);
  }
  const positive = fixture.cases.filter((row: any) => row.expectedCode === "planned").length;
  console.log(`native-catalog-selection-oracle cases=${fixture.cases.length} positive=${positive} denied=${fixture.cases.length - positive} authority=planning-only published=0`);
}
