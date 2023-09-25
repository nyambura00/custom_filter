fn main() {
    let sample_vec = vec!["aaa", "baa", "ddd", "daada", "zap"];
    let filter_object = CustomFilter { col: &sample_vec, filter_value: "aa" };
    let result = custom_filter(&filter_object);

    println!("Filtered Results: {:?}", result);
}

struct CustomFilter<'a, T> {
    col: &'a Vec<T>,
    filter_value: &'a str,
}

impl<'a, T> CustomFilter<'a, T> {
    fn is_match(&self, data: &T) -> bool
    where
        T: AsRef<str>,
    {
        data.as_ref().contains(self.filter_value)
    }
}

fn custom_filter<'a, T>(filter_condition: &CustomFilter<'a, T>) -> Vec<&'a T>
where
    T: AsRef<str>,
{
    filter_condition
        .col
        .iter()
        .filter(|&item| filter_condition.is_match(item))
        .collect()
}