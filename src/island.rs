use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    fn new(name: String) -> Result<Self, String> {
        match name {
            s if s.is_empty() => Err("Island name is missing".to_string()),
            s if s.chars().count() > 10 => Err("Island name is too long".to_string()),
            s => Ok(Name(s)),
        }
    }
}

// implementation of the Into trait for the Name struct
// Into is the general conversion trait, the opposite direction is done with the From trait
impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }  
}

#[derive(Clone, Debug)]
pub struct Island {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: Name,
    pub is_active: bool,
}

impl Island {
    pub fn new(command: CreateIsland) -> Result<Self, String> {
        let name = Name::new(command.name)?;

        let island = Island {
            id: Uuid::new_v4(),
            owner_id: command.owner_id,
            name,
            is_active: command.is_active,
        };

        Ok(island)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateIsland {
    owner_id: Uuid,
    name: String,
    is_active: bool,
}

#[cfg(test)]
mod island_tests {
    use super::*;

    fn default_create_island() -> CreateIsland {
        CreateIsland {
            owner_id: Uuid::new_v4(),
            name: "valid name".into(),
            is_active: true,
        }
    }

    #[test]
    fn create_island_invalid_name() {
        let test_case = CreateIsland {
            name: "".into(),
            ..default_create_island()
        };

        assert!(Island::new(test_case).is_err());
    }

    #[test]
    fn create_island_valid_command() {
        let test_case = CreateIsland {
            owner_id: Uuid::new_v4(),
            name: "valid name".into(),
            is_active: true,
        };

        assert!(Island::new(test_case).is_ok());
    }
}

#[cfg(test)]
mod name_tests {
    use super::*;

    #[test]
    fn weird_name() {
        let test_case = "vy꙲ꙈᴫѱΆῨῨ";

        assert!(Name::new(test_case.into()).is_ok())
    }

    #[test]
    fn empty_string() {
        let test_case = "";

        assert!(Name::new(test_case.into()).is_err())
    }

    #[test]
    fn long_string() {
        let test_case = "12345678900";

        assert!(Name::new(test_case.into()).is_err())
    }
}