use super::{build_codes, distance_symbol, fixed_dist_lengths, fixed_lit_lengths, length_symbol, BitWriter, MAX_MATCH, MIN_MATCH};

const BL_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
const HEAP_SIZE: usize = 573;

#[derive(Clone, Copy, value_derive::ToValue, value_derive::FromValue)]
pub(super) struct Policy {
    pub window_bits: usize,
    pub memory_level: usize,
    pub good: usize,
    pub lazy: usize,
    pub nice: usize,
    pub chain: usize,
    pub finish: Finish,
}

#[derive(Clone, Copy, value_derive::ToValue, value_derive::FromValue)]
pub(super) enum Finish {
    Raw,
    Sync,
    Partial,
}

#[derive(Clone, Copy, value_derive::ToValue, value_derive::FromValue)]
enum Token {
    Literal(u8),
    Match { length: usize, distance: usize },
}

#[derive(Clone, Copy)]
enum LengthRun {
    Code(usize),
    Previous(usize),
    ZeroShort(usize),
    ZeroLong(usize),
}

struct Tree {
    lengths: Vec<u8>,
    codes: Vec<(u32, u8)>,
    max_code: usize,
}

#[derive(value_derive::ToValue, value_derive::FromValue)]
pub(super) struct Encoder {
    input: Vec<u8>,
    policy: Policy,
    writer: BitWriter,
    head: Vec<isize>,
    previous: Vec<isize>,
    tokens: Vec<Token>,
    token_limit: usize,
    block_start: usize,
    position: usize,
    lookahead: usize,
    match_length: usize,
    match_start: usize,
    match_available: bool,
    complete: bool,
}

impl Encoder {
    fn new(input: Vec<u8>, policy: Policy) -> Self {
        let window = 1usize << policy.window_bits;
        let hash = 1usize << (policy.memory_level + 7);
        let token_limit = (1usize << (policy.memory_level + 6)) - 1;
        let lookahead = input.len();
        Self {
            input,
            policy,
            writer: BitWriter::new(),
            head: vec![-1; hash],
            previous: vec![-1; window],
            tokens: Vec::with_capacity(token_limit),
            token_limit,
            block_start: 0,
            position: 0,
            lookahead,
            match_length: MIN_MATCH - 1,
            match_start: 0,
            match_available: false,
            complete: false,
        }
    }

    fn hash(&self, position: usize) -> usize {
        let bits = self.policy.memory_level + 7;
        let shift = (bits + MIN_MATCH - 1) / MIN_MATCH;
        let mask = (1usize << bits) - 1;
        let mut hash = self.input[position] as usize;
        hash = ((hash << shift) ^ self.input[position + 1] as usize) & mask;
        ((hash << shift) ^ self.input[position + 2] as usize) & mask
    }

    fn insert(&mut self, position: usize) -> isize {
        if position + MIN_MATCH > self.input.len() {
            return -1;
        }
        let hash = self.hash(position);
        let candidate = self.head[hash];
        let index = position & (self.previous.len() - 1);
        self.previous[index] = candidate;
        self.head[hash] = position as isize;
        candidate
    }

    fn longest(&self, position: usize, mut candidate: isize, previous_length: usize) -> (usize, usize) {
        let window = 1usize << self.policy.window_bits;
        let maximum_distance = window - (MAX_MATCH + MIN_MATCH + 1);
        let limit = position.saturating_sub(maximum_distance) as isize;
        let lookahead = self.input.len() - position;
        let maximum = lookahead.min(MAX_MATCH);
        if previous_length >= maximum {
            return (maximum, 0);
        }
        let nice = self.policy.nice.min(lookahead);
        let mut chain = if previous_length >= self.policy.good { self.policy.chain >> 2 } else { self.policy.chain };
        let mut best_length = previous_length;
        let mut best_position = 0;
        while candidate > limit && candidate > 0 && chain != 0 {
            let candidate_position = candidate as usize;
            if candidate_position + best_length < self.input.len()
                && self.input[candidate_position + best_length] == self.input[position + best_length]
                && self.input[candidate_position] == self.input[position]
                && self.input[candidate_position + 1] == self.input[position + 1]
            {
                let mut length = 2;
                while length < maximum && self.input[candidate_position + length] == self.input[position + length] {
                    length += 1;
                }
                if length > best_length {
                    best_length = length;
                    best_position = candidate_position;
                    if length >= nice {
                        break;
                    }
                }
            }
            candidate = self.previous[candidate_position & (self.previous.len() - 1)];
            chain -= 1;
        }
        (best_length.min(lookahead), best_position)
    }

