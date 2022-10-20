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

for part in stbd.GetFootprints():
    loc = part.GetPosition()
    ref = part.GetReference()
    rot = part.GetOrientation().AsDegrees()
    if rot > 0:
        rot -= 180
    else:
        rot += 180

    locs[ref] = loc
    angs[ref] = rot

for part in port.GetFootprints():
    ref = part.GetReference()
    if ref in locs:
        newLoc = locs[ref]
        newLoc[0] *= -1
        newRot = angs[ref]
        part.SetPosition(newLoc)
        part.SetOrientation(EDA_ANGLE(newRot, DEGREES_T))
        print("Setting location of " + str(ref))

port.Save(portLoc)


