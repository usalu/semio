/** 🗿️ SDK entry `@semio-tech/framework/schema/artifact-reference`: explicit public surface of the framework module that plugins build on. */
export { parseArtifactRef, requireArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
export type { ArtifactDialect, ArtifactRef } from "@semio-tech/schema-ts/artifact-reference";
export { artifactReferenceSchema } from "@semio-tech/schema-ts/artifact-reference";
export { artifactChildSchema, contentId, parseArtifactChild } from "@semio-tech/schema-ts/artifact-reference";
export type { ArtifactChild } from "@semio-tech/schema-ts/artifact-reference";
export { artifactLinkSchema, blobRefSchema, parseArtifactLink, parseBlobRef, parseLinkPin } from "@semio-tech/schema-ts/artifact-reference";
export type { ArtifactLink, BlobRef, LinkPin } from "@semio-tech/schema-ts/artifact-reference";
