//! 📅️ The language-neutral date picker model: ISO civil dates, the locale's display format and week start, the six-week month grid, the keyboard moves and the popup geometry of a `date` input.

use semio_ui_theme::geometry::Rect;

/// 📆️ A civil date; the stored value of a `date` input is its `YYYY-MM-DD` text, as the DOM `<input type="date">` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CivilDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

impl CivilDate {
    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        ((1..=9999).contains(&year) && (1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month)).then_some(Self { year, month, day })
    }

    /// 📥️ The DOM value: exactly `YYYY-MM-DD`; anything else (including the empty text of a cleared field) is no date.
    pub fn parse_iso(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let digits = |range: std::ops::Range<usize>| text.get(range).filter(|part| part.bytes().all(|byte| byte.is_ascii_digit()))?.parse::<i32>().ok();
        Self::new(digits(0..4)?, u8::try_from(digits(5..7)?).ok()?, u8::try_from(digits(8..10)?).ok()?)
    }

    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// 🧮️ Days since 1970-01-01 (proleptic Gregorian).
    pub fn epoch_day(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let shifted_month = i64::from((self.month + 9) % 12);
        let day_of_year = (153 * shifted_month + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    pub fn from_epoch_day(days: i64) -> Option<Self> {
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted - era * 146_097;
        let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = u8::try_from(day_of_year - (153 * shifted_month + 2) / 5 + 1).ok()?;
        let month = u8::try_from(if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 }).ok()?;
        Self::new(i32::try_from(year_of_era + era * 400 + i64::from(month <= 2)).ok()?, month, day)
    }

    /// 🗓️ Monday is `0`, Sunday `6`.
    pub fn weekday_from_monday(self) -> u8 {
        (self.epoch_day() + 3).rem_euclid(7) as u8
    }

    pub fn plus_days(self, days: i64) -> Option<Self> {
        Self::from_epoch_day(self.epoch_day() + days)
    }

    /// 📅️ `months` later with the day clamped into the target month, as the native picker's month and year steps.
    pub fn plus_months(self, months: i32) -> Option<Self> {
        let index = self.year * 12 + i32::from(self.month) - 1 + months;
        let (year, month) = (index.div_euclid(12), u8::try_from(index.rem_euclid(12) + 1).ok()?);
        Self::new(year, month, self.day.min(days_in_month(year, month)))
    }
}

/// 🌐️ What a locale decides about a date field: its week start, display pattern and names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateLocale {
    pub id: &'static str,
    pub week_starts_monday: bool,
    pub day_first: bool,
    pub separator: char,
    pub months: [&'static str; 12],
    pub weekdays_short: [&'static str; 7],
}

const EN: DateLocale = DateLocale {
    id: "en",
    week_starts_monday: false,
    day_first: false,
    separator: '/',
    months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
    weekdays_short: ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
};

const DE: DateLocale = DateLocale {
    id: "de",
    week_starts_monday: true,
    day_first: true,
    separator: '.',
    months: ["Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember"],
    weekdays_short: ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
};

/// 🌐️ The date locale of a locale id; `None` for a locale the repo has no names for — there is no default language.
pub fn date_locale(id: &str) -> Option<&'static DateLocale> {
    match id {
        "en" => Some(&EN),
        "de" => Some(&DE),
        _ => None,
    }
}

impl DateLocale {
    /// 🖨️ The field text the native input shows: `MM/DD/YYYY` for en, `DD.MM.YYYY` for de.
    pub fn format(&self, date: CivilDate) -> String {
        let separator = self.separator;
        if self.day_first {
            format!("{:02}{separator}{:02}{separator}{:04}", date.day, date.month, date.year)
        } else {
            format!("{:02}{separator}{:02}{separator}{:04}", date.month, date.day, date.year)
        }
    }

    pub fn month_title(&self, year: i32, month: u8) -> String {
        format!("{} {year}", self.months[usize::from(month.clamp(1, 12)) - 1])
    }

    /// 🗓️ The weekday header cells, first weekday of the locale first.
    pub fn weekday_header(&self) -> [&'static str; 7] {
        std::array::from_fn(|index| self.weekdays_short[(index + usize::from(!self.week_starts_monday)) % 7])
    }
}

pub const DATE_GRID_ROWS: usize = 6;
pub const DATE_GRID_COLUMNS: usize = 7;

/// 🗓️ The 42 cells of the month shown around `shown`: whole weeks from the locale's week start, the month's own days and the neighbours' that fill the weeks.
pub fn month_grid(year: i32, month: u8, week_starts_monday: bool) -> Option<[CivilDate; DATE_GRID_ROWS * DATE_GRID_COLUMNS]> {
    let first = CivilDate::new(year, month, 1)?;
    let lead = i64::from(if week_starts_monday { first.weekday_from_monday() } else { (first.weekday_from_monday() + 1) % 7 });
    let start = first.epoch_day() - lead;
    let mut cells = [first; DATE_GRID_ROWS * DATE_GRID_COLUMNS];
    for (index, cell) in cells.iter_mut().enumerate() {
        *cell = CivilDate::from_epoch_day(start + index as i64)?;
    }
    Some(cells)
}

/// ⌨️ What a key does to an open date popup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateKey {
    Focus(CivilDate),
    Select(CivilDate),
    Close,
}

