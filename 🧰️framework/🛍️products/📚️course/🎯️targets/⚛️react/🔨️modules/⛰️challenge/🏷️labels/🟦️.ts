import type { Challenge } from "@semio-tech/course";
import type { IconName } from "@semio-tech/ui-react/chrome";
import type { CourseLabelKey } from "../../🌐️i18n/🟦️.ts";

/** 🏷️ The name of every {@link Challenge}. */
export const CHALLENGE_LABELS: { readonly [C in Challenge]: CourseLabelKey } = { easy: "course.challenge.easy", medium: "course.challenge.medium", hard: "course.challenge.hard", expert: "course.challenge.expert" };

/** 🔣️ The icon of every {@link Challenge}, for what sets it apart: a hint, the values in sight, the values hidden, the clock. */
export const CHALLENGE_ICONS: { readonly [C in Challenge]: IconName } = { easy: "info", medium: "check", hard: "eye-off", expert: "clock" };

/** 📝️ The one line saying what every {@link Challenge} asks and its most points (placeholder `par`). */
export const CHALLENGE_HINTS: { readonly [C in Challenge]: CourseLabelKey } = { easy: "course.challenge.easyHint", medium: "course.challenge.mediumHint", hard: "course.challenge.hardHint", expert: "course.challenge.expertHint" };

