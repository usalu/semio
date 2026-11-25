/** 🥞️ The layered overview warms pages the reader has not asked for (the first after 1.5 s, one more every 35 s, up to the budget of a level). Only the grid the reader is on may do that: a grid behind the glass that warmed its own pages would multiply the budget by every nested level, and an idle overview would keep mounting pages.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🥞️levels/🟦️.tsx — `Level`
 * @see ../../../../../../🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts — `nextWarmBoot`, the warm queue */
import { act, cleanup, render } from "@testing-library/react";
import { useEffect, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { Level, rowMajorCells, tracks, type LevelPage } from "../../🔨️modules/🥞️levels/🟦️.tsx";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";

const mounted = new Set<string>();

function Mounted(props: { readonly id: string; readonly children?: ReactElement }): ReactElement {
  useEffect(() => {
    mounted.add(props.id);
    return () => void mounted.delete(props.id);
  }, [props.id]);
  return <div data-mounted={props.id}>{props.children}</div>;
}

function Grid(props: { readonly name: string; readonly count: number; readonly interactive: boolean; readonly nested?: number }): ReactElement {
  const ids = Array.from({ length: props.count }, (_, index) => `${props.name}-${index}`);
  const pages: LevelPage[] = ids.map((id) => ({ id, label: id, render: () => <Mounted id={id}>{props.nested === undefined ? undefined : <Grid name={id} count={props.nested} interactive={false} />}</Mounted> }));
  return <Level label={props.name} pages={pages} cells={rowMajorCells(ids, 4)} openedId={null} interactive={props.interactive} mode="strip" onOpen={() => undefined} renderCard={(page) => <a href={`#/${page.id}`}>{page.label}</a>} columns={tracks([1, 1, 1, 1])} rows={tracks([1, 1])} />;
}

function show(grid: ReactElement): void {
  const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => { throw new Error("unused"); } });
  render(<ArchiveProvider catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])} language="en" t={translator("en")}>{grid}</ArchiveProvider>);
}

const idle = (seconds: number): void => { for (let second = 0; second < seconds; second += 1) act(() => void vi.advanceTimersByTime(1000)); };

beforeEach(() => void vi.useFakeTimers());
afterEach(() => { cleanup(); mounted.clear(); vi.useRealTimers(); });

describe("🥞️ level warming", () => {
  it("mounts no page of a grid behind the glass", () => {
    show(<Grid name="behind" count={8} interactive={false} />);
    idle(300);
    expect([...mounted]).toEqual([]);
  });

  it("mounts at most its budget on the grid the reader is on", () => {
    show(<Grid name="front" count={8} interactive />);
    idle(300);
    expect(mounted.size).toBeGreaterThan(0);
    expect(mounted.size).toBeLessThanOrEqual(4);
  });

  it("mounts at most the budget of one level when every page holds a grid of its own", () => {
    show(<Grid name="root" count={8} interactive nested={8} />);
    let peak = 0;
    for (let step = 0; step < 30; step += 1) {
      idle(10);
      peak = Math.max(peak, mounted.size);
    }
    expect(peak).toBeGreaterThan(0);
    expect(peak).toBeLessThanOrEqual(4);
    expect([...mounted].every((id) => /^root-\d+$/u.test(id))).toBe(true);
  });
});
