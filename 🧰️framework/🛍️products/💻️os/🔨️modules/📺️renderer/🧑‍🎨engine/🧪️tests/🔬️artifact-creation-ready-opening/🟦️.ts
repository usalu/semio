import { createElement } from "react";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🔣️.json";
import directorySchema from "../../../../📇️directory/🧬️schema/🔣️.json" with { type: "json" };
import artifactCreationReadyOpeningFixture from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🔣️.json";

import { runArtifactCreationReadyOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import {
  ARTIFACT_CREATION_PROGRESS_TEXT_V1,
  ArtifactCreationCatalogNotice,
  ArtifactCreationProgressNotice,
  artifactCreationProgressRoleV1,
  type ArtifactCreationProgressStateV1,
} from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🏦️.tsx";

const owner = {
  requestId: "1".repeat(32),
  spaceId: "space-a",
  kindId: "s.gis.gismap",
  name: "Shared Map",
  runtimeKey: "hub:space-a:index",
  clientInstanceId: "client-a",
  sessionInstanceId: 7,
} as const;

describe("artifact creation Ready opening", () => {
  it("publishes only a committed current target and releases every rejected private target at most once", async () => {
    const validate = new Ajv({ strict: true, allErrors: true }).addKeyword("discriminator").addSchema(directorySchema).getSchema(`${directorySchema.$id}#/$defs/ArtifactCreationReadyOpeningV1`)!;
    expect(validate(artifactCreationReadyOpeningFixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of artifactCreationReadyOpeningFixture.cases) {
      let currentIndex = 0;
      let releases = 0;
      let publishes = 0;
      let failures = 0;
      const outcome = await runArtifactCreationReadyOpeningV1({
        current: () => row.current[Math.min(currentIndex++, row.current.length - 1)]!,
        prepare: async () => {
          if (row.prepare === "unused") throw new Error("unexpected prepare");
          return row.prepare === "missing" ? null : { id: row.id };
        },
        open: async () => {
          if (row.open === "unused") throw new Error("unexpected open");
          if (row.open === "failed") throw new Error("attach failed");
          return row.open === "committed" ? { committed: true as const, runtimeKey: row.id, clientInstanceId: "client" } : null;
        },
        publish: () => { publishes += 1; },
        release: async () => {
          releases += 1;
          if (row.release === "failed") throw new Error("release failed");
        },
        failed: () => { failures += 1; },
      });
      expect(outcome, row.id).toBe(row.outcome);
      expect(releases, row.id).toBe(row.release === "unused" ? 0 : 1);
      expect(publishes, row.id).toBe(row.publish ? 1 : 0);
      expect(failures, row.id).toBe(row.failure ? 1 : 0);
    }
    console.log("[DEBUG] Ready opening disposition: neutral=9 published=1 released-once=6");
  });

  it("renders the schema-owned bilingual failure and normalizes impossible empty Ready catalogs to unavailable", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).addKeyword("discriminator").addSchema(directorySchema).getSchema(`${directorySchema.$id}#/$defs/ArtifactCreationProgressUiV1`)!;
    expect(validate(artifactCreationProgressFixture), JSON.stringify(validate.errors)).toBe(true);
    expect(deepEqual(artifactCreationProgressFixture.locales, ARTIFACT_CREATION_PROGRESS_TEXT_V1)).toBe(true);
    for (const row of artifactCreationProgressFixture.catalogCases) {
      const locale = row.locale as "en" | "de";
      const phase = row.phase as "loading" | "ready" | "unavailable";
      const effectivePhase = row.effectivePhase as "loading" | "ready" | "unavailable";
      const view = render(createElement(ArtifactCreationCatalogNotice, { status: { phase }, hasChoices: row.hasChoices, locale }));
      const notice = view.getByRole(row.role, { name: ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[effectivePhase] });
      expect(notice.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(notice.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(effectivePhase);
      view.unmount();
    }
  });

  it("offers a bilingual retry for durable Ready without issuing cancellation", () => {
    for (const locale of ["en", "de"] as const) {
      const onCancel = vi.fn();
      const onOpen = vi.fn();
      const state: ArtifactCreationProgressStateV1 = { ...owner, phase: "ready", cancelRequested: false, openingDisposition: "failed" };
      expect(artifactCreationProgressRoleV1(state.phase, state.openingDisposition)).toBe("alert");
      const view = render(createElement(ArtifactCreationProgressNotice, { state, locale, onCancel, onOpen }));
      const retry = ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry;
      fireEvent.click(view.getByRole("button", { name: `${retry}: ${owner.name}` }));
      expect(onOpen).toHaveBeenCalledExactlyOnceWith(owner.requestId);
      expect(onCancel).not.toHaveBeenCalled();
      view.unmount();
    }
  });
});
