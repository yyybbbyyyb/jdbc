import subprocess
import re
import time
import argparse
import numpy as np
from collections import defaultdict
from colorama import init, Fore, Back, Style

# 初始化colorama
init(autoreset=True)

def run_game(game_number, total_games, snake_count):
    """运行一局游戏并返回结果"""
    print(f"\n{Back.BLUE}{Fore.WHITE} Running Game {game_number}/{total_games} {Style.RESET_ALL}")
    
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
        # 对输出添加颜色
        if "Game seed:" in line:
            print(f"{Fore.CYAN}{line}", end='')
        elif "FINAL RESULTS" in line:
            print(f"{Fore.YELLOW}{Style.BRIGHT}{line}", end='')
        elif "Snake" in line and "points" in line:
            if "survived" in line:
                print(f"{Fore.GREEN}{line}", end='')
            elif "died" in line:
                print(f"{Fore.RED}{line}", end='')
            else:
                print(line, end='')
        else:
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
    
    # 计算排名和对战积分
    battle_points = calculate_battle_points(snake_scores, snake_times, snake_count)
    
    # 检查是否有平局(共享排名)
    is_draw = False
    # 检查是否有蛇的得分相同并且时间差在3%以内
    snakes = [(i, snake_scores[i], snake_times[i]) for i in range(snake_count)]
    snakes.sort(key=lambda x: (-x[1], x[2]))  # 按分数降序，时间升序
    
    i = 0
    while i < snake_count - 1:
        # 检查相邻蛇的分数是否相同
        if snakes[i][1] == snakes[i+1][1]:
            # 检查时间差异是否小于3%
            time1 = snakes[i][2]
            time2 = snakes[i+1][2]
            if time1 > 0 and (time2 - time1) / time1 * 100 < 3:
                is_draw = True
                break
        i += 1
    
    game_result = {
        'game_number': game_number,
        'seed': seed,
        'snake_scores': snake_scores,
        'snake_times': snake_times,
        'snake_status': snake_status,
        'winners': winners,
        'is_draw': is_draw,  # 使用新的平局定义
        'battle_points': battle_points
    }
    
    return game_result


def calculate_battle_points(scores, times, snake_count):
    """
    计算每局对战的积分
    第1/2/3/4名分别获得3/2/1/0对战积分
    局内得分相同，按时间排序
    时间差距小于3%，平分两个名次的分数
    """
    # 创建蛇的索引、分数和时间的列表
    snakes = [(i, scores[i], times[i]) for i in range(snake_count)]
    
    # 首先按分数降序排序，然后按时间升序排序
    snakes.sort(key=lambda x: (-x[1], x[2]))
    
    battle_points = [0] * snake_count
    points_per_rank = [3, 2, 1, 0]  # 各排名对应的分数
    
    i = 0
    while i < snake_count:
        # 找出分数相同的蛇
        same_score_group = [snakes[i]]
        j = i + 1
        while j < snake_count and snakes[j][1] == snakes[i][1]:
            same_score_group.append(snakes[j])
            j += 1
        
        if len(same_score_group) == 1:
            # 如果只有一条蛇有这个分数，直接分配对应排名的积分
            battle_points[same_score_group[0][0]] = points_per_rank[i] if i < len(points_per_rank) else 0
        else:
            # 处理分数相同的情况
            # 按时间排序
            same_score_group.sort(key=lambda x: x[2])
            
            # 检查时间差异并分配积分
            k = 0
            while k < len(same_score_group):
                current_snake = same_score_group[k]
                current_rank = i + k
                
                # 查找时间差距小于3%的蛇
                similar_time_group = [current_snake]
                l = k + 1
                while l < len(same_score_group):
                    next_snake = same_score_group[l]
                    # 计算时间差异百分比
                    time_diff_percent = 0
                    if current_snake[2] > 0:  # 避免除以零
                        time_diff_percent = (next_snake[2] - current_snake[2]) / current_snake[2] * 100
                    
                    if time_diff_percent < 3:
                        similar_time_group.append(next_snake)
                        l += 1
                    else:
                        break
                
                # 计算这组蛇应该平分的积分
                total_points = 0
                for rank in range(current_rank, current_rank + len(similar_time_group)):
                    if rank < len(points_per_rank):
                        total_points += points_per_rank[rank]
                
                # 平分积分
                points_each = total_points / len(similar_time_group)
                for snake in similar_time_group:
                    battle_points[snake[0]] = points_each
                
                k += len(similar_time_group)
        
        i += len(same_score_group)
    
    return battle_points


