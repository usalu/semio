/** 📚️ The architecture course as the site's material: the course and the quizzes it names, with their solutions.
 *
 * The material is a {@link CourseMaterialSource}: the documents are imported on demand, one chunk each (the course and
 * every quiz), all started at once and never part of the entry, so the first paint carries none of the solutions. The
 * client shows at once and the device decides by itself from the moment the material has arrived; the quizzes are listed
 * in the order of the course's paths. The release document hints the chunks at low priority, so they are in the
 * browser's cache by the time the client asks.
 * @see ../🔣️.json — the course: introduction, quiz paths, badges
 * @see ../🟦️.ts — the entry that hands this material to the quiz
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the Vite configuration that hints the chunks
 * @see ../🧪️tests/🧪️course/🟦️.ts — holds this module to the documents on disk
 * @see ../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🫡️deputy/🟦️.ts — what decides with it */
import type { Course, Quiz } from "@semio-tech/course";
import type { CourseMaterialSource } from "@semio-tech/course-react";

/** 📥️ Fetches the course and every quiz it names in parallel, and settles with them in course order. */
export const loadArchitectureCourseMaterial: CourseMaterialSource = async () => {
  const [course, ...quizzes] = await Promise.all([
    import("../🔣️.json"),
    import("../../⚡️energy/🧲️physics/❓️quiz/🔣️.json"),
    import("../../⚡️energy/🌬️ventilation/❓️quiz/🔣️.json"),
    import("../../⚡️energy/🔥️heating/❓️quiz/🔣️.json"),
    import("../../⚡️energy/❄️cooling/❓️quiz/🔣️.json"),
    import("../../⚡️energy/📊️demand/❓️quiz/🔣️.json"),
    import("../../📐️parametric/❓️quiz/🔣️.json"),
    import("../../🦗️grasshopper/❓️quiz/🔣️.json"),
    import("../../📊️excel/❓️quiz/🔣️.json"),
    import("../../🏗️bim/❓️quiz/🔣️.json"),
    import("../../🎯️optimization/❓️quiz/🔣️.json"),
  ]);
  return { course: course.default as unknown as Course, quizzes: quizzes.map((quiz) => quiz.default) as unknown as readonly Quiz[] };
};
