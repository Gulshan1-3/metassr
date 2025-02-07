use std::collections::HashMap;

/// Represents a matched route with parameters
#[derive(Debug)]
pub struct RouteMatch {
    pub params: HashMap<String, String>,
}

/// Simulated router context storing multiple route matches
pub struct RouterContext {
    pub matches: Vec<RouteMatch>,
}

/// Standalone function version of `use_params`
pub fn use_params(context: &RouterContext) -> HashMap<String, String> {
    if let Some(last_match) = context.matches.last() {
        if last_match.params.is_empty() {
            return HashMap::new(); // Return an empty owned HashMap
        }
        return last_match.params.clone(); // Return a cloned HashMap
    }
    HashMap::new() // Return an empty owned HashMap if no match exists
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_use_params_with_no_matches() {
        let context = RouterContext { matches: vec![] };
        let params = use_params(&context);
        assert!(params.is_empty(), "Expected empty HashMap for no matches");
    }

    #[test]
    fn test_use_params_with_empty_params() {
        let context = RouterContext {
            matches: vec![RouteMatch { params: HashMap::new() }],
        };
        let params = use_params(&context);
        assert!(params.is_empty(), "Expected empty HashMap for empty params");
    }

    #[test]
    fn test_use_params_with_non_empty_params() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), "42".to_string());
        let context = RouterContext {
            matches: vec![RouteMatch { params }],
        };

        let extracted_params = use_params(&context);
        assert_eq!(extracted_params.len(), 1, "Expected exactly one parameter");
        assert_eq!(
            extracted_params.get("id"),
            Some(&"42".to_string()),
            "Expected 'id' parameter with value '42'"
        );
    }

    #[test]
    fn test_use_params_with_multiple_matches() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), "1".to_string());

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), "2".to_string());

        let context = RouterContext {
            matches: vec![
                RouteMatch { params: params1 },
                RouteMatch { params: params2 },
            ],
        };

        let extracted_params = use_params(&context);
        assert_eq!(extracted_params.len(), 1, "Expected exactly one parameter");
        assert_eq!(
            extracted_params.get("id"),
            Some(&"2".to_string()),
            "Expected 'id' parameter with value '2' from the last match"
        );
    }

    #[test]
    fn test_use_params_with_different_keys() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Alice".to_string());

        let context = RouterContext {
            matches: vec![RouteMatch { params }],
        };

        let extracted_params = use_params(&context);
        assert_eq!(extracted_params.len(), 1, "Expected exactly one parameter");
        assert_eq!(
            extracted_params.get("name"),
            Some(&"Alice".to_string()),
            "Expected 'name' parameter with value 'Alice'"
        );
    }
}
