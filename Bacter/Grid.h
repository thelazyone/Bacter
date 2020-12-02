#pragma once

#include <memory>
#include <vector>
#include <stdint.h>

namespace Bacter
{
	class Bacter;
}

namespace Bacter
{
	class Results
	{
	public:
		uint32_t m_bactersCount = 0;
	};

	class Cell
	{
		// Linked Cells:
	public:
		std::shared_ptr<Cell> m_top;
		std::shared_ptr<Cell> m_bottom;
		std::shared_ptr<Cell> m_right;
		std::shared_ptr<Cell> m_left;
		std::shared_ptr<Cell> m_next;

		// Bacteria in it: 
	public:
		std::vector<std::shared_ptr<Bacter>> m_bacters;

		// Food in it:
	public:
		float m_food = 0;

		// Functions:
	public:
		void Run();
		void Populate(const uint32_t& i_number, const std::shared_ptr<Bacter>& i_bacter);
		void GetStatistics(std::shared_ptr<Results> & o_results);
	};

	class Grid
	{
	public:
		Grid(const uint32_t i_width, const uint32_t i_height);
		void Run();
		void Populate(const uint32_t& i_number, const std::shared_ptr<Bacter>& i_bacter);
		void GetStatistics(std::shared_ptr<Results> & o_results);

	private:
		std::shared_ptr<Cell> m_rootCell;
	};
}