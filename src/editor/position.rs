
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            y: self.y.saturating_sub(other.y),
            x: self.x.saturating_sub(other.x),
        }
    }
}