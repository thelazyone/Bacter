// Bacter.cpp : This file contains the 'main' function. Program execution begins and ends there.
//
#include "Bacter.h"
#include <cstdlib>
#include <algorithm>

namespace constants
{
	static const int32_t mutationValue = 5;
	static const float minVal = 0;
	static const float maxVal = 100;
}

float MutateValue(const float i_val)
{
	float resVal = i_val + (std::rand() % (constants::mutationValue * 100)) * 0.01 - (constants::mutationValue * 0.5);
	return std::min(std::max(constants::minVal, resVal), constants::maxVal);
}

Bacter::Bacter::Bacter(const Bacter & i_prev)
{
	// Modifying the hash: it's the same of the parent's hash with ONE bit flipped.
	m_hash = i_prev.m_hash ^ 1 << (std::rand() % 64);
	m_food = i_prev.m_food;

	// Now mutating the params.
	m_speed = MutateValue(i_prev.m_speed);
	m_aggro = MutateValue(i_prev.m_aggro);
	m_vegan = MutateValue(i_prev.m_vegan);
}