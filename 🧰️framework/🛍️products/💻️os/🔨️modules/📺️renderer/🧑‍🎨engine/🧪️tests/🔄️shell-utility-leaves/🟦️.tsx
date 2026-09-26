import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildFrameworkSyncUtilities } from "@semio-tech/framework-os";
import { cleanup, render } from "@semio-tech/ui-react/test";
import Ajv from "ajv";
import { afterEach, describe, expect, test } from "vitest";
import { TaskManagerWindow, type TaskManagerTaskV1 } from "../../🧱️elements/🧵️TaskManager/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🔄️shell-utility-leaves", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🔄️shell-utility-leaves", "🔣️.json"), "utf8"));

const independentSelectedChoice = (uri: string | null): string | null => {
  if (uri === null) return null;
  if (uri.startsWith("file://")) return "framework.sync.file";
  if (uri.startsWith("folder://")) return "framework.sync.folder";
  if (uri.startsWith("remote://")) return "framework.sync.remote";
  return null;
};

afterEach(cleanup);

describe("🔄️ target-neutral Shell utility leaves", () => {
  test("the shared fixture satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("the independent URI oracle preserves all three choices while detached and selects only the attached kind", () => {
    expect(independentSelectedChoice(null)).toBeNull();
    expect(independentSelectedChoice(fixture.sync.attached.uri)).toBe(fixture.sync.attached.selectedChoiceId);
    expect(fixture.sync.choiceIds).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
  });

  test("the React producer is the third-party Sync utility oracle", () => {
    const detached = buildFrameworkSyncUtilities(null);
    expect(detached.map(({ id }) => id)).toEqual(fixture.sync.choiceIds);
    expect(detached.every(({ pressed }) => pressed === false)).toBe(true);
    const attached = buildFrameworkSyncUtilities(fixture.sync.attached.uri);
    expect(attached.find(({ pressed }) => pressed)?.id).toBe(fixture.sync.attached.selectedChoiceId);
  });

  test("the React Task Manager oracle distinguishes an unattached runtime", () => {
    const noTasks: readonly TaskManagerTaskV1[] = [];
    const { container } = render(<TaskManagerWindow sources={{ registry: () => null, tasks: () => noTasks, subscribe: () => () => undefined, cancel: () => undefined, suspend: () => undefined, resume: () => undefined }} />);
    const status = container.querySelector("[data-semio-task-manager-empty]");
    expect(status?.getAttribute("role")).toBe("status");
    expect(status?.getAttribute("data-semio-task-manager-empty")).toBe(fixture.taskManager.state);
    expect(status?.textContent).toBe(fixture.taskManager.message.en);
  });

  test("the WGPU Shell owns both retained leaves and the Task Manager open command", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(shell).toContain('const FRAMEWORK_TASK_MANAGER_PANEL_ID: &str = "os.task-manager"');
    expect(shell).toContain("fn build_sync_attach_ui(&self) -> UiNode");
    expect(shell).toContain("fn build_task_manager_ui(&self) -> UiNode");
    expect(shell).toContain('"os.openTaskManager" =>');
    expect(shell).toContain("leaves.push(FRAMEWORK_SYNC_PANEL_TAB_ID.to_string())");
    expect(shell).toContain("leaves.push(FRAMEWORK_TASK_MANAGER_PANEL_ID.to_string())");
  });
});
