/** 🔄️ Presentation analogy and play: the analogy template between a source and a target and the definitions of the play apps.
 *
 * @see ../🎞️structure/🟦️.ts — the structure an analogy builds
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Participant, Disposition } from "../🧱️vocabulary/🟦️.ts";
import type { Embodiment } from "../🎭️embodiments/🟦️.ts";
import type { Thought, Presentation } from "../🎞️structure/🟦️.ts";

//#region 🔖️Analogy
export const ANALOGY_PARTICIPANT_LABEL = "label";

export const ANALOGY_PARTICIPANT_VISUAL = "visual";

const ANALOGY_EMBODIMENT_LABEL_SOURCE = "label--source";

const ANALOGY_EMBODIMENT_LABEL_TARGET = "label--target";

const ANALOGY_EMBODIMENT_VISUAL_SOURCE = "visual--source";

const ANALOGY_EMBODIMENT_VISUAL_TARGET = "visual--target";

/** 🔀️ Spec for a two-slide analogy (source concept morphs into target via shared participant ids). */
export interface AnalogySpec {
	readonly id?: string;
	readonly name?: string;
	readonly source: {
		readonly label: string;
		readonly figure?: string;
	};
	readonly target: {
		readonly label: string;
		readonly figure?: string;
	};
}

/** 🔀️ Builds a morph thought: source arrangement then mapping arrangement (reveal.js auto-animate). */
export function analogy(spec: AnalogySpec): Presentation {
	const participants: Participant[] = [{ id: ANALOGY_PARTICIPANT_LABEL }];
	const embodiments: Embodiment[] = [
		{
			kind: "text",
			id: ANALOGY_EMBODIMENT_LABEL_SOURCE,
			lines: [spec.source.label],
			level: "heading",
			morphRoot: "heading-line",
		},
		{
			kind: "text",
			id: ANALOGY_EMBODIMENT_LABEL_TARGET,
			lines: [spec.target.label],
			level: "heading",
			morphRoot: "heading-line",
		},
	];
	if (spec.source.figure) {
		embodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_SOURCE,
			src: spec.source.figure,
			alt: spec.source.label,
		});
	}
	if (spec.target.figure) {
		embodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_TARGET,
			src: spec.target.figure,
			alt: spec.target.label,
		});
	}
	if (spec.source.figure || spec.target.figure) {
		participants.push({ id: ANALOGY_PARTICIPANT_VISUAL });
	}

	const sourceDispositions: Disposition[] = [
		{
			participantId: ANALOGY_PARTICIPANT_LABEL,
			embodimentId: ANALOGY_EMBODIMENT_LABEL_SOURCE,
			emphasis: "active",
		},
	];
	const mappingDispositions: Disposition[] = [
		{
			participantId: ANALOGY_PARTICIPANT_LABEL,
			embodimentId: ANALOGY_EMBODIMENT_LABEL_TARGET,
			emphasis: "active",
		},
	];
	if (spec.source.figure) {
		sourceDispositions.push({
			participantId: ANALOGY_PARTICIPANT_VISUAL,
			embodimentId: ANALOGY_EMBODIMENT_VISUAL_SOURCE,
			emphasis: "active",
			position: { x: 0.1, y: 0.35, width: 0.8, height: 0.5 },
		});
	}
	if (spec.target.figure) {
		mappingDispositions.push({
			participantId: ANALOGY_PARTICIPANT_VISUAL,
			embodimentId: ANALOGY_EMBODIMENT_VISUAL_TARGET,
			emphasis: "active",
			position: { x: 0.1, y: 0.35, width: 0.8, height: 0.5 },
		});
	}

	const thought: Thought = {
		id: "analogy",
		participants,
		embodiments,
		slides: [
			{ arrangement: { id: "source", dispositions: sourceDispositions }, transition: { kind: "morph" } },
			{ arrangement: { id: "mapping", dispositions: mappingDispositions } },
		],
	};

	return {
		id: spec.id ?? "analogy",
		name: spec.name ?? `${spec.source.label} → ${spec.target.label}`,
		chapters: [
			{
				id: "main",
				sequences: [{ id: "main", thoughts: [thought] }],
			},
		],
	};
}
//#endregion 🔖️Analogy

//#region 🔖️Play
/** 🛝️ Minimal play-app registration stub for presentation hosts. */
export const animatePlayAppDefinition = {
	id: "animate",
	label: "Animate Presentation",
	controllerId: "animate",
	modes: [{ id: "presentation", label: "Presentation" }],
	defaultModeId: "presentation",
	devHost: {
		playEntryKind: "animate",
	},
} as const;

/** 📽️ Concrete play-app definition consumed by presentation hosts. */
export const presentationPlayAppDefinition = animatePlayAppDefinition;
//#endregion 🔖️Play
