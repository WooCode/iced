use iced::{
    button, scrollable, text_input, Align, Button, Checkbox, Column, Command,
    Container, Element, Font, HorizontalAlignment, Length, Row, Sandbox,
    Scrollable, Settings, Text, TextInput,
};

pub fn main() {
    LongList::run(Settings::default())
}

#[derive(Debug, Default)]
struct LongList {
    scroll: scrollable::State,
    tasks: Vec<Task>,
}

#[derive(Debug, Default)]
struct Task {
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for LongList {
    type Message = Message;

    fn new() -> Self {
        let mut tasks = Vec::new();
        for i in 0..1_000 {
            tasks.push(Task {
                text: format!("Row {}", i),
            })
        }

        LongList {
            scroll: Default::default(),
            tasks: tasks,
        }
    }

    fn title(&self) -> String {
        String::from("Long List - Iced")
    }

    fn update(&mut self, _message: Message) {
        Default::default()
    }

    fn view(&mut self) -> Element<Message> {
        let LongList { scroll, tasks } = self;

        let title = Text::new("LongList")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let mut rows = Column::new();
        for task in self.tasks.iter() {
            rows = rows.push(Text::new(task.text.to_string()).size(20));
        }

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(rows);

        Scrollable::new(scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }
}
