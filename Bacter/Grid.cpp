#include <vector>

#include "Grid.h"
#include "Bacter.h"


Bacter::Grid::Grid(const uint32_t i_width, const uint32_t i_height)
{
	// Building up a vector for the torus.
	std::vector<std::shared_ptr<Cell>> col(i_height);
	std::vector<std::vector<std::shared_ptr<Cell>>> gridSpace(i_width, col);

	// Instantiating 
	for (int32_t rowIdx = 0; rowIdx < i_height; rowIdx++)
	{
		for (int32_t colIdx = 0; colIdx < i_width; colIdx++)
		{
			gridSpace[colIdx][rowIdx] = std::make_shared<Cell>();
		}
	}

	// linking the first node.
	m_rootCell = gridSpace[0][0];

	// Linking all the cells, inlcuding the ones in the wrapped space 
	for (int32_t rowIdx = 0; rowIdx < i_height; rowIdx++)
	{
		for (int32_t colIdx = 0; colIdx < i_width; colIdx++)
		{
			gridSpace[colIdx][rowIdx]->m_right = gridSpace[colIdx][(rowIdx + 1) % i_width];
			gridSpace[colIdx][rowIdx]->m_left = gridSpace[colIdx][(rowIdx - 1 + i_width) % i_width];
			gridSpace[colIdx][rowIdx]->m_top = gridSpace[(colIdx - 1 + i_height) % i_height][rowIdx];
			gridSpace[colIdx][rowIdx]->m_bottom = gridSpace[(colIdx + 1) % i_height][rowIdx];

			if (colIdx == i_width - 1)
			{
				gridSpace[colIdx][rowIdx]->m_next = gridSpace[0][(rowIdx + 1) % i_width];
			}
			else
			{
				gridSpace[colIdx][rowIdx]->m_next = gridSpace[colIdx][rowIdx]->m_right;
			}
		}
	}
}

void Bacter::Grid::Run()
{
	// Starting with the root
	std::shared_ptr<Cell> currCell = m_rootCell;
	currCell->Run();
	currCell = currCell->m_next;
	while (currCell != m_rootCell)
	{
		currCell->Run();
		currCell = currCell->m_next;
	}
}

void Bacter::Grid::GetStatistics(std::shared_ptr<Results>& o_results)
{
	// Starting with the root
	std::shared_ptr<Cell> currCell = m_rootCell;
	currCell->GetStatistics(o_results);
	currCell = currCell->m_next;
	while (currCell != m_rootCell)
	{
		currCell->GetStatistics(o_results);
		currCell = currCell->m_next;
	}
}

void Bacter::Cell::Run()
{
	// Iterating throught the elements
	for (auto & bacter : m_bacters)
	{
		// Interaction procedure
		// TODO

		// Food procedure
		// TODO

		// Reproduction procedure
		// TODO

		// Moving procedure
		// TODO
	}
}

void Bacter::Cell::GetStatistics(std::shared_ptr<Results>& o_results)
{
	// Doing stuff here
	// TODO
}
