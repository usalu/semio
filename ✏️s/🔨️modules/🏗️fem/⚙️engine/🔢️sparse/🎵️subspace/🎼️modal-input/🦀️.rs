//! 🎼️ Modal input construction that feeds the subspace iteration.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModalInputStage {
    ValidateMass,
    CountUpper,
    ReserveColptr,
    ReserveRowind,
    ReserveValues,
    CopyUpper,
    ReserveMassIndptr,
    ReserveMassIndices,
    ReserveMassValues,
    CopyMountedMass,
    RetireMountedMass,
    BuildMass,
    Complete,
}

/// 🎼 Cursorized converged-stiffness and physical lumped-mass transfer into generalized modes.
pub struct ModalInputConstruction {
    pub(super) matrix: Option<Csr>,
    pub(super) mass: Option<VecD>,
    mounted_mass: Option<MountedScalarSlots>,
    pub(super) stage: ModalInputStage,
    row: usize,
    pub(super) entry: usize,
    upper_count: usize,
    pub(super) colptr: Vec<u32>,
    pub(super) rowind: Vec<u32>,
    pub(super) values: Vec<f64>,
    mass_indptr: Vec<u32>,
    mass_indices: Vec<u32>,
    mass_values: Vec<f64>,
    pub(super) complete: Option<(CscSym, Csr)>,
}

impl ModalInputConstruction {
    pub fn new(matrix: Csr, mass: VecD) -> Self {
        Self {
            matrix: Some(matrix),
            mass: Some(mass),
            mounted_mass: None,
            stage: ModalInputStage::ValidateMass,
            row: 0,
            entry: 0,
            upper_count: 0,
            colptr: Vec::new(),
            rowind: Vec::new(),
            values: Vec::new(),
            mass_indptr: Vec::new(),
            mass_indices: Vec::new(),
            mass_values: Vec::new(),
            complete: None,
        }
    }

    pub fn new_mounted(matrix: Csr, mass: MountedScalarSlots) -> Self {
        let mut construction = Self::new(matrix, VecD::from_vec(Vec::new()));
        construction.mass = None;
        construction.mounted_mass = Some(mass);
        construction
    }

