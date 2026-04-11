use serde::{Deserialize, Serialize};

macro_rules! word_ids {
    ($($name:ident = $id:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum WordId {
            $($name),*
        }

        impl WordId {
            pub const fn id(self) -> usize {
                match self {
                    $(Self::$name => $id),*
                }
            }

            pub const fn from_id(value: usize) -> Option<Self> {
                match value {
                    $($id => Some(Self::$name),)*
                    _ => None,
                }
            }
        }

        impl TryFrom<usize> for WordId {
            type Error = ();

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Self::from_id(value).ok_or(())
            }
        }

        impl From<WordId> for usize {
            fn from(value: WordId) -> Self {
                value.id()
            }
        }
    };
}

impl std::fmt::Display for WordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id().fmt(f)
    }
}

word_ids! {
    Garlic = 10770,
    Vegetable = 5166,
    Tomato = 6379,
    Broccoli = 60305,
    Avocado = 60291,
    Cucumber = 4629,
    Carrote = 12870,
    Onion = 2333,
    Potato = 9013,
    Salat = 7337,
    Cabbage = 4054,
    Lentil = 59928,
    Spinach = 59842,
    Courgette = 13710,
    Eggplant = 26810,
    Pumpkin = 11111,
    Corn = 7326,
    Pea = 8536,
    Bean = 20785,
    Radish = 47793,
    Argugula = 60608,
    Asparagus = 49245,
    Paprika = 60104,
    Mushroom = 1830,

    Fruit = 4080,
    Apple = 2471,
    Banana = 10845,
    Orange = 7363,
    Strawberry = 17379,
    Blueberry = 20545,
    Raspberry = 10981,
    Blackberry = 22980,
    Pear = 5497,
    Peach = 12092,
    Plum = 12834,
    Cherry = 12175,
    Kiwi = 60319,
    Pineapple = 17111,
    Mango = 40850,
    Watermelon = 8884,
    Lemon = 5650,
    Lime = 22711,
    Pomegranate = 6526,
}
