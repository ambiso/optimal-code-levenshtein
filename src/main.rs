use regex::Regex;

pub fn levenshtein(s: &[u8], t: &[u8]) -> usize {
    let len_s = s.len();
    let len_t = t.len();

    let mut mat: Vec<Vec<usize>> = vec![vec![0; len_t + 1]; len_s + 1];
    for i in 1..(len_s + 1) {
        mat[i][0] = i;
    }
    for i in 1..(len_t + 1) {
        mat[0][i] = i;
    }

    for (i, s_char) in s.iter().enumerate() {
        for (j, t_char) in t.iter().enumerate() {
            let substitution = if s_char == t_char { 0 } else { 1 };
            mat[i + 1][j + 1] = (
                // deletion
                mat[i][j + 1] + 1
            )
            .min(
                // insertion
                mat[i + 1][j] + 1,
            )
            .min(
                // substitution
                mat[i][j] + substitution,
            );
        }
    }

    return mat[len_s][len_t];
}

const Q: u8 = 26;

// fn hamming(x: &[u8], y: &[u8]) -> usize {
//     let mut d = 0;
//     for (a, b) in x.iter().zip(y.iter()) {
//         if a != b {
//             d += 1;
//         }
//     }
//     d
// }

fn cleaned(s: &str) -> String {
    s.chars().filter(|x| !x.is_numeric()).collect()
}

fn main() {
    // let mut code = Vec::new();

    // for a in 0..Q {
    //     for b in 0..Q {
    //         for c in 0..Q {
    //             if (a + b + c) % Q == 0 {
    //                 code.push([a, b, c]);
    //             }
    //         }
    //     }
    // }

    // for a in code.iter() {
    //     for b in code.iter() {
    //         if a == b {
    //             continue;
    //         }
    //         if levenshtein(a, b) < 2 {
    //             println!("violation: {a:?} {b:?}");
    //         }
    //     }
    // }
    let s = String::from_utf8(std::fs::read("pkgs.json").unwrap()).unwrap();
    let pkgs: Vec<String> = serde_json::from_str(&s).unwrap();
    let re = Regex::new(r"_\d+$").unwrap();
    let cleaned_pkgs: Vec<String> = pkgs.iter().map(|x| cleaned(x)).collect();
    for (i, a) in pkgs.iter().enumerate() {
        // ignore version suffixes
        if re.is_match(&a) {
            continue;
        }
        for (j, b) in pkgs[i + 1..].iter().enumerate() {
            // ignore if they're the same up to a number
            if cleaned_pkgs[i] == cleaned_pkgs[j + i + 1] {
                continue;
            }
            if levenshtein(a.as_bytes(), b.as_bytes()) < 2 {
                println!("{a} {b}");
            }
        }
    }
}
