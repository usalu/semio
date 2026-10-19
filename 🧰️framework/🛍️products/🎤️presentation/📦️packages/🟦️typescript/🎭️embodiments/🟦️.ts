/** 🎭️ Presentation embodiments: the text morph roots and every embodiment a participant takes — text, figure, video, pdf, iframe, markdown, json, bullets, authors and affiliations — with the media scroll origin and the morph labels of the affiliation steps.
 *
 * @see ../🧱️vocabulary/🟦️.ts — emphasis and dispositions
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { ParticipantEmphasis, DispositionPosition } from "../🧱️vocabulary/🟦️.ts";

//#region 🔖️Morph
/** 📐️ reveal.js auto-animate DOM root for {@link TextEmbodiment} (see temp/eg-ice-25 intro slides). */
export type TextMorphRoot = "title" | "heading-block" | "heading-line" | "subheading-line" | "body";

/** 🎯️ Stable reveal.js `data-id` for a placed participant (participant-scoped across embodiments). */
export function morphId(participantId: string): string {
	return participantId;
}

/** 📐️ Chooses the eg-ice-25 text DOM root for reveal.js `data-id` pairing. */
export function resolveTextMorphRoot(embodiment: TextEmbodiment): TextMorphRoot {
	if (embodiment.morphRoot) {
		return embodiment.morphRoot;
	}
	if (embodiment.level === "title") {
		return "title";
	}
	if (embodiment.level === "body") {
		return "body";
	}
	if (embodiment.lines.length === 1) {
		return embodiment.level === "subheading" ? "subheading-line" : "heading-line";
	}
	return "heading-block";
}
//#endregion 🔖️Morph

//#region 🔖️Embodiment
/** 📝️ Text lines at a heading level; optional `fit` hints fit-text in reveal renderers. */
export interface TextEmbodiment {
	readonly kind: "text";
	readonly id: string;
	readonly lines: readonly string[];
	readonly level: "title" | "heading" | "subheading" | "body";
	readonly fit?: boolean;
	readonly morphRoot?: TextMorphRoot;
	/** 🔀️ Prior lines keyed by index when visible lines shorten or relabel (e.g. full description → short). */
	readonly morphFromLines?: readonly string[];
}

/** 🧩️ Rows×columns grid from {@link split}; enables seam-aligned background positioning. */
export interface FigureMosaicGrid {
	readonly rows: number;
	readonly columns: number;
	/** 📐️ Normalized source region the grid subdivides (default full image). */
	readonly frame?: DispositionPosition;
}

/** ✨️ Frosted overlay hinting at hidden or upcoming media; optional {@link MediaTeaser.label}. */
export interface MediaTeaser {
	readonly label?: string;
}

/** 📜️ Normalized scroll origin for scrollable media (CSS background-position semantics). */
export interface MediaScrollOrigin {
	readonly x?: number;
	readonly y?: number;
}

export const MEDIA_SCROLL_ORIGIN_CENTER: MediaScrollOrigin = { x: 50, y: 50 };

export const MEDIA_SCROLL_ORIGIN_TOP_LEFT: MediaScrollOrigin = { x: 0, y: 0 };

/** 📜️ Resolves optional {@link MediaScrollOrigin} to x/y percents (default center). */
export function resolveMediaScrollOrigin(origin: MediaScrollOrigin | undefined): {
	readonly x: number;
	readonly y: number;
} {
	return { x: origin?.x ?? 50, y: origin?.y ?? 50 };
}

/** 📜️ Initial scroll percent along one cover-overflow axis from a scroll origin. */
export function mediaScrollPercentForAxis(
	axis: "x" | "y",
	origin: MediaScrollOrigin | undefined,
): number {
	const resolved = resolveMediaScrollOrigin(origin);
	return axis === "x" ? resolved.x : resolved.y;
}

/** 🖼️ Raster or vector figure on a slide; optional {@link FigureEmbodiment.crop} for a normalized source region. */
export interface FigureEmbodiment {
	readonly kind: "figure";
	readonly id: string;
	readonly src: string;
	readonly alt?: string;
	readonly crop?: DispositionPosition;
	/** 📐️ Source bitmap width÷height; used with {@link FigureEmbodiment.crop} for uniform cover (default 1). */
	readonly sourceAspect?: number;
	/** 🧩️ When set, crop backgrounds use edge-aligned positions so adjacent cells do not overlap. */
	readonly mosaic?: FigureMosaicGrid;
	/** 📜️ When false, clip cover overflow instead of one-axis scroll (default scrollable; mosaic tiles always clip). */
	readonly scroll?: boolean;
	/** ✨️ When set, shows a glassy veil over the figure (optional label). */
	readonly teaser?: MediaTeaser;
	/** 📜️ Initial scroll origin as CSS background-position percents (default center). */
	readonly scrollOrigin?: MediaScrollOrigin;
}

