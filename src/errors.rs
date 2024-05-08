pub type Result<T, E = ReplayError> = ::std::result::Result<T, E>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ReplayError {
    #[error("sequence number {0} is duplicated")]
    Duplicated(usize),
    #[error("sequence number {0} is outside the window")]
    OutsideWindow(usize),
}
