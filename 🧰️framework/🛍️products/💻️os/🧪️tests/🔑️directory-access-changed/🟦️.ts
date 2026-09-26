/** 🔑️ LAW (a reader whose own access to a space moves re-reads its directory from the origin): the shared hub fixture
 * `🌎️hub/🧫️fixtures/🔑️directory-access-changed-v1` — walked by the hub law over `directory_access_change_for_reader` — is
 * admitted by the directory stream schema through Ajv (third-party), an independent reference of the hub rule derives every
 * case's owed frame (a grant or redemption naming the reader for a space its socket never delivered the creation of, every
 * removal naming it), and the worker's projection wake answers every `wakes` row (ticket 26/09/23 C11: a human added to a
 * space created before their cursor never saw its `space.created`, so Home listed it only after a reload). */
import { directoryStreamWakeV1 } from "../../🔨️modules/📇️directory/🟦️.ts";
import type { DirectoryStreamMessage } from "../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";

export async function registerDirectoryAccessChangedTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../../../../🌎️hub/🧫️fixtures/🔑️directory-access-changed-v1/🔣️.json");
  const { default: schema } = await import("../../🔨️modules/📇️directory/🧬️schema/🔣️.json");
  type Case = { id: string; createdOnSocket: string[]; message: DirectoryStreamMessage; frame: DirectoryStreamMessage | null };
  const cases = fixture.cases as unknown as Case[];
  const wakes = fixture.wakes as unknown as { id: string; message: DirectoryStreamMessage; wake: "origin" | "next-page" | "none" }[];

  describe("DirectoryAccessChanged", () => {
    it("owns messages and frames the directory stream schema admits", async () => {
      const { default: Ajv } = await import("ajv");
      const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(schema);
      const validate = ajv.getSchema(`${(schema as { $id: string }).$id}#/$defs/DirectoryStreamMessage`)!;
      for (const message of [...cases.flatMap((row) => (row.frame === null ? [row.message] : [row.message, row.frame])), ...wakes.map((row) => row.message)]) expect(validate(message), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ kind: "access-changed", spaceId: "sp", change: "moved" })).toBe(false);
      expect(validate({ kind: "access-changed", spaceId: "", change: "granted" })).toBe(false);
    });

    it("derives every owed frame from the reader rule", () => {
      for (const row of cases) {
        const body = row.message.kind === "event" ? row.message.event.body : null;
        const names = body !== null && "userId" in body && body.userId === fixture.readerUserId ? body : null;
        const owed: DirectoryStreamMessage | null =
          names === null
            ? null
            : names.kind === "member.removed"
              ? { kind: "access-changed", spaceId: names.spaceId, change: "revoked" }
              : (names.kind === "member.upserted" || names.kind === "invite.redeemed") && !row.createdOnSocket.includes(names.spaceId)
                ? { kind: "access-changed", spaceId: names.spaceId, change: "granted" }
                : null;
        expect(owed, row.id).toEqual(row.frame);
      }
    });

    it("re-reads Home from the origin on the reader's access change, pages on events, ignores telemetry", () => {
      for (const row of wakes) expect(directoryStreamWakeV1(row.message), row.id).toBe(row.wake);
    });
  });
}
