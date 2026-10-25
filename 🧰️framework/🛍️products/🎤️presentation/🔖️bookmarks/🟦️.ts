/** 📑️ Presentation bookmarks: the query parameters and hash of a slide url, the bookmark names of entities and the collection of every slide of a presentation.
 *
 * @see ../📐️resolution/🟦️.ts — the expansion into slides
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { PresentationLanguageKind } from "../🧱️vocabulary/🟦️.ts";
import type { Presentation } from "../🎞️structure/🟦️.ts";
import { expandThoughtSlides } from "../📐️resolution/🟦️.ts";

//#region 🔖️Resolve
/** ⌨️ English bookmark query keys after the reveal.js hash path. */
export const PRESENTATION_CHAPTER_QUERY_PARAM = "chapter";

export const PRESENTATION_SEQUENCE_QUERY_PARAM = "sequence";

export const PRESENTATION_THOUGHT_QUERY_PARAM = "thought";

export const PRESENTATION_SLIDE_QUERY_PARAM = "slide";

/** 🔤️ Localized bookmark query keys for chapter, sequence, thought, and slide. */
export interface PresentationSlideBookmarkParamKeys {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** 🌐️ Resolves a deck's main language (`en` when unset). */
export function presentationLanguage(presentation: Presentation): PresentationLanguageKind {
	return presentation.language ?? "en";
}

/** 🗣️ Bookmark query param names for a presentation language (`sequenz`, `gedanke`, `folie` in German). */
export function presentationSlideBookmarkParamKeys(
	language: PresentationLanguageKind = "en",
): PresentationSlideBookmarkParamKeys {
	if (language === "de") {
		return {
			chapter: "kapitel",
			sequence: "sequenz",
			thought: "gedanke",
			slide: "folie",
		};
	}
	return {
		chapter: PRESENTATION_CHAPTER_QUERY_PARAM,
		sequence: PRESENTATION_SEQUENCE_QUERY_PARAM,
		thought: PRESENTATION_THOUGHT_QUERY_PARAM,
		slide: PRESENTATION_SLIDE_QUERY_PARAM,
	};
}

/** 🔑️ Bookmark ids carried in the URL hash query; navigation uses only the hash path. */
export interface PresentationSlideBookmark {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** 🔖️ Resolves the URL bookmark label for a sequence, thought, or arrangement. */
export function presentationEntityBookmarkName(entity: { readonly id: string; readonly name?: string }): string {
	return entity.name ?? entity.id;
}

/** 🔗️ One reveal.js slide location with localized bookmark labels for the URL. */
export interface PresentationSlideRef extends PresentationSlideBookmark {
	readonly h: number;
	readonly v: number;
}

/** 📋️ Lists every slide in reveal.js h/v order (one horizontal stack per sequence, chapters flattened). */
export function collectPresentationSlides(presentation: Presentation): readonly PresentationSlideRef[] {
	const slides: PresentationSlideRef[] = [];
	let h = 0;
	for (const chapter of presentation.chapters) {
		for (const sequence of chapter.sequences) {
			let v = 0;
			for (const thought of sequence.thoughts) {
				for (const renderSlide of expandThoughtSlides(thought)) {
					slides.push({
						h,
						v,
						chapter: presentationEntityBookmarkName(chapter),
						sequence: presentationEntityBookmarkName(sequence),
						thought: presentationEntityBookmarkName(thought),
						slide: presentationEntityBookmarkName(renderSlide),
					});
					v += 1;
				}
			}
			h += 1;
		}
	}
	return slides;
}

/** 🔎️ Resolves the slide at reveal.js indices within a deck. */
export function presentationSlideAt(
	presentation: Presentation,
	indices: { readonly h: number; readonly v: number },
): PresentationSlideRef | undefined {
	return collectPresentationSlides(presentation).find(
		(slide) => slide.h === indices.h && slide.v === indices.v,
	);
}

/** 🪧️ Formats reveal.js hash path (`/` = first slide, `/2/1` = h=2 v=1). */
export function formatPresentationSlideHash(indices: { readonly h: number; readonly v: number }): string {
	if (indices.h <= 0 && indices.v <= 0) {
		return "/";
	}
	let hash = `/${indices.h}`;
	if (indices.v > 0) {
		hash += `/${indices.v}`;
	}
	return hash;
}

/** 📖️ Parses reveal.js slide hash into zero-based h/v indices; ignores trailing bookmark query params. */
export function parsePresentationSlideHash(hash: string): { readonly h: number; readonly v: number } | null {
	const pathPart = hash.replace(/^#/, "").trim().split("?")[0]?.replace(/^\/?/, "").trim() ?? "";
	if (!pathPart) {
		return { h: 0, v: 0 };
	}
	const bits = pathPart.split("/");
	const h = Number.parseInt(bits[0] ?? "0", 10);
	const v = Number.parseInt(bits[1] ?? "0", 10);
	if (!Number.isFinite(h) || !Number.isFinite(v) || Number.isNaN(h) || Number.isNaN(v)) {
		return null;
	}
	return { h, v };
}

/** ⛓️ Formats reveal.js hash with localized sequence, thought, and slide bookmark params after the path. */
export function formatPresentationUrlHash(
	indices: { readonly h: number; readonly v: number },
	bookmark: PresentationSlideBookmark,
	language: PresentationLanguageKind = "en",
): string {
	const path = formatPresentationSlideHash(indices);
	const keys = presentationSlideBookmarkParamKeys(language);
	const params = new URLSearchParams([
		[keys.chapter, bookmark.chapter],
		[keys.sequence, bookmark.sequence],
		[keys.thought, bookmark.thought],
		[keys.slide, bookmark.slide],
	]);
	const query = params.toString();
	if (path === "/") {
		return `#/?${query}`;
	}
	return `#${path}?${query}`;
}
//#endregion 🔖️Resolve
