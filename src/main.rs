//使用警告和建议
#![warn(clippy::all, clippy::pedantic)]
//导入当前文件夹中存在的模块
mod editor;

//使用模块中的结构体
use editor::Editor;

fn main() {
    Editor::default().run();
}
