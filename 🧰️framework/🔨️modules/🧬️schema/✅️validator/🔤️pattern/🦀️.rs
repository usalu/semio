//! 🔤️ Owned backtracking matcher for the ECMA-262 `pattern` subset.

//#region 🔤️Pattern

/// 🔣️ Owned backtracking matcher for the ECMA-262 subset JSON Schema `pattern` needs — anchors,
/// classes, groups, lookahead, word boundaries, alternation and quantifiers over Unicode code points.
///
/// Not reusing `semio-framework-math`'s `🎯️sampling` DFA: that engine is byte-level with no anchors
/// and no `\d`/`\w`/`\s` shorthand, exposes an `async` per-byte `step`, and would invert the layering
/// by making the boundary schema module depend on the sampling crate.
/// See <https://262.ecma-international.org/#sec-patterns>.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct PatternMatcher {
    node: PatternNode,
}

#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
enum PatternNode {
    Empty,
    Literal(u32),
    Any,
    Class { negated: bool, items: Vec<ClassItem> },
    Start,
    Boundary(bool),
    End,
    Concat(Vec<PatternNode>),
    Alternate(Vec<PatternNode>),
    Repeat(Box<Repeat>),
    Look { negated: bool, node: Box<PatternNode> },
}

#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
struct Repeat {
    node: PatternNode,
    min: usize,
    max: Option<usize>,
    greedy: bool,
}

#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
enum ClassItem {
    Literal(u32),
    Range(u32, u32),
    Digit(bool),
    Word(bool),
    Space(bool),
}

impl PatternMatcher {
    /// 📥 Compiles one `pattern` string, rejecting constructs outside the supported subset.
    pub fn compile(pattern: &str) -> Result<Self, String> {
        let chars: Vec<char> = pattern.chars().collect();
        let mut parser = PatternParser { chars: &chars, position: 0 };
        let node = parser.parse_alternate()?;
        if parser.position != chars.len() {
            return Err(format!("unbalanced pattern at offset {}", parser.position));
        }
        Ok(Self { node })
    }

    /// 🔎 Whether the pattern matches anywhere in `text`, the unanchored JSON Schema semantics.
    pub fn is_match(&self, text: &str) -> bool {
        let input: Vec<char> = text.chars().collect();
        (0..=input.len()).any(|start| match_node(&self.node, &input, start, &mut |_| true))
    }
}

struct PatternParser<'a> {
    chars: &'a [char],
    position: usize,
}

