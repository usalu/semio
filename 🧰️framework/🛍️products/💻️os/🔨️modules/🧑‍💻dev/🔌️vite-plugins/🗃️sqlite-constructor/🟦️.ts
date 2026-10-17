/** 🗃️ The lazily imported `bun:sqlite` constructor of the dev server's Vite plugins: the Bun builtin assembled at runtime so no bundler can fold it into a static specifier, and the handle type. */
//#region BackboneVitePlugin
/** 🗃️ The Bun builtin, ASSEMBLED at runtime. A bundler asked to ANALYSE this module refuses the
 * specifier outright ("Cannot bundle built-in module"), which failed whole jsdom test files whose
 * graph reaches here; `@vite-ignore` beside a constant was not enough, because a `const` holding a
 * string literal is exactly what the analyser constant-folds back into a static specifier. Joining
 * the two halves leaves nothing to fold, and the module's real type is restored in a TYPE position,
 * which the bundler never sees. Resolution is left to the runtime that actually has it. */
const BUN_SQLITE_MODULE = ["bun", "sqlite"].join(":");

/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
let backboneDatabaseCtor: typeof import("bun:sqlite").Database | undefined;
export async function backboneDatabaseCtorLazy(): Promise<typeof import("bun:sqlite").Database> {
  if (backboneDatabaseCtor) return backboneDatabaseCtor;
  const { Database } = (await import(/* @vite-ignore */ BUN_SQLITE_MODULE)) as typeof import("bun:sqlite");
  backboneDatabaseCtor = Database;
  return Database;
}
export type BackboneSqliteHandle = InstanceType<typeof import("bun:sqlite").Database>;
//#endregion BackboneVitePlugin
