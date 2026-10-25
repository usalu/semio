/** 🎞️ Presentation structure: arrangements, slides, thoughts, sequences, chapters and the presentation, and the scope an arrangement resolves in.
 *
 * @see ../🔭️scope/🟦️.ts — the scopes the structure resolves in
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Transition, Disposition, PresentationLanguageKind } from "../🧱️vocabulary/🟦️.ts";
import { type ArtifactScope, type ResolutionScope, buildResolutionScope } from "../🔭️scope/🟦️.ts";

//#region 🔖️Presentation
//#region 🔖️Scope
/** 🔍️ Builds the resolution scope for one arrangement inside a deck hierarchy. */
export function resolutionScopeForArrangement(
	presentation: Presentation,
	chapter: Chapter,
	sequence: Sequence,
	thought: Thought,
	arrangement: Arrangement,
): ResolutionScope {
	return buildResolutionScope([presentation, chapter, sequence, thought, arrangement]);
}
//#endregion 🔖️Scope

//#region 🔖️Arrangement
/** 🖼️ One slide: participants disposed with emphasis, position, and style. */
export interface Arrangement extends ArtifactScope {
	readonly id: string;
	/** 🔖️ URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly dispositions: readonly Disposition[];
	/** ⏳️ Arrangement ids that settle ephemeral layout before auto-animating to them. */
	readonly settleBeforeMorphTo?: readonly string[];
}
//#endregion 🔖️Arrangement

//#region 🔖️Slide
/** 🎤️ One slide: an arrangement with an optional transition to the next slide. */
export interface Slide {
	readonly arrangement: Arrangement;
	readonly transition?: Transition;
}
//#endregion 🔖️Slide

//#region 🔖️Thought
/** 💭️ Idea developed across one or more slides with scoped participants and embodiments. */
export interface Thought extends ArtifactScope {
	readonly id: string;
	/** 🌐️ URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly slides: readonly Slide[];
}
//#endregion 🔖️Thought

//#region 🔖️Sequence
/** 📚️ Reveal.js horizontal stack: ordered thoughts rendered as one vertical slide column. */
export interface Sequence extends ArtifactScope {
	readonly id: string;
	/** 🔗️ URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly thoughts: readonly Thought[];
}
//#endregion 🔖️Sequence

//#region 🔖️Chapter
/** 📖️ Groups related sequences in a deck (bookmarks and authoring; sequences stay top-level in reveal.js). */
export interface Chapter extends ArtifactScope {
	readonly id: string;
	/** 🔤️ URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly sequences: readonly Sequence[];
}
//#endregion 🔖️Chapter

/** 📽️ Root deck: ordered chapters of sequences of thoughts. */
export interface Presentation extends ArtifactScope {
	readonly id: string;
	readonly name: string;
	readonly chapters: readonly Chapter[];
	readonly language?: PresentationLanguageKind;
	readonly width?: number;
	readonly height?: number;
}
//#endregion 🔖️Presentation

//#region 🔖️Traverse
/** 🧭️ Flattens all sequences in chapter order (one reveal.js horizontal stack per sequence). */
export function presentationSequences(presentation: Presentation): readonly Sequence[] {
	return presentation.chapters.flatMap((chapter) => chapter.sequences);
}
//#endregion 🔖️Traverse
