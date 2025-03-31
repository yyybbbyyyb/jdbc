use wasm_bindgen::prelude::*;  

#[wasm_bindgen]
pub fn get_food_surround_count(food_x: i32, food_y: i32, foods: &[i32], n: i32) -> i32 {
    let threshold = if n == 8 {
        1
    } else {
        1
    };
    let mut count = 0;

    for i in (0..foods.len()).step_by(2) {
        let x = foods[i];
        let y = foods[i + 1];
        // 以food_x和food_y为中心，计算周围有多少个食物
        if (x >= food_x - threshold && x <= food_x + threshold) && 
           (y >= food_y - threshold && y <= food_y + threshold) {
            count += 1;
        }
    }
    return count;
}

#[wasm_bindgen]
pub fn get_another(self_x: i32, self_y: i32, n: i32) -> Box<[i32]> {
    let result = if self_x == 1 && self_y == 2 {
        [2, 1]
    } else if self_x == 2 && self_y == 1 {
        [1, 2]
    } else if self_x == 1 && self_y == n - 1 {
        [2, n]
    } else if self_x == 2 && self_y == n {
        [1, n - 1]
    } else if self_x == n - 1 && self_y == 1 {
        [n, 2]
    } else if self_x == n && self_y == 2 {
        [n - 1, 1]
    } else if self_x == n - 1 && self_y == n {
        [n, n - 1]
    } else if self_x == n && self_y == n - 1 {
        [n - 1, n]
    } else {
        [-1, -1]
    };
    
    result.into()
}


#[wasm_bindgen]
pub fn get_evaluated(x: i32, y: i32, food_num: i32, foods: &[i32], other: &[i32], n: i32, snake_num: i32, round: i32, self_x: i32, self_y: i32) -> f32 {
    let mut min_evaluated = f32::MAX;

    let max_round = if n == 8 {
        100
    } else {
        50
    };
    
    let threshold = (max_round as f32 * 0.4) as i32;

    // 避障1：边界位置处考虑其他蛇会不会卡死我们
    if x <= 1 || x >= n || y <= 1 || y >= n {
        if (other.len() > 0) && (other.len() % 2 == 0) {
            for i in (0..other.len()).step_by(2) {
                if n == 8 {
                    if i / 2 % 4 == 3 {
                        continue;
                    }
                } else if n == 5 {
                    if (i / 2 % 4 == 3) || (i / 2 % 4 == 2) {
                        continue;
                    }
                }
                let other_x = other[i];
                let other_y = other[i + 1];
                let distance = ((x - other_x).abs() + (y - other_y).abs()) as i32;
                if distance < 2 {
                    return f32::MAX;
                }
            }
        }
    }

    // 避障2: 四角格外慎重
    if (x == 1 && y == 1) || (x == 1 && y == n) || (x == n && y == 1) || (x == n && y == n) {
        // 计算对脚
        let another = get_another(self_x, self_y, n);
        let another_x = another[0];
        let another_y = another[1];

        for i in (0..other.len()).step_by(9) {
            let other_x = other[i];
            let other_y = other[i + 1];
            let distance = ((another_x - other_x).abs() + (another_y - other_y).abs()) as i32;
            if distance < 2 {
                return f32::MAX;
            }
        }
    }

    // 计算一下离对方蛇最近的果子
    let mut other_min_distance = i32::MAX;
    let mut other_min_distance_idx = i32::MAX;
    if n == 5 {
        if snake_num == 1 {
            let other_x = other[0];
            let other_y = other[1];
            for i in (0..foods.len()).step_by(2) {
                let food_x = foods[i];
                let food_y = foods[i + 1];
                let distance = ((other_x - food_x).abs() + (other_y - food_y).abs()) as i32;
                if distance < other_min_distance {
                    other_min_distance = distance;
                    other_min_distance_idx = i as i32;
                }
            }    
        }
    }


    for i in (0..foods.len()).step_by(2) {
        let food_x = foods[i];
        let food_y = foods[i + 1];
        let count = get_food_surround_count(food_x, food_y, foods, n);
        let distance = (((x - food_x).abs() + (y - food_y).abs()) + 1) as i32;
        
        let mut evaluate = (distance) as f32 / (count as f32 / food_num as f32) as f32;


        // 避障3: 争抢（分1v1和4snakes）
        if n == 5 {
            // println!("food_x: {}, food_y: {}, x: {}, y: {}, other_min_distance: {}, distance: {}", food_x, food_y, x, y, other_min_distance, distance);
            
            if (i == other_min_distance_idx as usize) && (other_min_distance <= distance) {

                // TODO:考虑？
                if other_min_distance == 1 && distance == 1 && round > threshold {
                    return f32::MAX;
                }
                if other_min_distance < distance {
                    evaluate = f32::MAX;
                }
            }
        } else {
            for i in (0..other.len()).step_by(8) {
                let other_x = other[i];
                let other_y = other[i + 1];
                let other_distance = ((other_x - food_x).abs() + (other_y - food_y).abs()) as i32;
                if other_distance == 1 && distance == 1 {
                    return f32::MAX;
                }
            }
        }
        
        if evaluate < min_evaluated {
            min_evaluated = evaluate;
        }
    }

    return min_evaluated;
}

