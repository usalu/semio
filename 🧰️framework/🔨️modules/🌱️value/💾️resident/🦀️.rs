//! 💾️ Canonical composition resident capacity and exact allocation ownership.

//#region 📏️Capacity
pub const RESIDENT_MAXIMUM_COUNT: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentFault {
    Count,
    Capacity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentResources {
    bytes: u64,
    slots: u64,
    owners: u64,
}

impl ResidentResources {
    pub fn new(bytes: u64, slots: u64, owners: u64) -> Result<Self, ResidentFault> {
        if bytes > RESIDENT_MAXIMUM_COUNT || slots > RESIDENT_MAXIMUM_COUNT || owners > RESIDENT_MAXIMUM_COUNT { return Err(ResidentFault::Count); }
        Ok(Self { bytes, slots, owners })
    }

    pub fn bytes(self) -> u64 { self.bytes }
    pub fn slots(self) -> u64 { self.slots }
    pub fn owners(self) -> u64 { self.owners }

    pub fn checked_add(self, other: Self) -> Result<Self, ResidentFault> {
        Self::new(self.bytes.checked_add(other.bytes).ok_or(ResidentFault::Count)?, self.slots.checked_add(other.slots).ok_or(ResidentFault::Count)?, self.owners.checked_add(other.owners).ok_or(ResidentFault::Count)?)
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, ResidentFault> {
        Self::new(self.bytes.checked_sub(other.bytes).ok_or(ResidentFault::Capacity)?, self.slots.checked_sub(other.slots).ok_or(ResidentFault::Capacity)?, self.owners.checked_sub(other.owners).ok_or(ResidentFault::Capacity)?)
    }

    pub fn fits_within(self, capacity: Self) -> bool {
        self.bytes <= capacity.bytes && self.slots <= capacity.slots && self.owners <= capacity.owners
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentCapacity {
    total: ResidentResources,
    control: ResidentResources,
}

impl ResidentCapacity {
    pub fn new(total: ResidentResources, control: ResidentResources) -> Result<Self, ResidentFault> {
        if !control.fits_within(total) { return Err(ResidentFault::Capacity); }
        Ok(Self { total, control })
    }

    pub fn total(self) -> ResidentResources { self.total }
    pub fn control(self) -> ResidentResources { self.control }
    pub fn data(self) -> ResidentResources {
        ResidentResources { bytes: self.total.bytes - self.control.bytes, slots: self.total.slots - self.control.slots, owners: self.total.owners - self.control.owners }
    }
}
//#endregion 📏️Capacity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
