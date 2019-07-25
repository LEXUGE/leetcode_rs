// Copyright 2019 Harry Ying
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

/// An empty struct which is implemented with the functions which are solutions.
pub struct Solution;

impl Solution {
    /// Solution of the problem [Two Sum](https://leetcode.com/problems/two-sum/solution/).  
    /// Written by Harry Ying on Jun, 25th, 2019.  
    /// The solution follows the simple brute search approach as it is described on LeetCode solution page.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, v) in nums.iter().enumerate() {
            for j in i + 1..nums.len() {
                if v + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        unreachable!();
    }
}
