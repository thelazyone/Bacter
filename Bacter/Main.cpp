#include <iostream>
#include <chrono>

#include "Grid.h"

int main()
{
	std::cout << "Bacter - First Tests" << std::endl;

	std::chrono::steady_clock::time_point time0 = std::chrono::steady_clock::now();

	// Creating a grid
	Bacter::Grid testGrid (100, 100);

	std::chrono::steady_clock::time_point time1 = std::chrono::steady_clock::now();

	// Testing stuff
	testGrid.Run();

	std::chrono::steady_clock::time_point time2 = std::chrono::steady_clock::now();

	// Retrieving statistics
	std::shared_ptr<Bacter::Results> testResults = std::make_shared<Bacter::Results>();
	testGrid.GetStatistics(testResults);

	std::chrono::steady_clock::time_point time3 = std::chrono::steady_clock::now();

	std::cout << "done." << std::endl;


	// Writing performances
	std::cout << "time for grid creation: " <<
		std::chrono::duration_cast<std::chrono::milliseconds>(time1 - time0).count()
		<< " ms." << std::endl;
	std::cout << "time for test run: " <<
		std::chrono::duration_cast<std::chrono::milliseconds>(time2 - time1).count()
		<< " ms." << std::endl;
	std::cout << "time for retrieving: " <<
		std::chrono::duration_cast<std::chrono::milliseconds>(time3 - time2).count()
		<< " ms." << std::endl;
	
	std::cin.get();
}
