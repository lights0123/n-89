/*  TiEmu - a TI calculator emulator
 *
 *  Character to key conversion routine
 *  Copyright (c) 2006 Kevin Kofler
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */

#include "keydefs.h"
#include <stdlib.h>
#include <string.h>

static const int keys[256][5] = { { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { TIKEY_ENTER1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_ESCAPE, TIKEY_DIAMOND, TIKEY_Q, TIKEY_VOID },
	                              { TIKEY_CLEAR, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_2, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_3, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_4, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_5, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_6, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_7, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_8, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_9, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_A, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_B, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_C, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_D, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_E, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_2, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_3, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_4, -1 },
	                              { TIKEY_SPACE, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_2, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_3, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_4, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_5, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_6, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_7, -1 },
	                              { TIKEY_PALEFT, -1, -1, -1, -1 },
	                              { TIKEY_PARIGHT, -1, -1, -1, -1 },
	                              { TIKEY_MULTIPLY, -1, -1, -1, -1 },
	                              { TIKEY_PLUS, -1, -1, -1, -1 },
	                              { TIKEY_COMMA, -1, -1, -1, -1 },
	                              { TIKEY_MINUS, -1, -1, -1, -1 },
	                              { TIKEY_PERIOD, -1, -1, -1, -1 },
	                              { TIKEY_DIVIDE, -1, -1, -1, -1 },
	                              { TIKEY_0, -1, -1, -1, -1 },
	                              { TIKEY_1, -1, -1, -1, -1 },
	                              { TIKEY_2, -1, -1, -1, -1 },
	                              { TIKEY_3, -1, -1, -1, -1 },
	                              { TIKEY_4, -1, -1, -1, -1 },
	                              { TIKEY_5, -1, -1, -1, -1 },
	                              { TIKEY_6, -1, -1, -1, -1 },
	                              { TIKEY_7, -1, -1, -1, -1 },
	                              { TIKEY_8, -1, -1, -1, -1 },
	                              { TIKEY_9, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_THETA, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_M, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_0, -1, -1, -1 },
	                              { TIKEY_EQUALS, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PERIOD, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_8, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_9, -1 },
	                              { TIKEY_SHIFT, TIKEY_A, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_B, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_C, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_D, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_E, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_F, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_G, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_H, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_I, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_J, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_K, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_L, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_M, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_N, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_O, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_P, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_Q, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_R, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_S, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_T, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_U, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_V, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_W, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_X, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_Y, -1, -1, -1 },
	                              { TIKEY_SHIFT, TIKEY_Z, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_COMMA, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_EQUALS, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_DIVIDE, -1, -1, -1 },
	                              { TIKEY_POWER, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_A, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_B, -1 },
	                              { TIKEY_A, -1, -1, -1, -1 },
	                              { TIKEY_B, -1, -1, -1, -1 },
	                              { TIKEY_C, -1, -1, -1, -1 },
	                              { TIKEY_D, -1, -1, -1, -1 },
	                              { TIKEY_E, -1, -1, -1, -1 },
	                              { TIKEY_F, -1, -1, -1, -1 },
	                              { TIKEY_G, -1, -1, -1, -1 },
	                              { TIKEY_H, -1, -1, -1, -1 },
	                              { TIKEY_I, -1, -1, -1, -1 },
	                              { TIKEY_J, -1, -1, -1, -1 },
	                              { TIKEY_K, -1, -1, -1, -1 },
	                              { TIKEY_L, -1, -1, -1, -1 },
	                              { TIKEY_M, -1, -1, -1, -1 },
	                              { TIKEY_N, -1, -1, -1, -1 },
	                              { TIKEY_O, -1, -1, -1, -1 },
	                              { TIKEY_P, -1, -1, -1, -1 },
	                              { TIKEY_Q, -1, -1, -1, -1 },
	                              { TIKEY_R, -1, -1, -1, -1 },
	                              { TIKEY_S, -1, -1, -1, -1 },
	                              { TIKEY_T, -1, -1, -1, -1 },
	                              { TIKEY_U, -1, -1, -1, -1 },
	                              { TIKEY_V, -1, -1, -1, -1 },
	                              { TIKEY_W, -1, -1, -1, -1 },
	                              { TIKEY_X, -1, -1, -1, -1 },
	                              { TIKEY_Y, -1, -1, -1, -1 },
	                              { TIKEY_Z, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PALEFT, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_C, -1 },
	                              { TIKEY_2ND, TIKEY_PARIGHT, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_D, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_4, TIKEY_F, -1 },

	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_2, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_3, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_4, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_5, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_6, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_7, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_8, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_9, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_A, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_C, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_D, -1 },
	                              { TIKEY_2ND, TIKEY_POWER, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_F, -1 },
	                              { TIKEY_DIAMOND, TIKEY_G, TIKEY_SHIFT, TIKEY_S, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_G, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_H, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_I, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_J, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_K, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_L, -1 },
	                              { TIKEY_2ND, TIKEY_1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_5, -1 },
	                              { TIKEY_2ND, TIKEY_I, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_6, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_9, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_A, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_B, -1 },
	                              { TIKEY_DIAMOND, TIKEY_0, -1, -1, -1 },
	                              { TIKEY_DIAMOND, TIKEY_EQUALS, -1, -1, -1 },
	                              { TIKEY_DIAMOND, TIKEY_PERIOD, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_8, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_E, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_F, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_G, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_H, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_I, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_J, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_K, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_L, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_F, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_M, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_F },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_Q, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_G, -1 },
	                              { TIKEY_NEGATE, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_N, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_7, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_N, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_I, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_J, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_K, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_1, TIKEY_B, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_O, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { -1, -1, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_H, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_D },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_R, -1 },
	                              { TIKEY_2ND, TIKEY_8, TIKEY_BACKSPACE, -1, -1 },
	                              { TIKEY_2ND, TIKEY_7, TIKEY_BACKSPACE, -1, -1 },
	                              { TIKEY_2ND, TIKEY_J, -1, -1, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_3, TIKEY_P, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_9 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_B },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_D },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_9 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_L, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_B },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_3 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_5 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_A },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_7 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_A },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_C },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_1, TIKEY_E },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_2, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_3, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_A },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_2, TIKEY_M, -1 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_4, TIKEY_C },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_2 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_4 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_6 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_5, TIKEY_8 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_B },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_9 },
	                              { TIKEY_2ND, TIKEY_PLUS, TIKEY_5, TIKEY_6, TIKEY_C } };

