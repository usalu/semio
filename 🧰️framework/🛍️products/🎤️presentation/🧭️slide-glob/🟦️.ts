/** 🧭️ Presentation slide glob: loading a whole presentation from a glob of slide modules and thought files.
 *
 * @see ../📂️slide-files/🟦️.ts — the slide files
 * @see ../🎬️intro/🟦️.ts — the intro thought files that expand to slide modules
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { Presentation } from "../🎞️structure/🟦️.ts";
import { type SlideFile, type SlideFileModule, type ThoughtFile, type PresentationMeta, presentationThoughtFilePath, parsePresentationSlideFilePath, parsePresentationThoughtFilePath, assembleChapterFromSlideFiles } from "../📂️slide-files/🟦️.ts";
import { expandThoughtFileToSlideModules } from "../🎬️intro/🟦️.ts";

//#region 🔖️SlideFile
function isThoughtFile(module: SlideFile | ThoughtFile): module is ThoughtFile {
	return "template" in module;
}

/** 📽️ Assembles a deck from eager import.meta.glob slide and thought template modules. */
export function loadPresentationFromSlideGlob(
	meta: PresentationMeta,
	globModules: Readonly<Record<string, { readonly default: SlideFile | ThoughtFile }>>,
): Presentation {
	const byChapter = new Map<string, Map<string, readonly SlideFileModule[]>>();
	const thoughtTemplateKeys = new Set<string>();
	for (const [importPath, module] of Object.entries(globModules)) {
		const slidePath = parsePresentationSlideFilePath(importPath);
		const thoughtPath = slidePath ? null : parsePresentationThoughtFilePath(importPath);
		if (!slidePath && !thoughtPath) {
			continue;
		}
		const byThought =
			byChapter.get((slidePath ?? thoughtPath)!.chapter) ?? new Map<string, readonly SlideFileModule[]>();
		const keyPath = slidePath ?? thoughtPath!;
		const thoughtKey = `${keyPath.sequence}\0${keyPath.thought}`;
		const existing = byThought.get(thoughtKey) ?? [];
		if (thoughtPath) {
			if (existing.length > 0) {
				throw new Error(
					`Thought "${thoughtPath.thought}" is defined both as ${presentationThoughtFilePath(thoughtPath.chapter, thoughtPath.sequence, thoughtPath.thought)} and as slide files under that folder.`,
				);
			}
			if (!isThoughtFile(module.default)) {
				throw new Error(
					`Expected a thought template export from ${presentationThoughtFilePath(thoughtPath.chapter, thoughtPath.sequence, thoughtPath.thought)}.`,
				);
			}
			thoughtTemplateKeys.add(thoughtKey);
			byThought.set(thoughtKey, expandThoughtFileToSlideModules(thoughtPath, module.default));
		} else {
			if (thoughtTemplateKeys.has(thoughtKey)) {
				throw new Error(
					`Thought "${slidePath!.thought}" already uses template file ${presentationThoughtFilePath(slidePath!.chapter, slidePath!.sequence, slidePath!.thought)}.`,
				);
			}
			byThought.set(thoughtKey, [...existing, { path: slidePath!, file: module.default as SlideFile }]);
		}
		byChapter.set(keyPath.chapter, byThought);
	}
	return {
		id: meta.id,
		name: meta.name,
		...(meta.language ? { language: meta.language } : {}),
		...(meta.width ? { width: meta.width } : {}),
		...(meta.height ? { height: meta.height } : {}),
		chapters: [...byChapter.entries()]
			.sort(([left], [right]) => left.localeCompare(right, "de"))
			.map(([chapterName, thoughtModules]) => assembleChapterFromSlideFiles(chapterName, thoughtModules)),
	};
}
//#endregion 🔖️SlideFile
