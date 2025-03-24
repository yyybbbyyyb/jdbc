use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn func(flag: i32, seq: &[i32], size: i32) -> i32 {
    10
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_snake_move() {
        // 原有测试不变
        // Test case 1: Move right to reach the food
        let body = [4, 4, 4, 5, 4, 6, 4, 7];
        let food = [5, 4];
        assert_eq!(greedy_snake_move(&body, &food), 3);

        // Test case 2: Move down to reach the food
        let body = [1, 1, 1, 2, 1, 3, 1, 4];
        let food = [1, 5];
        assert_eq!(greedy_snake_move(&body, &food), 3);

        // Test case 3: Move left to reach the food
        let body = [5, 5, 5, 6, 5, 7, 5, 8];
        let food = [4, 5];
        assert_eq!(greedy_snake_move(&body, &food), 1);

        // Test case 4: Move up to reach the food
        let body = [5, 5, 5, 4, 5, 3, 5, 2];
        let food = [5, 1];
        assert_eq!(greedy_snake_move(&body, &food), 0);
        
        let body = [2, 2, 2, 3, 3, 3, 4, 3];
        let food = [2, 1];
        assert_eq!(greedy_snake_move(&body, &food), 2);
        
        // 蛇头在(2,2)，蛇身形成U形，食物在(2,1)
        // 向上会撞到蛇身，应该选择其他方向
        let body = [2, 2, 2, 3, 3, 3, 3, 2];
        let food = [2, 1];
        assert_eq!(greedy_snake_move(&body, &food), 2);
    }
}