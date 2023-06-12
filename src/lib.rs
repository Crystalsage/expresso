#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn len(self: &Self) -> usize {
        self.elements.len()
    }

    fn push(self: &mut Self, element: T) {
        self.elements.push(element);
    }

    fn pop(self: &mut Self) -> T {
        if let Some(num) = self.elements.pop() {
            return num;
        } else {
            panic!("Stack is empty. Nothing to pop!");
        }
    }

    fn peek(self: &Self) -> Option<&T> {
        self.elements.last()
    }
}

struct Operator;

impl Operator {
    fn is_valid(op: char) -> bool {
        match op {
            '+' | '-' | '/' | '*' | '^' => true,
            _ => false,
        }
    }

    /// Get left or right associativity
    /// `true` means left associative
    /// `false` means right associative
    fn get_associativity(op: char) -> bool {
        match op {
            '+' | '-' | '/' | '*' => true,
            '^' => false,
            _ => panic!("Unsupported operator: {}", op),
        }
    }

    /// Get precedence score of the operator
    fn get_precedence(op: &char) -> u8 {
        match op {
            '+' | '-' => 2,
            '/' | '*' => 3,
            '^' => 4,
            _ => panic!("Unsupported operator: {}", op),
        }
    }
}


pub fn evaluate_rpn(input: String) -> u32 {
    let mut st: Stack<u32> = Stack::new();

    let mut input_chars = input.chars();

    loop {
        let Some(mut current_char) = input_chars.next() else { break; };

        if current_char.is_whitespace() {
            continue;
        }

        if current_char.is_digit(10) {
            let mut num: String = String::new();
            while current_char.is_digit(10) {
                num += &current_char.to_string();
                current_char = input_chars.next().unwrap();
            }
            st.push(num.parse::<u32>().unwrap());
            continue;
        }

        // Otherwise, we're probably dealing with an operator.
        let right: u32 = st.pop();
        let left: u32 = st.pop();

        match current_char {
            '+' => st.push(right + left),
            '-' => st.push(right - left),
            '*' => st.push(right * left),
            '/' => st.push(right / left),
            '^' => st.push(right ^ left),
            _ => panic!("Unexpected operator: {}", current_char),
        }
    }

    return st.pop();
}


/// Converts an infix expression to a postfix expression
/// using the Shunting Yard algorithm.
///
/// Reference: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
///
/// `input` should be a infix expression.
pub fn infix_to_rpn(input: String) -> String {
    let mut st: Stack<char> = Stack::new();
    let mut output: String = String::new();

    let mut input_chars = input.chars();

    loop {
        let Some(input_char) = input_chars.next() else { break; };

        if input_char.is_whitespace() {
            continue;
        }

        if input_char.is_digit(10) {
            output += &input_char.to_string();
            output += " ";
            continue;
        }
        
        if input_char == '(' {
            st.push(input_char);
            continue;
        }

        if input_char == ')' {
            let mut top = st.peek();
            while top != Some(&'(') {
                assert_ne!(st.len(), 0);
                output += &st.pop().to_string();
                output += " ";
                top = st.peek();
            }
            assert_eq!(st.peek(), Some(&'('));
            st.pop();
            continue;
        }

        if Operator::is_valid(input_char) {
            let o1 = input_char;
            let mut o2 = st.peek();

            let o1_prec = Operator::get_precedence(&o1);
            let o2_prec = Operator::get_precedence(&o2.unwrap_or(&'+'));

            while o2.is_some() && o2 != Some(&'(')
                && (o2_prec > o1_prec || (o2_prec == o1_prec && Operator::get_associativity(o1) == true))

            {
                output += &st.pop().to_string();
                output += " ";
                o2 = st.peek();
            }

            st.push(o1);
        }
    }

    while st.len() != 0 {
        assert_ne!(st.peek(), Some(&'('));
        output += &st.pop().to_string();
        output += " ";
    }

    // Strip off the last space that remains.
    return output[..output.len() - 1].to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rpn_with_digits() {
        assert_eq!(evaluate_rpn("1 2 +".to_string()), 3);
    }

    #[test]
    fn parse_rpn_with_numbers() {
        assert_eq!(evaluate_rpn("11 22 +".to_string()), 33);
    }

    #[test]
    #[should_panic]
    fn parse_faulty_rpn() {
        assert_eq!(evaluate_rpn("11 + 22".to_string()), 33);
    }

    #[test]
    fn test_infix_to_postfix() {
        assert_eq!(infix_to_rpn("1 + 2 * 3 - 4".to_string()), "1 2 3 * + 4 -");
    }
}
