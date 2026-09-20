/** 🌐️ Neutral cursor and mounted React oracle for locale-owned shell panel refresh. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, render, screen } from "@semio-tech/ui-react/test";
import { Tree, uiI18n } from "@semio-tech/ui-react";
import Ajv from "ajv";
import { createElement as h } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { shellLabel } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧪️fixtures", "🌐️settings-locale-panel-refresh", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🌐️settings-locale-panel-refresh", "🔣️.json"), "utf8"));

type RefreshCursor = Readonly<{ generation: number; locale: string; pending: readonly string[] }>;

const advance = (cursor: RefreshCursor): RefreshCursor => ({ ...cursor, pending: cursor.pending.slice(fixture.maxPanelsPerStep) });

function LocalizedPanels() {
  return h(
    "div",
    null,
    ...fixture.labels.map((row: { key: Parameters<typeof shellLabel>[0] }) =>
      h(Tree, {
        key: row.key,
        sections: [{ id: row.key, label: shellLabel(row.key), defaultOpen: true, items: [] }],
      }),
    ),
  );
}

afterEach(async () => {
  cleanup();
  await uiI18n.changeLanguage(fixture.initialLocale);
});

describe("mounted shell locale refresh", () => {
  it("validates the shared schema and advances one exact mounted owner per step", () => {
    expect(new Ajv({ allErrors: true, strict: false }).validate(schema, fixture)).toBe(true);
    expect(fixture.requiresGuestRefresh).toBe(false);
    let cursor: RefreshCursor = { generation: 1, locale: fixture.nextLocale, pending: fixture.mountedSurfaceIds };
    for (let index = 0; index < fixture.mountedSurfaceIds.length; index += 1) {
      const before = cursor.pending;
      cursor = advance(cursor);
      expect(before.length - cursor.pending.length).toBe(1);
      expect(cursor.pending).toEqual(fixture.mountedSurfaceIds.slice(index + 1));
    }
    expect(cursor.pending).toEqual([]);
    expect(fixture.unmountedSurfaceIds.some((id: string) => fixture.mountedSurfaceIds.includes(id))).toBe(false);
  });

  it("re-renders the mounted shared Tree labels from the real React i18n authority", async () => {
    await uiI18n.changeLanguage(fixture.initialLocale);
    const mounted = render(h(LocalizedPanels));
    for (const row of fixture.labels) expect(screen.getByText(row.en)).toBeTruthy();
    await uiI18n.changeLanguage(fixture.nextLocale);
    mounted.rerender(h(LocalizedPanels));
    for (const row of fixture.labels) expect(screen.getByText(row.de)).toBeTruthy();
  });
});
