import { describe } from "vitest";
import { registerAssignsStableRanksDirectionTests } from "./🔶️assigns-stable-ranks-direction/🟦️.tsx";
import { registerAdmitsPublishesOnlyThroughTests } from "./🔸️admits-publishes-only-through/🟦️.tsx";
import { registerRetainsCommittedProxyThroughTests } from "./🟠️retains-committed-proxy-through/🟦️.tsx";
import { registerRetiresDisplayedSourceFallbackTests } from "./🟢️retires-displayed-source-fallback/🟦️.tsx";

describe("owned Diagram directed layout", () => {
  registerAssignsStableRanksDirectionTests();
  registerAdmitsPublishesOnlyThroughTests();
  registerRetainsCommittedProxyThroughTests();
  registerRetiresDisplayedSourceFallbackTests();
});
