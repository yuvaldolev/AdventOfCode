#[derive(Debug)]
pub struct SafetyManualUpdates {
    page_ordering_rules: Vec<(u32, u32)>,
    pages_to_produce_per_update: Vec<Vec<u32>>,
}

impl SafetyManualUpdates {
    pub fn new(
        page_ordering_rules: Vec<(u32, u32)>,
        pages_to_produce_per_update: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            page_ordering_rules,
            pages_to_produce_per_update,
        }
    }

    pub fn get_page_ordering_rules(&self) -> &Vec<(u32, u32)> {
        &self.page_ordering_rules
    }

    pub fn get_pages_to_produce_per_update(&self) -> &[Vec<u32>] {
        &self.pages_to_produce_per_update
    }
}
