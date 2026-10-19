/** 🛠️ The resident page fixture shared by the resident page and reader parts. */
import { loadRetainedResidentPageFixture } from "@semio-tech/framework/ui/testing/fixtures";
import type { ResidentTestKit } from "../🧰️resident-test-kit/🟦️.tsx";

/** 🔧 Creates the resident page fixture shared by the resident parts. */
export function createResidentPageHelpers(kit: ResidentTestKit) {
  const { residentBuilderFixture } = kit;
  async function residentPageFixture() {
    const base = await residentBuilderFixture(); const pageFixture = await loadRetainedResidentPageFixture(); let page: import("@semio-tech/framework/ui/contract/retained/resident").OwnedUiResidentPage | null = null;
    for (const bytes of [...pageFixture.storageOwnerGrants, ...pageFixture.admissionGrants]) page = base.payload.beginPage(base.builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing original page"); return { ...base, page, pageFixture };
  }
  return { residentPageFixture };
}
export type ResidentPageHelpers = ReturnType<typeof createResidentPageHelpers>;
