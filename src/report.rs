/// Trait for types that can generate a report
pub trait Reporter {
    /// Generates a text report about the state of the object
    fn report(&self) -> String;
}
