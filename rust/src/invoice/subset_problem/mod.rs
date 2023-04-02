use std::collections::HashMap;

pub mod greedy_search;

pub trait SubsetSolver {
    fn solve(&self, numbers: &[i64], target_sum: i64) -> Vec<i64>;

    fn solve_vector<'a,T>(&self, elements: &'a[&T], target_sum : i64, value_get:fn(&'a T) -> i64) -> Vec<&'a T> {
        let numbers: Vec<i64> = elements.iter().map(|x| value_get(x)).collect();
        let mut elements_map: HashMap<i64, Vec<&'a T>> = HashMap::new();

        for element in elements{
            elements_map.entry(value_get(element)).or_insert_with(|| Vec::with_capacity(1)).push(element);
        }

        //Get first elemnt in map of vec and remove it
        let subset : Vec<i64> = self.solve(&numbers, target_sum);
        return subset.iter().map(move |x| elements_map.get_mut(x).unwrap().pop().unwrap()).collect();
    }
}