use wasm_bindgen::prelude::*;

use std::collections::VecDeque;


// 全局静态可变变量
static mut ANS: Vec<i32> = Vec::new();

#[wasm_bindgen]
pub fn greedy_snake_move(body: &[i32], food: &[i32]) -> i32 {
    let head_x = body[0];
    let head_y = body[1];
    let food_x = food[0];
    let food_y = food[1];

    let directions = [
        (0, 1, 0),  // Up
        (0, -1, 2), // Down
        (-1, 0, 1), // Left
        (1, 0, 3),  // Right
    ];

    let mut safe_moves = Vec::new();

    for (dx, dy, direction) in directions.iter() {
        let new_x = head_x + dx;
        let new_y = head_y + dy;

        let mut collision = false;
        for i in (2..body.len() - 2).step_by(2) {
            if new_x == body[i] && new_y == body[i + 1] {
                collision = true;
                break;
            }
            if new_x < 1 || new_x > 8 || new_y < 1 || new_y > 8 {
                collision = true;
                break;
            }
        }

        if !collision {
            let dist = (new_x - food_x).abs() + (new_y - food_y).abs();
            safe_moves.push((direction, dist));
        }
    }

    if !safe_moves.is_empty() {
        safe_moves.sort_by_key(|&(_, dist)| dist);
        return *safe_moves[0].0;
    }

    let dx = food_x - head_x;
    let dy = food_y - head_y;

    if dx.abs() > dy.abs() {
        if dx > 0 {
            return 3;
        } else {
            return 1;
        }
    } else {
        if dy > 0 {
            return 0;
        } else {
            return 2;
        }
    }
}

#[wasm_bindgen]
pub fn greedy_BFS(body: &[i32], food: &[i32], barriers: &[i32]) -> bool {
    let directions = [
        (0, 1, 0),  // Up
        (0, -1, 2), // Down
        (-1, 0, 1), // Left
        (1, 0, 3),  // Right
    ];
    let mut movement = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = [[false; 9]; 9];
    let mut vis_body = [[[[false; 9]; 9]; 9]; 9];

    for i in (0..barriers.len()).step_by(2) {
        visited[barriers[i] as usize][barriers[i+1] as usize] = true;
    }

    queue.push_back((body[0], body[1], body[2], body[3], 0));
    movement.push((0,0));
    vis_body[body[0] as usize][body[1] as usize][body[2] as usize][body[3] as usize] = true;


    while !queue.is_empty(){
        let (head_x, head_y, body_x, body_y, move_step) = queue.pop_front().unwrap();
        if head_x == food[0] && head_y == food[1] {
            let mut mutStep = move_step;
            while mutStep != 0 {
                let (dirc, last_step) = movement[mutStep];
                mutStep = last_step;
                unsafe {
                    ANS.push(dirc);
                }
            }
            return true;
        }
            
        for (dx, dy, direction) in directions.iter() {
            let new_x = head_x + dx;
            let new_y = head_y + dy;
            if new_x == body_x && new_y == body_y{
                continue;
            }
            if new_x < 1 || new_x > 8 || new_y < 1 || new_y > 8 {
                continue;
            }
            if visited[new_x as usize][new_y as usize]{
                continue;
            }
            if vis_body[new_x as usize][new_y as usize][head_x as usize][head_y as usize]{
                continue;
            }
            vis_body[new_x as usize][new_y as usize][head_x as usize][head_y as usize] = true;
            movement.push((*direction, move_step));
            queue.push_back((new_x, new_y, head_x, head_y, movement.len()-1));
        }
    }

    return false;
}

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(body: &[i32], food: &[i32], barriers: &[i32]) -> i32 {
    if unsafe { ANS.len() } == 0 {
        if !greedy_BFS(body, food, barriers){
            return -1;
        }
    }

    return unsafe { ANS.pop().unwrap() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_snake_move() {
        
        let body = [3, 4, 3, 3, 3, 2, 3, 1];
        let food = [1, 2];
        let barriers = [1, 1, 2, 1, 1, 3, 2, 3];
        assert_eq!(greedy_snake_move_barriers(&body, &food, &barriers), 3);


        unsafe {
            ANS.clear();
        }

        let body = [5, 4, 5, 5, 5, 6, 5, 7];
        let food = [5, 8];
        let barriers = [6, 8, 6, 7, 6, 6, 6, 5, 6, 4, 6, 3, 6, 2, 5, 2, 4, 2, 4, 3, 4, 4, 4, 5, 4, 6, 4, 7, 4, 8];
        assert_eq!(greedy_snake_move_barriers(&body, &food, &barriers), -1);

        unsafe {
            ANS.clear();
        }

        let body = [5, 4, 5, 5, 5, 6, 5, 7];
        let food = [5, 8];
        let barriers = [6, 8, 6, 7, 6, 6, 6, 5, 6, 4, 6, 3, 6, 2, 4, 2, 4, 3, 4, 4, 4, 5, 4, 6, 4, 7, 4, 8];
        assert_eq!(greedy_snake_move_barriers(&body, &food, &barriers), 2);
        
    }
}