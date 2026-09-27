import Ajv from "ajv";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DEFAULT_UI_DRIVER, UiDriverProvider, detectShellLocale, uiI18n } from "@semio-tech/ui-react";
import { isShellLocale } from "@semio-tech/framework";
import { BlockListHost } from "../../🧱️elements/🧩️BlockListHost/🟦️.tsx";
import schema from "../../../../../../../🔨️modules/🖱️ui/🧬️schema/🧩️block-list-presentation/🔣️.json" with { type: "json" };
import fixture from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧩️block-list-presentation/🔣️.json" with { type: "json" };

afterEach(() => cleanup());

describe("BlockList presentation parity", () => {
  it("renders only the shared localized chrome and exposes palette keyboard activation", async () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const previousLocale = detectShellLocale(uiI18n.resolvedLanguage || uiI18n.language);
    try {
      for (const locale of fixture.locales) {
        if (!isShellLocale(locale.locale)) throw new Error(`block-list fixture names an unsupported locale ${locale.locale}`);
        await uiI18n.changeLanguage(locale.locale);
        const onAction = vi.fn();
        const node = {
          surfaceId: `block-list-${locale.locale}`,
          controllerId: "controller.block-list",
          blockList: { stepsJson: JSON.stringify(fixture.steps), paletteJson: JSON.stringify(fixture.palette) },
        };
        const view = render(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <BlockListHost node={node as never} onAction={onAction} />
          </UiDriverProvider>,
        );
        expect(view.container.textContent).toContain(locale.steps);
        expect(screen.getByRole("button", { name: locale.addStep })).toBeTruthy();
        const palette = screen.getByRole("button", { name: fixture.palette[0].label });
        expect(palette.getAttribute("data-block-kind")).toBe(fixture.palette[0].blockKind);
        expect(view.container.querySelector(".semio-palette")?.firstElementChild?.contains(palette)).toBe(true);
        for (const absent of fixture.absentVisualText) expect(view.container.textContent).not.toContain(absent);

        fireEvent.keyDown(palette, { key: "Enter" });
        fireEvent.keyDown(palette, { key: " " });
        expect(onAction).toHaveBeenNthCalledWith(1, { controllerId: node.controllerId, action: "addBlock", args: fixture.actions[1].args });
        expect(onAction).toHaveBeenNthCalledWith(2, { controllerId: node.controllerId, action: "addBlock", args: fixture.actions[1].args });
        fireEvent.click(screen.getByRole("button", { name: locale.addStep }));
        expect(onAction).toHaveBeenNthCalledWith(3, { controllerId: node.controllerId, action: "addStep", args: fixture.actions[0].args });
        view.unmount();
      }
    } finally {
      await uiI18n.changeLanguage(previousLocale);
    }
  });
});
