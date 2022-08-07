pub struct WordProblem;

enum Token<'a> {
    Number(i32),
    NonNumber(&'a str),
}

fn apply_op<'a, 'b>(num1: i32, words: &'a [Token<'b>]) -> Option<(i32, &'a [Token<'b>])> {
    let number_pos = words.iter().position(|w| match w { // 처음 숫자가 나오는 위치를 가져옴
        Token::Number(_) => true,
        Token::NonNumber(_) => false,
    })?;

    // op_and_num = words[..숫자가 있는 위치까지], remainder = words[숫자 나온 다음..] 으로 나눔
    // op = [operation], num2 = [숫자]
    let (op_and_num, remainder) = words.split_at(number_pos + 1);
    let (op, num2) = op_and_num.split_at(number_pos);
    // num2 = [숫자] -> 숫자
    let &num2 = match num2 {
        [Token::Number(i)] => i,
        _ => unreachable!("invalid value"),
    };

    // 연산 수행
    let result = match op {
        [Token::NonNumber("plus")] => Some(num1 + num2),
        [Token::NonNumber("minus")] => Some(num1 - num2),
        [Token::NonNumber("multiplied"), Token::NonNumber("by")] => Some(num1 * num2),
        [Token::NonNumber("divided"), Token::NonNumber("by")] => Some(num1 / num2),
        _ => None,
    };
    return Some((result.unwrap(), remainder));
}

pub fn answer(command: &str) -> Option<i32> {
    let words = command
        .trim_end_matches('?') // 맨 마지막 문자가 ? 인지 확인하고 그 문자를 trim 함
        .split_whitespace() // 공백을 기준으로 문자열을 split
        .map(|word| { // map 함수를 이용하여, 리스트내의 원소를 Token 타입으로 캐스팅
            // 1. 파싱한 값이 숫자면 Token::Number로 캐스팅
            // 2. 파싱한 값이 문자면 Token::NonNumber로 캐스팅
            if let Ok(i) = word.parse::<i32>() { Token::Number(i) }
            else { Token::NonNumber(word) }
        })
        .collect::<Vec<Token>>(); // Vector<Token> 형태로 바꿈

    let mut result: i32 = match words[0..3] { // rust의 패턴매칭을 이용해 [What, is, 숫자] 형태인지 확인
        [Token::NonNumber("What"), Token::NonNumber("is"), Token::Number(i)] => i,
        _ => return None,
    };
    let mut words = words.split_at(3).1; // words[0..3], words[3..] 으로 나누고 이 중 뒤에 있는 값을 words에 세팅
    while !words.is_empty() { // 남아있는 연산들을 수행
        let tmp = apply_op(result, words)?; // return: (연산된 결과, 남아있는 operation)
        result = tmp.0; // 연산된 결과 세팅
        words = tmp.1; // 남아있는 operation 재세팅
    }

    return Some(result);
}
