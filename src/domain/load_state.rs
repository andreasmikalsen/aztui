pub enum LoadState<T> {
    Loading,
    Ready(T),
    Error(String),
}

impl<T> LoadState<T> {
    pub fn from_result<E: std::fmt::Display>(result: Result<T, E>) -> LoadState<T> {
        match result {
            Ok(val) => Self::Ready(val),
            Err(err) => Self::Error(err.to_string()),
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Ready(val) => Some(val),
            _ => None,
        }
    }
}