    fn push(&mut self, token: Token, end: usize) {
        self.tokens.push(token);
        if self.tokens.len() == self.token_limit {
            self.flush_block(end, false);
        }
    }

    fn transition(&mut self) -> bool {
        if self.complete {
            return true;
        }
        if self.lookahead != 0 {
            let candidate = if self.lookahead >= MIN_MATCH { self.insert(self.position) } else { -1 };
            let previous_length = self.match_length;
            let previous_match = self.match_start;
            self.match_length = MIN_MATCH - 1;
            if candidate > 0 && previous_length < self.policy.lazy {
                (self.match_length, self.match_start) = self.longest(self.position, candidate, previous_length);
                if self.match_length == MIN_MATCH && self.position - self.match_start > 4096 {
                    self.match_length = MIN_MATCH - 1;
                }
            }
            if previous_length >= MIN_MATCH && self.match_length <= previous_length {
                self.push(Token::Match { length: previous_length, distance: self.position - 1 - previous_match }, self.position - 1 + previous_length);
                let maximum_insert = self.position + self.lookahead - MIN_MATCH;
                self.lookahead -= previous_length - 1;
                let mut remaining = previous_length - 2;
                while remaining != 0 {
                    self.position += 1;
                    if self.position <= maximum_insert {
                        self.insert(self.position);
                    }
                    remaining -= 1;
                }
                self.match_available = false;
                self.match_length = MIN_MATCH - 1;
                self.position += 1;
            } else if self.match_available {
                self.push(Token::Literal(self.input[self.position - 1]), self.position);
                self.position += 1;
                self.lookahead -= 1;
            } else {
                self.match_available = true;
                self.position += 1;
                self.lookahead -= 1;
            }
            return false;
        }
        if self.match_available {
            self.push(Token::Literal(self.input[self.position - 1]), self.position);
        }
        match self.policy.finish {
            Finish::Raw => {
                if self.tokens.is_empty() {
                    self.empty_fixed(true);
                } else {
                    self.flush_block(self.input.len(), true);
                }
            }
            Finish::Sync => {
                if !self.tokens.is_empty() {
                    self.flush_block(self.input.len(), false);
                }
                self.empty_stored(false);
                self.empty_fixed(true);
            }
            Finish::Partial => {
                if !self.tokens.is_empty() {
                    self.flush_block(self.input.len(), false);
                }
                self.empty_fixed(false);
                self.empty_fixed(true);
            }
        }
        self.complete = true;
        true
    }

    fn empty_stored(&mut self, last: bool) {
        self.writer.write_bits(u32::from(last), 3);
        self.writer.align_byte();
        self.writer.out.extend_from_slice(&[0, 0, 0xff, 0xff]);
    }

    fn empty_fixed(&mut self, last: bool) {
        self.writer.write_bits(0b010 | u32::from(last), 3);
        let codes = build_codes(&fixed_lit_lengths());
        self.writer.write_bits(codes[256].0, codes[256].1);
        if last {
            self.writer.align_byte();
        }
    }

