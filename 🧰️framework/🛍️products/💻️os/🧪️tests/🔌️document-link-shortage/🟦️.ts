/** 🔌️ LAW: the TS twin of the kernel's `DocumentLink` walks the SAME language-agnostic fixture as the Rust runner
 * (`🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-link-shortage/🦀️.rs`), and the fixture is admitted by its schema
 * through Ajv (third-party) — so the React worker, the native actor and the browser actor share one reconnect,
 * expiry and revocation rule (ticket 26/09/23 audit P2-2). The terminal texts equal React's execution-target
 * status texts, which the worker already shows. */
export async function registerDocumentLinkShortageTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  twin: {
    readonly DOCUMENT_LINK_ACCESS_REFUSED_STATUSES: ReadonlySet<number>;
    readonly DOCUMENT_LINK_SHORTAGE_POLICY: Readonly<{ reconnectMinMs: number; reconnectMaxMs: number; shortageBoundMs: number }>;
    readonly DOCUMENT_LINK_STATUS_TEXT: Readonly<Record<string, Readonly<Record<"en" | "de", string>>>>;
    readonly documentLinkAdmitsLocalEdits: (link: never) => boolean;
    readonly documentLinkExpiresAtMs: (link: never) => number | undefined;
    readonly documentLinkOpened: (nowMs: number) => unknown;
    readonly documentLinkStatus: (link: never) => string;
    readonly documentLinkTransition: (link: never, event: never) => unknown;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🔨️modules/🏪️store/🧫️fixtures/document-link-shortage-v1/🔣️.json");
  const { default: schema } = await import("../../🔨️modules/🏪️store/🔄️sync/🧬️schema/document-link-shortage/🔣️.json");
  const { DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1 } = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");

  describe("DocumentLinkShortage", () => {
    it("owns a fixture its schema admits and the kernel's policy", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect({ ...twin.DOCUMENT_LINK_SHORTAGE_POLICY }).toEqual(fixture.policy);
      expect([...twin.DOCUMENT_LINK_ACCESS_REFUSED_STATUSES]).toEqual(fixture.accessRefusedStatuses);
    });

    it("walks every fixture vector through the same link states", () => {
      let steps = 0;
      for (const vector of fixture.vectors) {
        let link = twin.documentLinkOpened(vector.openedAtMs);
        for (const [index, step] of vector.steps.entries()) {
          link = twin.documentLinkTransition(link as never, step.event as never);
          const observed: Record<string, unknown> = { state: link, status: twin.documentLinkStatus(link as never), admitsLocalEdits: twin.documentLinkAdmitsLocalEdits(link as never) };
          const expiresAtMs = twin.documentLinkExpiresAtMs(link as never);
          if (expiresAtMs !== undefined) observed.expiresAtMs = expiresAtMs;
          expect(observed, `${vector.id} step ${index}`).toEqual(step.expect);
          steps += 1;
        }
      }
      expect(steps).toBeGreaterThanOrEqual(20);
    });

    it("speaks the fixture texts in both tongues, and its terminal texts are the ones the React worker shows", () => {
      expect(twin.DOCUMENT_LINK_STATUS_TEXT).toEqual(fixture.texts);
      expect(twin.DOCUMENT_LINK_STATUS_TEXT["link-expired"]).toEqual(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1["link-expired"]);
      expect(twin.DOCUMENT_LINK_STATUS_TEXT["access-revoked"]).toEqual(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1["access-revoked"]);
    });
  });
}
