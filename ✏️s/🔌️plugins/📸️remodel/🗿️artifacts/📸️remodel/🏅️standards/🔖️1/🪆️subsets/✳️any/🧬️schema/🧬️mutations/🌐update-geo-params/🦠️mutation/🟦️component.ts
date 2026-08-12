/** ⚙️ update-geo-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateGeoParams {
  params: { enabled: boolean; originLon?: number; originLat?: number; originAlt?: number; gsdM: number; dsmCellM: number; dtmFilterRadiusM: number; orthoMaxPx: number; };
}