/** 🎬️ Video clip on a slide. */
export interface VideoEmbodiment {
	readonly kind: "video";
	readonly id: string;
	readonly src: string;
	readonly poster?: string;
	readonly autoplay?: boolean;
	readonly loop?: boolean;
	readonly muted?: boolean;
	readonly controls?: boolean;
	/** 📜️ When false, clip cover overflow instead of one-axis scroll (default scrollable). */
	readonly scroll?: boolean;
	/** ✨️ When set, shows a glassy veil over the video (optional label). */
	readonly teaser?: MediaTeaser;
	/** 📜️ Initial scroll origin as CSS background-position percents (default center). */
	readonly scrollOrigin?: MediaScrollOrigin;
}

/** 📄️ PDF document page on a slide. */
export interface PdfEmbodiment {
	readonly kind: "pdf";
	readonly id: string;
	readonly src: string;
	/** 📃️ Initial page when no ephemeral override exists; must be in {@link PdfEmbodiment.pages} when that is set. */
	readonly page?: number;
	/** 📑️ Ordered subset for prev/next navigation (e.g. thesis excerpt `[1, 12, 25, 35, 42, 43, 51]`). */
	readonly pages?: readonly number[];
	readonly alt?: string;
	/** 📜️ When false, clip cover overflow instead of one-axis scroll (default scrollable). */
	readonly scroll?: boolean;
	/** ✨️ When set, shows a glassy veil over the pdf (optional label). */
	readonly teaser?: MediaTeaser;
	/** 📜️ Initial scroll origin as CSS background-position percents (default center). */
	readonly scrollOrigin?: MediaScrollOrigin;
}

/** 🪟️ Embedded HTML document on a slide. */
export interface IframeEmbodiment {
	readonly kind: "iframe";
	readonly id: string;
	readonly src: string;
	readonly title?: string;
	/** ✨️ When set, shows a glassy veil in front of the iframe (optional label). */
	readonly teaser?: MediaTeaser;
}

/** 📝️ Markdown body on a slide; rendered with repo prose typography in React renderers. */
export interface MarkdownEmbodiment {
	readonly kind: "markdown";
	readonly id: string;
	readonly src: string;
	readonly markdown?: string;
	readonly title?: string;
}

/** 🧬️ JSON document on a slide; rendered as an expandable syntax tree in React renderers. */
export interface JsonEmbodiment {
	readonly kind: "json";
	readonly id: string;
	readonly src: string;
	readonly json?: unknown;
	readonly title?: string;
}

/** 🔘️ Bulleted list body. */
export interface BulletEmbodiment {
	readonly kind: "bullet";
	readonly id: string;
	readonly items: readonly string[];
}

/** 🔢️ One superscript affiliation mark on an author (optional fade for marks introduced earlier). */
export interface AuthorMark {
	readonly mark: string;
	readonly emphasis?: ParticipantEmphasis;
}

/** 👤️ One author name with optional affiliation marks. */
export interface AuthorPerson {
	readonly name: string;
	readonly marks?: readonly string[];
	readonly markEntries?: readonly AuthorMark[];
}

/** 👤️ Author rows (names with optional superscript marks); use `lines` for multiple rows. */
export interface AuthorsEmbodiment {
	readonly kind: "authors";
	readonly id: string;
	readonly people?: readonly AuthorPerson[];
	readonly lines?: readonly (readonly AuthorPerson[])[];
	readonly abbreviateFirstName?: boolean;
}

/** 👤️ Abbreviates the first given name (`Ueli Saluz` → `U. Saluz`) for compact author rows with affiliation marks. */
export function abbreviateAuthorFirstName(fullName: string): string {
	const parts = fullName.trim().split(/\s+/).filter(Boolean);
	if (parts.length < 2) {
		return fullName.trim();
	}
	const [first, ...rest] = parts;
	const initial = first.codePointAt(0);
	if (initial === undefined) {
		return fullName.trim();
	}
	return `${String.fromCodePoint(initial).toLocaleUpperCase("de-DE")}. ${rest.join(" ")}`;
}

/** 🏛️ One affiliation line with optional second mark+name on the same row (e.g. university + chair). */
export interface AffiliationEntry {
	readonly mark: string;
	readonly name: string;
	readonly shortName?: string;
	readonly suffix?: { readonly mark: string; readonly name: string };
	readonly lineEmphasis?: ParticipantEmphasis;
	readonly suffixEmphasis?: ParticipantEmphasis;
}

