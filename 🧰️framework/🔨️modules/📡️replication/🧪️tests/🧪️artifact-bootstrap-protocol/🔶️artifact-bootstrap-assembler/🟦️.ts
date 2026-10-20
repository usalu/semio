/** 🔶️ The artifact bootstrap assembler phase of registerTests1. */
/** 🔷 Builds ArtifactBootstrapAssembler, artifactBootstrapAggregateHash, artifactBootstrapSha256, decodeClientFrame of registerTests1. */
export function buildArtifactBootstrapAssemblerState({ dependencies, vitest }: { readonly dependencies: Pick<typeof import("../../../🟦️.ts"), "ArtifactBootstrapAssembler" | "artifactBootstrapAggregateHash" | "artifactBootstrapSha256" | "decodeClientFrame" | "decodePresencePeer" | "decodeServerFrame" | "encodeClientFrame" | "encodePresencePeer" | "encodeServerFrame">; readonly vitest: NonNullable<ImportMeta["vitest"]> }) {
  const { ArtifactBootstrapAssembler, artifactBootstrapAggregateHash, artifactBootstrapSha256, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame } = dependencies;

  const { describe, expect, it } = vitest;
  return { describe, it, expect, decodeServerFrame, encodeServerFrame, artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler, decodeClientFrame, encodeClientFrame, decodePresencePeer, encodePresencePeer };
}
