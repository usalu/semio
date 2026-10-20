import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";
import { buildCelebrateStampDurationState } from "./🔶️celebrate-stamp-duration/🟦️.tsx";
import { registerUiintroductionAppearanceTests } from "./🔸️uiintroduction-appearance/🟦️.tsx";
import { registerIntroductionRectRelativeHostTests } from "./🟠️introduction-rect-relative-host/🟦️.tsx";
import { registerResolveIntroductionPointTests } from "./🟢️resolve-introduction-point/🟦️.tsx";
import { registerSampleBezierSegmentsTests } from "./🟣️sample-bezier-segments/🟦️.tsx";

export async function registerUiintroductionAppearance2AndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { describe, it, render, UIIntroduction, vi, expect, screen, act, CELEBRATE_STAMP_DURATION_MS, fireEvent, introductionRectRelativeToHost, introductionPointRelativeToHost, clampIntroductionInfoBoxPosition, INTRODUCTION_INFO_BOX_GAP_PX, resolveIntroductionPlacement, resolveIntroductionPoint, registerIntroductionSurfaceResolver, polylinePointAt, sampleBezierSegments, ndcToViewportPoint, introductionDemoResolveVisual } = await buildCelebrateStampDurationState({ dependencies, vitest });
  registerUiintroductionAppearanceTests({ describe, it, render, UIIntroduction, vi, expect, screen, act, CELEBRATE_STAMP_DURATION_MS, fireEvent });
  registerIntroductionRectRelativeHostTests({ describe, it, expect, introductionRectRelativeToHost, introductionPointRelativeToHost, clampIntroductionInfoBoxPosition, INTRODUCTION_INFO_BOX_GAP_PX, resolveIntroductionPlacement });
  registerResolveIntroductionPointTests({ describe, it, render, vi, expect, resolveIntroductionPoint, registerIntroductionSurfaceResolver, polylinePointAt });
  registerSampleBezierSegmentsTests({ describe, it, sampleBezierSegments, expect, ndcToViewportPoint, introductionDemoResolveVisual });
}
