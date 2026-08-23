//! 🪢️ Immutable paged text roots and fixed-credit, cursor-stepped edit admission.

use std::sync::Arc;

//#region 🪢️PagedRoot

const TEXT_OPERATION_SLOTS: usize = 64;
const TEXT_PAGE_SLOTS: usize = 256;
const TEXT_PAGE_BYTES: usize = 16 * 1024;
const TEXT_INGRESS_BYTES: usize = 256 * 1024;
const TEXT_ROOT_DEPTH: usize = 64;
const TEXT_PROJECTION_BYTES: usize = 4 * 1024;
const TEXT_DISPOSER_DEPTH: usize = TEXT_ROOT_DEPTH * 4;
const TEXT_RETIRED_ROOT_SLOTS: usize = 4;

#[derive(Debug)]
enum TextNode {
    Page { storage: Arc<String>, start: usize, end: usize },
    Concat { left: Arc<TextNode>, right: Arc<TextNode>, left_bytes: usize, bytes: usize, depth: u16 },
}

#[derive(Debug, Default)]
pub struct TextRoot {
    node: Option<Arc<TextNode>>,
    bytes: usize,
}

impl TextRoot {
    pub fn from_owned(text: String) -> Result<Self, TextEditFault> {
        let bytes = text.len();
        if bytes > TEXT_PAGE_BYTES {
            return Err(TextEditFault::ChunkTooLarge);
        }
        let node = (bytes != 0).then(|| Arc::new(TextNode::Page { storage: Arc::new(text), start: 0, end: bytes }));
        Ok(Self { node, bytes })
    }

    pub const fn len(&self) -> usize {
        self.bytes
    }

    pub const fn is_empty(&self) -> bool {
        self.bytes == 0
    }

    fn projection(&self, start: usize, max_bytes: usize) -> Result<TextProjection, TextEditFault> {
        if max_bytes > TEXT_PROJECTION_BYTES {
            return Err(TextEditFault::ByteCredits);
        }
        let start = start.min(self.bytes);
        if !self.is_char_boundary(start)? {
            return Err(TextEditFault::Protocol);
        }
        let requested_end = start.saturating_add(max_bytes).min(self.bytes);
        let end = self.boundary_at_or_before(requested_end)?;
        let mut stack = NodeStack::default();
        if let Some(node) = self.node.clone() {
            stack.push(node)?;
        }
        Ok(TextProjection { start, end, offset: 0, stack, output: String::with_capacity(end - start), complete: false })
    }

    pub fn previous_boundary(&self, index: usize) -> Result<usize, TextEditFault> {
        if index == 0 {
            return Ok(0);
        }
        let probe = index.min(self.bytes) - 1;
        self.boundary_near(probe, false)
    }

    pub fn next_boundary(&self, index: usize) -> Result<usize, TextEditFault> {
        if index >= self.bytes {
            return Ok(self.bytes);
        }
        self.boundary_near(index, true)
    }

    pub fn is_char_boundary(&self, index: usize) -> Result<bool, TextEditFault> {
        if index == 0 || index == self.bytes {
            return Ok(true);
        }
        if index > self.bytes {
            return Ok(false);
        }
        let mut node = self.node.clone();
        let mut offset = 0usize;
        for _ in 0..TEXT_ROOT_DEPTH {
            let Some(current) = node else { return Ok(false) };
            match current.as_ref() {
                TextNode::Page { storage, start, end } => {
                    let page_end = offset + end - start;
                    if index <= page_end {
                        return Ok(storage.is_char_boundary(start + index - offset));
                    }
                    return Ok(false);
                }
                TextNode::Concat { left, right, left_bytes, .. } => {
                    if index <= offset + *left_bytes {
                        node = Some(left.clone());
                    } else {
                        offset += *left_bytes;
                        node = Some(right.clone());
                    }
                }
            }
        }
        Err(TextEditFault::RootDepth)
    }

    pub fn boundary_at_or_before(&self, index: usize) -> Result<usize, TextEditFault> {
        if self.is_char_boundary(index)? {
            return Ok(index);
        }
        self.previous_boundary(index.saturating_add(1))
    }

