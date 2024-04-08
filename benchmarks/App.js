"use strict";
exports.__esModule = true;
exports.Hello = void 0;
var react_1 = require("react");
var server_1 = require("react-dom/server");
function Hello(text, children) {
    // const [counter, setCounter] = useState<number>(0)
    return (0, server_1.renderToString)(<div>
			<script src="https://cdn.tailwindcss.com"></script>
			<h1 className="text-4xl font-bold">Hello {text}, </h1>
			{/* <button onClick={() => setCounter(counter + 1)}>Click {counter}</button> */}
		</div>);
}
exports.Hello = Hello;
