/** 🌐️ Every language of the client has arrived before a suite starts: the client fetches its texts as chunks of their own
 * and waits for them, which the suites that drive it in either language take as given — the one that holds the chunks to
 * their own laws starts from a fresh module graph.
 * @see ../🗣️translation-completeness/🟦️.tsx — each language arrives on its own
 * @see ../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts — the loader */
import { loadCourseLocales } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

await loadCourseLocales();
