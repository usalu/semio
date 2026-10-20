import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import { useBeforeState } from "./🔶️before/🟦️.ts";
import { buildPart2 } from "./🔸️part2/🟦️.ts";
import { usePart3 } from "./🟠️part3/🟦️.ts";
import type { Vitest } from "./🟢️shared/🟦️.ts";

export async function registerSessionBrokerPortTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { resetWorkerLaw, clearHubSessionCapability, testSeams } = useBeforeState({ vitest, dependencies, source });
  buildPart2({ vitest, resetWorkerLaw, source, clearHubSessionCapability, testSeams });
  usePart3({ vitest, resetWorkerLaw, source });
}