/// ⌨️ The native picker's grid keys: arrows one day or week, PageUp/PageDown one month (Shift: one year), Home/End the week's first and last day, Enter and Space pick the focused day, Escape closes.
pub fn date_key(key: &str, shift: bool, focused: CivilDate, week_starts_monday: bool) -> Option<DateKey> {
    let column = i64::from(if week_starts_monday { focused.weekday_from_monday() } else { (focused.weekday_from_monday() + 1) % 7 });
    Some(match key {
        "ArrowLeft" => DateKey::Focus(focused.plus_days(-1)?),
        "ArrowRight" => DateKey::Focus(focused.plus_days(1)?),
        "ArrowUp" => DateKey::Focus(focused.plus_days(-7)?),
        "ArrowDown" => DateKey::Focus(focused.plus_days(7)?),
        "PageUp" => DateKey::Focus(focused.plus_months(if shift { -12 } else { -1 })?),
        "PageDown" => DateKey::Focus(focused.plus_months(if shift { 12 } else { 1 })?),
        "Home" => DateKey::Focus(focused.plus_days(-column)?),
        "End" => DateKey::Focus(focused.plus_days(6 - column)?),
        "Enter" | " " => DateKey::Select(focused),
        "Escape" => DateKey::Close,
        _ => return None,
    })
}

/// 📐️ Where a popup's parts sit; every rect is absolute.
#[derive(Clone, Debug, PartialEq)]
pub struct DatePopupLayout {
    pub panel: Rect,
    pub title: Rect,
    pub previous: Rect,
    pub next: Rect,
    pub weekdays: [Rect; DATE_GRID_COLUMNS],
    pub cells: [Rect; DATE_GRID_ROWS * DATE_GRID_COLUMNS],
}

/// 📐️ The popup below `anchor` (above it when the viewport has no room), `cell` square cells inside a `pad` frame; `rtl` mirrors the columns.
pub fn date_popup_layout(anchor: Rect, viewport_h: f32, cell: f32, pad: f32, rtl: bool) -> DatePopupLayout {
    let width = cell * DATE_GRID_COLUMNS as f32 + pad * 2.0;
    let height = cell * (DATE_GRID_ROWS as f32 + 2.0) + pad * 2.0;
    let below = anchor.y + anchor.h;
    let y = if below + height <= viewport_h || anchor.y < height { below } else { anchor.y - height };
    let panel = Rect::new(anchor.x, y, width, height);
    let column_x = |column: usize| panel.x + pad + (if rtl { DATE_GRID_COLUMNS - 1 - column } else { column }) as f32 * cell;
    let row_y = |row: usize| panel.y + pad + row as f32 * cell;
    let (previous, next) = if rtl { (Rect::new(panel.x + panel.w - pad - cell, row_y(0), cell, cell), Rect::new(panel.x + pad, row_y(0), cell, cell)) } else { (Rect::new(panel.x + pad, row_y(0), cell, cell), Rect::new(panel.x + panel.w - pad - cell, row_y(0), cell, cell)) };
    DatePopupLayout {
        panel,
        title: Rect::new(panel.x + pad + cell, row_y(0), panel.w - pad * 2.0 - cell * 2.0, cell),
        previous,
        next,
        weekdays: std::array::from_fn(|column| Rect::new(column_x(column), row_y(1), cell, cell)),
        cells: std::array::from_fn(|index| Rect::new(column_x(index % DATE_GRID_COLUMNS), row_y(2 + index / DATE_GRID_COLUMNS), cell, cell)),
    }
}

static TODAY_EPOCH_DAY: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(i64::MIN);

/// 🕰️ The host clock port of the date picker: the shell publishes today (UTC days since 1970-01-01, from its wall clock — `app_now_ms`) once per frame, so a page build, which has no `std` clock, reads the same host clock as a native one.
pub fn set_today_epoch_day(day: i64) {
    TODAY_EPOCH_DAY.store(day, std::sync::atomic::Ordering::Relaxed);
}

/// 🕰️ Today as published by the host, else (native only) the system clock; `None` while a page host has not published one — no date is ever invented.
pub fn today_epoch_day() -> Option<i64> {
    let published = TODAY_EPOCH_DAY.load(std::sync::atomic::Ordering::Relaxed);
    if published != i64::MIN {
        return Some(published);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|elapsed| (elapsed.as_secs() / 86_400) as i64)
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// 🗓️ The input kinds that open a popup picker.
pub fn is_picker_kind(input_kind: &str) -> bool {
    matches!(input_kind, "date" | "color")
}

/// 📅️ The date a popup is centred on: the keyboard-focused day, else the field's value, else today.
pub fn focus_date(value: &str, picker_day: Option<i64>) -> Option<CivilDate> {
    picker_day.and_then(CivilDate::from_epoch_day).or_else(|| CivilDate::parse_iso(value)).or_else(|| today_epoch_day().and_then(CivilDate::from_epoch_day))
}

/// ⌨️ A typed date field's text: the stored `YYYY-MM-DD`, or the host language's field format as the field shows it (`MM/DD/YYYY` with `/`, `DD.MM.YYYY` with `.`), read to the date the DOM input would hold.
pub fn parse_typed(text: &str) -> Option<CivilDate> {
    if let Some(date) = CivilDate::parse_iso(text) {
        return Some(date);
    }
    let separator = if text.contains('.') { '.' } else { '/' };
    let mut parts = text.split(separator);
    let (first, second, year) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() || year.len() != 4 || ![first, second, year].iter().all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())) {
        return None;
    }
    let (first, second, year) = (first.parse::<u8>().ok()?, second.parse::<u8>().ok()?, year.parse::<i32>().ok()?);
    if separator == '.' { CivilDate::new(year, second, first) } else { CivilDate::new(year, first, second) }
}
