/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }

        let mut map = HashMap::new();
        // 统计每个数字出现的次数
        nums.into_iter().for_each(|x| {
            let k = map.entry(x).or_insert(0);
            // 由于要构建小根堆，这里出现的次数越多，对应的 value 越小
            *k -= 1;
        });

        // BinaryHeap 最大值始终会冒泡到队列前端
        // 这里 BinaryHeap 创建的容量为 k + 1 ，如果为 k 的话，那么 heap 的
        // 长度始终都不会超过 k，就不能 pop 元素了，因此应该要先多放进一个元素，
        // 接着再 pop 出去出现次数小的元素。
        let mut heap = BinaryHeap::with_capacity(k as usize + 1);
        
        map.into_iter().for_each(|x| {
            heap.push((x.1, x.0));

            if heap.len() > k as usize {
                // heap.pop() 函数从堆中移除并返回最大值。
                // 也就是移除那些出现次数少的元素。
                heap.pop();
            }
        });

        heap.into_iter().map(|x| x.1).collect::<Vec<i32>>()

    }
}
// @lc code=end

