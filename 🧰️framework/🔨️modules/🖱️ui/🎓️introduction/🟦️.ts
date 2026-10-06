/** 🎓️ Required caller-owned facts for offering an introduction in the current session. */
export interface IntroductionEligibility {
  readonly appId: string;
  readonly hasIntroduction: boolean;
  readonly tutorialActive: boolean;
  readonly suppressed: boolean;
  readonly replayOnLoad: boolean;
  readonly seenOnDevice: boolean;
  readonly dismissedInSession: boolean;
}

/** 🌱️ Offers an authored introduction when its host and session facts permit it. */
export function shouldStartIntroduction(input: IntroductionEligibility): boolean {
  return input.appId.length > 0 && input.hasIntroduction && !input.tutorialActive && !input.suppressed && !input.dismissedInSession && (input.replayOnLoad || !input.seenOnDevice);
}
