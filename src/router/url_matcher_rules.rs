use crate::router::rule;
use crate::router::url_matcher;
use url::Url;

#[derive(Debug)]
pub struct UrlMatcherRules {
    rules: Vec<rule::Rule>,
}

impl UrlMatcherRules {
    pub fn new(rules: Vec<rule::Rule>) -> UrlMatcherRules {
        UrlMatcherRules { rules }
    }
}

impl url_matcher::UrlMatcher for UrlMatcherRules {
    fn match_rule(&self, url: &Url) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
        let mut matched_rules = Vec::new();
        let mut path = url.path().to_string();

        if url.query() != None {
            path = [path, "?".to_string(), url.query().unwrap().to_string()].join("");
        }

        for rule in self.rules.as_slice() {
            if rule.is_match(path.as_str())? {
                matched_rules.push(rule);
            }
        }

        return Ok(matched_rules);
    }

    fn trace(&self, url: &Url) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let rules = self.match_rule(url)?;
        let mut rules_matched = Vec::new();

        for rule in rules {
            rules_matched.push(rule.clone());
        }

        return Ok(vec![rule::RouterTraceItem {
            matches: rules_matched.len() > 0,
            prefix: "".to_string(),
            rules_evaluated: self.rules.clone(),
            rules_matches: rules_matched,
        }]);
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        let mut rules = Vec::new();

        for rule in &self.rules {
            rules.push(rule);
        }

        return rules;
    }
}