    fn boundary_near(&self, probe: usize, forward: bool) -> Result<usize, TextEditFault> {
        let mut node = self.node.clone();
        let mut offset = 0usize;
        for _ in 0..TEXT_ROOT_DEPTH {
            let Some(current) = node else { return Ok(self.bytes) };
            match current.as_ref() {
                TextNode::Page { storage, start, end } => {
                    let page_end = offset + end - start;
                    if probe < page_end {
                        let local = start + probe.saturating_sub(offset);
                        if forward {
                            let mut boundary = (local + 1).min(*end);
                            while boundary < *end && !storage.is_char_boundary(boundary) {
                                boundary += 1;
                            }
                            return Ok(offset + boundary - start);
                        }
                        let mut boundary = local;
                        while boundary > *start && !storage.is_char_boundary(boundary) {
                            boundary -= 1;
                        }
                        return Ok(offset + boundary - start);
                    }
                    return Ok(self.bytes);
                }
                TextNode::Concat { left, right, left_bytes, .. } => {
                    if probe < offset + *left_bytes {
                        node = Some(left.clone());
                    } else {
                        offset += *left_bytes;
                        node = Some(right.clone());
                    }
                }
            }
        }
        Err(TextEditFault::RootDepth)
    }

    fn page(storage: Arc<String>, start: usize, end: usize) -> Self {
        Self { node: (start != end).then(|| Arc::new(TextNode::Page { storage, start, end })), bytes: end.saturating_sub(start) }
    }

    fn lease(&self) -> Self {
        Self { node: self.node.clone(), bytes: self.bytes }
    }

    fn concat(left: Self, right: Self) -> Result<Self, TextEditFault> {
        if left.is_empty() {
            return Ok(right);
        }
        if right.is_empty() {
            return Ok(left);
        }
        let node = balance(left.node.expect("left node"), right.node.expect("right node"))?;
        Ok(Self { bytes: node_bytes(&node), node: Some(node) })
    }

    #[cfg(test)]
    fn materialize(&self) -> String {
        let mut output = String::with_capacity(self.bytes);
        let mut stack = Vec::with_capacity(TEXT_ROOT_DEPTH);
        if let Some(node) = self.node.clone() {
            stack.push(node);
        }
        while let Some(node) = stack.pop() {
            match node.as_ref() {
                TextNode::Page { storage, start, end } => output.push_str(&storage[*start..*end]),
                TextNode::Concat { left, right, .. } => {
                    stack.push(right.clone());
                    stack.push(left.clone());
                }
            }
        }
        output
    }
}

fn node_bytes(node: &Arc<TextNode>) -> usize {
    match node.as_ref() {
        TextNode::Page { start, end, .. } => end - start,
        TextNode::Concat { bytes, .. } => *bytes,
    }
}

fn node_depth(node: &Arc<TextNode>) -> u16 {
    match node.as_ref() {
        TextNode::Page { .. } => 1,
        TextNode::Concat { depth, .. } => *depth,
    }
}

fn branch(left: Arc<TextNode>, right: Arc<TextNode>) -> Result<Arc<TextNode>, TextEditFault> {
    let depth = node_depth(&left).max(node_depth(&right)).saturating_add(1);
    if depth as usize > TEXT_ROOT_DEPTH {
        return Err(TextEditFault::RootDepth);
    }
    let left_bytes = node_bytes(&left);
    let bytes = left_bytes.saturating_add(node_bytes(&right));
    Ok(Arc::new(TextNode::Concat { left, right, left_bytes, bytes, depth }))
}

fn balance(left: Arc<TextNode>, right: Arc<TextNode>) -> Result<Arc<TextNode>, TextEditFault> {
    let left_depth = node_depth(&left);
    let right_depth = node_depth(&right);
    if left_depth > right_depth.saturating_add(1) {
        if let TextNode::Concat { left: ll, right: lr, .. } = left.as_ref() {
            if node_depth(ll) >= node_depth(lr) {
                return branch(ll.clone(), branch(lr.clone(), right)?);
            }
            if let TextNode::Concat { left: lrl, right: lrr, .. } = lr.as_ref() {
                return branch(branch(ll.clone(), lrl.clone())?, branch(lrr.clone(), right)?);
            }
        }
    }
    if right_depth > left_depth.saturating_add(1) {
        if let TextNode::Concat { left: rl, right: rr, .. } = right.as_ref() {
            if node_depth(rr) >= node_depth(rl) {
                return branch(branch(left, rl.clone())?, rr.clone());
            }
            if let TextNode::Concat { left: rll, right: rlr, .. } = rl.as_ref() {
                return branch(branch(left, rll.clone())?, branch(rlr.clone(), rr.clone())?);
            }
        }
    }
    branch(left, right)
}

struct NodeStack<const N: usize> {
    items: [Option<Arc<TextNode>>; N],
    len: usize,
}

impl<const N: usize> Default for NodeStack<N> {
    fn default() -> Self {
        Self { items: std::array::from_fn(|_| None), len: 0 }
    }
}

