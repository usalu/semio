/** 🧰️ The shared owned resident test kit: the destructured vitest, the document limits and the typed wire fixtures. */
import { createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../../🪛️typedwire-helpers/🟦️.tsx";

/** 🏭 Creates the shared resident test kit of one registration from the injected dependencies. */
export async function createResidentTestKit(vitest: TypedWireVitest, dependencies: TypedWireDependencies) {
  const { describe, expect, it, vi } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS } = dependencies;
  const { Ajv, close, nativePagedFieldFixture, produce, residentAuthoredClosePayload, residentBuilderFixture, residentCopiedRangeFixture, residentEvidenceFixture, residentPayloadFixture, residentReaderFixture } = await createTypedWireSupport(vitest, dependencies);
  return { Ajv, DEFAULT_UI_DOCUMENT_LIMITS, close, describe, expect, it, nativePagedFieldFixture, produce, residentAuthoredClosePayload, residentBuilderFixture, residentCopiedRangeFixture, residentEvidenceFixture, residentPayloadFixture, residentReaderFixture, vi };
}
export type ResidentTestKit = Awaited<ReturnType<typeof createResidentTestKit>>;
