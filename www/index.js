import { Petri, Bacter } from "bacter_rust";

// the _bg calls for the wasm linear memory
import { memory } from "bacter_rust/bacter_rust_bg";

const pre = document.getElementById("stats_canvas"); 
const universe = Petri.new();
var averageTickTime = 0.

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
      hAxis: {title: 'Aggressivity', minValue: 0., maxValue: 1.},
      vAxis: {title: 'Size', minValue: 0.2, maxValue: 1.},
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
    params_distribution_data.push([aggros[i] , sizes[i]]); 
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

  // Now, depending wether the "SLOW" button is pressed, the 
  // simulation proceeds by one tick or by 1000.

  if(document.getElementById('slow_button').checked == true)
  {
    // Ticking by one iteration:
    universe.tick(1);

    // Writing statistics as string
    pre.textContent = universe.get_stats_string();

    // Reading the linear memory for size and aggro:
    let bacters_number = universe.get_bacters_number();
    let algae_number = universe.get_algae_number();
    const aggros_ptr = universe.get_all_bacters_aggros();
    const aggros = new Float32Array(memory.buffer, aggros_ptr, bacters_number);
    const sizes_ptr = universe.get_all_bacters_sizes();
    const sizes = new Float32Array(memory.buffer, sizes_ptr, bacters_number);
    const positions_ptr = universe.get_all_bacters_position_interlaced();
    const positions = new Float32Array(memory.buffer, positions_ptr, bacters_number * 2);
    const algae_ptr = universe.get_all_algae_position_interlaced();
    const algae = new Float32Array(memory.buffer, algae_ptr, algae_number * 2);

    // Filling the canvas:
    var canvas = document.getElementById('petri_canvas');
    let ctx = canvas.getContext('2d')
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < bacters_number; i++) {
      drawCircle(ctx, positions[2*i], positions[2*i + 1], sizes[i] * 10., 'black', '', 2)
    }
    for (let i = 0; i < algae_number; i++) {
      drawCircle(ctx, algae[2*i], algae[2*i + 1], 1., 'green', '', 2)
    }
  }
  else
  {
    // Ticking the universe for a thousands iterations: 
    var startTime = new Date();
    universe.tick(1000);
    var endTime = new Date();
    var diffTime = endTime - startTime;
    averageTickTime = averageTickTime * 0.9 + diffTime;

    // Writing statistics as string
    var tmpString = (averageTickTime / 10).toString()
    pre.textContent = universe.get_stats_string() + " avg elapsed: " + tmpString + " ms.";

    // the populations growth plot needs an update at every tick (unregarding to the refresh rate)
    population_data.push([universe.get_iteration(),universe.get_bacters_number(), universe.get_algae_number()]); 
  }

  // looping, as always.
  requestAnimationFrame(renderLoop);
};

// Refreshing the image only when necessary! 
var intervalId = setInterval(function() { google.charts.setOnLoadCallback(drawChart); }, 500);

// Calling this once, it enters into an infinite loop.
requestAnimationFrame(renderLoop)


// CANVAS CIRCLES DRAWING:
// Taken from https://stackoverflow.com/questions/25095548/how-to-draw-a-circle-in-html5-canvas-using-javascript
function drawCircle(ctx, x, y, radius, fill, stroke, strokeWidth) {
  ctx.beginPath()
  ctx.arc(x, y, radius, 0, 2 * Math.PI, false)
  if (fill) {
    ctx.fillStyle = fill
    ctx.fill()
  }
  if (stroke) {
    ctx.lineWidth = strokeWidth
    ctx.strokeStyle = stroke
    ctx.stroke()
  }
}