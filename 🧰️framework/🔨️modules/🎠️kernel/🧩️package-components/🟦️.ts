/** 🧩️ Package components: a package whose apps close in several deployable components declares each one with the apps it serves
 * (`PackageDescriptor.components`, `PluginBuilder::component`), and a host loads only the component that owns the app an activation
 * opens, lazily, when that app is first opened. An activation of no app (an inbound-request actor) runs on the first component. A package
 * without the member is one component, and every activation loads it.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 *
 * @see ../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json
 * @see ./🧬️schema/🔣️.json
 */

/** 🏷️ One declared component as the descriptor carries it: its id and the ids of the apps it serves. */
export interface PackageComponentDeclaration {
  readonly id: string;
  readonly apps: readonly string[];
}

/** 🗺️ The admitted components of one package: the declarations in order and the component of every app. */
export interface PackageComponents {
  readonly declarations: readonly PackageComponentDeclaration[];
  readonly byApp: ReadonlyMap<string, string>;
}

/** 🚨️ A descriptor's components that do not partition its app roster, or an activation of an app no component serves. */
export class PackageComponentError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(`${code}: ${message}`);
    this.code = code;
    this.name = "PackageComponentError";
  }
}

/** 🔤️ The spelling a component id must have: it names served files and crosses into the worker as a module key. */
export const PACKAGE_COMPONENT_ID = /^[a-z0-9_-]{1,128}$/u;

/** 📏️ The most components one package declares. */
export const PACKAGE_COMPONENT_MAXIMUM = 64;

/** 🧾️ Admits the `components` member of a descriptor against its app roster: absent means one component (`null`); present, it must be
 * 1 to {@link PACKAGE_COMPONENT_MAXIMUM} declarations of exactly `{id, apps}` with unique ids, and every roster app must belong to
 * exactly one of them. */
export function readPackageComponents(descriptor: object, appIds: readonly string[]): PackageComponents | null {
  const value = (descriptor as { readonly components?: unknown }).components;
  if (value === undefined) return null;
  if (!Array.isArray(value) || value.length === 0 || value.length > PACKAGE_COMPONENT_MAXIMUM) throw new PackageComponentError("package-components.shape", `components must list 1 to ${PACKAGE_COMPONENT_MAXIMUM} declarations`);
  const roster = new Set(appIds);
  const byApp = new Map<string, string>();
  const declarations: PackageComponentDeclaration[] = [];
  for (const row of value as unknown[]) {
    if (row === null || typeof row !== "object" || Array.isArray(row) || Object.keys(row).sort().join(",") !== "apps,id") throw new PackageComponentError("package-components.shape", "a component declaration has exactly id and apps");
    const { id, apps } = row as { readonly id: unknown; readonly apps: unknown };
    if (typeof id !== "string" || !PACKAGE_COMPONENT_ID.test(id)) throw new PackageComponentError("package-components.id", `component id ${JSON.stringify(id)} must match ${PACKAGE_COMPONENT_ID.source}`);
    if (declarations.some(declaration => declaration.id === id)) throw new PackageComponentError("package-components.duplicate", `component ${id} is declared twice`);
    if (!Array.isArray(apps) || apps.length === 0 || apps.some(app => typeof app !== "string")) throw new PackageComponentError("package-components.apps", `component ${id} must serve at least one app id`);
    for (const app of apps as string[]) {
      if (!roster.has(app)) throw new PackageComponentError("package-components.unknown-app", `component ${id} serves ${app}, which the app roster does not declare`);
      const owner = byApp.get(app);
      if (owner !== undefined) throw new PackageComponentError("package-components.shared-app", `app ${app} belongs to both ${owner} and ${id}`);
      byApp.set(app, id);
    }
    declarations.push(Object.freeze({ id, apps: Object.freeze([...(apps as string[])]) }));
  }
  const unowned = appIds.find(app => !byApp.has(app));
  if (unowned !== undefined) throw new PackageComponentError("package-components.unowned-app", `app ${unowned} belongs to no component`);
  return Object.freeze({ declarations: Object.freeze(declarations), byApp });
}

/** 🎯️ The component an activation loads: `null` for a package of one component, the owner of `appId`, or the first component for an
 * activation of no app. An app no component serves is refused: it would load code that cannot open it. */
export function packageComponentForApp(components: PackageComponents | null, appId: string | null): string | null {
  if (components === null) return null;
  if (appId === null) return components.declarations[0]!.id;
  const owner = components.byApp.get(appId);
  if (owner === undefined) throw new PackageComponentError("package-components.unknown-app", `no component serves app ${appId}`);
  return owner;
}
