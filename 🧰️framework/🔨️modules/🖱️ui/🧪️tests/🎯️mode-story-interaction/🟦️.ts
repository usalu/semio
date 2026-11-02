import { expect, userEvent, waitFor, within } from "storybook/test";
import fixture from "../🧫️fixtures/🎭️mode/🔣️.json" with { type: "json" };

type CanvasContext = { canvasElement: HTMLElement };

export async function quadLayoutPlay({ canvasElement }: CanvasContext): Promise<void> {
  const panels = within(canvasElement).getAllByRole("tabpanel");
  expect(panels).toHaveLength(fixture.quadLabels.length);
  for (const label of fixture.quadLabels) expect(panels.some(panel => panel.textContent?.includes(label))).toBe(true);
}

export async function maximizeStackPlay({ canvasElement }: CanvasContext): Promise<void> {
  const stacks = () => canvasElement.querySelectorAll('[data-slot="mode-dock-stack"]');
  const focus = () => canvasElement.querySelector<HTMLButtonElement>('[data-slot="mode-dock-tab-focus"]');
  expect(stacks()).toHaveLength(fixture.restoredStacks);
  expect(focus()).not.toBeNull();
  await userEvent.click(focus()!);
  await waitFor(() => expect(stacks()).toHaveLength(fixture.maximizedStacks));
  await userEvent.click(focus()!);
  await waitFor(() => expect(stacks()).toHaveLength(fixture.restoredStacks));
}

export async function contentThroughGlassPlay({ canvasElement }: CanvasContext): Promise<void> {
  const canvas = within(canvasElement);
  expect(canvas.getByLabelText(fixture.flowContent)).toBeVisible();
  await userEvent.click(canvas.getByRole("tab", { name: fixture.referenceContent }));
  await waitFor(() => expect(canvas.getByRole("tabpanel")).toHaveTextContent(fixture.referenceContent));
  await userEvent.click(canvas.getByRole("tab", { name: "Flow / Fluss" }));
  await waitFor(() => expect(canvas.getByLabelText(fixture.flowContent)).toBeVisible());
}
