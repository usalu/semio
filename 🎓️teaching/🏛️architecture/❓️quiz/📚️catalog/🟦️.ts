/** 📚️ The architecture catalog as the site's material: the catalog and the quizzes it names, with their solutions.
 *
 * Every document is imported statically, so the material reaches a browser as part of the site's script and nothing is
 * fetched at run time; the quizzes are listed in the order of the catalog's paths. With it the quiz client shows the
 * catalog at once and decides by itself while the proctor is away.
 * @see ../🔣️.json — the catalog: introduction, quiz paths, badges
 * @see ../🟦️.ts — the entry that hands this material to the quiz
 * @see ../🧪️tests/🧪️catalog/🟦️.ts — holds this module to the documents on disk
 * @see ../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🫡️deputy/🟦️.ts — what decides with it */
import type { Catalog, Quiz } from "@semio-tech/quiz";
import catalog from "../🔣️.json";
import physics from "../../⚡️energy/🧲️physics/❓️quiz/🔣️.json";
import heating from "../../⚡️energy/🔥️heating/❓️quiz/🔣️.json";
import cooling from "../../⚡️energy/❄️cooling/❓️quiz/🔣️.json";
import demand from "../../⚡️energy/📊️demand/❓️quiz/🔣️.json";

/** 📚️ The catalog of the architecture and technology quizzes and its quizzes in catalog order. */
export const ARCHITECTURE_QUIZ_MATERIAL: { readonly catalog: Catalog; readonly quizzes: readonly Quiz[] } = { catalog: catalog as unknown as Catalog, quizzes: [physics, heating, cooling, demand] as unknown as readonly Quiz[] };
