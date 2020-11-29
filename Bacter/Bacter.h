#pragma once

#include <stdint.h>

namespace Bacter
{

#pragma pack(1)
	class Bacter
	{
	public:
		Bacter(const Bacter & i_prev);

		// Properties:
	public:
		int64_t m_hash = 0;
		bool m_ignoreNextIteration = false;
		int8_t m_speed = 0;
		int8_t m_aggro = 0;
		uint8_t m_food = 50; // between 0 and 100;
	};
}