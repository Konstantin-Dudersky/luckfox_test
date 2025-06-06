use slint::include_modules;

include_modules!();

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
