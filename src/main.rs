use iced::{
    canvas::{Cursor, Geometry, Path},
    Align, Application, Button, Canvas, Clipboard, Color, Column, Command, Container, Element,
    Executor, HorizontalAlignment, Length, Point, Rectangle, Settings, Size, Text,
};
use rand::random;

const FIELD_SIZE: usize = 16;

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
            const MARGIN: f32 = 3.;
            let cell_width = frame.size().width/(FIELD_SIZE as f32);
            let cell_height = frame.size().height/(FIELD_SIZE as f32);

            let get_cell_fill = |i: usize, j: usize| -> (Path, Path) {
                let point= Point::new((i as f32) *cell_width, (j as f32)*cell_height);
                let size = Size::new(
                    cell_width,
                    cell_height,
                );
                let cell_background = Path::rectangle(point, size);
                let point= Point::new((i as f32) *cell_width + MARGIN, (j as f32)*cell_height + MARGIN);
                let size = Size::new(
                    cell_width - 2.*MARGIN,
                    cell_height - 2.*MARGIN,
                );
                let cell_content = Path::rectangle(point, size);
                (cell_background, cell_content)
            };

            let cells = self.cells.0;
            for i in 0..cells.len() {
                for j in 0..cells[i].len() {
                    let (cell_background, cell_content) = get_cell_fill(i, j);
                    frame.fill(&cell_background, Color::BLACK);
                    frame.fill(&cell_content, Color::WHITE);
                }
            }

            if let Some(Dices(x, y)) = self.dices {
                let dices_field = (0..x).flat_map(move |i| (0..y).map(move |j| get_cell_fill(i.into(), j.into())));
                for (cell_background, cell_content) in dices_field {
                    frame.fill(&cell_background, Color::from_rgb(0.9, 0.9, 0.9));
                    frame.fill(&cell_content, Color::from_rgb(1., 0., 0.));
                }
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
    type Message = Message;
    type Flags = ();

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
        _: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ThrowDices => {
                let new_dices: u8 = random();
                let dice1 = (new_dices >> 4) % 6 + 1;
                let dice2 = new_dices % 6 + 1;
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
