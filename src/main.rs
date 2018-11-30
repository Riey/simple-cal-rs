#[derive(Debug)]
#[derive(Copy, Clone)]
enum Op {
    Plus,
    Minus,
    Div,
    Mul,
}

impl Op {
    pub fn parse(b: u8) -> Option<Op> {
        match b {
            b'+' => Some(Op::Plus),
            b'-' => Some(Op::Minus),
            b'/' => Some(Op::Div),
            b'*' => Some(Op::Mul),
            _ => None,
        }
    }

    pub fn order(&self) -> usize {
        match self {
            Op::Plus | Op::Minus => 0,
            Op::Mul | Op::Div => 1,
        }
    }

    pub fn cal(&self, stack: &mut Vec<i32>) {

        let rhs = stack.pop().unwrap();
        let lhs = stack.pop().unwrap();

        let ret = match self {
            Op::Plus => lhs + rhs,
            Op::Minus => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        };

        stack.push(ret);

    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Token {
    Int(i32),
    Op(Op),
    Lp,
    Rp,
}

#[derive(Debug)]
#[derive(Clone)]
enum PostFixTerm {
    Int(i32),
    Op(Op),
}

fn skip_ws(input: &[u8]) -> usize {
    let mut cur = 0;

    while input.len() > cur && input[cur].is_ascii_whitespace() {
        cur += 1;
    }

    cur
}

fn read_int(input: &[u8]) -> (i32, usize) {
    let mut cur = 0;

    while input.len() > cur && input[cur].is_ascii_digit() {
        cur += 1;
    }

    unsafe {
        (::std::str::from_utf8_unchecked(&input[..cur]).parse().unwrap(), cur)
    }
}

fn lex(input: &[u8]) -> Vec<Token> {
    let mut ret = Vec::new();

    let input = input;
    let mut cur = 0;

    while input.len() > cur {
        cur += skip_ws(&input[cur..]);

        if input.len() > cur {
            let token = match input[cur] {
                b'(' => {
                    cur += 1;
                    Token::Lp
                },
                b')' => {
                    cur += 1;
                    Token::Rp
                },
                c => {
                    match Op::parse(c) {
                        Some(op) => {
                            cur += 1;
                            Token::Op(op)
                        },
                        None => {
                            let (num, read) = read_int(&input[cur..]);
                            cur += read;
                            Token::Int(num)
                        }
                    }
                }
            };

            ret.push(token)
        }
    }

    ret
}

fn insert_op(op_stack: &mut Vec<Op>, op: Op) {
    match op_stack.pop() {
        Some(pre_op) => {
            if pre_op.order() < op.order() {
                insert_op(op_stack, op);
                op_stack.push(pre_op);
            } else {
                op_stack.push(pre_op);
                op_stack.push(op);
            }
        }
        None => op_stack.push(op),
    }
}

fn to_post_fix(tokens: &[Token]) -> Vec<PostFixTerm> {
    let mut ret = Vec::new();
    let mut op_stack = Vec::new();

    op_stack.push(Vec::new());

    for token in tokens {
        match token {
            Token::Int(num) => ret.push(PostFixTerm::Int(*num)),
            Token::Op(op) => insert_op(op_stack.last_mut().unwrap(), *op),
            Token::Lp => op_stack.push(Vec::new()),
            Token::Rp => {
                for op in op_stack.pop().unwrap() {
                    ret.push(PostFixTerm::Op(op));
                }
            }
        }
    }

    for op in op_stack.pop().unwrap() {
        ret.push(PostFixTerm::Op(op));
    }

    ret
}

fn eval(post_fix_terms: &[PostFixTerm]) -> i32 {
    let mut stack = Vec::new();

    for term in post_fix_terms {
        match term {
            PostFixTerm::Int(n) => stack.push(*n),
            PostFixTerm::Op(op) => op.cal(&mut stack),
        }
    }

    stack.pop().unwrap()
}

fn main() {

    let text = "2 + 1 * (1 * 2 + 1)";
    let bytes = text.as_bytes();

    let tokens = lex(bytes);
    println!("{:#?}", tokens);

    let terms = to_post_fix(&tokens);
    println!("{:#?}", terms);

    let ret = eval(&terms);

    assert_eq!(5, ret);
}
