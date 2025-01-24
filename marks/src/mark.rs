

#[derive(Clone, Copy, Debug)]
pub enum Shapes {
    Circle,
    Ellipse,
}



pub struct Mark {
    shape: Shapes,
    color: vello::peniko::Color,
    position: (f64, f64),
}

impl Mark {

    pub fn new(shape: Shapes, color: vello::peniko::Color, position: (f64, f64)) -> Mark {
        return Mark { shape: shape, color: color, position: position};
    }

    // Fetches the basic information of marks
    // TODO: Set shapes' width and height.  

    pub fn get_shape(&self) -> Shapes {
        self.shape
    }

    pub fn shape(&mut self, shape: Shapes) {
        self.shape = shape;
    }

    pub fn get_color(&self) -> vello::peniko::Color {
        self.color
    }

    pub fn color(&mut self, color: vello::peniko::Color) {
        self.color = color;
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn position(&mut self, position: (f64, f64)) {
        self.position = position;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let mut mark1 = Mark::new(Shapes::Circle, vello::peniko::Color::new([0.9529, 0.5451, 0.6588, 1.]), (200.0, 200.0));
        matches!(mark1.get_shape(), Shapes::Circle);
        mark1.shape(Shapes::Ellipse);
        matches!(mark1.get_shape(), Shapes::Ellipse);
    }

    #[test]
    fn test_color() {
        let color1 = vello::peniko::Color::new([0.9529, 0.5451, 0.6588, 1.]);
        let mut mark1 = Mark::new(Shapes::Circle, color1, (200.0, 200.0));
        matches!(mark1.get_color(), _color1);
        let color2 = vello::peniko::Color::new([0.6529, 0.2451, 0.3241, 1.]);
        mark1.color(color2);
        matches!(mark1.get_color(), _color2);
    }

    #[test]
    fn test_position() {
        let color = vello::peniko::Color::new([0.9529, 0.5451, 0.6588, 1.]);
        let mut mark1 = Mark::new(Shapes::Circle, color, (200.0, 200.0));
        matches!(mark1.get_position(), (200.0, 200.0));
        mark1.position((600.0, 200.0));
        matches!(mark1.get_position(), (600.0, 200.0));
    }

}