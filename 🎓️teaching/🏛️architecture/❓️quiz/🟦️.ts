/** ❓️ Entry of the architecture quiz website: loads the semio styles, then mounts the quiz renderer on `#root` for the
 * `architecture` catalog against the proctor a release build baked in (`VITE_PROCTOR_URL`); dev and tests keep the same
 * origin, which the dev server proxies to the dev proctor. The site hands the quiz its material — the catalog and its
 * quizzes — so the catalog shows at once and the page goes on working while the proctor cannot be reached: a learner
 * registers, plays and is scored on the device, and the proctor hears of it once it answers. The footer links the
 * legal pages the deployment names (`site.legal`: imprint, privacy policy); a page it does not name is not linked. The
 * pets are the architecture menagerie, a script chunk of its own that only a learner who wants pets downloads.
 * @see ./🔣️.json — the catalog this site offers
 * @see ./📚️catalog/🟦️.ts — the catalog and its quizzes as the material the quiz decides with
 * @see ../🐾️pets/🟦️.ts — the menagerie whose pets live on the cards
 * @see ./🎨️.css — the semio design system chain the quiz renders with
 * @see ./🚀️deploy/🔣️.json — the site and proctor hosts, and the site's legal pages
 * @see ../../../🧰️framework/🛍️products/❓️quiz/README.md — the quiz domain model */
import "./🎨️.css";
import { mountQuiz } from "@semio-tech/quiz-react";
import logo from "../../../🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg?raw";
import { ARCHITECTURE_QUIZ_MATERIAL } from "./📚️catalog/🟦️.ts";
import { site } from "./🚀️deploy/🔣️.json";

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
if (root) mountQuiz(root, { proctor: bakedProctorOrigin(), tenant: ARCHITECTURE_QUIZ_TENANT, material: ARCHITECTURE_QUIZ_MATERIAL, logo, legal: site.legal, pets: () => import("../🐾️pets/🟦️.ts").then((module) => module.ARCHITECTURE_MENAGERIE) });
