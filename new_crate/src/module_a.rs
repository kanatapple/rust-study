// 同じパッケージ内のモジュールを指定する場合は、クレート名ではなく crate を使う
// ルートディレクトリからの絶対パス
use crate::module_b;

// module_c を指定する場合の例
// use crate::module_b::module_c;
// use self::module_b::module_c;
// use super::module_b::module_c;
