
use crate::state::FmState;

use std::io::{Write, stdin};

use termion::input::TermRead;

pub fn start_loop<W:Write>(state: &mut FmState<W>) {

    state.reload();

    loop {
        state.show();
        let key = stdin().keys().next().unwrap().unwrap();
        match key {
            termion::event::Key::Char('Q') => break,
            _ => state.handle_key(key)	
        }
    }

}


