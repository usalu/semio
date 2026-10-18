//! 🏷️ Naming, capacities and pages of the board fill: the neutral naming, the fill capacity constants, the fill text and the page and fixed page containers. See [`super`].

use super::*;

/// ⚪️ The board's own naming until a consumer installs one: `board.brush.<serial>`, `board.fill.<serial>` and bare stage ids.
pub static NEUTRAL_BOARD_NAMING: BoardNaming = BoardNaming {
    brush_node_prefix: "board.brush.",
    brush_edge_prefix: "board.brush.edge.",
    fill_node_prefix: "board.fill.",
    fill_edge_prefix: "board.fill.edge.",
    fill_stage_label: BoardFillStage::id,
    fill_stale_operation_fault: "stale-board-fill-operation",
};

pub const BOARD_FILL_PAGE_ITEMS: usize = 4;

pub const BOARD_FILL_NODE_CAPACITY: usize = 1_024;

pub const BOARD_FILL_HANDLE_CAPACITY: usize = 1_024;

pub const BOARD_FILL_KIND_CAPACITY: usize = 64;

pub const BOARD_FILL_KIND_HANDLE_CAPACITY: usize = 16;

pub const BOARD_FILL_RULE_CAPACITY: usize = 1_024;

pub const BOARD_FILL_PLACEMENT_CAPACITY: usize = 1_000;

pub const BOARD_FILL_SOURCE_CAPACITY: usize = BOARD_FILL_HANDLE_CAPACITY + BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY;

pub(super) const BOARD_FILL_CANDIDATE_CAPACITY: usize = BOARD_FILL_KIND_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardFillText {
    pub bytes: [u8; BOARD_FILL_TEXT_BYTES],
    pub len: u16,
}

impl BoardFillText {
    pub fn empty() -> Self {
        Self { bytes: [0; BOARD_FILL_TEXT_BYTES], len: 0 }
    }

    pub fn try_from_str(value: &str) -> Result<Self, BoardFillCaptureFault> {
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        let mut bytes = [0; BOARD_FILL_TEXT_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    pub(super) fn try_indexed(prefix: &str, first: u64, middle: &str, second: Option<usize>) -> Result<Self, BoardFillCaptureFault> {
        let mut result = Self { bytes: [0; BOARD_FILL_TEXT_BYTES], len: 0 };
        result.try_extend(prefix.as_bytes())?;
        result.try_decimal(first)?;
        result.try_extend(middle.as_bytes())?;
        if let Some(second) = second {
            result.try_decimal(second as u64)?;
        }
        Ok(result)
    }

    fn try_extend(&mut self, bytes: &[u8]) -> Result<(), BoardFillCaptureFault> {
        let start = usize::from(self.len);
        let end = start.checked_add(bytes.len()).ok_or(BoardFillCaptureFault::TextCapacity)?;
        if end > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        self.bytes[start..end].copy_from_slice(bytes);
        self.len = end as u16;
        Ok(())
    }

    pub fn try_push_byte(&mut self, byte: u8) -> Result<(), BoardFillCaptureFault> {
        self.try_extend(std::slice::from_ref(&byte))
    }

    fn try_decimal(&mut self, mut value: u64) -> Result<(), BoardFillCaptureFault> {
        let mut digits = [0_u8; 20];
        let mut len = 0;
        loop {
            digits[len] = b'0' + (value % 10) as u8;
            len += 1;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        for digit in digits[..len].iter().rev() {
            self.try_extend(std::slice::from_ref(digit))?;
        }
        Ok(())
    }
}

struct BoardFillPage<T> {
    pub items: Vec<T>,
    pub backing_bytes: usize,
}

impl<T> BoardFillPage<T> {
    pub fn try_new() -> Result<Self, ()> {
        let mut items = Vec::new();
        items.try_reserve_exact(BOARD_FILL_PAGE_ITEMS).map_err(|_| ())?;
        let backing_bytes = items.capacity().checked_mul(size_of::<T>()).ok_or(())?;
        Ok(Self { items, backing_bytes })
    }
}

pub struct BoardFillFixedPages<T, const CAPACITY: usize> {
    pub pages: Box<[Option<BoardFillPage<T>>]>,
    pub len: usize,
    pub(super) page_count: usize,
}

impl<T, const CAPACITY: usize> BoardFillFixedPages<T, CAPACITY> {
    pub fn new() -> Self {
        Self::try_new().expect("Board fill fixed descriptor backing allocation")
    }

    pub fn try_new() -> Result<Self, ()> {
        let _ = CAPACITY.checked_mul(size_of::<Option<BoardFillPage<T>>>()).ok_or(())?;
        let mut pages = Vec::new();
        pages.try_reserve_exact(CAPACITY).map_err(|_| ())?;
        pages.resize_with(CAPACITY, || None);
        let pages = pages.into_boxed_slice();
        if pages.len() != CAPACITY {
            return Err(());
        }
        Ok(Self { pages, len: 0, page_count: 0 })
    }

    fn item_capacity() -> usize {
        CAPACITY * BOARD_FILL_PAGE_ITEMS
    }

    pub(super) fn try_push_owned(&mut self, item: T) -> Result<(), T> {
        if self.len == Self::item_capacity() {
            return Err(item);
        }
        let page_index = self.len / BOARD_FILL_PAGE_ITEMS;
        if self.pages[page_index].is_none() {
            let page = match BoardFillPage::try_new() {
                Ok(page) => page,
                Err(()) => return Err(item),
            };
            self.pages[page_index] = Some(page);
            self.page_count += 1;
        }
        let Some(page) = self.pages[page_index].as_mut() else { return Err(item) };
        page.items.push(item);
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        self.pages[index / BOARD_FILL_PAGE_ITEMS].as_ref()?.items.get(index % BOARD_FILL_PAGE_ITEMS)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        self.pages[index / BOARD_FILL_PAGE_ITEMS].as_mut()?.items.get_mut(index % BOARD_FILL_PAGE_ITEMS)
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        let item = self.pages[index / BOARD_FILL_PAGE_ITEMS].as_mut()?.items.pop()?;
        self.len = index;
        Some(item)
    }

    pub(super) fn retire_empty_page(&mut self) -> Option<usize> {
        let first_unused = self.len.div_ceil(BOARD_FILL_PAGE_ITEMS);
        let index = self.page_count.checked_sub(1)?;
        if index < first_unused || !self.pages[index].as_ref()?.items.is_empty() {
            return None;
        }
        let bytes = self.pages[index].as_ref()?.backing_bytes;
        self.pages[index] = None;
        self.page_count = index;
        Some(bytes)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.page_count == 0 && self.pages.is_empty()
    }

    pub fn retire_descriptor(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Option<semio_framework_job::InteractiveJobCloseStep> {
        if self.len!=0||self.page_count!=0||self.pages.is_empty(){return None;}
        let released_bytes=size_of_val(self.pages.as_ref());
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<released_bytes{return Some(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        self.pages=Box::new([]);
        Some(semio_framework_job::InteractiveJobCloseStep::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}})
    }

    pub(super) fn retained_page_bytes(&self) -> Option<usize> {
        let index = self.page_count.checked_sub(1)?;
        self.pages[index].as_ref().map(|page| page.backing_bytes)
    }

    pub(super) fn next_release_bytes(&self)->usize {
        if self.len!=0{return 0;}
        self.retained_page_bytes().unwrap_or_else(||size_of_val(self.pages.as_ref()))
    }
}
