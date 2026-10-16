/** 🎬️ Presentation intro: the template that turns title, description, goal, authors and affiliations into the slides and thought file of an introduction.
 *
 * @see ../📂️slide-files/🟦️.ts — the slide files the intro writes
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Participant, Disposition, PresentationLanguageKind } from "../🧱️vocabulary/🟦️.ts";
import { affiliationEmbodimentMorphLabels, authorLinesForAffiliationStep, highlightAffiliationDelta, type Embodiment } from "../🎭️embodiments/🟦️.ts";
import type { Arrangement, Presentation } from "../🎞️structure/🟦️.ts";
import { type SlideFile, type SlideFileModule, type ParsedThoughtFilePath, type ThoughtFile, presentationThoughtFilePath, assembleChapterFromSlideFiles, type IntroSpec } from "../📂️slide-files/🟦️.ts";

//#region 🔖️Intro
export const INTRO_PARTICIPANT_TITLE = "title";

export const INTRO_PARTICIPANT_DESCRIPTION = "description";

export const INTRO_PARTICIPANT_GOAL = "goal";

export const INTRO_PARTICIPANT_AUTHORS = "authors";

export const INTRO_PARTICIPANT_INSTITUTIONS = "institutions";

export const INTRO_EMBODIMENT_TITLE_FULL = "title--full";

export const INTRO_EMBODIMENT_TITLE_SHORT = "title--short";

export const INTRO_EMBODIMENT_DESCRIPTION_FULL = "description--full";

export const INTRO_EMBODIMENT_DESCRIPTION_SHORT = "description--short";

export const INTRO_EMBODIMENT_GOAL = "goal--main";

export const INTRO_EMBODIMENT_AUTHORS_PLAIN = "authors--plain";

export const INTRO_EMBODIMENT_AUTHORS_MARKED = "authors--marked";

export const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1 = "authors--marked-affiliations-step1";

export const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2 = "authors--marked-affiliations-step2";

export const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3 = "authors--marked-affiliations-step3";

export const INTRO_EMBODIMENT_INSTITUTIONS_STEP1 = "institutions--step1";

export const INTRO_EMBODIMENT_INSTITUTIONS_STEP2 = "institutions--step2";

export const INTRO_EMBODIMENT_INSTITUTIONS_STEP3 = "institutions--step3";

const INTRO_CHAPTER_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Main",
	de: "Hauptteil",
};

const INTRO_SEQUENCE_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Introduction",
	de: "Einführung",
};

const INTRO_THOUGHT_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Introduction",
	de: "Einleitung",
};

const INTRO_ARRANGEMENT_BOOKMARK: Record<PresentationLanguageKind, Record<string, string>> = {
	en: {
		title: "Title",
		description: "Description",
		goal: "Goal",
		authors: "Authors",
		"affiliations-1": "Faculty",
		"affiliations-2": "Universities",
		"affiliations-3": "Chairs",
	},
	de: {
		title: "Titel",
		description: "Beschreibung",
		goal: "Ziel",
		authors: "Autoren",
		"affiliations-1": "Fakultät",
		"affiliations-2": "Universitäten",
		"affiliations-3": "Lehrstühle",
	},
};

/** 🎬️ Arrangement ids produced by the intro template (language-independent). */
export const INTRO_ARRANGEMENT_IDS = new Set(Object.keys(INTRO_ARRANGEMENT_BOOKMARK.en)) as ReadonlySet<string>;

/** 🎬️ True when an arrangement belongs to the standard intro slide sequence. */
export function isIntroArrangementId(arrangementId: string): boolean {
	return INTRO_ARRANGEMENT_IDS.has(arrangementId);
}

function introBookmarkLanguage(language: PresentationLanguageKind | undefined): PresentationLanguageKind {
	return language ?? "en";
}

function introArrangementBookmarkName(
	arrangementId: string,
	language: PresentationLanguageKind | undefined,
): string {
	return INTRO_ARRANGEMENT_BOOKMARK[introBookmarkLanguage(language)][arrangementId] ?? arrangementId;
}

function introParticipants(): Participant[] {
	return [
		{ id: INTRO_PARTICIPANT_TITLE },
		{ id: INTRO_PARTICIPANT_DESCRIPTION },
		{ id: INTRO_PARTICIPANT_GOAL },
		{ id: INTRO_PARTICIPANT_AUTHORS },
		{ id: INTRO_PARTICIPANT_INSTITUTIONS },
	];
}

