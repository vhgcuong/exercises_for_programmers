slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        VerticalBox {
            Text { text: "Hello world"; }
            Button { text: "yay"; }
        }

    }
}

fn main() {
    App::new().unwrap().run().unwrap();
    println!("Hello, world!");
}
