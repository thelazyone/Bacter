#include <vector>
#include <algorithm>	// for remove_if
#include <ctime>		// for srand seed

#include "Grid.h"
#include "Bacter.h"


namespace constants
{
	//static const int32_t cellFoodIncrement = 20;
	//static const int32_t cellFoodMax = 100;
	static const float grassEat = 10;
	static const float aggroConsumption = 1;
	static const float speedConsumption = 0.2;
	static const float existingConsumption = 1;
	static const float foodForBaby = 100;
}

//notes 0112
// Infinite grass poses a problem: there is no motivation to move!
// - If you get to the right spot, you have an infinite growth of cows, because anyone not perfecly vegan would lose competitivity over the others.
// - At the same time, it reduces the amount of eating efficiency you can offer, so most of the times they just all starve


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

	// Linking all the cells, inlcuding the ones in the wrapped space 
	for (int32_t rowIdx = 0; rowIdx < i_height; rowIdx++)
	{
		for (int32_t colIdx = 0; colIdx < i_width; colIdx++)
		{
			gridSpace[colIdx][rowIdx]->m_right = gridSpace[colIdx][(rowIdx + 1) % (i_height - 1)];
			gridSpace[colIdx][rowIdx]->m_left = gridSpace[colIdx][(rowIdx - 1 + i_height) % (i_height - 1)];
			gridSpace[colIdx][rowIdx]->m_top = gridSpace[(colIdx - 1 + i_width) % (i_width - 1)][rowIdx];
			gridSpace[colIdx][rowIdx]->m_bottom = gridSpace[(colIdx + 1) % (i_width - 1)][rowIdx];

			//If it's the last element of a line, assigning the first element of the next line.
			if (colIdx == i_width - 1)
			{
				gridSpace[colIdx][rowIdx]->m_next = gridSpace[0][(rowIdx + 1) % (i_height - 1)];
			}

			// Otherwise assigning the one on its right.
			else
			{
				gridSpace[colIdx][rowIdx]->m_next = gridSpace[colIdx][rowIdx]->m_right;
			}
		}
	}

	// linking the first node.
	m_rootCell = gridSpace[0][0];
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

