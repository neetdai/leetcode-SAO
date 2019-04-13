/// 编写一个函数来查找字符串数组中的最长公共前缀。

/// 如果不存在公共前缀，返回空字符串 ""。
<<<<<<< HEAD

=======
>>>>>>> origin
/// ## Examples:
/// ### Example1:
/// 输入: ["flower","flow","flight"]
/// 输出: "fl"
<<<<<<< HEAD
/// 
/// ```rust
/// # use leetcode_SAO::problems::problem14::{longest_common_prefix};
=======
/// ```rust
/// use leetcode_SAO::problems::problem14::{longest_common_prefix};
/// 
>>>>>>> origin
/// assert_eq!(
///     longest_common_prefix(vec![
///         String::from("flower"),
///         String::from("flow"),
///         String::from("flight")
///     ]),
///     String::from("fl")
/// );
/// ```
/// 
/// ### Example2:
/// 输入: ["dog","racecar","car"]
/// 输出: ""
/// 解释: 输入不存在公共前缀。
/// 说明:
/// 所有输入只包含小写字母 a-z 。
/// ```rust
<<<<<<< HEAD
/// # use leetcode_SAO::problems::problem14::{longest_common_prefix};
=======
/// use leetcode_SAO::problems::problem14::{longest_common_prefix};
>>>>>>> origin
/// assert_eq!(
///     longest_common_prefix(vec![
///         String::from("dog"),
///         String::from("racecar"),
///         String::from("car")
///     ]),
///     String::new()
/// );
/// ```

<<<<<<< HEAD
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
=======

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
>>>>>>> origin
        return String::new();
    }

    let result: &[u8] = strs[0].as_bytes();
    let mut position: usize = result.len();

<<<<<<< HEAD
    for index in 1..strs.len() {
        let tmp: &[u8] = strs[index].as_bytes();
=======
    strs.iter().skip(1).for_each(|item| {
        let tmp: &[u8] = item.as_bytes();
>>>>>>> origin

        if tmp.len() < position {
            position = tmp.len();
        }

        while tmp[0..position] != result[0..position] {
            position -= 1;
        }
<<<<<<< HEAD
    }

    String::from_utf8(result[0..position].to_vec()).unwrap()
}

=======
    });

    String::from_utf8(result[0..position].to_vec()).unwrap()
}

>>>>>>> origin