    fn reserve<T>(owner: &mut Vec<T>, count: usize) -> Result<(), &'static [u8]> {
        owner.try_reserve_exact(count).map_err(|_| b"modal-input-owner-allocation" as &'static [u8])?;
        if owner.capacity().checked_mul(size_of::<T>()).is_none_or(|bytes| bytes > NUMERICAL_OWNER_PAGE_BYTES) {
            return Err(b"modal-input-owner-page");
        }
        Ok(())
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        let matrix = self.matrix.as_ref().ok_or(b"modal-input-matrix-owner" as &'static [u8])?;
        let n = matrix.n;
        match self.stage {
            ModalInputStage::ValidateMass => {
                let mass_len = self.mass.as_ref().map(VecD::len).or_else(|| self.mounted_mass.as_ref().map(MountedScalarSlots::len)).ok_or(b"modal-input-mass-owner" as &'static [u8])?;
                if mass_len != n {
                    return Err(b"modal-input-mass-order");
                }
                if self.row < n {
                    let value = self.mass.as_ref().map(|mass| mass.get(self.row)).or_else(|| self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row))).ok_or(b"modal-input-mass-owner" as &'static [u8])?;
                    if !value.is_finite() || value <= 0.0 {
                        return Err(b"modal-input-mass-value");
                    }
                    self.row += 1;
                } else {
                    self.row = 0;
                    self.entry = 0;
                    self.stage = ModalInputStage::CountUpper;
                }
            }
            ModalInputStage::CountUpper => {
                if self.row == n {
                    self.stage = ModalInputStage::ReserveColptr;
                } else {
                    let end = matrix.indptr(self.row + 1).ok_or(b"modal-input-row-end" as &'static [u8])? as usize;
                    if self.entry < end {
                        if matrix.index(self.entry).ok_or(b"modal-input-column" as &'static [u8])? as usize >= self.row {
                            self.upper_count = self.upper_count.checked_add(1).ok_or(b"modal-input-upper-overflow" as &'static [u8])?;
                        }
                        self.entry += 1;
                    } else {
                        self.row += 1;
                        self.entry = matrix.indptr(self.row).unwrap_or(0) as usize;
                    }
                }
            }
            ModalInputStage::ReserveColptr => {
                Self::reserve(&mut self.colptr, n + 1)?;
                self.colptr.push(0);
                self.stage = ModalInputStage::ReserveRowind;
            }
            ModalInputStage::ReserveRowind => {
                Self::reserve(&mut self.rowind, self.upper_count)?;
                self.stage = ModalInputStage::ReserveValues;
            }
            ModalInputStage::ReserveValues => {
                Self::reserve(&mut self.values, self.upper_count)?;
                self.row = 0;
                self.entry = 0;
                self.stage = ModalInputStage::CopyUpper;
            }
            ModalInputStage::CopyUpper => {
                if self.row == n {
                    self.stage = ModalInputStage::ReserveMassIndptr;
                } else {
                    let end = matrix.indptr(self.row + 1).ok_or(b"modal-input-row-end" as &'static [u8])? as usize;
                    if self.entry < end {
                        let column = matrix.index(self.entry).ok_or(b"modal-input-column" as &'static [u8])? as usize;
                        if column >= self.row {
                            self.rowind.push(column as u32);
                            self.values.push(matrix.value(self.entry).ok_or(b"modal-input-value" as &'static [u8])?);
                        }
                        self.entry += 1;
                    } else {
                        self.colptr.push(self.rowind.len() as u32);
                        self.row += 1;
                        self.entry = matrix.indptr(self.row).unwrap_or(0) as usize;
                    }
                }
            }
            ModalInputStage::ReserveMassIndptr => {
                Self::reserve(&mut self.mass_indptr, n + 1)?;
                self.mass_indptr.push(0);
                self.stage = ModalInputStage::ReserveMassIndices;
            }
            ModalInputStage::ReserveMassIndices => {
                Self::reserve(&mut self.mass_indices, n)?;
                self.stage = ModalInputStage::ReserveMassValues;
            }
            ModalInputStage::ReserveMassValues => {
                if let Some(mut mass) = self.mass.take() {
                    self.mass_values = std::mem::take(&mut mass.0);
                    self.row = 0;
                    self.stage = ModalInputStage::BuildMass;
                } else {
                    Self::reserve(&mut self.mass_values, n)?;
                    self.row = 0;
                    self.stage = ModalInputStage::CopyMountedMass;
                }
            }
            ModalInputStage::CopyMountedMass => {
                if self.row < n {
                    let value = self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row)).ok_or(b"modal-input-mounted-mass-owner" as &'static [u8])?;
                    self.mass_values.push(value);
                    self.row += 1;
                } else {
                    self.stage = ModalInputStage::RetireMountedMass;
                }
            }
            ModalInputStage::RetireMountedMass => {
                if self.mounted_mass.as_mut().is_some_and(|mass| mass.close_step()) {
                    self.mounted_mass = None;
                    self.row = 0;
                    self.stage = ModalInputStage::BuildMass;
                }
            }
            ModalInputStage::BuildMass => {
                if self.row < n {
                    self.mass_indices.push(self.row as u32);
                    self.mass_indptr.push((self.row + 1) as u32);
                    self.row += 1;
                } else {
                    let stiffness = CscSym { n, colptr: std::mem::take(&mut self.colptr), rowind: std::mem::take(&mut self.rowind), vals: std::mem::take(&mut self.values) };
                    let mass = Csr::from_owned_parts(n, std::mem::take(&mut self.mass_indptr), std::mem::take(&mut self.mass_indices), std::mem::take(&mut self.mass_values));
                    self.complete = Some((stiffness, mass));
                    self.stage = ModalInputStage::Complete;
                }
            }
            ModalInputStage::Complete => return Ok(true),
        }
        Ok(self.stage == ModalInputStage::Complete)
    }

    pub fn take_complete(&mut self) -> Option<(CscSym, Csr)> {
        (self.stage == ModalInputStage::Complete).then(|| self.complete.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some((stiffness, mass)) = self.complete.as_mut() {
            let (terminal, items, bytes) = stiffness.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            let (terminal, items, bytes) = mass.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.complete = None;
            return (false, 1, 0);
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.matrix = None;
            return (false, 1, 0);
        }
        if let Some(mass) = self.mass.as_mut() {
            match close_vec_owner_step(&mut mass.0, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            self.mass = None;
            return (false, 1, 0);
        }
        if let Some(mass) = self.mounted_mass.as_mut() {
            if !mass.close_step() {
                return (false, 1, 0);
            }
            self.mounted_mass = None;
            return (false, 1, 0);
        }
        for owner in [&mut self.values, &mut self.mass_values] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.colptr, &mut self.rowind, &mut self.mass_indptr, &mut self.mass_indices] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }
}
