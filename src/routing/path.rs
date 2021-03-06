use super::route_service::RouteService;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub path_segments: Vec<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Path {
    pub fn to_route_string(&self) -> String {
        let path = self.path_segments.join("/");
        let mut path = format!("/{}", path);
        if let Some(ref query) = self.query {
            path = format!("{}?{}", path, query);
        }
        if let Some(ref fragment) = self.fragment {
            path = format!("{}#{}", path, fragment);
        }
        path
    }

    pub fn current(route_service: &RouteService) -> Self {
        let path = route_service.get_path();
        let mut path_segments: Vec<String> = path.split("/").map(String::from).collect();
        path_segments.remove(0);

        let mut query: String = route_service.get_query();
        let query: Option<String> = if query.len() > 1 {
            query.remove(0);
            Some(query)
        } else {
            None
        };

        let mut fragment: String = route_service.get_fragment();
        let fragment: Option<String> = if fragment.len() > 1 {
            fragment.remove(0);
            Some(fragment)
        } else {
            None
        };

        Self {
            path_segments,
            query,
            fragment,
        }
    }
}
