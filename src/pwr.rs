/// Implemented for all low-power modes
pub trait PowerMode {
    /// Enters the low-power mode
    fn enter(&mut self);
}