/** 📞️ App channel client: local interaction queries, turn outcomes, conflicts, merge and dispatch reports, handshake and request sequences.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe } from "vitest";
import { registerLocalInteractionWireTests } from "./🔌️local-interaction-wire-and-lifecycle/🟦️.ts";
import { registerTypedOperationCompletionTests } from "./🏁️typed-operation-completion/🟦️.ts";
import { registerCommandAndDocumentArchiveFrameTests } from "./📨️command-and-document-archive-frames/🟦️.ts";
import { registerMergePolicyAndFaultDecodingTests } from "./🔀️merge-policy-and-fault-decoding/🟦️.ts";
import { registerComposerSurfaceAndOutcomeStreamTests } from "./🎛️composer-surface-and-outcome-streams/🟦️.ts";

describe("@semio-tech/framework-os AppChannelClient", () => {
  registerLocalInteractionWireTests();
  registerTypedOperationCompletionTests();
  registerCommandAndDocumentArchiveFrameTests();
  registerMergePolicyAndFaultDecodingTests();
  registerComposerSurfaceAndOutcomeStreamTests();
});
