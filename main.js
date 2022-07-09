// WebAssembly boilerplate.
import init, {BF, run} from "./pkg/brainfuck.js";
await init();

run();

console.log(`Hello,
if you wish to change the stepsize for both the start function and step function,
change the respective values, stepsize and runstepsize. You can also change delay

It defaults at:
- stepsize = 1 (How many steps the step button moves)
- runstepsize = 10000 (How many steps the start button moves per iteration)
- delay = 5 (The delay between execution of the start button iterations)
- timeDebug = true (Displays how long it took to execute a step/steps)
- bf (The brainfuck instance)
`)

// Setting code to one of these, randomly.
const programs = [
   `[Hello world]
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`,
   `[Sierpinski triangle, (c) 2016 Daniel B. Cristofani]
++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[-<<<[->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<]>.>+[>>]>+]`
]

document.getElementById("code").value = programs[Math.floor(Math.random()*programs.length)]

// Initializes brainfuck.
// Setting instance to window for ease of access in the js console.
window.BF = BF;
window.bf = BF.new();
// Number of steps per step. See fn steps() in src/lib.rs.
window.stepsize = 1;
window.runstepsize = 10000;
window.delay = 5;

// Helper function for creating buttons.
function button(id, func) {
   document.getElementById(id).addEventListener('click', func);
}

// Memory array for rendering memory.
let mem = new Uint8Array(256, 0);
let pcVal = 0; 

// Step button.
button("sp", () => {
   console.time(`Execution time for step`)
   window.bf.steps(window.stepsize);
   let json = JSON.parse(window.bf.get_state());
   console.timeEnd(`Execution time for step`)
   mem = json.memory;
   // Modulus 30000 so it does not overflow.
   pcVal = json.memory_counter%30000;
   renderMemory();
})


// button("print", () => window.bf.print())
// button("step", () => window.bf.step())
button("comp", () => {
   window.bf.set_code(document.getElementById("code").value)
   let json = JSON.parse(window.bf.get_state());
   pcVal = json.memory_counter%30000;
   mem = json.memory;
   renderMemory();
})
button("clear", () => document.getElementById("cout").innerText = '')

window.timeDebug = true

let running = false;
let intervalId = '';
button("start", () => {
   document.getElementById("start").style = 'display: none;';
   document.getElementById("stop").style = 'display: inline;';

   running = true;

   intervalId = setInterval(() => {
      if(running) {

         if(window.timeDebug)
            console.time(`Execution time for steps`)

         if(!window.bf.steps(window.runstepsize)) {
            clearInterval(intervalId);
            running = false;
            document.getElementById("stop").click();
         };

         if(window.timeDebug)
            console.timeEnd(`Execution time for steps`)

         let json = JSON.parse(window.bf.get_state());
         mem = json.memory;

         // Modulus 30000 so it does not overflow.
         pcVal = json.memory_counter%30000;
         renderMemory();
      }
   }, window.delay)
})

button("stop", () => {
   document.getElementById("start").style = 'display: inline;';
   document.getElementById("stop").style = 'display: none;';

   clearInterval(intervalId);

   running = false;
})

// Initialize memory map.

let rowLength = 0;
const ROWS = 2;

// Display memory.
const renderMemory = () => {
   // Calculate how many memory cells within the 800px or less space.
   document.getElementById("memory").innerHTML = '';

   let numberOfCells = document.getElementsByClassName("mid")[0].clientWidth/50
   rowLength = Math.floor(numberOfCells);
   const offset = Math.floor(pcVal/(Math.floor(numberOfCells)*ROWS))*(Math.floor(numberOfCells)*ROWS)

   // Rows.
   for(let j = 0; j < ROWS; j++) {
      let cont = document.createElement("div");
      cont.classList = "row"

      // Columns.
      for(let i = 0; i<Math.floor(numberOfCells); i++) {
         let el = document.createElement("span");

         const index = (j*(Math.floor(numberOfCells))+i)

         el.innerText = mem[index+offset];
         el.classList = "cell"

         // Mark the cell. Indicating that the program counter points to the cell.
         if((j*(Math.floor(numberOfCells))+i)==pcVal%(ROWS*Math.floor(numberOfCells))) {
            el.classList += " marked"
         }

         el.id = "m"+(j*(Math.floor(numberOfCells))+i);
         cont.append(el)
      }
      document.getElementById("memory").append(cont);
   }
   moveProgramCounter(pcVal%(ROWS*Math.floor(numberOfCells)))
   document.getElementById("loc").innerText = offset+" - "+(offset+32)
}

let pc = document.getElementById("pc");
const moveProgramCounter = (index) => {
   pc.style = `left: ${15+((index%rowLength)*50)}px; top: ${55+(56*Math.floor(index/rowLength))}px;`;
}

window.moveProgramCounter = moveProgramCounter

moveProgramCounter()

renderMemory()

// Redraw the memory on resize.
window.addEventListener("resize", renderMemory)

// Remove the loading screen.
document.getElementById("cover").remove()
