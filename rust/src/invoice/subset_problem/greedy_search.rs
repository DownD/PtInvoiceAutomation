use crate::invoice::subset_problem::SubsetSolver;



pub struct GreedySearchSolver;


impl SubsetSolver for GreedySearchSolver{
    fn solve(&self, numbers: &[i64], target: i64) -> Vec<i64> {
        let mut sorted_numbers = numbers.to_vec();
        sorted_numbers.sort(); // Sort the numbers in ascending order

        let mut best_sum = i64::MAX;
        let mut closest_subset: Vec<i64> = Vec::new();

        // Iterate through the sorted numbers
        for i in 0..sorted_numbers.len() {
            let mut current_sum = 0;
            let mut current_subset: Vec<i64> = Vec::new();

            // Calculate the sum of the current subset
            for j in i..sorted_numbers.len() {
                current_sum += sorted_numbers[j];
                current_subset.push(sorted_numbers[j]);

                // Update the best sum if the current sum is closer to the target
                if (current_sum - target).abs() < (best_sum - target).abs() {
                    best_sum = current_sum;
                    closest_subset = current_subset.clone();
                }

                // If the current sum is greater than or equal to the target, stop adding numbers
                if current_sum >= target {
                    break;
                }
            }
        }

        // Return the closest subset sum if it exists
        if best_sum != i64::MAX {
            closest_subset
        } else {
            Vec::<i64>::new()
        }
    }
}