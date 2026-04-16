#![allow(non_camel_case_types)]

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

            pub const fn all() -> &'static [Self] {
                &[
                    $(Self::$name),*
                ]
            }

            pub fn all_ids() -> Vec<usize> {
                Self::all().iter().map(|word| word.id()).collect()
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
    // A1 unsorted
    Can_AbleTo = 32,
    ToUnderstand = 139,
    ToLove = 193,
    ToBuy = 573,
    ToHelp = 721,
    ToAsk = 109,
    ToLive = 117,
    ToSpeak = 53,
    ToWant = 90,
    ToCost = 1495,
    Person = 34,
    Beer = 1390,
    Coffee = 1579,
    Hotel = 1662,
    Taxi = 2754,
    Toilet = 2839,
    Sock = 2975,
    Bar = 7583,
    Supermarket = 12953,
    Tea = 763,
    Wine = 911,
    Restaurant = 1355,
    Bill = 658,
    Ruble = 587,
    October = 1717,
    Work = 132,
    House = 100,
    Water = 157,
    Russia = 217,
    And = 1,
    Not = 3,
    What_That = 7, 
    With_From_Since = 10,
    ThatIs = 13,
    Also_Too = 101,
    Possession_By_At_With_From = 20,
    From = 21,
    Still_Else_Yet = 36,
    Only_Just = 39,
    Yes = 48,
    Or = 54,
    No_There_is_Not = 59,
    Where = 69,
    There_Is = 59757,
    There = 77,
    Afterwards_Then = 85,
    Very = 87,
    Without = 93,
    One_Can_May = 103,
    Here = 108,
    Good = 154,
    Today = 234,
    Fast = 270,
    Girl_Girlfriend = 334,
    Slowly = 532,
    Difficulty = 615,
    Thanks = 769,
    Please_YouAreWelcome = 829,
    Right_Correct = 884,
    Coldly = 2197,
    Beautifully = 3077,
    Hotly = 3174,
    Free_Fluently = 3390,
    Tasty = 7411,
    Russian = 32405,
    Hello = 33002,
    ATM = 50682,
    Sorry = 52416,
    GoodBye = 59737,
    ToBeCalled = 59774,
    I = 6,
    You = 24,
    He = 4,
    She = 15,
    It = 288,
    We = 22,
    You_They_Formal = 25,
    They = 18,
    Who = 46,
    Other = 65,
    HowMuch = 243,

    // Vegetables
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

    // Fruits
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
    Pomegranate = 6526,
}
