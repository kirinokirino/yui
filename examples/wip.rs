use yui::tui::Terminal;

fn main() {
    let terminal = Terminal::new();
    terminal.print();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
