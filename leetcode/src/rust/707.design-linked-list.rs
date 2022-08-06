/*
 * @lc app=leetcode id=707 lang=rust
 *
 * [707] Design Linked List
 */

// @lc code=start
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    /** 在这里初始化数据结构 */
    fn new() -> Self {
        Default::default()
    }
    
    /** 得到链表中的第 index 个节点。 如果 index 无效，返回 -1。 */
    /// O(n)
    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            // ref 和 & 的区别，ref 借用匹配值的元素，& 匹配引用。
            Some(ref a) => a,
            None => { return -1; },
        };
        let mut idx_cur = 0;
        println!("idx_cur: {}", idx_cur);
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            };
        }
        cur.val
    }
    
    /// O(1)
    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }))
    }
    
    /// O(n)
    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(Node { val, next: None}));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(Box::new(Node { val, next: None }));
    }
    
    /// O(n)
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            }
            idx += 1;
        }
        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));
        self.head = dummy_head.next;
    }
    
    /** 如果 index 是有效的，在链表中删除第 index 个节点。 */
    /// O(n)
    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
// @lc code=end

