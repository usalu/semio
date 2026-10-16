/** 🧱️ Presentation vocabulary: how strongly a participant is emphasised, the transition into a slide, the participant itself, the metadata of an arrangement, the dispositions that place and style a participant and the language kinds a presentation is written in.
 *
 * @see ../🟦️.ts — the package entry that publishes this module
 */

//#region 🔖️Emphasis
/** 🎚️ Visual emphasis for a participant on one slide (maps to opacity layering in renderers). */
export type ParticipantEmphasis = "active" | "muted";
//#endregion 🔖️Emphasis

//#region 🔖️Transition
/** ↔ Transition from one slide to the next within a thought. */
export interface Transition {
	readonly kind: "morph" | "fade";
}
//#endregion 🔖️Transition

//#region 🔖️Participant
/** 🧑️ Entity that may appear on one or many slides (identity only; embodiments live in a scope registry). */
export interface Participant {
	readonly id: string;
	readonly name?: string;
}
//#endregion 🔖️Participant

//#region 🔖️Scope
/** 🗂️ Optional scene clip metadata for animate video exports. */
export interface ArrangementMetadata {
	readonly sceneHash?: string;
}

//#region 🔖️Disposition
/** 📐️ Normalized slide rectangle (0..1 fractions) for a {@link Disposition}. */
export interface DispositionPosition {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
}

/** 🎨️ Optional style overrides on a {@link Disposition}. */
export interface DispositionStyle {
	readonly opacity?: number;
	readonly rotate?: number;
	readonly scale?: number;
}

/** 🔀️ One source participant that morphs into a target disposition (many-to-one). */
export interface MorphFromSlot {
	readonly participantId: string;
	/** 📐️ Target position and frame where the source travels before the target embodiment appears. */
	readonly position: DispositionPosition;
	/** 🧩️ Source embodiment shown during the position morph (e.g. figure crop), before switching to the target. */
	readonly embodimentId: string;
	/** 🎯️ Line index when the target uses a multi-line text morph root (`participantId--index`). */
	readonly targetLineIndex?: number;
}

/** 🔀️ One target participant that morphs from a source disposition (one-to-many). */
export interface MorphToSlot {
	readonly participantId: string;
	/** 📐️ Source position and frame on the source slide for this target during the morph. */
	readonly position: DispositionPosition;
	/** 🧩️ Target embodiment on the next slide (fallback when the target disposition is missing). */
	readonly embodimentId?: string;
}

/** 📍️ Concrete positioned, styled embodiment of a participant on one arrangement. */
export interface Disposition {
	readonly participantId: string;
	readonly embodimentId: string;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	/** 🔀️ Source participants that morph into this disposition (many-to-one). */
	readonly morphFrom?: readonly MorphFromSlot[];
	/** 🔀️ Target participants that morph from this disposition (one-to-many). */
	readonly morphTo?: readonly MorphToSlot[];
	/** 🎯️ Morph anchor id when it differs from {@link Participant.id} (e.g. line index on multi-line text). */
	readonly morphAnchorId?: string;
}
//#endregion 🔖️Disposition

//#region 🔖️Presentation
/** 🌐️ Main language of a deck; drives localized URL bookmark query keys. */
export type PresentationLanguageKind = "de" | "en";
//#endregion 🔖️Presentation
//#endregion 🔖️Scope
