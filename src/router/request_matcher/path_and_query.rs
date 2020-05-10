use crate::regex_radix_tree::{RegexRadixTree, Trace as NodeTrace, NodeItem};
use crate::router::marker_string::StaticOrDynamic;
use crate::router::request_matcher::matcher_tree_storage::{MatcherTreeStorage, ItemRoute};
use crate::router::request_matcher::{RequestMatcher, RouteMatcher};
use crate::router::{Route, RouteData, Trace};
use http::Request;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::{BTreeMap, HashMap};
use url::form_urlencoded::parse as parse_query;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Debug, Clone)]
pub struct PathAndQueryRegexNodeItem<T: RouteData> {
    route: Route<T>,
    path_regex: String,
}

impl<T: RouteData> NodeItem for PathAndQueryRegexNodeItem<T> {
    fn regex(&self) -> &str {
        self.path_regex.as_str()
    }
}
impl<T: RouteData> ItemRoute<T> for PathAndQueryRegexNodeItem<T> {
    fn route(self) -> Route<T> {
        self.route
    }
}

type PathAndQueryRegexTreeMatcher<T> = MatcherTreeStorage<T, PathAndQueryRegexNodeItem<T>, RouteMatcher<T>>;

#[derive(Debug, Clone)]
pub struct PathAndQueryMatcher<T: RouteData> {
    regex_tree_rule: RegexRadixTree<PathAndQueryRegexNodeItem<T>, PathAndQueryRegexTreeMatcher<T>>,
    static_rules: HashMap<String, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for PathAndQueryMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.path_and_query() {
            StaticOrDynamic::Static(path) => {
                if !self.static_rules.contains_key(path) {
                    self.static_rules.insert(path.clone(), PathAndQueryMatcher::create_sub_matcher());
                }

                self.static_rules.get_mut(path).unwrap().insert(route);
            }
            StaticOrDynamic::Dynamic(path) => {
                let item = PathAndQueryRegexNodeItem {
                    path_regex: path.regex.clone(),
                    route,
                };

                self.regex_tree_rule.insert(item)
            }
        }
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut routes = Vec::new();

        self.regex_tree_rule.remove(id);

        self.static_rules.retain(|_, matcher| {
            routes.extend(matcher.remove(id));

            matcher.len() > 0
        });

        self.count = self.static_rules.len() + self.regex_tree_rule.len();

        routes
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let path = PathAndQueryMatcher::<T>::request_to_path(request);
        let storages = self.regex_tree_rule.find(path.as_str());
        let mut routes = Vec::new();

        for storage in storages {
            routes.extend(storage.matcher.match_request(request));
        }

        match self.static_rules.get(path.as_str()) {
            None => (),
            Some(static_matcher) => {
                routes.extend(static_matcher.match_request(request));
            }
        }

        routes
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let path = PathAndQueryMatcher::<T>::request_to_path(request);
        let node_trace = self.regex_tree_rule.trace(path.as_str());
        let mut traces = vec![PathAndQueryMatcher::<T>::node_trace_to_router_trace(node_trace, request)];

        let static_traces = match self.static_rules.get(path.as_str()) {
            None => Vec::new(),
            Some(static_matcher) => static_matcher.trace(request),
        };

        traces.push(Trace::new(
            "Static path".to_string(),
            !static_traces.is_empty(),
            true,
            self.static_rules.len() as u64,
            static_traces,
            Vec::new(),
        ));

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.regex_tree_rule.cache(limit, level);

        for matcher in self.static_rules.values_mut() {
            new_limit = matcher.cache(new_limit, level)
        }

        new_limit
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T: RouteData> Default for PathAndQueryMatcher<T> {
    fn default() -> Self {
        PathAndQueryMatcher {
            regex_tree_rule: RegexRadixTree::default(),
            static_rules: HashMap::new(),
            count: 0,
        }
    }
}

impl<T: RouteData> PathAndQueryMatcher<T> {
    pub fn request_to_path(request: &Request<()>) -> String {
        let mut path = request.uri().path().to_string();

        if let Some(query) = request.uri().query() {
            match PathAndQueryMatcher::<T>::build_sorted_query(query) {
                None => (),
                Some(query_sorted) => {
                    path = [path.as_str(), "?", query_sorted.as_str()].join("");
                }
            }
        }

        path
    }

    pub fn build_sorted_query(query: &str) -> Option<String> {
        let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();

        let mut query_string = "".to_string();

        for (key, value) in &hash_query {
            query_string.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

            if !value.is_empty() {
                query_string.push_str("=");
                query_string.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
            }

            query_string.push_str("&");
        }

        query_string.pop();

        if query_string.is_empty() {
            return None;
        }

        Some(query_string)
    }

    fn node_trace_to_router_trace(trace: NodeTrace<PathAndQueryRegexNodeItem<T>, PathAndQueryRegexTreeMatcher<T>>, request: &Request<()>) -> Trace<T> {
        let mut children = Vec::new();

        for child in trace.children {
            children.push(PathAndQueryMatcher::<T>::node_trace_to_router_trace(child, request));
        }

        if let Some(storage) = trace.storage.as_ref() {
            children.extend(storage.matcher.trace(request));
        }

        Trace::new(
            format!("Regex tree prefix {}", trace.regex),
            trace.matched,
            true,
            trace.count,
            children,
            Vec::new(),
        )
    }

    fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(RouteMatcher::default())
    }
}
