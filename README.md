This is a simple simulation where boids (see https://en.wikipedia.org/wiki/Boids) evolve in a small environment where it can eat either food or poison.
A simple genetics evolutio algorithm makes each have distinct capabilities, resulting in various (but still pretty similar) behaviors: some will be very afraid of poison and will avoid it at all cost, even if they are starving, while others will run at the food, without thinking about the danger that poison in their way could represent.
Their differences include their perception of food or poison and how much they are attracted to / afraid of food / poison.
These differences can be visualised with the 'debug' executable, with the circles around them corresponding to their perception of food (green) and poison (red).
