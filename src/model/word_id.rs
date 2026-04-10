use serde::{Deserialize, Serialize};

macro_rules! word_ids {
    ($($name:ident = $id:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
}
