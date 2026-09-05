export type PresentationMutationKeyword =
  'no-mutation' | 'set-snapshot' | 'insert-slide' | 'remove-slide' | 'set-slide-layout' | 'set-slide-notes' | 'insert-shape' | 'remove-shape' | 'set-shape-frame' | 'set-textbox-blocks' | 'insert-master' | 'remove-master' | 'insert-layout' | 'remove-layout' | 'set-layout-master';
export interface PresentationMutationArg { name: string; value: string; }
