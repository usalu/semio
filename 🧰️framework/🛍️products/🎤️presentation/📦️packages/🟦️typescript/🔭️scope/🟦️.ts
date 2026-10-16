/** 🔭️ Presentation scope: the participants and embodiments an artifact scope holds and the resolution scope built from thoughts.
 *
 * @see ../🎭️embodiments/🟦️.ts — the embodiments a scope holds
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Participant, ArrangementMetadata } from "../🧱️vocabulary/🟦️.ts";
import type { Embodiment } from "../🎭️embodiments/🟦️.ts";

/** 🗂️ Optional participant and embodiment registries on a presentation artifact and its children. */
export interface ArtifactScope {
	readonly participants?: readonly Participant[];
	readonly embodiments?: readonly Embodiment[];
	readonly metadata?: ArrangementMetadata;
}

/** 🔍️ Merged participant and embodiment maps for resolving dispositions. */
export interface ResolutionScope {
	readonly participants: ReadonlyMap<string, Participant>;
	readonly embodiments: ReadonlyMap<string, Embodiment>;
}

/** 🔍️ Merges registries from outer to inner scope layers (later layers override earlier ids). */
export function buildResolutionScope(ancestors: readonly ArtifactScope[]): ResolutionScope {
	const participants = new Map<string, Participant>();
	const embodiments = new Map<string, Embodiment>();
	for (const layer of ancestors) {
		for (const participant of layer.participants ?? []) {
			participants.set(participant.id, participant);
		}
		for (const embodiment of layer.embodiments ?? []) {
			embodiments.set(embodiment.id, embodiment);
		}
	}
	return { participants, embodiments };
}
