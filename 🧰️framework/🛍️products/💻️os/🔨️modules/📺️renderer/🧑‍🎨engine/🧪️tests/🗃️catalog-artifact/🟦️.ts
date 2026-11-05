import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import { decodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { localArtifactAdmissionV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/📥️artifact/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

describe("kept artifact admission", () => {
  it("preserves recursive archive members and derives a bounded cross-platform folder from the real identity", async () => {
    for (const root of fixture.roots) {
      const admission = await localArtifactAdmissionV1(fixture, root, 10);
      expect("refusal" in admission).toBe(false);
      if ("refusal" in admission) continue;
      const hash = createHash("sha256").update(fixture.documentId).digest("hex");
      expect(admission.document.target).toBe(`${root}/os/local-documents/${hash}`);
      expect(admission.document.documentId).toBe(fixture.documentId);
      expect(admission.document.surface).toEqual(fixture.surface);
      expect(admission.document.target.length).toBeLessThanOrEqual(256);
      expect(decodeDocumentArchiveBytes(admission.archive)).toEqual(fixture.archive);
      expect(admission.binding).toEqual({ kind: "folder", dataClass: "persistedLocalOnly", path: admission.document.target });
    }
  });

  it("refuses missing storage, missing identity, invalid coordinates and oversized paths", async () => {
    expect(await localArtifactAdmissionV1(fixture, undefined, 10)).toEqual({ refusal: "no-data-folder" });
    for (const input of [{ ...fixture, documentId: "" }, { ...fixture, surface: { ...fixture.surface, appId: "" } }, { ...fixture, archive: { ...fixture.archive, parent_pack: [] } }]) {
      expect(await localArtifactAdmissionV1(input, "/device", 10)).toEqual({ refusal: "invalid-request" });
    }
    expect(await localArtifactAdmissionV1(fixture, `/${"x".repeat(220)}`, 10)).toEqual({ refusal: "invalid-request" });
  });
});
