use iced::{
    canvas::{Cursor, Geometry, Path},
    Align, Application, Button, Canvas, Clipboard, Color, Column, Command, Container, Element,
    Executor, HorizontalAlignment, Length, Point, Rectangle, Settings, Size, Text,
};
use rand::random;

const FIELD_SIZE: usize = 64;

type CellOwner = Option<Player>;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone, Copy, Default)]
struct Dices(u8, u8);

#[derive(Debug)]
struct Game {
    state: GameState,
}

#[derive(Debug, Clone)]
struct Cells([[CellOwner; FIELD_SIZE]; FIELD_SIZE]);

#[derive(Debug)]
struct DrawingPart {
    dices: Option<Dices>,
    cells: Cells,
    canvas: iced::canvas::Cache,
}

#[derive(Debug)]
pub struct GameState {
    drawing: DrawingPart,
    roll_dice: iced::button::State,
    current_player: Player,
}

impl<Message> iced::canvas::Program<Message> for DrawingPart {
    fn draw(&self, bounds: Rectangle<f32>, cursor: Cursor) -> Vec<Geometry> {
        let x = self.canvas.draw(bounds.size(), |frame| {
            let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());
            frame.fill(&space, Color::BLACK);
            if let Some(Dices(x, y)) = self.dices {
                let dice_throw = Path::rectangle(
                    Point::new(0.0, 0.0),
                    Size::new(
                        frame.size().width / (x as f32),
                        frame.size().height / (y as f32),
                    ),
                );
                frame.fill(&dice_throw, Color::from_rgb(0., 1., 0.));
            }
        });
        vec![x]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ThrowDices,
    ChangeOwner { player: Player, x: usize, y: usize },
}

fn color_changed(player: Player, x: usize, y: usize) -> Message {
    Message::ChangeOwner { player, x, y }
}

impl Application for Game {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Game {
                state: GameState {
                    drawing: DrawingPart {
                        dices: Default::default(),
                        cells: Cells([[None; FIELD_SIZE]; FIELD_SIZE]),
                        canvas: Default::default(),
                    },
                    roll_dice: iced::button::State::new(),
                    current_player: Player::Player1,
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dice wars")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ThrowDices => {
                let new_dices: u8 = random();
                let dice1 = (new_dices >> 4) % 5 + 1;
                let dice2 = new_dices % 5 + 1;
                self.state.drawing.dices = Some(Dices(dice1, dice2))
            },
            _ => {},
        }
        self.state.drawing.canvas.clear();
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(
                Button::new(&mut self.state.roll_dice, Text::new("Throw dices!"))
                    .on_press(Message::ThrowDices),
            )
            .push(
                Canvas::new(&mut self.state.drawing)
                    .width(Length::Fill)
                    .height(Length::Fill),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.resizable = false;
    Game::run(settings)
}
