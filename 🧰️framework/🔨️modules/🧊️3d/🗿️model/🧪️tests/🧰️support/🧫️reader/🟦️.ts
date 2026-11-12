/** 🧫️ The one reader of the language-agnostic fixtures of every kernel suite: `<directory>/<name>/🔣️.json` parsed as JSON. No third-party import, so any suite may use it; paths are plain strings because a DOM test environment replaces `URL` with a class Node's file APIs refuse. */
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 📍️ The directory `relative` to the module whose `import.meta.url` is `metaUrl`. */
export const directoryNear = (metaUrl: string, relative: string): string => resolve(dirname(fileURLToPath(metaUrl)), relative);

/** 📖️ Parses the fixture `name` of the fixtures `directory`. */
export const readFixture = <T = any>(directory: string, name: string): T => JSON.parse(readFileSync(join(directory, name, "🔣️.json"), "utf8"));

/** 🗂️ The fixture folder names of `directory`, sorted. */
export const fixtureNames = (directory: string): string[] => readdirSync(directory).sort();
