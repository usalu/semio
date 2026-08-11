/** 🧬️ DxfMutation union — mirrors `🧬️mutations/🦀️component.rs`'s `#[serde(tag = "mutation")]`
 * enum, one discriminated variant per Rust variant, camelCase field names. */

import type { DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfSnapshot, DxfStyle } from '../📸️snapshot/🟦️component.ts';

export type DxfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: DxfSnapshot }
  | { mutation: 'setHeaderVar'; name: string; headerVar: DxfHeaderVar }
  | { mutation: 'removeHeaderVar'; name: string }
  | { mutation: 'insertLayer'; index: number; layer: DxfLayer }
  | { mutation: 'removeLayer'; name: string }
  | { mutation: 'setLayer'; name: string; layer: DxfLayer }
  | { mutation: 'insertStyle'; index: number; style: DxfStyle }
  | { mutation: 'removeStyle'; name: string }
  | { mutation: 'setStyle'; name: string; style: DxfStyle }
  | { mutation: 'insertLinetype'; index: number; linetype: DxfLinetype }
  | { mutation: 'removeLinetype'; name: string }
  | { mutation: 'setLinetype'; name: string; linetype: DxfLinetype }
  | { mutation: 'insertEntity'; index: number; entity: DxfEntity }
  | { mutation: 'removeEntity'; index: number }
  | { mutation: 'setEntity'; index: number; entity: DxfEntity }
  | { mutation: 'insertBlock'; index: number; block: DxfBlock }
  | { mutation: 'removeBlock'; index: number }
  | { mutation: 'setBlock'; index: number; block: DxfBlock };
