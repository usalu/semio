/** 📐️ Presentation resolution: resolved dispositions with their bounds and centring, the expansion of thoughts into render slides and the resolution of embodiments and arrangements in a scope.
 *
 * @see ../🎞️structure/🟦️.ts — thoughts, slides and arrangements
 * @see ../🔭️scope/🟦️.ts — the resolution scope
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { ParticipantEmphasis, Transition, Participant, ArrangementMetadata, DispositionPosition, DispositionStyle, MorphFromSlot, MorphToSlot, Disposition } from "../🧱️vocabulary/🟦️.ts";
import { morphId, type Embodiment } from "../🎭️embodiments/🟦️.ts";
import type { ResolutionScope } from "../🔭️scope/🟦️.ts";
import { type Arrangement, type Slide, type Thought, type Presentation, presentationSequences } from "../🎞️structure/🟦️.ts";

//#region 🔖️Resolved
/** ✅️ One participant embodiment resolved for rendering a single arrangement. */
export interface ResolvedDisposition {
	readonly participant: Participant;
	readonly embodiment: Embodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly embodimentId: string;
	readonly morphId: string;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	/** 🔀️ Presentation when this disposition receives a many-to-one morph. */
	readonly morphFrom?: readonly MorphFromSlot[];
	/** 🔀️ Presentation when this disposition is the one in a one-to-many morph. */
	readonly morphTo?: readonly MorphToSlot[];
}

