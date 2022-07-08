import init, {BF, run} from "./pkg/brainfuck.js";
await init();

run();

// Initializes brainfuck.
// Setting instance to window for ease of access in the js console.
window.BF = BF;
window.bf = BF.new();
// Number of steps per step. See fn steps() in src/lib.rs.
window.stepsize = 100;

// Helper function for creating buttons.
function button(id, func) {
   document.getElementById(id).addEventListener('click', func);
}

// Step + print
button("sp", () => {
   console.time(`Execution time for step`)
   window.bf.steps(window.stepsize);
   document.getElementById("state").innerText = bf.get_state();
   console.timeEnd(`Execution time for step`)
})

button("print", () => window.bf.print())
button("step", () => window.bf.step())
button("comp", () => window.bf.set_code(document.getElementById("code").value))
button("clear", () => document.getElementById("cout").innerText = '')

// Initialize memory map.

// Display memory.
const renderMemory = () => {
   // Calculate how many memory cells within the 800px or less space.
   let numberOfCells = document.getElementsByClassName("mid")[0].clientWidth/50
   document.getElementById("memory").innerHTML = '';
   // Rows
   for(let j = 0; j<2; j++) {
      let cont = document.createElement("div");
      cont.classList = "row"

      // Columns
      for(let i = 0; i<Math.floor(numberOfCells); i++) {
         let el = document.createElement("span");
         el.innerText = 0;
         el.classList = "cell"
         el.id = "m"+(j*(Math.floor(numberOfCells))+i);
         cont.append(el)
      }
      document.getElementById("memory").append(cont);
   }
}

renderMemory()

window.addEventListener("resize", renderMemory)
