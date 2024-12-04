use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoordChar {
    x: usize,
    y: usize,
    char: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FoundWord {
    x: CoordChar,
    m: CoordChar,
    a: CoordChar,
    s: CoordChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FoundCrossedMas {
    m_up: CoordChar,
    s_up: CoordChar,
    a: CoordChar,
    m_down: CoordChar,
    s_down: CoordChar,
}

impl FoundCrossedMas {
    fn is_crossed_mas(&self) -> bool {
        self.m_up.char == 'M'
            && self.m_down.char == 'M'
            && self.a.char == 'A'
            && self.s_up.char == 'S'
            && self.s_down.char == 'S'
    }
}

impl FoundWord {
    fn is_xmas(&self) -> bool {
        self.x.char == 'X' && self.m.char == 'M' && self.a.char == 'A' && self.s.char == 'S'
    }
}

pub struct Puzzle {
    grid: [[char; 140]; 140],
}

impl Puzzle {
    pub fn new(input: &str) -> Self {
        let content = fs::read_to_string(input).expect("Failed to read file");
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() != 140 {
            panic!("File does not contain exactly 140 lines");
        }

        let mut grid = [[' '; 140]; 140];
        for (i, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() != 140 {
                panic!("Line {} does not contain exactly 140 characters", i + 1);
            }
            for (j, &c) in chars.iter().enumerate() {
                grid[i][j] = c;
            }
        }

        Self { grid }
    }

    fn access(&self, coord: (usize, usize)) -> Option<CoordChar> {
        let row = self.grid.get(coord.0)?;
        let char = row.get(coord.1)?;

        Some(CoordChar {
            x: coord.0,
            y: coord.1,
            char: *char,
        })
    }

    fn view_horizontal(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let x = self.access((coord.0, coord.1))?;
        let m = self.access((coord.0 + 1, coord.1))?;
        let a = self.access((coord.0 + 2, coord.1))?;
        let s = self.access((coord.0 + 3, coord.1))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_horizontal_reverse(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let s = self.access((coord.0, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1))?;
        let m = self.access((coord.0 + 2, coord.1))?;
        let x = self.access((coord.0 + 3, coord.1))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_vertical(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let x = self.access((coord.0, coord.1))?;
        let m = self.access((coord.0, coord.1 + 1))?;
        let a = self.access((coord.0, coord.1 + 2))?;
        let s = self.access((coord.0, coord.1 + 3))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_vertical_reverse(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let s = self.access((coord.0, coord.1))?;
        let a = self.access((coord.0, coord.1 + 1))?;
        let m = self.access((coord.0, coord.1 + 2))?;
        let x = self.access((coord.0, coord.1 + 3))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_diagonal_slash(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let x = self.access((coord.0, coord.1))?;
        let m = self.access((coord.0 + 1, coord.1 + 1))?;
        let a = self.access((coord.0 + 2, coord.1 + 2))?;
        let s = self.access((coord.0 + 3, coord.1 + 3))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_diagonal_slash_reverse(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let s = self.access((coord.0, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1 + 1))?;
        let m = self.access((coord.0 + 2, coord.1 + 2))?;
        let x = self.access((coord.0 + 3, coord.1 + 3))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_diagonal_backslash(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let x = self.access((coord.0, coord.1))?;
        let m = self.access((coord.0 + 1, coord.1.checked_sub(1)?))?;
        let a = self.access((coord.0 + 2, coord.1.checked_sub(2)?))?;
        let s = self.access((coord.0 + 3, coord.1.checked_sub(3)?))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_diagonal_backslash_reverse(&self, coord: (usize, usize)) -> Option<FoundWord> {
        let s = self.access((coord.0, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1.checked_sub(1)?))?;
        let m = self.access((coord.0 + 2, coord.1.checked_sub(2)?))?;
        let x = self.access((coord.0 + 3, coord.1.checked_sub(3)?))?;

        Some(FoundWord { x, m, a, s })
    }

    fn view_crossed_mas_north(&self, coord: (usize, usize)) -> Option<FoundCrossedMas> {
        let m_up = self.access((coord.0, coord.1))?;
        let s_up = self.access((coord.0 + 2, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1 + 1))?;
        let m_down = self.access((coord.0, coord.1 + 2))?;
        let s_down = self.access((coord.0 + 2, coord.1 + 2))?;

        Some(FoundCrossedMas {
            m_up,
            s_down,
            a,
            m_down,
            s_up,
        })
    }

    fn view_crossed_mas_east(&self, coord: (usize, usize)) -> Option<FoundCrossedMas> {
        let m_down = self.access((coord.0, coord.1))?;
        let m_up = self.access((coord.0 + 2, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1 + 1))?;
        let s_down = self.access((coord.0, coord.1 + 2))?;
        let s_up = self.access((coord.0 + 2, coord.1 + 2))?;

        Some(FoundCrossedMas {
            m_up,
            s_down,
            a,
            m_down,
            s_up,
        })
    }

    fn view_crossed_mas_south(&self, coord: (usize, usize)) -> Option<FoundCrossedMas> {
        let s_down = self.access((coord.0, coord.1))?;
        let m_down = self.access((coord.0 + 2, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1 + 1))?;
        let s_up = self.access((coord.0, coord.1 + 2))?;
        let m_up = self.access((coord.0 + 2, coord.1 + 2))?;

        Some(FoundCrossedMas {
            m_up,
            s_down,
            a,
            m_down,
            s_up,
        })
    }

    fn view_crossed_mas_west(&self, coord: (usize, usize)) -> Option<FoundCrossedMas> {
        let s_up = self.access((coord.0, coord.1))?;
        let s_down = self.access((coord.0 + 2, coord.1))?;
        let a = self.access((coord.0 + 1, coord.1 + 1))?;
        let m_up = self.access((coord.0, coord.1 + 2))?;
        let m_down = self.access((coord.0 + 2, coord.1 + 2))?;

        Some(FoundCrossedMas {
            m_up,
            s_down,
            a,
            m_down,
            s_up,
        })
    }

    pub fn solve(&self) -> usize {
        let mut all_words: HashSet<FoundWord> = HashSet::new();

        let mut all_crossed_mas: HashSet<FoundCrossedMas> = HashSet::new();

        for x in 0..140 {
            for y in 0..140 {
                let coord = (x, y);

                macro_rules! insert_view {
                    ($view:ident) => {
                        if let Some(word) = self.$view(coord) {
                            all_words.insert(word);
                        }
                    };
                }

                insert_view!(view_horizontal);
                insert_view!(view_horizontal_reverse);
                insert_view!(view_vertical);
                insert_view!(view_vertical_reverse);
                insert_view!(view_diagonal_slash);
                insert_view!(view_diagonal_slash_reverse);
                insert_view!(view_diagonal_backslash);
                insert_view!(view_diagonal_backslash_reverse);

                macro_rules! insert_mas_view {
                    ($view:ident) => {
                        if let Some(word) = self.$view(coord) {
                            all_crossed_mas.insert(word);
                        }
                    };
                }

                insert_mas_view!(view_crossed_mas_north);
                insert_mas_view!(view_crossed_mas_east);
                insert_mas_view!(view_crossed_mas_south);
                insert_mas_view!(view_crossed_mas_west);
            }
        }

        let correct_words = all_words.iter().filter(|w| w.is_xmas()).collect::<Vec<_>>();

        println!("{}/{}", correct_words.len(), all_words.len());

        let correct_crossed_mas = all_crossed_mas
            .iter()
            .filter(|c| c.is_crossed_mas())
            .collect::<Vec<_>>();

        println!("{}/{}", correct_crossed_mas.len(), all_crossed_mas.len());

        correct_words.len()
    }
}
