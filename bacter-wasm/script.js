import { wasm_memory, Petri, default as init } from './pkg/bacter_wasm.js';
//import { memory } from "./bacter_rust_bg.wasm";

  async function run() {
    await init('./pkg/bacter_wasm_bg.wasm');

    // Creating the petri dish. The amount of bacters is 100 for a 500x500 dish
    const pre = document.getElementById("bacter_canvas");
    pre.width = window.innerWidth;
    pre.height = window.innerHeight;
    const total_bacters = 500 / (500*500) * pre.width * pre.height;
    const universe = Petri.new_with_params(pre.width, pre.height, total_bacters);
    console.warn(pre.width, pre.height, 100);

    var averageTickTime = 0.
    var fastMode = false;

    // Attaching the button
    document.getElementById("fast_btn").onclick = toggleFast;

    // Population Chart variables
    var population_data = [['Iteration', 'Bacters', 'Algae']];
    var population_options = {
          title: 'Global Populations',
          curveType: 'none',
          legend: { position: 'bottom' }
        };

    // Performances Chart
    var performances_data = [['iteration', 'mm']];
    var performances_options = {
      title: 'Performances: ms per 1000 steps',
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
      const aggros = new Float32Array(wasm_memory().buffer, aggros_ptr, bacters_number);
      const sizes_ptr = universe.get_all_bacters_sizes();
      const sizes = new Float32Array(wasm_memory().buffer, sizes_ptr, bacters_number);

      // Accessing the linear memory and adding to the array.
      // TODO - there must be a way to memcpy it or even reference?
      params_distribution_data = [['Aggressivity', 'Size']];
      for (let i = 0; i < bacters_number; i++) {
        params_distribution_data.push([aggros[i] , sizes[i]]);
      }
    }

    // Generic looping render.
    const renderLoop = () => {

      // Now, depending whether the "SLOW" button is pressed, the
      // simulation proceeds by one tick or by 1000.

        // Ticking by one iteration:
        if (fastMode){
            universe.tick(1000);
        }
        else
        {
            universe.tick(10);
        }

        // Reading the linear memory for size and aggro:
        let bacters_number = universe.get_bacters_number();
        let algae_number = universe.get_algae_number();
        const aggros_ptr = universe.get_all_bacters_aggros();
        const aggros = new Float32Array(wasm_memory().buffer, aggros_ptr, bacters_number);
        const sizes_ptr = universe.get_all_bacters_sizes();
        const sizes = new Float32Array(wasm_memory().buffer, sizes_ptr, bacters_number);
        const positions_ptr = universe.get_all_bacters_position_interlaced();
        const positions = new Float32Array(wasm_memory().buffer, positions_ptr, bacters_number * 2);
        const algae_ptr = universe.get_all_algae_position_interlaced();
        const algae = new Float32Array(wasm_memory().buffer, algae_ptr, algae_number * 2);

        // Filling the canvas:
        var canvas = document.getElementById('bacter_canvas');
        let ctx = canvas.getContext('2d')
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.rect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "#1B222C";
        ctx.fill();
        for (let i = 0; i < bacters_number; i++) {
          drawCircle(ctx, positions[2*i], positions[2*i + 1], sizes[i] * 10., 'white', '', 2)
        }
        for (let i = 0; i < algae_number; i++) {
          drawCircle(ctx, algae[2*i], algae[2*i + 1], 1., '#AFFC8C', '', 2)
        }

        // Updating basic stats.
        document.getElementById("stats").textContent="Total Bacters: " + bacters_number;
        document.getElementById("iteration").textContent="iteration #" + universe.get_iteration();

      // looping, as always, with a time frame set
        if (fastMode) {
            requestAnimationFrame(renderLoop);
        }
        else {
            setTimeout(() => {
                requestAnimationFrame(renderLoop);
            }, 1000 / 60);
        }
    };

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

    // Buttons Functions:
    function toggleFast() {
        fastMode = !fastMode;
    }

  }

  run();