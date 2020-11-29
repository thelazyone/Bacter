// Bacter.cpp : This file contains the 'main' function. Program execution begins and ends there.
//
#include "Bacter.h"
#include <cstdlib>

namespace constants
{
	static const int32_t mutationValue = 5;
}

Bacter::Bacter::Bacter(const Bacter & i_prev)
{
	// Modifying the hash: it's the same of the parent's hash with ONE bit flipped.
	m_hash = i_prev.m_hash ^ 1 << (std::rand() % 64);

	// Now mutating the params.
	m_speed = i_prev.m_speed + std::rand() % constants::mutationValue - constants::mutationValue / 2;
	m_aggro = i_prev.m_aggro + std::rand() % constants::mutationValue - constants::mutationValue / 2;
} 