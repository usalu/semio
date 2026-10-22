import { SqliteDatabaseV1 as RealDatabase } from "../../../../../../⚙️runtime/🗃️sqlite/🟦️.ts";

export * from "bun:sqlite";
type Guard = { write(fn: string, args: readonly unknown[]): void };
const guard = (): Guard | undefined => (globalThis as { __semioTestGuardV1?: Guard }).__semioTestGuardV1;

/** 🗃️ Opens a database like bun:sqlite does, but reports a writable file database below the real checkout first; memory and read-only databases pass. */
export class Database extends RealDatabase {
  constructor(...args: ConstructorParameters<typeof RealDatabase>) {
    const [filename, options] = args, readonly = typeof options === "object" && options !== null && (options as { readonly?: boolean }).readonly === true;
    if (typeof filename === "string" && filename !== "" && filename !== ":memory:" && !readonly) guard()?.write("Database", [filename.replace(/^file:/u, "")]);
    super(...args);
  }
}
