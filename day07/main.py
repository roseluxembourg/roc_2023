import re

state = {}
lines = list(open("input.txt"))
print(len(lines))
count = 0


def get_value(state, wire):
    try:
        return int(wire)
    except ValueError:
        return state[wire]


state["b"] = 16076

for line in lines:
    if line.split()[-1] == "b":
        continue
    try:
        match line.split():
            case [wire1, "->", wire]:
                state[wire] = get_value(state, wire1)
            case [wire1, operation, wire2, "->", wire]:
                match operation:
                    case "AND":
                        state[wire] = get_value(state, wire1) & get_value(state, wire2)
                    case "OR":
                        state[wire] = get_value(state, wire1) | get_value(state, wire2)
                    case "LSHIFT":
                        state[wire] = get_value(state, wire1) << int(wire2)
                    case "RSHIFT":
                        state[wire] = get_value(state, wire1) >> int(wire2)
            case ["NOT", wire1, "->", wire]:
                state[wire] = ~get_value(state, wire1)
        count += 1
    except KeyError:
        lines.append(line)


print(state)
