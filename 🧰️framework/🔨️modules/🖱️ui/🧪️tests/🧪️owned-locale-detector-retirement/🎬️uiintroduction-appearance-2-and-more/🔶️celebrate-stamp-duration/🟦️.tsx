/** 🔶️ The celebrate stamp duration phase of registerUiintroductionAppearance2AndMoreTests. */
import type { IntroductionStepDefinition } from "@semio-tech/ui-react/source";
import type { Tests1Vitest, Tests1Dependencies } from "../../🧰️harness/🟦️.ts";

/** 🔷 Builds CELEBRATE_STAMP_DURATION_MS, INTRODUCTION_INFO_BOX_GAP_PX, UIIntroduction, clampIntroductionInfoBoxPosition of registerUiintroductionAppearance2AndMoreTests. */
export async function buildCelebrateStampDurationState({ dependencies, vitest }: { readonly dependencies: Tests1Dependencies; readonly vitest: Tests1Vitest }) {
  const { CELEBRATE_STAMP_DURATION_MS, INTRODUCTION_INFO_BOX_GAP_PX, UIIntroduction, clampIntroductionInfoBoxPosition, introductionDemoResolveVisual, introductionPointRelativeToHost, introductionRectRelativeToHost, ndcToViewportPoint, polylinePointAt, registerIntroductionSurfaceResolver, resolveIntroductionPlacement, resolveIntroductionPoint, sampleBezierSegments } = dependencies;
  type Anchor = any;
  type IntroductionStepDefinition = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, act } = await import("@testing-library/react");
  return { describe, it, render, UIIntroduction, vi, expect, screen, act, CELEBRATE_STAMP_DURATION_MS, fireEvent, introductionRectRelativeToHost, introductionPointRelativeToHost, clampIntroductionInfoBoxPosition, INTRODUCTION_INFO_BOX_GAP_PX, resolveIntroductionPlacement, resolveIntroductionPoint, registerIntroductionSurfaceResolver, polylinePointAt, sampleBezierSegments, ndcToViewportPoint, introductionDemoResolveVisual };
}
