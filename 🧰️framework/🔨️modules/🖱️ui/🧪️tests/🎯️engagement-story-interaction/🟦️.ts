import { expect, within } from "storybook/test";

export const searchStandalonePlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  expect(within(canvasElement).getByPlaceholderText("Ask or action…")).toBeTruthy();
};

export const standalonePlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  expect(within(canvasElement).getByText("Ready")).toBeTruthy();
};

export const withControlPlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  const canvas = within(canvasElement);
  expect(canvas.getByText("Step: Height")).toBeTruthy();
  expect(canvas.getByText("Height (m)")).toBeTruthy();
  expect(canvas.getByRole("spinbutton")).toHaveValue(3);
  expect(canvasElement.querySelector('[data-slot="engagement-control"][data-control-kind="stepper"]')).toBeTruthy();
};