def main():
    # 命令行参数
    parser = argparse.ArgumentParser(description='运行贪吃蛇比赛并统计结果')
    parser.add_argument('--games', type=int, default=10, help='要运行的游戏总数')
    parser.add_argument('--snakes', type=int, default=2, help='蛇的数量 (2 或 4)')
    args = parser.parse_args()
    
    # 验证蛇的数量参数
    if args.snakes not in [2, 4]:
        print(f"{Fore.RED}错误：蛇的数量必须是 2 或 4")
        return
    
    # 初始化统计数据
    stats = {
        'total_scores': [0] * args.snakes,
        'total_times': [0.0] * args.snakes,
        'battle_points': [0.0] * args.snakes,  # 新增：对战积分
        'wins': [0] * args.snakes,
        'survived': [0] * args.snakes,
        'died': [0] * args.snakes,
        'draws': 0,
        'all_scores': [[] for _ in range(args.snakes)],
        'all_times': [[] for _ in range(args.snakes)],
        'rank_counts': [defaultdict(int) for _ in range(args.snakes)]  # 新增：各排名次数统计
    }
    
    start_time = time.time()
    
    # 运行指定次数的游戏
    for i in range(1, args.games + 1):
        game_result = run_game(i, args.games, args.snakes)
        
        # 更新统计数据
        for j in range(args.snakes):
            stats['total_scores'][j] += game_result['snake_scores'][j]
            stats['total_times'][j] += game_result['snake_times'][j]
            stats['battle_points'][j] += game_result['battle_points'][j]  # 累计对战积分
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
                
        # 记录排名情况
        # 根据对战积分反向计算排名
        battle_points = game_result['battle_points']
        points_to_rank = {3.0: 1, 2.0: 2, 1.0: 3, 0.0: 4, 2.5: 1.5, 1.5: 2.5, 0.5: 3.5}
        for j in range(args.snakes):
            if battle_points[j] in points_to_rank:
                stats['rank_counts'][j][points_to_rank[battle_points[j]]] += 1
            else:
                # 处理其他可能的平分情况
                estimated_rank = 4 - battle_points[j]  # 粗略估计
                stats['rank_counts'][j][estimated_rank] += 1
    
    # 计算总运行时间
    total_time = time.time() - start_time
    
    # 输出统计结果
    print(f"\n{Back.WHITE}{Fore.BLACK}{'='*60}{Style.RESET_ALL}")
    print(f"{Back.YELLOW}{Fore.BLACK}{Style.BRIGHT}               FINAL STATISTICS               {Style.RESET_ALL}")
    print(f"{Back.WHITE}{Fore.BLACK}{'='*60}{Style.RESET_ALL}")
    print(f"{Fore.WHITE}Total Games: {Fore.YELLOW}{args.games}")
    print(f"{Fore.WHITE}Total Time: {Fore.YELLOW}{total_time:.2f} seconds")
    print(f"{Fore.CYAN}{'-'*60}")
    
    # 按对战积分排序蛇的索引
    snake_indices_by_points = sorted(range(args.snakes), key=lambda i: -stats['battle_points'][i])
    
    # 蛇的颜色
    snake_colors = [Fore.RED, Fore.GREEN, Fore.BLUE, Fore.MAGENTA]
    
    for i_idx, i in enumerate(snake_indices_by_points):
        snake_color = snake_colors[i % len(snake_colors)]
        win_rate = (stats['wins'][i] / args.games) * 100
        survival_rate = (stats['survived'][i] / args.games) * 100
        
        print(f"{snake_color}{Style.BRIGHT}Snake {i+1} {Fore.WHITE}(Rank {i_idx+1}):")
        print(f"{snake_color}  Total Score: {Fore.WHITE}{stats['total_scores'][i]}")
        print(f"{snake_color}  Total Battle Points: {Fore.WHITE}{stats['battle_points'][i]:.1f}")
        print(f"{snake_color}  Total Execution Time: {Fore.WHITE}{stats['total_times'][i]:.3f}ms")
        print(f"{snake_color}  Wins: {Fore.WHITE}{stats['wins'][i]} ({win_rate:.1f}%)")
        print(f"{snake_color}  Survival Rate: {Fore.WHITE}{survival_rate:.1f}%")
        
        # 输出排名分布
        print(f"{snake_color}  Rank Distribution: {Fore.WHITE}", end="")
        
        # 创建排名分布的字符串
        rank_strings = []
        ranks = stats['rank_counts'][i]
        for rank in sorted(ranks.keys()):
            rank_str = str(rank) if rank.is_integer() else f"{rank:.1f}"
            count = ranks[rank]
            percentage = (count / args.games) * 100
            rank_strings.append(f"Rank {rank_str}: {count} ({percentage:.1f}%)")
            
        # 用逗号加空格连接所有排名字符串
        print(", ".join(rank_strings))
        
        print(f"{Fore.CYAN}{'-'*60}")
    
    if args.games > 0:
        draw_rate = (stats['draws'] / args.games) * 100
        if draw_rate > 30:
            color = Fore.RED  # 高平局率用红色
        elif draw_rate > 10:
            color = Fore.YELLOW  # 中等平局率用黄色
        else:
            color = Fore.GREEN  # 低平局率用绿色
        print(f"{Fore.WHITE}Draws: {color}{stats['draws']} ({draw_rate:.1f}%)")
    
    print(f"{Back.WHITE}{Fore.BLACK}{'='*60}{Style.RESET_ALL}")


if __name__ == "__main__":
    main()