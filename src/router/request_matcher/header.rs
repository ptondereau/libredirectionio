use crate::router::request_matcher::{PathAndQueryMatcher, RequestMatcher};
use crate::router::{Route, RouteData, StaticOrDynamic, Trace};
use http::Request;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct HeaderMatcher<T: RouteData> {
    any_header: Box<dyn RequestMatcher<T>>,
    conditions: BTreeSet<HeaderCondition>,
    condition_groups: BTreeMap<BTreeSet<HeaderCondition>, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum ValueCondition {
    Static(String),
    Regex(String),
    NotExist,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeaderCondition {
    header_name: String,
    condition: ValueCondition,
}

impl<T: RouteData> RequestMatcher<T> for HeaderMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        if route.headers().is_empty() {
            self.any_header.insert(route);

            return;
        }

        let mut condition_group = BTreeSet::new();

        for header in route.headers() {
            let condition = match &header.value {
                None => ValueCondition::NotExist,
                Some(value) => match value {
                    StaticOrDynamic::Static(string) => ValueCondition::Static(string.clone()),
                    StaticOrDynamic::Dynamic(dynamic) => ValueCondition::Regex(dynamic.regex.clone()),
                },
            };

            let header_condition = HeaderCondition {
                header_name: header.name.to_lowercase(),
                condition,
            };

            condition_group.insert(header_condition.clone());
            self.conditions.insert(header_condition);
        }

        if !self.condition_groups.contains_key(&condition_group) {
            self.condition_groups.insert(condition_group.clone(), Self::create_sub_matcher());
        }

        let matcher = self.condition_groups.get_mut(&condition_group).unwrap();

        matcher.insert(route)
    }

    fn remove(&mut self, id: &str) -> bool {
        for matcher in self.condition_groups.values_mut() {
            if matcher.remove(id) {
                self.count -= 1;

                return true;
            }
        }

        self.any_header.remove(id)
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let mut rules = self.any_header.match_request(request);
        let mut execute_conditions = BTreeMap::new();

        'group: for (conditions, matcher) in &self.condition_groups {
            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        let result = condition.condition.match_value(request, condition.header_name.as_str());

                        // Save result
                        execute_conditions.insert(condition.clone(), result);

                        if !result {
                            continue 'group;
                        }
                    }
                    Some(result) => {
                        if !result {
                            continue 'group;
                        }
                    }
                }
            }

            rules.extend(matcher.match_request(request));
        }

        rules
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let mut traces = self.any_header.trace(request);
        let mut execute_conditions = BTreeMap::new();

        for (conditions, matcher) in &self.condition_groups {
            let mut matched = true;

            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        matched = matched && condition.condition.match_value(request, condition.header_name.as_str());

                        // Save result
                        execute_conditions.insert(condition.clone(), matched);

                        traces.push(Trace::new(
                            format!("Header condition on {}: {}", condition.header_name, condition.condition.format()),
                            matched,
                            true,
                            0,
                            Vec::new(),
                            Vec::new(),
                        ));

                        if !matched {
                            break;
                        }
                    }
                    Some(result) => {
                        matched = matched && *result;

                        if !matched {
                            break;
                        }
                    }
                }
            }

            if matched {
                traces.push(Trace::new(
                    "Header condition group result".to_string(),
                    matched,
                    true,
                    0,
                    matcher.trace(request),
                    Vec::new(),
                ));
            } else {
                traces.push(Trace::new(
                    "Header condition group result".to_string(),
                    matched,
                    true,
                    0,
                    Vec::new(),
                    Vec::new(),
                ));
            }
        }

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = limit;

        for matcher in self.condition_groups.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_header.cache(new_limit, level)
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

impl<T: RouteData> Default for HeaderMatcher<T> {
    fn default() -> Self {
        HeaderMatcher {
            any_header: HeaderMatcher::create_sub_matcher(),
            conditions: BTreeSet::new(),
            condition_groups: BTreeMap::new(),
            count: 0,
        }
    }
}

impl<T: RouteData> HeaderMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(PathAndQueryMatcher::default())
    }
}

impl ValueCondition {
    pub fn match_value(&self, request: &Request<()>, name: &str) -> bool {
        match self {
            ValueCondition::NotExist => !request.headers().contains_key(name),
            ValueCondition::Static(static_string) => {
                let values = request.headers().get_all(name);
                let mut result = false;

                for value in values {
                    result = result || value == static_string;
                }

                result
            }
            ValueCondition::Regex(regex_string) => match Regex::new(regex_string.as_str()) {
                Err(_) => false,
                Ok(regex) => {
                    let values = request.headers().get_all(name);
                    let mut result = false;

                    for header_value in values {
                        match header_value.to_str() {
                            Err(_) => continue,
                            Ok(header_value_str) => result = result || regex.is_match(header_value_str),
                        }
                    }

                    result
                }
            },
        }
    }

    pub fn format(&self) -> String {
        match self {
            ValueCondition::NotExist => "Not existing header".to_string(),
            ValueCondition::Static(static_string) => format!("Match value {}", static_string),
            ValueCondition::Regex(regex_string) => format!("Match regex {}", regex_string),
        }
    }
}
