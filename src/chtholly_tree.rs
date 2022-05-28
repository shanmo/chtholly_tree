/// Chtholly Tree allows queries and updates to be performed in a interval.
/// The input data needs to be random for it to achieve a time complexity of `O(nloglogn)`.
/// Representation of Chtholly Node used to build Chtholly Tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TreeNode {
    left: i32,
    right: i32,
    value: i32,
}

impl TreeNode {
    /// Creates a new TreeNode based on the interval `[left, right]` and `value`.
    pub fn new(left: i32, right: i32, value: i32) -> Self {
        TreeNode {
            left: left,
            right: right,
            value: value,
        }
    }
}

/// Representation of Chtholly Tree. The nodes are sorted based on `left` in interval `[left, right]`.  
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChthollyTree {
    nodes: Vec<TreeNode>,
}

impl ChthollyTree {
    pub fn new() -> Self {
        ChthollyTree { nodes: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    /// Split the intervals in the tree based on the given position.
    /// E.g. interval is [1, 5] and position is 3, then the interval will be split into [1, 2] and [3, 5].
    pub fn split(&mut self, pos: i32) -> usize {
        let n: usize = self.nodes.len();
        let mut left: usize = 0;
        let mut right: usize = n - 1;

        // use binary search to find the first interval
        // whose left end is smaller than or equal to pos
        // can also use built-in `binary_search`
        while left <= right {
            let mid = (right - left) / 2 + left;
            if self.nodes[mid].left < pos {
                left = mid + 1;
            } else if self.nodes[mid].left > pos {
                right = mid - 1;
            } else {
                return mid;
            }
        }

        left -= 1;
        let l = self.nodes[left].left;
        let r = self.nodes[left].right;
        let v = self.nodes[left].value;

        self.nodes.remove(left);
        let tr = TreeNode::new(l, pos - 1, v);
        self.nodes.insert(left, tr);
        let tr = TreeNode::new(pos, r, v);
        self.nodes.insert(left + 1, tr);
        return left + 1;
    }

    /// Assign the value to given interval,
    /// and remove all intervals in the tree that are covered by the given interval.
    pub fn assign(&mut self, left: i32, right: i32, value: i32) {
        // check whether there are nodes in the tree
        if self.is_empty() {
            let tr = TreeNode::new(left, right, value);
            self.nodes.push(tr);
            return;
        }

        // delete the intervals between itl and itr
        let itr = self.split(right + 1);
        let itl = self.split(left);
        let mut index = 0;
        self.nodes.retain(|_| {
            index += 1;
            index < itl + 1 || index >= itr + 1
        });

        // insert the new interval
        let tr = TreeNode::new(left, right, value);
        self.nodes.insert(itl, tr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Chtholly Tree using [leetcode 699. Falling Squares](https://leetcode.com/problems/falling-squares/).
    #[test]
    fn test_tree() {
        let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];

        let mut results = Vec::<i32>::new();
        let mut max_height = 0;
        let mut ct_tree = ChthollyTree::new();
        ct_tree.assign(1_i32, 10i32.pow(8), 0_i32);
        for pos in positions {
            let itr = ct_tree.split(pos[0] + pos[1]);
            let itl = ct_tree.split(pos[0]);
            let mut height = 0;
            for i in itl..itr + 1 {
                height = height.max(ct_tree.nodes[i].value + pos[1]);
                max_height = max_height.max(height);
            }
            ct_tree.assign(pos[0], pos[0] + pos[1] - 1, height);
            results.push(max_height);
        }
        assert_eq!(results, vec![2, 5, 5]);
    }
}
