/** 📥️ The client's own loads give up: a chunk or language that stays missing after a few tries — a redeploy replaced it —
 * is reported failed, cancelled when its owner leaves, tried afresh on request, and met by a fallback that names both
 * offered languages instead of an empty page.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/📚️app/📥️bounded-load/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx
 */

import { act, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { LANGUAGES } from "@semio-tech/course";
import { LOAD_ATTEMPTS, loadBounded, useBoundedLoad } from "../../🎯️targets/⚛️react/🔨️modules/📚️app/📥️bounded-load/🟦️.ts";
import { LOAD_FALLBACK_TEXTS, LoadFallback } from "../../🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx";

const FAST = { minMs: 1, maxMs: 2 };

describe("📥️ bounded load", () => {
  it("gives up with the last failure after LOAD_ATTEMPTS tries", async () => {
    let calls = 0;
    const load = vi.fn(async () => {
      calls += 1;
      throw new Error(`chunk gone ${calls}`);
    });
    await expect(loadBounded(load, undefined, FAST)).rejects.toThrow(`chunk gone ${LOAD_ATTEMPTS}`);
    expect(load).toHaveBeenCalledTimes(LOAD_ATTEMPTS);
  });

  it("delivers a load that succeeds within its tries and stops asking", async () => {
    let calls = 0;
    const load = vi.fn(async () => {
      calls += 1;
      if (calls < LOAD_ATTEMPTS) throw new Error("not yet");
      return "chunk";
    });
    await expect(loadBounded(load, undefined, FAST)).resolves.toBe("chunk");
    expect(load).toHaveBeenCalledTimes(LOAD_ATTEMPTS);
  });

  it("ends with the abort reason and asks no further", async () => {
    const stop = new AbortController();
    const load = vi.fn(async () => {
      throw new Error("down");
    });
    const loading = loadBounded(load, stop.signal, { minMs: 50, maxMs: 50 });
    await new Promise((resolve) => setTimeout(resolve, 5));
    stop.abort(new Error("left"));
    await expect(loading).rejects.toThrow("left");
    expect(load).toHaveBeenCalledTimes(1);
  });

  it("reports a load as failed once it gave up, tries it afresh on request and delivers it", async () => {
    let healthy = false;
    const load = vi.fn(async () => {
      if (!healthy) throw new Error("chunk gone");
      return "chunk";
    });
    const loaded = vi.fn();
    const { result } = renderHook(() => useBoundedLoad(true, "home", load, loaded, FAST));
    expect(result.current.failed).toBe(false);
    await waitFor(() => expect(result.current.failed).toBe(true));
    expect(load).toHaveBeenCalledTimes(LOAD_ATTEMPTS);
    expect(loaded).not.toHaveBeenCalled();
    healthy = true;
    act(() => result.current.retry());
    expect(result.current.failed).toBe(false);
    await waitFor(() => expect(loaded).toHaveBeenCalledWith("chunk"));
    expect(result.current.failed).toBe(false);
  });

  it("starts nothing while not wanted, forgets a failure when its key changes and cancels on unmount", async () => {
    const load = vi.fn(async () => {
      throw new Error("chunk gone");
    });
    const { result, rerender, unmount } = renderHook(({ wanted, key }) => useBoundedLoad(wanted, key, load, () => undefined, { minMs: 20, maxMs: 20 }), { initialProps: { wanted: false, key: "home" } });
    await new Promise((resolve) => setTimeout(resolve, 5));
    expect(load).not.toHaveBeenCalled();
    rerender({ wanted: true, key: "home" });
    await waitFor(() => expect(load).toHaveBeenCalledTimes(1));
    rerender({ wanted: true, key: "run" });
    expect(result.current.failed).toBe(false);
    unmount();
    const calls = load.mock.calls.length;
    await new Promise((resolve) => setTimeout(resolve, 120));
    expect(load).toHaveBeenCalledTimes(calls);
  });
});

describe("🛟️ load fallback", () => {
  it("waits in every offered language, English first, each marking its own language", () => {
    const { container } = render(<LoadFallback failed={false} onRetry={() => undefined} onReload={() => undefined} />);
    const parts = [...container.querySelectorAll("section[lang]")];
    expect(parts.map((part) => part.getAttribute("lang"))).toEqual([...LANGUAGES]);
    for (const part of parts) expect(within(part as HTMLElement).getByRole("status").textContent).toBe(LOAD_FALLBACK_TEXTS[part.getAttribute("lang") as keyof typeof LOAD_FALLBACK_TEXTS].loading);
    expect(container.querySelector("button")).toBeNull();
    expect(container.querySelector("[aria-busy='true']")).not.toBeNull();
  });

  it("explains the failure and offers a retry and a reload in every offered language", async () => {
    const user = userEvent.setup();
    const retry = vi.fn();
    const reload = vi.fn();
    render(<LoadFallback failed onRetry={retry} onReload={reload} />);
    expect(screen.getAllByRole("alert").map((alert) => alert.textContent)).toEqual(LANGUAGES.map((locale) => LOAD_FALLBACK_TEXTS[locale].failed));
    for (const locale of LANGUAGES) {
      await user.click(screen.getByRole("button", { name: LOAD_FALLBACK_TEXTS[locale].retry }));
      await user.click(screen.getByRole("button", { name: LOAD_FALLBACK_TEXTS[locale].reload }));
    }
    expect(retry).toHaveBeenCalledTimes(LANGUAGES.length);
    expect(reload).toHaveBeenCalledTimes(LANGUAGES.length);
  });

  it("speaks German informally, like the rest of the course", () => {
    expect(LOAD_FALLBACK_TEXTS.de.failed).not.toMatch(/\b(?:Sie|Ihr|Ihre|Ihren|Ihrem|Ihrer|Ihres|Ihnen)\b/u);
    expect(LANGUAGES).toEqual(["en", "de"]);
  });
});