/** 🏛️ Affiliation line label (`shortName` when set, else full `name`). */
export function affiliationLineName(entry: AffiliationEntry): string {
	return entry.shortName ?? entry.name;
}

/** 🔀️ Maps affiliation marks whose line label changes between steps to the prior label for reveal.js morph pairing. */
export function affiliationEmbodimentMorphLabels(
	previousStep: readonly AffiliationEntry[],
	currentStep: readonly AffiliationEntry[],
): Readonly<Record<string, string>> {
	const previousByMark = new Map(previousStep.map((entry) => [entry.mark, entry]));
	const labels: Record<string, string> = {};
	for (const entry of currentStep) {
		const previous = previousByMark.get(entry.mark);
		if (!previous) {
			continue;
		}
		const fromLabel = affiliationLineName(previous);
		const toLabel = affiliationLineName(entry);
		if (fromLabel !== toLabel) {
			labels[entry.mark] = fromLabel;
		}
	}
	return labels;
}

/** 🏛️ Collects `mark` and `suffix.mark` values presentation in one affiliation step. */
export function affiliationMarksInStep(step: readonly AffiliationEntry[]): ReadonlySet<string> {
	const marks = new Set<string>();
	for (const entry of step) {
		marks.add(entry.mark);
		if (entry.suffix) {
			marks.add(entry.suffix.mark);
		}
	}
	return marks;
}

/** 🏛️ Footnote mark order as listed in one affiliation step (line marks, then suffix marks). */
export function affiliationMarkOrderInStep(step: readonly AffiliationEntry[]): readonly string[] {
	const order: string[] = [];
	for (const entry of step) {
		if (!order.includes(entry.mark)) {
			order.push(entry.mark);
		}
		if (entry.suffix && !order.includes(entry.suffix.mark)) {
			order.push(entry.suffix.mark);
		}
	}
	return order;
}

/** 👤️ Author rows with only marks defined in `currentStep`; newer marks vs `previousStep` stay active. */
export function authorLinesForAffiliationStep(
	lines: readonly (readonly AuthorPerson[])[],
	currentStep: readonly AffiliationEntry[],
	previousStep: readonly AffiliationEntry[],
): readonly (readonly AuthorPerson[])[] {
	const allowed = affiliationMarksInStep(currentStep);
	const previous = affiliationMarksInStep(previousStep);
	const markOrder = affiliationMarkOrderInStep(currentStep);
	const markRank = new Map(markOrder.map((mark, index) => [mark, index]));
	return lines.map((line) =>
		line.map((author) => {
			const markEntries = (author.marks ?? [])
				.filter((mark) => allowed.has(mark))
				.map((mark) => ({
					mark,
					emphasis: previous.has(mark) ? ("muted" as const) : ("active" as const),
				}))
				.sort((left, right) => (markRank.get(left.mark) ?? 0) - (markRank.get(right.mark) ?? 0));
			return { name: author.name, markEntries };
		}),
	);
}

/** 🏛️ Mutes prior affiliation lines; only marks or suffixes new vs `previousStep` stay active. */
export function highlightAffiliationDelta(
	currentStep: readonly AffiliationEntry[],
	previousStep: readonly AffiliationEntry[],
): AffiliationEntry[] {
	const previousByMark = new Map(previousStep.map((entry) => [entry.mark, entry]));
	return currentStep.map((entry) => {
		const previous = previousByMark.get(entry.mark);
		if (!previous) {
			return { ...entry, lineEmphasis: "active" as const };
		}
		if (entry.suffix && !previous.suffix) {
			return {
				...entry,
				lineEmphasis: "muted" as const,
				suffixEmphasis: "active" as const,
			};
		}
		return { ...entry, lineEmphasis: "muted" as const, suffixEmphasis: "muted" as const };
	});
}

/** 🏛️ Affiliation footnotes keyed by mark. */
export interface AffiliationsEmbodiment {
	readonly kind: "affiliations";
	readonly id: string;
	readonly entries: readonly AffiliationEntry[];
	/** 🔀️ Prior line labels keyed by mark when the visible label changes after position morph (e.g. full name → shortName). */
	readonly morphLineLabels?: Readonly<Record<string, string>>;
}

/** 🎭️ One visual form a {@link Participant} may take on a slide. */
export type Embodiment =
	| TextEmbodiment
	| FigureEmbodiment
	| VideoEmbodiment
	| PdfEmbodiment
	| IframeEmbodiment
	| MarkdownEmbodiment
	| JsonEmbodiment
	| BulletEmbodiment
	| AuthorsEmbodiment
	| AffiliationsEmbodiment;
//#endregion 🔖️Embodiment