void Bacter::Grid::Populate(const uint32_t & i_number, const std::shared_ptr<Bacter>& i_bacter)
{
	// Starting with the root
	std::shared_ptr<Cell> currCell = m_rootCell;
	currCell->Populate(i_number, i_bacter);
	currCell = currCell->m_next;
	while (currCell != m_rootCell)
	{
		currCell->Populate(i_number, i_bacter);
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
	std::srand(std::time(0));

	if (m_bacters.empty())
	{
		// Nothing to do in this case
		return;
	}

	// Support list for newborns
	std::vector<std::shared_ptr<Bacter>> newborns;

	// Iterating throught the elements
	uint32_t bactIdx = m_bacters.size() > 1 ? std::rand() % (m_bacters.size() - 1) : 0;
	for (uint32_t sizeIdx = 0; sizeIdx < m_bacters.size(); sizeIdx++)
	{
		// Interaction procedure
		
		// If it's dead, skipping it and wait for the corpse collector later on.
		if (m_bacters[bactIdx]->m_status == Bacter::status::dead ||
			m_bacters[bactIdx]->m_status == Bacter::status::moved)
		{
			continue;
		}

		// Look IF there is an anthagonist
		if (m_bacters.size() > 1 && m_bacters[bactIdx]->m_aggro >= std::rand() % 100)
		{
			// If the aggro roll is success, you get a fight.

			// Picking the opponent: any one of the others, including the ones that already acted
			uint32_t oppIdx = 0;
			if (m_bacters.size() > 2)
			{
				oppIdx = (bactIdx + 1 + std::rand() % (m_bacters.size() - 2)) % (m_bacters.size() - 1);
			}
			else
			{
				oppIdx = (bactIdx + 1) % (m_bacters.size() - 1);
			}

			// If interacting with a corpse, skipping it for now.
			if (m_bacters[oppIdx]->m_status == Bacter::status::dead ||
				m_bacters[oppIdx]->m_status == Bacter::status::moved)
			{
				continue;
			}

			// Opposed roll on Aggro. Aggr vs Aggr +- 1d10
			uint32_t winner = 0;
			uint32_t loser = 0;
			if (m_bacters[bactIdx]->m_aggro - m_bacters[oppIdx]->m_aggro + (std::rand() % 20 - 10) >= 0)
			{
				winner = bactIdx;
				loser = oppIdx;
			}
			else
			{
				winner = oppIdx;
				loser = bactIdx;
			}

			// Killed it! Taking aaall its food. 
			m_bacters[loser]->m_status = Bacter::status::dead;
			m_bacters[winner]->m_food += m_bacters[loser]->m_food * (100 - m_bacters[winner]->m_vegan) / 100;
			m_bacters[loser]->m_food = 0;
		}

		if (m_bacters[bactIdx]->m_status == Bacter::status::ok)
		{
			// Food (grass) eating
			m_bacters[bactIdx]->m_food += 0.01 * (m_bacters[bactIdx]->m_vegan) * constants::grassEat;

			// Food consumption for aggro
			m_bacters[bactIdx]->m_food -= 0.01 * (m_bacters[bactIdx]->m_aggro) * constants::aggroConsumption;

			// Food consumption for existing:
			m_bacters[bactIdx]->m_food -= constants::existingConsumption;

			// Moving procedure
			if (m_bacters[bactIdx]->m_speed >= std::rand() % 100)
			{
				// Going into a random direction:
				switch (std::rand() % 3)
				{
				case 0:
					m_top->m_bacters.push_back(m_bacters[bactIdx]);
					break;
				case 1:
					m_left->m_bacters.push_back(m_bacters[bactIdx]);
					break;
				case 2:
					m_bottom->m_bacters.push_back(m_bacters[bactIdx]);
					break;
				case 3:
					m_right->m_bacters.push_back(m_bacters[bactIdx]);
					break;
				}

				// Set the element as "moved"
				m_bacters[bactIdx]->m_status = Bacter::status::moved;

				// Then consuming food
				m_bacters[bactIdx]->m_food -= constants::speedConsumption * (m_bacters[bactIdx]->m_speed) / 100;
			}

			// Reproduction procedure
			if (m_bacters[bactIdx]->m_food >= constants::foodForBaby)
			{
				// halving the food
				m_bacters[bactIdx]->m_food /= 2;

				// Creating a new baby
				newborns.push_back(std::shared_ptr<Bacter>(new Bacter(*m_bacters[bactIdx])));
			}

			// Starving
			if (m_bacters[bactIdx]->m_food <= 0)
			{
				m_bacters[bactIdx]->m_food = 0;

				// dead.
				m_bacters[bactIdx]->m_status = Bacter::status::dead;
			}
		}

		// Updating the index to the next value. If reaching the end, restarting
		// from the beginning
		bactIdx = m_bacters.size() > 1 ? (bactIdx + 1) % (m_bacters.size() - 1) : bactIdx;
	}

	// Corpses collector.
	// Note that remove_if returns the iterator of the "point to erase" in the vector. So the erase() method has to be called anyways.
	m_bacters.erase(std::remove_if(m_bacters.begin(), m_bacters.end(), [](const std::shared_ptr<Bacter>& elem)
	{
		if (!elem)
		{
			return true;
		}
		if(elem->m_status == Bacter::status::dead || elem->m_status == Bacter::status::moved)
		{
			return true;
		}

	}),	m_bacters.end());

	// Newborns:
	for (auto & elem : newborns)
	{
		m_bacters.push_back(elem);
	}
}

void Bacter::Cell::GetStatistics(std::shared_ptr<Results>& o_results)
{
	o_results->m_bactersCount += m_bacters.size();

	// Doing stuff here
	// TODO
}

void Bacter::Cell::Populate(const uint32_t& i_number, const std::shared_ptr<Bacter>& i_bacter)
{
	// adding N bacters to the grid
	for (uint32_t bactIdx = 0; bactIdx < i_number; bactIdx++)
	{
		m_bacters.push_back(std::shared_ptr<Bacter>(new Bacter(*i_bacter)));
	}
}
