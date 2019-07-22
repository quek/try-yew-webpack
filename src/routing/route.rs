use super::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Route {
    Tasks,
    TaskNew,
    PathNotFound(String),
}

impl Route {
    pub fn from_path(path: &Path) -> Self {
        // FIXME きれいにして
        if let Some(first_segment) = path.path_segments.get(0) {
            match first_segment.as_str() {
                "tasks" => {
                    if let Some(second_segment) = path.path_segments.get(1) {
                        match second_segment.as_str() {
                            "new" => Route::TaskNew,
                            other => Route::PathNotFound(other.into()),
                        }
                    } else {
                        Route::Tasks
                    }
                }
                other => Route::PathNotFound(other.into()),
            }
        } else {
            Route::PathNotFound("path_not_fount".into())
        }
    }

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