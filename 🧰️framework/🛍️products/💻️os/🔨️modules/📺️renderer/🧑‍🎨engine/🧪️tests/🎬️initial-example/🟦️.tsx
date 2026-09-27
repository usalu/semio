/** 🎬️ Language-neutral initial-view traces executed through the React scheduler. */
import { cleanup,render } from "@semio-tech/ui-react/test";
import { act,createElement,StrictMode } from "react";
import { afterEach,expect,it } from "vitest";
import Ajv from "ajv";
import fixture from "../../🧱️elements/🐚️Shell/🎬️initial-example/🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧱️elements/🐚️Shell/🎬️initial-example/🧬️schema/🔣️.json" with { type: "json" };
import { useInitialExampleReadiness } from "../../🧱️elements/🐚️Shell/🎬️initial-example/🟦️.ts";
afterEach(cleanup);
it("validates initial-view traces independently", () => expect(new Ajv().compile(schema)(fixture)).toBe(true));
for (const law of fixture.cases) it(law.name,async () => {
  const pending = new Map<string,{ resolve: () => void;reject: () => void }>();
  const starts = new Map<string,() => Promise<unknown>>();
  const publications = new Map<string,{ promise: Promise<void>; resolve: () => void }>();
  let count = 0;
  const start = (instance: string) => {
    if (!starts.has(instance)) starts.set(instance,() => {
      count++;
      let finish!: () => void;
      const promise = new Promise<void>((resolve) => { finish = resolve; });
      publications.set(instance,{ promise,resolve: finish });
      return new Promise<void>((resolve,reject) => pending.set(instance,{ resolve,reject: () => reject(new Error("terminal refusal")) }));
    });
    return starts.get(instance)!;
  };
  function View({ instance,enabled }: { instance: string;enabled: boolean }) {
    const ready = useInitialExampleReadiness(instance,enabled,start(instance),() => publications.get(instance)!.promise);
    return createElement("div",{ role: "status" },ready ? "ready" : "waiting");
  }
  let view: ReturnType<typeof render> | undefined;
  for (const step of law.steps) {
    if (step.event === "render") {
      const element = createElement(StrictMode,null,createElement(View,{ instance: step.instance,enabled: step.enabled }));
      if (view) view.rerender(element); else view = render(element);
    } else await act(async () => {
      if (step.event === "settle" || step.event === "complete") pending.get(step.instance)!.resolve();
      if (step.event === "settle" || step.event === "publish") publications.get(step.instance)!.resolve();
      if (step.event === "reject") pending.get(step.instance)!.reject();
    });
    expect(view!.container.textContent).toBe(step.ready ? "ready" : "waiting");
    expect(count).toBe(step.starts);
  }
  view!.unmount();
});
