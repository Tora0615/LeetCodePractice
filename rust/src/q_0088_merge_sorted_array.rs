/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    /*
    v1.0.0
    59/59 cases passed (1 ms)
    Your runtime beats 64.68 % of rust submissions
    Your memory usage beats 47.21 % of rust submissions (2.2 MB)
    */
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        /* Case of there have empty array */
        if n == 0 {
            // num2 array empty -> num1 don't need to touch
            return;
        }
        if m == 0 {
            // num1 array empty -> copy num2 to num1
            *nums1 = nums2.to_vec();
            return;
        }

        /* Normal case */
        let mut ans: Vec<i32> = vec![];
        let mut m_read_index = 0;
        let mut n_read_index = 0;
        for _i in 0..m + n {
            /* Upper bond check and some handle */
            if n_read_index >= n as usize {
                // if m_read_index large then the num2.len(), push remain num1
                ans.push(nums1[m_read_index]);
                m_read_index += 1;
                continue;
            }

            if m_read_index >= m as usize {
                // if n_read_index large then the num1.len(), push remain num2
                ans.push(nums2[n_read_index]);
                n_read_index += 1;
                continue;
            }

            /* Middle normal case */
            // push the small one
            if nums1[m_read_index] > nums2[n_read_index] {
                ans.push(nums2[n_read_index]);
                n_read_index += 1;
            } else {
                ans.push(nums1[m_read_index]);
                m_read_index += 1;
            }
        }
        // copy the vec to num1
        *nums1 = ans;
    }
}
// @lc code=end

/* ---- Example test case ----
mod q_0088_merge_sorted_array;

fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    let n = 3;

    q_0088_merge_sorted_array::Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("Merged array: {:?}", nums1);
}
*/
