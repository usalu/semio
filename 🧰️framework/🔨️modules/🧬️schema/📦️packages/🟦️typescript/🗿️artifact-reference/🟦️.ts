/** 🗿️ Public entry `@semio-tech/schema-ts/artifact-reference`: the explicit surface that importers outside the module use. */
export { default as artifactReferenceSchema } from "../../../🗿️artifact-reference/🔣️.json" with { type: "json" };
export { isCanonicalArtifactKind, parseArtifactDialect, parseArtifactRef, requireArtifactDialect } from "../../../🗿️artifact-reference/🟦️.ts";
export type { ArtifactDialect, ArtifactRef } from "../../../🗿️artifact-reference/🟦️.ts";
export { default as artifactChildSchema } from "../../../🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
export { contentId, parseArtifactChild } from "../../../🗿️artifact-reference/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../🗿️artifact-reference/🪆️child/🧬️schema/🟦️.ts";
export { default as artifactLinkSchema } from "../../../🗿️artifact-reference/🔗️link/🧬️schema/🔣️.json" with { type: "json" };
export { parseArtifactLink, parseLinkPin } from "../../../🗿️artifact-reference/🔗️link/🧬️schema/🟦️.ts";
export type { ArtifactLink, LinkPin } from "../../../🗿️artifact-reference/🔗️link/🧬️schema/🟦️.ts";
export { default as blobRefSchema } from "../../../🗿️artifact-reference/📦️blob/🧬️schema/🔣️.json" with { type: "json" };
export { parseBlobRef } from "../../../🗿️artifact-reference/📦️blob/🧬️schema/🟦️.ts";
export type { BlobRef } from "../../../🗿️artifact-reference/📦️blob/🧬️schema/🟦️.ts";
