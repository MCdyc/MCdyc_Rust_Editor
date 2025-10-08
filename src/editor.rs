use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
mod terminal;
//导入外部库
use std::{io::Error, string};
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
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
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }
    fn draw_message(str: String) -> Result<(), Error> {
        let mut message = str;
        let Size { width, height: _ } = Terminal::size()?;
        let len = message.len();
        if len >= width {
            Self::draw_empty_row()?;
            return Ok(());
        }
        let padding = width.saturating_sub(message.len()) / 2;
        let padding = padding.saturating_sub(1);
        let spaces = " ".repeat(padding);
        message = format!("~{}{}", spaces, message);
        // message.truncate(width as usize);
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { width: _, height } = Terminal::size()?;

        for current_rows in 0..height {
            Terminal::clear_line()?;
            match current_rows {
                r if r == height / 3 => {
                    Self::draw_message(format!("{} -- version {}", NAME, VERSION))?
                }
                r if r == height / 3 + 1 => {
                    Self::draw_message(format!("by {}", AUTHOR))?;
                }
                _ => Self::draw_empty_row()?,
            }
            if current_rows < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
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
    fn repl(&mut self) -> Result<(), Error> {
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
