mod q_0088_merge_sorted_array;

fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    let n = 3;

    // let mut nums1: Vec<i32> = vec![2, 0];
    // let m = 1;
    // let mut nums2: Vec<i32> = vec![1];
    // let n = 1;

    q_0088_merge_sorted_array::Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("Merged array: {:?}", nums1);
}

/* --- usage ---
mod $filename;
$result = $filename::Solution::$function($arg);
*/
