/** 🧬️ EpwMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
import type { EpwSnapshot, EpwLocation, EpwDataPeriods, EpwRecord } from '../📸️snapshot/🟦️component.ts';

export type EpwMutation =
  | { mutation: 'setSnapshot'; snapshot: EpwSnapshot }
  | { mutation: 'setLocation'; location: EpwLocation }
  | { mutation: 'setDesignConditions'; value: string }
  | { mutation: 'setTypicalExtremePeriods'; value: string }
  | { mutation: 'setGroundTemperatures'; value: string }
  | { mutation: 'setHolidaysDst'; value: string }
  | { mutation: 'setComments1'; value: string }
  | { mutation: 'setComments2'; value: string }
  | { mutation: 'setDataPeriods'; dataPeriods: EpwDataPeriods }
  | { mutation: 'insertRecord'; index: number; record: EpwRecord }
  | { mutation: 'removeRecord'; index: number }
  | { mutation: 'setRecordField'; recordIndex: number; fieldIndex: number; value: string };
