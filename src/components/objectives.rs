/// Game objectives that players must complete in sequence
#[derive(Debug, Clone, PartialEq)]
pub enum Objective {
    /// Escape Moon's gravitational influence
    EscapeMoon,
    /// Achieve stable orbit around Earth
    OrbitEarth,
    /// Successfully land on Earth's surface
    LandOnEarth,
}

impl Objective {
    pub fn title(&self) -> &'static str {
        match self {
            Objective::EscapeMoon => "Escape Moon",
            Objective::OrbitEarth => "Earth Orbit",
            Objective::LandOnEarth => "Earth Landing",
        }
    }
}

/// Tracks the player's progress through game objectives
#[derive(Debug, Clone)]
pub struct ObjectiveProgress {
    /// The objective currently being pursued
    pub current: Objective,
    /// List of objectives that have been completed
    pub completed: Vec<Objective>,
    /// Whether the current objective has been completed
    pub is_completed: bool,
    /// Time when the objective was completed (if any)
    pub completion_time: Option<f32>,
}

impl Default for ObjectiveProgress {
    fn default() -> Self {
        Self {
            current: Objective::EscapeMoon,
            completed: Vec::new(),
            is_completed: false,
            completion_time: None,
        }
    }
}

impl ObjectiveProgress {
    pub fn complete_current(&mut self, time: f32) {
        if !self.is_completed {
            self.completed.push(self.current.clone());
            self.completion_time = Some(time);
            self.is_completed = true;
        }
    }

    pub fn advance_to_next(&mut self) {
        if self.is_completed {
            match self.current {
                Objective::EscapeMoon => self.current = Objective::OrbitEarth,
                Objective::OrbitEarth => self.current = Objective::LandOnEarth,
                Objective::LandOnEarth => {
                    // All objectives completed
                    return;
                }
            }
            self.is_completed = false;
            self.completion_time = None;
        }
    }

    pub fn all_completed(&self) -> bool {
        self.completed.len() == 3
    }
}
