pub use color_eyre::Report as EffyError;

pub type EffyResult<T> = Result<T, EffyError>;

pub use color_eyre::eyre::anyhow as err;
pub use color_eyre::eyre::bail;
pub use color_eyre::eyre::ensure;
