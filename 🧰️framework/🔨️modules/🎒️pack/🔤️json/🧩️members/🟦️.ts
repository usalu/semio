/** 🧩️ Explicit decoded JSON member authority from the owned member-policy schema. */
export const JsonMemberPolicy = Object.freeze({ Reject: "Reject", Replace: "Replace" } as const);

/** 🏷️ Every JSON source reader receives a deliberate decoded-member selection. */
export type JsonMemberPolicy = typeof JsonMemberPolicy[keyof typeof JsonMemberPolicy];
