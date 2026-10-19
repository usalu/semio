import type { Challenge } from "@semio-tech/course";
import type { CourseLabelKey } from "../../🌐️i18n/🟦️.ts";

/** 🏷️ The name of every {@link Challenge}. */
export const CHALLENGE_LABELS: { readonly [C in Challenge]: CourseLabelKey } = { easy: "course.challenge.easy", medium: "course.challenge.medium", hard: "course.challenge.hard", expert: "course.challenge.expert" };

/** 📝️ The one line saying what every {@link Challenge} asks and its most points (placeholder `par`). */
export const CHALLENGE_HINTS: { readonly [C in Challenge]: CourseLabelKey } = { easy: "course.challenge.easyHint", medium: "course.challenge.mediumHint", hard: "course.challenge.hardHint", expert: "course.challenge.expertHint" };

