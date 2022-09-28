import { Petri } from "bacter_rust";

const pre = document.getElementById("stats_canvas");
const universe = Petri.new();

const renderLoop = () => {
    pre.textContent = universe.get_stats();
    universe.tick();
  
    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop)