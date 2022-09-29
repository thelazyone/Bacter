import { Petri } from "bacter_rust";

const pre = document.getElementById("stats_canvas");
const universe = Petri.new();

// Preparing the chart:
google.charts.load('current', {
  'packages':['corechart']
});
google.charts.setOnLoadCallback(drawChart); 

var plot_data = [['Iteration', 'Bacters', 'Algae']]; 
var plot_options = {
      title: 'Global Populations',
      curveType: 'none',
      legend: { position: 'bottom' }
    };

function drawChart() {
  var chart = new google.visualization.LineChart(document.getElementById('data_chart'));

  chart.draw(google.visualization.arrayToDataTable(plot_data), plot_options);
  //chart.draw(data, options);
}

// Generic looping render.
const renderLoop = () => {
  // Ticking the universe:
  universe.tick();

  // Writing statistics and updating the graph:
  pre.textContent = universe.get_stats_string();
  plot_data.push([universe.get_iteration(),universe.get_bacters_number(), universe.get_algae_number()]); 

  //google.charts.setOnLoadCallback(drawChart); 

  requestAnimationFrame(renderLoop);
};

var intervalId = setInterval(function() { google.charts.setOnLoadCallback(drawChart); }, 2500);

// Calling this once, it enters into an infinite loop.
requestAnimationFrame(renderLoop)