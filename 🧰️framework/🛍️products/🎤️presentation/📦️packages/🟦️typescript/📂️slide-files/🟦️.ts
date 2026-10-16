/** 📂️ Presentation slide files: the types and file paths of slide and thought files, the assembly of thoughts and chapters from them and the intro specification a thought file may carry.
 *
 * @see ../🎞️structure/🟦️.ts — the structure assembled
 * @see ../🔭️scope/🟦️.ts — the artifact scope merged
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Transition, Participant, PresentationLanguageKind } from "../🧱️vocabulary/🟦️.ts";
import type { AuthorPerson, AffiliationEntry, Embodiment } from "../🎭️embodiments/🟦️.ts";
import type { ArtifactScope } from "../🔭️scope/🟦️.ts";
import type { Arrangement, Thought, Chapter } from "../🎞️structure/🟦️.ts";

//#region 🔖️SlideFile
/** 📄️ One slide module under `🎞️slide/<chapter>/<sequence>/<thought>/<slide>.ts`. */
export interface SlideFile extends ArtifactScope {
	readonly order: number;
	readonly arrangement: Arrangement;
	readonly transition?: Transition;
}

/** 📁️ Parsed path segments for a slide module file. */
export interface ParsedSlideFilePath {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** 📁️ Slide module path plus its parsed segments. */
export interface SlideFileModule {
	readonly path: ParsedSlideFilePath;
	readonly file: SlideFile;
}

/** 📁️ Parsed path segments for a thought template module (`🎞️slide/<chapter>/<sequence>/<thought>.ts`). */
export interface ParsedThoughtFilePath {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
}

/** 📄️ Thought module expanded into slides via a named template (e.g. intro). */
export type ThoughtFile = {
	readonly template: "intro";
	readonly spec: IntroSpec;
};

/** 📽️ Deck metadata paired with slide modules discovered from a glob import. */
export interface PresentationMeta {
	readonly id: string;
	readonly name: string;
	readonly language?: PresentationLanguageKind;
	readonly width?: number;
	readonly height?: number;
}

/** 🔤️ Stable id derived from a titleized presentation entity name. */
export function presentationNameToId(name: string): string {
	return name
		.normalize("NFD")
		.replace(/\p{M}/gu, "")
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
}

/** 📁️ Builds the canonical slide module path for one arrangement bookmark name. */
export function presentationSlideFilePath(
	chapter: string,
	sequence: string,
	thought: string,
	slide: string,
): string {
	return `🎞️slide/${chapter}/${sequence}/${thought}/${slide}.ts`;
}

/** 📁️ Builds the canonical thought template path for one Gedanke bookmark name. */
export function presentationThoughtFilePath(chapter: string, sequence: string, thought: string): string {
	return `🎞️slide/${chapter}/${sequence}/${thought}.ts`;
}

/**
 * 🔤️ Drops the decorative emoji prefix a `🎞️slide` folder or file segment carries for
 * human sorting, so the entity name is the text an author wrote (`🌷️Einführung` → `Einführung`).
 */
export function presentationPathSegmentName(segment: string): string {
	const name = segment.replace(/^[\p{Extended_Pictographic}\u200D\uFE0F\u{1F3FB}-\u{1F3FF}]+\s*/u, "");
	return name.length > 0 ? name : segment;
}

/** 📁️ Parses `🎞️slide/<chapter>/<sequence>/<thought>/<slide>.ts` from an import path. */
export function parsePresentationSlideFilePath(path: string): ParsedSlideFilePath | null {
	const normalized = path.replace(/\\/g, "/");
	const match = normalized.match(/(?:^|\/)🎞️slide\/([^/]+)\/([^/]+)\/([^/]+)\/([^/]+)\.ts$/u);
	if (!match) {
		return null;
	}
	return {
		chapter: presentationPathSegmentName(match[1]!),
		sequence: presentationPathSegmentName(match[2]!),
		thought: presentationPathSegmentName(match[3]!),
		slide: presentationPathSegmentName(match[4]!),
	};
}

/** 📁️ Parses `🎞️slide/<chapter>/<sequence>/<thought>.ts` from an import path. */
export function parsePresentationThoughtFilePath(path: string): ParsedThoughtFilePath | null {
	const normalized = path.replace(/\\/g, "/");
	const match = normalized.match(/(?:^|\/)🎞️slide\/([^/]+)\/([^/]+)\/([^/]+)\.ts$/u);
	if (!match) {
		return null;
	}
	return {
		chapter: presentationPathSegmentName(match[1]!),
		sequence: presentationPathSegmentName(match[2]!),
		thought: presentationPathSegmentName(match[3]!),
	};
}

function mergeArtifactScopeFromSlideFiles(modules: readonly SlideFile[]): ArtifactScope {
	const participants = new Map<string, Participant>();
	const embodiments = new Map<string, Embodiment>();
	for (const module of modules) {
		for (const participant of module.participants ?? []) {
			participants.set(participant.id, participant);
		}
		for (const embodiment of module.embodiments ?? []) {
			embodiments.set(embodiment.id, embodiment);
		}
	}
	return {
		participants: [...participants.values()],
		embodiments: [...embodiments.values()],
	};
}

/** 🧩️ Assembles one thought from slide modules sharing the same folder path. */
export function assembleThoughtFromSlideFiles(thoughtName: string, modules: readonly SlideFileModule[]): Thought {
	const sorted = [...modules].sort((left, right) => left.file.order - right.file.order);
	const slideFiles = sorted.map((module) => module.file);
	const scope = mergeArtifactScopeFromSlideFiles(slideFiles);
	return {
		id: presentationNameToId(thoughtName),
		name: thoughtName,
		participants: scope.participants,
		embodiments: scope.embodiments,
		slides: sorted.map((module) => ({
			arrangement: {
				...module.file.arrangement,
				name: module.file.arrangement.name ?? module.path.slide,
			},
			transition: module.file.transition,
		})),
	};
}

/** 📚️ Assembles one chapter from nested slide modules. */
export function assembleChapterFromSlideFiles(
	chapterName: string,
	thoughtModules: ReadonlyMap<string, readonly SlideFileModule[]>,
): Chapter {
	const sequences = new Map<string, Map<string, SlideFileModule[]>>();
	for (const [key, modules] of thoughtModules) {
		const [sequenceName, thoughtName] = key.split("\0");
		if (!sequenceName || !thoughtName) {
			throw new Error(`Invalid thought key "${key}" while assembling chapter "${chapterName}".`);
		}
		const byThought = sequences.get(sequenceName) ?? new Map<string, SlideFileModule[]>();
		byThought.set(thoughtName, [...modules]);
		sequences.set(sequenceName, byThought);
	}
	return {
		id: presentationNameToId(chapterName),
		name: chapterName,
		sequences: [...sequences.entries()]
			.sort(([left], [right]) => left.localeCompare(right, "de"))
			.map(([sequenceName, byThought]) => ({
				id: presentationNameToId(sequenceName),
				name: sequenceName,
				thoughts: [...byThought.entries()]
					.sort(([left], [right]) => left.localeCompare(right, "de"))
					.map(([thoughtName, modules]) => assembleThoughtFromSlideFiles(thoughtName, modules)),
			})),
	};
}

//#region 🔖️Intro
/** 🎬️ Spec for the standard paper intro template (title → description → goal → authors → affiliations ×3). */
export interface IntroSpec {
	readonly id?: string;
	readonly name?: string;
	readonly language?: PresentationLanguageKind;
	readonly title: {
		readonly full: readonly string[];
		readonly short: string;
	};
	readonly description: {
		readonly full: readonly string[];
		readonly short: string;
	};
	readonly goal: readonly string[];
	readonly authors: {
		readonly lines: readonly (readonly AuthorPerson[])[];
	};
	readonly affiliations: {
		readonly steps: readonly [
			readonly AffiliationEntry[],
			readonly AffiliationEntry[],
			readonly AffiliationEntry[],
		];
	};
}
//#endregion 🔖️Intro
//#endregion 🔖️SlideFile
