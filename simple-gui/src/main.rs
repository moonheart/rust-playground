pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len() + 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "{}", self.label.as_str()).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "{}", "+").unwrap();
        for _i in 0..self.label.width() {
            write!(buffer, "{}", "-").unwrap();
        }
        write!(buffer, "{}\n", "+").unwrap();
        write!(buffer, "| {} |\n", self.label.label).unwrap();
        write!(buffer, "{}", "+").unwrap();
        for _i in 0..self.label.width() {
            write!(buffer, "{}", "-").unwrap();
        }
        write!(buffer, "{}\n", "+").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        let mut width = 0;
        for x in self.widgets.iter() {
            if x.width() > width {
                width = x.width()
            }
        }
        width
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        write!(buffer, "{}", "+").unwrap();
        for i in 0..width {
            write!(buffer, "{}", "-").unwrap();
        }
        write!(buffer, "{}\n", "+").unwrap();
        write!(buffer, "|{: ^width$}|\n", self.title).unwrap();
        write!(buffer, "{}", "+").unwrap();
        for i in 0..width {
            write!(buffer, "{}", "=").unwrap();
        }
        write!(buffer, "{}\n", "+").unwrap();
        for x in self.widgets.iter() {
            let mut buffer1 = String::new();
            x.draw_into(&mut buffer1);
            for line in buffer1.lines() {
                write!(buffer, "|{: <width$}|\n", line).unwrap();
            }
        }
        let width = self.width();
        write!(buffer, "{}", "+").unwrap();
        for i in 0..width {
            write!(buffer, "{}", "-").unwrap();
        }
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
