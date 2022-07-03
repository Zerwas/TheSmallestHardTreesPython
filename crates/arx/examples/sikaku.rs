//! Sikaku solver using the csp library
//!
//! Based on the sudoku solver by Ulrik Sverdrup "bluss"
//! https://github.com/bluss/dlx/tree/main/sudoku
/*

sikaku crate for Rust - Sikaku solver
Copyright (C) 2022 Michael Wernthaler "WhatDothLife"

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

*/

use std::error::Error;
use std::io::Read;
use std::{fmt, io};

use arx::solver::BackTrackSolver;
use arx::problem::*;

fn try_main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let sikaku = Sikaku::parse(&input)?;
    println!("{}", sikaku);

    let problem = sikaku.to_problem();
    let mut solutions = Vec::new();
    BackTrackSolver::new(&problem).solve_all(|sol| solutions.push(sol));

    for soln in &solutions {
        problem.print_solution(soln);
        println!();
    }

    match solutions.len() {
        0 => println!("No solution"),
        1 => println!("1 solution"),
        n => println!("{} solutions", n),
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        drop(e);
        std::process::exit(1);
    }
}

#[derive(Clone, Debug)]
pub enum ParseError {
    InvalidDigit(String),
    InvalidSize(String),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidSize(s) | Self::InvalidDigit(s) => s.fmt(f),
        }
    }
}

// is blank for sikaku
fn is_blank(s: &str) -> bool {
    match s {
        "." | "0" | "_" => true,
        _otherwise => false,
    }
}

fn is_spacer(s: &str) -> bool {
    match s {
        "" | ";" | "|" | "+" | "-" => true,
        _otherwise => false,
    }
}

/// Parse sikaku from multi-line representation
fn parse(s: &str) -> Result<Sikaku, ParseError> {
    let parts = s
        .split(char::is_whitespace)
        .filter(|s| !is_spacer(*s) && (s.len() <= 1 || !s.split("").all(is_spacer)))
        .map(|s| {
            if is_blank(s) {
                (s, Ok(None))
            } else {
                (s, Some(s.parse::<usize>()).transpose())
            }
        })
        .map(|(input, res)| {
            res.map_err(move |e| ParseError::InvalidDigit(format!("Error: {}: '{}'", e, input)))
        })
        .map(|res| res.map(|elt| elt.unwrap_or(0)))
        .collect::<Result<Vec<_>, _>>()?;

    if let Some(sz) = sikaku_size_for_len(parts.len()) {
        let mut sum = 0;
        for &part in &parts {
            if part > sz.pow(2) {
                return Err(ParseError::InvalidDigit(format!(
                    "Digit '{}' too large for sikaku size {}",
                    part, sz
                )));
            }
            sum += part;
        }
        if sum != parts.len() {
            return Err(ParseError::InvalidDigit(format!(
                "Sum of digits '{}' doesn't match sikaku size {}",
                sum, sz
            )));
        }
    } else {
        let len = parts.len();
        let msg = format!(
            "Unsupported size: got {} elements: {:?} [..]",
            len,
            &parts[..Ord::min(32, len)]
        );
        return Err(ParseError::InvalidSize(msg));
    }

    Ok(Sikaku { values: parts })
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
struct XY {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Sub for XY {
    type Output = XY;

    fn sub(self, rhs: Self) -> Self::Output {
        XY {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Rectangle {
    pub tl: XY, // top-left
    pub br: XY, // bottom-right
}

fn overlap(r1: Rectangle, r2: Rectangle) -> bool {
    // Above each other
    if r1.br.y < r2.tl.y || r2.br.y < r1.tl.y {
        return false;
    }
    // Next to each other
    if r1.tl.x > r2.br.x || r2.tl.x > r1.br.x {
        return false;
    }
    true
}

#[derive(Copy, Clone, Debug)]
struct Clue {
    pub pos: XY,
    pub value: usize,
}

impl std::fmt::Display for Clue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Clue(x: {:?}, y: {:?}, num:{:?})",
            self.pos.x, self.pos.y, self.value
        )
    }
}

fn factors(num: usize) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();

    for i in 1..=num {
        if num % i == 0 {
            factors.push((i, num / i))
        }
    }
    factors
}

#[derive(Clone)]
struct SikakuProblem {
    size: usize,
    clues: Vec<Clue>,
    rectangles: Vec<Vec<Rectangle>>,
}

impl SikakuProblem {
    fn print_solution(&self, sol: &Solution) {
        let mut values = vec!['0'; self.size * self.size];

        for (i, &a) in sol.iter().enumerate() {
            let rect = &self.rectangles[i][*a];

            for x in rect.tl.x..=rect.br.x {
                for y in rect.tl.y..=rect.br.y {
                    values[(y * self.size as i32 + x) as usize] =
                        char::from_u32(i as u32 + 97).unwrap();
                }
            }
        }
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{} ", values[j * self.size + i]);
            }
            println!();
        }
    }
}

