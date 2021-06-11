use frusk::Frusk;

fn main() {
    let app = Frusk::new(String::from("127.0.0.1"), 3000);
    app.run();
}
