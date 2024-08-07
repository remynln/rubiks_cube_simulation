
#[derive(Debug, Clone, Copy)]
pub enum SideRotation {
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct SideMoveEvent {
    pub side: (Axis, f32),
    pub rotate: SideRotation,
}

#[derive(Debug, Resource)]
pub struct SideMoveQueue(pub VecDeque<SideMoveEvent>);

#[derive(Debug, Resource)]
pub struct MouseDraggingRecorder {
    pub start_pos: Option<Vec3>,
    pub piece: Option<Entity>,
}

impl MouseDraggingRecorder {
    pub fn clear(&mut self) {
        self.start_pos = None;
        self.piece = None;
    }
}