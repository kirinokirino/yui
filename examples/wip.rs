use yui::tui::Terminal;

fn main() {
    let mut terminal = Terminal::new();
    let fps = 60;
    let frames = fps * 50;
    for i in 0..frames {
        terminal.display();
        std::thread::sleep(std::time::Duration::from_millis(1000/fps));
        terminal.update();
        dbg!(terminal);
        panic!();
    }
}