function introEmbodiments(spec: IntroSpec): Embodiment[] {
	return [
		{
			kind: "text",
			id: INTRO_EMBODIMENT_TITLE_FULL,
			lines: spec.title.full,
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "text",
			id: INTRO_EMBODIMENT_TITLE_SHORT,
			lines: [spec.title.short],
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "text",
			id: INTRO_EMBODIMENT_DESCRIPTION_FULL,
			lines: spec.description.full,
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "text",
			id: INTRO_EMBODIMENT_DESCRIPTION_SHORT,
			lines: [spec.description.short],
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "text",
			id: INTRO_EMBODIMENT_GOAL,
			lines: spec.goal,
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_PLAIN,
			lines: spec.authors.lines.map((line) => line.map((author) => ({ name: author.name }))),
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_MARKED,
			lines: spec.authors.lines,
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1,
			lines: authorLinesForAffiliationStep(spec.authors.lines, spec.affiliations.steps[0], []),
			abbreviateFirstName: true,
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2,
			lines: authorLinesForAffiliationStep(
				spec.authors.lines,
				spec.affiliations.steps[1],
				spec.affiliations.steps[0],
			),
			abbreviateFirstName: true,
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3,
			lines: authorLinesForAffiliationStep(
				spec.authors.lines,
				spec.affiliations.steps[2],
				spec.affiliations.steps[1],
			),
			abbreviateFirstName: true,
		},
		{
			kind: "affiliations",
			id: INTRO_EMBODIMENT_INSTITUTIONS_STEP1,
			entries: spec.affiliations.steps[0],
		},
		{
			kind: "affiliations",
			id: INTRO_EMBODIMENT_INSTITUTIONS_STEP2,
			entries: highlightAffiliationDelta(spec.affiliations.steps[1], spec.affiliations.steps[0]),
		},
		{
			kind: "affiliations",
			id: INTRO_EMBODIMENT_INSTITUTIONS_STEP3,
			entries: highlightAffiliationDelta(spec.affiliations.steps[2], spec.affiliations.steps[1]),
			morphLineLabels: affiliationEmbodimentMorphLabels(spec.affiliations.steps[1], spec.affiliations.steps[2]),
		},
	];
}

/** 🎬️ Slide modules for the standard intro thought (`🎞️slide/<chapter>/<sequence>/<thought>/<slide>.ts`). */
export function introSlideFiles(spec: IntroSpec): readonly SlideFile[] {
	const language = introBookmarkLanguage(spec.language);
	const participants = introParticipants();
	const embodiments = introEmbodiments(spec);
	const muted = (participantId: string, embodimentId: string): Disposition => ({
		participantId,
		embodimentId,
		emphasis: "muted",
	});
	const active = (participantId: string, embodimentId: string): Disposition => ({
		participantId,
		embodimentId,
		emphasis: "active",
	});
	const introMutedAboveAuthors = [
		muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
		muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
		muted(INTRO_PARTICIPANT_GOAL, INTRO_EMBODIMENT_GOAL),
	];
	const introArrangement = (
		arrangementId: string,
		dispositions: Arrangement["dispositions"],
	): Arrangement => ({
		id: arrangementId,
		name: introArrangementBookmarkName(arrangementId, language),
		dispositions,
	});
	return [
		{
			order: 0,
			participants,
			embodiments,
			arrangement: introArrangement("title", [active(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_FULL)]),
			transition: { kind: "morph" },
		},
		{
			order: 1,
			arrangement: introArrangement("description", [
				muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
				active(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_FULL),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 2,
			arrangement: introArrangement("goal", [
				muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
				muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
				active(INTRO_PARTICIPANT_GOAL, INTRO_EMBODIMENT_GOAL),
				muted(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 3,
			arrangement: introArrangement("authors", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 4,
			arrangement: introArrangement("affiliations-1", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP1),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 5,
			arrangement: introArrangement("affiliations-2", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP2),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 6,
			arrangement: introArrangement("affiliations-3", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP3),
			]),
		},
	];
}

/** 🎬️ Intro thought module referencing {@link IntroSpec}. */
export function introThoughtFile(spec: IntroSpec): ThoughtFile {
	return { template: "intro", spec };
}

/** 🧩️ Expands a thought template module into slide modules under its folder path. */
export function expandThoughtFileToSlideModules(path: ParsedThoughtFilePath, thought: ThoughtFile): SlideFileModule[] {
	if (thought.template !== "intro") {
		throw new Error(`Unknown thought template "${thought.template}" at ${presentationThoughtFilePath(path.chapter, path.sequence, path.thought)}.`);
	}
	const language = introBookmarkLanguage(thought.spec.language);
	return introSlideFiles(thought.spec).map((file) => ({
		path: {
			chapter: path.chapter,
			sequence: path.sequence,
			thought: path.thought,
			slide:
				file.arrangement.name ??
				INTRO_ARRANGEMENT_BOOKMARK[language][file.arrangement.id] ??
				file.arrangement.id,
		},
		file,
	}));
}

/** 🎬️ Builds a seven-slide intro; each arrangement is that slide's target content for reveal.js auto-animate. */
export function intro(spec: IntroSpec): Presentation {
	const language = introBookmarkLanguage(spec.language);
	const chapterName = INTRO_CHAPTER_BOOKMARK[language];
	const sequenceName = INTRO_SEQUENCE_BOOKMARK[language];
	const thoughtName = INTRO_THOUGHT_BOOKMARK[language];
	const modules: SlideFileModule[] = introSlideFiles(spec).map((file, index) => ({
		path: {
			chapter: chapterName,
			sequence: sequenceName,
			thought: thoughtName,
			slide: file.arrangement.name ?? INTRO_ARRANGEMENT_BOOKMARK[language][file.arrangement.id] ?? `${index}`,
		},
		file,
	}));
	const chapter = assembleChapterFromSlideFiles(chapterName, new Map([[`${sequenceName}\0${thoughtName}`, modules]]));
	return {
		id: spec.id ?? "presentation",
		name: spec.name ?? spec.title.short,
		...(spec.language ? { language: spec.language } : {}),
		chapters: [chapter],
	};
}
//#endregion 🔖️Intro
