// #region 🧲️Header
/** @emoji 🪞️ The text editor host's echo pack over `✏️TextEditor/🧫️fixtures/🪞️echo-pack/🔣️.json`: an echo of the editor's own
 * keystroke must sync a byte-identical pack (the host skips it, so the canvas paints once per key, not twice — ticket
 * 26/09/23 F3 measured writer 2.08 paints per key because the buffer's lane ref rode every echo), while an echo that changes any
 * other field, an undeclared lane, or another author's text must reach the session. Third-party oracle: Node's
 * `util.isDeepStrictEqual` judges the decoded pack against the fixture's expected synced scene. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { isDeepStrictEqual } from "node:util";
import { describe, expect, it } from "vitest";
import { decodePackValue } from "@semio-tech/framework-os";
import type { TextEditorScene } from "@semio-tech/framework";
import { sameScenePackV1, textEditorSyncPackV1 } from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🪞️echo-pack/🔣️.json";
// #endregion 🔌️Adapters

// #region 🪞️EchoPackLaws
type EchoPackCase = { readonly id: string; readonly external: boolean; readonly previous: TextEditorScene; readonly echo: TextEditorScene; readonly synced: Record<string, unknown>; readonly samePack: boolean };

describe("🪞️ the text editor syncs an echo of its own state as a byte-identical pack (echo-pack fixture)", () => {
  it("names the fixture schema", () => expect(fixture.schema).toBe("semio.renderer.text-editor.echo-pack/v1"));
  for (const row of fixture.cases as readonly EchoPackCase[]) {
    it(row.id, () => {
      const previous = textEditorSyncPackV1(row.previous, row.external);
      const echo = textEditorSyncPackV1(row.echo, row.external);
      expect(isDeepStrictEqual(decodePackValue(echo), row.synced), `${row.id}: synced scene`).toBe(true);
      expect(sameScenePackV1(previous, echo), `${row.id}: same pack`).toBe(row.samePack);
    });
  }
});
// #endregion 🪞️EchoPackLaws
