/** 🧾️ One test-declared negative expectation: the stage that rejects it and the reason code. */
export type ObservationExpectation = { readonly stage: "contract" | "domain" | "bounds"; readonly result: "accepted" | "rejected"; readonly code: string };

/** ✅️ Enforces one test-declared expectation against the observed contract-stage outcome. */
export function assertObservationExpectation(name: string, expectation: ObservationExpectation, admitted: boolean): void {
  if (expectation.result === "rejected" && expectation.stage === "contract" && admitted) throw new Error(`${name}: contract stage admitted ${expectation.code}`);
  if (expectation.result === "accepted" && !admitted) throw new Error(`${name}: contract stage rejected ${expectation.code}`);
}
