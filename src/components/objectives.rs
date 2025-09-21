#[derive(Debug, Clone, PartialEq)]
pub enum Objective {
    OrbitEarth,
    OrbitMoon,
    LandOnMoon,
}

impl Objective {
    pub fn description(&self) -> &'static str {
        match self {
            Objective::OrbitEarth => "Achieve stable low Earth orbit (250-400 km altitude)",
            Objective::OrbitMoon => "Achieve stable orbit around the Moon",
            Objective::LandOnMoon => "Successfully land on the Moon",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Objective::OrbitEarth => "Low Earth Orbit",
            Objective::OrbitMoon => "Moon Orbit",
            Objective::LandOnMoon => "Moon Landing",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectiveProgress {
    pub current: Objective,
    pub completed: Vec<Objective>,
    pub is_completed: bool,
    pub completion_time: Option<f32>,
}

impl Default for ObjectiveProgress {
    fn default() -> Self {
        Self {
            current: Objective::OrbitEarth,
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
                Objective::OrbitEarth => self.current = Objective::OrbitMoon,
                Objective::OrbitMoon => self.current = Objective::LandOnMoon,
                Objective::LandOnMoon => {
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