impl<const N: usize> NodeStack<N> {
    fn push(&mut self, node: Arc<TextNode>) -> Result<(), TextEditFault> {
        if self.len == N {
            std::mem::forget(node);
            return Err(TextEditFault::RootDepth);
        }
        self.items[self.len] = Some(node);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<Arc<TextNode>> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.items[self.len].take()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn has_capacity(&self) -> bool {
        self.len < N
    }
}

struct TextProjection {
    start: usize,
    end: usize,
    offset: usize,
    stack: NodeStack<TEXT_ROOT_DEPTH>,
    output: String,
    complete: bool,
}

impl TextProjection {
    pub fn step(&mut self, budget: usize) -> Result<bool, TextEditFault> {
        if budget == 0 || self.complete {
            return Ok(self.complete);
        }
        let Some(node) = self.stack.pop() else {
            self.complete = true;
            return Ok(true);
        };
        match node.as_ref() {
            TextNode::Page { storage, start, end } => {
                let offset = self.offset;
                let page_end = offset + end - start;
                if page_end > self.start && offset < self.end {
                    let local_start = start + self.start.saturating_sub(offset);
                    let local_end = start + (self.end.min(page_end) - offset);
                    self.output.push_str(&storage[local_start..local_end]);
                }
                self.offset = page_end;
            }
            TextNode::Concat { left, right, .. } => {
                self.stack.push(right.clone())?;
                self.stack.push(left.clone())?;
            }
        }
        Ok(false)
    }

