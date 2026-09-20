import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";
import { buildOsCommands, keyboardEventMatchesChord } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "⌨️os-command-shortcuts", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "⌨️os-command-shortcuts", "🔣️.json"), "utf8"));

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
