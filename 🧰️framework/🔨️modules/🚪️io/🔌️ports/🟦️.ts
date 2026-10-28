/** 🔌️ Ports through which a parent owner carries one child's inline content without importing the child's owner. */
import type {VectorSceneCursor,VectorSceneGrant,VectorSceneStep} from "./🧵️lower/🟦️.ts";
import type { ArtifactSqliteOptions, ArtifactSqliteProjection } from "@semio-tech/framework-sqlite-snapshot-ts/artifact";
import type { SqliteDatabase } from "@semio-tech/framework-sqlite-snapshot-ts";

/** 🧱️ The parse, native JSON and SQLite component facets of one child content type, supplied by the composition that knows the child's owner. */
export interface EmbeddedChildCodecV1<Content> {
  readonly parse: (value: unknown, at: string) => Content;
  readonly withCanvasSize:(content:Content,width:number,height:number)=>Content;
  readonly lowerStep: (content:Content,cursor:VectorSceneCursor,grant:VectorSceneGrant)=>VectorSceneStep;
  readonly nativeJson: { readonly from: (value: unknown) => Content; readonly to: (content: Content) => unknown };
  readonly sqlite: {
    readonly schema: string;
    readonly project: (content: Content, projection: ArtifactSqliteProjection, options: ArtifactSqliteOptions) => Promise<bigint>;
    readonly reconstruct: (database: SqliteDatabase, declaredSchema: string, options: ArtifactSqliteOptions) => Promise<ReadonlyMap<bigint,Content>>;
  };
}

/** 🧩️ A parent-declared position that exactly one child codec fills, bound once by the composition root. */
export interface EmbeddedChildSlotV1<Content> {
  readonly id: string;
  readonly bind: (codec: EmbeddedChildCodecV1<Content>) => void;
  readonly codec: () => EmbeddedChildCodecV1<Content>;
}

/** ♾️ Declares an unbound slot; binding the same codec again is idempotent and binding a different codec is refused. */
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

export { VectorSceneCursor } from "./🧵️lower/🟦️.ts";
export type { VectorSceneGrant,VectorSceneReceipt,VectorSceneStep,VectorSceneEvent,VectorPoint2,VectorPoint3,VectorQuaternion,VectorRgba,VectorCanvas,VectorTransform,VectorSegment } from "./🧵️lower/🟦️.ts";

export {EmbeddedChildAdmission,childAdmissionAdmits,emptyChildAdmissionProgress} from "./🧩️admission/🟦️.ts";
export type {ChildAdmissionSource,ChildAdmissionAdvance,ChildAdmissionGrant,ChildAdmissionDemand,ChildAdmissionProgress,EmbeddedChildAdmissionCursorV1,EmbeddedChildAdmitterV1} from "./🧩️admission/🟦️.ts";

export {ChildAdmissionAddress} from './🧩️admission/📖️provider/🟦️.ts';
export type {ChildAdmissionStamp,ChildAdmissionQualificationV1,ChildAdmissionRead,ChildAdmissionProviderV1} from './🧩️admission/📖️provider/🟦️.ts';

export type * from './🧩️admission/🧵️vector/🟦️.ts';

export {OriginalChildVectorPreparation} from './🧩️admission/🧵️vector/🧳️prepare/🟦️.ts';