    pub fn take(self) -> Result<String, TextEditFault> {
        self.complete.then_some(self.output).ok_or(TextEditFault::Protocol)
    }
}

//#endregion 🪢️PagedRoot

//#region 📥️FixedIngress

#[derive(Default)]
struct PageSlot {
    storage: Option<Arc<String>>,
    start: usize,
    end: usize,
    next: Option<u16>,
}

#[derive(Default)]
struct OperationSlot {
    occupied: bool,
    epoch: u64,
    generation: u64,
    declared_bytes: usize,
    received_bytes: usize,
    start: usize,
    end: usize,
    first: Option<u16>,
    last: Option<u16>,
    next: Option<u8>,
    committed: bool,
    retiring: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextIngressToken {
    slot: u8,
    generation: u64,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEditFault {
    ByteCredits,
    ItemCredits,
    PageCredits,
    ChunkTooLarge,
    Generation,
    Protocol,
    RootDepth,
}

pub struct TextEditAuthority {
    root: TextRoot,
    undo: Option<(TextRoot, usize)>,
    operations: [OperationSlot; TEXT_OPERATION_SLOTS],
    pages: [PageSlot; TEXT_PAGE_SLOTS],
    queue_head: Option<u8>,
    queue_tail: Option<u8>,
    active: Option<ActiveEdit>,
    cancelled_active: Option<ActiveEdit>,
    retirement_slots: [Option<u8>; TEXT_OPERATION_SLOTS],
    retirement_head: usize,
    retirement_tail: usize,
    retirement_count: usize,
    reserved_bytes: usize,
    generation: u64,
    next_epoch: u64,
    disposer: NodeStack<TEXT_DISPOSER_DEPTH>,
    retired_roots: [Option<TextRoot>; TEXT_RETIRED_ROOT_SLOTS],
    projection: Option<TextProjection>,
    closing: bool,
    closed_complete: bool,
    faulted: Option<TextEditFault>,
    close_cursor: usize,
}

impl Default for TextEditAuthority {
    fn default() -> Self {
        Self::new(TextRoot::default(), 1)
    }
}

impl TextEditAuthority {
    pub fn new(root: TextRoot, generation: u64) -> Self {
        Self {
            root,
            undo: None,
            operations: std::array::from_fn(|_| OperationSlot::default()),
            pages: std::array::from_fn(|_| PageSlot::default()),
            queue_head: None,
            queue_tail: None,
            active: None,
            cancelled_active: None,
            retirement_slots: [None; TEXT_OPERATION_SLOTS],
            retirement_head: 0,
            retirement_tail: 0,
            retirement_count: 0,
            reserved_bytes: 0,
            generation,
            next_epoch: 1,
            disposer: NodeStack::default(),
            retired_roots: std::array::from_fn(|_| None),
            projection: None,
            closing: false,
            closed_complete: false,
            faulted: None,
            close_cursor: 0,
        }
    }

    pub fn root(&self) -> &TextRoot {
        &self.root
    }

    pub const fn len(&self) -> usize {
        self.root.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn start_projection(&mut self, start: usize, max_bytes: usize) -> Result<(), TextEditFault> {
        if self.projection.is_some() {
            return Err(TextEditFault::Protocol);
        }
        self.projection = Some(self.root.projection(start, max_bytes)?);
        Ok(())
    }

    pub fn step_projection(&mut self, budget: usize) -> Result<Option<String>, TextEditFault> {
        let Some(projection) = self.projection.as_mut() else { return Ok(None) };
        if !projection.step(budget)? {
            return Ok(None);
        }
        let projection = self.projection.take().expect("completed projection");
        Ok(Some(projection.take()?))
    }

    pub fn reserved_bytes(&self) -> usize {
        self.reserved_bytes
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal_empty()
    }

    pub fn replace_owned(&mut self, text: String) -> Result<(), TextEditFault> {
        self.enqueue_owned(self.generation, text, 0, self.root.len())
    }

    pub fn begin(&mut self, generation: u64, declared_bytes: usize, start: usize, end: usize) -> Result<TextIngressToken, TextEditFault> {
        if self.closing {
            return Err(TextEditFault::Protocol);
        }
        if let Some(fault) = self.faulted {
            return Err(fault);
        }
        if generation != self.generation {
            return Err(TextEditFault::Generation);
        }
        if declared_bytes > TEXT_INGRESS_BYTES || self.reserved_bytes.saturating_add(declared_bytes) > TEXT_INGRESS_BYTES {
            return Err(TextEditFault::ByteCredits);
        }
        if start > end || end > self.root.len() || !self.root.is_char_boundary(start)? || !self.root.is_char_boundary(end)? {
            return Err(TextEditFault::Protocol);
        }
        let slot = self.operations.iter().position(|operation| !operation.occupied).ok_or(TextEditFault::ItemCredits)?;
        let epoch = self.next_epoch;
        self.next_epoch = self.next_epoch.wrapping_add(1).max(1);
        self.operations[slot] = OperationSlot { occupied: true, epoch, generation, declared_bytes, start, end, ..OperationSlot::default() };
        self.reserved_bytes += declared_bytes;
        Ok(TextIngressToken { slot: slot as u8, generation, epoch })
    }

    pub fn push(&mut self, token: TextIngressToken, chunk: String) -> Result<(), TextEditFault> {
        if self.closing {
            return Err(TextEditFault::Protocol);
        }
        if chunk.len() > TEXT_PAGE_BYTES {
            return Err(TextEditFault::ChunkTooLarge);
        }
        let operation = self.operations.get(token.slot as usize).ok_or(TextEditFault::Protocol)?;
        if !operation.occupied || operation.epoch != token.epoch || operation.generation != token.generation || operation.committed || operation.received_bytes.saturating_add(chunk.len()) > operation.declared_bytes {
            return Err(TextEditFault::Protocol);
        }
        let page = self.pages.iter().position(|page| page.storage.is_none()).ok_or(TextEditFault::PageCredits)?;
        let operation = &mut self.operations[token.slot as usize];
        let bytes = chunk.len();
        self.pages[page].storage = Some(Arc::new(chunk));
        self.pages[page].start = 0;
        self.pages[page].end = bytes;
        if let Some(last) = operation.last {
            self.pages[last as usize].next = Some(page as u16);
        } else {
            operation.first = Some(page as u16);
        }
        operation.last = Some(page as u16);
        operation.received_bytes += bytes;
        Ok(())
    }

    pub fn commit(&mut self, token: TextIngressToken) -> Result<(), TextEditFault> {
        if self.closing {
            return Err(TextEditFault::Protocol);
        }
        let operation = self.operations.get_mut(token.slot as usize).ok_or(TextEditFault::Protocol)?;
        if !operation.occupied || operation.epoch != token.epoch || operation.generation != token.generation || operation.committed || operation.received_bytes > operation.declared_bytes {
            return Err(TextEditFault::Protocol);
        }
        operation.committed = true;
        match self.queue_tail {
            Some(tail) => self.operations[tail as usize].next = Some(token.slot),
            None => self.queue_head = Some(token.slot),
        }
        self.queue_tail = Some(token.slot);
        Ok(())
    }

    pub fn abort(&mut self, token: TextIngressToken) -> Result<(), TextEditFault> {
        let operation = self.operations.get(token.slot as usize).ok_or(TextEditFault::Protocol)?;
        if !operation.occupied || operation.epoch != token.epoch || operation.generation != token.generation || operation.committed {
            return Err(TextEditFault::Protocol);
        }
        self.enqueue_retirement(token.slot)
    }

    pub fn enqueue_owned(&mut self, generation: u64, text: String, start: usize, end: usize) -> Result<(), TextEditFault> {
        let bytes = text.len();
        if bytes > TEXT_PAGE_BYTES {
            return Err(TextEditFault::ChunkTooLarge);
        }
        let token = self.begin(generation, bytes, start, end)?;
        if let Err(fault) = self.push(token, text) {
            self.enqueue_retirement(token.slot)?;
            return Err(fault);
        }
        self.commit(token)
    }

    pub fn step(&mut self, generation: u64, budget: usize, cancelled: bool) -> Result<TextEditProgress, TextEditFault> {
        if let Some(fault) = self.faulted {
            return Err(fault);
        }
        if generation != self.generation {
            return Err(TextEditFault::Generation);
        }
        if budget == 0 {
            return Ok(TextEditProgress::Yield);
        }
        if cancelled {
            self.cancel_step();
            return Ok(TextEditProgress::Yield);
        }
        if self.dispose_one() {
            return Ok(TextEditProgress::Yield);
        }
        if self.retire_one_operation() {
            return Ok(TextEditProgress::Yield);
        }
        if self.active.is_none() {
            let Some(slot) = self.queue_head else { return Ok(TextEditProgress::Idle) };
            self.queue_head = self.operations[slot as usize].next;
            if self.queue_head.is_none() {
                self.queue_tail = None;
            }
            self.operations[slot as usize].next = None;
            let operation = &self.operations[slot as usize];
            self.active = Some(ActiveEdit::new(slot, self.root.lease(), operation.start.min(operation.end), operation.start.max(operation.end))?);
        }
        let progress = self.active.as_mut().expect("active edit").step(&self.operations, &self.pages)?;
        if progress != TextEditProgress::Complete {
            return Ok(progress);
        }
        let active = self.active.take().expect("completed edit");
        if self.undo.is_some() && !self.can_retire_root() {
            self.active = Some(active);
            return Ok(TextEditProgress::Yield);
        }
        let inserted = self.operations[active.operation as usize].received_bytes;
        let previous = std::mem::replace(&mut self.root, active.output);
        if let Some((older, _)) = self.undo.take() {
            self.retire_root(older);
        }
        self.undo = Some((previous, self.operations[active.operation as usize].start));
        let next_caret = active.start.saturating_add(inserted);
        self.enqueue_retirement(active.operation)?;
        Ok(TextEditProgress::Published { caret: next_caret })
    }

    pub fn undo(&mut self) -> Option<usize> {
        if !self.can_retire_root() {
            return None;
        }
        let (previous, previous_caret) = self.undo.take()?;
        let retired = std::mem::replace(&mut self.root, previous);
        self.retire_root(retired);
        Some(previous_caret)
    }

    pub fn cancel_step(&mut self) -> bool {
        if self.cancelled_active.is_none() {
            self.cancelled_active = self.active.take();
        }
        if self.dispose_one() {
            return true;
        }
        if let Some(active) = self.cancelled_active.as_mut() {
            if let Some(node) = active.stack.pop() {
                let _ = self.disposer.push(node);
                return true;
            }
            if let Some(suffix) = active.deferred_suffix.take() {
                if let Some(node) = suffix.node {
                    let _ = self.disposer.push(node);
                }
                return true;
            }
            if !active.output.is_empty() {
                let output = std::mem::take(&mut active.output);
                if let Some(node) = output.node {
                    let _ = self.disposer.push(node);
                }
                return true;
            }
            let operation = active.operation;
            self.cancelled_active = None;
            let _ = self.enqueue_retirement(operation);
            return true;
        }
        if self.retire_one_operation() {
            return true;
        }
        if let Some(slot) = self.queue_head {
            self.queue_head = self.operations[slot as usize].next;
            if self.queue_head.is_none() {
                self.queue_tail = None;
            }
            let _ = self.enqueue_retirement(slot);
            return true;
        }
        self.dispose_one()
    }

    pub fn close_step(&mut self, budget: usize) -> Result<bool, TextEditFault> {
        if budget == 0 {
            return Ok(false);
        }
        self.closing = true;
        if self.cancel_step() {
            return Ok(false);
        }
        if let Some(projection) = self.projection.as_mut() {
            if let Some(node) = projection.stack.pop() {
                let _ = self.disposer.push(node);
                return Ok(false);
            }
            self.projection = None;
            return Ok(false);
        }
        while self.close_cursor < TEXT_OPERATION_SLOTS && !self.operations[self.close_cursor].occupied {
            self.close_cursor += 1;
        }
        if self.close_cursor < TEXT_OPERATION_SLOTS {
            let slot = self.close_cursor as u8;
            if self.release_operation_one(slot) {
                self.close_cursor += 1;
            }
            return Ok(false);
        }
        if !self.root.is_empty() {
            if !self.can_retire_root() {
                return Ok(false);
            }
            let root = std::mem::take(&mut self.root);
            self.retire_root(root);
            return Ok(false);
        }
        if self.undo.is_some() {
            if !self.can_retire_root() {
                return Ok(false);
            }
            let (undo, _) = self.undo.take().expect("undo root");
            self.retire_root(undo);
            return Ok(false);
        }
        if self.dispose_one() {
            return Ok(false);
        }
        self.closed_complete = self.terminal_empty();
        Ok(self.closed_complete)
    }

    fn enqueue_retirement(&mut self, slot: u8) -> Result<(), TextEditFault> {
        let operation = self.operations.get_mut(slot as usize).ok_or(TextEditFault::Protocol)?;
        if !operation.occupied || operation.retiring {
            return Err(TextEditFault::Protocol);
        }
        if self.retirement_count == TEXT_OPERATION_SLOTS {
            return Err(TextEditFault::ItemCredits);
        }
        operation.retiring = true;
        self.retirement_slots[self.retirement_tail] = Some(slot);
        self.retirement_tail = (self.retirement_tail + 1) % TEXT_OPERATION_SLOTS;
        self.retirement_count += 1;
        Ok(())
    }

    fn retire_one_operation(&mut self) -> bool {
        if self.retirement_count == 0 {
            return false;
        }
        let slot = self.retirement_slots[self.retirement_head].expect("retirement slot");
        if self.release_operation_one(slot) {
            self.retirement_slots[self.retirement_head] = None;
            self.retirement_head = (self.retirement_head + 1) % TEXT_OPERATION_SLOTS;
            self.retirement_count -= 1;
        }
        true
    }

    fn release_operation_one(&mut self, slot: u8) -> bool {
        let operation = &mut self.operations[slot as usize];
        if let Some(page) = operation.first {
            operation.first = self.pages[page as usize].next.take();
            self.pages[page as usize].storage = None;
            self.pages[page as usize].start = 0;
            self.pages[page as usize].end = 0;
            if operation.first.is_none() {
                self.reserved_bytes = self.reserved_bytes.saturating_sub(operation.declared_bytes);
                self.operations[slot as usize] = OperationSlot::default();
                return true;
            }
        } else {
            self.reserved_bytes = self.reserved_bytes.saturating_sub(operation.declared_bytes);
            self.operations[slot as usize] = OperationSlot::default();
            return true;
        }
        false
    }

    fn can_retire_root(&self) -> bool {
        self.retired_roots.iter().any(Option::is_none)
    }

    fn retire_root(&mut self, root: TextRoot) {
        if root.is_empty() {
            return;
        }
        let slot = self.retired_roots.iter().position(Option::is_none).expect("retired root credit preflight");
        self.retired_roots[slot] = Some(root);
    }

    fn dispose_one(&mut self) -> bool {
        if self.disposer.has_capacity() {
            if let Some(slot) = self.retired_roots.iter().position(Option::is_some) {
                let root = self.retired_roots[slot].take().expect("retired root");
                if let Some(node) = root.node {
                    self.disposer.push(node).expect("retired root disposer credit");
                }
                return true;
            }
        }
        let Some(node) = self.disposer.pop() else { return false };
        if let Ok(node) = Arc::try_unwrap(node) {
            if let TextNode::Concat { left, right, .. } = node {
                let _ = self.disposer.push(left);
                let _ = self.disposer.push(right);
            }
        }
        true
    }

    fn terminal_empty(&self) -> bool {
        self.root.is_empty()
            && self.undo.is_none()
            && self.active.is_none()
            && self.cancelled_active.is_none()
            && self.projection.is_none()
            && self.queue_head.is_none()
            && self.queue_tail.is_none()
            && self.retirement_count == 0
            && self.reserved_bytes == 0
            && self.disposer.is_empty()
            && self.retired_roots.iter().all(Option::is_none)
            && self.operations.iter().all(|operation| !operation.occupied)
            && self.pages.iter().all(|page| page.storage.is_none())
    }
}

impl Drop for TextEditAuthority {
    fn drop(&mut self) {
        if self.closed_complete || self.terminal_empty() {
            return;
        }
        std::mem::forget(std::mem::take(&mut self.root));
        if let Some((root, _)) = self.undo.take() {
            std::mem::forget(root);
        }
        if let Some(active) = self.active.take() {
            std::mem::forget(active);
        }
        if let Some(active) = self.cancelled_active.take() {
            std::mem::forget(active);
        }
        if let Some(projection) = self.projection.take() {
            std::mem::forget(projection);
        }
        for page in &mut self.pages {
            if let Some(storage) = page.storage.take() {
                std::mem::forget(storage);
            }
        }
        std::mem::forget(std::mem::take(&mut self.disposer));
        for root in &mut self.retired_roots {
            if let Some(root) = root.take() {
                std::mem::forget(root);
            }
        }
    }
}

//#endregion 📥️FixedIngress

//#region ⏳️PersistentEdit

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEditProgress {
    Idle,
    Yield,
    Complete,
    Published { caret: usize },
}

struct ActiveEdit {
    operation: u8,
    start: usize,
    end: usize,
    offset: usize,
    stack: NodeStack<TEXT_ROOT_DEPTH>,
    insert_page: Option<u16>,
    inserted: bool,
    deferred_suffix: Option<TextRoot>,
    output: TextRoot,
}

impl ActiveEdit {
    fn new(operation: u8, base: TextRoot, start: usize, end: usize) -> Result<Self, TextEditFault> {
        let mut stack = NodeStack::default();
        if let Some(node) = base.node {
            stack.push(node)?;
        }
        Ok(Self { operation, start: start.min(base.bytes), end: end.min(base.bytes), offset: 0, stack, insert_page: None, inserted: false, deferred_suffix: None, output: TextRoot::default() })
    }

    fn append(&mut self, root: TextRoot) -> Result<(), TextEditFault> {
        self.output = TextRoot::concat(std::mem::take(&mut self.output), root)?;
        Ok(())
    }

    fn append_insert_page(&mut self, operations: &[OperationSlot; TEXT_OPERATION_SLOTS], pages: &[PageSlot; TEXT_PAGE_SLOTS]) -> Result<bool, TextEditFault> {
        if self.insert_page.is_none() {
            self.insert_page = operations[self.operation as usize].first;
        }
        let Some(page) = self.insert_page else {
            self.inserted = true;
            return Ok(false);
        };
        let page_slot = &pages[page as usize];
        let storage = page_slot.storage.as_ref().expect("committed page").clone();
        self.append(TextRoot::page(storage, page_slot.start, page_slot.end))?;
        self.insert_page = pages[page as usize].next;
        if self.insert_page.is_none() {
            self.inserted = true;
        }
        Ok(true)
    }

    fn step(&mut self, operations: &[OperationSlot; TEXT_OPERATION_SLOTS], pages: &[PageSlot; TEXT_PAGE_SLOTS]) -> Result<TextEditProgress, TextEditFault> {
        if !self.inserted && self.offset >= self.start {
            if self.append_insert_page(operations, pages)? {
                return Ok(TextEditProgress::Yield);
            }
        }
        if self.inserted {
            if let Some(suffix) = self.deferred_suffix.take() {
                self.append(suffix)?;
                return Ok(TextEditProgress::Yield);
            }
        }
        let Some(node) = self.stack.pop() else {
            if !self.inserted && self.append_insert_page(operations, pages)? {
                return Ok(TextEditProgress::Yield);
            }
            return Ok(TextEditProgress::Complete);
        };
        match node.as_ref() {
            TextNode::Concat { left, right, .. } => {
                self.stack.push(right.clone())?;
                self.stack.push(left.clone())?;
            }
            TextNode::Page { storage, start, end } => {
                let page_len = end - start;
                let page_start = self.offset;
                let page_end = page_start + page_len;
                if page_start < self.start {
                    let keep_end = self.start.min(page_end) - page_start + start;
                    self.append(TextRoot::page(storage.clone(), *start, keep_end))?;
                }
                if page_end > self.end {
                    let keep_start = self.end.max(page_start) - page_start + start;
                    let suffix = TextRoot::page(storage.clone(), keep_start, *end);
                    if page_start < self.start {
                        self.deferred_suffix = Some(suffix);
                    } else {
                        self.append(suffix)?;
                    }
                }
                self.offset = page_end;
            }
        }
        Ok(TextEditProgress::Yield)
    }
}

//#endregion ⏳️PersistentEdit

#[cfg(test)]
mod tests {
    use super::*;

    fn publish(authority: &mut TextEditAuthority, text: String, start: usize, end: usize) -> usize {
        authority.enqueue_owned(authority.generation(), text, start, end).unwrap();
        for _ in 0..256 {
            if let TextEditProgress::Published { caret } = authority.step(authority.generation(), 1, false).unwrap() {
                while authority.retirement_count != 0 {
                    authority.step(authority.generation(), 1, false).unwrap();
                }
                return caret;
            }
        }
        panic!("edit did not publish");
    }

    #[test]
    fn multi_megabyte_root_middle_paste_is_atomic_and_exact() {
        let mut authority = TextEditAuthority::new(TextRoot::default(), 7);
        for _ in 0..128 {
            let end = authority.len();
            publish(&mut authority, "a".repeat(TEXT_PAGE_BYTES), end, end);
        }
        for _ in 0..128 {
            let end = authority.len();
            publish(&mut authority, "b".repeat(TEXT_PAGE_BYTES), end, end);
        }
        let token = authority.begin(7, TEXT_INGRESS_BYTES, 2 * 1024 * 1024, 2 * 1024 * 1024).unwrap();
        for _ in 0..16 {
            authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
        }
        authority.commit(token).unwrap();
        let mut published = None;
        for _ in 0..64 {
            if let TextEditProgress::Published { caret } = authority.step(7, 1, false).unwrap() {
                published = Some(caret);
                break;
            }
        }
        assert_eq!(published, Some(2 * 1024 * 1024 + TEXT_INGRESS_BYTES));
        let mut projection = authority.projection(2 * 1024 * 1024, TEXT_PROJECTION_BYTES).unwrap();
        while !projection.step(1).unwrap() {}
        assert_eq!(projection.take().unwrap(), "x".repeat(TEXT_PROJECTION_BYTES));
        assert_eq!(authority.undo(), Some(2 * 1024 * 1024));
        assert_eq!(authority.root().len(), 4 * 1024 * 1024);
    }

    #[test]
    fn aggregate_credits_zero_budget_and_bounded_cancel_are_deterministic() {
        let mut authority = TextEditAuthority::new(TextRoot::default(), 1);
        let token = authority.begin(1, TEXT_INGRESS_BYTES, 0, 0).unwrap();
        assert_eq!(authority.begin(1, 1, 0, 0), Err(TextEditFault::ByteCredits));
        assert_eq!(authority.step(1, 0, false), Ok(TextEditProgress::Yield));
        for _ in 0..16 {
            authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
        }
        authority.commit(token).unwrap();
        let mut turns = 0;
        while authority.cancel_step() {
            turns += 1;
            assert!(turns <= TEXT_PAGE_SLOTS + TEXT_ROOT_DEPTH);
        }
        assert_eq!(turns, 17);
        assert_eq!(authority.reserved_bytes(), 0);
    }

    #[test]
    fn stale_generation_and_failed_page_admission_do_not_leak_credits() {
        let mut authority = TextEditAuthority::default();
        assert_eq!(authority.begin(2, 1, 0, 0), Err(TextEditFault::Generation));
        let token = authority.begin(1, 0, 0, 0).unwrap();
        for _ in 0..TEXT_PAGE_SLOTS {
            authority.push(token, String::new()).unwrap();
        }
        let before = authority.reserved_bytes();
        assert_eq!(authority.enqueue_owned(1, "z".to_string(), 0, 0), Err(TextEditFault::PageCredits));
        assert_eq!(authority.reserved_bytes(), before);
    }

    #[test]
    fn close_drains_every_owned_page_and_root_incrementally() {
        let mut authority = TextEditAuthority::default();
        let token = authority.begin(1, TEXT_INGRESS_BYTES, 0, 0).unwrap();
        for _ in 0..16 {
            authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
        }
        authority.commit(token).unwrap();
        let mut turns = 0;
        while !authority.close_step(1).unwrap() {
            turns += 1;
            assert!(turns < TEXT_PAGE_SLOTS + TEXT_ROOT_DEPTH);
        }
        assert_eq!(authority.reserved_bytes(), 0);
    }

    #[test]
    fn slot_epoch_rejects_a_late_chunk_after_reuse() {
        let mut authority = TextEditAuthority::default();
        let stale = authority.begin(1, 1, 0, 0).unwrap();
        authority.abort(stale).unwrap();
        while authority.retire_one_operation() {}
        let current = authority.begin(1, 1, 0, 0).unwrap();
        assert_eq!(stale.slot, current.slot);
        assert_ne!(stale.epoch, current.epoch);
        assert_eq!(authority.push(stale, "x".to_string()), Err(TextEditFault::Protocol));
        authority.push(current, "y".to_string()).unwrap();
    }

    #[test]
    fn unicode_boundaries_keep_projection_and_caret_valid() {
        let mut authority = TextEditAuthority::default();
        let caret = publish(&mut authority, "aé🚀z".to_string(), 0, 0);
        assert_eq!(caret, "aé🚀z".len());
        assert!(!authority.root().is_char_boundary(2).unwrap());
        assert_eq!(authority.start_projection(2, 4), Err(TextEditFault::Protocol));
        let middle = authority.root().previous_boundary("aé🚀".len()).unwrap();
        assert_eq!(middle, "aé".len());
    }

    #[test]
    fn segmented_replacement_builds_independent_bounded_pages() {
        let mut authority = TextEditAuthority::default();
        let text = "🚀".repeat(TEXT_PAGE_BYTES / 2);
        assert_eq!(authority.replace_owned(text.clone()), Err(TextEditFault::ChunkTooLarge));
        let token = authority.begin(1, text.len(), 0, 0).unwrap();
        for chunk in text.as_bytes().chunks(TEXT_PAGE_BYTES) {
            authority.push(token, std::str::from_utf8(chunk).unwrap().to_string()).unwrap();
        }
        authority.commit(token).unwrap();
        let mut turns = 0;
        loop {
            turns += 1;
            if let TextEditProgress::Published { caret } = authority.step(1, 1, false).unwrap() {
                assert_eq!(caret, text.len());
                break;
            }
            assert!(turns < 128);
        }
        assert_eq!(authority.root().materialize(), text);
    }
}
