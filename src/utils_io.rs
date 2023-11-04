//! Utils for input/output.

#![allow(dead_code)]

use std::collections::HashMap;


pub fn press_enter_to_continue() {
    print("PRESS ENTER TO CONTINUE.");
    wait_for_enter();
}

pub fn wait_for_enter() {
    use std::io::stdin;
    let mut line: String = String::new();
    stdin().read_line(&mut line).unwrap();
}

pub fn print(msg: impl ToString) {
    print!("{}", msg.to_string());
    flush();
}

pub fn flush() {
    use std::io::{Write, stdout};
    stdout().flush().unwrap();
}


// TODO: make it macros
pub fn format_by_dollar_digit(str_to_fmt: &str, params: Vec<&str>) -> String {
    const MAX_ITERS: u32 = 10_000;
    let mut str_fmted: String = str_to_fmt.to_string();
    for _ in 0..MAX_ITERS {
        match str_fmted.find('$') {
            None => return str_fmted,
            Some(index_to_insert) => {
                let param_index: usize = str_fmted.chars().nth(index_to_insert+1).unwrap().to_string().parse().unwrap();
                str_fmted.replace_range(index_to_insert..=index_to_insert+1, params[param_index]);
            }
        }
    }
    panic!("hit max iters");
}

// TODO: make it macros
pub fn format_by_dollar_char(str_to_fmt: &str, params: Vec<(char, &str)>) -> String {
    const MAX_ITERS: u32 = 10_000;
    let params_len: usize = params.len();
    let params_hm: HashMap<char, &str> = params.into_iter().collect();
    assert_eq!(params_len, params_hm.len(), "found duplicate in params");
    let mut str_fmted: String = str_to_fmt.to_string();
    for _ in 0..MAX_ITERS {
        match str_fmted.find('$') {
            None => return str_fmted,
            Some(index_to_insert) => {
                let param_name: char = str_fmted.chars().nth(index_to_insert+1).unwrap();
                str_fmted.replace_range(index_to_insert..=index_to_insert+1, params_hm[&param_name]);
            }
        }
    }
    panic!("hit max iters");
}

// TODO: make it macros
pub fn format_by_dollar_str(str_to_fmt: &str, params: Vec<(&str, &str)>) -> String {
    const MAX_ITERS: u32 = 10_000;
    const MAX_PARAM_NAME_LEN: usize = 10;
    let params_len: usize = params.len();
    let params_hm: HashMap<&str, &str> = params.into_iter().collect();
    assert_eq!(params_len, params_hm.len(), "found duplicate in params");
    let mut str_fmted: String = str_to_fmt.to_string();
    for _ in 0..MAX_ITERS {
        match str_fmted.find('$') {
            None => return str_fmted,
            Some(index_to_insert) => {
                let max_len: usize = (0..MAX_PARAM_NAME_LEN).rev()
                    .find(|len| {
                        params_hm.get(str_fmted.chars().skip(index_to_insert+1).take(len+1).collect::<String>().as_str()).is_some()
                    })
                    .unwrap();
                let param_name: String = str_fmted.chars().skip(index_to_insert+1).take(max_len+1).collect::<String>();
                str_fmted.replace_range(index_to_insert..=index_to_insert+max_len+1, params_hm[param_name.as_str()]);
            }
        }
    }
    panic!("hit max iters");
}



// TODO(rename): better names for tests.

#[test]
fn format_by_dollar_digit_() {
    assert_eq!(
        "abc { 42 + 3.14/42 } def",
        format_by_dollar_digit(
            "abc { $0 + $1/$0 } def",
            vec![
                "42",
                "3.14",
            ]
        )
    );
}

#[test]
fn format_by_dollar_char_() {
    assert_eq!(
        "abc { 42 + 3.14/42 } def",
        format_by_dollar_char(
            "abc { $f + $p/$f } def",
            vec![
                ('f', "42"),
                ('p', "3.14"),
            ]
        )
    );
}

#[should_panic]
#[test]
fn format_by_dollar_char__() {
    let _ = format_by_dollar_char(
        "abc { $f + $p/$f } def",
        vec![
            ('f', "42"),
            ('p', "3.14"),
            ('f', "43"),
        ]
    );
}

#[test]
fn format_by_dollar_str_() {
    assert_eq!(
        "abc { 42 + 3.14/43 } def",
        format_by_dollar_str(
            "abc { $f + $p/$ft } def",
            vec![
                ("f", "42"),
                ("p", "3.14"),
                ("ft", "43"),
            ]
        )
    );
}

