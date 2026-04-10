use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Word {
    pub content: String,
    pub stress: Option<usize>,
}

impl Word {
    pub fn from_stressed(stressed: &str) -> Self {
        let mut content = String::new();
        let mut stress = None;
        let mut char_index = 0;
        let mut prev_char_index = None;

        for c in stressed.chars() {
            if c == '\'' {
                if let Some(idx) = prev_char_index {
                    stress = Some(idx);
                }
            } else {
                content.push(c);
                prev_char_index = Some(char_index);
                char_index += 1;
            }
        }

        Self { content, stress }
    }

    pub fn syllables(&self) -> Vec<String> {
        let mut syllables = Vec::new();

        for subword in self.content.split('-') {
            if !subword.is_empty() {
                syllables.extend(token_to_syllables(subword));
                syllables.push("-".to_string());
            } else {
                syllables.push("-".to_string());
            }
        }

        if !syllables.is_empty() {
            syllables.pop();
        }

        syllables
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'а' | 'е'
            | 'ё'
            | 'и'
            | 'о'
            | 'у'
            | 'ы'
            | 'э'
            | 'ю'
            | 'я'
            | 'А'
            | 'Е'
            | 'Ё'
            | 'И'
            | 'О'
            | 'У'
            | 'Ы'
            | 'Э'
            | 'Ю'
            | 'Я'
    )
}

fn is_unpaired_sonant(c: char) -> bool {
    matches!(c, 'р' | 'л' | 'м' | 'н' | 'Р' | 'Л' | 'М' | 'Н')
}

fn is_sign(c: char) -> bool {
    matches!(c, 'ь' | 'ъ' | 'Ь' | 'Ъ')
}

fn is_r(c: char) -> bool {
    matches!(c, 'р' | 'Р')
}

fn is_zh(c: char) -> bool {
    matches!(c, 'ж' | 'Ж')
}

fn is_yot(c: char) -> bool {
    matches!(c, 'й' | 'Й')
}

fn token_to_syllables(token: &str) -> Vec<String> {
    let chars: Vec<char> = token.chars().collect();

    if chars.len() < 2 {
        return vec![token.to_string()];
    }

    let mut break_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter_map(|(i, &ch)| if is_vowel(ch) { Some(i + 1) } else { None })
        .collect();

    if break_indices.len() < 2 {
        return vec![token.to_string()];
    }

    for i in 0..break_indices.len() - 1 {
        let idx = break_indices[i];
        let letter = chars.get(idx).copied();
        let next_letter = chars.get(idx + 1).copied();

        if let Some(letter) = letter {
            match next_letter {
                Some(next)
                    if is_unpaired_sonant(letter)
                        && !is_vowel(next)
                        && letter != next
                        && !(is_r(letter) && is_zh(next))
                        && !is_sign(next) =>
                {
                    break_indices[i] += 1;
                }
                Some(next) if is_unpaired_sonant(letter) && is_sign(next) => {
                    break_indices[i] += 2;
                }
                Some(next) if is_yot(letter) && !is_vowel(next) => {
                    break_indices[i] += 1;
                }
                _ => {}
            }
        }
    }

    break_indices.insert(0, 0);
    break_indices.pop();

    let mut result = Vec::new();
    for i in 0..break_indices.len() {
        let start = break_indices[i];
        let end = break_indices.get(i + 1).copied().unwrap_or(chars.len());
        result.push(chars[start..end].iter().collect());
    }

    result
}
