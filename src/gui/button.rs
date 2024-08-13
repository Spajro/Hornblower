use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::figure::Figure;
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::square::Square;
use crate::gui::area::Area;
use crate::gui::click::{Click, ClickHandler};


pub struct Button {
    area: Area,
    figure: Figure,
    click: bool,
}

impl Button {
    pub fn new(center: Point, size: u32) -> Self {
        Button {
            area: Area::new(
                center.x - (size / 2),
                center.y - (size / 2),
                center.x + (size / 2),
                center.y + (size / 2),
            ),
            figure: Figure::new()
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), 2 * size)))
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), (1.8 * size as f32) as u32))),
            click: false,
        }
    }

    pub fn with_plus(center: Point, size: u32) -> Self {
        let half_size = size / 2;
        Button {
            area: Area::new(
                center.x - (size / 2),
                center.y - (size / 2),
                center.x + (size / 2),
                center.y + (size / 2),
            ),
            figure: Figure::new()
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), 2 * size)))
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), (1.8 * size as f32) as u32)))
                .with(Box::new(Line::new(Point::new(center.x - half_size, center.y), Point::new(center.x + half_size, center.y))))
                .with(Box::new(Line::new(Point::new(center.x, center.y - half_size), Point::new(center.x, center.y + half_size)))),
            click: false,
        }
    }

    pub fn with_minus(center: Point, size: u32) -> Self {
        let half_size = size / 2;
        Button {
            area: Area::new(
                center.x - (size / 2),
                center.y - (size / 2),
                center.x + (size / 2),
                center.y + (size / 2),
            ),
            figure: Figure::new()
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), 2 * size)))
                .with(Box::new(Square::new(center, Normalized::new(0.0, -1.0), (1.8 * size as f32) as u32)))
                .with(Box::new(Line::new(Point::new(center.x - half_size, center.y), Point::new(center.x + half_size, center.y)))),
            click: false,
        }
    }

    pub fn clicked(&mut self) -> bool {
        let r = self.click;
        self.click = false;
        r
    }
}

impl Paintable for Button {
    fn paint(&self, buffer: &mut Buffer) {
        self.figure.paint(buffer);
    }
}

impl ClickHandler for Button {
    fn handle_click(&mut self, click: &Click) {
        if self.area.is_in(click) {
            self.click = true;
        }
    }
}