import { Petri, Bacter } from "bacter_rust";

// the _bg calls for the wasm linear memory
import { memory } from "bacter_rust/bacter_rust_bg";

const pre = document.getElementById("stats_canvas"); 
const universe = Petri.new();

// Preparing the chart:
google.charts.load('current', {
  'packages':['corechart']
});
google.charts.setOnLoadCallback(drawChart); 

// Population Chart variables
var population_data = [['Iteration', 'Bacters', 'Algae']]; 
var population_options = {
      title: 'Global Populations',
      curveType: 'none',
      legend: { position: 'bottom' }
    };


// Parameters Distribution Chart variables
var params_distribution_data = [['Aggressivity', 'Size']]; 
var params_distribution_options = {
      title: 'Aggressivity vs. Size projection',
      hAxis: {title: 'Aggressivity', minValue: -100, maxValue: 100},
      vAxis: {title: 'projection', minValue: -100, maxValue: 100},
      legend: 'none'
    };


function drawChart() {
  let bacters_number = universe.get_bacters_number();
  let algae_number = universe.get_algae_number();

  // Reading the linear memory for size and aggro:
  const aggros_ptr = universe.get_all_bacters_aggros();
  const aggros = new Float32Array(memory.buffer, aggros_ptr, bacters_number);
  const sizes_ptr = universe.get_all_bacters_sizes();
  const sizes = new Float32Array(memory.buffer, sizes_ptr, bacters_number);

  // Accessing the linear memory and adding to the array.
  // TODO - there must be a way to memcpy it or even reference?
  params_distribution_data = [['Aggressivity', 'Size']]; 
  for (let i = 0; i < bacters_number; i++) {
    params_distribution_data.push([aggros[i] * 200 - 100. , sizes[i] * 200 - 100]); 
  }

  // Drawing the populations plot:
  var population_chart = new google.visualization.LineChart(document.getElementById('data_chart'));
  population_chart.draw(google.visualization.arrayToDataTable(population_data), population_options);

  // Drawing the last-iteration parameters space:
  var params_distribution_chart = new google.visualization.ScatterChart(document.getElementById('params_chart'));
  params_distribution_chart.draw(google.visualization.arrayToDataTable(params_distribution_data), params_distribution_options);
}

// Generic looping render.
const renderLoop = () => {
  // Ticking the universe:
  universe.tick();

  // Writing statistics as string
  pre.textContent = universe.get_stats_string();

  // the populations growth plot needs an update at every tick (unregarding to the refresh rate)
 
  population_data.push([universe.get_iteration(),universe.get_bacters_number(), universe.get_algae_number()]); 

  // looping, as always.
  requestAnimationFrame(renderLoop);
};

// Refreshing the image only when necessary! 
var intervalId = setInterval(function() { google.charts.setOnLoadCallback(drawChart); }, 500);

// Calling this once, it enters into an infinite loop.
requestAnimationFrame(renderLoop)