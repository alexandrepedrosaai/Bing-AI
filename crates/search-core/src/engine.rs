pub struct SearchEngine;

impl SearchEngine {

    pub fn new() -> Self {

        Self
    }

    pub fn search(
        &self,
        query: &str
    ) -> Vec<String> {

        vec![
            format!(
                "document => {}",
                query
            )
        ]
    }
}
