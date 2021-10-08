fn main() {
    let s = "hello world!".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is :{}, s1:{}, s:{}", hello, s1, s);
}

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于delimiter可以是utf8，所以我们需要获得其utf8长度
        // 直接使用len返回的时字节长度，有问题
        let suffix = &s[(i+delimiter.len_utf8()..)];
        *s = suffix;
        prefix
    } else {
        // 没找到返回整个字符串，把源字符串指针s指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}