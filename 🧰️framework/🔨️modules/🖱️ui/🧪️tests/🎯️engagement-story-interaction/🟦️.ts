import { expect, within } from "storybook/test";

export const searchStandalonePlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  expect(within(canvasElement).getByPlaceholderText("Ask or action…")).toBeTruthy();
};

export const standalonePlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  expect(within(canvasElement).getByText("Ready")).toBeTruthy();
};

export const withControlPlay = async ({ canvasElement }: { canvasElement: HTMLElement }): Promise<void> => {
  expect(within(canvasElement).getByText("Height")).toBeTruthy();
  expect(canvasElement.querySelector('[data-slot="engagement-control"][data-control-kind="stepper"]')).toBeTruthy();
};
