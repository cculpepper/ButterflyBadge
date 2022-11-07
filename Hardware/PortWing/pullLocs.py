#! /usr/bin/env python3
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2022 Chris Culpepper <cculpepper1214@gmail.com>
#
# Distributed under terms of the MIT license.

"""

"""
from pcbnew import *
projLoc = "/home/chris/Code/ButterflyBadge/Hardware/"
portLoc = projLoc + "PortWing/PortWing.kicad_pcb"
stbdLoc = projLoc + "StarboardWing/StarboardWing.kicad_pcb"


port = LoadBoard(portLoc)
stbd = LoadBoard(stbdLoc)

locs = {}
angs = {}

def printFootprints(board, wing_side, string_number, x_offset):
    for part in board.GetFootprints():
        loc = part.GetPosition()
        ref = part.GetReference()
        if ref[1:].isnumeric():
            locs[ref] = loc
        else:
            print(f"{ref} is not numeric")

    refs = list(locs.keys())
    print(refs)
    refs.sort(key=lambda x:int(x[1:]))
    # D5 is at  -171, 131, Kicad shows it as 171958893, 131126393,
    for ref in refs:
        if "D" in ref:
            print(f"{wing_side}, {string_number}, {ref[1:]}, {ref}, {locs[ref][0] * 10**-6+x_offset}, {locs[ref][1] * 10**-6}, ")

print(f"Side, String number, LED Number, Reference, X, Y")
printFootprints(stbd, "stbd", 0, 0)
printFootprints(port, "port", 1, 300)

