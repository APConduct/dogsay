use crate::accessories::Accessory;
use std::fmt;

pub enum DogBreed {
    // Labrador,
    // Poodle,
    // GermanShepherd,
    WhateverBrianIs,
    Weirdo,
    Wolf,
    GreatDamn,
    Basset,
}

pub struct Dog {
    breed: DogBreed,
    accessory: Accessory,
}

impl Dog {
    pub fn new(breed: DogBreed, accessory: Accessory) -> Self {
        Self { breed, accessory }
    }

    fn get_art(self) -> String {
        match self.breed {
            DogBreed::WhateverBrianIs => r#"
                ╭━┳━╭━╭━╮╮
                ┃┈┈┈┣▅╋▅┫┃
                ┃┈┃┈╰━╰━━━━━━╮
                ╰┳╯┈┈┈┈┈┈┈┈┈◢▉◣
                ╲┃┈┈┈┈┈┈┈┈┈┈▉▉▉
                ╲┃┈┈┈┈┈┈┈┈┈┈◥▉◤
                ╲┃┈┈┈┈╭━┳━━━━╯
                ╲┣━━━━━━┫
                "#
            .to_string(),
            DogBreed::Weirdo => r#"

            ┈╭━━━━━━━━━━━╮┈
            ╭╯┈╭━━╮┈╭━━╮┈╰╮
            ┃┈┃┃╭╮┃┈┃╭╮┃┃┈┃
            ┃┈┃┻┻┻┛┈┗┻┻┻┃┈┃
            ┃┈┃╭╮┈◢▇◣┈╭╮┃┈┃
            ╰┳╯┃╰━━┳┳┳╯┃╰┳╯
            ┈┃┈╰━━━┫┃┣━╯┈┃┈
            ┈┃┈┈┈┈┈╰━╯┈┈┈┃┈
            "#
            .to_string(),
            DogBreed::Wolf => r#"
            ╱▏┈┈┈┈┈┈▕╲▕╲┈┈┈
            ▏▏┈┈┈┈┈┈▕▏▔▔╲┈┈
            ▏╲┈┈┈┈┈┈╱┈▔┈▔╲┈
            ╲▏▔▔▔▔▔▔╯╯╰┳━━▀
            ┈▏╯╯╯╯╯╯╯╯╱┃┈┈┈
            ┈┃┏┳┳━━━┫┣┳┃┈┈┈
            ┈┃┃┃┃┈┈┈┃┃┃┃┈┈┈
            ┈┗┛┗┛┈┈┈┗┛┗┛┈┈┈
                "#
            .to_string(),
            DogBreed::GreatDamn => r#"
                __      _
                \.'---.//|
                 |\./|  \/
                _|.|.|_  \
               /(  ) ' '  \
              |  \/   . |  \
               \_/\__/| |
                V  /V / |
                  /__/ /
                  \___/\

            "#
            .to_string(),
            DogBreed::Basset => r#"
                ___       ___
               |   \_____/   |
              /  |\/     \/|  \
              \_/ | /\ /\ | \_/
                  |_\/ \/_|
                 /   \o/   \
                 \___/"\___/
                "#
            .to_string(),
        }
    }
}
