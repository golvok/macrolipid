pub struct Lipid {
    pub head_position: (f32, f32),
    pub tail_position: (f32, f32),
    pub head_radius: f32,
}

pub enum MoleculeEnum {
    Lipid(Lipid),
}
