/** 🎪️ Pet companions: the pets on stage and the client around them. */
import { describe } from "vitest";
import { registerPetsStageTests } from "./🔶️pets-stage/🟦️.tsx";
import { registerClientTests } from "./🔸️client/🟦️.tsx";

describe("🐾️ pet companions", () => {
  registerPetsStageTests();
  registerClientTests();
});
