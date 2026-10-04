//! 🔍️ Admitted XML identity and parent/ordinal row indexes preserve declared relations.
use super::super::*;
use semio_framework_os_kernel::sqlite_snapshot::transfer::reserve;
pub(super) type Result<T> = std::result::Result<T, ValueError>;
pub(super) fn invalid(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}
fn tick(work: &mut usize, control: &mut SqliteSnapshotControl<'_>) -> Result<()> {
    *work = work.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML index work overflow"))?;
    if *work % 256 == 0 {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, *work, 0)?;
    }
    Ok(())
}
fn sort<T>(values: &mut [T], control: &mut SqliteSnapshotControl<'_>, mut compare: impl FnMut(&T, &T) -> Result<std::cmp::Ordering>) -> Result<()> {
    fn sift<T>(values: &mut [T], mut root: usize, end: usize, work: &mut usize, control: &mut SqliteSnapshotControl<'_>, compare: &mut impl FnMut(&T, &T) -> Result<std::cmp::Ordering>) -> Result<()> {
        loop {
            let Some(left) = root.checked_mul(2).and_then(|value| value.checked_add(1)).filter(|value| *value < end) else { return Ok(()) };
            let right = left + 1;
            let next = if right < end && compare(&values[left], &values[right])?.is_lt() { right } else { left };
            tick(work, control)?;
            if !compare(&values[root], &values[next])?.is_lt() {
                return Ok(());
            }
            values.swap(root, next);
            root = next;
        }
    }
    let mut work = 0;
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, values.len())?;
    for root in (0..values.len() / 2).rev() {
        sift(values, root, values.len(), &mut work, control, &mut compare)?;
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift(values, 0, end, &mut work, control, &mut compare)?;
    }
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, values.len(), values.len())
}
pub(super) struct Identities<'a> {
    pub(super) rows: Vec<&'a SqliteRow>,
}
impl<'a> Identities<'a> {
    pub(super) fn new(table: &'a semio_framework_os_kernel::sqlite_snapshot::SqliteTable, width: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Self> {
        let mut rows = reserve(table.rows.len(), control)?;
        let mut work = 0;
        for row in &table.rows {
            tick(&mut work, control)?;
            if row.rowid <= 0 {
                return Err(invalid("XML identity must be positive"));
            }
            identity(row, width)?;
            rows.push(row);
        }
        sort(&mut rows, control, |a, b| Ok(a.rowid.cmp(&b.rowid)))?;
        for pair in rows.windows(2) {
            tick(&mut work, control)?;
            if pair[0].rowid == pair[1].rowid {
                return Err(invalid("XML duplicate row identity"));
            }
        }
        Ok(Self { rows })
    }
    pub(super) fn index(&self, id: i64) -> Result<usize> {
        self.rows.binary_search_by_key(&id, |row| row.rowid).map_err(|_| invalid("XML relationship has an unknown owner"))
    }
    pub(super) fn get(&self, id: i64) -> Result<&'a SqliteRow> {
        self.index(id).map(|index| self.rows[index])
    }
}
pub(super) struct Groups<'a> {
    rows: Vec<&'a SqliteRow>,
    ordinal: Option<usize>,
    position: Option<usize>,
}
impl<'a> Groups<'a> {
    pub(super) fn new(source: &Identities<'a>, ordinal: Option<usize>, position: Option<usize>, control: &mut SqliteSnapshotControl<'_>) -> Result<Self> {
        let mut rows = reserve(source.rows.len(), control)?;
        let mut work = 0;
        for row in &source.rows {
            tick(&mut work, control)?;
            row.integer(1)?;
            if let Some(column) = ordinal {
                if row.integer(column)? < 0 {
                    return Err(invalid("XML ordinal must be nonnegative"));
                }
            }
            if let Some(column) = position {
                if !matches!(row.text(column)?, "prolog" | "epilog") {
                    return Err(invalid("XML boundary position differs"));
                }
            }
            rows.push(*row);
        }
        sort(&mut rows, control, |a, b| {
            let mut order = a.integer(1)?.cmp(&b.integer(1)?);
            if let Some(column) = position {
                order = order.then(a.text(column)?.cmp(b.text(column)?));
            }
            if let Some(column) = ordinal {
                order = order.then(a.integer(column)?.cmp(&b.integer(column)?));
            }
            Ok(order)
        })?;
        Ok(Self { rows, ordinal, position })
    }
    pub(super) fn all(&self) -> &[&'a SqliteRow] {
        &self.rows
    }
    pub(super) fn children(&self, parent: i64, position: Option<&str>, control: &mut SqliteSnapshotControl<'_>) -> Result<&[&'a SqliteRow]> {
        let compare = |row: &&SqliteRow| -> Result<std::cmp::Ordering> {
            let mut order = row.integer(1)?.cmp(&parent);
            if let Some(column) = self.position {
                order = order.then(row.text(column)?.cmp(position.ok_or_else(|| invalid("XML boundary range lacks its position"))?));
            }
            Ok(order)
        };
        let mut work = 0;
        let mut first = 0;
        let mut end = self.rows.len();
        while first < end {
            tick(&mut work, control)?;
            let middle = first + (end - first) / 2;
            if compare(&self.rows[middle])?.is_lt() {
                first = middle + 1
            } else {
                end = middle
            }
        }
        let start = first;
        end = self.rows.len();
        while first < end {
            tick(&mut work, control)?;
            let middle = first + (end - first) / 2;
            if !compare(&self.rows[middle])?.is_gt() {
                first = middle + 1
            } else {
                end = middle
            }
        }
        let rows = &self.rows[start..first];
        if let Some(column) = self.ordinal {
            for (index, row) in rows.iter().enumerate() {
                tick(&mut work, control)?;
                if row.integer(column)? != integer(index)? {
                    return Err(invalid("XML ordinals must be contiguous and unique"));
                }
            }
        } else if rows.len() > 1 {
            return Err(invalid("XML optional document component is duplicated"));
        }
        Ok(rows)
    }
}