impl PatternParser<'_> {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn parse_alternate(&mut self) -> Result<PatternNode, String> {
        let mut branches = vec![self.parse_concat()?];
        while self.peek() == Some('|') {
            self.position += 1;
            branches.push(self.parse_concat()?);
        }
        Ok(if branches.len() == 1 { branches.remove(0) } else { PatternNode::Alternate(branches) })
    }

    fn parse_concat(&mut self) -> Result<PatternNode, String> {
        let mut nodes = Vec::new();
        while !matches!(self.peek(), None | Some('|') | Some(')')) {
            nodes.push(self.parse_quantified()?);
        }
        Ok(match nodes.len() {
            0 => PatternNode::Empty,
            1 => nodes.remove(0),
            _ => PatternNode::Concat(nodes),
        })
    }

    fn parse_quantified(&mut self) -> Result<PatternNode, String> {
        let node = self.parse_atom()?;
        let (min, max) = match self.peek() {
            Some('*') => {
                self.position += 1;
                (0, None)
            }
            Some('+') => {
                self.position += 1;
                (1, None)
            }
            Some('?') => {
                self.position += 1;
                (0, Some(1))
            }
            Some('{') => match self.parse_bounds()? {
                Some(bounds) => bounds,
                None => return Ok(node),
            },
            _ => return Ok(node),
        };
        if matches!(node, PatternNode::Boundary(_)) {
            return Err("word boundary cannot be quantified".to_string());
        }
        let greedy = if self.peek() == Some('?') {
            self.position += 1;
            false
        } else {
            true
        };
        Ok(PatternNode::Repeat(Box::new(Repeat { node, min, max, greedy })))
    }

    fn parse_bounds(&mut self) -> Result<Option<(usize, Option<usize>)>, String> {
        let open = self.position;
        let Some(close) = self.chars[open..].iter().position(|entry| *entry == '}').map(|offset| open + offset) else {
            return Ok(None);
        };
        let body: String = self.chars[open + 1..close].iter().collect();
        if body.is_empty() || !body.chars().all(|entry| entry.is_ascii_digit() || entry == ',') {
            return Ok(None);
        }
        let bounds = match body.split_once(',') {
            None => {
                let exact = body.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?;
                (exact, Some(exact))
            }
            Some((minimum, "")) => (minimum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?, None),
            Some((minimum, maximum)) => (minimum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?, Some(maximum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?)),
        };
        if bounds.1.is_some_and(|maximum| maximum < bounds.0) {
            return Err(format!("descending repeat bound `{body}`"));
        }
        self.position = close + 1;
        Ok(Some(bounds))
    }

    fn parse_atom(&mut self) -> Result<PatternNode, String> {
        let Some(current) = self.peek() else { return Err("unexpected end of pattern".to_string()) };
        self.position += 1;
        match current {
            '^' => Ok(PatternNode::Start),
            '$' => Ok(PatternNode::End),
            '.' => Ok(PatternNode::Any),
            '(' => {
                let look = match (self.peek(), self.chars.get(self.position + 1)) {
                    (Some('?'), Some(':')) => {
                        self.position += 2;
                        None
                    }
                    (Some('?'), Some('=')) => {
                        self.position += 2;
                        Some(false)
                    }
                    (Some('?'), Some('!')) => {
                        self.position += 2;
                        Some(true)
                    }
                    (Some('?'), _) => return Err("lookbehind and named groups are outside the supported subset".to_string()),
                    _ => None,
                };
                let inner = self.parse_alternate()?;
                if self.peek() != Some(')') {
                    return Err("unclosed group".to_string());
                }
                self.position += 1;
                Ok(match look {
                    Some(negated) => PatternNode::Look { negated, node: Box::new(inner) },
                    None => inner,
                })
            }
            '[' => self.parse_class(),
            '\\' => self.parse_escape(false),
            ')' | ']' => Err(format!("unbalanced `{current}`")),
            '*' | '+' | '?' => Err(format!("quantifier `{current}` has nothing to repeat")),
            literal => Ok(PatternNode::Literal(literal as u32)),
        }
    }

    fn parse_class(&mut self) -> Result<PatternNode, String> {
        let negated = if self.peek() == Some('^') {
            self.position += 1;
            true
        } else {
            false
        };
        let mut items = Vec::new();
        loop {
            let Some(current) = self.peek() else { return Err("unclosed character class".to_string()) };
            self.position += 1;
            if current == ']' {
                break;
            }
            let item = if current == '\\' {
                match self.parse_escape(true)? {
                    PatternNode::Literal(literal) => ClassItem::Literal(literal),
                    PatternNode::Class { negated: shorthand_negated, items: shorthand } => {
                        items.extend(shorthand.into_iter().map(|item| match (item, shorthand_negated) {
                            (ClassItem::Digit(polarity), true) => ClassItem::Digit(!polarity),
                            (ClassItem::Word(polarity), true) => ClassItem::Word(!polarity),
                            (ClassItem::Space(polarity), true) => ClassItem::Space(!polarity),
                            (item, _) => item,
                        }));
                        continue;
                    }
                    _ => return Err("unsupported escape inside a character class".to_string()),
                }
            } else {
                ClassItem::Literal(current as u32)
            };
            let ClassItem::Literal(low) = item else {
                items.push(item);
                continue;
            };
            if self.peek() == Some('-') && self.chars.get(self.position + 1).is_some_and(|next| *next != ']') {
                self.position += 1;
                let Some(high) = self.peek() else { return Err("unclosed character class".to_string()) };
                self.position += 1;
                let high = if high == '\\' {
                    match self.parse_escape(true)? {
                        PatternNode::Literal(literal) => literal,
                        _ => return Err("unsupported escape as a class range bound".to_string()),
                    }
                } else {
                    high as u32
                };
                if high < low {
                    return Err(format!("descending class range `U+{low:X}-U+{high:X}`"));
                }
                items.push(ClassItem::Range(low, high));
                continue;
            }
            items.push(ClassItem::Literal(low));
        }
        Ok(PatternNode::Class { negated, items })
    }

    fn parse_escape(&mut self, in_class: bool) -> Result<PatternNode, String> {
        let Some(current) = self.peek() else { return Err("dangling escape".to_string()) };
        self.position += 1;
        let shorthand = |item: ClassItem| Ok(PatternNode::Class { negated: false, items: vec![item] });
        match current {
            'd' => shorthand(ClassItem::Digit(true)),
            'D' => shorthand(ClassItem::Digit(false)),
            'w' => shorthand(ClassItem::Word(true)),
            'W' => shorthand(ClassItem::Word(false)),
            's' => shorthand(ClassItem::Space(true)),
            'S' => shorthand(ClassItem::Space(false)),
            'n' => Ok(PatternNode::Literal('\n' as u32)),
            'r' => Ok(PatternNode::Literal('\r' as u32)),
            't' => Ok(PatternNode::Literal('\t' as u32)),
            'f' => Ok(PatternNode::Literal('\u{c}' as u32)),
            'v' => Ok(PatternNode::Literal('\u{b}' as u32)),
            '0' => Ok(PatternNode::Literal('\0' as u32)),
            'x' => self.parse_code_point(2),
            'u' => self.parse_code_point(4),
            digit if digit.is_ascii_digit() => Err("backreferences are outside the supported subset".to_string()),
            'b' if in_class => Ok(PatternNode::Literal(8)),
            'b' => Ok(PatternNode::Boundary(true)),
            'B' if !in_class => Ok(PatternNode::Boundary(false)),
            'B' => Err("invalid character class escape".to_string()),
            literal => Ok(PatternNode::Literal(literal as u32)),
        }
    }

    fn parse_code_point(&mut self, width: usize) -> Result<PatternNode, String> {
        let end = self.position + width;
        if end > self.chars.len() {
            return Err("truncated code point escape".to_string());
        }
        let digits: String = self.chars[self.position..end].iter().collect();
        let code = u32::from_str_radix(&digits, 16).map_err(|_| format!("invalid code point escape `{digits}`"))?;
        self.position = end;
        if (0xD800..=0xDBFF).contains(&code) && self.chars.get(self.position) == Some(&'\\') && self.chars.get(self.position + 1) == Some(&'u') {
            if let Some(digits) = self.chars.get(self.position + 2..self.position + 6) {
                let digits: String = digits.iter().collect();
                if let Ok(low) = u32::from_str_radix(&digits, 16) {
                    if (0xDC00..=0xDFFF).contains(&low) {
                        self.position += 6;
                        return Ok(PatternNode::Literal(0x10000 + ((code - 0xD800) << 10) + low - 0xDC00));
                    }
                }
            }
        }
        Ok(PatternNode::Literal(code))
    }
}

