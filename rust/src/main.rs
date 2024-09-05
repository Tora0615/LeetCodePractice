mod q_0001_two_sum;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = q_0001_two_sum::Solution::two_sum(nums, target);
    println!("{:?}", result); // Output should be [0, 1]
}

/* --- usage ---
mod $filename;
$result = $filename::Solution::$function($arg);
*/
