/** ❓️ Entry of the architecture quiz website: loads the semio styles, then mounts the quiz renderer on `#root` for the
 * `architecture` catalog against the proctor a release build baked in (`VITE_PROCTOR_URL`); dev and tests keep the same
 * origin, which the dev server proxies to the dev proctor.
 * @see ./🔣️.json — the catalog this site offers
 * @see ./🎨️.css — the semio design system chain the quiz renders with
 * @see ./🚀️deploy/🔣️.json — the site and proctor hosts
 * @see ../../../🧰️framework/🛍️products/❓️quiz/README.md — the quiz domain model */
import "./🎨️.css";
import { mountQuiz } from "@semio-tech/quiz-react";
import logo from "../../../🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg?raw";

/** 🏛️ The catalog id the proctor serves for this site. */
export const ARCHITECTURE_QUIZ_TENANT = "architecture";

/** 🌐️ The proctor origin a release build baked in, or `""` (the page's own origin) wherever nothing was baked. */
export function bakedProctorOrigin(): string {
  try {
    return (import.meta.env.VITE_PROCTOR_URL as string | undefined) ?? "";
  } catch {
    return "";
  }
}

const root = document.getElementById("root");
if (root) mountQuiz(root, { proctor: bakedProctorOrigin(), tenant: ARCHITECTURE_QUIZ_TENANT, logo });
