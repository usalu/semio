/** 🧰️ Shared helpers of the GLB encoder tests: the vectors of `../../🧫️fixtures`, discovered on disk, and the inert operation. */
import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { directoryNear, fixtureNames, readFixture } from "../../../../🧪️tests/🧰️support/🧫️reader/🟦️.ts";
import type { GlbOptions } from "../../🟦️.ts";

const FIXTURES = directoryNear(import.meta.url, "../../🧫️fixtures");

/** 🧫️ Every GLB vector (a document and its options, plus the expected digests), sorted by folder. */
export const vectors = <T extends { document: ModelDocument; options: GlbOptions }>(): T[] => fixtureNames(FIXTURES).map((name) => readFixture<T>(FIXTURES, name));

/** ⏱️ An operation that is never cancelled and ignores progress. */
export const quiet = { report: () => undefined, cancelled: () => false };
