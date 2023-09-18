// TASK: create a custom filtering function that allows 
// filtering elements from a given collection based on a specific condition

fn main() {
    let sample_vec = vec!["aaa", "baa", "ddd", "daada", "zap"];
    let filter_string = FilterCondition { filter_string: "aa".to_string() };
    let result =  custom_filter(sample_vec, filter_string);

    println!("Filtered Results: {:?}", result);
}

struct FilterCondition {
    filter_string: String
}

impl FilterCondition {
    fn is_match(&mut self, filter_input: &str) -> bool{
        filter_input.contains(&self.filter_string)
    }
}

fn custom_filter(collection: Vec<&str>, mut filter_condition: FilterCondition ) -> Vec<&str>{
    let col = collection
                .into_iter() 
                .filter(|&item| filter_condition.is_match(item))
                .collect();
    col
}