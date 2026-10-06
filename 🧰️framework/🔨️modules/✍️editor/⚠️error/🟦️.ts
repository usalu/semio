/** 🧭️ The Editor boundary that produced a failure. */
export type EditorErrorKind = "json" | "pack" | "scene";
/** 🧯️ Owns Editor failure categories while preserving the standard diagnostic chain. */
export class EditorError extends Error {
  readonly #kind: EditorErrorKind;
  constructor(kind: EditorErrorKind, cause: Error) {
    if (kind !== "json" && kind !== "pack" && kind !== "scene") throw new TypeError("Invalid Editor error kind");
    super(`${kind}: ${cause.message}`, { cause });
    this.name = "EditorError";
    this.#kind = kind;
  }
  /** 🪪️ Returns the owned failure category. */
  kind(): EditorErrorKind { return this.#kind; }
}
