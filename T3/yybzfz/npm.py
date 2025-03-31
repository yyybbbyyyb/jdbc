import subprocess
import re
import time
import argparse
import statistics

def run_game(game_number, total_games, snake_count):
    """运行一局游戏并返回结果"""
    print(f"\n--- Running Game {game_number}/{total_games} ---")
    
    # 运行npm命令
    process = subprocess.Popen(
        "npm run submit-test",
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )
    
    # 收集输出
    output = []
    seed = ""
    
    for line in process.stdout:
        print(line, end='')
        output.append(line)
        
        # 获取游戏种子
        if "Game seed:" in line:
            seed = line.strip().split("Game seed:")[1].strip()
    
    # 等待进程完成
    process.wait()
    
    # 解析结果
    snake_scores = [0] * snake_count
    snake_times = [0.0] * snake_count
    snake_status = ["unknown"] * snake_count
    
    output_text = ''.join(output)
    
    # 匹配所有蛇的分数和时间
    for i in range(1, snake_count + 1):
        # 两种可能的格式: survived 或 died in round X
        pattern = r"Snake {0}: (\d+) points \((survived|died in round \d+)\) spent ([\d.]+)ms".format(i)
        match = re.search(pattern, output_text)
        if match:
            snake_scores[i-1] = int(match.group(1))
            snake_status[i-1] = match.group(2)
            snake_times[i-1] = float(match.group(3))
    
    # 确定获胜者（得分最高的）
    max_score = max(snake_scores)
    winners = [i+1 for i, score in enumerate(snake_scores) if score == max_score]
    
    game_result = {
        'game_number': game_number,
        'seed': seed,
        'snake_scores': snake_scores,
        'snake_times': snake_times,
        'snake_status': snake_status,
        'winners': winners,
        'is_draw': len(winners) > 1
    }
    
    return game_result


def main():
    # 命令行参数
    parser = argparse.ArgumentParser(description='运行贪吃蛇比赛并统计结果')
    parser.add_argument('--games', type=int, default=10, help='要运行的游戏总数')
    parser.add_argument('--snakes', type=int, default=2, help='蛇的数量 (2 或 4)')
    args = parser.parse_args()
    
    # 验证蛇的数量参数
    if args.snakes not in [2, 4]:
        print("错误：蛇的数量必须是 2 或 4")
        return
    
    # 初始化统计数据
    stats = {
        'total_scores': [0] * args.snakes,
        'total_times': [0.0] * args.snakes,
        'wins': [0] * args.snakes,
        'survived': [0] * args.snakes,
        'died': [0] * args.snakes,
        'draws': 0,
        'all_scores': [[] for _ in range(args.snakes)],
        'all_times': [[] for _ in range(args.snakes)]
    }
    
    start_time = time.time()
    
    # 运行指定次数的游戏
    for i in range(1, args.games + 1):
        game_result = run_game(i, args.games, args.snakes)
        
        # 更新统计数据
        for j in range(args.snakes):
            stats['total_scores'][j] += game_result['snake_scores'][j]
            stats['total_times'][j] += game_result['snake_times'][j]
            stats['all_scores'][j].append(game_result['snake_scores'][j])
            stats['all_times'][j].append(game_result['snake_times'][j])
            
            if "survived" in game_result['snake_status'][j]:
                stats['survived'][j] += 1
            else:
                stats['died'][j] += 1
        
        # 记录胜利情况
        if game_result['is_draw']:
            stats['draws'] += 1
        else:
            for winner in game_result['winners']:
                stats['wins'][winner-1] += 1
    
    # 计算总运行时间
    total_time = time.time() - start_time
    
    # 输出统计结果
    print("\n" + "="*60)
    print("FINAL STATISTICS")
    print("="*60)
    print(f"Total Games: {args.games}")
    print(f"Total Time: {total_time:.2f} seconds")
    print("-"*60)
    
    for i in range(args.snakes):
        avg_score = stats['total_scores'][i] / args.games
        max_score = max(stats['all_scores'][i])
        min_score = min(stats['all_scores'][i])
        median_score = statistics.median(stats['all_scores'][i])
        avg_time = stats['total_times'][i] / args.games
        win_rate = (stats['wins'][i] / args.games) * 100
        survival_rate = (stats['survived'][i] / args.games) * 100
        
        print(f"Snake {i+1}:")
        print(f"  Total Score: {stats['total_scores'][i]}")
        print(f"  Average Score: {avg_score:.2f}")
        print(f"  Max/Min/Median Score: {max_score}/{min_score}/{median_score}")
        print(f"  Total Execution Time: {stats['total_times'][i]:.3f}ms")
        print(f"  Average Execution Time: {avg_time:.3f}ms")
        print(f"  Wins: {stats['wins'][i]} ({win_rate:.1f}%)")
        print(f"  Survival Rate: {survival_rate:.1f}%")
        print("-"*60)
    
    if args.games > 0:
        print(f"Draws: {stats['draws']} ({(stats['draws']/args.games)*100:.1f}%)")
    
    print("="*60)


if __name__ == "__main__":
    main()