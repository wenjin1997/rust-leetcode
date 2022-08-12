# 二叉树

## 二叉树的定义

```rust
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
```

## 二叉树的递归遍历

递归三要素：

1. 确定递归函数的参数和返回值。
2. 确定递归的终止条件。
3. 确定每层递归的逻辑。

前序遍历：

```rust
pub fn preorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  // 确定终止条件
  let root = match(root) {
      Some(a) => a,
      None => return vec![],
  };

  let mut res = vec![];
  res.push(root.borrow().val);
  // Vec 的 append 方法：
  // 将其他的元素全部移动进来，并将之前的都置空
  //
  // RefCell 的 borrow 方法：
  // 不可变借用包装的值。
  //
  // Rc 的 clone 方法：
  // 增加 strong reference count。
  res.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
  res.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
  res
}
```