/** 📐️ Union of normalized placement rectangles. */
export function unionDispositionPositions(positions: readonly DispositionPosition[]): DispositionPosition {
	if (positions.length === 0) {
		throw new Error("unionDispositionPositions: no positions.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const position of positions) {
		minX = Math.min(minX, position.x);
		minY = Math.min(minY, position.y);
		maxX = Math.max(maxX, position.x + position.width);
		maxY = Math.max(maxY, position.y + position.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** ⊕ Translation that centers {@link bounds} in the unit slide. */
export function centerDispositionBoundsOffset(bounds: DispositionPosition): DispositionPosition {
	return {
		x: (1 - bounds.width) / 2 - bounds.x,
		y: (1 - bounds.height) / 2 - bounds.y,
		width: 0,
		height: 0,
	};
}

/** 📐️ Default slide width÷height used by the React renderer when unset (960×700). */
export const PRESENTATION_DEFAULT_SLIDE_ASPECT = 960 / 700;

/** 📐️ Largest centered frame whose on-slide physical aspect matches {@link sourceAspect}. */
export function figureFrameForSourceAspect(
	sourceAspect: number,
	slideAspect = PRESENTATION_DEFAULT_SLIDE_ASPECT,
	padding = { x: 0.04, y: 0.06 },
): DispositionPosition {
	const maxWidth = 1 - padding.x * 2;
	const maxHeight = 1 - padding.y * 2;
	let height = maxHeight;
	let width = (height * sourceAspect) / slideAspect;
	if (width > maxWidth) {
		width = maxWidth;
		height = (width * slideAspect) / sourceAspect;
	}
	return {
		x: (1 - width) / 2,
		y: (1 - height) / 2,
		width,
		height,
	};
}

/** ↔ Moves a placement by a normalized offset. */
export function shiftDispositionPosition(
	position: DispositionPosition,
	offset: DispositionPosition,
): DispositionPosition {
	return {
		x: position.x + offset.x,
		y: position.y + offset.y,
		width: position.width,
		height: position.height,
	};
}

const CENTER_ARRANGEMENT_EPSILON = 1e-6;

function isDispositionVisibleForLayout(disposition: ResolvedDisposition): boolean {
	return disposition.style?.opacity !== 0;
}

/** 📍️ Dispositions declared on the arrangement at rest. */
export function arrangementRestDispositions(arrangement: Arrangement): readonly Disposition[] {
	return arrangement.dispositions;
}

/** 📍️ Slide positions for layout centering (visible tiles and frames; omits opacity-0 dispositions). */
export function visibleArrangementPositions(resolved: readonly ResolvedDisposition[]): DispositionPosition[] {
	const positions: DispositionPosition[] = [];
	for (const disposition of resolved) {
		if (!isDispositionVisibleForLayout(disposition)) {
			continue;
		}
		if (disposition.position) {
			positions.push(disposition.position);
		}
	}
	return positions;
}

function shiftResolvedDisposition(
	disposition: ResolvedDisposition,
	offset: DispositionPosition,
): ResolvedDisposition {
	if (!disposition.position) {
		return disposition;
	}
	return {
		...disposition,
		position: shiftDispositionPosition(disposition.position, offset),
	};
}

/** ⊕ Centers visible placements in the unit slide frame. */
export function centerResolvedArrangement(resolved: readonly ResolvedDisposition[]): ResolvedDisposition[] {
	const positions = visibleArrangementPositions(resolved);
	if (positions.length === 0) {
		return [...resolved];
	}
	const bounds = unionDispositionPositions(positions);
	const offset = centerDispositionBoundsOffset(bounds);
	if (
		Math.abs(offset.x) < CENTER_ARRANGEMENT_EPSILON &&
		Math.abs(offset.y) < CENTER_ARRANGEMENT_EPSILON
	) {
		return [...resolved];
	}
	return resolved.map((disposition) => shiftResolvedDisposition(disposition, offset));
}
//#endregion 🔖️Resolved

//#region 🔖️Expand
/** 🎞️ One renderable slide after morph expansion for reveal.js. */
export interface RenderSlide {
	readonly id: string;
	/** 🔖️ URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly arrangement: Arrangement;
	/** ↔ reveal.js `data-auto-animate-id` shared by consecutive morph-linked slides in one run. */
	readonly autoAnimateId?: string;
	readonly metadata?: ArrangementMetadata;
}

function transitionUsesMorph(transition: Transition | undefined): boolean {
	return (transition?.kind ?? "morph") === "morph";
}

function slideParticipantIds(slide: Slide): ReadonlySet<string> {
	return new Set(slide.arrangement.dispositions.map((disposition) => disposition.participantId));
}

function slideMorphParticipantIds(slide: Slide): ReadonlySet<string> {
	const ids = new Set(slideParticipantIds(slide));
	for (const disposition of slide.arrangement.dispositions) {
		for (const slot of disposition.morphFrom ?? []) {
			ids.add(slot.participantId);
		}
		for (const slot of disposition.morphTo ?? []) {
			ids.add(slot.participantId);
		}
	}
	return ids;
}

/** 🔗️ True when two consecutive slides share at least one participant for morph transitions. */
export function slidesShareMorphParticipants(source: Slide, target: Slide): boolean {
	const sourceIds = slideMorphParticipantIds(source);
	for (const participantId of slideMorphParticipantIds(target)) {
		if (sourceIds.has(participantId)) {
			return true;
		}
	}
	return false;
}

/** 🎞️ Expands {@link Thought.slides} with morph-run auto-animate ids for renderers that pair consecutive slides. */
export function expandThoughtSlides(thought: Thought): readonly RenderSlide[] {
	const slides = thought.slides;
	if (slides.length === 0) {
		return [];
	}
	const expanded: RenderSlide[] = [];
	let runIndex = 0;
	let index = 0;
	while (index < slides.length) {
		let runEnd = index;
		while (runEnd < slides.length - 1 && transitionUsesMorph(slides[runEnd]?.transition)) {
			const current = slides[runEnd]!;
			const next = slides[runEnd + 1]!;
			if (!slidesShareMorphParticipants(current, next)) {
				break;
			}
			runEnd += 1;
		}
		const autoAnimateId = runEnd > index ? `${thought.id}--m${runIndex}` : undefined;
		runIndex += 1;
		for (let slideIndex = index; slideIndex <= runEnd; slideIndex += 1) {
			const slide = slides[slideIndex]!;
			expanded.push({
				id: slide.arrangement.id,
				name: slide.arrangement.name,
				arrangement: slide.arrangement,
				autoAnimateId,
				metadata: slide.arrangement.metadata,
			});
		}
		index = runEnd + 1;
	}
	return expanded;
}
//#endregion 🔖️Expand

//#region 🔖️Resolve
/** 🔍️ Looks up one embodiment in a {@link ResolutionScope}. */
export function resolveEmbodiment(scope: ResolutionScope, embodimentId: string): Embodiment {
	const match = scope.embodiments.get(embodimentId);
	if (!match) {
		throw new Error(`Unknown embodiment "${embodimentId}".`);
	}
	return match;
}

/** 🔍️ Resolves all dispositions for one arrangement against a scope. */
export function resolveArrangement(scope: ResolutionScope, arrangement: Arrangement): ResolvedDisposition[] {
	return arrangement.dispositions.flatMap((disposition) => {
		const participant = scope.participants.get(disposition.participantId);
		if (!participant) {
			throw new Error(`Arrangement "${arrangement.id}" references unknown participant "${disposition.participantId}".`);
		}
		const morphIntoTarget = (disposition.morphFrom?.length ?? 0) > 0;
		const resolvedMorphId = morphIntoTarget
			? `${morphId(participant.id)}--label`
			: (disposition.morphAnchorId ?? morphId(participant.id));
		return [
			{
				participant,
				embodiment: resolveEmbodiment(scope, disposition.embodimentId),
				emphasis: disposition.emphasis,
				embodimentId: disposition.embodimentId,
				morphId: resolvedMorphId,
				position: disposition.position,
				style: disposition.style,
				morphFrom: disposition.morphFrom,
				morphTo: disposition.morphTo,
			},
		];
	});
}

/** 🔢️ Counts render slides across all chapters and sequences. */
export function countArrangements(presentation: Presentation): number {
	return presentationSequences(presentation).reduce(
		(sum, sequence) =>
			sum + sequence.thoughts.reduce((thoughtSum, thought) => thoughtSum + expandThoughtSlides(thought).length, 0),
		0,
	);
}
//#endregion 🔖️Resolve
