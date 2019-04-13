/// 给定一个仅包含大小写字母和空格 ' ' 的字符串，返回其最后一个单词的长度。

/// 如果不存在最后一个单词，请返回 0 。

/// 说明：一个单词是指由字母组成，但不包含任何空格的字符串。

/// ## Examples:
/// ### Example1:
/// 输入: "Hello World"
/// 输出: 5
/// ```rust
/// use leetcode_SAO::problems::problem58::{length_of_last_word};
/// assert_eq!(
///     length_of_last_word(String::from("Hello World")),
///     5
/// );
/// ```
/// 
/// ### Example2:
/// 输入: "Hello "
/// 输出: 5
/// ```rust
/// use leetcode_SAO::problems::problem58::{length_of_last_word};
/// assert_eq!(length_of_last_word(String::from("Hello ")), 5);
/// ```
/// 
/// ### Example3:
/// 输入: ""
/// 输出: 0
/// ```rust
/// use leetcode_SAO::problems::problem58::{length_of_last_word};
/// assert_eq!(length_of_last_word(String::from("")), 0);
/// ```


pub fn length_of_last_word(s: String) -> i32 {
    let list: Vec<&str> = s.split_whitespace().collect();
    let length: usize = list.len();

    if length == 0 {
        0
    } else {
        list[length - 1].len() as i32
    }
}

