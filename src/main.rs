use iced::{
    canvas::{Cursor, Geometry, Path},
    Align, Application, Button, Canvas, Clipboard, Color, Column, Command, Container, Element,
    Executor, HorizontalAlignment, Length, Point, Rectangle, Settings, Size, Text,
};
use rand::random;
use iced::canvas::{event, Event, Frame};

const FIELD_SIZE: usize = 16;
const MARGIN: f32 = 2.;

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

#[derive(Debug, Clone, Copy)]
struct PointsTranslator {
    cell_width: f32,
    cell_height: f32,
}

impl PointsTranslator {
    pub fn new(bounds: &Rectangle) -> Self {
        let cell_width = bounds.width/(FIELD_SIZE as f32);
        let cell_height = bounds.height/(FIELD_SIZE as f32);
        Self {
             cell_width,
             cell_height
        }
    }

    pub fn coords_to_position(&self, i: usize, j: usize) -> Point {
        Point::new((i as f32) *self.cell_width, (j as f32)*self.cell_height)
    }

    pub fn position_to_coords(&self, position: &Point) -> (usize, usize) {
        let i = (position.x / self.cell_width as f32).ceil() as usize;
        let j = (position.y / self.cell_height as f32).ceil() as usize;

        (i.saturating_sub(1), j.saturating_sub(2))
    }

    pub fn get_cell_fill(&self, i: usize, j: usize) -> (Path, Path) {
        let point = self.coords_to_position(i, j);
        let size = Size::new(
            self.cell_width,
            self.cell_height,
        );
        let cell_background = Path::rectangle(point, size);
        let mut point= Point::new(point.x + MARGIN, point.y + MARGIN);
        let size = Size::new(
            self.cell_width - 2.*MARGIN,
            self.cell_height - 2.*MARGIN,
        );
        let cell_content = Path::rectangle(point, size);
        (cell_background, cell_content)
    }
}

impl<Message> iced::canvas::Program<Message> for DrawingPart {
    // fn update(
    //     &mut self,
    //     event: Event,
    //     bounds: Rectangle,
    //     cursor: Cursor,
    // ) -> (event::Status, Option<Message>) {
    //     if !cursor.is_over(&bounds) {
    //         return (event::Status::Ignored, None);
    //     }
    //     match event {
    //         Event::Mouse(iced::mouse::Event::CursorMoved {position}) => {
    //             dbg!(position);
    //             (event::Status::Captured, None)
    //         }
    //         _ => (event::Status::Ignored, None)
    //     }
    // }

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let translator = PointsTranslator::new(&bounds);
        let x = self.canvas.draw(bounds.size(), |frame| {
            let cells = self.cells.0;
            for i in 0..cells.len() {
                for j in 0..cells[i].len() {
                    let (cell_background, cell_content) = translator.get_cell_fill(i, j);
                    frame.fill(&cell_background, Color::BLACK);
                    frame.fill(&cell_content, Color::WHITE);
                }
            }

            if let Some(Dices(x, y)) = self.dices {
                let dices_field = (0..x).flat_map(move |i| (0..y).map(move |j| translator.get_cell_fill(i.into(), j.into())));
                for (cell_background, cell_content) in dices_field {
                    frame.fill(&cell_background, Color::from_rgb(0.9, 0.9, 0.9));
                    frame.fill(&cell_content, Color::from_rgb(1., 0., 0.));
                }
            }
        });
        let overlay = {
            let mut frame = Frame::new(bounds.size());
            let current_pos = match cursor.position() {
                Some(pos) if cursor.is_over(&bounds) => {
                    dbg!(&pos);
                    dbg!(translator.position_to_coords(&pos));
                    Some(pos)
                }
                _ => None
            };

            if let Some(pos) = current_pos {
                let (i,j) = translator.position_to_coords(&pos);
                let (cell_background, cell_content) = translator.get_cell_fill(i, j);

                frame.fill(&cell_background, Color::from_rgb(0.5, 0.5, 0.5));
                frame.fill(&cell_content, Color::from_rgb(0., 1., 0.));
            }
            frame.into_geometry()
        };
        vec![x, overlay]
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
                Button::new(&mut self.state.roll_dice, Text::new("Roll the dice!"))
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
