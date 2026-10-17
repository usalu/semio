/** 🔌️ Ports through which a parent owner carries one child's inline content without importing the child's owner. */
import type { ArtifactSqliteOptions, ArtifactSqliteProjection } from "@semio-tech/framework-sqlite-snapshot-ts/artifact";
import type { SqliteDatabase } from "@semio-tech/framework-sqlite-snapshot-ts";

/** 🧱️ The parse, native JSON and SQLite component facets of one child content type, supplied by the composition that knows the child's owner. */
export interface EmbeddedChildCodecV1<Content> {
  readonly parse: (value: unknown, at: string) => Content;
  readonly nativeJson: { readonly from: (value: unknown) => Content; readonly to: (content: Content) => unknown };
  readonly sqlite: {
    readonly schema: string;
    readonly project: (content: Content, projection: ArtifactSqliteProjection, options: ArtifactSqliteOptions) => Promise<void>;
    readonly reconstruct: (database: SqliteDatabase, declaredSchema: string, options: ArtifactSqliteOptions) => Promise<Content>;
  };
}

/** 🧩️ A parent-declared position that exactly one child codec fills, bound once by the composition root. */
export interface EmbeddedChildSlotV1<Content> {
  readonly id: string;
  readonly bind: (codec: EmbeddedChildCodecV1<Content>) => void;
  readonly codec: () => EmbeddedChildCodecV1<Content>;
}

/** 🧩️ Declares an unbound slot; binding the same codec again is idempotent and binding a different codec is refused. */
export function embeddedChildSlotV1<Content>(id: string): EmbeddedChildSlotV1<Content> {
  let bound: EmbeddedChildCodecV1<Content> | undefined;
  return Object.freeze({
    id,
    bind(codec: EmbeddedChildCodecV1<Content>): void {
      if (bound !== undefined && bound !== codec) throw new Error(`embedded child slot ${id} is already bound to a different codec`);
      bound = codec;
    },
    codec(): EmbeddedChildCodecV1<Content> {
      if (bound === undefined) throw new Error(`embedded child slot ${id} has no bound codec`);
      return bound;
    },
  });
}
