use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
mod terminal;
//导入外部库
use std::io::Error;
use terminal::{Position, Size, Terminal};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    //初始化编辑器
    //常量函数
    pub const fn default() -> Self {
        Editor { should_quit: false }
    }
    //运行编辑器
    pub fn run(&mut self) {
        //使用unwarp 将Error转换为panic
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { width: _, height } = Terminal::size()?;

        for current_rows in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_rows < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                //使用.contains()代替指针解引用
                KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }
    //  运行编辑器的 REPL 循环，返回 Result
    fn repl(&mut self) -> Result<(), std::io::Error> {
        //简化错误处理
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let enent = read()?;
            self.evaluate_event(&enent);
        }
        Ok(())
    }
}
