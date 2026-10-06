/** ⏱️ Preparation and owner waits consume no part of the aggregate native capture allowance. */
export class CargoMetadataCaptureBudget {
  private capturedMs = 0;
  constructor(private readonly maximumMs: number) {
    if (!Number.isFinite(maximumMs) || maximumMs <= 0) throw Error("Invalid Cargo metadata capture budget");
  }
  get remainingMs(): number { return Math.max(0, this.maximumMs - this.capturedMs); }
  charge(milliseconds: number): void {
    if (!Number.isFinite(milliseconds) || milliseconds < 0) throw Error("Invalid Cargo metadata capture duration");
    this.capturedMs += milliseconds;
    if (this.capturedMs >= this.maximumMs) throw Error("Cargo metadata capture budget exhausted");
  }
}
