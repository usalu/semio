// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import userEvent from "@testing-library/user-event";
import { describe, expect, test } from "vitest";
import { buildOsCommands, keyboardEventMatchesChord, controlOwnsKeyboardEvent } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import controlCases from "../../🧫️fixtures/⌨️control-key-ownership/🔣️.json";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "⌨️os-command-shortcuts", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "⌨️os-command-shortcuts", "🔣️.json"), "utf8"));

describe("focused control keyboard ownership", () => {
  test.each(controlCases)("$id", (row) => {
    const target = document.createElement(row.tag);
    for (const [key, value] of Object.entries(row.attributes ?? {})) target.setAttribute(key, value);
    const event = new KeyboardEvent("keydown", {...row, bubbles:true});
    let owned: boolean | undefined;
    target.addEventListener("keydown", (event) => owned = controlOwnsKeyboardEvent(event));
    target.dispatchEvent(event);
    expect(owned).toBe(row.owned);
  });

  test("preserves native Enter and Space activation through the user-event oracle", async () => {
    const button = document.createElement("button");
    const input = document.createElement("input");
    document.body.append(button, input);
    let activations = 0;
    let shortcuts = 0;
    const route = (event: KeyboardEvent) => {
      if (controlOwnsKeyboardEvent(event)) return;
      if (!["Enter", " ", "z"].includes(event.key)) return;
      event.preventDefault();
      shortcuts += 1;
    };
    document.addEventListener("keydown", route);
    button.addEventListener("click", () => activations += 1);
    try {
      const user = userEvent.setup();
      button.focus();
      await user.keyboard("{Enter} ");
      expect(activations).toBe(2);
      expect(shortcuts).toBe(0);
      await user.keyboard("{Control>}z{/Control}");
      expect(shortcuts).toBe(1);
      await user.tab();
      expect(document.activeElement).toBe(input);
      await user.keyboard("vector");
      expect(input.value).toBe("vector");
    } finally {
      document.removeEventListener("keydown", route);
      button.remove();
      input.remove();
    }
  });
});

describe("⌨️ OS command shortcut ownership", () => {
  test("the shared keymap satisfies its neutral schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("React's command producer is the platform-keymap oracle", () => {
    const command = buildOsCommands([], [], false).find(({ id }) => id === fixture.commandId);
    expect(command?.keybindings).toEqual(fixture.bindings);
    expect(command?.keybindings.some(({ chord }) => chord === fixture.retiredChord)).toBe(false);
  });

  test("React's matcher keeps the explicit Mac chord distinct from the Find mod chord", () => {
    const event = { key: fixture.collision.event.key, ctrlKey: fixture.collision.event.ctrl, metaKey: fixture.collision.event.meta, shiftKey: fixture.collision.event.shift, altKey: fixture.collision.event.alt };
    expect(keyboardEventMatchesChord(event, fixture.collision.matches)).toBe(true);
    expect(keyboardEventMatchesChord(event, fixture.collision.refuses)).toBe(false);
  });

  test("WGPU routes matched OS commands locally and leaves argument commands staged", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(shell).not.toContain('(\"os.toggleFullscreen\", \"mod+shift+f\")');
    expect(shell).not.toContain("let fullscreen_chord =");
    expect(shell).toContain("self.apply_os_command(&entry.definition.id, None).await?");
    expect(shell).toContain("self.overlay_state = OverlayState::Search");
    expect(shell).toContain("self.dispatch_command(invocation).await?");
  });
});