    fn flush_block(&mut self, end: usize, last: bool) {
        let mut literal_frequencies = vec![0u32; 286];
        let mut distance_frequencies = vec![0u32; 30];
        literal_frequencies[256] = 1;
        for token in &self.tokens {
            match *token {
                Token::Literal(byte) => literal_frequencies[byte as usize] += 1,
                Token::Match { length, distance } => {
                    literal_frequencies[length_symbol(length).0] += 1;
                    distance_frequencies[distance_symbol(distance).0] += 1;
                }
            }
        }
        let literal = build_tree(&literal_frequencies, 15);
        let distance = build_tree(&distance_frequencies, 15);
        let literal_runs = length_runs(&literal.lengths, literal.max_code);
        let distance_runs = length_runs(&distance.lengths, distance.max_code);
        let mut bit_frequencies = vec![0u32; 19];
        for run in literal_runs.iter().chain(&distance_runs) {
            bit_frequencies[run_symbol(*run)] += 1;
        }
        let bit = build_tree(&bit_frequencies, 7);
        let max_bl = (3..BL_ORDER.len()).rev().find(|index| bit.lengths[BL_ORDER[*index]] != 0).unwrap_or(3);
        let dynamic_bits = 3 + 5 + 5 + 4 + 3 * (max_bl + 1) + run_cost(&literal_runs, &bit.lengths) + run_cost(&distance_runs, &bit.lengths) + token_cost(&self.tokens, &literal.lengths, &distance.lengths);
        let fixed_literal = fixed_lit_lengths();
        let fixed_distance = fixed_dist_lengths();
        let fixed_bits = 3 + token_cost(&self.tokens, &fixed_literal, &fixed_distance);
        let dynamic_bytes = (dynamic_bits + 7) >> 3;
        let fixed_bytes = (fixed_bits + 7) >> 3;
        let stored_length = end - self.block_start;
        if stored_length + 4 <= dynamic_bytes.min(fixed_bytes) && stored_length <= u16::MAX as usize {
            self.writer.write_bits(u32::from(last), 3);
            self.writer.align_byte();
            let length = stored_length as u16;
            self.writer.out.extend_from_slice(&length.to_le_bytes());
            self.writer.out.extend_from_slice(&(!length).to_le_bytes());
            self.writer.out.extend_from_slice(&self.input[self.block_start..end]);
        } else if fixed_bytes <= dynamic_bytes {
            self.writer.write_bits(0b010 | u32::from(last), 3);
            self.send_tokens(&build_codes(&fixed_literal), &build_codes(&fixed_distance));
        } else {
            self.writer.write_bits(0b100 | u32::from(last), 3);
            self.writer.write_bits((literal.max_code + 1 - 257) as u32, 5);
            self.writer.write_bits((distance.max_code + 1 - 1) as u32, 5);
            self.writer.write_bits((max_bl + 1 - 4) as u32, 4);
            for index in 0..=max_bl {
                self.writer.write_bits(bit.lengths[BL_ORDER[index]] as u32, 3);
            }
            send_runs(&mut self.writer, &literal_runs, &bit.codes);
            send_runs(&mut self.writer, &distance_runs, &bit.codes);
            self.send_tokens(&literal.codes, &distance.codes);
        }
        if last {
            self.writer.align_byte();
        }
        self.tokens.clear();
        self.block_start = end;
    }

    fn send_tokens(&mut self, literal: &[(u32, u8)], distance: &[(u32, u8)]) {
        for token in &self.tokens {
            match *token {
                Token::Literal(byte) => self.writer.write_bits(literal[byte as usize].0, literal[byte as usize].1),
                Token::Match { length, distance: value } => {
                    let (symbol, extra, bits) = length_symbol(length);
                    self.writer.write_bits(literal[symbol].0, literal[symbol].1);
                    self.writer.write_bits(extra, bits);
                    let (symbol, extra, bits) = distance_symbol(value);
                    self.writer.write_bits(distance[symbol].0, distance[symbol].1);
                    self.writer.write_bits(extra, bits);
                }
            }
        }
        self.writer.write_bits(literal[256].0, literal[256].1);
    }
}

fn token_cost(tokens: &[Token], literal: &[u8], distance: &[u8]) -> usize {
    let mut bits = literal[256] as usize;
    for token in tokens {
        bits += match *token {
            Token::Literal(byte) => literal[byte as usize] as usize,
            Token::Match { length, distance: value } => {
                let (length_symbol, _, length_extra) = length_symbol(length);
                let (distance_symbol, _, distance_extra) = distance_symbol(value);
                literal[length_symbol] as usize + length_extra as usize + distance[distance_symbol] as usize + distance_extra as usize
            }
        };
    }
    bits
}

