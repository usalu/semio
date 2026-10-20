/** 🐾️ Pet companions: the learner's choice between no pets, motionless, calm and lively ones is lively until they
 * choose otherwise, kept on the device and offered in both languages, and a switch on the footer of every screen hides
 * the pets and brings them back as lively as they were; a device that asks for reduced motion does not hold them still,
 * and what a learner chose in the preferences or with the switch is stored as a fact of its own;
 * every step has its scene — the quiz on screen or home; the species and the layer are fetched only when pets are
 * wanted, a fetch that fails or a layer that throws leaves the quiz untouched and silent; the layer stands on the
 * cards of the screen, keeps clear of what a learner drags and glances at the other learners' cursors; and the
 * preferences name the pets the layer says are on stage right now. The learner lets the pets answer clicks and be
 * picked up and lets them play with the page — both yes until the learner says no, both off while the pets are off or
 * still —; a press on what of the quiz acts on one is never a pet's; the quiz marks the topics pets may play with
 * (the tasks of a quiz's page, the items of a run that are no table rows, the true order of a sorting's results) and
 * gives the cards of the overview their quiz as a topic they are never lifted for; and the settings play with every
 * pet on stage by the keyboard, saying politely what was asked. The whole client is driven with the sample menagerie
 * of the pets product and the real layer of `@semio-tech/pets-react`; the screens are those of the shared sheet
 * `icons-1`.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx
 * @see ../../../🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json — the sample menagerie
 * @see ../../🧫️fixtures/🃏️sheet-assembly/🔣️.json — `icons-1`
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html
 * @see https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html
 */
import { describe } from "vitest";
import { registerLearnerSChoiceTests } from "./🔶️learner-s-choice/🟦️.tsx";
import { registerLivelinessTests } from "./🔸️liveliness/🟦️.tsx";
import { registerLayerTests } from "./🟠️layer/🟦️.tsx";

describe("🐾️ pet companions", () => {
  registerLearnerSChoiceTests();
  registerLivelinessTests();
  registerLayerTests();
});
