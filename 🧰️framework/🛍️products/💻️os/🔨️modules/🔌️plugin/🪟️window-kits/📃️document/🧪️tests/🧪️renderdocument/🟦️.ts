import { readFileSync } from "node:fs";

import type { UiValue } from "@semio-tech/framework";

type TestSource = { readonly directory: string; readonly url: string };

type Fixture = {
  readonly cases: readonly {
    readonly locale: "en" | "de";
    readonly page: number;
    readonly item: number;
    readonly text: string;
    readonly revision: string;
    readonly staticArguments?: Readonly<Record<string, UiValue>>;
    readonly labels: { readonly apply: string; readonly discard: string; readonly cancel: string };
  }[];
};

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "renderDocument" | "editableDocumentDraft">, source: TestSource): Promise<void> {
  const { renderDocument, editableDocumentDraft } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderDocument", () => {
    it("renders one child per page", () => {
      const node = renderDocument({ pages: [{ text: "p1" }, { text: "p2" }] });
      if (node.component.type !== "container") throw new Error("expected container");
      expect(node.children.length).toBe(2);
    });

    it("matches the language-neutral editable draft fixture", () => {
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/✏️editable/🔣️.json", source.url), "utf8")) as Fixture;
      
      
      
      for (const testCase of fixture.cases) {
        const draft = editableDocumentDraft({ pageIndex: testCase.page, itemIndex: testCase.item, text: testCase.text, arguments: testCase.staticArguments }, testCase.locale);
        expect(draft).toEqual({
          page: testCase.page,
          item: testCase.item,
          revision: testCase.revision,
          text: testCase.text,
          labels: testCase.labels,
          arguments: testCase.staticArguments ?? { page: testCase.page, item: testCase.item, revision: testCase.revision },
        });
        if (testCase.staticArguments) {
          expect(draft.arguments).not.toBe(testCase.staticArguments);
          expect(draft.arguments.address).not.toBe(testCase.staticArguments.address);
          expect(draft.arguments).not.toHaveProperty("page");
        }
      }
    });
  });

}
