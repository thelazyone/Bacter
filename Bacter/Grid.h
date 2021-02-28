#pragma once

#include <memory>
#include <vector>
#include <stdint.h>

// Forward declarations
namespace Bacter
{
	class Bacter;
}

namespace Bacter
{
	class BacterResults
	{
	public:
		BacterResults(
			const float& i_size,
			const float& i_speed,
			const float& i_diet,
			const uint64_t & i_hash):
			m_size(i_size), m_speed(i_speed), m_diet(i_diet), m_hash(i_hash)
		{};
		float m_size = 0;
		float m_speed = 0;
		float m_diet = 0;
		uint64_t m_hash = 0;
	};

	class CellResults
	{
	public:
		std::vector<BacterResults> m_bacters;
		float m_food = 0;
	};

	class Results
	{
	public:
		uint32_t m_iterationNumber = 0;
		uint32_t m_gridW = 0;
		uint32_t m_gridH = 0;

		std::vector<CellResults> m_cells;
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
		uint32_t GetIterationCounter() { return m_iterationCounter; };
		void GetGridSize(uint32_t & o_width, uint32_t & o_height);

	private:
		std::shared_ptr<Cell> m_rootCell;
		uint32_t m_iterationCounter = 0;
		uint32_t m_width = 0;
		uint32_t m_height = 0;
	};

	class HTTPInterface
	{
	public:
		HTTPInterface(const std::string & i_addr, const uint32_t & i_port);
		void SendGridStatistics(std::shared_ptr<Results> & i_results);

	private:
		std::string SerializeResults(std::shared_ptr<Results> & i_results);
		std::string m_address;
		uint32_t m_port = 0;
	};
}