#include <iostream>
#include <chrono>
#include <fstream>

#include "Grid.h"
#include "Bacter.h"

int main()
{
	std::cout << "Bacter - First Tests" << std::endl;

	std::chrono::steady_clock::time_point time0 = std::chrono::steady_clock::now();

	// Creating a grid
	Bacter::Grid testGrid (100, 100);
	std::shared_ptr<Bacter::Bacter> paragon;
	paragon.reset(new Bacter::Bacter);
	paragon->m_food = 50;
	for (int i = 0; i < 100; i++)
	{
		paragon->m_aggro = std::rand() % 100;
		paragon->m_speed = std::rand() % 100;
		paragon->m_vegan = std::rand() % 100;
		testGrid.Populate(1, paragon);
	}

	// Adding bacters?

	std::chrono::steady_clock::time_point time1 = std::chrono::steady_clock::now(); 

	// Testing stuff
	std::shared_ptr<Bacter::Results> testResults = std::make_shared<Bacter::Results>();
	std::ofstream ofs("out.txt", std::ofstream::out);
	for (int i = 0; i < 100000; i++)
	{
		testGrid.Run();

		// Retrieving statistics
		if (i % 10 == 0)
		{
			testResults.reset(new Bacter::Results());
			testGrid.GetStatistics(testResults);
			std::cout << "found " << testResults->m_bactersCount << " bacters at iteration " << i << "." << std::endl;
			std::cout << "done." << std::endl;

			ofs << testResults->m_bactersCount << std::endl;
		
			// If all dead, break.
			if (testResults->m_bactersCount == 0)
			{
				break;
			}
		}
	}

	std::chrono::steady_clock::time_point time2 = std::chrono::steady_clock::now();

	// Writing performances
	std::cout << "time for grid creation: " <<
		std::chrono::duration_cast<std::chrono::milliseconds>(time1 - time0).count()
		<< " ms." << std::endl;
	std::cout << "time for test run: " <<
		std::chrono::duration_cast<std::chrono::milliseconds>(time2 - time1).count()
		<< " ms." << std::endl;

	
	std::cin.get();
}
