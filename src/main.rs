
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use crossterm::event::poll;
use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, SetSize, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::{time};

mod board;


fn main() -> crossterm::Result<()> {
                    //x   ,y
    let board_size = (120, 40);
    let mut board = board::Board::new('0', board_size);
    let secs = time::Duration::from_millis(400);

    enable_raw_mode()?;

    

    execute!(stdout(),
    EnterAlternateScreen,
    Hide,
    SetSize(board_size.1 as u16, board_size.0 as u16), 
    EnableMouseCapture)?;

    draw_board(&board)?;

    'main: loop {

        loop { 
            
            if poll(time::Duration::from_millis(0))? {

                let event = crossterm::event::read()?;

                match event {
                    Event::Key(_) => (),
                    Event::Mouse(e) => match e.kind {
                        MouseEventKind::Up(_) => draw_board(&board)?,
                        MouseEventKind::Down(_) => board.manual_change((e.column.into(), e.row.into())),
                        MouseEventKind::Drag(_) => (),
                        _ => (),


                    },
                    Event::Resize(_, _) => (),

                }

                if event == Event::Key(KeyCode::Char('b').into()) {

                    board.spawn_blinker((30, 10));
                    board.update();
                    draw_board(&board)?;
                    
                }

                if event == Event::Key(KeyCode::Char('t').into()) {

                    board.spawn_tub((80, 10));
                    board.update();
                    draw_board(&board)?;
                    
                }

                if event == Event::Key(KeyCode::Char('c').into()) {

                    board.clear();
                    board.update();
                    draw_board(&board)?;
                    
                }

                if event == Event::Key(KeyCode::Char('r').into()) {
                

                    break;
                    
                }

                if event == Event::Key(KeyCode::Char('q').into()) {

                    crossterm::execute!(
                        stdout(),
                        DisableMouseCapture,
                        LeaveAlternateScreen,
                        ResetColor,
                        Show,
                        
                    )?;
                    
                    disable_raw_mode()?;

                    break 'main;
                    
                }

            }
        
        
        }

        loop {

            if poll(secs)? {

                let event = crossterm::event::read()?;

                match event {
                    Event::Key(e) => match e.code {
                        KeyCode::Char('b') => board.spawn_blinker((30, 10)),
                        KeyCode::Char('t') => board.spawn_tub((80, 10)),
                        KeyCode::Char('c') => board.clear(),
                        KeyCode::Char('r') => break,
                        _ => (),

                    },
                    Event::Mouse(e) => match e.kind {
                        MouseEventKind::Up(_) => draw_board(&board)?,
                        MouseEventKind::Down(_) => board.manual_change((e.column.into(), e.row.into())),
                        MouseEventKind::Drag(_) => (),
                        _ => (),


                    },
                    Event::Resize(_, _) => (),

                }

                if event == Event::Key(KeyCode::Char('q').into()) {

                    crossterm::execute!(
                        stdout(),
                        DisableMouseCapture,
                        ResetColor,
                    )?;
                    
                    disable_raw_mode()?;

                    break 'main;
                    
                }


            } else {

                board = update_board(board);
        
                draw_board(&board)?;
                
                
                    
            } 
            
        }
    }
    Ok(())

}

fn draw_board(b: &board::Board) -> Result<()> {

    //buffer
    let mut stdout = stdout();

    //keep track of board postion
    let mut x = 0;

    for row in &b.current_state {

        let mut y = 0;

        for ch in row {

            if *ch == 'X' {
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.queue(SetBackgroundColor(Color::Green))?;
            } else {
                stdout.queue(SetForegroundColor(Color::Black))?;
                stdout.queue(SetBackgroundColor(Color::Black))?;
            }

            stdout.queue(cursor::MoveTo(x, y))?;
            stdout.queue(Print(ch))?;

            y+=1
        }

        x+=1
    }
    stdout.flush()?;

    Ok(())

}

fn update_board(mut b:board::Board) -> board::Board {

    let mut x:usize = 0;

    for row in &b.current_state {

        let mut y:usize = 0;

        for spot in row {

            let n = b.find_neighbors((x, y));

            if spot == &'X' && n < 2 {

                b.new_state[x][y] = '0';

            } else if spot == &'X' && n == 2 || n == 3 {

                b.new_state[x][y] = 'X';

            } else if spot == &'X' && n > 3 {

                b.new_state[x][y] = '0';

            } else if spot == &'0' && n == 3 {

                b.new_state[x][y] = 'X';
            }

            y +=1

        }

        x += 1

    }

    b.update();

    b

}


