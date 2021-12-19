import itertools

import numpy as np
from typing import List, Tuple


def distances(coords):
    points = len(coords)
    pdist = np.zeros((points, points))
    for i in range(points):
        for j in range(i, points):
            pdist[i, j] = np.linalg.norm(coords[i, 0:3] - coords[j, 0:3])
    return pdist


class Scanner:
    def __init__(self, data):
        self.own_pos = (0, 0, 0)
        lines = data.split("\n")
        self.label = lines[0]
        self.coords = []
        for line in lines[1:]:
            x, y, z = tuple([int(a) for a in line.split(",")])
            self.coords.append((x, y, z))
        dist_array = distances(np.array(self.coords))
        self.distances = []
        for i in range(len(self.coords)):
            for j in range(i, len(self.coords)):
                if i != j:
                    self.distances.append((i, j, dist_array[i, j]))

    def shuffle(self, signs: List[int], shuffling: List[int], t: Tuple[int, int, int]) -> Tuple[int, int, int]:
        return signs[0] * t[shuffling[0]], signs[1] * t[shuffling[1]], signs[2] * t[shuffling[2]]

    def inv_shuffle(self, signs: List[int], shuffling: List[int], t: Tuple[int, int, int]) -> Tuple[int, int, int]:
        x = [signs[0] * t[0], signs[1] * t[1], signs[2] * t[2]]
        return x[shuffling.index(0)], x[shuffling.index(1)], x[shuffling.index(2)]

    def rotate(self, shuffling, signs):
        self.own_pos = self.shuffle(signs, shuffling, self.own_pos)
        self.coords = [self.shuffle(signs, shuffling, (x, y, z)) for x, y, z in self.coords]

    def inv_rotate(self, shuffling, signs):
        self.own_pos = self.inv_shuffle(signs, shuffling, self.own_pos)
        self.coords = [self.inv_shuffle(signs, shuffling, (x, y, z)) for x, y, z in self.coords]

    def shift(self, a, b, c):
        self.own_pos = (self.own_pos[0] + a, self.own_pos[1] + b, self.own_pos[2] + c)
        self.coords = [(x + a, y + b, z + c) for x, y, z in self.coords]

    def inv_shift(self, a, b, c):
        self.own_pos = (self.own_pos[0] - a, self.own_pos[1] - b, self.own_pos[2] - c)
        self.coords = [(x - a, y - b, z - c) for x, y, z in self.coords]

    def __repr__(self):
        return f"{self.label}:{self.coords}"


def verify_points_in_place(s1, s2, matched):
    res = []
    for p1, p2 in matched:
        a = (s1.coords[p1[0]] == s2.coords[p2[0]]) and (s1.coords[p1[1]] == s2.coords[p2[1]])
        b = (s1.coords[p1[0]] == s2.coords[p2[1]]) and (s1.coords[p1[1]] == s2.coords[p2[0]])
        res.append(a or b)
    return all(res)


def put_s2_in_place(s1, s2, first_src, first_target, matched):
    src_coords_1 = s1.coords[first_src[0]]
    for shuffling in itertools.permutations([0, 1, 2]):
        for signs in itertools.product([1, -1], repeat=3):
            for target in [first_target[0], first_target[1]]:
                x, y, z = src_coords_1
                s2.rotate(list(shuffling), list(signs))
                a, b, c = s2.coords[target]
                s2.shift(x - a, y - b, z - c)
                assert s2.coords[target] == src_coords_1
                if verify_points_in_place(s1, s2, matched):
                    print("Put in place %s to %s by %s %s shifting %s" % (s2.label, s1.label, shuffling, signs, (x - a, y - b, z - c)))
                    return
                else:
                    s2.inv_shift(x - a, y - b, z - c)
                    s2.inv_rotate(list(shuffling), list(signs))


def converge(scanners: List[Scanner]):
    in_place = {scanners[0]}
    while len(in_place) < len(scanners):
        print('IN PLACE', [x.label for x in in_place])
        for i in range(len(scanners)):
            s1 = scanners[i]
            for s2 in scanners[i + 1:]:
                if (s1 in in_place or s2 in in_place) and not (s1 in in_place and s2 in in_place):
                    matched = []
                    for a, b, d1 in s1.distances:
                        for c, d, d2 in s2.distances:
                            if d1 == d2:
                                matched.append(((a, b), (c, d)))
                    if len(matched) > 65:  # put s2 in place
                        print("GOOD", s1.label, s2.label)
                        first_src, first_target = matched[0]
                        if s2 in in_place:
                            put_s2_in_place(s2, s1, first_target, first_src, [(b, a) for a, b in matched])
                            in_place.add(s1)
                        else:
                            put_s2_in_place(s1, s2, first_src, first_target, matched)
                            in_place.add(s2)
                        verify_points_in_place(s1, s2, matched)


def part1(scanners) -> int:
    all_beacons = set()
    for s in scanners:
        all_beacons.update(set(s.coords))
    all_beacons = sorted(all_beacons, key=lambda x: x[0])
    return len(all_beacons)


def part2(scanners) -> int:
    distances = []
    for s1 in scanners:
        for s2 in scanners:
            distances.append(abs(s1.own_pos[0] - s2.own_pos[0]) + abs(s1.own_pos[1] - s2.own_pos[1]) + abs(s1.own_pos[2] - s2.own_pos[2]))
    return max(distances)


def main():
    data = open("19.txt", 'r').read()
    scanners = [Scanner(scanner) for scanner in data.split("\n\n")]
    converge(scanners)
    print(part1(scanners))
    print(part2(scanners))


main()