fn build_tree(frequencies: &[u32], maximum_bits: usize) -> Tree {
    let elements = frequencies.len();
    let mut frequency = vec![0u32; elements * 2 + 1];
    frequency[..elements].copy_from_slice(frequencies);
    let mut parent = vec![0usize; frequency.len()];
    let mut depth = vec![0u8; frequency.len()];
    let mut lengths = vec![0u8; frequency.len()];
    let mut heap = vec![0usize; HEAP_SIZE];
    let mut heap_length = 0usize;
    let mut heap_maximum = HEAP_SIZE;
    let mut max_code = 0usize;
    for index in 0..elements {
        if frequency[index] != 0 {
            heap_length += 1;
            heap[heap_length] = index;
            max_code = index;
        }
    }
    while heap_length < 2 {
        let index = if max_code < 2 { max_code + 1 } else { 0 };
        max_code = max_code.max(index);
        frequency[index] = 1;
        heap_length += 1;
        heap[heap_length] = index;
    }
    for index in (1..=heap_length / 2).rev() {
        down_heap(&mut heap, heap_length, index, &frequency, &depth);
    }
    let mut node = elements;
    while heap_length >= 2 {
        let first = heap[1];
        heap[1] = heap[heap_length];
        heap_length -= 1;
        down_heap(&mut heap, heap_length, 1, &frequency, &depth);
        let second = heap[1];
        heap_maximum -= 1;
        heap[heap_maximum] = first;
        heap_maximum -= 1;
        heap[heap_maximum] = second;
        frequency[node] = frequency[first] + frequency[second];
        depth[node] = depth[first].max(depth[second]) + 1;
        parent[first] = node;
        parent[second] = node;
        heap[1] = node;
        node += 1;
        down_heap(&mut heap, heap_length, 1, &frequency, &depth);
    }
    heap_maximum -= 1;
    heap[heap_maximum] = heap[1];
    let mut counts = vec![0usize; maximum_bits + 1];
    let mut overflow = 0isize;
    for index in heap_maximum + 1..HEAP_SIZE {
        let value = heap[index];
        let mut bits = lengths[parent[value]] as usize + 1;
        if bits > maximum_bits {
            bits = maximum_bits;
            overflow += 1;
        }
        lengths[value] = bits as u8;
        if value <= max_code {
            counts[bits] += 1;
        }
    }
    if overflow > 0 {
        while overflow > 0 {
            let mut bits = maximum_bits - 1;
            while counts[bits] == 0 {
                bits -= 1;
            }
            counts[bits] -= 1;
            counts[bits + 1] += 2;
            counts[maximum_bits] -= 1;
            overflow -= 2;
        }
        let mut cursor = HEAP_SIZE;
        for bits in (1..=maximum_bits).rev() {
            let mut count = counts[bits];
            while count != 0 {
                cursor -= 1;
                let value = heap[cursor];
                if value <= max_code {
                    lengths[value] = bits as u8;
                    count -= 1;
                }
            }
        }
    }
    lengths.truncate(elements);
    let codes = build_codes(&lengths);
    Tree { lengths, codes, max_code }
}

fn down_heap(heap: &mut [usize], length: usize, mut index: usize, frequency: &[u32], depth: &[u8]) {
    let value = heap[index];
    let mut child = index << 1;
    while child <= length {
        if child < length && smaller(heap[child + 1], heap[child], frequency, depth) {
            child += 1;
        }
        if smaller(value, heap[child], frequency, depth) {
            break;
        }
        heap[index] = heap[child];
        index = child;
        child <<= 1;
    }
    heap[index] = value;
}

fn smaller(left: usize, right: usize, frequency: &[u32], depth: &[u8]) -> bool {
    frequency[left] < frequency[right] || (frequency[left] == frequency[right] && depth[left] <= depth[right])
}

fn length_runs(lengths: &[u8], max_code: usize) -> Vec<LengthRun> {
    let mut output = Vec::new();
    let mut previous = -1i16;
    let mut next = lengths[0] as i16;
    let mut count = 0usize;
    let mut maximum = if next == 0 { 138 } else { 7 };
    let mut minimum = if next == 0 { 3 } else { 4 };
    for index in 0..=max_code {
        let current = next;
        next = if index == max_code { -1 } else { lengths[index + 1] as i16 };
        count += 1;
        if count < maximum && current == next {
            continue;
        }
        if count < minimum {
            output.extend(std::iter::repeat(LengthRun::Code(current as usize)).take(count));
        } else if current != 0 {
            if current != previous {
                output.push(LengthRun::Code(current as usize));
                count -= 1;
            }
            output.push(LengthRun::Previous(count));
        } else if count <= 10 {
            output.push(LengthRun::ZeroShort(count));
        } else {
            output.push(LengthRun::ZeroLong(count));
        }
        count = 0;
        previous = current;
        if next == 0 {
            maximum = 138;
            minimum = 3;
        } else if current == next {
            maximum = 6;
            minimum = 3;
        } else {
            maximum = 7;
            minimum = 4;
        }
    }
    output
}

fn run_symbol(run: LengthRun) -> usize {
    match run {
        LengthRun::Code(code) => code,
        LengthRun::Previous(_) => 16,
        LengthRun::ZeroShort(_) => 17,
        LengthRun::ZeroLong(_) => 18,
    }
}

