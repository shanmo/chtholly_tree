## About 

- This repo implements the `Chtholly Tree` in Rust 
- `Chtholly Tree` is a data structure originated from [C. Willem, Chtholly and Seniorious](https://codeforces.com/problemset/problem/896/C?mobile=true), which could be used to update the values of intervals
- Note that the input data should be random for it to achieve a time complexity of `O(nloglogn)`

## Cargo.toml

```
[dependencies]
chtholly_tree = "0.1.0"
```

## Example

To demonstrate its usage, [leetcode 699. Falling Squares](https://leetcode.com/problems/falling-squares/) is used as an example 

```rust
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
```

## Reference 

This repo is developed based on 
- [Crate chtholly](https://docs.rs/chtholly/latest/chtholly/)