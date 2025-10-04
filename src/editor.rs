//导入外部库
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    //初始化编辑器
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    //运行编辑器
    pub fn run(&mut self) {
        //调用并匹配错误
        if let Err(err) = self.repl() {
            //错误处理
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }
    //  运行编辑器的 REPL 循环，返回 Result
    fn repl(&mut self) -> Result<(), std::io::Error> {
        //简化错误处理
        enable_raw_mode()?;
        loop {
            //读取事件,获取当前按键
            //code为按键值，modifiers为修饰符，kind为按键类型，state为按键状态
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            //每次循环检查是否退出
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
