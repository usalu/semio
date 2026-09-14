type TestSource = { readonly directory: string; readonly url: string };

interface CommandIngressPagesFixture {
  readonly pageBytes: number;
  readonly commandMaximumBytes: number;
  readonly maximumPages: number;
  readonly rows: readonly { readonly name: string; readonly bytes: number; readonly pages: number }[];
  readonly refusals: readonly { readonly name: string; readonly bytes: number }[];
}

/** 🧱️ The same deterministic body the Rust twin cuts — every byte a function of its offset, so a
 * writer that duplicates, drops or reorders a page cannot concatenate back to it. */
function commandIngressBody(bytes: number): Uint8Array {
  const body = new Uint8Array(bytes);
  for (let offset = 0; offset < bytes; offset += 1) body[offset] = (offset % 251) + 1;
  return body;
}

/** 🔎️ First offset at which two bodies differ, or `-1` — a byte-for-byte comparison that stays cheap
 * for the multi-megabyte rows a deep-equality assertion would walk structurally. */
function firstDifference(left: Uint8Array, right: Uint8Array): number {
  if (left.length !== right.length) return Math.min(left.length, right.length);
  for (let offset = 0; offset < left.length; offset += 1) if (left[offset] !== right[offset]) return offset;
  return -1;
}

/** 📥️ LAW: the host writer cuts a command into exactly the pages the shared fixture declares, stamps
 * every page with its own index against one page count, and the pages concatenate back to the EXACT
 * bytes it was handed — the TypeScript twin of
 * `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`'s `every_command_ingress_row_reassembles_to_the_exact_bytes_it_was_cut_from`.
 *
 * 🧨️ The ceiling this pins is DERIVED: a command is an assembled host answer, so it is bound by
 * `GUEST_HOST_ANSWER_CEILING_BYTES` over the page extent and by nothing else. The constant it
 * replaces — a flat 64 pages — refused the procedural plugin's 272 089-char contributions pack with
 * `command ingress exceeds 64 pages`, which left the guest with no `brep` flow extension and every
 * example dead (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14 19:03). */
export async function registerCommandIngressPageTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: import("../../🟦️.ts").ShardClientTestDependenciesV1,
  testSource: TestSource,
): Promise<void> {
  const { describe, it, expect } = vitest;
  const { ACTOR_BYTE_PAGE_BYTES, createShardCommandIngressPages, SHARD_COMMAND_MAXIMUM_PAGES } = dependencies;
  const { readFileSync } = await import("node:fs");
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/📥️command-ingress-pages/🔣️.json", testSource.url), "utf8")) as CommandIngressPagesFixture;

  describe("command ingress pages", () => {
    it("derives its page authority from the guest linear-memory budget", () => {
      expect(ACTOR_BYTE_PAGE_BYTES).toBe(fixture.pageBytes);
      expect(SHARD_COMMAND_MAXIMUM_PAGES).toBe(fixture.maximumPages);
      expect(SHARD_COMMAND_MAXIMUM_PAGES * ACTOR_BYTE_PAGE_BYTES).toBe(fixture.commandMaximumBytes);
    });

    for (const row of fixture.rows) {
      it(`cuts ${row.name} into ${row.pages} pages that concatenate back byte for byte`, () => {
        const command = commandIngressBody(row.bytes);
        const pages = createShardCommandIngressPages({ owner: 5n, generation: 9n, commandIndex: 0, commandCount: 1, instance: 13, seq: 21n, command });
        expect(pages).toHaveLength(row.pages);
        const rejoined = new Uint8Array(row.bytes);
        let cut = 0;
        pages.forEach((page, index) => {
          expect(page.cursor).toMatchObject({ owner: 5n, generation: 9n, commandIndex: 0, commandCount: 1, instance: 13, seq: 21n, kind: command[0], pageIndex: index, pageCount: row.pages, itemCount: 0, metadata: 0 });
          expect(page.bytes.length).toBe(page.page.length);
          expect(index + 1 === pages.length || page.bytes.length === ACTOR_BYTE_PAGE_BYTES).toBe(true);
          rejoined.set(page.bytes, cut);
          cut += page.bytes.length;
        });
        expect(cut).toBe(row.bytes);
        expect(firstDifference(rejoined, command)).toBe(-1);
      });
    }

    for (const refusal of fixture.refusals) {
      it(`refuses ${refusal.name} rather than cutting it`, () => {
        expect(() => createShardCommandIngressPages({ owner: 5n, generation: 9n, commandIndex: 0, commandCount: 1, instance: 13, seq: 21n, command: commandIngressBody(refusal.bytes) })).toThrow();
      });
    }
  });
}
