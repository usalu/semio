/** ❓️ Entry of `quizze.architektur-und-technologie.de`: loads the semio palette, then mounts the quiz renderer on `#root`
 * against the proctor on the same origin, for the `architecture` catalog.
 * @see ./🔣️.json — the catalog this site offers
 * @see ./🎨️.css — the brand layer (palette variables and fonts)
 * @see ../../../🧰️framework/🛍️products/❓️quiz/README.md — the quiz domain model */
import "./🎨️.css";
import { mountQuiz } from "@semio-tech/quiz-react";

/** 🏛️ The catalog id the proctor serves for this site. */
export const ARCHITECTURE_QUIZ_TENANT = "architecture";

const root = document.getElementById("root");
if (root) mountQuiz(root, { proctor: "", tenant: ARCHITECTURE_QUIZ_TENANT });
