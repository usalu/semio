import type { ArtifactCreationReadyOpeningOutcomeV1 } from "../../../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import type { ReplayRefusalReasonV1 } from "../../../📣️replay-refusal/🟦️.ts";

/** 🚪️ Everything one replayed `os.open-artifact` gesture needs from its host, so the sequence is law-testable without React. */
export interface OpenArtifactReplayPortV1<TOpening> {
  readonly resolve: () => Promise<TOpening | null>;
  readonly open: (opening: TOpening) => Promise<ArtifactCreationReadyOpeningOutcomeV1>;
  readonly refuse: (reason: ReplayRefusalReasonV1, cause?: unknown) => void;
}

/** 📂️ Resolves and opens one artifact; a failed opening names a refusal, a retired effect owner stays silent. */
export async function replayOpenArtifactV1<TOpening>(port: OpenArtifactReplayPortV1<TOpening>): Promise<void> {
  try {
    const opening = await port.resolve();
    if (opening === null) return port.refuse("router-not-ready");
    if ((await port.open(opening)) === "failed") port.refuse("open-rejected");
  } catch (error) {
    port.refuse("open-rejected", error);
  }
}
