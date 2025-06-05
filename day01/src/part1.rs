use thiserror::Error;



pub fn part1(input: &str) -> anyhow::Result<()> {
    // println!("{input}");
    let parts = input
        .split(", ")
        .map(try_parse_path_fragment)
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();
    // println!("{:?}", parts);
    let mut position = (0,0);
    let mut direction = (0,1);
    for path_fragmen in parts {
        append_next_path_fragment(&mut position, &mut direction, &path_fragmen);
    }
    let distance = position.0.abs() + position.1.abs();
    println!("final position: ({}, {}, {}): ", position.0, position.1, distance);
    Ok(())
}

#[derive(Debug, PartialEq, Default)]
struct PathFragment {
    x: i16,
    y: i16,
}

#[derive(Error, Debug)]
enum Day1ApplicationError {
    #[error("The direction '{unknown_direction}' is not implemented")]
    UnknownDirectionError{unknown_direction: String},
    #[error("The moving direct ({x}, {y}) is not implemented")]
    UnknownMovingDirection{x: i16, y:i16},
}

fn try_parse_path_fragment(input: &str) -> anyhow::Result<PathFragment> {
    let direction = &input[0..1];
    let value = &input[1..];
    let value = value.parse::<i16>()?;
    match direction {
        "R" => Ok(PathFragment{x:value, ..Default::default()}),
        "L" => Ok(PathFragment{x:-value, ..Default::default()}),
        unknown_direction => Err(Day1ApplicationError::UnknownDirectionError { unknown_direction: unknown_direction.to_string() }.into())
    }
}

fn append_next_path_fragment(global_position: &mut (i16, i16), current_direction: &mut (i16, i16), fragment_to_append: &PathFragment){
    //let mut nextdirection = (0i8, 0i8);

    // println!("({global_position:?}, {current_direction:?}, {fragment_to_append:?})");
    match current_direction {
        // (x, y)
        // up
        (0, 1) => {
            global_position.0 += fragment_to_append.x;
            global_position.1 += fragment_to_append.y;
            current_direction.0 = fragment_to_append.x.signum();
            current_direction.1 = fragment_to_append.y.signum();
        }
        // right
        (1, 0) => {
            global_position.0 += fragment_to_append.y;
            global_position.1 -= fragment_to_append.x;
            current_direction.0 = fragment_to_append.y.signum();
            current_direction.1 = -fragment_to_append.x.signum();
        }
        // down
        (0, -1) => {
            global_position.0 -= fragment_to_append.x;
            global_position.1 -= fragment_to_append.y;
            current_direction.0 = -fragment_to_append.x.signum();
            current_direction.1 = -fragment_to_append.y.signum();
        }
        // left
        (-1, 0) => {
            global_position.0 -= fragment_to_append.y;
            global_position.1 += fragment_to_append.x;
            current_direction.0 = -fragment_to_append.y.signum();
            current_direction.1 = fragment_to_append.x.signum();
        }
        (x, y) => {
            let err = Day1ApplicationError::UnknownMovingDirection{x:x.clone(), y:y.clone()};
            println!("{err}");
        }



    }


}
