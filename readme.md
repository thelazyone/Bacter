# Bacter
**A simple evolution simulator with basic cellular automata**

## Global Scope
The application of genetic algorithms is a widespread and relatively efficient solution for optimizations in many fields. Removing all the usefulness from that, it is possible to simulate the environment of very simple entities while they compete for food and get a chance to multiply while passing a slightly modified version of their own parameters to the offsprings.
The goal of this project is to observe not only how the population will move through generations towards one or more "local minima" in the parameters space, but also how the minima itself change with the mutating generations
### Disclaimer
This is not, in any form, a simulation of a real biological environment. Evolution works with any set of rules, and the rules used here are meant to obtain interesting dynamics rather than an accurate representation of what happens in a petri dish.
## World Rules
Ignoring the implementation choices, the individual automata (**cells**) move in a 2D space with something that resembles a brownian motion. 
<p align="center">
<img src="https://github.com/thelazyone/Bacter/blob/master/doc/ShortLife.png?" width="400" align="center">
</p>

### cells
When they interact with other cells, the two can either interact (with a chance of one killing the other) or part ways in different directions.
Over time, cells need to consume food: if the food reserve is depleted, the cell dies. If the food reserve exceeds a maximum, the cell splits in two, each with half of the food reserve and slightly different parameters. 
### Food sources
In order to procure food cells need to take it from the environment. 
A basic source of food can be algae, static cells that randomly appear in the environment and that can be eaten.
Killing other cells also works in that direction, but it is a risky path, since the concept of predator and prey are fairly mixed up here.
### Parameters
The parameter a cell has determine its behaviour and properties. 
<p align="center">
<img src="https://github.com/thelazyone/Bacter/blob/master/doc/Parameters.png" width="400" align="center">
</p>

Not all parameters will be subject to mutations (this indeed is something that could vary with future versions of the simulator), but it's probably wise to list them all here.
* **Speed**: Movement speed in the 2D space. A faster cell could expect to interact with more other cells (including algae).
* **Erratic**: Chances of changing direction of movement. The higher the value, the more static the cell becomes.
* **Aggressivity**: Chances of attacking a cell that came into contact.
* **Carnivore**: Efficiency of conversion to food from the preys' food reserve.
* **Erbivore**: Efficiency of conversion to food from the algaes' food value.
* **Size**: Has an effect when interacting with another cell, and effects the food necessary to split
* **Food Consumption**: Amount of food that is consumed per each unit of time.
### Opposing parameters
For the sake of balance, parameters should always have a "drawback": their value could either be complementary (the value of Erbivore and Carnivore must add up to, say, 1), or increase the food consumption proportionally.
Even without a clear outcome in mind the most obvious min-maxing of the parameters should be prevented through these trade-offs.
### Tracking Generations
Being able to understand how far two different cells are genealogically would be precious: first of all, it would help clusterize the cells in what we'll decide to define as different species.
A simple way to do so could be to provide cells with an N-bits hash, which is then passed to the offsprings with one bit swapped. It won't be a solid way to track the generations, but it should be able to track with increasingly less precision the genetic distance between two cells over a few hundreds generation at last.
## Goals
The main goal for Bacter is to show how dynamic the evolution process is. starting with a bunch of randomly parametrized cells well should expect a very quick initial selection, followed by the specialization of the remaining cells into one or more groups with similar parameters. 
If the environment and the rules are not well designed, all the cells will converge towards a local maxima and the population will never change, or they all perish and we'll get a mass extintion. 
However, if the environment is challenging and rewarding enough, multiple "species" should appear and mutate over time. Every mutation changes the environment too (since a different composition of cells drives the evolution itself in a different direction) and this moves the minima, in a somewhat cyclic oscillation.
An expected cycle could be:
1. A non-specialized population of cells sit in the center of the parameters space
2. some specialize into hunting, others into eating algae
3. the hunters over-specialize, driving the erbivores to extinction
4. most of the hunters become extinct as well, only the less-specialized would remain
5. return to point 1.

### Nota Bene
This is not an implementation of the [Lotka–Volterra equations](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations): evolution pushes some groups of cells to become predators and other to become prey, but only for short times. Discendants of preys could become predators within a few generations and vice versa. 
On the other hand, it would be fascinating to see if the oscillating patterns of predator and prey species would fit well with the Lotka-Volterra model.
## Development
Currently the development is at 0%. The goal is to write this with Rust, to get acquainted with the language, which fits well with the scope of a simulator.
### Future Steps
* Write a basic simulator: 
    * All cells are in the same area, interactions are computationally expensive but it should be enough to recreate the desired dynamics.
    * Use a minimal set of mutating parameters: two pairs of complementary parameters should do.
    * Implement a minimal GUI to see the behaviours at runtime.
* Write a data visualization tool: 
    * The history of the cells should be visible over time.
    * This allows to better fine-tune the environment with feedbacks and iteration.
* Optimize the simulator: 
    * Dividing the cells in multiple areas reduces the n^2 computational cost of the interactions check.
    * Use some fancy Rust optimization tools!
* Analyze the gradient in the parameters space:
    * By saving extra information about a few previous generation of cells it is possible to obtain a "general direction" from each point of the parameter space: given all the offspring originated from one cell, tracking the number of alive ones (and successive grandkids) and how their parameters changed could give *for each parameter* a rough indication of where the evolutive pressure was pushing to.
