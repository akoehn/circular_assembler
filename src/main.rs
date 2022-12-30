mod find_assemblies;

use find_assemblies::Coord as C;
use find_assemblies::Piece as Piece;

fn build_target_shape() -> Piece {
    let mut target_shape = Vec::new();
    for x in 0..6 { 
        for y in 0..3 {
            if x == 0 && y < 2 {
                target_shape.push(C{x, y, z: 0});
            } else {
                target_shape.push(C{x, y, z: 0});
                target_shape.push(C{x, y, z: 1});
            }
        }
    }
    return target_shape;
}

fn must_fill_shape() -> Piece {
    let mut target_shape = Vec::new();
        target_shape.push(C{x: 1, y: 0, z: 0});
    for x in 0..4 {
        target_shape.push(C{x, y: 2, z: 1});
    }
    for y in 0..2 {
        target_shape.push(C{x: 3, y, z: 1});
    }
    return target_shape;
}


fn build_pieces() -> Vec<Piece> {
    return vec![
        // h1
        vec![C{x:0,y:0,z:0},C{x:0,y:0,z:1},
             C{x:1,y:0,z:0},
             C{x:2,y:0,z:0},C{x:2,y:0,z:1}],
        // h2
        vec![C{x:0,y:0,z:0},
             C{x:1,y:0,z:0},C{x:1,y:0,z:1},
             C{x:2,y:0,z:1}],
        // h3
        vec![C{x:0,y:0,z:1},
             C{x:1,y:0,z:1},
             C{x:2,y:0,z:0},C{x:2,y:0,z:1}],
        // v1
        vec![C{x:0,y:2,z:0},
             C{x:0,y:1,z:0},C{x:0,y:1,z:1},
             C{x:0,y:0,z:1}],
        // v1, again
        vec![C{x:0,y:2,z:0},
             C{x:0,y:1,z:0},C{x:0,y:1,z:1},
             C{x:0,y:0,z:1}],
        // v2
        vec![C{x:0,y:2,z:0},C{x:0,y:2,z:1},
             C{x:0,y:1,z:0},C{x:0,y:1,z:1},
             C{x:0,y:0,z:0}]
            ];
}


fn main() {
    let target = build_target_shape();
    let pieces = build_pieces();
    let must_fill = must_fill_shape();
    let results = find_assemblies::find_assemblies(&pieces, &target, &must_fill);
    for r in &results {
        println!("{:?}", r);
    }
    println!("num solutions: {}", results.len());
}
