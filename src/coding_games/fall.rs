


//height  :
///width : 


#[derive(Clone, PartialEq)]
enum Direction {
    UP, 
    DOWN, 
    RIGHT, 
    LEFT
} 

struct Maze {
    boxes : Vec<Vec<Block>>,
}

#[derive(Clone)]
struct Passage {
    entries : Vec<Direction>,
    exits : Vec<Direction>
}

#[derive(Clone)]
struct Block {
    type_ : usize,
    y : usize,
    x : usize,
    pos : Cord,
    passages : Vec<Passage>, 
    present : bool, 
} 

struct Cord {
    y : usize,
    x : usize,
}

struct Cord_Offset {
    plus_y : usize,
    plus_x : usize,
}



fn add_offset(c : Cord, d : Direction) -> Cord {
    match d {
        Direction::DOWN => Cord { y : c.y + 1, x : c.x},
        Direction::LEFT => Cord { y : c.y, x : c.x - 1},
        Direction::RIGHT => Cord { y : c.y , x : c.x + 1},
        _ => c,
    }
}


fn connects(cur_block : Block, other_block : Block) -> bool {
    /* you always go down cause you can't otherwise */
    /* as long other entry cur exit connect  the other can be ignored */
    for passage in cur_block.passages.iter()  { 
        for cur_exit in passage.exits.iter() {
            for other_entry in passage.entries.iter() {
                if (*cur_exit == Direction::DOWN && *other_entry == Direction::UP) ||
                    (*cur_exit == Direction::LEFT && *other_entry == Direction::RIGHT) || 
                    (*cur_exit == Direction::RIGHT && *other_entry == Direction::LEFT) {
                        return true;
                }

                //entry == LEFT && exits.containts(DOWN) &&  wanted_exit == DOWN,
                //entry == RIGHT && (exit == DOWN)
                //return True;
            }
        }
    }
    false



    
    
}




fn get_maze_block(the_maze : &Maze, idx : Cord) -> Option<Block> {
    let col_res : Option<&Vec<Block>> = the_maze.boxes.get(idx.y);
    if let Some(col) = col_res {
        let row_res = col.get(idx.x);
        if let Some(row) = row_res {
            return Some(row.clone());
        }
    }
    None
}   


/*fn check_around(the_maze : &Maze, cur_block : &Block) -> Vec<Block> {
    check_around();
    check_around();
    check_around();

    
}*/


fn check_around(the_maze : &Maze, cur_block : &Block) -> Block {

    let down_cord = add_offset(cur_block.pos, Direction::DOWN);

    /* check down (priority) right left */
    let maze_block_down_opt = get_maze_block(the_maze, down_cord);
    
    /* check if connects  */
    let down_connects = if maze_block_down_opt.is_some() 
        {connects(*cur_block, maze_block_down_opt.unwrap())} else {false};
    

    let mut opened_passages : Vec<Direction> = vec![];
    if down_connects {
        opened_passages.push(maze_block_down_opt.unwrap().);
    }

    /*let opened_passages : Vec<Direction> = (down_connects, right_connects, left_connects)
                            .iter()
                            .zip()
                            .filter(|connect| connect)
                            .collect();*/
    opened_passages
}


/*fn game() {

    loop {

    }

}*/











#[cfg(test)]
mod test_X {

    use super::*;

    #[test]
    fn test_get_maze_block() {
        
        

        let matrix = vec![vec![1,2], vec![0]];

        let a = get_maze_block(&matrix, Cord { y : 0, x : 0});
        assert!(a == Some(1));
        //let b = get_maze_block();
        //let c = get_maze_block();
    }
}

