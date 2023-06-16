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



fn evaluator_add_to_output(output: &mut Vec<u32>, n: u32) {
    output.push(n);
}

fn evaluator_handle_pop(st: &mut Stack<char>, output: &mut Vec<u32>) -> Option<u32> {
    let op = st.pop();

    if op == '(' {
        return None;
    }

    let right = output.pop().unwrap();
    let left = output.pop().unwrap();

    match op {
        '+' => Some(left + right),
        '-' => Some(left - right),
        '*' => Some(left * right),
        '/' => Some(left / right),
        '^' => Some(left.pow(right)),
        _ => panic!("Unexpected operator: {}", op),
    }
}


/// Same as Shunting Yard Algorithm, but also evaluates the expression
/// on-the-fly. Uses `Tokenizer`.
pub fn sy_evaulate(input: String) -> u32 {
    let mut st: Stack<char> = Stack::new();
    let mut output: Vec<u32> = Vec::new();

    let mut input_chars = input.chars();

    loop {
        let Some(input_char) = input_chars.next() else { break; };

        if input_char.is_whitespace() {
            continue;
        }

        if input_char.is_digit(10) {
            evaluator_add_to_output(&mut output, input_char.to_digit(10).unwrap());
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
                let res = evaluator_handle_pop(&mut st, &mut output);
                evaluator_add_to_output(&mut output, res.unwrap());
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
                let res = evaluator_handle_pop(&mut st, &mut output).unwrap();
                evaluator_add_to_output(&mut output, res);
                o2 = st.peek();
            }

            st.push(o1);
        }
    }

    while st.len() != 0 {
        assert_ne!(st.peek(), Some(&'('));
        let res = evaluator_handle_pop(&mut st, &mut output).unwrap();
        evaluator_add_to_output(&mut output, res);
    }

    return output[0];
}


// ============== TOKENIZER BELOW =================
#[derive(Debug, PartialEq)]
enum Tokens {
    Number(u32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    ParenLeft,
    ParenRight,
}

struct Tokenizer {
    tokens: Vec<Tokens>,
    raw_input: String,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        let mut input_chars = input.chars();
        let mut tokens: Vec<Tokens> = Vec::new();

        loop {
            let Some(input_char) = input_chars.next() else { break; };

            if input_char.is_digit(10) {
                let mut num = "".to_string();
                num.push(input_char);
                loop {
                    let Some(input_char) = input_chars.next() else { break; };
                    if input_char.is_digit(10) {
                        num.push(input_char);
                    } else {
                        break;
                    }
                }
                tokens.push(Tokens::Number(num.parse::<u32>().unwrap()));
                continue;
            }

            match input_char {
                '+' => tokens.push(Tokens::Plus),
                '-' => tokens.push(Tokens::Minus),
                '*' => tokens.push(Tokens::Asterisk),
                '/' => tokens.push(Tokens::Slash),
                '^' => tokens.push(Tokens::Caret),
                '(' => tokens.push(Tokens::ParenLeft),
                ')' => tokens.push(Tokens::ParenRight),
                ' ' => continue,
                _ => panic!("Unexpected character: {}", input_char),
            }
        }

        Tokenizer {
            tokens,
            raw_input: input,
        }
    }

    fn default() -> Self {
        Tokenizer {
            tokens: Vec::new(),
            raw_input: String::new(),
        }
    }

    fn iter(self: &Self) -> impl Iterator<Item=&Tokens> {
        self.tokens.iter()
    }
}
// ============== TOKENIZER ABOVE =================

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

    #[test]
    fn test_sy_evaluator() {
        assert_eq!(sy_evaulate("1 + 2 * 3 - 4".to_string()), 3);
    }

    #[test]
    fn test_tokenizer() {
        let tokens: Vec<Tokens> = vec![
            Tokens::Number(1),
            Tokens::Plus,
            Tokens::Number(2),
            Tokens::Asterisk,
            Tokens::Number(3),
            Tokens::Minus,
            Tokens::Number(4),
        ];

        let resulting_tokens = Tokenizer::new("1 + 2 * 3 - 4".to_string()).tokens;

        for (token1, token2) in tokens.iter().zip(resulting_tokens.iter()) {
            if token1 != token2 {
                assert!(false);
            }
        }
    }
}
