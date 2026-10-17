/** 🔗️ Binds the Semio drawing codecs to the Layout background-drawing slot: the one place where Layout and Stdio meet. */
import type { EmbeddedChildCodecV1 } from "@semio-tech/framework/io/ports";
import { LAYOUT_BACKGROUND_DRAWING, type LayoutDrawingContent } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema";
import { parseSemioDrawingSnapshot, type SemioDrawingSnapshot } from "@semio-tech/stdio-semio/standards/v1/subsets/drawing/schema/snapshot";
import { semioDrawingSnapshotFromNativeJson, semioDrawingSnapshotNativeJson } from "@semio-tech/stdio-semio/standards/v1/subsets/drawing/io/text/snapshot/native-json";
import { SEMIO_DRAWING_SQLITE_SCHEMA, projectSemioDrawingSqliteComponent, semioDrawingSnapshotFromSqliteComponent } from "@semio-tech/stdio-semio/standards/v1/subsets/drawing/io/sqlite/snapshot";

/** 🖊️ The Semio drawing standard's complete parse, native JSON and SQLite component facets. */
export const SEMIO_DRAWING_EMBEDDED_CODEC: EmbeddedChildCodecV1<LayoutDrawingContent> = {
  parse: (value, at) => parseSemioDrawingSnapshot(value, at),
  nativeJson: { from: semioDrawingSnapshotFromNativeJson, to: (content) => semioDrawingSnapshotNativeJson(content as SemioDrawingSnapshot) },
  sqlite: {
    schema: SEMIO_DRAWING_SQLITE_SCHEMA,
    project: (content, projection, options) => projectSemioDrawingSqliteComponent(content as SemioDrawingSnapshot, projection, options),
    reconstruct: (database, declaredSchema, options) => semioDrawingSnapshotFromSqliteComponent(database, declaredSchema, options),
  },
};

/** 🔗️ Idempotently fills the Layout background-drawing slot with the Semio drawing codecs. */
export function bindLayoutBackgroundDrawing(): void {
  LAYOUT_BACKGROUND_DRAWING.bind(SEMIO_DRAWING_EMBEDDED_CODEC);
}
