import * as monaco from "monaco-editor";

export function initEditor (el, value) {
	return monaco.editor.create(el, {
		value: value,
		minimap: {
			enabled: false
		},
		autoSurround: false,
		roundedSelection: true,
		scrollbar: {
			verticalScrollbarSize: 5,
			horizontalScrollbarSize: 5,
		},
		folding: false,
		lineDecorationsWidth: "1.5ch",
		lineNumbersMinChars: 3
	})
}

/**
 * 
 * @param {monaco.editor.ICodeEditor} editor 
 * @param {*} index 
 */
export function hightlightCharacter(editor, index, lastId) {
	const pos = editor.getModel().getPositionAt(getBfIndex(index, editor.getValue()));
	if(lastId) {
		editor.removeDecorations([lastId])
	}
	return editor.createDecorationsCollection([{
		range: {
			startColumn: pos.column,
			startLineNumber: pos.lineNumber,
			endColumn: pos.column+1,
			endLineNumber: pos.lineNumber
		},
		options: {
			isWholeLine: false,
			inlineClassName: "program-counter"
		}
	}])
}

function getBfIndex(index, value) {
	let i = 0;
	for(const match of String(value).matchAll(/[><\[\]\+-\.,]/g)) {
		if(index == i) {
			return match.index;
		}
		i++;
	}
}