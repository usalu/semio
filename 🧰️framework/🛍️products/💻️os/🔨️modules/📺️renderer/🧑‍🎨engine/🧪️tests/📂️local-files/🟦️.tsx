/** 📂️ Language-neutral local file reopening contract. */
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import Ajv from "ajv";
import type { LocalDocument } from "@semio-tech/framework/config/mutations";
import { LocalFilesV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/📂️files/🟦️.tsx";
import fixture from "./🔣️.json";

afterEach(cleanup);
const doc = (id: string, time = 1, generic = true): LocalDocument => ({ documentId: id, schema: "stdio.txt", name: id + ".txt", storage: "folder", target: "/data/" + id, admittedAtMs: time, ...(generic ? { surface: { pluginId: "stdio-text", appId: "s.stdio.txt@utf-8/*#editor" } } : {}) });
const props = () => ({ documents: [doc("z"), doc("a"), doc("new", 2), doc("studio", 3, false)], locale: "en", activeDocumentId: "a", persisted: new Set(["a"]), onOpen: vi.fn(async () => {}) });
it("validates the neutral locale corpus independently", () => { expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true); });
for (const row of fixture.cases) it("renders accessible ordered files in " + row.locale, () => {
  render(<LocalFilesV1 {...props()} locale={row.locale} />);
  expect(screen.getByRole("region", { name: row.title })).toBeTruthy();
  const buttons = screen.getAllByRole("button");
  expect(buttons.map(button => button.textContent)).toEqual(["new.txt", "a.txt", "z.txt"]);
  expect(buttons[1].getAttribute("aria-current")).toBe("true");
  expect(buttons[1].getAttribute("data-semio-local-persisted")).toBe("true");
  expect(buttons[0].getAttribute("data-semio-local-persisted")).toBe("false");
  buttons[1].focus(); expect(document.activeElement).toBe(buttons[1]);
});
it("renders unknown locale bilingually and excludes studio-only lists", () => {
  const view = render(<LocalFilesV1 {...props()} locale="fr" />);
  expect(screen.getByRole("region").getAttribute("aria-label")).toContain("Files on this device");
  expect(screen.getByRole("region").getAttribute("aria-label")).toContain("Dateien auf diesem Gerät");
  view.rerender(<LocalFilesV1 {...props()} documents={[doc("studio", 1, false)]} />);
  expect(view.container.innerHTML).toBe("");
});
it("admits one async open, cancels it, and safely retires after unmount", async () => {
  let finish!: () => void, signal!: AbortSignal;
  const onOpen = vi.fn((_document: LocalDocument, input: AbortSignal) => { signal = input; return new Promise<void>(resolve => { finish = resolve; }); });
  const view = render(<LocalFilesV1 {...props()} onOpen={onOpen} />);
  fireEvent.click(screen.getByRole("button", { name: "a.txt" }));
  fireEvent.click(screen.getByRole("button", { name: "z.txt" }));
  expect(onOpen).toHaveBeenCalledTimes(1);
  expect(screen.getByRole("status").textContent).toContain("a.txt");
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  expect(signal.aborted).toBe(true);
  view.unmount();
  await act(async () => finish());
});
it("shows opening failures and permits retry", async () => {
  const onOpen = vi.fn().mockRejectedValueOnce(new Error("disk missing")).mockResolvedValue(undefined);
  render(<LocalFilesV1 {...props()} onOpen={onOpen} />);
  await act(async () => fireEvent.click(screen.getByRole("button", { name: "a.txt" })));
  expect(screen.getByRole("alert").textContent).toContain("disk missing");
  await act(async () => fireEvent.click(screen.getByRole("button", { name: "a.txt" })));
  expect(onOpen).toHaveBeenCalledTimes(2);
});
