import { SqliteDatabaseV1 } from "../../../../../../🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts";
/** 🗃️ The lazily imported `bun:sqlite` constructor of the dev server's Vite plugins: the Bun builtin assembled at runtime so no bundler can fold it into a static specifier, and the handle type. */
//#region BackboneVitePlugin
/** 📥️ Gives the owned SQLite connection class of the hosting runtime; the facade resolves the runtime's own SQLite lazily, so loading this module under Vite's config bundler never touches a runtime builtin. */
export async function backboneDatabaseCtorLazy(): Promise<typeof SqliteDatabaseV1> {
  return SqliteDatabaseV1;
}
export type BackboneSqliteHandle = InstanceType<typeof SqliteDatabaseV1>;
//#endregion BackboneVitePlugin
