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

// Memory array for rendering memory.
let mem = new Uint8Array(256, 0);
let pcVal = 0; 

// Step + print
button("sp", () => {
   console.time(`Execution time for step`)
   window.bf.steps(window.stepsize);
   let json = JSON.parse(bf.get_state());
   console.timeEnd(`Execution time for step`)
   mem = json.memory;
   pcVal = json.memory_counter;
   renderMemory();
})

// button("print", () => window.bf.print())
//button("step", () => window.bf.step())
button("comp", () => window.bf.set_code(document.getElementById("code").value))
button("clear", () => document.getElementById("cout").innerText = '')

// Initialize memory map.

let rowLength = 0;
const ROWS = 2;

// Display memory.
const renderMemory = () => {
   // Calculate how many memory cells within the 800px or less space.
   let numberOfCells = document.getElementsByClassName("mid")[0].clientWidth/50
   document.getElementById("memory").innerHTML = '';
   rowLength = Math.floor(numberOfCells);
   // Rows
   for(let j = 0; j < ROWS; j++) {
      let cont = document.createElement("div");
      cont.classList = "row"

      // Columns
      for(let i = 0; i<Math.floor(numberOfCells); i++) {
         let el = document.createElement("span");
         el.innerText = mem[(j*(Math.floor(numberOfCells))+i)];
         el.classList = "cell"
         if((j*(Math.floor(numberOfCells))+i)==pcVal) {
            el.classList += " marked"
         }
         el.id = "m"+(j*(Math.floor(numberOfCells))+i);
         cont.append(el)
      }
      document.getElementById("memory").append(cont);
   }
   moveProgramCounter(pcVal%(ROWS*Math.floor(numberOfCells)))
}

let pc = document.getElementById("pc");
const moveProgramCounter = (index) => {
   pc.style = `left: ${15+((index%rowLength)*50)}px; top: ${55+(56*Math.floor(index/rowLength))}px;`;
}

window.moveProgramCounter = moveProgramCounter

moveProgramCounter()

renderMemory()

window.addEventListener("resize", renderMemory)
