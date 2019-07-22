use super::path::Path;

pub enum Route {
    Tasks,
    TaskNew,
    PathNotFound(String),
}

impl Route {
    pub fn to_path(&self) -> Path {
        let path_segments = match self {
            Route::Tasks => vec!["tasks".into()],
            Route::TaskNew => vec!["tasks".into(), "new".into()],
            Route::PathNotFound(_) => vec!["path_not_fount".into()],
        };
        Path {
            path_segments,
            query: None,
            fragment: None,
        }
    }
}