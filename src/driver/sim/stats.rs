
pub struct Stats {

}

impl Default for Stats {
    fn default() -> Self {
        Self {

        }
    }
}

impl std::fmt::Display for Stats {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("stats display not implemented yet");
        Ok(())
    }
}

impl Stats {
    pub fn accumulate (&mut self, state: &quattron::game::State) {
        
    }
}
