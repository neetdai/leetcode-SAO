/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。

/// 有效字符串需满足：

/// 左括号必须用相同类型的右括号闭合。
/// 左括号必须以正确的顺序闭合。
/// 注意空字符串可被认为是有效字符串。
///
/// ## Examples:
/// ### Example1:

/// 输入: "()"
/// 输出: true
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("()")), true);
/// ```
///
/// ### Example2:
/// 输入: "()[]{}"
/// 输出: true
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("()[]{}")), true);
/// ```
///
/// ### Example3:
/// 输入: "(]"
/// 输出: false
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("(]")), false);
/// ```
///
/// ### Example4:
/// 输入: "([)]"
/// 输出: false
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("([)]")), false);
/// ```
///
/// ### Example5:
/// 输入: "{[]}"
/// 输出: true
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("{[]}")), true);
/// ```
///
/// ### Example6:
/// 输入: "["
/// 输出: false
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("[")), false);
/// ```
/// 
/// ### Example7:
/// 输入: "(("
/// 输出: false
/// ```rust
/// use leetcode_SAO::problems::problem20::{is_valid};
/// assert_eq!(is_valid(String::from("((")), false);
/// ```

pub fn is_valid(s: String) -> bool {
    use std::str::Chars;

    let length: usize = s.len();

    if length == 0 {
        return true;
    }

    if length % 2 > 0 {
        return false;
    }

    let mut stack: Vec<char> = Vec::with_capacity(s.len());
    let tmp: Chars = s.chars();

    for item in tmp {
        if item == '(' || item == '{' || item == '[' {
            stack.push(item);
        } else if item == ']' {
            match stack.pop() {
                Some(i) if i == '[' => {}
                _ => {
                    return false;
                }
            }
        } else if item == '}' {
            match stack.pop() {
                Some(i) if i == '{' => {}
                _ => {
                    return false;
                }
            }
        } else if item == ')' {
            match stack.pop() {
                Some(i) if i == '(' => {}
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    if !stack.is_empty() {
        return false;
    }

    true
}
