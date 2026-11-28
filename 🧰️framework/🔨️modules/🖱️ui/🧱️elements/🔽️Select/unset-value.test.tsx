/** 🔽️ A controlled select whose value names no option shows its placeholder and lets every option, the first included, change it (the contract of the interpreter's `SelectView` for an unset "add"/"delete" select). */
import * as React from "react";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./🟦️.tsx";

afterEach(cleanup);
const options = [["dead", "Dead Load"], ["live", "Live Load"]] as const;
const fixture = (value: string | undefined, onValueChange: (value: string) => void) => (
  <Select id="unset" value={value} onValueChange={onValueChange}>
    <SelectTrigger aria-label="Delete Term"><SelectValue placeholder="Select" /></SelectTrigger>
    <SelectContent>{options.map(([id, label]) => <SelectItem key={id} value={id}>{label}</SelectItem>)}</SelectContent>
  </Select>
);

it("an unset controlled select shows the placeholder and commits its first option", async () => {
  const committed = vi.fn();
  render(fixture("", committed));
  const trigger = screen.getByRole("combobox", { name: "Delete Term" });
  expect(trigger.textContent).toContain("Select");
  fireEvent.click(trigger);
  fireEvent.click(await screen.findByRole("option", { name: "Dead Load" }));
  await waitFor(() => expect(committed).toHaveBeenCalledWith("dead"));
});

it("an undefined value is uncontrolled and shows the first option, which is why the interpreter must not pass undefined for an unset value", () => {
  render(fixture(undefined, vi.fn()));
  expect(screen.getByRole("combobox", { name: "Delete Term" }).textContent).toContain("Dead Load");
});