static const int keys89_12[5] = { TIKEY_2ND, TIKEY_ESCAPE, TIKEY_HOME, TIKEY_VOID, -1 };
static const int keys89_58[3] = { TIKEY_2ND, TIKEY_4, -1 };
static const int keys89_59[3] = { TIKEY_2ND, TIKEY_9, -1 };
static const int keys89_92[3] = { TIKEY_2ND, TIKEY_2, -1 };
static const int keys89_142[5] = { TIKEY_DIAMOND, TIKEY_PALEFT, TIKEY_SHIFT, TIKEY_3, -1 };
static const int keys89_149[2] = { TIKEY_EE, -1 };
static const int keys89_151[3] = { TIKEY_2ND, TIKEY_CATALOG, -1 };
static const int keys89_190[3] = { TIKEY_DIAMOND, TIKEY_CATALOG, -1 };

int *chars_to_keys(const char *chars, int ti89) {
	int *buffer, *q, i, shift = 0;
	const unsigned char *p;
	const int *row;
	buffer = malloc((strlen(chars) * 6 + 1) * sizeof(int));
	if(!buffer) return NULL;
	q = buffer;
	for(p = (const unsigned char *)chars; *p; p++) {
		row = keys[*p];
		if(ti89) {
			switch(*p) {
			case 12: // Form Feed => QUIT + HOME
				row = keys89_12;
				break;
			case 58: // :
				row = keys89_58;
				break;
			case 59: // ;
				row = keys89_59;
				break;
			case 92: // backslash
				row = keys89_92;
				break;
			case 142: // SIGMA
				row = keys89_142;
				break;
			case 149: // EE
				row = keys89_149;
				break;
			case 151: // imaginary i
				row = keys89_151;
				break;
			case 190: // infinity
				row = keys89_190;
				break;
			default:
				break;
			}
		}
		for(i = 0; i < 5; i++) {
			if(row[i] == -1) break;
			if(ti89) {
				switch(row[i]) {
#define ALPHA(letter, key89)             \
	case TIKEY_##letter:                 \
		if(!shift) *(q++) = TIKEY_ALPHA; \
		*(q++) = TIKEY_##key89;          \
		shift = 0;                       \
		break;
					ALPHA(A, EQUALS)
					ALPHA(B, PALEFT)
					ALPHA(C, PARIGHT)
					ALPHA(D, COMMA)
					ALPHA(E, DIVIDE)
					ALPHA(F, PIPE)
					ALPHA(G, 7)
					ALPHA(H, 8)
					ALPHA(I, 9)
					ALPHA(J, MULTIPLY)
					ALPHA(K, EE)
					ALPHA(L, 4)
					ALPHA(M, 5)
					ALPHA(N, 6)
					ALPHA(O, MINUS)
					ALPHA(P, STORE)
					ALPHA(Q, 1)
					ALPHA(R, 2)
					ALPHA(S, 3)
					ALPHA(U, PLUS)
					ALPHA(V, 0)
					ALPHA(W, PERIOD)
					ALPHA(SPACE, NEGATE)
				case TIKEY_SHIFT:
					*(q++) = TIKEY_SHIFT;
					shift = 1;
					break;
				default:
					*(q++) = row[i];
					shift = 0;
					break;
				}
			} else {
				*(q++) = row[i];
			}
		}
	}
	*q = -1;
	return buffer;
}