fn run_cost(runs: &[LengthRun], lengths: &[u8]) -> usize {
    runs.iter()
        .map(|run| {
            lengths[run_symbol(*run)] as usize
                + match run {
                    LengthRun::Code(_) => 0,
                    LengthRun::Previous(_) => 2,
                    LengthRun::ZeroShort(_) => 3,
                    LengthRun::ZeroLong(_) => 7,
                }
        })
        .sum()
}

fn send_runs(writer: &mut BitWriter, runs: &[LengthRun], codes: &[(u32, u8)]) {
    for run in runs {
        let symbol = run_symbol(*run);
        writer.write_bits(codes[symbol].0, codes[symbol].1);
        match *run {
            LengthRun::Code(_) => {}
            LengthRun::Previous(count) => writer.write_bits((count - 3) as u32, 2),
            LengthRun::ZeroShort(count) => writer.write_bits((count - 3) as u32, 3),
            LengthRun::ZeroLong(count) => writer.write_bits((count - 11) as u32, 7),
        }
    }
}

#[derive(value_derive::ToValue, value_derive::FromValue)]
pub(super) struct MinizEncoder {
    input: Vec<u8>,
    previous: Vec<isize>,
    head: Vec<isize>,
    init_position: usize,
    writer: BitWriter,
    tokens: Vec<Token>,
    code_position: usize,
    block_start: usize,
    position: usize,
    saved: Option<(usize, usize, usize)>,
    complete: bool,
}

impl MinizEncoder {
    fn new(input: Vec<u8>) -> Self {
        let previous = vec![-1; input.len()];
        Self { input, previous, head: vec![-1; 1 << 15], init_position: 0, writer: BitWriter::new(), tokens: Vec::new(), code_position: 1, block_start: 0, position: 0, saved: None, complete: false }
    }

    fn push(&mut self, token: Token) {
        let old_length = self.tokens.len();
        self.code_position += if matches!(token, Token::Literal(_)) { 1 } else { 3 };
        self.tokens.push(token);
        self.code_position += self.tokens.len() / 8 - old_length / 8;
    }

    fn transition(&mut self) -> bool {
        if self.complete {
            return true;
        }
        if self.init_position < self.input.len().saturating_sub(2) {
            let hash = miniz_hash(&self.input, self.init_position);
            self.previous[self.init_position] = self.head[hash];
            self.head[hash] = if self.init_position & 0xffff == 0 { -1 } else { self.init_position as isize };
            self.init_position += 1;
            return false;
        }
        if self.position < self.input.len() {
            let initial = self.saved.map_or(MIN_MATCH - 1, |(_, length, _)| length);
            let current = miniz_match(&self.input, &self.previous, self.position, initial);
            let mut movement = 1;
            match (self.saved.take(), current) {
                (Some((start, length, _)), Some((next_length, next_distance))) if next_length > length => {
                    self.push(Token::Literal(self.input[start]));
                    if next_length >= 128 {
                        self.push(Token::Match { length: next_length, distance: next_distance });
                        movement = next_length;
                    } else {
                        self.saved = Some((self.position, next_length, next_distance));
                    }
                }
                (Some((_, length, distance)), _) => {
                    self.push(Token::Match { length, distance });
                    movement = length - 1;
                }
                (None, Some((length, distance))) if length >= 128 => {
                    self.push(Token::Match { length, distance });
                    movement = length;
                }
                (None, Some((length, distance))) => self.saved = Some((self.position, length, distance)),
                (None, None) => self.push(Token::Literal(self.input[self.position])),
            }
            self.position = (self.position + movement).min(self.input.len());
            let total = self.position - self.block_start;
            if self.code_position > (64 * 1024) - 8 || (total > 31 * 1024 && ((self.code_position * 115) >> 7) >= total) {
                miniz_block(&mut self.writer, self.block_start, self.position, &self.tokens, false);
                self.tokens.clear();
                self.code_position = 1;
                self.block_start = self.position;
            }
            return false;
        }
        if let Some((_, length, distance)) = self.saved.take() {
            self.push(Token::Match { length, distance });
        }
        miniz_block(&mut self.writer, self.block_start, self.input.len(), &self.tokens, true);
        self.writer.align_byte();
        self.complete = true;
        true
    }
}

