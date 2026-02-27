#![allow(unused)]


fn main() {
    // array
    let arr: [u32; 3] = [1, 2, 3];
    // arr[0];
    // arr[1];
    // arr[2];
    let mut arr: [u32;3] = [1,2,3];
    arr[1] = 9;

    let arr: [u32; 10] = [0;10];
    // slice
    let nums: [i32; 10] = [-1, 1, -2,2,-3,3,-4,4,5,-5];

    let s: &[i32] = &nums[0..3];
}