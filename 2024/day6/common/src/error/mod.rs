mod result;

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {}
