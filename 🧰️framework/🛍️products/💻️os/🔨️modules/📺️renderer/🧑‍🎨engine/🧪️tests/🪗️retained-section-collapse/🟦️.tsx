/** 📂️ Neutral disclosure fixture validated against React's real Collapsible owner. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@semio-tech/ui-react";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { screen as testingScreen } from "@testing-library/react";
import Ajv from "ajv";
import * as React from "react";
import { afterEach, describe, expect, test } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const uiRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui");
const fixture = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🪗️retained-section-collapse/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(uiRoot, "🧬️schema/🪗️retained-section-collapse/🔣️.json"), "utf8"));

afterEach(cleanup);

describe("📂️ retained Section disclosure contract", () => {
  test("the language-neutral fixture satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("React removes closed content from layout, interaction, and accessibility until activation", () => {
    const { container } = render(
      <Collapsible defaultOpen={fixture.section.defaultOpen}>
        <CollapsibleTrigger>{fixture.section.label}</CollapsibleTrigger>
        <CollapsibleContent data-testid="section-content">
          <button id={fixture.section.childId}>{fixture.section.childLabel}</button>
        </CollapsibleContent>
      </Collapsible>,
    );

    const trigger = screen.getByRole("button", { name: fixture.section.label });
    const content = container.querySelector('[data-testid="section-content"]') as HTMLDivElement;
    expect(trigger.getAttribute("aria-expanded")).toBe(String(fixture.states.closed.expanded));
    expect(content.hidden).toBe(true);
    expect(testingScreen.queryByRole("button", { name: fixture.section.childLabel })).toBeNull();
    expect(testingScreen.getByRole("button", { name: fixture.section.childLabel, hidden: true })).toBeTruthy();

    fireEvent.click(trigger);

    expect(trigger.getAttribute("aria-expanded")).toBe(String(fixture.states.open.expanded));
    expect(content.hidden).toBe(false);
    expect(testingScreen.getByRole("button", { name: fixture.section.childLabel })).toBeTruthy();
  });
});