#[derive(value_derive::ToValue, value_derive::FromValue)]
pub(super) enum Job {
    Classic(Encoder),
    Miniz(MinizEncoder),
}

impl Job {
    pub(super) fn classic(input: Vec<u8>, policy: Policy) -> Self {
        Self::Classic(Encoder::new(input, policy))
    }

    pub(super) fn miniz(input: Vec<u8>) -> Self {
        Self::Miniz(MinizEncoder::new(input))
    }

    pub(super) fn step(&mut self) -> bool {
        match self {
            Self::Classic(state) => state.transition(),
            Self::Miniz(state) => state.transition(),
        }
    }

    pub(super) fn progress(&self) -> (usize, usize) {
        match self {
            Self::Classic(state) => (state.position, state.input.len()),
            Self::Miniz(state) if state.init_position < state.input.len().saturating_sub(2) => (state.init_position, state.input.len().saturating_mul(2)),
            Self::Miniz(state) => (state.input.len().saturating_add(state.position), state.input.len().saturating_mul(2)),
        }
    }

    pub(super) fn output(&self) -> &[u8] {
        match self {
            Self::Classic(state) => &state.writer.out,
            Self::Miniz(state) => &state.writer.out,
        }
    }

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> (bool, usize, usize) {
        let step = match self {
            Self::Classic(state) => retire_encoder_step(state, maximum_items, maximum_bytes),
            Self::Miniz(state) => retire_miniz_step(state, maximum_items, maximum_bytes),
        };
        match step {
            Some((released_items, released_bytes)) => (false, released_items, released_bytes),
            None => (true, 0, 0),
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Classic(state) => encoder_terminal_is_empty(state),
            Self::Miniz(state) => miniz_terminal_is_empty(state),
        }
    }
}

fn retire_vec_step<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    let item_bytes = std::mem::size_of::<T>();
    if !values.is_empty() {
        if maximum_items == 0 || maximum_bytes < item_bytes {
            return Some((0, 0));
        }
        drop(values.pop());
        return Some((1, item_bytes));
    }
    if values.capacity() == 0 {
        return None;
    }
    let backing_bytes = values.capacity().saturating_mul(item_bytes);
    if maximum_items == 0 || maximum_bytes < backing_bytes {
        return Some((0, 0));
    }
    drop(std::mem::take(values));
    Some((1, backing_bytes))
}

fn retire_encoder_step(state: &mut Encoder, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    retire_vec_step(&mut state.input, maximum_items, maximum_bytes)
        .or_else(|| retire_vec_step(&mut state.writer.out, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.head, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.previous, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.tokens, maximum_items, maximum_bytes))
}

fn retire_miniz_step(state: &mut MinizEncoder, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    retire_vec_step(&mut state.input, maximum_items, maximum_bytes)
        .or_else(|| retire_vec_step(&mut state.writer.out, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.head, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.previous, maximum_items, maximum_bytes))
        .or_else(|| retire_vec_step(&mut state.tokens, maximum_items, maximum_bytes))
}

fn encoder_terminal_is_empty(state: &Encoder) -> bool {
    state.input.capacity() == 0 && state.writer.out.capacity() == 0 && state.head.capacity() == 0 && state.previous.capacity() == 0 && state.tokens.capacity() == 0
}

fn miniz_terminal_is_empty(state: &MinizEncoder) -> bool {
    state.input.capacity() == 0 && state.writer.out.capacity() == 0 && state.head.capacity() == 0 && state.previous.capacity() == 0 && state.tokens.capacity() == 0
}

fn miniz_hash(input: &[u8], position: usize) -> usize {
    let first = input[position] as usize;
    let second = input[position + 1] as usize;
    let third = input[position + 2] as usize;
    ((first << 10) ^ (second << 5) ^ third) & 0x7fff
}

