/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    /*
        v1.0.3
        Your runtime beats 100 % of rust submissions
        Your memory usage beats 40.22 % of rust submissions (2.4 MB)

        * decrease the key query times
    */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new(); // create hash map

        for index in 0..nums.len() {
            let element = nums[index];
            match map.get(&element) {
                // get info by element(key) from hashmap,
                // it will return Some(if exist key) / None(No matched key)
                Some(&old_index) => {
                    // if matched, parse the value and save as old_index
                    return vec![old_index as i32, index as i32]; // uint to i32 and pack as vec to return
                }
                None => {
                    // if don't have matched one, add to_find to map,
                    // then compare it with the element at next round
                    let to_find: i32 = target - element;
                    map.insert(to_find, index);
                }
            }
        }
        // default return, but should never go to here
        return vec![0, 0];
    }
}
// @lc code=end

/* ----- Historical solutions --- */
/*
    v1.0.2
    Your runtime beats 86.88 % of rust submissions
    Your memory usage beats 8.51 % of rust submissions (2.6 MB)
*/
#[allow(dead_code)]
pub fn two_sum_v1_0_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new(); // create hash map

    for (index, element) in nums.iter().enumerate() {
        // iter nums (Vec<i32>),
        // get the index(by iter) and element(by enumerate)
        match map.get(element) {
            // get info by element(key) from hashmap,
            // it will return Some(if exist key) / None(No matched key)
            Some(old_index) => {
                // if matched, parse the value into old_index
                return vec![*old_index as i32, index as i32]; // uint to i32 and pack as vec to return
            }
            None => {
                // if don't have matched one, add to_find to map,
                // then compare it with the element at next round
                let to_find: i32 = target - element;
                map.insert(to_find, index);
            }
        }
    }
    // default return, but should never go to here
    return vec![0, 0];
}

/* ---- Example test case ----
mod two_sum;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum::Solution::two_sum(nums, target);
    println!("{:?}", result); // Output should be [0, 1]
}
*/