#[wasm_bindgen]
pub fn solution(n: i32, me: &[i32], other: &[i32], food_num: i32, foods: &[i32], snake_num: i32, round: i32) -> i32 {
    let head_x = me[0];
    let head_y = me[1];

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

        // 场地碰撞
        if new_x < 1 || new_x > n || new_y < 1 || new_y > n {
            continue;
        }
        
        let mut is_safe = true;
        // 自我碰撞
        for i in (2..me.len() - 2).step_by(2) {
            if new_x == me[i] && new_y == me[i + 1] {
                is_safe = false;
                break;
            }
        }

        // 和其他蛇的身子碰撞
        if (other.len() > 0) && (other.len() % 2 == 0) {
            for i in (0..other.len()).step_by(2) {
                if i / 2 % 4 == 3 {
                    continue;
                }
                if new_x == other[i] && new_y == other[i + 1] {
                    is_safe = false;
                    break;
                }
            }
        }

        if !is_safe {
            continue;
        }

        let eval = get_evaluated(new_x, new_y, food_num, foods, other, n, snake_num, round, head_x, head_y);
        safe_moves.push((direction, eval));
    }

    if !safe_moves.is_empty() {
        // 输出
        for i in safe_moves.iter() {
            let direction = i.0;
            let evaluate = i.1;
            println!("Direction: {}, Evaluate: {}", direction, evaluate);
        }
        safe_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        return *safe_moves[0].0;
    }

    return 0; // No safe moves available
}

#[wasm_bindgen]
pub fn greedy_snake_step(n: i32, me: &[i32], snake_num: i32, other: &[i32], 
                        food_num: i32, foods: &[i32], round: i32) -> i32 {

    return solution(n, me, other, food_num, foods, snake_num, round);
}



#[cfg(test)]

mod tests {
    use super::*;

    // #[test]
    // fn test_greedy_snake_move_1() {
    //     let n = 5;
    //     let me = [1, 1, 1, 2, 1, 3, 1, 4];
    //     let snake_num = 2;
    //     let other = [3, 3, 3, 4, 3, 5, 3, 6];
    //     let food_num = 2;
    //     let foods = [2, 2, 4, 4];
    //     let round = 1;

    //     let result = greedy_snake_step(n, &me, snake_num, &other, food_num, &foods, round);
    //     assert_eq!(result, 3);
    // }

    // #[test]
    // fn test_greedy_snake_move_2() {
    //     let n = 5;
    //     let me = [2, 5, 2, 4, 2, 3, 2, 2];
    //     let snake_num = 1;
    //     let other = [];
    //     let food_num = 5;
    //     let foods = [2, 1, 4, 5, 4, 4, 5, 5, 5, 4];
    //     let round = 1;

    //     let result = greedy_snake_step(n, &me, snake_num, &other, food_num, &foods, round);
    //     assert_eq!(result, 3);
    // }

    // #[test]
    // fn test_greedy_snake_move_3() {
    //     let n = 8;
    //     let me = [1,5,1,6,1,7,1,8];
    //     let snake_num = 4;
    //     let other = [7,6,7,5,8,5,8,6, 4,2,4,1,3,1,2,1, 8,1,7,1,7,2,7,3];
    //     let food_num = 10;
    //     let foods = [2,7, 3, 6, 4,5, 5,6, 5,7, 5,8, 6,2, 6,4, 8, 2, 8, 4];
    //     let round = 49;

    //     let result = greedy_snake_step(n, &me, snake_num, &other, food_num, &foods, round);
    //     assert_eq!(result, 3);
    // }

    // #[test]
    // fn test_greedy_snake_move_4() {
    //     let n = 8;
    //     let me = [2, 4, 2, 3, 2, 2, 2, 1];
    //     let snake_num = 4;
    //     let other = [7, 8, 8, 8, 8, 7, 8, 6, 3, 5, 4, 5, 4, 4, 4, 3, 3, 6, 4, 6, 5, 6, 5, 5];
    //     let food_num = 10;
    //     let foods = [2, 5, 2, 7, 3, 4, 3, 7, 4, 1, 5, 1, 6, 3, 6, 5, 6, 8, 8, 2];
    //     let round = 55;

    //     let result = greedy_snake_step(n, &me, snake_num, &other, food_num, &foods, round);
    //     assert_eq!(result, 1);
    // }

    #[test]
    fn test_greedy_snake_move_4() {
        let n = 5;
        let me = [3, 2, 3, 1, 4, 1, 5, 1];
        let snake_num = 1;
        let other = [2, 3, 1, 3, 1, 4, 1, 5];
        let food_num = 5;
        let foods = [1, 1, 2, 2, 4, 3, 4, 4, 5, 5];
        let round = 38;

        let result = greedy_snake_step(n, &me, snake_num, &other, food_num, &foods, round);
        assert_eq!(result, 1);
    }
}