fn miniz_match(input: &[u8], previous: &[isize], position: usize, initial: usize) -> Option<(usize, usize)> {
    if position + MIN_MATCH > input.len() {
        return None;
    }
    let maximum = (input.len() - position).min(MAX_MATCH);
    let mut length = initial.max(1).min(maximum);
    let mut distance = 0;
    let mut candidate = previous[position];
    let mut probes = if initial < 32 { 768 } else { 192 };
    while candidate > 0 {
        if probes == 0 {
            break;
        }
        probes -= 1;
        let candidate_position = candidate as usize;
        let candidate_distance = position - candidate_position;
        if candidate_distance > 32 * 1024 {
            break;
        }
        if input[candidate_position] == input[position]
            && input[candidate_position + 1] == input[position + 1]
            && input[candidate_position + length - 1] == input[position + length - 1]
            && (length == maximum || input[candidate_position + length] == input[position + length])
        {
            let mut matched = 2;
            while matched < maximum && input[candidate_position + matched] == input[position + matched] {
                matched += 1;
            }
            if matched > length {
                length = matched;
                distance = candidate_distance;
                if matched == maximum {
                    break;
                }
            }
        }
        candidate = previous[candidate_position];
    }
    if length < MIN_MATCH || distance == 0 || (length == MIN_MATCH && distance >= 8 * 1024) {
        None
    } else {
        Some((length, distance))
    }
}

fn miniz_tree(frequencies: &[u32], maximum_bits: usize) -> Tree {
    let mut symbols: Vec<(u16, usize)> = frequencies.iter().enumerate().filter_map(|(index, frequency)| (*frequency != 0).then_some((*frequency as u16, index))).collect();
    symbols.sort_by_key(|symbol| symbol.0);
    match symbols.len() {
        0 => {}
        1 => symbols[0].0 = 1,
        count => {
            symbols[0].0 = symbols[0].0.wrapping_add(symbols[1].0);
            let mut root = 0;
            let mut leaf = 2;
            for next in 1..count - 1 {
                if leaf >= count || symbols[root].0 < symbols[leaf].0 {
                    symbols[next].0 = symbols[root].0;
                    symbols[root].0 = next as u16;
                    root += 1;
                } else {
                    symbols[next].0 = symbols[leaf].0;
                    leaf += 1;
                }
                if leaf >= count || (root < next && symbols[root].0 < symbols[leaf].0) {
                    symbols[next].0 = symbols[next].0.wrapping_add(symbols[root].0);
                    symbols[root].0 = next as u16;
                    root += 1;
                } else {
                    symbols[next].0 = symbols[next].0.wrapping_add(symbols[leaf].0);
                    leaf += 1;
                }
            }
            symbols[count - 2].0 = 0;
            for next in (0..count - 2).rev() {
                symbols[next].0 = symbols[symbols[next].0 as usize].0 + 1;
            }
            let (mut available, mut used, mut depth, mut root, mut next) = (1, 0, 0, count as i32 - 2, count as i32 - 1);
            while available > 0 {
                while root >= 0 && symbols[root as usize].0 == depth {
                    used += 1;
                    root -= 1;
                }
                while available > used {
                    symbols[next as usize].0 = depth;
                    next -= 1;
                    available -= 1;
                }
                available = 2 * used;
                depth += 1;
                used = 0;
            }
        }
    }
    let mut counts = vec![0i32; 33];
    for symbol in &symbols {
        counts[symbol.0 as usize] += 1;
    }
    if symbols.len() > 1 {
        counts[maximum_bits] += counts[maximum_bits + 1..].iter().sum::<i32>();
        let total = counts[1..=maximum_bits].iter().rev().enumerate().fold(0u32, |sum, (index, count)| sum + ((*count as u32) << index));
        for _ in (1u32 << maximum_bits)..total {
            counts[maximum_bits] -= 1;
            for bits in (1..maximum_bits).rev() {
                if counts[bits] != 0 {
                    counts[bits] -= 1;
                    counts[bits + 1] += 2;
                    break;
                }
            }
        }
    }
    let mut lengths = vec![0u8; frequencies.len()];
    let mut last = symbols.len();
    for (bits, count) in counts.iter().enumerate().take(maximum_bits + 1).skip(1) {
        let first = last - *count as usize;
        for symbol in &symbols[first..last] {
            lengths[symbol.1] = bits as u8;
        }
        last = first;
    }
    let max_code = lengths.iter().rposition(|length| *length != 0).unwrap_or(0);
    let codes = build_codes(&lengths);
    Tree { lengths, codes, max_code }
}

