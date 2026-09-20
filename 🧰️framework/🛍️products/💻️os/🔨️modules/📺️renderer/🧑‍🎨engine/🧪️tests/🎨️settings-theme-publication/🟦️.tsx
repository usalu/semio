/** 🎨️ Neutral and mounted React oracle for retained Theme preference publication. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { fireEvent, render } from "@testing-library/react";
import { createElement as h, useState } from "react";
import { describe, expect, test } from "vitest";
import { Button } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔘️Button/🟦️.tsx";
import { Input } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx";
import { Tree } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx";
import { parseUiTheme } from "../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🎨️settings-theme-publication", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🎨️settings-theme-publication", "🔣️.json"), "utf8"));

const customThemeId = (label: string) => `custom.${label.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "")}`;

function ThemePublicationHarness({ importText }: { readonly importText: string }) {
  const [themeId, setThemeId] = useState("semio");
  const [primary, setPrimary] = useState("#336699");
  const [dirty, setDirty] = useState(false);
  const [saveLabel, setSaveLabel] = useState("");
  const [customThemes, setCustomThemes] = useState<string[]>([]);
  const selectTheme = (value: string) => {
    setThemeId(value);
    setDirty(false);
  };
  const saveTheme = () => {
    const id = customThemeId(saveLabel);
    setCustomThemes((current) => [...new Set([...current, id])]);
    setThemeId(id);
    setDirty(false);
    setSaveLabel("");
  };
  const importTheme = (text: string) => {
    const parsed = parseUiTheme(JSON.parse(text));
    const id = customThemeId(parsed.label || parsed.id);
    setCustomThemes((current) => [...new Set([...current, id])]);
    setThemeId(id);
    setDirty(false);
  };
  const resetTheme = () => {
    setThemeId("semio");
    setDirty(false);
  };
  const deleteTheme = () => {
    setCustomThemes((current) => current.filter((id) => id !== themeId));
    setThemeId("semio");
    setDirty(false);
  };
  return h(Tree, {
    sections: [
      {
        id: "framework.settings.theme.select.section",
        label: "Theme",
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: "Theme",
            control: h(
              Select,
              { value: themeId, onValueChange: selectTheme },
              h(SelectTrigger, { id: "framework.settings.theme.select", "aria-label": "framework.settings.theme.select" }, h(SelectValue)),
              h(
                SelectContent,
                null,
                ["semio", "mono", ...customThemes].map((id) => h(SelectItem, { key: id, value: id }, id)),
              ),
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: "Name",
            control: h(Input, {
              id: "framework.settings.theme.saveLabel",
              "aria-label": "framework.settings.theme.saveLabel",
              value: saveLabel,
              onChange: (event) => setSaveLabel(event.currentTarget.value),
            }),
          },
          {
            id: "framework.settings.theme.save.action",
            label: "Save",
            control: h(Button, { id: "framework.settings.theme.save", "aria-label": "framework.settings.theme.save", text: "Save", disabled: saveLabel.trim() === "", onClick: saveTheme }),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: "Reset",
            control: h(Button, { id: "framework.settings.theme.reset", "aria-label": "framework.settings.theme.reset", text: "Reset", disabled: !dirty && themeId === "semio", onClick: resetTheme }),
          },
          {
            id: "framework.settings.theme.import.action",
            label: "Import",
            control: h(Button, { id: "framework.settings.theme.import", "aria-label": "framework.settings.theme.import", text: "Import", onClick: () => importTheme(importText) }),
          },
          ...(themeId.startsWith("custom.")
            ? [{ id: "framework.settings.theme.delete.action", label: "Delete", control: h(Button, { id: "framework.settings.theme.delete", "aria-label": "framework.settings.theme.delete", text: "Delete", onClick: deleteTheme }) }]
            : []),
        ],
      },
      {
        id: "framework.settings.theme.colors",
        label: "Colors",
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.colors.primary",
            label: "primary",
            control: h("input", {
              id: "framework.settings.theme.colors.primary",
              "aria-label": "framework.settings.theme.colors.primary",
              type: "color",
              value: primary,
              onChange: (event) => {
                setPrimary((event.currentTarget as HTMLInputElement).value);
                setDirty(true);
              },
            }),
          },
        ],
      },
    ],
  });
}

const observe = (view: ReturnType<typeof render>, controlId: string) => {
  if (controlId.endsWith(".select")) return `select:${view.getByRole("combobox", { name: controlId }).textContent?.trim()}`;
  const textbox = view.queryByRole("textbox", { name: controlId }) as HTMLInputElement | null;
  if (textbox) return `input:${textbox.value}`;
  const color = view.container.querySelector<HTMLInputElement>(`input[aria-label='${controlId}']`);
  if (color) return `input:${color.value}`;
  const button = view.queryByRole("button", { name: controlId }) as HTMLButtonElement | null;
  return button ? `button:${button.disabled ? "disabled" : "enabled"}` : "absent";
};

describe("🎨️ Theme retained preference publication", () => {
  test("the language-neutral workflow satisfies its strict schema and names actual React controls", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const source = readFileSync(join(engineRoot, "🧱️elements", "📌️ChromePanels", "🟦️.tsx"), "utf8");
    for (const step of fixture.workflow) {
      const reactAction = step.action === "resetThemeId" ? "resetTheme" : step.action === "deleteThemeId" ? "deleteTheme" : step.action === "applyImportedTheme" ? "importTheme" : step.action;
      expect(source, step.action).toContain(reactAction);
      for (const observation of step.observations) {
        const authoredId = observation.controlId.startsWith("framework.settings.theme.colors.") ? "framework.settings.theme.colors." : observation.controlId;
        expect(source, observation.controlId).toContain(authoredId);
      }
    }
  });

  test("mounted React selection, draft, save, reset and delete publish every consequence immediately", () => {
    expect(fixture.requiresGuestRefresh).toBe(false);
    const importText = fixture.workflow.find((step: { action: string }) => step.action === "applyImportedTheme").arguments.value as string;
    const view = render(h(ThemePublicationHarness, { importText }));
    for (const step of fixture.workflow) {
      const value = step.arguments.value as string | undefined;
      if (step.action === "setThemeId") {
        fireEvent.click(view.getByRole("combobox", { name: "framework.settings.theme.select" }));
        fireEvent.click(view.getByRole("option", { name: value }));
      } else if (step.action === "setThemeColor") {
        fireEvent.change(view.container.querySelector("input[type='color']")!, { target: { value } });
      } else if (step.action === "setThemeSaveLabel") {
        fireEvent.change(view.getByRole("textbox", { name: "framework.settings.theme.saveLabel" }), { target: { value } });
      } else if (step.action === "applyImportedTheme") {
        fireEvent.click(view.getByRole("button", { name: "framework.settings.theme.import" }));
      } else {
        const controlId = step.action === "saveTheme" ? "framework.settings.theme.save" : step.action === "resetThemeId" ? "framework.settings.theme.reset" : "framework.settings.theme.delete";
        fireEvent.click(view.getByRole("button", { name: controlId }));
      }
      for (const observation of step.observations) expect(observe(view, observation.controlId), `${step.action}:${observation.controlId}`).toBe(observation.expectedState);
    }
    view.unmount();
  });
});
