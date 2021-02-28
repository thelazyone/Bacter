#pragma once

#include <stdint.h>

namespace Bacter
{

#pragma pack(1)
	class Bacter
	{
	public:
		Bacter(const Bacter & i_prev);
		Bacter() {};

	public:
		enum status
		{
			ok = 0,
			dead,
			moved
		};

		// Properties:
	public:
		int64_t m_hash = 0;
		bool m_ignoreNextIteration = false;
		float m_speed = 0; // from 0 to 100
		float m_aggro = 0; // from 0 to 100
		float m_vegan = 0; // from 0 to 100
		float m_food = 50; // between 0 and 100;
		float m_diet = 0; // TO ADD IN THE LOGIC!
		float m_size = 0; // TO ADD IN THE LOGIC!

		int8_t m_status = false;
	};
}