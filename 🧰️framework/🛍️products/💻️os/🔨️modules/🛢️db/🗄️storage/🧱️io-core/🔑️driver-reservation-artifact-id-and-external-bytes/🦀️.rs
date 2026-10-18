//! 🔑️ Driver reservation, artifact id and external bytes: the driver reservation, the artifact id owner and the external bytes owner. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🎟️ Pre-admitted capacity for one external-driver allocation owned by an aggregate operation.
pub struct DbIoDriverReservation {
    pub operation: u64,
    pub credit: DbIoCredit,
    pub returned: bool,
}

impl DbIoDriverReservation {
    pub fn try_reserve(operation: u64, maximum_capacity: usize) -> Result<Self, DbError> {
        let credit = DbIoCredit { pages: 0, bytes: maximum_capacity as u64, items: 1, controls: 1 };
        db_io_operation_add(operation, credit)?;
        Ok(Self { operation, credit, returned: false })
    }

    pub fn observe_capacity(&mut self, capacity: usize) -> Result<(), DbError> {
        if self.returned || capacity as u64 > self.credit.bytes {
            return Err(DbError::LimitExceeded("DB I/O external driver allocation capacity"));
        }
        let slack = self.credit.bytes - capacity as u64;
        if slack != 0 {
            db_io_operation_return(self.operation, DbIoCredit { pages: 0, bytes: slack, items: 0, controls: 0 })?;
            self.credit.bytes = capacity as u64;
        }
        Ok(())
    }

    pub fn close_step(&mut self) -> Result<(), DbError> {
        if !self.returned {
            db_io_operation_return(self.operation, self.credit)?;
            self.returned = true;
        }
        Ok(())
    }
}

impl Drop for DbIoDriverReservation {
    fn drop(&mut self) {
        if !self.returned {
            let owner = Self { operation: self.operation, credit: self.credit, returned: false };
            self.returned = true;
            if let Err(DbIoLostOwner::DriverReservation(owner)) = db_io_park_lost_owner(DbIoLostOwner::DriverReservation(owner)) {
                *self = owner;
            }
        }
    }
}

/// 🏷️ Exact fixed post-admission artifact identity retained without a heap string.
pub struct DbIoArtifactId {
    pub value: DbIoText,
    driver: Option<ArtifactId>,
    pub external: Option<DbIoExternalBytes>,
    pub reservation: Option<DbIoDriverReservation>,
    pub phase: u8,
}

impl DbIoArtifactId {
    pub fn try_from_text(operation: u64, source: &DbIoText) -> Result<Self, DbError> {
        if operation == 0 {
            return Err(DbError::InvalidArgument("artifact identity requires an admitted operation".to_string()));
        }
        let reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
        let mut owner = Self { value: DbIoText::try_from_str(source.as_str())?, driver: Some(ArtifactId(source.as_str().to_owned())), external: None, reservation: Some(reservation), phase: 0 };
        let capacity = owner.driver.as_ref().ok_or_else(|| DbError::Internal("artifact conversion lost its admitted driver identity".to_string()))?.0.capacity();
        owner.reservation.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its exact reservation".to_string()))?.observe_capacity(capacity)?;
        Ok(owner)
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn as_artifact(&self) -> Result<&ArtifactId, DbError> {
        self.driver.as_ref().ok_or(DbError::Closed)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        match self.phase {
            0 => {
                let driver = self.driver.take().ok_or_else(|| DbError::Internal("artifact conversion lost its exact driver identity".to_string()))?;
                self.external = Some(DbIoExternalBytes::new(driver.0.into_bytes()));
                self.phase = 1;
                Ok(true)
            }
            1 => {
                let external = self.external.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its external allocation".to_string()))?;
                if external.close_step() {
                    return Ok(true);
                }
                self.external.take();
                self.phase = 2;
                Ok(true)
            }
            2 => {
                let _ = self.value.close_step();
                self.phase = 3;
                Ok(true)
            }
            3 => {
                self.reservation.as_mut().ok_or_else(|| DbError::Internal("artifact conversion lost its reservation during close".to_string()))?.close_step()?;
                self.reservation.take();
                self.phase = 4;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == 4 && self.value.terminal_is_empty() && self.driver.is_none() && self.external.is_none() && self.reservation.is_none()
    }
}

impl Drop for DbIoArtifactId {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { value: std::mem::replace(&mut self.value, DbIoText::new()), driver: self.driver.take(), external: self.external.take(), reservation: self.reservation.take(), phase: self.phase };
        self.phase = 4;
        if let Err(DbIoLostOwner::ArtifactId(owner)) = db_io_park_lost_owner(DbIoLostOwner::ArtifactId(owner)) {
            *self = owner;
        }
    }
}

/// 🧳 Retained external allocation with one page-content or backing-release close opportunity.
pub struct DbIoExternalBytes {
    pub value: Option<Vec<u8>>,
    pub phase: u8,
}

impl DbIoExternalBytes {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value: Some(value), phase: 0 }
    }

    pub fn capacity(&self) -> Result<usize, DbError> {
        self.value.as_ref().map(Vec::capacity).ok_or_else(|| DbError::Closed)
    }

    pub fn as_slice(&self) -> Result<&[u8], DbError> {
        self.value.as_deref().ok_or(DbError::Closed)
    }

    pub fn into_value(mut self) -> Result<Vec<u8>, DbError> {
        let value = self.value.take().ok_or(DbError::Closed)?;
        self.phase = 2;
        Ok(value)
    }

    pub fn close_step(&mut self) -> bool {
        let Some(value) = self.value.as_mut() else { return false };
        if !value.is_empty() {
            value.truncate(value.len().saturating_sub(DB_IO_PAGE_BYTES));
            return true;
        }
        if self.phase == 0 {
            self.phase = 1;
            return true;
        }
        self.value.take();
        self.phase = 2;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.phase == 2
    }
}

impl Drop for DbIoExternalBytes {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let owner = Self { value: self.value.take(), phase: self.phase };
        self.phase = 2;
        if let Err(DbIoLostOwner::ExternalBytes(owner)) = db_io_park_lost_owner(DbIoLostOwner::ExternalBytes(owner)) {
            *self = owner;
        }
    }
}
//#endregion 🔖️Limits
