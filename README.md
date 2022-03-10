# acl_rs

AtCoder Library を Rust で書いたもの。

[ac-library-rs](https://github.com/rust-lang-ja/ac-library-rs) と異なる点
- かなり好き勝手にインターフェースを変更している。
  - 追加は避けている。
  - `sum(l, r)` を `sum(l..r)` にしたり
- そこそこ好き勝手に実装を変更している。
  - 多少の速度低下はやむを得ないと思っているが、たぶん問題ない程度になっている。
- クレートに分割し、依存関係をなくしている
