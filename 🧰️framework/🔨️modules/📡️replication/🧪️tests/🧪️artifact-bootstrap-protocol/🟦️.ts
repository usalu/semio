import { buildArtifactBootstrapAssemblerState } from "./🔶️artifact-bootstrap-assembler/🟦️.ts";
import { registerArtifactBootstrapProtocolTests } from "./🔸️artifact-bootstrap-protocol/🟦️.ts";
import { registerWireFixturesTests } from "./🟠️wire-fixtures/🟦️.ts";
import type { TestSource } from "./🟢️shared/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "ArtifactBootstrapAssembler" | "artifactBootstrapAggregateHash" | "artifactBootstrapSha256" | "decodeClientFrame" | "decodePresencePeer" | "decodeServerFrame" | "encodeClientFrame" | "encodePresencePeer" | "encodeServerFrame">, source: TestSource): Promise<void> {
  const { describe, it, expect, decodeServerFrame, encodeServerFrame, artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler, decodeClientFrame, encodeClientFrame, decodePresencePeer, encodePresencePeer } = buildArtifactBootstrapAssemblerState({ dependencies, vitest });
  registerArtifactBootstrapProtocolTests({ describe, source, it, expect, decodeServerFrame, encodeServerFrame, artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler });
  registerWireFixturesTests({ describe, it, source, decodeClientFrame, expect, encodeClientFrame, decodeServerFrame, encodeServerFrame, decodePresencePeer, encodePresencePeer });
}
