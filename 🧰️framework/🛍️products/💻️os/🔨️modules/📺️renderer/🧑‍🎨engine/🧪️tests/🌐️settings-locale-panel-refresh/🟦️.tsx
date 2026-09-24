/** 🌐️ Neutral full-refresh and mounted React oracle for locale-bearing view-model axes. */
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
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🌐️settings-locale-panel-refresh", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🌐️settings-locale-panel-refresh", "🔣️.json"), "utf8"));

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
  it("validates one settled full guest refresh for every locale-bearing mutation", () => {
    expect(new Ajv({ allErrors: true, strict: false }).validate(schema, fixture)).toBe(true);
    expect(fixture.requiresGuestRefresh).toBe(true);
    expect(fixture.refreshScope).toBe("full");
    expect(fixture.settleRequired).toBe(true);
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
