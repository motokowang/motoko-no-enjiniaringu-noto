// 35. Search Insert Position
//
// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// 
// You must write an algorithm with O(log n) runtime complexity.


use::std::cmp::Ordering;

fn main() {
    let n: Vec<i32> = vec![1,2,5,10];
    let t: i32 = 5;

    println!("Index is {}", search_insert(n, t));
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l:i32 = 0;
    let mut r:i32 = (nums.len() as i32) - 1;
    let mut i:i32 = r / 2 as i32;

    while l <= r {
        match nums[i as usize].cmp(&target) {
            Ordering::Equal => return i,            // Target found; return index of target
            Ordering::Less => l = i + 1,            // Too small; set l to be 1 more than current index i
            Ordering::Greater => r = i - 1,         // Too big; set r to be 1 less than current index i
        }
        i = (l + r) / 2 as i32;                     // Centre index i to be roughly midpoint of l & r
    }
    return l;                                       // Return l, because here, target was not found, and l will always be equal to max(l,r) as
                                                    // while condition has been broken
}