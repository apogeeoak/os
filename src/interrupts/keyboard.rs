pub use pc_keyboard::DecodedKey as Key;

use pc_keyboard::layouts::Us104Key as Layout;
use pc_keyboard::HandleControl;
use pc_keyboard::Keyboard as Board;
use pc_keyboard::ScancodeSet1;
use spin::Lazy;
use spin::Mutex;
use x86_64::instructions::port::Port;

struct Keyboard {
    board: Board<Layout, ScancodeSet1>,
    port: Port<u8>,
}

fn keyboard() -> &'static Mutex<Keyboard> {
    static INSTANCE: Lazy<Mutex<Keyboard>> = Lazy::new(|| {
        let board = Board::new(Layout, ScancodeSet1, HandleControl::Ignore);
        let port = Port::new(0x60);
        Mutex::new(Keyboard { board, port })
    });

    Lazy::force(&INSTANCE)
}

pub fn read_key() -> Option<Key> {
    let mut keyboard = keyboard().lock();
    let scancode = unsafe { keyboard.port.read() };
    let key_event = keyboard.board.add_byte(scancode).ok().flatten()?;
    keyboard.board.process_keyevent(key_event)
}