impl Problem for SikakuProblem {}

impl Domains for SikakuProblem {
    fn size(&self) -> usize {
        self.clues.len()
    }

    fn domain(&self, x: Variable) -> Vec<Value> {
        (0..self.rectangles[*x].len()).map(|v| Value(v)).collect()
    }
}

impl Constraints for SikakuProblem {
    fn arcs(&self) -> Vec<(Variable, Variable)> {
        let mut arcs = Vec::new();

        for i in 0..self.clues.len() {
            for j in 0..self.clues.len() {
                if i != j {
                    arcs.push((Variable(i), Variable(j)));
                }
            }
        }
        arcs
    }

    fn check(&self, (xi, ai): (Variable, Value), (xj, aj): (Variable, Value)) -> bool {
        let r1 = self.rectangles[*xi][*ai];
        let r2 = self.rectangles[*xj][*aj];

        !overlap(r1, r2)
    }
}

impl fmt::Display for Sikaku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nu = self.sikaku_size();
        let empty_str = ".";

        for x in 0..nu {
            for y in 0..nu {
                let value = self.values[(x * nu + y) as usize];
                if value == 0 {
                    write!(f, "{} ", empty_str)?;
                } else {
                    write!(f, "{} ", value)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn sikaku_size_for_len(len: usize) -> Option<usize> {
    match len {
        4 => Some(2),
        9 => Some(3),
        16 => Some(4),
        25 => Some(5),
        36 => Some(6),
        49 => Some(7),
        64 => Some(8),
        81 => Some(9),
        100 => Some(10),
        _ => None,
    }
}

pub struct Sikaku {
    values: Vec<usize>,
}

impl Sikaku {
    /// Parse Sikaku from multi-line input
    fn parse(input: &str) -> Result<Self, ParseError> {
        parse(input)
    }

    fn sikaku_size(&self) -> usize {
        sikaku_size_for_len(self.values.len())
            .unwrap_or_else(|| unimplemented!("doesn't support this sikaku size"))
    }

    fn to_problem(&self) -> SikakuProblem {
        create_problem(self)
    }
}

fn is_inside_grid(rect: &Rectangle) -> bool {
    0 <= rect.tl.x
        && 0 <= rect.tl.y
        && 0 <= rect.br.x
        && 0 <= rect.br.y
        && 9 >= rect.tl.x
        && 9 >= rect.tl.y
        && 9 >= rect.br.x
        && 9 >= rect.br.y
}

fn rectangles_of(clue: Clue) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    for (a, b) in factors(clue.value) {
        for dx in 0..a as i32 {
            for dy in 0..b as i32 {
                let top_left = XY {
                    x: clue.pos.x - dx,
                    y: clue.pos.y + dy - b as i32 + 1,
                };
                let bottom_right = XY {
                    x: clue.pos.x - dx + a as i32 - 1,
                    y: clue.pos.y + dy,
                };
                let r = Rectangle {
                    tl: top_left,
                    br: bottom_right,
                };
                if is_inside_grid(&r) {
                    rectangles.push(r);
                }
            }
        }
    }
    rectangles
}

fn create_problem(sikaku: &Sikaku) -> SikakuProblem {
    let size = sikaku.sikaku_size();
    let mut clues = Vec::new();
    let mut rectangles = Vec::new();

    for x in 0..size {
        for y in 0..size {
            let value = sikaku.values[(x * size + y) as usize];

            if value != 0 {
                let clue = Clue {
                    pos: XY {
                        x: x as i32,
                        y: y as i32,
                    },
                    value,
                };
                clues.push(clue);
                rectangles.push(rectangles_of(clue));
            }
        }
    }

    SikakuProblem {
        clues,
        rectangles,
        size,
    }
}
