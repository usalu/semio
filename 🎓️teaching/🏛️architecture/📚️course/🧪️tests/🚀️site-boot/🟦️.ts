/** 🚀️ The site of the stack under test boots: a first visitor gets the introduction of the course in the browser's
 * language, straight from the proctor. Every other spec runs after this one, so a cold dev server has transformed and
 * optimized its modules before anything is asserted about a clean console — the second, fresh visitor here must
 * already see one.
 * @see ../../🎭️e2e/🎚️config/🟦️.ts — the projects that depend on this one */
import { COURSE, arrive, expect, screen, test, unresolvedLabels } from "../../🎭️e2e/🚶️learner/🟦️.ts";
import logo from "../../🧫️fixtures/🪧️logo/🔣️.json" with { type: "json" };

test("the site boots and shows the course's introduction", async ({ device }) => {
  const warming = await device("en");
  await warming.expectingFailures(() => arrive(warming, 150_000));
  await warming.page.waitForLoadState("networkidle");
  for (const locale of ["en", "de"] as const) {
    const visitor = await device(locale);
    await arrive(visitor);
    const mark = visitor.page.locator(`[data-course-brand] ${logo.tag}`);
    await expect(mark).toBeVisible();
    await expect(mark).toHaveAttribute("alt", logo.decorative ? "" : "semio");
    await expect.poll(() => mark.evaluate((image) => (image as HTMLImageElement).naturalWidth)).toBeGreaterThanOrEqual(logo.minimumWidth);
    const introduction = screen(visitor.page, "introduction");
    await expect(introduction.locator('[data-slot="introduction-info-box-title"]')).toHaveText(COURSE.introduction.title[locale]);
    for (const paragraph of COURSE.introduction.paragraphs) await expect(introduction).toContainText(paragraph[locale]);
    expect(await unresolvedLabels(visitor)).toEqual([]);
  }
});
