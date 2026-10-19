/** 📚️ The architecture course as the site's material: the course and the quizzes it names, with their solutions.
 *
 * Every document is imported statically, so the material reaches a browser as part of the site's script and nothing is
 * fetched at run time; the quizzes are listed in the order of the course's paths. With it the quiz client shows the
 * course at once and decides by itself while the proctor is away.
 * @see ../🔣️.json — the course: introduction, quiz paths, badges
 * @see ../🟦️.ts — the entry that hands this material to the quiz
 * @see ../🧪️tests/🧪️course/🟦️.ts — holds this module to the documents on disk
 * @see ../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🫡️deputy/🟦️.ts — what decides with it */
import type { Course, Quiz } from "@semio-tech/course";
import course from "../🔣️.json";
import physics from "../../⚡️energy/🧲️physics/❓️quiz/🔣️.json";
import ventilation from "../../⚡️energy/🌬️ventilation/❓️quiz/🔣️.json";
import heating from "../../⚡️energy/🔥️heating/❓️quiz/🔣️.json";
import cooling from "../../⚡️energy/❄️cooling/❓️quiz/🔣️.json";
import demand from "../../⚡️energy/📊️demand/❓️quiz/🔣️.json";

/** 📚️ The course of the architecture and technology quizzes and its quizzes in course order. */
export const ARCHITECTURE_COURSE_MATERIAL: { readonly course: Course; readonly quizzes: readonly Quiz[] } = { course: course as unknown as Course, quizzes: [physics, ventilation, heating, cooling, demand] as unknown as readonly Quiz[] };
