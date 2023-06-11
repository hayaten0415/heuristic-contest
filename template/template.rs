// https://img.atcoder.jp/intro-heuristics/editorial.pdf
struct Input {
    D: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>
}

fn compute_score(input: &Input, out: &Vec<usize>) -> i64 {
    let mut score = 0;
    let mut last = vec![0; 26];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
    for i in 0..26 {
        score -= (d + 1 - last[i]) as i64 * input.c[i];
    }
        score += input.s[d][out[d]];
    }
    score
}