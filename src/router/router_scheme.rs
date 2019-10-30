use crate::router;
use core::borrow::BorrowMut;
use url::Url;

#[derive(Debug)]
pub struct RouterScheme {
    http_router: router::router_host::RouterHost,
    https_router: router::router_host::RouterHost,
    any_scheme_router: router::router_host::RouterHost,
}

impl RouterScheme {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterScheme, Box<dyn std::error::Error>> {
        let mut http_rules = Vec::new();
        let mut https_rules = Vec::new();
        let mut any_scheme_rules = Vec::new();

        for rule in rules {
            let scheme = rule.source.scheme.clone();

            match scheme {
                None => {
                    any_scheme_rules.push(rule);
                }
                Some(string) => match string.as_str() {
                    "https" => https_rules.push(rule),
                    "http" => http_rules.push(rule),
                    _ => any_scheme_rules.push(rule),
                },
            }
        }

        Ok(RouterScheme {
            http_router: router::router_host::RouterHost::new(http_rules)?,
            https_router: router::router_host::RouterHost::new(https_rules)?,
            any_scheme_router: router::router_host::RouterHost::new(any_scheme_rules)?,
        })
    }
}

impl router::Router for RouterScheme {
    fn match_rule(&self, url: Url) -> Result<Vec<&router::rule::Rule>, Box<dyn std::error::Error>> {
        let mut rules_found = Vec::new();

        rules_found.append(self.any_scheme_router.match_rule(url.clone())?.borrow_mut());

        if url.scheme() == "http" {
            rules_found.append(self.http_router.match_rule(url.clone())?.borrow_mut());
        }

        if url.scheme() == "https" {
            rules_found.append(self.https_router.match_rule(url.clone())?.borrow_mut());
        }

        Ok(rules_found)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        let mut new_cache_limit = cache_limit;

        new_cache_limit = self.https_router.build_cache(new_cache_limit, level);
        new_cache_limit = self.http_router.build_cache(new_cache_limit, level);

        self.any_scheme_router.build_cache(new_cache_limit, level)
    }

    fn trace(
        &self,
        url: Url,
    ) -> Result<Vec<router::rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut traces = Vec::new();

        traces.push(router::rule::RouterTraceItem {
            rules_matches: Vec::new(),
            rules_evaluated: Vec::new(),
            matches: true,
            prefix: "://".to_string(),
        });

        traces.append(self.any_scheme_router.trace(url.clone())?.borrow_mut());

        if url.scheme() == "http" {
            traces.push(router::rule::RouterTraceItem {
                rules_matches: Vec::new(),
                rules_evaluated: Vec::new(),
                matches: true,
                prefix: "http://".to_string(),
            });

            traces.append(self.http_router.trace(url.clone())?.borrow_mut());
        }

        if url.scheme() == "https" {
            traces.push(router::rule::RouterTraceItem {
                rules_matches: Vec::new(),
                rules_evaluated: Vec::new(),
                matches: true,
                prefix: "https://".to_string(),
            });

            traces.append(self.https_router.trace(url.clone())?.borrow_mut());
        }

        Ok(traces)
    }
}
