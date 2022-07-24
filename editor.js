
const colors = {
   "+": "00ff00",
   "-": "ff0000"
}

const initEditor = (id) => {

   const editorElement = document.getElementById(id)
   editorElement.style = 'display: block;';
   editorElement.tabIndex = "0";

   editorElement.addEventListener("keydown", (k) => {
      const el = document.createElement("span");
      switch(k.key) {
         case "Shift":
            k.preventDefault()
            break;
         case "Control":
            k.preventDefault()
            break;
         case "Backspace":
            document.getElementById("editor-test").lastChild.remove()
            break;
         case "Enter":
            document.getElementById("editor-test").append(document.createElement("br"));
            k.preventDefault();
            break;
         case "Tab":
            el.innerText = "   ";
            document.getElementById("editor-test").append(el);
            k.preventDefault();
            break;
         default:
            el.innerText = k.key;
            if(colors[k.key]) {
               el.style = `color: #${colors[k.key]};`
            }
            document.getElementById("editor-test").;
            k.preventDefault();
            break;
      }
   })

   return {
      getText: () => {
         console.log("text");
      }
   }
}

export default initEditor