fn miniz_runs(lengths: &[u8]) -> Vec<LengthRun> {
    let mut output = Vec::new();
    let (mut zeros, mut repeats, mut previous) = (0usize, 0usize, u8::MAX);
    let flush_repeats = |output: &mut Vec<LengthRun>, repeats: &mut usize, previous: u8| {
        if *repeats != 0 {
            if *repeats < 3 {
                output.extend(std::iter::repeat(LengthRun::Code(previous as usize)).take(*repeats));
            } else {
                output.push(LengthRun::Previous(*repeats));
            }
            *repeats = 0;
        }
    };
    let flush_zeros = |output: &mut Vec<LengthRun>, zeros: &mut usize| {
        if *zeros != 0 {
            if *zeros < 3 {
                output.extend(std::iter::repeat(LengthRun::Code(0)).take(*zeros));
            } else if *zeros <= 10 {
                output.push(LengthRun::ZeroShort(*zeros));
            } else {
                output.push(LengthRun::ZeroLong(*zeros));
            }
            *zeros = 0;
        }
    };
    for length in lengths {
        if *length == 0 {
            flush_repeats(&mut output, &mut repeats, previous);
            zeros += 1;
            if zeros == 138 {
                flush_zeros(&mut output, &mut zeros);
            }
        } else {
            flush_zeros(&mut output, &mut zeros);
            if *length != previous {
                flush_repeats(&mut output, &mut repeats, previous);
                output.push(LengthRun::Code(*length as usize));
            } else {
                repeats += 1;
                if repeats == 6 {
                    flush_repeats(&mut output, &mut repeats, previous);
                }
            }
        }
        previous = *length;
    }
    if repeats != 0 {
        flush_repeats(&mut output, &mut repeats, previous);
    } else {
        flush_zeros(&mut output, &mut zeros);
    }
    output
}

fn miniz_block(writer: &mut BitWriter, start: usize, end: usize, tokens: &[Token], last: bool) {
    let mut literal_frequencies = vec![0u32; 286];
    let mut distance_frequencies = vec![0u32; 30];
    literal_frequencies[256] = 1;
    for token in tokens {
        match *token {
            Token::Literal(byte) => literal_frequencies[byte as usize] += 1,
            Token::Match { length, distance } => {
                literal_frequencies[length_symbol(length).0] += 1;
                distance_frequencies[distance_symbol(distance).0] += 1;
            }
        }
    }
    if end - start < 48 {
        writer.write_bits(0b010 | u32::from(last), 3);
        miniz_send_tokens(writer, tokens, &build_codes(&fixed_lit_lengths()), &build_codes(&fixed_dist_lengths()));
        if last {
            writer.align_byte();
        }
        return;
    }
    let literal = miniz_tree(&literal_frequencies, 15);
    let distance = miniz_tree(&distance_frequencies, 15);
    let literal_count = literal.max_code.max(256) + 1;
    let distance_count = distance.max_code.max(0) + 1;
    let mut lengths = literal.lengths[..literal_count].to_vec();
    lengths.extend_from_slice(&distance.lengths[..distance_count]);
    let runs = miniz_runs(&lengths);
    let mut bit_frequencies = vec![0u32; 19];
    for run in &runs {
        bit_frequencies[run_symbol(*run)] += 1;
    }
    let bit = miniz_tree(&bit_frequencies, 7);
    let bit_count = BL_ORDER.iter().rposition(|symbol| bit.lengths[*symbol] != 0).map_or(4, |index| (index + 1).max(4));
    writer.write_bits(0b100 | u32::from(last), 3);
    writer.write_bits((literal_count - 257) as u32, 5);
    writer.write_bits((distance_count - 1) as u32, 5);
    writer.write_bits((bit_count - 4) as u32, 4);
    for symbol in &BL_ORDER[..bit_count] {
        writer.write_bits(bit.lengths[*symbol] as u32, 3);
    }
    send_runs(writer, &runs, &bit.codes);
    miniz_send_tokens(writer, tokens, &literal.codes, &distance.codes);
    if last {
        writer.align_byte();
    }
}

fn miniz_send_tokens(writer: &mut BitWriter, tokens: &[Token], literal: &[(u32, u8)], distance: &[(u32, u8)]) {
    for token in tokens {
        match *token {
            Token::Literal(byte) => writer.write_bits(literal[byte as usize].0, literal[byte as usize].1),
            Token::Match { length, distance: value } => {
                let (symbol, extra, bits) = length_symbol(length);
                writer.write_bits(literal[symbol].0, literal[symbol].1);
                writer.write_bits(extra, bits);
                let (symbol, extra, bits) = distance_symbol(value);
                writer.write_bits(distance[symbol].0, distance[symbol].1);
                writer.write_bits(extra, bits);
            }
        }
    }
    writer.write_bits(literal[256].0, literal[256].1);
}
