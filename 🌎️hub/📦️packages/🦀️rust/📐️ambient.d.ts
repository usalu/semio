/** 🌎️ Ambient declarations for the two module families this router imports that no installed package types.
 *
 * `lodash-es` publishes no types and has no `@types/*` companion reachable under `moduleResolution: "bundler"`;
 * it is imported per function so the oracles stay independent of the implementation they verify. The
 * dev-server surface below is served by Vite at absolute browser URLs inside `page.evaluate`, so it is a runtime
 * contract rather than a bundler specifier. Declared here only: the exact members those call sites use. Delete an
 * entry the moment its real types become reachable.
 */

/** 🔎️ Third-party predicate-index oracle for the directory authority trace. */
declare module "lodash-es/findIndex.js" {
  /** 🔎️ Index of the first element satisfying `predicate`, or `-1`. */
  const findIndex: <T>(collection: readonly T[], predicate: (value: T) => boolean) => number;
  export default findIndex;
}

/** ✅️ Third-party universal-quantifier oracle for the directory binding trace. */
declare module "lodash-es/every.js" {
  /** ✅️ Whether every element satisfies `predicate`. */
  const every: <T>(collection: readonly T[], predicate: (value: T) => unknown) => boolean;
  export default every;
}

/** ➕️ Third-party summation oracle for the shared-space capacity ledger. */
declare module "lodash-es/sum.js" {
  /** ➕️ Sum of a numeric collection. */
  const sum: (collection: readonly number[]) => number;
  export default sum;
}

/** 🎛️ The directory-Home owner controller as the dev server serves it at `/controller.js`.
 *
 * A rooted browser URL is a relative specifier to TypeScript, so it can carry no ambient module
 * declaration; the shape is declared globally and applied where the served module is imported. */
interface DirectoryHomeControllerSurfaceV1 {
  /** 🎛️ Opens a directory-Home owner over a loaded plugin module. */
  openDirectoryHomeOwnerV1(options: Readonly<Record<string, unknown>>): Promise<unknown>;
  /** 📄️ Applies one canonical directory event page to an open owner. */
  applyDirectoryEventPageBootstrapV1(owner: unknown, page: Readonly<Record<string, unknown>>, post: (message: unknown) => void): Promise<{ readonly state: { readonly kind: string } }>;
  /** 🚪️ Closes an open directory-Home owner. */
  closeDirectoryHomeOwnerV1(owner: unknown, post: (message: unknown) => void): Promise<void>;
}

/** 🔌️ The browser plugin runtime as the dev server serves it at `/plugin-runtime.js`. */
interface BrowserPluginRuntimeSurfaceV1 {
  /** 🙋️ Binds the actor every subsequent plugin call is attributed to. */
  setPluginRuntimeActor(actor: string): void;
  /** 📦️ Loads one plugin bridge module by served URL. */
  loadPluginModule(pluginId: string, bridgeUrl: string, signal: AbortSignal): Promise<{ readonly manifest: Record<string, any>; dispose(): void }>;
}
