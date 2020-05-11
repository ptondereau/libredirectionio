use crate::regex_radix_tree::{NodeItem, Storage, Trace as NodeTrace};
use crate::router::{RequestMatcher, Route, RouteData, Trace};
use http::Request;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MatcherTreeStorage<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone> {
    pub matcher: M,
    regex: String,
    phantom_route: PhantomData<T>,
    phantom_item: PhantomData<S>,
}

pub trait ItemRoute<T: RouteData>: NodeItem {
    fn route(self) -> Route<T>;
}

impl<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone + 'static> Storage<S> for MatcherTreeStorage<T, S, M> {
    fn push(&mut self, item: S) {
        self.matcher.insert(item.route());
    }

    fn remove(&mut self, id: &str) -> bool {
        self.matcher.remove(id)
    }

    fn len(&self) -> usize {
        self.matcher.len()
    }

    fn is_empty(&self) -> bool {
        self.matcher.is_empty()
    }

    fn new(regex: &str) -> Self {
        MatcherTreeStorage {
            matcher: M::default(),
            regex: regex.to_string(),
            phantom_route: PhantomData,
            phantom_item: PhantomData,
        }
    }
}

impl<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone + 'static> MatcherTreeStorage<T, S, M> {
    pub fn node_trace_to_router_trace(trace: NodeTrace<S, Self>, request: &Request<()>) -> Trace<T> {
        let mut children = Vec::new();

        for child in trace.children {
            children.push(Self::node_trace_to_router_trace(child, request));
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
}