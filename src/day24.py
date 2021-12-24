from typing import List
from z3 import *


def generate_code():
    x = open("24.txt", 'r').read().split("inp w")
    instructions = [[instruction for instruction in digit_verif.split("\n") if len(instruction) > 0] for digit_verif in x][1:]
    code = '''
x, y, z = 0, 0, 0
s = Solver()
ws = [Int('w_%d'%i) for i in range(14)]
'''
    for i, block in enumerate(instructions):
        code += "w = ws[%d]\n" % i
        for instruction in block:
            command, op1, op2 = tuple(instruction.split(" "))
            if command == "add":
                code += f"{op1} = {op1}+{op2}\n"
            elif command == "mul":
                code += f"{op1} = {op1}*{op2}\n"
            elif command == "div":
                code += f"{op1} = {op1}/{op2}\n"
            elif command == "mod":
                code += f"{op1} = {op1}%{op2}\n"
            elif command == "eql":
                code += f"{op1} = IntSort().cast(({op1}=={op2}))\n"
    return code


def find_best(s: Solver, ws: List[Int], z: Int, digit_order) -> str:
    best = {}
    for i in range(14):
        if i in best:
            continue
        for digit in digit_order:
            print("testing", i, digit)
            s.reset()
            for w in ws:
                s.add(w > 0)
            for w in ws:
                s.add(w < 10)
            s.add(z == 0)
            for index, stored_digit in best.items():
                s.add(ws[index] == stored_digit)
            s.add(ws[i] == digit)
            if s.check() == z3.sat:
                best[i] = digit
                print(s.check(), i, digit)
                break
    print(best)
    return ''.join([str(best[i]) for i in range(14)])


def part1(code):
    code += "print(find_best(s, ws, z, range(9,0,-1)))"
    exec(code)


def part2(code):
    code += "print(find_best(s, ws, z, range(1,10)))"
    exec(code)


def main():
    code = generate_code()
    part1(code)
    part2(code)


main()
