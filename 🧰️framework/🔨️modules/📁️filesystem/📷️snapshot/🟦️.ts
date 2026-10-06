/** 📄️ An exact source transition; null represents absence. */
export interface SourceChange { readonly path: string; readonly before: string | null; readonly after: string | null; }
/** 🧯️ An owned refusal category for source projection. */
export type SourceProjectionCode = "invalid-path" | "duplicate-change" | "empty-change" | "predecessor-mismatch" | "deleted-reference" | "missing-reference";
/** ⚠️ An exact source projection refusal without backend types. */
export class SourceProjectionError extends Error {
  constructor(readonly code: SourceProjectionCode, readonly path: string) { super(`${code}: ${path}`); this.name = "SourceProjectionError"; }
}
function validatePath(path: string): void {
  if (!path || /[\\:\u0000]/u.test(path) || path.split("/").some(part => !part || part === "." || part === "..")) throw new SourceProjectionError("invalid-path", path);
}
/** 📷️ Projects exact owned source changes without resurrecting explicit deletions. */
export class SourceProjection {
  readonly #changes = new Map<string, SourceChange>();
  readonly #read: (path: string) => string | null;
  constructor(changes: readonly SourceChange[], read: (path: string) => string | null) {
    this.#read = read;
    for (const change of changes) {
      validatePath(change.path);
      if (this.#changes.has(change.path)) throw new SourceProjectionError("duplicate-change", change.path);
      if (change.before === null && change.after === null) throw new SourceProjectionError("empty-change", change.path);
      this.#changes.set(change.path, Object.freeze({ ...change }));
    }
    for (const change of this.#changes.values()) if (read(change.path) !== change.before) throw new SourceProjectionError("predecessor-mismatch", change.path);
  }
  /** 🪦️ Identifies an explicit retirement without consulting physical sources. */
  isDeleted(path: string): boolean { validatePath(path); return this.#changes.get(path)?.after === null; }
  /** 🔎️ Resolves a source; only an unowned path can read the physical fallback. */
  resolve(path: string): string | null {
    validatePath(path);
    const change = this.#changes.get(path);
    return change === undefined ? this.#read(path) : change.after;
  }
  /** 🔗️ Resolves a required source or reports its exact absent identity. */
  require(path: string): string {
    const source = this.resolve(path);
    if (source === null) throw new SourceProjectionError(this.isDeleted(path) ? "deleted-reference" : "missing-reference", path);
    return source;
  }
  /** ↩️ Returns inverse transitions in their original authority order. */
  inverse(): SourceChange[] { return [...this.#changes.values()].map(change => ({ path: change.path, before: change.after, after: change.before })); }
}
