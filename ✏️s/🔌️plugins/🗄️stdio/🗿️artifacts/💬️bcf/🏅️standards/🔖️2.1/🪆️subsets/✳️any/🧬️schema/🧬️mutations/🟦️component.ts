/** 🧬️ BcfMutation union. Mirrors only `SetSnapshot` of the Rust `BcfMutation` enum's 13 variants
 * — `../📸️snapshot/🟦️component.ts`'s `BcfSnapshot` is still the pre-rewrite raw-`entries` stub,
 * so the other 12 variants (SetVersion, InsertTopic, RemoveTopic, SetTopicMarkup, InsertComment,
 * RemoveComment, SetComment, InsertViewpoint, RemoveViewpoint, SetViewpointCamera,
 * SetViewpointComponents, SetViewpointSnapshot) have no TS payload types to mirror against yet;
 * see `🦀️.rs` in this directory. */
export type BcfMutation =
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').BcfSnapshot };
