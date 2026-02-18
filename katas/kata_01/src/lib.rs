/// Returns indices of the two numbers such that they add up to target.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
/// * `target` - Target sum.
///
/// # Returns
///
/// `Some((i, j))` if found, otherwise `None`.
///
/// # Examples
///
/// ```
/// use kata_01::two_sum;
///
/// assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((0, 1)));
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if target == nums[i] + nums[j] {
                return Some((i, j))
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn basic_case() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), Some((1, 2)));
    }

    #[test]
    fn another_case() {
        assert_eq!(two_sum(vec![3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn no_solution() {
        assert_eq!(two_sum(vec![1, 2, 3], 7), None);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(two_sum(vec![-1, -5, 5, 10], 4), Some((0, 2)));
    }

    #[test]
    fn empty_vec() {
        assert_eq!(two_sum(vec![], 1), None);
    }

    #[test]
    fn single_element() {
        assert_eq!(two_sum(vec![5], 5), None);
    }
}