fn is_word(value: &char) -> bool {
    value.is_ascii_alphanumeric() || *value == '_'
}

/// ␠️ ECMA-262 whitespace and line terminators used by pattern character classes.
/// @see <https://tc39.es/ecma262/#sec-runtime-semantics-compiletocharset>
fn is_space(value: char) -> bool {
    matches!(value, '\u{9}'..='\u{d}' | '\u{20}' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

fn class_item_matches(item: ClassItem, value: char) -> bool {
    match item {
        ClassItem::Literal(literal) => literal == value as u32,
        ClassItem::Range(low, high) => low <= value as u32 && value as u32 <= high,
        ClassItem::Digit(polarity) => value.is_ascii_digit() == polarity,
        ClassItem::Word(polarity) => is_word(&value) == polarity,
        ClassItem::Space(polarity) => is_space(value) == polarity,
    }
}

fn match_node(node: &PatternNode, input: &[char], position: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    match node {
        PatternNode::Empty => next(position),
        PatternNode::Literal(literal) => input.get(position).is_some_and(|value| *value as u32 == *literal) && next(position + 1),
        PatternNode::Any => input.get(position).is_some_and(|value| *value != '\n') && next(position + 1),
        PatternNode::Class { negated, items } => input.get(position).is_some_and(|value| items.iter().any(|item| class_item_matches(*item, *value)) != *negated) && next(position + 1),
        PatternNode::Boundary(positive) => {
            let left = position.checked_sub(1).and_then(|index| input.get(index)).is_some_and(is_word);
            let right = input.get(position).is_some_and(is_word);
            (left != right) == *positive && next(position)
        }
        PatternNode::Start => position == 0 && next(position),
        PatternNode::End => position == input.len() && next(position),
        PatternNode::Concat(nodes) => match_concat(nodes, input, position, next),
        PatternNode::Alternate(branches) => branches.iter().any(|branch| match_node(branch, input, position, next)),
        PatternNode::Repeat(repeat) => match_repeat(repeat, input, position, 0, next),
        PatternNode::Look { negated, node } => match_node(node, input, position, &mut |_| true) != *negated && next(position),
    }
}

fn match_concat(nodes: &[PatternNode], input: &[char], position: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    match nodes.split_first() {
        None => next(position),
        Some((head, rest)) => {
            let mut continuation = |resumed: usize| match_concat(rest, input, resumed, next);
            match_node(head, input, position, &mut continuation)
        }
    }
}

fn match_repeat(repeat: &Repeat, input: &[char], position: usize, count: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    let can_stop = count >= repeat.min;
    let can_repeat = match repeat.max {
        Some(maximum) => count < maximum,
        None => true,
    };
    if !repeat.greedy && can_stop && next(position) {
        return true;
    }
    if can_repeat {
        let mut continuation = |resumed: usize| {
            if resumed == position {
                return count + 1 >= repeat.min && next(position);
            }
            match_repeat(repeat, input, resumed, count + 1, next)
        };
        if match_node(&repeat.node, input, position, &mut continuation) {
            return true;
        }
    }
    repeat.greedy && can_stop && next(position)
}

//#endregion 🔤️Pattern
