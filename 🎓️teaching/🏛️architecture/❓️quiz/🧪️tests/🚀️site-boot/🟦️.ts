/** 🚀️ The site of the stack under test boots: a first visitor gets the introduction of the catalog in the browser's
 * language, straight from the proctor. Every other spec runs after this one, so a cold dev server has transformed and
 * optimized its modules before anything is asserted about a clean console — the second, fresh visitor here must
 * already see one.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the projects that depend on this one */
import { CATALOG, arrive, expect, screen, test, unresolvedLabels } from "../../🎭️e2e/🚶️learner/🟦️.ts";

test("the site boots and shows the catalog's introduction", async ({ device }) => {
  const warming = await device("en");
  await warming.expectingFailures(() => arrive(warming, 150_000));
  await warming.page.waitForLoadState("networkidle");
  for (const locale of ["en", "de"] as const) {
    const visitor = await device(locale);
    await arrive(visitor);
    const introduction = screen(visitor.page, "introduction");
    await expect(introduction.getByRole("heading", { level: 1 })).toHaveText(CATALOG.introduction.title[locale]);
    for (const paragraph of CATALOG.introduction.paragraphs) await expect(introduction).toContainText(paragraph[locale]);
    expect(await unresolvedLabels(visitor)).toEqual([]);
  }
});
