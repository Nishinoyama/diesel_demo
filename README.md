# diesel_demo
mysqlを用いて、diesel_demoのチュートリアルを踏襲

## 問題点
[create_post](https://github.com/Nishinoyama/diesel_demo/blob/6d1a33fc3c1a5dd79a9914a96d2fd75bc80cce44/src/lib.rs#L21) の返り値が意味を持たない（postgressなら返り値に挿入した値を入れることが出来る）
