fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Rectangle {
        x: 10px;
        y: 10px;
        width: 50px;
        height: 50px;
        background: blue;
    
    TextInput {
        text: "Replace me with a name";
        single-line:false;
        wrap:word-wrap;
        accepted => {}
    }
    }
}
}