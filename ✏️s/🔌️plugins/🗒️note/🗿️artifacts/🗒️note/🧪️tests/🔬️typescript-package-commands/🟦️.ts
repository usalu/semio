/** 🏃️ Runs the canonical architecture steps of the note artifact in order: the document contract oracle, then the strict type check. */
export async function runCanonicalArchitectureScript(steps: Readonly<{ contract: () => Promise<void>; check: () => void }>): Promise<void> {
  await steps.contract();
  steps.check();
}
