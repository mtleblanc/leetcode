impl Solution {
    /*
     * This is O(n^2) worst case - if the input is n/3 nested parens
     * surrounding n/3 characters, then we will make n/3 stacks each
     * reversing n/3 characters, but this runs in 0ms on the test
     * suite.  It feels like a linear solution exsits where we can
     * write characters directly to the correct location, but we
     * need to reach a depth of 0 parens before we can calculate
     * what that location is.  Initial attempts showed this calculation
     * is more complex than I expected.  An idea I haven't tried
     * is representing brackets as a tree and recording start
     * and end indices.  That should have sufficient information
     * to find target indices, and feels at least O(n ln n)
     */
    pub fn reverse_parentheses(s: String) -> String {
        let mut stacks = Vec::new();
        let mut current_stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stacks.push(current_stack);
                    current_stack = Vec::new();
                }
                ')' => {
                    let mut reverse = current_stack;
                    current_stack = stacks.pop().unwrap();
                    while let Some(d) = reverse.pop() {
                        current_stack.push(d);
                    }
                }
                _ => {
                    current_stack.push(c);
                }
            }
        }
        current_stack.into_iter().collect()
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        fn test(input: &str, output: &str) {
            assert_eq!(
                Solution::reverse_parentheses(input.to_string()),
                output.to_string()
            );
        }
        test("(abcd)", "dcba");
        test("(u(love)i)", "iloveu");
        test("(ed(et(oc))el)", "leetcode");
        test("a(bcdefghijkl(mno)p)q", "apmnolkjihgfedcbq");
    }
}
