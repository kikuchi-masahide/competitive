use cargo_snippet::snippet;

#[snippet("shakutori")]
//配列[0..n]に対し、尺取り法を行う
//情報のメモ化・条件判断には、struct Memoを用いる。
//judge(s,e,memo): 現在保持している部分列[s:e]が条件を満たすか、memoの情報を用い判断
//append(i,memo): i番目の要素を現在保持する部分列に追加。memoの更新
//remove(i,memo): i番目の要素を現在保持する部分列から除外。memoの更新
fn shakutori<Memo>(
    n: usize,
    judge: impl Fn(usize, usize, &Memo) -> bool,
    mut append: impl FnMut(usize, &mut Memo),
    mut remove: impl FnMut(usize, &mut Memo),
    memo: &mut Memo,
) -> usize {
    let mut answer = 0;

    // [s,e]の範囲をチェック
    let mut s = 0;
    let mut e = 0;
    append(0, memo);
    while e < n {
        if judge(s, e, memo) {
            answer = max(answer, e - s + 1);
            e += 1;
            if e == n {
                break;
            }
            append(e, memo);
        } else {
            remove(s, memo);
            s += 1;
            if s > e {
                e += 1;
                if e == n {
                    break;
                }
            }
        }
    }
    return answer;
}
