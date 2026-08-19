//! 🎖️ Trust-ladder branded types after strict lex/validation.

//#region 🔖️Trust
/// @emoji 🛂️ A value that has passed [`self::lex`] in strict mode. Constructible only within
/// this crate/its trusted callers — public API never lets a caller wrap arbitrary text as
/// `Sanitized` without going through the real check.
#[derive(Clone, Debug)]
pub struct Sanitized<T>(T);

impl<T> Sanitized<T> {
    pub(crate) async fn new_trusted(value: T) -> Self {
        Self(value)
    }

    pub async fn into_inner(self) -> T {
        self.0
    }

    pub async fn get(&self) -> &T {
        &self.0
    }
}

/// @emoji 🛂️ A value that has additionally passed schema validation. Reserved for the
/// `dsl_schema` layer to construct.
#[derive(Clone, Debug)]
pub struct SchemaValid<T>(T);

impl<T> SchemaValid<T> {
    pub async fn new_trusted(value: T) -> Self {
        Self(value)
    }

    pub async fn into_inner(self) -> T {
        self.0
    }

    pub async fn get(&self) -> &T {
        &self.0
    }
}
//#endregion 🔖️Trust
