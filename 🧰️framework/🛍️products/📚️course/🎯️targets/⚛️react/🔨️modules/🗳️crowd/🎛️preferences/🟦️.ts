/** 👥️ When other learners' answers are shown. */
export const OTHERS_CHOICES = ["never", "submitted", "always"] as const;

/** 🔘️ A persisted choice for showing other learners' answers. */
export type OthersChoice = (typeof OTHERS_CHOICES)[number];