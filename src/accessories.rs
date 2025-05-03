use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Accessory {
    None,
    Hat,
    Glasses,
    Collar,
}
