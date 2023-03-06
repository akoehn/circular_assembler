/* Copyright 2022-2023 Arne Köhn <arne@chark.eu>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use bitvec::prelude::*;
use std::cmp;

type PuzzArray = BitArr!(for 36);

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub flipped: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

pub type Piece = Vec<Coord>;

pub fn find_assemblies(
    pieces: &Vec<Piece>,
    target: &Piece,
    must_fill: &Piece,
) -> Vec<Vec<Position>> {
    let target_matrix = piece_to_matrix(
        &target,
        &Position {
            x: 0,
            y: 0,
            flipped: false,
        },
    );
    let must_fill_matrix = piece_to_matrix(
        &must_fill,
        &Position {
            x: 0,
            y: 0,
            flipped: false,
        },
    );
    let mut pieces_positions: Vec<(&Piece, Vec<(Position, PuzzArray)>)> = Vec::new();
    for p in pieces {
        let pos_arr_list = get_matrices_for_piece(p, &target_matrix);
        pieces_positions.push((p, pos_arr_list));
    }

    // TODO: handle duplicates by starting at previous piece position +1 instead of 0

    let mut result: Vec<Vec<Position>> = Vec::new();

    let num_pieces = pieces.len();
    let max_positions = pieces_positions
        .iter()
        .map(|x| x.1.len() - 1)
        .collect::<Vec<_>>();

    let mut puzz_arrays: Vec<PuzzArray> = vec![bitarr!(0;36); num_pieces];
    let mut position_indices: Vec<usize> = vec![0; num_pieces];
    let mut curr_piece_index = 0;

    let mut num_iterations = 0;

    /*
    iterate over piece number
    for every piece number: store current and maximal position index & puzzarray
    step: fill puzzarray of curr piece with position, check if possible. if yes: increase piece number
    if no, next position. if last position & position is possible: add to results
     */

    loop {
        num_iterations += 1;

        let curr_position = position_indices[curr_piece_index];

        if curr_position == max_positions[curr_piece_index] {
            // we are at the end for this current piece, either:
            //  - we are done (if this is the initial piece)
            // or:
            //  - we continue with the next position of the previous
            //    piece and the first of this and all subequent ones
            if curr_piece_index == 0 {
                // we are done
                break;
            } else {
                for idx in curr_piece_index..num_pieces {
                    position_indices[idx] = 0;
                }
                position_indices[curr_piece_index - 1] += 1;
                curr_piece_index -= 1;
                continue;
            }
        }

        if curr_piece_index == 0 {
            // initialize with empty array as base
            puzz_arrays[0] = bitarr![0;36] | pieces_positions[0].1[curr_position].1;
        } else {
            // not first piece
            let prev_array = puzz_arrays[curr_piece_index - 1];
            let piece_array = pieces_positions[curr_piece_index].1[curr_position].1;
            let overlaps = (prev_array & piece_array).any();
            if overlaps {
                position_indices[curr_piece_index] += 1;
                continue;
            } else {
                puzz_arrays[curr_piece_index] = prev_array | piece_array;
            }
        }
        if curr_piece_index == num_pieces - 1 {
            // found a result! check whether all needed voxels are filled
            let not_filled = (puzz_arrays[curr_piece_index] & must_fill_matrix) ^ must_fill_matrix;
            if not_filled.not_any() {
                let assembly = position_indices
                    .iter()
                    .enumerate()
                    .map(|x| pieces_positions[x.0].1[*x.1].0.clone())
                    .collect::<Vec<_>>();
                if position_indices[3] < position_indices[4] {
                    // hack for duplicate
                    result.push(assembly);
                }
            }
            position_indices[curr_piece_index] += 1;
        } else {
            curr_piece_index += 1
        }
    }
    println!("{} iterations", num_iterations);

    return result;
}

pub fn get_matrices_for_piece(piece: &Piece, target: &PuzzArray) -> Vec<(Position, PuzzArray)> {
    let mut res: Vec<(Position, PuzzArray)> = Vec::new();
    let positions = positions_for_pieces(&piece);
    for p in positions {
        let arr = piece_to_matrix(&piece, &p);
        let non_overlap: PuzzArray = (*target & arr) ^ arr;
        if non_overlap.not_any() {
	    let mut already_added = false;
	    for prev_res in &res {
		if arr == prev_res.1 {
		    already_added = true;
		    break;
		}
	    }
	    if !already_added {
		res.push((p, arr));
	    }
        }
    }
    return res;
}

pub fn positions_for_pieces(piece: &Piece) -> Vec<Position> {
    let mut max_y = 0;
    for coord in piece {
        max_y = cmp::max(max_y, coord.y);
    }
    let mut res: Vec<Position> = Vec::new();
    for x in 0..6 {
        for y in 0..(3 - max_y) {
            res.push(Position {
                x,
                y,
                flipped: false,
            });
            res.push(Position {
                x,
                y,
                flipped: true,
            });
        }
    }
    return res;
}

pub fn piece_to_matrix(piece: &Piece, position: &Position) -> PuzzArray {
    let mut res: PuzzArray = bitarr!(0;36);
    let mut max_x: u16 = 0;
    let mut max_y: u16 = 0;
    for coord in piece {
        max_x = cmp::max(max_x, coord.x);
        max_y = cmp::max(max_y, coord.y);
    }

    if position.flipped {
        for coord in piece {
            res.set(
                ((max_x-coord.x+position.x) % 6  // wrap around cylinder
		    + (max_y-coord.y+position.y) * 6
		    + coord.z * 18) as usize,
                true,
            );
        }
    } else {
        for coord in piece {
            res.set(
                ((coord.x + position.x) % 6 // wrap around cylinder
		    + (coord.y + position.y) * 6
		    + coord.z * 18) as usize,
                true,
            );
        }
    }
    return res;
}
