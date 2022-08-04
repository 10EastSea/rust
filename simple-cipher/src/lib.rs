pub fn encode(key: &str, s: &str) -> Option<String> {
    let mut cipher_text = String::new(); // 암호문이 담길 변수

    let mut key_arr = Vec::new();
    for k in key.chars() { key_arr.push(k); } // key를 string에서 char vector로 바꿈

    let mut i = 0; // key 위치를 이동할 변수
    for c in s.chars() {
        let shift = (key_arr[i % key_arr.len()] as i8) - ('a' as i8); // key의 해당 위치에 존재하는 값에서 'a'로 부터 위치 계산
        let n = ((c as i8) - (b'a' as i8) + shift) % 26; // shift 해야할 값 계산 -> shift된 후 c의 값: n
        cipher_text.push(char::from(b'a' + n as u8)); // cipher_text에 추가
        i += 1;
    }
    
    Some(cipher_text) // 암호문 반환
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let mut plain_text = String::new(); // 평문이 담길 변수

    let mut key_arr = Vec::new();
    for k in key.chars() { key_arr.push(k); } // key를 string에서 char vector로 바꿈

    let mut i = 0; // key 위치를 이동할 변수
    for c in s.chars() {
        let shift = (key_arr[i % key_arr.len()] as i8) - ('a' as i8); // key의 해당 위치에 존재하는 값에서 'a'로 부터 위치 계산
        let n = (((c as i8) - (b'a' as i8) - shift) % 26 + 26) % 26; // shift 해야할 값 계산 -> shift된 후 c의 값: n (c-'a'-shift 가 음수 일 수 있어 26을 한 번 더 더해줌)
        plain_text.push(char::from(b'a' + n as u8)); // plain_text에 추가
        i += 1;
    }
    
    Some(plain_text) // 평문 반환
}

// pub fn encode_random(s: &str) -> (String, String) {
//     unimplemented!(
//         "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
//         s
//     )
// }
