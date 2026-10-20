import { buildBaseState } from "./🔶️base/🟦️.ts";
import { buildRawCloseState } from "./🔸️raw-close/🟦️.ts";
import { buildMutations } from "./🟠️mutations/🟦️.ts";

/** 🧪️ Cross-checks full-domain scope fixtures with system deep equality and semantic Ajv scope validation and guards the active retained admission/publication seam. */
export function toolJobLatestWinsSelfTests(): number {
  const { rawCloseBlock, rawSource, base, Ajv, source, body, ajv, fixture, obligations, integration, taskFixture, taskHostiles, rawFixture } = buildBaseState();
  const { mutateFunction, dispatchFixture, pipelineGuard, production, validDispatch, rawHostiles, mountedChecks, childCloseHostiles } = buildRawCloseState({ rawCloseBlock, rawSource, base, Ajv, source, body, ajv });
  buildMutations({ mutateFunction, dispatchFixture, pipelineGuard, production, validDispatch });
  return fixture.cases.length + 6 + obligations.length + integration.cases.length + taskFixture.observations.length + taskHostiles.length + rawFixture.cases.length + 4 + rawHostiles.length + 3 + mountedChecks.length * 2 + 2 + dispatchFixture.cases.length * 2 + childCloseHostiles.length + 3;
}
