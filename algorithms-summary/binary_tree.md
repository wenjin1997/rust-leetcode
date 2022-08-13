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

中序遍历：

```rust
/// 中序遍历
/// 
/// 递归法
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let root = match root {
      Some(a) => a,
      None => return vec![],
  };
  let mut res = vec![];
  res.append(&mut Self::inorder_traversal(root.borrow().left.clone()));
  res.push(root.borrow().val);
  res.append(&mut Self::inorder_traversal(root.borrow().right.clone()));
  res
}
```

后序遍历：

```rust
/// 后序遍历
/// 
/// 递归法
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let root = match root {
    Some(a) => a,
    None => return vec![],
  };

  let mut res = vec![];
  res.append(&mut Self::postorder_traversal(root.borrow().left.clone()));
  res.append(&mut Self::postorder_traversal(root.borrow().right.clone()));
  res.push(root.borrow().val);

  res
}
```


## 二叉树的迭代遍历

前序遍历：

```rust
/// 迭代方法
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let root = match root {
      Some(a) => a,
      None => return vec![],
  };

  let mut res = vec![];
  let mut stack = vec![root];
  while let Some(node) = stack.pop() {
      res.push(node.borrow().val);
      if let Some(ref r) = node.borrow().right {
          stack.push(r.clone());
      }
      if let Some(ref l) = node.borrow().left {
          stack.push(l.clone());
      }
  }
  res
}
```

中序遍历：

```rust
/// 迭代法
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut node = root;
  let mut res: Vec<i32> = vec![];
  let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

  while !stack.is_empty() || node.is_some() {
      while let Some(n) = node {
          node = n.borrow_mut().left.take();
          stack.push(Some(n));
      }

      if let Some(Some(n)) = stack.pop() {
          res.push(n.borrow().val);
          node = n.borrow_mut().right.take();
      }
  }
  res
}
```

后序遍历：

```rust
/// 迭代法
/// 
/// 先是 中 -> 右 -> 左，再对结果反转。
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let root = match root {
    Some(a) => a,
    None => return vec![],
  };

  let mut res = vec![];
  let mut stack = vec![root];
  // 先前序遍历，不过顺序是 中 -> 右 -> 左
  // 因此入栈顺序是先左后右
  while let Some(node) = stack.pop() {
    res.push(node.borrow().val);
    
    if let Some(ref l) = node.borrow().left {
        stack.push(l.clone());
    }

    if let Some(ref r) = node.borrow().right {
        stack.push(r.clone());
    }
  }

  // 反转结果
  res.into_iter().rev().collect()
}
```
