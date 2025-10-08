use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{Error, Write, stdout};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Terminal;

impl Terminal {
    //刷新输出缓冲区
    pub fn terminate() -> Result<(), Error> {
        //刷新缓冲区
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    //进入原始模式，清屏并将光标移动到(0,0)
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }
    //刷新输出缓冲区
    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
    //获取终端大小
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        let (width, height) = (width as usize, height as usize);
        Ok(Size { width, height })
    }
    //加入字符串到输出缓冲区
    fn queue_command<T>(command: T) -> Result<(), Error>
    where
        T: crossterm::Command,
    {
        queue!(stdout(), command)
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }
    //清屏
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }
    //清除当前行
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }
    //将光标移动到(x,y)
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))
    }
    //隐藏和显示光标
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }
}
