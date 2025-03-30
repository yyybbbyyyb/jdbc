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
    fn test_basic_movement() {
        // 直线朝向食物
        assert_eq!(greedy_snake_move(&[4, 4, 4, 5, 4, 6, 4, 7], &[5, 4]), 3); // 右
        assert_eq!(greedy_snake_move(&[4, 4, 4, 3, 4, 2, 4, 1], &[3, 4]), 1); // 左
        assert_eq!(greedy_snake_move(&[4, 4, 3, 4, 2, 4, 1, 4], &[4, 5]), 0); // 上
        assert_eq!(greedy_snake_move(&[4, 4, 5, 4, 6, 4, 7, 4], &[4, 3]), 2); // 下
    }

    #[test]
    fn test_avoid_walls() {
        // 靠近边界，避免超出范围
        assert_ne!(greedy_snake_move(&[1, 1, 1, 2, 1, 3, 1, 4], &[0, 1]), 1); // 不应向左 (越界)
        assert_ne!(greedy_snake_move(&[8, 8, 8, 7, 8, 6, 8, 5], &[9, 8]), 3); // 不应向右 (越界)
        assert_ne!(greedy_snake_move(&[1, 8, 2, 8, 3, 8, 4, 8], &[1, 9]), 0); // 不应向上 (越界)
        assert_ne!(greedy_snake_move(&[8, 1, 7, 1, 6, 1, 5, 1], &[8, 0]), 2); // 不应向下 (越界)
    }

    #[test]
    fn test_avoid_self() {
        // 蛇身形成障碍，必须绕行
        let body = [3, 3, 3, 4, 4, 4, 4, 3];
        let food = [2, 3];
        assert_eq!(greedy_snake_move(&body, &food), 1); // 只能向左

        let body = [4, 4, 4, 5, 5, 5, 4, 5];
        let food = [4, 6];
        assert_ne!(greedy_snake_move(&body, &food), 0); // 不能往上 (有身体)
    }

    #[test]
    fn test_complex_path() {
        // 必须绕路才能到达食物
        let body = [4, 4, 4, 5, 3, 5, 3, 4];
        let food = [2, 4];
        assert_eq!(greedy_snake_move(&body, &food), 1); // 选择向左绕行
    }
}